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

    pub fn call_library_function(&mut self, target: Option<&MapperLocation>, function:LibraryFunction, ops: Vec<&MapperLocation>, op_size: ValueSize, result_size: ValueSize, scratch_variable_map : &mut Vec<MapperLocation>) -> MapperLocation {
        self.push_instruction(Instr::SVLCX);
        self.setup_ops(op_size, ops, scratch_variable_map);
        self.perform_external_call(function);
        self.process_external_result(target, result_size, scratch_variable_map)
    }

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

    fn perform_external_call(&mut self, function: LibraryFunction) {
        let library_function_ptr = match function {
            LibraryFunction::F32Sqrt => libm::sqrtf as u32,
            LibraryFunction::F64Eq => WasmRuntime::f64_eq as u32,
            LibraryFunction::F64Sub => WasmRuntime::f64_sub as u32,
            LibraryFunction::F64Mul => compiler_builtins::float::mul::__muldf3 as u32,
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

    fn setup_ops(&mut self, op_size: ValueSize, mut ops: Vec<&MapperLocation>, scratch_variable_map: &mut Vec<MapperLocation>) {
        let start_index = 4;
        let increment = match op_size {
            ValueSize::Word => 1,
            ValueSize::DoubleWord => 2
        };
        let mut index = start_index + increment * ops.len() as u8;
        if ops.len() == 2 {
            match ops[0]{
                MapperLocation::DataRegister(DataRegister(i)) if *i == index-increment => {
                    self.push_instruction(Instr::MOV { src: RegisterOrLargeConst::DataRegister(DataRegister(*i)), dest: Register::DataRegister(DataRegister(0)) });
                    ops[0] = &MapperLocation::DataRegister(DataRegister(0));
                },
                MapperLocation::ExtendedRegister(ExtendedRegister(i)) if *i == index-increment => {
                    self.push_instruction(Instr::MOV { src: RegisterOrLargeConst::RegisterCouple {lower: DataRegister(*i), upper: DataRegister(*i+1)}, dest: Register::ExtendedRegister(ExtendedRegister(0)) });
                    ops[0] = &MapperLocation::ExtendedRegister(ExtendedRegister(0));
                },
                _ => ()
            }
            ops.swap(0, 1);
        }
        for op in ops {
            index -= increment;
            match op_size {
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

    pub extern "C" fn compare_subtypes(types: *const SubType, table_type_indices: *const u32, table_offset :u32, target_type_index : u32)  {
        let type_index =  unsafe{ *table_type_indices.wrapping_add(table_offset as usize)};
        let ty = types.wrapping_add(type_index as usize);
        let target_ty = types.wrapping_add(target_type_index as usize);
        unsafe {
            assert_eq!((*ty).composite_type, (*target_ty).composite_type)
        }
    }
}
