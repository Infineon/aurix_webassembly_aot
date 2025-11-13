#![forbid(unsafe_code)]

//! # VB (Valent Block) Resolution Engine
//!
//! This module implements the core VB resolution logic that converts virtual WebAssembly operand
//! stack expressions into concrete machine code for the Aurix processor.
//!
//! ## Key Concepts
//!
//! - **VB (Valent Block)**: An intermediate representation (IR) for WebAssembly operand stack expressions
//! - **VB Resolution**: The process of converting VB expressions into machine instructions
//! - **Scratch Variables**: Temporary registers and memory locations used during code generation
//! - **Target Mapping**: Specifying where the result of an operation should be placed
//!
//! ## VB Types
//!
//! - **AtomicVB**: Leaf nodes representing constants, locals, globals, and resolved values
//! - **UnaryVB**: Single-operand operations (negation, conversion, loads, etc.)
//! - **BinaryVB**: Two-operand operations (arithmetic, comparison, bitwise, etc.)
//! - **Select**: Conditional selection between two values, the only ternary operation
//!
//! ## Resolution Process
//!
//! 1. **Post-order DFS traversal**: Process child nodes before parent nodes
//! 2. **Register allocation**: Assign scratch registers for intermediate results
//! 3. **Instruction generation**: Emit appropriate machine instructions
//! 4. **Target mapping**: Place final result in the specified location
//!
//! ## Key Functions
//!
//! - `resolve_all()`: Resolves all VBs on the stack to concrete stack locations
//! - `resolve_with_target()`: Resolves a single VB with an optional target location
//! - `dispatch_*()`: Handles resolution for different VB types
//! - `gen_*()`: Generates machine code for specific operations

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use crate::isa_model::{Const16, Const9, DataRegister, ExtendedRegister, Immediate, LocationCouple, MapperLocation, Memsize, Register, RegisterOrConst, RegisterOrLargeConst, RegisterOrSmallConst, SignValue, ValueSize};
use crate::isa_model::machine_instructions::Instr;
use crate::translator::library_function::LibraryFunction;
use crate::translator::Translator;
use crate::vb::{AtomicVB, BinaryVB, UnaryVB, VB};
use super::StackHeight;

// ================================================================================
// FLOATING-POINT COMPARISON CONSTANTS
// ================================================================================

/// Constants for CMPF (floating-point comparison) instruction result bits.
///
/// The CMPF instruction sets specific bits in the result register based on the comparison:
/// - Bit 0: Set if LHS < RHS (less than)
/// - Bit 1: Set if LHS == RHS (equal)
/// - Bit 2: Set if LHS > RHS (greater than)
/// - Bit 3: Set if either operand is NaN (unordered)
///
/// These constants can be combined with bitwise OR to create masks for complex comparisons.
pub struct CmpfBits;

impl CmpfBits {
    /// LHS < RHS (less than)
    pub const LT: u16 = 1 << 0;  // 0b0001

    /// LHS == RHS (equal)
    pub const EQ: u16 = 1 << 1;  // 0b0010

    /// LHS > RHS (greater than)
    pub const GT: u16 = 1 << 2;  // 0b0100

    /// LHS <= RHS (less than or equal)
    pub const LE: u16 = Self::LT | Self::EQ;  // 0b0011

    /// LHS >= RHS (greater than or equal)
    pub const GE: u16 = Self::GT | Self::EQ;  // 0b0110
}


// ================================================================================
//  INSTRUCTION ENUMS
// ================================================================================
//
// These enums are made to allow passing instructions as parameters to functions for code reuse

// for gen_i32_comparison_with_imm_opt
enum ImmCompInstr {
    GEU,
    GE,
    LTU,
    LT
}

// for gen_i32_simple_binary_op
enum SimpleBinaryOpInstr {
    XOR,
    OR,
    AND,
    MUL,
    EQ,
    NE
}

// for gen_single_operand_op
enum SingleOperandInstr {
    CLZ,
    POPCNT,
    UTOF,
    ITOF,
    FTOUZ,
    FTOIZ
}

// for gen_div_rem_op
enum DivRemInstr {
    DIV,
    DIVU
}

// for gen_i64_comparison_op
enum AndCompInstr {
    ANDLTU,
    ANDGEU
}

// for gen_i64_comparison_op
enum OrCompInstr {
    ORLTU,
    ORLT
}

// for gen_f32_binary_op
enum F32BinInstr {
    ADDF,
    SUBF,
    MULF,
    DIVF
}

// for gen_i64_bitwise_with_zero_opt
enum BitwiseZeroOptInstr {
    OR,
    XOR
}

// for gen_i32_shift_op
enum ShiftOpInstr {
    SH,
    SHA
}


impl<'a,'b> Translator<'a,'b> {

    // ================================================================================
    // CORE VB RESOLUTION FUNCTIONS
    // ================================================================================

    /// Resolves all VB expressions on the stack to concrete stack locations.
    ///
    /// This function processes the entire VB stack, converting each virtual expression
    /// into machine code and updating stack offsets accordingly. Already-resolved VBs
    /// are skipped to avoid redundant work.
    ///
    /// ## Process
    /// 1. Iterate through all VBs on the stack
    /// 2. Skip already-resolved VBs (maintain stack offset tracking)
    /// 3. For unresolved VBs:
    ///    - Handle NaN canonicalization if needed
    ///    - Perform post-order DFS traversal for code generation
    ///    - Update stack offset and mark as resolved
    ///
    /// ## Stack Layout
    /// The runtime stack grows downward with each resolved VB consuming space
    /// based on its value size (4 bytes for Word, 8 bytes for DoubleWord).
    pub fn resolve_all(&mut self) {
        let mut stack_offset = StackHeight(0);

        for index in 0..self.vb_stack.len() {
            let mut vb = self.vb_stack[index].clone();
            match vb {
                VB::AtomicVB(AtomicVB::Resolved { offset, .. }) => {
                    // Already resolved - just update our stack offset tracking
                    stack_offset = offset;
                },
                _ => {
                    // Handle NaN canonicalization for floating-point operations
                    if vb.produces_non_canonical_nan() {
                        vb = vb.adjust_for_non_canonical_nan()
                    }

                    let size = vb.val_size(&self.locals_map, &self.global_translator.globals_map);
                    let mut scratch_variable_map = Vec::new();

                    // Perform post-order DFS to generate machine code
                    vb.post_order_dfs(|vb, is_top| {
                        let stack_location = MapperLocation::Stack { size };
                        self.resolve_vb(if is_top { Some(&stack_location) } else { None }, vb, &mut scratch_variable_map);
                    });

                    // Update stack offset and mark as resolved
                    stack_offset = stack_offset.add(size.as_bytes());
                    self.vb_stack[index] = VB::AtomicVB(AtomicVB::Resolved { size, offset: stack_offset.into() })
                }
            }
        }
    }

    /// Resolves a single VB expression from the top of the stack with an optional target location.
    ///
    /// This is the primary function for converting a VB expression into machine code when
    /// you need the result in a specific location (register, memory, etc.) or want to
    /// determine where the result ended up.
    ///
    /// ## Parameters
    /// - `target`: Optional target location where the result should be placed
    ///   - `Some(location)`: Force result to specific location (register, memory, etc.)
    ///   - `None`: Let the resolution choose the most efficient location
    ///
    /// ## Returns
    /// The `MapperLocation` where the result was actually placed. This may differ
    /// from the requested target if optimizations were applied.
    ///
    /// ## Process
    /// 1. Pop the top VB from the stack
    /// 2. Handle NaN canonicalization if needed
    /// 3. Perform post-order DFS traversal
    /// 4. Return the final result location
    pub fn resolve_with_target(&mut self, target: Option<&MapperLocation>) -> MapperLocation {
        let mut vb = self.vb_stack.pop().unwrap();
        if vb.produces_non_canonical_nan() { vb = vb.adjust_for_non_canonical_nan() }
        let mut scratch_variable_map: Vec<MapperLocation> = Vec::new();
        vb.post_order_dfs(|vb, is_top| self.resolve_vb(if is_top { target } else { None }, vb, &mut scratch_variable_map));
        scratch_variable_map.pop().unwrap()
    }

    // ================================================================================
    // VB DISPATCH AND RESOLUTION
    // ================================================================================

    /// Core VB resolution function that dispatches to appropriate handlers based on VB type.
    ///
    /// This function is called during the post-order DFS traversal and is responsible
    /// for generating the appropriate machine code for each VB node.
    ///
    /// ## Parameters
    /// - `potential_target`: Where the result should be placed (if specified)
    /// - `vb`: The VB expression to resolve
    /// - `scratch_variable_map`: Stack of intermediate result locations
    ///
    /// ## VB Type Dispatch
    /// - **AtomicVB**: Constants, locals, globals, resolved values
    /// - **UnaryVB**: Single-operand operations (loads, conversions, arithmetic)
    /// - **BinaryVB**: Two-operand operations (arithmetic, comparisons, bitwise)
    /// - **Select**: Conditional selection (WebAssembly select instruction)
    fn resolve_vb(&mut self, potential_target: Option<&MapperLocation>, vb: &VB, scratch_variable_map: &mut Vec<MapperLocation>) {
        let result = match vb {
            VB::AtomicVB(atomic_vb) => self.dispatch_atomic_vb(atomic_vb, potential_target, scratch_variable_map),
            VB::UnaryVB { vb, .. } => self.dispatch_unary_vb(scratch_variable_map, vb, potential_target),
            VB::BinaryVB { vb, .. } => self.dispatch_binary_vb(scratch_variable_map, vb, potential_target),
            VB::Select { size, .. } => self.gen_select(scratch_variable_map, potential_target, *size),
        };
        scratch_variable_map.push(result);
    }

    // ================================================================================
    // ATOMIC VB RESOLUTION (Constants, Variables, Resolved Values)
    // ================================================================================

    /// Resolves atomic VB expressions (leaf nodes in the expression tree).
    ///
    /// Atomic VBs represent the simplest form of values that don't require computation:
    /// - **Constants**: Immediate values (i32, i64, f32, f64)
    /// - **Local variables**: Function parameters and local variables
    /// - **Global variables**: Module-level variables
    /// - **Resolved values**: Previously computed results on the stack
    /// - **Special values**: Memory size, unreachable markers
    ///
    /// ## Target Handling
    /// If a target location is specified, the atomic value is moved/copied to that location.
    /// Otherwise, the most efficient representation is used (immediate values stay as
    /// immediates, variables reference their storage locations, etc.).
    fn dispatch_atomic_vb(&mut self, atomic_vb: &AtomicVB, potential_target: Option<&MapperLocation>, scratch_variable_map: &mut Vec<MapperLocation>) -> MapperLocation {
        let result = match atomic_vb {
            AtomicVB::I32Const { imm } => MapperLocation::Immediate(Immediate::Word(*imm as u32)),
            AtomicVB::I64Const { imm } => MapperLocation::Immediate(Immediate::DoubleWord(*imm as u64)),
            AtomicVB::F32Const { imm } => MapperLocation::Immediate(Immediate::Word(*imm)),
            AtomicVB::F64Const { imm } => MapperLocation::Immediate(Immediate::DoubleWord(*imm)),
            AtomicVB::Local { index } => self.locals_map[*index as usize].clone(),
            AtomicVB::Global { index } => {
                let (offset, size) = self.global_translator.globals_map[*index as usize];
                MapperLocation::Global { offset, size }
            },
            AtomicVB::Resolved { size, .. } => MapperLocation::Stack { size: *size },
            AtomicVB::MemorySize => MapperLocation::Global { offset: 0, size: ValueSize::Word }, // Memory size is stored in global space at offset 0
        };
        match potential_target {
            Some(target) => result.map_to_location(target, self, scratch_variable_map, &vec![]),
            None => result
        }
    }

    // ================================================================================
    // UNARY VB RESOLUTION (Single-Operand Operations)
    // ================================================================================

    /// Resolves unary VB expressions (operations with a single operand).
    ///
    /// Unary operations include:
    /// - **Arithmetic**: Negation, absolute value, square root
    /// - **Bitwise**: Count leading/trailing zeros, population count
    /// - **Conversions**: Type conversions between integers and floats
    /// - **Comparisons**: Zero comparisons (eqz)
    /// - **Memory loads**: All load operations with different sizes and signedness
    /// - **Math functions**: Ceiling, floor, truncate, nearest (often library calls)
    ///
    /// ## Process
    /// 1. Pop the child operand from the scratch variable stack
    /// 2. Dispatch to the appropriate generator function based on operation type
    /// 3. Many operations are implemented as library function calls for complex operations
    /// 4. Simple operations generate direct machine instructions
    fn dispatch_unary_vb(&mut self, scratch_variable_map: &mut Vec<MapperLocation>, vb: &UnaryVB, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let child = scratch_variable_map.pop().unwrap();
        match vb {
            UnaryVB::I32Clz => self.gen_i32_clz(&child, scratch_variable_map, potential_target),
            UnaryVB::I32Ctz => self.gen_i32_ctz(&child, scratch_variable_map, potential_target),
            UnaryVB::I32PopCnt => self.gen_i32_popcnt(&child, scratch_variable_map, potential_target),
            UnaryVB::F32Abs => self.gen_f32_abs(&child, scratch_variable_map, potential_target),
            UnaryVB::F32Neg => self.gen_f32_neg(&child, scratch_variable_map, potential_target),
            UnaryVB::F32Sqrt => self.call_library_function(potential_target, LibraryFunction::F32Sqrt, vec![&child], ValueSize::Word, ValueSize::Word, scratch_variable_map),
            UnaryVB::F32Ceil => self.call_library_function(potential_target, LibraryFunction::F32Ceil, vec![&child], ValueSize::Word, ValueSize::Word, scratch_variable_map),
            UnaryVB::F32Floor => self.call_library_function(potential_target, LibraryFunction::F32Floor, vec![&child], ValueSize::Word, ValueSize::Word, scratch_variable_map),
            UnaryVB::F32Trunc => self.call_library_function(potential_target, LibraryFunction::F32Trunc, vec![&child], ValueSize::Word, ValueSize::Word, scratch_variable_map),
            UnaryVB::F32Nearest => self.call_library_function(potential_target, LibraryFunction::F32Nearest, vec![&child], ValueSize::Word, ValueSize::Word, scratch_variable_map),
            UnaryVB::I32EqZ => self.gen_i32_eqz(&child, scratch_variable_map, potential_target),
            UnaryVB::I32WrapI64 => match potential_target {
                None => child.lower_half(),
                Some(target) => child.lower_half().map_to_location(target, self, scratch_variable_map, &vec![])
            }
            UnaryVB::I32TruncF32S => self.gen_i32_trunc_f32s(&child, scratch_variable_map, potential_target),
            UnaryVB::I32TruncF32U => self.gen_i32_trunc_f32u(&child, scratch_variable_map, potential_target),
            UnaryVB::I32TruncF64S => self.call_library_function(potential_target, LibraryFunction::I32TruncF64S, vec![&child], ValueSize::DoubleWord, ValueSize::Word, scratch_variable_map),
            UnaryVB::I32TruncF64U => self.call_library_function(potential_target, LibraryFunction::I32TruncF64U, vec![&child], ValueSize::DoubleWord, ValueSize::Word, scratch_variable_map),
            UnaryVB::F32DemoteF64 => self.call_library_function(potential_target, LibraryFunction::F32DemoteF64, vec![&child], ValueSize::DoubleWord, ValueSize::Word, scratch_variable_map),
            UnaryVB::F32ConvertI32S => self.gen_f32_convert_i32s(&child, scratch_variable_map, potential_target),
            UnaryVB::F32ConvertI32U => self.gen_f32_convert_i32u(&child, scratch_variable_map, potential_target),
            UnaryVB::F32ConvertI64S => self.call_library_function(potential_target, LibraryFunction::F32ConvertI64S, vec![&child], ValueSize::DoubleWord, ValueSize::Word, scratch_variable_map),
            UnaryVB::F32ConvertI64U => self.call_library_function(potential_target, LibraryFunction::F32ConvertI64U, vec![&child], ValueSize::DoubleWord, ValueSize::Word, scratch_variable_map),
            UnaryVB::I64Clz => self.call_library_function(potential_target, LibraryFunction::I64Clz, vec![&child], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            UnaryVB::I64Ctz => self.call_library_function(potential_target, LibraryFunction::I64Ctz, vec![&child], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            UnaryVB::I64PopCnt => self.gen_i64_popcnt(&child, scratch_variable_map, potential_target),
            UnaryVB::F64Abs => self.gen_f64_abs(&child, scratch_variable_map, potential_target),
            UnaryVB::F64Neg => self.gen_f64_neg(&child, scratch_variable_map, potential_target),
            UnaryVB::F64Sqrt => self.call_library_function(potential_target, LibraryFunction::F64Sqrt, vec![&child], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            UnaryVB::F64Ceil => self.call_library_function(potential_target, LibraryFunction::F64Ceil, vec![&child], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            UnaryVB::F64Floor => self.call_library_function(potential_target, LibraryFunction::F64Floor, vec![&child], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            UnaryVB::F64Trunc => self.call_library_function(potential_target, LibraryFunction::F64Trunc, vec![&child], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            UnaryVB::F64Nearest => self.call_library_function(potential_target, LibraryFunction::F64Nearest, vec![&child], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            UnaryVB::I64ExtendI32U => self.gen_i64_extend_i32u(potential_target, scratch_variable_map, &child),
            UnaryVB::I64ExtendI32S => self.gen_i64_extend_i32s(potential_target, &child, scratch_variable_map),
            UnaryVB::I64TruncF32S => self.call_library_function(potential_target, LibraryFunction::I64TruncF32S, vec![&child], ValueSize::Word, ValueSize::DoubleWord, scratch_variable_map),
            UnaryVB::I64TruncF32U => self.call_library_function(potential_target, LibraryFunction::I64TruncF32U, vec![&child], ValueSize::Word, ValueSize::DoubleWord, scratch_variable_map),
            UnaryVB::I64TruncF64S => self.call_library_function(potential_target, LibraryFunction::I64TruncF64S, vec![&child], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            UnaryVB::I64TruncF64U => self.call_library_function(potential_target, LibraryFunction::I64TruncF64U, vec![&child], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            UnaryVB::F64ConvertI32S => self.call_library_function(potential_target, LibraryFunction::F64ConvertI32S, vec![&child], ValueSize::Word, ValueSize::DoubleWord, scratch_variable_map),
            UnaryVB::F64ConvertI32U => self.call_library_function(potential_target, LibraryFunction::F64ConvertI32U, vec![&child], ValueSize::Word, ValueSize::DoubleWord, scratch_variable_map),
            UnaryVB::F64ConvertI64S => self.call_library_function(potential_target, LibraryFunction::F64ConvertI64S, vec![&child], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            UnaryVB::F64ConvertI64U => self.call_library_function(potential_target, LibraryFunction::F64ConvertI64U, vec![&child], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            UnaryVB::F64PromoteF32 => self.call_library_function(potential_target, LibraryFunction::F64PromoteF32, vec![&child], ValueSize::Word, ValueSize::DoubleWord, scratch_variable_map),
            UnaryVB::I64EqZ => self.gen_i64_eqz(&child, scratch_variable_map, potential_target),
            UnaryVB::I32Load { offset, align } => self.gen_load(&child, *offset, *align, Memsize::Word, SignValue::Signed, potential_target, scratch_variable_map),
            UnaryVB::I64Load { offset, align } => self.gen_load(&child, *offset, *align, Memsize::DoubleWord, SignValue::Signed, potential_target, scratch_variable_map),
            UnaryVB::F32Load { offset, align } => self.gen_load(&child, *offset, *align, Memsize::Word, SignValue::Signed, potential_target, scratch_variable_map),
            UnaryVB::F64Load { offset, align } => self.gen_load(&child, *offset, *align, Memsize::DoubleWord, SignValue::Signed, potential_target, scratch_variable_map),
            UnaryVB::I32Load8s { offset, align } => self.gen_load(&child, *offset, *align, Memsize::Byte, SignValue::Signed, potential_target, scratch_variable_map),
            UnaryVB::I32Load8u { offset, align } => self.gen_load(&child, *offset, *align, Memsize::Byte, SignValue::Unsigned, potential_target, scratch_variable_map),
            UnaryVB::I32Load16s { offset, align } => self.gen_load(&child, *offset, *align, Memsize::HalfWord, SignValue::Signed, potential_target, scratch_variable_map),
            UnaryVB::I32Load16u { offset, align } => self.gen_load(&child, *offset, *align, Memsize::HalfWord, SignValue::Unsigned, potential_target, scratch_variable_map),
            UnaryVB::I64Load8s { offset, align } => self.gen_load(&child, *offset, *align, Memsize::Byte, SignValue::Signed, potential_target, scratch_variable_map),
            UnaryVB::I64Load8u { offset, align } => self.gen_load(&child, *offset, *align, Memsize::Byte, SignValue::Unsigned, potential_target, scratch_variable_map),
            UnaryVB::I64Load16s { offset, align } => self.gen_load(&child, *offset, *align, Memsize::HalfWord, SignValue::Signed, potential_target, scratch_variable_map),
            UnaryVB::I64Load16u { offset, align } => self.gen_load(&child, *offset, *align, Memsize::HalfWord, SignValue::Unsigned, potential_target, scratch_variable_map),
            UnaryVB::I64Load32s { offset, align } => self.gen_load(&child, *offset, *align, Memsize::Word, SignValue::Signed, potential_target, scratch_variable_map),
            UnaryVB::I64Load32u { offset, align } => self.gen_load(&child, *offset, *align, Memsize::Word, SignValue::Unsigned, potential_target, scratch_variable_map),
        }
    }

    // ================================================================================
    // BINARY VB RESOLUTION (Two-Operand Operations)
    // ================================================================================

    /// Resolves binary VB expressions (operations with two operands).
    ///
    /// Binary operations include:
    /// - **Arithmetic**: Addition, subtraction, multiplication, division, remainder
    /// - **Comparisons**: Equality, inequality, less than, greater than, etc.
    /// - **Bitwise**: AND, OR, XOR, shifts, rotations
    /// - **Floating-point**: All floating-point arithmetic and comparisons
    ///
    /// ## Process
    /// 1. Pop the right-hand operand (RHS) from scratch variable stack
    /// 2. Pop the left-hand operand (LHS) from scratch variable stack
    /// 3. Dispatch to appropriate generator based on operation type
    /// 4. Complex operations (64-bit, floating-point) often use library calls
    /// 5. Simple operations generate direct machine instructions
    ///
    /// ## Operand Order
    /// Note that operands are popped in reverse order (RHS first, then LHS) due to
    /// stack-based evaluation order in the post-order traversal.
    fn dispatch_binary_vb(&mut self, scratch_variable_map: &mut Vec<MapperLocation>, vb: &BinaryVB, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let rhs = scratch_variable_map.pop().unwrap();
        let lhs = scratch_variable_map.pop().unwrap();
        match vb {
            BinaryVB::I32Eq => self.gen_i32_eq(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I32Ne => self.gen_i32_ne(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I32LtS => self.gen_i32_lts(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I32LtU => self.gen_i32_ltu(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I32GeS => self.gen_i32_ges(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I32GeU => self.gen_i32_geu(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I32LeS => self.gen_i32_les(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I32LeU => self.gen_i32_leu(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I32GtS => self.gen_i32_gts(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I32GtU => self.gen_i32_gtu(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I64Eq => self.gen_i64_eq(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I64Ne => self.gen_i64_ne(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I64LtS => self.gen_i64_lts(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I64LtU => self.gen_i64_ltu(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I64GeS => self.gen_i64_ges(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I64GeU => self.gen_i64_geu(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I64GtS => self.gen_i64_gts(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I64GtU => self.gen_i64_gtu(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I64LeS => self.gen_i64_les(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I64LeU => self.gen_i64_leu(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::F32Eq => self.gen_f32_eq(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::F32Ne => self.gen_f32_ne(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::F32Lt => self.gen_f32_lt(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::F32Ge => self.gen_f32_ge(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::F64Eq => self.call_library_function(potential_target, LibraryFunction::F64Eq, vec![&lhs, &rhs], ValueSize::DoubleWord, ValueSize::Word, scratch_variable_map),
            BinaryVB::F64Ne => self.call_library_function(potential_target, LibraryFunction::F64Ne, vec![&lhs, &rhs], ValueSize::DoubleWord, ValueSize::Word, scratch_variable_map),
            BinaryVB::F64Lt => self.call_library_function(potential_target, LibraryFunction::F64Lt, vec![&lhs, &rhs], ValueSize::DoubleWord, ValueSize::Word, scratch_variable_map),
            BinaryVB::F64Ge => self.call_library_function(potential_target, LibraryFunction::F64Ge, vec![&lhs, &rhs], ValueSize::DoubleWord, ValueSize::Word, scratch_variable_map),
            BinaryVB::I32Add => self.gen_i32_add(potential_target, scratch_variable_map, &lhs, &rhs),
            BinaryVB::I32Sub => self.gen_i32_sub(potential_target, scratch_variable_map, &lhs, &rhs),
            BinaryVB::I32Mul => self.gen_i32_mul(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I32DivS => self.gen_i32_div_s(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I32DivU => self.gen_i32_div_u(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I32RemS => self.gen_i32_rem_s(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I32RemU => self.gen_i32_rem_u(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I32And => self.gen_i32_and(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I32Or => self.gen_i32_or(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I32Xor => self.gen_i32_xor(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I32Shl => self.gen_i32_shl(&rhs, scratch_variable_map, &lhs, potential_target),
            BinaryVB::I32ShrS => self.gen_i32_shr_s(&rhs, scratch_variable_map, &lhs, potential_target),
            BinaryVB::I32ShrU => self.gen_i32_shr_u(&rhs, scratch_variable_map, &lhs, potential_target),
            BinaryVB::I32Rotl => self.gen_i32_rotl(&rhs, scratch_variable_map, &lhs, potential_target),
            BinaryVB::I32Rotr => self.gen_i32_rotr(&rhs, scratch_variable_map, &lhs, potential_target),
            BinaryVB::I64Add => self.gen_i64_add(potential_target, scratch_variable_map, &lhs, &rhs),
            BinaryVB::I64Sub => self.gen_i64_sub(potential_target, scratch_variable_map, &lhs, &rhs),
            BinaryVB::I64Mul => self.gen_i64_mul(potential_target, scratch_variable_map, &lhs, &rhs),
            BinaryVB::I64DivS => self.call_library_function(potential_target, LibraryFunction::I64DivS, vec![&lhs, &rhs], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            BinaryVB::I64DivU => self.call_library_function(potential_target, LibraryFunction::I64DivU, vec![&lhs, &rhs], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            BinaryVB::I64RemS => self.call_library_function(potential_target, LibraryFunction::I64RemS, vec![&lhs, &rhs], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            BinaryVB::I64RemU => self.call_library_function(potential_target, LibraryFunction::I64RemU, vec![&lhs, &rhs], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            BinaryVB::I64And => self.gen_i64_and(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I64Or => self.gen_i64_or(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I64Xor => self.gen_i64_xor(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::I64Shl => self.call_library_function(potential_target, LibraryFunction::I64Shl, vec![&lhs, &rhs], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            BinaryVB::I64ShrS => self.call_library_function(potential_target, LibraryFunction::I64ShrS, vec![&lhs, &rhs], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            BinaryVB::I64ShrU => self.call_library_function(potential_target, LibraryFunction::I64ShrU, vec![&lhs, &rhs], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            BinaryVB::I64Rotl => self.call_library_function(potential_target, LibraryFunction::I64Rotl, vec![&lhs, &rhs], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            BinaryVB::I64Rotr => self.call_library_function(potential_target, LibraryFunction::I64Rotr, vec![&lhs, &rhs], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            BinaryVB::F32Add => self.gen_f32_add(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::F32Sub => self.gen_f32_sub(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::F32Mul => self.gen_f32_mul(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::F32Div => self.gen_f32_div(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::F32Min => self.call_library_function(potential_target, LibraryFunction::F32Min, vec![&lhs, &rhs], ValueSize::Word, ValueSize::Word, scratch_variable_map),
            BinaryVB::F32Max => self.call_library_function(potential_target, LibraryFunction::F32Max, vec![&lhs, &rhs], ValueSize::Word, ValueSize::Word, scratch_variable_map),
            BinaryVB::F32CopySign => self.gen_f32_copysign(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::F64Add => self.call_library_function(potential_target, LibraryFunction::F64Add, vec![&lhs, &rhs], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            BinaryVB::F64Sub => self.call_library_function(potential_target, LibraryFunction::F64Sub, vec![&lhs, &rhs], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            BinaryVB::F64Mul => self.call_library_function(potential_target, LibraryFunction::F64Mul, vec![&lhs, &rhs], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            BinaryVB::F64Div => self.call_library_function(potential_target, LibraryFunction::F64Div, vec![&lhs, &rhs], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            BinaryVB::F64Min => self.call_library_function(potential_target, LibraryFunction::F64Min, vec![&lhs, &rhs], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            BinaryVB::F64Max => self.call_library_function(potential_target, LibraryFunction::F64Max, vec![&lhs, &rhs], ValueSize::DoubleWord, ValueSize::DoubleWord, scratch_variable_map),
            BinaryVB::F64CopySign => self.gen_f64_copysign(potential_target, scratch_variable_map, &lhs, &rhs),
            BinaryVB::F32Gt => self.gen_f32_gt(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::F32Le => self.gen_f32_le(&lhs, &rhs, scratch_variable_map, potential_target),
            BinaryVB::F64Gt => self.call_library_function(potential_target, LibraryFunction::F64Gt, vec![&lhs, &rhs], ValueSize::DoubleWord, ValueSize::Word, scratch_variable_map),
            BinaryVB::F64Le => self.call_library_function(potential_target, LibraryFunction::F64Le, vec![&lhs, &rhs], ValueSize::DoubleWord, ValueSize::Word, scratch_variable_map),
        }
    }

    // ================================================================================
    // OPERATION GENERATORS (Machine Code Generation)
    // ================================================================================

    /// Generates machine code for the WebAssembly `select` instruction.
    ///
    /// The select instruction chooses between two values based on a condition:
    /// `select(lhs, rhs, selector) = selector ? lhs : rhs`
    ///
    /// ## Implementation Strategy
    ///
    /// ### For 32-bit values (Word):
    /// - **Optimization**: Use `SELN` instruction when LHS is a small immediate
    /// - **General case**: Use `SEL` instruction with three registers
    ///
    /// ### For 64-bit values (DoubleWord):
    /// - Use conditional branches due to lack of 64-bit select instruction
    /// - Generate: `if (selector == 0) use RHS else use LHS`
    ///
    /// ## Parameters
    /// - Values are popped from scratch stack in order: selector, rhs, lhs
    /// - `size`: Value size (Word or DoubleWord) determines implementation
    fn gen_select(&mut self, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>, size: ValueSize) -> MapperLocation {
        let selector = scratch_variable_map.pop().unwrap();
        let rhs = scratch_variable_map.pop().unwrap();
        let lhs = scratch_variable_map.pop().unwrap();
        match size {
            ValueSize::Word => {
                let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
                match lhs {
                    MapperLocation::Immediate(imm)  if imm.as_i32() >> 8 == 0 || imm.as_i32() >> 8 == -1 => {
                        let selector_register = selector.map_to_data_register(None, self, scratch_variable_map, &vec![rhs.clone()]);
                        let rhs_register = rhs.map_to_data_register(None, self, scratch_variable_map, &vec![MapperLocation::DataRegister(selector_register)]);
                        self.push_instruction(Instr::SELN { selector: selector_register, lhs: rhs_register, rhs: RegisterOrConst::new_const(imm.as_i32() as u16), dest: dest_register });
                    },
                    _ => {
                        let selector_register = selector.map_to_data_register(None, self, scratch_variable_map, &vec![rhs.clone(), lhs.clone()]);
                        let rhs_register = rhs.map_to_register_or_const(SignValue::Signed, self, scratch_variable_map, &vec![MapperLocation::DataRegister(selector_register), lhs.clone()]);
                        let lhs_register = lhs.map_to_data_register(None, self, scratch_variable_map, &vec![MapperLocation::DataRegister(selector_register), rhs_register.to_mapper_location()]);
                        self.push_instruction(Instr::SEL { selector: selector_register, lhs: lhs_register, rhs: rhs_register, dest: dest_register });
                    }
                }
                dest_register.map_to_location(potential_target, self, scratch_variable_map)
            },
            ValueSize::DoubleWord => { // TODO: refactor to use SELN for 64-bit
                let dest_register = self.get_dest_extended_register(potential_target, scratch_variable_map, &vec![]);
                let selector_register = selector.map_to_data_register(None, self, scratch_variable_map, &vec![lhs.clone(), rhs.clone()]);
                self.push_instruction(Instr::JEQ { target: self.cfg_label_map.len().into(), lhs: selector_register, rhs: RegisterOrSmallConst::new_const(0) });
                lhs.map_to_extended_register(Some(dest_register), self, scratch_variable_map, &vec![]);
                self.push_instruction(Instr::J { target: (self.cfg_label_map.len() + 1).into() });
                self.cfg_label_map.push(Some(self.wasm_runtime.instructions_count));
                rhs.map_to_extended_register(Some(dest_register), self, scratch_variable_map, &vec![]);
                self.cfg_label_map.push(Some(self.wasm_runtime.instructions_count));
                dest_register.map_to_location(potential_target, self, scratch_variable_map)
            }
        }
    }

    // ================================================================================
    // FLOATING-POINT OPERATION GENERATORS
    // ================================================================================

    fn gen_f32_comparison_op(&mut self, mask: u16, add_ne:bool, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_data_registers(self, scratch_variable_map);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::CMPF { lhs: lhs_register, rhs: rhs_register, dest: dest_register });
        self.push_instruction(Instr::AND { lhs: dest_register, rhs: RegisterOrConst::new_const(mask), dest: dest_register });
        if add_ne {
            self.push_instruction(Instr::NE { lhs: dest_register, rhs: RegisterOrConst::new_const(0), dest: dest_register });
        }
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_f32_gt(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_f32_comparison_op(CmpfBits::GT, true, lhs, rhs, scratch_variable_map, potential_target)
    }

    fn gen_f32_ge(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_f32_comparison_op(CmpfBits::GE, true, lhs, rhs, scratch_variable_map, potential_target)
    }

    fn gen_f32_lt(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_f32_comparison_op(CmpfBits::LT, false, lhs, rhs, scratch_variable_map, potential_target)
    }

    fn gen_f32_le(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_f32_comparison_op(CmpfBits::LE, true, lhs, rhs, scratch_variable_map, potential_target)
    }

    fn gen_f64_copysign(&mut self, potential_target: Option<&MapperLocation>, scratch_variable_map: &mut Vec<MapperLocation>, lhs: &MapperLocation, rhs: &MapperLocation) -> MapperLocation {
        let (ExtendedRegister(index_lhs), ExtendedRegister(index_rhs)) = (lhs, rhs).map_to_extended_registers(self, scratch_variable_map);
        let lower_lhs = DataRegister(index_lhs);
        let upper_lhs = DataRegister(index_lhs + 1);
        let upper_rhs = DataRegister(index_rhs + 1);
        let ExtendedRegister(index_dest) = self.get_dest_extended_register(potential_target, scratch_variable_map, &vec![MapperLocation::DataRegister(upper_rhs)]);
        lower_lhs.map_to_location(Some(&MapperLocation::new_data_register(index_dest)), self, scratch_variable_map);
        let intermediate = self.next_available_data_register(scratch_variable_map, &vec![MapperLocation::ExtendedRegister(ExtendedRegister(index_dest))]);
        self.push_instruction(Instr::SH { src: upper_lhs, count: RegisterOrConst::new_const(1), dest: DataRegister(index_dest + 1) });
        self.push_instruction(Instr::SH { src: DataRegister(index_dest + 1), count: RegisterOrConst::new_const(-1i16 as u16), dest: DataRegister(index_dest + 1) });
        self.push_instruction(Instr::SH { src: upper_rhs, count: RegisterOrConst::new_const(-31i16 as u16), dest: intermediate });
        self.push_instruction(Instr::SH { src: intermediate, count: RegisterOrConst::new_const(31 as u16), dest: intermediate });
        self.push_instruction(Instr::OR { lhs: DataRegister(index_dest + 1), rhs: RegisterOrConst::DataRegister(intermediate), dest: DataRegister(index_dest + 1) });
        ExtendedRegister(index_dest).map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_f32_copysign(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_data_registers(self, scratch_variable_map);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![MapperLocation::DataRegister(rhs_register)]);
        let intermediate = self.next_available_data_register(scratch_variable_map, &vec![MapperLocation::DataRegister(dest_register)]);
        self.push_instruction(Instr::SH { src: lhs_register, count: RegisterOrConst::new_const(1), dest: dest_register });
        self.push_instruction(Instr::SH { src: dest_register, count: RegisterOrConst::new_const(-1i16 as u16), dest: dest_register });
        self.push_instruction(Instr::SH { src: rhs_register, count: RegisterOrConst::new_const(-31i16 as u16), dest: intermediate });
        self.push_instruction(Instr::SH { src: intermediate, count: RegisterOrConst::new_const(31), dest: intermediate });
        self.push_instruction(Instr::OR { lhs: dest_register, rhs: RegisterOrConst::DataRegister(intermediate), dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_f32_binary_op(&mut self, instr: F32BinInstr, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_data_registers(self, scratch_variable_map);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        match instr {
            F32BinInstr::ADDF => self.push_instruction(Instr::ADDF { lhs: lhs_register, rhs: rhs_register, dest: dest_register }),
            F32BinInstr::SUBF => self.push_instruction(Instr::SUBF { lhs: lhs_register, rhs: rhs_register, dest: dest_register }),
            F32BinInstr::MULF => self.push_instruction(Instr::MULF { lhs: lhs_register, rhs: rhs_register, dest: dest_register }),
            F32BinInstr::DIVF => self.push_instruction(Instr::DIVF { lhs: lhs_register, rhs: rhs_register, dest: dest_register }),
        }
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }
    
    fn gen_f32_div(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_f32_binary_op(F32BinInstr::DIVF, lhs, rhs, scratch_variable_map, potential_target)
    }

    fn gen_f32_mul(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_f32_binary_op(F32BinInstr::MULF, lhs, rhs, scratch_variable_map, potential_target)
    }

    fn gen_f32_sub(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_f32_binary_op(F32BinInstr::SUBF, lhs, rhs, scratch_variable_map, potential_target)
    }

    fn gen_f32_add(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_f32_binary_op(F32BinInstr::ADDF, lhs, rhs, scratch_variable_map, potential_target)
    }

    // This is no longer used, we make an external call instead
    fn _gen_f32_min(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_data_registers(self, scratch_variable_map);
        let cmp_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![MapperLocation::DataRegister(lhs_register), MapperLocation::DataRegister(rhs_register)]);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::CMPF { lhs: lhs_register, rhs: rhs_register, dest: cmp_register });
        self.push_instruction(Instr::JZT { src: cmp_register, n: 3, target: self.cfg_label_map.len().into() });
        self.push_instruction(Instr::MOVH { src: Const16(0x7FC0), dest: dest_register });
        self.push_instruction(Instr::J { target: (self.cfg_label_map.len() + 3).into() });
        self.cfg_label_map.push(Some(self.wasm_runtime.instructions_count));
        self.push_instruction(Instr::JZT { src: cmp_register, n: 2, target: self.cfg_label_map.len().into() });
        rhs_register.map_to_location(Some(&MapperLocation::DataRegister(dest_register)), self, scratch_variable_map);
        self.push_instruction(Instr::J { target: (self.cfg_label_map.len() + 2).into() });
        self.cfg_label_map.push(Some(self.wasm_runtime.instructions_count));
        self.push_instruction(Instr::JZT { src: cmp_register, n: 1, target: self.cfg_label_map.len().into() });
        self.push_instruction(Instr::JEQ { target: self.cfg_label_map.len().into(), lhs: lhs_register, rhs: RegisterOrSmallConst::DataRegister(rhs_register) });
        self.push_instruction(Instr::MOVH { src: Const16(0), dest: dest_register });
        self.push_instruction(Instr::J { target: (self.cfg_label_map.len() + 1).into() });
        self.cfg_label_map.push(Some(self.wasm_runtime.instructions_count));
        lhs_register.map_to_location(Some(&MapperLocation::DataRegister(dest_register)), self, scratch_variable_map);
        self.cfg_label_map.push(Some(self.wasm_runtime.instructions_count));
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    // ================================================================================
    // INTEGER BITWISE OPERATION GENERATORS
    // ================================================================================

    /// Function to generate 64-bit bitwise operations with optimization for zero constants.
    /// You don't need to generate code for OR and XOR if one of the operands is zero,
    /// as the result will be the other operand.
    fn gen_i64_bitwise_with_zero_opt(&mut self, instr: BitwiseZeroOptInstr, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register_const_couple) = (lhs, rhs).map_abelian_large_children_to_register_or_const(SignValue::Unsigned, self, scratch_variable_map);
        let lower_lhs = lhs_register.lower_half();
        let upper_lhs = lhs_register.upper_half();
        let (lower_rhs, upper_rhs) = rhs_register_const_couple;
        let ExtendedRegister(index_dest) = self.get_dest_extended_register(potential_target, scratch_variable_map, &vec![]);
        match lower_rhs {
            RegisterOrConst::Const9(Const9(0)) => {
                lower_lhs.map_to_location(Some(&MapperLocation::new_data_register(index_dest)), self, scratch_variable_map);
            },
            _ => {
                match instr {
                    BitwiseZeroOptInstr::XOR => self.push_instruction(Instr::XOR { lhs: lower_lhs, rhs: lower_rhs, dest: DataRegister(index_dest) }),
                    BitwiseZeroOptInstr::OR => self.push_instruction(Instr::OR { lhs: lower_lhs, rhs: lower_rhs, dest: DataRegister(index_dest) }),
                }
            }
        };
        match upper_rhs {
            RegisterOrConst::Const9(Const9(0)) => {
                upper_lhs.map_to_location(Some(&MapperLocation::new_data_register(index_dest + 1)), self, scratch_variable_map);
            },
            _ => {
                match instr {
                    BitwiseZeroOptInstr::XOR => self.push_instruction(Instr::XOR { lhs: upper_lhs, rhs: upper_rhs, dest: DataRegister(index_dest + 1) }),
                    BitwiseZeroOptInstr::OR => self.push_instruction(Instr::OR { lhs: upper_lhs, rhs: upper_rhs, dest: DataRegister(index_dest + 1) }),
                }
            }
        };
        ExtendedRegister(index_dest).map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i64_xor(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_i64_bitwise_with_zero_opt(BitwiseZeroOptInstr::XOR, lhs, rhs, scratch_variable_map, potential_target)
    }

    fn gen_i64_or(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_i64_bitwise_with_zero_opt(BitwiseZeroOptInstr::OR, lhs, rhs, scratch_variable_map, potential_target)
    }

    fn gen_i64_and(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register_const_couple) = (lhs, rhs).map_abelian_large_children_to_register_or_const(SignValue::Unsigned, self, scratch_variable_map);
        let ExtendedRegister(index_dest) = self.get_dest_extended_register(potential_target, scratch_variable_map, &vec![]);
        let lower_lhs = lhs_register.lower_half();
        let upper_lhs = lhs_register.upper_half();
        let (lower_rhs, upper_rhs) = rhs_register_const_couple;
        self.push_instruction(Instr::AND { lhs: lower_lhs, rhs: lower_rhs, dest: DataRegister(index_dest) });
        self.push_instruction(Instr::AND { lhs: upper_lhs, rhs: upper_rhs, dest: DataRegister(index_dest + 1) });
        ExtendedRegister(index_dest).map_to_location(potential_target, self, scratch_variable_map)
    }

    //SUBX and ADDX set the carry flag for the lower half, SUBC and ADDC use it for the upper half.

    fn gen_i64_sub(&mut self, potential_target: Option<&MapperLocation>, scratch_variable_map: &mut Vec<MapperLocation>, lhs: &MapperLocation, rhs: &MapperLocation) -> MapperLocation {
        let (ExtendedRegister(index_lhs), ExtendedRegister(index_rhs)) = (lhs, rhs).map_to_extended_registers(self, scratch_variable_map);
        let ExtendedRegister(index_dest) = self.get_dest_extended_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::SUBX { lhs: DataRegister(index_lhs), rhs: DataRegister(index_rhs), dest: DataRegister(index_dest) });
        self.push_instruction(Instr::SUBC { lhs: DataRegister(index_lhs + 1), rhs: DataRegister(index_rhs + 1), dest: DataRegister(index_dest + 1) });
        ExtendedRegister(index_dest).map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i64_add(&mut self, potential_target: Option<&MapperLocation>, scratch_variable_map: &mut Vec<MapperLocation>, lhs: &MapperLocation, rhs: &MapperLocation) -> MapperLocation {
        let ExtendedRegister(index_dest) = self.get_dest_extended_register(potential_target, scratch_variable_map, &vec![]);
        let (ExtendedRegister(index_lhs), ExtendedRegister(index_rhs)) = (lhs, rhs).map_to_extended_registers(self, scratch_variable_map);
        self.push_instruction(Instr::ADDX { lhs: DataRegister(index_lhs), rhs: RegisterOrConst::new_register(index_rhs), dest: DataRegister(index_dest) });
        self.push_instruction(Instr::ADDC { lhs: DataRegister(index_lhs + 1), rhs: RegisterOrConst::new_register(index_rhs + 1), dest: DataRegister(index_dest + 1) });
        ExtendedRegister(index_dest).map_to_location(potential_target, self, scratch_variable_map)
    }

    // ================================================================================
    // INTEGER SHIFT AND ROTATE OPERATION GENERATORS
    // ================================================================================

    fn gen_i32_shift_op(&mut self, negate_count: bool, instr: ShiftOpInstr, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, lhs: &MapperLocation, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let rhs_register_const = match *rhs {
            MapperLocation::Immediate(imm) => {
                let masked_count = (imm.as_u32() & 0x1F) as u16;
                if negate_count {
                    RegisterOrConst::new_const((-(masked_count as i16)) as u16)
                } else {
                    RegisterOrConst::new_const(masked_count)
                }
            },
            _ => {
                let count_register = rhs.map_to_data_register(None, self, scratch_variable_map, &vec![]);
                self.push_instruction(Instr::AND { lhs: count_register, rhs: RegisterOrConst::new_const(0x1F), dest: count_register }); // mask to 5 bits
                if negate_count {
                    self.push_instruction(Instr::RSUB0 { src: count_register });
                }
                RegisterOrConst::DataRegister(count_register)
            }
        };
        let lhs_register: DataRegister = lhs.map_to_data_register(None, self, scratch_variable_map, &vec![rhs_register_const.to_mapper_location()]);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        match instr {
            ShiftOpInstr::SH => self.push_instruction(Instr::SH { src: lhs_register, count: rhs_register_const, dest: dest_register }),
            ShiftOpInstr::SHA => self.push_instruction(Instr::SHA { src: lhs_register, count: rhs_register_const, dest: dest_register }),
        }
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_shr_u(&mut self, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, lhs: &MapperLocation, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_i32_shift_op(true, ShiftOpInstr::SH, rhs, scratch_variable_map, lhs, potential_target)
    }

    fn gen_i32_shr_s(&mut self, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, lhs: &MapperLocation, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_i32_shift_op(true, ShiftOpInstr::SHA, rhs, scratch_variable_map, lhs, potential_target)
    }

    fn gen_i32_shl(&mut self, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, lhs: &MapperLocation, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_i32_shift_op(false, ShiftOpInstr::SH, rhs, scratch_variable_map, lhs, potential_target)
    }

    fn i32_imm_rotl(&mut self, count: u16, scratch_variable_map: &mut Vec<MapperLocation>, lhs: &MapperLocation, potential_target: Option<&MapperLocation>) -> MapperLocation {
        if count == 0 { // No rotation needed and can't EXTR with width 0
            if potential_target.is_some() {
                return lhs.map_to_location(potential_target.unwrap(), self, scratch_variable_map, &vec![])
            }
            else {
                return lhs.clone();
            }
        }
        let lhs_register: DataRegister = lhs.map_to_data_register(None, self, scratch_variable_map, &vec![]);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        let intermediate = self.next_available_data_register(scratch_variable_map, &vec![MapperLocation::DataRegister(dest_register), MapperLocation::DataRegister(lhs_register)]);
        self.push_instruction(Instr::EXTRUI { src: lhs_register, width: Const9::new(count), pos: Const9(32 - count), dest: intermediate });
        self.push_instruction(Instr::SH { src: lhs_register, count: RegisterOrConst::new_const(count), dest: dest_register });
        self.push_instruction(Instr::OR { lhs: dest_register, rhs: RegisterOrConst::DataRegister(intermediate), dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_rotl(&mut self, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, lhs: &MapperLocation, potential_target: Option<&MapperLocation>) -> MapperLocation {
        match *rhs {
            MapperLocation::Immediate(imm) => {
                let count = (imm.as_u32() & 0x1F) as u16;
                self.i32_imm_rotl(count, scratch_variable_map, lhs, potential_target)
            },
            _ => {
                self.call_library_function(potential_target, LibraryFunction::I32Rotl, vec![&lhs, &rhs], ValueSize::Word, ValueSize::Word, scratch_variable_map)
            }
        }
    }


    fn gen_i32_rotr(&mut self, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, lhs: &MapperLocation, potential_target: Option<&MapperLocation>) -> MapperLocation {
        match *rhs {
            MapperLocation::Immediate(imm) => {
                // rotr by n is equivalent to rotl by (32 - n)
                let count = ((32 - (imm.as_u32() & 0x1F)) & 0x1F) as u16; // convoluted way to do 32 - n because imm can be > 32 and we can't overflow
                self.i32_imm_rotl(count, scratch_variable_map, lhs, potential_target)
            },
            _ => {
                self.call_library_function(potential_target, LibraryFunction::I32Rotr, vec![&lhs, &rhs], ValueSize::Word, ValueSize::Word, scratch_variable_map)
            }
        }
    }

    fn gen_simple_binary_op(&mut self, instr: SimpleBinaryOpInstr, sign: SignValue,lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register_const) = (lhs, rhs).map_abelian_children_to_register_or_const(sign, self, scratch_variable_map);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        match instr {
            SimpleBinaryOpInstr::XOR => self.push_instruction(Instr::XOR { lhs: lhs_register, rhs: rhs_register_const, dest: dest_register }),
            SimpleBinaryOpInstr::OR => self.push_instruction(Instr::OR { lhs: lhs_register, rhs: rhs_register_const, dest: dest_register }),
            SimpleBinaryOpInstr::AND => self.push_instruction(Instr::AND { lhs: lhs_register, rhs: rhs_register_const, dest: dest_register }),
            SimpleBinaryOpInstr::MUL => self.push_instruction(Instr::MUL { lhs: lhs_register, rhs: rhs_register_const, dest: dest_register }),
            SimpleBinaryOpInstr::EQ => self.push_instruction(Instr::EQ { lhs: lhs_register, rhs: rhs_register_const, dest: dest_register }),
            SimpleBinaryOpInstr::NE => self.push_instruction(Instr::NE { lhs: lhs_register, rhs: rhs_register_const, dest: dest_register }),
        }
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_xor(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_simple_binary_op(SimpleBinaryOpInstr::XOR, SignValue::Unsigned, lhs, rhs, scratch_variable_map, potential_target)
    }

    fn gen_i32_or(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_simple_binary_op(SimpleBinaryOpInstr::OR, SignValue::Unsigned, lhs, rhs, scratch_variable_map, potential_target)
    }

    fn gen_i32_and(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_simple_binary_op(SimpleBinaryOpInstr::AND, SignValue::Unsigned, lhs, rhs, scratch_variable_map, potential_target)
    }

    fn gen_i32_mul(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_simple_binary_op(SimpleBinaryOpInstr::MUL, SignValue::Signed, lhs, rhs, scratch_variable_map, potential_target)
    }

    fn gen_i32_eq(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_simple_binary_op(SimpleBinaryOpInstr::EQ, SignValue::Signed, lhs, rhs, scratch_variable_map, potential_target)
    }

    fn gen_i32_ne(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_simple_binary_op(SimpleBinaryOpInstr::NE, SignValue::Signed, lhs, rhs, scratch_variable_map, potential_target)
    }

    // ================================================================================
    // INTEGER ARITHMETIC OPERATION GENERATORS
    // ================================================================================

    fn gen_div_rem_op_impl(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>, instr: DivRemInstr, result_half: u8) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_data_registers(self, scratch_variable_map);
        let ExtendedRegister(index) = self.next_available_extended_register(scratch_variable_map, &vec![]);
        match instr {
            DivRemInstr::DIV => self.push_instruction(Instr::DIV { lhs: lhs_register, rhs: rhs_register, dest: ExtendedRegister(index) }),
            DivRemInstr::DIVU => self.push_instruction(Instr::DIVU { lhs: lhs_register, rhs: rhs_register, dest: ExtendedRegister(index) }),
        }
        DataRegister(index + result_half).map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_rem_u(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_div_rem_op_impl(lhs, rhs, scratch_variable_map, potential_target, DivRemInstr::DIVU, 1)
    }

    fn gen_i32_rem_s(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_div_rem_op_impl(lhs, rhs, scratch_variable_map, potential_target, DivRemInstr::DIV, 1)
    }

    fn gen_i32_div_u(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_div_rem_op_impl(lhs, rhs, scratch_variable_map, potential_target, DivRemInstr::DIVU, 0)
    }

    fn gen_i32_div_s(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_div_rem_op_impl(lhs, rhs, scratch_variable_map, potential_target, DivRemInstr::DIV, 0)
    }

    // Generate multiplication using simple binary operation macro


    fn gen_i64_mul(&mut self, potential_target: Option<&MapperLocation>, scratch_variable_map: &mut Vec<MapperLocation>, lhs: &MapperLocation, rhs: &MapperLocation) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_extended_registers(self, scratch_variable_map);
        let intermediate = self.next_available_extended_register(scratch_variable_map, &vec![MapperLocation::ExtendedRegister(lhs_register), MapperLocation::ExtendedRegister(rhs_register)]);
        self.push_instruction(Instr::MULU { lhs: lhs_register.lower_half(), rhs: RegisterOrConst::DataRegister(rhs_register.lower_half()), dest: intermediate });
        self.push_instruction(Instr::MADD { lhs: lhs_register.lower_half(), rhs: rhs_register.upper_half(), acc: intermediate.upper_half(), dest: intermediate.upper_half() });
        self.push_instruction(Instr::MADD { lhs: lhs_register.upper_half(), rhs: rhs_register.lower_half(), acc: intermediate.upper_half(), dest: intermediate.upper_half() });
        intermediate.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_sub(&mut self, potential_target: Option<&MapperLocation>, scratch_variable_map: &mut Vec<MapperLocation>, lhs: &MapperLocation, rhs: &MapperLocation) -> MapperLocation {
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        match (lhs, rhs) {
            (operand, MapperLocation::Immediate(imm)) => {
                let operand_register = operand.map_to_data_register(None, self, scratch_variable_map, &vec![]);
                let immediate = -imm.as_i32();
                let lower_immediate = immediate as u16;
                let sign_extension = if (immediate as i16) < 0 { 0xffff } else { 0 };
                let upper_immediate = (immediate >> 16) as u16 - sign_extension;
                if lower_immediate != 0 {
                    self.push_instruction(Instr::ADDI { lhs: operand_register, rhs: Const16::new(lower_immediate), dest: dest_register });
                }
                if upper_immediate != 0 {
                    self.push_instruction(Instr::ADDIH { lhs: dest_register, rhs: Const16::new(upper_immediate), dest: dest_register });
                }
            },
            (MapperLocation::Immediate(imm), operand) => {
                let operand_register = operand.map_to_data_register(None, self, scratch_variable_map, &vec![]);
                let immediate = imm.as_i32();
                if immediate >> 8 == 0 || immediate >> 8 == -1 { // TODO: replace with fits_as_comparison_immediate (is it actually better?)
                    self.push_instruction(Instr::RSUB { lhs: Const9::new(immediate as u16), rhs: operand_register, dest: dest_register });
                } else {
                    // Do -lhs + rhs and then negate the result
                    let immediate = -immediate;
                    let lower_immediate = immediate as u16;
                    let sign_extension = if (immediate as i16) < 0 { 0xffff } else { 0 };
                    let upper_immediate = (immediate >> 16) as u16 - sign_extension;

                    self.push_instruction(Instr::ADDI { lhs: operand_register, rhs: Const16::new(lower_immediate), dest: dest_register });

                    if upper_immediate != 0 {
                        self.push_instruction(Instr::ADDIH { lhs: dest_register, rhs: Const16::new(upper_immediate), dest: dest_register });
                    }
                    self.push_instruction(Instr::RSUB0 { src: dest_register });
                }
            },
            (lhs, rhs) => {
                let (lhs_register, rhs_register) = (lhs, rhs).map_to_data_registers(self, scratch_variable_map);
                self.push_instruction(Instr::SUB { lhs: lhs_register, rhs: rhs_register, dest: dest_register });
            }
        }
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_add(&mut self, potential_target: Option<&MapperLocation>, scratch_variable_map: &mut Vec<MapperLocation>, lhs: &MapperLocation, rhs: &MapperLocation) -> MapperLocation {
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        match (lhs, rhs) {
            (operand, MapperLocation::Immediate(imm)) | (MapperLocation::Immediate(imm), operand) => {
                let operand_register = operand.map_to_data_register(None, self, scratch_variable_map, &vec![]);
                let immediate = imm.as_i32();
                let lower_immediate = immediate as u16;
                let sign_extension = if (immediate as i16) < 0 { 0xffff } else { 0 };
                let upper_immediate = (immediate >> 16) as u16 - sign_extension;

                if lower_immediate != 0 {
                    self.push_instruction(Instr::ADDI { lhs: operand_register, rhs: Const16::new(lower_immediate), dest: dest_register });
                }

                if upper_immediate != 0 {
                    self.push_instruction(Instr::ADDIH { lhs: dest_register, rhs: Const16::new(upper_immediate), dest: dest_register });
                }
            },
            (lhs, rhs) => {
                let (lhs_register, rhs_register) = (lhs, rhs).map_to_data_registers(self, scratch_variable_map);
                self.push_instruction(Instr::ADD { lhs: lhs_register, rhs: RegisterOrConst::DataRegister(rhs_register), dest: dest_register });
            }
        }
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }


    /// Pattern: CMPF + AND with 2 + EQ with target value (2 for equality, 0 for inequality)
    fn gen_f32_eq_style_op(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>, compare_value: u16) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_data_registers(self, scratch_variable_map);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::CMPF { lhs: lhs_register, rhs: rhs_register, dest: dest_register });
        self.push_instruction(Instr::AND { lhs: dest_register, rhs: RegisterOrConst::new_const(2), dest: dest_register });
        self.push_instruction(Instr::EQ { lhs: dest_register, rhs: RegisterOrConst::new_const(compare_value), dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    // Equality: check if bit 1 is set (result equals 2)
    fn gen_f32_eq(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_f32_eq_style_op(lhs, rhs, scratch_variable_map, potential_target, 2)
    }

    // Inequality: check if bit 1 is not set (result equals 0)
    fn gen_f32_ne(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_f32_eq_style_op(lhs, rhs, scratch_variable_map, potential_target, 0)
    }



    // Both halves must be equal for overall equality
    fn gen_i64_eq(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register_const_couple) = (lhs, rhs).map_abelian_large_children_to_register_or_const(SignValue::Signed, self, scratch_variable_map);
        let lower_lhs = lhs_register.lower_half();
        let upper_lhs = lhs_register.upper_half();
        let (lower_rhs, upper_rhs) = rhs_register_const_couple;
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![MapperLocation::DataRegister(upper_lhs), upper_rhs.to_mapper_location()]);
        let intermediate = self.next_available_data_register(scratch_variable_map, &vec![MapperLocation::DataRegister(dest_register)]);
        self.push_instruction(Instr::EQ { lhs: lower_lhs, rhs: lower_rhs, dest: dest_register });
        self.push_instruction(Instr::EQ { lhs: upper_lhs, rhs: upper_rhs, dest: intermediate });
        self.push_instruction(Instr::AND { lhs: dest_register, rhs: RegisterOrConst::DataRegister(intermediate), dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    // Either half being not equal means overall inequality
    fn gen_i64_ne(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register_const_couple) = (lhs, rhs).map_abelian_large_children_to_register_or_const(SignValue::Signed, self, scratch_variable_map);
        let lower_lhs = lhs_register.lower_half();
        let upper_lhs = lhs_register.upper_half();
        let (lower_rhs, upper_rhs) = rhs_register_const_couple;
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![MapperLocation::DataRegister(upper_lhs), upper_rhs.to_mapper_location()]);
        let intermediate = self.next_available_data_register(scratch_variable_map, &vec![MapperLocation::DataRegister(dest_register)]);
        self.push_instruction(Instr::NE { lhs: lower_lhs, rhs: lower_rhs, dest: dest_register });
        self.push_instruction(Instr::NE { lhs: upper_lhs, rhs: upper_rhs, dest: intermediate });
        self.push_instruction(Instr::OR { lhs: dest_register, rhs: RegisterOrConst::DataRegister(intermediate), dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    /// ## Parameters:
    /// - `imm_instr`: Instruction to use when immediate optimization applies on the rhs operand
    /// - `imm_inc`: Value to add to immediate for `imm_instr` (+1 or 0)
    /// - `rev_imm_instr`: Instruction to use when immediate optimization applies on lhs operand and you need to reverse the operation
    /// - `rev_imm_inc`: Value to add to immediate for `rev_imm_instr`
    /// - `reverse_operands`: Boolean indicating if operands should be swapped for register-register case
    /// - `reg_instr`: Instruction to use for register-register case
    ///
    /// ## Automatic Inference:
    /// - **Immediate conditions**:
    ///   - Unsigned: `(immediate + increment) >> 9 == 0` (9-bit range check)
    ///   - Signed: `(immediate + increment) >> 8 == 0 || (immediate + increment) >> 8 == -1` (8-bit signed range check)
    /// - **Immediate side detection**: Automatically tries both lhs and rhs for immediate optimization
    ///
    /// ## Logic:
    /// For register case: Only LT and GE exist in Tricore, so you have to swap operands for GT and LE. eg: x GT y <=> y LT x
    /// For immediate optimization: immediate has to be rhs in Tricore, so you can't swap operands like in register case.
    /// Instead, you have to adjust the immediate value (add +1 or 0) eg: x GT c <=> x GE (c+1)
    /// The immediate can be at most 9 bits, so if c or c+1 is out of range, we store it in a register and fall back on the register case
    fn gen_i32_comparison_with_imm_opt(
        &mut self,
        lhs: &MapperLocation,
        rhs: &MapperLocation,
        scratch_variable_map: &mut Vec<MapperLocation>,
        potential_target: Option<&MapperLocation>,
        imm_inc: u32,
        rev_imm_inc: u32,
        reverse_operands: bool,
        sign: SignValue,
        imm_instr: ImmCompInstr,
        rev_imm_instr: ImmCompInstr,
        reg_instr: ImmCompInstr,
    ) -> MapperLocation
    {
        // Try immediate optimization on right side first (lhs OP rhs_imm)
        'outer_rhs: {
            if let MapperLocation::Immediate(imm) = rhs {
                let (adjusted_imm, overflowed) = imm.as_u32().overflowing_add(imm_inc);
                if overflowed {
                    break 'outer_rhs;
                }
                let adjusted_imm = Immediate::Word(adjusted_imm as u32);
                if adjusted_imm.fits_as_comparison_immediate(sign) {
                    let lhs_register = lhs.map_to_data_register(None, self, scratch_variable_map, &vec![]);
                    let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
                    match imm_instr {
                        ImmCompInstr::GEU => self.push_instruction(Instr::GEU { lhs: lhs_register, rhs: RegisterOrConst::new_const(adjusted_imm.as_u32() as u16), dest: dest_register }),
                        ImmCompInstr::GE => self.push_instruction(Instr::GE { lhs: lhs_register, rhs: RegisterOrConst::new_const(adjusted_imm.as_u32() as u16), dest: dest_register }),
                        ImmCompInstr::LTU => self.push_instruction(Instr::LTU { lhs: lhs_register, rhs: RegisterOrConst::new_const(adjusted_imm.as_u32() as u16), dest: dest_register }),
                        ImmCompInstr::LT => self.push_instruction(Instr::LT { lhs: lhs_register, rhs: RegisterOrConst::new_const(adjusted_imm.as_u32() as u16), dest: dest_register }),
                    }
                    return dest_register.map_to_location(potential_target, self, scratch_variable_map);
                }
            }
        }

        // Try immediate optimization on left side (lhs_imm OP rhs)
        'outer_lhs: {
            if let MapperLocation::Immediate(imm) = lhs {
                let (adjusted_imm, overflowed) = imm.as_u32().overflowing_add(rev_imm_inc);
                if overflowed {
                    break 'outer_lhs;
                }
                let adjusted_imm = Immediate::Word(adjusted_imm as u32);
                if adjusted_imm.fits_as_comparison_immediate(sign) {
                    let rhs_register = rhs.map_to_data_register(None, self, scratch_variable_map, &vec![]);
                    let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
                    match rev_imm_instr {
                        ImmCompInstr::GEU => self.push_instruction(Instr::GEU { lhs: rhs_register, rhs: RegisterOrConst::new_const(adjusted_imm.as_u32() as u16), dest: dest_register }),
                        ImmCompInstr::GE => self.push_instruction(Instr::GE { lhs: rhs_register, rhs: RegisterOrConst::new_const(adjusted_imm.as_u32() as u16), dest: dest_register }),
                        ImmCompInstr::LTU => self.push_instruction(Instr::LTU { lhs: rhs_register, rhs: RegisterOrConst::new_const(adjusted_imm.as_u32() as u16), dest: dest_register }),
                        ImmCompInstr::LT => self.push_instruction(Instr::LT { lhs: rhs_register, rhs: RegisterOrConst::new_const(adjusted_imm.as_u32() as u16), dest: dest_register }),
                    }
                    return dest_register.map_to_location(potential_target, self, scratch_variable_map);
                }
            }
        }

        // Register-register case: apply operand swapping based on semantic intent
        if reverse_operands {
            let rhs_register = rhs.map_to_data_register(None, self, scratch_variable_map, &vec![lhs.clone()]);
            let lhs_register = lhs.map_to_register_or_const(sign, self, scratch_variable_map, &vec![MapperLocation::DataRegister(rhs_register)]);
            let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
            match reg_instr {
                ImmCompInstr::GEU => self.push_instruction(Instr::GEU { lhs: rhs_register, rhs: lhs_register, dest: dest_register }),
                ImmCompInstr::GE => self.push_instruction(Instr::GE { lhs: rhs_register, rhs: lhs_register, dest: dest_register }),
                ImmCompInstr::LTU => self.push_instruction(Instr::LTU { lhs: rhs_register, rhs: lhs_register, dest: dest_register }),
                ImmCompInstr::LT => self.push_instruction(Instr::LT { lhs: rhs_register, rhs: lhs_register, dest: dest_register }),
            }
            dest_register.map_to_location(potential_target, self, scratch_variable_map)
        } else {
            let rhs_register = rhs.map_to_register_or_const(sign, self, scratch_variable_map, &vec![lhs.clone()]);
            let lhs_register = lhs.map_to_data_register(None, self, scratch_variable_map, &vec![rhs_register.to_mapper_location()]);
            let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
            match reg_instr {
                ImmCompInstr::GEU => self.push_instruction(Instr::GEU { lhs: lhs_register, rhs: rhs_register, dest: dest_register }),
                ImmCompInstr::GE => self.push_instruction(Instr::GE { lhs: lhs_register, rhs: rhs_register, dest: dest_register }),
                ImmCompInstr::LTU => self.push_instruction(Instr::LTU { lhs: lhs_register, rhs: rhs_register, dest: dest_register }),
                ImmCompInstr::LT => self.push_instruction(Instr::LT { lhs: lhs_register, rhs: rhs_register, dest: dest_register }),
            }
            dest_register.map_to_location(potential_target, self, scratch_variable_map)
        }
    }


    fn gen_i32_gtu(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_i32_comparison_with_imm_opt(lhs, rhs, scratch_variable_map, potential_target, 1, 0, true, 
            SignValue::Unsigned, ImmCompInstr::GEU, ImmCompInstr::LTU, ImmCompInstr::LTU)
    }

    fn gen_i32_gts(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_i32_comparison_with_imm_opt(lhs, rhs, scratch_variable_map, potential_target, 1, 0, true, 
            SignValue::Signed, ImmCompInstr::GE, ImmCompInstr::LT, ImmCompInstr::LT)
    }

    fn gen_i32_leu(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_i32_comparison_with_imm_opt(lhs, rhs, scratch_variable_map, potential_target, 1, 0, true,
            SignValue::Unsigned, ImmCompInstr::LTU, ImmCompInstr::GEU, ImmCompInstr::GEU)
    }

    fn gen_i32_les(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_i32_comparison_with_imm_opt(lhs, rhs, scratch_variable_map, potential_target, 1, 0, true,
            SignValue::Signed, ImmCompInstr::LT, ImmCompInstr::GE, ImmCompInstr::GE)
    }

    fn gen_i32_geu(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_i32_comparison_with_imm_opt(lhs, rhs, scratch_variable_map, potential_target, 0, 1, false,
            SignValue::Unsigned, ImmCompInstr::GEU, ImmCompInstr::LTU, ImmCompInstr::GEU)
    }

    fn gen_i32_ges(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_i32_comparison_with_imm_opt(lhs, rhs, scratch_variable_map, potential_target, 0, 1, false,
            SignValue::Signed, ImmCompInstr::GE, ImmCompInstr::LT, ImmCompInstr::GE)
    }

    fn gen_i32_ltu(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_i32_comparison_with_imm_opt(lhs, rhs, scratch_variable_map, potential_target, 0, 1, false,
            SignValue::Unsigned, ImmCompInstr::LTU, ImmCompInstr::GEU, ImmCompInstr::LTU)
    }

    fn gen_i32_lts(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_i32_comparison_with_imm_opt(lhs, rhs, scratch_variable_map, potential_target, 0, 1, false,
            SignValue::Signed, ImmCompInstr::LT, ImmCompInstr::GE, ImmCompInstr::LT)
    }

    fn gen_i64_comparison_op(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>, and_swap: bool, or_swap: bool, and_instr: AndCompInstr, or_instr: OrCompInstr) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_extended_registers(self, scratch_variable_map);
        let intermediate = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![MapperLocation::ExtendedRegister(lhs_register), MapperLocation::ExtendedRegister(rhs_register)]);
        self.push_instruction(Instr::EQ { lhs: lhs_register.upper_half(), rhs: RegisterOrConst::DataRegister(rhs_register.upper_half()), dest: intermediate });
        if and_swap {
            match and_instr {
                AndCompInstr::ANDLTU => self.push_instruction(Instr::ANDLTU { lhs: rhs_register.lower_half(), rhs: RegisterOrConst::DataRegister(lhs_register.lower_half()), dest: intermediate }),
                AndCompInstr::ANDGEU => self.push_instruction(Instr::ANDGEU { lhs: rhs_register.lower_half(), rhs: RegisterOrConst::DataRegister(lhs_register.lower_half()), dest: intermediate }),
            }
        } else {
            match and_instr {
                AndCompInstr::ANDLTU => self.push_instruction(Instr::ANDLTU { lhs: lhs_register.lower_half(), rhs: RegisterOrConst::DataRegister(rhs_register.lower_half()), dest: intermediate }),
                AndCompInstr::ANDGEU => self.push_instruction(Instr::ANDGEU { lhs: lhs_register.lower_half(), rhs: RegisterOrConst::DataRegister(rhs_register.lower_half()), dest: intermediate }),
            }
        }
        if or_swap {
            match or_instr {
                OrCompInstr::ORLT => self.push_instruction(Instr::ORLT { lhs: rhs_register.upper_half(), rhs: RegisterOrConst::DataRegister(lhs_register.upper_half()), dest: intermediate }),
                OrCompInstr::ORLTU => self.push_instruction(Instr::ORLTU { lhs: rhs_register.upper_half(), rhs: RegisterOrConst::DataRegister(lhs_register.upper_half()), dest: intermediate }),
            }
        } else {
            match or_instr {
                OrCompInstr::ORLT => self.push_instruction(Instr::ORLT { lhs: lhs_register.upper_half(), rhs: RegisterOrConst::DataRegister(rhs_register.upper_half()), dest: intermediate }),
                OrCompInstr::ORLTU => self.push_instruction(Instr::ORLTU { lhs: lhs_register.upper_half(), rhs: RegisterOrConst::DataRegister(rhs_register.upper_half()), dest: intermediate }),
            }
        }
        intermediate.map_to_location(potential_target, self, scratch_variable_map)
    }

    // Generate 64-bit comparison operations using specialized macros
    fn gen_i64_lts(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_i64_comparison_op(lhs, rhs, scratch_variable_map, potential_target, false, false, AndCompInstr::ANDLTU, OrCompInstr::ORLT)
    }
    fn gen_i64_ltu(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_i64_comparison_op(lhs, rhs, scratch_variable_map, potential_target, false, false, AndCompInstr::ANDLTU, OrCompInstr::ORLTU)
    }
    fn gen_i64_ges(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_i64_comparison_op(lhs, rhs, scratch_variable_map, potential_target, false, true, AndCompInstr::ANDGEU, OrCompInstr::ORLT)
    }
    fn gen_i64_geu(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_i64_comparison_op(lhs, rhs, scratch_variable_map, potential_target, false, true, AndCompInstr::ANDGEU, OrCompInstr::ORLTU)
    }
    fn gen_i64_les(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_i64_comparison_op(lhs, rhs, scratch_variable_map, potential_target, true, false, AndCompInstr::ANDGEU, OrCompInstr::ORLT)
    }
    fn gen_i64_leu(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_i64_comparison_op(lhs, rhs, scratch_variable_map, potential_target, true, false, AndCompInstr::ANDGEU, OrCompInstr::ORLTU)
    }
    fn gen_i64_gts(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_i64_comparison_op(lhs, rhs, scratch_variable_map, potential_target, true, true, AndCompInstr::ANDLTU, OrCompInstr::ORLT)
    }
    fn gen_i64_gtu(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_i64_comparison_op(lhs, rhs, scratch_variable_map, potential_target, true, true, AndCompInstr::ANDLTU, OrCompInstr::ORLTU)
    }




    // ================================================================================
    // INTEGER COMPARISON OPERATION GENERATORS
    // ================================================================================

    // i32 version: direct EQ with 0
    fn gen_i32_eqz(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let child_register = child.map_to_data_register(None, self, scratch_variable_map, &vec![]);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::EQ { lhs: child_register, rhs: RegisterOrConst::new_const(0), dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    // i64 version: OR both halves then EQ with 0
    fn gen_i64_eqz(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let ExtendedRegister(register_index) = child.map_to_extended_register(None, self, scratch_variable_map, &vec![]);
        let lower_register = DataRegister::new(register_index);
        let upper_register = DataRegister::new(register_index + 1);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::OR { lhs: lower_register, rhs: RegisterOrConst::DataRegister(upper_register), dest: dest_register });
        self.push_instruction(Instr::EQ { lhs: dest_register, rhs: RegisterOrConst::new_const(0), dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }


    fn gen_i64_extend_i32s(&mut self, potential_target: Option<&MapperLocation>, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>) -> MapperLocation {
        let target = match potential_target {
            Some(MapperLocation::ExtendedRegister(register)) => Some(*register),
            _ => None,
        };
        child.map_to_extended_register(target, self, scratch_variable_map, &vec![]).map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i64_extend_i32u(&mut self, potential_target: Option<&MapperLocation>, scratch_variable_map: &mut Vec<MapperLocation>, child: &MapperLocation) -> MapperLocation {
        let dest = self.get_dest_extended_register(potential_target, scratch_variable_map, &vec![]);
        child.map_to_data_register(Some(dest.lower_half()), self, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::MOV { src: RegisterOrLargeConst::new_const(0), dest: Register::DataRegister(dest.upper_half()) });
        dest.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_f64_neg(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let src = child.map_to_extended_register(None, self, scratch_variable_map, &vec![]);
        let dest = self.get_dest_extended_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::ADDIH { lhs: src.upper_half(), rhs: Const16(0x8000), dest: dest.upper_half() });
        src.lower_half().map_to_location(Some(&MapperLocation::DataRegister(dest.lower_half())), self, scratch_variable_map);
        dest.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_f64_abs(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let src = child.map_to_extended_register(None, self, scratch_variable_map, &vec![]);
        let dest = self.get_dest_extended_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::SH { src: src.upper_half(), count: RegisterOrConst::new_const(1), dest: dest.upper_half() });
        self.push_instruction(Instr::SH { src: dest.upper_half(), count: RegisterOrConst::new_const(-1i16 as u16), dest: dest.upper_half() });
        src.lower_half().map_to_location(Some(&MapperLocation::DataRegister(dest.lower_half())), self, scratch_variable_map);
        dest.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i64_popcnt(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let src = child.map_to_extended_register(None, self, scratch_variable_map, &vec![]);
        let dest = self.get_dest_extended_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::POPCNT { src: src.lower_half(), dest: dest.lower_half() });
        self.push_instruction(Instr::POPCNT { src: src.upper_half(), dest: dest.upper_half() });
        self.push_instruction(Instr::ADD { lhs: dest.lower_half(), rhs: RegisterOrConst::DataRegister(dest.upper_half()), dest: dest.lower_half() });
        self.push_instruction(Instr::MOV { src: RegisterOrLargeConst::new_const(0), dest: Register::DataRegister(dest.upper_half()) });
        dest.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_single_operand_op(&mut self, instr: SingleOperandInstr, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let child_register = child.map_to_data_register(None, self, scratch_variable_map, &vec![]);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        match instr {
            SingleOperandInstr::CLZ => self.push_instruction(Instr::CLZ { src: child_register, dest: dest_register }),
            SingleOperandInstr::UTOF => self.push_instruction(Instr::UTOF { src: child_register, dest: dest_register }),
            SingleOperandInstr::ITOF => self.push_instruction(Instr::ITOF { src: child_register, dest: dest_register }),
            SingleOperandInstr::FTOUZ => self.push_instruction(Instr::FTOUZ { src: child_register, dest: dest_register }),
            SingleOperandInstr::FTOIZ => self.push_instruction(Instr::FTOIZ { src: child_register, dest: dest_register }),
            SingleOperandInstr::POPCNT => self.push_instruction(Instr::POPCNT { src: child_register, dest: dest_register }),
        }
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    // Generate conversion operations using macro
    fn gen_f32_convert_i32u(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_single_operand_op(SingleOperandInstr::UTOF, child, scratch_variable_map, potential_target)
    }
    fn gen_f32_convert_i32s(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_single_operand_op(SingleOperandInstr::ITOF, child, scratch_variable_map, potential_target)
    }
    fn gen_i32_trunc_f32u(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_single_operand_op(SingleOperandInstr::FTOUZ, child, scratch_variable_map, potential_target)
    }
    fn gen_i32_trunc_f32s(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_single_operand_op(SingleOperandInstr::FTOIZ, child, scratch_variable_map, potential_target)
    }



    fn gen_f32_neg(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let child_register = child.map_to_data_register(None, self, scratch_variable_map, &vec![]);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::ADDIH { lhs: child_register, rhs: Const16::new(0x8000), dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_f32_abs(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let child_register = child.map_to_data_register(None, self, scratch_variable_map, &vec![]);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::SH { src: child_register, count: RegisterOrConst::new_const(1), dest: dest_register });
        self.push_instruction(Instr::SH { src: dest_register, count: RegisterOrConst::new_const(-1i16 as u16), dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    // Generate simple unary operations using macro
    fn gen_i32_popcnt(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_single_operand_op(SingleOperandInstr::POPCNT, child, scratch_variable_map, potential_target)
    }

    // ====================================================================
    // BIT MANIPULATION AND UTILITY OPERATION GENERATORS
    // ====================================================================
    // Functions for generating bit manipulation operations (CLZ, CTZ) and
    // memory load operations.

    // CTZ requires SHUFFLE preprocessing, so implement manually
    fn gen_i32_ctz(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let child_register = child.map_to_data_register(None, self, scratch_variable_map, &vec![]);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::SHUFFLE { src: child_register, dest: dest_register, mask: Const9::new(0x11B) });
        self.push_instruction(Instr::CLZ { src: dest_register, dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_clz(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        self.gen_single_operand_op(SingleOperandInstr::CLZ, child, scratch_variable_map, potential_target)
    }

    fn _gen_i64_clz(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let src = child.map_to_extended_register(None, self, scratch_variable_map, &vec![]);
        let dest = self.get_dest_extended_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::CLZ {src: src.lower_half(), dest: dest.lower_half()});
        self.push_instruction(Instr::CLZ {src: src.upper_half(), dest: dest.upper_half()});
        self.push_instruction(Instr::ADDI {lhs: dest.upper_half() , rhs: Const16(-32i16 as u16), dest: dest.upper_half()});
        self.push_instruction(Instr::CADDN { lhs: dest.lower_half() , rhs: RegisterOrConst::DataRegister(dest.upper_half()), cond: dest.upper_half(), dest: dest.upper_half() });
        self.push_instruction(Instr::ADDI {lhs: dest.upper_half() , rhs: Const16(32i16 as u16), dest: dest.upper_half()});
        self.push_instruction(Instr::MOV {src: RegisterOrLargeConst::DataRegister(dest.upper_half()), dest: Register::DataRegister(dest.lower_half())});
        self.push_instruction(Instr::MOV {src: RegisterOrLargeConst::new_const(0), dest: Register::DataRegister(dest.upper_half())});
        dest.lower_half().map_to_location(Some(&MapperLocation::ExtendedRegister(dest)), self, scratch_variable_map);
        dest.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn _gen_i64_ctz(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let src = child.map_to_extended_register(None, self, scratch_variable_map, &vec![]);
        let dest = self.get_dest_extended_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::SHUFFLE { src: src.lower_half(), dest: dest.lower_half(), mask: Const9::new(0x11B)});
        self.push_instruction(Instr::CLZ {src: dest.lower_half(), dest: dest.lower_half()});
        self.push_instruction(Instr::SHUFFLE { src: src.upper_half(), dest: dest.upper_half(), mask: Const9::new(0x11B)});
        self.push_instruction(Instr::CLZ {src: dest.upper_half(), dest: dest.upper_half()});
        self.push_instruction(Instr::ADDI {lhs: dest.lower_half() , rhs: Const16(-32i16 as u16), dest: dest.lower_half()});
        self.push_instruction(Instr::CADDN { lhs: dest.upper_half() , rhs: RegisterOrConst::DataRegister(dest.lower_half()), cond: dest.lower_half(), dest: dest.lower_half() });
        self.push_instruction(Instr::ADDI {lhs: dest.lower_half() , rhs: Const16(32i16 as u16), dest: dest.lower_half()});
        self.push_instruction(Instr::MOV {src: RegisterOrLargeConst::new_const(0), dest: Register::DataRegister(dest.upper_half())});
        dest.lower_half().map_to_location(Some(&MapperLocation::DataRegister(dest.upper_half())), self, scratch_variable_map);
        dest.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_load(&mut self, child: &MapperLocation, offset:u32, align:u8, src_size: Memsize, ext_sign: SignValue, potential_target: Option<&MapperLocation>, scratch_variable_map : &mut Vec<MapperLocation> ) -> MapperLocation{
        let location = match *child {
            MapperLocation::Immediate(imm) => {
                let immediate = imm.as_u32();
                MapperLocation::LinearMemory { static_offset: immediate as usize + offset as usize, src_size, dynamic_offset: None, ext_sign, align}
            },
            _ => {
                MapperLocation::LinearMemory { static_offset: offset as usize, src_size, dynamic_offset: Some(Box::new(child.clone())), ext_sign, align }
            }
        };
        match potential_target {
            None => location,
            Some(target) => location.map_to_location(target, self, scratch_variable_map, &vec![])
        }
    }
}

// ====================================================================
// END OF VB RESOLUTION MODULE
// ====================================================================
// This completes the VBResolver implementation with comprehensive support
// for all WebAssembly operations and Aurix-specific machine code generation.