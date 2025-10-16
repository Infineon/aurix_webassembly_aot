mod library_function;
mod linear_memory_access;
mod register_allocation;
mod vb_resolution;
mod visit_operator;

use crate::parse_and_translate::GlobalTranslator;
use crate::{
    isa_model::{
        self, machine_instructions::Instr, Const10, Const16, DataRegister, ExtendedRegister,
        MapperLocation, Register, RegisterOrLargeConst, ValueSize, FRAME_POINTER, GLOBAL_BASE,
        STACK_BASE, STACK_POINTER,
    },
    parse_and_translate::WasmRuntime,
    vb::{AtomicVB, BinaryVB, UnaryVB, VB},
};
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use wasmparser::LocalsReader;
use defmt::Format;


const MAX_LOCAL_REGISTERS: u8 = 8;
const MAX_ALL_REGISTERS: u8 = 16;

/// Wrapper for label indices used in control flow constructs. Values are unique and monotonically increasing.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Format, PartialEq)]
pub struct LabelIndex (usize);


impl From<usize> for LabelIndex {
    fn from(value: usize) -> Self {
        LabelIndex(value)
    }
}

impl LabelIndex {
    fn add(self, rhs: usize) -> Self {
        LabelIndex(self.0 + rhs)
    }

    pub fn to_usize(self) -> usize {
        self.0
    }

    pub fn to_u32(self) -> u32 {
        self.0 as u32
    }
}

/// Represents a control flow label, which can be a block, a loop or an if-else-end structure. Each label has a unique identifier.
/// For Block and Loop, there is a single LabelIndex representing the end of the block or the beginning of the loop.
/// For If, there are two LabelIndexs: one for the else branch and one for the end branch.
pub enum BlockLabel {
    BlockLoop(LabelIndex),
    If { else_label: LabelIndex, end_label: LabelIndex },
}

/// Wrapper for stack height values used in control flow constructs. Values are in bytes.
#[repr(transparent)]
#[derive(Debug, Format, Copy, PartialEq, Clone)]
pub struct StackHeight(usize);

impl From<usize> for StackHeight {
    fn from(value: usize) -> Self {
        StackHeight(value)
    }
}

impl StackHeight {
    fn add(self, rhs: usize) -> Self {
        StackHeight(self.0 + rhs)
    }

    fn to_i16(self) -> i16 {
        self.0 as i16
    }
}

/// Represents the state of the wasm stack after exiting a control flow block.
/// The first element is the height of the stack in bytes after exiting the block.
/// The second element is the size of the result value of the block, if any.
/// The stack state after exiting the block is the same as before entering the block plus the result value at the top, if any
#[derive(Debug, Clone, Copy)]
pub struct StackState {
    pub height: StackHeight,
    pub result_size: Option<ValueSize>,
}

/// Represents the result of a control flow block.
/// Includes the stack state when reaching the end of the block naturally and when branching to the block's label position (e.g. with a break instruction).
/// The two states are usually the same, except for loops where the label state has no result value.
/// That is because breaking to a loop label means jumping to the beginning of the loop, not the end unlike with blocks and ifs.
pub struct BlockResult {
    pub end_state: StackState, // Stack state when reaching the end of the block naturally
    pub label_state: StackState, // Stack state when branching to this block's label position, only different for loops
}

pub struct Translator<'a, 'b> {
    pub vb_stack: Vec<VB>, // valent blocks
    pub locals_map: Vec<MapperLocation>, // location of local variables
    pub locked_register: Option<Register>,
    pub cfg_label_stack: Vec<BlockLabel>, // stack of control flow labels for if, block, loop...
    pub cfg_block_result_stack: Vec<BlockResult>, // stack of control flow block results
    pub vb_stack_ptr_stack: Vec<usize>, // points to the top of the stack at the beginning of each cfg block to allow reset when seeing br
    pub dead_code_flag_stack: Vec<bool>, // to keep track of nested blocks and set dead code flag for the current block
    pub function_type_index: u32,
    pub global_translator: &'a mut GlobalTranslator,
    pub wasm_runtime: &'a mut WasmRuntime<'b>,
    pub cfg_label_map: Vec<Option<usize>>, // offset of each cfg label in the instruction memory
    pub cfg_jobs: Vec<usize>,
}

impl<'a, 'b> Translator<'a, 'b> {
    /// Creates a new Translator instance. Translating a WebAssembly function requires a Translator instance. 
    /// This function generates the machine code of the prologue for the function as well.
    ///
    /// # Arguments
    ///
    /// * `type_index` - The index of the function type in the wasm module.
    /// * `locals_reader` - The locals reader of the function , provided from the wasm Parser
    /// * `global_translator` - The global translator instance.
    /// * `wasm_runtime` - The wasm runtime instance.
    ///
    /// # Returns
    ///  A Translator instance.
    ///
    pub fn new(
        type_index: u32,
        locals_reader: LocalsReader,
        global_translator: &'a mut GlobalTranslator,
        wasm_runtime: &'a mut WasmRuntime<'b>,
    ) -> Self {
        wasm_runtime.add_instruction(Instr::LEA { // initialize frame pointer with stack pointer
            base: STACK_POINTER,
            offset: Const16(0),
            dest: FRAME_POINTER,
        });

        let function_type = wasm_runtime.types[type_index as usize].unwrap_func();
        let params_size: Vec<ValueSize> = function_type // vec of size for every param
            .params()
            .iter()
            .map(ValueSize::from_valtype)
            .collect();

        // initialize the locals offset vector
        // it contains the initial offset of each local variable wrt. the frame pointer before moving them (partially) to registers
        let params_offset: Vec<i16> = {
            if params_size.len() == 0 {
                vec![]
            } else {
                let mut params_offset_inner: Vec<i16> = Vec::new();
                for v in params_size.iter().skip(1) {
                    params_offset_inner.push(v.as_bytes() as i16);
                }
                params_offset_inner.push(0);
                if params_offset_inner.len() > 1 {
                    for i in (0..params_offset_inner.len() - 2).rev() {
                        params_offset_inner[i] += params_offset_inner[i + 1];
                    }
                }
                params_offset_inner
            }
        };

        // if we do bimasking D0 is for the bitmask
        #[cfg(feature = "address-masking")]
        let mut alloc_register_index: u8 = 1;
        #[cfg(not(feature = "address-masking"))]
        let mut alloc_register_index: u8 = 0;
        let mut missed_register: Option<u8> = None;
        let mut locals_map = Vec::new();

        // allocate registers for parameters, if there are not enough registers, the parameter is kept where it is on the stack
        // 2 word params need an extended registers, for 1 word values we put them in a missed spot left by extended registers
        for i in 0..params_offset.len() {
            match params_size[i] {
                ValueSize::Word => match missed_register {
                    None if alloc_register_index < MAX_LOCAL_REGISTERS => {
                        locals_map.push(MapperLocation::DataRegister(DataRegister(
                            alloc_register_index,
                        )));
                        wasm_runtime.add_instruction(Instr::LDW {
                            dest: DataRegister(alloc_register_index),
                            base: FRAME_POINTER,
                            offset: Const16(params_offset[i] as u16),
                        });
                        alloc_register_index += 1;
                    }
                    Some(register_index) => {
                        locals_map.push(MapperLocation::DataRegister(DataRegister(register_index)));
                        wasm_runtime.add_instruction(Instr::LDW {
                            dest: DataRegister(register_index),
                            base: FRAME_POINTER,
                            offset: Const16(params_offset[i] as u16),
                        });
                        missed_register = None;
                    }
                    _ => {
                        locals_map.push(MapperLocation::Frame {
                            size: ValueSize::Word,
                            offset: params_offset[i],
                        });
                    }
                },
                ValueSize::DoubleWord => {
                    if alloc_register_index % 2 == 1 {
                        missed_register = Some(alloc_register_index);
                        alloc_register_index += 1;
                    }
                    if alloc_register_index < MAX_LOCAL_REGISTERS {
                        locals_map.push(MapperLocation::ExtendedRegister(ExtendedRegister(
                            alloc_register_index,
                        )));
                        wasm_runtime.add_instruction(Instr::LDD {
                            dest: ExtendedRegister(alloc_register_index),
                            base: FRAME_POINTER,
                            offset: Const10(params_offset[i]),
                        });
                        alloc_register_index += 2;
                    } else {
                        locals_map.push(MapperLocation::Frame {
                            size: ValueSize::DoubleWord,
                            offset: params_offset[i],
                        });
                    }
                }
            }
        }

        // initialize extended register 14 to zero, as it is used to initialize local variables that are assigned to the stack
        wasm_runtime.add_instruction(Instr::MOV {
            src: RegisterOrLargeConst::Const16(isa_model::Const16(0)),
            dest: isa_model::Register::ExtendedRegister(ExtendedRegister(14)),
        });


        // allocate registers for locals introduced by the function body
        // if there are not enough registers, the local is pushed onto the stack
        // the initial value of the local is zero
        let mut stack_pointer_offset = 0;
        for (count, val_type) in locals_reader.into_iter().map(Result::unwrap) {
            for _ in 0..count {
                match ValueSize::from_valtype(&val_type) {
                    ValueSize::Word => match missed_register {
                        None if alloc_register_index < MAX_LOCAL_REGISTERS => {
                            locals_map.push(MapperLocation::DataRegister(DataRegister(
                                alloc_register_index,
                            )));
                            wasm_runtime.add_instruction(Instr::MOV {
                                src: RegisterOrLargeConst::Const16(isa_model::Const16(0)),
                                dest: isa_model::Register::DataRegister(DataRegister(
                                    alloc_register_index,
                                )),
                            });
                            alloc_register_index += 1;
                        }
                        Some(register_index) => {
                            locals_map
                                .push(MapperLocation::DataRegister(DataRegister(register_index)));
                            wasm_runtime.add_instruction(Instr::MOV {
                                src: RegisterOrLargeConst::Const16(isa_model::Const16(0)),
                                dest: isa_model::Register::DataRegister(DataRegister(
                                    register_index,
                                )),
                            });
                            missed_register = None;
                        }
                        _ => {
                            stack_pointer_offset -= 4;
                            locals_map.push(MapperLocation::Frame {
                                size: ValueSize::Word,
                                offset: stack_pointer_offset,
                            });
                            wasm_runtime.add_instruction(Instr::STW {
                                src: DataRegister(15),
                                base: FRAME_POINTER,
                                offset: Const16(stack_pointer_offset as u16),
                            });
                        }
                    },
                    ValueSize::DoubleWord => {
                        if alloc_register_index % 2 == 1 {
                            missed_register = Some(alloc_register_index);
                            alloc_register_index += 1;
                        }
                        if alloc_register_index < MAX_LOCAL_REGISTERS {
                            locals_map.push(MapperLocation::ExtendedRegister(ExtendedRegister(
                                alloc_register_index,
                            )));
                            wasm_runtime.add_instruction(Instr::MOV {
                                src: RegisterOrLargeConst::Const16(isa_model::Const16(0)),
                                dest: isa_model::Register::ExtendedRegister(ExtendedRegister(
                                    alloc_register_index,
                                )),
                            });
                            alloc_register_index += 2;
                        } else {
                            stack_pointer_offset -= 8;
                            locals_map.push(MapperLocation::Frame {
                                size: ValueSize::DoubleWord,
                                offset: stack_pointer_offset,
                            });
                            wasm_runtime.add_instruction(Instr::STD {
                                src: ExtendedRegister(14),
                                base: FRAME_POINTER,
                                offset: Const10(stack_pointer_offset as i16),
                            });
                        }
                    }
                }
            }
        }

        // update the stack pointer to point to the top of the stack
        wasm_runtime.add_instruction(Instr::LEA {
            base: STACK_POINTER,
            offset: Const16(stack_pointer_offset as u16),
            dest: STACK_POINTER,
        });
        // initialize the stack base to point to the top of the stack
        // the stack base will always point to the top of the stack at the beginning of the function
        wasm_runtime.add_instruction(Instr::MOVAA {
            src: STACK_POINTER,
            dest: STACK_BASE,
        });

        Translator {
            vb_stack: vec![],
            locals_map,
            cfg_label_map: vec![],
            cfg_block_result_stack: vec![],
            locked_register: None,
            cfg_label_stack: vec![],
            dead_code_flag_stack: vec![false],
            function_type_index: type_index,
            vb_stack_ptr_stack: vec![0],
            global_translator,
            wasm_runtime,
            cfg_jobs: vec![],
        }
    }

    /// Translates a machine instruction to binary format and adds it to the instruction array.
    /// Control flow instructions (Jumps and function calls) are marked for later processing, given that the target address is not known at this point.  
    pub fn push_instruction(&mut self, instr: Instr) {
        match instr {
            Instr::CALL { .. } => self
                .global_translator
                .function_call_jobs
                .push(self.wasm_runtime.instructions_count),
            Instr::J { .. }
            | Instr::JEQ { .. }
            | Instr::JNE { .. }
            | Instr::JZT { .. }
            | Instr::LOOPU { .. } => self.cfg_jobs.push(self.wasm_runtime.instructions_count),
            _ => (),
        }
        self.wasm_runtime.add_instruction(instr);
    }

    pub fn add_atomic_vb(&mut self, new_vb: AtomicVB) {
        let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
        if dead_code_flag {
            return;
        }
        self.vb_stack.push(VB::AtomicVB(new_vb));
    }

    pub fn add_unary_vb(&mut self, new_vb: UnaryVB) {
        let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
        if dead_code_flag {
            return;
        }
        let child = self.vb_stack.pop().unwrap();
        self.vb_stack.push(VB::UnaryVB {
            vb: new_vb,
            child: Box::new(child),
        });
    }

    pub fn add_binary_vb(&mut self, new_vb: BinaryVB) {
        let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
        if dead_code_flag {
            return;
        }
        let rhs = self.vb_stack.pop().unwrap();
        let lhs = self.vb_stack.pop().unwrap();
        self.vb_stack.push(VB::BinaryVB {
            vb: new_vb,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        });
    }

    pub fn store_word_to_global(&mut self, offset: u32, src_register: DataRegister) {
        self.push_instruction(Instr::STW {
            src: src_register,
            base: GLOBAL_BASE,
            offset: Const16(offset as u16),
        })
    }

    pub fn load_word_from_global(&mut self, dest_register: DataRegister, offset: u32) {
        self.push_instruction(Instr::LDW {
            dest: dest_register,
            base: GLOBAL_BASE,
            offset: Const16(offset as u16),
        })
    }

    pub fn store_double_word_to_global(&mut self, offset: u32, src_register: ExtendedRegister) {
        self.push_instruction(Instr::STD {
            src: src_register,
            base: GLOBAL_BASE,
            offset: Const10(offset as i16),
        })
    }

    pub fn load_double_word_from_global(&mut self, dest_register: ExtendedRegister, offset: u32) {
        self.push_instruction(Instr::LDD {
            dest: dest_register,
            base: GLOBAL_BASE,
            offset: Const10(offset as i16),
        })
    }
}
