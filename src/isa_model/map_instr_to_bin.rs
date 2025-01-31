use alloc::vec::Vec;
use alloc::vec;

use super::{AddressRegister, Const10, Const16, Const18, Const4, Const9, DataRegister, ExtendedRegister, Instr};

struct BinaryField {
    field: u32,
    start: u32,
}

impl BinaryField {
    fn new(field: u32, start: u32) -> Self {
        Self { field, start }
    }
}

struct BinaryFieldVec(Vec<BinaryField>);

impl BinaryFieldVec {
    fn to_binary(&self) -> u32 {
        let mut binary = 0;
        for field in &self.0 {
            binary |= field.field << field.start;
        }
        binary
    }
}

macro_rules! binary_fields {
    ( $( ($field:expr, $start:expr) ),* ) => {
        BinaryFieldVec(vec![
            $( BinaryField::new($field, $start) ),*
        ]).to_binary()
    };
}
impl Instr {
    pub fn map_to_binary (self) -> u32 {
        match self {
            Instr::Trap => 0xffffffff,
            Instr::NOP => binary_fields!(),
            Instr::RET => binary_fields!((0x6,22),(0xd,0)),
            Instr::MOV { src, dest} => match (dest,src)
            {
                (super::Register::DataRegister(DataRegister(c)), super::RegisterOrLargeConst::DataRegister(DataRegister(b))) => binary_fields!((c as u32, 28), (0x1f, 20), (b as u32, 12), (0xb, 0)),
                (super::Register::DataRegister(DataRegister(c)), super::RegisterOrLargeConst::Const16(Const16(const16))) => binary_fields!((c as u32, 28), (const16 as u32, 12), (0x3b, 0)),
                (super::Register::DataRegister(..), super::RegisterOrLargeConst::RegisterCouple {.. }) => unimplemented!(),
                (super::Register::ExtendedRegister(ExtendedRegister(c)), super::RegisterOrLargeConst::DataRegister(DataRegister(b))) => binary_fields!((c as u32,28), (0x80,20),(b as u32,12),(0xb,0)),
                (super::Register::ExtendedRegister(ExtendedRegister(c)), super::RegisterOrLargeConst::Const16(Const16(const16))) => binary_fields!((c as u32,28), (const16 as u32,12), (0xfb,0)),
                (super::Register::ExtendedRegister(ExtendedRegister(c)), super::RegisterOrLargeConst::RegisterCouple { lower: DataRegister(b), upper : DataRegister(a)}) => binary_fields!((c as u32,28), (0x81,20),(b as u32,12), (a as u32,8), (0xb,0)),
            },
            Instr::MOVU { src: Const16(const16), dest: DataRegister(c)} => binary_fields!((c as u32, 28), (const16 as u32, 12), (0xbb, 0)),
            Instr::MOVA { src: DataRegister(b), dest: AddressRegister(c) } => binary_fields!((c as u32, 28), (0x63, 20),(b as u32, 12), (0x1, 0)),
            Instr::MOVHA { src: Const16(const16), dest: AddressRegister(c) } => binary_fields!( (c as u32, 28), (const16 as u32, 12), (0x91, 0)),
            Instr::LEA { base: AddressRegister(b), offset: Const16(const16), dest: AddressRegister(a) } => {
                let offset_lower = const16 & 0x3f;
                let offset_mid = (const16 >> 6) & 0xf;
                let offset_upper = const16 >> 10;
                binary_fields!((offset_mid as u32, 28), (offset_upper as u32,22), (offset_lower as u32, 16), (b as u32, 12), (a as u32, 8), (0xd9, 0))
            } ,
            Instr::LDA { base: AddressRegister(b), offset: Const16(const16), dest: AddressRegister(a) } => {
                let offset_lower = const16 & 0x3f;
                let offset_mid = (const16 >> 6) & 0xf;
                let offset_upper = const16 >> 10;
                binary_fields!((offset_mid as u32, 28), (offset_upper as u32,22), (offset_lower as u32, 16), (b as u32, 12), (a as u32, 8), (0x99, 0))
            },
            Instr::LDAPI { base: AddressRegister(b), offset: Const10(const10), dest: AddressRegister(a) } => {
                let offset_lower = const10 & 0x3f;
                let offset_upper = const10 >> 6;
                binary_fields!((offset_upper as u32, 28), (0x6,22), (offset_lower as u32, 16), (b as u32, 12), (a as u32, 8), (0x9d, 0))
            },
            Instr::LDB { base: AddressRegister(b), offset: Const16(const16), dest: DataRegister(a) } =>{
                let offset_lower = const16 & 0x3f;
                let offset_mid = (const16 >> 6) & 0xf;
                let offset_upper = const16 >> 10;
                binary_fields!((offset_mid as u32, 28), (offset_upper as u32,22), (offset_lower as u32, 16), (b as u32, 12), (a as u32, 8), (0x79, 0))
            },
            Instr::LDBU { base: AddressRegister(b), offset: Const16(const16), dest: DataRegister(a) } => {
                let offset_lower = const16 & 0x3f;
                let offset_mid = (const16 >> 6) & 0xf;
                let offset_upper = const16 >> 10;
                binary_fields!((offset_mid as u32, 28), (offset_upper as u32,22), (offset_lower as u32, 16), (b as u32, 12), (a as u32, 8), (0x39, 0))
            },
            Instr::LDH { base: AddressRegister(b), offset: Const16(const16), dest: DataRegister(a) } => {
                let offset_lower = const16 & 0x3f;
                let offset_mid = (const16 >> 6) & 0xf;
                let offset_upper = const16 >> 10;
                binary_fields!((offset_mid as u32, 28), (offset_upper as u32,22), (offset_lower as u32, 16), (b as u32, 12), (a as u32, 8), (0xc9, 0))
            },
            Instr::LDHU { base: AddressRegister(b), offset: Const16(const16), dest: DataRegister(a) } => {
                let offset_lower = const16 & 0x3f;
                let offset_mid = (const16 >> 6) & 0xf;
                let offset_upper = const16 >> 10;
                binary_fields!((offset_mid as u32, 28), (offset_upper as u32,22), (offset_lower as u32, 16), (b as u32, 12), (a as u32, 8), (0xb9, 0))
            },
            Instr::LDW { base: AddressRegister(b), offset: Const16(const16), dest: DataRegister(a) } => {
                let offset_lower = const16 & 0x3f;
                let offset_mid = (const16 >> 6) & 0xf;
                let offset_upper = const16 >> 10;
                binary_fields!((offset_mid as u32, 28), (offset_upper as u32,22), (offset_lower as u32, 16), (b as u32, 12), (a as u32, 8), (0x19, 0))
            },
            Instr::LDWPI { base: AddressRegister(b), offset: Const10(const10), dest: DataRegister(a) } => {
                let offset_lower = const10 & 0x3f;
                let offset_upper = const10 >> 6;
                binary_fields!((offset_upper as u32, 28), (0x4,22), (offset_lower as u32, 16), (b as u32, 12), (a as u32, 8), (0x9, 0))
            },
            Instr::LDD { base: AddressRegister(b), offset: Const10(const10), dest: ExtendedRegister(a) } => {
                let offset_lower = const10 & 0x3f;
                let offset_upper = const10 >> 6;
                binary_fields!((offset_upper as u32, 28), (0x25,22), (offset_lower as u32, 16), (b as u32, 12), (a as u32, 8), (0x9, 0))
            },
            Instr::LDDPI { base: AddressRegister(b), offset: Const10(const10), dest: ExtendedRegister(a) } => {
                let offset_lower = const10 & 0x3f;
                let offset_upper = const10 >> 6;
                binary_fields!((offset_upper as u32, 28), (0x5,22), (offset_lower as u32, 16), (b as u32, 12), (a as u32, 8), (0x9, 0))
            },
            Instr::STB { src: DataRegister(a), base: AddressRegister(b), offset: Const16(const16) } => {
                let offset_lower = const16 & 0x3f;
                let offset_mid = (const16 >> 6) & 0xf;
                let offset_upper = const16 >> 10;
                binary_fields!((offset_mid as u32, 28), (offset_upper as u32,22), (offset_lower as u32, 16), (b as u32, 12), (a as u32, 8), (0xe9, 0))
            },
            Instr::STH { src: DataRegister(a), base: AddressRegister(b), offset: Const16(const16) } => {
                let offset_lower = const16 & 0x3f;
                let offset_mid = (const16 >> 6) & 0xf;
                let offset_upper = const16 >> 10;
                binary_fields!((offset_mid as u32, 28), (offset_upper as u32,22), (offset_lower as u32, 16), (b as u32, 12), (a as u32, 8), (0xf9, 0))
            },
            Instr::STW { src: DataRegister(a), base: AddressRegister(b), offset: Const16(const16) } => {
                let offset_lower = const16 & 0x3f;
                let offset_mid = (const16 >> 6) & 0xf;
                let offset_upper = const16 >> 10;
                binary_fields!((offset_mid as u32, 28), (offset_upper as u32,22), (offset_lower as u32, 16), (b as u32, 12), (a as u32, 8), (0x59, 0))
            },
            Instr::STD { src: ExtendedRegister(a), base: AddressRegister(b), offset: Const10(const10) } =>{
                let offset_lower = const10 & 0x3f;
                let offset_upper = const10 >> 6;
                binary_fields!((offset_upper as u32, 28), (0x25,22), (offset_lower as u32, 16), (b as u32, 12), (a as u32, 8), (0x89, 0))
            },
            Instr::STDPI { src: ExtendedRegister(a), base: AddressRegister(b), offset: Const10(const10) } => {
                let offset_lower = const10 & 0x3f;
                let offset_upper = const10 >> 6;
                binary_fields!((offset_upper as u32, 28), (0x15,22), (offset_lower as u32, 16), (b as u32, 12), (a as u32, 8), (0x89, 0))
            },
            Instr::STWPI { src: DataRegister(a), base: AddressRegister(b), offset: Const10(const10) } => {
                let offset_lower = const10 & 0x3f;
                let offset_upper = const10 >> 6;
                binary_fields!((offset_upper as u32, 28), (0x14,22), (offset_lower as u32, 16), (b as u32, 12), (a as u32, 8), (0x89, 0))
            },
            Instr::MOVH { src: Const16(const16), dest: DataRegister(c) } => binary_fields!((c as u32, 28), (const16 as u32, 12), (0x7b, 0)),
            Instr::CLZ { src: DataRegister(a), dest: DataRegister(c) } => binary_fields!((c as u32, 28), (0x1b,20), (a as u32, 12), (0xf, 0)),
            Instr::SHUFFLE { src: DataRegister(a), dest: DataRegister(c), mask: Const9(const9) } => binary_fields!((c as u32, 28), (0x7,21), (const9 as u32, 12), (a as u32, 8), (0x8f, 0)),
            Instr::POPCNT { src: DataRegister(a), dest: DataRegister(c) } => binary_fields!((c as u32, 28), (0x22, 20), (0,16), (a as u32, 8), (0x4b, 0)),
            Instr::AND { lhs: DataRegister(a), rhs, dest: DataRegister(c)  } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0x8, 20), (b as u32, 12), (a as u32, 8), (0xf, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0x8,21), (const9 as u32, 12), (a as u32, 8), (0x8f, 0)),
            },
            Instr::XOR { lhs: DataRegister(a), rhs, dest: DataRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0xc, 20), (b as u32, 12), (a as u32, 8), (0xf, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0xc,21), (const9 as u32, 12), (a as u32, 8), (0x8f, 0)),
            },
            Instr::CALL { target } => binary_fields!( (target, 8), (0x6d, 0)),
            Instr::CALLI { target: AddressRegister(a) } => binary_fields!((0,20), (a as u32,8), (0x2d, 0)),
            Instr::SVLCX => binary_fields!((0x8,22),(0xd,0)),
            Instr::RSLCX => binary_fields!((0x9,22),(0xd,0)),
            Instr::EQ { lhs: DataRegister(a), rhs, dest: DataRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0x10, 20), (b as u32, 12), (a as u32, 8), (0xb, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0x10,21), (const9 as u32, 12), (a as u32, 8), (0x8b, 0)),
            },
            Instr::NE { lhs: DataRegister(a), rhs, dest: DataRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0x11, 20), (b as u32, 12), (a as u32, 8), (0xb, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0x11,21), (const9 as u32, 12), (a as u32, 8), (0x8b, 0)),
            },
            Instr::ADD { lhs: DataRegister(a), rhs, dest: DataRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0x0, 20), (b as u32, 12), (a as u32, 8), (0xb, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0x0,21), (const9 as u32, 12), (a as u32, 8), (0x8b, 0)),
            },
            Instr::ADDF { lhs: DataRegister(a), rhs: DataRegister(d), dest: DataRegister(c) } => binary_fields!((c as u32, 28), (d as u32, 24), (0x2,20), (0x1,16),(a as u32, 8), (0x6b, 0)),
            Instr::ADDX { lhs: DataRegister(a), rhs, dest: DataRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0x4, 20), (b as u32, 12), (a as u32, 8), (0xb, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0x4,21), (const9 as u32, 12), (a as u32, 8), (0x8b, 0)),
            },
            Instr::ADDC { lhs: DataRegister(a), rhs, dest: DataRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0x5, 20), (b as u32, 12), (a as u32, 8), (0xb, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0x5,21), (const9 as u32, 12), (a as u32, 8), (0x8b, 0)),
            },
            Instr::ADDI { lhs: DataRegister(a), rhs: Const16(const16), dest: DataRegister(c) } => binary_fields!((c as u32, 28), (const16 as u32, 12), (a as u32,8), (0x1b, 0)),
            Instr::ADDIH { lhs: DataRegister(a), rhs: Const16(const16), dest: DataRegister(c) } => binary_fields!((c as u32, 28), (const16 as u32, 12), (a as u32, 8), (0x9b, 0)),
            Instr::ADDIHA { lhs: AddressRegister(a), rhs: Const16(const16), dest: AddressRegister(c) } => binary_fields!((c as u32, 28), (const16 as u32, 12), (a as u32, 8), (0x11, 0)),
            Instr::ADDA { lhs: AddressRegister(a), rhs: AddressRegister(b), dest: AddressRegister(c) } => binary_fields!((c as u32, 28), (0x1,20), (b as u32, 12), (a as u32, 8), (0x1, 0)),
            Instr::CADDN { lhs: DataRegister(a), rhs, cond: DataRegister(d), dest: DataRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (d as u32, 24), (0x1,20), (b as u32, 12), (a as u32, 8), (0x2b, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (d as u32, 24), (0x1,21), (const9 as u32, 12), (a as u32, 8), (0xab, 0)),         
            }
            Instr::SUB { lhs: DataRegister(a), rhs: DataRegister(b), dest: DataRegister(c) } => binary_fields!((c as u32, 28), (0x8,20), (b as u32, 12), (a as u32, 8), (0xb, 0)),
            Instr::SUBF { lhs: DataRegister(d), rhs: DataRegister(a), dest: DataRegister(c)} => binary_fields!((c as u32, 28), (d as u32, 24), (0x3,20), (0x1,16), (a as u32, 8), (0x6b, 0)),
            Instr::SUBX { lhs: DataRegister(a), rhs: DataRegister(b), dest: DataRegister(c) } => binary_fields!((c as u32, 28), (0xc,20), (b as u32, 12), (a as u32, 8), (0xb, 0)),
            Instr::SUBC { lhs: DataRegister(a) , rhs: DataRegister(b), dest: DataRegister(c) } => binary_fields!( (c as u32, 28), (0xd,20), (b as u32, 12), (a as u32, 8), (0xb, 0)),
            Instr::OR { lhs: DataRegister(a), rhs, dest: DataRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0xa, 20), (b as u32, 12), (a as u32, 8), (0xf, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0xa,21), (const9 as u32, 12), (a as u32, 8), (0x8f, 0)),
            },
            Instr::LT { lhs: DataRegister(a), rhs, dest: DataRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0x12, 20), (b as u32, 12), (a as u32, 8), (0xb, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0x12,21), (const9 as u32, 12), (a as u32, 8), (0x8b, 0)),
            },
            Instr::LTU { lhs: DataRegister(a), rhs, dest: DataRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0x13, 20), (b as u32, 12), (a as u32, 8), (0xb, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0x13,21), (const9 as u32, 12), (a as u32, 8), (0x8b, 0)),
            },
            Instr::GE { lhs: DataRegister(a), rhs, dest: DataRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0x14, 20), (b as u32, 12), (a as u32, 8), (0xb, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0x14,21), (const9 as u32, 12), (a as u32, 8), (0x8b, 0)),
            },
            Instr::GEU { lhs: DataRegister(a), rhs, dest: DataRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0x15, 20), (b as u32, 12), (a as u32, 8), (0xb, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0x15,21), (const9 as u32, 12), (a as u32, 8), (0x8b, 0)),
            },
            Instr::FTOIZ { src: DataRegister(a), dest: DataRegister(c) } => binary_fields!((c as u32, 28), (0x13, 20), (0x1,16),(a as u32, 8), (0x4b, 0)),
            Instr::FTOUZ { src: DataRegister(a), dest: DataRegister(c) } => binary_fields!((c as u32, 28), (0x17, 20), (0x1,16),(a as u32, 8), (0x4b, 0)),
            Instr::ITOF { src: DataRegister(a), dest: DataRegister(c) } => binary_fields!((c as u32, 28), (0x14, 20), (0x1,16),(a as u32, 8), (0x4b, 0)),
            Instr::UTOF { src: DataRegister(a), dest: DataRegister(c) } => binary_fields!((c as u32, 28), (0x16, 20), (0x1,16),(a as u32, 8), (0x4b, 0)),
            Instr::MUL { lhs: DataRegister(a), rhs, dest: DataRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0xa, 16), (b as u32, 12), (a as u32, 8), (0x73, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0x1,21), (const9 as u32, 12), (a as u32, 8), (0x53, 0)),
            },
            Instr::MULU { lhs: DataRegister(a), rhs, dest: ExtendedRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0x68, 16), (b as u32, 12), (a as u32, 8), (0x73, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0x2,21), (const9 as u32, 12), (a as u32, 8), (0x53, 0)),
            },
            Instr::MADD { lhs: DataRegister(a), rhs: DataRegister(b), acc: DataRegister(d), dest: DataRegister(c) } => binary_fields!((c as u32, 28), (d as u32, 24), (0xa,16), (b as u32, 12), (a as u32, 8), (0x3, 0)),
            Instr::MULF { lhs: DataRegister(a), rhs: DataRegister(b), dest: DataRegister(c) } => binary_fields!((c as u32, 28), (0x4,20), (0x1,16), (b as u32,12), (a as u32, 8), (0x4b, 0)),
            Instr::DIV { lhs: DataRegister(a), rhs: DataRegister(b), dest: ExtendedRegister(c) } => binary_fields!((c as u32, 28), (0x20,20), (0x1,16), (b as u32, 12), (a as u32, 8), (0x4b, 0)),
            Instr::DIVU { lhs: DataRegister(a), rhs: DataRegister(b), dest: ExtendedRegister(c) } => binary_fields!((c as u32, 28), (0x21,20), (0x1,16), (b as u32, 12), (a as u32, 8), (0x4b, 0)),
            Instr::DIVF { lhs: DataRegister(a), rhs: DataRegister(b), dest: DataRegister(c) } => binary_fields!((c as u32, 28), (0x5,20), (0x1,16), (b as u32, 12), (a as u32, 8), (0x4b, 0)),
            Instr::SH { src: DataRegister(a), count, dest: DataRegister(c) } => match count {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0x0, 20), (b as u32, 12), (a as u32, 8), (0xf, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0x0,21), (const9 as u32, 12), (a as u32, 8), (0x8f, 0)),
            },
            Instr::SHA { src: DataRegister(a), count, dest: DataRegister(c) } => match count {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0x1, 20), (b as u32, 12), (a as u32, 8), (0xf, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0x1,21), (const9 as u32, 12), (a as u32, 8), (0x8f, 0)),
            },
            Instr::CMPF { lhs: DataRegister(a), rhs: DataRegister(b), dest: DataRegister(c) } => binary_fields!((c as u32, 28), (0,20),(0x1,16), (b as u32, 12), (a as u32, 8), (0x4b, 0)),
            Instr::SEL { selector: DataRegister(d), lhs: DataRegister(a), rhs, dest: DataRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (d as u32, 24), (0x4,20), (b as u32, 12), (a as u32, 8), (0x2b, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (d as u32, 24), (0x4,21), (const9 as u32, 12), (a as u32, 8), (0xab, 0)),
            },
            Instr::SELN { selector: DataRegister(d), lhs: DataRegister(a), rhs, dest: DataRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (d as u32, 24), (0x5,20), (b as u32, 12), (a as u32, 8), (0x2b, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (d as u32, 24), (0x5,21), (const9 as u32, 12), (a as u32, 8), (0xab, 0)),
            },
            Instr::RSUB { lhs: Const9(const9), rhs: DataRegister(a), dest: DataRegister(c) } =>  binary_fields!((c as u32, 28), (0x8,21),(const9 as u32, 12), (a as u32, 8), (0x8b, 0)),
            Instr::RSUB0 { src: DataRegister(a) } => binary_fields!((0x5,12),(a as u32, 8), (0x32, 0)),
            Instr::J { target } =>binary_fields!((target as u32, 8), (0x1d, 0)),
            Instr::LOOPU {target} => binary_fields!((0x1,31), (target as u32, 16), (0xfd,0)),
            Instr::JEQ { target, lhs: DataRegister(a), rhs } => {
                let target = target as u32 & 0x7fff;
                match rhs{
                    super::RegisterOrSmallConst::DataRegister(DataRegister(b)) => binary_fields!((0, 31),(target , 16), (b as u32, 12), (a as u32,8), (0x5f, 0)),
                    super::RegisterOrSmallConst::Const4(Const4(const4)) => binary_fields!( (0, 31),(target, 16), (const4 as u32, 12), (a as u32,8), (0xdf, 0))
                }
            },
            Instr::JNE { target, lhs: DataRegister(a), rhs: Const4(const4) } => binary_fields!( (1, 31),(target as u32 & 0x7fff, 16), (const4 as u32, 12), (a as u32,8), (0xdf, 0)),
            Instr::JI { src: AddressRegister(a) } => binary_fields!((3,20), (a as u32,8), (0x2d, 0)),
            Instr::ADDSCA { lhs: AddressRegister(b), rhs: DataRegister(a), dest: AddressRegister(c), shift: Const4(n) } => binary_fields!((c as u32, 28), (0x60,20), (n as u32, 16), (b as u32,12), (a as u32, 8), (0x1, 0)),
            Instr::LDAABS { address:Const18(off18), dest: AddressRegister(a)} =>{
                let offset_lower = off18 & 0x3f;
                let offset_mid_low = (off18 >> 6) & 0xf;
                let offset_mid_up = (off18 >> 10) & 0xf;
                let offset_upper = off18 >> 14;
                binary_fields!((offset_mid_low as u32, 28), (0x2,26), (offset_mid_up as u32,22),(offset_lower as u32, 16), (offset_upper,12),(a as u32, 8), (0x85, 0))
            },
            Instr::LDWABS { address: Const18(off18), dest: DataRegister(a)} => {
                let offset_lower = off18 & 0x3f;
                let offset_mid_low = (off18 >> 6) & 0xf;
                let offset_mid_up = (off18 >> 10) & 0xf;
                let offset_upper = off18 >> 14;
                binary_fields!((offset_mid_low as u32, 28), (0,26), (offset_mid_up as u32,22),(offset_lower as u32, 16), (offset_upper,12),(a as u32, 8), (0x85, 0))
            },
            Instr::LDDABS { address: Const18(off18), dest: ExtendedRegister(a) } => {
                let offset_lower = off18 & 0x3f;
                let offset_mid_low = (off18 >> 6) & 0xf;
                let offset_mid_up = (off18 >> 10) & 0xf;
                let offset_upper = off18 >> 14;
                binary_fields!((offset_mid_low as u32, 28), (0x1,26), (offset_mid_up as u32,22),(offset_lower as u32, 16), (offset_upper,12),(a as u32, 8), (0x85, 0))
            },
            Instr::STWABS { src: DataRegister(a), address: Const18(off18) } => {
                let offset_lower = off18 & 0x3f;
                let offset_mid_low = (off18 >> 6) & 0xf;
                let offset_mid_up = (off18 >> 10) & 0xf;
                let offset_upper = off18 >> 14;
                binary_fields!((offset_mid_low as u32, 28), (0,26), (offset_mid_up as u32,22),(offset_lower as u32, 16), (offset_upper,12),(a as u32, 8), (0xa5, 0))
            },
            Instr::STDABS { src: ExtendedRegister(a), address: Const18(off18) } => {
                let offset_lower = off18 & 0x3f;
                let offset_mid_low = (off18 >> 6) & 0xf;
                let offset_mid_up = (off18 >> 10) & 0xf;
                let offset_upper = off18 >> 14;
                binary_fields!((offset_mid_low as u32, 28), (0x2,26), (offset_mid_up as u32,22),(offset_lower as u32, 16), (offset_upper,12),(a as u32, 8), (0x95, 0))
            },
            Instr::ANDLT { lhs: DataRegister(a), rhs, dest: DataRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0x22, 20), (b as u32, 12), (a as u32, 8), (0xb, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0x22,21), (const9 as u32, 12), (a as u32, 8), (0x8b, 0)),
            },
            Instr::ANDLTU { lhs: DataRegister(a), rhs, dest: DataRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0x23, 20), (b as u32, 12), (a as u32, 8), (0xb, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0x23,21), (const9 as u32, 12), (a as u32, 8), (0x8b, 0)),
            },
            Instr::ORLT { lhs: DataRegister(a), rhs, dest: DataRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0x29, 20), (b as u32, 12), (a as u32, 8), (0xb, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0x29,21), (const9 as u32, 12), (a as u32, 8), (0x8b, 0)),
            },
            Instr::ORLTU { lhs: DataRegister(a), rhs, dest: DataRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0x2a, 20), (b as u32, 12), (a as u32, 8), (0xb, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0x2a,21), (const9 as u32, 12), (a as u32, 8), (0x8b, 0)),
            },
            Instr::ANDGE { lhs: DataRegister(a), rhs, dest: DataRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0x24, 20), (b as u32, 12), (a as u32, 8), (0xb, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0x24,21), (const9 as u32, 12), (a as u32, 8), (0x8b, 0)),
            },
            Instr::ANDGEU { lhs: DataRegister(a), rhs, dest: DataRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0x25, 20), (b as u32, 12), (a as u32, 8), (0xb, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0x25,21), (const9 as u32, 12), (a as u32, 8), (0x8b, 0)),
            },
            Instr::ORGE { lhs: DataRegister(a), rhs, dest: DataRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0x2b, 20), (b as u32, 12), (a as u32, 8), (0xb, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0x2b,21), (const9 as u32, 12), (a as u32, 8), (0x8b, 0)),
            },
            Instr::ORGEU { lhs: DataRegister(a), rhs, dest: DataRegister(c) } => match rhs {
                super::RegisterOrConst::DataRegister(DataRegister(b)) => binary_fields!((c as u32, 28), (0x2c, 20), (b as u32, 12), (a as u32, 8), (0xb, 0)),
                super::RegisterOrConst::Const9(Const9(const9)) => binary_fields!((c as u32, 28), (0x2c,21), (const9 as u32, 12), (a as u32, 8), (0x8b, 0)),
            },
            Instr::EXTRU { src: DataRegister(a), width_pos:ExtendedRegister(d), dest: DataRegister(c)} => binary_fields!((c as u32, 28), (d as u32, 24), (0x3,21), (a as u32, 8), (0x17, 0)),
            Instr::EXTRUI { src: DataRegister(a), width:Const9(width), pos:Const9(pos) , dest: DataRegister(c)} => binary_fields!((c as u32, 28), (pos as u32, 23), (0x3,21), (width as u32, 16), (a as u32, 8), (0x37, 0)),
            Instr::JZT { src: DataRegister(a), n, target } =>binary_fields!((0,31),(target as u32 & 0x7fff, 16), ( (n &0xf) as u32, 12), (a as u32,8), ((n >>4 & 0x1) as u32 ,7), (0x6f, 0))
        }
    }
}          