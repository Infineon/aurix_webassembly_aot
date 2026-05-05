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
#[doc = r"SENT"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sent(pub(super) *mut u8);
unsafe impl core::marker::Send for Sent {}
unsafe impl core::marker::Sync for Sent {}
impl Sent {
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

    #[doc = "SENT Fractional Divider Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fdr(&self) -> crate::common::Reg<self::Fdr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x080C000}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Interrupt Overview Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn intov(&self) -> crate::common::Reg<self::Intov_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst0(&self) -> crate::common::Reg<self::Krst0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(244usize)) }
    }

    #[doc = "Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst1(&self) -> crate::common::Reg<self::Krst1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(240usize)) }
    }

    #[doc = "Kernel Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krstclr(&self) -> crate::common::Reg<self::Krstclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(236usize)) }
    }

    #[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(232usize)) }
    }

    #[doc = "Receive Data Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rdrx(&self) -> [crate::common::Reg<self::RdRx_SPEC, crate::common::R>; 15] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x38usize)),
            ]
        }
    }

    #[doc = "Receive Time Stamp Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rtsx(&self) -> [crate::common::Reg<self::RtSx_SPEC, crate::common::R>; 15] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0xa80usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0xa80usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0xa80usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0xa80usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0xa80usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0xa80usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0xa80usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0xa80usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0xa80usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0xa80usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0xa80usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0xa80usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0xa80usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0xa80usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0xa80usize + 0x38usize)),
            ]
        }
    }

    #[doc = "Time Stamp Predivider Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tpd(&self) -> crate::common::Reg<self::Tpd_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "Module Time Stamp Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tsr(&self) -> crate::common::Reg<self::Tsr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "CH"]
    #[inline(always)]
    pub fn ch(self) -> [self::Ch; 15] {
        unsafe {
            [
                self::Ch(self.0.add(0x100usize + 0x0usize)),
                self::Ch(self.0.add(0x100usize + 0x40usize)),
                self::Ch(self.0.add(0x100usize + 0x80usize)),
                self::Ch(self.0.add(0x100usize + 0xc0usize)),
                self::Ch(self.0.add(0x100usize + 0x100usize)),
                self::Ch(self.0.add(0x100usize + 0x140usize)),
                self::Ch(self.0.add(0x100usize + 0x180usize)),
                self::Ch(self.0.add(0x100usize + 0x1c0usize)),
                self::Ch(self.0.add(0x100usize + 0x200usize)),
                self::Ch(self.0.add(0x100usize + 0x240usize)),
                self::Ch(self.0.add(0x100usize + 0x280usize)),
                self::Ch(self.0.add(0x100usize + 0x2c0usize)),
                self::Ch(self.0.add(0x100usize + 0x300usize)),
                self::Ch(self.0.add(0x100usize + 0x340usize)),
                self::Ch(self.0.add(0x100usize + 0x380usize)),
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
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, accen0::En0, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,accen0::En0, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, accen0::En1, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,accen0::En1, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, accen0::En2, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,accen0::En2, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, accen0::En3, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,accen0::En3, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, accen0::En4, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,accen0::En4, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, accen0::En5, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,accen0::En5, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, accen0::En6, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,accen0::En6, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, accen0::En7, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,accen0::En7, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, accen0::En8, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,accen0::En8, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, accen0::En9, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,accen0::En9, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, accen0::En10, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,accen0::En10, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, accen0::En11, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,accen0::En11, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, accen0::En12, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,accen0::En12, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, accen0::En13, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,accen0::En13, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, accen0::En14, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,accen0::En14, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, accen0::En15, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,accen0::En15, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, accen0::En16, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,accen0::En16, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, accen0::En17, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,accen0::En17, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, accen0::En18, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,accen0::En18, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, accen0::En19, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,accen0::En19, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, accen0::En20, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,accen0::En20, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, accen0::En21, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,accen0::En21, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, accen0::En22, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,accen0::En22, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, accen0::En23, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,accen0::En23, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, accen0::En24, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,accen0::En24, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, accen0::En25, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,accen0::En25, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, accen0::En26, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,accen0::En26, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, accen0::En27, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,accen0::En27, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, accen0::En28, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,accen0::En28, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, accen0::En29, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,accen0::En29, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, accen0::En30, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,accen0::En30, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
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
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control of the module."]
    #[inline(always)]
    pub fn disr(self) -> crate::common::RegisterFieldBool<0, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Disable Status Bit   DISS. Bit indicates the current status of the module."]
    #[inline(always)]
    pub fn diss(self) -> crate::common::RegisterFieldBool<1, 1, 0, Clc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Clc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Sleep Mode Enable Control   EDIS. Used to control module  8217 s sleep mode."]
    #[inline(always)]
    pub fn edis(self) -> crate::common::RegisterFieldBool<3, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Clc {
    #[inline(always)]
    fn default() -> Clc {
        <crate::RegValueT<Clc_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdr_SPEC;
impl crate::sealed::RegSpec for Fdr_SPEC {
    type DataType = u32;
}
#[doc = "SENT Fractional Divider Register\n resetvalue={Application Reset:0x0}"]
pub type Fdr = crate::RegValueT<Fdr_SPEC>;

impl Fdr {
    #[doc = "Step Value   STEP. Reload or addition value for RESULT."]
    #[inline(always)]
    pub fn step(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Fdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Fdr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Divider Mode   DM. DM selects normal or fractional divider mode."]
    #[inline(always)]
    pub fn dm(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, fdr::Dm, Fdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,fdr::Dm, Fdr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Result Value   RESULT. Bit field for the addition result."]
    #[inline(always)]
    pub fn result(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Fdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Fdr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Fdr {
    #[inline(always)]
    fn default() -> Fdr {
        <crate::RegValueT<Fdr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fdr {
    pub struct Dm_SPEC;
    pub type Dm = crate::EnumBitfieldStruct<u8, Dm_SPEC>;
    impl Dm {
        #[doc = "00 Fractional divider is switched off  no output clock is generated. The Reset External Divider signal is 1. RESULT is not updated  default after System   class 0 Reset ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Normal Divider Mode selected."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Fractional Divider Mode selected."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Fractional divider is switched off  no output clock is generated. RESULT is not updated."]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x080C000}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MODREV. This bit field defines the module revision number. The value of a module        revision starts with 01 H  first        revision ."]
    #[inline(always)]
    pub fn modrev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MODTYPE. This bit field defines the module as a 32 bit module  C0 H"]
    #[inline(always)]
    pub fn modtype(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MODNUM. This bit field defines the module identification number for the SENT          0080 H"]
    #[inline(always)]
    pub fn modnum(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Id_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Id {
    #[inline(always)]
    fn default() -> Id {
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(8437760)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intov_SPEC;
impl crate::sealed::RegSpec for Intov_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Overview Register\n resetvalue={Application Reset:0x0}"]
pub type Intov = crate::RegValueT<Intov_SPEC>;

impl Intov {
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc16(self) -> crate::common::RegisterFieldBool<16, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc17(self) -> crate::common::RegisterFieldBool<17, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc18(self) -> crate::common::RegisterFieldBool<18, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc19(self) -> crate::common::RegisterFieldBool<19, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc20(self) -> crate::common::RegisterFieldBool<20, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc21(self) -> crate::common::RegisterFieldBool<21, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc22(self) -> crate::common::RegisterFieldBool<22, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc23(self) -> crate::common::RegisterFieldBool<23, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Pending on Channel 24   IPC24. If any interrupt requested flag is set for channel y in register        INTSTATy AND the referring interrupt is enabled in INTENx then IPCy is        set. It is automatically reset if all flags in INTSTATy are cleared for        which the referring interrupt is enabled in INTENx. Not all IPC0 24 are available on all products  the number of Interrupt          Pending on Channel is equivalent to the SENT channels available  e.g 4          SENT channels has 4 Interrupt Pending IPC0 3 and vice versa."]
    #[inline(always)]
    pub fn ipc24(self) -> crate::common::RegisterFieldBool<24, 1, 0, Intov_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Intov_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Intov {
    #[inline(always)]
    fn default() -> Intov {
        <crate::RegValueT<Intov_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krst0_SPEC;
impl crate::sealed::RegSpec for Krst0_SPEC {
    type DataType = u32;
}
#[doc = "Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
pub type Krst0 = crate::RegValueT<Krst0_SPEC>;

impl Krst0 {
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel registers are set. The RST bit will be cleared  re set to  0   by the BPI FPI after the kernel reset was executed."]
    #[inline(always)]
    pub fn rst(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, krst0::Rst, Krst0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,krst0::Rst, Krst0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Kernel Reset Status   RSTSTAT. This bit indicates wether a kernel reset was executed or not. This bit is set by the BPI FPI after the execution of a kernel reset in the same clock cycle both reset bits. This bit can be cleared by writing with  1  to the CLR bit in the related KRSTCLR register."]
    #[inline(always)]
    pub fn rststat(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, krst0::Rststat, Krst0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,krst0::Rststat, Krst0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Krst0 {
    #[inline(always)]
    fn default() -> Krst0 {
        <crate::RegValueT<Krst0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod krst0 {
    pub struct Rst_SPEC;
    pub type Rst = crate::EnumBitfieldStruct<u8, Rst_SPEC>;
    impl Rst {
        #[doc = "0 No kernel reset was requested"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A kernel reset was requested"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rststat_SPEC;
    pub type Rststat = crate::EnumBitfieldStruct<u8, Rststat_SPEC>;
    impl Rststat {
        #[doc = "0 No kernel reset was executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Kernel reset was executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krst1_SPEC;
impl crate::sealed::RegSpec for Krst1_SPEC {
    type DataType = u32;
}
#[doc = "Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
pub type Krst1 = crate::RegValueT<Krst1_SPEC>;

impl Krst1 {
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel reset registers is set. The RST bit will be cleared  re set to  0   by the BPI FPI after the kernel reset was executed."]
    #[inline(always)]
    pub fn rst(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, krst1::Rst, Krst1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,krst1::Rst, Krst1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Krst1 {
    #[inline(always)]
    fn default() -> Krst1 {
        <crate::RegValueT<Krst1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod krst1 {
    pub struct Rst_SPEC;
    pub type Rst = crate::EnumBitfieldStruct<u8, Rst_SPEC>;
    impl Rst {
        #[doc = "0 No kernel reset was requested"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A kernel reset was requested"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krstclr_SPEC;
impl crate::sealed::RegSpec for Krstclr_SPEC {
    type DataType = u32;
}
#[doc = "Kernel Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
pub type Krstclr = crate::RegValueT<Krstclr_SPEC>;

impl Krstclr {
    #[doc = "Kernel Reset Status Clear   CLR. Read always as 0."]
    #[inline(always)]
    pub fn clr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, krstclr::Clr, Krstclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x1,1,0,krstclr::Clr, Krstclr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Krstclr {
    #[inline(always)]
    fn default() -> Krstclr {
        <crate::RegValueT<Krstclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod krstclr {
    pub struct Clr_SPEC;
    pub type Clr = crate::EnumBitfieldStruct<u8, Clr_SPEC>;
    impl Clr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear Kernel Reset Status KRST0.RSTSTAT"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ocs_SPEC;
impl crate::sealed::RegSpec for Ocs_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0}"]
pub type Ocs = crate::RegValueT<Ocs_SPEC>;

impl Ocs {
    #[doc = "OCDS Suspend Control   SUS. Controls the sensitivity to the suspend signal coming from the OCDS        Trigger Switch  OTGS"]
    #[inline(always)]
    pub fn sus(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, ocs::Sus, Ocs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,ocs::Sus, Ocs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SUS Write Protection   SUS P. SUS is only written when SUS P is 1  otherwise unchanged. Read as 0."]
    #[inline(always)]
    pub fn sus_p(self) -> crate::common::RegisterFieldBool<28, 1, 0, Ocs_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Ocs_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Suspend State   SUSSTA"]
    #[inline(always)]
    pub fn sussta(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, ocs::Sussta, Ocs_SPEC, crate::common::R> {
        crate::common::RegisterField::<29,0x1,1,0,ocs::Sussta, Ocs_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ocs {
    #[inline(always)]
    fn default() -> Ocs {
        <crate::RegValueT<Ocs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ocs {
    pub struct Sus_SPEC;
    pub type Sus = crate::EnumBitfieldStruct<u8, Sus_SPEC>;
    impl Sus {
        #[doc = "0 Will not suspend"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Hard suspend. Clock is switched off immediately.  SPC low pulse breaks immediately  Port Pin keeps the last value if suspend state is entered"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Soft suspend  SPC low pulse will be finished before suspend state is entered"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Sussta_SPEC;
    pub type Sussta = crate::EnumBitfieldStruct<u8, Sussta_SPEC>;
    impl Sussta {
        #[doc = "0 Module is not  yet  suspended"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module is suspended  for channels where SUSEN is set"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RdRx_SPEC;
impl crate::sealed::RegSpec for RdRx_SPEC {
    type DataType = u32;
}
#[doc = "Receive Data Register 0\n resetvalue={Application Reset:0x0}"]
pub type RdRx = crate::RegValueT<RdRx_SPEC>;

impl RdRx {
    #[doc = "Receive Data Nibble 7   RD7. RDy shows the nibble from the received frame that is sorted to this        position. It can be selected by any of VIEWx.RDNPy  y  160    160 0 7 . By default        all nibbles are sorted to RD0 as the reset value of VIEW is        0x0000  160 0000H. I.e. at the end of frame reception RD0 contains the last        data nibble of the frame."]
    #[inline(always)]
    pub fn rd0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, RdRx_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xf, 1, 0, u8, RdRx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive Data Nibble 7   RD7. RDy shows the nibble from the received frame that is sorted to this        position. It can be selected by any of VIEWx.RDNPy  y  160    160 0 7 . By default        all nibbles are sorted to RD0 as the reset value of VIEW is        0x0000  160 0000H. I.e. at the end of frame reception RD0 contains the last        data nibble of the frame."]
    #[inline(always)]
    pub fn rd1(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, RdRx_SPEC, crate::common::R> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, RdRx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive Data Nibble 7   RD7. RDy shows the nibble from the received frame that is sorted to this        position. It can be selected by any of VIEWx.RDNPy  y  160    160 0 7 . By default        all nibbles are sorted to RD0 as the reset value of VIEW is        0x0000  160 0000H. I.e. at the end of frame reception RD0 contains the last        data nibble of the frame."]
    #[inline(always)]
    pub fn rd2(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, RdRx_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xf, 1, 0, u8, RdRx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive Data Nibble 7   RD7. RDy shows the nibble from the received frame that is sorted to this        position. It can be selected by any of VIEWx.RDNPy  y  160    160 0 7 . By default        all nibbles are sorted to RD0 as the reset value of VIEW is        0x0000  160 0000H. I.e. at the end of frame reception RD0 contains the last        data nibble of the frame."]
    #[inline(always)]
    pub fn rd3(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, RdRx_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0xf,1,0,u8, RdRx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive Data Nibble 7   RD7. RDy shows the nibble from the received frame that is sorted to this        position. It can be selected by any of VIEWx.RDNPy  y  160    160 0 7 . By default        all nibbles are sorted to RD0 as the reset value of VIEW is        0x0000  160 0000H. I.e. at the end of frame reception RD0 contains the last        data nibble of the frame."]
    #[inline(always)]
    pub fn rd4(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, RdRx_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xf,1,0,u8, RdRx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive Data Nibble 7   RD7. RDy shows the nibble from the received frame that is sorted to this        position. It can be selected by any of VIEWx.RDNPy  y  160    160 0 7 . By default        all nibbles are sorted to RD0 as the reset value of VIEW is        0x0000  160 0000H. I.e. at the end of frame reception RD0 contains the last        data nibble of the frame."]
    #[inline(always)]
    pub fn rd5(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, RdRx_SPEC, crate::common::R> {
        crate::common::RegisterField::<20,0xf,1,0,u8, RdRx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive Data Nibble 7   RD7. RDy shows the nibble from the received frame that is sorted to this        position. It can be selected by any of VIEWx.RDNPy  y  160    160 0 7 . By default        all nibbles are sorted to RD0 as the reset value of VIEW is        0x0000  160 0000H. I.e. at the end of frame reception RD0 contains the last        data nibble of the frame."]
    #[inline(always)]
    pub fn rd6(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, RdRx_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xf,1,0,u8, RdRx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive Data Nibble 7   RD7. RDy shows the nibble from the received frame that is sorted to this        position. It can be selected by any of VIEWx.RDNPy  y  160    160 0 7 . By default        all nibbles are sorted to RD0 as the reset value of VIEW is        0x0000  160 0000H. I.e. at the end of frame reception RD0 contains the last        data nibble of the frame."]
    #[inline(always)]
    pub fn rd7(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, RdRx_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0xf,1,0,u8, RdRx_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for RdRx {
    #[inline(always)]
    fn default() -> RdRx {
        <crate::RegValueT<RdRx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtSx_SPEC;
impl crate::sealed::RegSpec for RtSx_SPEC {
    type DataType = u32;
}
#[doc = "Receive Time Stamp Register 0\n resetvalue={Application Reset:0x0}"]
pub type RtSx = crate::RegValueT<RtSx_SPEC>;

impl RtSx {
    #[doc = "Last Receive Time Stamp for Channel x   LTS. This bit field shows the time stamp of the last frame on channel x."]
    #[inline(always)]
    pub fn lts(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, RtSx_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, RtSx_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for RtSx {
    #[inline(always)]
    fn default() -> RtSx {
        <crate::RegValueT<RtSx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tpd_SPEC;
impl crate::sealed::RegSpec for Tpd_SPEC {
    type DataType = u32;
}
#[doc = "Time Stamp Predivider Register\n resetvalue={Application Reset:0x0}"]
pub type Tpd = crate::RegValueT<Tpd_SPEC>;

impl Tpd {
    #[doc = "Divider Factor of Pre Divider for TSR   TDIV. Divides f fracdiv by  TDIV   1  and drives TSR."]
    #[inline(always)]
    pub fn tdiv(
        self,
    ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, Tpd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfffff,1,0,u32, Tpd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Tpd {
    #[inline(always)]
    fn default() -> Tpd {
        <crate::RegValueT<Tpd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsr_SPEC;
impl crate::sealed::RegSpec for Tsr_SPEC {
    type DataType = u32;
}
#[doc = "Module Time Stamp Register\n resetvalue={Application Reset:0x0}"]
pub type Tsr = crate::RegValueT<Tsr_SPEC>;

impl Tsr {
    #[doc = "Current Time Stamp for the Module   CTS. This bit field shows the current time stamp."]
    #[inline(always)]
    pub fn cts(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Tsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Tsr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Tsr {
    #[inline(always)]
    fn default() -> Tsr {
        <crate::RegValueT<Tsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc = "CH"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch(pub(super) *mut u8);
unsafe impl core::marker::Send for Ch {}
unsafe impl core::marker::Sync for Ch {}
impl Ch {
    #[doc = "Channel Fractional Divider Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cfdrx(&self) -> crate::common::Reg<ch::CfdRx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Channel Pre Divider Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cpdrx(&self) -> crate::common::Reg<ch::CpdRx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Interrupt Node Pointer Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn inpx(&self) -> crate::common::Reg<ch::InPx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }
    #[doc = "Interrupt Clear Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn intclrx(&self) -> crate::common::Reg<ch::IntclRx_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }
    #[doc = "Interrupt Enable Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn intenx(&self) -> crate::common::Reg<ch::InteNx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }
    #[doc = "Interrupt Set Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn intsetx(&self) -> crate::common::Reg<ch::IntseTx_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Interrupt Status Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn intstatx(&self) -> crate::common::Reg<ch::IntstaTx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "Input and Output Control Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn iocrx(&self) -> crate::common::Reg<ch::IocRx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Receiver Control Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rcrx(&self) -> crate::common::Reg<ch::RcRx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Receive Status Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rsrx(&self) -> crate::common::Reg<ch::RsRx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "SPC Control Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn scrx(&self) -> crate::common::Reg<ch::ScRx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Serial Data and Status Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sdsx(&self) -> crate::common::Reg<ch::SdSx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Receive Data View Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn viewx(&self) -> crate::common::Reg<ch::VieWx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Watch Dog Timer Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn wdtx(&self) -> crate::common::Reg<ch::WdTx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }
}
pub mod ch {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CfdRx_SPEC;
    impl crate::sealed::RegSpec for CfdRx_SPEC {
        type DataType = u32;
    }
    #[doc = "Channel Fractional Divider Register 0\n resetvalue={Application Reset:0x0}"]
    pub type CfdRx = crate::RegValueT<CfdRx_SPEC>;

    impl CfdRx {
        #[doc = "Divider Value   DIV. Initial and reference divider value for the CFDR.  DIV must be programmed  gt  0. If cleared  DIV becomes 1.  If written  DIVM is updated automatically with the same value. This can be helpful to accelerate a return to normal. RCR.CEN must be cleared before changing CPDR.PDIV or CFDR.DIV. DIV must be set in the range of  2200  49100 ."]
        #[inline(always)]
        pub fn div(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, CfdRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, CfdRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Measured Divider Value   DIVM. DIVM is automatically updated by HW to adjust the receiver frequency to the current sender frequency. This value is kept automatically in the range of 75  DIV  lt  DIVM  lt  125  DIV Write data is ignored."]
        #[inline(always)]
        pub fn divm(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, CfdRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, CfdRx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for CfdRx {
        #[inline(always)]
        fn default() -> CfdRx {
            <crate::RegValueT<CfdRx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpdRx_SPEC;
    impl crate::sealed::RegSpec for CpdRx_SPEC {
        type DataType = u32;
    }
    #[doc = "Channel Pre Divider Register 0\n resetvalue={Application Reset:0x0}"]
    pub type CpdRx = crate::RegValueT<CpdRx_SPEC>;

    impl CpdRx {
        #[doc = "Divider Factor of Pre Divider for Channel x   PDIV. Divides f fracdiv by  PDIV   1  and delivers f pdiv x to the Channel Fractional Divider. RCR.CEN must be cleared before        changing CPDR.PDIV or CFDR.DIV."]
        #[inline(always)]
        pub fn pdiv(
            self,
        ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, CpdRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xfff,1,0,u16, CpdRx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for CpdRx {
        #[inline(always)]
        fn default() -> CpdRx {
            <crate::RegValueT<CpdRx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct InPx_SPEC;
    impl crate::sealed::RegSpec for InPx_SPEC {
        type DataType = u32;
    }
    #[doc = "Interrupt Node Pointer Register 0\n resetvalue={Application Reset:0x0}"]
    pub type InPx = crate::RegValueT<InPx_SPEC>;

    impl InPx {
        #[doc = "Interrupt Node Pointer for Interrupt RSI   RSI. This bit field defines the interrupt node  that is requested due to the        set condition for bit INTSTATx.RSI  if enabled by bit INTENx.RSI ."]
        #[inline(always)]
        pub fn rsi(
            self,
        ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, InPx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<0,0xf,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Interrupt Node Pointer for Interrupt RDI   RDI. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATx.RDI  if enabled by bit INTENx.RDI . For bit field definition  see RSI."]
        #[inline(always)]
        pub fn rdi(
            self,
        ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, InPx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<4,0xf,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Interrupt Node Pointer for Interrupt RBI   RBI. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATx.RBI  if enabled by bit INTENx.RBI . For bit field definition  see RSI."]
        #[inline(always)]
        pub fn rbi(
            self,
        ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, InPx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<8,0xf,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Interrupt Node Pointer for Interrupt TDI   TDI. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATx.TDI  if enabled by bit INTENx.TDI . For bit field definition  see RSI."]
        #[inline(always)]
        pub fn tdi(
            self,
        ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, InPx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<12,0xf,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Interrupt Node Pointer for Interrupt TBI   TBI. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATx.TBI  if enabled by bit INTENx.TBI . For bit field definition  see RSI."]
        #[inline(always)]
        pub fn tbi(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, InPx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<16,0xf,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Interrupt Node Pointer for Interrupt FRI  FDI  NNI  NVI  CRCI  WSI  SCRI   ERRI. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATx.FRI  if enabled by bit INTENx.FRI  or INTSTATx.FDI  if enabled by bit INTENx.FDI  or INTSTATx.NNI  if enabled by bit INTENx.NNI  or INTSTATx.NVI  if enabled by bit INTENx.NVI  or INTSTATx.CRCI  if enabled by bit INTENx.CRCI  or INTSTATx.WSI  if enabled by bit INTENx.WSI  or INTSTATx.SCRI  if enabled by bit INTENx.SCRI  For bit field definition  see RSI."]
        #[inline(always)]
        pub fn erri(
            self,
        ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, InPx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<20,0xf,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Interrupt Node Pointer for Interrupt SDI   SDI. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATx.SDI  if enabled by bit INTENx.SDI . For bit field definition  see RSI."]
        #[inline(always)]
        pub fn sdi(
            self,
        ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, InPx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<24,0xf,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Interrupt Node Pointer for Interrupt WDI   WDI. This bit field defines the interrupt node  that is requested due to the set condition for bit INTSTATx.WDI  if enabled by bit INTENx.WDI . For bit field definition  see RSI."]
        #[inline(always)]
        pub fn wdi(
            self,
        ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, InPx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<28,0xf,1,0,u8, InPx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for InPx {
        #[inline(always)]
        fn default() -> InPx {
            <crate::RegValueT<InPx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntclRx_SPEC;
    impl crate::sealed::RegSpec for IntclRx_SPEC {
        type DataType = u32;
    }
    #[doc = "Interrupt Clear Register 0\n resetvalue={Application Reset:0x0}"]
    pub type IntclRx = crate::RegValueT<IntclRx_SPEC>;

    impl IntclRx {
        #[doc = "Clear Interrupt Request Flag RSI   RSI. Setting this bit clears bit INTSTATx.RSI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn rsi(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, IntclRx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<0,1,0,IntclRx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Interrupt Request Flag RDI   RDI. Setting this bit clears bit INTSTATx.RDI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn rdi(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, IntclRx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<1,1,0,IntclRx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Interrupt Request Flag RBI   RBI. Setting this bit clears bit INTSTATx.RBI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn rbi(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, IntclRx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<2,1,0,IntclRx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Interrupt Request Flag TDI   TDI. Setting this bit clears bit INTSTATx.TDI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn tdi(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, IntclRx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<3,1,0,IntclRx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Interrupt Request Flag TBI   TBI. Setting this bit clears bit INTSTATx.TBI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn tbi(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, IntclRx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<4,1,0,IntclRx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Interrupt Request Flag FRI   FRI. Setting this bit clears bit INTSTATx.FRI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn fri(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, IntclRx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<5,1,0,IntclRx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Interrupt Request Flag FDI   FDI. Setting this bit clears bit INTSTATx.FDI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn fdi(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, IntclRx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<6,1,0,IntclRx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Interrupt Request Flag NNI   NNI. Setting this bit clears bit INTSTATx.NNI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn nni(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, IntclRx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<7,1,0,IntclRx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Interrupt Request Flag NVI   NVI. Setting this bit clears bit INTSTATx.NVI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn nvi(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, IntclRx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<8,1,0,IntclRx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Interrupt Request Flag CRCI   CRCI. Setting this bit clears bit INTSTATx.CRCI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn crci(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, IntclRx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<9,1,0,IntclRx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Interrupt Request Flag WSI   WSI. Setting this bit clears bit INTSTATx.WSI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn wsi(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, IntclRx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<10,1,0,IntclRx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Interrupt Request Flag SDI   SDI. Setting this bit clears bit INTSTATx.SDI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn sdi(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, IntclRx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<11,1,0,IntclRx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Interrupt Request Flag SCRI   SCRI. Setting this bit clears bit INTSTATx.SCRI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn scri(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, IntclRx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<12,1,0,IntclRx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Interrupt Request Flag WDI   WDI. Setting this bit clears bit INTSTATx.WDI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn wdi(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, IntclRx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<13,1,0,IntclRx_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for IntclRx {
        #[inline(always)]
        fn default() -> IntclRx {
            <crate::RegValueT<IntclRx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct InteNx_SPEC;
    impl crate::sealed::RegSpec for InteNx_SPEC {
        type DataType = u32;
    }
    #[doc = "Interrupt Enable Register 0\n resetvalue={Application Reset:0x0}"]
    pub type InteNx = crate::RegValueT<InteNx_SPEC>;

    impl InteNx {
        #[doc = "Enable Interrupt Request RSI   RSI"]
        #[inline(always)]
        pub fn rsi(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, intenx::Rsi, InteNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1,1,0,intenx::Rsi, InteNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable Interrupt Request RDI   RDI"]
        #[inline(always)]
        pub fn rdi(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, intenx::Rdi, InteNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0x1,1,0,intenx::Rdi, InteNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable Interrupt Request RBI   RBI"]
        #[inline(always)]
        pub fn rbi(
            self,
        ) -> crate::common::RegisterField<2, 0x1, 1, 0, intenx::Rbi, InteNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x1,1,0,intenx::Rbi, InteNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable Interrupt Request TDI   TDI"]
        #[inline(always)]
        pub fn tdi(
            self,
        ) -> crate::common::RegisterField<3, 0x1, 1, 0, intenx::Tdi, InteNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<3,0x1,1,0,intenx::Tdi, InteNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable Interrupt Request TBI   TBI"]
        #[inline(always)]
        pub fn tbi(
            self,
        ) -> crate::common::RegisterField<4, 0x1, 1, 0, intenx::Tbi, InteNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x1,1,0,intenx::Tbi, InteNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable Interrupt Request FRI   FRI"]
        #[inline(always)]
        pub fn fri(
            self,
        ) -> crate::common::RegisterField<5, 0x1, 1, 0, intenx::Fri, InteNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x1,1,0,intenx::Fri, InteNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable Interrupt Request FDI   FDI"]
        #[inline(always)]
        pub fn fdi(
            self,
        ) -> crate::common::RegisterField<6, 0x1, 1, 0, intenx::Fdi, InteNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<6,0x1,1,0,intenx::Fdi, InteNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable Interrupt Request NNI   NNI"]
        #[inline(always)]
        pub fn nni(
            self,
        ) -> crate::common::RegisterField<7, 0x1, 1, 0, intenx::Nni, InteNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<7,0x1,1,0,intenx::Nni, InteNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable Interrupt Request NVI   NVI"]
        #[inline(always)]
        pub fn nvi(
            self,
        ) -> crate::common::RegisterField<8, 0x1, 1, 0, intenx::Nvi, InteNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x1,1,0,intenx::Nvi, InteNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable Interrupt Request CRCI   CRCI"]
        #[inline(always)]
        pub fn crci(
            self,
        ) -> crate::common::RegisterField<9, 0x1, 1, 0, intenx::Crci, InteNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<9,0x1,1,0,intenx::Crci, InteNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable Interrupt Request WSI   WSI"]
        #[inline(always)]
        pub fn wsi(
            self,
        ) -> crate::common::RegisterField<10, 0x1, 1, 0, intenx::Wsi, InteNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<10,0x1,1,0,intenx::Wsi, InteNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable Interrupt Request SDI   SDI"]
        #[inline(always)]
        pub fn sdi(
            self,
        ) -> crate::common::RegisterField<11, 0x1, 1, 0, intenx::Sdi, InteNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<11,0x1,1,0,intenx::Sdi, InteNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable Interrupt Request SCRI   SCRI"]
        #[inline(always)]
        pub fn scri(
            self,
        ) -> crate::common::RegisterField<12, 0x1, 1, 0, intenx::Scri, InteNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                12,
                0x1,
                1,
                0,
                intenx::Scri,
                InteNx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Enable Interrupt Request WDI   WDI"]
        #[inline(always)]
        pub fn wdi(
            self,
        ) -> crate::common::RegisterField<13, 0x1, 1, 0, intenx::Wdi, InteNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<13,0x1,1,0,intenx::Wdi, InteNx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for InteNx {
        #[inline(always)]
        fn default() -> InteNx {
            <crate::RegValueT<InteNx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod intenx {
        pub struct Rsi_SPEC;
        pub type Rsi = crate::EnumBitfieldStruct<u8, Rsi_SPEC>;
        impl Rsi {
            #[doc = "0 No interrupt        request can be generated for this source"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt request can be generated for this source"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rdi_SPEC;
        pub type Rdi = crate::EnumBitfieldStruct<u8, Rdi_SPEC>;
        impl Rdi {
            #[doc = "0 No interrupt        request can be generated for this source"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt request can be generated for this source"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rbi_SPEC;
        pub type Rbi = crate::EnumBitfieldStruct<u8, Rbi_SPEC>;
        impl Rbi {
            #[doc = "0 No interrupt        request can be generated for this source"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt request can be generated for this source"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tdi_SPEC;
        pub type Tdi = crate::EnumBitfieldStruct<u8, Tdi_SPEC>;
        impl Tdi {
            #[doc = "0 No interrupt        request can be generated for this source"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt request can be generated for this source"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tbi_SPEC;
        pub type Tbi = crate::EnumBitfieldStruct<u8, Tbi_SPEC>;
        impl Tbi {
            #[doc = "0 No interrupt        request can be generated for this source"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt request can be generated for this source"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Fri_SPEC;
        pub type Fri = crate::EnumBitfieldStruct<u8, Fri_SPEC>;
        impl Fri {
            #[doc = "0 No interrupt        request can be generated for this source"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt request can be generated for this source"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Fdi_SPEC;
        pub type Fdi = crate::EnumBitfieldStruct<u8, Fdi_SPEC>;
        impl Fdi {
            #[doc = "0 No interrupt        request can be generated for this source"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt request can be generated for this source"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nni_SPEC;
        pub type Nni = crate::EnumBitfieldStruct<u8, Nni_SPEC>;
        impl Nni {
            #[doc = "0 No interrupt        request can be generated for this source"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt request can be generated for this source"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nvi_SPEC;
        pub type Nvi = crate::EnumBitfieldStruct<u8, Nvi_SPEC>;
        impl Nvi {
            #[doc = "0 No interrupt        request can be generated for this source"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt request can be generated for this source"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Crci_SPEC;
        pub type Crci = crate::EnumBitfieldStruct<u8, Crci_SPEC>;
        impl Crci {
            #[doc = "0 No interrupt        request can be generated for this source"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt request can be generated for this source"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Wsi_SPEC;
        pub type Wsi = crate::EnumBitfieldStruct<u8, Wsi_SPEC>;
        impl Wsi {
            #[doc = "0 No interrupt        request can be generated for this source"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt request can be generated for this source"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Sdi_SPEC;
        pub type Sdi = crate::EnumBitfieldStruct<u8, Sdi_SPEC>;
        impl Sdi {
            #[doc = "0 No interrupt        request can be generated for this source"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt request can be generated for this source"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Scri_SPEC;
        pub type Scri = crate::EnumBitfieldStruct<u8, Scri_SPEC>;
        impl Scri {
            #[doc = "0 No interrupt        request can be generated for this source"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt request can be generated for this source"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Wdi_SPEC;
        pub type Wdi = crate::EnumBitfieldStruct<u8, Wdi_SPEC>;
        impl Wdi {
            #[doc = "0 No interrupt        request can be generated for this source"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt request can be generated for this source"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntseTx_SPEC;
    impl crate::sealed::RegSpec for IntseTx_SPEC {
        type DataType = u32;
    }
    #[doc = "Interrupt Set Register 0\n resetvalue={Application Reset:0x0}"]
    pub type IntseTx = crate::RegValueT<IntseTx_SPEC>;

    impl IntseTx {
        #[doc = "Set Interrupt Request Flag RSI   RSI. Setting this bit set bit INTSTATx.RSI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn rsi(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, IntseTx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<0,1,0,IntseTx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Set Interrupt Request Flag RDI   RDI. Setting this bit set bit INTSTATx.RDI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn rdi(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, IntseTx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<1,1,0,IntseTx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Set Interrupt Request Flag RBI   RBI. Setting this bit set bit INTSTATx.RBI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn rbi(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, IntseTx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<2,1,0,IntseTx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Set Interrupt Request Flag TDI   TDI. Setting this bit set bit INTSTATx.TDI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn tdi(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, IntseTx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<3,1,0,IntseTx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Set Interrupt Request Flag TBI   TBI. Setting this bit set bit INTSTATx.TBI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn tbi(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, IntseTx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<4,1,0,IntseTx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Set Interrupt Request Flag FRI   FRI. Setting this bit set bit INTSTATx.FRI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn fri(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, IntseTx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<5,1,0,IntseTx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Set Interrupt Request Flag FDI   FDI. Setting this bit set bit INTSTATx.FDI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn fdi(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, IntseTx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<6,1,0,IntseTx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Set Interrupt Request Flag NNI   NNI. Setting this bit set bit INTSTATx.NNI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn nni(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, IntseTx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<7,1,0,IntseTx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Set Interrupt Request Flag NVI   NVI. Setting this bit set bit INTSTATx.NVI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn nvi(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, IntseTx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<8,1,0,IntseTx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Set Interrupt Request Flag CRCI   CRCI. Setting this bit set bit INTSTATx.CRCI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn crci(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, IntseTx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<9,1,0,IntseTx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Set Interrupt Request Flag WSI   WSI. Setting this bit set bit INTSTATx.WSI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn wsi(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, IntseTx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<10,1,0,IntseTx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Set Interrupt Request Flag SDI   SDI. Setting this bit set bit INTSTATx.SDI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn sdi(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, IntseTx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<11,1,0,IntseTx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Set Interrupt Request Flag SCRI   SCRI. Setting this bit set bit INTSTATx.SCRI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn scri(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, IntseTx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<12,1,0,IntseTx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Set Interrupt Request Flag WDI   WDI. Setting this bit set bit INTSTATx.WDI. Clearing this bit has no effect. Reading this bit returns always zero."]
        #[inline(always)]
        pub fn wdi(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, IntseTx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<13,1,0,IntseTx_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for IntseTx {
        #[inline(always)]
        fn default() -> IntseTx {
            <crate::RegValueT<IntseTx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntstaTx_SPEC;
    impl crate::sealed::RegSpec for IntstaTx_SPEC {
        type DataType = u32;
    }
    #[doc = "Interrupt Status Register 0\n resetvalue={Application Reset:0x0}"]
    pub type IntstaTx = crate::RegValueT<IntstaTx_SPEC>;

    impl IntstaTx {
        #[doc = "Receive Success Interrupt Request Flag   RSI. This bit is set at the successfully received end of a frame. Depending on bit RCRx.CDIS this indicates a successful check of the CRC. This bit can be cleared by bit INTCLRx.RSI. This bit can be set by bit INTSETx.RSI. This bit is set independently from INTENx."]
        #[inline(always)]
        pub fn rsi(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            intstatx::Rsi,
            IntstaTx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                intstatx::Rsi,
                IntstaTx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Receive Data Interrupt Request Flag   RDI. RDI is activated when a received frame is moved to a Receive Data        Register RDR. Both RDI and RSI will be issued together in normal use        cases where the frame size is not bigger than 8 nibbles and CRC is        correct or not checked  if RCRx.CDIS is cleared . For frames with more        than 8 nibbles RDI is issued after 8 nibbles. After the last nibbles of        the frame are received  a last RDI and an RSI are issued together. Note        that this is true independently from RCR.FDFL. This bit can be cleared by bit INTCLRx.RDI. This bit can be set by bit INTSETx.RDI. This bit is set independently from INTENx."]
        #[inline(always)]
        pub fn rdi(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            intstatx::Rdi,
            IntstaTx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                intstatx::Rdi,
                IntstaTx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Receive Buffer Overflow Interrupt Request Flag   RBI. This bit is set after a frame has been received while the old one was not read from RDRx. I.e. the kernel wants to set any of the two interrupts RSI and RDI and finds any of these two interrupts already set.The old data is overwritten by the new data. This bit is NOT cleared by reading RDRx. This bit can be cleared by bit INTCLRx.RBI. This bit can be set by bit INTSETx.RBI. This bit is set independently from INTENx."]
        #[inline(always)]
        pub fn rbi(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            intstatx::Rbi,
            IntstaTx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                intstatx::Rbi,
                IntstaTx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Transfer Data Interrupt Request Flag   TDI. This bit is set after the trigger condition was detected. Data to be transferred has been moved internally. Thus a new value can be written to SCRx. This can be used for back to back transfers. This bit is automatically cleared by writing SCRx. This bit can be cleared by bit INTCLRx.TDI. This bit can be set by bit INTSETx.TDI. This bit is set independently from INTENx."]
        #[inline(always)]
        pub fn tdi(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            intstatx::Tdi,
            IntstaTx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                intstatx::Tdi,
                IntstaTx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Transmit Buffer Underflow Interrupt Request Flag   TBI. This bit is set after data has been completely transferred  PLEN exceeded  and no new data was written to SCRx. This bit is NOT cleared by writing SCRx. This bit can be cleared by bit INTCLRx.TBI. This bit can be set by bit INTSETx.TBI. This bit is set independently from INTENx."]
        #[inline(always)]
        pub fn tbi(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            intstatx::Tbi,
            IntstaTx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                intstatx::Tbi,
                IntstaTx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Frequency Range Interrupt Request Flag   FRI. This bit is set after a Synchronization   Calibration pulse was received that deviates more than     25  from the nominal value. The referring data is ignored. This bit can be cleared by bit INTCLRx.FRI. This bit can be set by bit INTSETx.FRI. This bit is set independently from INTENx."]
        #[inline(always)]
        pub fn fri(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            intstatx::Fri,
            IntstaTx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                intstatx::Fri,
                IntstaTx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Frequency Drift Interrupt Request Flag   FDI. This bit is set after a subsequent Synchronization   Calibration pulse was received that deviates more than 1.5625   1 64  from its predecessor.  See RCR.CFC  This bit can be cleared by bit INTCLRx.FDI. This bit can be set by bit INTSETx.FDI. This bit is set independently from INTENx."]
        #[inline(always)]
        pub fn fdi(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x1,
            1,
            0,
            intstatx::Fdi,
            IntstaTx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                6,
                0x1,
                1,
                0,
                intstatx::Fdi,
                IntstaTx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Number of Nibbles Wrong Request Flag   NNI. This bit is set after more nibbles have been received than expected or a        Synchronization   Calibration Pulse is received too early thus too few        nibbles have been received. This bit can be cleared by bit INTCLRx.NNI. This bit can be set by bit INTSETx.NNI. This bit is set independently from INTENx."]
        #[inline(always)]
        pub fn nni(
            self,
        ) -> crate::common::RegisterField<
            7,
            0x1,
            1,
            0,
            intstatx::Nni,
            IntstaTx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                intstatx::Nni,
                IntstaTx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Nibbles Value out of Range Request Flag   NVI. This bit is set after a too long or too short nibble pulse has been        received. I.e. value  lt  0 or value  gt  15. This bit can be cleared by bit INTCLRx.NVI. This bit can be set by bit INTSETx.NVI. This bit is set independently from INTENx."]
        #[inline(always)]
        pub fn nvi(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1,
            1,
            0,
            intstatx::Nvi,
            IntstaTx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                intstatx::Nvi,
                IntstaTx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "CRC Error Request Flag   CRCI. This bit is set if the CRC fails. This bit can be cleared by bit INTCLRx.CRCI. This bit can be set by bit INTSETx.CRCI. This bit is set independently from INTENx."]
        #[inline(always)]
        pub fn crci(
            self,
        ) -> crate::common::RegisterField<
            9,
            0x1,
            1,
            0,
            intstatx::Crci,
            IntstaTx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                9,
                0x1,
                1,
                0,
                intstatx::Crci,
                IntstaTx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Wrong Status and Communication Nibble Error Request Flag   WSI. In Short Serial Frame Mode  RCR.ESF is cleared   this bit is set if the        Status and Communication nibble shows a start bit in a frame other than        frame number n x 16. In Enhanced Serial Frame Mode this bit is without function. This bit can be cleared by bit INTCLRx.WSI. This bit can be set by bit INTSETx.WSI. This bit is set independently from INTENx."]
        #[inline(always)]
        pub fn wsi(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            intstatx::Wsi,
            IntstaTx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                intstatx::Wsi,
                IntstaTx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Serial Data Receive Interrupt Request Flag   SDI. This bit is set after all serial data bits have been received via the Status and Communication nibble. Depending on bit RCRx.SCDIS this indicates a successful check of the CRC. This bit can be cleared by bit INTCLRx.SDI. This bit can be set by bit INTSETx.SDI. This bit is set independently from INTENx."]
        #[inline(always)]
        pub fn sdi(
            self,
        ) -> crate::common::RegisterField<
            11,
            0x1,
            1,
            0,
            intstatx::Sdi,
            IntstaTx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                intstatx::Sdi,
                IntstaTx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Serial Data CRC Error Request Flag   SCRI. This bit is set if the CRC of the serial message fails. In Enhanced        Serial Message Format  this includes a check of the Serial Communication        Nibble for correct 0 values of bit 3 in frames 7  13 and 18. This bit can be cleared by bit INTCLRx.SCRI. This bit can be set by bit INTSETx.SCRI. This bit is set independently from INTENx."]
        #[inline(always)]
        pub fn scri(
            self,
        ) -> crate::common::RegisterField<
            12,
            0x1,
            1,
            0,
            intstatx::Scri,
            IntstaTx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                12,
                0x1,
                1,
                0,
                intstatx::Scri,
                IntstaTx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Watch Dog Error Request Flag   WDI. This bit is set if the Watch Dog Timer of the channel x expires. This bit can be cleared by bit INTCLRx.WDI. This bit can be set by bit INTSETx.WDI. This bit is set independently from INTENx."]
        #[inline(always)]
        pub fn wdi(
            self,
        ) -> crate::common::RegisterField<
            13,
            0x1,
            1,
            0,
            intstatx::Wdi,
            IntstaTx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                13,
                0x1,
                1,
                0,
                intstatx::Wdi,
                IntstaTx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for IntstaTx {
        #[inline(always)]
        fn default() -> IntstaTx {
            <crate::RegValueT<IntstaTx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod intstatx {
        pub struct Rsi_SPEC;
        pub type Rsi = crate::EnumBitfieldStruct<u8, Rsi_SPEC>;
        impl Rsi {
            #[doc = "0 No interrupt        was requested since this bit was cleared the last time"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt was requested since this bit was cleared the last time"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rdi_SPEC;
        pub type Rdi = crate::EnumBitfieldStruct<u8, Rdi_SPEC>;
        impl Rdi {
            #[doc = "0 No interrupt        was requested since this bit was cleared the last time"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt was requested since this bit was cleared the last time"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rbi_SPEC;
        pub type Rbi = crate::EnumBitfieldStruct<u8, Rbi_SPEC>;
        impl Rbi {
            #[doc = "0 No interrupt        was requested since this bit was cleared the last time"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt was requested since this bit was cleared the last time"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tdi_SPEC;
        pub type Tdi = crate::EnumBitfieldStruct<u8, Tdi_SPEC>;
        impl Tdi {
            #[doc = "0 No interrupt        was requested since this bit was cleared the last time"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt was requested since this bit was cleared the last time"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tbi_SPEC;
        pub type Tbi = crate::EnumBitfieldStruct<u8, Tbi_SPEC>;
        impl Tbi {
            #[doc = "0 No interrupt        was requested since this bit was cleared the last time"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt was requested since this bit was cleared the last time"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Fri_SPEC;
        pub type Fri = crate::EnumBitfieldStruct<u8, Fri_SPEC>;
        impl Fri {
            #[doc = "0 No interrupt        was requested since this bit was cleared the last time"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt was requested since this bit was cleared the last time"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Fdi_SPEC;
        pub type Fdi = crate::EnumBitfieldStruct<u8, Fdi_SPEC>;
        impl Fdi {
            #[doc = "0 No interrupt        was requested since this bit was cleared the last time"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt was requested since this bit was cleared the last time"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nni_SPEC;
        pub type Nni = crate::EnumBitfieldStruct<u8, Nni_SPEC>;
        impl Nni {
            #[doc = "0 No interrupt        was requested since this bit was cleared the last time"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt was requested since this bit was cleared the last time"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nvi_SPEC;
        pub type Nvi = crate::EnumBitfieldStruct<u8, Nvi_SPEC>;
        impl Nvi {
            #[doc = "0 No interrupt        was requested since this bit was cleared the last time"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt was requested since this bit was cleared the last time"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Crci_SPEC;
        pub type Crci = crate::EnumBitfieldStruct<u8, Crci_SPEC>;
        impl Crci {
            #[doc = "0 No interrupt        was requested since this bit was cleared the last time"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt was requested since this bit was cleared the last time"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Wsi_SPEC;
        pub type Wsi = crate::EnumBitfieldStruct<u8, Wsi_SPEC>;
        impl Wsi {
            #[doc = "0 No interrupt        was requested since this bit was cleared the last time"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt was requested since this bit was cleared the last time"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Sdi_SPEC;
        pub type Sdi = crate::EnumBitfieldStruct<u8, Sdi_SPEC>;
        impl Sdi {
            #[doc = "0 No interrupt        was requested since this bit was cleared the last time"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt was requested since this bit was cleared the last time"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Scri_SPEC;
        pub type Scri = crate::EnumBitfieldStruct<u8, Scri_SPEC>;
        impl Scri {
            #[doc = "0 No interrupt        was requested since this bit was cleared the last time"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt was requested since this bit was cleared the last time"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Wdi_SPEC;
        pub type Wdi = crate::EnumBitfieldStruct<u8, Wdi_SPEC>;
        impl Wdi {
            #[doc = "0 No interrupt        was requested since this bit was cleared the last time"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt was requested since this bit was cleared the last time"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IocRx_SPEC;
    impl crate::sealed::RegSpec for IocRx_SPEC {
        type DataType = u32;
    }
    #[doc = "Input and Output Control Register 0\n resetvalue={Application Reset:0x0}"]
    pub type IocRx = crate::RegValueT<IocRx_SPEC>;

    impl IocRx {
        #[doc = "Alternate Input Select   ALTI. Selects the alternate input for channel y"]
        #[inline(always)]
        pub fn alti(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, iocrx::Alti, IocRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3,1,0,iocrx::Alti, IocRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Digital Glitch Filter Depth   DEPTH. DEPTH determines the number of port input samples clocked with f pdiv that are taken into account for the calculation of the floating average.        The higher DEPTH is chosen to be  the longer the glitches that are        suppressed and the longer the delay of the input signal introduced by        this filter."]
        #[inline(always)]
        pub fn depth(
            self,
        ) -> crate::common::RegisterField<4, 0xf, 1, 0, iocrx::Depth, IocRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0xf,1,0,iocrx::Depth, IocRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Output Inverter Enable Channel x   OIE. Selects the Pulse Polarity of the output of channel x"]
        #[inline(always)]
        pub fn oie(
            self,
        ) -> crate::common::RegisterField<8, 0x1, 1, 0, iocrx::Oie, IocRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x1,1,0,iocrx::Oie, IocRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Input Inverter Enable Channel x   IIE. Selects the Pulse Polarity of the input of channel x"]
        #[inline(always)]
        pub fn iie(
            self,
        ) -> crate::common::RegisterField<9, 0x1, 1, 0, iocrx::Iie, IocRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<9,0x1,1,0,iocrx::Iie, IocRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Clear Edge Counter   CEC. If this bit is set  IOCR.EC is cleared.  Always reads back as  0 ."]
        #[inline(always)]
        pub fn cec(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, IocRx_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<10,1,0,IocRx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Rising Edge Glitch Flag for Channel x   REG. Shows the status of the glitch detection of channel x REG is cleared by setting CREG."]
        #[inline(always)]
        pub fn reg(
            self,
        ) -> crate::common::RegisterField<12, 0x1, 1, 0, iocrx::Reg, IocRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<12,0x1,1,0,iocrx::Reg, IocRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Falling Edge Glitch Flag for Channel x   FEG. Shows the status of the glitch detection of channel x FEG is cleared by setting CFEG."]
        #[inline(always)]
        pub fn feg(
            self,
        ) -> crate::common::RegisterField<13, 0x1, 1, 0, iocrx::Feg, IocRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<13,0x1,1,0,iocrx::Feg, IocRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Clear Rising Edge Glitch Flag for Channel x   CREG. Clears the status flag REG CREG always read zero."]
        #[inline(always)]
        pub fn creg(
            self,
        ) -> crate::common::RegisterField<14, 0x1, 1, 0, iocrx::Creg, IocRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<14,0x1,1,0,iocrx::Creg, IocRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Clear Falling Edge Glitch Flag for Channel x   CFEG. Clears the status flag FEG CFEG always read zero."]
        #[inline(always)]
        pub fn cfeg(
            self,
        ) -> crate::common::RegisterField<15, 0x1, 1, 0, iocrx::Cfeg, IocRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<15,0x1,1,0,iocrx::Cfeg, IocRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "External Trigger Select   ETS. Selects the external trigger line if SCRx.TRIG is programmed to 11B. In        some products  not all inputs are connected. See CROSSREFERENCE ."]
        #[inline(always)]
        pub fn ets(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, IocRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, IocRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Edge Counter   EC. This bit field contains a counter with saturation  stops at 0xFF . It is incremented with any falling edge that appears on the input pin selected by IOCR.ALTI. Note that this holds true in all states  STOP  INITIALIZED  RUNNING  SYNCHRONIZED .  It is intended for debugging  in particular to find a bubbling idiot that sends before enabling the module."]
        #[inline(always)]
        pub fn ec(
            self,
        ) -> crate::common::RegisterField<20, 0xff, 1, 0, u8, IocRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<20,0xff,1,0,u8, IocRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Clear Trigger Monitor Flag for Channel x   CTR. Clears the status flag TRM CTR always read zero. Reset value of CTR is 0."]
        #[inline(always)]
        pub fn ctr(
            self,
        ) -> crate::common::RegisterField<28, 0x1, 1, 0, iocrx::Ctr, IocRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0x1,1,0,iocrx::Ctr, IocRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Trigger Monitor Flag for Channel x   TRM. Shows the status of the trigger detection of channel x TRM is cleared by setting CTR. Reset value of TRM is 0."]
        #[inline(always)]
        pub fn trm(
            self,
        ) -> crate::common::RegisterField<29, 0x1, 1, 0, iocrx::Trm, IocRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<29,0x1,1,0,iocrx::Trm, IocRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Receive Monitor for Channel x   RXM. Shows the status of the receive signal of channel x after glitch        filtering and inverted as specified by IIE. Reset value of RXM is X."]
        #[inline(always)]
        pub fn rxm(
            self,
        ) -> crate::common::RegisterField<30, 0x1, 1, 0, iocrx::Rxm, IocRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<30,0x1,1,0,iocrx::Rxm, IocRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Transmit Monitor for Channel x   TXM. Shows the status of the transmit signal of channel x inverted as        specified by OIE. Reset value of TXM is X."]
        #[inline(always)]
        pub fn txm(
            self,
        ) -> crate::common::RegisterField<31, 0x1, 1, 0, iocrx::Txm, IocRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<31,0x1,1,0,iocrx::Txm, IocRx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for IocRx {
        #[inline(always)]
        fn default() -> IocRx {
            <crate::RegValueT<IocRx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod iocrx {
        pub struct Alti_SPEC;
        pub type Alti = crate::EnumBitfieldStruct<u8, Alti_SPEC>;
        impl Alti {
            #[doc = "00 Alternate        Input 0 selected"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Alternate        Input 1 selected"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "11 Alternate        Input 3 selected"]
            pub const CONST_33: Self = Self::new(3);
            #[doc = "Alternate Input 2 selected"]
            pub const CONST_22: Self = Self::new(2);
        }
        pub struct Depth_SPEC;
        pub type Depth = crate::EnumBitfieldStruct<u8, Depth_SPEC>;
        impl Depth {
            #[doc = "0000 off  default"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "0001 1 T pdiv"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Oie_SPEC;
        pub type Oie = crate::EnumBitfieldStruct<u8, Oie_SPEC>;
        impl Oie {
            #[doc = "0 Pulse polarity        is active low"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Pulse polarity is active high"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Iie_SPEC;
        pub type Iie = crate::EnumBitfieldStruct<u8, Iie_SPEC>;
        impl Iie {
            #[doc = "0 Pulse polarity        is active low"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Pulse polarity is active high"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Reg_SPEC;
        pub type Reg = crate::EnumBitfieldStruct<u8, Reg_SPEC>;
        impl Reg {
            #[doc = "0 No Glitch        detected on rising edge"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Glitch detected on rising edge"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Feg_SPEC;
        pub type Feg = crate::EnumBitfieldStruct<u8, Feg_SPEC>;
        impl Feg {
            #[doc = "0 No Glitch        detected on falling edge"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Glitch detected on falling edge"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Creg_SPEC;
        pub type Creg = crate::EnumBitfieldStruct<u8, Creg_SPEC>;
        impl Creg {
            #[doc = "0 REG is not        cleared"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 REG is cleared"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cfeg_SPEC;
        pub type Cfeg = crate::EnumBitfieldStruct<u8, Cfeg_SPEC>;
        impl Cfeg {
            #[doc = "0 FEG is not        cleared"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 FEG is cleared"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Ctr_SPEC;
        pub type Ctr = crate::EnumBitfieldStruct<u8, Ctr_SPEC>;
        impl Ctr {
            #[doc = "0 TRM is not        cleared"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 TRM is cleared"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Trm_SPEC;
        pub type Trm = crate::EnumBitfieldStruct<u8, Trm_SPEC>;
        impl Trm {
            #[doc = "0 No Trigger        detected"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Trigger detected  one or several"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rxm_SPEC;
        pub type Rxm = crate::EnumBitfieldStruct<u8, Rxm_SPEC>;
        impl Rxm {
            #[doc = "0 Current signal        is low."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Current signal is high."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Txm_SPEC;
        pub type Txm = crate::EnumBitfieldStruct<u8, Txm_SPEC>;
        impl Txm {
            #[doc = "0 Current signal        is low."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Current signal is high."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RcRx_SPEC;
    impl crate::sealed::RegSpec for RcRx_SPEC {
        type DataType = u32;
    }
    #[doc = "Receiver Control Register 0\n resetvalue={Application Reset:0x0}"]
    pub type RcRx = crate::RegValueT<RcRx_SPEC>;

    impl RcRx {
        #[doc = "Channel Enable   CEN. When CEN is set  the receiver of channel x is enabled. The internal receiver state machine can be initialized by switching the channel off and on. This does not change the current register content."]
        #[inline(always)]
        pub fn cen(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, rcrx::Cen, RcRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1,1,0,rcrx::Cen, RcRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Ignore End Pulse   IEP. When IEP is set  an end pulse is ignored. An end pulse can be generated in SPC mode or as pause pulse. For systems with an end pulse  during synchronize or re synchronize of reception  if calibration pulses are detected one immediately following the other  the first calibration pulse shall be ignored as it may be a pause pulse with duration matching the calibration pulse range."]
        #[inline(always)]
        pub fn iep(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, rcrx::Iep, RcRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0x1,1,0,rcrx::Iep, RcRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Alternative CRC Mode Enable   ACE. When ACE is set  the CRC is calculated in an alternative way for both  fast  signal  and slow  serial message  data path. If ESF is set  the standard 6 bit CRC is always used for the serial message and ACE is ignored."]
        #[inline(always)]
        pub fn ace(
            self,
        ) -> crate::common::RegisterField<2, 0x1, 1, 0, rcrx::Ace, RcRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x1,1,0,rcrx::Ace, RcRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Status Nibble Included in CRC   SNI. When SNI is set  the status Nibble is included in  signal data  CRC."]
        #[inline(always)]
        pub fn sni(
            self,
        ) -> crate::common::RegisterField<3, 0x1, 1, 0, rcrx::Sni, RcRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<3,0x1,1,0,rcrx::Sni, RcRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Serial Data Processing Mode   SDP. This bit switches automatic serial data processing on."]
        #[inline(always)]
        pub fn sdp(
            self,
        ) -> crate::common::RegisterField<4, 0x1, 1, 0, rcrx::Sdp, RcRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x1,1,0,rcrx::Sdp, RcRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "CRC for Serial Data Disabled Mode   SCDIS. This bit selects the CRC disabled mode."]
        #[inline(always)]
        pub fn scdis(
            self,
        ) -> crate::common::RegisterField<5, 0x1, 1, 0, rcrx::Scdis, RcRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x1,1,0,rcrx::Scdis, RcRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "CRC Disabled Mode   CDIS. This bit selects the CRC disabled mode."]
        #[inline(always)]
        pub fn cdis(
            self,
        ) -> crate::common::RegisterField<6, 0x1, 1, 0, rcrx::Cdis, RcRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<6,0x1,1,0,rcrx::Cdis, RcRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Consecutive Frame Check   CFC. This bit determines the way the most recently received frame buffer is        indicated as valid. Note  If FDFL is set  CFC is ignored and the checks        described here are not executed."]
        #[inline(always)]
        pub fn cfc(
            self,
        ) -> crate::common::RegisterField<7, 0x1, 1, 0, rcrx::Cfc, RcRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<7,0x1,1,0,rcrx::Cfc, RcRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Frame Length   FRL. FRL determines the number of data nibbles per frame that the SENT channel x is setup for. Note that FRL does not include the        Synchronization   Calibration Pulse  the Status and Communication        nibble  the CRC nibble nor the additional zero length nibble that might        be introduced by use of SPC If more than 8 nibbles are configured  please note  In addition to the        receive success interrupt RSI at the successfully received end of a        frame  a receive data interrupt RDI is issued each time 8 nibbles have        been transferred to the Receive Data Register RDRx. At the end of a        frame  RDI is issued if RSI is issued. If an error occurred  RDI is not        set at the end of a frame. If no CRC has been received at the point in        time where RDI is issued  the receive data interrupt is no indication        whether or not the transfer was successful so far. A CRC Error Interrupt        is issued at the end of the frame if Automatic CRC check is enabled and        the CRC is wrong."]
        #[inline(always)]
        pub fn frl(
            self,
        ) -> crate::common::RegisterField<8, 0xff, 1, 0, rcrx::Frl, RcRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xff,1,0,rcrx::Frl, RcRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "CRC with Zero Nibble for Serial Data   CRZ. This bit selects the CRC method. If CRZ is cleared  augmentation is        selected   i.e a ZERO NIBBLE is added at the end of CRC calculation         only in calculation  . E.g. as 7th nibble  in case of 6 data nibbles  The enhanced serial message  18 Frames  6 bit CRC  is not controlled by        CRZ but the 6 bit CRC is always augmented according to standard."]
        #[inline(always)]
        pub fn crz(
            self,
        ) -> crate::common::RegisterField<16, 0x1, 1, 0, rcrx::Crz, RcRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1,1,0,rcrx::Crz, RcRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enhanced Serial Frame Mode   ESF. This bit selects the serial frame structure."]
        #[inline(always)]
        pub fn esf(
            self,
        ) -> crate::common::RegisterField<17, 0x1, 1, 0, rcrx::Esf, RcRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<17,0x1,1,0,rcrx::Esf, RcRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Ignore Drift Error Mode   IDE. This bit selects if drift errors lead to frame rejection and if an interrupt  INTSTAT.FDI  is generated.  Used  if sensors are triggered by SPC. During a long pause period the accumulated drift could be more than 1.5625 . In this special case setting IDE is useful."]
        #[inline(always)]
        pub fn ide(
            self,
        ) -> crate::common::RegisterField<18, 0x1, 1, 0, rcrx::Ide, RcRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<18,0x1,1,0,rcrx::Ide, RcRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Suspend Enable   SUSEN. This bit makes it possible to set the SENT channel into Suspend Mode via OCDS  on chip debug support   Bit SUSEN is reset via OCDS Reset."]
        #[inline(always)]
        pub fn susen(
            self,
        ) -> crate::common::RegisterField<19, 0x1, 1, 0, rcrx::Susen, RcRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<19,0x1,1,0,rcrx::Susen, RcRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Frequency Drift Check based on Frame Length   FDFL. This bit is used for frames with pause pulse only. If set the drift        error is not checked by HW. Instead  counter values DIVM and FRLEN are        provided to the SW for checking the frequency drift. IEP  Pause Pulse        expected  and IDE  no HW check of drift error  must always be set   1         together with FDFL. Note  If FDFL is set  RCR.CFC is ignored and the        checks described there are not executed."]
        #[inline(always)]
        pub fn fdfl(
            self,
        ) -> crate::common::RegisterField<20, 0x1, 1, 0, rcrx::Fdfl, RcRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0x1,1,0,rcrx::Fdfl, RcRx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for RcRx {
        #[inline(always)]
        fn default() -> RcRx {
            <crate::RegValueT<RcRx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod rcrx {
        pub struct Cen_SPEC;
        pub type Cen = crate::EnumBitfieldStruct<u8, Cen_SPEC>;
        impl Cen {
            #[doc = "0 channel x        disabled  default"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 channel x enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Iep_SPEC;
        pub type Iep = crate::EnumBitfieldStruct<u8, Iep_SPEC>;
        impl Iep {
            #[doc = "0 End Pulse not        ignored  default"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 End Pulse ignored"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Ace_SPEC;
        pub type Ace = crate::EnumBitfieldStruct<u8, Ace_SPEC>;
        impl Ace {
            #[doc = "0 Serial CRC        calculation as specified in J2716        JAN2010  default"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Alternative  4 bit in parallel CRC calculation as used e.g. in hall sensor TLE4998C."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Sni_SPEC;
        pub type Sni = crate::EnumBitfieldStruct<u8, Sni_SPEC>;
        impl Sni {
            #[doc = "0 Status Nibble        not included in CRC  default"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Status Nibble        included in CRC  as used e.g. in hall sensor TLE4998C ."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Sdp_SPEC;
        pub type Sdp = crate::EnumBitfieldStruct<u8, Sdp_SPEC>;
        impl Sdp {
            #[doc = "0 Automatic        Serial Data Processing is disabled. Status and Communication nibble can        be read from RSRx for SW processing.  default"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Automatic Serial Data Processing is enabled. Status and Communication nibble can be read from RSRx  Message ID  Serial Data and SCRC can be read from SDSx after serial data interrupt SDI is activated."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Scdis_SPEC;
        pub type Scdis = crate::EnumBitfieldStruct<u8, Scdis_SPEC>;
        impl Scdis {
            #[doc = "0 CRC is enabled         default"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 CRC is disabled  CRC nibble can be read from SDSx. The CPU must perform the CRC on the current data by SW."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cdis_SPEC;
        pub type Cdis = crate::EnumBitfieldStruct<u8, Cdis_SPEC>;
        impl Cdis {
            #[doc = "0 CRC is enabled         default"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 CRC is disabled  CRC nibble can be read from RSRx. The CPU must perform the CRC on the current data by SW."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cfc_SPEC;
        pub type Cfc = crate::EnumBitfieldStruct<u8, Cfc_SPEC>;
        impl Cfc {
            #[doc = "Check against Past Sync Pulse. The current Synchronization   Calibration Pulse is compared to the        Synchronization   Calibration Pulse received immediately before. The        whole frame is invalid if the Synchronization   Calibration Pulse length        differs from the length of the Synchronization   Calibration Pulse        before by more than 1.5625 . In this case of error  its length is not        used as new reference. In case the check passes and no other error        occurs the Frame Buffer is indicated valid  RSI  immediately after CRC        calculation result is correct. Resynchronization  On the third        successive calibration pulse error  the current calibration pulse value        is considered as valid and the message accepted unless the message frame        contains other errors. In both cases a receive data interrupt  RDI   or        a referring error interrupt is issued."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Check against Future Sync Pulse. The current Synchronization   Calibration Pulse is compared with the        Synchronization   Calibration Pulse received immediately after the        current frame. The whole frame is invalid if the Synchronization          Calibration Pulse length differs from the length of the following        Synchronization   Calibration Pulse by more than 1.5625 . In case the        check passes and no other error occurs the Frame Buffer is indicated        valid  RSI  immediately after the Synchronization   Calibration Pulse        following the current frame. Resynchronization  In this case of error         the current length is used as new reference.  Preferred        Option in J2716 JAN2010   Note  The whole frame can be indicated valid only after additionally        measuring the Synchronization   Calibration Pulse of the successive        frame."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Frl_SPEC;
        pub type Frl = crate::EnumBitfieldStruct<u8, Frl_SPEC>;
        impl Frl {
            #[doc = "00000000   160  No        data nibble"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "00000001   160   160 1 data        nibble"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "00000010   160   160 2 data        nibbles"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "00001000   160   160 8        nibbles   160   160   160   160 Maximum in normal length mode"]
            pub const CONST_88: Self = Self::new(8);
        }
        pub struct Crz_SPEC;
        pub type Crz = crate::EnumBitfieldStruct<u8, Crz_SPEC>;
        impl Crz {
            #[doc = "0 Augmentation is        selected for both 4 bit message CRC and the 4 bit CRC of the serial        messages  legacy  16 frames   default"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Augmentation is switched off for both 4 bit message CRC and the 4 bit CRC of the serial messages  legacy  16 frames ."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Esf_SPEC;
        pub type Esf = crate::EnumBitfieldStruct<u8, Esf_SPEC>;
        impl Esf {
            #[doc = "0 short  16        frames  4 bit ID  8 bit data  4 bit CRC"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 enhanced  18 frames  4 or 8 bit ID  12 or 16 bit data  6 bit CRC"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Ide_SPEC;
        pub type Ide = crate::EnumBitfieldStruct<u8, Ide_SPEC>;
        impl Ide {
            #[doc = "0 Drift Errors        enabled  default"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Drift Errors disabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Susen_SPEC;
        pub type Susen = crate::EnumBitfieldStruct<u8, Susen_SPEC>;
        impl Susen {
            #[doc = "0 An OCDS suspend        trigger is ignored by this SENT channel."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An OCDS suspend trigger disables the SENT channel  As soon as the SPC sender logic of the SENT channel becomes idle  the module is stopped while all registers of the channel stay readable. The receiver is stopped unconditionally. A partly received frame is discarded."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Fdfl_SPEC;
        pub type Fdfl = crate::EnumBitfieldStruct<u8, Fdfl_SPEC>;
        impl Fdfl {
            #[doc = "0 FDFL mode        deactivated  RCR.CFC is valid  if IDE is cleared ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 FDFL mode is activated  IEP and IDE must be set too for proper function  RCR.CFC is ignored. If no other error occurs the Frame Buffer is indicated valid  RSI  immediately after the Pause Pulse belonging to the current frame."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RsRx_SPEC;
    impl crate::sealed::RegSpec for RsRx_SPEC {
        type DataType = u32;
    }
    #[doc = "Receive Status Register 0\n resetvalue={Application Reset:0x0}"]
    pub type RsRx = crate::RegValueT<RsRx_SPEC>;

    impl RsRx {
        #[doc = "CRC   CRC. of last frame. CRC0 is on bit position 0."]
        #[inline(always)]
        pub fn crc(
            self,
        ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, RsRx_SPEC, crate::common::R> {
            crate::common::RegisterField::<0,0xf,1,0,u8, RsRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Channel Status   CST. CST shows the current status of channel x."]
        #[inline(always)]
        pub fn cst(
            self,
        ) -> crate::common::RegisterField<4, 0x3, 1, 0, rsrx::Cst, RsRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<4,0x3,1,0,rsrx::Cst, RsRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Status and Communication Nibble   SCN. of last frame. SCN0 is on bit position 8."]
        #[inline(always)]
        pub fn scn(
            self,
        ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, RsRx_SPEC, crate::common::R> {
            crate::common::RegisterField::<8,0xf,1,0,u8, RsRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Frame Length including Pause Nibble   FRLEN. of last frame. Bit FRLEN0 is on bit position 16. FRLEN is a 16 bit        counter with saturation  stops at 0xFFFF   65.535  and is driven by f pdiv . This counter supports monitoring the following condition for messages        with pause pulse and fixed message length  Ratio of calibration pulse to        message length varies by  gt  1 64 or  lt   1 64 from one message to another.        FRLEN is valid after RSI. See chapter   8220 Support for Frequency Drift        Analysis in Frames with Pause Pulse  8221 . FRLEN        must not be clocked if IEP or IDE or FDFL is cleared."]
        #[inline(always)]
        pub fn frlen(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, RsRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, RsRx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for RsRx {
        #[inline(always)]
        fn default() -> RsRx {
            <crate::RegValueT<RsRx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod rsrx {
        pub struct Cst_SPEC;
        pub type Cst = crate::EnumBitfieldStruct<u8, Cst_SPEC>;
        impl Cst {
            #[doc = "00 STOP Channel        is disabled and can be configured"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 INITIALIZED Channel is configured and enabled and no Synchronization   Calibration Pulse was received since last enable."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 RUNNING one or        more Synchronization   Calibration Pulses were received and Frequency        Range or Frequency Drift not or no longer in range. Fallback status from        SYNCHRONIZED."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 SYNCHRONIZED Frequency Range and Frequency Drift in range"]
            pub const CONST_33: Self = Self::new(3);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ScRx_SPEC;
    impl crate::sealed::RegSpec for ScRx_SPEC {
        type DataType = u32;
    }
    #[doc = "SPC Control Register 0\n resetvalue={Application Reset:0x0}"]
    pub type ScRx = crate::RegValueT<ScRx_SPEC>;

    impl ScRx {
        #[doc = "Pulse Length   PLEN. Defines the length of the pulse in tick times. The time base is the        measured tick time of the latest received frame if selected so by BASE.        In case this measured tick time was invalid or not already available        after enable of the channel  the nominal time base of the module is used."]
        #[inline(always)]
        pub fn plen(
            self,
        ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, ScRx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<0,0x3f,1,0,u8, ScRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Trigger Source and Mode Selection   TRIG. Selects the Trigger Source and Mode. The internal sender state machine        can be initialized by switching the channel off  TRIG is cleared  and        on. This does not change the current register content."]
        #[inline(always)]
        pub fn trig(
            self,
        ) -> crate::common::RegisterField<6, 0x3, 1, 0, scrx::Trig, ScRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<6,0x3,1,0,scrx::Trig, ScRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Delay Length   DEL. Selects how long the SPC pulse is delayed after the trigger condition.        The time base is the measured tick time of the latest received frame if        selected so by BASE. In case this measured tick time was invalid or not        already available after enable of the channel  the nominal time base of        the module is used."]
        #[inline(always)]
        pub fn del(
            self,
        ) -> crate::common::RegisterField<8, 0x3f, 1, 0, scrx::Del, ScRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x3f,1,0,scrx::Del, ScRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Time Base   BASE. Selects the Pulse Time Base"]
        #[inline(always)]
        pub fn base(
            self,
        ) -> crate::common::RegisterField<14, 0x1, 1, 0, scrx::Base, ScRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<14,0x1,1,0,scrx::Base, ScRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Transfer Request in Progress   TRQ. While an SPC Pulse is being sent this bit is set. Write access is ignored."]
        #[inline(always)]
        pub fn trq(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, ScRx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<15, 1, 0, ScRx_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
    }
    impl core::default::Default for ScRx {
        #[inline(always)]
        fn default() -> ScRx {
            <crate::RegValueT<ScRx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod scrx {
        pub struct Trig_SPEC;
        pub type Trig = crate::EnumBitfieldStruct<u8, Trig_SPEC>;
        impl Trig {
            #[doc = "00 No Pulse is        generated  OFF When cleared  an ongoing transfer is stopped immediately        and the transmit output is driven recessive."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Pulse starts immediately  no auto repetition"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Pulse starts each time the first falling edge of any Synchronization   Calibration Pulse is received  auto repetition on next Sync.   Cal. Pulses"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Pulse starts after each external trigger event.   auto repetition on next trigger  IOCRx.ETS selects the source of this event."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Del_SPEC;
        pub type Del = crate::EnumBitfieldStruct<u8, Del_SPEC>;
        impl Del {
            #[doc = "000000 Pulse is        not delayed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "000001 Pulse is        delayed by 1 tick"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Base_SPEC;
        pub type Base = crate::EnumBitfieldStruct<u8, Base_SPEC>;
        impl Base {
            #[doc = "0 Pulse is based        on measured frequency of last Synchronization Calibration Pulse"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Pulse is based on nominal frequency"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdSx_SPEC;
    impl crate::sealed::RegSpec for SdSx_SPEC {
        type DataType = u32;
    }
    #[doc = "Serial Data and Status Register 0\n resetvalue={Application Reset:0x0}"]
    pub type SdSx = crate::RegValueT<SdSx_SPEC>;

    impl SdSx {
        #[doc = "Serial Data   SD. of last serial data frame. SD0 is on bit position 0. Usually all 16 data bits are used. If RCR.ESF is cleared 8 bits of data are available and bits  15 8  are zero. If RCR.ESF is set and if SDS.CON is cleared 12 bits of data are available and bits  15 12  are zero."]
        #[inline(always)]
        pub fn sd(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, SdSx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, SdSx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Message ID   MID. of last serial data frame. ID0 is on bit position 16. If RCR.ESF is cleared  or if SDS.CON is set  bits  23 20  are zero."]
        #[inline(always)]
        pub fn mid(
            self,
        ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, SdSx_SPEC, crate::common::R> {
            crate::common::RegisterField::<16,0xff,1,0,u8, SdSx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "SCRC   SCRC. CRC of last serial data frame. CRC0 is on position 24. If RCR.ESF is cleared  bits  29 28  are always zero."]
        #[inline(always)]
        pub fn scrc(
            self,
        ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, SdSx_SPEC, crate::common::R> {
            crate::common::RegisterField::<24,0x3f,1,0,u8, SdSx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Configuration bit   CON. of last serial frame."]
        #[inline(always)]
        pub fn con(
            self,
        ) -> crate::common::RegisterField<31, 0x1, 1, 0, sdsx::Con, SdSx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<31,0x1,1,0,sdsx::Con, SdSx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for SdSx {
        #[inline(always)]
        fn default() -> SdSx {
            <crate::RegValueT<SdSx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod sdsx {
        pub struct Con_SPEC;
        pub type Con = crate::EnumBitfieldStruct<u8, Con_SPEC>;
        impl Con {
            #[doc = "0 12 bit data and        8 bit message ID"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 16 bit data and 4 bit message ID"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VieWx_SPEC;
    impl crate::sealed::RegSpec for VieWx_SPEC {
        type DataType = u32;
    }
    #[doc = "Receive Data View Register 0\n resetvalue={Application Reset:0x0}"]
    pub type VieWx = crate::RegValueT<VieWx_SPEC>;

    impl VieWx {
        #[doc = "Receive Data Target Nibble Pointer 7   RDNP7. RDNPy points to the Nibble in Receive Data Register RDRx where the        nibble y from the received frame is sorted to. Nibble 0 is the first        data nibble in the frame. It gets moved to the position defined in        RDNP0. And on. RDNPy must be written before first frame reception. All RDNPy must have        different values.  Higher RDNPy overwrite lower RDNPy."]
        #[inline(always)]
        pub fn rdnp0(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, VieWx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<0,0x7,1,0,u8, VieWx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Receive Data Target Nibble Pointer 7   RDNP7. RDNPy points to the Nibble in Receive Data Register RDRx where the        nibble y from the received frame is sorted to. Nibble 0 is the first        data nibble in the frame. It gets moved to the position defined in        RDNP0. And on. RDNPy must be written before first frame reception. All RDNPy must have        different values.  Higher RDNPy overwrite lower RDNPy."]
        #[inline(always)]
        pub fn rdnp1(
            self,
        ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, VieWx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<4,0x7,1,0,u8, VieWx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Receive Data Target Nibble Pointer 7   RDNP7. RDNPy points to the Nibble in Receive Data Register RDRx where the        nibble y from the received frame is sorted to. Nibble 0 is the first        data nibble in the frame. It gets moved to the position defined in        RDNP0. And on. RDNPy must be written before first frame reception. All RDNPy must have        different values.  Higher RDNPy overwrite lower RDNPy."]
        #[inline(always)]
        pub fn rdnp2(
            self,
        ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, VieWx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<8,0x7,1,0,u8, VieWx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Receive Data Target Nibble Pointer 7   RDNP7. RDNPy points to the Nibble in Receive Data Register RDRx where the        nibble y from the received frame is sorted to. Nibble 0 is the first        data nibble in the frame. It gets moved to the position defined in        RDNP0. And on. RDNPy must be written before first frame reception. All RDNPy must have        different values.  Higher RDNPy overwrite lower RDNPy."]
        #[inline(always)]
        pub fn rdnp3(
            self,
        ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, VieWx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0x7,1,0,u8, VieWx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Receive Data Target Nibble Pointer 7   RDNP7. RDNPy points to the Nibble in Receive Data Register RDRx where the        nibble y from the received frame is sorted to. Nibble 0 is the first        data nibble in the frame. It gets moved to the position defined in        RDNP0. And on. RDNPy must be written before first frame reception. All RDNPy must have        different values.  Higher RDNPy overwrite lower RDNPy."]
        #[inline(always)]
        pub fn rdnp4(
            self,
        ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, VieWx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x7,1,0,u8, VieWx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Receive Data Target Nibble Pointer 7   RDNP7. RDNPy points to the Nibble in Receive Data Register RDRx where the        nibble y from the received frame is sorted to. Nibble 0 is the first        data nibble in the frame. It gets moved to the position defined in        RDNP0. And on. RDNPy must be written before first frame reception. All RDNPy must have        different values.  Higher RDNPy overwrite lower RDNPy."]
        #[inline(always)]
        pub fn rdnp5(
            self,
        ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, VieWx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0x7,1,0,u8, VieWx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Receive Data Target Nibble Pointer 7   RDNP7. RDNPy points to the Nibble in Receive Data Register RDRx where the        nibble y from the received frame is sorted to. Nibble 0 is the first        data nibble in the frame. It gets moved to the position defined in        RDNP0. And on. RDNPy must be written before first frame reception. All RDNPy must have        different values.  Higher RDNPy overwrite lower RDNPy."]
        #[inline(always)]
        pub fn rdnp6(
            self,
        ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, VieWx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0x7,1,0,u8, VieWx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Receive Data Target Nibble Pointer 7   RDNP7. RDNPy points to the Nibble in Receive Data Register RDRx where the        nibble y from the received frame is sorted to. Nibble 0 is the first        data nibble in the frame. It gets moved to the position defined in        RDNP0. And on. RDNPy must be written before first frame reception. All RDNPy must have        different values.  Higher RDNPy overwrite lower RDNPy."]
        #[inline(always)]
        pub fn rdnp7(
            self,
        ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, VieWx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0x7,1,0,u8, VieWx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for VieWx {
        #[inline(always)]
        fn default() -> VieWx {
            <crate::RegValueT<VieWx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WdTx_SPEC;
    impl crate::sealed::RegSpec for WdTx_SPEC {
        type DataType = u32;
    }
    #[doc = "Watch Dog Timer Register 0\n resetvalue={Application Reset:0x0}"]
    pub type WdTx = crate::RegValueT<WdTx_SPEC>;

    impl WdTx {
        #[doc = "Watch Dog Timer Limit   WDL. for channel x ."]
        #[inline(always)]
        pub fn wdl(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, WdTx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, WdTx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for WdTx {
        #[inline(always)]
        fn default() -> WdTx {
            <crate::RegValueT<WdTx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
