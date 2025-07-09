#![forbid(unsafe_code)]
/// This module contains helper methods that allow allocating and managing registers.
/// They are used in order to generate code that matches the abstract operations from the valent-blocks. 
use alloc::vec;
use alloc::vec::Vec;
use core::iter;
use crate::isa_model::{Const10, DataRegister, ExtendedRegister, MapperLocation, Register, ValueSize, STACK_POINTER};
use crate::isa_model::machine_instructions::Instr;
use crate::translator::{Translator, MAX_ALL_REGISTERS, MAX_LOCAL_REGISTERS};

/// helper function for spilling scratch values
/// maps a scratch value location to the register it occupies:
/// In case the value is already stored in a register, this is straightforward
/// In case the value is stored in the linear memory, the register corresponds to the one occupied by the value corresponding to its address in the linear memory
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

/// Translator methods to allocate/manage registers to generate machine code.
impl <'a,'b> Translator<'a,'b> {
    /// helper function to spill a scratch value to the runtime stack
    /// Only values stored in (data/extended) registers need to be spilled.
    /// Values stored in the lower context registers are not to be spilled, given that some of which may be allocated to store a local variable (and therefore such register cannot be reused within the wasm function for other purposes)
    /// Values stored in the linear memory are not yet loaderd and do not need to be spilled. However their address in the linear memory to be accessed is a scratch value that may be spilled.
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

    /// allocates an available data/extended register for a scratch value, depending on the required size.
    /// In case no register is available, the oldest scratch value is spilled to the runtime stack until an available register with the appropriate size is found.
    /// It may take more than one spill to free an extended register if it is occupied by two different scratch values of 32-bit word size.
    /// Spills always occur in the same order the scratch variables appear in the scratch variable map so that they can be popped at the right order from the runtime stack, once ready to be used.
    pub(crate) fn next_available_register(&mut self, valsize : ValueSize, scratch_variable_map : &mut Vec<MapperLocation>) -> Register {

        let mut data_register_allocation_vec = vec![false; MAX_ALL_REGISTERS as usize];

        //exclude D[0] to dedicate it for virtual address bitmasking
        #[cfg(feature="address-masking")]
        {
        data_register_allocation_vec[0] = true;
        }

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

    /// allocates an availabe data register for a scratch value while avoiding using registers from used_registers
    /// used_registers is supposed to contasin other operands during the code generation for a VB (wasm instruction IR) 
    pub fn next_available_data_register (&mut self, scratch_variable_map : &mut Vec<MapperLocation>, used_registers: &Vec<MapperLocation>) -> DataRegister {
        // extending scratch_variable_map with the used_registers will indicate to the helper function that the corresponding registers contain values and therefore are not free.
        // appending the values at the end of the map put them at the end of the priorities when it comes to spilling. Typically the number of used registers will be small enough that a corresponding register
        // will be free before used registers have to be spilled (but this is unchecked and up to the code generation process)
        scratch_variable_map.extend(used_registers.clone());
        let result = match self.next_available_register(ValueSize::Word, scratch_variable_map) {
            Register::DataRegister(data_register) => data_register,
            _ => panic!("Expected a data register")
        };
        //undo extending the scratch_variable_map
        scratch_variable_map.truncate(scratch_variable_map.len() - used_registers.len());
        result
    }

    /// allocates an availabe extended register for a scratch value while avoiding using registers from used_registers
    /// used_registers is supposed to contasin other operands during the code generation for a VB (wasm instruction IR) 
    pub fn next_available_extended_register(&mut self, scratch_variable_map : &mut Vec<MapperLocation>, used_registers: &Vec<MapperLocation>) -> ExtendedRegister {
        // extending scratch_variable_map with the used_registers will indicate to the helper function that the corresponding registers contain values and therefore are not free.
        // appending the values at the end of the map put them at the end of the priorities when it comes to spilling. Typically the number of used registers will be small enough that a corresponding register
        // will be free before used registers have to be spilled (but this is unchecked and up to the code generation process)
        scratch_variable_map.extend(used_registers.clone());
        let result =match self.next_available_register(ValueSize::DoubleWord, scratch_variable_map) {
            Register::ExtendedRegister(extended_register) => extended_register,
            _ => panic!("Expected an extended register")
        };
        //undo extending the scratch_variable_map
        scratch_variable_map.truncate(scratch_variable_map.len() - used_registers.len());
        result
    }

    /// obtain a data register for the result/target of the code generation from a VB.
    /// returns the target if existent and is not a register contained in the used registers
    /// returns a new available register otherwise.
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

    /// obtain an extended register for the result/target of the code generation from a VB.
    /// returns the target if existent and is not a register contained in the used registers
    /// returns a new available register otherwise.
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