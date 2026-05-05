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
#[doc = r"HSSL"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hssl0(pub(super) *mut u8);
unsafe impl core::marker::Send for Hssl0 {}
unsafe impl core::marker::Sync for Hssl0 {}
impl Hssl0 {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(252usize)) }
    }

    #[doc = "Access Rules Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ar(&self) -> crate::common::Reg<self::Ar_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(224usize)) }
    }

    #[doc = "Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cfg(&self) -> crate::common::Reg<self::Cfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "CRC Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn crc(&self) -> crate::common::Reg<self::Crc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0CBC000}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
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

    #[doc = "Miscellaneous Flags Register\n resetvalue={Application Reset:0x080000000}"]
    #[inline(always)]
    pub const fn mflags(&self) -> crate::common::Reg<self::Mflags_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }

    #[doc = "Miscellaneous Flags Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mflagscl(&self) -> crate::common::Reg<self::Mflagscl_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "Flags Enable Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mflagsen(&self) -> crate::common::Reg<self::Mflagsen_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "Miscellaneous Flags Set Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mflagsset(&self) -> crate::common::Reg<self::Mflagsset_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "Multi Slave Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mscr(&self) -> crate::common::Reg<self::Mscr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(156usize)) }
    }

    #[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(232usize)) }
    }

    #[doc = "Request Flags Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn qflags(&self) -> crate::common::Reg<self::Qflags_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Security Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sec(&self) -> crate::common::Reg<self::Sec_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(152usize)) }
    }

    #[doc = "Stream FIFOs Status Flags Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sfsflags(&self) -> crate::common::Reg<self::Sfsflags_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }

    #[doc = "Target ID Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tidadd(&self) -> crate::common::Reg<self::Tidadd_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(148usize)) }
    }

    #[doc = "Target Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tstat(&self) -> crate::common::Reg<self::Tstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }
    #[doc = "AW"]
    #[inline(always)]
    pub fn aw(self) -> [self::Aw; 4] {
        unsafe {
            [
                self::Aw(self.0.add(0xc0usize + 0x0usize)),
                self::Aw(self.0.add(0xc0usize + 0x8usize)),
                self::Aw(self.0.add(0xc0usize + 0x10usize)),
                self::Aw(self.0.add(0xc0usize + 0x18usize)),
            ]
        }
    }
    #[doc = "IS"]
    #[inline(always)]
    pub fn is(self) -> self::Is {
        unsafe { self::Is(self.0.add(160usize)) }
    }
    #[doc = "I"]
    #[inline(always)]
    pub fn i(self) -> [self::I; 4] {
        unsafe {
            [
                self::I(self.0.add(0x30usize + 0x0usize)),
                self::I(self.0.add(0x30usize + 0x10usize)),
                self::I(self.0.add(0x30usize + 0x20usize)),
                self::I(self.0.add(0x30usize + 0x30usize)),
            ]
        }
    }
    #[doc = "TS"]
    #[inline(always)]
    pub fn ts(self) -> self::Ts {
        unsafe { self::Ts(self.0.add(176usize)) }
    }
    #[doc = "T"]
    #[inline(always)]
    pub fn t(self) -> [self::T; 4] {
        unsafe {
            [
                self::T(self.0.add(0x70usize + 0x0usize)),
                self::T(self.0.add(0x70usize + 0x8usize)),
                self::T(self.0.add(0x70usize + 0x10usize)),
                self::T(self.0.add(0x70usize + 0x18usize)),
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
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, accen0::En0, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,accen0::En0, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, accen0::En1, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,accen0::En1, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, accen0::En2, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,accen0::En2, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, accen0::En3, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,accen0::En3, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, accen0::En4, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,accen0::En4, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, accen0::En5, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,accen0::En5, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, accen0::En6, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,accen0::En6, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, accen0::En7, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,accen0::En7, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, accen0::En8, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,accen0::En8, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, accen0::En9, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,accen0::En9, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, accen0::En10, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,accen0::En10, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, accen0::En11, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,accen0::En11, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, accen0::En12, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,accen0::En12, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, accen0::En13, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,accen0::En13, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, accen0::En14, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,accen0::En14, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, accen0::En15, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,accen0::En15, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, accen0::En16, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,accen0::En16, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, accen0::En17, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,accen0::En17, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, accen0::En18, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,accen0::En18, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, accen0::En19, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,accen0::En19, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, accen0::En20, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,accen0::En20, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, accen0::En21, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,accen0::En21, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, accen0::En22, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,accen0::En22, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, accen0::En23, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,accen0::En23, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, accen0::En24, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,accen0::En24, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, accen0::En25, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,accen0::En25, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, accen0::En26, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,accen0::En26, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, accen0::En27, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,accen0::En27, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, accen0::En28, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,accen0::En28, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, accen0::En29, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,accen0::En29, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, accen0::En30, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,accen0::En30, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
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
pub struct Ar_SPEC;
impl crate::sealed::RegSpec for Ar_SPEC {
    type DataType = u32;
}
#[doc = "Access Rules Register\n resetvalue={Application Reset:0x0}"]
pub type Ar = crate::RegValueT<Ar_SPEC>;

impl Ar {
    #[doc = "Access Rule for Window 3   ARW3"]
    #[inline(always)]
    pub fn arw0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, ar::Arw0, Ar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,ar::Arw0, Ar_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Rule for Window 3   ARW3"]
    #[inline(always)]
    pub fn arw1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, ar::Arw1, Ar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,ar::Arw1, Ar_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Rule for Window 3   ARW3"]
    #[inline(always)]
    pub fn arw2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, ar::Arw2, Ar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,ar::Arw2, Ar_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Rule for Window 3   ARW3"]
    #[inline(always)]
    pub fn arw3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, ar::Arw3, Ar_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,ar::Arw3, Ar_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Memory Access Violation Channel   MAVCH. This bit field shows the number of the latest channel that attempted a        not allowed access."]
    #[inline(always)]
    pub fn mavch(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, ar::Mavch, Ar_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3,1,0,ar::Mavch, Ar_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ar {
    #[inline(always)]
    fn default() -> Ar {
        <crate::RegValueT<Ar_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ar {
    pub struct Arw0_SPEC;
    pub type Arw0 = crate::EnumBitfieldStruct<u8, Arw0_SPEC>;
    impl Arw0 {
        #[doc = "00 No access  window disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Read access allowed"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Write access allowed"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Read and write access allowed"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Arw1_SPEC;
    pub type Arw1 = crate::EnumBitfieldStruct<u8, Arw1_SPEC>;
    impl Arw1 {
        #[doc = "00 No access  window disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Read access allowed"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Write access allowed"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Read and write access allowed"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Arw2_SPEC;
    pub type Arw2 = crate::EnumBitfieldStruct<u8, Arw2_SPEC>;
    impl Arw2 {
        #[doc = "00 No access  window disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Read access allowed"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Write access allowed"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Read and write access allowed"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Arw3_SPEC;
    pub type Arw3 = crate::EnumBitfieldStruct<u8, Arw3_SPEC>;
    impl Arw3 {
        #[doc = "00 No access  window disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Read access allowed"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Write access allowed"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Read and write access allowed"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Mavch_SPEC;
    pub type Mavch = crate::EnumBitfieldStruct<u8, Mavch_SPEC>;
    impl Mavch {
        #[doc = "00 Channel 0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Channel 1"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Channel 2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Channel 3"]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg_SPEC;
impl crate::sealed::RegSpec for Cfg_SPEC {
    type DataType = u32;
}
#[doc = "Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Cfg = crate::RegValueT<Cfg_SPEC>;

impl Cfg {
    #[doc = "Global Predivider   PREDIV. Defines the down scaled module clock to be used by all channel timeout        timers."]
    #[inline(always)]
    pub fn prediv(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, cfg::Prediv, Cfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3fff,1,0,cfg::Prediv, Cfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Streaming Mode Transmitter   SMT"]
    #[inline(always)]
    pub fn smt(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, cfg::Smt, Cfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1,1,0,cfg::Smt, Cfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Streaming Mode Receiver   SMR"]
    #[inline(always)]
    pub fn smr(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, cfg::Smr, Cfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<17,0x1,1,0,cfg::Smr, Cfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Streaming Channel Mode   SCM. Defines if the channel 2 is used in a streaming or command mode."]
    #[inline(always)]
    pub fn scm(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, cfg::Scm, Cfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x1,1,0,cfg::Scm, Cfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel Code Control   CCC. Defines the coding of the channel number in the HSSL header."]
    #[inline(always)]
    pub fn ccc(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, cfg::Ccc, Cfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<19,0x1,1,0,cfg::Ccc, Cfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Cfg {
    #[inline(always)]
    fn default() -> Cfg {
        <crate::RegValueT<Cfg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cfg {
    pub struct Prediv_SPEC;
    pub type Prediv = crate::EnumBitfieldStruct<u8, Prediv_SPEC>;
    impl Prediv {
        #[doc = "0 1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 2"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 3"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Smt_SPEC;
    pub type Smt = crate::EnumBitfieldStruct<u8, Smt_SPEC>;
    impl Smt {
        #[doc = "0 Continuous"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Single"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Smr_SPEC;
    pub type Smr = crate::EnumBitfieldStruct<u8, Smr_SPEC>;
    impl Smr {
        #[doc = "0 Continuous"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Single"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scm_SPEC;
    pub type Scm = crate::EnumBitfieldStruct<u8, Scm_SPEC>;
    impl Scm {
        #[doc = "0 Command"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Streaming"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ccc_SPEC;
    pub type Ccc = crate::EnumBitfieldStruct<u8, Ccc_SPEC>;
    impl Ccc {
        #[doc = "0 Binary"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Special"]
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
pub struct Crc_SPEC;
impl crate::sealed::RegSpec for Crc_SPEC {
    type DataType = u32;
}
#[doc = "CRC Control Register\n resetvalue={Application Reset:0x0}"]
pub type Crc = crate::RegValueT<Crc_SPEC>;

impl Crc {
    #[doc = "Value to be XORed with the Calculated CRC   XORMASK. Used for error injection  160    160 test purposes."]
    #[inline(always)]
    pub fn xormask(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Crc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Crc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable the Error Injection via XORMASK   XEN"]
    #[inline(always)]
    pub fn xen(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, crc::Xen, Crc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1,1,0,crc::Xen, Crc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Crc {
    #[inline(always)]
    fn default() -> Crc {
        <crate::RegValueT<Crc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod crc {
    pub struct Xen_SPEC;
    pub type Xen = crate::EnumBitfieldStruct<u8, Xen_SPEC>;
    impl Xen {
        #[doc = "0 disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0CBC000}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MODREV. MODREV defines the module revision number. The value of a module        revision starts with 01 H  first        revision ."]
    #[inline(always)]
    pub fn modrev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MODTYPE. This bit field is C0 H . It defines a        32 bit module."]
    #[inline(always)]
    pub fn modtype(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MODNUMBER. This bit field together with MODTYPE uniquely identifies a module."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(13352960)
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel        reset will be executed if the reset bits of both kernel registers are        set. The RST bit will be cleared  re set to   180 0  180   by the BPI FPI after the        kernel reset was executed."]
    #[inline(always)]
    pub fn rst(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, krst0::Rst, Krst0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,krst0::Rst, Krst0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Kernel Reset Status   RSTSTAT. This bit indicates wether a kernel reset was executed or not. This bit        is set by the BPI FPI after the execution of a kernel reset in the same        clock cycle both reset bits. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related KRSTCLR register."]
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel        reset will be executed if the reset bits of both kernel reset registers        is set. The RST bit will be cleared  re set to   180 0  180   by the BPI FPI after the        kernel reset was executed."]
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
pub struct Mflags_SPEC;
impl crate::sealed::RegSpec for Mflags_SPEC {
    type DataType = u32;
}
#[doc = "Miscellaneous Flags Register\n resetvalue={Application Reset:0x080000000}"]
pub type Mflags = crate::RegValueT<Mflags_SPEC>;

impl Mflags {
    #[doc = "Not Acknowledge Error   Target Error   NACK. Indicates for each channel that a target error frame has been received."]
    #[inline(always)]
    pub fn nack(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Mflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Mflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transaction Tag Error   TTE. Indicates for each channel if a CRC correct acknowledge frame with an        unexpected transaction tag number has been received."]
    #[inline(always)]
    pub fn tte(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Mflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Mflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Timeout Error   TIMEOUT. Indicates for each channel if an timeout event has occurred."]
    #[inline(always)]
    pub fn timeout(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Mflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Mflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Unexpected Type of Frame Error   UNEXPECTED. Indicates for each channel if an unexpected or inappropriate response is        received. For example a NACK for a Trigger frame or DATA for WRITE frame."]
    #[inline(always)]
    pub fn unexpected(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Mflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Mflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Target Memory Block   TMB. Selects the currently active memory block used by the target as a target        for the streaming data  with its start address and frame counter.        Switching the active block in the middle of a block transfer is not        allowed."]
    #[inline(always)]
    pub fn tmb(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, mflags::Tmb, Mflags_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<18,0x1,1,0,mflags::Tmb, Mflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Initiator Memory Block   IMB. Selects the currently active memory block used by the initiator as a        source for the streaming data  with its start address and frame counter.        Switching the active block in the middle of a block transfer is not        allowed."]
    #[inline(always)]
    pub fn imb(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, mflags::Imb, Mflags_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<19,0x1,1,0,mflags::Imb, Mflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Initiator Stream Block Request   ISB. Indicates if stream block request is pending. Set by the software to        start a stream block transfer by using the MFLAGSSET.ISBS bit  clear by        the software possible  if needed  by using the MFLAGSCL.ISBC bit         cleared by hardware at the end of the current block transfer in single        mode  but not in continuous mode."]
    #[inline(always)]
    pub fn isb(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, mflags::Isb, Mflags_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<20,0x1,1,0,mflags::Isb, Mflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Memory Access Violation   MAV. Indicates a memory access violation."]
    #[inline(always)]
    pub fn mav(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, mflags::Mav, Mflags_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<21,0x1,1,0,mflags::Mav, Mflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SRI SPB Bus Access Error   SRIE. Indicates an error on the SRI bus   transaction ID  ECC error or error        acknowledge. Indicates an error on the SPB bus  error acknowledge or timeout."]
    #[inline(always)]
    pub fn srie(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, mflags::Srie, Mflags_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<22,0x1,1,0,mflags::Srie, Mflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PHY Inconsistency Error 1 Channel Number Code Error    PIE1. Indicates if HSCT to HSSL channel number code comparator has detected an        inconsistency error."]
    #[inline(always)]
    pub fn pie1(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, mflags::Pie1, Mflags_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<23,0x1,1,0,mflags::Pie1, Mflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PHY Inconsistency Error 2 Data Length Error    PIE2. Indicates if HSCT to HSSL data length comparator has detected an        inconsistency error."]
    #[inline(always)]
    pub fn pie2(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, mflags::Pie2, Mflags_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0x1,1,0,mflags::Pie2, Mflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CRC Error   CRCE. Indicates if CRC checker has detected a CRC error."]
    #[inline(always)]
    pub fn crce(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, mflags::Crce, Mflags_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<25,0x1,1,0,mflags::Crce, Mflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Target Stream Enable   TSE. Used by the hardware to handle the single and continuous streaming. In        single mode  cleared by hardware after the current block transfer ends.        The module ignores afterwards the incoming steam frames."]
    #[inline(always)]
    pub fn tse(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, mflags::Tse, Mflags_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<28,0x1,1,0,mflags::Tse, Mflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit Enable Input   TEI. Indicates the state of the TEI input signal of the HSSL module  which is        driven by the CTS output signal of the HSCT module. Any edge on this        signal triggers an EXI interrupt. This low level signal stops the transmission of both command and        response frames."]
    #[inline(always)]
    pub fn tei(self) -> crate::common::RegisterFieldBool<29, 1, 0, Mflags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Mflags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmit Enable Output   TEO. Indicates the state of the TEO output signal of the HSSL module  which        drives thee CTS input signal of the HSCT module. This bit is cleared by        hardware at entering the INIT and Soft Suspend state."]
    #[inline(always)]
    pub fn teo(self) -> crate::common::RegisterFieldBool<30, 1, 0, Mflags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Mflags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Initialize Mode   INI. Indicates if the module is in the Initialize or Run mode."]
    #[inline(always)]
    pub fn ini(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, mflags::Ini, Mflags_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<31,0x1,1,0,mflags::Ini, Mflags_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Mflags {
    #[inline(always)]
    fn default() -> Mflags {
        <crate::RegValueT<Mflags_SPEC> as RegisterValue<_>>::new(2147483648)
    }
}
pub mod mflags {
    pub struct Tmb_SPEC;
    pub type Tmb = crate::EnumBitfieldStruct<u8, Tmb_SPEC>;
    impl Tmb {
        #[doc = "0 Memory block 0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Memory block 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Imb_SPEC;
    pub type Imb = crate::EnumBitfieldStruct<u8, Imb_SPEC>;
    impl Imb {
        #[doc = "0 Memory block 0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Memory block 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Isb_SPEC;
    pub type Isb = crate::EnumBitfieldStruct<u8, Isb_SPEC>;
    impl Isb {
        #[doc = "0 No request or streaming ongoing"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Streaming ongoing"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mav_SPEC;
    pub type Mav = crate::EnumBitfieldStruct<u8, Mav_SPEC>;
    impl Mav {
        #[doc = "0 No violation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Violation"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Srie_SPEC;
    pub type Srie = crate::EnumBitfieldStruct<u8, Srie_SPEC>;
    impl Srie {
        #[doc = "0 No error"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pie1_SPEC;
    pub type Pie1 = crate::EnumBitfieldStruct<u8, Pie1_SPEC>;
    impl Pie1 {
        #[doc = "0 No error"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pie2_SPEC;
    pub type Pie2 = crate::EnumBitfieldStruct<u8, Pie2_SPEC>;
    impl Pie2 {
        #[doc = "0 No error"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Crce_SPEC;
    pub type Crce = crate::EnumBitfieldStruct<u8, Crce_SPEC>;
    impl Crce {
        #[doc = "0 No error"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tse_SPEC;
    pub type Tse = crate::EnumBitfieldStruct<u8, Tse_SPEC>;
    impl Tse {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ini_SPEC;
    pub type Ini = crate::EnumBitfieldStruct<u8, Ini_SPEC>;
    impl Ini {
        #[doc = "0 Run mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Initialize mode"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mflagscl_SPEC;
impl crate::sealed::RegSpec for Mflagscl_SPEC {
    type DataType = u32;
}
#[doc = "Miscellaneous Flags Clear Register\n resetvalue={Application Reset:0x0}"]
pub type Mflagscl = crate::RegValueT<Mflagscl_SPEC>;

impl Mflagscl {
    #[doc = "NACK Flags Clear   NACKC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn nackc(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, mflagscl::Nackc, Mflagscl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xf,1,0,mflagscl::Nackc, Mflagscl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Transaction Tag Error Flags Clear   TTEC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn ttec(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, mflagscl::Ttec, Mflagscl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<4,0xf,1,0,mflagscl::Ttec, Mflagscl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Timeout Error Flags Clear   TIMEOUTC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn timeoutc(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        mflagscl::Timeoutc,
        Mflagscl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            mflagscl::Timeoutc,
            Mflagscl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Unexpected Error Flags Clear   UNEXPECTEDC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn unexpectedc(
        self,
    ) -> crate::common::RegisterField<
        12,
        0xf,
        1,
        0,
        mflagscl::Unexpectedc,
        Mflagscl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0xf,
            1,
            0,
            mflagscl::Unexpectedc,
            Mflagscl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Target Memory Block Flag Clear   TMBC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn tmbc(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, mflagscl::Tmbc, Mflagscl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<18,0x1,1,0,mflagscl::Tmbc, Mflagscl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Initiator Memory Block Flag Clear   IMBC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn imbc(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, mflagscl::Imbc, Mflagscl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<19,0x1,1,0,mflagscl::Imbc, Mflagscl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Initiator Stream Block Request Clear   ISBC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn isbc(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, mflagscl::Isbc, Mflagscl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<20,0x1,1,0,mflagscl::Isbc, Mflagscl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "MAV Flag Clear   MAVC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect"]
    #[inline(always)]
    pub fn mavc(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, mflagscl::Mavc, Mflagscl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<21,0x1,1,0,mflagscl::Mavc, Mflagscl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "SRI SPB Bus Access Error Flag Clear   SRIEC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect"]
    #[inline(always)]
    pub fn sriec(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, mflagscl::Sriec, Mflagscl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            mflagscl::Sriec,
            Mflagscl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "PIE1 Error Flag Clear   PIE1C. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect"]
    #[inline(always)]
    pub fn pie1c(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, mflagscl::Pie1C, Mflagscl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            mflagscl::Pie1C,
            Mflagscl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "PIE2 Error Flag Clear   PIE2C. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn pie2c(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, mflagscl::Pie2C, Mflagscl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            mflagscl::Pie2C,
            Mflagscl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "CRC Error Flag Clear   CRCEC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn crcec(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, mflagscl::Crcec, Mflagscl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            mflagscl::Crcec,
            Mflagscl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Target Stream Enable Flag Clear   TSEC. Writing 1 sets the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn tsec(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, mflagscl::Tsec, Mflagscl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<28,0x1,1,0,mflagscl::Tsec, Mflagscl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Transmit Enable Flag Clear   TEOC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn teoc(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, mflagscl::Teoc, Mflagscl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<30,0x1,1,0,mflagscl::Teoc, Mflagscl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Initialize Mode Flag Clear   INIC. Writing 1 clears the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn inic(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, mflagscl::Inic, Mflagscl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<31,0x1,1,0,mflagscl::Inic, Mflagscl_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Mflagscl {
    #[inline(always)]
    fn default() -> Mflagscl {
        <crate::RegValueT<Mflagscl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mflagscl {
    pub struct Nackc_SPEC;
    pub type Nackc = crate::EnumBitfieldStruct<u8, Nackc_SPEC>;
    impl Nackc {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ttec_SPEC;
    pub type Ttec = crate::EnumBitfieldStruct<u8, Ttec_SPEC>;
    impl Ttec {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Timeoutc_SPEC;
    pub type Timeoutc = crate::EnumBitfieldStruct<u8, Timeoutc_SPEC>;
    impl Timeoutc {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Unexpectedc_SPEC;
    pub type Unexpectedc = crate::EnumBitfieldStruct<u8, Unexpectedc_SPEC>;
    impl Unexpectedc {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tmbc_SPEC;
    pub type Tmbc = crate::EnumBitfieldStruct<u8, Tmbc_SPEC>;
    impl Tmbc {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Imbc_SPEC;
    pub type Imbc = crate::EnumBitfieldStruct<u8, Imbc_SPEC>;
    impl Imbc {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Isbc_SPEC;
    pub type Isbc = crate::EnumBitfieldStruct<u8, Isbc_SPEC>;
    impl Isbc {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mavc_SPEC;
    pub type Mavc = crate::EnumBitfieldStruct<u8, Mavc_SPEC>;
    impl Mavc {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sriec_SPEC;
    pub type Sriec = crate::EnumBitfieldStruct<u8, Sriec_SPEC>;
    impl Sriec {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pie1C_SPEC;
    pub type Pie1C = crate::EnumBitfieldStruct<u8, Pie1C_SPEC>;
    impl Pie1C {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pie2C_SPEC;
    pub type Pie2C = crate::EnumBitfieldStruct<u8, Pie2C_SPEC>;
    impl Pie2C {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Crcec_SPEC;
    pub type Crcec = crate::EnumBitfieldStruct<u8, Crcec_SPEC>;
    impl Crcec {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tsec_SPEC;
    pub type Tsec = crate::EnumBitfieldStruct<u8, Tsec_SPEC>;
    impl Tsec {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Teoc_SPEC;
    pub type Teoc = crate::EnumBitfieldStruct<u8, Teoc_SPEC>;
    impl Teoc {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Inic_SPEC;
    pub type Inic = crate::EnumBitfieldStruct<u8, Inic_SPEC>;
    impl Inic {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mflagsen_SPEC;
impl crate::sealed::RegSpec for Mflagsen_SPEC {
    type DataType = u32;
}
#[doc = "Flags Enable Register\n resetvalue={Application Reset:0x0}"]
pub type Mflagsen = crate::RegValueT<Mflagsen_SPEC>;

impl Mflagsen {
    #[doc = "Not Acknowledge Error Enable Bits   NACKEN. Used to enable the error interrupt associated to the corresponding bit        in the MFLAGS register."]
    #[inline(always)]
    pub fn nacken(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        mflagsen::Nacken,
        Mflagsen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            mflagsen::Nacken,
            Mflagsen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Transaction Tag Error Enable Bits   TTEEN. Used to enable the error interrupt associated to the corresponding bit        in the MFLAGS register."]
    #[inline(always)]
    pub fn tteen(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, mflagsen::Tteen, Mflagsen_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            mflagsen::Tteen,
            Mflagsen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Timeout Error Enable Bits   TIMEOUTEN. Used to enable the error interrupt associated to the corresponding bit        in the MFLAGS register."]
    #[inline(always)]
    pub fn timeouten(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        mflagsen::Timeouten,
        Mflagsen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            mflagsen::Timeouten,
            Mflagsen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Unexpected Error Enable Bits   UNEXPECTEDEN. Used to enable the error interrupt associated to the corresponding bit        in the MFLAGS register."]
    #[inline(always)]
    pub fn unexpecteden(
        self,
    ) -> crate::common::RegisterField<
        12,
        0xf,
        1,
        0,
        mflagsen::Unexpecteden,
        Mflagsen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0xf,
            1,
            0,
            mflagsen::Unexpecteden,
            Mflagsen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "MAV Enable Bit   MAVEN. Used to enable the interrupt associated to the corresponding bit in the        MFLAGS register."]
    #[inline(always)]
    pub fn maven(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        mflagsen::Maven,
        Mflagsen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            mflagsen::Maven,
            Mflagsen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "SRI SPB Bus Access Error Enable Bit   SRIEEN. Used to enable the error interrupt associated to the corresponding bit        in the MFLAGS register."]
    #[inline(always)]
    pub fn srieen(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        mflagsen::Srieen,
        Mflagsen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            mflagsen::Srieen,
            Mflagsen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "PIE1 Error Enable Bit   PIE1EN. Used to enable the error interrupt associated to the corresponding bit        in the MFLAGS register."]
    #[inline(always)]
    pub fn pie1en(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        mflagsen::Pie1En,
        Mflagsen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            mflagsen::Pie1En,
            Mflagsen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "PIE2 Error Enable Bit   PIE2EN. Used to enable the error interrupt associated to the corresponding bit        in the MFLAGS register."]
    #[inline(always)]
    pub fn pie2en(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        mflagsen::Pie2En,
        Mflagsen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            mflagsen::Pie2En,
            Mflagsen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CRC Error Enable Bit   CRCEEN. Used to enable the error interrupt associated to the corresponding bit        in the MFLAGS register."]
    #[inline(always)]
    pub fn crceen(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        mflagsen::Crceen,
        Mflagsen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            mflagsen::Crceen,
            Mflagsen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "TEI Enable Bit   TEIEN. Used to enable the interrupt associated to the corresponding bit in the        MFLAGS register."]
    #[inline(always)]
    pub fn teien(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        mflagsen::Teien,
        Mflagsen_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            mflagsen::Teien,
            Mflagsen_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Mflagsen {
    #[inline(always)]
    fn default() -> Mflagsen {
        <crate::RegValueT<Mflagsen_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mflagsen {
    pub struct Nacken_SPEC;
    pub type Nacken = crate::EnumBitfieldStruct<u8, Nacken_SPEC>;
    impl Nacken {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tteen_SPEC;
    pub type Tteen = crate::EnumBitfieldStruct<u8, Tteen_SPEC>;
    impl Tteen {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Timeouten_SPEC;
    pub type Timeouten = crate::EnumBitfieldStruct<u8, Timeouten_SPEC>;
    impl Timeouten {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Unexpecteden_SPEC;
    pub type Unexpecteden = crate::EnumBitfieldStruct<u8, Unexpecteden_SPEC>;
    impl Unexpecteden {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Maven_SPEC;
    pub type Maven = crate::EnumBitfieldStruct<u8, Maven_SPEC>;
    impl Maven {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Srieen_SPEC;
    pub type Srieen = crate::EnumBitfieldStruct<u8, Srieen_SPEC>;
    impl Srieen {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pie1En_SPEC;
    pub type Pie1En = crate::EnumBitfieldStruct<u8, Pie1En_SPEC>;
    impl Pie1En {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pie2En_SPEC;
    pub type Pie2En = crate::EnumBitfieldStruct<u8, Pie2En_SPEC>;
    impl Pie2En {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Crceen_SPEC;
    pub type Crceen = crate::EnumBitfieldStruct<u8, Crceen_SPEC>;
    impl Crceen {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Teien_SPEC;
    pub type Teien = crate::EnumBitfieldStruct<u8, Teien_SPEC>;
    impl Teien {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mflagsset_SPEC;
impl crate::sealed::RegSpec for Mflagsset_SPEC {
    type DataType = u32;
}
#[doc = "Miscellaneous Flags Set Register\n resetvalue={Application Reset:0x0}"]
pub type Mflagsset = crate::RegValueT<Mflagsset_SPEC>;

impl Mflagsset {
    #[doc = "NACK Flags Set   NACKS. Writing 1 sets the corresponding bit in the MFLAGS register and triggers        the ERR interrupt for the corresponding channel  if enabled in the        MFLAGSEN register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn nacks(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        mflagsset::Nacks,
        Mflagsset_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            mflagsset::Nacks,
            Mflagsset_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Transaction Tag Error Flags Set   TTES. Writing 1 sets the corresponding bit in the MFLAGS register and triggers        the ERR interrupt for the corresponding channel  if enabled in the        MFLAGSEN register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ttes(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, mflagsset::Ttes, Mflagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            mflagsset::Ttes,
            Mflagsset_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Timeout Error Flags Set   TIMEOUTS. Writing 1 sets the corresponding bit in the MFLAGS register and triggers        the ERR interrupt for the corresponding channel  if enabled in the        MFLAGSEN register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn timeouts(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        mflagsset::Timeouts,
        Mflagsset_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            mflagsset::Timeouts,
            Mflagsset_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Unexpected Error Flags Set   UNEXPECTEDS. Writing 1 sets the corresponding bit in the MFLAGS register and triggers        the ERR interrupt for the corresponding channel  if enabled in the        MFLAGSEN register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn unexpecteds(
        self,
    ) -> crate::common::RegisterField<
        12,
        0xf,
        1,
        0,
        mflagsset::Unexpecteds,
        Mflagsset_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0xf,
            1,
            0,
            mflagsset::Unexpecteds,
            Mflagsset_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Target Memory Block Flag Set   TMBS. Writing 1 sets the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn tmbs(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        mflagsset::Tmbs,
        Mflagsset_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            mflagsset::Tmbs,
            Mflagsset_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Initiator Memory Block Flag Set   IMBS. Writing 1 sets the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn imbs(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        mflagsset::Imbs,
        Mflagsset_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            mflagsset::Imbs,
            Mflagsset_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Initiator Stream Block Request Set   ISBS. Writing 1 sets the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn isbs(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        mflagsset::Isbs,
        Mflagsset_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            mflagsset::Isbs,
            Mflagsset_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "MAV Flag Set   MAVS. Writing 1 sets the corresponding bit in the MFLAGS register  and        generates an EXI interrupt if enabled in the corresponding bit of the        MFLAGSEN register. Writing 0 has no effect"]
    #[inline(always)]
    pub fn mavs(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        mflagsset::Mavs,
        Mflagsset_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            mflagsset::Mavs,
            Mflagsset_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "SRI SPB Bus Access Error Flag Set   SRIES. Writing 1 sets the corresponding bit in the MFLAGS register  and        generates an EXI interrupt if enabled in the corresponding bit of the        MFLAGSEN register. Writing 0 has no effect"]
    #[inline(always)]
    pub fn sries(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        mflagsset::Sries,
        Mflagsset_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            mflagsset::Sries,
            Mflagsset_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "PIE1 Error Flag Set   PIE1S. Writing 1 sets the corresponding bit in the MFLAGS register  and        generates an EXI interrupt if enabled in the corresponding bit of the        MFLAGSEN register. Writing 0 has no effect"]
    #[inline(always)]
    pub fn pie1s(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        mflagsset::Pie1S,
        Mflagsset_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            mflagsset::Pie1S,
            Mflagsset_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "PIE2 Error Flag Set   PIE2S. Writing 1 sets the corresponding bit in the MFLAGS register  and        generates an EXI interrupt if enabled in the corresponding bit of the        MFLAGSEN register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn pie2s(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        mflagsset::Pie2S,
        Mflagsset_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            mflagsset::Pie2S,
            Mflagsset_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "CRC Error Flag Set   CRCES. Writing 1 sets the corresponding bit in the MFLAGS register  and        generates an EXI interrupt if enabled in the corresponding bit of the        MFLAGSEN register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn crces(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        mflagsset::Crces,
        Mflagsset_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            mflagsset::Crces,
            Mflagsset_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Target Stream Enable Flag Set   TSES. Writing 1 sets the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn tses(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        mflagsset::Tses,
        Mflagsset_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            mflagsset::Tses,
            Mflagsset_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Transmit Enable Flag Set   TEOS. Writing 1 sets the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn teos(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        mflagsset::Teos,
        Mflagsset_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            mflagsset::Teos,
            Mflagsset_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Initialize Mode Flag Set   INIS. Writing 1 sets the corresponding bit in the MFLAGS register. Writing 0        has no effect."]
    #[inline(always)]
    pub fn inis(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        mflagsset::Inis,
        Mflagsset_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            mflagsset::Inis,
            Mflagsset_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Mflagsset {
    #[inline(always)]
    fn default() -> Mflagsset {
        <crate::RegValueT<Mflagsset_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mflagsset {
    pub struct Nacks_SPEC;
    pub type Nacks = crate::EnumBitfieldStruct<u8, Nacks_SPEC>;
    impl Nacks {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ttes_SPEC;
    pub type Ttes = crate::EnumBitfieldStruct<u8, Ttes_SPEC>;
    impl Ttes {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Timeouts_SPEC;
    pub type Timeouts = crate::EnumBitfieldStruct<u8, Timeouts_SPEC>;
    impl Timeouts {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Unexpecteds_SPEC;
    pub type Unexpecteds = crate::EnumBitfieldStruct<u8, Unexpecteds_SPEC>;
    impl Unexpecteds {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tmbs_SPEC;
    pub type Tmbs = crate::EnumBitfieldStruct<u8, Tmbs_SPEC>;
    impl Tmbs {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Imbs_SPEC;
    pub type Imbs = crate::EnumBitfieldStruct<u8, Imbs_SPEC>;
    impl Imbs {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Isbs_SPEC;
    pub type Isbs = crate::EnumBitfieldStruct<u8, Isbs_SPEC>;
    impl Isbs {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mavs_SPEC;
    pub type Mavs = crate::EnumBitfieldStruct<u8, Mavs_SPEC>;
    impl Mavs {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sries_SPEC;
    pub type Sries = crate::EnumBitfieldStruct<u8, Sries_SPEC>;
    impl Sries {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pie1S_SPEC;
    pub type Pie1S = crate::EnumBitfieldStruct<u8, Pie1S_SPEC>;
    impl Pie1S {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pie2S_SPEC;
    pub type Pie2S = crate::EnumBitfieldStruct<u8, Pie2S_SPEC>;
    impl Pie2S {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Crces_SPEC;
    pub type Crces = crate::EnumBitfieldStruct<u8, Crces_SPEC>;
    impl Crces {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tses_SPEC;
    pub type Tses = crate::EnumBitfieldStruct<u8, Tses_SPEC>;
    impl Tses {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Teos_SPEC;
    pub type Teos = crate::EnumBitfieldStruct<u8, Teos_SPEC>;
    impl Teos {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Inis_SPEC;
    pub type Inis = crate::EnumBitfieldStruct<u8, Inis_SPEC>;
    impl Inis {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mscr_SPEC;
impl crate::sealed::RegSpec for Mscr_SPEC {
    type DataType = u32;
}
#[doc = "Multi Slave Control Register\n resetvalue={Application Reset:0x0}"]
pub type Mscr = crate::RegValueT<Mscr_SPEC>;

impl Mscr {
    #[doc = "Multi Slave Mode Enable   EN. This bit enables the multi slave mode of operation of the HSSL module."]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mscr::En, Mscr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,mscr::En, Mscr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slave Tag   SLAVETAG. The master uses this bit field to define the current slave it sends to        and receives from. The slave uses this bit field to pass through only        the relevant incoming frames and to tag its own transmit frames. This        tag is injected in the header by the Slave Tag Translator block and used        for filtering in Slave Tag Filter Block."]
    #[inline(always)]
    pub fn slavetag(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, mscr::Slavetag, Mscr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,mscr::Slavetag, Mscr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Initiator Transmission Stop   ITXSTOP. This bit is intended to be set or cleared by the master in a multi slave        operation by using 16 bit write command frames. MSCR.ITXSTOP        functionality can be activated only if MSCR.EN  160    160 1  otherwise it is        ignored. Setting this bit stops the arbitration and transmission of new command        and stream frames of the initiator but does not stop the ongoing frames."]
    #[inline(always)]
    pub fn itxstop(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, mscr::Itxstop, Mscr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,mscr::Itxstop, Mscr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Mscr {
    #[inline(always)]
    fn default() -> Mscr {
        <crate::RegValueT<Mscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mscr {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Slavetag_SPEC;
    pub type Slavetag = crate::EnumBitfieldStruct<u8, Slavetag_SPEC>;
    impl Slavetag {
        #[doc = "01 Slave 1"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Slave 2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Slave 3"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Itxstop_SPEC;
    pub type Itxstop = crate::EnumBitfieldStruct<u8, Itxstop_SPEC>;
    impl Itxstop {
        #[doc = "0 Initiator transmission active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Initiator transmission stopped"]
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
    #[doc = "Trigger Set for OTGB0 1   TGS"]
    #[inline(always)]
    pub fn tgs(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, ocs::Tgs, Ocs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,ocs::Tgs, Ocs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OTGB0 1 Bus Select   TGB"]
    #[inline(always)]
    pub fn tgb(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ocs::Tgb, Ocs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,ocs::Tgb, Ocs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TGS  TGB Write Protection   TG P. TGS and TGB are only written when TG P is 1  otherwise unchanged. Read        as 0."]
    #[inline(always)]
    pub fn tg_p(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ocs_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ocs_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
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
    pub struct Tgs_SPEC;
    pub type Tgs = crate::EnumBitfieldStruct<u8, Tgs_SPEC>;
    impl Tgs {
        #[doc = "0 No Trigger Set output"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 TS16 STR  Streaming Channel Trigger Set"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 TS16 ERR  Errors Trigger Set"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Tgb_SPEC;
    pub type Tgb = crate::EnumBitfieldStruct<u8, Tgb_SPEC>;
    impl Tgb {
        #[doc = "0 Trigger Set is output on OTGB0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Trigger Set is output on OTGB1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sus_SPEC;
    pub type Sus = crate::EnumBitfieldStruct<u8, Sus_SPEC>;
    impl Sus {
        #[doc = "0 Will not suspend"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Hard suspend. Clock is switched off immediately."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Soft suspend"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Sussta_SPEC;
    pub type Sussta = crate::EnumBitfieldStruct<u8, Sussta_SPEC>;
    impl Sussta {
        #[doc = "0 Module is not  yet  suspended"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module is suspended"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qflags_SPEC;
impl crate::sealed::RegSpec for Qflags_SPEC {
    type DataType = u32;
}
#[doc = "Request Flags Register\n resetvalue={Application Reset:0x0}"]
pub type Qflags = crate::RegValueT<Qflags_SPEC>;

impl Qflags {
    #[doc = "Request Flags for Initiated Commands   I. These flags are set by the corresponding channel when a WRTS command is        initiated. The WRT commands are initiated via the SPB bus. The S tream         commands are initiated by the module internally  by the TXFIFO  except        for the first stream frame start  which is done via the SPB bus. See        MFLAGS.ISB and ISF flags."]
    #[inline(always)]
    pub fn i(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, qflags::I, Qflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,qflags::I, Qflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Flags for Commands Arrived at Target   T. These flags are set by the hardware according to the header information        of a frame arrived at the target without a CRC error. They are used by        the arbiter of the SRI  SPB        master and cleared when the appropriate command is executed."]
    #[inline(always)]
    pub fn t(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, qflags::T, Qflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0xf,1,0,qflags::T, Qflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Flags for Response Frames at the Target   R. After a command has been executed by the SRI  SPB        master  an appropriate flag is being set which indicates that an        ACK NACK DATA is pending."]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, qflags::R, Qflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xf,1,0,qflags::R, Qflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Expect Flags for Activated Timeout Timer 0   E0. An appropriate two bit flag is set by the hardware when a timeout timer        for a channel is started. The hardware clears the appropriate flag when        any response frame arrives at the initiator. In case of an unexpected response the flag UNEXPECTED is additionally        set."]
    #[inline(always)]
    pub fn e0(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, qflags::E0, Qflags_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x3,1,0,qflags::E0, Qflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Expect Flags for Activated Timeout Timer 1   E1"]
    #[inline(always)]
    pub fn e1(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, Qflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<18,0x3,1,0,u8, Qflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Expect Flags for Activated Timeout Timer 2   E2"]
    #[inline(always)]
    pub fn e2(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Qflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Qflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Expect Flags for Activated Timeout Timer 3   E3"]
    #[inline(always)]
    pub fn e3(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, Qflags_SPEC, crate::common::R> {
        crate::common::RegisterField::<22,0x3,1,0,u8, Qflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "I Flag for Stream Frames   IS. See the   8220 I  8221  flag description above."]
    #[inline(always)]
    pub fn is(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, qflags::Is, Qflags_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<28,0x1,1,0,qflags::Is, Qflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "R Flag for Stream Frames   RS. See the   8220 R  8221  flag description above."]
    #[inline(always)]
    pub fn rs(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, qflags::Rs, Qflags_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<29,0x1,1,0,qflags::Rs, Qflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "T Flag for Stream Frames   TS. See the   8220 T  8221  flag description above."]
    #[inline(always)]
    pub fn ts(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, qflags::Ts, Qflags_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<30,0x1,1,0,qflags::Ts, Qflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "E Flag for Stream Frames   ES. See the   8220 E  8221  flag description above."]
    #[inline(always)]
    pub fn es(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, qflags::Es, Qflags_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<31,0x1,1,0,qflags::Es, Qflags_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Qflags {
    #[inline(always)]
    fn default() -> Qflags {
        <crate::RegValueT<Qflags_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod qflags {
    pub struct I_SPEC;
    pub type I = crate::EnumBitfieldStruct<u8, I_SPEC>;
    impl I {
        #[doc = "0000 No request pending"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "0001 Channel 0 request pending"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "0010 Channel 1 request pending"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "0100 Channel 2 request pending"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "1000 Channel 3 request pending"]
        pub const CONST_88: Self = Self::new(8);
    }
    pub struct T_SPEC;
    pub type T = crate::EnumBitfieldStruct<u8, T_SPEC>;
    impl T {
        #[doc = "0000 No request pending"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "0001 Channel 0 request pending"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "0010 Channel 1 request pending"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "0100 Channel 2 request pending"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "1000 Channel 3 request pending"]
        pub const CONST_88: Self = Self::new(8);
    }
    pub struct R_SPEC;
    pub type R = crate::EnumBitfieldStruct<u8, R_SPEC>;
    impl R {
        #[doc = "0000 No request pending"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "0001 Channel 0 request pending"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "0010 Channel 1 request pending"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "0100 Channel 2 request pending"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "1000 Channel 3 request pending"]
        pub const CONST_88: Self = Self::new(8);
    }
    pub struct E0_SPEC;
    pub type E0 = crate::EnumBitfieldStruct<u8, E0_SPEC>;
    impl E0 {
        #[doc = "00 No request pending"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 write request pending"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 read request pending"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 trigger request pending"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Is_SPEC;
    pub type Is = crate::EnumBitfieldStruct<u8, Is_SPEC>;
    impl Is {
        #[doc = "0 No request pending"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rs_SPEC;
    pub type Rs = crate::EnumBitfieldStruct<u8, Rs_SPEC>;
    impl Rs {
        #[doc = "0 No request pending"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ts_SPEC;
    pub type Ts = crate::EnumBitfieldStruct<u8, Ts_SPEC>;
    impl Ts {
        #[doc = "0 No request pending"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Es_SPEC;
    pub type Es = crate::EnumBitfieldStruct<u8, Es_SPEC>;
    impl Es {
        #[doc = "0 No request pending"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sec_SPEC;
impl crate::sealed::RegSpec for Sec_SPEC {
    type DataType = u32;
}
#[doc = "Security Control Register\n resetvalue={Application Reset:0x0}"]
pub type Sec = crate::RegValueT<Sec_SPEC>;

impl Sec {
    #[doc = "Lock the HSSL Module   LCK. Setting this bit field prevents the MFLAGS.INI bit to be cleared  that        is prevents to leave the INIT state and go to RUN mode. If INIT bit has        already been cleared  it is possible to set it  but not to clear it        afterwards. This bit can only be written by an access from the HSM master  TAG          000011B . A write operation performed by any other master is ignored and        the bit remains unchanged."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, sec::Lck, Sec_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,sec::Lck, Sec_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock the Address Windows Registers   LAW. This bit field shows if the Address Window Registers AWSTARTx  AWENDx         AR and TIDADD are locked or not. If locked  the properties of the        address windows cannot be changed any more. This bit can only be written by an access from the HSM master  TAG          000011B . A write operation performed by any other master is ignored and        the bit remains unchanged."]
    #[inline(always)]
    pub fn law(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, sec::Law, Sec_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,sec::Law, Sec_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Sec {
    #[inline(always)]
    fn default() -> Sec {
        <crate::RegValueT<Sec_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sec {
    pub struct Lck_SPEC;
    pub type Lck = crate::EnumBitfieldStruct<u8, Lck_SPEC>;
    impl Lck {
        #[doc = "0 Unlocked"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Locked"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Law_SPEC;
    pub type Law = crate::EnumBitfieldStruct<u8, Law_SPEC>;
    impl Law {
        #[doc = "0 Unlocked"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Locked"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfsflags_SPEC;
impl crate::sealed::RegSpec for Sfsflags_SPEC {
    type DataType = u32;
}
#[doc = "Stream FIFOs Status Flags Register\n resetvalue={Application Reset:0x0}"]
pub type Sfsflags = crate::RegValueT<Sfsflags_SPEC>;

impl Sfsflags {
    #[doc = "Stream RxFIFO Filling Level   RXFL. Indicates the filling level of the FIFO with granularity of 32 bytes         one stream frame payload size ."]
    #[inline(always)]
    pub fn rxfl(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, sfsflags::Rxfl, Sfsflags_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3,1,0,sfsflags::Rxfl, Sfsflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Stream TxFIFO Filling Level   TXFL. Indicates the filling level of the FIFO with granularity of 32 bytes         one stream frame payload size ."]
    #[inline(always)]
    pub fn txfl(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, sfsflags::Txfl, Sfsflags_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x3,1,0,sfsflags::Txfl, Sfsflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Stream Expect FIFO Filling Level   EXFL. Indicates the filling level of the FIFO with granularity of 32 bytes         one stream frame payload size ."]
    #[inline(always)]
    pub fn exfl(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, sfsflags::Exfl, Sfsflags_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x3,1,0,sfsflags::Exfl, Sfsflags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Initiator Stream Frame Request   ISF. Indicates if stream TXFIFO request is pending. Set and cleared by the        TXFIFO."]
    #[inline(always)]
    pub fn isf(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, sfsflags::Isf, Sfsflags_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<15,0x1,1,0,sfsflags::Isf, Sfsflags_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Sfsflags {
    #[inline(always)]
    fn default() -> Sfsflags {
        <crate::RegValueT<Sfsflags_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sfsflags {
    pub struct Rxfl_SPEC;
    pub type Rxfl = crate::EnumBitfieldStruct<u8, Rxfl_SPEC>;
    impl Rxfl {
        #[doc = "00 0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 1"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 2"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Txfl_SPEC;
    pub type Txfl = crate::EnumBitfieldStruct<u8, Txfl_SPEC>;
    impl Txfl {
        #[doc = "00 0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 1"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 2"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Exfl_SPEC;
    pub type Exfl = crate::EnumBitfieldStruct<u8, Exfl_SPEC>;
    impl Exfl {
        #[doc = "00 0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 1"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 2"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Isf_SPEC;
    pub type Isf = crate::EnumBitfieldStruct<u8, Isf_SPEC>;
    impl Isf {
        #[doc = "0 TXFIFO fill request not pending"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 TXFIFO fill request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tidadd_SPEC;
impl crate::sealed::RegSpec for Tidadd_SPEC {
    type DataType = u32;
}
#[doc = "Target ID Address Register\n resetvalue={Application Reset:0x0}"]
pub type Tidadd = crate::RegValueT<Tidadd_SPEC>;

impl Tidadd {
    #[doc = "Address Pointer   A. Address pointer containing the address of the memory location containing        the unique ID data."]
    #[inline(always)]
    pub fn a(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Tidadd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Tidadd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Tidadd {
    #[inline(always)]
    fn default() -> Tidadd {
        <crate::RegValueT<Tidadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tstat_SPEC;
impl crate::sealed::RegSpec for Tstat_SPEC {
    type DataType = u32;
}
#[doc = "Target Status Register\n resetvalue={Application Reset:0x0}"]
pub type Tstat = crate::RegValueT<Tstat_SPEC>;

impl Tstat {
    #[doc = "Last Command Code   LASTCC3. Indicates the latest command code from the header for the corresponding        channel."]
    #[inline(always)]
    pub fn lastcc0(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, Tstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, Tstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Last Transaction Tag   LASTTT3. Indicates the transaction tag of the latest command or stream frame for        the corresponding channel."]
    #[inline(always)]
    pub fn lasttt0(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, Tstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x7,1,0,u8, Tstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Last Command Code   LASTCC3. Indicates the latest command code from the header for the corresponding        channel."]
    #[inline(always)]
    pub fn lastcc1(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, Tstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, Tstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Last Transaction Tag   LASTTT3. Indicates the transaction tag of the latest command or stream frame for        the corresponding channel."]
    #[inline(always)]
    pub fn lasttt1(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, Tstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<13,0x7,1,0,u8, Tstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Last Command Code   LASTCC3. Indicates the latest command code from the header for the corresponding        channel."]
    #[inline(always)]
    pub fn lastcc2(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Tstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Tstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Last Transaction Tag   LASTTT3. Indicates the transaction tag of the latest command or stream frame for        the corresponding channel."]
    #[inline(always)]
    pub fn lasttt2(
        self,
    ) -> crate::common::RegisterField<21, 0x7, 1, 0, u8, Tstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<21,0x7,1,0,u8, Tstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Last Command Code   LASTCC3. Indicates the latest command code from the header for the corresponding        channel."]
    #[inline(always)]
    pub fn lastcc3(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, Tstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, Tstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Last Transaction Tag   LASTTT3. Indicates the transaction tag of the latest command or stream frame for        the corresponding channel."]
    #[inline(always)]
    pub fn lasttt3(
        self,
    ) -> crate::common::RegisterField<29, 0x7, 1, 0, u8, Tstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<29,0x7,1,0,u8, Tstat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Tstat {
    #[inline(always)]
    fn default() -> Tstat {
        <crate::RegValueT<Tstat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc = "AW"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aw(pub(super) *mut u8);
unsafe impl core::marker::Send for Aw {}
unsafe impl core::marker::Sync for Aw {}
impl Aw {
    #[doc = "Access Window End Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn awendi(&self) -> crate::common::Reg<aw::AwenDi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Access Window Start Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn awstarti(&self) -> crate::common::Reg<aw::AwstarTi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod aw {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AwenDi_SPEC;
    impl crate::sealed::RegSpec for AwenDi_SPEC {
        type DataType = u32;
    }
    #[doc = "Access Window End Register 0\n resetvalue={Application Reset:0x0}"]
    pub type AwenDi = crate::RegValueT<AwenDi_SPEC>;

    impl AwenDi {
        #[doc = "Access Window End Address   AWE. This bit field defines the upper 24 bits of the end address of the        corresponding access window. This results in a granularity of 256 bytes        for the end address."]
        #[inline(always)]
        pub fn awe(
            self,
        ) -> crate::common::RegisterField<8, 0xffffff, 1, 0, u32, AwenDi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xffffff,1,0,u32, AwenDi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for AwenDi {
        #[inline(always)]
        fn default() -> AwenDi {
            <crate::RegValueT<AwenDi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AwstarTi_SPEC;
    impl crate::sealed::RegSpec for AwstarTi_SPEC {
        type DataType = u32;
    }
    #[doc = "Access Window Start Register 0\n resetvalue={Application Reset:0x0}"]
    pub type AwstarTi = crate::RegValueT<AwstarTi_SPEC>;

    impl AwstarTi {
        #[doc = "Access Window Start Address   AWS. This bit field defines the upper 24 bits of the start address of the        corresponding access window. This results in a granularity of 256 bytes        for the start address."]
        #[inline(always)]
        pub fn aws(
            self,
        ) -> crate::common::RegisterField<8, 0xffffff, 1, 0, u32, AwstarTi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xffffff,1,0,u32, AwstarTi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for AwstarTi {
        #[inline(always)]
        fn default() -> AwstarTi {
            <crate::RegValueT<AwstarTi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "IS"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Is(pub(super) *mut u8);
unsafe impl core::marker::Send for Is {}
unsafe impl core::marker::Sync for Is {}
impl Is {
    #[doc = "Initiator Stream Current Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn isca(&self) -> crate::common::Reg<is::Isca_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Initiator Stream Frame Count Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn isfc(&self) -> crate::common::Reg<is::Isfc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Initiator Stream Start Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn issax(&self) -> [crate::common::Reg<is::IssAx_SPEC, crate::common::RW>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
            ]
        }
    }
}
pub mod is {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isca_SPEC;
    impl crate::sealed::RegSpec for Isca_SPEC {
        type DataType = u32;
    }
    #[doc = "Initiator Stream Current Address Register\n resetvalue={Application Reset:0x0}"]
    pub type Isca = crate::RegValueT<Isca_SPEC>;

    impl Isca {
        #[doc = "Address of the Memory Location for the Current Transfer   CURR. Aligned on 256 bit limit  stream frame payload size ."]
        #[inline(always)]
        pub fn curr(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, Isca_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<5,0x7ffffff,1,0,u32, Isca_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for Isca {
        #[inline(always)]
        fn default() -> Isca {
            <crate::RegValueT<Isca_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isfc_SPEC;
    impl crate::sealed::RegSpec for Isfc_SPEC {
        type DataType = u32;
    }
    #[doc = "Initiator Stream Frame Count Register\n resetvalue={Application Reset:0x0}"]
    pub type Isfc = crate::RegValueT<Isfc_SPEC>;

    impl Isfc {
        #[doc = "Reload Count Number   RELCOUNT. Contains the number of frames to transfer per memory block. Bit field        length depends on application requirements."]
        #[inline(always)]
        pub fn relcount(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffff,
            1,
            0,
            isfc::Relcount,
            Isfc_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffff,
                1,
                0,
                isfc::Relcount,
                Isfc_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Current Count Number   CURCOUNT. Displays the current count number  which is generated by down counting from the RELCOUNT value. Bit field length depends on application requirements."]
        #[inline(always)]
        pub fn curcount(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Isfc_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, Isfc_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for Isfc {
        #[inline(always)]
        fn default() -> Isfc {
            <crate::RegValueT<Isfc_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod isfc {
        pub struct Relcount_SPEC;
        pub type Relcount = crate::EnumBitfieldStruct<u8, Relcount_SPEC>;
        impl Relcount {
            #[doc = "0 1"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 1"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "2 2"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "3 3"]
            pub const CONST_33: Self = Self::new(3);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IssAx_SPEC;
    impl crate::sealed::RegSpec for IssAx_SPEC {
        type DataType = u32;
    }
    #[doc = "Initiator Stream Start Address Register\n resetvalue={Application Reset:0x0}"]
    pub type IssAx = crate::RegValueT<IssAx_SPEC>;

    impl IssAx {
        #[doc = "Start Address for the Memory Range   START. Aligned on 256 bit limit  stream frame payload size ."]
        #[inline(always)]
        pub fn start(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, IssAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x7ffffff,1,0,u32, IssAx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for IssAx {
        #[inline(always)]
        fn default() -> IssAx {
            <crate::RegValueT<IssAx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "I"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I(pub(super) *mut u8);
unsafe impl core::marker::Send for I {}
unsafe impl core::marker::Sync for I {}
impl I {
    #[doc = "Initiator Control Data Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn iconx(&self) -> crate::common::Reg<i::IcoNx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Initiator Read Data Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn irdx(&self) -> crate::common::Reg<i::IrDx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Initiator Read Write Address Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn irwax(&self) -> crate::common::Reg<i::IrwAx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Initiator Write Data Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn iwdx(&self) -> crate::common::Reg<i::IwDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod i {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IcoNx_SPEC;
    impl crate::sealed::RegSpec for IcoNx_SPEC {
        type DataType = u32;
    }
    #[doc = "Initiator Control Data Register 0\n resetvalue={Application Reset:0x0}"]
    pub type IcoNx = crate::RegValueT<IcoNx_SPEC>;

    impl IcoNx {
        #[doc = "Read ID Request   IDQ. This bit provides the only way to request a read ID frame. Reads always 0. Write of 1 commences a request. In case of parallel write of 1 to TQ and IDQ  the IDQ request has higher priority."]
        #[inline(always)]
        pub fn idq(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, iconx::Idq, IcoNx_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<0,0x1,1,0,iconx::Idq, IcoNx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Trigger Request   TQ. This bit provides an alternative way to request a trigger frame  without having to write to the channel address register. Reads always 0. Write of 1 commences a request. In case of parallel write of 1 to TQ and IDQ  the IDQ request has higher priority."]
        #[inline(always)]
        pub fn tq(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, iconx::Tq, IcoNx_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<1,0x1,1,0,iconx::Tq, IcoNx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Last Error Transaction Tag   LETT"]
        #[inline(always)]
        pub fn lett(
            self,
        ) -> crate::common::RegisterField<2, 0x7, 1, 0, u8, IcoNx_SPEC, crate::common::R> {
            crate::common::RegisterField::<2,0x7,1,0,u8, IcoNx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Currently Expected Transaction Tag   CETT"]
        #[inline(always)]
        pub fn cett(
            self,
        ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, IcoNx_SPEC, crate::common::R> {
            crate::common::RegisterField::<5,0x7,1,0,u8, IcoNx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Time Out Current Value   TOCV"]
        #[inline(always)]
        pub fn tocv(
            self,
        ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, IcoNx_SPEC, crate::common::R> {
            crate::common::RegisterField::<8,0xff,1,0,u8, IcoNx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Data Length   DATLEN. Defines the length of the data in bits of the write and read command."]
        #[inline(always)]
        pub fn datlen(
            self,
        ) -> crate::common::RegisterField<16, 0x3, 1, 0, iconx::Datlen, IcoNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                16,
                0x3,
                1,
                0,
                iconx::Datlen,
                IcoNx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Read Write Trigger Command Type   RWT. Defines if the write to the IRWA register will trigger read  write or trigger request."]
        #[inline(always)]
        pub fn rwt(
            self,
        ) -> crate::common::RegisterField<18, 0x3, 1, 0, iconx::Rwt, IcoNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<18,0x3,1,0,iconx::Rwt, IcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Channel Busy   BSY"]
        #[inline(always)]
        pub fn bsy(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, IcoNx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<20,1,0,IcoNx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Initiator Transaction Tag   ITTAG. This bit displays the current value of the three bit counter generating the new transaction tag value."]
        #[inline(always)]
        pub fn ittag(
            self,
        ) -> crate::common::RegisterField<21, 0x7, 1, 0, u8, IcoNx_SPEC, crate::common::R> {
            crate::common::RegisterField::<21,0x7,1,0,u8, IcoNx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Time Out Reload Value   TOREL. Defines the duration of the timeout in units of prescaled clock periods. This parameter is valid both in command and stream mode of the channel 2."]
        #[inline(always)]
        pub fn torel(
            self,
        ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, IcoNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0xff,1,0,u8, IcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for IcoNx {
        #[inline(always)]
        fn default() -> IcoNx {
            <crate::RegValueT<IcoNx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod iconx {
        pub struct Idq_SPEC;
        pub type Idq = crate::EnumBitfieldStruct<u8, Idq_SPEC>;
        impl Idq {
            #[doc = "0 No read ID request"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read ID frame request"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tq_SPEC;
        pub type Tq = crate::EnumBitfieldStruct<u8, Tq_SPEC>;
        impl Tq {
            #[doc = "0 No trigger request"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Trigger frame request"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Datlen_SPEC;
        pub type Datlen = crate::EnumBitfieldStruct<u8, Datlen_SPEC>;
        impl Datlen {
            #[doc = "00 8 bit"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 16 bit"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 32 bit"]
            pub const CONST_22: Self = Self::new(2);
        }
        pub struct Rwt_SPEC;
        pub type Rwt = crate::EnumBitfieldStruct<u8, Rwt_SPEC>;
        impl Rwt {
            #[doc = "00 No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Read Frame"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Write Frame"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Trigger Frame"]
            pub const CONST_33: Self = Self::new(3);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrDx_SPEC;
    impl crate::sealed::RegSpec for IrDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Initiator Read Data Register 0\n resetvalue={Application Reset:0x0}"]
    pub type IrDx = crate::RegValueT<IrDx_SPEC>;

    impl IrDx {
        #[doc = "Data Delivered by a Read Response Frame   DATA"]
        #[inline(always)]
        pub fn data(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, IrDx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, IrDx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for IrDx {
        #[inline(always)]
        fn default() -> IrDx {
            <crate::RegValueT<IrDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IrwAx_SPEC;
    impl crate::sealed::RegSpec for IrwAx_SPEC {
        type DataType = u32;
    }
    #[doc = "Initiator Read Write Address Register 0\n resetvalue={Application Reset:0x0}"]
    pub type IrwAx = crate::RegValueT<IrwAx_SPEC>;

    impl IrwAx {
        #[doc = "Address Part of the Payload of a Write Frame   ADDRESS. Writing to this registers triggers transmission of a Write Frame. The        address must be aligned according to the data width  byte addresses for        8 bit data  half word addresses for 16 bit data  word addresses for        32 bit data."]
        #[inline(always)]
        pub fn address(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, IrwAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, IrwAx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for IrwAx {
        #[inline(always)]
        fn default() -> IrwAx {
            <crate::RegValueT<IrwAx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IwDx_SPEC;
    impl crate::sealed::RegSpec for IwDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Initiator Write Data Register 0\n resetvalue={Application Reset:0x0}"]
    pub type IwDx = crate::RegValueT<IwDx_SPEC>;

    impl IwDx {
        #[doc = "Data Part of the Payload of a Write Frame   DATA. For 8 bit and16 bit write command frames  the whole frame payload width of 32 bit is automatically filled with copies of the lower 8 bits or 16 bits of the register."]
        #[inline(always)]
        pub fn data(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, IwDx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, IwDx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for IwDx {
        #[inline(always)]
        fn default() -> IwDx {
            <crate::RegValueT<IwDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "TS"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ts(pub(super) *mut u8);
unsafe impl core::marker::Send for Ts {}
unsafe impl core::marker::Sync for Ts {}
impl Ts {
    #[doc = "Target Stream Current Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tsca(&self) -> crate::common::Reg<ts::Tsca_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Target Stream Frame Count Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tsfc(&self) -> crate::common::Reg<ts::Tsfc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Target Stream Start Address Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tssax(&self) -> [crate::common::Reg<ts::TssAx_SPEC, crate::common::RW>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
            ]
        }
    }
}
pub mod ts {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tsca_SPEC;
    impl crate::sealed::RegSpec for Tsca_SPEC {
        type DataType = u32;
    }
    #[doc = "Target Stream Current Address Register\n resetvalue={Application Reset:0x0}"]
    pub type Tsca = crate::RegValueT<Tsca_SPEC>;

    impl Tsca {
        #[doc = "Address of the Memory Location for the Current Transfer   CURR. Aligned on 256 bit  or 32 byte  limit  stream frame payload size ."]
        #[inline(always)]
        pub fn curr(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, Tsca_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<5,0x7ffffff,1,0,u32, Tsca_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for Tsca {
        #[inline(always)]
        fn default() -> Tsca {
            <crate::RegValueT<Tsca_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tsfc_SPEC;
    impl crate::sealed::RegSpec for Tsfc_SPEC {
        type DataType = u32;
    }
    #[doc = "Target Stream Frame Count Register\n resetvalue={Application Reset:0x0}"]
    pub type Tsfc = crate::RegValueT<Tsfc_SPEC>;

    impl Tsfc {
        #[doc = "Reload Count Number   RELCOUNT. Contains the number of frames to transfer per memory block. Bit field        length depends on application requirements."]
        #[inline(always)]
        pub fn relcount(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffff,
            1,
            0,
            tsfc::Relcount,
            Tsfc_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffff,
                1,
                0,
                tsfc::Relcount,
                Tsfc_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Current Count Number   CURCOUNT. Displays the current count number  which is generated by down counting        from the RELCOUNT value. Bit field length depends on application        requirements."]
        #[inline(always)]
        pub fn curcount(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Tsfc_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, Tsfc_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for Tsfc {
        #[inline(always)]
        fn default() -> Tsfc {
            <crate::RegValueT<Tsfc_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod tsfc {
        pub struct Relcount_SPEC;
        pub type Relcount = crate::EnumBitfieldStruct<u8, Relcount_SPEC>;
        impl Relcount {
            #[doc = "0 1"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 1"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "2 2"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "3 3"]
            pub const CONST_33: Self = Self::new(3);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TssAx_SPEC;
    impl crate::sealed::RegSpec for TssAx_SPEC {
        type DataType = u32;
    }
    #[doc = "Target Stream Start Address Register 0\n resetvalue={Application Reset:0x0}"]
    pub type TssAx = crate::RegValueT<TssAx_SPEC>;

    impl TssAx {
        #[doc = "Start Address for the Memory Range   ADDR. Aligned on 256 bit  or 32 byte  limit  stream frame payload size ."]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, TssAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x7ffffff,1,0,u32, TssAx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for TssAx {
        #[inline(always)]
        fn default() -> TssAx {
            <crate::RegValueT<TssAx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "T"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T(pub(super) *mut u8);
unsafe impl core::marker::Send for T {}
unsafe impl core::marker::Sync for T {}
impl T {
    #[doc = "Target Current Address Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tcai(&self) -> crate::common::Reg<t::TcAi_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Target Current Data Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tcdi(&self) -> crate::common::Reg<t::TcDi_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod t {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcAi_SPEC;
    impl crate::sealed::RegSpec for TcAi_SPEC {
        type DataType = u32;
    }
    #[doc = "Target Current Address Register 0\n resetvalue={Application Reset:0x0}"]
    pub type TcAi = crate::RegValueT<TcAi_SPEC>;

    impl TcAi {
        #[doc = "Address Part of the Payload of a Write Command Frame or a Read Command Frame or ID Frame   A"]
        #[inline(always)]
        pub fn a(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, TcAi_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, TcAi_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for TcAi {
        #[inline(always)]
        fn default() -> TcAi {
            <crate::RegValueT<TcAi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcDi_SPEC;
    impl crate::sealed::RegSpec for TcDi_SPEC {
        type DataType = u32;
    }
    #[doc = "Target Current Data Register 0\n resetvalue={Application Reset:0x0}"]
    pub type TcDi = crate::RegValueT<TcDi_SPEC>;

    impl TcDi {
        #[doc = "Data Part of the Payload of a Write Command Frame or Read Data of a Read Command Frame   D"]
        #[inline(always)]
        pub fn d(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, TcDi_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, TcDi_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for TcDi {
        #[inline(always)]
        fn default() -> TcDi {
            <crate::RegValueT<TcDi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
