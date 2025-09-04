#![forbid(unsafe_code)]

//! # WebAssembly Instruction Translator
//!
//! This module translates WebAssembly instructions into target machine code for the Aurix processor.
//! It implements the `VisitOperator` trait to handle each WebAssembly instruction type.
//!
//! ## Key Concepts
//!
//! - **VB (Valent Block)**: An IR to represent expressions of values in the WebAssembly operand stack
//! - **VB Stack**: Virtualizes the operand stack at compile-time during translation to machine code
//! - **Dead Code**: Code that will never be executed (e.g., after unreachable or return)
//! - **Label Management**: Tracks jump targets for control flow (blocks, loops, branches)
//! - **Runtime Stack**: The actual stack used during execution (different from VB stack)
//!
//! ## Glossary
//!
//! - **AtomicVB**: A VB representing a single value expression (constant, local, global)
//! - **BinaryVB**: A VB representing an operation expression on two operands
//! - **UnaryVB**: A VB representing an operation expression on one operand
//! - **VB Stack**: Compile-time virtualization of the WebAssembly operand stack
//! - **CFG**: Control Flow Graph - represents the structure of jumps and branches
//! - **MemArg**: Memory argument containing offset and alignment for memory operations
//! - **BlockType**: Describes the signature of a WebAssembly block (empty, single type, or function type)
//! - **MapperLocation**: Specifies where a value should be placed (register, stack, memory)
//! - **SVLCX/RSLCX**: Save/Restore Lower Context - Aurix instructions for function calls
//!
//! ## Architecture
//!
//! The translator maintains several pieces of state:
//! - `vb_stack`: Virtual representation of the WebAssembly operand stack using VB expressions
//! - `cfg_label_map`: Maps label indices to instruction positions
//! - `dead_code_flag_stack`: Tracks dead code regions in nested blocks
//!
//! ## Translation Process
//!
//! 1. Each `visit_*` method handles one WebAssembly instruction
//! 2. VBs are created to represent value expressions on the operand stack
//! 3. Helper functions centralize common patterns (dead code checks, branching, etc.)
//! 4. VBs are resolved to actual machine instructions when needed
//!
/// This module contains methods that match each wasm instruction to the expected behavior. 
use alloc::vec;
use alloc::boxed::Box;
use wasmparser::{BlockType, BrTable, Ieee32, Ieee64, MemArg, ValType, VisitOperator};


use crate::isa_model::{self, Const10, DataRegister, ExtendedRegister, RegisterOrSmallConst, ADDRESS_ACCUMULATOR, GLOBAL_BASE, STACK_BASE, STACK_POINTER};
use crate::parse_and_translate::WasmRuntime;
use crate::vb::{Address, AtomicVB, BinaryVB, UnaryVB, VB};
use crate::translator::{BlockLabel, BlockResult, Translator};

use crate::isa_model::{Const4, Const16, AddressRegister, TABLE_BASE, machine_instructions::Instr, Register, ValueSize, Memsize, SignValue, MapperLocation};

/// macro helps implementing the OperatorVisitor trait. The MVP instructions are left to be implemented manually, while the others default to a panic.
macro_rules! _visit_only_mvp {
    // delegate the macro invocation to sub-invocations of this macro to
    // deal with each instruction on a case-by-case basis.
    ($( @$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident ($($ann:tt)*))*) => {
        $(
            _visit_only_mvp!(visit_one @$proposal $op $({ $($arg: $argty),* })? => $visit);
        )*
    };

    // MVP instructions are defined manually, so do nothing.
    (visit_one @mvp $($rest:tt)*) => {};

    // a Non-MVP instruction will cause a panic  
    (visit_one @$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident) => {
        fn $visit(&mut self $($(,$arg: $argty)*)?) -> Self::Output {
            panic!("Operator {:?} is not part of the MVP proposal", stringify!($op));
        }
    }

}

enum BlockStyle {
    Block,
    Loop
}
impl<'a,'b> Translator<'a,'b> {

    // ================================================================================
    // BLOCK AND CONTROL FLOW MANAGEMENT
    // ================================================================================

    /// Helper method to be called while entering a new block.
    /// 
    /// This method handles the complex state management required when entering WebAssembly
    /// blocks, loops, and if statements. It manages:
    /// - Dead code propagation through nested blocks
    /// - Stack offset calculation for proper cleanup
    /// - Label management for forward/backward jumps
    /// - VB stack state preservation
    fn enter_block(&mut self, blockty: BlockType, block_style: BlockStyle) {

        // Propagate dead code state: if we're already in dead code, the entire block is dead
        let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
        self.dead_code_flag_stack.push(dead_code_flag);

        // Skip all processing if we're in dead code - no instructions will be generated
        if dead_code_flag {
            return;
        }
        
        // Determine the result type and size for this block
        // Empty blocks have no result, Type blocks have a single result,
        // FuncType blocks use the function signature's first result
        // TODO: The fact that block types are always a single valtype (or a function with no argument and one output)
        // is specific to 1.0, might not be checked by wasmparser
        let blockty_value_size = match blockty {
            BlockType::Empty => None,
            BlockType::Type(ty) => Some(val_type_size(&ty)),
            BlockType::FuncType(type_index) => {
                let funtype = self.wasm_runtime.types[type_index as usize].unwrap_func();
                 funtype.results().get(0).map(|ty| val_type_size(ty))
            }
        };
        
        // If the block produces a result, we need to resolve all pending VBs now
        // to ensure proper stack ordering when the result expression is eventually produced
        if blockty_value_size.is_some(){
            self.resolve_all();
        }

        // Calculate the runtime stack offset at the start of this block
        // This is needed for proper stack cleanup when the block ends
        let runtime_stack_offset = self.get_current_stack_offset();

        // Convert result size to bytes for stack offset calculations
        let blockty_value_byte_size = blockty_value_size.map(|size| size.as_bytes()).unwrap_or(0);
    
        // Calculate stack offsets for two scenarios:
        // 1. end_state: Stack state when reaching the end of the block naturally
        // 2. label_state: Stack state when branching to this block's label position
        //
        // For regular blocks: both scenarios have the same stack state (original + result)
        // For loops: label_state points to loop start (no result), end_state to loop end (with result)
        let block_result = BlockResult {
            // When block ends naturally: stack = original position + result size
            end_state: (runtime_stack_offset + blockty_value_byte_size as usize, blockty_value_size),
            label_state: match block_style {
                // Regular block: branch target same as block end
                BlockStyle::Block => (runtime_stack_offset + blockty_value_byte_size as usize, blockty_value_size),
                // Loop: branch target is loop start (no accumulated result)
                BlockStyle::Loop => (runtime_stack_offset, None),
            }
        };

        // Track block results for nested blocks using a stack
        self.cfg_block_result_stack.push(block_result);

        // LABEL MANAGEMENT:
        // We maintain a label map (cfg_label_map) that maps label indices to instruction positions.
        // Each nested block gets a label index stored in cfg_label_stack.
        //
        // Two types of jumps:
        // - Backward jumps (loops): target is known immediately (current position)
        // - Forward jumps (blocks): target unknown, use placeholder until block end
        
        // Get next available label index and push to label stack
        self.cfg_label_stack.push(BlockLabel::Block(self.get_current_label_index()));

        // Set label address based on block style
        match block_style {
            // Block: forward jump target unknown, use placeholder
            BlockStyle::Block => self.add_placeholder_label(),
            // Loop: backward jump target known, use current position
            BlockStyle::Loop => self.add_current_position_label(),
        };

        // Preserve VB stack state at block entry for proper cleanup
        // Each VB represents a value expression on the operand stack
        self.vb_stack_ptr_stack.push(self.vb_stack.len());
        
    }

    /// Helper function that resolves the block's result and adjusts the stack pointer.
    /// 
    /// This handles the complex stack management when a block completes:
    /// - If the block has a result, it's moved to the correct stack position
    /// - Stack pointer is adjusted to clean up the block's local stack space
    /// 
    /// Example: Block with i32 result, original stack offset 16, current offset 24
    /// - Result is in some scratch register
    /// - Need to write result to stack\[16\] and set SP to point to stack\[20\]
    fn resolve_block_result(&mut self, target : (usize, Option<ValueSize>) ) {
        let (offset, size) = target;
        
        // If block has a result, resolve it to a register first
        let result_register = self.resolve_to_register(size);
        
        // Current stack position (may have grown during block execution)
        let current_stack_offset = self.get_current_stack_offset();

        match size {
            // No result: just restore stack pointer if it changed
            None if current_stack_offset != offset => {
                self.push_instruction(Instr::LEA { base: STACK_BASE, offset: Const16(-(offset as i16) as u16), dest: STACK_POINTER });
            },
            // Has result: store result on the stack and adjust stack pointer
            Some(_) => match result_register {
                Some(Register::DataRegister(result_register)) => self.push_instruction(Instr::STWPI{ base: STACK_POINTER, offset: Const10(-(offset as i16)+ (current_stack_offset as i16)), src: result_register }),
                Some(Register::ExtendedRegister(result_register)) => self.push_instruction(Instr::STDPI{ base: STACK_POINTER, offset: Const10(-(offset as i16) + (current_stack_offset as i16)), src: result_register }),
                None => panic!("Expected result register")
            },
            _ => ()
        }
    }

    /// resolve the return VB value (if existent) for the wasm function at the end or at a return instruction
    /// result is stored in D[0]/E[0] and then used after function call, then we restore lower context and bitmask goes back to D[0]
    fn resolve_return_value(&mut self) {
        let result_type = self.get_current_function_result_type();
        match result_type {
            Some(ty) => {
                let location  = match ValueSize::from_valtype(ty) {
                    ValueSize::Word =>  MapperLocation::new_data_register(0), 
                    ValueSize::DoubleWord =>  MapperLocation::new_extended_register(0),
                };
                self.resolve_with_target(Some (&location));
            },
            None => ()
        }
    }
    
    /// Helper function for emitting machine code that stores a value in the linear memory.
    /// Both the value expression and the address expression (dynamic offset) are available as VBs on the stack
    fn store_value_in_memory(&mut self, memarg: MemArg, mem_size: Memsize, val_size: ValueSize) {
        if self.check_dead_code() {
            return;
        }
        
        // If any VB expressions depend on memory, we resolve them first to maintain proper ordering
        if self.vb_stack.iter().any(VB::depends_on_memory) {
            let value_vb = self.vb_stack.pop().unwrap();
            let dynamic_offset_vb = self.vb_stack.pop().unwrap();
            self.resolve_all();
            self.vb_stack.push(dynamic_offset_vb);
            self.vb_stack.push(value_vb);
        }

        let value_register = self.resolve_to_register(Some(val_size)).unwrap();

        //TODO: can the locked register be eliminated? I tried and it didn't work
        self.locked_register = Some(value_register.clone());
        let dynamic_offset = self.resolve_with_target(None);
        self.locked_register = None;
        value_register.map_to_location(Some(&MapperLocation::LinearMemory { static_offset: memarg.offset as usize, src_size: mem_size, dynamic_offset: Some(Box::new(dynamic_offset)), align: memarg.align, ext_sign: SignValue::Signed }),  self, &mut vec![]);
    }

    /// Helper to check if we're in dead code and return early if so.
    /// 
    /// Dead code occurs after unconditional jumps (br, return, unreachable, br_table)
    /// and needs to be tracked through nested blocks.
    fn check_dead_code(&self) -> bool {
        *self.dead_code_flag_stack.last().unwrap_or(&false)
    }

    /// Helper to set dead code flag and truncate VB stack (used after unconditional jumps).
    /// 
    /// When we encounter an unconditional jump:
    /// 1. Mark current block as dead code
    /// 2. Truncate VB stack to block entry state (no more value expressions will be consumed)
    fn set_dead_code_and_truncate_vb_stack(&mut self) {
        self.dead_code_flag_stack.last_mut().map(|flag| *flag = true);
        self.vb_stack.truncate(*(self.vb_stack_ptr_stack.last()).unwrap());
    }

    /// Helper to save and restore a VB for conditional branches.
    /// 
    /// For conditional branches that may have a result:
    /// - Save the result VB expression before branching
    /// - Restore it after the branch for the fall-through case
    fn save_vb_for_branch(&mut self, has_result: bool) -> Option<VB> {
        match has_result {
            true => self.vb_stack.last().cloned(),
            false => None,
        }
    }

    /// Helper to restore VB after conditional branch.
    /// 
    /// Restores the saved VB expression for the fall-through path of a conditional branch.
    fn restore_vb_after_branch(&mut self, vb: Option<VB>) {
        vb.map(|vb| self.vb_stack.push(vb));
    }

    /// Helper to calculate branch target index from relative depth.
    /// 
    /// WebAssembly branches use relative depths:
    /// - 0 = current block
    /// - 1 = parent block
    /// - etc.
    /// 
    /// This converts to absolute index in our block result stack.
    fn calculate_branch_target_index(&self, relative_depth: u32) -> i32 {
        self.cfg_block_result_stack.len() as i32 - (1 + relative_depth as i32)
    }

    /// Helper to handle function return case in branches
    fn handle_function_return_branch(&mut self) {
        let return_type = self.get_current_function_result_type();
        let last_vb = match return_type {
            Some(..) => self.vb_stack.last().cloned(),
            None => None,
        };
        self.resolve_return_value();
        self.push_instruction(Instr::RET);
        self.restore_vb_after_branch(last_vb);
    }

    /// Helper to handle block branch case
    fn handle_block_branch(&mut self, index: usize) {
        let label_state = self.cfg_block_result_stack.get(index).map(|block_result| block_result.label_state).unwrap();
        let last_vb = self.save_vb_for_branch(label_state.1.is_some());
        
        self.resolve_block_result(label_state);
        self.restore_vb_after_branch(last_vb);

        let target = match self.cfg_label_stack[index] {
            BlockLabel::Block(index) => index,
            BlockLabel::If { else_label: _, end_label } => end_label,
        };

        self.generate_jump_instruction(target);
    }

    /// Helper to generate appropriate jump instruction based on label state
    fn generate_jump_instruction(&mut self, target: usize) {
        match self.cfg_label_map[target] {
            None => self.push_instruction(Instr::J { target }),
            Some(..) => self.push_instruction(Instr::LOOPU { target }), //TODO: is LOOPU really useful?
        }
    }

    /// Remove parameters from the VB stack as well as the memory stack after a function call
    fn cleanup_function_call_parameters(&mut self, function_type: &wasmparser::FuncType) {
        let params_count = function_type.params().len();
        self.vb_stack.truncate(self.vb_stack.len() - params_count);

        let params_size = Self::calculate_params_size(function_type);
        
        if params_size > 0 {
            self.push_instruction(Instr::LEA{base: STACK_POINTER, offset: Const16(params_size), dest: STACK_POINTER});
        }
    }

    /// Push function result onto the stack and add it to the VB stack
    fn handle_function_call_result(&mut self, function_type: &wasmparser::FuncType) {
        function_type.results().get(0).map(|ty|{
            let runtime_stack_offset = self.get_runtime_stack_offset_from_last_vb();
            self.vb_stack.push(VB::AtomicVB(AtomicVB::Resolved{size: val_type_size(ty), offset: runtime_stack_offset + val_type_size(ty).as_bytes() as usize})); 
        
            match val_type_size(ty){
                ValueSize::Word => self.push_instruction(Instr::STWPI { base: STACK_POINTER, offset: Const10(-4), src: DataRegister(0) }),
                ValueSize::DoubleWord => self.push_instruction(Instr::STDPI { base: STACK_POINTER, offset: Const10(-8), src: ExtendedRegister(0) }),
            }
        });
    }

    /// Helper to load a 32-bit pointer into an address register
    fn load_pointer_to_address_register(&mut self, ptr: u32, dest: AddressRegister) {
        let ptr_upper = ((ptr + 0x8000) >> 16) as u16;
        let ptr_lower = ptr as u16;
        self.push_instruction(Instr::MOVHA { src: Const16::new(ptr_upper), dest });
        self.push_instruction(Instr::LEA { base: dest, offset: Const16::new(ptr_lower), dest });
    }

    /// Helper to calculate total size of function parameters in bytes
    fn calculate_params_size(function_type: &wasmparser::FuncType) -> u16 {
        function_type.params().iter().map(|ty| val_type_size(ty).as_bytes() as u16).sum()
    }

    /// Helper to update label addresses for forward jumps
    fn update_label_addresses(&mut self, block_label: BlockLabel) {
        let label_indices = match block_label {
            // index is the index of the label in cfg_label_map
            BlockLabel::Block(index) => vec![index],
            //encountered in case of an if construct without an else
            BlockLabel::If { else_label, end_label } => vec![else_label, end_label],
        };

        label_indices.iter().for_each(|&label_index| {
            self.update_label_to_current_position(label_index);
        });
    }

    /// Helper to handle function pointer loading for indirect calls
    fn load_function_pointer(&mut self, function_label: u32) {
        let fun_ptr_lower = function_label as u16;
        let fun_ptr_upper = (function_label.wrapping_add(0x8000) >> 16) as u16;
        self.push_instruction(Instr::MOVHA { src: Const16(fun_ptr_upper), dest: ADDRESS_ACCUMULATOR });
        self.push_instruction(Instr::LEA { base: ADDRESS_ACCUMULATOR, offset: Const16(fun_ptr_lower), dest: ADDRESS_ACCUMULATOR });
        self.push_instruction(Instr::CALLI { target: ADDRESS_ACCUMULATOR });
    }

    // ================================================================================
    // STACK AND MEMORY MANAGEMENT HELPERS
    // ================================================================================

    /// Helper to get current runtime stack offset
    fn get_current_stack_offset(&self) -> usize {
        let mut stack_offset = 0;
        for vb in self.vb_stack.iter().rev() {
            let offset = vb.get_runtime_stack_offset();
            if offset.is_some() {
                stack_offset = offset.unwrap();
                break;
            }
        }
        stack_offset
    }

    /// Helper to get runtime stack offset from last VB on stack
    fn get_runtime_stack_offset_from_last_vb(&self) -> usize {
        match self.vb_stack.last() {
            Some(VB::AtomicVB(AtomicVB::Resolved { offset, .. })) => *offset,
            _ => 0,
        }
    }

    // ================================================================================
    // FUNCTION TYPE AND METADATA HELPERS
    // ================================================================================

    /// Helper to get current function's result type
    fn get_current_function_result_type(&self) -> Option<&ValType> {
        self.wasm_runtime.types[self.function_type_index as usize].unwrap_func().results().get(0)
    }

    /// Helper to get function type by function index
    fn get_function_type(&self, function_index: u32) -> wasmparser::FuncType {
        self.wasm_runtime.types[self.global_translator.function_type_map[function_index as usize] as usize].unwrap_func().clone()
    }

    /// Helper to get function type by type index  
    fn get_function_type_by_type_index(&self, type_index: u32) -> wasmparser::FuncType {
        self.wasm_runtime.types[type_index as usize].unwrap_func().clone()
    }

    // ================================================================================
    // VB STACK MANIPULATION HELPERS
    // ================================================================================

    /// Helper to resolve a value expression to a scratch data register
    fn resolve_to_data_register(&mut self) -> DataRegister {
        self.resolve_with_target(None).map_to_data_register(None, self, &mut vec![], &vec![])
    }

    /// Helper to resolve a value expression to a scratch extended register
    fn resolve_to_extended_register(&mut self) -> ExtendedRegister {
        self.resolve_with_target(None).map_to_extended_register(None, self, &mut vec![], &vec![])
    }

    /// Helper to resolve a value expression to an appropriate register based on value size
    fn resolve_to_register(&mut self, val_size: Option<ValueSize>) -> Option<Register> {
        val_size.map(|size| match size {
            ValueSize::Word => Register::DataRegister(self.resolve_to_data_register()),
            ValueSize::DoubleWord => Register::ExtendedRegister(self.resolve_to_extended_register()),
        })
    }

    /// Helper to pop VB expression from stack
    fn pop_vb(&mut self) -> VB {
        self.vb_stack.pop().unwrap()
    }

    // ================================================================================
    // FUNCTION CALL HELPERS
    // ================================================================================

    /// Helper to setup function call context (resolve all, save context)
    fn setup_function_call(&mut self) {
        self.resolve_all();
        self.push_instruction(Instr::SVLCX);
    }

    /// Helper to cleanup function call context (restore context)
    fn cleanup_function_call(&mut self) {
        self.push_instruction(Instr::RSLCX);
    }

    /// Helper to generate forward jump if condition is false (jumps to next label to be created)
    fn generate_forward_jump_if_false(&mut self, condition_reg: DataRegister) {
        self.push_instruction(Instr::JEQ { 
            target: self.get_current_label_index(), 
            lhs: condition_reg, 
            rhs: RegisterOrSmallConst::new_const(0) 
        });
    }

    /// Helper to handle local variable dependency resolution
    fn handle_local_dependency(&mut self, local_index: u32) {
        // Check if any existing VB expressions depend on this local variable
        let vb: VB = self.vb_stack.pop().unwrap();
        if self.vb_stack.iter().any(|vb| vb.depends_on_local(local_index)) {
            self.resolve_all();
        }
        self.vb_stack.push(vb);
    }

    /// Add a label to the cfg_label_map
    fn add_label(&mut self, label: Option<usize>) {
        self.cfg_label_map.push(label);
    }
    
    /// Add a label for the current instruction position
    fn add_current_position_label(&mut self) {
        self.add_label(Some(self.wasm_runtime.instructions_count));
    }
    
    /// Add a placeholder label (None) for forward jumps
    fn add_placeholder_label(&mut self) {
        self.add_label(None);
    }
    
    /// Get the current length of the cfg_label_map (used for generating new label indices)
    fn get_current_label_index(&self) -> usize {
        self.cfg_label_map.len()
    }
    
    /// Update a label at the given index with the current instruction position
    fn update_label_to_current_position(&mut self, index: usize) {
        if self.cfg_label_map[index].is_some() {
            return; // Label already set, no need to update. This is a backward jump.
        }
        self.cfg_label_map[index] = Some(self.wasm_runtime.instructions_count);
    }

    /// Helper to generate appropriate call instruction based on function index:
    /// CALL if the displacement is small enough, otherwise CALLI with the function pointer loaded into an address register.
    fn generate_call_instruction(&mut self, function_index: u32) {
        let function_label = self.wasm_runtime.function_labels.get(function_index as usize).copied();
        match function_label {
            Some(function_label) => {
                let current_ptr = (self.wasm_runtime.instructions.as_ptr() as u32) + ((self.wasm_runtime.instructions_count as u32) << 2); // base of instructions + offset
                let disp = function_label.wrapping_sub(current_ptr);
                assert_eq!(disp & 1, 0);
                let disp = disp >> 1; // Call actually goes to PC + 2*disp
                
                // Call uses a 24-bit displacement that is sign-extended to 32 bits, we need to check that this doesn't change the value
                // If the displacement is too large, or if the sign extension is wrong, we need to load the function pointer into an address register
                if disp.wrapping_add(1 << 23) & 0xff000000 != 0 {
                    self.load_function_pointer(function_label);
                } else {
                    self.push_instruction(Instr::CALL { target: function_index });
                }
            },
            None => self.push_instruction(Instr::CALL { target: function_index })
        }
    }

}
use alloc::vec::Vec;
#[allow(unused_variables)]
impl <'a,'b> VisitOperator <'a> for Translator<'a,'b>{
    
    type Output = ();
    
    wasmparser::for_each_operator!(_visit_only_mvp);

    // ================================================================================
    // CONTROL FLOW INSTRUCTIONS
    // ================================================================================

    fn visit_unreachable(&mut self) {
        if self.check_dead_code() {
            return;
        }

        self.push_instruction(Instr::Trap);
        self.set_dead_code_and_truncate_vb_stack();
    }

    fn visit_nop(&mut self) {
        // Do nothing
    }

    fn visit_block(&mut self,blockty:BlockType) {
        self.enter_block(blockty, BlockStyle::Block);
    }

    fn visit_loop(&mut self,blockty:BlockType) {
        self.enter_block(blockty, BlockStyle::Loop);
    }

    fn visit_if(&mut self,blockty:BlockType) {

        //dead code handling
        if self.check_dead_code() {
            // in case the if block is not inside dead code, this is handled inside enter_block
            self.dead_code_flag_stack.push(true);
            return;
        }

        // condition VB is popped out so that it does NOT get resolved or considered 
        // as part of the state of the VB stack prior to the entry.
        let condition_vb = self.vb_stack.pop().unwrap();
        self.enter_block(blockty, BlockStyle::Block);
        self.vb_stack.push(condition_vb);

        // resolve condition
        let condition_register = self.resolve_to_data_register();

        // creating label for beginning of else block
        // note that a label for the end of the if block (after the else part) has been already created within enter_block
        let else_label = self.get_current_label_index();

        //generate jump instruction to the beginning of else block if condition is equal to 0 (i.e it is false)
        self.push_instruction(Instr::JEQ {target: else_label, lhs: condition_register, rhs: RegisterOrSmallConst::new_const(0)});

        // replace the label stack last item (inserted within enter_block) to include both the else label (the newly created one) and the end label
        // (created within enter_block)
        self.cfg_label_stack.last_mut().map(|label|{
            let end_label = match label {
                BlockLabel::Block(index) => index,
                _ => panic!("Expected block label")
            };
            *label = BlockLabel::If { else_label, end_label: *end_label };

        } );

        // placeholder for the address of the beginning of the else block
        self.add_placeholder_label();

    }

    
    fn visit_else(&mut self){ 
        //an else instruction is encountered.
        //2 dead code flags exist

        // 1) is the code right before the else instruction (i.e.) inside the if block dead code ?
        let inside_dead_code_flag = self.dead_code_flag_stack.pop().unwrap_or(false);

        // 2) is the whole if block (inside) dead code ?
        let outside_dead_flag_code = *self.dead_code_flag_stack.last().unwrap_or(&false);
        // initial state of the dead code flag inside the block matches
        // initially whether the whole block is inside dead code
        self.dead_code_flag_stack.push(outside_dead_flag_code);

        if outside_dead_flag_code {
            return;
        }

        // pop labels from the label stack pushed at the beginning of the if block
        let (else_label, end_label) = match self.cfg_label_stack.pop().unwrap() {
            BlockLabel::If { else_label, end_label } => (else_label, end_label),
            _ => panic!("Expected if label")
        };

        // resolve the if block unless it was dead code (if it was dead code, you met a br and it was already resolved)
        // then add the jump instruction to the end of the if block (same thing, if it was dead code you already did the jump)
        if !inside_dead_code_flag {
            self.resolve_block_result(self.cfg_block_result_stack.last().unwrap().end_state);
            self.push_instruction(Instr::J {target: end_label});
        }

        // else label address is the current position in the instructions, directly after the jump
        self.update_label_to_current_position(else_label);

        // start of the else branch: replace the if label (that was already popped) with a block label
        // else label is irrelevant at this point and the rest can be treated like a block instruction.
        self.cfg_label_stack.push(BlockLabel::Block(end_label));
    }

    // end instruction matches the end of if/else, block, and loop constructs as well as the end of a function
    // the behavior is generalized to cover all of the possibilities based on the information collected
    // while passing through the wasm instructions
    fn visit_end(&mut self){
        
        // dead code flag of whether code before the end instruction inside the innermost construct is 
        // dead code.
        let inside_dead_code_flag = self.dead_code_flag_stack.pop().unwrap_or(false);

        // dead code flag indicating whether the whole construct was inside a dead code zone
        let outside_dead_flag_code = *self.dead_code_flag_stack.last().unwrap_or(&false);
        if outside_dead_flag_code {
            return;
        }

        // pop the block result from the stack
        let end_state = self.cfg_block_result_stack.pop().map(|block_result| block_result.end_state);

        if !inside_dead_code_flag {
            // return block result or function return value.
            // end of a function is distinguished through an empty block state stack -> end_state is None
            match end_state{
                Some(end_state) => self.resolve_block_result(end_state),
                None => {
                    self.resolve_return_value();
                    self.push_instruction(Instr::RET);
                }
            }
        }

        // update label addresses (previously unknown) for forward jump labels to current instruction count
        // and pop value from stack
        self.cfg_label_stack.pop().map(|block_label| self.update_label_addresses(block_label));

        // current construct/block is ending. Pop value from stack
        self.vb_stack_ptr_stack.pop();

        // result of a block is returned on the runtime stack
        // and on a virtual level is on top of the operand stack now
        // a VB is pushed on top of the VB stack to update it with the location of new value.
        end_state.map(|(offset,size)| {
            size.map(|size| {
                self.add_atomic_vb(AtomicVB::Resolved{size, offset});
            });
        });

    }

    fn visit_br(&mut self,relative_depth:u32){
        if self.check_dead_code() {
            return;
        }

        let index = self.calculate_branch_target_index(relative_depth);
        if index < 0 {
            self.resolve_return_value();
            self.push_instruction(Instr::RET);
        } else { 
            let label_state = self.cfg_block_result_stack.get(index as usize).map(|block_result| block_result.label_state).unwrap();
            self.resolve_block_result(label_state);

            let target = match self.cfg_label_stack[index as usize] {
                BlockLabel::Block(index) => index,
                BlockLabel::If { else_label: _, end_label } => end_label,
            };
    
            self.generate_jump_instruction(target);
        }

        self.set_dead_code_and_truncate_vb_stack();
    }

    fn visit_br_if(&mut self,relative_depth:u32){

        // ignore if dead code
        if self.check_dead_code() {
            return;
        }
        
        // resolve condition to temporary register
        let condition_register = self.resolve_to_data_register();

        // retrieve index of the target construct in the stack
        let index = self.calculate_branch_target_index(relative_depth);

        // if condition is false. jump to after the code that handles the branching.
        // Label to be created when the corresponding position is reached
        self.generate_forward_jump_if_false(condition_register);
        
        if index < 0 {
            self.handle_function_return_branch();
        } else {
            self.handle_block_branch(index as usize);
        }
        
        self.add_current_position_label();
    }

    fn visit_br_table(&mut self, targets:BrTable<'a>) {
        if self.check_dead_code() {
            return;
        }
        // TODO: replace name offset by index (it's for the BrTable)
        // resolving all VBs prior to the last one (dynamic index) and pushing them to the stack
        let offset_vb = self.vb_stack.pop().unwrap();
        self.resolve_all();
        self.vb_stack.push(offset_vb);

        // resolving the dynamic offset to a register 
        let offset_register = self.resolve_to_data_register();

        // for every index of the BrTable, emit a JNE instruction that will compare your target index
        // TODO: this is not very efficient, but simpler. Could be improved by directly fetching the relative break index in a table in memory
        // But you would need to allocate memory for that.
        // TODO: given that each block that would could break from has the same type, (same as the default target), we know what needs to be resolved
        // this would allow us to not do the resolving inside the loop, but before. It would also reduce code size.
        targets.targets().enumerate().map(|(offset,target)| (Some(offset), target.unwrap()) ).chain(vec![(None,targets.default())].into_iter()).for_each(|(offset,target)| { 
            offset.map(|offset| self.push_instruction(Instr::JNE { target: self.get_current_label_index(), lhs: offset_register, rhs: Const4(offset as u8) }));
            let relative_index = target;
            let index = self.calculate_branch_target_index(relative_index);
            if index < 0 {
                self.handle_function_return_branch();
            } else {
                self.handle_block_branch(index as usize);
            };
            self.add_current_position_label();
        });

        self.set_dead_code_and_truncate_vb_stack();

    }

    fn visit_return(&mut self)  {
        if self.check_dead_code() {
            return;
        }

        self.resolve_return_value();

        self.push_instruction(Instr::RET);
        self.set_dead_code_and_truncate_vb_stack();
    }

    fn visit_call(&mut self,function_index:u32)  {
        if self.check_dead_code() {
            return;
        }

        let function_type = self.get_function_type(function_index);
        self.setup_function_call();

        self.generate_call_instruction(function_index);

        self.cleanup_function_call_parameters(&function_type);
        self.handle_function_call_result(&function_type);

        self.cleanup_function_call();
    }

    /*
        Here we generate the machine code for an indirect call, which goes as follows:
        1) resolve all VBs on the stack (TODO: see if this can be refined to resolve only when necessary.
            Optimizing resolving should be a task on its own).
            The subroutine panics if any of the checks fails  
        2) Call the indirect call safety subroutine:
            - The subroutine checks whether the dynamic index is valid (smaller than the table size)
            then checks the type of the dynamically referenced function to the statically provided type.
            - The subroutine is provided as a runtime function that is to be compiled according to the C ABI.
              It should be called using the following parameters:
                - indirect call dynamic index (element index of the function reference in the table) in D[4]
                - type index of the statically provided type in D[5]
                - size of the table in D[6]
                - pointer to the types array in A[4]
                - pointer to the table type indices array (maps function references to their respective type indices) in A[5]
            - The subroutine start address (statically known) is loaded to the address accumulator  
        3) Load the start address of the callee into the address accumulator: The address is located in the table in the
            corresponding dynamic index (which is guaranteed to be safe after the check)
        4) Perform the call (indirect call using CALLI)
            - parameters are expected to be on the stack thanks to the already performed resolve_all
        5) place result on the stack if exists (similarly to direct calls)
     */
    fn visit_call_indirect(&mut self,type_index:u32,_table_index:u32) {

        if self.check_dead_code() {
            return;
        }

        let function_index_vb = self.vb_stack.pop().unwrap();

        self.resolve_all();

        self.vb_stack.push(function_index_vb);

        let table_offset = self.resolve_to_data_register();

        self.push_instruction(Instr::SVLCX);
       
        if table_offset != DataRegister(4) {
            self.push_instruction(Instr::MOV {src: isa_model::RegisterOrLargeConst::DataRegister(table_offset), dest: Register::DataRegister(DataRegister(4))});
        }
        self.push_instruction(Instr::MOVU { src: Const16::new(type_index as u16), dest: DataRegister(5) });

        #[cfg(feature="address-masking")]
        self.push_instruction(Instr::MOVU { src: Const16::new(self.global_translator.table_size as u16), dest: DataRegister(6)});

        let types_ptr = self.wasm_runtime.types.as_ptr() as u32;
        self.load_pointer_to_address_register(types_ptr, AddressRegister(4));

        let table_type_indices_ptr = self.wasm_runtime.table_type_indices.as_ptr() as u32;
        self.load_pointer_to_address_register(table_type_indices_ptr, AddressRegister(5));

        // Call the subroutine that checks the dynamic index and compares the types
        let call_ptr = WasmRuntime::compare_subtypes as u32;
        self.load_pointer_to_address_register(call_ptr, AddressRegister(2));
        self.push_instruction(Instr::CALLI{target: AddressRegister(2)}); // TODO: could use something similar to generate_call_instruction here
        self.push_instruction(Instr::RSLCX);
        
        // Put table element address into AddressRegister(2)
        self.push_instruction(Instr::ADDSCA { lhs: TABLE_BASE, rhs: table_offset, dest: AddressRegister(2), shift: Const4::new(2) });
        // Load the function pointer from the table into AddressRegister(2)
        self.push_instruction(Instr::LDA { base: AddressRegister(2), offset: Const16::new(0), dest: AddressRegister(2) });

        let function_type = self.get_function_type_by_type_index(type_index); 

        self.push_instruction(Instr::SVLCX);
        self.push_instruction(Instr::CALLI{target: AddressRegister(2)});

        self.cleanup_function_call_parameters(&function_type);
        self.handle_function_call_result(&function_type);

        self.push_instruction(Instr::RSLCX);

    }

    // ================================================================================
    // STACK MANIPULATION INSTRUCTIONS
    // ================================================================================

    /// Pop the VB stack and if the value is on the runtime stack, remove it from there as well
    fn visit_drop(&mut self) {
        if self.check_dead_code() {
            return;
        }

        let vb = self.pop_vb();
        match vb{
            VB::AtomicVB(AtomicVB::Resolved{size, ..}) => {
                let byte_offset = size.as_bytes() as u16;
                self.push_instruction(Instr::LEA{base: STACK_POINTER, offset: Const16(byte_offset), dest: STACK_POINTER});
            },
            _ => ()
        };

    }

    fn visit_select(&mut self) {
        if self.check_dead_code() {
            return;
        }
       let selector = self.vb_stack.pop().unwrap();
       let rhs = self.vb_stack.pop().unwrap();
       let lhs = self.vb_stack.pop().unwrap();
       let size = lhs.val_size(&self.locals_map, &self.global_translator.globals_map);
       self.vb_stack.push(VB::Select {selector: Box::new(selector), lhs: Box::new(lhs), rhs: Box::new(rhs), size});
    }

    // ================================================================================
    // VARIABLE ACCESS INSTRUCTIONS
    // ================================================================================

    fn visit_local_get(&mut self,local_index:u32)  {
        self.add_atomic_vb(AtomicVB::Local {index: local_index})
    }

    fn visit_local_set(&mut self,local_index:u32) {
        if self.check_dead_code() {
            return;
        }
        self.handle_local_dependency(local_index);
        self.resolve_with_target(Some(&self.locals_map[local_index as usize].clone()));
    }

    fn visit_local_tee(&mut self,local_index:u32) {
        if self.check_dead_code() {
            return;
        }
        self.handle_local_dependency(local_index);
        self.resolve_with_target(Some(&self.locals_map[local_index as usize].clone()));
        self.add_atomic_vb(AtomicVB::Local {index: local_index});
    }

    fn visit_global_get(&mut self,global_index:u32) {
        self.add_atomic_vb(AtomicVB::Global {index: global_index})
    }

    fn visit_global_set(&mut self,global_index:u32) { // TODO: there should be a handle global dependency here
        if self.check_dead_code() {
            return;
        }
        let (offset, size) =self.global_translator.globals_map[global_index as usize];
        self.resolve_with_target(Some(&MapperLocation::Global { offset, size }));
    }

    // ================================================================================
    // MEMORY ACCESS INSTRUCTIONS
    // ================================================================================

    fn visit_i32_load(&mut self,memarg:MemArg) {
        self.add_unary_vb(UnaryVB::I32Load { offset: memarg.offset as Address, align: memarg.align });
    }

    fn visit_i64_load(&mut self,memarg:MemArg) {
        self.add_unary_vb(UnaryVB::I64Load { offset: memarg.offset as Address, align: memarg.align });
    }

    fn visit_f32_load(&mut self,memarg:MemArg) {
        self.add_unary_vb(UnaryVB::F32Load { offset: memarg.offset as Address, align: memarg.align });
    }

    fn visit_f64_load(&mut self,memarg:MemArg) {
        self.add_unary_vb(UnaryVB::F64Load { offset: memarg.offset as Address, align: memarg.align });
    }

    fn visit_i32_load8_s(&mut self,memarg:MemArg) {
        self.add_unary_vb(UnaryVB::I32Load8s { offset: memarg.offset as Address, align: memarg.align })
    }

    fn visit_i32_load8_u(&mut self,memarg:MemArg) {
        self.add_unary_vb(UnaryVB::I32Load8u { offset: memarg.offset as Address, align: memarg.align })
    }

    fn visit_i32_load16_s(&mut self,memarg:MemArg) {
        self.add_unary_vb(UnaryVB::I32Load16s { offset: memarg.offset as Address, align: memarg.align })
    }

    fn visit_i32_load16_u(&mut self,memarg:MemArg) {
        self.add_unary_vb(UnaryVB::I32Load16u { offset: memarg.offset as Address, align: memarg.align })
    }

    fn visit_i64_load8_s(&mut self,memarg:MemArg) {
        self.add_unary_vb(UnaryVB::I64Load8s { offset: memarg.offset as Address, align: memarg.align })
    }

    fn visit_i64_load8_u(&mut self,memarg:MemArg) {
        self.add_unary_vb(UnaryVB::I64Load8u { offset: memarg.offset as Address, align: memarg.align })
    }

    fn visit_i64_load16_s(&mut self,memarg:MemArg) {
        self.add_unary_vb(UnaryVB::I64Load16s { offset: memarg.offset as Address, align: memarg.align })
    }

    fn visit_i64_load16_u(&mut self,memarg:MemArg) {
        self.add_unary_vb(UnaryVB::I64Load16u { offset: memarg.offset as Address, align: memarg.align })
    }

    fn visit_i64_load32_s(&mut self,memarg:MemArg) {
        self.add_unary_vb(UnaryVB::I64Load32s { offset: memarg.offset as Address, align: memarg.align })
    }

    fn visit_i64_load32_u(&mut self,memarg:MemArg) {
        self.add_unary_vb(UnaryVB::I64Load32u { offset: memarg.offset as Address, align: memarg.align })
    }

    fn visit_i32_store(&mut self,memarg:MemArg) {
       self.store_value_in_memory(memarg, Memsize::Word, ValueSize::Word);
    }

    fn visit_i64_store(&mut self,memarg:MemArg) {
        self.store_value_in_memory(memarg, Memsize::DoubleWord, ValueSize::DoubleWord);
    }

    fn visit_f32_store(&mut self,memarg:MemArg) {
        self.store_value_in_memory(memarg, Memsize::Word, ValueSize::Word);
    }

    fn visit_f64_store(&mut self,memarg:MemArg) {
        self.store_value_in_memory(memarg, Memsize::DoubleWord, ValueSize::DoubleWord);
    }

    fn visit_i32_store8(&mut self,memarg:MemArg) {
        self.store_value_in_memory(memarg, Memsize::Byte, ValueSize::Word);
    }

    fn visit_i32_store16(&mut self,memarg:MemArg) {
        self.store_value_in_memory(memarg, Memsize::HalfWord, ValueSize::Word);
    }

    fn visit_i64_store8(&mut self,memarg:MemArg) {
        self.store_value_in_memory(memarg, Memsize::Byte, ValueSize::DoubleWord);
    }

    fn visit_i64_store16(&mut self,memarg:MemArg) {
        self.store_value_in_memory(memarg, Memsize::HalfWord, ValueSize::DoubleWord);
    }

    fn visit_i64_store32(&mut self,memarg:MemArg) {
        self.store_value_in_memory(memarg, Memsize::Word, ValueSize::DoubleWord);
    }

    fn visit_memory_size(&mut self,_mem:u32) {
        if self.check_dead_code() {
            return;
        }
        
        self.add_atomic_vb(AtomicVB::MemorySize);
    }

    /// This uses an external call to grow the memory.
    /// The function simply prepares the registers for the external call using the C ABI.
    /// Then the old size of the memory is pushed on the stack.
    fn visit_memory_grow(&mut self,_mem:u32) {
        if self.check_dead_code() {
            return;
        }

        let offset_vb = self.vb_stack.pop().unwrap();
        self.resolve_all();
        self.vb_stack.push(offset_vb);
        self.push_instruction(Instr::SVLCX);
        let grow_offset_register = self.resolve_with_target(Some(&MapperLocation::new_data_register(4)));

        MapperLocation::Immediate(isa_model::Immediate::Word(self.global_translator.memory_size_limit as u32))
           .map_to_data_register(Some(DataRegister::new(5)), self, &mut vec![], &vec![]);
        
        // Memory size is the first global variable
        self.push_instruction(Instr::MOVAA { src: GLOBAL_BASE , dest: AddressRegister::new(4) });

        // Load the pointer to the grow memory function into AddressRegister(2)
        let call_ptr = WasmRuntime::grow_memory as u32;
        self.load_pointer_to_address_register(call_ptr, AddressRegister(2));
        self.push_instruction(Instr::CALLI{target: AddressRegister(2)});

        // The result (the old size of the memory) will be returned in DataRegister(2), we push it on the stack
        self.push_instruction(Instr::STWPI { src: DataRegister::new(2), base: STACK_POINTER , offset: Const10(-4) });
        let offset = self.get_runtime_stack_offset_from_last_vb() + 4;
        self.add_atomic_vb(AtomicVB::Resolved{size: ValueSize::Word, offset});

        self.push_instruction(Instr::RSLCX);
    }

    fn visit_i32_const(&mut self,value:i32) {
        self.add_atomic_vb(AtomicVB::I32Const { imm: value });
    }

    fn visit_i64_const(&mut self,value:i64) {
        self.add_atomic_vb(AtomicVB::I64Const { imm: value });
    }

    fn visit_f32_const(&mut self,value:Ieee32) {
        self.add_atomic_vb(AtomicVB::F32Const { imm: value.bits() });
    }

    fn visit_f64_const(&mut self,value:Ieee64) {
        self.add_atomic_vb(AtomicVB::F64Const { imm: value.bits() });
    }

    fn visit_i32_eqz(&mut self) {
        self.add_unary_vb(UnaryVB::I32EqZ);
    }

    fn visit_i32_eq(&mut self) {
        self.add_binary_vb(BinaryVB::I32Eq);
    }

    fn visit_i32_ne(&mut self) {
        self.add_binary_vb(BinaryVB::I32Ne);
    }

    fn visit_i32_lt_s(&mut self) {
        self.add_binary_vb(BinaryVB::I32LtS);
    }

    fn visit_i32_lt_u(&mut self) {
        self.add_binary_vb(BinaryVB::I32LtU);
    }

    fn visit_i32_gt_s(&mut self) {
        self.add_binary_vb(BinaryVB::I32GtS);
    }

    fn visit_i32_gt_u(&mut self) {
        self.add_binary_vb(BinaryVB::I32GtU);
    }

    fn visit_i32_le_s(&mut self) {
        self.add_binary_vb(BinaryVB::I32LeS);
    }

    fn visit_i32_le_u(&mut self) {
        self.add_binary_vb(BinaryVB::I32LeU);
    }

    fn visit_i32_ge_s(&mut self) {
        self.add_binary_vb(BinaryVB::I32GeS);
    }

    fn visit_i32_ge_u(&mut self) {
        self.add_binary_vb(BinaryVB::I32GeU);
    } 
    
    fn visit_i64_eqz(&mut self) {
        self.add_unary_vb(UnaryVB::I64EqZ);
    }

    fn visit_i64_eq(&mut self) {
        self.add_binary_vb(BinaryVB::I64Eq);
    }

    fn visit_i64_ne(&mut self) {
        self.add_binary_vb(BinaryVB::I64Ne);
    }

    fn visit_i64_lt_s(&mut self) {
        self.add_binary_vb(BinaryVB::I64LtS);
    }

    fn visit_i64_lt_u(&mut self) {
        self.add_binary_vb(BinaryVB::I64LtU);
    }

    fn visit_i64_gt_s(&mut self) {
        self.add_binary_vb(BinaryVB::I64GtS);
    }

    fn visit_i64_gt_u(&mut self) {
        self.add_binary_vb(BinaryVB::I64GtU);
    }

    fn visit_i64_le_s(&mut self) {
        self.add_binary_vb(BinaryVB::I64LeS);
    }

    fn visit_i64_le_u(&mut self) {
        self.add_binary_vb(BinaryVB::I64LeU);
    }

    fn visit_i64_ge_s(&mut self) {
        self.add_binary_vb(BinaryVB::I64GeS);
    }

    fn visit_i64_ge_u(&mut self) {
        self.add_binary_vb(BinaryVB::I64GeU);
    }

    fn visit_f32_eq(&mut self) {
        self.add_binary_vb(BinaryVB::F32Eq);
    }

    fn visit_f32_ne(&mut self) {
        self.add_binary_vb(BinaryVB::F32Ne);
    }

    fn visit_f32_lt(&mut self) {
        self.add_binary_vb(BinaryVB::F32Lt);
    }

    fn visit_f32_gt(&mut self) {
        self.add_binary_vb(BinaryVB::F32Gt);
    }

    fn visit_f32_le(&mut self) {
        self.add_binary_vb(BinaryVB::F32Le);
    }

    fn visit_f32_ge(&mut self) {
        self.add_binary_vb(BinaryVB::F32Ge);
    }

    fn visit_f64_eq(&mut self) {
        self.add_binary_vb(BinaryVB::F64Eq);
    }

    fn visit_f64_ne(&mut self) {
        self.add_binary_vb(BinaryVB::F64Ne);
    }

    fn visit_f64_lt(&mut self) {
        self.add_binary_vb(BinaryVB::F64Lt);
    }

    fn visit_f64_gt(&mut self) {
        self.add_binary_vb(BinaryVB::F64Gt);
    }

    fn visit_f64_le(&mut self) {
        self.add_binary_vb(BinaryVB::F64Le);
    }

    fn visit_f64_ge(&mut self) {
        self.add_binary_vb(BinaryVB::F64Ge);
    }

    fn visit_i32_clz(&mut self) {
        self.add_unary_vb(UnaryVB::I32Clz);
    }

    fn visit_i32_ctz(&mut self) {
        self.add_unary_vb(UnaryVB::I32Ctz);
    }

    fn visit_i32_popcnt(&mut self) {
        self.add_unary_vb(UnaryVB::I32PopCnt);
    }

    fn visit_i32_add(&mut self) {
        self.add_binary_vb(BinaryVB::I32Add);
    }

    fn visit_i32_sub(&mut self) {
        self.add_binary_vb(BinaryVB::I32Sub);
    }

    fn visit_i32_mul(&mut self) {
        self.add_binary_vb(BinaryVB::I32Mul);
    }

    fn visit_i32_div_s(&mut self) {
        self.add_binary_vb(BinaryVB::I32DivS);
    }

    fn visit_i32_div_u(&mut self) {
        self.add_binary_vb(BinaryVB::I32DivU);
    }

    fn visit_i32_rem_s(&mut self) {
        self.add_binary_vb(BinaryVB::I32RemS);
    }

    fn visit_i32_rem_u(&mut self) {
        self.add_binary_vb(BinaryVB::I32RemU);
    }

    fn visit_i32_and(&mut self) {
        self.add_binary_vb(BinaryVB::I32And);
    }

    fn visit_i32_or(&mut self) {
        self.add_binary_vb(BinaryVB::I32Or);
    }

    fn visit_i32_xor(&mut self) {
        self.add_binary_vb(BinaryVB::I32Xor);
    }

    fn visit_i32_shl(&mut self) {
        self.add_binary_vb(BinaryVB::I32Shl);
    }

    fn visit_i32_shr_s(&mut self) {
        self.add_binary_vb(BinaryVB::I32ShrS);
    }

    fn visit_i32_shr_u(&mut self) {
        self.add_binary_vb(BinaryVB::I32ShrU);
    }

    fn visit_i32_rotl(&mut self) {
        self.add_binary_vb(BinaryVB::I32Rotl);
    }

    fn visit_i32_rotr(&mut self) {
        self.add_binary_vb(BinaryVB::I32Rotr);
    }

    fn visit_i64_clz(&mut self) {
        self.add_unary_vb(UnaryVB::I64Clz);
    }

    fn visit_i64_ctz(&mut self) {
        self.add_unary_vb(UnaryVB::I64Ctz);
    }

    fn visit_i64_popcnt(&mut self) {
        self.add_unary_vb(UnaryVB::I64PopCnt);
    }

    fn visit_i64_add(&mut self) {
        self.add_binary_vb(BinaryVB::I64Add);
    }

    fn visit_i64_sub(&mut self) {
        self.add_binary_vb(BinaryVB::I64Sub);
    }

    fn visit_i64_mul(&mut self) {
        self.add_binary_vb(BinaryVB::I64Mul);
    }

    fn visit_i64_div_s(&mut self) {
        self.add_binary_vb(BinaryVB::I64DivS);
    }

    fn visit_i64_div_u(&mut self) {
        self.add_binary_vb(BinaryVB::I64DivU);
    }

    fn visit_i64_rem_s(&mut self) {
        self.add_binary_vb(BinaryVB::I64RemS);
    }

    fn visit_i64_rem_u(&mut self) {
        self.add_binary_vb(BinaryVB::I64RemU);
    }

    fn visit_i64_and(&mut self) {
        self.add_binary_vb(BinaryVB::I64And);
    }

    fn visit_i64_or(&mut self) {
        self.add_binary_vb(BinaryVB::I64Or);
    }

    fn visit_i64_xor(&mut self) {
        self.add_binary_vb(BinaryVB::I64Xor);
    }

    fn visit_i64_shl(&mut self) {
        self.add_binary_vb(BinaryVB::I64Shl);
    }

    fn visit_i64_shr_s(&mut self) {
        self.add_binary_vb(BinaryVB::I64ShrS);
    }

    fn visit_i64_shr_u(&mut self) {
        self.add_binary_vb(BinaryVB::I64ShrU);
    }

    fn visit_i64_rotl(&mut self) {
        self.add_binary_vb(BinaryVB::I64Rotl);
    }

    fn visit_i64_rotr(&mut self) {
        self.add_binary_vb(BinaryVB::I64Rotr);
    }

    fn visit_f32_abs(&mut self) {
       self.add_unary_vb(UnaryVB::F32Abs);
    }

    fn visit_f32_neg(&mut self) {
        self.add_unary_vb(UnaryVB::F32Neg);
    }

    fn visit_f32_ceil(&mut self) {
        self.add_unary_vb(UnaryVB::F32Ceil);
    }

    fn visit_f32_floor(&mut self) {
        self.add_unary_vb(UnaryVB::F32Floor);
    }

    fn visit_f32_trunc(&mut self) {
        self.add_unary_vb(UnaryVB::F32Trunc);
    }

    fn visit_f32_nearest(&mut self) {
        self.add_unary_vb(UnaryVB::F32Nearest);
    }

    fn visit_f32_sqrt(&mut self) {
        self.add_unary_vb(UnaryVB::F32Sqrt);
    }

    fn visit_f32_add(&mut self) {
        self.add_binary_vb(BinaryVB::F32Add);
    }

    fn visit_f32_sub(&mut self) {
        self.add_binary_vb(BinaryVB::F32Sub);
    }

    fn visit_f32_mul(&mut self) {
        self.add_binary_vb(BinaryVB::F32Mul);
    }

    fn visit_f32_div(&mut self) {
        self.add_binary_vb(BinaryVB::F32Div);
    }

    fn visit_f32_min(&mut self) {
        self.add_binary_vb(BinaryVB::F32Min);
    }

    fn visit_f32_max(&mut self) {
        self.add_binary_vb(BinaryVB::F32Max);
    }

    fn visit_f32_copysign(&mut self) {
        self.add_binary_vb(BinaryVB::F32CopySign);
    }

    fn visit_f64_abs(&mut self) {
        self.add_unary_vb(UnaryVB::F64Abs);
    }

    fn visit_f64_neg(&mut self) {
        self.add_unary_vb(UnaryVB::F64Neg);
    }

    fn visit_f64_ceil(&mut self) {
        self.add_unary_vb(UnaryVB::F64Ceil);
    }

    fn visit_f64_floor(&mut self) {
        self.add_unary_vb(UnaryVB::F64Floor);
    }

    fn visit_f64_trunc(&mut self) {
        self.add_unary_vb(UnaryVB::F64Trunc);
    }

    fn visit_f64_nearest(&mut self) {
        self.add_unary_vb(UnaryVB::F64Nearest);
    }

    fn visit_f64_sqrt(&mut self) {
        self.add_unary_vb(UnaryVB::F64Sqrt);
    }

    fn visit_f64_add(&mut self) {
        self.add_binary_vb(BinaryVB::F64Add);
    }

    fn visit_f64_sub(&mut self) {
        self.add_binary_vb(BinaryVB::F64Sub);
    }

    fn visit_f64_mul(&mut self) {
        self.add_binary_vb(BinaryVB::F64Mul);
    }

    fn visit_f64_div(&mut self) {
        self.add_binary_vb(BinaryVB::F64Div);
    }

    fn visit_f64_min(&mut self) {
        self.add_binary_vb(BinaryVB::F64Min);
    }

    fn visit_f64_max(&mut self) {
        self.add_binary_vb(BinaryVB::F64Max);
    }

    fn visit_f64_copysign(&mut self) {
        self.add_binary_vb(BinaryVB::F64CopySign);
    }

    fn visit_i32_wrap_i64(&mut self) {
        self.add_unary_vb(UnaryVB::I32WrapI64);
    }

    fn visit_i32_trunc_f32_s(&mut self) {
        self.add_unary_vb(UnaryVB::I32TruncF32S);
    }

    fn visit_i32_trunc_f32_u(&mut self) {
        self.add_unary_vb(UnaryVB::I32TruncF32U);
    }

    fn visit_i32_trunc_f64_s(&mut self) {
        self.add_unary_vb(UnaryVB::I32TruncF64S);
    }

    fn visit_i32_trunc_f64_u(&mut self) {
        self.add_unary_vb(UnaryVB::I32TruncF64U);
    }

    fn visit_i64_extend_i32_s(&mut self) {
        self.add_unary_vb(UnaryVB::I64ExtendI32S);
    }

    fn visit_i64_extend_i32_u(&mut self) {
        self.add_unary_vb(UnaryVB::I64ExtendI32U);
    }

    fn visit_i64_trunc_f32_s(&mut self) {
        self.add_unary_vb(UnaryVB::I64TruncF32S);
    }

    fn visit_i64_trunc_f32_u(&mut self) {
        self.add_unary_vb(UnaryVB::I64TruncF32U);
    }

    fn visit_i64_trunc_f64_s(&mut self) {
        self.add_unary_vb(UnaryVB::I64TruncF64S);
    }

    fn visit_i64_trunc_f64_u(&mut self) {
        self.add_unary_vb(UnaryVB::I64TruncF64U);
    }

    fn visit_f32_convert_i32_s(&mut self) {
        self.add_unary_vb(UnaryVB::F32ConvertI32S);
    }

    fn visit_f32_convert_i32_u(&mut self) {
        self.add_unary_vb(UnaryVB::F32ConvertI32U);
    }

    fn visit_f32_convert_i64_s(&mut self) {
        self.add_unary_vb(UnaryVB::F32ConvertI64S);
    }

    fn visit_f32_convert_i64_u(&mut self) {
        self.add_unary_vb(UnaryVB::F32ConvertI64U);
    }

    fn visit_f32_demote_f64(&mut self) {
        self.add_unary_vb(UnaryVB::F32DemoteF64);
    }

    fn visit_f64_convert_i32_s(&mut self) {
        self.add_unary_vb(UnaryVB::F64ConvertI32S);
    }

    fn visit_f64_convert_i32_u(&mut self) {
        self.add_unary_vb(UnaryVB::F64ConvertI32U);
    }

    fn visit_f64_convert_i64_s(&mut self) {
        self.add_unary_vb(UnaryVB::F64ConvertI64S);
    }

    fn visit_f64_convert_i64_u(&mut self) {
        self.add_unary_vb(UnaryVB::F64ConvertI64U);
    }

    fn visit_f64_promote_f32(&mut self) {
        self.add_unary_vb(UnaryVB::F64PromoteF32);
    }

    fn visit_i32_reinterpret_f32(&mut self) {
       let mut vb = self.vb_stack.pop().unwrap();
       if vb.produces_non_canonical_nan() {vb = vb.adjust_for_non_canonical_nan();}
       self.vb_stack.push(vb); 
    }

    fn visit_i64_reinterpret_f64(&mut self) {
        ()
    }

    fn visit_f32_reinterpret_i32(&mut self) {
        ()
    }

    fn visit_f64_reinterpret_i64(&mut self) {
        ()
    }
}

fn val_type_size(ty: &ValType) -> ValueSize {
    match ty{
        ValType::I32 | ValType::F32 => ValueSize::Word,
        ValType::I64 | ValType::F64 => ValueSize::DoubleWord,
        ValType::V128 => panic!("V128 not supported"),
        ValType::Ref(..) => panic!("Ref not supported")
    }
}
