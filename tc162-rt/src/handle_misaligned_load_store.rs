use core::arch::asm;
use core::hint::unreachable_unchecked;
#[link_section = ".ramcode"]
static mut HANDLE_MISALIGNED_ACCESS_CODE_PATCH: [u32; 0x1000] = [0; 0x1000];

#[derive(Clone, Copy)]
struct AddressRegister(u8);

#[derive(Clone, Copy)]
struct DataRegister(u8);

#[derive(Clone, Copy)]

struct ExtendedRegister(u8);

struct Offset(u16);

enum Instr{
    LDHU { dest: DataRegister, base: AddressRegister, offset: Offset },
    LDW { dest: DataRegister, base: AddressRegister, offset: Offset },
    LDD { dest: ExtendedRegister, base: AddressRegister, offset: Offset },
    STB { src: DataRegister, base: AddressRegister, offset: Offset },
    STH { src: DataRegister, base: AddressRegister, offset: Offset },
    SVLCX,
    RSLCX,
    SH { src: DataRegister, count: i8, dest: DataRegister },
    SHA { src: DataRegister, count: i8, dest: DataRegister },
    OR { lhs: DataRegister, rhs: DataRegister, dest: DataRegister },
    JI {target: AddressRegister},
    LEA {dest: AddressRegister, base: AddressRegister, offset: Offset},
    RFE,
    DEXTR {src_lower: DataRegister, src_higher: DataRegister, dest: DataRegister, pos: u8}
}

impl Instr {
    fn map_to_binary(&self) -> u32 {
        match self {
            Instr::LDHU { dest, base, offset } => {
                let mut result = 0xb9;
                result |= (dest.0 as u32) << 8;
                result |= (base.0 as u32) << 12;
                let lower_offset = offset.0 & 0x3f;
                let mid_offset = (offset.0 >> 6) & 0xf;
                let high_offset = (offset.0 >> 10) & 0x3f;
                result |= (lower_offset as u32) << 16;
                result |= (high_offset as u32) << 22;
                result |= (mid_offset as u32) << 28;
                result
            }
            Instr::LDW { dest, base, offset } => {
                let mut result = 0x19;
                result |= (dest.0 as u32) << 8;
                result |= (base.0 as u32) << 12;
                let lower_offset = offset.0 & 0x3f;
                let mid_offset = (offset.0 >> 6) & 0xf;
                let high_offset = (offset.0 >> 10) & 0x3f;
                result |= (lower_offset as u32) << 16;
                result |= (high_offset as u32) << 22;
                result |= (mid_offset as u32) << 28;
                result
            }
            Instr::LDD { dest, base, offset } => {
                let mut result = 0x09;
                result |= (dest.0 as u32) << 8;
                result |= (base.0 as u32) << 12;
                let lower_offset = offset.0 & 0x3f;
                let mid_offset = (offset.0 >> 6) & 0xf;
                let high_offset = 0x25;
                result |= (lower_offset as u32) << 16;
                result |= (high_offset as u32) << 22;
                result |= (mid_offset as u32) << 28;
                result
            }
            Instr::STB { src, base, offset } => {
                let mut result = 0xe9;
                result |= (src.0 as u32) << 8;
                result |= (base.0 as u32) << 12;
                let lower_offset = offset.0 & 0x3f;
                let mid_offset = (offset.0 >> 6) & 0xf;
                let high_offset = (offset.0 >> 10) & 0x3f;
                result |= (lower_offset as u32) << 16;
                result |= (high_offset as u32) << 22;
                result |= (mid_offset as u32) << 28;
                result
            }
            Instr::STH { src, base, offset } => {
                let mut result = 0xf9;
                result |= (src.0 as u32) << 8;
                result |= (base.0 as u32) << 12;
                let lower_offset = offset.0 & 0x3f;
                let mid_offset = (offset.0 >> 6) & 0xf;
                let high_offset = (offset.0 >> 10) & 0x3f;
                result |= (lower_offset as u32) << 16;
                result |= (high_offset as u32) << 22;
                result |= (mid_offset as u32)<< 28;
                result
            }
            Instr::SVLCX => 0x200000d,
            Instr::RSLCX => 0x240000d,
            Instr::SH { src, count, dest } => {
                let mut result = 0x8f;
                result |= (src.0 as u32) << 8;
                result |= ((*count as u32) & 0x3f) << 12; 
                result |= (dest.0 as u32) << 28;
                result
            },
            Instr::SHA { src, count, dest } => {
                let mut result = 0x8f;
                result |= (src.0 as u32) << 8;
                result |= ((*count as u32) & 0x3f) << 12;
                result |= 0x1 << 21;
                result |= (dest.0 as u32) << 28;
                result
            },
            Instr::OR { lhs, rhs, dest } => {
                let mut result = 0xf;
                result |= (lhs.0 as u32) << 8;
                result |= (rhs.0 as u32) << 12;
                result |= 0xa << 20;
                result |= (dest.0 as u32) << 28;
                result
            }
            Instr::JI { target } => {
                let mut result = 0x2d;
                result |= (target.0 as u32) << 8;
                result |= 0x3 << 20;
                result
            }
            Instr::LEA { dest, base, offset } => {
                let mut result = 0xd9;
                result |= (dest.0 as u32) << 8;
                result |= (base.0 as u32) << 12;
                let lower_offset = offset.0 & 0x3f;
                let mid_offset = (offset.0 >> 6) & 0xf;
                let high_offset = (offset.0 >> 10) & 0x3f;
                result |= (lower_offset as u32) << 16;
                result |= (high_offset as u32) << 22;
                result |= (mid_offset as u32) << 28;
                result
            }
            Instr::RFE => 0xd | (0x7 << 22),
            Instr::DEXTR { src_lower, src_higher, dest, pos } => {
                let mut result = 0x77;
                result |= (src_higher.0 as u32) << 8;
                result |= (src_lower.0 as u32) << 12;
                result |= (*pos as u32 & 0x1F) << 23;
                result |= (dest.0 as u32) << 28;
                result
            }
    }
    
}
}

#[derive(PartialEq)]
enum RegisterContext {
    LowerContext,
    UpperContext
}

fn _push_instruction(instr: Instr, index: &mut usize) {
    let binary = instr.map_to_binary();
    unsafe {
        HANDLE_MISALIGNED_ACCESS_CODE_PATCH[*index] = binary;
    }
    *index += 1;
}

fn _push_transition(context_to_be_saved: &RegisterContext, index: &mut usize, restore_context: bool) {
    if *context_to_be_saved == RegisterContext::LowerContext {
        if restore_context {
            _push_instruction(Instr::RSLCX, index);
        }
        _push_instruction(Instr::LEA{dest: AddressRegister(3), offset:Offset(4), base:AddressRegister(3)  }, index);
        _push_instruction(Instr::JI { target: AddressRegister(3) }, index);
    } else {
        _push_instruction(Instr::LEA {base: AddressRegister(11), dest: AddressRegister(11), offset: Offset(4)}, index);
        _push_instruction(Instr::RFE, index);
    }
}

/// It shall be called from Data Address Aligment trap
#[no_mangle]
pub unsafe extern "C" fn handle_misaligned_load_store() -> ! {
    unsafe{asm!("SVLCX")};
    let failed_instruction_ptr: *const u32;
    unsafe {
        asm!(
            "mov.aa {x}, %a11",
            x = out(reg_ptr) failed_instruction_ptr
        );
    }
    let failed_instruction = unsafe { *failed_instruction_ptr };
    let opcode = failed_instruction as u8;
    let data_register = ((failed_instruction >> 8) & 0xf) as u8;
    let base_register = ((failed_instruction >> 12) & 0xf) as u8;
    let mut offset = {
        let low_offset = failed_instruction >> 16 & 0x3f;
        let high_offset = failed_instruction >> 22 & 0x3f;
        let mid_offset = failed_instruction >> 28 & 0xf;
        ((high_offset << 10) | (mid_offset << 6) | low_offset) as u16
    };

    let context_to_be_saved = if data_register < 8 {
        RegisterContext::UpperContext
    } else {
        RegisterContext::LowerContext
    };

    let mut index = 0;
    let mut push_instruction = |instr: Instr| _push_instruction(instr, &mut index);
    
    match opcode {
        0xc9 => {
            // LD.H
            push_instruction(Instr::LDW {
                dest: DataRegister(data_register),
                base: AddressRegister(base_register),
                offset: Offset(offset.wrapping_sub(1)),
            });

            push_instruction(Instr::SH { src: DataRegister(data_register), count: 8, dest: DataRegister(data_register) });
            push_instruction(Instr::SHA { src: DataRegister(data_register), count: -16, dest: DataRegister(data_register) });

            _push_transition(&context_to_be_saved, &mut index, false);

        },
        0xb9 => {
            // LD.HU
            push_instruction(Instr::LDW {
                dest: DataRegister(data_register),
                base: AddressRegister(base_register),
                offset: Offset(offset.wrapping_sub(1)),
            });

            push_instruction(Instr::SH { src: DataRegister(data_register), count: 8, dest: DataRegister(data_register) });
            push_instruction(Instr::SH { src: DataRegister(data_register), count: -16, dest: DataRegister(data_register) });

            _push_transition(&context_to_be_saved, &mut index, false);
        },
        0x19 => {
            // LD.W
            if context_to_be_saved == RegisterContext::LowerContext {
                push_instruction(Instr::SVLCX);
            }
            let dest = match context_to_be_saved {
                RegisterContext::LowerContext => DataRegister(0),
                RegisterContext::UpperContext => DataRegister(8),
            };

            push_instruction(Instr::LDHU { dest:DataRegister(data_register), base: AddressRegister(base_register), offset: Offset(offset.wrapping_sub(1)) });
            push_instruction(Instr::SH { src: DataRegister(data_register), count: -8, dest: DataRegister(data_register) });
            push_instruction(Instr::LDW { dest: dest, base: AddressRegister(base_register), offset: Offset(offset + 1) });
            push_instruction(Instr::SH { src: dest, count: 8, dest });
            push_instruction(Instr::OR { lhs: dest, rhs: DataRegister(data_register), dest: DataRegister(data_register) });

            _push_transition(&context_to_be_saved, &mut index,true);
        },
        0x09 => {
            // LD.D
            offset = offset & 0x3ff;

            if context_to_be_saved == RegisterContext::LowerContext {
                push_instruction(Instr::SVLCX);
            }

            let dest = match context_to_be_saved {
                RegisterContext::LowerContext => DataRegister(0),
                RegisterContext::UpperContext => DataRegister(8),
            };


            push_instruction(Instr::LDHU { dest, base: AddressRegister(base_register), offset: Offset(offset.wrapping_sub(1)) });
            push_instruction(Instr::SH { src: dest, count: -8, dest });
            push_instruction(Instr::LDD { dest: ExtendedRegister(data_register), base: AddressRegister(base_register), offset: Offset(offset + 1) });
            push_instruction(Instr::DEXTR { src_lower: DataRegister(data_register) , src_higher: DataRegister(data_register + 1), dest: DataRegister(data_register + 1), pos: 8 });
            push_instruction(Instr::SH { src: DataRegister(data_register), count: 8, dest: DataRegister(data_register) });
            push_instruction(Instr::OR { lhs: DataRegister(data_register), rhs: dest, dest: DataRegister(data_register) });

            _push_transition(&context_to_be_saved, &mut index,true);
        },
        0xf9 => {
            //ST.H
            if context_to_be_saved == RegisterContext::LowerContext {
                push_instruction(Instr::SVLCX);
            }

            let src = match context_to_be_saved {
                RegisterContext::LowerContext => DataRegister(0),
                RegisterContext::UpperContext => DataRegister(8),
            };

            push_instruction(Instr::STB{src: DataRegister(data_register), base: AddressRegister(base_register), offset: Offset(offset)});
            push_instruction(Instr::SH { src: DataRegister(data_register), count: -8, dest: src});
            push_instruction(Instr::STB{src, base: AddressRegister(base_register), offset: Offset(offset+1)});

            _push_transition(&context_to_be_saved, &mut index,true);
        },
        0x59 => {
            // ST.W
            if context_to_be_saved == RegisterContext::LowerContext {
                push_instruction(Instr::SVLCX);
            }

            let src = match context_to_be_saved {
                RegisterContext::LowerContext => DataRegister(0),
                RegisterContext::UpperContext => DataRegister(8),
            };

            push_instruction(Instr::STB{src: DataRegister(data_register), base: AddressRegister(base_register), offset: Offset(offset)});
            push_instruction(Instr::SH { src: DataRegister(data_register), count: -8, dest: src});
            push_instruction(Instr::STH{ src, base: AddressRegister(base_register), offset: Offset(offset+1)});
            push_instruction(Instr::SH { src, count: -16, dest: src});
            push_instruction(Instr::STB{src, base: AddressRegister(base_register), offset: Offset(offset+3)});

            _push_transition(&context_to_be_saved, &mut index, true);
        },
        0x89 => {
            // ST.D
            offset = offset & 0x3ff;

            if context_to_be_saved == RegisterContext::LowerContext {
                push_instruction(Instr::SVLCX);
            }

            let src = match context_to_be_saved {
                RegisterContext::LowerContext => DataRegister(0),
                RegisterContext::UpperContext => DataRegister(8),
            };

            push_instruction(Instr::STB{src: DataRegister(data_register), base: AddressRegister(base_register), offset: Offset(offset)});
            push_instruction(Instr::SH { src: DataRegister(data_register), count: -8, dest: src});
            push_instruction(Instr::STH{ src, base: AddressRegister(base_register), offset: Offset(offset+1)});
            push_instruction(Instr::SH { src, count: -16, dest: src});
            push_instruction(Instr::STB{src, base: AddressRegister(base_register), offset: Offset(offset+3)});
            push_instruction(Instr::STB { src: DataRegister(data_register + 1) , base: AddressRegister(base_register) , offset: Offset(offset + 4) });
            push_instruction(Instr::SH { src: DataRegister(data_register + 1), count: -8, dest: src});
            push_instruction(Instr::STH { src: src, base: AddressRegister(base_register), offset: Offset(offset + 5) });
            push_instruction(Instr::SH { src, count: -16, dest: src});
            push_instruction(Instr::STB { src, base: AddressRegister(base_register), offset: Offset(offset + 7) });

            _push_transition(&context_to_be_saved, &mut index, true);
        },
        _ => {}
    }

    match context_to_be_saved {
        RegisterContext::LowerContext => {
            unsafe {
                asm!(
                    "RSLCX",
                    "mov.aa %a3, %a11",
                    "mov.aa %a11, {code_patch_ptr} ",
                    "RFE",
                    code_patch_ptr = in(reg_ptr) HANDLE_MISALIGNED_ACCESS_CODE_PATCH.as_ptr(),
                    out("a3") _,
                );
            }
        }
        RegisterContext::UpperContext => {
            unsafe {
                asm!(
                    "RSLCX",
                    "mov.aa %a3, {code_patch_ptr}",
                    "ji %a3",
                    code_patch_ptr = in(reg_ptr) HANDLE_MISALIGNED_ACCESS_CODE_PATCH.as_ptr(),
                    out("a3") _,
                );
            }
        }
       
    }

    unreachable_unchecked()
   
}
