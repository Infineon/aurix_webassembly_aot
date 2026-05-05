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
#[doc = r"IOM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iom(pub(super) *mut u8);
unsafe impl core::marker::Send for Iom {}
unsafe impl core::marker::Sync for Iom {}
impl Iom {
    #[doc = "IOM Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }

    #[doc = "IOM Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "IOM Event Combiner Module Counter Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ecmccfg(&self) -> crate::common::Reg<self::Ecmccfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }

    #[doc = "IOM Event Combiner Module Event Trigger History Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ecmeth0(&self) -> crate::common::Reg<self::Ecmeth0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }

    #[doc = "IOM Event Combiner Module Event Trigger History Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ecmeth1(&self) -> crate::common::Reg<self::Ecmeth1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "IOM Event Combiner Module Global Event Selection Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ecmselr(&self) -> crate::common::Reg<self::Ecmselr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "IOM Filter and Prescaler Channel Control Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fpcctrk(&self) -> [crate::common::Reg<self::FpcctRk_SPEC, crate::common::RW>; 16] {
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
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x3cusize)),
            ]
        }
    }

    #[doc = "IOM Filter and Prescaler Channels Rising   Falling Edge Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fpcesr(&self) -> crate::common::Reg<self::Fpcesr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(120usize)) }
    }

    #[doc = "IOM Filter and Prescaler Channel Timer Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fpctimk(&self) -> [crate::common::Reg<self::FpctiMk_SPEC, crate::common::RW>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x3cusize)),
            ]
        }
    }

    #[doc = "IOM GTM Input EXOR Combiner Selection Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gtmexr(&self) -> crate::common::Reg<self::Gtmexr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "IOM Identification Register\n resetvalue={Application Reset:0x0CCC002}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "IOM Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst0(&self) -> crate::common::Reg<self::Krst0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "IOM Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst1(&self) -> crate::common::Reg<self::Krst1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "IOM Kernel Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krstclr(&self) -> crate::common::Reg<self::Krstclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "IOM Logic Analyzer Module Configuration Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn lamcfgm(&self) -> [crate::common::Reg<self::LamcfGm_SPEC, crate::common::RW>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x180usize + 0x3cusize)),
            ]
        }
    }

    #[doc = "IOM Logic Analyzer Module Event Window Count Status Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn lamewcm(&self) -> [crate::common::Reg<self::LamewCm_SPEC, crate::common::RW>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x3cusize)),
            ]
        }
    }

    #[doc = "IOM Logic Analyzer Module Event Window Configuration Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn lamewsm(&self) -> [crate::common::Reg<self::LamewSm_SPEC, crate::common::RW>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c0usize + 0x3cusize)),
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
#[doc = "IOM Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
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
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En1_SPEC;
    pub type En1 = crate::EnumBitfieldStruct<u8, En1_SPEC>;
    impl En1 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En2_SPEC;
    pub type En2 = crate::EnumBitfieldStruct<u8, En2_SPEC>;
    impl En2 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En3_SPEC;
    pub type En3 = crate::EnumBitfieldStruct<u8, En3_SPEC>;
    impl En3 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En4_SPEC;
    pub type En4 = crate::EnumBitfieldStruct<u8, En4_SPEC>;
    impl En4 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En5_SPEC;
    pub type En5 = crate::EnumBitfieldStruct<u8, En5_SPEC>;
    impl En5 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En6_SPEC;
    pub type En6 = crate::EnumBitfieldStruct<u8, En6_SPEC>;
    impl En6 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En7_SPEC;
    pub type En7 = crate::EnumBitfieldStruct<u8, En7_SPEC>;
    impl En7 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En8_SPEC;
    pub type En8 = crate::EnumBitfieldStruct<u8, En8_SPEC>;
    impl En8 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En9_SPEC;
    pub type En9 = crate::EnumBitfieldStruct<u8, En9_SPEC>;
    impl En9 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En10_SPEC;
    pub type En10 = crate::EnumBitfieldStruct<u8, En10_SPEC>;
    impl En10 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En11_SPEC;
    pub type En11 = crate::EnumBitfieldStruct<u8, En11_SPEC>;
    impl En11 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En12_SPEC;
    pub type En12 = crate::EnumBitfieldStruct<u8, En12_SPEC>;
    impl En12 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En13_SPEC;
    pub type En13 = crate::EnumBitfieldStruct<u8, En13_SPEC>;
    impl En13 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En14_SPEC;
    pub type En14 = crate::EnumBitfieldStruct<u8, En14_SPEC>;
    impl En14 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En15_SPEC;
    pub type En15 = crate::EnumBitfieldStruct<u8, En15_SPEC>;
    impl En15 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En16_SPEC;
    pub type En16 = crate::EnumBitfieldStruct<u8, En16_SPEC>;
    impl En16 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En17_SPEC;
    pub type En17 = crate::EnumBitfieldStruct<u8, En17_SPEC>;
    impl En17 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En18_SPEC;
    pub type En18 = crate::EnumBitfieldStruct<u8, En18_SPEC>;
    impl En18 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En19_SPEC;
    pub type En19 = crate::EnumBitfieldStruct<u8, En19_SPEC>;
    impl En19 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En20_SPEC;
    pub type En20 = crate::EnumBitfieldStruct<u8, En20_SPEC>;
    impl En20 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En21_SPEC;
    pub type En21 = crate::EnumBitfieldStruct<u8, En21_SPEC>;
    impl En21 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En22_SPEC;
    pub type En22 = crate::EnumBitfieldStruct<u8, En22_SPEC>;
    impl En22 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En23_SPEC;
    pub type En23 = crate::EnumBitfieldStruct<u8, En23_SPEC>;
    impl En23 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En24_SPEC;
    pub type En24 = crate::EnumBitfieldStruct<u8, En24_SPEC>;
    impl En24 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En25_SPEC;
    pub type En25 = crate::EnumBitfieldStruct<u8, En25_SPEC>;
    impl En25 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En26_SPEC;
    pub type En26 = crate::EnumBitfieldStruct<u8, En26_SPEC>;
    impl En26 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En27_SPEC;
    pub type En27 = crate::EnumBitfieldStruct<u8, En27_SPEC>;
    impl En27 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En28_SPEC;
    pub type En28 = crate::EnumBitfieldStruct<u8, En28_SPEC>;
    impl En28 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En29_SPEC;
    pub type En29 = crate::EnumBitfieldStruct<u8, En29_SPEC>;
    impl En29 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En30_SPEC;
    pub type En30 = crate::EnumBitfieldStruct<u8, En30_SPEC>;
    impl En30 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En31_SPEC;
    pub type En31 = crate::EnumBitfieldStruct<u8, En31_SPEC>;
    impl En31 {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clc_SPEC;
impl crate::sealed::RegSpec for Clc_SPEC {
    type DataType = u32;
}
#[doc = "IOM Clock Control Register\n resetvalue={Application Reset:0x3}"]
pub type Clc = crate::RegValueT<Clc_SPEC>;

impl Clc {
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control of the module."]
    #[inline(always)]
    pub fn disr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, clc::Disr, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,clc::Disr, Clc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Module Disable Status Bit   DISS. Bit indicates the current status of the module."]
    #[inline(always)]
    pub fn diss(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, clc::Diss, Clc_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,clc::Diss, Clc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Sleep Mode Enable Control   EDIS. Used to control module  8217 s sleep mode."]
    #[inline(always)]
    pub fn edis(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, clc::Edis, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,clc::Edis, Clc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "8 bit Clock Divider Value in RUN Mode   RMC"]
    #[inline(always)]
    pub fn rmc(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, clc::Rmc, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,clc::Rmc, Clc_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "Module disable is not requested"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module disable is requested"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Diss_SPEC;
    pub type Diss = crate::EnumBitfieldStruct<u8, Diss_SPEC>;
    impl Diss {
        #[doc = "Module is enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Edis_SPEC;
    pub type Edis = crate::EnumBitfieldStruct<u8, Edis_SPEC>;
    impl Edis {
        #[doc = "Sleep mode request is regarded. Module is enabled to go to into Sleep Mode."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Sleep mode request is disregarded. Sleep Mode cannot be entered upon a request."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rmc_SPEC;
    pub type Rmc = crate::EnumBitfieldStruct<u8, Rmc_SPEC>;
    impl Rmc {
        #[doc = "No clock signal fIOM generated  default after reset"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Clock fIOM equal to the input clock frequency selected"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecmccfg_SPEC;
impl crate::sealed::RegSpec for Ecmccfg_SPEC {
    type DataType = u32;
}
#[doc = "IOM Event Combiner Module Counter Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Ecmccfg = crate::RegValueT<Ecmccfg_SPEC>;

impl Ecmccfg {
    #[doc = "Event Channel Select. This bit field determines which channel event output is to be routed to counter C0."]
    #[inline(always)]
    pub fn selc0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, ecmccfg::Selc0, Ecmccfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,ecmccfg::Selc0, Ecmccfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel Event Counter Threshold. This bit field determines the threshold count value. Upon the counter meeting the threshold  the counter event output becomes active high for one clock cycle and the count is reset to zero."]
    #[inline(always)]
    pub fn thrc0(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, ecmccfg::Thrc0, Ecmccfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,ecmccfg::Thrc0, Ecmccfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Channel Select . This bit field determines which channel event output is to be routed to counter C1."]
    #[inline(always)]
    pub fn selc1(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, ecmccfg::Selc1, Ecmccfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,ecmccfg::Selc1, Ecmccfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel Event Counter Threshold . This bit field determines the threshold count value. Upon the counter meeting the threshold  the counter event output becomes active high for one clock cycle and the count is reset to zero."]
    #[inline(always)]
    pub fn thrc1(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, ecmccfg::Thrc1, Ecmccfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,ecmccfg::Thrc1, Ecmccfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Channel Select . This bit field determines which channel event output is to be routed to counter C2."]
    #[inline(always)]
    pub fn selc2(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, ecmccfg::Selc2, Ecmccfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,ecmccfg::Selc2, Ecmccfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel Event Counter Threshold. This bit field determines the threshold count value. Upon the counter meeting the threshold  the counter event output becomes active high for one clock cycle and the count is reset to zero."]
    #[inline(always)]
    pub fn thrc2(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, ecmccfg::Thrc2, Ecmccfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0xf,1,0,ecmccfg::Thrc2, Ecmccfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Channel Select. This bit field determines which channel event output is to be routed to counter C3."]
    #[inline(always)]
    pub fn selc3(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, ecmccfg::Selc3, Ecmccfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xf,1,0,ecmccfg::Selc3, Ecmccfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel Event Counter Threshold. This bit field determines the threshold count value. Upon the counter meeting the threshold  the counter event output becomes active high for one clock cycle and the count is reset to zero."]
    #[inline(always)]
    pub fn thrc3(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, ecmccfg::Thrc3, Ecmccfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0xf,1,0,ecmccfg::Thrc3, Ecmccfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ecmccfg {
    #[inline(always)]
    fn default() -> Ecmccfg {
        <crate::RegValueT<Ecmccfg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ecmccfg {
    pub struct Selc0_SPEC;
    pub type Selc0 = crate::EnumBitfieldStruct<u8, Selc0_SPEC>;
    impl Selc0 {
        #[doc = "0000 Select channel 0 event output to be counted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "0001 Select channel 1 event output to be counted."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "0010 Select channel 2 event output to be counted."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "0011 Select channel 3 event output to be counted."]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "0100 Select channel 4 event output to be counted."]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "0101 Select channel 5 event output to be counted."]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "0110 Select channel 6 event output to be counted."]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "0111 Select channel 7 event output to be counted."]
        pub const CONST_77: Self = Self::new(7);
        #[doc = "1000 Select channel 8 event output to be counted."]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "1001 Select channel 9 event output to be counted."]
        pub const CONST_99: Self = Self::new(9);
        #[doc = "1010 Select channel 10 event output to be counted."]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "1011 Select channel 11 event output to be counted."]
        pub const CONST_1111: Self = Self::new(11);
        #[doc = "1100 Select channel 12 event output to be counted."]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "1101 Select channel 13 event output to be counted."]
        pub const CONST_1313: Self = Self::new(13);
        #[doc = "1110 Select channel 14 event output to be counted."]
        pub const CONST_1414: Self = Self::new(14);
        #[doc = "1111 Select channel 15 event output to be counted."]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Thrc0_SPEC;
    pub type Thrc0 = crate::EnumBitfieldStruct<u8, Thrc0_SPEC>;
    impl Thrc0 {
        #[doc = "0000 Counter disabled  counter event output set to inactive  despite any incoming channel events ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "0001 Minimum threshold count value."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "1111 Maximum threshold count value."]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Selc1_SPEC;
    pub type Selc1 = crate::EnumBitfieldStruct<u8, Selc1_SPEC>;
    impl Selc1 {
        #[doc = "0000 Select channel 0 event output to be counted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "0001 Select channel 1 event output to be counted."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "0010 Select channel 2 event output to be counted."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "0011 Select channel 3 event output to be counted."]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "0100 Select channel 4 event output to be counted."]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "0101 Select channel 5 event output to be counted."]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "0110 Select channel 6 event output to be counted."]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "0111 Select channel 7 event output to be counted."]
        pub const CONST_77: Self = Self::new(7);
        #[doc = "1000 Select channel 8 event output to be counted."]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "1001 Select channel 9 event output to be counted."]
        pub const CONST_99: Self = Self::new(9);
        #[doc = "1010 Select channel 10 event output to be counted."]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "1011 Select channel 11 event output to be counted."]
        pub const CONST_1111: Self = Self::new(11);
        #[doc = "1100 Select channel 12 event output to be counted."]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "1101 Select channel 13 event output to be counted."]
        pub const CONST_1313: Self = Self::new(13);
        #[doc = "1110 Select channel 14 event output to be counted."]
        pub const CONST_1414: Self = Self::new(14);
        #[doc = "1111 Select channel 15 event output to be counted."]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Thrc1_SPEC;
    pub type Thrc1 = crate::EnumBitfieldStruct<u8, Thrc1_SPEC>;
    impl Thrc1 {
        #[doc = "0000 Counter disabled  counter event output set to inactive  despite any incoming channel events ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "0001 Minimum threshold count value."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "1111 Maximum threshold count value."]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Selc2_SPEC;
    pub type Selc2 = crate::EnumBitfieldStruct<u8, Selc2_SPEC>;
    impl Selc2 {
        #[doc = "0000 Select channel 0 event output to be counted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "0001 Select channel 1 event output to be counted."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "0010 Select channel 2 event output to be counted."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "0011 Select channel 3 event output to be counted."]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "0100 Select channel 4 event output to be counted."]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "0101 Select channel 5 event output to be counted."]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "0110 Select channel 6 event output to be counted."]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "0111 Select channel 7 event output to be counted."]
        pub const CONST_77: Self = Self::new(7);
        #[doc = "1000 Select channel 8 event output to be counted."]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "1001 Select channel 9 event output to be counted."]
        pub const CONST_99: Self = Self::new(9);
        #[doc = "1010 Select channel 10 event output to be counted."]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "1011 Select channel 11 event output to be counted."]
        pub const CONST_1111: Self = Self::new(11);
        #[doc = "1100 Select channel 12 event output to be counted."]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "1101 Select channel 13 event output to be counted."]
        pub const CONST_1313: Self = Self::new(13);
        #[doc = "1110 Select channel 14 event output to be counted."]
        pub const CONST_1414: Self = Self::new(14);
        #[doc = "1111 Select channel 15 event output to be counted."]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Thrc2_SPEC;
    pub type Thrc2 = crate::EnumBitfieldStruct<u8, Thrc2_SPEC>;
    impl Thrc2 {
        #[doc = "0000 Counter disabled  counter event output set to inactive  despite any incoming channel events ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "0001 Minimum threshold count value."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "1111 Maximum threshold count value."]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Selc3_SPEC;
    pub type Selc3 = crate::EnumBitfieldStruct<u8, Selc3_SPEC>;
    impl Selc3 {
        #[doc = "0000 Select channel 0 event output to be counted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "0001 Select channel 1 event output to be counted."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "0010 Select channel 2 event output to be counted."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "0011 Select channel 3 event output to be counted."]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "0100 Select channel 4 event output to be counted."]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "0101 Select channel 5 event output to be counted."]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "0110 Select channel 6 event output to be counted."]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "0111 Select channel 7 event output to be counted."]
        pub const CONST_77: Self = Self::new(7);
        #[doc = "1000 Select channel 8 event output to be counted."]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "1001 Select channel 9 event output to be counted."]
        pub const CONST_99: Self = Self::new(9);
        #[doc = "1010 Select channel 10 event output to be counted."]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "1011 Select channel 11 event output to be counted."]
        pub const CONST_1111: Self = Self::new(11);
        #[doc = "1100 Select channel 12 event output to be counted."]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "1101 Select channel 13 event output to be counted."]
        pub const CONST_1313: Self = Self::new(13);
        #[doc = "1110 Select channel 14 event output to be counted."]
        pub const CONST_1414: Self = Self::new(14);
        #[doc = "1111 Select channel 15 event output to be counted."]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Thrc3_SPEC;
    pub type Thrc3 = crate::EnumBitfieldStruct<u8, Thrc3_SPEC>;
    impl Thrc3 {
        #[doc = "0000 Counter disabled  counter event output set to inactive  despite any incoming channel events ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "0001 Minimum threshold count value."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "1111 Maximum threshold count value."]
        pub const CONST_1515: Self = Self::new(15);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecmeth0_SPEC;
impl crate::sealed::RegSpec for Ecmeth0_SPEC {
    type DataType = u32;
}
#[doc = "IOM Event Combiner Module Event Trigger History Register 0\n resetvalue={Application Reset:0x0}"]
pub type Ecmeth0 = crate::RegValueT<Ecmeth0_SPEC>;

impl Ecmeth0 {
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ecmeth0::Eta0, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,ecmeth0::Eta0, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ecmeth0::Eta1, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,ecmeth0::Eta1, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ecmeth0::Eta2, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,ecmeth0::Eta2, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ecmeth0::Eta3, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,ecmeth0::Eta3, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ecmeth0::Eta4, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,ecmeth0::Eta4, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ecmeth0::Eta5, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,ecmeth0::Eta5, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, ecmeth0::Eta6, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,ecmeth0::Eta6, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, ecmeth0::Eta7, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,ecmeth0::Eta7, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, ecmeth0::Eta8, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,ecmeth0::Eta8, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, ecmeth0::Eta9, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,ecmeth0::Eta9, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, ecmeth0::Eta10, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,ecmeth0::Eta10, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, ecmeth0::Eta11, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,ecmeth0::Eta11, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, ecmeth0::Eta12, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,ecmeth0::Eta12, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, ecmeth0::Eta13, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,ecmeth0::Eta13, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, ecmeth0::Eta14, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,ecmeth0::Eta14, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  last    ETA15. Contains the status of the event trigger outputs for each of the LAM blocks for the last generated event s ."]
    #[inline(always)]
    pub fn eta15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, ecmeth0::Eta15, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,ecmeth0::Eta15, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb0(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, ecmeth0::Etb0, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,ecmeth0::Etb0, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb1(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, ecmeth0::Etb1, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,ecmeth0::Etb1, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb2(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, ecmeth0::Etb2, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,ecmeth0::Etb2, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb3(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, ecmeth0::Etb3, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,ecmeth0::Etb3, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb4(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, ecmeth0::Etb4, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,ecmeth0::Etb4, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb5(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, ecmeth0::Etb5, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,ecmeth0::Etb5, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb6(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, ecmeth0::Etb6, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,ecmeth0::Etb6, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb7(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, ecmeth0::Etb7, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,ecmeth0::Etb7, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb8(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, ecmeth0::Etb8, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,ecmeth0::Etb8, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb9(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, ecmeth0::Etb9, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,ecmeth0::Etb9, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb10(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, ecmeth0::Etb10, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,ecmeth0::Etb10, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb11(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, ecmeth0::Etb11, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,ecmeth0::Etb11, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb12(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, ecmeth0::Etb12, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,ecmeth0::Etb12, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb13(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, ecmeth0::Etb13, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,ecmeth0::Etb13, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb14(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, ecmeth0::Etb14, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,ecmeth0::Etb14, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETA0 15    ETB15. Contains the previous contents of ETA0 15 prior to being updated."]
    #[inline(always)]
    pub fn etb15(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, ecmeth0::Etb15, Ecmeth0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,ecmeth0::Etb15, Ecmeth0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ecmeth0 {
    #[inline(always)]
    fn default() -> Ecmeth0 {
        <crate::RegValueT<Ecmeth0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ecmeth0 {
    pub struct Eta0_SPEC;
    pub type Eta0 = crate::EnumBitfieldStruct<u8, Eta0_SPEC>;
    impl Eta0 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eta1_SPEC;
    pub type Eta1 = crate::EnumBitfieldStruct<u8, Eta1_SPEC>;
    impl Eta1 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eta2_SPEC;
    pub type Eta2 = crate::EnumBitfieldStruct<u8, Eta2_SPEC>;
    impl Eta2 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eta3_SPEC;
    pub type Eta3 = crate::EnumBitfieldStruct<u8, Eta3_SPEC>;
    impl Eta3 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eta4_SPEC;
    pub type Eta4 = crate::EnumBitfieldStruct<u8, Eta4_SPEC>;
    impl Eta4 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eta5_SPEC;
    pub type Eta5 = crate::EnumBitfieldStruct<u8, Eta5_SPEC>;
    impl Eta5 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eta6_SPEC;
    pub type Eta6 = crate::EnumBitfieldStruct<u8, Eta6_SPEC>;
    impl Eta6 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eta7_SPEC;
    pub type Eta7 = crate::EnumBitfieldStruct<u8, Eta7_SPEC>;
    impl Eta7 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eta8_SPEC;
    pub type Eta8 = crate::EnumBitfieldStruct<u8, Eta8_SPEC>;
    impl Eta8 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eta9_SPEC;
    pub type Eta9 = crate::EnumBitfieldStruct<u8, Eta9_SPEC>;
    impl Eta9 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eta10_SPEC;
    pub type Eta10 = crate::EnumBitfieldStruct<u8, Eta10_SPEC>;
    impl Eta10 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eta11_SPEC;
    pub type Eta11 = crate::EnumBitfieldStruct<u8, Eta11_SPEC>;
    impl Eta11 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eta12_SPEC;
    pub type Eta12 = crate::EnumBitfieldStruct<u8, Eta12_SPEC>;
    impl Eta12 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eta13_SPEC;
    pub type Eta13 = crate::EnumBitfieldStruct<u8, Eta13_SPEC>;
    impl Eta13 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eta14_SPEC;
    pub type Eta14 = crate::EnumBitfieldStruct<u8, Eta14_SPEC>;
    impl Eta14 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eta15_SPEC;
    pub type Eta15 = crate::EnumBitfieldStruct<u8, Eta15_SPEC>;
    impl Eta15 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etb0_SPEC;
    pub type Etb0 = crate::EnumBitfieldStruct<u8, Etb0_SPEC>;
    impl Etb0 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etb1_SPEC;
    pub type Etb1 = crate::EnumBitfieldStruct<u8, Etb1_SPEC>;
    impl Etb1 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etb2_SPEC;
    pub type Etb2 = crate::EnumBitfieldStruct<u8, Etb2_SPEC>;
    impl Etb2 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etb3_SPEC;
    pub type Etb3 = crate::EnumBitfieldStruct<u8, Etb3_SPEC>;
    impl Etb3 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etb4_SPEC;
    pub type Etb4 = crate::EnumBitfieldStruct<u8, Etb4_SPEC>;
    impl Etb4 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etb5_SPEC;
    pub type Etb5 = crate::EnumBitfieldStruct<u8, Etb5_SPEC>;
    impl Etb5 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etb6_SPEC;
    pub type Etb6 = crate::EnumBitfieldStruct<u8, Etb6_SPEC>;
    impl Etb6 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etb7_SPEC;
    pub type Etb7 = crate::EnumBitfieldStruct<u8, Etb7_SPEC>;
    impl Etb7 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etb8_SPEC;
    pub type Etb8 = crate::EnumBitfieldStruct<u8, Etb8_SPEC>;
    impl Etb8 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etb9_SPEC;
    pub type Etb9 = crate::EnumBitfieldStruct<u8, Etb9_SPEC>;
    impl Etb9 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etb10_SPEC;
    pub type Etb10 = crate::EnumBitfieldStruct<u8, Etb10_SPEC>;
    impl Etb10 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etb11_SPEC;
    pub type Etb11 = crate::EnumBitfieldStruct<u8, Etb11_SPEC>;
    impl Etb11 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etb12_SPEC;
    pub type Etb12 = crate::EnumBitfieldStruct<u8, Etb12_SPEC>;
    impl Etb12 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etb13_SPEC;
    pub type Etb13 = crate::EnumBitfieldStruct<u8, Etb13_SPEC>;
    impl Etb13 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etb14_SPEC;
    pub type Etb14 = crate::EnumBitfieldStruct<u8, Etb14_SPEC>;
    impl Etb14 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etb15_SPEC;
    pub type Etb15 = crate::EnumBitfieldStruct<u8, Etb15_SPEC>;
    impl Etb15 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecmeth1_SPEC;
impl crate::sealed::RegSpec for Ecmeth1_SPEC {
    type DataType = u32;
}
#[doc = "IOM Event Combiner Module Event Trigger History Register 1\n resetvalue={Application Reset:0x0}"]
pub type Ecmeth1 = crate::RegValueT<Ecmeth1_SPEC>;

impl Ecmeth1 {
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ecmeth1::Etc0, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,ecmeth1::Etc0, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ecmeth1::Etc1, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,ecmeth1::Etc1, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ecmeth1::Etc2, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,ecmeth1::Etc2, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ecmeth1::Etc3, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,ecmeth1::Etc3, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ecmeth1::Etc4, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,ecmeth1::Etc4, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ecmeth1::Etc5, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,ecmeth1::Etc5, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, ecmeth1::Etc6, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,ecmeth1::Etc6, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, ecmeth1::Etc7, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,ecmeth1::Etc7, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, ecmeth1::Etc8, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,ecmeth1::Etc8, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, ecmeth1::Etc9, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,ecmeth1::Etc9, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, ecmeth1::Etc10, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,ecmeth1::Etc10, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, ecmeth1::Etc11, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,ecmeth1::Etc11, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, ecmeth1::Etc12, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,ecmeth1::Etc12, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, ecmeth1::Etc13, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,ecmeth1::Etc13, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, ecmeth1::Etc14, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,ecmeth1::Etc14, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous ETB0 15    ETC15. Contains the previous contents of ETB0 15 prior to being updated."]
    #[inline(always)]
    pub fn etc15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, ecmeth1::Etc15, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,ecmeth1::Etc15, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd0(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, ecmeth1::Etd0, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,ecmeth1::Etd0, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd1(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, ecmeth1::Etd1, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,ecmeth1::Etd1, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd2(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, ecmeth1::Etd2, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,ecmeth1::Etd2, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd3(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, ecmeth1::Etd3, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,ecmeth1::Etd3, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd4(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, ecmeth1::Etd4, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,ecmeth1::Etd4, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd5(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, ecmeth1::Etd5, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,ecmeth1::Etd5, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd6(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, ecmeth1::Etd6, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,ecmeth1::Etd6, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd7(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, ecmeth1::Etd7, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,ecmeth1::Etd7, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd8(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, ecmeth1::Etd8, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,ecmeth1::Etd8, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd9(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, ecmeth1::Etd9, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,ecmeth1::Etd9, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd10(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, ecmeth1::Etd10, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,ecmeth1::Etd10, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd11(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, ecmeth1::Etd11, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,ecmeth1::Etd11, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd12(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, ecmeth1::Etd12, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,ecmeth1::Etd12, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd13(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, ecmeth1::Etd13, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,ecmeth1::Etd13, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd14(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, ecmeth1::Etd14, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,ecmeth1::Etd14, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LAM 0 15 Eve15t Trigger Activity  previous contents of ETC0 15 OR ed with the previous value of ETD0 15    ETD15. Contains the previous content of ETC0 15 OR ed with the previous value of ETD0 15."]
    #[inline(always)]
    pub fn etd15(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, ecmeth1::Etd15, Ecmeth1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,ecmeth1::Etd15, Ecmeth1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ecmeth1 {
    #[inline(always)]
    fn default() -> Ecmeth1 {
        <crate::RegValueT<Ecmeth1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ecmeth1 {
    pub struct Etc0_SPEC;
    pub type Etc0 = crate::EnumBitfieldStruct<u8, Etc0_SPEC>;
    impl Etc0 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etc1_SPEC;
    pub type Etc1 = crate::EnumBitfieldStruct<u8, Etc1_SPEC>;
    impl Etc1 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etc2_SPEC;
    pub type Etc2 = crate::EnumBitfieldStruct<u8, Etc2_SPEC>;
    impl Etc2 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etc3_SPEC;
    pub type Etc3 = crate::EnumBitfieldStruct<u8, Etc3_SPEC>;
    impl Etc3 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etc4_SPEC;
    pub type Etc4 = crate::EnumBitfieldStruct<u8, Etc4_SPEC>;
    impl Etc4 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etc5_SPEC;
    pub type Etc5 = crate::EnumBitfieldStruct<u8, Etc5_SPEC>;
    impl Etc5 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etc6_SPEC;
    pub type Etc6 = crate::EnumBitfieldStruct<u8, Etc6_SPEC>;
    impl Etc6 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etc7_SPEC;
    pub type Etc7 = crate::EnumBitfieldStruct<u8, Etc7_SPEC>;
    impl Etc7 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etc8_SPEC;
    pub type Etc8 = crate::EnumBitfieldStruct<u8, Etc8_SPEC>;
    impl Etc8 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etc9_SPEC;
    pub type Etc9 = crate::EnumBitfieldStruct<u8, Etc9_SPEC>;
    impl Etc9 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etc10_SPEC;
    pub type Etc10 = crate::EnumBitfieldStruct<u8, Etc10_SPEC>;
    impl Etc10 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etc11_SPEC;
    pub type Etc11 = crate::EnumBitfieldStruct<u8, Etc11_SPEC>;
    impl Etc11 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etc12_SPEC;
    pub type Etc12 = crate::EnumBitfieldStruct<u8, Etc12_SPEC>;
    impl Etc12 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etc13_SPEC;
    pub type Etc13 = crate::EnumBitfieldStruct<u8, Etc13_SPEC>;
    impl Etc13 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etc14_SPEC;
    pub type Etc14 = crate::EnumBitfieldStruct<u8, Etc14_SPEC>;
    impl Etc14 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etc15_SPEC;
    pub type Etc15 = crate::EnumBitfieldStruct<u8, Etc15_SPEC>;
    impl Etc15 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etd0_SPEC;
    pub type Etd0 = crate::EnumBitfieldStruct<u8, Etd0_SPEC>;
    impl Etd0 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etd1_SPEC;
    pub type Etd1 = crate::EnumBitfieldStruct<u8, Etd1_SPEC>;
    impl Etd1 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etd2_SPEC;
    pub type Etd2 = crate::EnumBitfieldStruct<u8, Etd2_SPEC>;
    impl Etd2 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etd3_SPEC;
    pub type Etd3 = crate::EnumBitfieldStruct<u8, Etd3_SPEC>;
    impl Etd3 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etd4_SPEC;
    pub type Etd4 = crate::EnumBitfieldStruct<u8, Etd4_SPEC>;
    impl Etd4 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etd5_SPEC;
    pub type Etd5 = crate::EnumBitfieldStruct<u8, Etd5_SPEC>;
    impl Etd5 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etd6_SPEC;
    pub type Etd6 = crate::EnumBitfieldStruct<u8, Etd6_SPEC>;
    impl Etd6 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etd7_SPEC;
    pub type Etd7 = crate::EnumBitfieldStruct<u8, Etd7_SPEC>;
    impl Etd7 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etd8_SPEC;
    pub type Etd8 = crate::EnumBitfieldStruct<u8, Etd8_SPEC>;
    impl Etd8 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etd9_SPEC;
    pub type Etd9 = crate::EnumBitfieldStruct<u8, Etd9_SPEC>;
    impl Etd9 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etd10_SPEC;
    pub type Etd10 = crate::EnumBitfieldStruct<u8, Etd10_SPEC>;
    impl Etd10 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etd11_SPEC;
    pub type Etd11 = crate::EnumBitfieldStruct<u8, Etd11_SPEC>;
    impl Etd11 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etd12_SPEC;
    pub type Etd12 = crate::EnumBitfieldStruct<u8, Etd12_SPEC>;
    impl Etd12 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etd13_SPEC;
    pub type Etd13 = crate::EnumBitfieldStruct<u8, Etd13_SPEC>;
    impl Etd13 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etd14_SPEC;
    pub type Etd14 = crate::EnumBitfieldStruct<u8, Etd14_SPEC>;
    impl Etd14 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etd15_SPEC;
    pub type Etd15 = crate::EnumBitfieldStruct<u8, Etd15_SPEC>;
    impl Etd15 {
        #[doc = "LAM channel n did not trigger a system event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LAM channel n triggered a system event."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecmselr_SPEC;
impl crate::sealed::RegSpec for Ecmselr_SPEC {
    type DataType = u32;
}
#[doc = "IOM Event Combiner Module Global Event Selection Register\n resetvalue={Application Reset:0x0}"]
pub type Ecmselr = crate::RegValueT<Ecmselr_SPEC>;

impl Ecmselr {
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ecmselr::Ces0, Ecmselr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,ecmselr::Ces0, Ecmselr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ecmselr::Ces1, Ecmselr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,ecmselr::Ces1, Ecmselr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ecmselr::Ces2, Ecmselr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,ecmselr::Ces2, Ecmselr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ecmselr::Ces3, Ecmselr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,ecmselr::Ces3, Ecmselr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ecmselr::Ces4, Ecmselr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,ecmselr::Ces4, Ecmselr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ecmselr::Ces5, Ecmselr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,ecmselr::Ces5, Ecmselr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, ecmselr::Ces6, Ecmselr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,ecmselr::Ces6, Ecmselr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, ecmselr::Ces7, Ecmselr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,ecmselr::Ces7, Ecmselr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, ecmselr::Ces8, Ecmselr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,ecmselr::Ces8, Ecmselr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, ecmselr::Ces9, Ecmselr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,ecmselr::Ces9, Ecmselr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, ecmselr::Ces10, Ecmselr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,ecmselr::Ces10, Ecmselr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, ecmselr::Ces11, Ecmselr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,ecmselr::Ces11, Ecmselr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, ecmselr::Ces12, Ecmselr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,ecmselr::Ces12, Ecmselr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, ecmselr::Ces13, Ecmselr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,ecmselr::Ces13, Ecmselr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, ecmselr::Ces14, Ecmselr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,ecmselr::Ces14, Ecmselr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Eve15t Combiner Selection   CES15. The setting of individual bitfields determines the inclusion of the respective channel event in the generation of the global event."]
    #[inline(always)]
    pub fn ces15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, ecmselr::Ces15, Ecmselr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,ecmselr::Ces15, Ecmselr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Accumulated  Cou3ted  Event Combiner Selection   CTS3. The setting of individual bitfields determines the inclusion of the respective channel event counter output  1 of 4  in the generation of the global event."]
    #[inline(always)]
    pub fn cts0(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, ecmselr::Cts0, Ecmselr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,ecmselr::Cts0, Ecmselr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Accumulated  Cou3ted  Event Combiner Selection   CTS3. The setting of individual bitfields determines the inclusion of the respective channel event counter output  1 of 4  in the generation of the global event."]
    #[inline(always)]
    pub fn cts1(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, ecmselr::Cts1, Ecmselr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,ecmselr::Cts1, Ecmselr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Accumulated  Cou3ted  Event Combiner Selection   CTS3. The setting of individual bitfields determines the inclusion of the respective channel event counter output  1 of 4  in the generation of the global event."]
    #[inline(always)]
    pub fn cts2(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, ecmselr::Cts2, Ecmselr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,ecmselr::Cts2, Ecmselr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Accumulated  Cou3ted  Event Combiner Selection   CTS3. The setting of individual bitfields determines the inclusion of the respective channel event counter output  1 of 4  in the generation of the global event."]
    #[inline(always)]
    pub fn cts3(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, ecmselr::Cts3, Ecmselr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,ecmselr::Cts3, Ecmselr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ecmselr {
    #[inline(always)]
    fn default() -> Ecmselr {
        <crate::RegValueT<Ecmselr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ecmselr {
    pub struct Ces0_SPEC;
    pub type Ces0 = crate::EnumBitfieldStruct<u8, Ces0_SPEC>;
    impl Ces0 {
        #[doc = "0 Don  x2019 t include channel event event counter output within the global event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Include channel event event counter output within the global event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ces1_SPEC;
    pub type Ces1 = crate::EnumBitfieldStruct<u8, Ces1_SPEC>;
    impl Ces1 {
        #[doc = "0 Don  x2019 t include channel event event counter output within the global event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Include channel event event counter output within the global event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ces2_SPEC;
    pub type Ces2 = crate::EnumBitfieldStruct<u8, Ces2_SPEC>;
    impl Ces2 {
        #[doc = "0 Don  x2019 t include channel event event counter output within the global event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Include channel event event counter output within the global event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ces3_SPEC;
    pub type Ces3 = crate::EnumBitfieldStruct<u8, Ces3_SPEC>;
    impl Ces3 {
        #[doc = "0 Don  x2019 t include channel event event counter output within the global event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Include channel event event counter output within the global event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ces4_SPEC;
    pub type Ces4 = crate::EnumBitfieldStruct<u8, Ces4_SPEC>;
    impl Ces4 {
        #[doc = "0 Don  x2019 t include channel event event counter output within the global event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Include channel event event counter output within the global event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ces5_SPEC;
    pub type Ces5 = crate::EnumBitfieldStruct<u8, Ces5_SPEC>;
    impl Ces5 {
        #[doc = "0 Don  x2019 t include channel event event counter output within the global event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Include channel event event counter output within the global event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ces6_SPEC;
    pub type Ces6 = crate::EnumBitfieldStruct<u8, Ces6_SPEC>;
    impl Ces6 {
        #[doc = "0 Don  x2019 t include channel event event counter output within the global event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Include channel event event counter output within the global event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ces7_SPEC;
    pub type Ces7 = crate::EnumBitfieldStruct<u8, Ces7_SPEC>;
    impl Ces7 {
        #[doc = "0 Don  x2019 t include channel event event counter output within the global event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Include channel event event counter output within the global event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ces8_SPEC;
    pub type Ces8 = crate::EnumBitfieldStruct<u8, Ces8_SPEC>;
    impl Ces8 {
        #[doc = "0 Don  x2019 t include channel event event counter output within the global event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Include channel event event counter output within the global event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ces9_SPEC;
    pub type Ces9 = crate::EnumBitfieldStruct<u8, Ces9_SPEC>;
    impl Ces9 {
        #[doc = "0 Don  x2019 t include channel event event counter output within the global event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Include channel event event counter output within the global event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ces10_SPEC;
    pub type Ces10 = crate::EnumBitfieldStruct<u8, Ces10_SPEC>;
    impl Ces10 {
        #[doc = "0 Don  x2019 t include channel event event counter output within the global event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Include channel event event counter output within the global event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ces11_SPEC;
    pub type Ces11 = crate::EnumBitfieldStruct<u8, Ces11_SPEC>;
    impl Ces11 {
        #[doc = "0 Don  x2019 t include channel event event counter output within the global event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Include channel event event counter output within the global event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ces12_SPEC;
    pub type Ces12 = crate::EnumBitfieldStruct<u8, Ces12_SPEC>;
    impl Ces12 {
        #[doc = "0 Don  x2019 t include channel event event counter output within the global event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Include channel event event counter output within the global event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ces13_SPEC;
    pub type Ces13 = crate::EnumBitfieldStruct<u8, Ces13_SPEC>;
    impl Ces13 {
        #[doc = "0 Don  x2019 t include channel event event counter output within the global event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Include channel event event counter output within the global event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ces14_SPEC;
    pub type Ces14 = crate::EnumBitfieldStruct<u8, Ces14_SPEC>;
    impl Ces14 {
        #[doc = "0 Don  x2019 t include channel event event counter output within the global event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Include channel event event counter output within the global event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ces15_SPEC;
    pub type Ces15 = crate::EnumBitfieldStruct<u8, Ces15_SPEC>;
    impl Ces15 {
        #[doc = "0 Don  x2019 t include channel event event counter output within the global event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Include channel event event counter output within the global event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cts0_SPEC;
    pub type Cts0 = crate::EnumBitfieldStruct<u8, Cts0_SPEC>;
    impl Cts0 {
        #[doc = "0 Don  x2019 t include channel event event counter output within the global event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Include channel event event counter output within the global event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cts1_SPEC;
    pub type Cts1 = crate::EnumBitfieldStruct<u8, Cts1_SPEC>;
    impl Cts1 {
        #[doc = "0 Don  x2019 t include channel event event counter output within the global event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Include channel event event counter output within the global event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cts2_SPEC;
    pub type Cts2 = crate::EnumBitfieldStruct<u8, Cts2_SPEC>;
    impl Cts2 {
        #[doc = "0 Don  x2019 t include channel event event counter output within the global event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Include channel event event counter output within the global event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cts3_SPEC;
    pub type Cts3 = crate::EnumBitfieldStruct<u8, Cts3_SPEC>;
    impl Cts3 {
        #[doc = "0 Don  x2019 t include channel event event counter output within the global event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Include channel event event counter output within the global event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FpcctRk_SPEC;
impl crate::sealed::RegSpec for FpcctRk_SPEC {
    type DataType = u32;
}
#[doc = "IOM Filter and Prescaler Channel Control Register 0\n resetvalue={Application Reset:0x0}"]
pub type FpcctRk = crate::RegValueT<FpcctRk_SPEC>;

impl FpcctRk {
    #[doc = "Threshold Value of Filter   Prescaler Channel k   CMP. CMP is the 16 bit threshold value that is compared with the 16 bit timer value FPCTIMk.TIM."]
    #[inline(always)]
    pub fn cmp(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, FpcctRk_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, FpcctRk_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Operation Mode Selection for Filter   Prescaler Channel k   MOD"]
    #[inline(always)]
    pub fn r#mod(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, fpcctrk::Mod, FpcctRk_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7,1,0,fpcctrk::Mod, FpcctRk_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Monitor Input Signal Selection for Filter   Prescaler Channel k   ISM. ISM selects the monitor signal that goes to the monitor input  MON  of        the Logic Analyzer Module."]
    #[inline(always)]
    pub fn ism(
        self,
    ) -> crate::common::RegisterField<19, 0x3, 1, 0, fpcctrk::Ism, FpcctRk_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x3,1,0,fpcctrk::Ism, FpcctRk_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reset Timer behavior for Filter   Prescaler Channel k on Glitch   RTG. This bit is effective in Delayed Debounce Filter Mode only."]
    #[inline(always)]
    pub fn rtg(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, fpcctrk::Rtg, FpcctRk_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,fpcctrk::Rtg, FpcctRk_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reference Input Signal Selection for Filter   Prescaler Channel k   ISR. ISR select the reference signal that goes to the reference input  REF         of the Logic Analyzer Module."]
    #[inline(always)]
    pub fn isr(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, fpcctrk::Isr, FpcctRk_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x7,1,0,fpcctrk::Isr, FpcctRk_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for FpcctRk {
    #[inline(always)]
    fn default() -> FpcctRk {
        <crate::RegValueT<FpcctRk_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fpcctrk {
    pub struct Mod_SPEC;
    pub type Mod = crate::EnumBitfieldStruct<u8, Mod_SPEC>;
    impl Mod {
        #[doc = "000 Delayed        Debounce Filter Mode on both edges"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 Immediate Debounce Filter Mode on both edges"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 Rising edge         Immediate Debounce Filter Mode  falling edge  no filtering"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 Rising edge         no filtering  falling edge  Immediate Debounce Filter Mode"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 Rising edge  Delayed Debounce Filter Mode  falling edge  Immediate Debounce Filter Mode"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 Rising edge  Immediate Debounce Filter Mode  falling edge  Delayed Debounce Filter Mode"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 Prescaler Mode  triggered on rising edge"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "111 Prescaler        Mode  triggered on falling edge"]
        pub const CONST_77: Self = Self::new(7);
    }
    pub struct Ism_SPEC;
    pub type Ism = crate::EnumBitfieldStruct<u8, Ism_SPEC>;
    impl Ism {
        #[doc = "00 Processed        input signal pin selected  SOL"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Monitor Signal Input 0   MON0  selected"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Monitor Signal Input 1  MON1  selected"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Monitor Signal Input 2  MON2  selected"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Rtg_SPEC;
    pub type Rtg = crate::EnumBitfieldStruct<u8, Rtg_SPEC>;
    impl Rtg {
        #[doc = "0 Timer for FPCk is decremented on glitch"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer for FPCk is cleared on glitch"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Isr_SPEC;
    pub type Isr = crate::EnumBitfieldStruct<u8, Isr_SPEC>;
    impl Isr {
        #[doc = "000 Processed        input signal pin selected  SOL"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 Reference Signal Input 0selected"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 Reference        Signal Input 1selected"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 Reference Signal Input 2selected"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "1xx GTM XOR Combiner selected"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "1xx GTM XOR Combiner selected"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "1xx GTM XOR Combiner selected"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "1xx GTM XOR Combiner selected"]
        pub const CONST_77: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fpcesr_SPEC;
impl crate::sealed::RegSpec for Fpcesr_SPEC {
    type DataType = u32;
}
#[doc = "IOM Filter and Prescaler Channels Rising   Falling Edge Status Register\n resetvalue={Application Reset:0x0}"]
pub type Fpcesr = crate::RegValueT<Fpcesr_SPEC>;

impl Fpcesr {
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, fpcesr::Feg0, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,fpcesr::Feg0, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, fpcesr::Feg1, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,fpcesr::Feg1, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, fpcesr::Feg2, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,fpcesr::Feg2, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, fpcesr::Feg3, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,fpcesr::Feg3, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, fpcesr::Feg4, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,fpcesr::Feg4, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, fpcesr::Feg5, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,fpcesr::Feg5, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, fpcesr::Feg6, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,fpcesr::Feg6, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, fpcesr::Feg7, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,fpcesr::Feg7, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, fpcesr::Feg8, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,fpcesr::Feg8, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, fpcesr::Feg9, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,fpcesr::Feg9, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, fpcesr::Feg10, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,fpcesr::Feg10, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, fpcesr::Feg11, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,fpcesr::Feg11, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, fpcesr::Feg12, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,fpcesr::Feg12, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, fpcesr::Feg13, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,fpcesr::Feg13, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, fpcesr::Feg14, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,fpcesr::Feg14, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Falling Edge Glitch Flag for FPC15   FEG15"]
    #[inline(always)]
    pub fn feg15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, fpcesr::Feg15, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,fpcesr::Feg15, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg0(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, fpcesr::Reg0, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,fpcesr::Reg0, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg1(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, fpcesr::Reg1, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,fpcesr::Reg1, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg2(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, fpcesr::Reg2, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,fpcesr::Reg2, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg3(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, fpcesr::Reg3, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,fpcesr::Reg3, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg4(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, fpcesr::Reg4, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,fpcesr::Reg4, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg5(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, fpcesr::Reg5, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,fpcesr::Reg5, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg6(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, fpcesr::Reg6, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,fpcesr::Reg6, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg7(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, fpcesr::Reg7, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,fpcesr::Reg7, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg8(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, fpcesr::Reg8, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,fpcesr::Reg8, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg9(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, fpcesr::Reg9, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,fpcesr::Reg9, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg10(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, fpcesr::Reg10, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,fpcesr::Reg10, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg11(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, fpcesr::Reg11, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,fpcesr::Reg11, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg12(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, fpcesr::Reg12, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,fpcesr::Reg12, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg13(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, fpcesr::Reg13, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,fpcesr::Reg13, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg14(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, fpcesr::Reg14, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,fpcesr::Reg14, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Rising Edge Glitch Flag for FPC15   REG15"]
    #[inline(always)]
    pub fn reg15(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, fpcesr::Reg15, Fpcesr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,fpcesr::Reg15, Fpcesr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Fpcesr {
    #[inline(always)]
    fn default() -> Fpcesr {
        <crate::RegValueT<Fpcesr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fpcesr {
    pub struct Feg0_SPEC;
    pub type Feg0 = crate::EnumBitfieldStruct<u8, Feg0_SPEC>;
    impl Feg0 {
        #[doc = "0 No falling edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Falling edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Feg1_SPEC;
    pub type Feg1 = crate::EnumBitfieldStruct<u8, Feg1_SPEC>;
    impl Feg1 {
        #[doc = "0 No falling edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Falling edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Feg2_SPEC;
    pub type Feg2 = crate::EnumBitfieldStruct<u8, Feg2_SPEC>;
    impl Feg2 {
        #[doc = "0 No falling edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Falling edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Feg3_SPEC;
    pub type Feg3 = crate::EnumBitfieldStruct<u8, Feg3_SPEC>;
    impl Feg3 {
        #[doc = "0 No falling edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Falling edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Feg4_SPEC;
    pub type Feg4 = crate::EnumBitfieldStruct<u8, Feg4_SPEC>;
    impl Feg4 {
        #[doc = "0 No falling edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Falling edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Feg5_SPEC;
    pub type Feg5 = crate::EnumBitfieldStruct<u8, Feg5_SPEC>;
    impl Feg5 {
        #[doc = "0 No falling edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Falling edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Feg6_SPEC;
    pub type Feg6 = crate::EnumBitfieldStruct<u8, Feg6_SPEC>;
    impl Feg6 {
        #[doc = "0 No falling edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Falling edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Feg7_SPEC;
    pub type Feg7 = crate::EnumBitfieldStruct<u8, Feg7_SPEC>;
    impl Feg7 {
        #[doc = "0 No falling edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Falling edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Feg8_SPEC;
    pub type Feg8 = crate::EnumBitfieldStruct<u8, Feg8_SPEC>;
    impl Feg8 {
        #[doc = "0 No falling edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Falling edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Feg9_SPEC;
    pub type Feg9 = crate::EnumBitfieldStruct<u8, Feg9_SPEC>;
    impl Feg9 {
        #[doc = "0 No falling edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Falling edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Feg10_SPEC;
    pub type Feg10 = crate::EnumBitfieldStruct<u8, Feg10_SPEC>;
    impl Feg10 {
        #[doc = "0 No falling edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Falling edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Feg11_SPEC;
    pub type Feg11 = crate::EnumBitfieldStruct<u8, Feg11_SPEC>;
    impl Feg11 {
        #[doc = "0 No falling edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Falling edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Feg12_SPEC;
    pub type Feg12 = crate::EnumBitfieldStruct<u8, Feg12_SPEC>;
    impl Feg12 {
        #[doc = "0 No falling edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Falling edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Feg13_SPEC;
    pub type Feg13 = crate::EnumBitfieldStruct<u8, Feg13_SPEC>;
    impl Feg13 {
        #[doc = "0 No falling edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Falling edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Feg14_SPEC;
    pub type Feg14 = crate::EnumBitfieldStruct<u8, Feg14_SPEC>;
    impl Feg14 {
        #[doc = "0 No falling edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Falling edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Feg15_SPEC;
    pub type Feg15 = crate::EnumBitfieldStruct<u8, Feg15_SPEC>;
    impl Feg15 {
        #[doc = "0 No falling edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Falling edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Reg0_SPEC;
    pub type Reg0 = crate::EnumBitfieldStruct<u8, Reg0_SPEC>;
    impl Reg0 {
        #[doc = "0 No rising edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Rising edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Reg1_SPEC;
    pub type Reg1 = crate::EnumBitfieldStruct<u8, Reg1_SPEC>;
    impl Reg1 {
        #[doc = "0 No rising edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Rising edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Reg2_SPEC;
    pub type Reg2 = crate::EnumBitfieldStruct<u8, Reg2_SPEC>;
    impl Reg2 {
        #[doc = "0 No rising edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Rising edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Reg3_SPEC;
    pub type Reg3 = crate::EnumBitfieldStruct<u8, Reg3_SPEC>;
    impl Reg3 {
        #[doc = "0 No rising edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Rising edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Reg4_SPEC;
    pub type Reg4 = crate::EnumBitfieldStruct<u8, Reg4_SPEC>;
    impl Reg4 {
        #[doc = "0 No rising edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Rising edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Reg5_SPEC;
    pub type Reg5 = crate::EnumBitfieldStruct<u8, Reg5_SPEC>;
    impl Reg5 {
        #[doc = "0 No rising edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Rising edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Reg6_SPEC;
    pub type Reg6 = crate::EnumBitfieldStruct<u8, Reg6_SPEC>;
    impl Reg6 {
        #[doc = "0 No rising edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Rising edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Reg7_SPEC;
    pub type Reg7 = crate::EnumBitfieldStruct<u8, Reg7_SPEC>;
    impl Reg7 {
        #[doc = "0 No rising edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Rising edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Reg8_SPEC;
    pub type Reg8 = crate::EnumBitfieldStruct<u8, Reg8_SPEC>;
    impl Reg8 {
        #[doc = "0 No rising edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Rising edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Reg9_SPEC;
    pub type Reg9 = crate::EnumBitfieldStruct<u8, Reg9_SPEC>;
    impl Reg9 {
        #[doc = "0 No rising edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Rising edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Reg10_SPEC;
    pub type Reg10 = crate::EnumBitfieldStruct<u8, Reg10_SPEC>;
    impl Reg10 {
        #[doc = "0 No rising edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Rising edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Reg11_SPEC;
    pub type Reg11 = crate::EnumBitfieldStruct<u8, Reg11_SPEC>;
    impl Reg11 {
        #[doc = "0 No rising edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Rising edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Reg12_SPEC;
    pub type Reg12 = crate::EnumBitfieldStruct<u8, Reg12_SPEC>;
    impl Reg12 {
        #[doc = "0 No rising edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Rising edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Reg13_SPEC;
    pub type Reg13 = crate::EnumBitfieldStruct<u8, Reg13_SPEC>;
    impl Reg13 {
        #[doc = "0 No rising edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Rising edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Reg14_SPEC;
    pub type Reg14 = crate::EnumBitfieldStruct<u8, Reg14_SPEC>;
    impl Reg14 {
        #[doc = "0 No rising edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Rising edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Reg15_SPEC;
    pub type Reg15 = crate::EnumBitfieldStruct<u8, Reg15_SPEC>;
    impl Reg15 {
        #[doc = "0 No rising edge of glitch detected during filtering"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Rising edge of glitch detected during filtering"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FpctiMk_SPEC;
impl crate::sealed::RegSpec for FpctiMk_SPEC {
    type DataType = u32;
}
#[doc = "IOM Filter and Prescaler Channel Timer Register 0\n resetvalue={Application Reset:0x0}"]
pub type FpctiMk = crate::RegValueT<FpctiMk_SPEC>;

impl FpctiMk {
    #[doc = "Timer Value of Filter and Prescaler Channel k   TIM. This bit bitfield shows the values of the timer of the Filter and Prescaler Channel. Writing to TIM will result in its content being set to its reset value."]
    #[inline(always)]
    pub fn tim(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, FpctiMk_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, FpctiMk_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for FpctiMk {
    #[inline(always)]
    fn default() -> FpctiMk {
        <crate::RegValueT<FpctiMk_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtmexr_SPEC;
impl crate::sealed::RegSpec for Gtmexr_SPEC {
    type DataType = u32;
}
#[doc = "IOM GTM Input EXOR Combiner Selection Register\n resetvalue={Application Reset:0x0}"]
pub type Gtmexr = crate::RegValueT<Gtmexr_SPEC>;

impl Gtmexr {
    #[doc = "GTM input 0 selection for EXOR combiner   EN0"]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, gtmexr::En0, Gtmexr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,gtmexr::En0, Gtmexr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTM input 1 selection for EXOR combiner   EN1"]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, gtmexr::En1, Gtmexr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,gtmexr::En1, Gtmexr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTM input 2 selection for EXOR combiner   EN2"]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, gtmexr::En2, Gtmexr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,gtmexr::En2, Gtmexr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTM input 3 selection for EXOR combiner   EN3"]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, gtmexr::En3, Gtmexr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,gtmexr::En3, Gtmexr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTM input 4 selection for EXOR combiner   EN4"]
    #[inline(always)]
    pub fn en4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, gtmexr::En4, Gtmexr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,gtmexr::En4, Gtmexr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTM input 5 selection for EXOR combiner   EN5"]
    #[inline(always)]
    pub fn en5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, gtmexr::En5, Gtmexr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,gtmexr::En5, Gtmexr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTM input 6 selection for EXOR combiner   EN6"]
    #[inline(always)]
    pub fn en6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, gtmexr::En6, Gtmexr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,gtmexr::En6, Gtmexr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTM input 7 selection for EXOR combiner   EN7"]
    #[inline(always)]
    pub fn en7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, gtmexr::En7, Gtmexr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,gtmexr::En7, Gtmexr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gtmexr {
    #[inline(always)]
    fn default() -> Gtmexr {
        <crate::RegValueT<Gtmexr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtmexr {
    pub struct En0_SPEC;
    pub type En0 = crate::EnumBitfieldStruct<u8, En0_SPEC>;
    impl En0 {
        #[doc = "0 Input not        selected for EXOR combiner."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Input selected        for EXOR combiner."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En1_SPEC;
    pub type En1 = crate::EnumBitfieldStruct<u8, En1_SPEC>;
    impl En1 {
        #[doc = "0 Input not        selected for EXOR combiner."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Input selected        for EXOR combiner."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En2_SPEC;
    pub type En2 = crate::EnumBitfieldStruct<u8, En2_SPEC>;
    impl En2 {
        #[doc = "0 Input not        selected for EXOR combiner."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Input selected        for EXOR combiner."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En3_SPEC;
    pub type En3 = crate::EnumBitfieldStruct<u8, En3_SPEC>;
    impl En3 {
        #[doc = "0 Input not        selected for EXOR combiner."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Input selected        for EXOR combiner."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En4_SPEC;
    pub type En4 = crate::EnumBitfieldStruct<u8, En4_SPEC>;
    impl En4 {
        #[doc = "0 Input not        selected for EXOR combiner."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Input selected        for EXOR combiner."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En5_SPEC;
    pub type En5 = crate::EnumBitfieldStruct<u8, En5_SPEC>;
    impl En5 {
        #[doc = "0 Input not        selected for EXOR combiner."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Input selected        for EXOR combiner."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En6_SPEC;
    pub type En6 = crate::EnumBitfieldStruct<u8, En6_SPEC>;
    impl En6 {
        #[doc = "0 Input not selected for EXOR combiner."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Input selected for EXOR combiner."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En7_SPEC;
    pub type En7 = crate::EnumBitfieldStruct<u8, En7_SPEC>;
    impl En7 {
        #[doc = "0 Input not selected for EXOR combiner."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Input selected for EXOR combiner."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "IOM Identification Register\n resetvalue={Application Reset:0x0CCC002}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MOD REV. MOD REV defines the Module revision number."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MOD TYPE. This bit field defines the module as a 32 bit module  C0 H"]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MOD NUM. This bit field defines the identification number for the IOM."]
    #[inline(always)]
    pub fn mod_num(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Id_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Id {
    #[inline(always)]
    fn default() -> Id {
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(13418498)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krst0_SPEC;
impl crate::sealed::RegSpec for Krst0_SPEC {
    type DataType = u32;
}
#[doc = "IOM Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
pub type Krst0 = crate::RegValueT<Krst0_SPEC>;

impl Krst0 {
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel registers are set. The RST bit will be cleared  re set to   xb4 0  xb4   by the BPI FPI after the kernel reset was executed."]
    #[inline(always)]
    pub fn rst(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, krst0::Rst, Krst0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,krst0::Rst, Krst0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Kernel Reset Status   RSTSTAT. This bit indicates wether a kernel reset was executed or not. This bit is set by the BPI FPI after the execution of a kernel reset in the same clock cycle both reset bits. This bit can be cleared by writing with   xb4 1  xb4  to the CLR bit in the related KRSTCLR register."]
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
        #[doc = "0 No kernel reset        was requested"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A kernel reset        was requested"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rststat_SPEC;
    pub type Rststat = crate::EnumBitfieldStruct<u8, Rststat_SPEC>;
    impl Rststat {
        #[doc = "0 No kernel reset        was executed"]
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
#[doc = "IOM Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
pub type Krst1 = crate::RegValueT<Krst1_SPEC>;

impl Krst1 {
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel reset registers is set. The RST bit will be cleared  re set to   xb4 0  xb4   by the BPI FPI after the kernel reset was executed."]
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
#[doc = "IOM Kernel Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
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
        #[doc = "1 Clear Kernel        Reset Status KRST0.RSTSTAT"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LamcfGm_SPEC;
impl crate::sealed::RegSpec for LamcfGm_SPEC {
    type DataType = u32;
}
#[doc = "IOM Logic Analyzer Module Configuration Register 0\n resetvalue={Application Reset:0x0}"]
pub type LamcfGm = crate::RegValueT<LamcfGm_SPEC>;

impl LamcfGm {
    #[doc = "Invert Reference LAM block m   IVR. This bit field determines whether the reference signal from the FPC  reference channel  is inverted or not."]
    #[inline(always)]
    pub fn ivr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, lamcfgm::Ivr, LamcfGm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,lamcfgm::Ivr, LamcfGm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Invert Monitor LAM block m   IVM. This bit field determines whether the monitor signal from the FPC  monitor channel  is inverted or not."]
    #[inline(always)]
    pub fn ivm(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, lamcfgm::Ivm, LamcfGm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,lamcfgm::Ivm, LamcfGm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Monitor Source Select LAM block m   MOS. This bit field determines whether the monitor signal from the FPC  monitor channel  is sourced directly or compared  EXOR  x2019 d  with the reference signal from the FPC reference channel  for the event compare."]
    #[inline(always)]
    pub fn mos(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, lamcfgm::Mos, LamcfGm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,lamcfgm::Mos, LamcfGm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Runmode Select LAM block m   RMS. This bit field determines whether the event window generation is free running or gated with the monitor or reference."]
    #[inline(always)]
    pub fn rms(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, lamcfgm::Rms, LamcfGm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,lamcfgm::Rms, LamcfGm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Window Select LAM block m   EWS. This bit field determines whether the event window generation is from the monitor or reference signal."]
    #[inline(always)]
    pub fn ews(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, lamcfgm::Ews, LamcfGm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,lamcfgm::Ews, LamcfGm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disable Events LAM block m   DISEV. This bit field allows to suppress alarm outputs from LAM block m to the ECM. Except for sending alarms to the ECM  all other effects of an alarm condition being detected  for instance  setting LAMEWCm.CNT to the value of the counter  will still take place inside LAM block m."]
    #[inline(always)]
    pub fn disev(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, lamcfgm::Disev, LamcfGm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,lamcfgm::Disev, LamcfGm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Window Active Edge Selection LAM block m   EDS. This bit field determines which active edges of the monitor and reference signals are used for the event window generation."]
    #[inline(always)]
    pub fn eds(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, lamcfgm::Eds, LamcfGm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,lamcfgm::Eds, LamcfGm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Invert Event Window LAM block m   IVW. This bit field determines whether the event window polarity is inverted or not."]
    #[inline(always)]
    pub fn ivw(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, lamcfgm::Ivw, LamcfGm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,lamcfgm::Ivw, LamcfGm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Monitor Input Signal Selection LAM block m   MCS. This bit field determines which FPC mux block k monitor output signal is to be used for LAM block m."]
    #[inline(always)]
    pub fn mcs(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, lamcfgm::Mcs, LamcfGm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,lamcfgm::Mcs, LamcfGm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reference Input Signal Selection LAM block m   RCS. This bit field determines which FPC mux block k reference output signal is to be used for LAM block m."]
    #[inline(always)]
    pub fn rcs(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, lamcfgm::Rcs, LamcfGm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0xf,1,0,lamcfgm::Rcs, LamcfGm_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for LamcfGm {
    #[inline(always)]
    fn default() -> LamcfGm {
        <crate::RegValueT<LamcfGm_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lamcfgm {
    pub struct Ivr_SPEC;
    pub type Ivr = crate::EnumBitfieldStruct<u8, Ivr_SPEC>;
    impl Ivr {
        #[doc = "0 Don  8217 t invert        reference signal from FPC."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Invert        reference signal from FPC."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ivm_SPEC;
    pub type Ivm = crate::EnumBitfieldStruct<u8, Ivm_SPEC>;
    impl Ivm {
        #[doc = "0 Don  8217 t invert        monitor signal from FPC."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Invert monitor        signal from FPC."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mos_SPEC;
    pub type Mos = crate::EnumBitfieldStruct<u8, Mos_SPEC>;
    impl Mos {
        #[doc = "0 Monitor signal        is sourced directly from FPC monitor channel ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Monitor signal        is EXOR  8217 d with FPC reference channel ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rms_SPEC;
    pub type Rms = crate::EnumBitfieldStruct<u8, Rms_SPEC>;
    impl Rms {
        #[doc = "0 Event window generation is free running."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Event window generation is gated with the monitor or reference signal."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ews_SPEC;
    pub type Ews = crate::EnumBitfieldStruct<u8, Ews_SPEC>;
    impl Ews {
        #[doc = "0 Event window generation is determined from the reference signal."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Event window generation is determined from the monitor signal."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Disev_SPEC;
    pub type Disev = crate::EnumBitfieldStruct<u8, Disev_SPEC>;
    impl Disev {
        #[doc = "Events will be generated."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "No events will be sent form LAM to ECM. Except for sending alarms to the        ECM  all other effects of an alarm condition being detected  for        instance  setting LAMEWCm.CNT to the value of the counter  will still        take place inside LAM block m."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eds_SPEC;
    pub type Eds = crate::EnumBitfieldStruct<u8, Eds_SPEC>;
    impl Eds {
        #[doc = "xx00 Neither edge used to clear event window counter. 00xx Neither edge used to gate event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "xx00 Neither edge used to clear event window counter. 01xx Positive edge used to gate event generation."]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "xx00 Neither edge used to clear event window counter. 10xx Negative edge used to gate event generation."]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "xx00 Neither edge used to clear event window counter. 11xx Either edge used to gate event generation."]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "xx01 Positive edge used to clear event window counter. 00xx Neither edge used to gate event generation."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "xx01 Positive edge used to clear event window counter. 01xx Positive edge used to gate event generation."]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "xx01 Positive edge used to clear event window counter. 10xx Negative edge used to gate event generation."]
        pub const CONST_99: Self = Self::new(9);
        #[doc = "xx01 Positive edge used to clear event window counter. 11xx Either edge used to gate event generation."]
        pub const CONST_1313: Self = Self::new(13);
        #[doc = "xx10 Negative edge used to clear event window counter. 00xx Neither edge used to gate event generation."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "xx10 Negative edge used to clear event window counter. 01xx Positive edge used to gate event generation."]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "xx10 Negative edge used to clear event window counter. 10xx Negative edge used to gate event generation."]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "xx10 Negative edge used to clear event window counter. 11xx Either edge used to gate event generation."]
        pub const CONST_1414: Self = Self::new(14);
        #[doc = "xx11 Either edge used to clear event window counter. 00xx Neither edge used to gate event generation."]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "xx11 Either edge used to clear event window counter. 01xx Positive edge used to gate event generation."]
        pub const CONST_77: Self = Self::new(7);
        #[doc = "xx11 Either edge used to clear event window counter. 10xx Negative edge used to gate event generation."]
        pub const CONST_1111: Self = Self::new(11);
        #[doc = "xx11 Either edge used to clear event window counter. 11xx Either edge used to gate event generation."]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Ivw_SPEC;
    pub type Ivw = crate::EnumBitfieldStruct<u8, Ivw_SPEC>;
    impl Ivw {
        #[doc = "0 Event window non inverted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Event window inverted."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mcs_SPEC;
    pub type Mcs = crate::EnumBitfieldStruct<u8, Mcs_SPEC>;
    impl Mcs {
        #[doc = "0000 Monitor signal provided by FPC mux block 0."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "0001 Monitor signal provided by FPC mux block 1."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "0010 Monitor signal provided by FPC mux block 2."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "0011 Monitor signal provided by FPC mux block 3."]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "0100 Monitor signal provided by FPC mux block 4."]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "0101 Monitor signal provided by FPC mux block 5."]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "0110 Monitor signal provided by FPC mux block 6."]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "0111 Monitor signal provided by FPC mux block 7."]
        pub const CONST_77: Self = Self::new(7);
        #[doc = "1000 Monitor signal provided by FPC mux block 8."]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "1001 Monitor signal provided by FPC mux block 9."]
        pub const CONST_99: Self = Self::new(9);
        #[doc = "1010 Monitor signal provided by FPC mux block 10."]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "1011 Monitor signal provided by FPC mux block 11."]
        pub const CONST_1111: Self = Self::new(11);
        #[doc = "1100 Monitor signal provided by FPC mux block 12."]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "1101 Monitor signal provided by FPC mux block 13."]
        pub const CONST_1313: Self = Self::new(13);
        #[doc = "1110 Monitor signal provided by FPC mux block 14."]
        pub const CONST_1414: Self = Self::new(14);
        #[doc = "1111 Monitor signal provided by FPC mux block 15."]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Rcs_SPEC;
    pub type Rcs = crate::EnumBitfieldStruct<u8, Rcs_SPEC>;
    impl Rcs {
        #[doc = "0000 Reference signal provided by FPC mux block 0."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "0001 Reference signal provided by FPC mux block 1."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "0010 Reference signal provided by FPC mux block 2."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "0011 Reference signal provided by FPC mux block 3."]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "0100 Reference signal provided by FPC mux block 4."]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "0101 Reference signal provided by FPC mux block 5."]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "0110 Reference signal provided by FPC mux block 6."]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "0111 Reference signal provided by FPC mux block 7."]
        pub const CONST_77: Self = Self::new(7);
        #[doc = "1000 Reference signal provided by FPC mux block 8."]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "1001 Reference signal provided by FPC mux block 9."]
        pub const CONST_99: Self = Self::new(9);
        #[doc = "1010 Reference signal provided by FPC mux block 10."]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "1011 Reference signal provided by FPC mux block 11."]
        pub const CONST_1111: Self = Self::new(11);
        #[doc = "1100 Reference signal provided by FPC mux block 12."]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "1101 Reference signal provided by FPC mux block 13."]
        pub const CONST_1313: Self = Self::new(13);
        #[doc = "1110 Reference signal provided by FPC mux block 14."]
        pub const CONST_1414: Self = Self::new(14);
        #[doc = "1111 Reference signal provided by FPC mux block 15."]
        pub const CONST_1515: Self = Self::new(15);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LamewCm_SPEC;
impl crate::sealed::RegSpec for LamewCm_SPEC {
    type DataType = u32;
}
#[doc = "IOM Logic Analyzer Module Event Window Count Status Register 0\n resetvalue={Application Reset:0x0}"]
pub type LamewCm = crate::RegValueT<LamewCm_SPEC>;

impl LamewCm {
    #[doc = "Event Window Count Value LAM block m   CNT. The count value of the event window attained coincident with an event occurring."]
    #[inline(always)]
    pub fn cnt(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, LamewCm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffff,1,0,u32, LamewCm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Count Overflow Flag LAM block m   CNTO. This bit field provides the information wether the count has reached its maximum value."]
    #[inline(always)]
    pub fn cnto(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, lamewcm::Cnto, LamewCm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,lamewcm::Cnto, LamewCm_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for LamewCm {
    #[inline(always)]
    fn default() -> LamewCm {
        <crate::RegValueT<LamewCm_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lamewcm {
    pub struct Cnto_SPEC;
    pub type Cnto = crate::EnumBitfieldStruct<u8, Cnto_SPEC>;
    impl Cnto {
        #[doc = "The count value has not reached its maximum value since the previous        count clear."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "The count value has reached its maximum value since the previous count        clear."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LamewSm_SPEC;
impl crate::sealed::RegSpec for LamewSm_SPEC {
    type DataType = u32;
}
#[doc = "IOM Logic Analyzer Module Event Window Configuration Register 0\n resetvalue={Application Reset:0x0}"]
pub type LamewSm = crate::RegValueT<LamewSm_SPEC>;

impl LamewSm {
    #[doc = "Event Window Count Threshold   THR. This bit field contains the value of the counter at which the event window becomes active  before optional inversion ."]
    #[inline(always)]
    pub fn thr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, LamewSm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffff,1,0,u32, LamewSm_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for LamewSm {
    #[inline(always)]
    fn default() -> LamewSm {
        <crate::RegValueT<LamewSm_SPEC> as RegisterValue<_>>::new(0)
    }
}
