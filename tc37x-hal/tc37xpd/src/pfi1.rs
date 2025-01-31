/*
*****************************************************************************
	*
	* Copyright (C) 2024 Infineon Technologies AG. All rights reserved.
	*
	* Infineon Technologies AG (Infineon) is supplying this software for use with
	* Infineon's microcontrollers. This file can be freely distributed within
	* development tools that are supporting such microcontrollers.
	*
	* THIS SOFTWARE IS PROVIDED "AS IS". NO WARRANTIES, WHETHER EXPRESS, IMPLIED
	* OR STATUTORY, INCLUDING, BUT NOT LIMITED TO, IMPLIED WARRANTIES OF
	* MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE APPLY TO THIS SOFTWARE.
	* INFINEON SHALL NOT, IN ANY CIRCUMSTANCES, BE LIABLE FOR SPECIAL, INCIDENTAL,
	* OR CONSEQUENTIAL DAMAGES, FOR ANY REASON WHATSOEVER.
	*
	******************************************************************************
*/
#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"PFI"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfi1(pub(super) *mut u8);
unsafe impl core::marker::Send for Pfi1 {}
unsafe impl core::marker::Sync for Pfi1 {}
impl Pfi1 {
    #[doc = "ADDMOD 0 Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn addmod0(&self) -> crate::common::Reg<self::Addmod0_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(274496usize)) }
    }

    #[doc = "ADDMOD 1 Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn addmod1(&self) -> crate::common::Reg<self::Addmod1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(274528usize)) }
    }

    #[doc = "Bitline Address Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn bladdr(&self) -> crate::common::Reg<self::Bladdr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(274464usize)) }
    }

    #[doc = "ECC Read Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn eccr(&self) -> crate::common::Reg<self::Eccr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "ECC Status Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn eccs(&self) -> crate::common::Reg<self::Eccs_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "Fail Count Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn failcount(&self) -> crate::common::Reg<self::Failcount_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(274432usize)) }
    }
    #[doc = "BITLINE"]
    #[inline(always)]
    pub fn bitline(self) -> [self::Bitline; 256] {
        unsafe {
            [
                self::Bitline(self.0.add(0x40000usize + 0x0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x20usize)),
                self::Bitline(self.0.add(0x40000usize + 0x40usize)),
                self::Bitline(self.0.add(0x40000usize + 0x60usize)),
                self::Bitline(self.0.add(0x40000usize + 0x80usize)),
                self::Bitline(self.0.add(0x40000usize + 0xa0usize)),
                self::Bitline(self.0.add(0x40000usize + 0xc0usize)),
                self::Bitline(self.0.add(0x40000usize + 0xe0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x100usize)),
                self::Bitline(self.0.add(0x40000usize + 0x120usize)),
                self::Bitline(self.0.add(0x40000usize + 0x140usize)),
                self::Bitline(self.0.add(0x40000usize + 0x160usize)),
                self::Bitline(self.0.add(0x40000usize + 0x180usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1a0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1c0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1e0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x200usize)),
                self::Bitline(self.0.add(0x40000usize + 0x220usize)),
                self::Bitline(self.0.add(0x40000usize + 0x240usize)),
                self::Bitline(self.0.add(0x40000usize + 0x260usize)),
                self::Bitline(self.0.add(0x40000usize + 0x280usize)),
                self::Bitline(self.0.add(0x40000usize + 0x2a0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x2c0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x2e0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x300usize)),
                self::Bitline(self.0.add(0x40000usize + 0x320usize)),
                self::Bitline(self.0.add(0x40000usize + 0x340usize)),
                self::Bitline(self.0.add(0x40000usize + 0x360usize)),
                self::Bitline(self.0.add(0x40000usize + 0x380usize)),
                self::Bitline(self.0.add(0x40000usize + 0x3a0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x3c0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x3e0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x400usize)),
                self::Bitline(self.0.add(0x40000usize + 0x420usize)),
                self::Bitline(self.0.add(0x40000usize + 0x440usize)),
                self::Bitline(self.0.add(0x40000usize + 0x460usize)),
                self::Bitline(self.0.add(0x40000usize + 0x480usize)),
                self::Bitline(self.0.add(0x40000usize + 0x4a0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x4c0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x4e0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x500usize)),
                self::Bitline(self.0.add(0x40000usize + 0x520usize)),
                self::Bitline(self.0.add(0x40000usize + 0x540usize)),
                self::Bitline(self.0.add(0x40000usize + 0x560usize)),
                self::Bitline(self.0.add(0x40000usize + 0x580usize)),
                self::Bitline(self.0.add(0x40000usize + 0x5a0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x5c0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x5e0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x600usize)),
                self::Bitline(self.0.add(0x40000usize + 0x620usize)),
                self::Bitline(self.0.add(0x40000usize + 0x640usize)),
                self::Bitline(self.0.add(0x40000usize + 0x660usize)),
                self::Bitline(self.0.add(0x40000usize + 0x680usize)),
                self::Bitline(self.0.add(0x40000usize + 0x6a0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x6c0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x6e0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x700usize)),
                self::Bitline(self.0.add(0x40000usize + 0x720usize)),
                self::Bitline(self.0.add(0x40000usize + 0x740usize)),
                self::Bitline(self.0.add(0x40000usize + 0x760usize)),
                self::Bitline(self.0.add(0x40000usize + 0x780usize)),
                self::Bitline(self.0.add(0x40000usize + 0x7a0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x7c0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x7e0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x800usize)),
                self::Bitline(self.0.add(0x40000usize + 0x820usize)),
                self::Bitline(self.0.add(0x40000usize + 0x840usize)),
                self::Bitline(self.0.add(0x40000usize + 0x860usize)),
                self::Bitline(self.0.add(0x40000usize + 0x880usize)),
                self::Bitline(self.0.add(0x40000usize + 0x8a0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x8c0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x8e0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x900usize)),
                self::Bitline(self.0.add(0x40000usize + 0x920usize)),
                self::Bitline(self.0.add(0x40000usize + 0x940usize)),
                self::Bitline(self.0.add(0x40000usize + 0x960usize)),
                self::Bitline(self.0.add(0x40000usize + 0x980usize)),
                self::Bitline(self.0.add(0x40000usize + 0x9a0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x9c0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x9e0usize)),
                self::Bitline(self.0.add(0x40000usize + 0xa00usize)),
                self::Bitline(self.0.add(0x40000usize + 0xa20usize)),
                self::Bitline(self.0.add(0x40000usize + 0xa40usize)),
                self::Bitline(self.0.add(0x40000usize + 0xa60usize)),
                self::Bitline(self.0.add(0x40000usize + 0xa80usize)),
                self::Bitline(self.0.add(0x40000usize + 0xaa0usize)),
                self::Bitline(self.0.add(0x40000usize + 0xac0usize)),
                self::Bitline(self.0.add(0x40000usize + 0xae0usize)),
                self::Bitline(self.0.add(0x40000usize + 0xb00usize)),
                self::Bitline(self.0.add(0x40000usize + 0xb20usize)),
                self::Bitline(self.0.add(0x40000usize + 0xb40usize)),
                self::Bitline(self.0.add(0x40000usize + 0xb60usize)),
                self::Bitline(self.0.add(0x40000usize + 0xb80usize)),
                self::Bitline(self.0.add(0x40000usize + 0xba0usize)),
                self::Bitline(self.0.add(0x40000usize + 0xbc0usize)),
                self::Bitline(self.0.add(0x40000usize + 0xbe0usize)),
                self::Bitline(self.0.add(0x40000usize + 0xc00usize)),
                self::Bitline(self.0.add(0x40000usize + 0xc20usize)),
                self::Bitline(self.0.add(0x40000usize + 0xc40usize)),
                self::Bitline(self.0.add(0x40000usize + 0xc60usize)),
                self::Bitline(self.0.add(0x40000usize + 0xc80usize)),
                self::Bitline(self.0.add(0x40000usize + 0xca0usize)),
                self::Bitline(self.0.add(0x40000usize + 0xcc0usize)),
                self::Bitline(self.0.add(0x40000usize + 0xce0usize)),
                self::Bitline(self.0.add(0x40000usize + 0xd00usize)),
                self::Bitline(self.0.add(0x40000usize + 0xd20usize)),
                self::Bitline(self.0.add(0x40000usize + 0xd40usize)),
                self::Bitline(self.0.add(0x40000usize + 0xd60usize)),
                self::Bitline(self.0.add(0x40000usize + 0xd80usize)),
                self::Bitline(self.0.add(0x40000usize + 0xda0usize)),
                self::Bitline(self.0.add(0x40000usize + 0xdc0usize)),
                self::Bitline(self.0.add(0x40000usize + 0xde0usize)),
                self::Bitline(self.0.add(0x40000usize + 0xe00usize)),
                self::Bitline(self.0.add(0x40000usize + 0xe20usize)),
                self::Bitline(self.0.add(0x40000usize + 0xe40usize)),
                self::Bitline(self.0.add(0x40000usize + 0xe60usize)),
                self::Bitline(self.0.add(0x40000usize + 0xe80usize)),
                self::Bitline(self.0.add(0x40000usize + 0xea0usize)),
                self::Bitline(self.0.add(0x40000usize + 0xec0usize)),
                self::Bitline(self.0.add(0x40000usize + 0xee0usize)),
                self::Bitline(self.0.add(0x40000usize + 0xf00usize)),
                self::Bitline(self.0.add(0x40000usize + 0xf20usize)),
                self::Bitline(self.0.add(0x40000usize + 0xf40usize)),
                self::Bitline(self.0.add(0x40000usize + 0xf60usize)),
                self::Bitline(self.0.add(0x40000usize + 0xf80usize)),
                self::Bitline(self.0.add(0x40000usize + 0xfa0usize)),
                self::Bitline(self.0.add(0x40000usize + 0xfc0usize)),
                self::Bitline(self.0.add(0x40000usize + 0xfe0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1000usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1020usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1040usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1060usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1080usize)),
                self::Bitline(self.0.add(0x40000usize + 0x10a0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x10c0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x10e0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1100usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1120usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1140usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1160usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1180usize)),
                self::Bitline(self.0.add(0x40000usize + 0x11a0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x11c0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x11e0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1200usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1220usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1240usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1260usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1280usize)),
                self::Bitline(self.0.add(0x40000usize + 0x12a0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x12c0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x12e0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1300usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1320usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1340usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1360usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1380usize)),
                self::Bitline(self.0.add(0x40000usize + 0x13a0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x13c0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x13e0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1400usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1420usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1440usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1460usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1480usize)),
                self::Bitline(self.0.add(0x40000usize + 0x14a0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x14c0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x14e0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1500usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1520usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1540usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1560usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1580usize)),
                self::Bitline(self.0.add(0x40000usize + 0x15a0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x15c0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x15e0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1600usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1620usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1640usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1660usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1680usize)),
                self::Bitline(self.0.add(0x40000usize + 0x16a0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x16c0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x16e0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1700usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1720usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1740usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1760usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1780usize)),
                self::Bitline(self.0.add(0x40000usize + 0x17a0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x17c0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x17e0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1800usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1820usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1840usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1860usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1880usize)),
                self::Bitline(self.0.add(0x40000usize + 0x18a0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x18c0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x18e0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1900usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1920usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1940usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1960usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1980usize)),
                self::Bitline(self.0.add(0x40000usize + 0x19a0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x19c0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x19e0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1a00usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1a20usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1a40usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1a60usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1a80usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1aa0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1ac0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1ae0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1b00usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1b20usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1b40usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1b60usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1b80usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1ba0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1bc0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1be0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1c00usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1c20usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1c40usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1c60usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1c80usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1ca0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1cc0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1ce0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1d00usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1d20usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1d40usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1d60usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1d80usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1da0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1dc0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1de0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1e00usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1e20usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1e40usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1e60usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1e80usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1ea0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1ec0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1ee0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1f00usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1f20usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1f40usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1f60usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1f80usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1fa0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1fc0usize)),
                self::Bitline(self.0.add(0x40000usize + 0x1fe0usize)),
            ]
        }
    }
    #[doc = "DBAB"]
    #[inline(always)]
    pub fn dbab(self) -> [self::Dbab; 2] {
        unsafe {
            [
                self::Dbab(self.0.add(0x4000usize + 0x0usize)),
                self::Dbab(self.0.add(0x4000usize + 0x20usize)),
            ]
        }
    }
    #[doc = "MBAB"]
    #[inline(always)]
    pub fn mbab(self) -> self::Mbab {
        unsafe { self::Mbab(self.0.add(32768usize)) }
    }
    #[doc = "SBAB"]
    #[inline(always)]
    pub fn sbab(self) -> [self::Sbab; 17] {
        unsafe {
            [
                self::Sbab(self.0.add(0x2000usize + 0x0usize)),
                self::Sbab(self.0.add(0x2000usize + 0x20usize)),
                self::Sbab(self.0.add(0x2000usize + 0x40usize)),
                self::Sbab(self.0.add(0x2000usize + 0x60usize)),
                self::Sbab(self.0.add(0x2000usize + 0x80usize)),
                self::Sbab(self.0.add(0x2000usize + 0xa0usize)),
                self::Sbab(self.0.add(0x2000usize + 0xc0usize)),
                self::Sbab(self.0.add(0x2000usize + 0xe0usize)),
                self::Sbab(self.0.add(0x2000usize + 0x100usize)),
                self::Sbab(self.0.add(0x2000usize + 0x120usize)),
                self::Sbab(self.0.add(0x2000usize + 0x140usize)),
                self::Sbab(self.0.add(0x2000usize + 0x160usize)),
                self::Sbab(self.0.add(0x2000usize + 0x180usize)),
                self::Sbab(self.0.add(0x2000usize + 0x1a0usize)),
                self::Sbab(self.0.add(0x2000usize + 0x1c0usize)),
                self::Sbab(self.0.add(0x2000usize + 0x1e0usize)),
                self::Sbab(self.0.add(0x2000usize + 0x200usize)),
            ]
        }
    }
    #[doc = "SECTOR"]
    #[inline(always)]
    pub fn sector(self) -> [self::Sector; 8] {
        unsafe {
            [
                self::Sector(self.0.add(0x42000usize + 0x0usize)),
                self::Sector(self.0.add(0x42000usize + 0x20usize)),
                self::Sector(self.0.add(0x42000usize + 0x40usize)),
                self::Sector(self.0.add(0x42000usize + 0x60usize)),
                self::Sector(self.0.add(0x42000usize + 0x80usize)),
                self::Sector(self.0.add(0x42000usize + 0xa0usize)),
                self::Sector(self.0.add(0x42000usize + 0xc0usize)),
                self::Sector(self.0.add(0x42000usize + 0xe0usize)),
            ]
        }
    }
    #[doc = "ZBAB"]
    #[inline(always)]
    pub fn zbab(self) -> [self::Zbab; 4] {
        unsafe {
            [
                self::Zbab(self.0.add(0xc000usize + 0x0usize)),
                self::Zbab(self.0.add(0xc000usize + 0x20usize)),
                self::Zbab(self.0.add(0xc000usize + 0x40usize)),
                self::Zbab(self.0.add(0xc000usize + 0x60usize)),
            ]
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addmod0_SPEC;
impl crate::sealed::RegSpec for Addmod0_SPEC {
    type DataType = u32;
}
#[doc = "ADDMOD 0 Register\n resetvalue={System Reset:0x0}"]
pub type Addmod0 = crate::RegValueT<Addmod0_SPEC>;

impl Addmod0 {
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data9(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Addmod0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Addmod0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Addmod0 {
    #[inline(always)]
    fn default() -> Addmod0 {
        <crate::RegValueT<Addmod0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addmod1_SPEC;
impl crate::sealed::RegSpec for Addmod1_SPEC {
    type DataType = u32;
}
#[doc = "ADDMOD 1 Register\n resetvalue={System Reset:0x0}"]
pub type Addmod1 = crate::RegValueT<Addmod1_SPEC>;

impl Addmod1 {
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data32(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data33(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data34(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data35(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data36(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data37(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data38(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data39(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data40(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data41(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data42(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data43(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data44(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data45(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data46(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data47(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data48(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data49(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data50(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data51(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data52(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data53(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data54(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data55(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data56(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data57(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data58(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data59(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data60(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data61(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data62(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Modifier Data. Test data."]
    #[inline(always)]
    pub fn data63(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Addmod1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Addmod1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Addmod1 {
    #[inline(always)]
    fn default() -> Addmod1 {
        <crate::RegValueT<Addmod1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bladdr_SPEC;
impl crate::sealed::RegSpec for Bladdr_SPEC {
    type DataType = u32;
}
#[doc = "Bitline Address Register\n resetvalue={System Reset:0x0}"]
pub type Bladdr = crate::RegValueT<Bladdr_SPEC>;

impl Bladdr {
    #[doc = "Address   ADDR. Test."]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<5, 0x7fffff, 1, 0, u32, Bladdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x7fffff,1,0,u32, Bladdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Bladdr {
    #[inline(always)]
    fn default() -> Bladdr {
        <crate::RegValueT<Bladdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccr_SPEC;
impl crate::sealed::RegSpec for Eccr_SPEC {
    type DataType = u32;
}
#[doc = "ECC Read Register\n resetvalue={System Reset:0x0}"]
pub type Eccr = crate::RegValueT<Eccr_SPEC>;

impl Eccr {
    #[doc = "Error Correction Read Code   RCODE. ECC code  read from the Flash read buffer with last data read operation."]
    #[inline(always)]
    pub fn rcode(
        self,
    ) -> crate::common::RegisterField<0, 0x3fffff, 1, 0, u32, Eccr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3fffff,1,0,u32, Eccr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Eccr {
    #[inline(always)]
    fn default() -> Eccr {
        <crate::RegValueT<Eccr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eccs_SPEC;
impl crate::sealed::RegSpec for Eccs_SPEC {
    type DataType = u32;
}
#[doc = "ECC Status Register\n resetvalue={System Reset:0x0}"]
pub type Eccs = crate::RegValueT<Eccs_SPEC>;

impl Eccs {
    #[doc = "Read Access Single Bit ECC Error   ERR1. The flag reports a single bit ECC failure during the last NVM read access."]
    #[inline(always)]
    pub fn err1(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, eccs::Err1, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,eccs::Err1, Eccs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Read Access Double Bit ECC Error   ERR2. The flag reports a double bit ECC failure during the last NVM read access."]
    #[inline(always)]
    pub fn err2(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, eccs::Err2, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,eccs::Err2, Eccs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Read Access Multi bit ECC Error   ERRM. The flag reports multi bit ECC failure during the last NVM read access."]
    #[inline(always)]
    pub fn errm(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, eccs::Errm, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1,1,0,eccs::Errm, Eccs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Read Access ECC Error Within the Address   ERRA. The flag reports an address error during the last NVM read access."]
    #[inline(always)]
    pub fn erra(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, eccs::Erra, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x1,1,0,eccs::Erra, Eccs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Read Access All Zeros   ALL0. The flag reports the All Zeros condition during the last NVM read access."]
    #[inline(always)]
    pub fn all0(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, eccs::All0, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x1,1,0,eccs::All0, Eccs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "All Ones   ALL1. The flag reports the All Ones condition during the last NVM read access."]
    #[inline(always)]
    pub fn all1(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, eccs::All1, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x1,1,0,eccs::All1, Eccs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Any Read Access ECC Error   ERRANY. The flag reports any ECC failure during the last NVM read access."]
    #[inline(always)]
    pub fn errany(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, eccs::Errany, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterField::<7,0x1,1,0,eccs::Errany, Eccs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Accumulated Single Bit ECC Errors   AER1. The flag accumulates single bit failures during NVM read operations."]
    #[inline(always)]
    pub fn aer1(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, eccs::Aer1, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x1,1,0,eccs::Aer1, Eccs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Accumulated Double Bit ECC Errors   AER2. The flag accumulates double bit failures during NVM read operations."]
    #[inline(always)]
    pub fn aer2(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, eccs::Aer2, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterField::<17,0x1,1,0,eccs::Aer2, Eccs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Accumulated Multi bit ECC Errors   AERM. The flag accumulates multi bit failures during NVM read accesses."]
    #[inline(always)]
    pub fn aerm(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, eccs::Aerm, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterField::<19,0x1,1,0,eccs::Aerm, Eccs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Accumulated ECC Error Within the Address   ARRA. The flag accumulates an address errors during NVM read accesses."]
    #[inline(always)]
    pub fn arra(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, eccs::Arra, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterField::<20,0x1,1,0,eccs::Arra, Eccs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Accumulated All Zeros   AAL0. The flag accumulates the All Zeros condition during NVM read accesses."]
    #[inline(always)]
    pub fn aal0(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, eccs::Aal0, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterField::<21,0x1,1,0,eccs::Aal0, Eccs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Accumulated All Ones   AAL1. The flag accumulates the All Ones condition during NVM read accesses."]
    #[inline(always)]
    pub fn aal1(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, eccs::Aal1, Eccs_SPEC, crate::common::R> {
        crate::common::RegisterField::<22,0x1,1,0,eccs::Aal1, Eccs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Accumulated Any Read Access ECC Error   AERANY. The status bit accumulates ECC failures during NVM read accesses."]
    #[inline(always)]
    pub fn aerany(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, eccs::Aerany, Eccs_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<23,0x1,1,0,eccs::Aerany, Eccs_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Eccs {
    #[inline(always)]
    fn default() -> Eccs {
        <crate::RegValueT<Eccs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eccs {
    pub struct Err1_SPEC;
    pub type Err1 = crate::EnumBitfieldStruct<u8, Err1_SPEC>;
    impl Err1 {
        #[doc = "0 No single bit        ECC failure occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A single bit        ECC failure occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Err2_SPEC;
    pub type Err2 = crate::EnumBitfieldStruct<u8, Err2_SPEC>;
    impl Err2 {
        #[doc = "0 No double bit        ECC failure occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A double bit        ECC failure occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Errm_SPEC;
    pub type Errm = crate::EnumBitfieldStruct<u8, Errm_SPEC>;
    impl Errm {
        #[doc = "0 No multi bit ECC failure occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Multi bit ECC        failure occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Erra_SPEC;
    pub type Erra = crate::EnumBitfieldStruct<u8, Erra_SPEC>;
    impl Erra {
        #[doc = "0 No Address error detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Address        detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct All0_SPEC;
    pub type All0 = crate::EnumBitfieldStruct<u8, All0_SPEC>;
    impl All0 {
        #[doc = "0 No All Zeros detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 All zeros detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct All1_SPEC;
    pub type All1 = crate::EnumBitfieldStruct<u8, All1_SPEC>;
    impl All1 {
        #[doc = "0 No All Ones detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 All ones detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Errany_SPEC;
    pub type Errany = crate::EnumBitfieldStruct<u8, Errany_SPEC>;
    impl Errany {
        #[doc = "0 No ECC failure occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ECC failure        occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Aer1_SPEC;
    pub type Aer1 = crate::EnumBitfieldStruct<u8, Aer1_SPEC>;
    impl Aer1 {
        #[doc = "0 No single bit ECC failure occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 At least one        single bit ECC failure occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Aer2_SPEC;
    pub type Aer2 = crate::EnumBitfieldStruct<u8, Aer2_SPEC>;
    impl Aer2 {
        #[doc = "0 No double bit ECC failure occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 At least one        double bit ECC failure occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Aerm_SPEC;
    pub type Aerm = crate::EnumBitfieldStruct<u8, Aerm_SPEC>;
    impl Aerm {
        #[doc = "0 No multi bit ECC failure occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Multi bit ECC        failure occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Arra_SPEC;
    pub type Arra = crate::EnumBitfieldStruct<u8, Arra_SPEC>;
    impl Arra {
        #[doc = "0 No Address error detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Address detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Aal0_SPEC;
    pub type Aal0 = crate::EnumBitfieldStruct<u8, Aal0_SPEC>;
    impl Aal0 {
        #[doc = "0No All Zeros detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 All zeros        detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Aal1_SPEC;
    pub type Aal1 = crate::EnumBitfieldStruct<u8, Aal1_SPEC>;
    impl Aal1 {
        #[doc = "0No All Ones detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 All ones        detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Aerany_SPEC;
    pub type Aerany = crate::EnumBitfieldStruct<u8, Aerany_SPEC>;
    impl Aerany {
        #[doc = "0 No ECC failure occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ECC failure occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Failcount_SPEC;
impl crate::sealed::RegSpec for Failcount_SPEC {
    type DataType = u32;
}
#[doc = "Fail Count Register\n resetvalue={System Reset:0x0}"]
pub type Failcount = crate::RegValueT<Failcount_SPEC>;

impl Failcount {
    #[doc = "Fail Count   FCOUNT. Test data."]
    #[inline(always)]
    pub fn fcount(
        self,
    ) -> crate::common::RegisterField<0, 0x7fff, 1, 0, u16, Failcount_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7fff,1,0,u16, Failcount_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Pulse Time Monitor   PTM. Test."]
    #[inline(always)]
    pub fn ptm(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Failcount_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16,1,0,Failcount_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reference Strobe   REFSTROBE. Test."]
    #[inline(always)]
    pub fn refstrobe(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Failcount_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17,1,0,Failcount_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Failcount {
    #[inline(always)]
    fn default() -> Failcount {
        <crate::RegValueT<Failcount_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc = "BITLINE"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bitline(pub(super) *mut u8);
unsafe impl core::marker::Send for Bitline {}
unsafe impl core::marker::Sync for Bitline {}
impl Bitline {
    #[doc = "Bitline Redundancy Register 0\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn bitlineredx(
        &self,
    ) -> crate::common::Reg<bitline::BitlinereDx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod bitline {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BitlinereDx_SPEC;
    impl crate::sealed::RegSpec for BitlinereDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Bitline Redundancy Register 0\n resetvalue={System Reset:0x0}"]
    pub type BitlinereDx = crate::RegValueT<BitlinereDx_SPEC>;

    impl BitlinereDx {
        #[doc = "Address   ADDRESS. Address of the defective bitline in the 280 bit bitslice."]
        #[inline(always)]
        pub fn address(
            self,
        ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, BitlinereDx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0x1ff,1,0,u16, BitlinereDx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Used   USED"]
        #[inline(always)]
        pub fn used(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x1,
            1,
            0,
            bitlineredx::Used,
            BitlinereDx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                bitlineredx::Used,
                BitlinereDx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Fail   FAIL. Redundant bitline test result."]
        #[inline(always)]
        pub fn fail(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            bitlineredx::Fail,
            BitlinereDx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                bitlineredx::Fail,
                BitlinereDx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for BitlinereDx {
        #[inline(always)]
        fn default() -> BitlinereDx {
            <crate::RegValueT<BitlinereDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod bitlineredx {
        pub struct Used_SPEC;
        pub type Used = crate::EnumBitfieldStruct<u8, Used_SPEC>;
        impl Used {
            #[doc = "0 Redundant bitline is not used."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Redundant bitline is used."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Fail_SPEC;
        pub type Fail = crate::EnumBitfieldStruct<u8, Fail_SPEC>;
        impl Fail {
            #[doc = "0 PASS."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 FAIL."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
}
#[doc = "DBAB"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbab(pub(super) *mut u8);
unsafe impl core::marker::Send for Dbab {}
unsafe impl core::marker::Sync for Dbab {}
impl Dbab {
    #[doc = "DBAB Record 0\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn dbabrecordx(
        &self,
    ) -> crate::common::Reg<dbab::DbabrecorDx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod dbab {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbabrecorDx_SPEC;
    impl crate::sealed::RegSpec for DbabrecorDx_SPEC {
        type DataType = u32;
    }
    #[doc = "DBAB Record 0\n resetvalue={System Reset:0x0}"]
    pub type DbabrecorDx = crate::RegValueT<DbabrecorDx_SPEC>;

    impl DbabrecorDx {
        #[doc = "Address   ADDR. Captured address of local PFLASH bank with detected error."]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffff, 1, 0, u32, DbabrecorDx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<5,0x7ffff,1,0,u32, DbabrecorDx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Valid   VLD. Cleared when CPUx FLASHCON2.DBABCLR is written to 01 ."]
        #[inline(always)]
        pub fn vld(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            dbabrecordx::Vld,
            DbabrecorDx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                dbabrecordx::Vld,
                DbabrecorDx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for DbabrecorDx {
        #[inline(always)]
        fn default() -> DbabrecorDx {
            <crate::RegValueT<DbabrecorDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod dbabrecordx {
        pub struct Vld_SPEC;
        pub type Vld = crate::EnumBitfieldStruct<u8, Vld_SPEC>;
        impl Vld {
            #[doc = "0 No entry recorded."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Valid entry recorded."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
}
#[doc = "MBAB"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbab(pub(super) *mut u8);
unsafe impl core::marker::Send for Mbab {}
unsafe impl core::marker::Sync for Mbab {}
impl Mbab {
    #[doc = "MBAB Record 0\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn mbabrecordx(
        &self,
    ) -> crate::common::Reg<mbab::MbabrecorDx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod mbab {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MbabrecorDx_SPEC;
    impl crate::sealed::RegSpec for MbabrecorDx_SPEC {
        type DataType = u32;
    }
    #[doc = "MBAB Record 0\n resetvalue={System Reset:0x0}"]
    pub type MbabrecorDx = crate::RegValueT<MbabrecorDx_SPEC>;

    impl MbabrecorDx {
        #[doc = "Address   ADDR. Captured address of local PFLASH bank with detected error."]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffff, 1, 0, u32, MbabrecorDx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<5,0x7ffff,1,0,u32, MbabrecorDx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Valid   VLD. Cleared when CPUx FLASHCON2.MBABCLR is written to 01 ."]
        #[inline(always)]
        pub fn vld(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            mbabrecordx::Vld,
            MbabrecorDx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                mbabrecordx::Vld,
                MbabrecorDx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for MbabrecorDx {
        #[inline(always)]
        fn default() -> MbabrecorDx {
            <crate::RegValueT<MbabrecorDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod mbabrecordx {
        pub struct Vld_SPEC;
        pub type Vld = crate::EnumBitfieldStruct<u8, Vld_SPEC>;
        impl Vld {
            #[doc = "0 No entry recorded."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Valid entry recorded."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
}
#[doc = "SBAB"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbab(pub(super) *mut u8);
unsafe impl core::marker::Send for Sbab {}
unsafe impl core::marker::Sync for Sbab {}
impl Sbab {
    #[doc = "SBAB Record 0\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn sbabrecordx(
        &self,
    ) -> crate::common::Reg<sbab::SbabrecorDx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod sbab {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbabrecorDx_SPEC;
    impl crate::sealed::RegSpec for SbabrecorDx_SPEC {
        type DataType = u32;
    }
    #[doc = "SBAB Record 0\n resetvalue={System Reset:0x0}"]
    pub type SbabrecorDx = crate::RegValueT<SbabrecorDx_SPEC>;

    impl SbabrecorDx {
        #[doc = "Address   ADDR. Captured address of local PFLASH bank with detected error."]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffff, 1, 0, u32, SbabrecorDx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<5,0x7ffff,1,0,u32, SbabrecorDx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Valid   VLD. Cleared when CPUx FLASHCON2.SBABCLR is written to 01 ."]
        #[inline(always)]
        pub fn vld(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            sbabrecordx::Vld,
            SbabrecorDx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                sbabrecordx::Vld,
                SbabrecorDx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for SbabrecorDx {
        #[inline(always)]
        fn default() -> SbabrecorDx {
            <crate::RegValueT<SbabrecorDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod sbabrecordx {
        pub struct Vld_SPEC;
        pub type Vld = crate::EnumBitfieldStruct<u8, Vld_SPEC>;
        impl Vld {
            #[doc = "0 No entry recorded."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Valid entry recorded."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
}
#[doc = "SECTOR"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sector(pub(super) *mut u8);
unsafe impl core::marker::Send for Sector {}
unsafe impl core::marker::Sync for Sector {}
impl Sector {
    #[doc = "Sector Redundancy Register 0\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn sectorredx(
        &self,
    ) -> crate::common::Reg<sector::SectorreDx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod sector {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SectorreDx_SPEC;
    impl crate::sealed::RegSpec for SectorreDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Sector Redundancy Register 0\n resetvalue={System Reset:0x0}"]
    pub type SectorreDx = crate::RegValueT<SectorreDx_SPEC>;

    impl SectorreDx {
        #[doc = "Address   ADDRESS. Address of the defective sector in 1024 Kbyte block."]
        #[inline(always)]
        pub fn address(
            self,
        ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, SectorreDx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0x3f,1,0,u8, SectorreDx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Used   USED"]
        #[inline(always)]
        pub fn used(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x1,
            1,
            0,
            sectorredx::Used,
            SectorreDx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                sectorredx::Used,
                SectorreDx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Fail   FAIL. Redundant sector test result."]
        #[inline(always)]
        pub fn fail(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            sectorredx::Fail,
            SectorreDx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                sectorredx::Fail,
                SectorreDx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for SectorreDx {
        #[inline(always)]
        fn default() -> SectorreDx {
            <crate::RegValueT<SectorreDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod sectorredx {
        pub struct Used_SPEC;
        pub type Used = crate::EnumBitfieldStruct<u8, Used_SPEC>;
        impl Used {
            #[doc = "0 Redundant sector is not used."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Redundant sector is used."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Fail_SPEC;
        pub type Fail = crate::EnumBitfieldStruct<u8, Fail_SPEC>;
        impl Fail {
            #[doc = "0 PASS."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 FAIL."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
}
#[doc = "ZBAB"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Zbab(pub(super) *mut u8);
unsafe impl core::marker::Send for Zbab {}
unsafe impl core::marker::Sync for Zbab {}
impl Zbab {
    #[doc = "ZBAB Record 0\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn zbabrecordx(
        &self,
    ) -> crate::common::Reg<zbab::ZbabrecorDx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod zbab {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ZbabrecorDx_SPEC;
    impl crate::sealed::RegSpec for ZbabrecorDx_SPEC {
        type DataType = u32;
    }
    #[doc = "ZBAB Record 0\n resetvalue={System Reset:0x0}"]
    pub type ZbabrecorDx = crate::RegValueT<ZbabrecorDx_SPEC>;

    impl ZbabrecorDx {
        #[doc = "Address   ADDR. Captured address of local PFLASH bank with detected error."]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffff, 1, 0, u32, ZbabrecorDx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<5,0x7ffff,1,0,u32, ZbabrecorDx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Valid   VLD. Cleared when CPUx FLASHCON2.ZBABCLR is written to 01 ."]
        #[inline(always)]
        pub fn vld(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            zbabrecordx::Vld,
            ZbabrecorDx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                zbabrecordx::Vld,
                ZbabrecorDx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for ZbabrecorDx {
        #[inline(always)]
        fn default() -> ZbabrecorDx {
            <crate::RegValueT<ZbabrecorDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod zbabrecordx {
        pub struct Vld_SPEC;
        pub type Vld = crate::EnumBitfieldStruct<u8, Vld_SPEC>;
        impl Vld {
            #[doc = "0 No entry recorded."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Valid entry recorded."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
}
