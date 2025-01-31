use alloc::vec;
use alloc::boxed::Box;
use wasmparser::{BlockType, BrTable, Ieee32, Ieee64, MemArg, ValType, VisitOperator};


use crate::isa_model::{self, Const10, DataRegister, ExtendedRegister, RegisterOrConst, RegisterOrSmallConst, GLOBAL_BASE, STACK_BASE, STACK_POINTER};
use crate::parse_and_translate::WasmRuntime;
use crate::vb::{Address, AtomicVB, BinaryVB, UnaryVB, VB};
use crate::translator::{BlockLabel, BlockTypes, Translator};

use crate::isa_model::{Const4, Const16, AddressRegister, TABLE_BASE, machine_instructions::Instr, Register, ValueSize, Memsize, SignValue, MapperLocation};

macro_rules! _visit_only_mvp {
     // delegate the macro invocation to sub-invocations of this macro to
    // deal with each instruction on a case-by-case basis.
    ($( @$proposal:ident $op:ident $({ $($arg:ident: $argty:ty),* })? => $visit:ident)*) => {
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

pub trait VisitOperatorMvp <'a> {
    fn visit_unreachable(&mut self);
    fn visit_nop(&mut self);
    fn visit_block(&mut self,blockty:BlockType);
    fn visit_loop(&mut self,blockty:BlockType);
    fn visit_if(&mut self,blockty:BlockType);
    fn visit_else(&mut self);
    fn visit_end(&mut self);
    fn visit_br(&mut self,relative_depth:u32);
    fn visit_br_if(&mut self,relative_depth:u32);
    fn visit_br_table(&mut self,targets:BrTable<'a>);
    fn visit_return(&mut self);
    fn visit_call(&mut self,function_index:u32);
    fn visit_call_indirect(&mut self,type_index:u32,table_index:u32);
    fn visit_drop(&mut self);
    fn visit_select(&mut self);
    fn visit_local_get(&mut self,local_index:u32);
    fn visit_local_set(&mut self,local_index:u32);
    fn visit_local_tee(&mut self,local_index:u32);
    fn visit_global_get(&mut self,global_index:u32);
    fn visit_global_set(&mut self,global_index:u32);
    fn visit_i32_load(&mut self,memarg:MemArg);
    fn visit_i64_load(&mut self,memarg:MemArg);
    fn visit_f32_load(&mut self,memarg:MemArg);
    fn visit_f64_load(&mut self,memarg:MemArg);
    fn visit_i32_load8_s(&mut self,memarg:MemArg);
    fn visit_i32_load8_u(&mut self,memarg:MemArg);
    fn visit_i32_load16_s(&mut self,memarg:MemArg);
    fn visit_i32_load16_u(&mut self,memarg:MemArg);
    fn visit_i64_load8_s(&mut self,memarg:MemArg);
    fn visit_i64_load8_u(&mut self,memarg:MemArg);
    fn visit_i64_load16_s(&mut self,memarg:MemArg);
    fn visit_i64_load16_u(&mut self,memarg:MemArg);
    fn visit_i64_load32_s(&mut self,memarg:MemArg);
    fn visit_i64_load32_u(&mut self,memarg:MemArg);
    fn visit_i32_store(&mut self,memarg:MemArg);
    fn visit_i64_store(&mut self,memarg:MemArg);
    fn visit_f32_store(&mut self,memarg:MemArg);
    fn visit_f64_store(&mut self,memarg:MemArg);
    fn visit_i32_store8(&mut self,memarg:MemArg);
    fn visit_i32_store16(&mut self,memarg:MemArg);
    fn visit_i64_store8(&mut self,memarg:MemArg);
    fn visit_i64_store16(&mut self,memarg:MemArg);
    fn visit_i64_store32(&mut self,memarg:MemArg);
    fn visit_memory_size(&mut self,mem:u32);
    fn visit_memory_grow(&mut self,mem:u32);
    fn visit_i32_const(&mut self,value:i32);
    fn visit_i64_const(&mut self,value:i64);
    fn visit_f32_const(&mut self,value:Ieee32);
    fn visit_f64_const(&mut self,value:Ieee64);
    fn visit_i32_eqz(&mut self);
    fn visit_i32_eq(&mut self);
    fn visit_i32_ne(&mut self);
    fn visit_i32_lt_s(&mut self);
    fn visit_i32_lt_u(&mut self);
    fn visit_i32_gt_s(&mut self);
    fn visit_i32_gt_u(&mut self);
    fn visit_i32_le_s(&mut self);
    fn visit_i32_le_u(&mut self);
    fn visit_i32_ge_s(&mut self);
    fn visit_i32_ge_u(&mut self);
    fn visit_i64_eqz(&mut self);
    fn visit_i64_eq(&mut self);
    fn visit_i64_ne(&mut self);
    fn visit_i64_lt_s(&mut self);
    fn visit_i64_lt_u(&mut self);
    fn visit_i64_gt_s(&mut self);
    fn visit_i64_gt_u(&mut self);
    fn visit_i64_le_s(&mut self);
    fn visit_i64_le_u(&mut self);
    fn visit_i64_ge_s(&mut self);
    fn visit_i64_ge_u(&mut self);
    fn visit_f32_eq(&mut self);
    fn visit_f32_ne(&mut self);
    fn visit_f32_lt(&mut self);
    fn visit_f32_gt(&mut self);
    fn visit_f32_le(&mut self);
    fn visit_f32_ge(&mut self);
    fn visit_f64_eq(&mut self);
    fn visit_f64_ne(&mut self);
    fn visit_f64_lt(&mut self);
    fn visit_f64_gt(&mut self);
    fn visit_f64_le(&mut self);
    fn visit_f64_ge(&mut self);
    fn visit_i32_clz(&mut self);
    fn visit_i32_ctz(&mut self);
    fn visit_i32_popcnt(&mut self);
    fn visit_i32_add(&mut self);
    fn visit_i32_sub(&mut self);
    fn visit_i32_mul(&mut self);
    fn visit_i32_div_s(&mut self);
    fn visit_i32_div_u(&mut self);
    fn visit_i32_rem_s(&mut self);
    fn visit_i32_rem_u(&mut self);
    fn visit_i32_and(&mut self);
    fn visit_i32_or(&mut self);
    fn visit_i32_xor(&mut self);
    fn visit_i32_shl(&mut self);
    fn visit_i32_shr_s(&mut self);
    fn visit_i32_shr_u(&mut self);
    fn visit_i32_rotl(&mut self);
    fn visit_i32_rotr(&mut self);
    fn visit_i64_clz(&mut self);
    fn visit_i64_ctz(&mut self);
    fn visit_i64_popcnt(&mut self);
    fn visit_i64_add(&mut self);
    fn visit_i64_sub(&mut self);
    fn visit_i64_mul(&mut self);
    fn visit_i64_div_s(&mut self);
    fn visit_i64_div_u(&mut self);
    fn visit_i64_rem_s(&mut self);
    fn visit_i64_rem_u(&mut self);
    fn visit_i64_and(&mut self);
    fn visit_i64_or(&mut self);
    fn visit_i64_xor(&mut self);
    fn visit_i64_shl(&mut self);
    fn visit_i64_shr_s(&mut self);
    fn visit_i64_shr_u(&mut self);
    fn visit_i64_rotl(&mut self);
    fn visit_i64_rotr(&mut self);
    fn visit_f32_abs(&mut self);
    fn visit_f32_neg(&mut self);
    fn visit_f32_ceil(&mut self);
    fn visit_f32_floor(&mut self);
    fn visit_f32_trunc(&mut self);
    fn visit_f32_nearest(&mut self);
    fn visit_f32_sqrt(&mut self);
    fn visit_f32_add(&mut self);
    fn visit_f32_sub(&mut self);
    fn visit_f32_mul(&mut self);
    fn visit_f32_div(&mut self);
    fn visit_f32_min(&mut self);
    fn visit_f32_max(&mut self);
    fn visit_f32_copysign(&mut self);
    fn visit_f64_abs(&mut self);
    fn visit_f64_neg(&mut self);
    fn visit_f64_ceil(&mut self);
    fn visit_f64_floor(&mut self);
    fn visit_f64_trunc(&mut self);
    fn visit_f64_nearest(&mut self);
    fn visit_f64_sqrt(&mut self);
    fn visit_f64_add(&mut self);
    fn visit_f64_sub(&mut self);
    fn visit_f64_mul(&mut self);
    fn visit_f64_div(&mut self);
    fn visit_f64_min(&mut self);
    fn visit_f64_max(&mut self);
    fn visit_f64_copysign(&mut self);
    fn visit_i32_wrap_i64(&mut self);
    fn visit_i32_trunc_f32_s(&mut self);
    fn visit_i32_trunc_f32_u(&mut self);
    fn visit_i32_trunc_f64_s(&mut self);
    fn visit_i32_trunc_f64_u(&mut self);
    fn visit_i64_extend_i32_s(&mut self);
    fn visit_i64_extend_i32_u(&mut self);
    fn visit_i64_trunc_f32_s(&mut self);
    fn visit_i64_trunc_f32_u(&mut self);
    fn visit_i64_trunc_f64_s(&mut self);
    fn visit_i64_trunc_f64_u(&mut self);
    fn visit_f32_convert_i32_s(&mut self);
    fn visit_f32_convert_i32_u(&mut self);
    fn visit_f32_convert_i64_s(&mut self);
    fn visit_f32_convert_i64_u(&mut self);
    fn visit_f32_demote_f64(&mut self);
    fn visit_f64_convert_i32_s(&mut self);
    fn visit_f64_convert_i32_u(&mut self);
    fn visit_f64_convert_i64_s(&mut self);
    fn visit_f64_convert_i64_u(&mut self);
    fn visit_f64_promote_f32(&mut self);
    fn visit_i32_reinterpret_f32(&mut self);
    fn visit_i64_reinterpret_f64(&mut self);
    fn visit_f32_reinterpret_i32(&mut self);
    fn visit_f64_reinterpret_i64(&mut self);
}

enum BlockStyle {
    Block,
    Loop
}
impl<'a,'b> Translator<'a,'b> {
    fn enter_block(&mut self, blockty: BlockType, block_style: BlockStyle) {

        let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
        self.dead_code_flag_stack.push(dead_code_flag);

        if dead_code_flag {
            return;
        }
        
        let blockty_value_size = match blockty {
            BlockType::Empty => None,
            BlockType::Type(ty) => Some(val_type_size(&ty)),
            BlockType::FuncType(type_index) => {
                let funtype = self.wasm_runtime.types[type_index as usize].unwrap_func();
                 funtype.results().get(0).map(|ty| val_type_size(ty))
            }
        };
        
        if blockty_value_size.is_some(){
            self.resolve_all();
        }

        let runtime_stack_offset = {
            let mut stack_offset = 0;
            for vb in self.vb_stack.iter().rev() {
                let offset = vb.get_runtime_stack_offset();
                if offset.is_some() {
                    stack_offset = offset.unwrap();
                    break;
                }
            }
            stack_offset
        };

        let  blockty_value_byte_size = blockty_value_size.map(|size| size.as_bytes()).unwrap_or(0);
    

        let block_types = BlockTypes {
            block_type: (runtime_stack_offset + blockty_value_byte_size as usize , blockty_value_size),
            label_type: match block_style {
                BlockStyle::Block => (runtime_stack_offset + blockty_value_byte_size as usize, blockty_value_size),
                BlockStyle::Loop => (runtime_stack_offset, None),
            }
        };

        self.cfg_block_type_stack.push(block_types);

        self.cfg_label_stack.push(BlockLabel::Block(self.cfg_label_map.len()));

        let br_label_target = match block_style {
            BlockStyle::Block => None,
            BlockStyle::Loop => Some(self.wasm_runtime.instructions_count),
        };
        self.cfg_label_map.push(br_label_target);

        self.vb_stack_ptr_stack.push(self.vb_stack.len());
        
    }

    fn resolve_block_result(&mut self, target : (usize, Option<ValueSize>) ) {
        let (offset, size) = target;
        let result_register = size.map(|size| {
            let initial_location = self.resolve_with_target(None);
            match size {
                ValueSize::Word => Register::DataRegister(initial_location.map_to_data_register(None, self, &mut vec![], &vec![])),
                ValueSize::DoubleWord => Register::ExtendedRegister(initial_location.map_to_extended_register(None, self, &mut vec![], &vec![])),
            }

        });
        
        let current_stack_offset = {
            let mut stack_offset = 0;
            for vb in self.vb_stack.iter().rev() {
                let offset = vb.get_runtime_stack_offset();
                if offset.is_some() {
                    stack_offset = offset.unwrap();
                    break;
                }
            }
            stack_offset
        };

        match size {
            None  if current_stack_offset != offset => {
                self.push_instruction(Instr::LEA { base: STACK_BASE, offset: Const16(-(offset as i16) as u16), dest: STACK_POINTER });
            },
            Some(_) => match result_register {
                Some(Register::DataRegister(result_register)) => self.push_instruction(Instr::STWPI{ base: STACK_POINTER, offset: Const10(-(offset as i16)+ (current_stack_offset as i16)), src: result_register }),
                Some(Register::ExtendedRegister(result_register)) => self.push_instruction(Instr::STDPI{ base: STACK_POINTER, offset: Const10(-(offset as i16) + (current_stack_offset as i16)), src: result_register }),
                None => panic!("Expected result register")
            },
            _ => ()
        }
    }

    fn resolve_return_value(&mut self) {
        let result_type = self.wasm_runtime.types[self.function_type_index as usize].unwrap_func().results().get(0);
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
    
    fn store_value_in_memory(&mut self, memarg: MemArg, mem_size: Memsize, val_size: ValueSize, align: u8) {
        let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
        if dead_code_flag {
            return;
        }
        
        if self.vb_stack.iter().any(VB::depends_on_memory) {
            let value_vb = self.vb_stack.pop().unwrap();
            let dynamic_offset_vb = self.vb_stack.pop().unwrap();
            self.resolve_all();
            self.vb_stack.push(dynamic_offset_vb);
            self.vb_stack.push(value_vb);
        }

        let value_location = self.resolve_with_target(None);

        let value_register: Register = match val_size{
            ValueSize::Word => Register::DataRegister(value_location.map_to_data_register(None, self, &mut vec![], &vec![])),
            ValueSize::DoubleWord => Register::ExtendedRegister(value_location.map_to_extended_register(None, self, &mut vec![], &vec![])),
        };
        
        self.locked_register = Some(value_register.clone());
        let dynamic_offset = self.resolve_with_target(None);
        self.locked_register = None;
        value_register.map_to_location(Some(&MapperLocation::LinearMemory { static_offset: memarg.offset as usize, src_size: mem_size, dynamic_offset: Some(Box::new(dynamic_offset)), align, ext_sign: SignValue::Signed }),  self, &mut vec![]);
    }

}

#[allow(unused_variables)]
impl <'a,'b> VisitOperator <'a> for Translator<'a,'b>{
    
    type Output = ();
    
    wasmparser::for_each_operator!(_visit_only_mvp);

    fn visit_unreachable(&mut self) {
        let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
        if dead_code_flag {
            return;
        }

        self.push_instruction(Instr::Trap);
        self.dead_code_flag_stack.last_mut().map(|flag| *flag = true);
        self.vb_stack.truncate(*(self.vb_stack_ptr_stack.last()).unwrap());
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

        let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
        if dead_code_flag {
            self.dead_code_flag_stack.push(true);
            return;
        }


        let condition_vb = self.vb_stack.pop().unwrap();
        self.enter_block(blockty, BlockStyle::Block);
        self.vb_stack.push(condition_vb);

        let condition_register = self.resolve_with_target(None).map_to_data_register(None, self, &mut vec![], &vec![]);

        let else_label = self.cfg_label_map.len();
        self.push_instruction(Instr::JEQ {target: else_label, lhs: condition_register, rhs: RegisterOrSmallConst::new_const(0)});
        self.cfg_label_stack.last_mut().map(|label|{
            let end_label = match label {
                BlockLabel::Block(index) => index,
                _ => panic!("Expected block label")
            };
            *label = BlockLabel::If { else_label, end_label: *end_label };

        } );
        self.cfg_label_map.push(None);

    }

    fn visit_else(&mut self){ 
        let inside_dead_code_flag = self.dead_code_flag_stack.pop().unwrap_or(false);

        let outside_dead_flag_code = *self.dead_code_flag_stack.last().unwrap_or(&false);
        self.dead_code_flag_stack.push(outside_dead_flag_code);

        if outside_dead_flag_code {
            return;
        }

        if !inside_dead_code_flag {
        self.resolve_block_result(self.cfg_block_type_stack.last().unwrap().block_type);
        }

        let (else_label, end_label) = match self.cfg_label_stack.pop().unwrap() {
            BlockLabel::If { else_label, end_label } => (else_label, end_label),
            _ => panic!("Expected if label")
        };

        if!inside_dead_code_flag {
            self.push_instruction(Instr::J {target: end_label});
        }

        self.cfg_label_map[else_label] = Some(self.wasm_runtime.instructions_count);

        self.cfg_label_stack.push(BlockLabel::Block(end_label));
    }

    fn visit_end(&mut self){
        
        let inside_dead_code_flag = self.dead_code_flag_stack.pop().unwrap_or(false);

        let outside_dead_flag_code = *self.dead_code_flag_stack.last().unwrap_or(&false);
        if outside_dead_flag_code {
            return;
        }

        let block_type = self.cfg_block_type_stack.pop().map(|block_types| block_types.block_type);

        if !inside_dead_code_flag {
            match block_type{
                Some(block_type) => self.resolve_block_result(block_type),
                None => self.resolve_return_value(),
            }

            if let None = block_type{
                self.push_instruction(Instr::RET);
            }
        }

        self.cfg_label_stack.pop().map(|block_label| {
            let label_indices = match block_label {
                BlockLabel::Block(index) => vec![index],
                BlockLabel::If { else_label, end_label } => vec![else_label, end_label],
            };

            label_indices.iter().for_each(|&label_index| {

            let label = &mut self.cfg_label_map[label_index];
            if let None = label {
                *label = Some(self.wasm_runtime.instructions_count);
            } 
        });
        });

        self.vb_stack_ptr_stack.pop();

        block_type.map(|(offset,size)| {
            size.map(|size| {
                self.add_atomic_vb(AtomicVB::Resolved{size, offset});
            });
        });

    }

    fn visit_br(&mut self,relative_depth:u32){
        let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
        if dead_code_flag {
            return;
        }

        let index = self.cfg_block_type_stack.len() as i32 - (1 + relative_depth as i32);
        if index < 0 {
            self.resolve_return_value();
            self.push_instruction(Instr::RET);
        } else {
            let block_type = self.cfg_block_type_stack.get(index as usize).map(|block_types| block_types.label_type).unwrap();
            self.resolve_block_result(block_type);

            let target =  match self.cfg_label_stack[index as usize] {
                BlockLabel::Block(index) => index,
                BlockLabel::If { else_label: _, end_label } => end_label,
            };
    
            match self.cfg_label_map[target] {
                None => self.push_instruction(Instr::J {target}),
                Some(..) => self.push_instruction(Instr::LOOPU {target}),
            }
           
        };

        self.dead_code_flag_stack.last_mut().map(|flag| *flag = true);
        self.vb_stack.truncate(*(self.vb_stack_ptr_stack.last()).unwrap());

    }

    fn visit_br_if(&mut self,relative_depth:u32){

        let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
        if dead_code_flag {
            return;
        }
        
        let condition_register = self.resolve_with_target(None).map_to_data_register(None, self, &mut vec![], &vec![]);

        let index = self.cfg_block_type_stack.len() as i32 - (1 + relative_depth as i32);

        self.push_instruction(Instr::JEQ { target: self.cfg_label_map.len(), lhs: condition_register, rhs: RegisterOrSmallConst::new_const(0) });
        if index < 0 {
            let return_type = self.wasm_runtime.types[self.function_type_index as usize].unwrap_func().results().get(0);
            let last_vb = match return_type {
                Some(..) => self.vb_stack.last().cloned(),
                None => None,
            };
            self.resolve_return_value();
            self.push_instruction(Instr::RET);
            last_vb.map(|vb| self.vb_stack.push(vb));
        } else {
            let block_type = self.cfg_block_type_stack.get(index as usize).map(|block_types| block_types.label_type).unwrap();
            let last_vb = match block_type.1 {
                Some(..) => self.vb_stack.last().cloned(),
                None => None,
            };
            self.resolve_block_result(block_type);
            last_vb.map(|vb| self.vb_stack.push(vb));

            let target =  match self.cfg_label_stack[index as usize] {
                BlockLabel::Block(index) => index,
                BlockLabel::If { else_label: _, end_label } => end_label,
            };
            
            match self.cfg_label_map[target] {
                None => self.push_instruction(Instr::J {target}),
                Some(..) => self.push_instruction(Instr::LOOPU {target}),
            }
        };
        self.cfg_label_map.push(Some(self.wasm_runtime.instructions_count));
    }

    fn visit_br_table(&mut self,targets:BrTable<'a>) {

        let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
        if dead_code_flag {
            return;
        }

        let offset_vb = self.vb_stack.pop().unwrap();
        self.resolve_all();
        self.vb_stack.push(offset_vb);

        let offset_register = self.resolve_with_target(None).map_to_data_register(None, self, &mut vec![], &vec![]);

        targets.targets().enumerate().map(|(offset,target)| (Some(offset), target.unwrap()) ).chain(vec![(None,targets.default())].into_iter()).for_each(|(offset,target)| { 
            offset.map(|offset| self.push_instruction(Instr::JNE { target: self.cfg_label_map.len(), lhs: offset_register, rhs: Const4(offset as u8) }));
            let relative_index = target;
            let index = self.cfg_block_type_stack.len() as i32 - (1 + relative_index as i32);
            if index < 0 {
                let return_type = self.wasm_runtime.types[self.function_type_index as usize].unwrap_func().results().get(0);
            let last_vb = match return_type {
                Some(..) => self.vb_stack.last().cloned(),
                None => None,
            };
            self.resolve_return_value();
            self.push_instruction(Instr::RET);
            last_vb.map(|vb| self.vb_stack.push(vb));
            } else {
                let block_type = self.cfg_block_type_stack.get(index as usize).map(|block_types| block_types.label_type).unwrap();
                let last_vb = match block_type.1 {
                    Some(..) => self.vb_stack.last().cloned(),
                    None => None,
                };
                self.resolve_block_result(block_type);
                last_vb.map(|vb| self.vb_stack.push(vb));

    
                let target =  match self.cfg_label_stack[index as usize] {
                    BlockLabel::Block(index) => index,
                    BlockLabel::If { else_label: _, end_label } => end_label,
                };

                match self.cfg_label_map[target] {
                    None => self.push_instruction(Instr::J {target}),
                    Some(..) => self.push_instruction(Instr::LOOPU {target}),
                }
            };
            self.cfg_label_map.push(Some(self.wasm_runtime.instructions_count));
        });

        self.dead_code_flag_stack.last_mut().map(|flag| *flag = true);
        self.vb_stack.truncate(*(self.vb_stack_ptr_stack.last()).unwrap());

    }

    fn visit_return(&mut self)  {
        let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
        if dead_code_flag {
            return;
        }

        self.resolve_return_value();

        self.push_instruction(Instr::RET);
        self.dead_code_flag_stack.last_mut().map(|flag| *flag = true);
        self.vb_stack.truncate(*(self.vb_stack_ptr_stack.last()).unwrap());
    }

    fn visit_call(&mut self,function_index:u32)  {
        let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
        if dead_code_flag {
            return;
        }

        let function_type = self.wasm_runtime.types[self.global_translator.function_type_map[function_index as usize] as usize].unwrap_func().clone();
        self.resolve_all();
        self.push_instruction(Instr::SVLCX);
        self.push_instruction(Instr::CALL{target: function_index});

        let params_count = function_type.params().len();
        self.vb_stack.truncate(self.vb_stack.len() - params_count);

        let params_size: u16 = function_type.params().iter().map(|ty| val_type_size(ty).as_bytes() as u16).sum();
        
        if params_size > 0 {
            self.push_instruction(Instr::LEA{base: STACK_POINTER, offset: Const16(params_size), dest: STACK_POINTER});
        }
        
        function_type.results().get(0).map(|ty|{
            let runtime_stack_offset = match self.vb_stack.last() {
                Some(VB::AtomicVB(AtomicVB::Resolved { offset, .. })) => *offset,
                _ => 0,
            };
            self.vb_stack.push(VB::AtomicVB(AtomicVB::Resolved{size: val_type_size(ty), offset: runtime_stack_offset + val_type_size(ty).as_bytes() as usize})); 
        
            match val_type_size(ty){
                ValueSize::Word => self.push_instruction(Instr::STWPI { base: STACK_POINTER, offset: Const10(-4), src: DataRegister(0) }),
                ValueSize::DoubleWord => self.push_instruction(Instr::STDPI { base: STACK_POINTER, offset: Const10(-8), src: ExtendedRegister(0) }),
            }
        });
        self.push_instruction(Instr::RSLCX);
    }

    fn visit_call_indirect(&mut self,type_index:u32,_table_index:u32, _table_byte: u8) {

        let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
        if dead_code_flag {
            return;
        }

        let function_index_vb = self.vb_stack.pop().unwrap();

        self.resolve_all();

        self.vb_stack.push(function_index_vb);

        // TODO: should we check if the operand exceeds the table length?
        let table_offset = self.resolve_with_target(None).map_to_data_register(None, self, &mut vec![], &vec![]);
        self.push_instruction(Instr::SVLCX);
        if table_offset != DataRegister(4) {
            self.push_instruction(Instr::MOV {src: isa_model::RegisterOrLargeConst::DataRegister(table_offset), dest: Register::DataRegister(DataRegister(4))});
        }
        self.push_instruction(Instr::MOVU { src: Const16::new(type_index as u16), dest: DataRegister(5) });

        let types_ptr = self.wasm_runtime.types.as_ptr() as u32;
        let types_ptr_upper = ((types_ptr+0x8000) >> 16) as u16;
        let types_ptr_lower = types_ptr as u16;
        self.push_instruction(Instr::MOVHA { src: Const16::new(types_ptr_upper), dest: AddressRegister(4) });
        self.push_instruction(Instr::LEA { base: AddressRegister(4), offset: Const16::new(types_ptr_lower), dest: AddressRegister(4) });

        let table_type_indices_ptr = self.wasm_runtime.table_type_indices.as_ptr() as u32;
        let table_type_indices_ptr_upper = ((table_type_indices_ptr+0x8000) >> 16) as u16;
        let table_type_indices_ptr_lower = table_type_indices_ptr as u16;
        self.push_instruction(Instr::MOVHA { src: Const16::new(table_type_indices_ptr_upper), dest: AddressRegister(5) });
        self.push_instruction(Instr::LEA { base: AddressRegister(5), offset: Const16::new(table_type_indices_ptr_lower), dest: AddressRegister(5) });

        let call_ptr = WasmRuntime::compare_subtypes as u32;
        let call_ptr_upper = ((call_ptr+0x8000) >> 16) as u16;
        let call_ptr_lower = call_ptr as u16;
        self.push_instruction(Instr::MOVHA { src: Const16::new(call_ptr_upper), dest: AddressRegister(2) });
        self.push_instruction(Instr::LEA { base: AddressRegister(2), offset: Const16::new(call_ptr_lower), dest: AddressRegister(2) });
        self.push_instruction(Instr::CALLI{target: AddressRegister(2)});
        self.push_instruction(Instr::RSLCX);

        self.push_instruction(Instr::ADDSCA { lhs: TABLE_BASE, rhs: table_offset, dest: AddressRegister(2), shift: Const4::new(2) });
        self.push_instruction(Instr::LDA { base: AddressRegister(2), offset: Const16::new(0), dest: AddressRegister(2) });

        let function_type = self.wasm_runtime.types[type_index as usize].unwrap_func().clone(); 

        self.push_instruction(Instr::SVLCX);
        self.push_instruction(Instr::CALLI{target: AddressRegister(2)});

        let params_count = function_type.params().len();
        
        self.vb_stack.truncate(self.vb_stack.len() - params_count);

        let params_size: u16 = function_type.params().iter().map(|ty| val_type_size(ty).as_bytes() as u16).sum();
        
        if params_size > 0 {
            self.push_instruction(Instr::LEA{base: STACK_POINTER, offset: Const16(params_size), dest: STACK_POINTER});
        }

        function_type.results().get(0).map(|ty|{
            let runtime_stack_offset = match self.vb_stack.last() {
                Some(VB::AtomicVB(AtomicVB::Resolved { offset, .. })) => *offset,
                _ => 0,
            };
            self.vb_stack.push(VB::AtomicVB(AtomicVB::Resolved{size: val_type_size(ty), offset: runtime_stack_offset + val_type_size(ty).as_bytes() as usize})); 
        
            match val_type_size(ty){
                ValueSize::Word => self.push_instruction(Instr::STWPI { base: STACK_POINTER, offset: Const10(-4), src: DataRegister(0) }),
                ValueSize::DoubleWord => self.push_instruction(Instr::STDPI { base: STACK_POINTER, offset: Const10(-8), src: ExtendedRegister(0) }),
            }
        });
        self.push_instruction(Instr::RSLCX);

    }

    fn visit_drop(&mut self) {
        let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
        if dead_code_flag {
            return;
        }

        let vb = self.vb_stack.pop().unwrap();
        match vb{
            VB::AtomicVB(AtomicVB::Resolved{size, ..}) => {
                let byte_offset = match size{
                    ValueSize::Word => 4,
                    ValueSize::DoubleWord => 8
                };
                self.push_instruction(Instr::LEA{base: STACK_POINTER, offset: Const16(byte_offset), dest: STACK_POINTER});
            },
            _ => ()
        };

    }

    fn visit_select(&mut self) {
        let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
        if dead_code_flag {
            return;
        }
       let selector = self.vb_stack.pop().unwrap();
       let rhs = self.vb_stack.pop().unwrap();
       let lhs = self.vb_stack.pop().unwrap();
       let size = lhs.val_size(&self.locals_map, &self.global_translator.globals_map);
       self.vb_stack.push(VB::Select {selector: Box::new(selector), lhs: Box::new(lhs), rhs: Box::new(rhs), size});
    }

    fn visit_local_get(&mut self,local_index:u32)  {
        self.add_atomic_vb(AtomicVB::Local {index: local_index})
    }

    fn visit_local_set(&mut self,local_index:u32) {
        let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
        if dead_code_flag {
            return;
        }
        let vb: VB = self.vb_stack.pop().unwrap();
        if self.vb_stack.iter().any(|vb| vb.depends_on_local(local_index)) {
            self.resolve_all();
        }
        self.vb_stack.push(vb);
        self.resolve_with_target(Some(&self.locals_map[local_index as usize].clone()));
    }

    fn visit_local_tee(&mut self,local_index:u32) {
        let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
        if dead_code_flag {
            return;
        }
        let vb = self.vb_stack.pop().unwrap();
        if self.vb_stack.iter().any(|vb| vb.depends_on_local(local_index)) {
            self.resolve_all();
        }
        self.vb_stack.push(vb);
        self.resolve_with_target(Some(&self.locals_map[local_index as usize].clone()));
        self.add_atomic_vb(AtomicVB::Local {index: local_index});
    }

    fn visit_global_get(&mut self,global_index:u32) {
        self.add_atomic_vb(AtomicVB::Global {index: global_index})
    }

    fn visit_global_set(&mut self,global_index:u32) {
        let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
        if dead_code_flag {
            return;
        }
        let (offset, size) =self.global_translator.globals_map[global_index as usize];
        self.resolve_with_target(Some(&MapperLocation::Global { offset, size }));
    }

    fn visit_i32_load(&mut self,memarg:MemArg) {
        self.add_unary_vb(UnaryVB::I32Load { offset: memarg.offset as Address, align: memarg.align })
    }

    fn visit_i64_load(&mut self,memarg:MemArg) {
        self.add_unary_vb(UnaryVB::I64Load { offset: memarg.offset as Address, align: memarg.align })
    }

    fn visit_f32_load(&mut self,memarg:MemArg) {
        self.add_unary_vb(UnaryVB::F32Load { offset: memarg.offset as Address, align: memarg.align })
    }

    fn visit_f64_load(&mut self,memarg:MemArg) {
        self.add_unary_vb(UnaryVB::F64Load { offset: memarg.offset as Address, align: memarg.align })
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
       self.store_value_in_memory(memarg, Memsize::Word, ValueSize::Word, memarg.align);
    }

    fn visit_i64_store(&mut self,memarg:MemArg) {
        self.store_value_in_memory(memarg, Memsize::DoubleWord, ValueSize::DoubleWord, memarg.align);
    }

    fn visit_f32_store(&mut self,memarg:MemArg) {
        self.store_value_in_memory(memarg, Memsize::Word, ValueSize::Word, memarg.align);
    }

    fn visit_f64_store(&mut self,memarg:MemArg) {
        self.store_value_in_memory(memarg, Memsize::DoubleWord, ValueSize::DoubleWord, memarg.align);
    }

    fn visit_i32_store8(&mut self,memarg:MemArg) {
        self.store_value_in_memory(memarg, Memsize::Byte, ValueSize::Word, memarg.align);
    }

    fn visit_i32_store16(&mut self,memarg:MemArg) {
        self.store_value_in_memory(memarg, Memsize::HalfWord, ValueSize::Word, memarg.align);
    }

    fn visit_i64_store8(&mut self,memarg:MemArg) {
        self.store_value_in_memory(memarg, Memsize::Byte, ValueSize::DoubleWord, memarg.align);
    }

    fn visit_i64_store16(&mut self,memarg:MemArg) {
        self.store_value_in_memory(memarg, Memsize::HalfWord, ValueSize::DoubleWord, memarg.align);
    }

    fn visit_i64_store32(&mut self,memarg:MemArg) {
        self.store_value_in_memory(memarg, Memsize::Word, ValueSize::DoubleWord, memarg.align);
    }

    fn visit_memory_size(&mut self,_mem:u32, _mem_byte: u8 ) {
        let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
        if dead_code_flag {
            return;
        }
        
        self.add_atomic_vb(AtomicVB::MemorySize);
    }

    fn visit_memory_grow(&mut self,_mem:u32, _mem_byte: u8 ) {
        let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
        if dead_code_flag {
            return;
        }

        let offset_vb = self.vb_stack.pop().unwrap();
        self.resolve_all();
        self.vb_stack.push(offset_vb);
        let grow_offset_register = self.resolve_with_target(None).map_to_data_register(None, self, &mut vec![], &vec![]);
        let current_memory_size_register = self.next_available_data_register(&mut vec![MapperLocation::DataRegister(grow_offset_register)], &mut vec![]);
        self.push_instruction(Instr::LDW{base: GLOBAL_BASE, offset: Const16(0), dest: current_memory_size_register});
        self.push_instruction(Instr::STWPI { src: current_memory_size_register, base: STACK_POINTER, offset: Const10(-4) });
        self.push_instruction(Instr::ADD{lhs: current_memory_size_register, rhs: RegisterOrConst::DataRegister(grow_offset_register), dest: current_memory_size_register});
        self.push_instruction(Instr::LT{lhs: current_memory_size_register, rhs: RegisterOrConst::Const9(isa_model::Const9(self.global_translator.memory_size_limit as u16 + 1)), dest: grow_offset_register});
        self.push_instruction(Instr::JEQ { target: self.cfg_label_map.len(), lhs: grow_offset_register, rhs: RegisterOrSmallConst::new_const(0) });
        self.push_instruction(Instr::STW{base: GLOBAL_BASE, offset: Const16(0), src: current_memory_size_register});
        self.push_instruction(Instr::J{target: self.cfg_label_map.len()+1});
        self.cfg_label_map.push(Some(self.wasm_runtime.instructions_count));
        self.push_instruction(Instr::MOV { src: isa_model::RegisterOrLargeConst::Const16(isa_model::Const16(0xffff)), dest: isa_model::Register::DataRegister(current_memory_size_register) });
        self.push_instruction(Instr::STW { src: current_memory_size_register, base: STACK_POINTER, offset: Const16(0) });
        self.cfg_label_map.push(Some(self.wasm_runtime.instructions_count));
        let offset = match self.vb_stack.last(){
            Some(VB::AtomicVB(AtomicVB::Resolved { offset, .. })) => *offset+4,
            None => 0,
            _ => panic!("Expected resolved atomic value")
        };
        self.add_atomic_vb(AtomicVB::Resolved{size: ValueSize::Word, offset});
    }

    fn visit_i32_const(&mut self,value:i32) {
        self.add_atomic_vb(AtomicVB::I32Const { imm: value })
    }

    fn visit_i64_const(&mut self,value:i64) {
        self.add_atomic_vb(AtomicVB::I64Const { imm: value })
    }

    fn visit_f32_const(&mut self,value:Ieee32) {
        self.add_atomic_vb(AtomicVB::F32Const { imm: value.bits() })
    }

    fn visit_f64_const(&mut self,value:Ieee64) {
        self.add_atomic_vb(AtomicVB::F64Const { imm: value.bits() })
    }

    fn visit_i32_eqz(&mut self) {
        self.add_unary_vb(UnaryVB::I32EqZ)
    }

    fn visit_i32_eq(&mut self) {
        self.add_binary_vb(BinaryVB::I32Eq)
    }

    fn visit_i32_ne(&mut self) {
        self.add_binary_vb(BinaryVB::I32Ne)
    }

    fn visit_i32_lt_s(&mut self) {
        self.add_binary_vb(BinaryVB::I32LtS)
    }

    fn visit_i32_lt_u(&mut self) {
        self.add_binary_vb(BinaryVB::I32LtU)
    }

    fn visit_i32_gt_s(&mut self) {
        self.add_binary_vb(BinaryVB::I32GtS)
    }

    fn visit_i32_gt_u(&mut self) {
        self.add_binary_vb(BinaryVB::I32GtU)
    }

    fn visit_i32_le_s(&mut self) {
        self.add_binary_vb(BinaryVB::I32LeS)
    }

    fn visit_i32_le_u(&mut self) {
        self.add_binary_vb(BinaryVB::I32LeU)
    }

    fn visit_i32_ge_s(&mut self) {
        self.add_binary_vb(BinaryVB::I32GeS)
    }

    fn visit_i32_ge_u(&mut self) {
        self.add_binary_vb(BinaryVB::I32GeU)
    }

    fn visit_i64_eqz(&mut self) {
        self.add_unary_vb(UnaryVB::I64EqZ)
    }

    fn visit_i64_eq(&mut self) {
        self.add_binary_vb(BinaryVB::I64Eq)
    }

    fn visit_i64_ne(&mut self) {
        self.add_binary_vb(BinaryVB::I64Ne)
    }

    fn visit_i64_lt_s(&mut self) {
        self.add_binary_vb(BinaryVB::I64LtS)
    }

    fn visit_i64_lt_u(&mut self) {
        self.add_binary_vb(BinaryVB::I64LtU)
    }

    fn visit_i64_gt_s(&mut self) {
        self.add_binary_vb(BinaryVB::I64GtS)
    }

    fn visit_i64_gt_u(&mut self) {
        self.add_binary_vb(BinaryVB::I64GtU)
    }

    fn visit_i64_le_s(&mut self) {
        self.add_binary_vb(BinaryVB::I64LeS)
    }

    fn visit_i64_le_u(&mut self) {
        self.add_binary_vb(BinaryVB::I64LeU)
    }

    fn visit_i64_ge_s(&mut self) {
        self.add_binary_vb(BinaryVB::I64GeS)
    }

    fn visit_i64_ge_u(&mut self) {
        self.add_binary_vb(BinaryVB::I64GeU)
    }

    fn visit_f32_eq(&mut self) {
        self.add_binary_vb(BinaryVB::F32Eq)
    }

    fn visit_f32_ne(&mut self) {
        self.add_binary_vb(BinaryVB::F32Ne)
    }

    fn visit_f32_lt(&mut self) {
        self.add_binary_vb(BinaryVB::F32Lt)
    }

    fn visit_f32_gt(&mut self) {
        self.add_binary_vb(BinaryVB::F32Gt)
    }

    fn visit_f32_le(&mut self) {
        self.add_binary_vb(BinaryVB::F32Le)
    }

    fn visit_f32_ge(&mut self) {
        self.add_binary_vb(BinaryVB::F32Ge)
    }

    fn visit_f64_eq(&mut self) {
        self.add_binary_vb(BinaryVB::F64Eq)
    }

    fn visit_f64_ne(&mut self) {
        self.add_binary_vb(BinaryVB::F64Ne)
    }

    fn visit_f64_lt(&mut self) {
        self.add_binary_vb(BinaryVB::F64Lt)
    }

    fn visit_f64_gt(&mut self) {
        self.add_binary_vb(BinaryVB::F64Gt)
    }

    fn visit_f64_le(&mut self) {
        self.add_binary_vb(BinaryVB::F64Le)
    }

    fn visit_f64_ge(&mut self) {
       self.add_binary_vb(BinaryVB::F64Ge)
    }

    fn visit_i32_clz(&mut self) {
        self.add_unary_vb(UnaryVB::I32Clz)
    }

    fn visit_i32_ctz(&mut self) {
        self.add_unary_vb(UnaryVB::I32Ctz)
    }

    fn visit_i32_popcnt(&mut self) {
        self.add_unary_vb(UnaryVB::I32PopCnt)
    }

    fn visit_i32_add(&mut self) {
        self.add_binary_vb(BinaryVB::I32Add)
    }

    fn visit_i32_sub(&mut self) {
        self.add_binary_vb(BinaryVB::I32Sub)
    }

    fn visit_i32_mul(&mut self) {
        self.add_binary_vb(BinaryVB::I32Mul)
    }

    fn visit_i32_div_s(&mut self) {
        self.add_binary_vb(BinaryVB::I32DivS)
    }

    fn visit_i32_div_u(&mut self) {
        self.add_binary_vb(BinaryVB::I32DivU)
    }

    fn visit_i32_rem_s(&mut self) {
        self.add_binary_vb(BinaryVB::I32RemS)
    }

    fn visit_i32_rem_u(&mut self) {
        self.add_binary_vb(BinaryVB::I32RemU)
    }

    fn visit_i32_and(&mut self) {
        self.add_binary_vb(BinaryVB::I32And)
    }

    fn visit_i32_or(&mut self) {
        self.add_binary_vb(BinaryVB::I32Or)
    }

    fn visit_i32_xor(&mut self) {
        self.add_binary_vb(BinaryVB::I32Xor)
    }

    fn visit_i32_shl(&mut self) {
        self.add_binary_vb(BinaryVB::I32Shl)
    }

    fn visit_i32_shr_s(&mut self) {
        self.add_binary_vb(BinaryVB::I32ShrS)
    }

    fn visit_i32_shr_u(&mut self) {
        self.add_binary_vb(BinaryVB::I32ShrU)
    }

    fn visit_i32_rotl(&mut self) {
        self.add_binary_vb(BinaryVB::I32Rotl)
    }

    fn visit_i32_rotr(&mut self) {
        self.add_binary_vb(BinaryVB::I32Rotr)
    }

    fn visit_i64_clz(&mut self) {
        self.add_unary_vb(UnaryVB::I64Clz)
    }

    fn visit_i64_ctz(&mut self) {
        self.add_unary_vb(UnaryVB::I64Ctz)
    }

    fn visit_i64_popcnt(&mut self) {
        self.add_unary_vb(UnaryVB::I64PopCnt)
    }

    fn visit_i64_add(&mut self) {
        self.add_binary_vb(BinaryVB::I64Add)
    }

    fn visit_i64_sub(&mut self) {
        self.add_binary_vb(BinaryVB::I64Sub)
    }

    fn visit_i64_mul(&mut self) {
        self.add_binary_vb(BinaryVB::I64Mul)
    }

    fn visit_i64_div_s(&mut self) {
        self.add_binary_vb(BinaryVB::I64DivS)
    }

    fn visit_i64_div_u(&mut self) {
        self.add_binary_vb(BinaryVB::I64DivU)
    }

    fn visit_i64_rem_s(&mut self) {
        self.add_binary_vb(BinaryVB::I64RemS)
    }

    fn visit_i64_rem_u(&mut self) {
        self.add_binary_vb(BinaryVB::I64RemU)
    }

    fn visit_i64_and(&mut self) {
        self.add_binary_vb(BinaryVB::I64And)
    }

    fn visit_i64_or(&mut self) {
        self.add_binary_vb(BinaryVB::I64Or)
    }

    fn visit_i64_xor(&mut self) {
        self.add_binary_vb(BinaryVB::I64Xor)
    }

    fn visit_i64_shl(&mut self) {
        self.add_binary_vb(BinaryVB::I64Shl)
    }

    fn visit_i64_shr_s(&mut self) {
        self.add_binary_vb(BinaryVB::I64ShrS)
    }

    fn visit_i64_shr_u(&mut self) {
        self.add_binary_vb(BinaryVB::I64ShrU)
    }

    fn visit_i64_rotl(&mut self) {
        self.add_binary_vb(BinaryVB::I64Rotl)
    }

    fn visit_i64_rotr(&mut self) {
        self.add_binary_vb(BinaryVB::I64Rotr)
    }

    fn visit_f32_abs(&mut self) {
       self.add_unary_vb(UnaryVB::F32Abs)
    }

    fn visit_f32_neg(&mut self) {
        self.add_unary_vb(UnaryVB::F32Neg)
    }

    fn visit_f32_ceil(&mut self) {
        self.add_unary_vb(UnaryVB::F32Ceil)
    }

    fn visit_f32_floor(&mut self) {
        self.add_unary_vb(UnaryVB::F32Floor)
    }

    fn visit_f32_trunc(&mut self) {
        self.add_unary_vb(UnaryVB::F32Trunc)
    }

    fn visit_f32_nearest(&mut self) {
        self.add_unary_vb(UnaryVB::F32Nearest)
    }

    fn visit_f32_sqrt(&mut self) {
        self.add_unary_vb(UnaryVB::F32Sqrt)
    }

    fn visit_f32_add(&mut self) {
        self.add_binary_vb(BinaryVB::F32Add)
    }

    fn visit_f32_sub(&mut self) {
        self.add_binary_vb(BinaryVB::F32Sub)
    }

    fn visit_f32_mul(&mut self) {
        self.add_binary_vb(BinaryVB::F32Mul)
    }

    fn visit_f32_div(&mut self) {
        self.add_binary_vb(BinaryVB::F32Div)
    }

    fn visit_f32_min(&mut self) {
        self.add_binary_vb(BinaryVB::F32Min)
    }

    fn visit_f32_max(&mut self) {
        self.add_binary_vb(BinaryVB::F32Max)
    }

    fn visit_f32_copysign(&mut self) {
        self.add_binary_vb(BinaryVB::F32CopySign)
    }

    fn visit_f64_abs(&mut self) {
        self.add_unary_vb(UnaryVB::F64Abs)
    }

    fn visit_f64_neg(&mut self) {
        self.add_unary_vb(UnaryVB::F64Neg)
    }

    fn visit_f64_ceil(&mut self) {
        self.add_unary_vb(UnaryVB::F64Ceil)
    }

    fn visit_f64_floor(&mut self) {
        self.add_unary_vb(UnaryVB::F64Floor)
    }

    fn visit_f64_trunc(&mut self) {
        self.add_unary_vb(UnaryVB::F64Trunc)
    }

    fn visit_f64_nearest(&mut self) {
        self.add_unary_vb(UnaryVB::F64Nearest)
    }

    fn visit_f64_sqrt(&mut self) {
        self.add_unary_vb(UnaryVB::F64Sqrt)
    }

    fn visit_f64_add(&mut self) {
        self.add_binary_vb(BinaryVB::F64Add)
    }

    fn visit_f64_sub(&mut self) {
        self.add_binary_vb(BinaryVB::F64Sub)
    }

    fn visit_f64_mul(&mut self) {
        self.add_binary_vb(BinaryVB::F64Mul)
    }

    fn visit_f64_div(&mut self) {
        self.add_binary_vb(BinaryVB::F64Div)
    }

    fn visit_f64_min(&mut self) {
        self.add_binary_vb(BinaryVB::F64Min)
    }

    fn visit_f64_max(&mut self) {
        self.add_binary_vb(BinaryVB::F64Max)
    }

    fn visit_f64_copysign(&mut self) {
        self.add_binary_vb(BinaryVB::F64CopySign)
    }

    fn visit_i32_wrap_i64(&mut self) {
        self.add_unary_vb(UnaryVB::I32WrapI64)
    }

    fn visit_i32_trunc_f32_s(&mut self) {
        self.add_unary_vb(UnaryVB::I32TruncF32S)
    }

    fn visit_i32_trunc_f32_u(&mut self) {
        self.add_unary_vb(UnaryVB::I32TruncF32U)
    }

    fn visit_i32_trunc_f64_s(&mut self) {
        self.add_unary_vb(UnaryVB::I32TruncF64S)
    }

    fn visit_i32_trunc_f64_u(&mut self) {
        self.add_unary_vb(UnaryVB::I32TruncF64U)
    }

    fn visit_i64_extend_i32_s(&mut self) {
        self.add_unary_vb(UnaryVB::I64ExtendI32S)
    }

    fn visit_i64_extend_i32_u(&mut self) {
        self.add_unary_vb(UnaryVB::I64ExtendI32U)
    }

    fn visit_i64_trunc_f32_s(&mut self) {
        self.add_unary_vb(UnaryVB::I64TruncF32S)
    }

    fn visit_i64_trunc_f32_u(&mut self) {
        self.add_unary_vb(UnaryVB::I64TruncF32U)
    }

    fn visit_i64_trunc_f64_s(&mut self) {
        self.add_unary_vb(UnaryVB::I64TruncF64S)
    }

    fn visit_i64_trunc_f64_u(&mut self) {
        self.add_unary_vb(UnaryVB::I64TruncF64U)
    }

    fn visit_f32_convert_i32_s(&mut self) {
        self.add_unary_vb(UnaryVB::F32ConvertI32S)
    }

    fn visit_f32_convert_i32_u(&mut self) {
        self.add_unary_vb(UnaryVB::F32ConvertI32U)
    }

    fn visit_f32_convert_i64_s(&mut self) {
        self.add_unary_vb(UnaryVB::F32ConvertI64S)
    }

    fn visit_f32_convert_i64_u(&mut self) {
        self.add_unary_vb(UnaryVB::F32ConvertI64U)
    }

    fn visit_f32_demote_f64(&mut self) {
        self.add_unary_vb(UnaryVB::F32DemoteF64)
    }

    fn visit_f64_convert_i32_s(&mut self) {
        self.add_unary_vb(UnaryVB::F64ConvertI32S)
    }

    fn visit_f64_convert_i32_u(&mut self) {
        self.add_unary_vb(UnaryVB::F64ConvertI32U)
    }

    fn visit_f64_convert_i64_s(&mut self) {
        self.add_unary_vb(UnaryVB::F64ConvertI64S)
    }

    fn visit_f64_convert_i64_u(&mut self) {
        self.add_unary_vb(UnaryVB::F64ConvertI64U)
    }

    fn visit_f64_promote_f32(&mut self) {
        self.add_unary_vb(UnaryVB::F64PromoteF32)
    }

    fn visit_i32_reinterpret_f32(&mut self) {
       let mut vb = self.vb_stack.pop().unwrap();
       if vb.produces_non_canonical_nan() {vb = vb.adjust_for_snan();}
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
