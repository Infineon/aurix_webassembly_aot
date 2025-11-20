#![forbid(unsafe_code)]
use defmt::Format;
use crate::{isa_model::{AddressRegister, Const4, Const9, Const10, Const16, DataRegister, ExtendedRegister, Register, RegisterOrConst, RegisterOrLargeConst, RegisterOrSmallConst}, translator::LabelIndex};
#[cfg(feature = "full_instructions")]
use crate::isa_model::Const18;


macro_rules! define_instr {
    (
        $(
            $(#[$core_attr:meta])*
            $core_variant:ident $({$($core_field:tt)*})?
        ),* $(,)?
        $(;
        // Full instruction variants
        $(
            $(#[$full_attr:meta])*
            $full_variant:ident $({$($full_field:tt)*})?
        ),* $(,)?)?
    ) => {
        #[derive(Debug, Clone, Format)]
        pub(crate) enum Instr {
            $(
                $(#[$core_attr])*
                $core_variant $({$($core_field)*})?,
            )*
            $(
                $(
                    #[cfg(feature="full_instructions")]
                    $(#[$full_attr])*
                    $full_variant $({$($full_field)*})?,
                )*
            )?
        }
    };
}


define_instr! {
    RET,
    MOV {src: RegisterOrLargeConst, dest: Register},
    MOVU {src: Const16, dest: DataRegister},
    MOVHA {src: Const16, dest: AddressRegister},
    LEA {base: AddressRegister, offset: Const16, dest: AddressRegister},
    LDA {base: AddressRegister, offset: Const16, dest: AddressRegister},
    LDB {base: AddressRegister, offset: Const16, dest: DataRegister},
    LDBU {base: AddressRegister, offset: Const16, dest: DataRegister},
    LDH {base: AddressRegister, offset: Const16, dest: DataRegister},
    LDHU {base: AddressRegister, offset: Const16, dest: DataRegister},
    LDW {base: AddressRegister, offset: Const16, dest: DataRegister},
    LDWPI {base: AddressRegister, offset: Const10, dest: DataRegister}, // PI is post increment here
    LDD {base: AddressRegister, offset: Const10, dest: ExtendedRegister},
    LDDPI {base: AddressRegister, offset: Const10, dest: ExtendedRegister}, // same as above
    STB {src: DataRegister, base: AddressRegister, offset: Const16},
    STH {src: DataRegister, base: AddressRegister, offset: Const16},
    STW {src: DataRegister, base: AddressRegister, offset: Const16},
    STD {src: ExtendedRegister, base: AddressRegister, offset: Const10},
    STDPI {src: ExtendedRegister, base: AddressRegister, offset: Const10}, // same as below but for double word
    STWPI {src: DataRegister, base: AddressRegister, offset: Const10}, // PI is pre increment here, same instruction as STW but different adressing mode
    CLZ {src:DataRegister, dest:DataRegister},
    SHUFFLE {src:DataRegister, dest:DataRegister, mask:Const9 },
    POPCNT {src:DataRegister, dest:DataRegister},
    AND {lhs:DataRegister, rhs:RegisterOrConst, dest:DataRegister},
    XOR {lhs:DataRegister, rhs:RegisterOrConst, dest:DataRegister},
    CALL {target: u32},
    CALLI {target: AddressRegister},
    SVLCX,
    RSLCX,
    EQ {lhs:DataRegister, rhs:RegisterOrConst, dest:DataRegister},
    NE {lhs:DataRegister, rhs:RegisterOrConst, dest:DataRegister},
    ADD {lhs: DataRegister, rhs: RegisterOrConst, dest: DataRegister},
    ADDF {lhs:DataRegister, rhs:DataRegister, dest:DataRegister},
    ADDX {lhs:DataRegister, rhs: RegisterOrConst, dest:DataRegister},
    ADDC {lhs:DataRegister, rhs:RegisterOrConst, dest:DataRegister},
    ADDI {lhs: DataRegister, rhs: Const16, dest: DataRegister},
    ADDIH {lhs: DataRegister, rhs: Const16, dest: DataRegister},
    ADDIHA { lhs: AddressRegister, rhs: Const16, dest: AddressRegister},
    SUB {lhs: DataRegister, rhs:DataRegister, dest:DataRegister},
    SUBF {lhs:DataRegister, rhs:DataRegister, dest:DataRegister},
    SUBX {lhs:DataRegister, rhs:DataRegister, dest:DataRegister},
    SUBC {lhs:DataRegister, rhs:DataRegister, dest:DataRegister},
    OR {lhs:DataRegister, rhs:RegisterOrConst, dest:DataRegister},
    LT {lhs:DataRegister, rhs:RegisterOrConst, dest:DataRegister},
    ORLT {lhs:DataRegister, rhs:RegisterOrConst, dest:DataRegister},
    ANDLTU {lhs:DataRegister, rhs:RegisterOrConst, dest:DataRegister},
    ORLTU {lhs:DataRegister, rhs:RegisterOrConst, dest:DataRegister},
    ANDGEU {lhs:DataRegister, rhs:RegisterOrConst, dest:DataRegister},
    LTU {lhs:DataRegister, rhs:RegisterOrConst, dest:DataRegister},
    GE {lhs:DataRegister, rhs:RegisterOrConst, dest:DataRegister},
    GEU {lhs:DataRegister, rhs:RegisterOrConst, dest:DataRegister},
    FTOIZ {src:DataRegister, dest:DataRegister},
    FTOUZ {src:DataRegister, dest:DataRegister},
    ITOF {src:DataRegister, dest:DataRegister},
    UTOF {src:DataRegister, dest:DataRegister},
    MUL {lhs:DataRegister, rhs:RegisterOrConst, dest:DataRegister},
    MULU {lhs:DataRegister, rhs:RegisterOrConst, dest:ExtendedRegister},
    MADD {lhs:DataRegister, rhs:DataRegister, acc:DataRegister, dest:DataRegister},
    MULF {lhs:DataRegister, rhs:DataRegister, dest:DataRegister},
    DIV {lhs:DataRegister, rhs:DataRegister, dest:ExtendedRegister},
    DIVU{lhs:DataRegister, rhs:DataRegister, dest:ExtendedRegister},
    DIVF{lhs:DataRegister, rhs:DataRegister, dest:DataRegister},
    SH {src:DataRegister, count:RegisterOrConst, dest:DataRegister},
    SHA {src: DataRegister, count:RegisterOrConst, dest:DataRegister},
    CMPF{lhs: DataRegister, rhs: DataRegister, dest: DataRegister},
    SEL {selector:DataRegister, lhs:DataRegister, rhs:RegisterOrConst, dest:DataRegister},
    SELN {selector:DataRegister, lhs:DataRegister, rhs:RegisterOrConst, dest:DataRegister},
    RSUB { lhs : Const9, rhs: DataRegister, dest: DataRegister},
    RSUB0 { src: DataRegister},
    J {target: LabelIndex},
    JEQ {target: LabelIndex, lhs: DataRegister, rhs: RegisterOrSmallConst },
    JI {src: AddressRegister},
    ADDSCA{lhs: AddressRegister, rhs: DataRegister, dest: AddressRegister, shift: Const4},
    EXTRUI{src: DataRegister, width: Const9, pos: Const9, dest: DataRegister},
    LOOPU {target: LabelIndex},
    MOVAA {src: AddressRegister, dest: AddressRegister},
    MINU {lhs: DataRegister, rhs: RegisterOrConst, dest: DataRegister}

    ; // Semi colon used by macro to separate additional variants included with "full_instructions" feature
    // These variants are not curently used by the translator but are kept "just in case"
    
    NOP,
    MOVA{src: DataRegister, dest: AddressRegister},
    LDAABS {address: Const18, dest: AddressRegister},
    LDAPI {base: AddressRegister, offset: Const10, dest: AddressRegister},
    LDWABS {address: Const18, dest: DataRegister},
    LDDABS {address: Const18, dest: ExtendedRegister},
    STWABS {src: DataRegister, address: Const18},
    STDABS {src: ExtendedRegister, address: Const18},
    MOVH {src: Const16, dest: DataRegister},
    ADDA {lhs: AddressRegister, rhs: AddressRegister, dest: AddressRegister},
    CADDN {lhs: DataRegister, rhs: RegisterOrConst, cond: DataRegister,  dest: DataRegister},
    ANDLT {lhs:DataRegister, rhs:RegisterOrConst, dest:DataRegister},
    ANDGE {lhs:DataRegister, rhs:RegisterOrConst, dest:DataRegister},
    ORGE {lhs:DataRegister, rhs:RegisterOrConst, dest:DataRegister},
    ORGEU {lhs:DataRegister, rhs:RegisterOrConst, dest:DataRegister},
    JNE {target: LabelIndex, lhs: DataRegister, rhs: Const4 },
    EXTRU{src: DataRegister, width_pos: ExtendedRegister, dest: DataRegister},
    JZT {src: DataRegister, n: u8, target: LabelIndex},
}