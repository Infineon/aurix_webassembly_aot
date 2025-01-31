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
#[doc = r"MTU"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtu(pub(super) *mut u8);
unsafe impl core::marker::Send for Mtu {}
unsafe impl core::marker::Sync for Mtu {}
impl Mtu {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(252usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "HSM Memory Protection Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hsmmemcon(&self) -> crate::common::Reg<self::Hsmmemcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }

    #[doc = "HSM Memory Access Protection Status\n resetvalue={System Reset:0x0,Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hsmmemstat(&self) -> crate::common::Reg<self::Hsmmemstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "HSM ROM ECC Detection Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hsmrom_eccd(
        &self,
    ) -> crate::common::Reg<self::HsmromEccd_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "HSM ROM Safety Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hsmrom_eccs(
        &self,
    ) -> crate::common::Reg<self::HsmromEccs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "Identification Register\n resetvalue={Application Reset:0x0B2C003}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Memory Test Done Status Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn memdone(&self) -> [crate::common::Reg<self::Memdone_SPEC, crate::common::R>; 3] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x50usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x50usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x50usize + 0x8usize)),
            ]
        }
    }

    #[doc = "Memory Test FDA Status Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn memfda(&self) -> [crate::common::Reg<self::Memfda_SPEC, crate::common::R>; 3] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x8usize)),
            ]
        }
    }

    #[doc = "Memory Mapping Enable Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn memmap(&self) -> crate::common::Reg<self::Memmap_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "Memory Status Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn memstat(&self) -> [crate::common::Reg<self::Memstat_SPEC, crate::common::R>; 3] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x38usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x38usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x38usize + 0x8usize)),
            ]
        }
    }

    #[doc = "Memory MBIST Enable Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn memtest(&self) -> [crate::common::Reg<self::Memtest_SPEC, crate::common::RW>; 3] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x10usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x10usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x10usize + 0x8usize)),
            ]
        }
    }

    #[doc = "PKC Memory Protection Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pkcmemcon(&self) -> crate::common::Reg<self::Pkcmemcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }

    #[doc = "PKC Memory Access Protection Status\n resetvalue={System Reset:0x0,Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pkcmemstat(&self) -> crate::common::Reg<self::Pkcmemstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(148usize)) }
    }

    #[doc = "PKC ROM ECC Detection Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pkcrom_eccd(
        &self,
    ) -> crate::common::Reg<self::PkcromEccd_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }

    #[doc = "PKC ROM Safety Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pkcrom_eccs(
        &self,
    ) -> crate::common::Reg<self::PkcromEccs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }
    #[doc = "MC"]
    #[inline(always)]
    pub fn mc(self) -> [self::Mc; 96] {
        unsafe {
            [
                self::Mc(self.0.add(0x1000usize + 0x0usize)),
                self::Mc(self.0.add(0x1000usize + 0x100usize)),
                self::Mc(self.0.add(0x1000usize + 0x200usize)),
                self::Mc(self.0.add(0x1000usize + 0x300usize)),
                self::Mc(self.0.add(0x1000usize + 0x400usize)),
                self::Mc(self.0.add(0x1000usize + 0x500usize)),
                self::Mc(self.0.add(0x1000usize + 0x600usize)),
                self::Mc(self.0.add(0x1000usize + 0x700usize)),
                self::Mc(self.0.add(0x1000usize + 0x800usize)),
                self::Mc(self.0.add(0x1000usize + 0x900usize)),
                self::Mc(self.0.add(0x1000usize + 0xa00usize)),
                self::Mc(self.0.add(0x1000usize + 0xb00usize)),
                self::Mc(self.0.add(0x1000usize + 0xc00usize)),
                self::Mc(self.0.add(0x1000usize + 0xd00usize)),
                self::Mc(self.0.add(0x1000usize + 0xe00usize)),
                self::Mc(self.0.add(0x1000usize + 0xf00usize)),
                self::Mc(self.0.add(0x1000usize + 0x1000usize)),
                self::Mc(self.0.add(0x1000usize + 0x1100usize)),
                self::Mc(self.0.add(0x1000usize + 0x1200usize)),
                self::Mc(self.0.add(0x1000usize + 0x1300usize)),
                self::Mc(self.0.add(0x1000usize + 0x1400usize)),
                self::Mc(self.0.add(0x1000usize + 0x1500usize)),
                self::Mc(self.0.add(0x1000usize + 0x1600usize)),
                self::Mc(self.0.add(0x1000usize + 0x1700usize)),
                self::Mc(self.0.add(0x1000usize + 0x1800usize)),
                self::Mc(self.0.add(0x1000usize + 0x1900usize)),
                self::Mc(self.0.add(0x1000usize + 0x1a00usize)),
                self::Mc(self.0.add(0x1000usize + 0x1b00usize)),
                self::Mc(self.0.add(0x1000usize + 0x1c00usize)),
                self::Mc(self.0.add(0x1000usize + 0x1d00usize)),
                self::Mc(self.0.add(0x1000usize + 0x1e00usize)),
                self::Mc(self.0.add(0x1000usize + 0x1f00usize)),
                self::Mc(self.0.add(0x1000usize + 0x2000usize)),
                self::Mc(self.0.add(0x1000usize + 0x2100usize)),
                self::Mc(self.0.add(0x1000usize + 0x2200usize)),
                self::Mc(self.0.add(0x1000usize + 0x2300usize)),
                self::Mc(self.0.add(0x1000usize + 0x2400usize)),
                self::Mc(self.0.add(0x1000usize + 0x2500usize)),
                self::Mc(self.0.add(0x1000usize + 0x2600usize)),
                self::Mc(self.0.add(0x1000usize + 0x2700usize)),
                self::Mc(self.0.add(0x1000usize + 0x2800usize)),
                self::Mc(self.0.add(0x1000usize + 0x2900usize)),
                self::Mc(self.0.add(0x1000usize + 0x2a00usize)),
                self::Mc(self.0.add(0x1000usize + 0x2b00usize)),
                self::Mc(self.0.add(0x1000usize + 0x2c00usize)),
                self::Mc(self.0.add(0x1000usize + 0x2d00usize)),
                self::Mc(self.0.add(0x1000usize + 0x2e00usize)),
                self::Mc(self.0.add(0x1000usize + 0x2f00usize)),
                self::Mc(self.0.add(0x1000usize + 0x3000usize)),
                self::Mc(self.0.add(0x1000usize + 0x3100usize)),
                self::Mc(self.0.add(0x1000usize + 0x3200usize)),
                self::Mc(self.0.add(0x1000usize + 0x3300usize)),
                self::Mc(self.0.add(0x1000usize + 0x3400usize)),
                self::Mc(self.0.add(0x1000usize + 0x3500usize)),
                self::Mc(self.0.add(0x1000usize + 0x3600usize)),
                self::Mc(self.0.add(0x1000usize + 0x3700usize)),
                self::Mc(self.0.add(0x1000usize + 0x3800usize)),
                self::Mc(self.0.add(0x1000usize + 0x3900usize)),
                self::Mc(self.0.add(0x1000usize + 0x3a00usize)),
                self::Mc(self.0.add(0x1000usize + 0x3b00usize)),
                self::Mc(self.0.add(0x1000usize + 0x3c00usize)),
                self::Mc(self.0.add(0x1000usize + 0x3d00usize)),
                self::Mc(self.0.add(0x1000usize + 0x3e00usize)),
                self::Mc(self.0.add(0x1000usize + 0x3f00usize)),
                self::Mc(self.0.add(0x1000usize + 0x4000usize)),
                self::Mc(self.0.add(0x1000usize + 0x4100usize)),
                self::Mc(self.0.add(0x1000usize + 0x4200usize)),
                self::Mc(self.0.add(0x1000usize + 0x4300usize)),
                self::Mc(self.0.add(0x1000usize + 0x4400usize)),
                self::Mc(self.0.add(0x1000usize + 0x4500usize)),
                self::Mc(self.0.add(0x1000usize + 0x4600usize)),
                self::Mc(self.0.add(0x1000usize + 0x4700usize)),
                self::Mc(self.0.add(0x1000usize + 0x4800usize)),
                self::Mc(self.0.add(0x1000usize + 0x4900usize)),
                self::Mc(self.0.add(0x1000usize + 0x4a00usize)),
                self::Mc(self.0.add(0x1000usize + 0x4b00usize)),
                self::Mc(self.0.add(0x1000usize + 0x4c00usize)),
                self::Mc(self.0.add(0x1000usize + 0x4d00usize)),
                self::Mc(self.0.add(0x1000usize + 0x4e00usize)),
                self::Mc(self.0.add(0x1000usize + 0x4f00usize)),
                self::Mc(self.0.add(0x1000usize + 0x5000usize)),
                self::Mc(self.0.add(0x1000usize + 0x5100usize)),
                self::Mc(self.0.add(0x1000usize + 0x5200usize)),
                self::Mc(self.0.add(0x1000usize + 0x5300usize)),
                self::Mc(self.0.add(0x1000usize + 0x5400usize)),
                self::Mc(self.0.add(0x1000usize + 0x5500usize)),
                self::Mc(self.0.add(0x1000usize + 0x5600usize)),
                self::Mc(self.0.add(0x1000usize + 0x5700usize)),
                self::Mc(self.0.add(0x1000usize + 0x5800usize)),
                self::Mc(self.0.add(0x1000usize + 0x5900usize)),
                self::Mc(self.0.add(0x1000usize + 0x5a00usize)),
                self::Mc(self.0.add(0x1000usize + 0x5b00usize)),
                self::Mc(self.0.add(0x1000usize + 0x5c00usize)),
                self::Mc(self.0.add(0x1000usize + 0x5d00usize)),
                self::Mc(self.0.add(0x1000usize + 0x5e00usize)),
                self::Mc(self.0.add(0x1000usize + 0x5f00usize)),
            ]
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accen0_SPEC;
impl crate::sealed::RegSpec for Accen0_SPEC {
    type DataType = u32;
}
#[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Accen0 = crate::RegValueT<Accen0_SPEC>;

impl Accen0 {
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, accen0::En0, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,accen0::En0, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, accen0::En1, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,accen0::En1, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, accen0::En2, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,accen0::En2, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, accen0::En3, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,accen0::En3, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, accen0::En4, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,accen0::En4, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, accen0::En5, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,accen0::En5, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, accen0::En6, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,accen0::En6, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, accen0::En7, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,accen0::En7, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, accen0::En8, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,accen0::En8, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, accen0::En9, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,accen0::En9, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, accen0::En10, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,accen0::En10, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, accen0::En11, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,accen0::En11, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, accen0::En12, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,accen0::En12, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, accen0::En13, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,accen0::En13, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, accen0::En14, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,accen0::En14, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, accen0::En15, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,accen0::En15, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, accen0::En16, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,accen0::En16, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, accen0::En17, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,accen0::En17, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, accen0::En18, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,accen0::En18, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, accen0::En19, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,accen0::En19, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, accen0::En20, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,accen0::En20, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, accen0::En21, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,accen0::En21, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, accen0::En22, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,accen0::En22, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, accen0::En23, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,accen0::En23, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, accen0::En24, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,accen0::En24, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, accen0::En25, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,accen0::En25, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, accen0::En26, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,accen0::En26, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, accen0::En27, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,accen0::En27, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, accen0::En28, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,accen0::En28, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, accen0::En29, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,accen0::En29, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, accen0::En30, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,accen0::En30, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en31(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, accen0::En31, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,accen0::En31, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Accen0 {
    #[inline(always)]
    fn default() -> Accen0 {
        <crate::RegValueT<Accen0_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod accen0 {
    pub struct En0_SPEC;
    pub type En0 = crate::EnumBitfieldStruct<u8, En0_SPEC>;
    impl En0 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En1_SPEC;
    pub type En1 = crate::EnumBitfieldStruct<u8, En1_SPEC>;
    impl En1 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En2_SPEC;
    pub type En2 = crate::EnumBitfieldStruct<u8, En2_SPEC>;
    impl En2 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En3_SPEC;
    pub type En3 = crate::EnumBitfieldStruct<u8, En3_SPEC>;
    impl En3 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En4_SPEC;
    pub type En4 = crate::EnumBitfieldStruct<u8, En4_SPEC>;
    impl En4 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En5_SPEC;
    pub type En5 = crate::EnumBitfieldStruct<u8, En5_SPEC>;
    impl En5 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En6_SPEC;
    pub type En6 = crate::EnumBitfieldStruct<u8, En6_SPEC>;
    impl En6 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En7_SPEC;
    pub type En7 = crate::EnumBitfieldStruct<u8, En7_SPEC>;
    impl En7 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En8_SPEC;
    pub type En8 = crate::EnumBitfieldStruct<u8, En8_SPEC>;
    impl En8 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En9_SPEC;
    pub type En9 = crate::EnumBitfieldStruct<u8, En9_SPEC>;
    impl En9 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En10_SPEC;
    pub type En10 = crate::EnumBitfieldStruct<u8, En10_SPEC>;
    impl En10 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En11_SPEC;
    pub type En11 = crate::EnumBitfieldStruct<u8, En11_SPEC>;
    impl En11 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En12_SPEC;
    pub type En12 = crate::EnumBitfieldStruct<u8, En12_SPEC>;
    impl En12 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En13_SPEC;
    pub type En13 = crate::EnumBitfieldStruct<u8, En13_SPEC>;
    impl En13 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En14_SPEC;
    pub type En14 = crate::EnumBitfieldStruct<u8, En14_SPEC>;
    impl En14 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En15_SPEC;
    pub type En15 = crate::EnumBitfieldStruct<u8, En15_SPEC>;
    impl En15 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En16_SPEC;
    pub type En16 = crate::EnumBitfieldStruct<u8, En16_SPEC>;
    impl En16 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En17_SPEC;
    pub type En17 = crate::EnumBitfieldStruct<u8, En17_SPEC>;
    impl En17 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En18_SPEC;
    pub type En18 = crate::EnumBitfieldStruct<u8, En18_SPEC>;
    impl En18 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En19_SPEC;
    pub type En19 = crate::EnumBitfieldStruct<u8, En19_SPEC>;
    impl En19 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En20_SPEC;
    pub type En20 = crate::EnumBitfieldStruct<u8, En20_SPEC>;
    impl En20 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En21_SPEC;
    pub type En21 = crate::EnumBitfieldStruct<u8, En21_SPEC>;
    impl En21 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En22_SPEC;
    pub type En22 = crate::EnumBitfieldStruct<u8, En22_SPEC>;
    impl En22 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En23_SPEC;
    pub type En23 = crate::EnumBitfieldStruct<u8, En23_SPEC>;
    impl En23 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En24_SPEC;
    pub type En24 = crate::EnumBitfieldStruct<u8, En24_SPEC>;
    impl En24 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En25_SPEC;
    pub type En25 = crate::EnumBitfieldStruct<u8, En25_SPEC>;
    impl En25 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En26_SPEC;
    pub type En26 = crate::EnumBitfieldStruct<u8, En26_SPEC>;
    impl En26 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En27_SPEC;
    pub type En27 = crate::EnumBitfieldStruct<u8, En27_SPEC>;
    impl En27 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En28_SPEC;
    pub type En28 = crate::EnumBitfieldStruct<u8, En28_SPEC>;
    impl En28 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En29_SPEC;
    pub type En29 = crate::EnumBitfieldStruct<u8, En29_SPEC>;
    impl En29 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En30_SPEC;
    pub type En30 = crate::EnumBitfieldStruct<u8, En30_SPEC>;
    impl En30 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En31_SPEC;
    pub type En31 = crate::EnumBitfieldStruct<u8, En31_SPEC>;
    impl En31 {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clc_SPEC;
impl crate::sealed::RegSpec for Clc_SPEC {
    type DataType = u32;
}
#[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
pub type Clc = crate::RegValueT<Clc_SPEC>;

impl Clc {
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control of the module. While any bit of tcu mbist en i x  is asserted  CLC.DISR is ignored and        the MTU kernel clock runs  until all tcu mbist en i x  bits are        deasserted."]
    #[inline(always)]
    pub fn disr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, clc::Disr, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,clc::Disr, Clc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Module Disable Status Bit   DISS. Bit indicates the current status of the module If the RMC field is implemented and if it is 0  DISS is set        automatically."]
    #[inline(always)]
    pub fn diss(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, clc::Diss, Clc_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,clc::Diss, Clc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Resvd   Resvd. Read as 0. Must be written with 0 H"]
    #[inline(always)]
    pub fn resvd(self) -> crate::common::RegisterFieldBool<2, 1, 0, Clc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Clc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Sleep Mode Enable Control   EDIS. Used for module Sleep Mode control. While any bit of tcu mbist en i x  is asserted  sleep mode is ignored        and the MTU kernel clock runs  until all tcu mbist en i x  bits are        deasserted."]
    #[inline(always)]
    pub fn edis(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, clc::Edis, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,clc::Edis, Clc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Clc {
    #[inline(always)]
    fn default() -> Clc {
        <crate::RegValueT<Clc_SPEC> as RegisterValue<_>>::new(3)
    }
}
pub mod clc {
    pub struct Disr_SPEC;
    pub type Disr = crate::EnumBitfieldStruct<u8, Disr_SPEC>;
    impl Disr {
        #[doc = "0 Module disable        is not requested"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module disable        is requested"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Diss_SPEC;
    pub type Diss = crate::EnumBitfieldStruct<u8, Diss_SPEC>;
    impl Diss {
        #[doc = "0 Module is        enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module is        disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Edis_SPEC;
    pub type Edis = crate::EnumBitfieldStruct<u8, Edis_SPEC>;
    impl Edis {
        #[doc = "0 Sleep Mode        request is regarded. Module is enabled to go into Sleep Mode on a        request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Sleep Mode        request is disregarded  Sleep Mode cannot be entered on a request."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsmmemcon_SPEC;
impl crate::sealed::RegSpec for Hsmmemcon_SPEC {
    type DataType = u32;
}
#[doc = "HSM Memory Protection Register\n resetvalue={Application Reset:0x0}"]
pub type Hsmmemcon = crate::RegValueT<Hsmmemcon_SPEC>;

impl Hsmmemcon {
    #[doc = "Check ROM CRC   ROMCRC. Reads as   8216 0  8217  Writes of   8216 0  8217  have no effect Writes of   8216 1  8217  have effect only when HSMMEMSTAT.ROMSTAT is 000B or 010B        or 110B"]
    #[inline(always)]
    pub fn romcrc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        hsmmemcon::Romcrc,
        Hsmmemcon_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            hsmmemcon::Romcrc,
            Hsmmemcon_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Lock HSM ROM CRC Interface   ROMLCK. Reads as   8216 0  8217  Writes of   8216 0  8217  have no effect Writes of   8216 1  8217  have effect only when HSMMEMSTAT.ROMSTAT is 110B Reset        only by System Reset"]
    #[inline(always)]
    pub fn romlck(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        hsmmemcon::Romlck,
        Hsmmemcon_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            hsmmemcon::Romlck,
            Hsmmemcon_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "HSM TRNG Burn In Stress Enable   TRBURN. Controls hsm trng burnin stress en o  except in HSM run mode  If HSMMEMSTAT.RAMRDY  1B and HSMMEMSTAT.ROMSTAT  100B  i.e. HSM        operational mode  then this bit has no effect and the HSM burn in stress        is disabled regardless of the state of TRBURN."]
    #[inline(always)]
    pub fn trburn(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        hsmmemcon::Trburn,
        Hsmmemcon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            hsmmemcon::Trburn,
            Hsmmemcon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Hsmmemcon {
    #[inline(always)]
    fn default() -> Hsmmemcon {
        <crate::RegValueT<Hsmmemcon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod hsmmemcon {
    pub struct Romcrc_SPEC;
    pub type Romcrc = crate::EnumBitfieldStruct<u8, Romcrc_SPEC>;
    impl Romcrc {
        #[doc = "0 No effect"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Automatically start CRC check of HSM ROM Setting this bit changes the state of HSMMEMSTAT.ROMSTAT"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Romlck_SPEC;
    pub type Romlck = crate::EnumBitfieldStruct<u8, Romlck_SPEC>;
    impl Romlck {
        #[doc = "0 No effect"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 If ROMSTAT   110B   CRC passed   then this bit will cause transition of ROMSTAT to state 100B   locked  . If ROMSTAT is in any other state this bit has no effect."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trburn_SPEC;
    pub type Trburn = crate::EnumBitfieldStruct<u8, Trburn_SPEC>;
    impl Trburn {
        #[doc = "0 Disable        continuous TRNG stress"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Run continuous        TRNG stress"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsmmemstat_SPEC;
impl crate::sealed::RegSpec for Hsmmemstat_SPEC {
    type DataType = u32;
}
#[doc = "HSM Memory Access Protection Status\n resetvalue={System Reset:0x0,Application Reset:0x0}"]
pub type Hsmmemstat = crate::RegValueT<Hsmmemstat_SPEC>;

impl Hsmmemstat {
    #[doc = "HSM RAM ReadyState   RAMRDY. If PROCONRAM.RAMIN in the DMU is configured for security  then the HSM        is ready for secure operation when RAMRDY 1 and ROMSTAT 100  i.e. RAM        and ROM testing have completed and are now locked  hsm rom ram chk rdy is asserted   8216 1  8217  if this is satisfied Writes to this register have no effect This bit is cleared only by System Reset."]
    #[inline(always)]
    pub fn ramrdy(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        hsmmemstat::Ramrdy,
        Hsmmemstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            hsmmemstat::Ramrdy,
            Hsmmemstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "HSM ROM State   ROMSTAT. HSM is ready for operation when RAMRDY 1 and ROMSTAT 100 hsm rom ram chk rdy asserted   8216 1  8217  Writes to this register have no effect. System reset or power on reset value is 000B. Application Reset  Class 3  clears only ROMSTAT 1 0  and not bit        ROMSTAT 2  The HSMROM ECCS and HSMROM ECCD register read write access protection        does not depend on the value of these ROMSTAT bits."]
    #[inline(always)]
    pub fn romstat(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        hsmmemstat::Romstat,
        Hsmmemstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            hsmmemstat::Romstat,
            Hsmmemstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Hsmmemstat {
    #[inline(always)]
    fn default() -> Hsmmemstat {
        <crate::RegValueT<Hsmmemstat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod hsmmemstat {
    pub struct Ramrdy_SPEC;
    pub type Ramrdy = crate::EnumBitfieldStruct<u8, Ramrdy_SPEC>;
    impl Ramrdy {
        #[doc = "0 HSM RAMs        Uninitialized. Not all HSM related MEMxEN bits have been changed from        one to zero since last System Reset"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 HSM RAMs        Initialized All HSM related MEMxEN bits have been changed from one to        zero since last System Reset by SSW during STP. Further attempted writes        to HSM MEMTESTx.MEMEN bits have no effect Remains in this state until        System Reset"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Romstat_SPEC;
    pub type Romstat = crate::EnumBitfieldStruct<u8, Romstat_SPEC>;
    impl Romstat {
        #[doc = "000 HSM ROM        Unchecked. Transition to state 001 on a write of HSMMEMCON.ROMCRC   1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 CRC check        running  set hsm crc en  Automatic transition to state 010 or 011 on        completion of the HSM ROM CRC check. Further attempted writes to        HSMMEMCON.ROMCRC have no effect."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 CRC check        completed with fail  i.e. hsm crc done and hsm crc err det   clear        hsm crc en . Transition back to state 001 on write of        HSMMEMCON.ROMCRC   1"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 CRC check        completed with pass  i.e. hsm crc done and not hsm crc err det   clear        hsm crc en . Transition back to state 001 on write of        HSMMEMCON.ROMCRC   1 Transition to state 100 on a write of        HSMMEMCON.ROMLCK     8216 1  8217"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 Attempted        writes to HSMMEMCON.ROMCRC or HSMMEMCON.ROMLCK have no effect. Remains        in this state until system reset"]
        pub const CONST_44: Self = Self::new(4);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HsmromEccd_SPEC;
impl crate::sealed::RegSpec for HsmromEccd_SPEC {
    type DataType = u32;
}
#[doc = "HSM ROM ECC Detection Register\n resetvalue={Application Reset:0x0}"]
pub type HsmromEccd = crate::RegValueT<HsmromEccd_SPEC>;

impl HsmromEccd {
    #[doc = "Error Detected   SERR. Write of   8216 0  8217  clears the sticky status. Write of   8216 1  8217  has no effect. In the case of a write of   8216 0  8217  simultaneously with an error detection         the setting of the bit by hardware will take priority. Read as"]
    #[inline(always)]
    pub fn serr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        hsmrom_eccd::Serr,
        HsmromEccd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            hsmrom_eccd::Serr,
            HsmromEccd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Correctable Error Detected   CERR. Write of   8216 0  8217  clears the sticky status. Write of   8216 1  8217  has no effect. In the case of a write of   8216 0  8217  simultaneously with an error detection         the setting of the bit by hardware will take priority. Read as"]
    #[inline(always)]
    pub fn cerr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        hsmrom_eccd::Cerr,
        HsmromEccd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            hsmrom_eccd::Cerr,
            HsmromEccd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Uncorrectable Error Detected   UERR. Write of   8216 0  8217  clears the sticky status. Write of   8216 1  8217  has no effect. In the case of a write of   8216 0  8217  simultaneously with an error detection         the setting of the bit by hardware will take priority. Read as"]
    #[inline(always)]
    pub fn uerr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        hsmrom_eccd::Uerr,
        HsmromEccd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            hsmrom_eccd::Uerr,
            HsmromEccd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for HsmromEccd {
    #[inline(always)]
    fn default() -> HsmromEccd {
        <crate::RegValueT<HsmromEccd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod hsmrom_eccd {
    pub struct Serr_SPEC;
    pub type Serr = crate::EnumBitfieldStruct<u8, Serr_SPEC>;
    impl Serr {
        #[doc = "0 No error        detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Any error        detected  CERR or UERR  depending on CENE  UENE"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cerr_SPEC;
    pub type Cerr = crate::EnumBitfieldStruct<u8, Cerr_SPEC>;
    impl Cerr {
        #[doc = "0 No correctable error detected or ROM ECCS.CENE   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Correctable error detected  CERR  and ROM ECCS.CENE   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Uerr_SPEC;
    pub type Uerr = crate::EnumBitfieldStruct<u8, Uerr_SPEC>;
    impl Uerr {
        #[doc = "0 No uncorrectable error detected or ROM ECCS.UENE   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Uncorrectable error detected  UERR  and ROM ECCS.UENE   1"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HsmromEccs_SPEC;
impl crate::sealed::RegSpec for HsmromEccs_SPEC {
    type DataType = u32;
}
#[doc = "HSM ROM Safety Register\n resetvalue={Application Reset:0x0}"]
pub type HsmromEccs = crate::RegValueT<HsmromEccs_SPEC>;

impl HsmromEccs {
    #[doc = "Correctable Error Notification Enable   CENE"]
    #[inline(always)]
    pub fn cene(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        hsmrom_eccs::Cene,
        HsmromEccs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            hsmrom_eccs::Cene,
            HsmromEccs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Uncorrectable Error Notification Enable   UENE"]
    #[inline(always)]
    pub fn uene(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        hsmrom_eccs::Uene,
        HsmromEccs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            hsmrom_eccs::Uene,
            HsmromEccs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Error Correction Enable   ECE"]
    #[inline(always)]
    pub fn ece(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        hsmrom_eccs::Ece,
        HsmromEccs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            hsmrom_eccs::Ece,
            HsmromEccs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for HsmromEccs {
    #[inline(always)]
    fn default() -> HsmromEccs {
        <crate::RegValueT<HsmromEccs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod hsmrom_eccs {
    pub struct Cene_SPEC;
    pub type Cene = crate::EnumBitfieldStruct<u8, Cene_SPEC>;
    impl Cene {
        #[doc = "0 Do not report        correctable data errors"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Report detected        correctable errors"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Uene_SPEC;
    pub type Uene = crate::EnumBitfieldStruct<u8, Uene_SPEC>;
    impl Uene {
        #[doc = "0 Do not report        uncorrectable data errors"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Report detected        uncorrectable errors"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ece_SPEC;
    pub type Ece = crate::EnumBitfieldStruct<u8, Ece_SPEC>;
    impl Ece {
        #[doc = "0 Do not correct        correctable errors"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Correct        correctable errors"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Identification Register\n resetvalue={Application Reset:0x0B2C003}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MODREV. This bit field indicates the revision number of the MTU module  provided        by design team ."]
    #[inline(always)]
    pub fn modrev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MODTYPE. This bit field is C0 H . It defines a        32 bit module"]
    #[inline(always)]
    pub fn modtype(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number   MODNUMBER. This bit field defines the module identification number. The idenfication number for the AurixPlus Platform MTU module is 00B2 H"]
    #[inline(always)]
    pub fn modnumber(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Id_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Id {
    #[inline(always)]
    fn default() -> Id {
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(11714563)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memdone_SPEC;
impl crate::sealed::RegSpec for Memdone_SPEC {
    type DataType = u32;
}
#[doc = "Memory Test Done Status Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Memdone = crate::RegValueT<Memdone_SPEC>;

impl Memdone {
    #[doc = "CPU0 DMEM Test Done Status   CPU0 DMEM DONE"]
    #[inline(always)]
    pub fn cpu0_dmem_done(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        memdone::Cpu0DmemDone,
        Memdone_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            memdone::Cpu0DmemDone,
            Memdone_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU0 DTAG Test Done Status   CPU0 DTAG DONE"]
    #[inline(always)]
    pub fn cpu0_dtag_done(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        memdone::Cpu0DtagDone,
        Memdone_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            memdone::Cpu0DtagDone,
            Memdone_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU0 PMEM Test Done Status   CPU0 PMEM DONE"]
    #[inline(always)]
    pub fn cpu0_pmem_done(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        memdone::Cpu0PmemDone,
        Memdone_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            memdone::Cpu0PmemDone,
            Memdone_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU0 PTAG Test Done Status   CPU0 PTAG DONE"]
    #[inline(always)]
    pub fn cpu0_ptag_done(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        memdone::Cpu0PtagDone,
        Memdone_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            memdone::Cpu0PtagDone,
            Memdone_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU0 STANDBY DLMU Test Done Status   CPU0 DLMU STBY DONE"]
    #[inline(always)]
    pub fn cpu0_dlmu_stby_done(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        memdone::Cpu0DlmuStbyDone,
        Memdone_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            memdone::Cpu0DlmuStbyDone,
            Memdone_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU1 DMEM Test Done Status   CPU1 DMEM DONE"]
    #[inline(always)]
    pub fn cpu1_dmem_done(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        memdone::Cpu1DmemDone,
        Memdone_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            memdone::Cpu1DmemDone,
            Memdone_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU1 DTAG Test Done Status   CPU1 DTAG DONE"]
    #[inline(always)]
    pub fn cpu1_dtag_done(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        memdone::Cpu1DtagDone,
        Memdone_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            memdone::Cpu1DtagDone,
            Memdone_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU1 PMEM Test Done Status   CPU1 PMEM DONE"]
    #[inline(always)]
    pub fn cpu1_pmem_done(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        memdone::Cpu1PmemDone,
        Memdone_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            memdone::Cpu1PmemDone,
            Memdone_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU1 PTAG Test Done Status   CPU1 PTAG DONE"]
    #[inline(always)]
    pub fn cpu1_ptag_done(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        memdone::Cpu1PtagDone,
        Memdone_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            memdone::Cpu1PtagDone,
            Memdone_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU1 STANDBY DLMU Test Done Status   CPU1 DLMU STBY DONE"]
    #[inline(always)]
    pub fn cpu1_dlmu_stby_done(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        memdone::Cpu1DlmuStbyDone,
        Memdone_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            memdone::Cpu1DlmuStbyDone,
            Memdone_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU2 DMEM Test Done Status   CPU2 DMEM DONE"]
    #[inline(always)]
    pub fn cpu2_dmem_done(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        memdone::Cpu2DmemDone,
        Memdone_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            memdone::Cpu2DmemDone,
            Memdone_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU2 DTAG Test Done Status   CPU2 DTAG DONE"]
    #[inline(always)]
    pub fn cpu2_dtag_done(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        memdone::Cpu2DtagDone,
        Memdone_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            memdone::Cpu2DtagDone,
            Memdone_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU2 PMEM Test Done Status   CPU2 PMEM DONE"]
    #[inline(always)]
    pub fn cpu2_pmem_done(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        memdone::Cpu2PmemDone,
        Memdone_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            memdone::Cpu2PmemDone,
            Memdone_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU2 PTAG Test Done Status   CPU2 PTAG DONE"]
    #[inline(always)]
    pub fn cpu2_ptag_done(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        memdone::Cpu2PtagDone,
        Memdone_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            memdone::Cpu2PtagDone,
            Memdone_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU2 DLMU memory Test Done Status   CPU2 DLMU DONE"]
    #[inline(always)]
    pub fn cpu2_dlmu_done(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        memdone::Cpu2DlmuDone,
        Memdone_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            memdone::Cpu2DlmuDone,
            Memdone_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Memdone {
    #[inline(always)]
    fn default() -> Memdone {
        <crate::RegValueT<Memdone_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod memdone {
    pub struct Cpu0DmemDone_SPEC;
    pub type Cpu0DmemDone = crate::EnumBitfieldStruct<u8, Cpu0DmemDone_SPEC>;
    impl Cpu0DmemDone {
        #[doc = "0 SSH MSTATUS.DONE   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.DONE   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0DtagDone_SPEC;
    pub type Cpu0DtagDone = crate::EnumBitfieldStruct<u8, Cpu0DtagDone_SPEC>;
    impl Cpu0DtagDone {
        #[doc = "0 SSH MSTATUS.DONE   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.DONE   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0PmemDone_SPEC;
    pub type Cpu0PmemDone = crate::EnumBitfieldStruct<u8, Cpu0PmemDone_SPEC>;
    impl Cpu0PmemDone {
        #[doc = "0 SSH MSTATUS.DONE   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.DONE   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0PtagDone_SPEC;
    pub type Cpu0PtagDone = crate::EnumBitfieldStruct<u8, Cpu0PtagDone_SPEC>;
    impl Cpu0PtagDone {
        #[doc = "0 SSH MSTATUS.DONE   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.DONE   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0DlmuStbyDone_SPEC;
    pub type Cpu0DlmuStbyDone = crate::EnumBitfieldStruct<u8, Cpu0DlmuStbyDone_SPEC>;
    impl Cpu0DlmuStbyDone {
        #[doc = "0 SSH MSTATUS.DONE   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.DONE   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1DmemDone_SPEC;
    pub type Cpu1DmemDone = crate::EnumBitfieldStruct<u8, Cpu1DmemDone_SPEC>;
    impl Cpu1DmemDone {
        #[doc = "0 SSH MSTATUS.DONE   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.DONE   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1DtagDone_SPEC;
    pub type Cpu1DtagDone = crate::EnumBitfieldStruct<u8, Cpu1DtagDone_SPEC>;
    impl Cpu1DtagDone {
        #[doc = "0 SSH MSTATUS.DONE   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.DONE   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1PmemDone_SPEC;
    pub type Cpu1PmemDone = crate::EnumBitfieldStruct<u8, Cpu1PmemDone_SPEC>;
    impl Cpu1PmemDone {
        #[doc = "0 SSH MSTATUS.DONE   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.DONE   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1PtagDone_SPEC;
    pub type Cpu1PtagDone = crate::EnumBitfieldStruct<u8, Cpu1PtagDone_SPEC>;
    impl Cpu1PtagDone {
        #[doc = "0 SSH MSTATUS.DONE   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.DONE   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1DlmuStbyDone_SPEC;
    pub type Cpu1DlmuStbyDone = crate::EnumBitfieldStruct<u8, Cpu1DlmuStbyDone_SPEC>;
    impl Cpu1DlmuStbyDone {
        #[doc = "0 SSH MSTATUS.DONE   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.DONE   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2DmemDone_SPEC;
    pub type Cpu2DmemDone = crate::EnumBitfieldStruct<u8, Cpu2DmemDone_SPEC>;
    impl Cpu2DmemDone {
        #[doc = "0 SSH MSTATUS.DONE   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.DONE   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2DtagDone_SPEC;
    pub type Cpu2DtagDone = crate::EnumBitfieldStruct<u8, Cpu2DtagDone_SPEC>;
    impl Cpu2DtagDone {
        #[doc = "0 SSH MSTATUS.DONE   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.DONE   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2PmemDone_SPEC;
    pub type Cpu2PmemDone = crate::EnumBitfieldStruct<u8, Cpu2PmemDone_SPEC>;
    impl Cpu2PmemDone {
        #[doc = "0 SSH MSTATUS.DONE   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.DONE   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2PtagDone_SPEC;
    pub type Cpu2PtagDone = crate::EnumBitfieldStruct<u8, Cpu2PtagDone_SPEC>;
    impl Cpu2PtagDone {
        #[doc = "0 SSH MSTATUS.DONE   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.DONE   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2DlmuDone_SPEC;
    pub type Cpu2DlmuDone = crate::EnumBitfieldStruct<u8, Cpu2DlmuDone_SPEC>;
    impl Cpu2DlmuDone {
        #[doc = "0 SSH MSTATUS.DONE   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.DONE   1"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memfda_SPEC;
impl crate::sealed::RegSpec for Memfda_SPEC {
    type DataType = u32;
}
#[doc = "Memory Test FDA Status Register 0\n resetvalue={Application Reset:0x0}"]
pub type Memfda = crate::RegValueT<Memfda_SPEC>;

impl Memfda {
    #[doc = "CPU0 DMEM Test FDA Status   CPU0 DMEM FDA"]
    #[inline(always)]
    pub fn cpu0_dmem_fda(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        memfda::Cpu0DmemFda,
        Memfda_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            memfda::Cpu0DmemFda,
            Memfda_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU0 DTAG Test FDA Status   CPU0 DTAG FDA"]
    #[inline(always)]
    pub fn cpu0_dtag_fda(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        memfda::Cpu0DtagFda,
        Memfda_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            memfda::Cpu0DtagFda,
            Memfda_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU0 PMEM Test FDA Status   CPU0 PMEM FDA"]
    #[inline(always)]
    pub fn cpu0_pmem_fda(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        memfda::Cpu0PmemFda,
        Memfda_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            memfda::Cpu0PmemFda,
            Memfda_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU0 PTAG Test FDA Status   CPU0 PTAG FDA"]
    #[inline(always)]
    pub fn cpu0_ptag_fda(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        memfda::Cpu0PtagFda,
        Memfda_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            memfda::Cpu0PtagFda,
            Memfda_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU0 STANDBY DLMU Test FDA Status   CPU0 DLMU STBY FDA"]
    #[inline(always)]
    pub fn cpu0_dlmu_stby_fda(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        memfda::Cpu0DlmuStbyFda,
        Memfda_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            memfda::Cpu0DlmuStbyFda,
            Memfda_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU1 DMEM Test FDA Status   CPU1 DMEM FDA"]
    #[inline(always)]
    pub fn cpu1_dmem_fda(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        memfda::Cpu1DmemFda,
        Memfda_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            memfda::Cpu1DmemFda,
            Memfda_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU1 DTAG Test FDA Status   CPU1 DTAG FDA"]
    #[inline(always)]
    pub fn cpu1_dtag_fda(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        memfda::Cpu1DtagFda,
        Memfda_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            memfda::Cpu1DtagFda,
            Memfda_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU1 PMEM Test FDA Status   CPU1 PMEM FDA"]
    #[inline(always)]
    pub fn cpu1_pmem_fda(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        memfda::Cpu1PmemFda,
        Memfda_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            memfda::Cpu1PmemFda,
            Memfda_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU1 PTAG Test FDA Status   CPU1 PTAG FDA"]
    #[inline(always)]
    pub fn cpu1_ptag_fda(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        memfda::Cpu1PtagFda,
        Memfda_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            memfda::Cpu1PtagFda,
            Memfda_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU1 STANDBY DLMU Test FDA Status   CPU1 DLMU STBY FDA"]
    #[inline(always)]
    pub fn cpu1_dlmu_stby_fda(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        memfda::Cpu1DlmuStbyFda,
        Memfda_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            memfda::Cpu1DlmuStbyFda,
            Memfda_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU2 DMEM Test FDA Status   CPU2 DMEM FDA"]
    #[inline(always)]
    pub fn cpu2_dmem_fda(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        memfda::Cpu2DmemFda,
        Memfda_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            memfda::Cpu2DmemFda,
            Memfda_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU2 DTAG Test FDA Status   CPU2 DTAG FDA"]
    #[inline(always)]
    pub fn cpu2_dtag_fda(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        memfda::Cpu2DtagFda,
        Memfda_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            memfda::Cpu2DtagFda,
            Memfda_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU2 PMEM Test FDA Status   CPU2 PMEM FDA"]
    #[inline(always)]
    pub fn cpu2_pmem_fda(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        memfda::Cpu2PmemFda,
        Memfda_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            memfda::Cpu2PmemFda,
            Memfda_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU2 PTAG Test FDA Status   CPU2 PTAG FDA"]
    #[inline(always)]
    pub fn cpu2_ptag_fda(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        memfda::Cpu2PtagFda,
        Memfda_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            memfda::Cpu2PtagFda,
            Memfda_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU2 DLMU memory Test FDA Status   CPU2 DLMU FDA"]
    #[inline(always)]
    pub fn cpu2_dlmu_fda(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        memfda::Cpu2DlmuFda,
        Memfda_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            memfda::Cpu2DlmuFda,
            Memfda_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Memfda {
    #[inline(always)]
    fn default() -> Memfda {
        <crate::RegValueT<Memfda_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod memfda {
    pub struct Cpu0DmemFda_SPEC;
    pub type Cpu0DmemFda = crate::EnumBitfieldStruct<u8, Cpu0DmemFda_SPEC>;
    impl Cpu0DmemFda {
        #[doc = "0 SSH MSTATUS.FDA   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.FDA   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0DtagFda_SPEC;
    pub type Cpu0DtagFda = crate::EnumBitfieldStruct<u8, Cpu0DtagFda_SPEC>;
    impl Cpu0DtagFda {
        #[doc = "0 SSH MSTATUS.FDA   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.FDA   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0PmemFda_SPEC;
    pub type Cpu0PmemFda = crate::EnumBitfieldStruct<u8, Cpu0PmemFda_SPEC>;
    impl Cpu0PmemFda {
        #[doc = "0 SSH MSTATUS.FDA   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.FDA   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0PtagFda_SPEC;
    pub type Cpu0PtagFda = crate::EnumBitfieldStruct<u8, Cpu0PtagFda_SPEC>;
    impl Cpu0PtagFda {
        #[doc = "0 SSH MSTATUS.FDA   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.FDA   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0DlmuStbyFda_SPEC;
    pub type Cpu0DlmuStbyFda = crate::EnumBitfieldStruct<u8, Cpu0DlmuStbyFda_SPEC>;
    impl Cpu0DlmuStbyFda {
        #[doc = "0 SSH MSTATUS.FDA   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.FDA   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1DmemFda_SPEC;
    pub type Cpu1DmemFda = crate::EnumBitfieldStruct<u8, Cpu1DmemFda_SPEC>;
    impl Cpu1DmemFda {
        #[doc = "0 SSH MSTATUS.FDA   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.FDA   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1DtagFda_SPEC;
    pub type Cpu1DtagFda = crate::EnumBitfieldStruct<u8, Cpu1DtagFda_SPEC>;
    impl Cpu1DtagFda {
        #[doc = "0 SSH MSTATUS.FDA   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.FDA   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1PmemFda_SPEC;
    pub type Cpu1PmemFda = crate::EnumBitfieldStruct<u8, Cpu1PmemFda_SPEC>;
    impl Cpu1PmemFda {
        #[doc = "0 SSH MSTATUS.FDA   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.FDA   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1PtagFda_SPEC;
    pub type Cpu1PtagFda = crate::EnumBitfieldStruct<u8, Cpu1PtagFda_SPEC>;
    impl Cpu1PtagFda {
        #[doc = "0 SSH MSTATUS.FDA   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.FDA   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1DlmuStbyFda_SPEC;
    pub type Cpu1DlmuStbyFda = crate::EnumBitfieldStruct<u8, Cpu1DlmuStbyFda_SPEC>;
    impl Cpu1DlmuStbyFda {
        #[doc = "0 SSH MSTATUS.FDA   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.FDA   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2DmemFda_SPEC;
    pub type Cpu2DmemFda = crate::EnumBitfieldStruct<u8, Cpu2DmemFda_SPEC>;
    impl Cpu2DmemFda {
        #[doc = "0 SSH MSTATUS.FDA   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.FDA   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2DtagFda_SPEC;
    pub type Cpu2DtagFda = crate::EnumBitfieldStruct<u8, Cpu2DtagFda_SPEC>;
    impl Cpu2DtagFda {
        #[doc = "0 SSH MSTATUS.FDA   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.FDA   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2PmemFda_SPEC;
    pub type Cpu2PmemFda = crate::EnumBitfieldStruct<u8, Cpu2PmemFda_SPEC>;
    impl Cpu2PmemFda {
        #[doc = "0 SSH MSTATUS.FDA   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.FDA   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2PtagFda_SPEC;
    pub type Cpu2PtagFda = crate::EnumBitfieldStruct<u8, Cpu2PtagFda_SPEC>;
    impl Cpu2PtagFda {
        #[doc = "0 SSH MSTATUS.FDA   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.FDA   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2DlmuFda_SPEC;
    pub type Cpu2DlmuFda = crate::EnumBitfieldStruct<u8, Cpu2DlmuFda_SPEC>;
    impl Cpu2DlmuFda {
        #[doc = "0 SSH MSTATUS.FDA   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH MSTATUS.FDA   1"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memmap_SPEC;
impl crate::sealed::RegSpec for Memmap_SPEC {
    type DataType = u32;
}
#[doc = "Memory Mapping Enable Register\n resetvalue={Application Reset:0x0}"]
pub type Memmap = crate::RegValueT<Memmap_SPEC>;

impl Memmap {
    #[doc = "CPU0 DCache Mapping   CPU0 DCMAP"]
    #[inline(always)]
    pub fn cpu0_dcmap(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, memmap::Cpu0Dcmap, Memmap_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            memmap::Cpu0Dcmap,
            Memmap_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU0 DTAG Mapping   CPU0 DTMAP. Read only. Mirrors the state of CPU0 DCMAP. CPU D cache memories may only be mapped simultaneously."]
    #[inline(always)]
    pub fn cpu0_dtmap(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, memmap::Cpu0Dtmap, Memmap_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,memmap::Cpu0Dtmap, Memmap_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CPU0 PCACHE Mapping   CPU0 PCMAP"]
    #[inline(always)]
    pub fn cpu0_pcmap(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, memmap::Cpu0Pcmap, Memmap_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            memmap::Cpu0Pcmap,
            Memmap_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU0 PTAG Mapping   CPU0 PTMAP. Read only. Mirrors the state of CPU0 PCMAP. CPU P cache memories may only be mapped simultaneously."]
    #[inline(always)]
    pub fn cpu0_ptmap(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, memmap::Cpu0Ptmap, Memmap_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x1,1,0,memmap::Cpu0Ptmap, Memmap_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CPU1 DCache Mapping   CPU1 DCMAP"]
    #[inline(always)]
    pub fn cpu1_dcmap(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, memmap::Cpu1Dcmap, Memmap_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            memmap::Cpu1Dcmap,
            Memmap_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU1 DTAG Mapping   CPU1 DTMAP. Read only. Mirrors the state of CPU1 DCMAP. CPU D cache memories may only be mapped simultaneously."]
    #[inline(always)]
    pub fn cpu1_dtmap(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, memmap::Cpu1Dtmap, Memmap_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x1,1,0,memmap::Cpu1Dtmap, Memmap_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CPU1 PCACHE Mapping   CPU1 PCMAP"]
    #[inline(always)]
    pub fn cpu1_pcmap(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, memmap::Cpu1Pcmap, Memmap_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            memmap::Cpu1Pcmap,
            Memmap_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU1 PTAG Mapping   CPU1 PTMAP. Read only. Mirrors the state of CPU1 PCMAP. CPU P cache memories may only be mapped simultaneously."]
    #[inline(always)]
    pub fn cpu1_ptmap(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, memmap::Cpu1Ptmap, Memmap_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x1,1,0,memmap::Cpu1Ptmap, Memmap_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CPU2 DCache Mapping   CPU2 DCMAP"]
    #[inline(always)]
    pub fn cpu2_dcmap(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        memmap::Cpu2Dcmap,
        Memmap_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            memmap::Cpu2Dcmap,
            Memmap_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU2 DTAG Mapping   CPU2 DTMAP. Read only. Mirrors the state of CPU2 DCMAP. CPU D cache memories may only be mapped simultaneously."]
    #[inline(always)]
    pub fn cpu2_dtmap(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, memmap::Cpu2Dtmap, Memmap_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            memmap::Cpu2Dtmap,
            Memmap_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU2 PCACHE Mapping   CPU2 PCMAP"]
    #[inline(always)]
    pub fn cpu2_pcmap(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        memmap::Cpu2Pcmap,
        Memmap_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            memmap::Cpu2Pcmap,
            Memmap_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU2 PTAG Mapping   CPU2 PTMAP. Read only. Mirrors the state of CPU2 PCMAP. CPU P cache memories may only be mapped simultaneously."]
    #[inline(always)]
    pub fn cpu2_ptmap(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, memmap::Cpu2Ptmap, Memmap_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            memmap::Cpu2Ptmap,
            Memmap_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Memmap {
    #[inline(always)]
    fn default() -> Memmap {
        <crate::RegValueT<Memmap_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod memmap {
    pub struct Cpu0Dcmap_SPEC;
    pub type Cpu0Dcmap = crate::EnumBitfieldStruct<u8, Cpu0Dcmap_SPEC>;
    impl Cpu0Dcmap {
        #[doc = "0 Normal cache function"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Memory mapped"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0Dtmap_SPEC;
    pub type Cpu0Dtmap = crate::EnumBitfieldStruct<u8, Cpu0Dtmap_SPEC>;
    impl Cpu0Dtmap {
        #[doc = "0 Normal cache        function"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Memory mapped"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0Pcmap_SPEC;
    pub type Cpu0Pcmap = crate::EnumBitfieldStruct<u8, Cpu0Pcmap_SPEC>;
    impl Cpu0Pcmap {
        #[doc = "0 Normal cache function"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Memory mapped"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0Ptmap_SPEC;
    pub type Cpu0Ptmap = crate::EnumBitfieldStruct<u8, Cpu0Ptmap_SPEC>;
    impl Cpu0Ptmap {
        #[doc = "0 Normal cache function"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Memory mapped"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1Dcmap_SPEC;
    pub type Cpu1Dcmap = crate::EnumBitfieldStruct<u8, Cpu1Dcmap_SPEC>;
    impl Cpu1Dcmap {
        #[doc = "0 Normal cache function"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Memory mapped"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1Dtmap_SPEC;
    pub type Cpu1Dtmap = crate::EnumBitfieldStruct<u8, Cpu1Dtmap_SPEC>;
    impl Cpu1Dtmap {
        #[doc = "0 Normal cache function"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Memory mapped"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1Pcmap_SPEC;
    pub type Cpu1Pcmap = crate::EnumBitfieldStruct<u8, Cpu1Pcmap_SPEC>;
    impl Cpu1Pcmap {
        #[doc = "0 Normal cache function"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Memory mapped"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1Ptmap_SPEC;
    pub type Cpu1Ptmap = crate::EnumBitfieldStruct<u8, Cpu1Ptmap_SPEC>;
    impl Cpu1Ptmap {
        #[doc = "0 Normal cache function"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Memory mapped"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2Dcmap_SPEC;
    pub type Cpu2Dcmap = crate::EnumBitfieldStruct<u8, Cpu2Dcmap_SPEC>;
    impl Cpu2Dcmap {
        #[doc = "0 Normal cache function"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Memory mapped"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2Dtmap_SPEC;
    pub type Cpu2Dtmap = crate::EnumBitfieldStruct<u8, Cpu2Dtmap_SPEC>;
    impl Cpu2Dtmap {
        #[doc = "0 Normal cache function"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Memory mapped"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2Pcmap_SPEC;
    pub type Cpu2Pcmap = crate::EnumBitfieldStruct<u8, Cpu2Pcmap_SPEC>;
    impl Cpu2Pcmap {
        #[doc = "0 Normal cache function"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Memory mapped"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2Ptmap_SPEC;
    pub type Cpu2Ptmap = crate::EnumBitfieldStruct<u8, Cpu2Ptmap_SPEC>;
    impl Cpu2Ptmap {
        #[doc = "0 Normal cache function"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Memory mapped"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memstat_SPEC;
impl crate::sealed::RegSpec for Memstat_SPEC {
    type DataType = u32;
}
#[doc = "Memory Status Register 0\n resetvalue={Application Reset:0x0}"]
pub type Memstat = crate::RegValueT<Memstat_SPEC>;

impl Memstat {
    #[doc = "CPU0 DMEM Partial AutoInitialize of Cache Partition Underway   CPU0 DMEM AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu0_dmem_aiu(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        memstat::Cpu0DmemAiu,
        Memstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            memstat::Cpu0DmemAiu,
            Memstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU0 DTAG MBIST AutoInitialize Underway   CPU0 DTAG AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu0_dtag_aiu(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        memstat::Cpu0DtagAiu,
        Memstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            memstat::Cpu0DtagAiu,
            Memstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU0 PMEM Partial AutoInitialize of Cache Partition Underway   CPU0 PMEM AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu0_pmem_aiu(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        memstat::Cpu0PmemAiu,
        Memstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            memstat::Cpu0PmemAiu,
            Memstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU0 PTAG MBIST AutoInitialize Underway   CPU0 PTAG AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu0_ptag_aiu(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        memstat::Cpu0PtagAiu,
        Memstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            memstat::Cpu0PtagAiu,
            Memstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU1 DMEM Partial AutoInitialize of Cache Partition Underway   CPU1 DMEM AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu1_dmem_aiu(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        memstat::Cpu1DmemAiu,
        Memstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            memstat::Cpu1DmemAiu,
            Memstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU1 DTAG MBIST AutoInitialize Underway   CPU1 DTAG AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu1_dtag_aiu(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        memstat::Cpu1DtagAiu,
        Memstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            memstat::Cpu1DtagAiu,
            Memstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU1 PMEM Partial AutoInitialize of Cache Partition Underway   CPU1 PMEM AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu1_pmem_aiu(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        memstat::Cpu1PmemAiu,
        Memstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            memstat::Cpu1PmemAiu,
            Memstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU1 PTAG MBIST AutoInitialize Underway   CPU1 PTAG AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu1_ptag_aiu(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        memstat::Cpu1PtagAiu,
        Memstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            memstat::Cpu1PtagAiu,
            Memstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU2 DMEM Partial AutoInitialize of Cache Partition Underway   CPU2 DMEM AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu2_dmem_aiu(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        memstat::Cpu2DmemAiu,
        Memstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            memstat::Cpu2DmemAiu,
            Memstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU2 DTAG MBIST AutoInitialize Underway   CPU2 DTAG AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu2_dtag_aiu(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        memstat::Cpu2DtagAiu,
        Memstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            memstat::Cpu2DtagAiu,
            Memstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU2 PMEM Partial AutoInitialize of Cache Partition Underway   CPU2 PMEM AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu2_pmem_aiu(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        memstat::Cpu2PmemAiu,
        Memstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            memstat::Cpu2PmemAiu,
            Memstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "CPU2 PTAG MBIST AutoInitialize Underway   CPU2 PTAG AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu2_ptag_aiu(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        memstat::Cpu2PtagAiu,
        Memstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            memstat::Cpu2PtagAiu,
            Memstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Memstat {
    #[inline(always)]
    fn default() -> Memstat {
        <crate::RegValueT<Memstat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod memstat {
    pub struct Cpu0DmemAiu_SPEC;
    pub type Cpu0DmemAiu = crate::EnumBitfieldStruct<u8, Cpu0DmemAiu_SPEC>;
    impl Cpu0DmemAiu {
        #[doc = "0 MBIST not running autoinitialize"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBIST running autoinitialize"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0DtagAiu_SPEC;
    pub type Cpu0DtagAiu = crate::EnumBitfieldStruct<u8, Cpu0DtagAiu_SPEC>;
    impl Cpu0DtagAiu {
        #[doc = "0 MBIST not running autoinitialize"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBIST running autoinitialize"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0PmemAiu_SPEC;
    pub type Cpu0PmemAiu = crate::EnumBitfieldStruct<u8, Cpu0PmemAiu_SPEC>;
    impl Cpu0PmemAiu {
        #[doc = "0 MBIST not running autoinitialize"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBIST running autoinitialize"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0PtagAiu_SPEC;
    pub type Cpu0PtagAiu = crate::EnumBitfieldStruct<u8, Cpu0PtagAiu_SPEC>;
    impl Cpu0PtagAiu {
        #[doc = "0 MBIST not running autoinitialize"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBIST running autoinitialize"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1DmemAiu_SPEC;
    pub type Cpu1DmemAiu = crate::EnumBitfieldStruct<u8, Cpu1DmemAiu_SPEC>;
    impl Cpu1DmemAiu {
        #[doc = "0 MBIST not running autoinitialize"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBIST running autoinitialize"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1DtagAiu_SPEC;
    pub type Cpu1DtagAiu = crate::EnumBitfieldStruct<u8, Cpu1DtagAiu_SPEC>;
    impl Cpu1DtagAiu {
        #[doc = "0 MBIST not running autoinitialize"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBIST running autoinitialize"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1PmemAiu_SPEC;
    pub type Cpu1PmemAiu = crate::EnumBitfieldStruct<u8, Cpu1PmemAiu_SPEC>;
    impl Cpu1PmemAiu {
        #[doc = "0 MBIST not running autoinitialize"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBIST running autoinitialize"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1PtagAiu_SPEC;
    pub type Cpu1PtagAiu = crate::EnumBitfieldStruct<u8, Cpu1PtagAiu_SPEC>;
    impl Cpu1PtagAiu {
        #[doc = "0 MBIST not        running autoinitialize"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBIST running autoinitialize"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2DmemAiu_SPEC;
    pub type Cpu2DmemAiu = crate::EnumBitfieldStruct<u8, Cpu2DmemAiu_SPEC>;
    impl Cpu2DmemAiu {
        #[doc = "0 MBIST not running autoinitialize"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBIST running autoinitialize"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2DtagAiu_SPEC;
    pub type Cpu2DtagAiu = crate::EnumBitfieldStruct<u8, Cpu2DtagAiu_SPEC>;
    impl Cpu2DtagAiu {
        #[doc = "0 MBIST not running autoinitialize"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBIST running autoinitialize"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2PmemAiu_SPEC;
    pub type Cpu2PmemAiu = crate::EnumBitfieldStruct<u8, Cpu2PmemAiu_SPEC>;
    impl Cpu2PmemAiu {
        #[doc = "0 MBIST not running autoinitialize"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBIST running autoinitialize"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2PtagAiu_SPEC;
    pub type Cpu2PtagAiu = crate::EnumBitfieldStruct<u8, Cpu2PtagAiu_SPEC>;
    impl Cpu2PtagAiu {
        #[doc = "0 MBIST not running autoinitialize"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBIST running autoinitialize"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memtest_SPEC;
impl crate::sealed::RegSpec for Memtest_SPEC {
    type DataType = u32;
}
#[doc = "Memory MBIST Enable Register 0\n resetvalue={Application Reset:0x0}"]
pub type Memtest = crate::RegValueT<Memtest_SPEC>;

impl Memtest {
    #[doc = "CPU0 DMEM SSH instance Enable   CPU0 DMEM EN"]
    #[inline(always)]
    pub fn cpu0_dmem_en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        memtest::Cpu0DmemEn,
        Memtest_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            memtest::Cpu0DmemEn,
            Memtest_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU0 DTAG SSH instance Enable   CPU0 DTAG EN"]
    #[inline(always)]
    pub fn cpu0_dtag_en(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        memtest::Cpu0DtagEn,
        Memtest_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            memtest::Cpu0DtagEn,
            Memtest_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU0 PMEM SSH instance Enable   CPU0 PMEM EN"]
    #[inline(always)]
    pub fn cpu0_pmem_en(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        memtest::Cpu0PmemEn,
        Memtest_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            memtest::Cpu0PmemEn,
            Memtest_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU0 PTAG SSH instance Enable   CPU0 PTAG EN"]
    #[inline(always)]
    pub fn cpu0_ptag_en(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        memtest::Cpu0PtagEn,
        Memtest_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            memtest::Cpu0PtagEn,
            Memtest_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU0 STANDBY DLMU SSH instance Enable   CPU0 DLMU STBY EN"]
    #[inline(always)]
    pub fn cpu0_dlmu_stby_en(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        memtest::Cpu0DlmuStbyEn,
        Memtest_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            memtest::Cpu0DlmuStbyEn,
            Memtest_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU1 DMEM SSH instance Enable   CPU1 DMEM EN"]
    #[inline(always)]
    pub fn cpu1_dmem_en(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        memtest::Cpu1DmemEn,
        Memtest_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            memtest::Cpu1DmemEn,
            Memtest_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU1 DTAG SSH instance Enable   CPU1 DTAG EN"]
    #[inline(always)]
    pub fn cpu1_dtag_en(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        memtest::Cpu1DtagEn,
        Memtest_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            memtest::Cpu1DtagEn,
            Memtest_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU1 PMEM SSH instance Enable   CPU1 PMEM EN"]
    #[inline(always)]
    pub fn cpu1_pmem_en(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        memtest::Cpu1PmemEn,
        Memtest_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            memtest::Cpu1PmemEn,
            Memtest_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU1 PTAG SSH instance Enable   CPU1 PTAG EN"]
    #[inline(always)]
    pub fn cpu1_ptag_en(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        memtest::Cpu1PtagEn,
        Memtest_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            memtest::Cpu1PtagEn,
            Memtest_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU1 STANDBY DLMU SSH instance Enable   CPU1 DLMU STBY EN"]
    #[inline(always)]
    pub fn cpu1_dlmu_stby_en(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        memtest::Cpu1DlmuStbyEn,
        Memtest_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            memtest::Cpu1DlmuStbyEn,
            Memtest_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU2 DMEM SSH instance Enable   CPU2 DMEM EN"]
    #[inline(always)]
    pub fn cpu2_dmem_en(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        memtest::Cpu2DmemEn,
        Memtest_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            memtest::Cpu2DmemEn,
            Memtest_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU2 DTAG SSH instance Enable   CPU2 DTAG EN"]
    #[inline(always)]
    pub fn cpu2_dtag_en(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        memtest::Cpu2DtagEn,
        Memtest_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            memtest::Cpu2DtagEn,
            Memtest_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU2 PMEM SSH instance Enable   CPU2 PMEM EN"]
    #[inline(always)]
    pub fn cpu2_pmem_en(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        memtest::Cpu2PmemEn,
        Memtest_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            memtest::Cpu2PmemEn,
            Memtest_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU2 PTAG SSH instance Enable   CPU2 PTAG EN"]
    #[inline(always)]
    pub fn cpu2_ptag_en(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        memtest::Cpu2PtagEn,
        Memtest_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            memtest::Cpu2PtagEn,
            Memtest_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU2 DLMU memory SSH instance Enable   CPU2 DLMU EN"]
    #[inline(always)]
    pub fn cpu2_dlmu_en(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        memtest::Cpu2DlmuEn,
        Memtest_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            memtest::Cpu2DlmuEn,
            Memtest_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Memtest {
    #[inline(always)]
    fn default() -> Memtest {
        <crate::RegValueT<Memtest_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod memtest {
    pub struct Cpu0DmemEn_SPEC;
    pub type Cpu0DmemEn = crate::EnumBitfieldStruct<u8, Cpu0DmemEn_SPEC>;
    impl Cpu0DmemEn {
        #[doc = "0 SSH instance is        disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH instance is        enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0DtagEn_SPEC;
    pub type Cpu0DtagEn = crate::EnumBitfieldStruct<u8, Cpu0DtagEn_SPEC>;
    impl Cpu0DtagEn {
        #[doc = "0 SSH instance is        disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH instance is        enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0PmemEn_SPEC;
    pub type Cpu0PmemEn = crate::EnumBitfieldStruct<u8, Cpu0PmemEn_SPEC>;
    impl Cpu0PmemEn {
        #[doc = "0 SSH instance is        disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH instance is        enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0PtagEn_SPEC;
    pub type Cpu0PtagEn = crate::EnumBitfieldStruct<u8, Cpu0PtagEn_SPEC>;
    impl Cpu0PtagEn {
        #[doc = "0 SSH instance is        disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH instance is        enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0DlmuStbyEn_SPEC;
    pub type Cpu0DlmuStbyEn = crate::EnumBitfieldStruct<u8, Cpu0DlmuStbyEn_SPEC>;
    impl Cpu0DlmuStbyEn {
        #[doc = "0 SSH instance is        disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH instance is        enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1DmemEn_SPEC;
    pub type Cpu1DmemEn = crate::EnumBitfieldStruct<u8, Cpu1DmemEn_SPEC>;
    impl Cpu1DmemEn {
        #[doc = "0 SSH instance is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH instance is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1DtagEn_SPEC;
    pub type Cpu1DtagEn = crate::EnumBitfieldStruct<u8, Cpu1DtagEn_SPEC>;
    impl Cpu1DtagEn {
        #[doc = "0 SSH instance is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH instance is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1PmemEn_SPEC;
    pub type Cpu1PmemEn = crate::EnumBitfieldStruct<u8, Cpu1PmemEn_SPEC>;
    impl Cpu1PmemEn {
        #[doc = "0 SSH instance is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH instance is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1PtagEn_SPEC;
    pub type Cpu1PtagEn = crate::EnumBitfieldStruct<u8, Cpu1PtagEn_SPEC>;
    impl Cpu1PtagEn {
        #[doc = "0 SSH instance is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH instance is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1DlmuStbyEn_SPEC;
    pub type Cpu1DlmuStbyEn = crate::EnumBitfieldStruct<u8, Cpu1DlmuStbyEn_SPEC>;
    impl Cpu1DlmuStbyEn {
        #[doc = "0 SSH instance is        disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH instance is        enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2DmemEn_SPEC;
    pub type Cpu2DmemEn = crate::EnumBitfieldStruct<u8, Cpu2DmemEn_SPEC>;
    impl Cpu2DmemEn {
        #[doc = "0 SSH instance is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH instance is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2DtagEn_SPEC;
    pub type Cpu2DtagEn = crate::EnumBitfieldStruct<u8, Cpu2DtagEn_SPEC>;
    impl Cpu2DtagEn {
        #[doc = "0 SSH instance is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH instance is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2PmemEn_SPEC;
    pub type Cpu2PmemEn = crate::EnumBitfieldStruct<u8, Cpu2PmemEn_SPEC>;
    impl Cpu2PmemEn {
        #[doc = "0 SSH instance is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH instance is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2PtagEn_SPEC;
    pub type Cpu2PtagEn = crate::EnumBitfieldStruct<u8, Cpu2PtagEn_SPEC>;
    impl Cpu2PtagEn {
        #[doc = "0 SSH instance is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH instance is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2DlmuEn_SPEC;
    pub type Cpu2DlmuEn = crate::EnumBitfieldStruct<u8, Cpu2DlmuEn_SPEC>;
    impl Cpu2DlmuEn {
        #[doc = "0 SSH instance is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SSH instance is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pkcmemcon_SPEC;
impl crate::sealed::RegSpec for Pkcmemcon_SPEC {
    type DataType = u32;
}
#[doc = "PKC Memory Protection Register\n resetvalue={Application Reset:0x0}"]
pub type Pkcmemcon = crate::RegValueT<Pkcmemcon_SPEC>;

impl Pkcmemcon {
    #[doc = "Check ROM CRC   ROMCRC. Reads as   8216 0  8217  Writes of   8216 0  8217  have no effect Writes of   8216 1  8217  have effect only when PKCMEMSTAT.ROMSTAT is 000B or 010B        or 110B"]
    #[inline(always)]
    pub fn romcrc(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pkcmemcon::Romcrc,
        Pkcmemcon_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pkcmemcon::Romcrc,
            Pkcmemcon_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Lock PKC ROM CRC Interface   ROMLCK. Reads as   8216 0  8217  Writes of   8216 0  8217  have no effect Writes of   8216 1  8217  have effect only when PKCMEMSTAT.ROMSTAT is 110B Reset        only by System Reset"]
    #[inline(always)]
    pub fn romlck(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pkcmemcon::Romlck,
        Pkcmemcon_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pkcmemcon::Romlck,
            Pkcmemcon_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "PKC TRNG Burn In Stress Enable   TRBURN. Controls pkc trng burnin stress en o  except in PKC run mode . If        PKCMEMSTAT.RAMRDY  1B and PKCMEMSTAT.ROMSTAT  100B  i.e. PKC operational        mode  then this bit has no effect and the PKC burn in stress is disabled        regardless of the state of TRBURN."]
    #[inline(always)]
    pub fn trburn(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pkcmemcon::Trburn,
        Pkcmemcon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pkcmemcon::Trburn,
            Pkcmemcon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Pkcmemcon {
    #[inline(always)]
    fn default() -> Pkcmemcon {
        <crate::RegValueT<Pkcmemcon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pkcmemcon {
    pub struct Romcrc_SPEC;
    pub type Romcrc = crate::EnumBitfieldStruct<u8, Romcrc_SPEC>;
    impl Romcrc {
        #[doc = "0 No effect"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Automatically        start CRC check of PKCROM. Setting this bit changes the state of        PKCMEMSTAT.ROMSTAT"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Romlck_SPEC;
    pub type Romlck = crate::EnumBitfieldStruct<u8, Romlck_SPEC>;
    impl Romlck {
        #[doc = "0 No effect"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 If ROMSTAT   110B   CRC passed   then this bit will cause transition of ROMSTAT to state 100B   locked  . If ROMSTAT is in any other state this bit has no effect."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trburn_SPEC;
    pub type Trburn = crate::EnumBitfieldStruct<u8, Trburn_SPEC>;
    impl Trburn {
        #[doc = "0 Disable continuous TRNG stress"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Run continuous TRNG stress"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pkcmemstat_SPEC;
impl crate::sealed::RegSpec for Pkcmemstat_SPEC {
    type DataType = u32;
}
#[doc = "PKC Memory Access Protection Status\n resetvalue={System Reset:0x0,Application Reset:0x0}"]
pub type Pkcmemstat = crate::RegValueT<Pkcmemstat_SPEC>;

impl Pkcmemstat {
    #[doc = "PKC RAM ReadyState   RAMRDY. If PROCONRAM.RAMIN bits in the PMU is configured for security  then the        PKCis ready for secure operation when RAMRDY 1 and ROMSTAT 100  i.e. RAM        and ROM testing have completed and are now locked  pkc rom ram chk rdy is asserted   8216 1  8217  if this is satisfied Writes to this register have no effect This bit is cleared only by System Reset."]
    #[inline(always)]
    pub fn ramrdy(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pkcmemstat::Ramrdy,
        Pkcmemstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pkcmemstat::Ramrdy,
            Pkcmemstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "PKC ROM State   ROMSTAT. PKC is ready for operation when RAMRDY 1 and ROMSTAT 100 pkc rom ram chk rdy asserted   8216 1  8217  Writes to this register have no effect. System reset or power on reset value is 000B. Application Reset  Class 3  clears only ROMSTAT 1 0  and not bit        ROMSTAT 2  The PKCROM ECCS and PKCROM ECCD register read write access protection        does not depend on the value of these ROMSTAT bits."]
    #[inline(always)]
    pub fn romstat(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x7,
        1,
        0,
        pkcmemstat::Romstat,
        Pkcmemstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x7,
            1,
            0,
            pkcmemstat::Romstat,
            Pkcmemstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Pkcmemstat {
    #[inline(always)]
    fn default() -> Pkcmemstat {
        <crate::RegValueT<Pkcmemstat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pkcmemstat {
    pub struct Ramrdy_SPEC;
    pub type Ramrdy = crate::EnumBitfieldStruct<u8, Ramrdy_SPEC>;
    impl Ramrdy {
        #[doc = "0 PKC RAMs        Uninitialized. Not all PKC related MEMxEN bits have been changed from        one to zero since last System Reset"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PKC RAMs        Initialized All PKC related MEMxEN bits have been changed from one to        zero since last System Reset by SSW during STP. Further attempted writes        to PKC MEMTESTx.MEMEN bits have no effect Remains in this state until        System Reset"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Romstat_SPEC;
    pub type Romstat = crate::EnumBitfieldStruct<u8, Romstat_SPEC>;
    impl Romstat {
        #[doc = "000 PKC ROM        Unchecked. Transition to state 001 on a write of PKCMEMCON.ROMCRC   1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 CRC check        running  set pkc crc en . Automatic transition to state 010 or 011 on        completion of the PKC ROM CRC check. Further attempted writes to        PKCMEMCON.ROMCRC have no effect."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 CRC check        completed with fail  i.e. pkc crc done and pkc crc err det   clear        pkc crc en . Transition back to state 001 on write of        PKCMEMCON.ROMCRC   1"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 CRC check        completed with pass  i.e. pkc crc done and not pkc crc err det   clear        pkc crc en . Transition back to state 001 on write of        PKCMEMCON.ROMCRC   1. Transition to state 100 on a write of        PKCMEMCON.ROMLCK     8216 1  8217 ."]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 Attempted        writes to PKCMEMCON.ROMCRC or PKCMEMCON.ROMLCK have no effect. Remains        in this state until system reset."]
        pub const CONST_44: Self = Self::new(4);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcromEccd_SPEC;
impl crate::sealed::RegSpec for PkcromEccd_SPEC {
    type DataType = u32;
}
#[doc = "PKC ROM ECC Detection Register\n resetvalue={Application Reset:0x0}"]
pub type PkcromEccd = crate::RegValueT<PkcromEccd_SPEC>;

impl PkcromEccd {
    #[doc = "Error Detected   SERR. Write of   8216 0  8217  clears the sticky status. Write of   8216 1  8217  has no effect. In the case of a write of   8216 0  8217  simultaneously with an error detection         the setting of the bit by hardware will take priority. Read as"]
    #[inline(always)]
    pub fn serr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pkcrom_eccd::Serr,
        PkcromEccd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pkcrom_eccd::Serr,
            PkcromEccd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Correctable Error Detected   CERR. Write of   8216 0  8217  clears the sticky status. Write of   8216 1  8217  has no effect. In the case of a write of   8216 0  8217  simultaneously with an error detection         the setting of the bit by hardware will take priority. Read as"]
    #[inline(always)]
    pub fn cerr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pkcrom_eccd::Cerr,
        PkcromEccd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pkcrom_eccd::Cerr,
            PkcromEccd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Uncorrectable Error Detected   UERR. Read as Write of   8216 0  8217  clears the sticky status. Write of   8216 1  8217  has no effect. In the case of a write of   8216 0  8217  simultaneously with an error detection         the setting of the bit by hardware will take priority."]
    #[inline(always)]
    pub fn uerr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pkcrom_eccd::Uerr,
        PkcromEccd_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pkcrom_eccd::Uerr,
            PkcromEccd_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for PkcromEccd {
    #[inline(always)]
    fn default() -> PkcromEccd {
        <crate::RegValueT<PkcromEccd_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pkcrom_eccd {
    pub struct Serr_SPEC;
    pub type Serr = crate::EnumBitfieldStruct<u8, Serr_SPEC>;
    impl Serr {
        #[doc = "0 No error detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Any error detected  CERR or UERR  depending on CENE  UENE"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cerr_SPEC;
    pub type Cerr = crate::EnumBitfieldStruct<u8, Cerr_SPEC>;
    impl Cerr {
        #[doc = "0 No correctable error detected or ROM ECCS.CENE   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Correctable error detected  CERR  and ROM ECCS.CENE   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Uerr_SPEC;
    pub type Uerr = crate::EnumBitfieldStruct<u8, Uerr_SPEC>;
    impl Uerr {
        #[doc = "0 No uncorrectable error detected or ROM ECCS.UENE   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Uncorrectable error detected  UERR  and ROM ECCS.UENE   1"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PkcromEccs_SPEC;
impl crate::sealed::RegSpec for PkcromEccs_SPEC {
    type DataType = u32;
}
#[doc = "PKC ROM Safety Register\n resetvalue={Application Reset:0x0}"]
pub type PkcromEccs = crate::RegValueT<PkcromEccs_SPEC>;

impl PkcromEccs {
    #[doc = "Correctable Error Notification Enable   CENE"]
    #[inline(always)]
    pub fn cene(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pkcrom_eccs::Cene,
        PkcromEccs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pkcrom_eccs::Cene,
            PkcromEccs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Uncorrectable Error Notification Enable   UENE"]
    #[inline(always)]
    pub fn uene(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pkcrom_eccs::Uene,
        PkcromEccs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pkcrom_eccs::Uene,
            PkcromEccs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Error Correction Enable   ECE"]
    #[inline(always)]
    pub fn ece(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pkcrom_eccs::Ece,
        PkcromEccs_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pkcrom_eccs::Ece,
            PkcromEccs_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for PkcromEccs {
    #[inline(always)]
    fn default() -> PkcromEccs {
        <crate::RegValueT<PkcromEccs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pkcrom_eccs {
    pub struct Cene_SPEC;
    pub type Cene = crate::EnumBitfieldStruct<u8, Cene_SPEC>;
    impl Cene {
        #[doc = "0 Do not report correctable data errors"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Report detected correctable errors"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Uene_SPEC;
    pub type Uene = crate::EnumBitfieldStruct<u8, Uene_SPEC>;
    impl Uene {
        #[doc = "0 Do not report uncorrectable data errors"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Report detected uncorrectable errors"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ece_SPEC;
    pub type Ece = crate::EnumBitfieldStruct<u8, Ece_SPEC>;
    impl Ece {
        #[doc = "0 Do not correct correctable errors"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Correct correctable errors"]
        pub const CONST_11: Self = Self::new(1);
    }
}

#[doc = "MC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mc(pub(super) *mut u8);
unsafe impl core::marker::Send for Mc {}
unsafe impl core::marker::Sync for Mc {}
impl Mc {
    #[doc = "Alarm Sources Configuration Register\n resetvalue={Application Reset:0x3F}"]
    #[inline(always)]
    pub const fn almsrcs(&self) -> crate::common::Reg<mc::Almsrcs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(238usize)) }
    }
    #[doc = "Configuration Registers\n resetvalue={Application Reset:0x2002}"]
    #[inline(always)]
    pub const fn config0(&self) -> crate::common::Reg<mc::Config0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Configuration Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn config1(&self) -> crate::common::Reg<mc::Config1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2usize)) }
    }
    #[doc = "Memory ECC Detection Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0}"]
    #[inline(always)]
    pub const fn eccd(&self) -> crate::common::Reg<mc::Eccd_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "ECC Safety Register\n resetvalue={Application Reset:0x1F}"]
    #[inline(always)]
    pub const fn eccs(&self) -> crate::common::Reg<mc::Eccs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(14usize)) }
    }
    #[doc = "Error Information Register 0\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn errinfo(&self) -> [crate::common::Reg<mc::Errinfo_SPEC, crate::common::R>; 5] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0xf2usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0xf2usize + 0x2usize)),
                crate::common::Reg::from_ptr(self.0.add(0xf2usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0xf2usize + 0x6usize)),
                crate::common::Reg::from_ptr(self.0.add(0xf2usize + 0x8usize)),
            ]
        }
    }
    #[doc = "Error Tracking Register 0\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn etrr(&self) -> [crate::common::Reg<mc::Etrr_SPEC, crate::common::R>; 5] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x12usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x12usize + 0x2usize)),
                crate::common::Reg::from_ptr(self.0.add(0x12usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x12usize + 0x6usize)),
                crate::common::Reg::from_ptr(self.0.add(0x12usize + 0x8usize)),
            ]
        }
    }
    #[doc = "MBIST Control Register\n resetvalue={Application Reset:0x4008}"]
    #[inline(always)]
    pub const fn mcontrol(&self) -> crate::common::Reg<mc::Mcontrol_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Status Register\n resetvalue={Application Reset:0x1}"]
    #[inline(always)]
    pub const fn mstatus(&self) -> crate::common::Reg<mc::Mstatus_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(6usize)) }
    }
    #[doc = "Programmable Self Timing Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn ramtest(&self) -> crate::common::Reg<mc::Ramtest_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(10usize)) }
    }
    #[doc = "Range Register  single address mode\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn range(&self) -> crate::common::Reg<mc::Range_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Redundant Address Register 0\n resetvalue={Test Reset:0x0}"]
    #[inline(always)]
    pub const fn rar(&self) -> [crate::common::Reg<mc::Rar_SPEC, crate::common::RW>; 32] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x2usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x6usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0xausize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0xeusize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x12usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x16usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x1ausize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x1eusize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x22usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x26usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x2ausize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x2eusize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x32usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x36usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x3ausize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x3eusize)),
            ]
        }
    }
    #[doc = "Read Data and Bit Flip Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rdbfl(&self) -> [crate::common::Reg<mc::Rdbfl_SPEC, crate::common::RW>; 67] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x2usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x6usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0xausize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0xeusize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x12usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x16usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x1ausize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x1eusize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x22usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x26usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x2ausize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x2eusize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x32usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x36usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x3ausize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x3eusize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x42usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x46usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x4ausize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x4eusize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x52usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x56usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x5ausize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x5cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x5eusize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x60usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x62usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x64usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x66usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x68usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x6ausize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x6cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x6eusize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x70usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x72usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x74usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x76usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x78usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x7ausize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x7cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x7eusize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x80usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x82usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x84usize)),
            ]
        }
    }
    #[doc = "Revision ID Register\n resetvalue={Application Reset:0x610}"]
    #[inline(always)]
    pub const fn revid(&self) -> crate::common::Reg<mc::Revid_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
}
pub mod mc {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Almsrcs_SPEC;
    impl crate::sealed::RegSpec for Almsrcs_SPEC {
        type DataType = u16;
    }
    #[doc = "Alarm Sources Configuration Register\n resetvalue={Application Reset:0x3F}"]
    pub type Almsrcs = crate::RegValueT<Almsrcs_SPEC>;

    impl Almsrcs {
        #[doc = "Single Bit Error Notification   Tracking Enable   SBE. This bit enables ECC Single Bit Detection Correction event to be tracked        forwarded to the CE or UCE alarm. If ECCS.ECE bit is  1   then SBE        errors are forwarded to CE alarm. Otherwise to UCE alarm. The error        status can be read from the ERRINFO registers  ERRINFO x .SBERR"]
        #[inline(always)]
        pub fn sbe(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, almsrcs::Sbe, Almsrcs_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                almsrcs::Sbe,
                Almsrcs_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Double Bit Error Notification and Tracking Enable   DBE. This bit enables ECC Double Bit Errors in the SRAM to be tracked and        forwarded as an UCE alarm. The error status can be read from the ERRINFO        registers  ERRINFO.DBERR ."]
        #[inline(always)]
        pub fn dbe(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, almsrcs::Dbe, Almsrcs_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                almsrcs::Dbe,
                Almsrcs_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Address Error Notification Enable   ADDRE. This bit enables the detection and tracking of Address Faults in the        SRAM  and forward them as a source of UCE alarm. The error status can be        read from the ERRINFO registers  ERRINFO.ADDRERR ."]
        #[inline(always)]
        pub fn addre(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            almsrcs::Addre,
            Almsrcs_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                almsrcs::Addre,
                Almsrcs_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "ETRR Overflow notification enable  OVFE. This bit enables the forwarding of the ETRR Overflow event as an alarm        source to the UCE alarm. The Error information can be obtained via the        ECCD.VALID bits and the EOV bit."]
        #[inline(always)]
        pub fn ovfe(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            almsrcs::Ovfe,
            Almsrcs_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                almsrcs::Ovfe,
                Almsrcs_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "SSH Operational Error Notification Enable   OPENE. This bit enables the forwarding of many errors which are critical to the        operation of the SRAM or SSH. These errors are forwarded as one of the        sources of the UCE alarm. The error status can be read from        FAULTSTS.OPERR bits. This bit is enabled by default."]
        #[inline(always)]
        pub fn opene(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            almsrcs::Opene,
            Almsrcs_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                almsrcs::Opene,
                Almsrcs_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "SSH Misc. Errors Notification Enable   MISCE. This bit enables the forwarding of many errors which may be       critical to the operation of the SRAM or SSH in the future. These errors are forwarded as one       of the sources of the ME alarm. The error status can be read from FAULSTS.MISCERR. This bit is enabled by default."]
        #[inline(always)]
        pub fn misce(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            almsrcs::Misce,
            Almsrcs_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                almsrcs::Misce,
                Almsrcs_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for Almsrcs {
        #[inline(always)]
        fn default() -> Almsrcs {
            <crate::RegValueT<Almsrcs_SPEC> as RegisterValue<_>>::new(63)
        }
    }
    pub mod almsrcs {
        pub struct Sbe_SPEC;
        pub type Sbe = crate::EnumBitfieldStruct<u8, Sbe_SPEC>;
        impl Sbe {
            #[doc = "SBE errors are neither tracked in the ETRR  nor notified via an alarm."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 SBE errors are        tracked in the ETRR  amp  ERRINFO  and notified via an alarm  CE if ECE   1         UCE if ECE   0 ."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Dbe_SPEC;
        pub type Dbe = crate::EnumBitfieldStruct<u8, Dbe_SPEC>;
        impl Dbe {
            #[doc = "DBE errors are neither tracked in the ETRR  nor notified via an alarm."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 DBE errors are        tracked in the ETRR  amp  ERRINFO  and notified via a UCE alarm."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Addre_SPEC;
        pub type Addre = crate::EnumBitfieldStruct<u8, Addre_SPEC>;
        impl Addre {
            #[doc = "Address Faults in the SRAM are neither tracked in the ETRR  nor notified        via an alarm."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Address Faults        in the SRAM are tracked in the ETRR  amp  ERRINFO  and notified via a UCE        alarm."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Ovfe_SPEC;
        pub type Ovfe = crate::EnumBitfieldStruct<u8, Ovfe_SPEC>;
        impl Ovfe {
            #[doc = "Do not report Error Tracking  ETRR  Buffer Overflow Error."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Report Error        Tracking  ETRR  Buffer Overflow Error via the UCE alarm"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Opene_SPEC;
        pub type Opene = crate::EnumBitfieldStruct<u8, Opene_SPEC>;
        impl Opene {
            #[doc = "Do not enable the detection and forwarding SSH SRAM operation critical        errors as a source to the UCE alarm."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Enable the        detection and forwarding SSH SRAM operation critical errors as a source        to the UCE alarm"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Misce_SPEC;
        pub type Misce = crate::EnumBitfieldStruct<u8, Misce_SPEC>;
        impl Misce {
            #[doc = "Do not enable the detection and forwarding of misc. SSH SRAM errors as a        source to the ME alarm."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Enable the        detection and forwarding of misc. SSH SRAM errors as a source to the ME        alarm"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Config0_SPEC;
    impl crate::sealed::RegSpec for Config0_SPEC {
        type DataType = u16;
    }
    #[doc = "Configuration Registers\n resetvalue={Application Reset:0x2002}"]
    pub type Config0 = crate::RegValueT<Config0_SPEC>;

    impl Config0 {
        #[doc = "Access type   ACCSTYPE. This field specifies the type of access which is being performed to each        single address in the current marching element. ACCSTYPE n  specifies        the n th access of the marching element."]
        #[inline(always)]
        pub fn accstype(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xff,
            1,
            0,
            config0::Accstype,
            Config0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xff,
                1,
                0,
                config0::Accstype,
                Config0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Number of accesses per address   NUMACCS. This field specifies the total number of accesses which are being        performed to each single address in the current marching element. Allowed values  0 8  Due to size limitation of CONFIG0.ACCSTYPE and        CONFIG1.ACCSPAT fields . If NUMACCS 0 will not access a memory. If NUMACCS  gt  8  8 accesses will be performed."]
        #[inline(always)]
        pub fn numaccs(
            self,
        ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Config0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0xf,1,0,u8, Config0_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Config0 {
        #[inline(always)]
        fn default() -> Config0 {
            <crate::RegValueT<Config0_SPEC> as RegisterValue<_>>::new(8194)
        }
    }
    pub mod config0 {
        pub struct Accstype_SPEC;
        pub type Accstype = crate::EnumBitfieldStruct<u8, Accstype_SPEC>;
        impl Accstype {
            #[doc = "0 write        access"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 read        access"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Config1_SPEC;
    impl crate::sealed::RegSpec for Config1_SPEC {
        type DataType = u16;
    }
    #[doc = "Configuration Register 1\n resetvalue={Application Reset:0x0}"]
    pub type Config1 = crate::RegValueT<Config1_SPEC>;

    impl Config1 {
        #[doc = "Access pattern   ACCSPAT. When AG MOD is selected for any test other than the       Non Destructive test  this field specifies directly the bit pattern  i.e.  0  or  1   which is       being used for an access to each single address in the current marching element. ACCSPAT n        specifies the n th access of the marching element. These patterns are toggled according to       MCONTROL.BITTOG and MCONTROL.ROWTOG. When AG MOD selects the the Non Destructive       test  For corresponding ACCSTYPE as READ or WRITE access  Program 0 when the previous read       access was with normal data  and 1 when the previous read was with inverted data. Note  When       considering the previous read access  consider that the last access is a previous access to       the first  as a  wrap around . Please refer to section on Non Destructive test for more       details on how to program these bits."]
        #[inline(always)]
        pub fn accspat(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Config1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xff,1,0,u8, Config1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Select Fast Bit   SELFASTB. This field defines during a 2 i test        the address bit position that has the Hamming distance of 1  i. e.        changes fastest. Bit 0 of either column or row address is swapped with        the indicated bit of either column or row according to MCONTROL.RCADR. MCONTROL.RCADR 0          gt  column MCONTROL.RCADR 1   gt  row"]
        #[inline(always)]
        pub fn selfastb(
            self,
        ) -> crate::common::RegisterField<
            8,
            0xf,
            1,
            0,
            config1::Selfastb,
            Config1_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0xf,
                1,
                0,
                config1::Selfastb,
                Config1_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Address Generator Mode   AG MOD. These bits enable the special hardware for performing the more complex        addressing schemes. In case RANGE.RAEN  range enable  is set to 0  single access  linear        address mode has to be selected and NUMACCS set to 1."]
        #[inline(always)]
        pub fn ag_mod(
            self,
        ) -> crate::common::RegisterField<
            12,
            0xf,
            1,
            0,
            config1::AgMod,
            Config1_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                12,
                0xf,
                1,
                0,
                config1::AgMod,
                Config1_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for Config1 {
        #[inline(always)]
        fn default() -> Config1 {
            <crate::RegValueT<Config1_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod config1 {
        pub struct Selfastb_SPEC;
        pub type Selfastb = crate::EnumBitfieldStruct<u8, Selfastb_SPEC>;
        impl Selfastb {
            #[doc = "0000 normal        addressing sequence  bit 0 in its normal position."]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct AgMod_SPEC;
        pub type AgMod = crate::EnumBitfieldStruct<u8, AgMod_SPEC>;
        impl AgMod {
            #[doc = "0000 run        the test with linear address generation"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "0001 run        the right half select test"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "0010 run        the test with GALPAT9 algorithm"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "0011 run        the left half select test"]
            pub const CONST_33: Self = Self::new(3);
            #[doc = "0100 run        the test with the GALPAT5 algorithm"]
            pub const CONST_44: Self = Self::new(4);
            #[doc = "0101 run        the non destructive test. The march elements  direction and backgrounds        are defined by other settings in CONFIG0 1 and MCONTROL. For this test         the SRAM has to be pre initialized with valid content  i.e. with ECC        correct data . Unlike a normal MBIST march test  this test uses the ECC        itself to find errors in the data. The result of the test is not        reflected via MSTATUS.FAIL   instead  the detected ECC errors are        tracked in the ETRR  amp  ERRINFO registers  and additionally ECCD  ERR bits        if the alarm notifications are enabled. registers"]
            pub const CONST_55: Self = Self::new(5);
            #[doc = "1000 run        the write mask test"]
            pub const CONST_88: Self = Self::new(8);
            #[doc = "1010 run        the test with 2 i address generation"]
            pub const CONST_1010: Self = Self::new(10);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccd_SPEC;
    impl crate::sealed::RegSpec for Eccd_SPEC {
        type DataType = u16;
    }
    #[doc = "Memory ECC Detection Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0}"]
    pub type Eccd = crate::RegValueT<Eccd_SPEC>;

    impl Eccd {
        #[doc = "Error Detected   SERR. Write of   8217 0  8217  clears the sticky status. Write of   8217 1  8217  has no effect. In the case of a write of   8216 0  8217  simultaneously with an error detection         the setting of the bit by hardware will take priority. This bit is reset        with an Application Reset. Read as"]
        #[inline(always)]
        pub fn serr(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, eccd::Serr, Eccd_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1,1,0,eccd::Serr, Eccd_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "CE alarm occured   CERR. Write of   8217 0  8217  clears the bit  and enables further alarms to be forwarded        to SMU. Write of   8217 1  8217  has no effect. When the bit is set  software can perform additional diagnostics from        the information in the ETRR ERRINFO registers. Please refer to the        safety section for more details. This bit is reset with an Application        Reset. Read as"]
        #[inline(always)]
        pub fn cerr(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, eccd::Cerr, Eccd_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0x1,1,0,eccd::Cerr, Eccd_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Uncorrectable Error Alarm Occured   UCERR. Write of   8217 0  8217  clears the bit  and enables further alarms to be forwarded        to SMU. When the bit is set  software can perform additional diagnostics        from the information in the ETRR ERRINFO registers. Please refer to the        safety section for more details. Write of  1  has no effect. This bit is        cleared on an application reset. Read as"]
        #[inline(always)]
        pub fn ucerr(
            self,
        ) -> crate::common::RegisterField<2, 0x1, 1, 0, eccd::Ucerr, Eccd_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x1,1,0,eccd::Ucerr, Eccd_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Miscellaneous Error Alarm Occured   MERR. Write of   8217 0  8217  clears the bit  and enables further alarms to be forwarded        to SMU. When the bit is set  software can perform additional diagnostics        from the information in the ETRR ERRINFO and ALMSRCS registers. Please        refer to the safety section for more details. Write of  1  has no        effect. This bit is reset with an application reset. Read as"]
        #[inline(always)]
        pub fn merr(
            self,
        ) -> crate::common::RegisterField<3, 0x1, 1, 0, eccd::Merr, Eccd_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<3,0x1,1,0,eccd::Merr, Eccd_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Tracking Clear   TRC. Writing this bit with  1  clears the EOV  VAL bits plus the ETRR       and ERRINFO registers  depending on the PERMERR settings. This bit will always read 0."]
        #[inline(always)]
        pub fn trc(
            self,
        ) -> crate::common::RegisterField<4, 0x1, 1, 0, eccd::Trc, Eccd_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<4,0x1,1,0,eccd::Trc, Eccd_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Permanent Error in ETRR Entry   PERMERR. Denotes an ETRR entry that shall not be cleared by setting the TRC or        moved up when a new error occurs. With this bit set  the corresponding        ETRR ERRINFO entry remain as they are until a PORST."]
        #[inline(always)]
        pub fn permerr(
            self,
        ) -> crate::common::RegisterField<10, 0x1f, 1, 0, eccd::Permerr, Eccd_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                10,
                0x1f,
                1,
                0,
                eccd::Permerr,
                Eccd_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for Eccd {
        #[inline(always)]
        fn default() -> Eccd {
            <crate::RegValueT<Eccd_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod eccd {
        pub struct Serr_SPEC;
        pub type Serr = crate::EnumBitfieldStruct<u8, Serr_SPEC>;
        impl Serr {
            #[doc = "No error detected."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "An error was detected and alarm forwarded  CERR  UCERR or MERR."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cerr_SPEC;
        pub type Cerr = crate::EnumBitfieldStruct<u8, Cerr_SPEC>;
        impl Cerr {
            #[doc = "No CE alarm event occured."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "CE alarm event occured."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Ucerr_SPEC;
        pub type Ucerr = crate::EnumBitfieldStruct<u8, Ucerr_SPEC>;
        impl Ucerr {
            #[doc = "No UCE alarm event occured."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 UCE alarm event        occured"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Merr_SPEC;
        pub type Merr = crate::EnumBitfieldStruct<u8, Merr_SPEC>;
        impl Merr {
            #[doc = "No ME Alarm Event occured."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "ME Alarm Event occured."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Trc_SPEC;
        pub type Trc = crate::EnumBitfieldStruct<u8, Trc_SPEC>;
        impl Trc {
            #[doc = "No effect."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the ETRR  ERRINFO and       ECCD.VAL  amp  EOV bits. If a PERMERR bit is set  then the corresponding entries are not       cleared.  Note  If PERMERR and TRC are written at the same time  the clearing due to TRC       takes place with the previous PERMERR settings  and the new settings take effect only after."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Permerr_SPEC;
        pub type Permerr = crate::EnumBitfieldStruct<u8, Permerr_SPEC>;
        impl Permerr {
            #[doc = "The corresponding entry in ETRR can be cleared by setting TRC."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "The corresponding entry in ETRR shall not be cleared by setting TRC."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccs_SPEC;
    impl crate::sealed::RegSpec for Eccs_SPEC {
        type DataType = u16;
    }
    #[doc = "ECC Safety Register\n resetvalue={Application Reset:0x1F}"]
    pub type Eccs = crate::RegValueT<Eccs_SPEC>;

    impl Eccs {
        #[doc = "ECC Correction Event Alarm Notification Enable   CENE. This bit enables the forwarding of the CE alarm from the SSH to the SMU."]
        #[inline(always)]
        pub fn cene(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, eccs::Cene, Eccs_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1,1,0,eccs::Cene, Eccs_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Uncorrectable Error Affecting SRAM   SSH Operation  Alarm Notification Enable   UENE. This bit enables the forwarding of the UCE alarm from the SSH to       the SMU. Please refer to the section on safety for more details."]
        #[inline(always)]
        pub fn ucene(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, eccs::Ucene, Eccs_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0x1,1,0,eccs::Ucene, Eccs_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Miscellaneous Alarm Notification Enable  MENE. This bit enables the forwarding of the ME alarm from the SSH to the SMU.        Please refer to the section on safety for more details."]
        #[inline(always)]
        pub fn mene(
            self,
        ) -> crate::common::RegisterField<2, 0x1, 1, 0, eccs::Mene, Eccs_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x1,1,0,eccs::Mene, Eccs_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Error Correction Enable   ECE. This enables the single bit error correction by the ECC. If this bit is        1  single bit errors are flagged via the CE alarm. If this bit is 0         single bit errors are flagged via the UE alarm."]
        #[inline(always)]
        pub fn ece(
            self,
        ) -> crate::common::RegisterField<3, 0x1, 1, 0, eccs::Ece, Eccs_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<3,0x1,1,0,eccs::Ece, Eccs_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Tracking Enable   TRE. All errors will be tracked  if the associated notification enable bit is        set. This bit is enabled by default."]
        #[inline(always)]
        pub fn tre(
            self,
        ) -> crate::common::RegisterField<4, 0x1, 1, 0, eccs::Tre, Eccs_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x1,1,0,eccs::Tre, Eccs_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Bit Flip Enable   BFLE"]
        #[inline(always)]
        pub fn bfle(
            self,
        ) -> crate::common::RegisterField<5, 0x1, 1, 0, eccs::Bfle, Eccs_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x1,1,0,eccs::Bfle, Eccs_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Signature Bit Flip Enables   SFLE. This bit injects an address error by flipping       the LSB address bit to the address error detection unit  but not to the SRAM. If address error detection is enabled  ALMSRCS.ADDRE   1  and If       this bit is set and the SRAM is read  an address error is notified  and tracked in the ETRR        amp  ERRINFO registers  as well as an alarm is generated  if enabled. Note that for       SRAMs with Address ECC  refer the Appendix chapter for the list   this bit is ignored  and no       error will be generated."]
        #[inline(always)]
        pub fn sfle(
            self,
        ) -> crate::common::RegisterField<6, 0x1, 1, 0, eccs::Sfle, Eccs_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<6,0x1,1,0,eccs::Sfle, Eccs_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "ECC Bit Mapping Mode   ECCMAP. ECCMAP sets three different test modes to allow access to data or ECC        bits separately and independently. Also         memory bypass  11  mode enables a complete bypass of the whole memory.        The same mode is enabled independently from the bit setting by hardware        if sx ssh com lbist i input signal is set to high. All bypass        modes can only be used if the memory is accessible via some bus by a        processor. Otherwise these modes cannot be supported."]
        #[inline(always)]
        pub fn eccmap(
            self,
        ) -> crate::common::RegisterField<8, 0x3, 1, 0, eccs::Eccmap, Eccs_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x3,1,0,eccs::Eccmap, Eccs_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "TriCore Tower Select   TC TWR SEL. For TriCore PMEM only. This bit selects a cache way to run the        non destructive inversion test on. This bit represents the Tower number."]
        #[inline(always)]
        pub fn tc_twr_sel(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, Eccs_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,Eccs_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Safety Flip Flop Diagnostics   SFFD. Safety Flip Flop Diagnostics bit. Setting this bit triggers a Safety        Flip Flop self test. The result of the test  i.e. any error status in        the safety FFs    can be obtained from the OPERR or MISCERR bits in the        FAULTSTS register."]
        #[inline(always)]
        pub fn sffd(
            self,
        ) -> crate::common::RegisterField<11, 0x1, 1, 0, eccs::Sffd, Eccs_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<11,0x1,1,0,eccs::Sffd, Eccs_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Eccs {
        #[inline(always)]
        fn default() -> Eccs {
            <crate::RegValueT<Eccs_SPEC> as RegisterValue<_>>::new(31)
        }
    }
    pub mod eccs {
        pub struct Cene_SPEC;
        pub type Cene = crate::EnumBitfieldStruct<u8, Cene_SPEC>;
        impl Cene {
            #[doc = "Do not forward CE alarm to SMU."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Forward the CE alarm to SMU."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Ucene_SPEC;
        pub type Ucene = crate::EnumBitfieldStruct<u8, Ucene_SPEC>;
        impl Ucene {
            #[doc = "Do not forward the UCE alarm       to the SMU."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Forward the UE alarm to the SMU"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Mene_SPEC;
        pub type Mene = crate::EnumBitfieldStruct<u8, Mene_SPEC>;
        impl Mene {
            #[doc = "Do not forward the ME alarm to the SMU."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Forward the ME alarm to the SMU."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Ece_SPEC;
        pub type Ece = crate::EnumBitfieldStruct<u8, Ece_SPEC>;
        impl Ece {
            #[doc = "Do not correct correctable errors."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Correct        correctable errors"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tre_SPEC;
        pub type Tre = crate::EnumBitfieldStruct<u8, Tre_SPEC>;
        impl Tre {
            #[doc = "Do not track address of detected error."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Track address of detected error."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Bfle_SPEC;
        pub type Bfle = crate::EnumBitfieldStruct<u8, Bfle_SPEC>;
        impl Bfle {
            #[doc = "Normal operation."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Test mode only. Flips data and check bits according to RDBFL."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Sfle_SPEC;
        pub type Sfle = crate::EnumBitfieldStruct<u8, Sfle_SPEC>;
        impl Sfle {
            #[doc = "1 Forces address error injection by flipping bit 0  of         the address to the address error detection logic  but not to the SRAM. This results in an         address error to be generated."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "0 Do        not force address error injection."]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Eccmap_SPEC;
        pub type Eccmap = crate::EnumBitfieldStruct<u8, Eccmap_SPEC>;
        impl Eccmap {
            #[doc = "00 Normal        operation"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Test mode.        Only data bits mapped. All ECC functionality disabled."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Test mode. ECC        check bits mapped to lower data bit positions. Other bits read as zero.        All ECC functionality disabled. Data bits are not affected by write        operations."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Memory           bypass mode. Instead of memory locations the RDBFLregister is used for read and write. Only full width memory accesses allowed  i.e. no half select  Do not use this setting."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Sffd_SPEC;
        pub type Sffd = crate::EnumBitfieldStruct<u8, Sffd_SPEC>;
        impl Sffd {
            #[doc = "Do not trigger an SFF self test."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Trigger an SFF self test. Bit is cleared automatically by the hardware        when the test is complete."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Errinfo_SPEC;
    impl crate::sealed::RegSpec for Errinfo_SPEC {
        type DataType = u16;
    }
    #[doc = "Error Information Register 0\n resetvalue={PowerOn Reset:0x0}"]
    pub type Errinfo = crate::RegValueT<Errinfo_SPEC>;

    impl Errinfo {
        #[doc = "Single Bit Error Detected   SBERR. Read as"]
        #[inline(always)]
        pub fn sberr(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            errinfo::Sberr,
            Errinfo_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                errinfo::Sberr,
                Errinfo_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Double Bit Error Detected   DBERR. Read as"]
        #[inline(always)]
        pub fn dberr(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            errinfo::Dberr,
            Errinfo_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                errinfo::Dberr,
                Errinfo_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Address Fault Detected   ADDRERR. Read as"]
        #[inline(always)]
        pub fn addrerr(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            errinfo::Addrerr,
            Errinfo_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                errinfo::Addrerr,
                Errinfo_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for Errinfo {
        #[inline(always)]
        fn default() -> Errinfo {
            <crate::RegValueT<Errinfo_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod errinfo {
        pub struct Sberr_SPEC;
        pub type Sberr = crate::EnumBitfieldStruct<u8, Sberr_SPEC>;
        impl Sberr {
            #[doc = "No Single Bit error detected at the memory address in the corresponding        ETRRx register."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Single Bit        detected at the memory address in the corresponding ETRRx register."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Dberr_SPEC;
        pub type Dberr = crate::EnumBitfieldStruct<u8, Dberr_SPEC>;
        impl Dberr {
            #[doc = "No Double Bit error detected at the memory address in the corresponding        ETRRx register."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Double Bit         error detected at the memory address in the corresponding ETRRx register. Note       that for SRAMs with Address ECC  refer to the Appendix chapter for the list of such SRAMs in       the device   this bit is also set if an error in the Address bits are detected."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Addrerr_SPEC;
        pub type Addrerr = crate::EnumBitfieldStruct<u8, Addrerr_SPEC>;
        impl Addrerr {
            #[doc = "No address error detected at the memory address in the corresponding        ETRRx register."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Address         error detected at the memory address in the corresponding ETRRx register. Note       that for SRAMs with Address ECC  this bit is not used. For such SRAMs  errors in both Address       and Data bits are notified by the DBERR bits."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Etrr_SPEC;
    impl crate::sealed::RegSpec for Etrr_SPEC {
        type DataType = u16;
    }
    #[doc = "Error Tracking Register 0\n resetvalue={PowerOn Reset:0x0}"]
    pub type Etrr = crate::RegValueT<Etrr_SPEC>;

    impl Etrr {
        #[doc = "Memory Block Index of Error i    MBI. If more than one memory is implemented in parallel  these three bits        contain the index of the memory block in error to identify the memory in        error and the tracked address belongs to this memory. Otherwise these        bits always are set to 0."]
        #[inline(always)]
        pub fn mbi(
            self,
        ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, Etrr_SPEC, crate::common::R> {
            crate::common::RegisterField::<13,0x7,1,0,u8, Etrr_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for Etrr {
        #[inline(always)]
        fn default() -> Etrr {
            <crate::RegValueT<Etrr_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcontrol_SPEC;
    impl crate::sealed::RegSpec for Mcontrol_SPEC {
        type DataType = u16;
    }
    #[doc = "MBIST Control Register\n resetvalue={Application Reset:0x4008}"]
    pub type Mcontrol = crate::RegValueT<Mcontrol_SPEC>;

    impl Mcontrol {
        #[doc = "START   START. If this bit is written to  160   8217 1  8217  by software the memory test will start. If        it is reset by software  and the test has finished  MSTATUS.DONE will be        set to 1. If MCONTROL.FAILDMP is set  a fail will stop the current execution.        RESUME will continue a suspended test."]
        #[inline(always)]
        pub fn start(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            mcontrol::Start,
            Mcontrol_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                mcontrol::Start,
                Mcontrol_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Resume failed test   RESUME. This bit allows a test with fail that got suspended to be resumed after        the dump of the fail bit map. A restart is possible only if MSTATUS.FDA        was reset by hardware. It will be reset by hardware once the test is        resumed."]
        #[inline(always)]
        pub fn resume(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            mcontrol::Resume,
            Mcontrol_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                mcontrol::Resume,
                Mcontrol_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Enable Sticky Fail Bit   ESTF. This bit enables the sticky fail bit MSTATUS.SFAIL. If set any fails        will be collected in MSTATUS.SFAIL. Resetting this bit to 0 will also        reset MSTATUS.SFAIL."]
        #[inline(always)]
        pub fn estf(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            mcontrol::Estf,
            Mcontrol_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                mcontrol::Estf,
                Mcontrol_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Direction Select   DIR. This field specifies the direction of a memory test operation."]
        #[inline(always)]
        pub fn dir(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            mcontrol::Dir,
            Mcontrol_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                mcontrol::Dir,
                Mcontrol_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Data Initialization Enable   DINIT. This bit enables a write of the RDBFL data to all locations defined by        the range register. RDBFL can contain data that will produce an ECC        error. Execution is started with MCONTROL.START. For this predefined        action any information contained in CONFIG0 1 registers and the bits        BITTOG  ROWTOG and DIR are ignored."]
        #[inline(always)]
        pub fn dinit(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            mcontrol::Dinit,
            Mcontrol_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                mcontrol::Dinit,
                Mcontrol_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Fast Row   Fast Column Addressing Scheme Select   RCADR. This bit selects between fast row and fast column addressing.   8220 Fast Row  8221         moves along the word lines first and then in bit line direction    8220 Fast        Column  8221  along the bit lines first."]
        #[inline(always)]
        pub fn rcadr(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            mcontrol::Rcadr,
            Mcontrol_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                mcontrol::Rcadr,
                Mcontrol_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Row toggling   ROWTOG. This field specifies whether to toggle the used bit pattern  non        inverted inverted  with each physical memory row. This is required when        writing a checkerboard pattern or a row stripe pattern. For GALPAT this bit has to be 0."]
        #[inline(always)]
        pub fn rowtog(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x1,
            1,
            0,
            mcontrol::Rowtog,
            Mcontrol_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                6,
                0x1,
                1,
                0,
                mcontrol::Rowtog,
                Mcontrol_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Bit toggling   BITTOG. This field specifies whether to toggle the used bit pattern  non        inverted inverted  with each physical memory column. This is required        when writing a checkerboard pattern or a column stripe pattern. For GALPAT this bit has to be 0."]
        #[inline(always)]
        pub fn bittog(
            self,
        ) -> crate::common::RegisterField<
            7,
            0x1,
            1,
            0,
            mcontrol::Bittog,
            Mcontrol_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                mcontrol::Bittog,
                Mcontrol_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Galpat Base   GP BASE. This bit defines the value which is written into the base cell during        the galpat mode. MCONTROL.BITTOG and MCONTROL.ROWTOG have to be 0 for        GALPAT mode."]
        #[inline(always)]
        pub fn gp_base(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1,
            1,
            0,
            mcontrol::GpBase,
            Mcontrol_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                mcontrol::GpBase,
                Mcontrol_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Fail bitmap dump   FAILDMP. This field enables a dump of the failing address and a fail bit map        after a fault has been detected. The memory test is suspended afterwards        and resumed by MCONTROL.RESUME. MSTATUS.FDA shows that a fail dump is        available. This functionality can be used only if bit MCONTROL.LDRED    160 1. In case a fail dump is available  RDBFL will contain the fail bit map        and ETRR the failing address."]
        #[inline(always)]
        pub fn faildmp(
            self,
        ) -> crate::common::RegisterField<
            9,
            0x1,
            1,
            0,
            mcontrol::Faildmp,
            Mcontrol_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                9,
                0x1,
                1,
                0,
                mcontrol::Faildmp,
                Mcontrol_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Enable Descrambling. This bit has an effect only when the SSH itself is enabled. If this bit        is set  the internal address de scrambler in the SSH will be enabled. That        is  if this bit is set  the logical addresses generated from the SSH        state machine and given to the SRAM input are translated to physical        addresses. The reset value is 0  hence the de scrambler is not        enabled by default  i.e.         by default the logical addresses are not translated to physical  ."]
        #[inline(always)]
        pub fn en_descr(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            mcontrol::EnDescr,
            Mcontrol_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                mcontrol::EnDescr,
                Mcontrol_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Initialize Redundancy Settings   INITRED. This field enables initialization  i.e. clearing  of the RAR register        inside of the redundancy wrapper in the first clock cycle after the        memory test has been started."]
        #[inline(always)]
        pub fn initred(
            self,
        ) -> crate::common::RegisterField<
            12,
            0x1,
            1,
            0,
            mcontrol::Initred,
            Mcontrol_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                12,
                0x1,
                1,
                0,
                mcontrol::Initred,
                Mcontrol_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Load Redundancy   LDRED. Setting both USERED and LDRED to 1 together has      the same effect as setting USERED to 1. This field enables the dynamic update of fault information in the        redundancy wrapper  register RAR and RDR lt k gt   during the memory test."]
        #[inline(always)]
        pub fn ldred(
            self,
        ) -> crate::common::RegisterField<
            13,
            0x1,
            1,
            0,
            mcontrol::Ldred,
            Mcontrol_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                13,
                0x1,
                1,
                0,
                mcontrol::Ldred,
                Mcontrol_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Use Redundancy   USERED. Setting both USERED and      LDRED to 1 together has the same effect as setting USERED to 1. This field activates the replacement of faulty memory cells by redundant        cells  register RDR lt k gt   during the memory test. Thus a test on a          8220 repaired  8221  memory can be executed."]
        #[inline(always)]
        pub fn usered(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x1,
            1,
            0,
            mcontrol::Usered,
            Mcontrol_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                mcontrol::Usered,
                Mcontrol_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Clear the SRAM   SRAM CLR. This bit initializes the complete SRAM with ECC correct  All 0        data. Execution is started with MCONTROL.START. For this predefined action any information       contained in CONFIG0 1  RANGE registers and the bits BITTOG  ROWTOG and DIR are ignored. This       bit shall not be set together with other initialization or test configurations. After the SRAM       clearing is complete  the software has to reset this bit back to  0  before disabling the SSH. From a normal application  it is forbidden to         set this bit together with other initialization features. However  in case it happens. then         this bit has lower priority than auto data init or partial erase triggered via MTU socket         signals  but higher priority than initialization triggered by MCONTROL.DINIT and the effect         will be always that of executing the higher priority alone."]
        #[inline(always)]
        pub fn sram_clr(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            mcontrol::SramClr,
            Mcontrol_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                mcontrol::SramClr,
                Mcontrol_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for Mcontrol {
        #[inline(always)]
        fn default() -> Mcontrol {
            <crate::RegValueT<Mcontrol_SPEC> as RegisterValue<_>>::new(16392)
        }
    }
    pub mod mcontrol {
        pub struct Start_SPEC;
        pub type Start = crate::EnumBitfieldStruct<u8, Start_SPEC>;
        impl Start {
            #[doc = "1 Start memory        test"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "0 No test        started  finished or waiting for test end"]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Resume_SPEC;
        pub type Resume = crate::EnumBitfieldStruct<u8, Resume_SPEC>;
        impl Resume {
            #[doc = "0 Do not resume"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Resume suspended MBIST run"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Estf_SPEC;
        pub type Estf = crate::EnumBitfieldStruct<u8, Estf_SPEC>;
        impl Estf {
            #[doc = "0 Do not collect fail events"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Collect fail events"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Dir_SPEC;
        pub type Dir = crate::EnumBitfieldStruct<u8, Dir_SPEC>;
        impl Dir {
            #[doc = "0 DOWN  Address direction is highest to lowest."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 UP  Address direction is lowest to highest."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Dinit_SPEC;
        pub type Dinit = crate::EnumBitfieldStruct<u8, Dinit_SPEC>;
        impl Dinit {
            #[doc = "0 Disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rcadr_SPEC;
        pub type Rcadr = crate::EnumBitfieldStruct<u8, Rcadr_SPEC>;
        impl Rcadr {
            #[doc = "0 Fast row"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Fast column"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rowtog_SPEC;
        pub type Rowtog = crate::EnumBitfieldStruct<u8, Rowtog_SPEC>;
        impl Rowtog {
            #[doc = "0 Do not toggle"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Do toggle with each row"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Bittog_SPEC;
        pub type Bittog = crate::EnumBitfieldStruct<u8, Bittog_SPEC>;
        impl Bittog {
            #[doc = "0 Do not toggle"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Do toggle with each column"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct GpBase_SPEC;
        pub type GpBase = crate::EnumBitfieldStruct<u8, GpBase_SPEC>;
        impl GpBase {
            #[doc = "0 Write        the inverted value of CONFIG.ACCSPAT into the base cell prior to reading        the background pattern from the adjacent cells"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write        the non inverted value of CONFIG.ACCSPAT into the base cell prior to        reading the background pattern from the adjacent cells"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Faildmp_SPEC;
        pub type Faildmp = crate::EnumBitfieldStruct<u8, Faildmp_SPEC>;
        impl Faildmp {
            #[doc = "0 Do not dump"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Dump each fault"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct EnDescr_SPEC;
        pub type EnDescr = crate::EnumBitfieldStruct<u8, EnDescr_SPEC>;
        impl EnDescr {
            #[doc = "Descrambler is not enabled in the address generation path within the SSH  logical        addresses generated within the SSH will be presented directly to SRAM        inputs ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Descrambler is enabled in the address generation path within the SSH  logical        addresses generated within the SSH will be first descrambled to physical        addresses before presenting to SRAM inputs ."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Initred_SPEC;
        pub type Initred = crate::EnumBitfieldStruct<u8, Initred_SPEC>;
        impl Initred {
            #[doc = "0 INB do        not initialize redundancy"]
            pub const INB_0: Self = Self::new(0);
            #[doc = "1 IN do initialize"]
            pub const IN_1: Self = Self::new(1);
        }
        pub struct Ldred_SPEC;
        pub type Ldred = crate::EnumBitfieldStruct<u8, Ldred_SPEC>;
        impl Ldred {
            #[doc = "0 UPB do not update fault information"]
            pub const UPB_0: Self = Self::new(0);
            #[doc = "1 UP do update"]
            pub const UP_1: Self = Self::new(1);
        }
        pub struct Usered_SPEC;
        pub type Usered = crate::EnumBitfieldStruct<u8, Usered_SPEC>;
        impl Usered {
            #[doc = "0 REDB do        not consider redundancy"]
            pub const REDB_0: Self = Self::new(0);
            #[doc = "1 RED use        redundant cells"]
            pub const RED_1: Self = Self::new(1);
        }
        pub struct SramClr_SPEC;
        pub type SramClr = crate::EnumBitfieldStruct<u8, SramClr_SPEC>;
        impl SramClr {
            #[doc = "Clear the entire SRAM. The SRAM is fully filled with zeroes  and is also        ECC correct."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Do not clear the entire SRAM."]
            pub const CONST_00: Self = Self::new(0);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mstatus_SPEC;
    impl crate::sealed::RegSpec for Mstatus_SPEC {
        type DataType = u16;
    }
    #[doc = "Status Register\n resetvalue={Application Reset:0x1}"]
    pub type Mstatus = crate::RegValueT<Mstatus_SPEC>;

    impl Mstatus {
        #[doc = "DONE   DONE. This bit is reset at the start of a test and set when a test is        completed and MCONTROL.START was reset by software. It is not set when a        test is interrupted for fail dump."]
        #[inline(always)]
        pub fn done(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, Mstatus_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0,1,0,Mstatus_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "FAIL   FAIL. This bit will be reset when a test is being started. It will be set to          8216 1  8217  by hardware under the following conditions  a  when performing a test without redundancy  MCONTROL.LDRED   8217 0  8217  and        MCONTROL.USERED   8217 0  8217    FAIL    8217 1  8217  if the memory has at least one fault. Here fault includes any        error in the data  as well as an address error  when enabled via        ALMSRCS.ADDRE. In this case  an application reset has to be issued to        clear the FAIL bit. b  when performing a test with dynamic update of redundancy         MCONTROL.LDRED   8217 1  8217  and MCONTROL.USERED   8217 0  8217    FAIL    8217 1  8217  if the number of memory faults exceeds the number of redundant        cells. In this case  an application reset has to be performed to clear        the FAIL bit. c  when performing a test with pre configured redundancy         MCONTROL.USERED   8217 1  8217    FAIL    8217 1  8217  if the memory or redundancy has at least one fault"]
        #[inline(always)]
        pub fn fail(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, mstatus::Fail, Mstatus_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                mstatus::Fail,
                Mstatus_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Fail Dump Available   FDA. This bit shows that a fail has occurred if MCONTROL.FAILDMP is set. The        test is suspended and fail dump information is available. The fail bit        map is in RDBFL and the associated address is in ETRR 0 . As long as no        fail has occurred RDBFL contains the last read information and ETRR has        no valid data . This bit will be set by hardware. It will be reset when MSTATUS was read with MSTATUS.FDA   1 and the dump        information was read from ETRR and RDBFL. Only the last read from the        last word of RDBFL is checked by the hardware and taken as an indication        for a complete read. A suspended test will be resumed by MCONTROL.RESUME if FDA was reset.        This forms some sort of handshake to insure that a suspended test can        only be resumed  by a broadcasted  MCONTROL.RESUME if the last fail        information was actually collected."]
        #[inline(always)]
        pub fn fda(
            self,
        ) -> crate::common::RegisterField<2, 0x1, 1, 0, mstatus::Fda, Mstatus_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<2,0x1,1,0,mstatus::Fda, Mstatus_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Sticky Fail Bit   SFAIL. This bit is set to 1 together with MSTATUS.FAIL provided MCONTROL.ESTF        is set. In contrast to FAIL it will not be reset when a new test is        started. Therefore it will collect fail information over more than one        MBIST run. It will be reset when MCONTROL.ESTF is reset  or MBIST mode        is switched off."]
        #[inline(always)]
        pub fn sfail(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            mstatus::Sfail,
            Mstatus_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                mstatus::Sfail,
                Mstatus_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "More Than One FAIL in RDR   MTOF RDR. This bit is meaningful only when USERED   0. When USERED   1  this bit        has no meaning and stays at 0. Whenever USERED   0  this bit reflects        combinatorially the current contents of all RDR registers  and is set if        the value of any  one or more  of the RDR contains more than one  1 .        This bit is automatically updated  may be after a few clock cycles for        the bit accumulation logic  to reflect the updated current contents of        RDR. If it needs to be found which indivudual RDR s  have more than        one  1    each of the RDR registers would have to be indivudually read        by the software   test program  and the bitmaps indivudually examined. When        USERED is 1  this bit is automatically updated to 0  and as long as        USERED is 1  stays at 0 and does not reflect the RDR contents anymore. Usecase  This        bit is used to get a More Than One Fail status. When running tests with        LDRED   1  the RDR reflects the fail bitmap. The bitmap in each        indivudual RDR register itself is accumulated over multiple MBIST tests         see description of LDRED   1 . During the test flow  the RDR RAR may        also be preloaded with a specific value to get incremental test results.        Hence the need to combinatorially reflect the contents of RDR        continuously when USERED   0."]
        #[inline(always)]
        pub fn mtof_rdr(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            mstatus::MtofRdr,
            Mstatus_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                mstatus::MtofRdr,
                Mstatus_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for Mstatus {
        #[inline(always)]
        fn default() -> Mstatus {
            <crate::RegValueT<Mstatus_SPEC> as RegisterValue<_>>::new(1)
        }
    }
    pub mod mstatus {
        pub struct Fail_SPEC;
        pub type Fail = crate::EnumBitfieldStruct<u8, Fail_SPEC>;
        impl Fail {
            #[doc = "0 no error        occurred"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 detailed        description see above"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Fda_SPEC;
        pub type Fda = crate::EnumBitfieldStruct<u8, Fda_SPEC>;
        impl Fda {
            #[doc = "0 No        fail dump data available. A suspended MBIST run can be resumed."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Fail dump data is available and waiting for read."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Sfail_SPEC;
        pub type Sfail = crate::EnumBitfieldStruct<u8, Sfail_SPEC>;
        impl Sfail {
            #[doc = "0 No        fail collected."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 A fail occured during one of the test runs since MCONTROL.ESTF was set to 1."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct MtofRdr_SPEC;
        pub type MtofRdr = crate::EnumBitfieldStruct<u8, MtofRdr_SPEC>;
        impl MtofRdr {
            #[doc = "0 If USERED   0         it means no RDR register value has more than one 1 set. If USERED   1         the bit is meaningless and stays at 0."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 If USERED   0         it means some RDR register s  value has have  more than one 1 set. If        USERED   1  the bit is meaningless and stays at 0. detailed description        see above"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ramtest_SPEC;
    impl crate::sealed::RegSpec for Ramtest_SPEC {
        type DataType = u16;
    }
    #[doc = "Programmable Self Timing Register\n resetvalue={PowerOn Reset:0x0}"]
    pub type Ramtest = crate::RegValueT<Ramtest_SPEC>;

    impl Ramtest {
        #[doc = "Assist for Stability Writability   R AST. Feature to detect weak cells. Please refer C40 SRAM manual for detailed        description."]
        #[inline(always)]
        pub fn r_ast(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Ramtest_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7,1,0,u8, Ramtest_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Programmable Self Timer Write   R PSTW. The R PSTW bus influences the write window."]
        #[inline(always)]
        pub fn r_pstw(
            self,
        ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Ramtest_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0xf,1,0,u8, Ramtest_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Programmable Self Timer Read   R PSTR. The R PSTR bus influences the read window shortening or fastening the        read timing."]
        #[inline(always)]
        pub fn r_pstr(
            self,
        ) -> crate::common::RegisterField<9, 0xf, 1, 0, u8, Ramtest_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<9,0xf,1,0,u8, Ramtest_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Burn In Word Line Select Even   BIWLSO. This bit enables a special burn in stress with odd word lines selected. See BIWLSE for more information."]
        #[inline(always)]
        pub fn biwlso(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, Ramtest_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,Ramtest_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Burn In Word Line Select Even   BIWLSE. This bit enables a special burn in stress with even word lines selected. Either BIWLSE or BIWLSO or none can be selected. During these special        stress situations the clock to the SRAMs is switched off. See the c40 SRAM manual before using this feature."]
        #[inline(always)]
        pub fn biwlse(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, Ramtest_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,Ramtest_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Ramtest {
        #[inline(always)]
        fn default() -> Ramtest {
            <crate::RegValueT<Ramtest_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Range_SPEC;
    impl crate::sealed::RegSpec for Range_SPEC {
        type DataType = u16;
    }
    #[doc = "Range Register  single address mode\n resetvalue={Application Reset:0x0}"]
    pub type Range = crate::RegValueT<Range_SPEC>;

    impl Range {
        #[doc = "Range Enable   RAEN. 0 Disabled  single address mode. In this case a single word can be        addressed for read or write. Config registers have to be set as follows CONFIG.NUMACCS     8220 0001  8221   single access  CONFIG.AG MOD      8220 0000  8221   linear  MCONTROL.DIR   1  up  For read just the value in this location will be delivered. No check        against expected values is made  i.e. MSTATUS.FAIL will not be set. 1 Enabled. ADDR 13 7  is interpreted as Upper Range Limit. ADDR 6 0  is        interpreted as Lower Range Limit."]
        #[inline(always)]
        pub fn raen(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, Range_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,Range_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Range {
        #[inline(always)]
        fn default() -> Range {
            <crate::RegValueT<Range_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rar_SPEC;
    impl crate::sealed::RegSpec for Rar_SPEC {
        type DataType = u16;
    }
    #[doc = "Redundant Address Register 0\n resetvalue={Test Reset:0x0}"]
    pub type Rar = crate::RegValueT<Rar_SPEC>;

    impl Rar {
        #[doc = "Valid Flag   VALID. If this bit is set  the address entered in the ADDR field is valid to be        replaced by a redundancy cell."]
        #[inline(always)]
        pub fn valid(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, Rar_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15, 1, 0, Rar_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
    }
    impl core::default::Default for Rar {
        #[inline(always)]
        fn default() -> Rar {
            <crate::RegValueT<Rar_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rdbfl_SPEC;
    impl crate::sealed::RegSpec for Rdbfl_SPEC {
        type DataType = u16;
    }
    #[doc = "Read Data and Bit Flip Register 0\n resetvalue={Application Reset:0x0}"]
    pub type Rdbfl = crate::RegValueT<Rdbfl_SPEC>;

    impl Rdbfl {
        #[doc = "Word Data   WDATA. This field contains the data of the last memory read operation."]
        #[inline(always)]
        pub fn wdata(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Rdbfl_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, Rdbfl_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Rdbfl {
        #[inline(always)]
        fn default() -> Rdbfl {
            <crate::RegValueT<Rdbfl_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Revid_SPEC;
    impl crate::sealed::RegSpec for Revid_SPEC {
        type DataType = u16;
    }
    #[doc = "Revision ID Register\n resetvalue={Application Reset:0x610}"]
    pub type Revid = crate::RegValueT<Revid_SPEC>;

    impl Revid {
        #[doc = "Revision Identifier   REV ID. This field defines the currently implemented release  version and        functionality of the used MBIST ECC controller to track the MBIST ECC        version for easier handling at the tester."]
        #[inline(always)]
        pub fn rev_id(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Revid_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, Revid_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for Revid {
        #[inline(always)]
        fn default() -> Revid {
            <crate::RegValueT<Revid_SPEC> as RegisterValue<_>>::new(1552)
        }
    }
}
