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
#[doc = r"QSPI"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspi4(pub(super) *mut u8);
unsafe impl core::marker::Send for Qspi4 {}
unsafe impl core::marker::Sync for Qspi4 {}
impl Qspi4 {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(252usize)) }
    }

    #[doc = "Basic Configuration Register\n resetvalue={Application Reset:0x0F871C71}"]
    #[inline(always)]
    pub const fn bacon(&self) -> crate::common::Reg<self::Bacon_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }

    #[doc = "BACON ENTRY Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn baconentry(&self) -> crate::common::Reg<self::Baconentry_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "DATA ENTRY Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dataentryx(
        &self,
    ) -> [crate::common::Reg<self::DataentrYx_SPEC, crate::common::W>; 8] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x64usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x64usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x64usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x64usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x64usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x64usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x64usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x64usize + 0x1cusize)),
            ]
        }
    }

    #[doc = "Configuration Extension 0\n resetvalue={Application Reset:0x1450}"]
    #[inline(always)]
    pub const fn econz(&self) -> [crate::common::Reg<self::EcoNz_SPEC, crate::common::RW>; 8] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x1cusize)),
            ]
        }
    }

    #[doc = "Flags Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn flagsclear(&self) -> crate::common::Reg<self::Flagsclear_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(84usize)) }
    }

    #[doc = "Global Configuration Register\n resetvalue={Application Reset:0x0F30FF}"]
    #[inline(always)]
    pub const fn globalcon(&self) -> crate::common::Reg<self::Globalcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "Global Configuration Register 1\n resetvalue={Application Reset:0x50000}"]
    #[inline(always)]
    pub const fn globalcon1(&self) -> crate::common::Reg<self::Globalcon1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0C0C000}"]
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

    #[doc = "Move Counter Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mc(&self) -> crate::common::Reg<self::Mc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(164usize)) }
    }

    #[doc = "Move Counter control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mccon(&self) -> crate::common::Reg<self::Mccon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(168usize)) }
    }

    #[doc = "MIX ENTRY Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mixentry(&self) -> crate::common::Reg<self::Mixentry_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(92usize)) }
    }

    #[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(232usize)) }
    }

    #[doc = "Port Input Select Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pisel(&self) -> crate::common::Reg<self::Pisel_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }

    #[doc = "RX EXIT Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rxexit(&self) -> crate::common::Reg<self::Rxexit_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }

    #[doc = "RX EXIT Debug Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rxexitd(&self) -> crate::common::Reg<self::Rxexitd_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(148usize)) }
    }

    #[doc = "Slave Select Output Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ssoc(&self) -> crate::common::Reg<self::Ssoc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }

    #[doc = "Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn status(&self) -> crate::common::Reg<self::Status_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "Status Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn status1(&self) -> crate::common::Reg<self::Status1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }

    #[doc = "Extra Large Data Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn xxlcon(&self) -> crate::common::Reg<self::Xxlcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(88usize)) }
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
pub struct Bacon_SPEC;
impl crate::sealed::RegSpec for Bacon_SPEC {
    type DataType = u32;
}
#[doc = "Basic Configuration Register\n resetvalue={Application Reset:0x0F871C71}"]
pub type Bacon = crate::RegValueT<Bacon_SPEC>;

impl Bacon {
    #[doc = "Last Word in a Frame   LAST. Defines if the following data word is last in the current frame or not"]
    #[inline(always)]
    pub fn last(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, bacon::Last, Bacon_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,bacon::Last, Bacon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Prescaler for the Idle Delay   IPRE. Length in T PER units"]
    #[inline(always)]
    pub fn ipre(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, bacon::Ipre, Bacon_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x7,1,0,bacon::Ipre, Bacon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Idle Delay Length   IDLE. Defines the length of both idle delays  IDLEA and IDLEB  in T PER units pre scaled with IPRE"]
    #[inline(always)]
    pub fn idle(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, bacon::Idle, Bacon_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x7,1,0,bacon::Idle, Bacon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Prescaler for the Leading Delay   LPRE. Length in T PER units"]
    #[inline(always)]
    pub fn lpre(
        self,
    ) -> crate::common::RegisterField<7, 0x7, 1, 0, bacon::Lpre, Bacon_SPEC, crate::common::R> {
        crate::common::RegisterField::<7,0x7,1,0,bacon::Lpre, Bacon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Leading Delay Length   LEAD. Defines the length of the leading delay  in T PER units pre scaled with LPRE"]
    #[inline(always)]
    pub fn lead(
        self,
    ) -> crate::common::RegisterField<10, 0x7, 1, 0, bacon::Lead, Bacon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<10,0x7,1,0,bacon::Lead, Bacon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Prescaler for the Trailing Delay   TPRE. Length in T PER units"]
    #[inline(always)]
    pub fn tpre(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, bacon::Tpre, Bacon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<13,0x7,1,0,bacon::Tpre, Bacon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Trailing Delay Length   TRAIL. Defines the length of the trailing delay  in T PER units pre scaled with TPRE"]
    #[inline(always)]
    pub fn trail(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, bacon::Trail, Bacon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x7,1,0,bacon::Trail, Bacon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Parity Type   PARTYP. Valid for both receive and transmit direction"]
    #[inline(always)]
    pub fn partyp(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, bacon::Partyp, Bacon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<19,0x1,1,0,bacon::Partyp, Bacon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "User Interrupt at the PT1 Event in the Subsequent Frames   UINT. This bit is an enable signal for the PT1 event routed to the User        Interrupt Service Request. The interrupt signals are generated until        disabled with the next BACON."]
    #[inline(always)]
    pub fn uint(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, bacon::Uint, Bacon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<20,0x1,1,0,bacon::Uint, Bacon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Shift MSB or LSB First   MSB. This bit sets the shift direction of the shift register. If the MSB        option is set  and the data is a block longer than 32 bits  the block        must be fed into the TXFIFO in reverse direction  from the end of the        block until its beginning."]
    #[inline(always)]
    pub fn msb(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, bacon::Msb, Bacon_SPEC, crate::common::R> {
        crate::common::RegisterField::<21,0x1,1,0,bacon::Msb, Bacon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Byte   BYTE. Defines if data length is expressed in bits or bytes. In Slave Mode BYTE must be  0 ."]
    #[inline(always)]
    pub fn byte(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, bacon::Byte, Bacon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<22,0x1,1,0,bacon::Byte, Bacon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Data Length   DL. Defines the data length in bits or bytes of one data block  depending on        the setting of the bit field BYTE. For the maximum baud rate of 50  160 MBaud  the minimum data length possible        is four."]
    #[inline(always)]
    pub fn dl(
        self,
    ) -> crate::common::RegisterField<23, 0x1f, 1, 0, bacon::Dl, Bacon_SPEC, crate::common::R> {
        crate::common::RegisterField::<23,0x1f,1,0,bacon::Dl, Bacon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Channel Select   CS. Selects the channel to which the subsequent data entry belongs  channel          the SLSO signal to be activated and the corresponding ECON        configuration extension  This bit field selects one slave in a range of 0 to 15  by driving one        SLSO signal out of 16 available. In case of an external demux mode  this bit field appears on the lines        SLS01 to SLSO4 as it is  additionally inverted or not  as defined in the SSOC register."]
    #[inline(always)]
    pub fn cs(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Bacon_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Bacon_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Bacon {
    #[inline(always)]
    fn default() -> Bacon {
        <crate::RegValueT<Bacon_SPEC> as RegisterValue<_>>::new(260512881)
    }
}
pub mod bacon {
    pub struct Last_SPEC;
    pub type Last = crate::EnumBitfieldStruct<u8, Last_SPEC>;
    impl Last {
        #[doc = "0 Not Last"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Last"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ipre_SPEC;
    pub type Ipre = crate::EnumBitfieldStruct<u8, Ipre_SPEC>;
    impl Ipre {
        #[doc = "000 1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 4"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Idle_SPEC;
    pub type Idle = crate::EnumBitfieldStruct<u8, Idle_SPEC>;
    impl Idle {
        #[doc = "0 1 unit"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Lpre_SPEC;
    pub type Lpre = crate::EnumBitfieldStruct<u8, Lpre_SPEC>;
    impl Lpre {
        #[doc = "000 1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 4"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lead_SPEC;
    pub type Lead = crate::EnumBitfieldStruct<u8, Lead_SPEC>;
    impl Lead {
        #[doc = "0 1 unit"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Tpre_SPEC;
    pub type Tpre = crate::EnumBitfieldStruct<u8, Tpre_SPEC>;
    impl Tpre {
        #[doc = "000 1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 4"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trail_SPEC;
    pub type Trail = crate::EnumBitfieldStruct<u8, Trail_SPEC>;
    impl Trail {
        #[doc = "0 1 unit"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Partyp_SPEC;
    pub type Partyp = crate::EnumBitfieldStruct<u8, Partyp_SPEC>;
    impl Partyp {
        #[doc = "0 Even parity"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Odd parity"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Uint_SPEC;
    pub type Uint = crate::EnumBitfieldStruct<u8, Uint_SPEC>;
    impl Uint {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msb_SPEC;
    pub type Msb = crate::EnumBitfieldStruct<u8, Msb_SPEC>;
    impl Msb {
        #[doc = "0 Shift LSB first"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Shift MSB first"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Byte_SPEC;
    pub type Byte = crate::EnumBitfieldStruct<u8, Byte_SPEC>;
    impl Byte {
        #[doc = "0 DL defines the data length in bits"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 DL defines the data length in bytes"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Dl_SPEC;
    pub type Dl = crate::EnumBitfieldStruct<u8, Dl_SPEC>;
    impl Dl {
        #[doc = "0 2 bits if        BYTE 0  XXL mode if BYTE 1"]
        pub const CONST_00: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Baconentry_SPEC;
impl crate::sealed::RegSpec for Baconentry_SPEC {
    type DataType = u32;
}
#[doc = "BACON ENTRY Register\n resetvalue={Application Reset:0x0}"]
pub type Baconentry = crate::RegValueT<Baconentry_SPEC>;

impl Baconentry {
    #[doc = "Entry Point to the TxFIFO   E"]
    #[inline(always)]
    pub fn e(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Baconentry_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Baconentry_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Baconentry {
    #[inline(always)]
    fn default() -> Baconentry {
        <crate::RegValueT<Baconentry_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Sleep Mode Enable Control   EDIS. Used to enable the module  8217 s sleep mode."]
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
    pub struct Edis_SPEC;
    pub type Edis = crate::EnumBitfieldStruct<u8, Edis_SPEC>;
    impl Edis {
        #[doc = "0 Enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DataentrYx_SPEC;
impl crate::sealed::RegSpec for DataentrYx_SPEC {
    type DataType = u32;
}
#[doc = "DATA ENTRY Register 0\n resetvalue={Application Reset:0x0}"]
pub type DataentrYx = crate::RegValueT<DataentrYx_SPEC>;

impl DataentrYx {
    #[doc = "Entry Point to the TxFIFO   E"]
    #[inline(always)]
    pub fn e(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, DataentrYx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, DataentrYx_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for DataentrYx {
    #[inline(always)]
    fn default() -> DataentrYx {
        <crate::RegValueT<DataentrYx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EcoNz_SPEC;
impl crate::sealed::RegSpec for EcoNz_SPEC {
    type DataType = u32;
}
#[doc = "Configuration Extension 0\n resetvalue={Application Reset:0x1450}"]
pub type EcoNz = crate::RegValueT<EcoNz_SPEC>;

impl EcoNz {
    #[doc = "Time Quantum   Q. Defines the time quantum length used by A  B  and C to define the baud        rate and duty cycle by. This prescaler cascades the prescaler        GLOBALCON.TQ."]
    #[inline(always)]
    pub fn q(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, econz::Q, EcoNz_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,econz::Q, EcoNz_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bit Segment 1   A. Length expressed in time quantums of ECONz.Q."]
    #[inline(always)]
    pub fn a(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, econz::A, EcoNz_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,econz::A, EcoNz_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bit Segment 2   B. Length expressed in time quantums of ECONz.Q."]
    #[inline(always)]
    pub fn b(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, econz::B, EcoNz_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,econz::B, EcoNz_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bit Segment 3   C. Length expressed in time quantums of ECONz.Q."]
    #[inline(always)]
    pub fn c(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, econz::C, EcoNz_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,econz::C, EcoNz_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Phase   CPH. Delay of one half SCLK clock cycle."]
    #[inline(always)]
    pub fn cph(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, econz::Cph, EcoNz_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,econz::Cph, EcoNz_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Polarity   CPOL. Idle level of the shift clock signal at the SCLK pin"]
    #[inline(always)]
    pub fn cpol(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, econz::Cpol, EcoNz_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,econz::Cpol, EcoNz_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Parity Check   PAREN. This bit field enables both the parity generation in transmit and parity        check in receive direction."]
    #[inline(always)]
    pub fn paren(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, econz::Paren, EcoNz_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,econz::Paren, EcoNz_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Permutate bytes to   from Big Endian   BE"]
    #[inline(always)]
    pub fn be(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, econz::Be, EcoNz_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,econz::Be, EcoNz_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for EcoNz {
    #[inline(always)]
    fn default() -> EcoNz {
        <crate::RegValueT<EcoNz_SPEC> as RegisterValue<_>>::new(5200)
    }
}
pub mod econz {
    pub struct Q_SPEC;
    pub type Q = crate::EnumBitfieldStruct<u8, Q_SPEC>;
    impl Q {
        #[doc = "000000 1"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct A_SPEC;
    pub type A = crate::EnumBitfieldStruct<u8, A_SPEC>;
    impl A {
        #[doc = "00 1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 2"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 3"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 4"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct B_SPEC;
    pub type B = crate::EnumBitfieldStruct<u8, B_SPEC>;
    impl B {
        #[doc = "00 0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 1"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 3"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct C_SPEC;
    pub type C = crate::EnumBitfieldStruct<u8, C_SPEC>;
    impl C {
        #[doc = "00 0  if B 0  than C is minimum 1 per hardware"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 1"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 3"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Cph_SPEC;
    pub type Cph = crate::EnumBitfieldStruct<u8, Cph_SPEC>;
    impl Cph {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpol_SPEC;
    pub type Cpol = crate::EnumBitfieldStruct<u8, Cpol_SPEC>;
    impl Cpol {
        #[doc = "0 Idle level low"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Idle level high"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Paren_SPEC;
    pub type Paren = crate::EnumBitfieldStruct<u8, Paren_SPEC>;
    impl Paren {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Be_SPEC;
    pub type Be = crate::EnumBitfieldStruct<u8, Be_SPEC>;
    impl Be {
        #[doc = "00 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 16 bit big endian"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 32 bit big endian"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Disabled"]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flagsclear_SPEC;
impl crate::sealed::RegSpec for Flagsclear_SPEC {
    type DataType = u32;
}
#[doc = "Flags Clear Register\n resetvalue={Application Reset:0x0}"]
pub type Flagsclear = crate::RegValueT<Flagsclear_SPEC>;

impl Flagsclear {
    #[doc = "Write Only Bits for Clearing the Error Flags   ERRORCLEARS. Writing 1 clears the corresponding error flag in the ERORRFLAGS bit        field. Reading returns 0."]
    #[inline(always)]
    pub fn errorclears(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1ff,
        1,
        0,
        flagsclear::Errorclears,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1ff,
            1,
            0,
            flagsclear::Errorclears,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Transmit Event Flag Clear   TXC. Write of 1 clears the STATUS .TXF        bit. Write of 0 has no effect. Read delivers 0."]
    #[inline(always)]
    pub fn txc(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        flagsclear::Txc,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            flagsclear::Txc,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Receive Event Flag Clear   RXC. Write of 1 clears the STATUS .RXF        bit. Write of 0 has no effect. Read delivers 0."]
    #[inline(always)]
    pub fn rxc(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        flagsclear::Rxc,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            flagsclear::Rxc,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "PT1 Event Flag Clear   PT1C. Write of 1 clears the STATUS .PT1F        bit. Write of 0 has no effect. Read delivers 0."]
    #[inline(always)]
    pub fn pt1c(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        flagsclear::Pt1C,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            flagsclear::Pt1C,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "PT2 Event Flag Clear   PT2C. Write of 1 clears the STATUS .PT2F        bit. Write of 0 has no effect. Read delivers 0."]
    #[inline(always)]
    pub fn pt2c(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        flagsclear::Pt2C,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            flagsclear::Pt2C,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "User Event Flag Clear   USRC. Write of 1 clears the STATUS .USRF        bit. Write of 0 has no effect. Read delivers 0."]
    #[inline(always)]
    pub fn usrc(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        flagsclear::Usrc,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            flagsclear::Usrc,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Flagsclear {
    #[inline(always)]
    fn default() -> Flagsclear {
        <crate::RegValueT<Flagsclear_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod flagsclear {
    pub struct Errorclears_SPEC;
    pub type Errorclears = crate::EnumBitfieldStruct<u16, Errorclears_SPEC>;
    impl Errorclears {
        #[doc = "000000000 No clear"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "000000001 Parity Error clear"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "000000010 Unexpected Configuration Error clear"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "000000100 Baud Rate Error clear"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "000001000 TXFIFO overflow clear"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "000010000 TXFIFO underflow clear"]
        pub const CONST_1616: Self = Self::new(16);
        #[doc = "000100000 RXFIFO overflow clear"]
        pub const CONST_3232: Self = Self::new(32);
        #[doc = "001000000 RXFIFO underflow clear"]
        pub const CONST_6464: Self = Self::new(64);
        #[doc = "010000000 EXPECT time out clear"]
        pub const CONST_128128: Self = Self::new(128);
        #[doc = "100000000 SLSI misplaced inactivation clear"]
        pub const CONST_256256: Self = Self::new(256);
    }
    pub struct Txc_SPEC;
    pub type Txc = crate::EnumBitfieldStruct<u8, Txc_SPEC>;
    impl Txc {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rxc_SPEC;
    pub type Rxc = crate::EnumBitfieldStruct<u8, Rxc_SPEC>;
    impl Rxc {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pt1C_SPEC;
    pub type Pt1C = crate::EnumBitfieldStruct<u8, Pt1C_SPEC>;
    impl Pt1C {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pt2C_SPEC;
    pub type Pt2C = crate::EnumBitfieldStruct<u8, Pt2C_SPEC>;
    impl Pt2C {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Usrc_SPEC;
    pub type Usrc = crate::EnumBitfieldStruct<u8, Usrc_SPEC>;
    impl Usrc {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Globalcon_SPEC;
impl crate::sealed::RegSpec for Globalcon_SPEC {
    type DataType = u32;
}
#[doc = "Global Configuration Register\n resetvalue={Application Reset:0x0F30FF}"]
pub type Globalcon = crate::RegValueT<Globalcon_SPEC>;

impl Globalcon {
    #[doc = "Global Time Quantum Length   TQ. Common n divider scaling the baud rates of all channels in direction of        higher or lower baud rates. Must not be changed during a running        transaction."]
    #[inline(always)]
    pub fn tq(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, globalcon::Tq, Globalcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xff,
            1,
            0,
            globalcon::Tq,
            Globalcon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Status Injection   SI. Selects if the status register content injection into the RxFIFO is        enabled or disabled. The status injections  if enabled  is performed after each data block         depending on the BACON .DL        and BYTE setting."]
    #[inline(always)]
    pub fn si(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, globalcon::Si, Globalcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,globalcon::Si, Globalcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Time Out Value for the Expect Phase   EXPECT. expressed in T QSPI units"]
    #[inline(always)]
    pub fn expect(
        self,
    ) -> crate::common::RegisterField<
        10,
        0xf,
        1,
        0,
        globalcon::Expect,
        Globalcon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0xf,
            1,
            0,
            globalcon::Expect,
            Globalcon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Loop Back Control   LB. Selects if the transmit output is internally connected to the receive        input for test purposes. For detailed description  see the Loop Back Mode section."]
    #[inline(always)]
    pub fn lb(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, globalcon::Lb, Globalcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            globalcon::Lb,
            Globalcon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Delayed Mode for SLSO0   DEL0. Switches the delayed mode  external slave select expansion mode  on and        off"]
    #[inline(always)]
    pub fn del0(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        globalcon::Del0,
        Globalcon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            globalcon::Del0,
            Globalcon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Strobe Delay for SLSO0 in Delayed Mode   STROBE. Defines the length of the SLSO0 delay in T Q time units as defined for channel z   T Q units  selected by the current BACON .CS         if GLOBALCON .DEL0  160    160 1."]
    #[inline(always)]
    pub fn strobe(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1f,
        1,
        0,
        globalcon::Strobe,
        Globalcon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1f,
            1,
            0,
            globalcon::Strobe,
            Globalcon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Stop on RxFIFO Full   SRF. If this bit is set  the data fetching out of the TxFIFO by the shift        register stops when the RxFIFO is full  in order to prevent RxFIFO        overflow. Master mode only."]
    #[inline(always)]
    pub fn srf(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        globalcon::Srf,
        Globalcon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            globalcon::Srf,
            Globalcon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Slave Transmit Idle State Polarity   STIP. This bit determines the logic level of the Slave Mode transmit signal        MRST when the QSPI slave select input signals are inactive         PISEL.SLSIS  160   185   160 000 B  ."]
    #[inline(always)]
    pub fn stip(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        globalcon::Stip,
        Globalcon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            globalcon::Stip,
            Globalcon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Enable Bit   EN. Used to request transition between PAUSE and RUN mode per software.        Cleared by hardware automatically at leaving the following states         disabled  suspend and sleep. In order to determine if the requested state has actually been reached         the STATUS .PHASE        bit field should be polled. See also Operation Modes."]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, globalcon::En, Globalcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            globalcon::En,
            Globalcon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Master Slave Mode   MS. Selects if the module operates in master or slave mode. This bit field must be configured before the first write to the TXFIFO."]
    #[inline(always)]
    pub fn ms(
        self,
    ) -> crate::common::RegisterField<25, 0x3, 1, 0, globalcon::Ms, Globalcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            25,
            0x3,
            1,
            0,
            globalcon::Ms,
            Globalcon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Automatic Reset Enable   AREN. Enables the reset of the GLOBALCON .EN        on baud rate and spike error in slave mode."]
    #[inline(always)]
    pub fn aren(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        globalcon::Aren,
        Globalcon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            globalcon::Aren,
            Globalcon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Clock Select   CLKSEL. Selects the clock source for the asynchronous block."]
    #[inline(always)]
    pub fn clksel(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        globalcon::Clksel,
        Globalcon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            globalcon::Clksel,
            Globalcon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Bits for resetting sub modules per software   RESETS. Write to this bit field triggers a reset operation. The reset operation   cycle count depends on the Clock setup in the 2 Clock domains. The shortest time is 3 x T fast   3 x T slow. With the added margin the delay will be  10 times slowest clock between SPB and Kernel. The duration of reset operation depends on the used clock setups. It is 10 times of the slowest clock between SPB and Kernel. CLOCKSEL shall be enabled to use GLOBALCON.RESETS. After the Reset the CLOCKSEL will be also cleared in case of 10 11. For resetting the whole module kernel  use alternatively the registers KRST0   xa0    xa0  KRST1 reset mechanism."]
    #[inline(always)]
    pub fn resets(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x3,
        1,
        0,
        globalcon::Resets,
        Globalcon_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            globalcon::Resets,
            Globalcon_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Globalcon {
    #[inline(always)]
    fn default() -> Globalcon {
        <crate::RegValueT<Globalcon_SPEC> as RegisterValue<_>>::new(995583)
    }
}
pub mod globalcon {
    pub struct Tq_SPEC;
    pub type Tq = crate::EnumBitfieldStruct<u8, Tq_SPEC>;
    impl Tq {
        #[doc = "division by 1"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Si_SPEC;
    pub type Si = crate::EnumBitfieldStruct<u8, Si_SPEC>;
    impl Si {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Expect_SPEC;
    pub type Expect = crate::EnumBitfieldStruct<u8, Expect_SPEC>;
    impl Expect {
        #[doc = "0 64  2 6"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Lb_SPEC;
    pub type Lb = crate::EnumBitfieldStruct<u8, Lb_SPEC>;
    impl Lb {
        #[doc = "0 Loop Back inactive"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Loop Back active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Del0_SPEC;
    pub type Del0 = crate::EnumBitfieldStruct<u8, Del0_SPEC>;
    impl Del0 {
        #[doc = "0 Delayed mode off"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Delayed mode on"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Strobe_SPEC;
    pub type Strobe = crate::EnumBitfieldStruct<u8, Strobe_SPEC>;
    impl Strobe {
        #[doc = "00000 1"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Srf_SPEC;
    pub type Srf = crate::EnumBitfieldStruct<u8, Srf_SPEC>;
    impl Srf {
        #[doc = "0 Feature disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Feature enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Stip_SPEC;
    pub type Stip = crate::EnumBitfieldStruct<u8, Stip_SPEC>;
    impl Stip {
        #[doc = "0 MRST   0 when QSPI is deselected in Slave Mode."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MRST   1 when QSPI is deselected in Slave Mode"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 PAUSE requested"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 RUN requested"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ms_SPEC;
    pub type Ms = crate::EnumBitfieldStruct<u8, Ms_SPEC>;
    impl Ms {
        #[doc = "00 Master        Transmit and Receive"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 Slave Transmit        and Receive"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Slave Transmit        and Receive"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Aren_SPEC;
    pub type Aren = crate::EnumBitfieldStruct<u8, Aren_SPEC>;
    impl Aren {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clksel_SPEC;
    pub type Clksel = crate::EnumBitfieldStruct<u8, Clksel_SPEC>;
    impl Clksel {
        #[doc = "0 no clock"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 f PER"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Resets_SPEC;
    pub type Resets = crate::EnumBitfieldStruct<u8, Resets_SPEC>;
    impl Resets {
        #[doc = "00 No reset triggered"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 State Machine  TXFIFO and RXFIFO reset  registers not reseted. Only control structures shall be reset. It avoids a re configuration of the whole IP. After this reset Communication can be restarted."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Registers reset. Clears the configuration of the QSPI registers. New setup have to be programmed."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 State Machine  TXFIFO and RXFIFO reset and registers reset"]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Globalcon1_SPEC;
impl crate::sealed::RegSpec for Globalcon1_SPEC {
    type DataType = u32;
}
#[doc = "Global Configuration Register 1\n resetvalue={Application Reset:0x50000}"]
pub type Globalcon1 = crate::RegValueT<Globalcon1_SPEC>;

impl Globalcon1 {
    #[doc = "Error Enable Bits   ERRORENS. Bits for enabling interrupt on all available error types"]
    #[inline(always)]
    pub fn errorens(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1ff,
        1,
        0,
        globalcon1::Errorens,
        Globalcon1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1ff,
            1,
            0,
            globalcon1::Errorens,
            Globalcon1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tx Interrupt Event Enable   TXEN. Enables the Tx interrupt."]
    #[inline(always)]
    pub fn txen(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        globalcon1::Txen,
        Globalcon1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            globalcon1::Txen,
            Globalcon1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "RX Interrupt Event Enable   RXEN. Enables the Rx interrupt."]
    #[inline(always)]
    pub fn rxen(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        globalcon1::Rxen,
        Globalcon1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            globalcon1::Rxen,
            Globalcon1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Interrupt on PT1 Event Enable   PT1EN. Enables the PT interrupt on an PT1 event  as selected by the PT1 bit        field."]
    #[inline(always)]
    pub fn pt1en(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        globalcon1::Pt1En,
        Globalcon1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            globalcon1::Pt1En,
            Globalcon1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Interrupt on PT2 Event Enable   PT2EN. Enables the PT interrupt on an PT2 event  as selected by the PT2 bit        field."]
    #[inline(always)]
    pub fn pt2en(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        globalcon1::Pt2En,
        Globalcon1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            globalcon1::Pt2En,
            Globalcon1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Interrupt on USR Event Enable   USREN. Enables the USR interrupt on an USR event  as selected by the PT1 bit        field. In Move Counter mode of operation  User Interrupt line is reused for          the IAL and IBL interrupts  which can be enabled by MCCON .IALEN          and MCCON .IBLEN.          Bit USREN does not influence these interrupts."]
    #[inline(always)]
    pub fn usren(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        globalcon1::Usren,
        Globalcon1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            globalcon1::Usren,
            Globalcon1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Transmit FIFO Interrupt Threshold   TXFIFOINT. In Combined Mode  as long as the TXFIFO filling level is equal or less        than this threshold  than each move of data or configuration from the        TXFIFO triggers a transmit interrupt. In Batch Mode  interrupt is generated only at the moment of filling        level falling below the threshold level. In Single Mode  this bit field is don  8217 t care. Reset value of the level is 01 ."]
    #[inline(always)]
    pub fn txfifoint(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        globalcon1::Txfifoint,
        Globalcon1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            globalcon1::Txfifoint,
            Globalcon1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Receive FIFO Interrupt Threshold   RXFIFOINT. In Combined Mode  as long as the RXFIFO filling level is equal or        greater than this threshold  than each move of data or status  if        enabled  into the RXFIFO triggers a receive interrupt. In Batch mode  interrupt is generated only at the moment of filling        level exceeding this threshold level. In Single Mode  this bit field is don  8217 t care. Reset value of the level is 01 ."]
    #[inline(always)]
    pub fn rxfifoint(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x3,
        1,
        0,
        globalcon1::Rxfifoint,
        Globalcon1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x3,
            1,
            0,
            globalcon1::Rxfifoint,
            Globalcon1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Phase Transition Event 1   PT1. Selects the first phase transition to trigger the PT interrupt."]
    #[inline(always)]
    pub fn pt1(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x7,
        1,
        0,
        globalcon1::Pt1,
        Globalcon1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x7,
            1,
            0,
            globalcon1::Pt1,
            Globalcon1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Phase Transition Event 2   PT2. Selects the second phase transition to trigger the PT2 interrupt. In Master Mode  the following events are available. In Slave Mode  only the SLSI signal  rising edge  triggers the interrupt for PT2. For this purpose  always use the setting 101  EOF . This interrupt is independent of the CS value."]
    #[inline(always)]
    pub fn pt2(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x7,
        1,
        0,
        globalcon1::Pt2,
        Globalcon1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x7,
            1,
            0,
            globalcon1::Pt2,
            Globalcon1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "TXFIFO Mode   TXFM. Selects the TXFIFO mode."]
    #[inline(always)]
    pub fn txfm(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x3,
        1,
        0,
        globalcon1::Txfm,
        Globalcon1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x3,
            1,
            0,
            globalcon1::Txfm,
            Globalcon1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "RXFIFO Mode   RXFM. Selects the RXFIFO mode."]
    #[inline(always)]
    pub fn rxfm(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x3,
        1,
        0,
        globalcon1::Rxfm,
        Globalcon1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x3,
            1,
            0,
            globalcon1::Rxfm,
            Globalcon1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Globalcon1 {
    #[inline(always)]
    fn default() -> Globalcon1 {
        <crate::RegValueT<Globalcon1_SPEC> as RegisterValue<_>>::new(327680)
    }
}
pub mod globalcon1 {
    pub struct Errorens_SPEC;
    pub type Errorens = crate::EnumBitfieldStruct<u16, Errorens_SPEC>;
    impl Errorens {
        #[doc = "000000000 All errors disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "000000001 Parity Error  PAREEN"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "000000010 Unexpected Configuration Error"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "000000100 Baud Rate Error  slave mode  BEN"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "000001000 TXFIFO overflow  software error"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "000010000 TXFIFO underflow  slave mode  TEN"]
        pub const CONST_1616: Self = Self::new(16);
        #[doc = "000100000 RXFIFO overflow REN"]
        pub const CONST_3232: Self = Self::new(32);
        #[doc = "001000000 RXFIFO underflow  software error"]
        pub const CONST_6464: Self = Self::new(64);
        #[doc = "010000000 EXPECT timeout"]
        pub const CONST_128128: Self = Self::new(128);
        #[doc = "100000000 SLSI misplaced inactivation enable"]
        pub const CONST_256256: Self = Self::new(256);
    }
    pub struct Txen_SPEC;
    pub type Txen = crate::EnumBitfieldStruct<u8, Txen_SPEC>;
    impl Txen {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rxen_SPEC;
    pub type Rxen = crate::EnumBitfieldStruct<u8, Rxen_SPEC>;
    impl Rxen {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pt1En_SPEC;
    pub type Pt1En = crate::EnumBitfieldStruct<u8, Pt1En_SPEC>;
    impl Pt1En {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pt2En_SPEC;
    pub type Pt2En = crate::EnumBitfieldStruct<u8, Pt2En_SPEC>;
    impl Pt2En {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Usren_SPEC;
    pub type Usren = crate::EnumBitfieldStruct<u8, Usren_SPEC>;
    impl Usren {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Txfifoint_SPEC;
    pub type Txfifoint = crate::EnumBitfieldStruct<u8, Txfifoint_SPEC>;
    impl Txfifoint {
        #[doc = "00 1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 2"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 3"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 4"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Rxfifoint_SPEC;
    pub type Rxfifoint = crate::EnumBitfieldStruct<u8, Rxfifoint_SPEC>;
    impl Rxfifoint {
        #[doc = "00 0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 1"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 3"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Pt1_SPEC;
    pub type Pt1 = crate::EnumBitfieldStruct<u8, Pt1_SPEC>;
    impl Pt1 {
        #[doc = "000 BUSY Master Mode  End of WAIT phase Slave Mode  Transmit data is present waiting for the shift clock"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 SCLKPC Master Mode  Serial clock polarity change"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 SOF Master Mode  Start of Frame Slave Mode  Transmission of the first data bit started"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 TBE Master Mode  Transmit Buffer Emptied Slave Mode  Data is taken from the TXFIFO by the QSPI shift engine"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 RBF Master Mode  Receive Buffer Filled Slave Mode  Received data is written to the RXFIFO"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 EOF Master Mode  End of Frame Slave Mode  The last data bit has been received"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 DNA  Master Mode  Data not Available   Start of Expect"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "111 CONT  Master Mode  End of EXPECT phase"]
        pub const CONST_77: Self = Self::new(7);
    }
    pub struct Pt2_SPEC;
    pub type Pt2 = crate::EnumBitfieldStruct<u8, Pt2_SPEC>;
    impl Pt2 {
        #[doc = "000 BUSY  Master Mode  end of WAIT phase"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 SCLKPC  Master Mode  serial clock polarity change"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 SOF  Master Mode  Start of Frame"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 TBE  Master Mode  Transmit Buffer Emptied"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 RBF  Master Mode  Receive Buffer Filled"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 EOF  Master Mode  End of Frame Slave Mode  SLSI deactivated  rising edge on the SLSI pin"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 DNA  Master Mode  Data not Available   Start of Expect"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "111 CONT  Master Mode  End of EXPECT phase"]
        pub const CONST_77: Self = Self::new(7);
    }
    pub struct Txfm_SPEC;
    pub type Txfm = crate::EnumBitfieldStruct<u8, Txfm_SPEC>;
    impl Txfm {
        #[doc = "00 Combined Move Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Single Move Mode"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Batch Move Mode"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Rxfm_SPEC;
    pub type Rxfm = crate::EnumBitfieldStruct<u8, Rxfm_SPEC>;
    impl Rxfm {
        #[doc = "00 Combined Move Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Single Move Mode"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Batch Move Mode"]
        pub const CONST_22: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0C0C000}"]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(12632064)
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
pub struct Mc_SPEC;
impl crate::sealed::RegSpec for Mc_SPEC {
    type DataType = u32;
}
#[doc = "Move Counter Register\n resetvalue={Application Reset:0x0}"]
pub type Mc = crate::RegValueT<Mc_SPEC>;

impl Mc {
    #[doc = "Move Count   MCOUNT. Defines the number of moves to be performed in short mode  in range of 1        to 8191."]
    #[inline(always)]
    pub fn mcount(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, Mc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1fff,1,0,u16, Mc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Current Status of the Move Counter   CURRENT. Shows the current status of the Move Counter  that is  how many data blocks are to be transmitted until the end of the frame."]
    #[inline(always)]
    pub fn current(
        self,
    ) -> crate::common::RegisterField<16, 0x1fff, 1, 0, mc::Current, Mc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1fff,1,0,mc::Current, Mc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Mc {
    #[inline(always)]
    fn default() -> Mc {
        <crate::RegValueT<Mc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mc {
    pub struct Current_SPEC;
    pub type Current = crate::EnumBitfieldStruct<u8, Current_SPEC>;
    impl Current {
        #[doc = "0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mccon_SPEC;
impl crate::sealed::RegSpec for Mccon_SPEC {
    type DataType = u32;
}
#[doc = "Move Counter control Register\n resetvalue={Application Reset:0x0}"]
pub type Mccon = crate::RegValueT<Mccon_SPEC>;

impl Mccon {
    #[doc = "Prescaler for the Trailing Delay 2   TPRE2. Trailing delay injected in the configuration register for the last data        if T2EN is set. Length in units T PER"]
    #[inline(always)]
    pub fn tpre2(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, mccon::Tpre2, Mccon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,mccon::Tpre2, Mccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Last Trailing Delay   TRAIL2. Trailing delay injected in the configuration register for the last data        if T2EN is set. Defines the length of the leading delay  in T PER units pre scaled with TPRE"]
    #[inline(always)]
    pub fn trail2(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, mccon::Trail2, Mccon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x7,1,0,mccon::Trail2, Mccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Before Last Enable   IBLEN. Enable bit for this event."]
    #[inline(always)]
    pub fn iblen(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, mccon::Iblen, Mccon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,mccon::Iblen, Mccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Before Last Flag   IBLF. Flag bit for this event."]
    #[inline(always)]
    pub fn iblf(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, mccon::Iblf, Mccon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<17,0x1,1,0,mccon::Iblf, Mccon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Clear Bit for IBLF   IBLC. Writing 1 clears the IBLF. Writing 0 has no effect. Returns 0 on read."]
    #[inline(always)]
    pub fn iblc(self) -> crate::common::RegisterFieldBool<18, 1, 0, Mccon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, Mccon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit for IBLF   IBLS. Writing 1 sets the IBLF and triggers an interrupt  if enabled . Writing        0 has no effect. Returns 0 on read."]
    #[inline(always)]
    pub fn ibls(self) -> crate::common::RegisterFieldBool<19, 1, 0, Mccon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, Mccon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt After Last Enable   IALEN. Enable bit for this event."]
    #[inline(always)]
    pub fn ialen(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, mccon::Ialen, Mccon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,mccon::Ialen, Mccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt After Last Flag   IALF. Flag bit for this event."]
    #[inline(always)]
    pub fn ialf(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, mccon::Ialf, Mccon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<21,0x1,1,0,mccon::Ialf, Mccon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Clear Bit for IALF   IALC. Writing 1 clears the IALF. Writing 0 has no effect. Returns 0 on read."]
    #[inline(always)]
    pub fn ialc(self) -> crate::common::RegisterFieldBool<22, 1, 0, Mccon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, Mccon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Bit for IALF   IALS. Writing 1 sets the IALF and triggers an interrupt  if enabled . Writing        0 has no effect. Returns 0 on read."]
    #[inline(always)]
    pub fn ials(self) -> crate::common::RegisterFieldBool<23, 1, 0, Mccon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, Mccon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "TRAIL 2 Injection Enable   T2EN. This bit has to be configured before the transmission of the frame        starts. If set  a new value for the last trailing delay will be injected for the        last data block  as defined with the bit field TRAIL2. If not set  the TRAIL value from the latest BACON will be valid also as the last trailing delay."]
    #[inline(always)]
    pub fn t2en(self) -> crate::common::RegisterFieldBool<30, 1, 0, Mccon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Mccon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Move Counter Enable   MCEN. Enables the Move Counter feature. If enabled  the MCOUNT value is taken        in consideration  otherwise the standard continuous mode is active."]
    #[inline(always)]
    pub fn mcen(self) -> crate::common::RegisterFieldBool<31, 1, 0, Mccon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Mccon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Mccon {
    #[inline(always)]
    fn default() -> Mccon {
        <crate::RegValueT<Mccon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mccon {
    pub struct Tpre2_SPEC;
    pub type Tpre2 = crate::EnumBitfieldStruct<u8, Tpre2_SPEC>;
    impl Tpre2 {
        #[doc = "000 1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 4"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trail2_SPEC;
    pub type Trail2 = crate::EnumBitfieldStruct<u8, Trail2_SPEC>;
    impl Trail2 {
        #[doc = "0 1 unit"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Iblen_SPEC;
    pub type Iblen = crate::EnumBitfieldStruct<u8, Iblen_SPEC>;
    impl Iblen {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enable"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Iblf_SPEC;
    pub type Iblf = crate::EnumBitfieldStruct<u8, Iblf_SPEC>;
    impl Iblf {
        #[doc = "0 No event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Event occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ialen_SPEC;
    pub type Ialen = crate::EnumBitfieldStruct<u8, Ialen_SPEC>;
    impl Ialen {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enable"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ialf_SPEC;
    pub type Ialf = crate::EnumBitfieldStruct<u8, Ialf_SPEC>;
    impl Ialf {
        #[doc = "0 No event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Event occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mixentry_SPEC;
impl crate::sealed::RegSpec for Mixentry_SPEC {
    type DataType = u32;
}
#[doc = "MIX ENTRY Register\n resetvalue={Application Reset:0x0}"]
pub type Mixentry = crate::RegValueT<Mixentry_SPEC>;

impl Mixentry {
    #[doc = "Entry Point to the TxFIFO   E"]
    #[inline(always)]
    pub fn e(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Mixentry_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Mixentry_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Mixentry {
    #[inline(always)]
    fn default() -> Mixentry {
        <crate::RegValueT<Mixentry_SPEC> as RegisterValue<_>>::new(0)
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
pub struct Pisel_SPEC;
impl crate::sealed::RegSpec for Pisel_SPEC {
    type DataType = u32;
}
#[doc = "Port Input Select Register\n resetvalue={Application Reset:0x0}"]
pub type Pisel = crate::RegValueT<Pisel_SPEC>;

impl Pisel {
    #[doc = "Master Mode Receive Input Select. MRIS selects one out of eight MRST receive input lines  used in Master        Mode. Note that not all inputs are used in every device of the family.        Selecting an unused input returns a continuous low value. The following signal sources are available in this product  if supported        by the package"]
    #[inline(always)]
    pub fn mris(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Pisel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Pisel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slave Mode Receive Input Select. SRIS selects one out of eight MTSR receive input lines  used in Slave        Mode. Note that not all inputs are used in every device of the family.        Selecting an unused input returns a continuous low value. The following signal sources are available in this product  if supported        by the package"]
    #[inline(always)]
    pub fn sris(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, Pisel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8, Pisel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slave Mode Clock Input Select. SCIS selects one out of eight module kernel SCLK input lines that is        used as clock input line in slave mode. Note that not all inputs are        used in every device of the family. Selecting an unused input returns a        continuous low value. The following signal sources are available in this product  if supported        by the package"]
    #[inline(always)]
    pub fn scis(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Pisel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Pisel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slave Mode Slave Select Input Selection. The SLSIS must be programmed properly before the slave mode is set with GLOBALCON.MODE and the module is set to RUN mode. The following signal sources are available in this product  if supported by the package"]
    #[inline(always)]
    pub fn slsis(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, Pisel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8, Pisel_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pisel {
    #[inline(always)]
    fn default() -> Pisel {
        <crate::RegValueT<Pisel_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxexit_SPEC;
impl crate::sealed::RegSpec for Rxexit_SPEC {
    type DataType = u32;
}
#[doc = "RX EXIT Register\n resetvalue={Application Reset:0x0}"]
pub type Rxexit = crate::RegValueT<Rxexit_SPEC>;

impl Rxexit {
    #[doc = "Read Point from the RxFIFO   E"]
    #[inline(always)]
    pub fn e(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Rxexit_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Rxexit_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Rxexit {
    #[inline(always)]
    fn default() -> Rxexit {
        <crate::RegValueT<Rxexit_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxexitd_SPEC;
impl crate::sealed::RegSpec for Rxexitd_SPEC {
    type DataType = u32;
}
#[doc = "RX EXIT Debug Register\n resetvalue={Application Reset:0x0}"]
pub type Rxexitd = crate::RegValueT<Rxexitd_SPEC>;

impl Rxexitd {
    #[doc = "Read Point from the RxFIFO   E"]
    #[inline(always)]
    pub fn e(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Rxexitd_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Rxexitd_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Rxexitd {
    #[inline(always)]
    fn default() -> Rxexitd {
        <crate::RegValueT<Rxexitd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssoc_SPEC;
impl crate::sealed::RegSpec for Ssoc_SPEC {
    type DataType = u32;
}
#[doc = "Slave Select Output Control Register\n resetvalue={Application Reset:0x0}"]
pub type Ssoc = crate::RegValueT<Ssoc_SPEC>;

impl Ssoc {
    #[doc = "Active Output Level for the SLSO Outputs   AOL. The idle level is the inverted one.   8220 0  8221  at certain position means active low level for the corresponding        SLSO.   8220 1  8221  means active high."]
    #[inline(always)]
    pub fn aol(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Ssoc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Ssoc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Bits for the SLSO Outputs   OEN. In disabled state the SLSO output drives the idle level as defined by        the AOL bit field.   8220 0  8221  at certain position means that the corresponding SLSO is disabled.   8220 1  8221  means enabled."]
    #[inline(always)]
    pub fn oen(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Ssoc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Ssoc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ssoc {
    #[inline(always)]
    fn default() -> Ssoc {
        <crate::RegValueT<Ssoc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status_SPEC;
impl crate::sealed::RegSpec for Status_SPEC {
    type DataType = u32;
}
#[doc = "Status Register\n resetvalue={Application Reset:0x0}"]
pub type Status = crate::RegValueT<Status_SPEC>;

impl Status {
    #[doc = "Sticky Flags Signalling Errors   ERRORFLAGS. Writing 1 sets the error Flag and triggers an error interrupt  if        enabled. Writing 0 has no effect."]
    #[inline(always)]
    pub fn errorflags(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1ff,
        1,
        0,
        status::Errorflags,
        Status_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1ff,
            1,
            0,
            status::Errorflags,
            Status_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Transmit Interrupt Request Flag   TXF. Flags an occurrence of a request to feed the TXFIFO  which is generated        when an element is fetched from the FIFO  and the FIFO filling level is        equal or less than the set threshold level. Writing 1 sets the flag and triggers an interrupt if GLOBALCON1 .TXEN  160    160 1. Writing 0 has no effect."]
    #[inline(always)]
    pub fn txf(self) -> crate::common::RegisterFieldBool<9, 1, 0, Status_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Status_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive Interrupt Request Flag   RXF. Flags an occurrence of a request to empty the RXFIFO  which is generated        when an element is written into the FIFO  and the FIFO filling level is        equal or greater than the set threshold level. Writing 1 sets the flag and triggers an interrupt if GLOBALCON1 .RXEN  160    160 1. Writing 0 has no effect."]
    #[inline(always)]
    pub fn rxf(self) -> crate::common::RegisterFieldBool<10, 1, 0, Status_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Status_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Phase Transition 1 Flag   PT1F. Flags an occurrence of a PT1 event  as selected with the GLOBALCON1 .PT1         and triggers an interrupt if GLOBALCON1 .PT1EN  160    160 1. Writing 1 sets the flag and triggers an error interrupt. Writing 0 has no effect."]
    #[inline(always)]
    pub fn pt1f(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Status_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Status_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Phase Transition 2 Flag   PT2F. In master mode  flags an occurrence of a PT2 event  as selected with the GLOBALCON1 .PT2         and triggers an interrupt if GLOBALCON1 .PT2EN  160    160 1. In slave mode  set by the SLSI deactivated event. Writing 1 sets the flag and triggers an error interrupt. Writing 0 has no effect."]
    #[inline(always)]
    pub fn pt2f(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Status_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Status_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "User Interrupt Request Flag   USRF. Flags an occurrence of an USR event. Writing 1 sets the flag and triggers an interrupt if GLOBALCON1 .USREN  160    160 1. Writing 0 has no effect."]
    #[inline(always)]
    pub fn usrf(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Status_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Status_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "TXFIFO Filling Level   TXFIFOLEVEL. Shows how many entries in the TXFIFO are waiting for transmission"]
    #[inline(always)]
    pub fn txfifolevel(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        status::Txfifolevel,
        Status_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            status::Txfifolevel,
            Status_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "RXFIFO Filling Level   RXFIFOLEVEL. Shows how many entries in the RXFIFO are waiting for software to move        them to RAM"]
    #[inline(always)]
    pub fn rxfifolevel(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x7,
        1,
        0,
        status::Rxfifolevel,
        Status_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            19,
            0x7,
            1,
            0,
            status::Rxfifolevel,
            Status_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Currently Active Slave Select Flag   SLAVESEL. Displays the currently active slave select."]
    #[inline(always)]
    pub fn slavesel(
        self,
    ) -> crate::common::RegisterField<22, 0xf, 1, 0, u8, Status_SPEC, crate::common::R> {
        crate::common::RegisterField::<22,0xf,1,0,u8, Status_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Received Parity Value   RPV. Displays the last received parity bit  if parity was enabled. Else if        the parity is disabled  reads 0."]
    #[inline(always)]
    pub fn rpv(self) -> crate::common::RegisterFieldBool<26, 1, 0, Status_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Status_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmitted Parity Value   TPV. Displays the last transmitted parity bit  if parity was enabled. Else 0."]
    #[inline(always)]
    pub fn tpv(self) -> crate::common::RegisterFieldBool<27, 1, 0, Status_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Status_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Flags the ongoing phase   PHASE. Displays the current phase number. Relevant only in master mode. In        slave mode this bit field indicates always 0. Not 0 means busy."]
    #[inline(always)]
    pub fn phase(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, status::Phase, Status_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<28,0xf,1,0,status::Phase, Status_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Status {
    #[inline(always)]
    fn default() -> Status {
        <crate::RegValueT<Status_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod status {
    pub struct Errorflags_SPEC;
    pub type Errorflags = crate::EnumBitfieldStruct<u16, Errorflags_SPEC>;
    impl Errorflags {
        #[doc = "000000000 No Error"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "000000001 Parity Error"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "000000010 Unexpected Configuration Error"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "000000100 Baud Rate Error  slave mode"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "000001000 TXFIFO overflow  software error"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "000010000 TXFIFO underflow  slave mode"]
        pub const CONST_1616: Self = Self::new(16);
        #[doc = "000100000 RXFIFO overflow"]
        pub const CONST_3232: Self = Self::new(32);
        #[doc = "001000000 RXFIFO underflow  software error"]
        pub const CONST_6464: Self = Self::new(64);
        #[doc = "010000000 EXPECT time out error"]
        pub const CONST_128128: Self = Self::new(128);
        #[doc = "100000000 SLSI misplaced inactivation  slave mode"]
        pub const CONST_256256: Self = Self::new(256);
    }
    pub struct Txfifolevel_SPEC;
    pub type Txfifolevel = crate::EnumBitfieldStruct<u8, Txfifolevel_SPEC>;
    impl Txfifolevel {
        #[doc = "000 0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 1"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 3"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 4"]
        pub const CONST_44: Self = Self::new(4);
    }
    pub struct Rxfifolevel_SPEC;
    pub type Rxfifolevel = crate::EnumBitfieldStruct<u8, Rxfifolevel_SPEC>;
    impl Rxfifolevel {
        #[doc = "000 0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 1"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 3"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 4"]
        pub const CONST_44: Self = Self::new(4);
    }
    pub struct Phase_SPEC;
    pub type Phase = crate::EnumBitfieldStruct<u8, Phase_SPEC>;
    impl Phase {
        #[doc = "0000 Wait"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "0001 Idle A"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "0010 Idle B"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "0011 Lead"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "0100 Data"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "0101 Trail"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "0110 Expect"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "0111 Lead Strobe"]
        pub const CONST_77: Self = Self::new(7);
        #[doc = "1000 Trail Strobe"]
        pub const CONST_88: Self = Self::new(8);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status1_SPEC;
impl crate::sealed::RegSpec for Status1_SPEC {
    type DataType = u32;
}
#[doc = "Status Register 1\n resetvalue={Application Reset:0x0}"]
pub type Status1 = crate::RegValueT<Status1_SPEC>;

impl Status1 {
    #[doc = "Number of bits shifted out   BITCOUNT. Supports up to 256 bits. A BITCOUNT value of greater than 0 indicates that a transmission is in progress. The value is not accurate  it may be lower than the number of bits shifted. After transmission of the last bit  BITCOUNT is set to zero."]
    #[inline(always)]
    pub fn bitcount(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Status1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Status1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Baud Rate Deviation Enable   BRDEN. Enables the signal path."]
    #[inline(always)]
    pub fn brden(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, status1::Brden, Status1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,status1::Brden, Status1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Baud Rate Deviation Flag   BRD. Shows if baud rate deviation has been detected. Write of 1 sets the bit        and raises the event per software. Write of 0 has no effect."]
    #[inline(always)]
    pub fn brd(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, status1::Brd, Status1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,status1::Brd, Status1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Spike Detection Enable   SPDEN. Enables the signal path."]
    #[inline(always)]
    pub fn spden(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, status1::Spden, Status1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,status1::Spden, Status1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Spike Detection Flag   SPD. Shows if spike has been detected. Write of 1 sets the bit and raises the        event per software. Write of 0 has no effect."]
    #[inline(always)]
    pub fn spd(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, status1::Spd, Status1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,status1::Spd, Status1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Status1 {
    #[inline(always)]
    fn default() -> Status1 {
        <crate::RegValueT<Status1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod status1 {
    pub struct Brden_SPEC;
    pub type Brden = crate::EnumBitfieldStruct<u8, Brden_SPEC>;
    impl Brden {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Brd_SPEC;
    pub type Brd = crate::EnumBitfieldStruct<u8, Brd_SPEC>;
    impl Brd {
        #[doc = "0 no event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 event detected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Spden_SPEC;
    pub type Spden = crate::EnumBitfieldStruct<u8, Spden_SPEC>;
    impl Spden {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Spd_SPEC;
    pub type Spd = crate::EnumBitfieldStruct<u8, Spd_SPEC>;
    impl Spd {
        #[doc = "0 No event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Event detected"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xxlcon_SPEC;
impl crate::sealed::RegSpec for Xxlcon_SPEC {
    type DataType = u32;
}
#[doc = "Extra Large Data Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Xxlcon = crate::RegValueT<Xxlcon_SPEC>;

impl Xxlcon {
    #[doc = "Extended Data Length   XDL. Defines the length of the data block in bytes in range of 2 to 65536.        Overrides BACON .DL        when BACON .BYTE 1        and BACON .DL 0."]
    #[inline(always)]
    pub fn xdl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, xxlcon::Xdl, Xxlcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,xxlcon::Xdl, Xxlcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extended Data Length   BYTECOUNT. In the XXL mode  shows the current state of the internal byte down        counter  bytes remaining to be sent . In short and long modes  the value        of this bit field is don  8217 t care."]
    #[inline(always)]
    pub fn bytecount(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xffff,
        1,
        0,
        xxlcon::Bytecount,
        Xxlcon_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0xffff,
            1,
            0,
            xxlcon::Bytecount,
            Xxlcon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Xxlcon {
    #[inline(always)]
    fn default() -> Xxlcon {
        <crate::RegValueT<Xxlcon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod xxlcon {
    pub struct Xdl_SPEC;
    pub type Xdl = crate::EnumBitfieldStruct<u8, Xdl_SPEC>;
    impl Xdl {
        #[doc = "0 2 bytes"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Bytecount_SPEC;
    pub type Bytecount = crate::EnumBitfieldStruct<u8, Bytecount_SPEC>;
    impl Bytecount {
        #[doc = "0 0 bytes"]
        pub const CONST_00: Self = Self::new(0);
    }
}
