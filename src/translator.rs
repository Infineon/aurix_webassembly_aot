mod vb_resolution;
mod register_allocation;
mod library_function;
mod visit_operator;
mod linear_memory_access;

use alloc::vec::Vec;
use alloc::vec;
use alloc::boxed::Box;
use wasmparser::LocalsReader;
use crate::{isa_model::{self, machine_instructions::Instr, Const10, Const16, DataRegister, ExtendedRegister, MapperLocation, Register, RegisterOrLargeConst, ValueSize, FRAME_POINTER, GLOBAL_BASE, STACK_BASE, STACK_POINTER}, parse_and_translate::WasmRuntime, vb::{AtomicVB, BinaryVB, UnaryVB, VB}};
use crate::parse_and_translate::GlobalTranslator;

const MAX_LOCAL_REGISTERS:u8 = 8;
const MAX_ALL_REGISTERS: u8 = 16;

pub enum BlockLabel {
    Block(usize),
    If { else_label: usize, end_label: usize },
}

pub struct BlockTypes {
    pub block_type: (usize, Option<ValueSize>),
    pub label_type: (usize, Option<ValueSize>),
}

pub struct Translator<'a,'b> {
    pub vb_stack:Vec<VB>,
    pub locals_map:Vec<MapperLocation>,
    pub locked_register: Option<Register>,
    pub cfg_label_stack: Vec<BlockLabel>,
    pub cfg_block_type_stack:Vec<BlockTypes>,
    pub vb_stack_ptr_stack: Vec<usize>,
    pub dead_code_flag_stack: Vec<bool>,
    pub function_type_index: u32,
    pub global_translator: &'a mut GlobalTranslator,
    pub wasm_runtime: &'a mut WasmRuntime<'b>,
    pub cfg_label_map: Vec<Option<usize>>,
    pub cfg_jobs: Vec<usize>,
}



impl<'a,'b> Translator<'a,'b> {
    
    /// Creates a new Translator instance. Translating a WebAssembly function requires a Translator instance. This function generates the machine code of the prologue for the function as well.
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
   pub fn new(type_index:u32, locals_reader: LocalsReader, global_translator: &'a mut GlobalTranslator, wasm_runtime: &'a mut WasmRuntime<'b>) -> Self {
       wasm_runtime.add_instruction(Instr::LEA { base: STACK_POINTER, offset: Const16(0), dest: FRAME_POINTER });

       let function_type = wasm_runtime.types[type_index as usize].unwrap_func();
       let locals_size: Vec<ValueSize> = function_type.params().iter().map(ValueSize::from_valtype).collect();
        
        let locals_offset :Vec<i16> = {
        if locals_size.len() == 0 {
            vec![]
        }
        else {
        let mut locals_offset_inner: Vec<i16> = Vec::new();
        for v in locals_size.iter().skip(1) {
            locals_offset_inner.push(v.as_bytes() as i16);
        }
        locals_offset_inner.push(0);
        if locals_offset_inner.len() > 1 {
            for i in (0..locals_offset_inner.len()-2).rev() {
                locals_offset_inner[i] += locals_offset_inner[i+1]; 
            }
        }
          locals_offset_inner
    }
        };

        let mut alloc_register_index:u8 = 0;
        let mut missed_register: Option<u8> = None;
        let mut locals_map = Vec::new();

        for i in 0..locals_offset.len() {
            match locals_size[i] {
                ValueSize::Word => {
                    match missed_register {
                        None if alloc_register_index < MAX_LOCAL_REGISTERS  => {
                            locals_map.push(MapperLocation::DataRegister(DataRegister(alloc_register_index)));
                            wasm_runtime.add_instruction(Instr::LDW{dest: DataRegister(alloc_register_index), base: FRAME_POINTER, offset: Const16(locals_offset[i] as u16)});
                            alloc_register_index += 1;
                        },
                        Some(register_index) => {
                            locals_map.push(MapperLocation::DataRegister(DataRegister(register_index)));
                            wasm_runtime.add_instruction(Instr::LDW{dest: DataRegister(register_index), base: FRAME_POINTER, offset: Const16(locals_offset[i] as u16)});
                            missed_register = None;
                        },
                        _ => {
                            locals_map.push(MapperLocation::Frame { size: ValueSize::Word, offset: locals_offset[i] });
                        }
                    }
                },
                ValueSize::DoubleWord => {
                    if alloc_register_index %2 == 1 {
                        missed_register = Some(alloc_register_index);
                        alloc_register_index += 1;
                    }
                    if alloc_register_index < MAX_LOCAL_REGISTERS {
                        locals_map.push(MapperLocation::ExtendedRegister(ExtendedRegister(alloc_register_index)));
                        wasm_runtime.add_instruction(Instr::LDD{dest: ExtendedRegister(alloc_register_index), base: FRAME_POINTER, offset: Const10(locals_offset[i])});
                        alloc_register_index += 2;
                    } else {
                        locals_map.push(MapperLocation::Frame { size: ValueSize::DoubleWord, offset: locals_offset[i] });
                    }
                }
            }
        }

       wasm_runtime.add_instruction(Instr::MOV{src: RegisterOrLargeConst::Const16(isa_model::Const16(0)), dest: isa_model::Register::ExtendedRegister(ExtendedRegister(14))});

        let mut stack_pointer_offset = 0;
        for (count,val_type) in locals_reader.into_iter().map(Result::unwrap) {
            for _ in 0..count {
                match ValueSize::from_valtype(&val_type) {
                    ValueSize::Word => match missed_register {
                        None if alloc_register_index < MAX_LOCAL_REGISTERS => {
                            locals_map.push(MapperLocation::DataRegister(DataRegister(alloc_register_index)));
                            wasm_runtime.add_instruction(Instr::MOV { src: RegisterOrLargeConst::Const16(isa_model::Const16(0)), dest: isa_model::Register::DataRegister(DataRegister(alloc_register_index)) });
                            alloc_register_index += 1;
                        },
                        Some(register_index) => {
                            locals_map.push(MapperLocation::DataRegister(DataRegister(register_index)));
                            wasm_runtime.add_instruction(Instr::MOV{ src: RegisterOrLargeConst::Const16(isa_model::Const16(0)), dest: isa_model::Register::DataRegister(DataRegister(register_index)) });
                            missed_register = None;
                        },
                        _ => {
                            stack_pointer_offset -= 4;
                            locals_map.push(MapperLocation::Frame { size: ValueSize::Word, offset: stack_pointer_offset });
                            wasm_runtime.add_instruction(Instr::STW { src: DataRegister(15), base: FRAME_POINTER, offset: Const16(stack_pointer_offset as u16) });
                        }
                    },
                    ValueSize::DoubleWord => {
                        if alloc_register_index % 2 == 1 {
                            missed_register = Some(alloc_register_index);
                            alloc_register_index += 1;
                        }
                        if alloc_register_index < MAX_LOCAL_REGISTERS {
                            locals_map.push(MapperLocation::ExtendedRegister(ExtendedRegister(alloc_register_index)));
                            wasm_runtime.add_instruction(Instr::MOV{ src: RegisterOrLargeConst::Const16(isa_model::Const16(0)), dest: isa_model::Register::ExtendedRegister(ExtendedRegister(alloc_register_index)) });
                            alloc_register_index += 2;
                        } else {
                            stack_pointer_offset -= 8;
                            locals_map.push(MapperLocation::Frame { size: ValueSize::DoubleWord, offset: stack_pointer_offset });
                            wasm_runtime.add_instruction(Instr::STD { src: ExtendedRegister(14), base: FRAME_POINTER, offset: Const10(stack_pointer_offset as i16) });
                        }
                    }
                }
            }

        }

       wasm_runtime.add_instruction(Instr::LEA { base: STACK_POINTER, offset: Const16(stack_pointer_offset as u16), dest: STACK_POINTER });
       wasm_runtime.add_instruction(Instr::LEA { base: STACK_POINTER, offset: Const16(0), dest: STACK_BASE });

       Translator{
           vb_stack: vec![],
           locals_map,
           cfg_label_map: vec![],
           cfg_block_type_stack:vec![],
           locked_register: None,
           cfg_label_stack: Vec::new(),
           dead_code_flag_stack: vec![false],
           function_type_index: type_index,
           vb_stack_ptr_stack: vec![0],
           global_translator,
           wasm_runtime,
           cfg_jobs: vec![],
           }
   }

    /// translates a machine instruction to binary format and adds it to the instruction array. Control flow instructions (Jumps and function calls) are marked for later processing, given that the target address is not known at this point.  
    pub fn push_instruction(&mut self, instr: Instr) {
        match instr {
            Instr::CALL{..} => self.global_translator.function_call_jobs.push(self.wasm_runtime.instructions_count),
            Instr::J {..} | Instr::JEQ {..} | Instr::JNE {..} | Instr::JZT {..} | Instr::LOOPU {..}  => self.cfg_jobs.push(self.wasm_runtime.instructions_count),
            _ => ()
        }
        self.wasm_runtime.add_instruction(instr);
    }

   pub fn add_atomic_vb (&mut self, new_vb:AtomicVB)  {
        let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
        if dead_code_flag {
            return;
        }
       self.vb_stack.push(VB::AtomicVB(new_vb));
   }

   pub fn add_unary_vb(&mut self, new_vb:UnaryVB) {
    let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
    if dead_code_flag {
        return;
    } 
       let child = self.vb_stack.pop().unwrap();
       self.vb_stack.push(VB::UnaryVB { vb: new_vb, child:Box::new(child) });
   }

   pub fn add_binary_vb(&mut self, new_vb:BinaryVB){
        let dead_code_flag = *self.dead_code_flag_stack.last().unwrap_or(&false);
        if dead_code_flag {
            return;
        }
       let rhs = self.vb_stack.pop().unwrap();
       let lhs = self.vb_stack.pop().unwrap();
       self.vb_stack.push(VB::BinaryVB { vb: new_vb, lhs: Box::new(lhs), rhs: Box::new(rhs) });
   }
    
    
   pub fn store_word_to_global(&mut self, offset:u32, src_register: DataRegister) {
       self.push_instruction(Instr::STW { src: src_register, base: GLOBAL_BASE, offset: Const16(offset as u16)})
   }

   pub fn load_word_from_global(&mut self, dest_register: DataRegister, offset:u32) {
       self.push_instruction(Instr::LDW { dest: dest_register, base: GLOBAL_BASE, offset: Const16(offset as u16) })
   }

   pub fn store_double_word_to_global(&mut self, offset:u32, src_register: ExtendedRegister) {
       self.push_instruction(Instr::STD { src: src_register, base: GLOBAL_BASE, offset: Const10(offset as i16) })
   }

    pub fn load_double_word_from_global(&mut self, dest_register: ExtendedRegister, offset:u32) {
        self.push_instruction(Instr::LDD { dest: dest_register, base: GLOBAL_BASE, offset: Const10(offset as i16) })
    }
}

