//! # DESCRIPTION:
//!
//!	The module provides the TSIM support for TC162 Tricore.
//!
//!
//! FILE HISTORY:
//!
//!    V1  01.2023
//!
//**************************************************************************************************
// The source code is under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
//**************************************************************************************************
use core::arch::asm;

use crate::vio_syscalls::VioSyscalls;

#[no_mangle]
#[allow(unused_variables)]
pub fn write(fd: u32, buf: *const u8, len: u32) -> i32 {
    let x: u32 = VioSyscalls::SysWrite as u32;
    unsafe {
        asm!(
            "mov %d4,{fd}",
            "mov %d5,{len}",

            "mov.aa %a4, {buf}",
            "mov %d12, {x}",
            "j ___virtio",
            x = in(reg32) x,
            fd = in (reg32) fd,
            len = in (reg32) len,
            buf = in (reg_ptr) buf
        );
    }
    return 0;
}
