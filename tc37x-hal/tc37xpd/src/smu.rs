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
#[doc = r"SMU"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smu(pub(super) *mut u8);
unsafe impl core::marker::Send for Smu {}
unsafe impl core::marker::Sync for Smu {}
impl Smu {
    #[doc = "SMU core Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2044usize)) }
    }

    #[doc = "Alarm Debug Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn ad(&self) -> [crate::common::Reg<self::Ad_SPEC, crate::common::R>; 12] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x2cusize)),
            ]
        }
    }

    #[doc = "Alarm Executed Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn aex(&self) -> crate::common::Reg<self::Aex_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(112usize)) }
    }

    #[doc = "Alarm Executed Status Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn aexclr(&self) -> crate::common::Reg<self::Aexclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(116usize)) }
    }

    #[doc = "Alarm and Fault Counter\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn afcnt(&self) -> crate::common::Reg<self::Afcnt_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "Alarm Global Configuration\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn agc(&self) -> crate::common::Reg<self::Agc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }

    #[doc = "Alarm Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn agi(&self) -> [crate::common::Reg<self::AGi_SPEC, crate::common::RW>; 12] {
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
            ]
        }
    }

    #[doc = "SMU core FSP Configuration Register\n resetvalue={Application Reset:0x0,Application Reset:0x30000}"]
    #[inline(always)]
    pub const fn agifsp(&self) -> [crate::common::Reg<self::AGiFsp_SPEC, crate::common::RW>; 12] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x190usize + 0x2cusize)),
            ]
        }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Command Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cmd(&self) -> crate::common::Reg<self::Cmd_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "Debug Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn dbg(&self) -> crate::common::Reg<self::Dbg_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }

    #[doc = "Fault Signaling Protocol\n resetvalue={PowerOn Reset:0x3FFF00}"]
    #[inline(always)]
    pub const fn fsp(&self) -> crate::common::Reg<self::Fsp_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x089C001}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Key Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn keys(&self) -> crate::common::Reg<self::Keys_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2024usize)) }
    }

    #[doc = "Port Control\n resetvalue={PowerOn Reset:0x08000}"]
    #[inline(always)]
    pub const fn pctl(&self) -> crate::common::Reg<self::Pctl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "Register Monitor Control\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rmctl(&self) -> crate::common::Reg<self::Rmctl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(768usize)) }
    }

    #[doc = "Register Monitor Error Flags\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rmef(&self) -> crate::common::Reg<self::Rmef_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(772usize)) }
    }

    #[doc = "Register Monitor Self Test Status\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rmsts(&self) -> crate::common::Reg<self::Rmsts_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(776usize)) }
    }

    #[doc = "Recovery Timer 0 Alarm Configuration 0\n resetvalue={Application Reset:0x0A80108}"]
    #[inline(always)]
    pub const fn rtac00(&self) -> crate::common::Reg<self::Rtac00_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }

    #[doc = "Recovery Timer 0 Alarm Configuration 1\n resetvalue={Application Reset:0x0C800B8}"]
    #[inline(always)]
    pub const fn rtac01(&self) -> crate::common::Reg<self::Rtac01_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(100usize)) }
    }

    #[doc = "Recovery Timer 1 Alarm Configuration 0\n resetvalue={Application Reset:0x0E800D8}"]
    #[inline(always)]
    pub const fn rtac10(&self) -> crate::common::Reg<self::Rtac10_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(104usize)) }
    }

    #[doc = "Recovery Timer 1 Alarm Configuration 1\n resetvalue={Application Reset:0x0F800F8}"]
    #[inline(always)]
    pub const fn rtac11(&self) -> crate::common::Reg<self::Rtac11_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(108usize)) }
    }

    #[doc = "Recovery Timer Configuration\n resetvalue={Application Reset:0x3FFF03}"]
    #[inline(always)]
    pub const fn rtc(&self) -> crate::common::Reg<self::Rtc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }

    #[doc = "Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sts(&self) -> crate::common::Reg<self::Sts_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "AGRP"]
    #[inline(always)]
    pub fn agicfj(self) -> [self::AGiCFj; 12] {
        unsafe {
            [
                self::AGiCFj(self.0.add(0x100usize + 0x0usize)),
                self::AGiCFj(self.0.add(0x100usize + 0x4usize)),
                self::AGiCFj(self.0.add(0x100usize + 0x8usize)),
                self::AGiCFj(self.0.add(0x100usize + 0xcusize)),
                self::AGiCFj(self.0.add(0x100usize + 0x10usize)),
                self::AGiCFj(self.0.add(0x100usize + 0x14usize)),
                self::AGiCFj(self.0.add(0x100usize + 0x18usize)),
                self::AGiCFj(self.0.add(0x100usize + 0x1cusize)),
                self::AGiCFj(self.0.add(0x100usize + 0x20usize)),
                self::AGiCFj(self.0.add(0x100usize + 0x24usize)),
                self::AGiCFj(self.0.add(0x100usize + 0x28usize)),
                self::AGiCFj(self.0.add(0x100usize + 0x2cusize)),
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
#[doc = "SMU core Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Accen0 = crate::RegValueT<Accen0_SPEC>;

impl Accen0 {
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, accen0::En0, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,accen0::En0, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, accen0::En1, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,accen0::En1, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, accen0::En2, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,accen0::En2, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, accen0::En3, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,accen0::En3, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, accen0::En4, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,accen0::En4, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, accen0::En5, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,accen0::En5, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, accen0::En6, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,accen0::En6, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, accen0::En7, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,accen0::En7, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, accen0::En8, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,accen0::En8, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, accen0::En9, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,accen0::En9, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, accen0::En10, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,accen0::En10, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, accen0::En11, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,accen0::En11, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, accen0::En12, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,accen0::En12, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, accen0::En13, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,accen0::En13, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, accen0::En14, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,accen0::En14, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, accen0::En15, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,accen0::En15, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, accen0::En16, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,accen0::En16, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, accen0::En17, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,accen0::En17, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, accen0::En18, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,accen0::En18, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, accen0::En19, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,accen0::En19, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, accen0::En20, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,accen0::En20, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, accen0::En21, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,accen0::En21, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, accen0::En22, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,accen0::En22, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, accen0::En23, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,accen0::En23, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, accen0::En24, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,accen0::En24, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, accen0::En25, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,accen0::En25, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, accen0::En26, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,accen0::En26, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, accen0::En27, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,accen0::En27, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, accen0::En28, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,accen0::En28, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, accen0::En29, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,accen0::En29, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, accen0::En30, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,accen0::En30, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
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
pub struct Ad_SPEC;
impl crate::sealed::RegSpec for Ad_SPEC {
    type DataType = u32;
}
#[doc = "Alarm Debug Register\n resetvalue={PowerOn Reset:0x0}"]
pub type Ad = crate::RegValueT<Ad_SPEC>;

impl Ad {
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ad::Df0, Ad_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,ad::Df0, Ad_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ad::Df1, Ad_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,ad::Df1, Ad_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ad::Df2, Ad_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,ad::Df2, Ad_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ad::Df4, Ad_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x1,1,0,ad::Df4, Ad_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ad::Df5, Ad_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x1,1,0,ad::Df5, Ad_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, ad::Df6, Ad_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x1,1,0,ad::Df6, Ad_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, ad::Df7, Ad_SPEC, crate::common::R> {
        crate::common::RegisterField::<7,0x1,1,0,ad::Df7, Ad_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, ad::Df8, Ad_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x1,1,0,ad::Df8, Ad_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, ad::Df9, Ad_SPEC, crate::common::R> {
        crate::common::RegisterField::<9,0x1,1,0,ad::Df9, Ad_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, ad::Df10, Ad_SPEC, crate::common::R> {
        crate::common::RegisterField::<10,0x1,1,0,ad::Df10, Ad_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, ad::Df11, Ad_SPEC, crate::common::R> {
        crate::common::RegisterField::<11,0x1,1,0,ad::Df11, Ad_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, ad::Df12, Ad_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x1,1,0,ad::Df12, Ad_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, ad::Df13, Ad_SPEC, crate::common::R> {
        crate::common::RegisterField::<13,0x1,1,0,ad::Df13, Ad_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, ad::Df14, Ad_SPEC, crate::common::R> {
        crate::common::RegisterField::<14,0x1,1,0,ad::Df14, Ad_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, ad::Df22, Ad_SPEC, crate::common::R> {
        crate::common::RegisterField::<22,0x1,1,0,ad::Df22, Ad_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, ad::Df23, Ad_SPEC, crate::common::R> {
        crate::common::RegisterField::<23,0x1,1,0,ad::Df23, Ad_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Diagnosis flag for alarm 31 belonging to alarm group i.   DF31. The diagnosis registers make a snapshot of the alarm group status        registers when either the executed alarm action is a reset or a state        machine transition to FAULT state takes place."]
    #[inline(always)]
    pub fn df24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, ad::Df24, Ad_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x1,1,0,ad::Df24, Ad_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ad {
    #[inline(always)]
    fn default() -> Ad {
        <crate::RegValueT<Ad_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ad {
    pub struct Df0_SPEC;
    pub type Df0 = crate::EnumBitfieldStruct<u8, Df0_SPEC>;
    impl Df0 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Df1_SPEC;
    pub type Df1 = crate::EnumBitfieldStruct<u8, Df1_SPEC>;
    impl Df1 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Df2_SPEC;
    pub type Df2 = crate::EnumBitfieldStruct<u8, Df2_SPEC>;
    impl Df2 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Df4_SPEC;
    pub type Df4 = crate::EnumBitfieldStruct<u8, Df4_SPEC>;
    impl Df4 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Df5_SPEC;
    pub type Df5 = crate::EnumBitfieldStruct<u8, Df5_SPEC>;
    impl Df5 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Df6_SPEC;
    pub type Df6 = crate::EnumBitfieldStruct<u8, Df6_SPEC>;
    impl Df6 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Df7_SPEC;
    pub type Df7 = crate::EnumBitfieldStruct<u8, Df7_SPEC>;
    impl Df7 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Df8_SPEC;
    pub type Df8 = crate::EnumBitfieldStruct<u8, Df8_SPEC>;
    impl Df8 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Df9_SPEC;
    pub type Df9 = crate::EnumBitfieldStruct<u8, Df9_SPEC>;
    impl Df9 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Df10_SPEC;
    pub type Df10 = crate::EnumBitfieldStruct<u8, Df10_SPEC>;
    impl Df10 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Df11_SPEC;
    pub type Df11 = crate::EnumBitfieldStruct<u8, Df11_SPEC>;
    impl Df11 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Df12_SPEC;
    pub type Df12 = crate::EnumBitfieldStruct<u8, Df12_SPEC>;
    impl Df12 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Df13_SPEC;
    pub type Df13 = crate::EnumBitfieldStruct<u8, Df13_SPEC>;
    impl Df13 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Df14_SPEC;
    pub type Df14 = crate::EnumBitfieldStruct<u8, Df14_SPEC>;
    impl Df14 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Df22_SPEC;
    pub type Df22 = crate::EnumBitfieldStruct<u8, Df22_SPEC>;
    impl Df22 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Df23_SPEC;
    pub type Df23 = crate::EnumBitfieldStruct<u8, Df23_SPEC>;
    impl Df23 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Df24_SPEC;
    pub type Df24 = crate::EnumBitfieldStruct<u8, Df24_SPEC>;
    impl Df24 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aex_SPEC;
impl crate::sealed::RegSpec for Aex_SPEC {
    type DataType = u32;
}
#[doc = "Alarm Executed Status Register\n resetvalue={Application Reset:0x0}"]
pub type Aex = crate::RegValueT<Aex_SPEC>;

impl Aex {
    #[doc = "IRQ0 Request Status   IRQ0STS. This bit indicates whether a IRQ0 request was serviced or not. This bit        is set by the SMU core after a alarm configured for IRQ0 is detected. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn irq0sts(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, aex::Irq0Sts, Aex_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,aex::Irq0Sts, Aex_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "IRQ1 Request Status   IRQ1STS. This bit indicates whether a IRQ1 request was serviced or not. This bit        is set by the SMU core after a alarm configured for IRQ1 is detected. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn irq1sts(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, aex::Irq1Sts, Aex_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,aex::Irq1Sts, Aex_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "IRQ2 Request Status   IRQ2STS. This bit indicates whether a IRQ2 request was serviced or not. This bit        is set by the SMU core after a alarm configured for IRQ2 is detected. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn irq2sts(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, aex::Irq2Sts, Aex_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,aex::Irq2Sts, Aex_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RST0 Request Status   RST0STS. This bit indicates whether a RST0 request was serviced or not. This bit        is set by the SMU core after a alarm configured for RST0 is detected. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst0sts(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, aex::Rst0Sts, Aex_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1,1,0,aex::Rst0Sts, Aex_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RST1 Request Status   RST1STS. This bit indicates whether a RST1 request was serviced or not. This bit        is set by the SMU core after a alarm configured for RST1 is detected. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst1sts(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, aex::Rst1Sts, Aex_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x1,1,0,aex::Rst1Sts, Aex_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RST2 Request Status   RST2STS. This bit indicates whether a RST2 request was serviced or not. This bit        is set by the SMU core after a alarm configured for RST2 is detected. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst2sts(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, aex::Rst2Sts, Aex_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x1,1,0,aex::Rst2Sts, Aex_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RST3 Request Status   RST3STS. This bit indicates whether a RST3 request was serviced or not. This bit        is set by the SMU core after a alarm configured for RST3 is detected. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst3sts(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, aex::Rst3Sts, Aex_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x1,1,0,aex::Rst3Sts, Aex_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RST4 Request Status   RST4STS. This bit indicates whether a RST4 request was serviced or not. This bit        is set by the SMU core after a alarm configured for RST4 is detected. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst4sts(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, aex::Rst4Sts, Aex_SPEC, crate::common::R> {
        crate::common::RegisterField::<7,0x1,1,0,aex::Rst4Sts, Aex_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RST5 Request Status   RST5STS. This bit indicates whether a RST5 request was serviced or not. This bit        is set by the SMU core after a alarm configured for RST5 is detected. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst5sts(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, aex::Rst5Sts, Aex_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x1,1,0,aex::Rst5Sts, Aex_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "NMI Request Status   NMISTS. This bit indicates whether a NMI request was serviced or not. This bit        is set by the SMU core after a alarm configured for NMI is detected. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn nmists(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, aex::Nmists, Aex_SPEC, crate::common::R> {
        crate::common::RegisterField::<9,0x1,1,0,aex::Nmists, Aex_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EMS Request Status   EMSSTS. This bit indicates whether a EMS request  triggered by an alarm  not        SMU ActivatePES   was serviced or not. This bit is set by the SMU core        after a alarm configured for EMS is detected. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn emssts(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, aex::Emssts, Aex_SPEC, crate::common::R> {
        crate::common::RegisterField::<11,0x1,1,0,aex::Emssts, Aex_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "IRQ0 AEM   IRQ0AEM. This bit indicates that an alarm event is missed. That means that an        alarm has occurred with configuration for IRQ0 Request where this alarm        handler was blocked because of AEX.IRQ0STS. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn irq0aem(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, aex::Irq0Aem, Aex_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x1,1,0,aex::Irq0Aem, Aex_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "IRQ1 AEM   IRQ1AEM. This bit indicates that an alarm event is missed. That means that an        alarm has occurred with configuration for IRQ1 Request where this alarm        handler was blocked because of AEX.IRQ1STS. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn irq1aem(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, aex::Irq1Aem, Aex_SPEC, crate::common::R> {
        crate::common::RegisterField::<17,0x1,1,0,aex::Irq1Aem, Aex_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "IRQ2 AEM   IRQ2AEM. This bit indicates that an alarm event is missed. That means that an        alarm has occurred with configuration for IRQ2 Request where this alarm        handler was blocked because of AEX.IRQ2STS. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn irq2aem(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, aex::Irq2Aem, Aex_SPEC, crate::common::R> {
        crate::common::RegisterField::<18,0x1,1,0,aex::Irq2Aem, Aex_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RST0 AEM   RST0AEM. This bit indicates that an alarm event is missed. That means that an        alarm has occurred with configuration for CPU RST0 Request where this        alarm handler was blocked because of AEX.RST0STS. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst0aem(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, aex::Rst0Aem, Aex_SPEC, crate::common::R> {
        crate::common::RegisterField::<19,0x1,1,0,aex::Rst0Aem, Aex_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RST1 AEM   RST1AEM. This bit indicates that an alarm event is missed. That means that an        alarm has occurred with configuration for CPU RST1 Request where this        alarm handler was blocked because of AEX.RST1STS. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst1aem(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, aex::Rst1Aem, Aex_SPEC, crate::common::R> {
        crate::common::RegisterField::<20,0x1,1,0,aex::Rst1Aem, Aex_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RST2 AEM   RST2AEM. This bit indicates that an alarm event is missed. That means that an        alarm has occurred with configuration for CPU RST2 Request where this        alarm handler was blocked because of AEX.RST2STS. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst2aem(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, aex::Rst2Aem, Aex_SPEC, crate::common::R> {
        crate::common::RegisterField::<21,0x1,1,0,aex::Rst2Aem, Aex_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RST3 AEM   RST3AEM. This bit indicates that an alarm event is missed. That means that an        alarm has occurred with configuration for CPU RST3 Request where this        alarm handler was blocked because of AEX.RST3STS. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst3aem(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, aex::Rst3Aem, Aex_SPEC, crate::common::R> {
        crate::common::RegisterField::<22,0x1,1,0,aex::Rst3Aem, Aex_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RST4 AEM   RST4AEM. This bit indicates that an alarm event is missed. That means that an        alarm has occurred with configuration for CPU RST4 Request where this        alarm handler was blocked because of AEX.RST4STS. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst4aem(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, aex::Rst4Aem, Aex_SPEC, crate::common::R> {
        crate::common::RegisterField::<23,0x1,1,0,aex::Rst4Aem, Aex_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RST5 AEM   RST5AEM. This bit indicates that an alarm event is missed. That means that an        alarm has occurred with configuration for CPU RST5 Request where this        alarm handler was blocked because of AEX.RST5STS. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn rst5aem(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, aex::Rst5Aem, Aex_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x1,1,0,aex::Rst5Aem, Aex_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "NMI AEM   NMIAEM. This bit indicates that an alarm event is missed. That means that an        alarm has occurred with configuration for NMI Request where this alarm        handler was blocked because of AEX.NMISTS. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn nmiaem(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, aex::Nmiaem, Aex_SPEC, crate::common::R> {
        crate::common::RegisterField::<25,0x1,1,0,aex::Nmiaem, Aex_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RST AEM   RSTAEM. This bit indicates that an alarm event is missed. That means that an        alarm has occurred with configuration for RST Request where this        alarm handler was blocked because of AEX.RSTSTS."]
    #[inline(always)]
    pub fn rstaem(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, aex::Rstaem, Aex_SPEC, crate::common::R> {
        crate::common::RegisterField::<26,0x1,1,0,aex::Rstaem, Aex_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EMS AEM   EMSAEM. This bit indicates that an alarm event is missed. That means that an        alarm has occurred with configuration for EMS Request where this alarm        handler was blocked because of AEX.EMSSTS. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related AEXCLR register."]
    #[inline(always)]
    pub fn emsaem(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, aex::Emsaem, Aex_SPEC, crate::common::R> {
        crate::common::RegisterField::<27,0x1,1,0,aex::Emsaem, Aex_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Aex {
    #[inline(always)]
    fn default() -> Aex {
        <crate::RegValueT<Aex_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod aex {
    pub struct Irq0Sts_SPEC;
    pub type Irq0Sts = crate::EnumBitfieldStruct<u8, Irq0Sts_SPEC>;
    impl Irq0Sts {
        #[doc = "0 NO IRQ0 request        was serviced"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 IRQ0 request        was serviced"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Irq1Sts_SPEC;
    pub type Irq1Sts = crate::EnumBitfieldStruct<u8, Irq1Sts_SPEC>;
    impl Irq1Sts {
        #[doc = "0 NO IRQ1 request was serviced"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 IRQ1 request was serviced"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Irq2Sts_SPEC;
    pub type Irq2Sts = crate::EnumBitfieldStruct<u8, Irq2Sts_SPEC>;
    impl Irq2Sts {
        #[doc = "0 NO IRQ2 request was serviced"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 IRQ2 request was serviced"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst0Sts_SPEC;
    pub type Rst0Sts = crate::EnumBitfieldStruct<u8, Rst0Sts_SPEC>;
    impl Rst0Sts {
        #[doc = "0 NO RST0 request was serviced"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 RST0 request was serviced"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst1Sts_SPEC;
    pub type Rst1Sts = crate::EnumBitfieldStruct<u8, Rst1Sts_SPEC>;
    impl Rst1Sts {
        #[doc = "0 NO RST1 request was serviced"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 RST1 request was serviced"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst2Sts_SPEC;
    pub type Rst2Sts = crate::EnumBitfieldStruct<u8, Rst2Sts_SPEC>;
    impl Rst2Sts {
        #[doc = "0 NO RST2 request was serviced"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 RST2 request was serviced"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst3Sts_SPEC;
    pub type Rst3Sts = crate::EnumBitfieldStruct<u8, Rst3Sts_SPEC>;
    impl Rst3Sts {
        #[doc = "0 NO RST3 request was serviced"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 RST3 request was serviced"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst4Sts_SPEC;
    pub type Rst4Sts = crate::EnumBitfieldStruct<u8, Rst4Sts_SPEC>;
    impl Rst4Sts {
        #[doc = "0 NO RST4 request was serviced"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 RST4 request was serviced"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst5Sts_SPEC;
    pub type Rst5Sts = crate::EnumBitfieldStruct<u8, Rst5Sts_SPEC>;
    impl Rst5Sts {
        #[doc = "0 NO RST5 request was serviced"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 RST5 request was serviced"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Nmists_SPEC;
    pub type Nmists = crate::EnumBitfieldStruct<u8, Nmists_SPEC>;
    impl Nmists {
        #[doc = "0 NO NMI request was serviced"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NMI request was serviced"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Emssts_SPEC;
    pub type Emssts = crate::EnumBitfieldStruct<u8, Emssts_SPEC>;
    impl Emssts {
        #[doc = "0 NO EMS request was serviced"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 EMS request was serviced"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Irq0Aem_SPEC;
    pub type Irq0Aem = crate::EnumBitfieldStruct<u8, Irq0Aem_SPEC>;
    impl Irq0Aem {
        #[doc = "0 No alarm is missed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 At least one alarm is missed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Irq1Aem_SPEC;
    pub type Irq1Aem = crate::EnumBitfieldStruct<u8, Irq1Aem_SPEC>;
    impl Irq1Aem {
        #[doc = "0 No alarm is missed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 At least one alarm is missed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Irq2Aem_SPEC;
    pub type Irq2Aem = crate::EnumBitfieldStruct<u8, Irq2Aem_SPEC>;
    impl Irq2Aem {
        #[doc = "0 No alarm is missed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 At least one alarm is missed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst0Aem_SPEC;
    pub type Rst0Aem = crate::EnumBitfieldStruct<u8, Rst0Aem_SPEC>;
    impl Rst0Aem {
        #[doc = "0 No alarm is missed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 At least one alarm is missed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst1Aem_SPEC;
    pub type Rst1Aem = crate::EnumBitfieldStruct<u8, Rst1Aem_SPEC>;
    impl Rst1Aem {
        #[doc = "0 No alarm is missed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 At least one alarm is missed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst2Aem_SPEC;
    pub type Rst2Aem = crate::EnumBitfieldStruct<u8, Rst2Aem_SPEC>;
    impl Rst2Aem {
        #[doc = "0 No alarm is missed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 At least one alarm is missed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst3Aem_SPEC;
    pub type Rst3Aem = crate::EnumBitfieldStruct<u8, Rst3Aem_SPEC>;
    impl Rst3Aem {
        #[doc = "0 No alarm is missed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 At least one alarm is missed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst4Aem_SPEC;
    pub type Rst4Aem = crate::EnumBitfieldStruct<u8, Rst4Aem_SPEC>;
    impl Rst4Aem {
        #[doc = "0 No alarm is missed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 At least one alarm is missed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst5Aem_SPEC;
    pub type Rst5Aem = crate::EnumBitfieldStruct<u8, Rst5Aem_SPEC>;
    impl Rst5Aem {
        #[doc = "0 No alarm is missed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 At least one alarm is missed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Nmiaem_SPEC;
    pub type Nmiaem = crate::EnumBitfieldStruct<u8, Nmiaem_SPEC>;
    impl Nmiaem {
        #[doc = "0 No alarm is missed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 At least one alarm is missed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rstaem_SPEC;
    pub type Rstaem = crate::EnumBitfieldStruct<u8, Rstaem_SPEC>;
    impl Rstaem {
        #[doc = "0 No alarm is missed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 At least one alarm is missed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Emsaem_SPEC;
    pub type Emsaem = crate::EnumBitfieldStruct<u8, Emsaem_SPEC>;
    impl Emsaem {
        #[doc = "0 No alarm is missed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 At least one alarm is missed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aexclr_SPEC;
impl crate::sealed::RegSpec for Aexclr_SPEC {
    type DataType = u32;
}
#[doc = "Alarm Executed Status Clear Register\n resetvalue={Application Reset:0x0}"]
pub type Aexclr = crate::RegValueT<Aexclr_SPEC>;

impl Aexclr {
    #[doc = "IRQ0 Request Status Clear   IRQ0CLR. Read always as 0."]
    #[inline(always)]
    pub fn irq0clr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, aexclr::Irq0Clr, Aexclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x1,1,0,aexclr::Irq0Clr, Aexclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "IRQ1 Request Status Clear   IRQ1CLR. Read always as 0."]
    #[inline(always)]
    pub fn irq1clr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, aexclr::Irq1Clr, Aexclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<1,0x1,1,0,aexclr::Irq1Clr, Aexclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "IRQ2 Request Status Clear   IRQ2CLR. Read always as 0."]
    #[inline(always)]
    pub fn irq2clr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, aexclr::Irq2Clr, Aexclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<2,0x1,1,0,aexclr::Irq2Clr, Aexclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "RST0 Request Status Clear   RST0CLR. Read always as 0."]
    #[inline(always)]
    pub fn rst0clr(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, aexclr::Rst0Clr, Aexclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<3,0x1,1,0,aexclr::Rst0Clr, Aexclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "RST1 Request Status Clear   RST1CLR. Read always as 0."]
    #[inline(always)]
    pub fn rst1clr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, aexclr::Rst1Clr, Aexclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<4,0x1,1,0,aexclr::Rst1Clr, Aexclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "RST2 Request Status Clear   RST2CLR. Read always as 0."]
    #[inline(always)]
    pub fn rst2clr(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, aexclr::Rst2Clr, Aexclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<5,0x1,1,0,aexclr::Rst2Clr, Aexclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "RST3 Request Status Clear   RST3CLR. Read always as 0."]
    #[inline(always)]
    pub fn rst3clr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, aexclr::Rst3Clr, Aexclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<6,0x1,1,0,aexclr::Rst3Clr, Aexclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "RST4 Request Status Clear   RST4CLR. Read always as 0."]
    #[inline(always)]
    pub fn rst4clr(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, aexclr::Rst4Clr, Aexclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<7,0x1,1,0,aexclr::Rst4Clr, Aexclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "RST5 Request Status Clear   RST5CLR. Read always as 0."]
    #[inline(always)]
    pub fn rst5clr(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, aexclr::Rst5Clr, Aexclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0x1,1,0,aexclr::Rst5Clr, Aexclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "NMI Request Status Clear   NMICLR. Read always as 0."]
    #[inline(always)]
    pub fn nmiclr(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, aexclr::Nmiclr, Aexclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<9,0x1,1,0,aexclr::Nmiclr, Aexclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "RST Request Status Clear   RSTCLR. Reads always as 0."]
    #[inline(always)]
    pub fn rstclr(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, aexclr::Rstclr, Aexclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<10,0x1,1,0,aexclr::Rstclr, Aexclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "EMS Request Status Clear   EMSCLR. Read always as 0."]
    #[inline(always)]
    pub fn emsclr(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, aexclr::Emsclr, Aexclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<11,0x1,1,0,aexclr::Emsclr, Aexclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "IRQ0 AEM Status Clear   IRQ0AEMCLR. Read always as 0."]
    #[inline(always)]
    pub fn irq0aemclr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        aexclr::Irq0Aemclr,
        Aexclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            aexclr::Irq0Aemclr,
            Aexclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "IRQ1 AEM Status Clear   IRQ1AEMCLR. Read always as 0."]
    #[inline(always)]
    pub fn irq1aemclr(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        aexclr::Irq1Aemclr,
        Aexclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            aexclr::Irq1Aemclr,
            Aexclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "IRQ2 AEM Status Clear   IRQ2AEMCLR. Read always as 0."]
    #[inline(always)]
    pub fn irq2aemclr(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        aexclr::Irq2Aemclr,
        Aexclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            aexclr::Irq2Aemclr,
            Aexclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "RST0 AEM Status Clear   RST0AEMCLR. Read always as 0."]
    #[inline(always)]
    pub fn rst0aemclr(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        aexclr::Rst0Aemclr,
        Aexclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            aexclr::Rst0Aemclr,
            Aexclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "RST1 AEM Status Clear   RST1AEMCLR. Read always as 0."]
    #[inline(always)]
    pub fn rst1aemclr(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        aexclr::Rst1Aemclr,
        Aexclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            aexclr::Rst1Aemclr,
            Aexclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "RST2 AEM Status Clear   RST2AEMCLR. Read always as 0."]
    #[inline(always)]
    pub fn rst2aemclr(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        aexclr::Rst2Aemclr,
        Aexclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            aexclr::Rst2Aemclr,
            Aexclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "RST3 AEM Status Clear   RST3AEMCLR. Read always as 0."]
    #[inline(always)]
    pub fn rst3aemclr(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        aexclr::Rst3Aemclr,
        Aexclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            aexclr::Rst3Aemclr,
            Aexclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "RST4 AEM Status Clear   RST4AEMCLR. Read always as 0."]
    #[inline(always)]
    pub fn rst4aemclr(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        aexclr::Rst4Aemclr,
        Aexclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            aexclr::Rst4Aemclr,
            Aexclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "RST5 AEM Status Clear   RST5AEMCLR. Read always as 0."]
    #[inline(always)]
    pub fn rst5aemclr(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        aexclr::Rst5Aemclr,
        Aexclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            aexclr::Rst5Aemclr,
            Aexclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "NMI AEM Status Clear   NMIAEMCLR. Read always as 0."]
    #[inline(always)]
    pub fn nmiaemclr(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, aexclr::Nmiaemclr, Aexclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            aexclr::Nmiaemclr,
            Aexclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "RST AEM Status Clear   RSTAEMCLR. Read always as 0."]
    #[inline(always)]
    pub fn rstaemclr(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, aexclr::Rstaemclr, Aexclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            aexclr::Rstaemclr,
            Aexclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "EMS AEM Status Clear   EMSAEMCLR. Read always as 0."]
    #[inline(always)]
    pub fn emsaemclr(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, aexclr::Emsaemclr, Aexclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            aexclr::Emsaemclr,
            Aexclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Aexclr {
    #[inline(always)]
    fn default() -> Aexclr {
        <crate::RegValueT<Aexclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod aexclr {
    pub struct Irq0Clr_SPEC;
    pub type Irq0Clr = crate::EnumBitfieldStruct<u8, Irq0Clr_SPEC>;
    impl Irq0Clr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear IRQ0 request status AEX.IRQ0STAT"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Irq1Clr_SPEC;
    pub type Irq1Clr = crate::EnumBitfieldStruct<u8, Irq1Clr_SPEC>;
    impl Irq1Clr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear IRQ1 request status AEX.IRQ1STAT"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Irq2Clr_SPEC;
    pub type Irq2Clr = crate::EnumBitfieldStruct<u8, Irq2Clr_SPEC>;
    impl Irq2Clr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear IRQ2 request status AEX.IRQ2STAT"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst0Clr_SPEC;
    pub type Rst0Clr = crate::EnumBitfieldStruct<u8, Rst0Clr_SPEC>;
    impl Rst0Clr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear RST0 request status AEX.RST0STAT"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst1Clr_SPEC;
    pub type Rst1Clr = crate::EnumBitfieldStruct<u8, Rst1Clr_SPEC>;
    impl Rst1Clr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear RST1 request status AEX.RST1STAT"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst2Clr_SPEC;
    pub type Rst2Clr = crate::EnumBitfieldStruct<u8, Rst2Clr_SPEC>;
    impl Rst2Clr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear RST2 request status AEX.RST2STAT"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst3Clr_SPEC;
    pub type Rst3Clr = crate::EnumBitfieldStruct<u8, Rst3Clr_SPEC>;
    impl Rst3Clr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear RST3 request status AEX.RST3STAT"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst4Clr_SPEC;
    pub type Rst4Clr = crate::EnumBitfieldStruct<u8, Rst4Clr_SPEC>;
    impl Rst4Clr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear RST4        request status AEX.RST4STAT"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst5Clr_SPEC;
    pub type Rst5Clr = crate::EnumBitfieldStruct<u8, Rst5Clr_SPEC>;
    impl Rst5Clr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear RST5 request status AEX.RST5STAT"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Nmiclr_SPEC;
    pub type Nmiclr = crate::EnumBitfieldStruct<u8, Nmiclr_SPEC>;
    impl Nmiclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear NMI request status AEX.NMISTAT"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rstclr_SPEC;
    pub type Rstclr = crate::EnumBitfieldStruct<u8, Rstclr_SPEC>;
    impl Rstclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear RST request status AEX.RSTSTAT"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Emsclr_SPEC;
    pub type Emsclr = crate::EnumBitfieldStruct<u8, Emsclr_SPEC>;
    impl Emsclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear EMS request status AEX.EMSSTAT"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Irq0Aemclr_SPEC;
    pub type Irq0Aemclr = crate::EnumBitfieldStruct<u8, Irq0Aemclr_SPEC>;
    impl Irq0Aemclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear AEM status AEX.IRQ0AEM"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Irq1Aemclr_SPEC;
    pub type Irq1Aemclr = crate::EnumBitfieldStruct<u8, Irq1Aemclr_SPEC>;
    impl Irq1Aemclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear AEM status AEX.IRQ1AEM"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Irq2Aemclr_SPEC;
    pub type Irq2Aemclr = crate::EnumBitfieldStruct<u8, Irq2Aemclr_SPEC>;
    impl Irq2Aemclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear AEM status AEX.IRQ2AEM"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst0Aemclr_SPEC;
    pub type Rst0Aemclr = crate::EnumBitfieldStruct<u8, Rst0Aemclr_SPEC>;
    impl Rst0Aemclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear AEM status AEX.RST0AEM"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst1Aemclr_SPEC;
    pub type Rst1Aemclr = crate::EnumBitfieldStruct<u8, Rst1Aemclr_SPEC>;
    impl Rst1Aemclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear AEM status AEX.RST1AEM"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst2Aemclr_SPEC;
    pub type Rst2Aemclr = crate::EnumBitfieldStruct<u8, Rst2Aemclr_SPEC>;
    impl Rst2Aemclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear AEM status AEX.RST2AEM"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst3Aemclr_SPEC;
    pub type Rst3Aemclr = crate::EnumBitfieldStruct<u8, Rst3Aemclr_SPEC>;
    impl Rst3Aemclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear AEM status AEX.RST3AEM"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst4Aemclr_SPEC;
    pub type Rst4Aemclr = crate::EnumBitfieldStruct<u8, Rst4Aemclr_SPEC>;
    impl Rst4Aemclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear AEM status AEX.RST4AEM"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst5Aemclr_SPEC;
    pub type Rst5Aemclr = crate::EnumBitfieldStruct<u8, Rst5Aemclr_SPEC>;
    impl Rst5Aemclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear AEM status AEX.RST5AEM"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Nmiaemclr_SPEC;
    pub type Nmiaemclr = crate::EnumBitfieldStruct<u8, Nmiaemclr_SPEC>;
    impl Nmiaemclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear AEM status AEX.NMIAEM"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rstaemclr_SPEC;
    pub type Rstaemclr = crate::EnumBitfieldStruct<u8, Rstaemclr_SPEC>;
    impl Rstaemclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear AEM status AEX.RSTAEM"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Emsaemclr_SPEC;
    pub type Emsaemclr = crate::EnumBitfieldStruct<u8, Emsaemclr_SPEC>;
    impl Emsaemclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear AEM status AEX.EMSAEM"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Afcnt_SPEC;
impl crate::sealed::RegSpec for Afcnt_SPEC {
    type DataType = u32;
}
#[doc = "Alarm and Fault Counter\n resetvalue={PowerOn Reset:0x0}"]
pub type Afcnt = crate::RegValueT<Afcnt_SPEC>;

impl Afcnt {
    #[doc = "Fault Counter.   FCNT. This field is incremented by hardware when the SMU core state machine        goes from the RUN state to the FAULT state  see CROSSREFERENCE  .        The counter value holds if the maximum value is reached."]
    #[inline(always)]
    pub fn fcnt(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Afcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Afcnt_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Alarm Counter.   ACNT. This field is incremented by hardware when the SMU core processes an        internal action related to an alarm event  see CROSSREFERENCE  .        The counter value holds if the maximum value is reached."]
    #[inline(always)]
    pub fn acnt(
        self,
    ) -> crate::common::RegisterField<4, 0xfff, 1, 0, u16, Afcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0xfff,1,0,u16, Afcnt_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Fault Counter Overflow.   FCO. This bit is set by hardware if the FCNT counter reached the maximum        value and an increment condition is present."]
    #[inline(always)]
    pub fn fco(self) -> crate::common::RegisterFieldBool<30, 1, 0, Afcnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Afcnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm Counter Overflow.   ACO. This bit is set by hardware if the ACNT counter reached the maximum        value and an increment condition is present."]
    #[inline(always)]
    pub fn aco(self) -> crate::common::RegisterFieldBool<31, 1, 0, Afcnt_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Afcnt_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Afcnt {
    #[inline(always)]
    fn default() -> Afcnt {
        <crate::RegValueT<Afcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agc_SPEC;
impl crate::sealed::RegSpec for Agc_SPEC {
    type DataType = u32;
}
#[doc = "Alarm Global Configuration\n resetvalue={Application Reset:0x0}"]
pub type Agc = crate::RegValueT<Agc_SPEC>;

impl Agc {
    #[doc = "Interrupt Generation Configuration Set 0   IGCS0. Defines the output value of the interrupt request vector when the alarm        configuration flag selects the interrupt configuration set 0. Enables to        issue an interrupt request to several CPUs  see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn igcs0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Agc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0x7, 1, 0, u8, Agc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Generation Configuration Set 1   IGCS1. Defines the output value of the interrupt request vector when the alarm        configuration flag selects the interrupt configuration set 1. Enables to        issue an interrupt request to several CPUs  see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn igcs1(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, Agc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4, 0x7, 1, 0, u8, Agc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Generation Configuration Set 2   IGCS2. Defines the output value of the interrupt request vector when the alarm        configuration flag selects the interrupt configuration set 2. Enables to        issue an interrupt request to several CPUs  see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn igcs2(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Agc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8, 0x7, 1, 0, u8, Agc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU Reset Configuration Set   RCS. Defines the output value of the CPU reset request vector when the alarm configuration flag selects the CPU Reset Configuration Set. Enables to issue an reset request to several CPUs if required. More complex reset scenarios can be handled by using software interrupts. Setting the bit n to 1 enables issuing a reset request to CPUn."]
    #[inline(always)]
    pub fn rcs(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Agc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Agc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Emergency Stop   PES. This field enables control of the Port Emergency Stop  PES  feature        independently for each internal action. When an action is triggered and        if the corresponding bit  as defined below  is set  the hardware        triggers automatically a port emergency stop request. Each bit of PES is        allocated to an action as follows"]
    #[inline(always)]
    pub fn pes(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, agc::Pes, Agc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,agc::Pes, Agc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable FAULT to RUN State Transition   EFRST. See CROSSREFERENCE chapter for the usage of this field."]
    #[inline(always)]
    pub fn efrst(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, agc::Efrst, Agc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x1,1,0,agc::Efrst, Agc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Agc {
    #[inline(always)]
    fn default() -> Agc {
        <crate::RegValueT<Agc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod agc {
    pub struct Pes_SPEC;
    pub type Pes = crate::EnumBitfieldStruct<u8, Pes_SPEC>;
    impl Pes {
        #[doc = "1 SMU IGCS0        activates PES"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 SMU IGCS1        activates PES"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "4 SMU IGCS2        activates PES"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "8 SMU NMI        activates PES"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "10 SMU CPU RESET        activates PES"]
        pub const CONST_1616: Self = Self::new(16);
    }
    pub struct Efrst_SPEC;
    pub type Efrst = crate::EnumBitfieldStruct<u8, Efrst_SPEC>;
    impl Efrst {
        #[doc = "0 FAULT to RUN State Transition disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FAULT to RUN State Transition enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AGi_SPEC;
impl crate::sealed::RegSpec for AGi_SPEC {
    type DataType = u32;
}
#[doc = "Alarm Status Register\n resetvalue={Application Reset:0x0}"]
pub type AGi = crate::RegValueT<AGi_SPEC>;

impl AGi {
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, agi::Sf0, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,agi::Sf0, AGi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, agi::Sf1, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,agi::Sf1, AGi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, agi::Sf2, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,agi::Sf2, AGi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, agi::Sf4, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,agi::Sf4, AGi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, agi::Sf5, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,agi::Sf5, AGi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, agi::Sf6, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,agi::Sf6, AGi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, agi::Sf7, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,agi::Sf7, AGi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, agi::Sf8, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,agi::Sf8, AGi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, agi::Sf9, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,agi::Sf9, AGi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, agi::Sf10, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x1,1,0,agi::Sf10, AGi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, agi::Sf11, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x1,1,0,agi::Sf11, AGi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, agi::Sf12, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x1,1,0,agi::Sf12, AGi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, agi::Sf13, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x1,1,0,agi::Sf13, AGi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, agi::Sf14, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x1,1,0,agi::Sf14, AGi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, agi::Sf22, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x1,1,0,agi::Sf22, AGi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, agi::Sf23, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<23,0x1,1,0,agi::Sf23, AGi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag for alarm 31 belonging to alarm group i.   SF31"]
    #[inline(always)]
    pub fn sf24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, agi::Sf24, AGi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1,1,0,agi::Sf24, AGi_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for AGi {
    #[inline(always)]
    fn default() -> AGi {
        <crate::RegValueT<AGi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod agi {
    pub struct Sf0_SPEC;
    pub type Sf0 = crate::EnumBitfieldStruct<u8, Sf0_SPEC>;
    impl Sf0 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf1_SPEC;
    pub type Sf1 = crate::EnumBitfieldStruct<u8, Sf1_SPEC>;
    impl Sf1 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf2_SPEC;
    pub type Sf2 = crate::EnumBitfieldStruct<u8, Sf2_SPEC>;
    impl Sf2 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf4_SPEC;
    pub type Sf4 = crate::EnumBitfieldStruct<u8, Sf4_SPEC>;
    impl Sf4 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf5_SPEC;
    pub type Sf5 = crate::EnumBitfieldStruct<u8, Sf5_SPEC>;
    impl Sf5 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf6_SPEC;
    pub type Sf6 = crate::EnumBitfieldStruct<u8, Sf6_SPEC>;
    impl Sf6 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf7_SPEC;
    pub type Sf7 = crate::EnumBitfieldStruct<u8, Sf7_SPEC>;
    impl Sf7 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf8_SPEC;
    pub type Sf8 = crate::EnumBitfieldStruct<u8, Sf8_SPEC>;
    impl Sf8 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf9_SPEC;
    pub type Sf9 = crate::EnumBitfieldStruct<u8, Sf9_SPEC>;
    impl Sf9 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf10_SPEC;
    pub type Sf10 = crate::EnumBitfieldStruct<u8, Sf10_SPEC>;
    impl Sf10 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf11_SPEC;
    pub type Sf11 = crate::EnumBitfieldStruct<u8, Sf11_SPEC>;
    impl Sf11 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf12_SPEC;
    pub type Sf12 = crate::EnumBitfieldStruct<u8, Sf12_SPEC>;
    impl Sf12 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf13_SPEC;
    pub type Sf13 = crate::EnumBitfieldStruct<u8, Sf13_SPEC>;
    impl Sf13 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf14_SPEC;
    pub type Sf14 = crate::EnumBitfieldStruct<u8, Sf14_SPEC>;
    impl Sf14 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf22_SPEC;
    pub type Sf22 = crate::EnumBitfieldStruct<u8, Sf22_SPEC>;
    impl Sf22 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf23_SPEC;
    pub type Sf23 = crate::EnumBitfieldStruct<u8, Sf23_SPEC>;
    impl Sf23 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf24_SPEC;
    pub type Sf24 = crate::EnumBitfieldStruct<u8, Sf24_SPEC>;
    impl Sf24 {
        #[doc = "0 Status flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Status flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AGiFsp_SPEC;
impl crate::sealed::RegSpec for AGiFsp_SPEC {
    type DataType = u32;
}
#[doc = "SMU core FSP Configuration Register\n resetvalue={Application Reset:0x0,Application Reset:0x30000}"]
pub type AGiFsp = crate::RegValueT<AGiFsp_SPEC>;

impl AGiFsp {
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, agifsp::Fe0, AGiFsp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,agifsp::Fe0, AGiFsp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, agifsp::Fe1, AGiFsp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,agifsp::Fe1, AGiFsp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, agifsp::Fe2, AGiFsp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,agifsp::Fe2, AGiFsp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, agifsp::Fe4, AGiFsp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,agifsp::Fe4, AGiFsp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, agifsp::Fe5, AGiFsp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,agifsp::Fe5, AGiFsp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, agifsp::Fe6, AGiFsp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,agifsp::Fe6, AGiFsp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, agifsp::Fe7, AGiFsp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,agifsp::Fe7, AGiFsp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, agifsp::Fe8, AGiFsp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,agifsp::Fe8, AGiFsp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, agifsp::Fe9, AGiFsp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,agifsp::Fe9, AGiFsp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, agifsp::Fe10, AGiFsp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,agifsp::Fe10, AGiFsp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, agifsp::Fe11, AGiFsp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,agifsp::Fe11, AGiFsp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, agifsp::Fe12, AGiFsp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,agifsp::Fe12, AGiFsp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, agifsp::Fe13, AGiFsp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,agifsp::Fe13, AGiFsp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, agifsp::Fe14, AGiFsp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,agifsp::Fe14, AGiFsp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, agifsp::Fe22, AGiFsp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,agifsp::Fe22, AGiFsp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, agifsp::Fe23, AGiFsp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,agifsp::Fe23, AGiFsp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault signaling configuration flag for alarm 31 belonging to alarm group i.   FE31"]
    #[inline(always)]
    pub fn fe24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, agifsp::Fe24, AGiFsp_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,agifsp::Fe24, AGiFsp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for AGiFsp {
    #[inline(always)]
    fn default() -> AGiFsp {
        <crate::RegValueT<AGiFsp_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod agifsp {
    pub struct Fe0_SPEC;
    pub type Fe0 = crate::EnumBitfieldStruct<u8, Fe0_SPEC>;
    impl Fe0 {
        #[doc = "0 FSP disabled        for this alarm event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP enabled for        this alarm event"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe1_SPEC;
    pub type Fe1 = crate::EnumBitfieldStruct<u8, Fe1_SPEC>;
    impl Fe1 {
        #[doc = "0 FSP disabled        for this alarm event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP enabled for        this alarm event"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe2_SPEC;
    pub type Fe2 = crate::EnumBitfieldStruct<u8, Fe2_SPEC>;
    impl Fe2 {
        #[doc = "0 FSP disabled        for this alarm event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP enabled for        this alarm event"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe4_SPEC;
    pub type Fe4 = crate::EnumBitfieldStruct<u8, Fe4_SPEC>;
    impl Fe4 {
        #[doc = "0 FSP disabled        for this alarm event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP enabled for        this alarm event"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe5_SPEC;
    pub type Fe5 = crate::EnumBitfieldStruct<u8, Fe5_SPEC>;
    impl Fe5 {
        #[doc = "0 FSP disabled        for this alarm event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP enabled for        this alarm event"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe6_SPEC;
    pub type Fe6 = crate::EnumBitfieldStruct<u8, Fe6_SPEC>;
    impl Fe6 {
        #[doc = "0 FSP disabled        for this alarm event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP enabled for        this alarm event"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe7_SPEC;
    pub type Fe7 = crate::EnumBitfieldStruct<u8, Fe7_SPEC>;
    impl Fe7 {
        #[doc = "0 FSP disabled        for this alarm event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP enabled for        this alarm event"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe8_SPEC;
    pub type Fe8 = crate::EnumBitfieldStruct<u8, Fe8_SPEC>;
    impl Fe8 {
        #[doc = "0 FSP disabled        for this alarm event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP enabled for        this alarm event"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe9_SPEC;
    pub type Fe9 = crate::EnumBitfieldStruct<u8, Fe9_SPEC>;
    impl Fe9 {
        #[doc = "0 FSP disabled        for this alarm event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP enabled for        this alarm event"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe10_SPEC;
    pub type Fe10 = crate::EnumBitfieldStruct<u8, Fe10_SPEC>;
    impl Fe10 {
        #[doc = "0 FSP disabled        for this alarm event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP enabled for        this alarm event"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe11_SPEC;
    pub type Fe11 = crate::EnumBitfieldStruct<u8, Fe11_SPEC>;
    impl Fe11 {
        #[doc = "0 FSP disabled        for this alarm event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP enabled for        this alarm event"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe12_SPEC;
    pub type Fe12 = crate::EnumBitfieldStruct<u8, Fe12_SPEC>;
    impl Fe12 {
        #[doc = "0 FSP disabled        for this alarm event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP enabled for        this alarm event"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe13_SPEC;
    pub type Fe13 = crate::EnumBitfieldStruct<u8, Fe13_SPEC>;
    impl Fe13 {
        #[doc = "0 FSP disabled        for this alarm event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP enabled for        this alarm event"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe14_SPEC;
    pub type Fe14 = crate::EnumBitfieldStruct<u8, Fe14_SPEC>;
    impl Fe14 {
        #[doc = "0 FSP disabled        for this alarm event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP enabled for        this alarm event"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe22_SPEC;
    pub type Fe22 = crate::EnumBitfieldStruct<u8, Fe22_SPEC>;
    impl Fe22 {
        #[doc = "0 FSP disabled        for this alarm event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP enabled for        this alarm event"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe23_SPEC;
    pub type Fe23 = crate::EnumBitfieldStruct<u8, Fe23_SPEC>;
    impl Fe23 {
        #[doc = "0 FSP disabled        for this alarm event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP enabled for        this alarm event"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe24_SPEC;
    pub type Fe24 = crate::EnumBitfieldStruct<u8, Fe24_SPEC>;
    impl Fe24 {
        #[doc = "0 FSP disabled        for this alarm event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP enabled for        this alarm event"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clc_SPEC;
impl crate::sealed::RegSpec for Clc_SPEC {
    type DataType = u32;
}
#[doc = "Clock Control Register\n resetvalue={Application Reset:0x0}"]
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
    #[doc = "Sleep Mode Enable Control   EDIS. Used to control module  8217 s sleep mode. Sleep Mode is not supported by the safety applications. During the        process of entering and resuming from sleep mode  the intended        processing of alarm events is not guaranteed."]
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
        <crate::RegValueT<Clc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod clc {
    pub struct Disr_SPEC;
    pub type Disr = crate::EnumBitfieldStruct<u8, Disr_SPEC>;
    impl Disr {
        #[doc = "0 Module disable is not requested"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module disable is requested"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Diss_SPEC;
    pub type Diss = crate::EnumBitfieldStruct<u8, Diss_SPEC>;
    impl Diss {
        #[doc = "0 Module is enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module is disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd_SPEC;
impl crate::sealed::RegSpec for Cmd_SPEC {
    type DataType = u32;
}
#[doc = "Command Register\n resetvalue={Application Reset:0x0}"]
pub type Cmd = crate::RegValueT<Cmd_SPEC>;

impl Cmd {
    #[doc = "Implements the SMU core Command Interface.   CMD. See CROSSREFERENCE for the command encoding. Read as 0."]
    #[inline(always)]
    pub fn cmd(self) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Cmd_SPEC, crate::common::W> {
        crate::common::RegisterField::<0, 0xf, 1, 0, u8, Cmd_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Implements the SMU core Command Interface.   ARG. Argument to be used with the command. See CROSSREFERENCE for the argument encoding. Read as 0."]
    #[inline(always)]
    pub fn arg(self) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Cmd_SPEC, crate::common::W> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, Cmd_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Cmd {
    #[inline(always)]
    fn default() -> Cmd {
        <crate::RegValueT<Cmd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbg_SPEC;
impl crate::sealed::RegSpec for Dbg_SPEC {
    type DataType = u32;
}
#[doc = "Debug Register\n resetvalue={PowerOn Reset:0x0}"]
pub type Dbg = crate::RegValueT<Dbg_SPEC>;

impl Dbg {
    #[doc = "Running state of the SMU core State Machine   SSM"]
    #[inline(always)]
    pub fn ssm(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, dbg::Ssm, Dbg_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3,1,0,dbg::Ssm, Dbg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Dbg {
    #[inline(always)]
    fn default() -> Dbg {
        <crate::RegValueT<Dbg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dbg {
    pub struct Ssm_SPEC;
    pub type Ssm = crate::EnumBitfieldStruct<u8, Ssm_SPEC>;
    impl Ssm {
        #[doc = "0 START state"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 RUN state"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 FAULT state"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 unspecified        state"]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsp_SPEC;
impl crate::sealed::RegSpec for Fsp_SPEC {
    type DataType = u32;
}
#[doc = "Fault Signaling Protocol\n resetvalue={PowerOn Reset:0x3FFF00}"]
pub type Fsp = crate::RegValueT<Fsp_SPEC>;

impl Fsp {
    #[doc = "Prescaler1   PRE1. Dividing factor to apply to the reference clock fBACK. It is assumed        that the maximal value for fBACK is 100 Mhz with a precision of 5 . The        divided clock is used as reference to generate the timing of the fault        signaling protocol fault state. The frequency of the divided clock         called FSMU FS  is defined as follows"]
    #[inline(always)]
    pub fn pre1(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, fsp::Pre1, Fsp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,fsp::Pre1, Fsp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Prescaler2   PRE2. Dividing factor to apply to the reference clock fBACK in order to        generate the timing of the fault free state for the dynamic dual rail        and time switching modes of the fault signaling protocol. The frequency        of the divided clock  called FSMU FFS  is defined as follows"]
    #[inline(always)]
    pub fn pre2(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, fsp::Pre2, Fsp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x3,1,0,fsp::Pre2, Fsp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault Signaling Protocol configuration   MODE"]
    #[inline(always)]
    pub fn mode(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, fsp::Mode, Fsp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,fsp::Mode, Fsp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Emergency Stop  PES    PES. When this bit is set a Port Emergency Stop is automatically requested        when an alarm event configured to start the Fault Signaling Protocol is        detected."]
    #[inline(always)]
    pub fn pes(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, fsp::Pes, Fsp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,fsp::Pes, Fsp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Specifies the FSP fault state duration   TFSP LOW. TFSP FS  TFSP HIGH  amp  TPSP LOW. TFSP LOW shall be specified as a number        of FSMU FS ticks. TFSP LOW is defined so that the minimum duration is        greater than 250 us. It can not be changed by software."]
    #[inline(always)]
    pub fn tfsp_low(
        self,
    ) -> crate::common::RegisterField<8, 0x3fff, 1, 0, u16, Fsp_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3fff,1,0,u16, Fsp_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Specifies the FSP fault state duration   TFSP HIGH. TFSP FS  TFSP HIGH  amp  TPSP LOW. TFSP HIGH shall be specified as a number        of FSMU FS ticks. TFSP HIGH and PRE1 shall enable to configure a fault        state duration of 500 ms."]
    #[inline(always)]
    pub fn tfsp_high(
        self,
    ) -> crate::common::RegisterField<22, 0x3ff, 1, 0, u16, Fsp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3ff,1,0,u16, Fsp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Fsp {
    #[inline(always)]
    fn default() -> Fsp {
        <crate::RegValueT<Fsp_SPEC> as RegisterValue<_>>::new(4194048)
    }
}
pub mod fsp {
    pub struct Pre1_SPEC;
    pub type Pre1 = crate::EnumBitfieldStruct<u8, Pre1_SPEC>;
    impl Pre1 {
        #[doc = "0 reference clock        frequency divided by 2"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 reference clock        frequency divided by 4"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 reference clock        frequency divided by 8"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 reference clock        frequency divided by 16"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "4 reference clock        frequency divided by 32"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "5 reference clock        frequency divided by 64"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "6 reference clock frequency divided by 128"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "7 reference clock        frequency divided by 256"]
        pub const CONST_77: Self = Self::new(7);
    }
    pub struct Pre2_SPEC;
    pub type Pre2 = crate::EnumBitfieldStruct<u8, Pre2_SPEC>;
    impl Pre2 {
        #[doc = "0 reference clock        frequency divided by 512"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 reference clock frequency divided by 1024"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 reference clock        frequency divided by 2048"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 reference clock        frequency divided by 4096"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Mode_SPEC;
    pub type Mode = crate::EnumBitfieldStruct<u8, Mode_SPEC>;
    impl Mode {
        #[doc = "0 Bi stable        protocol"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Dual Rail        protocol"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Time switching        protocol"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Pes_SPEC;
    pub type Pes = crate::EnumBitfieldStruct<u8, Pes_SPEC>;
    impl Pes {
        #[doc = "0 Port Emergency        Stop disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Port Emergency        Stop enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x089C001}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MOD REV. This bit field defines the module revision number. The value of a module        revision starts with 01H  first revision ."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MOD TYPE. The bit field is set to C0H which defines the module as a 32 bit module."]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MOD NUMBER. This bit field defines a module identification number. The value for the SMU module is 0089H."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(9027585)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keys_SPEC;
impl crate::sealed::RegSpec for Keys_SPEC {
    type DataType = u32;
}
#[doc = "Key Register\n resetvalue={Application Reset:0x0}"]
pub type Keys = crate::RegValueT<Keys_SPEC>;

impl Keys {
    #[doc = "Configuration Lock   CFGLCK. The SMU core configuration is only possible if this field is set to        0xBC. Refer to CROSSREFERENCE for the list of registers controlled by this field."]
    #[inline(always)]
    pub fn cfglck(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Keys_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Keys_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Permanent Lock   PERLCK. If this field is set to 0xFF  no further configuration of the SMU core        is possible. Refer to CROSSREFERENCE for the list of registers controlled by this field."]
    #[inline(always)]
    pub fn perlck(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Keys_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Keys_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Keys {
    #[inline(always)]
    fn default() -> Keys {
        <crate::RegValueT<Keys_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Suspend State   SUSSTA. Read as 0  must be written with 0."]
    #[inline(always)]
    pub fn sussta(self) -> crate::common::RegisterFieldBool<29, 1, 0, Ocs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Ocs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
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
        #[doc = "0 No Trigger Set        output"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 TS16 SMU"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tgb_SPEC;
    pub type Tgb = crate::EnumBitfieldStruct<u8, Tgb_SPEC>;
    impl Tgb {
        #[doc = "0 Trigger Set is        output on OTGB0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Trigger Set is        output on OTGB1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sus_SPEC;
    pub type Sus = crate::EnumBitfieldStruct<u8, Sus_SPEC>;
    impl Sus {
        #[doc = "0 Will not        suspend"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Hard suspend.        Clock is switched off immediately."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Soft suspend."]
        pub const CONST_22: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pctl_SPEC;
impl crate::sealed::RegSpec for Pctl_SPEC {
    type DataType = u32;
}
#[doc = "Port Control\n resetvalue={PowerOn Reset:0x08000}"]
pub type Pctl = crate::RegValueT<Pctl_SPEC>;

impl Pctl {
    #[doc = "Port Direction.   HWDIR. This bitfield directly controls the value of the FSP DIR output signal. Also        refer to the General Purpose I O Ports chapter for the HW DIR signal        specification."]
    #[inline(always)]
    pub fn hwdir(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, pctl::Hwdir, Pctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,pctl::Hwdir, Pctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Enable   HWEN. This bitfied directly controls the value of the FSP EN output signal. When        set to 11b the port output is directly driven by SMU core  FSP 1 0  . Also        refer to the General Purpose I O Ports chapter for the HW EN signal        specification."]
    #[inline(always)]
    pub fn hwen(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, pctl::Hwen, Pctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,pctl::Hwen, Pctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Glitch Filter for ErrorPin SMU FSP0 to SCU enable   GFSCU EN"]
    #[inline(always)]
    pub fn gfscu_en(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, pctl::GfscuEn, Pctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,pctl::GfscuEn, Pctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Glitch Filter for ErrorPin SMU FSP0 to register SMU STS enable   GFSTS EN"]
    #[inline(always)]
    pub fn gfsts_en(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, pctl::GfstsEn, Pctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,pctl::GfstsEn, Pctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PAD Configuration Select   PCS. This bit controls the latching of the SMU core FSP  Error Pin  PAD        configuration signals to ensure that upon an application reset or system        reset the SMU core FSP  Error Pin  PAD configuration is not affected.        This field is only reset by power on reset. Only with the first transition from 0 to 1 of          this field the SMU core FSP is operational. Any further configuration          change in this bit field has no effect to the hardware . The fields HWDIR  HWEN and PCS shall be          configured with a single software write command. Configuring each          bit field separately may lead to configuration inconsistencies. Refer          to CROSSREFERENCE for the overview of the SMU core FSP  Error Pin  connectivity. The Error Pin Pad shall be configured to the          targeted function in the Port control logic before the SMU core takes          over the control."]
    #[inline(always)]
    pub fn pcs(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, pctl::Pcs, Pctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,pctl::Pcs, Pctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PAD Configuration   PCFG. This field contains the FSP PAD configuration signals driven by the SMU. The PAD configuration related to FSP 0  is provided by PCFG 8 0  The mapping to the FSP 0  PAD is defined as follows  PCFG 0    ENAQB  reset value   0  PCFG 1    ENQ  reset value   1  PCFG 2    0  no ENTTL for P33.8  PCFG 3    PD  reset value   0  PCFG 4    PPEN  reset value   0  PCFG 5    PUQ  reset value   1  PCFG 6    PDM0  reset value   0  PCFG 7    PDM1  reset value   0  PCFG 8    TTL3V3  reset value   0"]
    #[inline(always)]
    pub fn pcfg(
        self,
    ) -> crate::common::RegisterField<14, 0x1ff, 1, 0, u16, Pctl_SPEC, crate::common::R> {
        crate::common::RegisterField::<14,0x1ff,1,0,u16, Pctl_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pctl {
    #[inline(always)]
    fn default() -> Pctl {
        <crate::RegValueT<Pctl_SPEC> as RegisterValue<_>>::new(32768)
    }
}
pub mod pctl {
    pub struct Hwdir_SPEC;
    pub type Hwdir = crate::EnumBitfieldStruct<u8, Hwdir_SPEC>;
    impl Hwdir {
        #[doc = "00 sets FSP 0  and FSP 1  to input state"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 sets FSP 0  to output state and FSP 1  to input state"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 invalid input"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 sets FSP 0  and FSP 1  to output state"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Hwen_SPEC;
    pub type Hwen = crate::EnumBitfieldStruct<u8, Hwen_SPEC>;
    impl Hwen {
        #[doc = "00 FSP 1 0  port output is not driven by SMU core."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 FSP 0  port output is directly driven by SMU core and FSP 1  port output is not driven by SMU core."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 invalid input"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 FSP 1 0  port output is directly driven by SMU core"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct GfscuEn_SPEC;
    pub type GfscuEn = crate::EnumBitfieldStruct<u8, GfscuEn_SPEC>;
    impl GfscuEn {
        #[doc = "0 Glitch Filter        disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Glitch Filter        enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct GfstsEn_SPEC;
    pub type GfstsEn = crate::EnumBitfieldStruct<u8, GfstsEn_SPEC>;
    impl GfstsEn {
        #[doc = "0 Glitch Filter        disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Glitch FIlter        enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pcs_SPEC;
    pub type Pcs = crate::EnumBitfieldStruct<u8, Pcs_SPEC>;
    impl Pcs {
        #[doc = "0 The PAD configuration is controlled by the PORT registers."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The PAD configuration is controlled by the SMU."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmctl_SPEC;
impl crate::sealed::RegSpec for Rmctl_SPEC {
    type DataType = u32;
}
#[doc = "Register Monitor Control\n resetvalue={Application Reset:0x0}"]
pub type Rmctl = crate::RegValueT<Rmctl_SPEC>;

impl Rmctl {
    #[doc = "Test Enable.   TE31. This bit controls the test of the of the register monitor safety mechanism. Setting this bit starts the selft test of the safety flip flops in the corresponding module."]
    #[inline(always)]
    pub fn te0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, rmctl::Te0, Rmctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,rmctl::Te0, Rmctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Test Enable.   TE31. This bit controls the test of the of the register monitor safety mechanism. Setting this bit starts the selft test of the safety flip flops in the corresponding module."]
    #[inline(always)]
    pub fn te1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, rmctl::Te1, Rmctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,rmctl::Te1, Rmctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Test Enable.   TE31. This bit controls the test of the of the register monitor safety mechanism. Setting this bit starts the selft test of the safety flip flops in the corresponding module."]
    #[inline(always)]
    pub fn te2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, rmctl::Te2, Rmctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,rmctl::Te2, Rmctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Test Enable.   TE31. This bit controls the test of the of the register monitor safety mechanism. Setting this bit starts the selft test of the safety flip flops in the corresponding module."]
    #[inline(always)]
    pub fn te3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, rmctl::Te3, Rmctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,rmctl::Te3, Rmctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Test Enable.   TE31. This bit controls the test of the of the register monitor safety mechanism. Setting this bit starts the selft test of the safety flip flops in the corresponding module."]
    #[inline(always)]
    pub fn te4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, rmctl::Te4, Rmctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,rmctl::Te4, Rmctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Test Enable.   TE31. This bit controls the test of the of the register monitor safety mechanism. Setting this bit starts the selft test of the safety flip flops in the corresponding module."]
    #[inline(always)]
    pub fn te5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, rmctl::Te5, Rmctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,rmctl::Te5, Rmctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Test Enable.   TE31. This bit controls the test of the of the register monitor safety mechanism. Setting this bit starts the selft test of the safety flip flops in the corresponding module."]
    #[inline(always)]
    pub fn te6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, rmctl::Te6, Rmctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,rmctl::Te6, Rmctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Test Enable.   TE31. This bit controls the test of the of the register monitor safety mechanism. Setting this bit starts the selft test of the safety flip flops in the corresponding module."]
    #[inline(always)]
    pub fn te7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, rmctl::Te7, Rmctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,rmctl::Te7, Rmctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Test Enable.   TE31. This bit controls the test of the of the register monitor safety mechanism. Setting this bit starts the selft test of the safety flip flops in the corresponding module."]
    #[inline(always)]
    pub fn te8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, rmctl::Te8, Rmctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,rmctl::Te8, Rmctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Test Enable.   TE31. This bit controls the test of the of the register monitor safety mechanism. Setting this bit starts the selft test of the safety flip flops in the corresponding module."]
    #[inline(always)]
    pub fn te9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, rmctl::Te9, Rmctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,rmctl::Te9, Rmctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Test Enable.   TE31. This bit controls the test of the of the register monitor safety mechanism. Setting this bit starts the selft test of the safety flip flops in the corresponding module."]
    #[inline(always)]
    pub fn te10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, rmctl::Te10, Rmctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,rmctl::Te10, Rmctl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Rmctl {
    #[inline(always)]
    fn default() -> Rmctl {
        <crate::RegValueT<Rmctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rmctl {
    pub struct Te0_SPEC;
    pub type Te0 = crate::EnumBitfieldStruct<u8, Te0_SPEC>;
    impl Te0 {
        #[doc = "0 Test mode disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Test mode enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Te1_SPEC;
    pub type Te1 = crate::EnumBitfieldStruct<u8, Te1_SPEC>;
    impl Te1 {
        #[doc = "0 Test mode disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Test mode enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Te2_SPEC;
    pub type Te2 = crate::EnumBitfieldStruct<u8, Te2_SPEC>;
    impl Te2 {
        #[doc = "0 Test mode disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Test mode enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Te3_SPEC;
    pub type Te3 = crate::EnumBitfieldStruct<u8, Te3_SPEC>;
    impl Te3 {
        #[doc = "0 Test mode disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Test mode enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Te4_SPEC;
    pub type Te4 = crate::EnumBitfieldStruct<u8, Te4_SPEC>;
    impl Te4 {
        #[doc = "0 Test mode disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Test mode enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Te5_SPEC;
    pub type Te5 = crate::EnumBitfieldStruct<u8, Te5_SPEC>;
    impl Te5 {
        #[doc = "0 Test mode disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Test mode enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Te6_SPEC;
    pub type Te6 = crate::EnumBitfieldStruct<u8, Te6_SPEC>;
    impl Te6 {
        #[doc = "0 Test mode disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Test mode enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Te7_SPEC;
    pub type Te7 = crate::EnumBitfieldStruct<u8, Te7_SPEC>;
    impl Te7 {
        #[doc = "0 Test mode disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Test mode enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Te8_SPEC;
    pub type Te8 = crate::EnumBitfieldStruct<u8, Te8_SPEC>;
    impl Te8 {
        #[doc = "0 Test mode disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Test mode enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Te9_SPEC;
    pub type Te9 = crate::EnumBitfieldStruct<u8, Te9_SPEC>;
    impl Te9 {
        #[doc = "0 Test mode disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Test mode enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Te10_SPEC;
    pub type Te10 = crate::EnumBitfieldStruct<u8, Te10_SPEC>;
    impl Te10 {
        #[doc = "0 Test mode disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Test mode enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmef_SPEC;
impl crate::sealed::RegSpec for Rmef_SPEC {
    type DataType = u32;
}
#[doc = "Register Monitor Error Flags\n resetvalue={Application Reset:0x0}"]
pub type Rmef = crate::RegValueT<Rmef_SPEC>;

impl Rmef {
    #[doc = "Status flag related to the different instances of the register monitor safety mechanism.   EF31. It reports a real flip flop failure in non test mode as well as an        unexpected behavior in test mode. This flag can only be cleared by software  a set by software has no        effect"]
    #[inline(always)]
    pub fn ef0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, rmef::Ef0, Rmef_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,rmef::Ef0, Rmef_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag related to the different instances of the register monitor safety mechanism.   EF31. It reports a real flip flop failure in non test mode as well as an        unexpected behavior in test mode. This flag can only be cleared by software  a set by software has no        effect"]
    #[inline(always)]
    pub fn ef1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, rmef::Ef1, Rmef_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,rmef::Ef1, Rmef_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag related to the different instances of the register monitor safety mechanism.   EF31. It reports a real flip flop failure in non test mode as well as an        unexpected behavior in test mode. This flag can only be cleared by software  a set by software has no        effect"]
    #[inline(always)]
    pub fn ef2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, rmef::Ef2, Rmef_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,rmef::Ef2, Rmef_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag related to the different instances of the register monitor safety mechanism.   EF31. It reports a real flip flop failure in non test mode as well as an        unexpected behavior in test mode. This flag can only be cleared by software  a set by software has no        effect"]
    #[inline(always)]
    pub fn ef3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, rmef::Ef3, Rmef_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,rmef::Ef3, Rmef_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag related to the different instances of the register monitor safety mechanism.   EF31. It reports a real flip flop failure in non test mode as well as an        unexpected behavior in test mode. This flag can only be cleared by software  a set by software has no        effect"]
    #[inline(always)]
    pub fn ef4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, rmef::Ef4, Rmef_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,rmef::Ef4, Rmef_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag related to the different instances of the register monitor safety mechanism.   EF31. It reports a real flip flop failure in non test mode as well as an        unexpected behavior in test mode. This flag can only be cleared by software  a set by software has no        effect"]
    #[inline(always)]
    pub fn ef5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, rmef::Ef5, Rmef_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,rmef::Ef5, Rmef_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag related to the different instances of the register monitor safety mechanism.   EF31. It reports a real flip flop failure in non test mode as well as an        unexpected behavior in test mode. This flag can only be cleared by software  a set by software has no        effect"]
    #[inline(always)]
    pub fn ef6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, rmef::Ef6, Rmef_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,rmef::Ef6, Rmef_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag related to the different instances of the register monitor safety mechanism.   EF31. It reports a real flip flop failure in non test mode as well as an        unexpected behavior in test mode. This flag can only be cleared by software  a set by software has no        effect"]
    #[inline(always)]
    pub fn ef7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, rmef::Ef7, Rmef_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,rmef::Ef7, Rmef_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag related to the different instances of the register monitor safety mechanism.   EF31. It reports a real flip flop failure in non test mode as well as an        unexpected behavior in test mode. This flag can only be cleared by software  a set by software has no        effect"]
    #[inline(always)]
    pub fn ef8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, rmef::Ef8, Rmef_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,rmef::Ef8, Rmef_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag related to the different instances of the register monitor safety mechanism.   EF31. It reports a real flip flop failure in non test mode as well as an        unexpected behavior in test mode. This flag can only be cleared by software  a set by software has no        effect"]
    #[inline(always)]
    pub fn ef9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, rmef::Ef9, Rmef_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,rmef::Ef9, Rmef_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status flag related to the different instances of the register monitor safety mechanism.   EF31. It reports a real flip flop failure in non test mode as well as an        unexpected behavior in test mode. This flag can only be cleared by software  a set by software has no        effect"]
    #[inline(always)]
    pub fn ef10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, rmef::Ef10, Rmef_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x1,1,0,rmef::Ef10, Rmef_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Rmef {
    #[inline(always)]
    fn default() -> Rmef {
        <crate::RegValueT<Rmef_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rmef {
    pub struct Ef0_SPEC;
    pub type Ef0 = crate::EnumBitfieldStruct<u8, Ef0_SPEC>;
    impl Ef0 {
        #[doc = "0 Error flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ef1_SPEC;
    pub type Ef1 = crate::EnumBitfieldStruct<u8, Ef1_SPEC>;
    impl Ef1 {
        #[doc = "0 Error flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ef2_SPEC;
    pub type Ef2 = crate::EnumBitfieldStruct<u8, Ef2_SPEC>;
    impl Ef2 {
        #[doc = "0 Error flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ef3_SPEC;
    pub type Ef3 = crate::EnumBitfieldStruct<u8, Ef3_SPEC>;
    impl Ef3 {
        #[doc = "0 Error flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ef4_SPEC;
    pub type Ef4 = crate::EnumBitfieldStruct<u8, Ef4_SPEC>;
    impl Ef4 {
        #[doc = "0 Error flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ef5_SPEC;
    pub type Ef5 = crate::EnumBitfieldStruct<u8, Ef5_SPEC>;
    impl Ef5 {
        #[doc = "0 Error flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ef6_SPEC;
    pub type Ef6 = crate::EnumBitfieldStruct<u8, Ef6_SPEC>;
    impl Ef6 {
        #[doc = "0 Error flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ef7_SPEC;
    pub type Ef7 = crate::EnumBitfieldStruct<u8, Ef7_SPEC>;
    impl Ef7 {
        #[doc = "0 Error flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ef8_SPEC;
    pub type Ef8 = crate::EnumBitfieldStruct<u8, Ef8_SPEC>;
    impl Ef8 {
        #[doc = "0 Error flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ef9_SPEC;
    pub type Ef9 = crate::EnumBitfieldStruct<u8, Ef9_SPEC>;
    impl Ef9 {
        #[doc = "0 Error flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ef10_SPEC;
    pub type Ef10 = crate::EnumBitfieldStruct<u8, Ef10_SPEC>;
    impl Ef10 {
        #[doc = "0 Error flag z        does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error flag z        reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rmsts_SPEC;
impl crate::sealed::RegSpec for Rmsts_SPEC {
    type DataType = u32;
}
#[doc = "Register Monitor Self Test Status\n resetvalue={Application Reset:0x0}"]
pub type Rmsts = crate::RegValueT<Rmsts_SPEC>;

impl Rmsts {
    #[doc = "Ready flag related to the different instances of the register monitor safety mechanism.   STS31. A logical   8216 1  8217  of this bit indicates that the register monitor test has        been executed. This bit can only be cleared by software  a set by        software has no effect."]
    #[inline(always)]
    pub fn sts0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, rmsts::Sts0, Rmsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,rmsts::Sts0, Rmsts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Ready flag related to the different instances of the register monitor safety mechanism.   STS31. A logical   8216 1  8217  of this bit indicates that the register monitor test has        been executed. This bit can only be cleared by software  a set by        software has no effect."]
    #[inline(always)]
    pub fn sts1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, rmsts::Sts1, Rmsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,rmsts::Sts1, Rmsts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Ready flag related to the different instances of the register monitor safety mechanism.   STS31. A logical   8216 1  8217  of this bit indicates that the register monitor test has        been executed. This bit can only be cleared by software  a set by        software has no effect."]
    #[inline(always)]
    pub fn sts2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, rmsts::Sts2, Rmsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,rmsts::Sts2, Rmsts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Ready flag related to the different instances of the register monitor safety mechanism.   STS31. A logical   8216 1  8217  of this bit indicates that the register monitor test has        been executed. This bit can only be cleared by software  a set by        software has no effect."]
    #[inline(always)]
    pub fn sts3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, rmsts::Sts3, Rmsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,rmsts::Sts3, Rmsts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Ready flag related to the different instances of the register monitor safety mechanism.   STS31. A logical   8216 1  8217  of this bit indicates that the register monitor test has        been executed. This bit can only be cleared by software  a set by        software has no effect."]
    #[inline(always)]
    pub fn sts4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, rmsts::Sts4, Rmsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,rmsts::Sts4, Rmsts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Ready flag related to the different instances of the register monitor safety mechanism.   STS31. A logical   8216 1  8217  of this bit indicates that the register monitor test has        been executed. This bit can only be cleared by software  a set by        software has no effect."]
    #[inline(always)]
    pub fn sts5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, rmsts::Sts5, Rmsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,rmsts::Sts5, Rmsts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Ready flag related to the different instances of the register monitor safety mechanism.   STS31. A logical   8216 1  8217  of this bit indicates that the register monitor test has        been executed. This bit can only be cleared by software  a set by        software has no effect."]
    #[inline(always)]
    pub fn sts6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, rmsts::Sts6, Rmsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,rmsts::Sts6, Rmsts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Ready flag related to the different instances of the register monitor safety mechanism.   STS31. A logical   8216 1  8217  of this bit indicates that the register monitor test has        been executed. This bit can only be cleared by software  a set by        software has no effect."]
    #[inline(always)]
    pub fn sts7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, rmsts::Sts7, Rmsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,rmsts::Sts7, Rmsts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Ready flag related to the different instances of the register monitor safety mechanism.   STS31. A logical   8216 1  8217  of this bit indicates that the register monitor test has        been executed. This bit can only be cleared by software  a set by        software has no effect."]
    #[inline(always)]
    pub fn sts8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, rmsts::Sts8, Rmsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,rmsts::Sts8, Rmsts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Ready flag related to the different instances of the register monitor safety mechanism.   STS31. A logical   8216 1  8217  of this bit indicates that the register monitor test has        been executed. This bit can only be cleared by software  a set by        software has no effect."]
    #[inline(always)]
    pub fn sts9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, rmsts::Sts9, Rmsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,rmsts::Sts9, Rmsts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Ready flag related to the different instances of the register monitor safety mechanism.   STS31. A logical   8216 1  8217  of this bit indicates that the register monitor test has        been executed. This bit can only be cleared by software  a set by        software has no effect."]
    #[inline(always)]
    pub fn sts10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, rmsts::Sts10, Rmsts_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,rmsts::Sts10, Rmsts_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Rmsts {
    #[inline(always)]
    fn default() -> Rmsts {
        <crate::RegValueT<Rmsts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rmsts {
    pub struct Sts0_SPEC;
    pub type Sts0 = crate::EnumBitfieldStruct<u8, Sts0_SPEC>;
    impl Sts0 {
        #[doc = "0 Self test has        not completed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Self test has        completed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sts1_SPEC;
    pub type Sts1 = crate::EnumBitfieldStruct<u8, Sts1_SPEC>;
    impl Sts1 {
        #[doc = "0 Self test has        not completed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Self test has        completed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sts2_SPEC;
    pub type Sts2 = crate::EnumBitfieldStruct<u8, Sts2_SPEC>;
    impl Sts2 {
        #[doc = "0 Self test has        not completed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Self test has        completed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sts3_SPEC;
    pub type Sts3 = crate::EnumBitfieldStruct<u8, Sts3_SPEC>;
    impl Sts3 {
        #[doc = "0 Self test has        not completed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Self test has        completed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sts4_SPEC;
    pub type Sts4 = crate::EnumBitfieldStruct<u8, Sts4_SPEC>;
    impl Sts4 {
        #[doc = "0 Self test has        not completed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Self test has        completed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sts5_SPEC;
    pub type Sts5 = crate::EnumBitfieldStruct<u8, Sts5_SPEC>;
    impl Sts5 {
        #[doc = "0 Self test has        not completed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Self test has        completed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sts6_SPEC;
    pub type Sts6 = crate::EnumBitfieldStruct<u8, Sts6_SPEC>;
    impl Sts6 {
        #[doc = "0 Self test has        not completed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Self test has        completed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sts7_SPEC;
    pub type Sts7 = crate::EnumBitfieldStruct<u8, Sts7_SPEC>;
    impl Sts7 {
        #[doc = "0 Self test has        not completed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Self test has        completed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sts8_SPEC;
    pub type Sts8 = crate::EnumBitfieldStruct<u8, Sts8_SPEC>;
    impl Sts8 {
        #[doc = "0 Self test has        not completed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Self test has        completed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sts9_SPEC;
    pub type Sts9 = crate::EnumBitfieldStruct<u8, Sts9_SPEC>;
    impl Sts9 {
        #[doc = "0 Self test has        not completed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Self test has        completed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sts10_SPEC;
    pub type Sts10 = crate::EnumBitfieldStruct<u8, Sts10_SPEC>;
    impl Sts10 {
        #[doc = "0 Self test has        not completed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Self test has        completed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtac00_SPEC;
impl crate::sealed::RegSpec for Rtac00_SPEC {
    type DataType = u32;
}
#[doc = "Recovery Timer 0 Alarm Configuration 0\n resetvalue={Application Reset:0x0A80108}"]
pub type Rtac00 = crate::RegValueT<Rtac00_SPEC>;

impl Rtac00 {
    #[doc = "Group Index 0.   GID0. This field enables to specify if an alarm from this alarm group can use        the recovery timer 0. The functionality of this field is described in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn gid0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Rtac00_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Rtac00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alarm Identifier 0.   ALID0. This field specifies the alarm index related to the group index        specified in GID0."]
    #[inline(always)]
    pub fn alid0(
        self,
    ) -> crate::common::RegisterField<4, 0x1f, 1, 0, u8, Rtac00_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1f,1,0,u8, Rtac00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Group Index 1.   GID1. This field enables to specify if an alarm from this alarm group can use        the recovery timer 0. The functionality of this field is described in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn gid1(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Rtac00_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Rtac00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alarm Identifier 1.   ALID1. This field specifies the alarm index related to the group index        specified in GID1."]
    #[inline(always)]
    pub fn alid1(
        self,
    ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, Rtac00_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x1f,1,0,u8, Rtac00_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Rtac00 {
    #[inline(always)]
    fn default() -> Rtac00 {
        <crate::RegValueT<Rtac00_SPEC> as RegisterValue<_>>::new(11010312)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtac01_SPEC;
impl crate::sealed::RegSpec for Rtac01_SPEC {
    type DataType = u32;
}
#[doc = "Recovery Timer 0 Alarm Configuration 1\n resetvalue={Application Reset:0x0C800B8}"]
pub type Rtac01 = crate::RegValueT<Rtac01_SPEC>;

impl Rtac01 {
    #[doc = "Group Index 2.   GID2. This field enables to specify if an alarm from this alarm group can use        the recovery timer 0. The functionality of this field is described in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn gid2(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Rtac01_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Rtac01_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alarm Identifier 0.   ALID2. This field specifies the alarm index related to the group index        specified in GID2."]
    #[inline(always)]
    pub fn alid2(
        self,
    ) -> crate::common::RegisterField<4, 0x1f, 1, 0, u8, Rtac01_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1f,1,0,u8, Rtac01_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Group Index 3.   GID3. This field enables to specify if an alarm from this alarm group can use        the recovery timer 3. The functionality of this field is described in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn gid3(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Rtac01_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Rtac01_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alarm Identifier 1.   ALID3. This field specifies the alarm index related to the group index        specified in GID3."]
    #[inline(always)]
    pub fn alid3(
        self,
    ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, Rtac01_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x1f,1,0,u8, Rtac01_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Rtac01 {
    #[inline(always)]
    fn default() -> Rtac01 {
        <crate::RegValueT<Rtac01_SPEC> as RegisterValue<_>>::new(13107384)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtac10_SPEC;
impl crate::sealed::RegSpec for Rtac10_SPEC {
    type DataType = u32;
}
#[doc = "Recovery Timer 1 Alarm Configuration 0\n resetvalue={Application Reset:0x0E800D8}"]
pub type Rtac10 = crate::RegValueT<Rtac10_SPEC>;

impl Rtac10 {
    #[doc = "Group Index 0.   GID0. This field enables to specify if an alarm from this alarm group can use the recovery timer 1. The functionality of this field is described in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn gid0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Rtac10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Rtac10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alarm Identifier 0.   ALID0. This field specifies the alarm index related to the group index specified in GID0."]
    #[inline(always)]
    pub fn alid0(
        self,
    ) -> crate::common::RegisterField<4, 0x1f, 1, 0, u8, Rtac10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1f,1,0,u8, Rtac10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Group Index 1.   GID1. This field enables to specify if an alarm from this alarm group can use the recovery timer 1. The functionality of this field is described in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn gid1(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Rtac10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Rtac10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alarm Identifier 1.   ALID1. This field specifies the alarm index related to the group index specified in GID1."]
    #[inline(always)]
    pub fn alid1(
        self,
    ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, Rtac10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x1f,1,0,u8, Rtac10_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Rtac10 {
    #[inline(always)]
    fn default() -> Rtac10 {
        <crate::RegValueT<Rtac10_SPEC> as RegisterValue<_>>::new(15204568)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtac11_SPEC;
impl crate::sealed::RegSpec for Rtac11_SPEC {
    type DataType = u32;
}
#[doc = "Recovery Timer 1 Alarm Configuration 1\n resetvalue={Application Reset:0x0F800F8}"]
pub type Rtac11 = crate::RegValueT<Rtac11_SPEC>;

impl Rtac11 {
    #[doc = "Group Index 2.   GID2. This field enables to specify if an alarm from this alarm group can use the recovery timer 1. The functionality of this field is described in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn gid2(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Rtac11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Rtac11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alarm Identifier 2.   ALID2. This field specifies the alarm index related to the group index specified in GID2."]
    #[inline(always)]
    pub fn alid2(
        self,
    ) -> crate::common::RegisterField<4, 0x1f, 1, 0, u8, Rtac11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1f,1,0,u8, Rtac11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Group Index 3.   GID3. This field enables to specify if an alarm from this alarm group can use the recovery timer 1. The functionality of this field is described in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn gid3(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Rtac11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Rtac11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alarm Identifier 3.   ALID3. This field specifies the alarm index related to the group index specified in GID3."]
    #[inline(always)]
    pub fn alid3(
        self,
    ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, Rtac11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x1f,1,0,u8, Rtac11_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Rtac11 {
    #[inline(always)]
    fn default() -> Rtac11 {
        <crate::RegValueT<Rtac11_SPEC> as RegisterValue<_>>::new(16253176)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtc_SPEC;
impl crate::sealed::RegSpec for Rtc_SPEC {
    type DataType = u32;
}
#[doc = "Recovery Timer Configuration\n resetvalue={Application Reset:0x3FFF03}"]
pub type Rtc = crate::RegValueT<Rtc_SPEC>;

impl Rtc {
    #[doc = "RT0 Enable Bit   RT0E"]
    #[inline(always)]
    pub fn rt0e(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, rtc::Rt0E, Rtc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,rtc::Rt0E, Rtc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RT1 Enable Bit   RT1E"]
    #[inline(always)]
    pub fn rt1e(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, rtc::Rt1E, Rtc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,rtc::Rt1E, Rtc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Recovery Timer Duration   RTD. This field specifies the maximum duration of the recovery timer. When        the timer counter reaches the programmed value  the internal alarm        rt timeout is issued. The timer is stopped by a SMU RTStop   command        before the recovery timer. RTD shall be specified as a number of the FSMU FS clock ticks."]
    #[inline(always)]
    pub fn rtd(
        self,
    ) -> crate::common::RegisterField<8, 0xffffff, 1, 0, u32, Rtc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xffffff,1,0,u32, Rtc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Rtc {
    #[inline(always)]
    fn default() -> Rtc {
        <crate::RegValueT<Rtc_SPEC> as RegisterValue<_>>::new(4194051)
    }
}
pub mod rtc {
    pub struct Rt0E_SPEC;
    pub type Rt0E = crate::EnumBitfieldStruct<u8, Rt0E_SPEC>;
    impl Rt0E {
        #[doc = "0 Recovery Timer 0 is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Recovery Timer 0 is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rt1E_SPEC;
    pub type Rt1E = crate::EnumBitfieldStruct<u8, Rt1E_SPEC>;
    impl Rt1E {
        #[doc = "0 Recovery Timer 1 is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Recovery Timer 1 is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sts_SPEC;
impl crate::sealed::RegSpec for Sts_SPEC {
    type DataType = u32;
}
#[doc = "Status Register\n resetvalue={Application Reset:0x0}"]
pub type Sts = crate::RegValueT<Sts_SPEC>;

impl Sts {
    #[doc = "Last command received   CMD. Same encoding as CMD field of SMU CMD register"]
    #[inline(always)]
    pub fn cmd(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Sts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0xf, 1, 0, u8, Sts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Last command argument received   ARG. Same encoding as ARG field of SMU CMD register"]
    #[inline(always)]
    pub fn arg(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Sts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, Sts_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Result of last received command   RES"]
    #[inline(always)]
    pub fn res(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, sts::Res, Sts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,sts::Res, Sts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alarm Status Clear Enable   ASCE. This bit controls if a status flag set in an AG lt x gt  register upon        detection of an alarm event can be cleared by software or not. When ASCE is enabled software shall write a 1 to the bit position in        AG lt x gt  to clear the bit  W1C . When a W1C action takes place the ASCE bit        is automatically cleared to 0 by hardware and software shall set the        ASCE bit again by using the SMU ASCE   command."]
    #[inline(always)]
    pub fn asce(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, sts::Asce, Sts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,sts::Asce, Sts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault Signaling Protocol status   FSP. FSP 0    FSP STS 0  input signal FSP 1    FSP STS 1  input signal This field is updated by hardware every clock cycle  therefore a        software clear on write is not meaningful for this field."]
    #[inline(always)]
    pub fn fsp(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Sts_SPEC, crate::common::R> {
        crate::common::RegisterField::<10, 0x3, 1, 0, u8, Sts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Fault State Timing Status   FSTS. This bit indicates if the minimum timing duration of the FSP fault state        has been reached or not. The bit is cleared by hardware when the fault        state is entered."]
    #[inline(always)]
    pub fn fsts(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, sts::Fsts, Sts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x1,1,0,sts::Fsts, Sts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Recovery Timer 0 Status   RTS0. See CROSSREFERENCE for the usage of this field."]
    #[inline(always)]
    pub fn rts0(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, sts::Rts0, Sts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1,1,0,sts::Rts0, Sts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Recovery Timer 0 Missed Event   RTME0. See CROSSREFERENCE for the usage of this field."]
    #[inline(always)]
    pub fn rtme0(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, sts::Rtme0, Sts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<17,0x1,1,0,sts::Rtme0, Sts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Recovery Timer 1 Status   RTS1. See CROSSREFERENCE for the usage of this field."]
    #[inline(always)]
    pub fn rts1(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, sts::Rts1, Sts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x1,1,0,sts::Rts1, Sts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Recovery Timer 1 Missed Event   RTME1. See CROSSREFERENCE for the usage of this field."]
    #[inline(always)]
    pub fn rtme1(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, sts::Rtme1, Sts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<19,0x1,1,0,sts::Rtme1, Sts_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Sts {
    #[inline(always)]
    fn default() -> Sts {
        <crate::RegValueT<Sts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sts {
    pub struct Res_SPEC;
    pub type Res = crate::EnumBitfieldStruct<u8, Res_SPEC>;
    impl Res {
        #[doc = "0 Command was successful"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Command failed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Asce_SPEC;
    pub type Asce = crate::EnumBitfieldStruct<u8, Asce_SPEC>;
    impl Asce {
        #[doc = "0 Alarm status bits AG lt x gt  can not be cleared"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Alarm status bits AG lt x gt  can be cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fsts_SPEC;
    pub type Fsts = crate::EnumBitfieldStruct<u8, Fsts_SPEC>;
    impl Fsts {
        #[doc = "0 Minimum timing duration not reached"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Minimum timing duration reached"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rts0_SPEC;
    pub type Rts0 = crate::EnumBitfieldStruct<u8, Rts0_SPEC>;
    impl Rts0 {
        #[doc = "0 Recovery Timer not running"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Recovery Timer running"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rtme0_SPEC;
    pub type Rtme0 = crate::EnumBitfieldStruct<u8, Rtme0_SPEC>;
    impl Rtme0 {
        #[doc = "0 Recovery Timer event not detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Recovery Timer event detected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rts1_SPEC;
    pub type Rts1 = crate::EnumBitfieldStruct<u8, Rts1_SPEC>;
    impl Rts1 {
        #[doc = "0 Recovery Timer not running"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Recovery Timer running"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rtme1_SPEC;
    pub type Rtme1 = crate::EnumBitfieldStruct<u8, Rtme1_SPEC>;
    impl Rtme1 {
        #[doc = "0 Recovery Timer event not detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Recovery Timer event detected"]
        pub const CONST_11: Self = Self::new(1);
    }
}

#[doc = "AGRP"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AGiCFj(pub(super) *mut u8);
unsafe impl core::marker::Send for AGiCFj {}
unsafe impl core::marker::Sync for AGiCFj {}
impl AGiCFj {
    #[doc = "Alarm Configuration Register\n resetvalue={Application Reset:0x0,Application Reset:0x1FC00,Application Reset:0x30000}"]
    #[inline(always)]
    pub const fn agicfj_(&self) -> [crate::common::Reg<agicfj::AGiCFj_SPEC, crate::common::RW>; 3] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x8usize)),
            ]
        }
    }
}
pub mod agicfj {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AGiCFj_SPEC;
    impl crate::sealed::RegSpec for AGiCFj_SPEC {
        type DataType = u32;
    }
    #[doc = "Alarm Configuration Register\n resetvalue={Application Reset:0x0,Application Reset:0x1FC00,Application Reset:0x30000}"]
    pub type AGiCFj = crate::RegValueT<AGiCFj_SPEC>;

    impl AGiCFj {
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group  see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn cf0(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, agicfj_::Cf0, AGiCFj_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1,1,0,agicfj_::Cf0, AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group  see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn cf1(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, agicfj_::Cf1, AGiCFj_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0x1,1,0,agicfj_::Cf1, AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group  see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn cf2(
            self,
        ) -> crate::common::RegisterField<2, 0x1, 1, 0, agicfj_::Cf2, AGiCFj_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x1,1,0,agicfj_::Cf2, AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group  see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn cf4(
            self,
        ) -> crate::common::RegisterField<4, 0x1, 1, 0, agicfj_::Cf4, AGiCFj_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x1,1,0,agicfj_::Cf4, AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group  see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn cf5(
            self,
        ) -> crate::common::RegisterField<5, 0x1, 1, 0, agicfj_::Cf5, AGiCFj_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x1,1,0,agicfj_::Cf5, AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group  see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn cf6(
            self,
        ) -> crate::common::RegisterField<6, 0x1, 1, 0, agicfj_::Cf6, AGiCFj_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<6,0x1,1,0,agicfj_::Cf6, AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group  see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn cf7(
            self,
        ) -> crate::common::RegisterField<7, 0x1, 1, 0, agicfj_::Cf7, AGiCFj_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<7,0x1,1,0,agicfj_::Cf7, AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group  see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn cf8(
            self,
        ) -> crate::common::RegisterField<8, 0x1, 1, 0, agicfj_::Cf8, AGiCFj_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x1,1,0,agicfj_::Cf8, AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group  see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn cf9(
            self,
        ) -> crate::common::RegisterField<9, 0x1, 1, 0, agicfj_::Cf9, AGiCFj_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<9,0x1,1,0,agicfj_::Cf9, AGiCFj_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group  see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn cf10(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            agicfj_::Cf10,
            AGiCFj_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                agicfj_::Cf10,
                AGiCFj_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group  see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn cf11(
            self,
        ) -> crate::common::RegisterField<
            11,
            0x1,
            1,
            0,
            agicfj_::Cf11,
            AGiCFj_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                agicfj_::Cf11,
                AGiCFj_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group  see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn cf12(
            self,
        ) -> crate::common::RegisterField<
            12,
            0x1,
            1,
            0,
            agicfj_::Cf12,
            AGiCFj_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                12,
                0x1,
                1,
                0,
                agicfj_::Cf12,
                AGiCFj_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group  see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn cf13(
            self,
        ) -> crate::common::RegisterField<
            13,
            0x1,
            1,
            0,
            agicfj_::Cf13,
            AGiCFj_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                13,
                0x1,
                1,
                0,
                agicfj_::Cf13,
                AGiCFj_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group  see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn cf14(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x1,
            1,
            0,
            agicfj_::Cf14,
            AGiCFj_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                agicfj_::Cf14,
                AGiCFj_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group  see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn cf22(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x1,
            1,
            0,
            agicfj_::Cf22,
            AGiCFj_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                22,
                0x1,
                1,
                0,
                agicfj_::Cf22,
                AGiCFj_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group  see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn cf23(
            self,
        ) -> crate::common::RegisterField<
            23,
            0x1,
            1,
            0,
            agicfj_::Cf23,
            AGiCFj_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                23,
                0x1,
                1,
                0,
                agicfj_::Cf23,
                AGiCFj_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Configuration flag x  x 0 2  for alarm 31 belonging to alarm group i.   CF31. The configuration flags 0  1 and 2 must be used together to define the        behavior of the SMU core when a fault state is reported by the alarm n        belonging to this group  see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn cf24(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x1,
            1,
            0,
            agicfj_::Cf24,
            AGiCFj_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0x1,
                1,
                0,
                agicfj_::Cf24,
                AGiCFj_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for AGiCFj {
        #[inline(always)]
        fn default() -> AGiCFj {
            <crate::RegValueT<AGiCFj_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod agicfj_ {
        pub struct Cf0_SPEC;
        pub type Cf0 = crate::EnumBitfieldStruct<u8, Cf0_SPEC>;
        impl Cf0 {
            #[doc = "0 Configuration        flag x  x 0 2  is set to 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Configuration        flag x  x 0 2  is set to 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cf1_SPEC;
        pub type Cf1 = crate::EnumBitfieldStruct<u8, Cf1_SPEC>;
        impl Cf1 {
            #[doc = "0 Configuration        flag x  x 0 2  is set to 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Configuration        flag x  x 0 2  is set to 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cf2_SPEC;
        pub type Cf2 = crate::EnumBitfieldStruct<u8, Cf2_SPEC>;
        impl Cf2 {
            #[doc = "0 Configuration        flag x  x 0 2  is set to 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Configuration        flag x  x 0 2  is set to 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cf4_SPEC;
        pub type Cf4 = crate::EnumBitfieldStruct<u8, Cf4_SPEC>;
        impl Cf4 {
            #[doc = "0 Configuration        flag x  x 0 2  is set to 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Configuration        flag x  x 0 2  is set to 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cf5_SPEC;
        pub type Cf5 = crate::EnumBitfieldStruct<u8, Cf5_SPEC>;
        impl Cf5 {
            #[doc = "0 Configuration        flag x  x 0 2  is set to 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Configuration        flag x  x 0 2  is set to 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cf6_SPEC;
        pub type Cf6 = crate::EnumBitfieldStruct<u8, Cf6_SPEC>;
        impl Cf6 {
            #[doc = "0 Configuration        flag x  x 0 2  is set to 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Configuration        flag x  x 0 2  is set to 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cf7_SPEC;
        pub type Cf7 = crate::EnumBitfieldStruct<u8, Cf7_SPEC>;
        impl Cf7 {
            #[doc = "0 Configuration        flag x  x 0 2  is set to 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Configuration        flag x  x 0 2  is set to 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cf8_SPEC;
        pub type Cf8 = crate::EnumBitfieldStruct<u8, Cf8_SPEC>;
        impl Cf8 {
            #[doc = "0 Configuration        flag x  x 0 2  is set to 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Configuration        flag x  x 0 2  is set to 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cf9_SPEC;
        pub type Cf9 = crate::EnumBitfieldStruct<u8, Cf9_SPEC>;
        impl Cf9 {
            #[doc = "0 Configuration        flag x  x 0 2  is set to 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Configuration        flag x  x 0 2  is set to 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cf10_SPEC;
        pub type Cf10 = crate::EnumBitfieldStruct<u8, Cf10_SPEC>;
        impl Cf10 {
            #[doc = "0 Configuration        flag x  x 0 2  is set to 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Configuration        flag x  x 0 2  is set to 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cf11_SPEC;
        pub type Cf11 = crate::EnumBitfieldStruct<u8, Cf11_SPEC>;
        impl Cf11 {
            #[doc = "0 Configuration        flag x  x 0 2  is set to 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Configuration        flag x  x 0 2  is set to 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cf12_SPEC;
        pub type Cf12 = crate::EnumBitfieldStruct<u8, Cf12_SPEC>;
        impl Cf12 {
            #[doc = "0 Configuration        flag x  x 0 2  is set to 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Configuration        flag x  x 0 2  is set to 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cf13_SPEC;
        pub type Cf13 = crate::EnumBitfieldStruct<u8, Cf13_SPEC>;
        impl Cf13 {
            #[doc = "0 Configuration        flag x  x 0 2  is set to 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Configuration        flag x  x 0 2  is set to 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cf14_SPEC;
        pub type Cf14 = crate::EnumBitfieldStruct<u8, Cf14_SPEC>;
        impl Cf14 {
            #[doc = "0 Configuration        flag x  x 0 2  is set to 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Configuration        flag x  x 0 2  is set to 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cf22_SPEC;
        pub type Cf22 = crate::EnumBitfieldStruct<u8, Cf22_SPEC>;
        impl Cf22 {
            #[doc = "0 Configuration        flag x  x 0 2  is set to 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Configuration        flag x  x 0 2  is set to 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cf23_SPEC;
        pub type Cf23 = crate::EnumBitfieldStruct<u8, Cf23_SPEC>;
        impl Cf23 {
            #[doc = "0 Configuration        flag x  x 0 2  is set to 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Configuration        flag x  x 0 2  is set to 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cf24_SPEC;
        pub type Cf24 = crate::EnumBitfieldStruct<u8, Cf24_SPEC>;
        impl Cf24 {
            #[doc = "0 Configuration        flag x  x 0 2  is set to 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Configuration        flag x  x 0 2  is set to 1"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
}
