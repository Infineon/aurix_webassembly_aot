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
use core::arch::global_asm;

extern "C" {
    pub fn ___virtio_hnd() -> ();
    pub fn ___virtio() -> ();
}

#[no_mangle]
pub fn __virtio_dummy_hnd() -> () {
    unsafe {
        ___virtio();
    }
}
global_asm! {
    ".ascii \"_vio\"	",
    ".globl ___virtio_hnd",
    ".type ___virtio_hnd,@function",
    ".globl ___virtio	",
    ".type ___virtio,@function",
    "___virtio_hnd:		",
    "___virtio:		",
    "call __errno	",
    "st.w [%a2],%d12	",
    "mov %d2,%d11	",
    "ret",
}
