use alloc::vec;
use alloc::vec::Vec;
use core::iter;
use crate::isa_model::{Const10, DataRegister, ExtendedRegister, MapperLocation, Register, ValueSize, STACK_POINTER};
use crate::isa_model::machine_instructions::Instr;
use crate::translator::{Translator, MAX_ALL_REGISTERS, MAX_LOCAL_REGISTERS};

fn location_to_register(location: &MapperLocation) -> Option<Register> {
    match location {
        MapperLocation::DataRegister(DataRegister(index)) => Some(Register::DataRegister(DataRegister(*index))),
        MapperLocation::ExtendedRegister(ExtendedRegister(index)) => Some(Register::ExtendedRegister(ExtendedRegister(*index))),
        MapperLocation::LinearMemory {dynamic_offset, ..} => match dynamic_offset{
            None => None,
            Some(location) => location_to_register(location)
        },
        _ => None
    }
}
impl <'a,'b> Translator<'a,'b> {

    fn spill_location(&mut self, location: &mut MapperLocation, data_register_allocation_vec: &mut Vec<bool> ) -> bool {
        match location {
            MapperLocation::DataRegister(DataRegister(index)) if *index >= MAX_LOCAL_REGISTERS => {
                self.push_instruction(Instr::STWPI { src: DataRegister(*index), base: STACK_POINTER, offset: Const10(-4) });
                data_register_allocation_vec[*index as usize] = false;
                *location = MapperLocation::Stack { size: ValueSize::Word };
                true
            },
            MapperLocation::ExtendedRegister(ExtendedRegister(index))if *index >= MAX_LOCAL_REGISTERS => {
                self.push_instruction(Instr::STDPI { src: ExtendedRegister(*index), base: STACK_POINTER, offset: Const10(-8) });
                data_register_allocation_vec[*index as usize] = false;
                data_register_allocation_vec[*index as usize + 1] = false;
                *location = MapperLocation::Stack { size: ValueSize::DoubleWord };
                true
            },
            MapperLocation::LinearMemory {dynamic_offset: Some(dynamic_offset), ..} => self.spill_location(dynamic_offset, data_register_allocation_vec),
            _ => false
        }
    }

    pub(crate) fn next_available_register(&mut self, valsize : ValueSize, scratch_variable_map : &mut Vec<MapperLocation>) -> Register {

        let mut data_register_allocation_vec = vec![false; MAX_ALL_REGISTERS as usize];

        self.locals_map.iter().chain(scratch_variable_map.iter()).map(location_to_register).chain(iter::once(self.locked_register.clone())).for_each(|register| {
            match register {
                Some(Register::DataRegister(DataRegister(index))) => data_register_allocation_vec[index as usize] = true,
                Some(Register::ExtendedRegister(ExtendedRegister(index))) => {
                    data_register_allocation_vec[index as usize] = true;
                    data_register_allocation_vec[index as usize + 1] = true;
                },
                None => ()
            }
        });

        loop {
            match valsize {
                ValueSize::Word => {
                    for i in 0..MAX_ALL_REGISTERS {
                        if !data_register_allocation_vec[i as usize] {
                            return Register::DataRegister(DataRegister(i));
                        }
                    }
                },
                ValueSize::DoubleWord => {
                    for i in 0..MAX_ALL_REGISTERS {
                        if i%2==0 && !data_register_allocation_vec[i as usize] && !data_register_allocation_vec[i as usize + 1] {
                            return Register::ExtendedRegister(ExtendedRegister(i));
                        }
                    }
                }
            }

            for location in scratch_variable_map.iter_mut() {
                if self.spill_location(location, &mut data_register_allocation_vec) {
                    break;
                }
            }
        }

    }

    pub fn next_available_data_register (&mut self, scratch_variable_map : &mut Vec<MapperLocation>, used_registers: &Vec<MapperLocation>) -> DataRegister {
        scratch_variable_map.extend(used_registers.clone());
        let result = match self.next_available_register(ValueSize::Word, scratch_variable_map) {
            Register::DataRegister(data_register) => data_register,
            _ => panic!("Expected a data register")
        };
        scratch_variable_map.truncate(scratch_variable_map.len() - used_registers.len());
        result
    }


    pub fn next_available_extended_register(&mut self, scratch_variable_map : &mut Vec<MapperLocation>, used_registers: &Vec<MapperLocation>) -> ExtendedRegister {
        scratch_variable_map.extend(used_registers.clone());
        let result =match self.next_available_register(ValueSize::DoubleWord, scratch_variable_map) {
            Register::ExtendedRegister(extended_register) => extended_register,
            _ => panic!("Expected an extended register")
        };
        scratch_variable_map.truncate(scratch_variable_map.len() - used_registers.len());
        result
    }

    pub(crate) fn get_dest_data_register(&mut self, target: Option<&MapperLocation>, scratch_variable_map : &mut Vec<MapperLocation>, used_registers: &Vec<MapperLocation>) -> DataRegister {
        let mut occupied_used_registers = vec![false;16];
        for used_register in used_registers {
            match used_register {
                MapperLocation::DataRegister(DataRegister(index)) => occupied_used_registers[*index as usize] = true,
                MapperLocation::ExtendedRegister(ExtendedRegister(index)) => {
                    occupied_used_registers[*index as usize] = true;
                    occupied_used_registers[*index as usize + 1] = true;
                },
                _ => ()
            }
        }
        match target {
            Some(MapperLocation::DataRegister(DataRegister(index)) ) if !occupied_used_registers[*index as usize] => DataRegister(*index),
            _ => self.next_available_data_register(scratch_variable_map, used_registers)
        }
    }

    pub(crate) fn get_dest_extended_register(&mut self, target: Option<&MapperLocation>, scratch_variable_map : &mut Vec<MapperLocation>, used_registers: &Vec<MapperLocation>) -> ExtendedRegister {
        let mut occupied_used_registers = vec![false;16];
        for used_register in used_registers {
            match used_register {
                MapperLocation::DataRegister(DataRegister(index)) => occupied_used_registers[*index as usize] = true,
                MapperLocation::ExtendedRegister(ExtendedRegister(index)) => {
                    occupied_used_registers[*index as usize] = true;
                    occupied_used_registers[*index as usize + 1] = true;
                },
                _ => ()
            }
        }

        match target {
            Some(MapperLocation::ExtendedRegister(ExtendedRegister(index))) if !occupied_used_registers[*index as usize] && !occupied_used_registers[*index as usize +1]  => ExtendedRegister(*index),
            _ =>  self.next_available_extended_register( scratch_variable_map, used_registers)

        }
    }
    
}