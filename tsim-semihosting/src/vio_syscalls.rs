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
#[allow(dead_code)]
pub enum VioSyscalls {
    SysOpen = 0x01,
    SysClose = 0x02,
    SysLseek = 0x03,
    SysRead = 0x04,
    SysWrite = 0x05,
    SysCreat = 0x06,
    SysUnlink = 0x07,
    SysStat = 0x08,
    SysFstat = 0x09,
    SysGetTime = 0x0a,
    SysRename = 0x0d,
}
