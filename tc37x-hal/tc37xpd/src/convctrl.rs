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
#[doc = r"CONVERTER"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Convctrl(pub(super) *mut u8);
unsafe impl core::marker::Send for Convctrl {}
unsafe impl core::marker::Sync for Convctrl {}
impl Convctrl {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "Converter Control Block Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ccctrl(&self) -> crate::common::Reg<self::Ccctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(124usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Flags Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn flags(&self) -> crate::common::Reg<self::Flags_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(224usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={PowerOn Reset:0x0FFC001}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst0(&self) -> crate::common::Reg<self::Krst0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst1(&self) -> crate::common::Reg<self::Krst1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }

    #[doc = "Kernel Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krstclr(&self) -> crate::common::Reg<self::Krstclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }

    #[doc = "Maximum Value Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn max(&self) -> crate::common::Reg<self::Max_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(212usize)) }
    }

    #[doc = "Minimum Value Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn min(&self) -> crate::common::Reg<self::Min_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(208usize)) }
    }

    #[doc = "Miscellaneous Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn misc(&self) -> crate::common::Reg<self::Misc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(228usize)) }
    }

    #[doc = "OCDS Control and Status Register\n resetvalue={PowerOn Reset:0x0,Debug Clear:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }

    #[doc = "Push Pull Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pctrl(&self) -> crate::common::Reg<self::Pctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(232usize)) }
    }

    #[doc = "Phase Synchronizer Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn phscfg(&self) -> crate::common::Reg<self::Phscfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "Phase Synchronizer Safety Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn phssfty(&self) -> crate::common::Reg<self::Phssfty_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }

    #[doc = "P I Regulator Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn picfg(&self) -> crate::common::Reg<self::Picfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(192usize)) }
    }

    #[doc = "Sinewave Gen. Config. Register  A Factor\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sgcfga(&self) -> crate::common::Reg<self::Sgcfga_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(196usize)) }
    }

    #[doc = "Sinewave Gen. Config. Register  K Factor\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sgcfgk(&self) -> crate::common::Reg<self::Sgcfgk_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(200usize)) }
    }

    #[doc = "Shift Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn shift(&self) -> crate::common::Reg<self::Shift_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(216usize)) }
    }

    #[doc = "Slope Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn slope(&self) -> crate::common::Reg<self::Slope_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(204usize)) }
    }

    #[doc = "Test Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn testcfg(&self) -> crate::common::Reg<self::Testcfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(220usize)) }
    }

    #[doc = "Voltage Regulator Configuration Register\n resetvalue={PowerOn Reset:0x0C3,CFS Value:0x0C3}"]
    #[inline(always)]
    pub const fn vrcfg(&self) -> crate::common::Reg<self::Vrcfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }

    #[doc = "Voltage Regulator Status Register\n resetvalue={Application Reset:0x1}"]
    #[inline(always)]
    pub const fn vrstat(&self) -> crate::common::Reg<self::Vrstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(164usize)) }
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
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, accen0::En0, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,accen0::En0, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, accen0::En1, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,accen0::En1, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, accen0::En2, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,accen0::En2, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, accen0::En3, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,accen0::En3, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, accen0::En4, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,accen0::En4, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, accen0::En5, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,accen0::En5, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, accen0::En6, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,accen0::En6, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, accen0::En7, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,accen0::En7, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, accen0::En8, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,accen0::En8, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, accen0::En9, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,accen0::En9, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, accen0::En10, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,accen0::En10, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, accen0::En11, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,accen0::En11, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, accen0::En12, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,accen0::En12, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, accen0::En13, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,accen0::En13, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, accen0::En14, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,accen0::En14, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, accen0::En15, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,accen0::En15, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, accen0::En16, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,accen0::En16, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, accen0::En17, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,accen0::En17, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, accen0::En18, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,accen0::En18, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, accen0::En19, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,accen0::En19, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, accen0::En20, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,accen0::En20, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, accen0::En21, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,accen0::En21, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, accen0::En22, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,accen0::En22, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, accen0::En23, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,accen0::En23, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, accen0::En24, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,accen0::En24, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, accen0::En25, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,accen0::En25, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, accen0::En26, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,accen0::En26, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, accen0::En27, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,accen0::En27, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, accen0::En28, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,accen0::En28, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, accen0::En29, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,accen0::En29, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, accen0::En30, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,accen0::En30, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
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
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En1_SPEC;
    pub type En1 = crate::EnumBitfieldStruct<u8, En1_SPEC>;
    impl En1 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En2_SPEC;
    pub type En2 = crate::EnumBitfieldStruct<u8, En2_SPEC>;
    impl En2 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En3_SPEC;
    pub type En3 = crate::EnumBitfieldStruct<u8, En3_SPEC>;
    impl En3 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En4_SPEC;
    pub type En4 = crate::EnumBitfieldStruct<u8, En4_SPEC>;
    impl En4 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En5_SPEC;
    pub type En5 = crate::EnumBitfieldStruct<u8, En5_SPEC>;
    impl En5 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En6_SPEC;
    pub type En6 = crate::EnumBitfieldStruct<u8, En6_SPEC>;
    impl En6 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En7_SPEC;
    pub type En7 = crate::EnumBitfieldStruct<u8, En7_SPEC>;
    impl En7 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En8_SPEC;
    pub type En8 = crate::EnumBitfieldStruct<u8, En8_SPEC>;
    impl En8 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En9_SPEC;
    pub type En9 = crate::EnumBitfieldStruct<u8, En9_SPEC>;
    impl En9 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En10_SPEC;
    pub type En10 = crate::EnumBitfieldStruct<u8, En10_SPEC>;
    impl En10 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En11_SPEC;
    pub type En11 = crate::EnumBitfieldStruct<u8, En11_SPEC>;
    impl En11 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En12_SPEC;
    pub type En12 = crate::EnumBitfieldStruct<u8, En12_SPEC>;
    impl En12 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En13_SPEC;
    pub type En13 = crate::EnumBitfieldStruct<u8, En13_SPEC>;
    impl En13 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En14_SPEC;
    pub type En14 = crate::EnumBitfieldStruct<u8, En14_SPEC>;
    impl En14 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En15_SPEC;
    pub type En15 = crate::EnumBitfieldStruct<u8, En15_SPEC>;
    impl En15 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En16_SPEC;
    pub type En16 = crate::EnumBitfieldStruct<u8, En16_SPEC>;
    impl En16 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En17_SPEC;
    pub type En17 = crate::EnumBitfieldStruct<u8, En17_SPEC>;
    impl En17 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En18_SPEC;
    pub type En18 = crate::EnumBitfieldStruct<u8, En18_SPEC>;
    impl En18 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En19_SPEC;
    pub type En19 = crate::EnumBitfieldStruct<u8, En19_SPEC>;
    impl En19 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En20_SPEC;
    pub type En20 = crate::EnumBitfieldStruct<u8, En20_SPEC>;
    impl En20 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En21_SPEC;
    pub type En21 = crate::EnumBitfieldStruct<u8, En21_SPEC>;
    impl En21 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En22_SPEC;
    pub type En22 = crate::EnumBitfieldStruct<u8, En22_SPEC>;
    impl En22 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En23_SPEC;
    pub type En23 = crate::EnumBitfieldStruct<u8, En23_SPEC>;
    impl En23 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En24_SPEC;
    pub type En24 = crate::EnumBitfieldStruct<u8, En24_SPEC>;
    impl En24 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En25_SPEC;
    pub type En25 = crate::EnumBitfieldStruct<u8, En25_SPEC>;
    impl En25 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En26_SPEC;
    pub type En26 = crate::EnumBitfieldStruct<u8, En26_SPEC>;
    impl En26 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En27_SPEC;
    pub type En27 = crate::EnumBitfieldStruct<u8, En27_SPEC>;
    impl En27 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En28_SPEC;
    pub type En28 = crate::EnumBitfieldStruct<u8, En28_SPEC>;
    impl En28 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En29_SPEC;
    pub type En29 = crate::EnumBitfieldStruct<u8, En29_SPEC>;
    impl En29 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En30_SPEC;
    pub type En30 = crate::EnumBitfieldStruct<u8, En30_SPEC>;
    impl En30 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En31_SPEC;
    pub type En31 = crate::EnumBitfieldStruct<u8, En31_SPEC>;
    impl En31 {
        #[doc = "No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccctrl_SPEC;
impl crate::sealed::RegSpec for Ccctrl_SPEC {
    type DataType = u32;
}
#[doc = "Converter Control Block Control Register\n resetvalue={Application Reset:0x0}"]
pub type Ccctrl = crate::RegValueT<Ccctrl_SPEC>;

impl Ccctrl {
    #[doc = "Test Control   TC. Not listed combinations write protect CONVCTRL registers."]
    #[inline(always)]
    pub fn tc(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, ccctrl::Tc, Ccctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0xf,1,0,ccctrl::Tc, Ccctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ccctrl {
    #[inline(always)]
    fn default() -> Ccctrl {
        <crate::RegValueT<Ccctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccctrl {
    pub struct Tc_SPEC;
    pub type Tc = crate::EnumBitfieldStruct<u8, Tc_SPEC>;
    impl Tc {
        #[doc = "Access to CONVCTRL registers is enabled"]
        pub const CONST_1111: Self = Self::new(11);
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
    pub fn disr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, clc::Disr, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,clc::Disr, Clc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Module Disable Status Bit   DISS"]
    #[inline(always)]
    pub fn diss(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, clc::Diss, Clc_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,clc::Diss, Clc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Sleep Mode Enable Control   EDIS. Used to control module  8217 s reaction to sleep mode."]
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
        #[doc = "On request  enable the module clock"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Off request  stop the module clock"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Diss_SPEC;
    pub type Diss = crate::EnumBitfieldStruct<u8, Diss_SPEC>;
    impl Diss {
        #[doc = "Module clock is enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Off  module is not clocked"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Edis_SPEC;
    pub type Edis = crate::EnumBitfieldStruct<u8, Edis_SPEC>;
    impl Edis {
        #[doc = "Sleep mode request is enabled and functional"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module disregards the sleep mode control signal"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flags_SPEC;
impl crate::sealed::RegSpec for Flags_SPEC {
    type DataType = u32;
}
#[doc = "Flags Register\n resetvalue={Application Reset:0x0}"]
pub type Flags = crate::RegValueT<Flags_SPEC>;

impl Flags {
    #[doc = "Shift Register Overrun   SHOR. Error flag for data overrun"]
    #[inline(always)]
    pub fn shor(self) -> crate::common::RegisterFieldBool<0, 1, 0, Flags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Flags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Shift Register Underrun   SHUR. Error flag for data underrun"]
    #[inline(always)]
    pub fn shur(self) -> crate::common::RegisterFieldBool<1, 1, 0, Flags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Flags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Flags {
    #[inline(always)]
    fn default() -> Flags {
        <crate::RegValueT<Flags_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={PowerOn Reset:0x0FFC001}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision   MOD REV. Indicates the revision number of the implementation. This information        depends on the design step."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MOD TYPE. This internal marker is fixed to C0 ."]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number   MOD NUMBER. Indicates the module identification number   00FF   CONVCTRL"]
    #[inline(always)]
    pub fn mod_number(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Id_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Id {
    #[inline(always)]
    fn default() -> Id {
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(16760833)
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
    #[doc = "Kernel Reset   RST. Request a kernel reset. The reset is executed if the reset bits of both        kernel reset registers are set. RST is cleared after the kernel reset was executed."]
    #[inline(always)]
    pub fn rst(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, krst0::Rst, Krst0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,krst0::Rst, Krst0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Kernel Reset Status   RSTSTAT. Indicates an executed kernel reset. RSTSTAT is set after the execution        of a kernel reset in the same clock cycle in which the reset bits are        cleared. Clear RSTSTAT by setting bit CLR in register KRSTCLR."]
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
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "A kernel reset was requested"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rststat_SPEC;
    pub type Rststat = crate::EnumBitfieldStruct<u8, Rststat_SPEC>;
    impl Rststat {
        #[doc = "No kernel reset was executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Kernel reset was executed"]
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
    #[doc = "Kernel Reset   RST. Request a kernel reset. The reset is executed if the reset bits of both        kernel reset registers are set. RST is cleared after the kernel reset was executed."]
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
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "A kernel reset was requested"]
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
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Clear Kernel Reset Status KRST0.RSTSTAT"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Max_SPEC;
impl crate::sealed::RegSpec for Max_SPEC {
    type DataType = u32;
}
#[doc = "Maximum Value Register\n resetvalue={Application Reset:0x0}"]
pub type Max = crate::RegValueT<Max_SPEC>;

impl Max {
    #[doc = "Ramp Generator Maxvalue   MAXVALUE. Maximum Value"]
    #[inline(always)]
    pub fn maxvalue(
        self,
    ) -> crate::common::RegisterField<0, 0xfffffff, 1, 0, u32, Max_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfffffff,1,0,u32, Max_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Max {
    #[inline(always)]
    fn default() -> Max {
        <crate::RegValueT<Max_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Min_SPEC;
impl crate::sealed::RegSpec for Min_SPEC {
    type DataType = u32;
}
#[doc = "Minimum Value Register\n resetvalue={Application Reset:0x0}"]
pub type Min = crate::RegValueT<Min_SPEC>;

impl Min {
    #[doc = "Ramp Generator Minvalue   MINVALUE. Minimum Value"]
    #[inline(always)]
    pub fn minvalue(
        self,
    ) -> crate::common::RegisterField<0, 0xfffffff, 1, 0, u32, Min_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfffffff,1,0,u32, Min_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Min {
    #[inline(always)]
    fn default() -> Min {
        <crate::RegValueT<Min_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misc_SPEC;
impl crate::sealed::RegSpec for Misc_SPEC {
    type DataType = u32;
}
#[doc = "Miscellaneous Control Register\n resetvalue={Application Reset:0x0}"]
pub type Misc = crate::RegValueT<Misc_SPEC>;

impl Misc {
    #[doc = "Ramp Generator Wait Time   WAIT. Wait time before slope starts"]
    #[inline(always)]
    pub fn wait(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Misc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Misc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Ramp Generator Operation Control   STAY"]
    #[inline(always)]
    pub fn stay(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, misc::Stay, Misc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1,1,0,misc::Stay, Misc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sinewave Generator Frequency   SWGK. Bits 36 32 of the 37 bit value. Lower bits see SGCFGK."]
    #[inline(always)]
    pub fn swgk(
        self,
    ) -> crate::common::RegisterField<17, 0x1f, 1, 0, u8, Misc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<17,0x1f,1,0,u8, Misc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Push Pull Switch Control"]
    #[inline(always)]
    pub fn ppsw(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, misc::Ppsw, Misc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1,1,0,misc::Ppsw, Misc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Analog ADC Test Connection Enable"]
    #[inline(always)]
    pub fn atcen(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, misc::Atcen, Misc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,misc::Atcen, Misc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Misc {
    #[inline(always)]
    fn default() -> Misc {
        <crate::RegValueT<Misc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod misc {
    pub struct Stay_SPEC;
    pub type Stay = crate::EnumBitfieldStruct<u8, Stay_SPEC>;
    impl Stay {
        #[doc = "Repetitive slopes"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Single slope"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ppsw_SPEC;
    pub type Ppsw = crate::EnumBitfieldStruct<u8, Ppsw_SPEC>;
    impl Ppsw {
        #[doc = "Switch open"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Switch is closed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Atcen_SPEC;
    pub type Atcen = crate::EnumBitfieldStruct<u8, Atcen_SPEC>;
    impl Atcen {
        #[doc = "ATC disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Analog Test Connection enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ocs_SPEC;
impl crate::sealed::RegSpec for Ocs_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Control and Status Register\n resetvalue={PowerOn Reset:0x0,Debug Clear:0x0}"]
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
        #[doc = "Will not suspend"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Suspend mode 1  Stop generating synchronization pulses"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "Suspend mode 2  Disable the phase synchronizer.  Synchronization signal constantly active"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Sussta_SPEC;
    pub type Sussta = crate::EnumBitfieldStruct<u8, Sussta_SPEC>;
    impl Sussta {
        #[doc = "Module is not  yet  suspended"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is suspended"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pctrl_SPEC;
impl crate::sealed::RegSpec for Pctrl_SPEC {
    type DataType = u32;
}
#[doc = "Push Pull Control Register\n resetvalue={Application Reset:0x0}"]
pub type Pctrl = crate::RegValueT<Pctrl_SPEC>;

impl Pctrl {
    #[doc = "Blanking Enable for Group 11"]
    #[inline(always)]
    pub fn pbl0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, pctrl::Pbl0, Pctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,pctrl::Pbl0, Pctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Blanking Enable for Group 11"]
    #[inline(always)]
    pub fn pbl1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, pctrl::Pbl1, Pctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,pctrl::Pbl1, Pctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Blanking Enable for Group 11"]
    #[inline(always)]
    pub fn pbl2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, pctrl::Pbl2, Pctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,pctrl::Pbl2, Pctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Blanking Enable for Group 11"]
    #[inline(always)]
    pub fn pbl3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, pctrl::Pbl3, Pctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,pctrl::Pbl3, Pctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Blanking Enable for Group 11"]
    #[inline(always)]
    pub fn pbl4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, pctrl::Pbl4, Pctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,pctrl::Pbl4, Pctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Blanking Enable for Group 11"]
    #[inline(always)]
    pub fn pbl5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, pctrl::Pbl5, Pctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,pctrl::Pbl5, Pctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Blanking Enable for Group 11"]
    #[inline(always)]
    pub fn pbl6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, pctrl::Pbl6, Pctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,pctrl::Pbl6, Pctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Blanking Enable for Group 11"]
    #[inline(always)]
    pub fn pbl7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, pctrl::Pbl7, Pctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,pctrl::Pbl7, Pctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Blanking Enable for Group 11"]
    #[inline(always)]
    pub fn pbl8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, pctrl::Pbl8, Pctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,pctrl::Pbl8, Pctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Blanking Enable for Group 11"]
    #[inline(always)]
    pub fn pbl9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, pctrl::Pbl9, Pctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,pctrl::Pbl9, Pctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Blanking Enable for Group 11"]
    #[inline(always)]
    pub fn pbl10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, pctrl::Pbl10, Pctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,pctrl::Pbl10, Pctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Blanking Enable for Group 11"]
    #[inline(always)]
    pub fn pbl11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, pctrl::Pbl11, Pctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,pctrl::Pbl11, Pctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Push Pull Blanking Delay  End of Signal. Prolongues the blanking output signal by a configurable number of clock        cycles."]
    #[inline(always)]
    pub fn bldele(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, pctrl::Bldele, Pctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,pctrl::Bldele, Pctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Blanking Enable for Fast Compare Channel 7"]
    #[inline(always)]
    pub fn pblf0(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, pctrl::Pblf0, Pctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,pctrl::Pblf0, Pctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Blanking Enable for Fast Compare Channel 7"]
    #[inline(always)]
    pub fn pblf1(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, pctrl::Pblf1, Pctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,pctrl::Pblf1, Pctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Blanking Enable for Fast Compare Channel 7"]
    #[inline(always)]
    pub fn pblf2(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, pctrl::Pblf2, Pctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,pctrl::Pblf2, Pctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Blanking Enable for Fast Compare Channel 7"]
    #[inline(always)]
    pub fn pblf3(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, pctrl::Pblf3, Pctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,pctrl::Pblf3, Pctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Blanking Enable for Fast Compare Channel 7"]
    #[inline(always)]
    pub fn pblf4(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, pctrl::Pblf4, Pctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,pctrl::Pblf4, Pctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Blanking Enable for Fast Compare Channel 7"]
    #[inline(always)]
    pub fn pblf5(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, pctrl::Pblf5, Pctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,pctrl::Pblf5, Pctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Blanking Enable for Fast Compare Channel 7"]
    #[inline(always)]
    pub fn pblf6(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, pctrl::Pblf6, Pctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,pctrl::Pblf6, Pctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Blanking Enable for Fast Compare Channel 7"]
    #[inline(always)]
    pub fn pblf7(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, pctrl::Pblf7, Pctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,pctrl::Pblf7, Pctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Push Pull Blanking Delay  Start of Signal. Number of clock cycles until the push pull blanking signal is activated in relation to the begin of the sample phase. See table below."]
    #[inline(always)]
    pub fn bldels(
        self,
    ) -> crate::common::RegisterField<26, 0x3f, 1, 0, u8, Pctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3f,1,0,u8, Pctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pctrl {
    #[inline(always)]
    fn default() -> Pctrl {
        <crate::RegValueT<Pctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pctrl {
    pub struct Pbl0_SPEC;
    pub type Pbl0 = crate::EnumBitfieldStruct<u8, Pbl0_SPEC>;
    impl Pbl0 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Disable charge pump while this group is sampling"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pbl1_SPEC;
    pub type Pbl1 = crate::EnumBitfieldStruct<u8, Pbl1_SPEC>;
    impl Pbl1 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Disable charge pump while this group is sampling"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pbl2_SPEC;
    pub type Pbl2 = crate::EnumBitfieldStruct<u8, Pbl2_SPEC>;
    impl Pbl2 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Disable charge pump while this group is sampling"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pbl3_SPEC;
    pub type Pbl3 = crate::EnumBitfieldStruct<u8, Pbl3_SPEC>;
    impl Pbl3 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Disable charge pump while this group is sampling"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pbl4_SPEC;
    pub type Pbl4 = crate::EnumBitfieldStruct<u8, Pbl4_SPEC>;
    impl Pbl4 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Disable charge pump while this group is sampling"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pbl5_SPEC;
    pub type Pbl5 = crate::EnumBitfieldStruct<u8, Pbl5_SPEC>;
    impl Pbl5 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Disable charge pump while this group is sampling"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pbl6_SPEC;
    pub type Pbl6 = crate::EnumBitfieldStruct<u8, Pbl6_SPEC>;
    impl Pbl6 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Disable charge pump while this group is sampling"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pbl7_SPEC;
    pub type Pbl7 = crate::EnumBitfieldStruct<u8, Pbl7_SPEC>;
    impl Pbl7 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Disable charge pump while this group is sampling"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pbl8_SPEC;
    pub type Pbl8 = crate::EnumBitfieldStruct<u8, Pbl8_SPEC>;
    impl Pbl8 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Disable charge pump while this group is sampling"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pbl9_SPEC;
    pub type Pbl9 = crate::EnumBitfieldStruct<u8, Pbl9_SPEC>;
    impl Pbl9 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Disable charge pump while this group is sampling"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pbl10_SPEC;
    pub type Pbl10 = crate::EnumBitfieldStruct<u8, Pbl10_SPEC>;
    impl Pbl10 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Disable charge pump while this group is sampling"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pbl11_SPEC;
    pub type Pbl11 = crate::EnumBitfieldStruct<u8, Pbl11_SPEC>;
    impl Pbl11 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Disable charge pump while this group is sampling"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Bldele_SPEC;
    pub type Bldele = crate::EnumBitfieldStruct<u8, Bldele_SPEC>;
    impl Bldele {
        #[doc = "Off  no additional delay"]
        pub const CONST_000: Self = Self::new(0);
        #[doc = "Add 1 clock cycle to the blanking signal"]
        pub const CONST_011: Self = Self::new(1);
        #[doc = "Add 2 clock cycles to the blanking signal"]
        pub const CONST_102: Self = Self::new(2);
        #[doc = "Disable the blanking signal"]
        pub const CONST_113: Self = Self::new(3);
    }
    pub struct Pblf0_SPEC;
    pub type Pblf0 = crate::EnumBitfieldStruct<u8, Pblf0_SPEC>;
    impl Pblf0 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Disable charge pump while this group is sampling"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pblf1_SPEC;
    pub type Pblf1 = crate::EnumBitfieldStruct<u8, Pblf1_SPEC>;
    impl Pblf1 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Disable charge pump while this group is sampling"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pblf2_SPEC;
    pub type Pblf2 = crate::EnumBitfieldStruct<u8, Pblf2_SPEC>;
    impl Pblf2 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Disable charge pump while this group is sampling"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pblf3_SPEC;
    pub type Pblf3 = crate::EnumBitfieldStruct<u8, Pblf3_SPEC>;
    impl Pblf3 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Disable charge pump while this group is sampling"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pblf4_SPEC;
    pub type Pblf4 = crate::EnumBitfieldStruct<u8, Pblf4_SPEC>;
    impl Pblf4 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Disable charge pump while this group is sampling"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pblf5_SPEC;
    pub type Pblf5 = crate::EnumBitfieldStruct<u8, Pblf5_SPEC>;
    impl Pblf5 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Disable charge pump while this group is sampling"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pblf6_SPEC;
    pub type Pblf6 = crate::EnumBitfieldStruct<u8, Pblf6_SPEC>;
    impl Pblf6 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Disable charge pump while this group is sampling"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pblf7_SPEC;
    pub type Pblf7 = crate::EnumBitfieldStruct<u8, Pblf7_SPEC>;
    impl Pblf7 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Disable charge pump while this group is sampling"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Phscfg_SPEC;
impl crate::sealed::RegSpec for Phscfg_SPEC {
    type DataType = u32;
}
#[doc = "Phase Synchronizer Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Phscfg = crate::RegValueT<Phscfg_SPEC>;

impl Phscfg {
    #[doc = "Phase Synchronizer Divider   PHSDIV. Selects the prescaling factor between the peripheral clock and the phase        synchronization signal."]
    #[inline(always)]
    pub fn phsdiv(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, phscfg::Phsdiv, Phscfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,phscfg::Phsdiv, Phscfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Write Control for Phase Sync. Divider   PDWC. No flipflop        required."]
    #[inline(always)]
    pub fn pdwc(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, phscfg::Pdwc, Phscfg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<15,0x1,1,0,phscfg::Pdwc, Phscfg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Phscfg {
    #[inline(always)]
    fn default() -> Phscfg {
        <crate::RegValueT<Phscfg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod phscfg {
    pub struct Phsdiv_SPEC;
    pub type Phsdiv = crate::EnumBitfieldStruct<u8, Phsdiv_SPEC>;
    impl Phsdiv {
        #[doc = "Off  the phase synchronization signal is constantly active"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Pdwc_SPEC;
    pub type Pdwc = crate::EnumBitfieldStruct<u8, Pdwc_SPEC>;
    impl Pdwc {
        #[doc = "No write access to divider factor"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Bitfield PHSDIV can be written"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Phssfty_SPEC;
impl crate::sealed::RegSpec for Phssfty_SPEC {
    type DataType = u32;
}
#[doc = "Phase Synchronizer Safety Control Register\n resetvalue={Application Reset:0x0}"]
pub type Phssfty = crate::RegValueT<Phssfty_SPEC>;

impl Phssfty {
    #[doc = "Alarm Flag for Safety Features. This sticky flag is set when a described error is detected  see  quot Safety        Measures quot    it is cleared by writing a  quot 1 quot  to bit ALFCLR."]
    #[inline(always)]
    pub fn alf(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, phssfty::Alf, Phssfty_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,phssfty::Alf, Phssfty_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Fault Injection Phase sync Divider"]
    #[inline(always)]
    pub fn fipd0(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, phssfty::Fipd0, Phssfty_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<4,0x1,1,0,phssfty::Fipd0, Phssfty_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Fault Injection Counter"]
    #[inline(always)]
    pub fn ficn0(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, phssfty::Ficn0, Phssfty_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<5,0x1,1,0,phssfty::Ficn0, Phssfty_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Alarm Flag ALF Clear"]
    #[inline(always)]
    pub fn alfclr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, phssfty::Alfclr, Phssfty_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0x1,1,0,phssfty::Alfclr, Phssfty_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Fault Injection Phase sync Divider"]
    #[inline(always)]
    pub fn fipd1(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, phssfty::Fipd1, Phssfty_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<20,0x1,1,0,phssfty::Fipd1, Phssfty_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Fault Injection Phase sync Divider"]
    #[inline(always)]
    pub fn ficn1(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, phssfty::Ficn1, Phssfty_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<21,0x1,1,0,phssfty::Ficn1, Phssfty_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Phssfty {
    #[inline(always)]
    fn default() -> Phssfty {
        <crate::RegValueT<Phssfty_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod phssfty {
    pub struct Alf_SPEC;
    pub type Alf = crate::EnumBitfieldStruct<u8, Alf_SPEC>;
    impl Alf {
        #[doc = "No error indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Safety problem has been detected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fipd0_SPEC;
    pub type Fipd0 = crate::EnumBitfieldStruct<u8, Fipd0_SPEC>;
    impl Fipd0 {
        #[doc = "Inject fault if FIPD1 is written with 1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "No action"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ficn0_SPEC;
    pub type Ficn0 = crate::EnumBitfieldStruct<u8, Ficn0_SPEC>;
    impl Ficn0 {
        #[doc = "Inject fault if FICN1 is written with 1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "No action"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Alfclr_SPEC;
    pub type Alfclr = crate::EnumBitfieldStruct<u8, Alfclr_SPEC>;
    impl Alfclr {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Clear alarm flag ALF"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fipd1_SPEC;
    pub type Fipd1 = crate::EnumBitfieldStruct<u8, Fipd1_SPEC>;
    impl Fipd1 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Inject fault if FIPD0 is written with 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ficn1_SPEC;
    pub type Ficn1 = crate::EnumBitfieldStruct<u8, Ficn1_SPEC>;
    impl Ficn1 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Inject fault if FICN0 is written with 0"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Picfg_SPEC;
impl crate::sealed::RegSpec for Picfg_SPEC {
    type DataType = u32;
}
#[doc = "P I Regulator Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Picfg = crate::RegValueT<Picfg_SPEC>;

impl Picfg {
    #[doc = "P Parameter for the P I Regulator   PVALUE. Upper 4 bits  mantissa  lower 4 bits  exponent"]
    #[inline(always)]
    pub fn pvalue(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Picfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Picfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "I Parameter for the P I Regulator   IVALUE. Upper 4 bits  mantissa  lower 4 bits  exponent"]
    #[inline(always)]
    pub fn ivalue(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Picfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Picfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Picfg {
    #[inline(always)]
    fn default() -> Picfg {
        <crate::RegValueT<Picfg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sgcfga_SPEC;
impl crate::sealed::RegSpec for Sgcfga_SPEC {
    type DataType = u32;
}
#[doc = "Sinewave Gen. Config. Register  A Factor\n resetvalue={Application Reset:0x0}"]
pub type Sgcfga = crate::RegValueT<Sgcfga_SPEC>;

impl Sgcfga {
    #[doc = "Sinewave Generator Amplitude   SWGA. A   A0   215  K  A0   normalized amplitude  usually 0   8230  25 39  K   frequency  Valid values are 0000   8230  3FFF  unsigned value ."]
    #[inline(always)]
    pub fn swga(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Sgcfga_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Sgcfga_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Sgcfga {
    #[inline(always)]
    fn default() -> Sgcfga {
        <crate::RegValueT<Sgcfga_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sgcfgk_SPEC;
impl crate::sealed::RegSpec for Sgcfgk_SPEC {
    type DataType = u32;
}
#[doc = "Sinewave Gen. Config. Register  K Factor\n resetvalue={Application Reset:0x0}"]
pub type Sgcfgk = crate::RegValueT<Sgcfgk_SPEC>;

impl Sgcfgk {
    #[doc = "Sinewave Generator Frequency   SWGK. SWGK is stored as floating point number K   I.D  where I   SWGK 36 19         and D   SWGK 18 0 . The value is computed as   f CLK   f SIG             2   215  pi . Bits 31 0 of the 37 bit value. Upper bits see register MISC."]
    #[inline(always)]
    pub fn swgk(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Sgcfgk_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Sgcfgk_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Sgcfgk {
    #[inline(always)]
    fn default() -> Sgcfgk {
        <crate::RegValueT<Sgcfgk_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Shift_SPEC;
impl crate::sealed::RegSpec for Shift_SPEC {
    type DataType = u32;
}
#[doc = "Shift Register\n resetvalue={Application Reset:0x0}"]
pub type Shift = crate::RegValueT<Shift_SPEC>;

impl Shift {
    #[doc = "Shift Register Pattern   SHPAT. Shift Pattern"]
    #[inline(always)]
    pub fn shpat(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Shift_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Shift_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Shift {
    #[inline(always)]
    fn default() -> Shift {
        <crate::RegValueT<Shift_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slope_SPEC;
impl crate::sealed::RegSpec for Slope_SPEC {
    type DataType = u32;
}
#[doc = "Slope Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Slope = crate::RegValueT<Slope_SPEC>;

impl Slope {
    #[doc = "Ramp Generator Slope   SLOPEVALUE. Control value to define the slope"]
    #[inline(always)]
    pub fn slopevalue(
        self,
    ) -> crate::common::RegisterField<0, 0x3fffff, 1, 0, u32, Slope_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3fffff,1,0,u32, Slope_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Slope {
    #[inline(always)]
    fn default() -> Slope {
        <crate::RegValueT<Slope_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Testcfg_SPEC;
impl crate::sealed::RegSpec for Testcfg_SPEC {
    type DataType = u32;
}
#[doc = "Test Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Testcfg = crate::RegValueT<Testcfg_SPEC>;

impl Testcfg {
    #[doc = "Sinewave Generator Enable   SINEN. Switches the BIST sinewave generator on or off."]
    #[inline(always)]
    pub fn sinen(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, testcfg::Sinen, Testcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,testcfg::Sinen, Testcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Shift Register Enable   SHIFTEN. Shift enable"]
    #[inline(always)]
    pub fn shiften(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Testcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Testcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Ramp Generator Enable   RAMPEN. Ramp Enable"]
    #[inline(always)]
    pub fn rampen(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Testcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Testcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PI Regulator Enable   CTRLEN. Enables the P I controller."]
    #[inline(always)]
    pub fn ctrlen(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Testcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Testcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Push Pull Enable   PPEN. Enables the current source"]
    #[inline(always)]
    pub fn ppen(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Testcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Testcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Push Pull Double Capacitor Select   PPDCAP"]
    #[inline(always)]
    pub fn ppdcap(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, testcfg::Ppdcap, Testcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,testcfg::Ppdcap, Testcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Push Pull Switched Capacitance Clock Select   PPSCC. Selects a prescaler for switched cap operation."]
    #[inline(always)]
    pub fn ppscc(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, testcfg::Ppscc, Testcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,testcfg::Ppscc, Testcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PI Regulator Input Select   FROMTB"]
    #[inline(always)]
    pub fn fromtb(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, testcfg::Fromtb, Testcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            testcfg::Fromtb,
            Testcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Sigma Delta Generator Output Format   ONEAHALF"]
    #[inline(always)]
    pub fn oneahalf(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        testcfg::Oneahalf,
        Testcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            testcfg::Oneahalf,
            Testcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Push Pull Test   PPTEST. Controls the push pull module isolation."]
    #[inline(always)]
    pub fn pptest(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, testcfg::Pptest, Testcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            testcfg::Pptest,
            Testcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Push Pull Force Down   PPFDN. Controls the push pull down control signal."]
    #[inline(always)]
    pub fn ppfdn(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, testcfg::Ppfdn, Testcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,testcfg::Ppfdn, Testcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Push Pull Force Up   PPFUP. Controls the push pull up control signal."]
    #[inline(always)]
    pub fn ppfup(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, testcfg::Ppfup, Testcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,testcfg::Ppfup, Testcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Synchronizer  Unsynchronized Clock Generation   CLKASYN. Defines the way the modulator clock is generated."]
    #[inline(always)]
    pub fn clkasyn(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        testcfg::Clkasyn,
        Testcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            testcfg::Clkasyn,
            Testcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Clock Synchronizer  Analog Clock Synchronization Delay   CLKDEL. Defines the delay in clocks after the sync signal Valid only if the phase synchronizer is selected  CLKASYN  160    160  0"]
    #[inline(always)]
    pub fn clkdel(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, testcfg::Clkdel, Testcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            25,
            0x7,
            1,
            0,
            testcfg::Clkdel,
            Testcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Clock Synchronizer  Modulator Clock Period   CLKDIV. Defines the period of the modulator clock  on chip external   derived        from the peripheral clock  t MOD   t PER   215  CP   f MOD   f PER   CP ."]
    #[inline(always)]
    pub fn clkdiv(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, Testcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x7,1,0,u8, Testcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Synchronizer  Clock Enable   CLKEN. Enables the clock signal"]
    #[inline(always)]
    pub fn clken(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Testcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Testcfg_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Testcfg {
    #[inline(always)]
    fn default() -> Testcfg {
        <crate::RegValueT<Testcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod testcfg {
    pub struct Sinen_SPEC;
    pub type Sinen = crate::EnumBitfieldStruct<u8, Sinen_SPEC>;
    impl Sinen {
        #[doc = "Off  standard operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "ON  the sinewave generator is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ppdcap_SPEC;
    pub type Ppdcap = crate::EnumBitfieldStruct<u8, Ppdcap_SPEC>;
    impl Ppdcap {
        #[doc = "Standard capacitance"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Double capacitance"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ppscc_SPEC;
    pub type Ppscc = crate::EnumBitfieldStruct<u8, Ppscc_SPEC>;
    impl Ppscc {
        #[doc = "00 f PER   1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 f PER   2"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 f PER   4"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 f PER   8"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Fromtb_SPEC;
    pub type Fromtb = crate::EnumBitfieldStruct<u8, Fromtb_SPEC>;
    impl Fromtb {
        #[doc = "Input signal from sigma delta"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Input signal from testbus"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oneahalf_SPEC;
    pub type Oneahalf = crate::EnumBitfieldStruct<u8, Oneahalf_SPEC>;
    impl Oneahalf {
        #[doc = "One bit operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "One a half bit operation"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pptest_SPEC;
    pub type Pptest = crate::EnumBitfieldStruct<u8, Pptest_SPEC>;
    impl Pptest {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Connects  pupl up  to vtp29 and  pupl down  to vtp27"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ppfdn_SPEC;
    pub type Ppfdn = crate::EnumBitfieldStruct<u8, Ppfdn_SPEC>;
    impl Ppfdn {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Forces  pupl down  to 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ppfup_SPEC;
    pub type Ppfup = crate::EnumBitfieldStruct<u8, Ppfup_SPEC>;
    impl Ppfup {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Forces  pupl up  to 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clkasyn_SPEC;
    pub type Clkasyn = crate::EnumBitfieldStruct<u8, Clkasyn_SPEC>;
    impl Clkasyn {
        #[doc = "Synchronized mode. Rising clock edge is defined by the phase synchronizer"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Unsynchronized mode. The modulator clock is generated independently."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clkdel_SPEC;
    pub type Clkdel = crate::EnumBitfieldStruct<u8, Clkdel_SPEC>;
    impl Clkdel {
        #[doc = "0  no delay"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 clock cycle delay"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vrcfg_SPEC;
impl crate::sealed::RegSpec for Vrcfg_SPEC {
    type DataType = u32;
}
#[doc = "Voltage Regulator Configuration Register\n resetvalue={PowerOn Reset:0x0C3,CFS Value:0x0C3}"]
pub type Vrcfg = crate::RegValueT<Vrcfg_SPEC>;

impl Vrcfg {
    #[doc = "Bias Distributor and Supply Monitoring Enable   BDON. Controls the bias distributor  charge pump  V I converter  and the        supply detectors."]
    #[inline(always)]
    pub fn bdon(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, vrcfg::Bdon, Vrcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,vrcfg::Bdon, Vrcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Supply Voltage Level   SUPLEV. Adjusts the circuitry to the supply voltage used in the application        system. Make sure to keep SUPLEV  160    160  00 or 01 in the case of a 5  160 V supply."]
    #[inline(always)]
    pub fn suplev(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, vrcfg::Suplev, Vrcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,vrcfg::Suplev, Vrcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trim Value for Reference Current   ITRIM. Adjusts the V I converter of the local bandgap  setting for bd ibn iref 5u ao . The setting is derived from PMS HPBG ITRIM  master slave trimming ."]
    #[inline(always)]
    pub fn itrim(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, Vrcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8, Vrcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Bandgap Monitoring   BGMON"]
    #[inline(always)]
    pub fn bgmon(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, vrcfg::Bgmon, Vrcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,vrcfg::Bgmon, Vrcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bandgap Select for IREF Current   IREFSEL"]
    #[inline(always)]
    pub fn irefsel(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, vrcfg::Irefsel, Vrcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,vrcfg::Irefsel, Vrcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bandgap Select for VREF Current   VREFSEL"]
    #[inline(always)]
    pub fn vrefsel(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, vrcfg::Vrefsel, Vrcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,vrcfg::Vrefsel, Vrcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Write Control for Voltage Regulator Control   VRWC. No flipflop        required."]
    #[inline(always)]
    pub fn vrwc(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, vrcfg::Vrwc, Vrcfg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<15,0x1,1,0,vrcfg::Vrwc, Vrcfg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Vrcfg {
    #[inline(always)]
    fn default() -> Vrcfg {
        <crate::RegValueT<Vrcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod vrcfg {
    pub struct Bdon_SPEC;
    pub type Bdon = crate::EnumBitfieldStruct<u8, Bdon_SPEC>;
    impl Bdon {
        #[doc = "Off  can be used e.g. for BI or scan test . Supply monitoring  VDDANA detector  is inactive  VDDEXT detector is active  Bias distributor is inactive."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Off  can be used e.g. for BI or scan test . Supply monitoring  VDDANA detector  is inactive  VDDEXT detector is active  Bias distributor is inactive."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "Fast Wakeup from Standby. Supply monitoring  VDDANA detector  is active  VDDEXT detector is active  Bias distributor is inactive."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "Operating mode  default after reset . Supply monitoring  VDDANA detector  is active  VDDEXT detector is active  Bias distributor is active."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Suplev_SPEC;
    pub type Suplev = crate::EnumBitfieldStruct<u8, Suplev_SPEC>;
    impl Suplev {
        #[doc = "Automatic control  voltage range is controlled by the power supply"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Upper voltage range  assume a 5 V power supply is connected"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "Lower voltage range  assume a 3.3 V power supply is connected"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Bgmon_SPEC;
    pub type Bgmon = crate::EnumBitfieldStruct<u8, Bgmon_SPEC>;
    impl Bgmon {
        #[doc = "Off  monitor output always set to  OK"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "ON  the monitor is active and controls the monitor output  default"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Irefsel_SPEC;
    pub type Irefsel = crate::EnumBitfieldStruct<u8, Irefsel_SPEC>;
    impl Irefsel {
        #[doc = "Local LPBG provides iref current"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "HPBG provides iref current"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Vrefsel_SPEC;
    pub type Vrefsel = crate::EnumBitfieldStruct<u8, Vrefsel_SPEC>;
    impl Vrefsel {
        #[doc = "Local LPBG provides i vref current"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "HPBG provides i vref current"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Vrwc_SPEC;
    pub type Vrwc = crate::EnumBitfieldStruct<u8, Vrwc_SPEC>;
    impl Vrwc {
        #[doc = "No write access to control bits"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Bitfields VREFSEL  IREFSEL  BGMON  ITRIM SUPLEV  BDON can be written"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vrstat_SPEC;
impl crate::sealed::RegSpec for Vrstat_SPEC {
    type DataType = u32;
}
#[doc = "Voltage Regulator Status Register\n resetvalue={Application Reset:0x1}"]
pub type Vrstat = crate::RegValueT<Vrstat_SPEC>;

impl Vrstat {
    #[doc = "Bandgap Monitoring Status Flag   BGOK. Monitors the bandgap and can be used as safety measure."]
    #[inline(always)]
    pub fn bgok(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, vrstat::Bgok, Vrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,vrstat::Bgok, Vrstat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Vrstat {
    #[inline(always)]
    fn default() -> Vrstat {
        <crate::RegValueT<Vrstat_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod vrstat {
    pub struct Bgok_SPEC;
    pub type Bgok = crate::EnumBitfieldStruct<u8, Bgok_SPEC>;
    impl Bgok {
        #[doc = "Not OK. indicates bandgap drift of either HPBG  PMS  or LPBG  bias distributor"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "OK. Bandgap monitor disabled  or HPBG in PMS and LPBG in bias distributor operate properly"]
        pub const CONST_11: Self = Self::new(1);
    }
}
