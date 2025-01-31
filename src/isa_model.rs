pub mod map_instr_to_bin;
pub(crate) mod machine_instructions;

use alloc::vec::Vec;
use alloc::vec;
use alloc::boxed::Box;
use defmt::Format;
use wasmparser::ValType;
use crate::isa_model::machine_instructions::Instr;
use crate::translator::Translator;

#[derive(Copy,Clone, PartialEq, Debug)]
pub enum Immediate {
    Word(u32),
    DoubleWord(u64),
}
const LINEAR_MEMORY_BASE : AddressRegister = AddressRegister(6);
pub const GLOBAL_BASE : AddressRegister = AddressRegister(5);
pub const STACK_POINTER : AddressRegister = AddressRegister(10);
pub const FRAME_POINTER : AddressRegister = AddressRegister(12);
pub const STACK_BASE : AddressRegister = AddressRegister(13);
pub const TABLE_BASE : AddressRegister = AddressRegister(4);

pub const ADDRESS_ACCUMULATOR: AddressRegister = AddressRegister(2);

fn immediate_fits (imm:u32, is_signed:SignValue) -> bool {
    match is_signed {
        SignValue::Signed => {
            let shifted_imm = imm as i32 >> 8;
            shifted_imm == 0 || shifted_imm == -1
        },
        SignValue::Unsigned => {
            let shifted_imm = imm >> 9;
            shifted_imm == 0
        }
    }
}

impl Immediate {
    pub fn as_i32(&self) -> i32 {
        match self {
            Immediate::Word(imm) => *imm as i32,
            _ => panic!("Expected i32")
        }
    }

    pub fn as_u32(&self) -> u32 {
        match self {
            Immediate::Word(imm) => *imm,
            _ => panic!("Expected f32")
        }
    }
    pub fn as_u64(&self) -> u64 {
        match self {
            Immediate::DoubleWord(imm) => *imm,
            _ => panic!("Expected f64")
        }
    } 

    pub fn lower_half(&self) -> u32 {
        match self {
            Immediate::Word(imm) => *imm ,
            Immediate::DoubleWord(imm) => *imm as u32,
        }
    }

    pub fn map_to_register_or_const(&self, is_immediate_signed: SignValue, translator : &mut Translator, scratch_variable_map : &mut Vec<MapperLocation>, used_registers: &Vec<MapperLocation> ) -> RegisterOrConst {
        let imm = self.as_u32();
        if immediate_fits(imm, is_immediate_signed) {
            return RegisterOrConst::Const9(Const9::new(imm as u16));
        }
        let register = translator.next_available_data_register(scratch_variable_map, used_registers);
        let lower_immediate = imm as u16;
        let upper_immediate = (imm >> 16) as u16;
        translator.push_instruction(Instr::MOVU { src: Const16::new(lower_immediate), dest: register});
        if upper_immediate != 0 {
                translator.push_instruction(Instr::ADDIH { lhs: register, rhs: Const16(upper_immediate), dest: register } );
        }
        RegisterOrConst::DataRegister(register)
    }

    pub fn map_to_register_or_const_couple(&self, is_immediate_signed: SignValue, translator : &mut Translator, scratch_variable_map : &mut Vec<MapperLocation>, used_registers: &Vec<MapperLocation>) -> (RegisterOrConst, RegisterOrConst){
        let imm = self.as_u64();
        let immediate_lower: u32 = imm  as u32;
        let immediate_upper: u32 = (imm >> 32) as u32;
        let lower = Immediate::Word(immediate_lower).map_to_register_or_const(is_immediate_signed, translator, scratch_variable_map, used_registers);
        let upper = Immediate::Word(immediate_upper).map_to_register_or_const(is_immediate_signed, translator, scratch_variable_map, used_registers);
        (lower, upper)
    }

    pub fn as_word_vector(&self) -> Vec<u32> {
        match self {
            Immediate::Word(imm) => vec![*imm],
            Immediate::DoubleWord(imm) => vec![ (*imm >> 32) as u32, *imm as u32],
        }
    }
}


#[derive(Copy,Clone, PartialEq, Debug, Format)]
pub struct DataRegister(pub u8);

impl DataRegister {
    pub fn new(register:u8) -> Self {
        DataRegister(register & 0xF)
    }

    pub fn map_to_location (&self, target: Option<&MapperLocation> ,translator: &mut Translator, scratch_variable_map: &mut Vec<MapperLocation> ) -> MapperLocation {
        match target {
            None => MapperLocation::DataRegister(*self),
            Some(MapperLocation::DataRegister(register)) if register == self => MapperLocation::DataRegister(*self),
            Some(MapperLocation::DataRegister(register)) => {
                translator.push_instruction(Instr::MOV { src: RegisterOrLargeConst::DataRegister(*self), dest: Register::DataRegister(*register)});
                MapperLocation::DataRegister(*register)
            },
            Some(MapperLocation::ExtendedRegister(register)) => {
                translator.push_instruction(Instr::MOV { src: RegisterOrLargeConst::DataRegister(*self), dest: Register::ExtendedRegister(*register)});
                MapperLocation::ExtendedRegister(*register)
            },
            Some(MapperLocation::LinearMemory { static_offset, src_size, dynamic_offset, align , ext_sign }) => {
                let (offset, base) = process_memory_access_offset(dynamic_offset, static_offset, translator, scratch_variable_map, &vec![MapperLocation::DataRegister(*self)]);

                match src_size {
                    Memsize::DoubleWord => panic!("Expected word"),
                    Memsize::Word => translator.store_word_to_memory(*self, base, offset, *align, scratch_variable_map),
                    Memsize::Byte => translator.store_byte_to_memory(*self, base, offset),
                    Memsize::HalfWord => translator.store_halfword_to_memory(*self, base, offset, *align, scratch_variable_map)
                } 

                MapperLocation::LinearMemory { static_offset: *static_offset, src_size: *src_size, dynamic_offset: dynamic_offset.clone(), ext_sign: *ext_sign, align: *align}
                },
            Some(MapperLocation::Immediate(..)) => panic!("Cannot target immediate"),
            Some(MapperLocation::Frame { offset, size }) => {
                translator.push_instruction(Instr::STW { base: FRAME_POINTER, offset: Const16(*offset as u16), src: *self });
                MapperLocation::Frame { offset: *offset, size: *size }
            },
            Some(MapperLocation::Stack { size }) => {
                translator.push_instruction(Instr::STWPI { base: STACK_POINTER, offset: Const10(-4), src: *self });
                MapperLocation::Stack { size: *size }
            },
            Some(MapperLocation::Global { offset, size }) => {
                translator.store_word_to_global( *offset, *self);
                MapperLocation::Global { offset: *offset, size: *size }
            },
            Some(MapperLocation::Unreachable) => MapperLocation::Unreachable
        }
    }

}

fn process_memory_access_offset<'a,'b>(dynamic_offset: &Option<Box<MapperLocation>>, static_offset: &usize, translator: &mut Translator<'a,'b>, scratch_variable_map: &mut Vec<MapperLocation>, used_registers: &Vec<MapperLocation>) -> (u32, AddressRegister) {
    let (offset, dynamic_offset) = match dynamic_offset {
        None => (*static_offset as u32, None),
        Some(dynamic_offset)  => match **dynamic_offset {
            MapperLocation::Immediate(Immediate::Word(imm)) => (*static_offset as u32 + imm as u32,None),
            _ => (*static_offset as u32, Some(dynamic_offset))
        }
    };

    let mut base = LINEAR_MEMORY_BASE;
    if let Some (dynamic_offset) = dynamic_offset {
        let dynamic_register = dynamic_offset.map_to_data_register(None, translator, scratch_variable_map, used_registers);
        translator.push_instruction(Instr::ADDSCA { lhs:LINEAR_MEMORY_BASE, rhs: dynamic_register, dest: ADDRESS_ACCUMULATOR, shift: Const4(0) });
        base = ADDRESS_ACCUMULATOR;
    }
    (offset, base)
}

#[derive(Copy,Clone, PartialEq, Debug, Format)]
pub struct ExtendedRegister(pub u8);

impl ExtendedRegister {
    pub fn new(register:u8) -> Self {
        ExtendedRegister(register & 0xE)
    }

    pub fn map_to_location (&self, target: Option<&MapperLocation>, translator: &mut Translator, scratch_variable_map: &mut Vec<MapperLocation>) -> MapperLocation {
        match target {
            None => MapperLocation::ExtendedRegister(*self),
            Some(MapperLocation::ExtendedRegister(register)) if register == self => MapperLocation::ExtendedRegister(*self),
            Some(MapperLocation::ExtendedRegister(register)) => {
                let ExtendedRegister(index) = self;
                translator.push_instruction(Instr::MOV { src: RegisterOrLargeConst::RegisterCouple{lower: DataRegister(*index), upper: DataRegister(*index+1)}, dest: Register::ExtendedRegister(*register)});
                MapperLocation::ExtendedRegister(*register)
            },
            Some(MapperLocation::DataRegister(DataRegister(index)))  if *self == ExtendedRegister(*index) => MapperLocation::DataRegister(DataRegister(*index)),
            Some(MapperLocation::DataRegister(register)) => {
                let ExtendedRegister(index) = self;
                translator.push_instruction(Instr::MOV { src: RegisterOrLargeConst::new_register(*index), dest: Register::DataRegister(*register)});
                MapperLocation::DataRegister(*register)
            },
            Some(MapperLocation::LinearMemory { static_offset, src_size, dynamic_offset, align, ext_sign }) => {
                let (offset, base) = process_memory_access_offset(dynamic_offset, static_offset, translator, scratch_variable_map, &vec![MapperLocation::ExtendedRegister(*self)]);
                match src_size {
                    Memsize::DoubleWord => translator.store_double_word_to_memory(*self, base, offset, *align, scratch_variable_map),
                    Memsize::Byte => translator.store_byte_to_memory(self.lower_half(), base, offset),
                    Memsize::HalfWord =>translator.store_halfword_to_memory(self.lower_half(), base, offset, *align, scratch_variable_map),
                    Memsize::Word => translator.store_word_to_memory(self.lower_half(), base, offset, *align, scratch_variable_map),
                }
                MapperLocation::LinearMemory { static_offset: *static_offset, src_size: *src_size, dynamic_offset: dynamic_offset.clone(), ext_sign: *ext_sign, align: *align}
            },
            Some(MapperLocation::Immediate(..)) => panic!("Cannot target immediate"),
            Some(MapperLocation::Frame { offset, size }) => {
                translator.push_instruction(Instr::STD { base: FRAME_POINTER, offset: Const10(*offset as i16), src: *self });
                MapperLocation::Frame { offset: *offset, size: *size }
            },
            Some(MapperLocation::Stack { size }) => {
                translator.push_instruction(Instr::STDPI { base: STACK_POINTER, offset: Const10(-8), src: *self });
                MapperLocation::Stack { size: *size }
            },
            Some(MapperLocation::Global { offset, size }) => {
                translator.store_double_word_to_global(*offset, *self);
                MapperLocation::Global { offset: *offset, size: *size }
            },
            Some(MapperLocation::Unreachable) => MapperLocation::Unreachable
        }
    }

    pub fn lower_half (&self) -> DataRegister {
        DataRegister(self.0)
    }

    pub fn upper_half (&self) -> DataRegister {
        DataRegister(self.0 + 1)
    }  
}

#[derive(Copy,Clone, PartialEq, Debug, Format)]
pub struct AddressRegister(pub u8);

impl AddressRegister {
    pub fn new(register:u8) -> Self {
        AddressRegister(register & 0xF)
    }
}

#[derive(Clone, PartialEq, Debug, Format)]
pub struct Const9(pub u16);

#[derive(Debug, Clone, Format)]
pub struct Const10(pub i16);

#[derive(Debug, Clone, Format)]
pub struct Const16(pub u16);

#[derive(Debug, Clone, Format)]
pub struct Const18(pub u32);

#[derive(Clone, PartialEq, Debug, Format)]
pub struct Const4(pub u8);

impl Const9 {
    pub fn new(constant:u16) -> Self {
        Const9(constant & 0x1FF)
    }
}

impl Const10 {
    pub fn new(constant:i16) -> Self {
        Const10(constant & 0x3FF)
    }
}

impl Const16 {
    pub fn new(constant:u16) -> Self {
        Const16(constant & 0xFFFF)
    }
}

impl Const4 {
    pub fn new(constant:u8) -> Self {
        Const4(constant & 0xF)
    }
}

impl Const18 {
    pub fn new(constant:u32) -> Self {
        Const18(constant & 0x3FFFF)
    }
    
    pub fn from_address (address:u32) -> Self {
        Const18(address & 0x3FFF | ((address & 0xF000000) >> 14))
    }
}

#[derive(Clone, PartialEq, Debug, Format)]
pub enum RegisterOrConst {
    DataRegister(DataRegister),
    Const9(Const9),
}

#[derive(Clone, PartialEq, Debug, Format)]
pub enum RegisterOrSmallConst {
    DataRegister(DataRegister),
    Const4(Const4),
}

#[derive(Debug, Clone, Format)]
pub enum RegisterOrLargeConst {
    DataRegister(DataRegister),
    Const16(Const16),
    RegisterCouple{lower:DataRegister, upper:DataRegister},
}

#[derive(Debug, Clone, Format)]
pub enum Register {
    DataRegister(DataRegister),
    ExtendedRegister(ExtendedRegister),
}

impl RegisterOrConst {
    pub fn new_register(register:u8) -> Self {
        RegisterOrConst::DataRegister(DataRegister::new(register))
    }
    pub fn new_const(constant:u16) -> Self {
        RegisterOrConst::Const9(Const9::new(constant))
    }

    pub fn to_mapper_location(&self) -> MapperLocation {
        match self {
            RegisterOrConst::DataRegister(register) => MapperLocation::DataRegister(*register),
            RegisterOrConst::Const9(constant) => MapperLocation::Immediate(Immediate::Word(constant.0 as u32))
        }
    }
}

impl RegisterOrLargeConst {
    pub fn new_register(register:u8) -> Self {
        RegisterOrLargeConst::DataRegister(DataRegister::new(register))
    }
    pub fn new_const(constant:u16) -> Self {
        RegisterOrLargeConst::Const16(Const16::new(constant))
    }
}

impl RegisterOrSmallConst{
    pub fn new_register(register:u8) -> Self {
        RegisterOrSmallConst::DataRegister(DataRegister::new(register))
    }
    pub fn new_const(constant:u8) -> Self {
        RegisterOrSmallConst::Const4(Const4::new(constant))
    }
}

impl Register {
    pub fn new_data_register(register:u8) -> Self {
        Register::DataRegister(DataRegister::new(register))
    }
    pub fn new_extended_register(register:u8) -> Self {
        Register::ExtendedRegister(ExtendedRegister::new(register))
    }

    pub fn as_location(&self) -> MapperLocation {
        match self {
            Register::DataRegister(register) => MapperLocation::DataRegister(*register),
            Register::ExtendedRegister(register) => MapperLocation::ExtendedRegister(*register)
        }
    }

    pub fn map_to_location(&self, target: Option<&MapperLocation>, translator: &mut Translator, scratch_variable_map: &mut Vec<MapperLocation>) -> MapperLocation {
        match self {
            Register::DataRegister(register) => register.map_to_location(target, translator, scratch_variable_map),
            Register::ExtendedRegister(register) => register.map_to_location(target, translator, scratch_variable_map)
        }
    }

    pub fn get_lower_register(&self) -> DataRegister {
        match self {
            Register::DataRegister(register) => *register,
            Register::ExtendedRegister(ExtendedRegister(index)) => DataRegister(*index)
        }
    }

    pub fn get_upper_register(&self) -> Option<DataRegister>{
        match self {
            Register::DataRegister(..) => None,
            Register::ExtendedRegister(ExtendedRegister(index)) => Some(DataRegister(*index + 1))
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Memsize {
    Byte,
    HalfWord,
    Word,
    DoubleWord,
}

#[derive(Clone, PartialEq)]
pub enum MapperLocation {
    DataRegister(DataRegister),
    ExtendedRegister(ExtendedRegister),
    Frame{offset:i16, size:ValueSize},
    Stack{size:ValueSize},
    LinearMemory{static_offset:usize, src_size:Memsize, dynamic_offset:Option<Box<MapperLocation>>, align:u8 , ext_sign:SignValue},
    Immediate(Immediate),
    Global{offset:u32, size:ValueSize},
    Unreachable,
}
#[derive(Clone, Copy, PartialEq)]
pub enum SignValue {
    Signed,
    Unsigned,
} 

impl MapperLocation {
    pub fn as_register(&self) -> Register {
        match self {
            MapperLocation::DataRegister(register) => Register::DataRegister(*register),
            MapperLocation::ExtendedRegister(register) => Register::ExtendedRegister(*register),
            _ => panic!("Expected register")
        }
    }

    pub fn new_data_register(register:u8) -> Self {
        MapperLocation::DataRegister(DataRegister::new(register))
    }

    pub fn new_extended_register(register:u8) -> Self {
        MapperLocation::ExtendedRegister(ExtendedRegister::new(register))
    }

    pub fn lower_half (&self) -> Self {
        match self {
            MapperLocation::DataRegister(register) => MapperLocation::DataRegister(*register),
            MapperLocation::ExtendedRegister(ExtendedRegister(index)) => MapperLocation::DataRegister(DataRegister(*index)),
            MapperLocation::Frame { offset, size: _} => MapperLocation::Frame {offset:*offset, size: ValueSize::Word},
            MapperLocation::LinearMemory { static_offset, src_size, dynamic_offset, align, ext_sign } => MapperLocation::LinearMemory { static_offset: *static_offset, dynamic_offset: dynamic_offset.clone(), src_size: *src_size, ext_sign: *ext_sign, align: *align},
            MapperLocation::Immediate(immediate) => MapperLocation::Immediate(Immediate::Word(immediate.lower_half())),
            MapperLocation::Stack { .. } => MapperLocation::Stack { size: ValueSize::Word},
            MapperLocation::Global { offset, ..} => MapperLocation::Global { offset: *offset, size: ValueSize::Word},
            MapperLocation::Unreachable => MapperLocation::Unreachable
        }
    }

    pub fn map_to_data_register(&self, target: Option<DataRegister>, translator : &mut Translator, scratch_variable_map : &mut Vec<MapperLocation>, used_registers: &Vec<MapperLocation>) -> DataRegister {
        match self {
            MapperLocation::DataRegister(register) => match target {
            None => *register,
            Some(target_register) if target_register == *register => *register,
            Some(target_register) => {
                    translator.push_instruction(Instr::MOV { src: RegisterOrLargeConst::DataRegister(*register), dest: Register::DataRegister(target_register)});
                    target_register
                }
            },
            MapperLocation::ExtendedRegister(ExtendedRegister(index)) => match target{
            None => DataRegister(*index),
            Some (target_register) if target_register == DataRegister(*index) => DataRegister(*index),
            Some (target_register) => {
                    translator.push_instruction(Instr::MOV { src: RegisterOrLargeConst::DataRegister(DataRegister(*index)), dest: Register::DataRegister(target_register)});
                    target_register
                }
            },
            MapperLocation::LinearMemory { static_offset, dynamic_offset, src_size, ext_sign, align } => {
                let (offset, base) = process_memory_access_offset(dynamic_offset, static_offset, translator, scratch_variable_map, used_registers);
                let register =  target.unwrap_or_else(|| translator.next_available_data_register(scratch_variable_map, used_registers));

                match src_size {
                   Memsize::DoubleWord | Memsize::Word =>  translator.load_word_from_memory(Register::DataRegister(register), base, offset,  *ext_sign,  *align, scratch_variable_map),
                   Memsize::Byte => translator.load_byte_from_memory(Register::DataRegister(register), base, offset, *ext_sign),
                   Memsize::HalfWord => translator.load_halfword_from_memory(Register::DataRegister(register), base, offset, *ext_sign, *align, scratch_variable_map)
                }
                register
            },
            MapperLocation::Immediate(imm) => {
                let immediate =  imm.as_u32();
                let immediate_lower: u16 = immediate  as u16;
                let immediate_upper: u16 = (immediate >> 16) as u16;
                let register = target.unwrap_or_else(|| translator.next_available_data_register(scratch_variable_map, used_registers));

                translator.push_instruction(Instr::MOVU { src: Const16::new(immediate_lower) , dest: register});
                if immediate_upper != 0 {
                     translator.push_instruction(Instr::ADDIH { lhs: register, rhs: Const16(immediate_upper), dest: register } );
                }
                register
            },
            MapperLocation::Frame { offset, .. } => {
                    let register = target.unwrap_or_else(|| translator.next_available_data_register(scratch_variable_map, used_registers));
                    translator.push_instruction(Instr::LDW { base: FRAME_POINTER, offset: Const16(*offset as u16), dest: register });
                    register
            },
            MapperLocation::Stack { size } => {
                let register = target.unwrap_or_else(|| translator.next_available_data_register(scratch_variable_map, used_registers));
                match size {
                    ValueSize::Word => translator.push_instruction(Instr::LDWPI { base: STACK_POINTER, offset: Const10(4), dest: register }),
                    ValueSize::DoubleWord => {
                        translator.push_instruction(Instr::LDW { base: STACK_POINTER, offset: Const16(0), dest: register});
                        translator.push_instruction(Instr::LEA { base: STACK_POINTER, offset: Const16(8), dest: STACK_POINTER});
                    },
                }
                register
            },
            MapperLocation::Global { offset, .. } => {
                let register = target.unwrap_or_else(|| translator.next_available_data_register(scratch_variable_map, used_registers));
                translator.load_word_from_global(register, *offset);
                register
            },
            MapperLocation::Unreachable => {
                translator.push_instruction(Instr::Trap);
                DataRegister(15)
        }
    }
}


    pub fn map_to_extended_register(&self, target: Option<ExtendedRegister>, translator: &mut Translator, scratch_variable_map : &mut Vec<MapperLocation>, used_registers: &Vec<MapperLocation>) -> ExtendedRegister {
        match self {
            MapperLocation::ExtendedRegister(register) => match target {
               None => *register,
               Some (target_register) if target_register == *register => *register,
               Some (target_register) => {
                    let ExtendedRegister(index) = register;
                      translator.push_instruction(Instr::MOV { src: RegisterOrLargeConst::RegisterCouple { lower: DataRegister::new(*index), upper: DataRegister::new(*index+1) }, dest: Register::ExtendedRegister(target_register)});
                      target_register
                 }
            },
            MapperLocation::DataRegister(DataRegister(index)) => match target {
                None => ExtendedRegister::new(*index),
                Some(target_register) => {
                    translator.push_instruction(Instr::MOV { src: RegisterOrLargeConst::DataRegister(DataRegister(*index)), dest: Register::ExtendedRegister(target_register)});
                    target_register
                }
            },
            MapperLocation::LinearMemory { static_offset, dynamic_offset, src_size, ext_sign, align } => {
                let (offset, base) = process_memory_access_offset(dynamic_offset, static_offset, translator, scratch_variable_map, used_registers);
                let register = target.unwrap_or_else( || translator.next_available_extended_register(scratch_variable_map, used_registers));
            

                match src_size {
                   Memsize::DoubleWord => translator.load_double_word_from_memory(register, base, offset, *align, scratch_variable_map),
                   Memsize::Word =>  translator.load_word_from_memory(Register::ExtendedRegister(register), base, offset,  *ext_sign,  *align, scratch_variable_map),
                   Memsize::Byte => translator.load_byte_from_memory(Register::ExtendedRegister(register), base, offset, *ext_sign),
                   Memsize::HalfWord => translator.load_halfword_from_memory(Register::ExtendedRegister(register), base, offset, *ext_sign, *align, scratch_variable_map)
                }
                register
            },
            MapperLocation::Immediate(imm) => {
                let immediate =  imm.as_u64();
                let ExtendedRegister(index) = target.unwrap_or_else(|| translator.next_available_extended_register(scratch_variable_map, used_registers));
                let immediate_first_lower: u16 = immediate  as u16;
                let immediate_first_lower_sign_bit = (immediate_first_lower  & 0x8000) >> 15;
                let immediate_first_upper: u16 = immediate_first_lower_sign_bit.wrapping_add((immediate >> 16) as u16 );
                
                if (immediate_first_lower_sign_bit as u32).wrapping_add((immediate >> 32) as u32) == 0{
                    // Fits in 32 bits
                    translator.push_instruction(Instr::MOV { src: RegisterOrLargeConst::Const16(Const16::new(immediate_first_lower)), dest: Register::ExtendedRegister(ExtendedRegister(index))});
                    if immediate_first_upper != 0 {
                        translator.push_instruction(Instr::ADDIH { lhs: DataRegister::new(index), rhs: Const16(immediate_first_upper), dest: DataRegister::new(index) } );
                    }
                } else {
                    // Does not fit in 32 bits
                    translator.push_instruction(Instr::MOV { src: RegisterOrLargeConst::Const16(Const16::new(immediate_first_lower)), dest: Register::DataRegister(DataRegister(index))});
                    if immediate_first_upper != 0 {
                        translator.push_instruction(Instr::ADDIH { lhs: DataRegister::new(index), rhs: Const16(immediate_first_upper), dest: DataRegister::new(index) } );
                    }
                    
                    let immediate_second = (immediate >> 32) as u32;
                    let immediate_second_lower: u16 = immediate_second as u16;
                    let immediate_second_upper: u16 = (immediate_second.wrapping_add(0x8000) >> 16) as u16;
                    
                    translator.push_instruction(Instr::MOV { src: RegisterOrLargeConst::Const16(Const16::new(immediate_second_lower)), dest: Register::DataRegister(DataRegister(index + 1))});
                    if immediate_second_upper != 0 {
                        translator.push_instruction(Instr::ADDIH { lhs: DataRegister::new(index + 1), rhs: Const16(immediate_second_upper), dest: DataRegister::new(index + 1) } );
                    }
                }
                ExtendedRegister(index)
            },
            MapperLocation::Frame { offset, size } => {
                let register = target.unwrap_or_else(|| translator.next_available_extended_register(scratch_variable_map, used_registers));
                match size {
                    ValueSize::Word => translator.push_instruction(Instr::LDW { base: FRAME_POINTER, offset: Const16::new(*offset as u16), dest:register.lower_half() }),
                    ValueSize::DoubleWord => translator.push_instruction(Instr::LDD { base: FRAME_POINTER, offset: Const10::new(*offset), dest:register }),
                }
                register
            },
            MapperLocation::Stack { size } => {
                let register = target.unwrap_or_else(|| translator.next_available_extended_register(scratch_variable_map, used_registers));
                match size {
                    ValueSize::Word => {
                        translator.push_instruction(Instr::LDWPI { base: STACK_POINTER, offset: Const10(4), dest: register.lower_half()});
                        translator.push_instruction(Instr::MOV { src: RegisterOrLargeConst::DataRegister(register.lower_half()), dest: Register::ExtendedRegister(register)});
                    }
                    ValueSize::DoubleWord => translator.push_instruction(Instr::LDDPI { base: STACK_POINTER, offset: Const10(8), dest: register }),
                }
                register
            },
            MapperLocation::Global { offset, size } => {
                let register = target.unwrap_or_else(|| translator.next_available_extended_register(scratch_variable_map, used_registers));
                match size {
                    ValueSize::Word => {
                        translator.load_word_from_global(register.lower_half(), *offset);
                        translator.push_instruction(Instr::MOV { src: RegisterOrLargeConst::DataRegister(register.lower_half()), dest: Register::ExtendedRegister(register)});
                    }
                    ValueSize::DoubleWord => translator.load_double_word_from_global(register, *offset),
                }
                register
            },
            MapperLocation::Unreachable => {
                translator.push_instruction(Instr::Trap);
                ExtendedRegister(14)
        }
    }
    }

    pub fn get_size(&self) -> ValueSize {
        match self {
            MapperLocation::DataRegister(_) => ValueSize::Word,
            MapperLocation::ExtendedRegister(_) => ValueSize::DoubleWord,
            MapperLocation::Frame { size, .. } | MapperLocation::Stack { size } => *size,
            MapperLocation::LinearMemory { src_size, .. } => match src_size {
                Memsize::DoubleWord => ValueSize::DoubleWord,
                Memsize::Word | Memsize::Byte | Memsize::HalfWord => ValueSize::Word,
            },
            MapperLocation::Immediate(imm) => match imm {
                Immediate::Word(_) => ValueSize::Word,
                Immediate::DoubleWord(_) => ValueSize::DoubleWord,
            },
            MapperLocation::Global { size, .. } => *size,
            MapperLocation::Unreachable => panic!("Unreachable"),
        }
    }

    pub fn map_to_location(&self, target: &MapperLocation, translator: &mut Translator, scratch_variable_map : &mut Vec<MapperLocation>, used_registers: &Vec<MapperLocation>) -> MapperLocation {
        if self == target {
            return self.clone();
        }
        let target_data_register = match target {
            MapperLocation::DataRegister(register) => Some(*register),
            _ => None
        };
        let target_extended_register = match target {
            MapperLocation::ExtendedRegister(register) => Some(*register),
            _ => None
        };
        match self {
            MapperLocation::DataRegister(register) => register.map_to_location(Some(target), translator, scratch_variable_map),
            MapperLocation::ExtendedRegister(register) => register.map_to_location(Some(target), translator, scratch_variable_map),
            MapperLocation::Frame { size, .. } => match size {
                ValueSize::Word =>  self.map_to_data_register( target_data_register, translator, scratch_variable_map, used_registers).map_to_location(Some(target), translator, scratch_variable_map),
                ValueSize::DoubleWord => self.map_to_extended_register(target_extended_register, translator, scratch_variable_map, used_registers).map_to_location(Some(target), translator, scratch_variable_map),
            },
            MapperLocation::Stack { size } => match size {
                ValueSize::Word =>  self.map_to_data_register( target_data_register, translator, scratch_variable_map, used_registers).map_to_location(Some(target), translator, scratch_variable_map),
                ValueSize::DoubleWord => self.map_to_extended_register(target_extended_register, translator, scratch_variable_map, used_registers).map_to_location(Some(target), translator, scratch_variable_map),
            },
            MapperLocation::LinearMemory { src_size, .. } => {
                if *src_size == Memsize::DoubleWord || target.get_size() == ValueSize::DoubleWord {
                    self.map_to_extended_register(target_extended_register, translator, scratch_variable_map, used_registers).map_to_location(Some(target), translator, scratch_variable_map)
                } else {
                    self.map_to_data_register( target_data_register, translator, scratch_variable_map, used_registers).map_to_location(Some(target), translator, scratch_variable_map)
                }
            },
            MapperLocation::Immediate(imm) => match imm {
                Immediate::Word(_) => self.map_to_data_register( target_data_register, translator, scratch_variable_map, used_registers).map_to_location(Some(target), translator, scratch_variable_map),
                Immediate::DoubleWord(_) => self.map_to_extended_register(target_extended_register, translator, scratch_variable_map, used_registers).map_to_location(Some(target), translator, scratch_variable_map),
            },
            MapperLocation::Global { size , ..} => match size {
                ValueSize::Word =>  self.map_to_data_register( target_data_register, translator, scratch_variable_map, used_registers).map_to_location(Some(target), translator, scratch_variable_map),
                ValueSize::DoubleWord => self.map_to_extended_register(target_extended_register, translator, scratch_variable_map, used_registers).map_to_location(Some(target), translator, scratch_variable_map),
            },
            MapperLocation::Unreachable => {
                translator.push_instruction(Instr::Trap);
                MapperLocation::Unreachable
            }
        }
            
}

    pub fn map_to_register_or_const(&self, immediate_sign: SignValue, translator: &mut Translator, scratch_variable_map : &mut Vec<MapperLocation>, used_registers: &Vec<MapperLocation>) -> RegisterOrConst {
        match self {
            MapperLocation::Immediate(imm) => {
                let immediate =  imm.as_u32();
                let immediate_fits = match immediate_sign {
                    SignValue::Signed => {
                        let shifted_imm = imm.as_i32() >> 8;
                        shifted_imm == 0 || shifted_imm == -1
                    },
                    SignValue::Unsigned => {
                        let shifted_imm = imm.as_u32() >> 9;
                        shifted_imm == 0
                    }    
                };
                if immediate_fits {
                    return RegisterOrConst::Const9(Const9::new(immediate as u16));
                }
                let register = translator.next_available_data_register(scratch_variable_map, used_registers);
                let lower_immediate = immediate as u16;
                let upper_immediate = (immediate >> 16) as u16;
                translator.push_instruction(Instr::MOVU { src: Const16::new(lower_immediate), dest: register});
                if upper_immediate != 0 {
                    translator.push_instruction(Instr::ADDIH { lhs: register, rhs: Const16(upper_immediate), dest: register } );
                }
                RegisterOrConst::DataRegister(register)

            },
            _ => RegisterOrConst::DataRegister(self.map_to_data_register(None, translator, scratch_variable_map, used_registers))
        }
    }

    pub fn map_to_register_or_const_couple(&self, immediate_sign: SignValue, translator: &mut Translator, scratch_variable_map : &mut Vec<MapperLocation>, used_registers: &Vec<MapperLocation> ) -> (RegisterOrConst, RegisterOrConst) {
        match self {
            MapperLocation::Immediate(imm) => imm.map_to_register_or_const_couple(immediate_sign, translator, scratch_variable_map, used_registers),
            _ => { 
                let ExtendedRegister(index)=  self.map_to_extended_register(None, translator, scratch_variable_map, used_registers);
                (RegisterOrConst::DataRegister(DataRegister(index)), RegisterOrConst::DataRegister(DataRegister(index + 1)))
            }
        }
    }

}

pub trait LocationCouple {
    fn map_to_data_registers(self, translator: &mut Translator, scratch_variable_map : &mut Vec<MapperLocation>) -> (DataRegister, DataRegister);
    fn map_abelian_children_to_register_or_const(self, is_signed_immediate:SignValue, translator: &mut Translator, scratch_variable_map : &mut Vec<MapperLocation>) -> (DataRegister, RegisterOrConst);
    fn map_to_extended_registers(self, translator: &mut Translator, scratch_variable_map : &mut Vec<MapperLocation>) -> (ExtendedRegister, ExtendedRegister);
    fn map_abelian_large_children_to_register_or_const(self, is_signed_immediate:SignValue, translator: &mut Translator, scratch_variable_map : &mut Vec<MapperLocation>) -> (ExtendedRegister, (RegisterOrConst,RegisterOrConst));
}

impl LocationCouple for  (&MapperLocation,&MapperLocation) {
    fn map_to_data_registers(self, translator: &mut Translator, scratch_variable_map : &mut Vec<MapperLocation>) -> (DataRegister, DataRegister) {
        let (lhs,rhs) = self;
        let rhs_register = rhs.map_to_data_register(None, translator, scratch_variable_map, &vec![lhs.clone()]);
        let lhs_register = lhs.map_to_data_register(None, translator, scratch_variable_map, &vec![MapperLocation::DataRegister(rhs_register)]);
        (lhs_register, rhs_register)    
    }

    fn map_abelian_children_to_register_or_const(self, is_signed_immediate:SignValue, translator: &mut Translator, scratch_variable_map : &mut Vec<MapperLocation>) -> (DataRegister, RegisterOrConst) {
        match self {
             (lhs@MapperLocation::Immediate(..), rhs) => {
               let rhs_register = rhs.map_to_data_register(None,  translator, scratch_variable_map, &vec![]);
               let lhs_register = lhs.map_to_register_or_const(is_signed_immediate, translator, scratch_variable_map, &vec![MapperLocation::DataRegister(rhs_register)]);
               (rhs_register, lhs_register)
           },
           (lhs,rhs) => {
               let rhs_register = rhs.map_to_register_or_const(is_signed_immediate, translator, scratch_variable_map, &vec![lhs.clone()]);
               let lhs_register = lhs.map_to_data_register(None, translator, scratch_variable_map, &vec![rhs_register.to_mapper_location()]);
               (lhs_register, rhs_register)
           }
       }   
   }

   fn map_to_extended_registers(self, translator: &mut Translator, scratch_variable_map : &mut Vec<MapperLocation>) -> (ExtendedRegister, ExtendedRegister) {
    let (lhs,rhs) = self;
    let rhs_register = rhs.map_to_extended_register(None,  translator, scratch_variable_map, &vec![lhs.clone()]);
    let lhs_register = lhs.map_to_extended_register(None,  translator, scratch_variable_map, &vec![MapperLocation::ExtendedRegister(rhs_register)]);
    (lhs_register, rhs_register)
   }

    fn map_abelian_large_children_to_register_or_const(self, is_signed_immediate:SignValue, translator: &mut Translator, scratch_variable_map : &mut Vec<MapperLocation>) -> (ExtendedRegister, (RegisterOrConst,RegisterOrConst)) {
        match self {
            (MapperLocation::Immediate(imm), rhs) => {
                let rhs_register = rhs.map_to_extended_register(None,  translator, scratch_variable_map, &vec![]);
                let lhs_register = imm.map_to_register_or_const_couple(is_signed_immediate,translator, scratch_variable_map, &vec![MapperLocation::ExtendedRegister(rhs_register)]);
                (rhs_register, lhs_register)
            },
            (lhs,rhs) => {
                let rhs_register = rhs.map_to_register_or_const_couple(is_signed_immediate, translator, scratch_variable_map, &vec![lhs.clone()]);
                let lhs_register = lhs.map_to_extended_register(None, translator, scratch_variable_map, &vec![rhs_register.0.to_mapper_location(), rhs_register.1.to_mapper_location()]);
                (lhs_register, rhs_register)
            }
        }
    }
}



#[derive(Copy,Clone, PartialEq, Debug, Format)]
pub enum ValueSize {
    Word=1, //Need to specify enum value to avoid compiler bug activated by optimization.
    DoubleWord=2
}

impl ValueSize {
    pub fn as_bytes(&self) -> u8 {
        match self {
            ValueSize::Word => 4,
            ValueSize::DoubleWord => 8
        }
    }

    pub fn from_valtype(valtype: &ValType) -> Self {
        match valtype {
            ValType::I32 | ValType::F32 => ValueSize::Word,
            ValType::I64 | ValType::F64 => ValueSize::DoubleWord,
            _ => panic!("Unsupported value type")
        }
    }
}
