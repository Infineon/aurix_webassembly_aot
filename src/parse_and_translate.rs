use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;
use fnv::FnvBuildHasher;
use indexmap::IndexMap;
use core::{arch::asm, panic};

use wasmparser::{BinaryReaderError, Parser, Payload::*, RecGroup, SubType};

use crate::{constant_expression_eval::ConstantExpressionEval, isa_model::{Immediate, ValueSize}, translator::Translator};
use crate::isa_model::machine_instructions::Instr;


pub struct WasmRuntime <'a> {
    pub instructions: &'a mut[u32],
    pub linear_memory: &'a mut[u8],
    pub global_space: &'a mut[u8],
    pub table: &'a mut[u32],
    pub function_labels: Vec<u32>,
    pub types: Vec<SubType>,
    pub table_type_indices: Vec<u32>,
    pub export_map: IndexMap<String, u32,FnvBuildHasher>,
    pub instructions_count: usize,
} 

pub struct GlobalTranslator{
    pub function_type_map: Vec<u32>,
    pub globals_map: Vec<(u32, ValueSize)>,
    pub memory_size_limit: u32,
    pub function_call_jobs: Vec<usize>,
}

impl <'a> WasmRuntime <'a> {


    /// Creates a new `WasmRuntime` instance.
    ///
    /// # Arguments
    ///
    /// * `instructions` - A mutable reference to an array of u32 instructions.
    /// * `linear_memory` - A mutable reference to an array of u8 representing the linear memory.
    /// * `global_space` - A mutable reference to an array of u8 representing the global space.
    /// * `table` - A mutable reference to an array of u32 representing the table.
    ///
    /// # Returns
    ///
    /// A new instance of `WasmRuntime`.    
pub fn new(instructions: &'a mut[u32], linear_memory: &'a mut[u8], global_space: &'a mut[u8], table: &'a mut[u32]) -> Self {
    WasmRuntime {
        instructions,
        instructions_count: 0,
        linear_memory,
        global_space,
        table,
        function_labels: Vec::new(),
        types: Vec::new(),
        table_type_indices: Vec::new(),
        export_map: IndexMap::with_hasher( fnv::FnvBuildHasher::default()),
    }
}

    /// Swaps the label index with a displacement jump in the jump instructions. This is used to replace the CFG label, placed as a placeholder, with the actual target
    /// in the instructions after the translation of a WebAssembly function.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the instruction to modify in the instructions array.
    /// * `cfg_label_map` - The CFG label map that maps each label index to the corresponding instruction index.
pub fn swap_target_with_disp_jump(&mut self, index:usize, cfg_label_map: &Vec<Option<usize>>) {
    let instruction = self.instructions[index];
    let opcode = instruction & 0xff;
    match opcode{
        // The J instruction has a different layout for the displacement field
        0x1d => {
            let target = instruction >> 8;
            let disp=  ((cfg_label_map[target as usize].unwrap() as i32 - (index as i32))<<1) & 0xffffff;
            let disp_upper = disp as u32 >> 16 ;
            let disp_lower = disp as u32 & 0xffff;
            self.instructions[index] = 0x1d | (disp_upper << 8)| (disp_lower << 16);
        },
        _ => {
            let target = instruction >> 16 & 0x7fff;
            let disp = ((cfg_label_map[target as usize].unwrap() as i32 - (index as i32))<<1) & 0x7fff;
            self.instructions[index] = (instruction & 0x8000ffff) | ((disp as u32) << 16);
        }
    }
}
    /// Swaps the function index with a displacement in the call instructions. This is used to replace the function index, placed as a placeholder, with the actual target after the end of the translation of all WebAssembly functions. 
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the instruction to modify in the instructions array.
pub fn swap_target_with_disp_call(&mut self, index:usize) {
    let instruction = self.instructions[index];
    let target = instruction >> 8;
    let disp = ((self.function_labels[target as usize] as i32 - (index as i32))<<1) & 0xffffff;
    let disp_upper = disp as u32 >> 16 ;
    let disp_lower = disp as u32 & 0xffff;
    self.instructions[index] = 0x6d | (disp_upper << 8)| (disp_lower << 16);
}
    
  pub fn add_instruction(&mut self, instr: Instr) {
        self.instructions[self.instructions_count] = instr.map_to_binary();
        self.instructions_count += 1;
 }

    /// instantiates a new WebAssembly module into the runtime. This parses a WebAssembly binary and initializes the table, global space and linear memory. It also translates the WebAssembly functions into machine code.
    /// 
    /// # Arguments
    /// 
    /// * `wasm_code` - A slice of u8 representing the WebAssembly module in binary format.
pub fn parse_and_translate(&mut self, wasm_code: &[u8] ) -> Result<(), BinaryReaderError> {
    let parser = Parser::new(0);

    let mut global_translator = GlobalTranslator{
        function_type_map: Vec::new(),
        globals_map: Vec::new(),
        memory_size_limit: 0,
        function_call_jobs: Vec::new(),
    };
    
    let mut code_index = 0;

    let mut table_size: usize = 0;
    
    let mut start_function: Option<u32> = None;
    
    self.instructions_count = 0;
    self.function_labels.clear();
    
    
    

    for payload in parser.parse_all(wasm_code){
        match payload? {
            TypeSection(type_section_reader) => {
                self.types = type_section_reader.into_iter().map(Result::unwrap).flat_map(RecGroup::into_types).collect();
            },

            GlobalSection(global_section_reader) => {
                let globals_reader = global_section_reader.clone();
                let mut offset: usize = 4;

                for global in globals_reader.into_iter().map(Result::unwrap) {
                        let val_size = ValueSize::from_valtype(&global.ty.content_type);
                        
                        let val = ConstantExpressionEval::eval_const_expr(global.init_expr);

                        if self.global_space.len() < offset + val_size.as_bytes() as usize {
                            panic!("Global space overflow");
                        }
                        let byte_vector  = match val_size {
                            ValueSize::Word => val.as_u32().to_le_bytes().to_vec(),
                            ValueSize::DoubleWord => val.as_u64().to_le_bytes().to_vec(),
                        };

                        self.global_space[offset..offset + val_size.as_bytes() as usize].copy_from_slice(byte_vector.as_slice());
                        global_translator.globals_map.push((offset as u32, val_size));
                        offset += val_size.as_bytes() as usize;
                    }
            },

            FunctionSection(function_section_reader) => {
                global_translator.function_type_map = function_section_reader.into_iter().map(Result::unwrap).collect();
            }

            CodeSectionEntry(body) => {
                let type_index = global_translator.function_type_map[code_index];
                let locals_reader = body.get_locals_reader()?;
                let mut operators_reader = body.get_operators_reader()?;
                self.function_labels.push(self.instructions_count as u32);
                
                let mut translator = Translator::new(type_index, locals_reader, &mut global_translator, self);
                while let Ok( ..) = operators_reader.visit_operator(&mut translator){}
                
                let cfg_label_map = &translator.cfg_label_map;
                for index in translator.cfg_jobs{
                    self.swap_target_with_disp_jump(index,cfg_label_map);
                }
                code_index += 1;
            }

            TableSection(table_section_reader) => {
                match  table_section_reader.into_iter().next().map(Result::unwrap) {
                    Some(table_object) => {
                        if !table_object.ty.element_type.is_func_ref() {
                            panic!("Unexpected table element type");
                        }
                        table_size = table_object.ty.initial as usize;
                        if self.table.len() < table_size as usize {
                            panic!("Not enough table space allocated");
                        }
                        self.table_type_indices = vec![0; table_size];
                        for i in 0..table_size {
                            self.table[i] = -1i32 as u32;
                        }
                    },
                    None => {}
                }
            }

            MemorySection(memory_section_reader) => {
                let available_pages_count = self.linear_memory.len() as u32 >> 16;
               memory_section_reader.into_iter().next().map(Result::unwrap).map(
                    |memory| {
                        let memory_size_pages = memory.initial as u32;
                        global_translator.memory_size_limit = available_pages_count;
                        memory.maximum.map(|max|  if max as u32 <= available_pages_count { global_translator.memory_size_limit = max as u32 });

                        let memory_size = memory_size_pages << 16;
                        if memory_size > self.linear_memory.len() as u32 {
                            panic!("Memory size too large");
                        }

                    if self.linear_memory.len() < memory_size as usize {
                        panic!("Not enough memory allocated");
                    }

                     self.global_space[0..4].copy_from_slice(&memory_size_pages.to_le_bytes());

                    let mem_ptr = self.linear_memory.as_ptr();
                    unsafe{
                            asm!(
                            "MOV %e0, 0",
                            "LEA %a15, 0x1fff",
                            "1:",
                            "ST.D [{mem_ptr}+], 8, %e0",
                            "LOOP %a15,1b",
                            mem_ptr = in(reg_ptr) mem_ptr,
                            out("a15") _, out("e0") _
                        );
                    }
                }
            );        
        }

            DataSection(data_section_reader) => {
                data_section_reader.into_iter().map(Result::unwrap).for_each(|data_segment| {
                    let offset = match data_segment.kind {
                        wasmparser::DataKind::Active { memory_index, offset_expr } => {
                            if memory_index != 0 {
                                panic!("Unexpected memory index");
                            }
                            ConstantExpressionEval::eval_const_expr(offset_expr).as_u32() as usize
                        }
                        wasmparser::DataKind::Passive => {
                            panic!("Unexpected passive data segment");
                        }
                    };
                    let data = data_segment.data;

                    //write data to memory at offset
                    self.linear_memory[offset..(offset + data.len())].copy_from_slice(data);
                });
            }

            ElementSection(element_section_reader) => {
                element_section_reader.into_iter().map(Result::unwrap).for_each(|element_segment| {
                    let offset = match element_segment.kind {
                        wasmparser::ElementKind::Active { table_index, offset_expr } => {
                            match table_index {
                                None | Some(0) => {},
                                _ => panic!("Unexpected table index")
                            }
                            ConstantExpressionEval::eval_const_expr(offset_expr).as_u32() as usize
                        }
                        wasmparser::ElementKind::Passive => {
                            panic!("Unexpected passive element segment");
                        },
                        wasmparser::ElementKind::Declared => {
                            panic!("Unexpected declared element segment");
                        }
                    };
                    match element_segment.items {
                        wasmparser::ElementItems::Functions(functions) =>{
                        for (i, func) in functions.into_iter().map(Result::unwrap).enumerate() {
                            self.table[offset + i] = func;
                            self.table_type_indices[offset+i] =global_translator.function_type_map[func as usize];
                        }
                    },
                        wasmparser::ElementItems::Expressions(..) => panic!("Unexpected expressions in element segment"),
                    };
                    
                });
            }

            StartSection { func, .. } => {
                start_function = Some(func);
            }

            ExportSection(export_section_reader) => {
                self.export_map.clear();
                export_section_reader.into_iter().map(Result::unwrap).for_each(|export| {
                    match export.kind {
                        wasmparser::ExternalKind::Func => {
                            let func_index: u32 = export.index;
                            let func_name = export.name;
                            self.export_map.insert(func_name.to_string(), func_index);
                        }
                        _ => {}
                    }
                });
            }

            _ => {
                //println!("[WasmParser] Unhandled section: {:?}", t);
            }

        
        }
    }


    for index in global_translator.function_call_jobs.into_iter(){
        self.swap_target_with_disp_call(index);
    }
    
    for func_label in self.function_labels.iter_mut(){
        *func_label = (*func_label << 2) + (self.instructions.as_ptr() as u32);
    }

    for i in 0..table_size{
        if self.table[i] != -1i32 as u32 {
            self.table[i] = self.function_labels[self.table[i] as usize];
        }
    }

    start_function.map(|func_index| self.call_function(func_index, vec![], None));

    Ok(())
}

    /// Calls a compiled WebAssembly function by its index. This function passes the arguments, initializes the dedicated registers, calls the function and returns the result.
    /// 
    /// # Arguments
    /// 
    /// * `function_index` - The index of the function to call.
    /// * `args` - A vector of `Immediate` values representing the arguments to pass to the function. `i32` and `f32` values are passed as `Immediate::Word`, while `i64` and `f64` values are passed as `Immediate::DoubleWord`.
    /// * `return_size` - An optional `ValueSize` representing the size of the return value. If the return value is a `f32` or `i32`, the `ValueSize` is `ValueSize::Word`, while if the return value is a `f64` or `i64`, the `ValueSize` is `ValueSize::DoubleWord`.
    /// 
fn call_function(&mut self, function_index: u32, args:Vec<Immediate>, return_size: Option<ValueSize>) -> Option<Immediate> {
    let function_label = self.function_labels[function_index as usize];
    let mut arg_byte_vec = args.iter().flat_map(|arg| arg.as_word_vector()).collect::<Vec<u32>>();
    arg_byte_vec.reverse();
    let arg_bytes = arg_byte_vec.as_slice();
    let arg_bytes_ptr = arg_bytes.as_ptr();	
    let arg_len = - ((arg_bytes.len()<<2) as i32);
    let result:u64;
    let linear_memory_ptr = self.linear_memory.as_ptr() as u32;
    let global_space_ptr = self.global_space.as_ptr() as u32;
    let table_ptr = self.table.as_ptr() as u32;

    unsafe{
        asm!(
            "MOV.AA %a4, {table}",
            "MOV.AA %a5, {global_space}",
            "MOV.AA %a6, {linear_memory}",
            "MOV.AA %a15 , %a10",
            "ADDSC.A %a10, %a10, {arg_len}, 0",
            "1:",
            "JEQ.A %a10, %a15, 2f",
            "LD.W %d0, [{arg_bytes}+], 4",
            "ST.W [%a10+], 4, %d0",
            "J 1b",
            "2:",
            "ADDSC.A %a10, %a10, {arg_len}, 0",
            "CALLI {function_label}",
            "MOV.AA %a10 , %a15",
            "MOV {result}, %d1, %d0",
            result = out(reg64) result,
            arg_bytes = in(reg_ptr) arg_bytes_ptr,
            arg_len = in(reg32) arg_len,
            table = in(reg_ptr) table_ptr,
            global_space = in(reg_ptr) global_space_ptr,
            linear_memory = in(reg_ptr) linear_memory_ptr,
            function_label = in(reg_ptr) function_label,
            out("a4") _, out("a5") _, out("a6") _,  out("a15") _, out("d0") _
        );
    }
        match return_size {
            Some(ValueSize::Word) => {
                Some(Immediate::Word(result  as u32))
            },
            Some(ValueSize::DoubleWord) => {
                Some(Immediate::DoubleWord(result as u64))
            },
            None =>  None,
        }
    
    
}

    /// Calls an exported WebAssembly function by its name. This function passes the arguments.
    /// 
    /// # Arguments
    /// 
    /// * `function_name` - The name of the function to call.
    /// * `args` - A vector of `Immediate` values representing the arguments to pass to the function. `i32` and `f32` values are passed as `Immediate::Word`, while `i64` and `f64` values are passed as `Immediate::DoubleWord`.
    /// * `return_size` - An optional `ValueSize` representing the size of the return value. If the return value is a `f32` or `i32`, the `ValueSize` is `ValueSize::Word`, while if the return value is a `f64` or `i64`, the `ValueSize` is `ValueSize::DoubleWord`.
pub fn call_exported_function(&mut self, function_name: &str, args: Vec<Immediate>, return_size: Option<ValueSize>) -> Option<Immediate> {
    let function_index = self.export_map.get(function_name).unwrap();
    self.call_function(*function_index, args, return_size)
}
    
    /// Returns the size of a function in bytes. This is used for benchmarking in order to determine the size of the function in the instructions array.
    /// 
    /// # Arguments
    /// 
    /// * `function_name` - The name of the function.
    /// 
    /// # Returns
    /// 
    /// The size of the function in bytes.
pub fn get_function_size(&self, function_name: &str) -> i32 {
    let function_index = *self.export_map.get(function_name).unwrap() as usize;
    if function_index +1 < self.function_labels.len() {
        self.function_labels[function_index+1] as i32 - self.function_labels[function_index] as i32
    } else {
        ((self.instructions_count as i32) << 2) + (self.instructions.as_ptr() as i32) - (self.function_labels[function_index] as i32)
    }
}
}
