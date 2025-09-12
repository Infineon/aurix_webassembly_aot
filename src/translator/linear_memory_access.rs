#![forbid(unsafe_code)]
/// This module provides helper methods to emit instructions for accessing the linear memory 
/// The helper methods take typically among their arguments:
/// - a **base** address register
/// - an (at translation known) offset
/// The helper methods will emit insturctions to access the physical address expressed by (base + offset) while also taking into consideration an expected alignment hint, as provided from the wasm module.
/// 
/// Due to the fact that TriCore architecure supports only 10-bit and 16-bit long offsets, the offset is split into a higher and lower part prior to the memory load/store instruction (c.f. split_offset for more details).
/// It is assumed here that the linear memory is at least 2 bytes aligned.
/// If the alignment hint matches the TriCore requirements, a straightforward instruction is emitted. Otherwise the memory operation is performed byte per byte.
/// 
/// ### Usage:
/// In case the full offset (static+dynamic) is known at translation time, it can be provided through the offset argument, while setting the base address register to the dedicated register for the linear memory base.
/// 
/// Otherwise (if the dynamic offset is e.g. dependent from a local variable), the base address register will contain the sum of the linear memory base and the dynamic offset, typically computed 
use alloc::vec;
use alloc::vec::Vec;
use crate::isa_model::{AddressRegister, Const10, Const16, Const9, DataRegister, ExtendedRegister, MapperLocation, Register, RegisterOrConst, RegisterOrLargeConst, SignValue, ADDRESS_ACCUMULATOR};
use crate::isa_model::machine_instructions::Instr;
use crate::translator::Translator;

impl <'a,'b> Translator<'a,'b>{

    pub fn load_byte_from_memory(&mut self, dest: Register, base: AddressRegister, offset: u32, sign: SignValue){
        // loading can only take a 16 bit offset, you add the upper offset to the base (with some minor caveats)
        let (lower_offset, base) = self.split_offset(offset, base);
        let lower_dest = dest.get_lower_register();
        let upper_dest = dest.get_upper_register();

        match sign { // sign extension or not from 8 to 32 bits
            SignValue::Signed => {
                self.push_instruction(Instr::LDB {base, offset: Const16::new(lower_offset), dest: lower_dest});
            },
            SignValue::Unsigned => {
                self.push_instruction(Instr::LDBU {base, offset: Const16::new(lower_offset), dest: lower_dest});
            }
        }
        // sign extension for the upper destination
        self.extend_sign_over_dest(upper_dest, sign, lower_dest);
    }

    pub fn store_byte_to_memory(&mut self, src: DataRegister, base: AddressRegister, offset: u32){
        let (lower_offset, base) = self.split_offset(offset, base);
        self.push_instruction(Instr::STB {base, offset: Const16::new(lower_offset), src});
    }

    pub fn load_halfword_from_memory(&mut self, dest: Register, base: AddressRegister, offset: u32, sign: SignValue, align: u8, scratch_variable_map: &mut Vec<MapperLocation>){

        let (lower_offset, base) = self.split_offset(offset, base);
        let lower_dest = dest.get_lower_register();
        let upper_dest = dest.get_upper_register();

        match align {
            0 => { // aurix can't load unaligned half words so each byte needs to be loaded separately
                let intermediate = self.next_available_data_register(scratch_variable_map, &vec![dest.as_location()]);
                self.push_instruction(Instr::LDBU {base, offset: Const16::new(lower_offset), dest: intermediate}); // load first byte
                match sign { //load second byte with right sign extension
                    SignValue::Signed => self.push_instruction(Instr::LDB {base, offset: Const16::new(lower_offset + 1), dest: lower_dest}),
                    SignValue::Unsigned =>self.push_instruction(Instr::LDBU {base, offset: Const16::new(lower_offset + 1), dest: lower_dest})
                }
                self.push_instruction(Instr::SH {src: lower_dest, count: RegisterOrConst::Const9(Const9::new(8)), dest: lower_dest});
                self.push_instruction(Instr::OR {lhs: intermediate, rhs: RegisterOrConst::DataRegister(lower_dest), dest: lower_dest});
            },
            _ => {
                match sign {
                    SignValue::Signed => {
                        self.push_instruction(Instr::LDH {base, offset: Const16::new(lower_offset), dest: lower_dest});
                    },
                    SignValue::Unsigned => {
                        self.push_instruction(Instr::LDHU {base, offset: Const16::new(lower_offset), dest: lower_dest});
                    }
                }
            }
        }
        self.extend_sign_over_dest(upper_dest, sign, lower_dest);
    }

    pub fn store_halfword_to_memory(&mut self, src: DataRegister, base: AddressRegister, offset: u32, align: u8, scratch_variable_map: &mut Vec<MapperLocation>){
        let (lower_offset, base) = self.split_offset(offset, base);

        match align {
            0 => {
                let intermediate = self.next_available_data_register(scratch_variable_map, &vec![MapperLocation::DataRegister(src)]);
                self.push_instruction(Instr::STB {base, offset: Const16::new(lower_offset), src}); // store lower byte of register
                self.push_instruction(Instr::SH {src, count: RegisterOrConst::Const9(Const9::new(-8i16 as u16)), dest: intermediate}); // shift upper byte into lower byte of register
                self.push_instruction(Instr::STB {base, offset: Const16::new(lower_offset + 1), src: intermediate}); // store upper byte (that's now in the lower byte)
            },
            _ => {
                self.push_instruction(Instr::STH {base, offset: Const16::new(lower_offset), src});
            }
        }
    }

    pub fn load_word_from_memory(&mut self, dest: Register, base: AddressRegister, offset: u32, sign: SignValue, align: u8, scratch_variable_map: &mut Vec<MapperLocation>){
        let (lower_offset, base) = self.split_offset(offset, base);
        let lower_dest = dest.get_lower_register();
        let upper_dest = dest.get_upper_register();

        match align {
            0 => {
                let intermediate = self.next_available_data_register(scratch_variable_map, &vec![dest.as_location()]);
                self.push_instruction(Instr::LDB {base, offset: Const16::new(lower_offset + 3), dest: lower_dest});
                self.push_instruction(Instr::SH {src: lower_dest, count: RegisterOrConst::Const9(Const9::new(8)), dest: lower_dest});
                self.push_instruction(Instr::LDB {base, offset: Const16::new(lower_offset + 2), dest: intermediate});
                self.push_instruction(Instr::OR {lhs: intermediate, rhs: RegisterOrConst::DataRegister(lower_dest), dest: lower_dest});
                self.push_instruction(Instr::SH {src: lower_dest, count: RegisterOrConst::Const9(Const9::new(8)), dest: lower_dest});
                self.push_instruction(Instr::LDB {base, offset: Const16::new(lower_offset + 1), dest: intermediate});
                self.push_instruction(Instr::OR {lhs: intermediate, rhs: RegisterOrConst::DataRegister(lower_dest), dest: lower_dest});
                self.push_instruction(Instr::SH {src: lower_dest, count: RegisterOrConst::Const9(Const9::new(8)), dest: lower_dest});
                self.push_instruction(Instr::LDB {base, offset: Const16::new(lower_offset), dest: intermediate});
                self.push_instruction(Instr::OR {lhs: intermediate, rhs: RegisterOrConst::DataRegister(lower_dest), dest: lower_dest});
            },
            _ => self.push_instruction(Instr::LDW {base, offset: Const16::new(lower_offset), dest: lower_dest})
        }
        self.extend_sign_over_dest(upper_dest, sign, lower_dest);
    }

    pub fn store_word_to_memory(&mut self, src: DataRegister, base: AddressRegister, offset: u32, align: u8, scratch_variable_map: &mut Vec<MapperLocation>){
        let (lower_offset, base) = self.split_offset(offset, base);

        match align {
            0 => {
                let intermediate = self.next_available_data_register(scratch_variable_map, &vec![MapperLocation::DataRegister(src)]);
                self.push_instruction(Instr::STB {base, offset: Const16::new(lower_offset), src});
                self.push_instruction(Instr::SH {src, count: RegisterOrConst::Const9(Const9::new(-8i16 as u16)), dest: intermediate});
                self.push_instruction(Instr::STB {base, offset: Const16::new(lower_offset + 1), src: intermediate});
                self.push_instruction(Instr::SH {src: intermediate, count: RegisterOrConst::Const9(Const9::new(-8i16 as u16)), dest: intermediate});
                self.push_instruction(Instr::STB {base, offset: Const16::new(lower_offset + 2), src: intermediate});
                self.push_instruction(Instr::SH {src: intermediate, count: RegisterOrConst::Const9(Const9::new(-8i16 as u16)), dest: intermediate});
                self.push_instruction(Instr::STB {base, offset: Const16::new(lower_offset+3), src: intermediate});
            },
            _ => self.push_instruction(Instr::STW {base, offset: Const16::new(lower_offset), src})
        }
    }

    pub fn load_double_word_from_memory(&mut self, dest: ExtendedRegister, base: AddressRegister, offset: u32, align: u8, scratch_variable_map: &mut Vec<MapperLocation>){

        match align {
            0 => {
                let (lower_offset, base) = self.split_offset(offset, base);
                let lower_dest = DataRegister::new(dest.0);
                let upper_dest = DataRegister::new(dest.0 + 1);

                let intermediate = self.next_available_data_register(scratch_variable_map, &vec![MapperLocation::ExtendedRegister(dest)]);
                self.push_instruction(Instr::LDB {base, offset: Const16::new(lower_offset + 7), dest: upper_dest});
                self.push_instruction(Instr::SH {src: upper_dest, count: RegisterOrConst::Const9(Const9::new(8)), dest: upper_dest});
                self.push_instruction(Instr::LDB {base, offset: Const16::new(lower_offset + 6), dest: intermediate});
                self.push_instruction(Instr::OR {lhs: intermediate, rhs: RegisterOrConst::DataRegister(upper_dest), dest: upper_dest});
                self.push_instruction(Instr::SH {src: upper_dest, count: RegisterOrConst::Const9(Const9::new(8)), dest: upper_dest});
                self.push_instruction(Instr::LDB {base, offset: Const16::new(lower_offset + 5), dest: intermediate});
                self.push_instruction(Instr::OR {lhs: intermediate, rhs: RegisterOrConst::DataRegister(upper_dest), dest: upper_dest});
                self.push_instruction(Instr::SH {src: upper_dest, count: RegisterOrConst::Const9(Const9::new(8)), dest: upper_dest});
                self.push_instruction(Instr::LDB {base, offset: Const16::new(lower_offset + 4), dest: intermediate});
                self.push_instruction(Instr::OR {lhs: intermediate, rhs: RegisterOrConst::DataRegister(upper_dest), dest: upper_dest});
                self.push_instruction(Instr::LDB {base, offset: Const16::new(lower_offset + 3), dest: lower_dest});
                self.push_instruction(Instr::SH {src: lower_dest, count: RegisterOrConst::Const9(Const9::new(8)), dest: lower_dest});
                self.push_instruction(Instr::LDB {base, offset: Const16::new(lower_offset + 2), dest: intermediate});
                self.push_instruction(Instr::OR {lhs: intermediate, rhs: RegisterOrConst::DataRegister(lower_dest), dest: lower_dest});
                self.push_instruction(Instr::SH {src: lower_dest, count: RegisterOrConst::Const9(Const9::new(8)), dest: lower_dest});
                self.push_instruction(Instr::LDB {base, offset: Const16::new(lower_offset + 1), dest: intermediate});
                self.push_instruction(Instr::OR {lhs: intermediate, rhs: RegisterOrConst::DataRegister(lower_dest), dest: lower_dest});
                self.push_instruction(Instr::SH {src: lower_dest, count: RegisterOrConst::Const9(Const9::new(8)), dest: lower_dest});
                self.push_instruction(Instr::LDB {base, offset: Const16::new(lower_offset), dest: intermediate});
                self.push_instruction(Instr::OR {lhs: intermediate, rhs: RegisterOrConst::DataRegister(lower_dest), dest: lower_dest});
            },
            _ => {
                let (lower_offset, base) = self.split_small_offset(offset, base);
                self.push_instruction(Instr::LDD {base, offset: Const10::new(lower_offset as i16), dest});
            }
        }
    }

    pub fn store_double_word_to_memory(&mut self, src: ExtendedRegister, base: AddressRegister, offset: u32, align: u8, scratch_variable_map: &mut Vec<MapperLocation>){
        match align {
            0 => {
                let (lower_offset, base) = self.split_offset(offset, base);
                let lower_src = DataRegister::new(src.0);
                let upper_src = DataRegister::new(src.0 + 1);

                let intermediate = self.next_available_data_register(scratch_variable_map, &vec![MapperLocation::ExtendedRegister(src)]);
                self.push_instruction(Instr::STB {base, offset: Const16::new(lower_offset), src: lower_src});
                self.push_instruction(Instr::SH {src: lower_src, count: RegisterOrConst::Const9(Const9::new(-8i16 as u16)), dest: intermediate});
                self.push_instruction(Instr::STB {base, offset: Const16::new(lower_offset + 1), src: intermediate});
                self.push_instruction(Instr::SH {src: intermediate, count: RegisterOrConst::Const9(Const9::new(-8i16 as u16)), dest: intermediate});
                self.push_instruction(Instr::STB {base, offset: Const16::new(lower_offset + 2), src: intermediate});
                self.push_instruction(Instr::SH {src: intermediate, count: RegisterOrConst::Const9(Const9::new(-8i16 as u16)), dest: intermediate});
                self.push_instruction(Instr::STB {base, offset: Const16::new(lower_offset + 3), src: intermediate});
                self.push_instruction(Instr::SH {src: lower_src, count: RegisterOrConst::Const9(Const9::new(8)), dest: lower_src});
                self.push_instruction(Instr::STB {base, offset: Const16::new(lower_offset + 4), src: upper_src});
                self.push_instruction(Instr::SH {src: upper_src, count: RegisterOrConst::Const9(Const9::new(-8i16 as u16)), dest: intermediate});
                self.push_instruction(Instr::STB {base, offset: Const16::new(lower_offset + 5), src: intermediate});
                self.push_instruction(Instr::SH {src: intermediate, count:RegisterOrConst::Const9(Const9::new(-8i16 as u16)), dest: intermediate});
                self.push_instruction(Instr::STB {base, offset: Const16::new(lower_offset + 6), src: intermediate});
                self.push_instruction(Instr::SH {src: intermediate, count: RegisterOrConst::Const9(Const9::new(-8i16 as u16)), dest: intermediate});
                self.push_instruction(Instr::STB {base, offset: Const16::new(lower_offset + 7), src: intermediate});
            },
            _ => {
                let (lower_offset, base) = self.split_small_offset(offset, base);
                self.push_instruction(Instr::STD {base, offset: Const10::new(lower_offset as i16), src});
            }
        }
    }


    /// The Tricore architecture supports at most 16-bit offsets. Therefore 32-bit offsets should be split to be handled.
    /// the method splits the offset into an upper and lower part. A machine instruction is emitted to add the upper part to the address base (if non-zero).
    /// A 16-bit lower offset with a new address base register are returned. The latter corresponds to the location of (initial base address + upper_offset<<16)
    fn split_offset(&mut self, offset: u32, base: AddressRegister) -> (u16, AddressRegister) {
        // From Aurix manual volume 2 1.7 Address arithmetic, solves sign extension for the lower offset
        let upper_offset = (offset.wrapping_add(0x8000) >> 16) as u16;
        let lower_offset = offset as u16;

        let mut base = base;

        if upper_offset > 0 {
            self.push_instruction(Instr::ADDIHA { lhs: base, rhs: Const16::new(upper_offset), dest: ADDRESS_ACCUMULATOR });
            base = ADDRESS_ACCUMULATOR;
        }
        (lower_offset, base)
    }

    /// Similar to split_offset for instructions that support only 10-bit offsets (e.g. 64-bit double word memory operations)
    /// The upper part is added to the address base using  emitted machine instructions. The lower part is returned with the new address base.
    fn split_small_offset(&mut self, offset: u32, base: AddressRegister) -> (u16, AddressRegister) {
        let upper_mid_offset = offset.wrapping_add(0x200) >>10; // upper 22 bits that will be added to the base, must be split as we can only add 16 bit constants
        let lower_offset = (offset & 0x3FF) as u16;

        let mut base = base;

        let upper_offset = ( upper_mid_offset.wrapping_add(0x20) >> 6) as u16;
        let mid_offset = ((upper_mid_offset & 0x3F) << 10 ) as u16;

        if upper_offset > 0 {
            self.push_instruction(Instr::ADDIHA { lhs: base, rhs: Const16::new(upper_offset), dest: ADDRESS_ACCUMULATOR });
            base = ADDRESS_ACCUMULATOR;
        }

        if mid_offset > 0 {
            self.push_instruction(Instr::LEA { base, offset: Const16::new(mid_offset), dest: ADDRESS_ACCUMULATOR });
            base = ADDRESS_ACCUMULATOR;
        }

        (lower_offset, base)

    }

    /// helper method to extend the loaded value on a data register onto the wider extended register.
    /// extension may be a sign extension, where the upper register is filled corresponding with uppermost bit of the lower register,
    /// or a zero extension, where the upper register is set to zero
    fn extend_sign_over_dest(&mut self, upper_dest: Option<DataRegister>, sign: SignValue, lower_dest: DataRegister) {
        upper_dest.map(|upper_dest: DataRegister|  match sign {
            SignValue::Signed => {
                self.push_instruction(Instr::MOV{ src: RegisterOrLargeConst::DataRegister(lower_dest), dest: Register::DataRegister(upper_dest)});
                self.push_instruction(Instr::SHA { src: upper_dest, count: RegisterOrConst::Const9(Const9::new(32)), dest: upper_dest}); // here the count is a signed 6-bit so 32 will be interpreted as -32
            },
            SignValue::Unsigned => {
                self.push_instruction(Instr::MOV {src: RegisterOrLargeConst::Const16(Const16::new(0)), dest: Register::DataRegister(upper_dest)});
            }
        });
    }


    
}