//! # DESCRIPTION:
//!
//!	 The module provides the __errno and _Errno symbols only for the non-error build, not the
//!   functionality.
//!

use core::ptr::addr_of_mut;
//**************************************************************************************************
//* The source code is under the Apache License v2.0 with LLVM Exceptions.
//* See https://llvm.org/LICENSE.txt for license information.
//**************************************************************************************************
#[no_mangle]
#[used]
static mut _Errno: i32 = 0;

#[no_mangle]
pub unsafe fn __errno() -> *mut i32 {
    let errno = addr_of_mut!(_Errno) as *mut i32;
    return errno;
}
