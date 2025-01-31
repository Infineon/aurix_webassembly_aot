use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use crate::isa_model::{Const16, Const9, DataRegister, ExtendedRegister, Immediate, LocationCouple, MapperLocation, Memsize, Register, RegisterOrConst, RegisterOrLargeConst, RegisterOrSmallConst, SignValue, ValueSize};
use crate::isa_model::machine_instructions::Instr;
use crate::translator::library_function::LibraryFunction;
use crate::translator::Translator;
use crate::vb::{AtomicVB, BinaryVB, UnaryVB, VB};

impl<'a,'b> Translator<'a,'b> {
    
    pub fn resolve_all(&mut self) {
        let mut stack_offset = 0;

        for index in 0..self.vb_stack.len() {
            let mut vb = self.vb_stack[index].clone();
            match vb {
                VB::AtomicVB(AtomicVB::Resolved { offset, .. }) => {
                    stack_offset = offset;
                },
                _ => {
                    if vb.produces_non_canonical_nan() { vb = vb.adjust_for_snan() }
                    let size = vb.val_size(&self.locals_map, &self.global_translator.globals_map);
                    let mut scratch_variable_map = Vec::new();
                    vb.post_order_dfs(|vb, is_top| {
                        let stack_location = MapperLocation::Stack { size };
                        self.resolve_vb(if is_top { Some(&stack_location) } else { None }, vb, &mut scratch_variable_map);
                    });
                    stack_offset += size.as_bytes() as usize;
                    self.vb_stack[index] = VB::AtomicVB(AtomicVB::Resolved { size, offset: stack_offset })
                }
            }
        }
    }


    pub fn resolve_with_target(&mut self, target: Option<&MapperLocation>) -> MapperLocation {
        let mut vb = self.vb_stack.pop().unwrap();
        if vb.produces_non_canonical_nan() { vb = vb.adjust_for_snan() }
        let mut scratch_variable_map: Vec<MapperLocation> = Vec::new();
        vb.post_order_dfs(|vb, is_top| self.resolve_vb(if is_top { target } else { None }, vb, &mut scratch_variable_map));
        scratch_variable_map.pop().unwrap()
    }


    fn resolve_vb(&mut self, potential_target: Option<&MapperLocation>, vb: &VB, scratch_variable_map: &mut Vec<MapperLocation>) {
        let result = match vb {
            VB::AtomicVB(atomic_vb) => self.dispatch_atomic_vb(atomic_vb, potential_target, scratch_variable_map),
            VB::UnaryVB { vb, .. } => self.dispatch_unary_vb(scratch_variable_map, vb, potential_target),
            VB::BinaryVB { vb, .. } => self.dispatch_binary_vb(scratch_variable_map, vb, potential_target),
            VB::Select { size, .. } => self.gen_select(scratch_variable_map, potential_target, *size),
        };
        scratch_variable_map.push(result);
    }

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
            AtomicVB::Unreachable => MapperLocation::Unreachable,
            AtomicVB::MemorySize => MapperLocation::Global { offset: 0, size: ValueSize::Word },
        };
        match potential_target {
            Some(target) => result.map_to_location(target, self, scratch_variable_map, &vec![]),
            None => result
        }
    }

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
            ValueSize::DoubleWord => {
                let dest_register = self.get_dest_extended_register(potential_target, scratch_variable_map, &vec![]);
                let selector_register = selector.map_to_data_register(None, self, scratch_variable_map, &vec![lhs.clone(), rhs.clone()]);
                self.push_instruction(Instr::JEQ { target: self.cfg_label_map.len(), lhs: selector_register, rhs: RegisterOrSmallConst::new_const(0) });
                lhs.map_to_extended_register(Some(dest_register), self, scratch_variable_map, &vec![]);
                self.push_instruction(Instr::J { target: self.cfg_label_map.len() + 1 });
                self.cfg_label_map.push(Some(self.wasm_runtime.instructions_count));
                rhs.map_to_extended_register(Some(dest_register), self, scratch_variable_map, &vec![]);
                self.cfg_label_map.push(Some(self.wasm_runtime.instructions_count));
                dest_register.map_to_location(potential_target, self, scratch_variable_map)
            }
        }
    }

    fn gen_f32_le(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_data_registers(self, scratch_variable_map);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::CMPF { lhs: lhs_register, rhs: rhs_register, dest: dest_register });
        self.push_instruction(Instr::AND { lhs: dest_register, rhs: RegisterOrConst::new_const(0x0003), dest: dest_register });
        self.push_instruction(Instr::NE { lhs: dest_register, rhs: RegisterOrConst::new_const(0), dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_f32_gt(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_data_registers(self, scratch_variable_map);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::CMPF { lhs: rhs_register, rhs: lhs_register, dest: dest_register });
        self.push_instruction(Instr::AND { lhs: dest_register, rhs: RegisterOrConst::new_const(1), dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
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

    fn gen_f32_div(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_data_registers(self, scratch_variable_map);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::DIVF { lhs: lhs_register, rhs: rhs_register, dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_f32_mul(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_data_registers(self, scratch_variable_map);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::MULF { lhs: lhs_register, rhs: rhs_register, dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_f32_sub(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_data_registers(self, scratch_variable_map);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::SUBF { lhs: lhs_register, rhs: rhs_register, dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_f32_add(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_data_registers(self, scratch_variable_map);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::ADDF { lhs: lhs_register, rhs: rhs_register, dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn _gen_f32_min(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_data_registers(self, scratch_variable_map);
        let cmp_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![MapperLocation::DataRegister(lhs_register), MapperLocation::DataRegister(rhs_register)]);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::CMPF { lhs: lhs_register, rhs: rhs_register, dest: cmp_register });
        self.push_instruction(Instr::JZT { src: cmp_register, n: 3, target: self.cfg_label_map.len() });
        self.push_instruction(Instr::MOVH { src: Const16(0x7FC0), dest: dest_register });
        self.push_instruction(Instr::J { target: self.cfg_label_map.len() + 3 });
        self.cfg_label_map.push(Some(self.wasm_runtime.instructions_count));
        self.push_instruction(Instr::JZT { src: cmp_register, n: 2, target: self.cfg_label_map.len() });
        rhs_register.map_to_location(Some(&MapperLocation::DataRegister(dest_register)), self, scratch_variable_map);
        self.push_instruction(Instr::J { target: self.cfg_label_map.len() + 2 });
        self.cfg_label_map.push(Some(self.wasm_runtime.instructions_count));
        self.push_instruction(Instr::JZT { src: cmp_register, n: 1, target: self.cfg_label_map.len() });
        self.push_instruction(Instr::JEQ { target: self.cfg_label_map.len(), lhs: lhs_register, rhs: RegisterOrSmallConst::DataRegister(rhs_register) });
        self.push_instruction(Instr::MOVH { src: Const16(0), dest: dest_register });
        self.push_instruction(Instr::J { target: self.cfg_label_map.len() + 1 });
        self.cfg_label_map.push(Some(self.wasm_runtime.instructions_count));
        lhs_register.map_to_location(Some(&MapperLocation::DataRegister(dest_register)), self, scratch_variable_map);
        self.cfg_label_map.push(Some(self.wasm_runtime.instructions_count));
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i64_xor(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register_const_couple) = (lhs, rhs).map_abelian_large_children_to_register_or_const(SignValue::Unsigned, self, scratch_variable_map);
        let lower_lhs = lhs_register.lower_half();
        let upper_lhs = lhs_register.upper_half();
        let (lower_rhs, upper_rhs) = rhs_register_const_couple;
        let ExtendedRegister(index_dest) = self.get_dest_extended_register(potential_target, scratch_variable_map, &vec![]);
        match lower_rhs {
            RegisterOrConst::Const9(Const9(0)) => {
                lower_lhs.map_to_location(Some(&MapperLocation::new_data_register(index_dest)), self, scratch_variable_map);
            },
            _ => self.push_instruction(Instr::XOR { lhs: lower_lhs, rhs: lower_rhs, dest: DataRegister(index_dest) })
        };
        match upper_rhs {
            RegisterOrConst::Const9(Const9(0)) => {
                upper_lhs.map_to_location(Some(&MapperLocation::new_data_register(index_dest + 1)), self, scratch_variable_map);
            },
            _ => self.push_instruction(Instr::XOR { lhs: upper_lhs, rhs: upper_rhs, dest: DataRegister(index_dest + 1) })
        };
        ExtendedRegister(index_dest).map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i64_or(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register_const_couple) = (lhs, rhs).map_abelian_large_children_to_register_or_const(SignValue::Unsigned, self, scratch_variable_map);
        let ExtendedRegister(index_dest) = self.get_dest_extended_register(potential_target, scratch_variable_map, &vec![]);
        let lower_lhs = lhs_register.lower_half();
        let upper_lhs = lhs_register.upper_half();
        let (lower_rhs, upper_rhs) = rhs_register_const_couple;
        match lower_rhs {
            RegisterOrConst::Const9(Const9(0)) => {
                lower_lhs.map_to_location(Some(&MapperLocation::new_data_register(index_dest)), self, scratch_variable_map);
            },
            _ => self.push_instruction(Instr::OR { lhs: lower_lhs, rhs: lower_rhs, dest: DataRegister(index_dest) })
        };
        match upper_rhs {
            RegisterOrConst::Const9(Const9(0)) => {
                upper_lhs.map_to_location(Some(&MapperLocation::new_data_register(index_dest + 1)), self, scratch_variable_map);
            },
            _ => self.push_instruction(Instr::OR { lhs: upper_lhs, rhs: upper_rhs, dest: DataRegister(index_dest + 1) })
        };
        ExtendedRegister(index_dest).map_to_location(potential_target, self, scratch_variable_map)
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

    fn gen_i32_shr_u(&mut self, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, lhs: &MapperLocation, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let rhs_register_const = match *rhs {
            MapperLocation::Immediate(imm) => RegisterOrConst::new_const((-(imm.as_i32() & 0x1F)) as u16),
            _ => {
                let count_register = self.next_available_data_register(scratch_variable_map, &vec![]);
                rhs.map_to_data_register(Some(count_register), self, scratch_variable_map, &vec![]);
                self.push_instruction(Instr::AND { lhs: count_register, rhs: RegisterOrConst::new_const(0x1F), dest: count_register });
                self.push_instruction(Instr::RSUB0 { src: count_register });
                RegisterOrConst::DataRegister(count_register)
            }
        };
        let lhs_register: DataRegister = lhs.map_to_data_register(None, self, scratch_variable_map, &vec![rhs_register_const.to_mapper_location()]);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::SH { src: lhs_register, count: rhs_register_const, dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_rotl(&mut self, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, lhs: &MapperLocation, potential_target: Option<&MapperLocation>) -> MapperLocation {
        match *rhs {
            MapperLocation::Immediate(imm) => {
                let lhs_register: DataRegister = lhs.map_to_data_register(None, self, scratch_variable_map, &vec![]);
                let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
                let intermediate = self.next_available_data_register(scratch_variable_map, &vec![MapperLocation::DataRegister(dest_register), MapperLocation::DataRegister(lhs_register)]);
                let count = (imm.as_u32() & 0x1F) as u16;
                self.push_instruction(Instr::EXTRUI { src: lhs_register, width: Const9::new(count), pos: Const9(32 - count), dest: intermediate });
                self.push_instruction(Instr::SH { src: lhs_register, count: RegisterOrConst::new_const(count), dest: dest_register });
                self.push_instruction(Instr::OR { lhs: dest_register, rhs: RegisterOrConst::DataRegister(intermediate), dest: dest_register });
                dest_register.map_to_location(potential_target, self, scratch_variable_map)
            },
            _ => {
                let width_pos_register = self.next_available_extended_register(scratch_variable_map, &vec![lhs.clone()]);
                rhs.map_to_data_register(Some(width_pos_register.upper_half()), self, scratch_variable_map, &vec![lhs.clone()]);
                self.push_instruction(Instr::AND { lhs: width_pos_register.upper_half(), rhs: RegisterOrConst::new_const(0x1F), dest: width_pos_register.upper_half() });
                self.push_instruction(Instr::RSUB { lhs: Const9::new(32), rhs: width_pos_register.upper_half(), dest: width_pos_register.lower_half() });
                let lhs_register: DataRegister = lhs.map_to_data_register(None, self, scratch_variable_map, &vec![MapperLocation::ExtendedRegister(width_pos_register)]);
                let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
                let intermediate = self.next_available_data_register(scratch_variable_map, &vec![MapperLocation::DataRegister(dest_register), MapperLocation::DataRegister(lhs_register), MapperLocation::ExtendedRegister(width_pos_register)]);
                self.push_instruction(Instr::MOV { src: RegisterOrLargeConst::new_const(0), dest: Register::DataRegister(intermediate) });
                self.push_instruction(Instr::EXTRU { src: lhs_register, width_pos: width_pos_register, dest: intermediate });
                self.push_instruction(Instr::SH { src: lhs_register, count: RegisterOrConst::DataRegister(width_pos_register.upper_half()), dest: dest_register });
                self.push_instruction(Instr::OR { lhs: dest_register, rhs: RegisterOrConst::DataRegister(intermediate), dest: dest_register });
                dest_register.map_to_location(potential_target, self, scratch_variable_map)
            }
        }
    }

    fn gen_i32_rotr(&mut self, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, lhs: &MapperLocation, potential_target: Option<&MapperLocation>) -> MapperLocation {
        match *rhs {
            MapperLocation::Immediate(imm) => {
                let lhs_register: DataRegister = lhs.map_to_data_register(None, self, scratch_variable_map, &vec![]);
                let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
                let intermediate = self.next_available_data_register(scratch_variable_map, &vec![MapperLocation::DataRegister(dest_register), MapperLocation::DataRegister(lhs_register)]);
                let count = (imm.as_u32() & 0x1F) as u16;
                self.push_instruction(Instr::EXTRUI { src: lhs_register, width: Const9::new((32 - count) % 32), pos: Const9::new(count), dest: intermediate });
                self.push_instruction(Instr::SH { src: lhs_register, count: RegisterOrConst::new_const(32 - count), dest: dest_register });
                self.push_instruction(Instr::OR { lhs: dest_register, rhs: RegisterOrConst::DataRegister(intermediate), dest: dest_register });
                dest_register.map_to_location(potential_target, self, scratch_variable_map)
            },
            _ => {
                let width_pos_register = self.next_available_extended_register(scratch_variable_map, &vec![lhs.clone()]);
                rhs.map_to_data_register(Some(width_pos_register.lower_half()), self, scratch_variable_map, &vec![lhs.clone()]);
                self.push_instruction(Instr::AND { lhs: width_pos_register.lower_half(), rhs: RegisterOrConst::new_const(0x1F), dest: width_pos_register.lower_half() });
                self.push_instruction(Instr::RSUB { lhs: Const9::new(32), rhs: width_pos_register.lower_half(), dest: width_pos_register.upper_half() });
                self.push_instruction(Instr::AND { lhs: width_pos_register.upper_half(), rhs: RegisterOrConst::new_const(0x1F), dest: width_pos_register.upper_half() });
                let lhs_register: DataRegister = lhs.map_to_data_register(None, self, scratch_variable_map, &vec![MapperLocation::ExtendedRegister(width_pos_register)]);
                let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
                let intermediate = self.next_available_data_register(scratch_variable_map, &vec![MapperLocation::DataRegister(dest_register), MapperLocation::DataRegister(lhs_register), MapperLocation::ExtendedRegister(width_pos_register)]);
                self.push_instruction(Instr::MOV { src: RegisterOrLargeConst::new_const(0), dest: Register::DataRegister(intermediate) });
                self.push_instruction(Instr::EXTRU { src: lhs_register, width_pos: width_pos_register, dest: intermediate });
                self.push_instruction(Instr::SH { src: lhs_register, count: RegisterOrConst::DataRegister(width_pos_register.upper_half()), dest: dest_register });
                self.push_instruction(Instr::OR { lhs: dest_register, rhs: RegisterOrConst::DataRegister(intermediate), dest: dest_register });
                dest_register.map_to_location(potential_target, self, scratch_variable_map)
            }
        }
    }

    fn gen_i32_shr_s(&mut self, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, lhs: &MapperLocation, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let rhs_register_const = match *rhs {
            MapperLocation::Immediate(imm) => RegisterOrConst::new_const((-(imm.as_i32() & 0x1F)) as u16),
            _ => {
                let count_register = self.next_available_data_register(scratch_variable_map, &vec![]);
                rhs.map_to_data_register(Some(count_register), self, scratch_variable_map, &vec![]);
                self.push_instruction(Instr::AND { lhs: count_register, rhs: RegisterOrConst::new_const(0x1F), dest: count_register });
                self.push_instruction(Instr::RSUB0 { src: count_register });
                RegisterOrConst::DataRegister(count_register)
            }
        };
        let lhs_register: DataRegister = lhs.map_to_data_register(None, self, scratch_variable_map, &vec![rhs_register_const.to_mapper_location()]);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::SHA { src: lhs_register, count: rhs_register_const, dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_shl(&mut self, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, lhs: &MapperLocation, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let rhs_register_const = match *rhs {
            MapperLocation::Immediate(imm) => RegisterOrConst::new_const((imm.as_u32() & 0x1F) as u16),
            _ => {
                let count_register = self.next_available_data_register(scratch_variable_map, &vec![]);
                rhs.map_to_data_register(Some(count_register), self, scratch_variable_map, &vec![]);
                self.push_instruction(Instr::AND { lhs: count_register, rhs: RegisterOrConst::new_const(0x1F), dest: count_register });
                RegisterOrConst::DataRegister(count_register)
            }
        };
        let lhs_register: DataRegister = lhs.map_to_data_register(None, self, scratch_variable_map, &vec![rhs_register_const.to_mapper_location()]);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::SH { src: lhs_register, count: rhs_register_const, dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_xor(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register_const) = (lhs, rhs).map_abelian_children_to_register_or_const(SignValue::Unsigned, self, scratch_variable_map);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::XOR { lhs: lhs_register, rhs: rhs_register_const, dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_or(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register_const) = (lhs, rhs).map_abelian_children_to_register_or_const(SignValue::Unsigned, self, scratch_variable_map);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::OR { lhs: lhs_register, rhs: rhs_register_const, dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_and(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register_const) = (lhs, rhs).map_abelian_children_to_register_or_const(SignValue::Unsigned, self, scratch_variable_map);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::AND { lhs: lhs_register, rhs: rhs_register_const, dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_rem_u(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_data_registers(self, scratch_variable_map);
        let ExtendedRegister(index) = self.next_available_extended_register(scratch_variable_map, &vec![]);
        self.push_instruction(Instr::DIVU { lhs: lhs_register, rhs: rhs_register, dest: ExtendedRegister(index) });
        DataRegister(index + 1).map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_rem_s(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_data_registers(self, scratch_variable_map);
        let ExtendedRegister(index) = self.next_available_extended_register(scratch_variable_map, &vec![]);
        self.push_instruction(Instr::DIV { lhs: lhs_register, rhs: rhs_register, dest: ExtendedRegister(index) });
        DataRegister(index + 1).map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_div_u(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_data_registers(self, scratch_variable_map);
        let ExtendedRegister(index) = self.next_available_extended_register(scratch_variable_map, &vec![]);
        self.push_instruction(Instr::DIVU { lhs: lhs_register, rhs: rhs_register, dest: ExtendedRegister(index) });
        DataRegister(index).map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_div_s(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_data_registers(self, scratch_variable_map);
        let ExtendedRegister(index) = self.next_available_extended_register(scratch_variable_map, &vec![]);
        self.push_instruction(Instr::DIV { lhs: lhs_register, rhs: rhs_register, dest: ExtendedRegister(index) });
        DataRegister(index).map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_mul(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register_const) = (lhs, rhs).map_abelian_children_to_register_or_const(SignValue::Signed, self, scratch_variable_map);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::MUL { lhs: lhs_register, rhs: rhs_register_const, dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

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
                if immediate >> 8 == 0 || immediate >> 8 == -1 {
                    self.push_instruction(Instr::RSUB { lhs: Const9::new(immediate as u16), rhs: operand_register, dest: dest_register });
                } else {
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

    fn gen_f32_ge(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_data_registers(self, scratch_variable_map);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::CMPF { lhs: lhs_register, rhs: rhs_register, dest: dest_register });
        self.push_instruction(Instr::AND { lhs: dest_register, rhs: RegisterOrConst::new_const(0b110), dest: dest_register });
        self.push_instruction(Instr::NE { lhs: dest_register, rhs: RegisterOrConst::new_const(0), dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_f32_lt(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_data_registers(self, scratch_variable_map);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::CMPF { lhs: lhs_register, rhs: rhs_register, dest: dest_register });
        self.push_instruction(Instr::AND { lhs: dest_register, rhs: RegisterOrConst::new_const(1), dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_f32_ne(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_data_registers(self, scratch_variable_map);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::CMPF { lhs: lhs_register, rhs: rhs_register, dest: dest_register });
        self.push_instruction(Instr::SH { src: dest_register, count: RegisterOrConst::new_const(-1i16 as u16), dest: dest_register });
        self.push_instruction(Instr::AND { lhs: dest_register, rhs: RegisterOrConst::new_const(1), dest: dest_register });
        self.push_instruction(Instr::XOR { lhs: dest_register, rhs: RegisterOrConst::new_const(1), dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_f32_eq(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_data_registers(self, scratch_variable_map);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::CMPF { lhs: lhs_register, rhs: rhs_register, dest: dest_register });
        self.push_instruction(Instr::SH { src: dest_register, count: RegisterOrConst::new_const((-1i16) as u16), dest: dest_register });
        self.push_instruction(Instr::AND { lhs: dest_register, rhs: RegisterOrConst::new_const(1), dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

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

    fn gen_i32_gtu(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        match (lhs, rhs) {
            (lhs, MapperLocation::Immediate(imm)) if imm.as_u32() >> 9 == 0 => {
                let immediate = imm.as_u32();
                let lhs_register = lhs.map_to_data_register(None, self, scratch_variable_map, &vec![]);
                let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
                self.push_instruction(Instr::GEU { lhs: lhs_register, rhs: RegisterOrConst::new_const((immediate + 1) as u16), dest: dest_register });
                dest_register.map_to_location(potential_target, self, scratch_variable_map)
            },
            (lhs, rhs) => {
                let rhs_register = rhs.map_to_data_register(None, self, scratch_variable_map, &vec![lhs.clone()]);
                let lhs_register = lhs.map_to_register_or_const(SignValue::Unsigned, self, scratch_variable_map, &vec![MapperLocation::DataRegister(rhs_register)]);
                let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
                self.push_instruction(Instr::LTU { lhs: rhs_register, rhs: lhs_register, dest: dest_register });
                dest_register.map_to_location(potential_target, self, scratch_variable_map)
            }
        }
    }

    fn gen_i32_gts(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        match (lhs, rhs) {
            (lhs, MapperLocation::Immediate(imm)) if (imm.as_i32() + 1) >> 8 == 0 || (imm.as_i32() + 1) >> 8 == -1 => {
                let immediate = imm.as_i32();
                let lhs_register = lhs.map_to_data_register(None, self, scratch_variable_map, &vec![]);
                let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
                self.push_instruction(Instr::GE { lhs: lhs_register, rhs: RegisterOrConst::new_const((immediate + 1) as u16), dest: dest_register });
                dest_register.map_to_location(potential_target, self, scratch_variable_map)
            },
            (lhs, rhs) => {
                let rhs_register = rhs.map_to_data_register(None, self, scratch_variable_map, &vec![lhs.clone()]);
                let lhs_register = lhs.map_to_register_or_const(SignValue::Signed, self, scratch_variable_map, &vec![MapperLocation::DataRegister(rhs_register)]);
                let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
                self.push_instruction(Instr::LT { lhs: rhs_register, rhs: lhs_register, dest: dest_register });
                dest_register.map_to_location(potential_target, self, scratch_variable_map)
            }
        }
    }

    fn gen_i32_leu(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        match (lhs, rhs) {
            (lhs, MapperLocation::Immediate(imm)) if (imm.as_u32() + 1) >> 9 == 0 => {
                let immediate = imm.as_u32();
                let lhs_register = lhs.map_to_data_register(None, self, scratch_variable_map, &vec![]);
                let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
                self.push_instruction(Instr::LTU { lhs: lhs_register, rhs: RegisterOrConst::new_const((immediate + 1) as u16), dest: dest_register });
                dest_register.map_to_location(potential_target, self, scratch_variable_map)
            },
            (lhs, rhs) => {
                let rhs_register = rhs.map_to_data_register(None, self, scratch_variable_map, &vec![lhs.clone()]);
                let lhs_register = lhs.map_to_register_or_const(SignValue::Unsigned, self, scratch_variable_map, &vec![MapperLocation::DataRegister(rhs_register)]);
                let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
                self.push_instruction(Instr::GEU { lhs: rhs_register, rhs: lhs_register, dest: dest_register });
                dest_register.map_to_location(potential_target, self, scratch_variable_map)
            }
        }
    }

    fn gen_i32_les(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        match (lhs, rhs) {
            (lhs, MapperLocation::Immediate(imm)) if (imm.as_i32() + 1) >> 8 == 0 || (imm.as_i32() + 1) >> 8 == -1 => {
                let immediate = imm.as_i32();
                let lhs_register = lhs.map_to_data_register(None, self, scratch_variable_map, &vec![]);
                let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
                self.push_instruction(Instr::LT { lhs: lhs_register, rhs: RegisterOrConst::new_const((immediate + 1) as u16), dest: dest_register });
                dest_register.map_to_location(potential_target, self, scratch_variable_map)
            },
            (lhs, rhs) => {
                let rhs_register = rhs.map_to_data_register(None, self, scratch_variable_map, &vec![lhs.clone()]);
                let lhs_register = lhs.map_to_register_or_const(SignValue::Signed, self, scratch_variable_map, &vec![MapperLocation::DataRegister(rhs_register)]);
                let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
                self.push_instruction(Instr::GE { lhs: rhs_register, rhs: lhs_register, dest: dest_register });
                dest_register.map_to_location(potential_target, self, scratch_variable_map)
            }
        }
    }

    fn gen_i32_geu(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        match (lhs, rhs) {
            (MapperLocation::Immediate(imm), rhs) if (imm.as_u32() + 1) >> 9 == 0 => {
                let immediate = imm.as_u32();
                let rhs_register = rhs.map_to_data_register(None, self, scratch_variable_map, &vec![]);
                let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
                self.push_instruction(Instr::LTU { lhs: rhs_register, rhs: RegisterOrConst::new_const((immediate + 1) as u16), dest: dest_register });
                dest_register.map_to_location(potential_target, self, scratch_variable_map)
            },
            (lhs, rhs) => {
                let rhs_register = rhs.map_to_register_or_const(SignValue::Unsigned, self, scratch_variable_map, &vec![lhs.clone()]);
                let lhs_register = lhs.map_to_data_register(None, self, scratch_variable_map, &vec![rhs_register.to_mapper_location()]);
                let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
                self.push_instruction(Instr::GEU { lhs: lhs_register, rhs: rhs_register, dest: dest_register });
                dest_register.map_to_location(potential_target, self, scratch_variable_map)
            }
        }
    }

    fn gen_i32_ges(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        match (lhs, rhs) {
            (MapperLocation::Immediate(imm), rhs) if (imm.as_i32() + 1) >> 8 == 0 || (imm.as_i32() + 1) >> 8 == -1 => {
                let immediate = imm.as_i32();
                let rhs_register = rhs.map_to_data_register(None, self, scratch_variable_map, &vec![]);
                let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
                self.push_instruction(Instr::LT { lhs: rhs_register, rhs: RegisterOrConst::new_const((immediate + 1) as u16), dest: dest_register });
                dest_register.map_to_location(potential_target, self, scratch_variable_map)
            },
            (lhs, rhs) => {
                let rhs_register = rhs.map_to_register_or_const(SignValue::Signed, self, scratch_variable_map, &vec![lhs.clone()]);
                let lhs_register = lhs.map_to_data_register(None, self, scratch_variable_map, &vec![rhs_register.to_mapper_location()]);
                let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
                self.push_instruction(Instr::GE { lhs: lhs_register, rhs: rhs_register, dest: dest_register });
                dest_register.map_to_location(potential_target, self, scratch_variable_map)
            }
        }
    }

    fn gen_i32_ltu(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        match (lhs, rhs) {
            (MapperLocation::Immediate(imm), rhs) if (imm.as_u32() + 1) >> 9 == 0 => {
                let immediate = imm.as_u32();
                let rhs_register = rhs.map_to_data_register(None, self, scratch_variable_map, &vec![]);
                let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
                self.push_instruction(Instr::GEU { lhs: rhs_register, rhs: RegisterOrConst::new_const((immediate + 1) as u16), dest: dest_register });
                dest_register.map_to_location(potential_target, self, scratch_variable_map)
            },
            (lhs, rhs) => {
                let rhs_register = rhs.map_to_register_or_const(SignValue::Unsigned, self, scratch_variable_map, &vec![lhs.clone()]);
                let lhs_register = lhs.map_to_data_register(None, self, scratch_variable_map, &vec![rhs_register.to_mapper_location()]);
                let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
                self.push_instruction(Instr::LTU { lhs: lhs_register, rhs: rhs_register, dest: dest_register });
                dest_register.map_to_location(potential_target, self, scratch_variable_map)
            }
        }
    }

    fn gen_i32_lts(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        match (lhs, rhs) {
            (MapperLocation::Immediate(imm), rhs) if (imm.as_i32() + 1) >> 8 == 0 || (imm.as_i32() + 1) >> 8 == -1 => {
                let immediate = imm.as_i32();
                let rhs_register = rhs.map_to_data_register(None, self, scratch_variable_map, &vec![]);
                let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
                self.push_instruction(Instr::GE { lhs: rhs_register, rhs: RegisterOrConst::new_const((immediate + 1) as u16), dest: dest_register });
                dest_register.map_to_location(potential_target, self, scratch_variable_map)
            },
            (lhs, rhs) => {
                let rhs_register = rhs.map_to_register_or_const(SignValue::Signed, self, scratch_variable_map, &vec![lhs.clone()]);
                let lhs_register = lhs.map_to_data_register(None, self, scratch_variable_map, &vec![rhs_register.to_mapper_location()]);
                let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
                self.push_instruction(Instr::LT { lhs: lhs_register, rhs: rhs_register, dest: dest_register });
                dest_register.map_to_location(potential_target, self, scratch_variable_map)
            }
        }
    }

    fn gen_i64_lts(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_extended_registers(self, scratch_variable_map);
        let intermediate = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![MapperLocation::ExtendedRegister(lhs_register), MapperLocation::ExtendedRegister(rhs_register)]);
        self.push_instruction(Instr::EQ { lhs: lhs_register.upper_half(), rhs: RegisterOrConst::DataRegister(rhs_register.upper_half()), dest: intermediate });
        self.push_instruction(Instr::ANDLTU { lhs: lhs_register.lower_half(), rhs: RegisterOrConst::DataRegister(rhs_register.lower_half()), dest: intermediate });
        self.push_instruction(Instr::ORLT { lhs: lhs_register.upper_half(), rhs: RegisterOrConst::DataRegister(rhs_register.upper_half()), dest: intermediate });
        intermediate.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i64_ltu(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_extended_registers(self, scratch_variable_map);
        let intermediate = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![MapperLocation::ExtendedRegister(lhs_register), MapperLocation::ExtendedRegister(rhs_register)]);
        self.push_instruction(Instr::EQ { lhs: lhs_register.upper_half(), rhs: RegisterOrConst::DataRegister(rhs_register.upper_half()), dest: intermediate });
        self.push_instruction(Instr::ANDLTU { lhs: lhs_register.lower_half(), rhs: RegisterOrConst::DataRegister(rhs_register.lower_half()), dest: intermediate });
        self.push_instruction(Instr::ORLTU { lhs: lhs_register.upper_half(), rhs: RegisterOrConst::DataRegister(rhs_register.upper_half()), dest: intermediate });
        intermediate.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i64_gts(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_extended_registers(self, scratch_variable_map);
        let intermediate = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![MapperLocation::ExtendedRegister(lhs_register), MapperLocation::ExtendedRegister(rhs_register)]);
        self.push_instruction(Instr::EQ { lhs: lhs_register.upper_half(), rhs: RegisterOrConst::DataRegister(rhs_register.upper_half()), dest: intermediate });
        self.push_instruction(Instr::ANDLT { lhs: rhs_register.lower_half(), rhs: RegisterOrConst::DataRegister(lhs_register.lower_half()), dest: intermediate });
        self.push_instruction(Instr::ORLT { lhs: rhs_register.upper_half(), rhs: RegisterOrConst::DataRegister(lhs_register.upper_half()), dest: intermediate });
        intermediate.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i64_gtu(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_extended_registers(self, scratch_variable_map);
        let intermediate = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![MapperLocation::ExtendedRegister(lhs_register), MapperLocation::ExtendedRegister(rhs_register)]);
        self.push_instruction(Instr::EQ { lhs: lhs_register.upper_half(), rhs: RegisterOrConst::DataRegister(rhs_register.upper_half()), dest: intermediate });
        self.push_instruction(Instr::ANDLTU { lhs: rhs_register.lower_half(), rhs: RegisterOrConst::DataRegister(lhs_register.lower_half()), dest: intermediate });
        self.push_instruction(Instr::ORLTU { lhs: rhs_register.upper_half(), rhs: RegisterOrConst::DataRegister(lhs_register.upper_half()), dest: intermediate });
        intermediate.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i64_ges(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_extended_registers(self, scratch_variable_map);
        let intermediate = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![MapperLocation::ExtendedRegister(lhs_register), MapperLocation::ExtendedRegister(rhs_register)]);
        self.push_instruction(Instr::EQ { lhs: lhs_register.upper_half(), rhs: RegisterOrConst::DataRegister(rhs_register.upper_half()), dest: intermediate });
        self.push_instruction(Instr::ANDGE { lhs: lhs_register.lower_half(), rhs: RegisterOrConst::DataRegister(rhs_register.lower_half()), dest: intermediate });
        self.push_instruction(Instr::ORLT { lhs: rhs_register.upper_half(), rhs: RegisterOrConst::DataRegister(lhs_register.upper_half()), dest: intermediate });
        intermediate.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i64_geu(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_extended_registers(self, scratch_variable_map);
        let intermediate = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![MapperLocation::ExtendedRegister(lhs_register), MapperLocation::ExtendedRegister(rhs_register)]);
        self.push_instruction(Instr::EQ { lhs: lhs_register.upper_half(), rhs: RegisterOrConst::DataRegister(rhs_register.upper_half()), dest: intermediate });
        self.push_instruction(Instr::ANDGEU { lhs: lhs_register.lower_half(), rhs: RegisterOrConst::DataRegister(rhs_register.lower_half()), dest: intermediate });
        self.push_instruction(Instr::ORLTU { lhs: rhs_register.upper_half(), rhs: RegisterOrConst::DataRegister(lhs_register.upper_half()), dest: intermediate });
        intermediate.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i64_les(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_extended_registers(self, scratch_variable_map);
        let intermediate = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![MapperLocation::ExtendedRegister(lhs_register), MapperLocation::ExtendedRegister(rhs_register)]);
        self.push_instruction(Instr::EQ { lhs: lhs_register.upper_half(), rhs: RegisterOrConst::DataRegister(rhs_register.upper_half()), dest: intermediate });
        self.push_instruction(Instr::ANDGE { lhs: rhs_register.lower_half(), rhs: RegisterOrConst::DataRegister(lhs_register.lower_half()), dest: intermediate });
        self.push_instruction(Instr::ORLT { lhs: lhs_register.upper_half(), rhs: RegisterOrConst::DataRegister(rhs_register.upper_half()), dest: intermediate });
        intermediate.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i64_leu(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register) = (lhs, rhs).map_to_extended_registers(self, scratch_variable_map);
        let intermediate = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![MapperLocation::ExtendedRegister(lhs_register), MapperLocation::ExtendedRegister(rhs_register)]);
        self.push_instruction(Instr::EQ { lhs: lhs_register.upper_half(), rhs: RegisterOrConst::DataRegister(rhs_register.upper_half()), dest: intermediate });
        self.push_instruction(Instr::ANDGEU { lhs: rhs_register.lower_half(), rhs: RegisterOrConst::DataRegister(lhs_register.lower_half()), dest: intermediate });
        self.push_instruction(Instr::ORLTU { lhs: lhs_register.upper_half(), rhs: RegisterOrConst::DataRegister(rhs_register.upper_half()), dest: intermediate });
        intermediate.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_ne(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register_const) = (lhs, rhs).map_abelian_children_to_register_or_const(SignValue::Signed, self, scratch_variable_map);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::NE { lhs: lhs_register, rhs: rhs_register_const, dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_eq(&mut self, lhs: &MapperLocation, rhs: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let (lhs_register, rhs_register_const) = (lhs, rhs).map_abelian_children_to_register_or_const(SignValue::Signed, self, scratch_variable_map);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::EQ { lhs: lhs_register, rhs: rhs_register_const, dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

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
        let ExtendedRegister(index) = self.get_dest_extended_register(potential_target, scratch_variable_map, &vec![]);
        child.map_to_data_register(Some(DataRegister(index)), self, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::MOV { src: RegisterOrLargeConst::new_const(0), dest: Register::DataRegister(DataRegister(index + 1)) });
        ExtendedRegister(index).map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_f64_neg(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let ExtendedRegister(src_index) = child.map_to_extended_register(None, self, scratch_variable_map, &vec![]);
        let lower_src_register = DataRegister(src_index);
        let upper_src_register = DataRegister(src_index + 1);
        let ExtendedRegister(index) = self.get_dest_extended_register(potential_target, scratch_variable_map, &vec![]);
        let lower_dest_register = DataRegister(index);
        let upper_dest_register = DataRegister(index + 1);
        self.push_instruction(Instr::ADDIH { lhs: upper_src_register, rhs: Const16(0x8000), dest: upper_dest_register });
        lower_src_register.map_to_location(Some(&MapperLocation::DataRegister(lower_dest_register)), self, scratch_variable_map);
        ExtendedRegister(index).map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_f64_abs(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let ExtendedRegister(src_index) = child.map_to_extended_register(None, self, scratch_variable_map, &vec![]);
        let lower_src_register = DataRegister(src_index);
        let upper_src_register = DataRegister(src_index + 1);
        let ExtendedRegister(index) = self.get_dest_extended_register(potential_target, scratch_variable_map, &vec![]);
        let lower_dest_register = DataRegister(index);
        let upper_dest_register = DataRegister(index + 1);
        self.push_instruction(Instr::SH { src: upper_src_register, count: RegisterOrConst::new_const(1), dest: upper_dest_register });
        self.push_instruction(Instr::SH { src: upper_dest_register, count: RegisterOrConst::new_const(-1i16 as u16), dest: upper_dest_register });
        lower_src_register.map_to_location(Some(&MapperLocation::DataRegister(lower_dest_register)), self, scratch_variable_map);
        ExtendedRegister(index).map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i64_popcnt(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let ExtendedRegister(src_index) = child.map_to_extended_register(None, self, scratch_variable_map, &vec![]);
        let lower_src_register = DataRegister(src_index);
        let upper_src_register = DataRegister(src_index + 1);
        let ExtendedRegister(index) = self.get_dest_extended_register(potential_target, scratch_variable_map, &vec![]);
        let lower_dest_register = DataRegister(index);
        let upper_dest_register = DataRegister(index + 1);
        self.push_instruction(Instr::POPCNT { src: lower_src_register, dest: lower_dest_register });
        self.push_instruction(Instr::POPCNT { src: upper_src_register, dest: upper_dest_register });
        self.push_instruction(Instr::ADD { lhs: lower_dest_register, rhs: RegisterOrConst::DataRegister(upper_dest_register), dest: lower_dest_register });
        self.push_instruction(Instr::MOV { src: RegisterOrLargeConst::new_const(0), dest: Register::DataRegister(upper_dest_register) });
        ExtendedRegister(index).map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_f32_convert_i32u(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let child_register: DataRegister = child.map_to_data_register(None, self, scratch_variable_map, &vec![]);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::UTOF { src: child_register, dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_f32_convert_i32s(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let child_register = child.map_to_data_register(None, self, scratch_variable_map, &vec![]);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::ITOF { src: child_register, dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_trunc_f32u(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let child_register: DataRegister = child.map_to_data_register(None, self, scratch_variable_map, &vec![]);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::FTOUZ { src: child_register, dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_trunc_f32s(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let child_register: DataRegister = child.map_to_data_register(None, self, scratch_variable_map, &vec![]);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::FTOIZ { src: child_register, dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_eqz(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let child_register = child.map_to_data_register(None, self, scratch_variable_map, &vec![]);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::EQ { lhs: child_register, rhs: RegisterOrConst::new_const(0), dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
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

    fn gen_i32_popcnt(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let child_register = child.map_to_data_register(None, self, scratch_variable_map, &vec![]);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::POPCNT { src: child_register, dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_ctz(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let child_register = child.map_to_data_register(None, self, scratch_variable_map, &vec![]);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::SHUFFLE { src: child_register, dest: dest_register, mask: Const9::new(0x11B) });
        self.push_instruction(Instr::CLZ { src: dest_register, dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn gen_i32_clz(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let child_register = child.map_to_data_register(None, self, scratch_variable_map, &vec![]);
        let dest_register = self.get_dest_data_register(potential_target, scratch_variable_map, &vec![]);
        self.push_instruction(Instr::CLZ { src: child_register, dest: dest_register });
        dest_register.map_to_location(potential_target, self, scratch_variable_map)
    }

    fn _gen_i64_clz(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let ExtendedRegister(src_index) = child.map_to_extended_register(None, self, scratch_variable_map, &vec![]);
        let lower_src_register = DataRegister(src_index);
        let upper_src_register = DataRegister(src_index + 1);
        let ExtendedRegister(index) = self.get_dest_extended_register(potential_target, scratch_variable_map, &vec![]);
        let lower_dest_register = DataRegister(index);
        let upper_dest_register = DataRegister(index + 1);
        self.push_instruction(Instr::CLZ {src: lower_src_register, dest: lower_dest_register});
        self.push_instruction(Instr::CLZ {src: upper_src_register, dest: upper_dest_register});
        self.push_instruction(Instr::ADDI {lhs: upper_dest_register , rhs: Const16(-32i16 as u16), dest: upper_dest_register});
        self.push_instruction(Instr::CADDN { lhs: lower_dest_register , rhs: RegisterOrConst::DataRegister(upper_dest_register), cond: upper_dest_register, dest: upper_dest_register });
        self.push_instruction(Instr::ADDI {lhs: upper_dest_register , rhs: Const16(32i16 as u16), dest: upper_dest_register});
        self.push_instruction(Instr::MOV {src: RegisterOrLargeConst::DataRegister(upper_dest_register), dest: Register::DataRegister(lower_dest_register)});
        self.push_instruction(Instr::MOV {src: RegisterOrLargeConst::new_const(0), dest: Register::DataRegister(upper_dest_register)});
        lower_dest_register.map_to_location(Some(&MapperLocation::ExtendedRegister(ExtendedRegister(index))), self, scratch_variable_map);
        ExtendedRegister(index).map_to_location(potential_target, self, scratch_variable_map)
    }

    fn _gen_i64_ctz(&mut self, child: &MapperLocation, scratch_variable_map: &mut Vec<MapperLocation>, potential_target: Option<&MapperLocation>) -> MapperLocation {
        let ExtendedRegister(src_index) = child.map_to_extended_register(None, self, scratch_variable_map, &vec![]);
        let lower_src_register = DataRegister(src_index);
        let upper_src_register = DataRegister(src_index + 1);
        let ExtendedRegister(index) = self.get_dest_extended_register(potential_target, scratch_variable_map, &vec![]);
        let lower_dest_register = DataRegister(index);
        let upper_dest_register = DataRegister(index + 1);
        self.push_instruction(Instr::SHUFFLE { src: lower_src_register, dest: lower_dest_register, mask: Const9::new(0x11B)});
        self.push_instruction(Instr::CLZ {src: lower_dest_register, dest: lower_dest_register});
        self.push_instruction(Instr::SHUFFLE { src: upper_src_register, dest: upper_dest_register, mask: Const9::new(0x11B)});
        self.push_instruction(Instr::CLZ {src: upper_dest_register, dest: upper_dest_register});
        self.push_instruction(Instr::ADDI {lhs: lower_dest_register , rhs: Const16(-32i16 as u16), dest: lower_dest_register});
        self.push_instruction(Instr::CADDN { lhs: upper_dest_register , rhs: RegisterOrConst::DataRegister(lower_dest_register), cond: lower_dest_register, dest: lower_dest_register });
        self.push_instruction(Instr::ADDI {lhs: lower_dest_register , rhs: Const16(32i16 as u16), dest: lower_dest_register});
        self.push_instruction(Instr::MOV {src: RegisterOrLargeConst::new_const(0), dest: Register::DataRegister(upper_dest_register)});
        lower_dest_register.map_to_location(Some(&MapperLocation::DataRegister(upper_dest_register)), self, scratch_variable_map);
        ExtendedRegister(index).map_to_location(potential_target, self, scratch_variable_map)
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