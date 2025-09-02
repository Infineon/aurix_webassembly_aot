use env_wrapper::wrap_env;

use alloc::vec;
use alloc::vec::Vec;
use defmt::Format;
use crate::isa_model::{Const10, Const16, DataRegister, ExtendedRegister, MapperLocation, Register, RegisterOrLargeConst, ValueSize, ADDRESS_ACCUMULATOR, STACK_POINTER};
use crate::isa_model::machine_instructions::Instr;
use crate::parse_and_translate::WasmRuntime;
use crate::translator::Translator;
use wasmparser::SubType;

#[derive(Debug, Clone, Format)]
pub enum LibraryFunction {
    F32Sqrt,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    I32TruncF64S,
    I32TruncF64U,
    F32DemoteF64,
    F32ConvertI64S,
    F32ConvertI64U,
    F64Sqrt,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,
    F64PromoteF32,
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
    F32Min,
    F32Max,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,
    I64Clz,
    I64Ctz,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Eq,
    F64Ne,
    F64Lt,
    F64Ge,
    F64Le,
    F64Gt,
}

impl <'a,'b> Translator<'a,'b>{

    /// generates machine code for calling a runtime function from the translated wasm functions. This is to be used in code generation for subroutines that are not supported through a short sequence of machine instructions.
    /// It is assumed that the runtime function takes at most 2 arguments from the same size (32-bit word or 64-bit double-word) and returns one result.
    /// 
    /// ### Parameters:
    /// - **target**: the result is moved to the target location if available, otherwise placed in a scratch register
    /// - **function**: runtime function to be called
    /// - **ops**: vector containing the location of the operands
    /// - **op_size**: size of the operand(s)
    /// - **result_size**: size of the result
    /// - **scratch_variable_map**: needs to be passed down in order to allocate a free register for the result, in case no target location is specified.
    /// 
    /// ### Return value:
    ///  location of the result value
    pub fn call_library_function(&mut self, target: Option<&MapperLocation>, function:LibraryFunction, ops: Vec<&MapperLocation>, op_size: ValueSize, result_size: ValueSize, scratch_variable_map : &mut Vec<MapperLocation>) -> MapperLocation {
        self.setup_ops(op_size, ops, scratch_variable_map);
        self.perform_external_call(function);
        self.process_external_result(target, result_size, scratch_variable_map)
    }

    /// generates machine code to obtain the result returned by the runtime function from the return register (according to the C ABI calling conventions for TriCore (D[2] or E[2]))
    /// and moves it to the target location if available, otherwise to an available scratch register.
    /// 
    /// Note that the call for the runtime function saves the lower context that needs to be therefore restored. Therefore if the result is to be stored in a lower context register,
    /// it is first saved temporarily into the stack until the lower context is restored and is then retrieved to be loaded in its target. 
    ///  
    fn process_external_result(&mut self, target: Option<&MapperLocation>, result_size: ValueSize, scratch_variable_map: &mut Vec<MapperLocation>) -> MapperLocation {
        let target = target.cloned().unwrap_or_else(|| self.next_available_register(result_size, scratch_variable_map).as_location());
        let intermediate_target =  match &target {
            MapperLocation::DataRegister(DataRegister(index)) if *index < 8 => MapperLocation::Stack { size: result_size },
            MapperLocation::ExtendedRegister(ExtendedRegister(index)) if *index < 8 => MapperLocation::Stack { size: result_size },
            _ => target.clone()
        };
        match result_size{
            ValueSize::Word => DataRegister(2).map_to_location(Some(&intermediate_target), self, scratch_variable_map),
            ValueSize::DoubleWord => ExtendedRegister(2).map_to_location(Some(&intermediate_target), self, scratch_variable_map),
        };
        self.push_instruction(Instr::RSLCX);
        match target {
            MapperLocation::DataRegister(DataRegister(index)) if index < 8 => {
                self.push_instruction(Instr::LDWPI { dest: DataRegister(index), base: STACK_POINTER, offset: Const10(4) });
            },
            MapperLocation::ExtendedRegister(ExtendedRegister(index)) if index < 8 => {
                self.push_instruction(Instr::LDDPI { dest: ExtendedRegister(index), base: STACK_POINTER, offset: Const10(8) });
            },
            _ => ()
        }
        target
    }

    /// helper method that maps the function to be called to its address and generates the machine code that performs the actual call.
    /// 
    /// Here direct calls are not an option because absolute addressing is only available for a portion of the address space (First 2MiB of every segment).
    /// Relative addressing can also not be implemented given that it requires the displacement to be less than 32 MiB.
    /// 
    /// Therefore indirect calls are implemented. The jump address is loaded in the address accumulator (over 2 steps given that immediates are 16-bit wide)
    /// 
    /// 
    fn perform_external_call(&mut self, function: LibraryFunction) {
        let library_function_ptr = match function {
            LibraryFunction::F32Sqrt => libm::sqrtf as u32,
            LibraryFunction::F64Eq => WasmRuntime::f64_eq as u32,
            LibraryFunction::F64Sub => WasmRuntime::f64_sub as u32,
            LibraryFunction::F64Mul => compiler_builtins::float::mul::__muldf3 as u32, // The compiler builtin f64 mul is bugged so we use another implementation
            LibraryFunction::F64Le => WasmRuntime::f64_le as u32,
            LibraryFunction::F64Add => WasmRuntime::f64_add as u32,
            LibraryFunction::F64Div => WasmRuntime::f64_div as u32,
            LibraryFunction::F32Ceil => libm::ceilf as u32,
            LibraryFunction::F32Floor => libm::floorf as u32,
            LibraryFunction::F32Trunc => libm::truncf as u32,
            LibraryFunction::F32Nearest => libm::rintf as u32,
            LibraryFunction::I32TruncF64S => WasmRuntime::i32_trunc_f64_s as u32,
            LibraryFunction::I32TruncF64U => WasmRuntime::i32_trunc_f64_u as u32,
            LibraryFunction::I64TruncF32S => WasmRuntime::i64_trunc_f32_s as u32,
            LibraryFunction::I64TruncF32U => WasmRuntime::i64_trunc_f32_u as u32,
            LibraryFunction::I64TruncF64S => WasmRuntime::i64_trunc_f64_s as u32,
            LibraryFunction::I64TruncF64U => WasmRuntime::i64_trunc_f64_u as u32,
            LibraryFunction::F32DemoteF64 => WasmRuntime::f32_demote_f64 as u32,
            LibraryFunction::F32ConvertI64S => WasmRuntime::f32_convert_i64_s as u32,
            LibraryFunction::F32ConvertI64U => WasmRuntime::f32_convert_i64_u as u32,
            LibraryFunction::F64Sqrt => libm::sqrt as u32,
            LibraryFunction::F64Ceil => libm::ceil as u32,
            LibraryFunction::F64Floor => libm::floor as u32,
            LibraryFunction::F64Trunc => libm::trunc as u32,
            LibraryFunction::F64Nearest => libm::rint as u32,
            LibraryFunction::F64PromoteF32 => WasmRuntime::f64_promote_f32 as u32,
            LibraryFunction::F64ConvertI32S => WasmRuntime::f64_convert_i32_s as u32,
            LibraryFunction::F64ConvertI32U => WasmRuntime::f64_convert_i32_u as u32,
            LibraryFunction::F64ConvertI64S => WasmRuntime::f64_convert_i64_s as u32,
            LibraryFunction::F64ConvertI64U => WasmRuntime::f64_convert_i64_u as u32,
            LibraryFunction::F64Lt => WasmRuntime::f64_lt as u32,
            LibraryFunction::F64Ge => WasmRuntime::f64_ge as u32,
            LibraryFunction::F64Min => WasmRuntime::f64_min as u32,
            LibraryFunction::F64Max => WasmRuntime::f64_max as u32,
            LibraryFunction::I64DivS => WasmRuntime::i64_div_s as u32,
            LibraryFunction::I64DivU => WasmRuntime::i64_div_u as u32,
            LibraryFunction::I64RemS => WasmRuntime::i64_rem_s as u32,
            LibraryFunction::I64RemU => WasmRuntime::i64_rem_u as u32,
            LibraryFunction::F32Min => WasmRuntime::f32_min as u32,
            LibraryFunction::F32Max => WasmRuntime::f32_max as u32,
            LibraryFunction::I64Shl => WasmRuntime::i64_shl as u32,
            LibraryFunction::I64ShrS => WasmRuntime::i64_shr_s as u32,
            LibraryFunction::I64ShrU => WasmRuntime::i64_shr_u as u32,
            LibraryFunction::I64Rotl => WasmRuntime::i64_rotl as u32,
            LibraryFunction::I64Rotr => WasmRuntime::i64_rotr as u32,
            LibraryFunction::F64Ne => WasmRuntime::f64_ne as u32,
            LibraryFunction::F64Gt => WasmRuntime::f64_gt as u32,
            LibraryFunction::I64Clz => WasmRuntime::i64_clz as u32,
            LibraryFunction::I64Ctz => WasmRuntime::i64_ctz as u32,
        };
        let fun_ptr_lower =  library_function_ptr as u16;
        let fun_ptr_upper = (library_function_ptr.wrapping_add(0x8000) >> 16) as u16;
        self.push_instruction(Instr::MOVHA { src: Const16(fun_ptr_upper), dest: ADDRESS_ACCUMULATOR });
        self.push_instruction(Instr::LEA { base:ADDRESS_ACCUMULATOR,  offset: Const16(fun_ptr_lower), dest: ADDRESS_ACCUMULATOR });
        self.push_instruction(Instr::CALLI { target: ADDRESS_ACCUMULATOR });
    }

    /// helper function to load the operands in the respective registers according to the TriCore C ABI calling convention:
    /// If the arguments are 32-bit wide. The first argument is placed in D[4], while the second if existent will be in D[5].
    /// Otherwise if the arguments are 64-bit wide. The first is placed in E[4], while the second if existent will be in E[6].
    /// 
    /// Note that we need to account for the scenario where we have multiple arguments and one exists already in the target of the other one.
    /// In this implementation, the arguments are filled backward and the first argument is saved in another register first if it is located at the target of the second one.
    // TODO: this might need to be rewritten, this function is only called for library functions that take at most 2 arguments, 
    // so it might be better to just use a match statement.
    // Also using D[0] as a temporary register for swapping the arguments is not a good idea as it is used for the bitmask
    fn setup_ops(&mut self, arg_size: ValueSize, mut args: Vec<&MapperLocation>, scratch_variable_map: &mut Vec<MapperLocation>) {
        self.push_instruction(Instr::SVLCX);
        let start_index = 4;
        let increment = match arg_size {
            ValueSize::Word => 1,
            ValueSize::DoubleWord => 2
        };
        //start register index for filling the arguments backwards 
        // TODO: you don't need to fill backwards
        let mut index = start_index + increment * args.len() as u8;
        //checks if first argument is located in the target of the second one
        if args.len() == 2 {
            match args[0]{
                MapperLocation::DataRegister(DataRegister(i)) if *i == index-increment => {
                    self.push_instruction(Instr::MOV { src: RegisterOrLargeConst::DataRegister(DataRegister(*i)), dest: Register::DataRegister(DataRegister(0)) });
                    args[0] = &MapperLocation::DataRegister(DataRegister(0));
                },
                MapperLocation::ExtendedRegister(ExtendedRegister(i)) if *i == index-increment => {
                    self.push_instruction(Instr::MOV { src: RegisterOrLargeConst::RegisterCouple {lower: DataRegister(*i), upper: DataRegister(*i+1)}, dest: Register::ExtendedRegister(ExtendedRegister(0)) });
                    args[0] = &MapperLocation::ExtendedRegister(ExtendedRegister(0));
                },
                _ => ()
            }
            //swap arguments to iterate over them in reverse order.
            args.swap(0, 1);
        }
        for op in args {
            index -= increment;
            match arg_size {
                ValueSize::Word => {
                    op.map_to_data_register(Some(DataRegister::new(index)), self, scratch_variable_map, &vec![]);
                },
                ValueSize::DoubleWord =>{
                    op.map_to_extended_register(Some(ExtendedRegister::new(index)), self, scratch_variable_map, &vec![]);
                }
            };
        }
    }
}

impl <'a> WasmRuntime <'a> {
    pub extern "C" fn f64_eq(x: f64, y: f64) -> u32 {
        if x == y {1} else {0}
    }

    pub extern "C" fn f64_sub(x: f64, y: f64) -> f64 {
        x - y
    }

    pub extern "C" fn f64_le(x: f64, y: f64) -> u32 {
        if x <= y {1} else {0}
    }

    pub extern "C" fn f64_add(x: f64, y: f64) -> f64 {
        x + y
    }

    pub extern "C" fn f64_ge(x: f64, y: f64) -> u32 {
        if x >= y {1} else {0}
    }

    pub extern "C" fn f64_lt(x: f64, y: f64) -> u32 {
        if x < y {1} else {0}
    }

    pub extern "C" fn f64_ne(x: f64, y: f64) -> u32 {
        if x != y {1} else {0}
    }

    pub extern "C" fn f64_div(x: f64, y: f64) -> f64 {
        x / y
    }

    pub extern "C" fn f64_promote_f32(x: f32) -> f64 {
        x as f64
    }

    pub extern "C" fn f64_convert_i32_s(x: i32) -> f64 {
        x as f64
    }

    pub extern "C" fn f64_convert_i32_u(x: u32) -> f64 {
        x as f64
    }

    pub extern "C" fn f64_convert_i64_s(x: i64) -> f64 {
        x as f64
    }

    pub extern "C" fn f64_convert_i64_u(x: u64) -> f64 {
        x as f64
    }

    pub extern "C" fn f64_convert_f32(x: f64) -> f32 {
        x as f32
    }

    pub extern "C" fn i32_trunc_f64_s(x: f64) -> i32 {
        x as i32
    }

    pub extern "C" fn i32_trunc_f64_u(x: f64) -> u32 {
        x as u32
    }

    pub extern "C" fn i64_trunc_f64_s(x: f64) -> i64 {
        x as i64
    }

    pub extern "C" fn i64_trunc_f64_u(x: f64) -> u64 {
        x as u64
    }

    pub extern "C" fn i64_trunc_f32_s(x: f32) -> i64 {
        x as i64
    }

    pub extern "C" fn i64_trunc_f32_u(x: f32) -> u64 {
        x as u64
    }

    pub extern "C" fn f32_demote_f64(x: f64) -> f32 {
        x as f32
    }

    pub extern "C" fn f32_convert_i64_s(x: i64) -> f32 {
        x as f32
    }

    pub extern "C" fn f32_convert_i64_u(x: u64) -> f32 {
        x as f32
    }

    pub extern "C" fn i64_div_s(x: i64, y: i64) -> i64 {
        x / y
    }

    pub extern "C" fn i64_div_u(x: u64, y: u64) -> u64 {
        x / y
    }

    pub extern "C" fn i64_rem_s(x: i64, y: i64) -> i64 {
        x.wrapping_rem(y)
    }

    pub extern "C" fn i64_rem_u(x: u64, y: u64) -> u64 {
        x % y
    }

    pub extern "C" fn f64_gt(x: f64, y: f64) -> u32 {
        if x > y {1} else {0}
    }

    pub extern "C" fn i64_shl(x: u64, y: u64) -> u64 {
        x.wrapping_shl(y as u32)
    }

    pub extern "C" fn i64_shr_s(x: i64, y: u64) -> i64 {
        x.wrapping_shr(y as u32)
    }

    pub extern "C" fn i64_shr_u(x: u64, y: u64) -> u64 {
        x.wrapping_shr(y as u32)
    }

    pub extern "C" fn i64_rotl(x: u64, y: u64) -> u64 {
        x.rotate_left(y as u32)
    }

    pub extern "C" fn i64_rotr(x: u64, y: u64) -> u64 {
        x.rotate_right(y as u32)
    }

    pub extern "C" fn f32_max(x: f32, y: f32) -> f32 {
        if x.is_nan() || y.is_nan() {
            return f32::NAN;
        }

        if x == 0.0 && y == 0.0  &&  (x.is_sign_positive() || y.is_sign_positive()) {
            return 0.0;
        }

        if x < y {
            y
        } else {
            x
        }
    }

    pub extern "C" fn f32_min(x: f32, y: f32) -> f32 {
        if x.is_nan() || y.is_nan() {
            return f32::NAN;
        }

        if x == 0.0 && y == 0.0  &&  (x.is_sign_negative() || y.is_sign_negative()) {
            return -0.0;
        }

        if x < y {
            x
        } else {
            y
        }
    }

    pub extern "C" fn f64_max(x: f64, y: f64) -> f64 {
        if x.is_nan() || y.is_nan() {
            return f64::NAN;
        }

        if x == 0.0 && y == 0.0  &&  (x.is_sign_positive() || y.is_sign_positive()) {
            return 0.0;
        }

        if x < y {
            y
        } else {
            x
        }
    }

    pub extern "C" fn f64_min(x: f64, y: f64) -> f64 {
        if x.is_nan() || y.is_nan() {
            return f64::NAN;
        }

        if x == 0.0 && y == 0.0  &&  (x.is_sign_negative() || y.is_sign_negative()) {
            return -0.0;
        }

        if x < y {
            x
        } else {
            y
        }
    }
    
    pub extern "C" fn i64_clz(x: u64) -> u64 {
        x.leading_zeros() as u64
    }

    pub extern "C" fn i64_ctz(x: u64) -> u64 {
        x.trailing_zeros() as u64
    }

    /// This subroutine is to be called prior to an indirect call in a wasm function. It performs the necessary checks before running the indirect call to ensure
    /// type safety. It checks that the provided table offset does not exceed the table size (in case sandboxing is enabled) and checks whether the referenced function
    /// matches the statically annotated type.
    pub extern "C" fn compare_subtypes(types: *const SubType, table_type_indices: *const u32, table_offset :u32, target_type_index : u32, _table_size: u32)  {
        #[cfg(feature="address-masking")]
        assert!(table_offset < _table_size);
        let type_index =  unsafe{ *table_type_indices.wrapping_add(table_offset as usize)};
        let ty = types.wrapping_add(type_index as usize);
        let target_ty = types.wrapping_add(target_type_index as usize);
        unsafe {
            assert_eq!((*ty).composite_type, (*target_ty).composite_type)
        }
    }

    /// This subroutine is to be called for implementing the memory.grow instruction.
    /// It checks whether the new expected size of the memory (in pages) exceeds the maximum size allowed.
    /// The maximum allowed size is dependent both of the allocated space for the linear memory and the maximum indicated by the wasm module.
    pub extern "C" fn grow_memory(current_size: &mut u32, grow_size:u32, maximum_size:u32) -> u32 {
        let previous_size = *current_size;
        match previous_size.checked_add(grow_size){
            Some (new_size) if new_size <= maximum_size => {
                *current_size = new_size;
                previous_size
            }
            _ => u32::MAX
        }
    }

    wrap_env!{
        fn __write__(address: u32, value: u32){
            unsafe {
                let mem_ptr = address as *mut u32;
                *mem_ptr = value;
            }
        }
    }

    wrap_env!{
        fn __read__(address:u32) -> u32 {
            unsafe {
                let mem_ptr = address as *const u32;
                *mem_ptr
            }
        }
    }

}
