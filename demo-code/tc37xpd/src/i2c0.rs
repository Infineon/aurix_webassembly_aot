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
#[doc = r"I2C"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C0(pub(super) *mut u8);
unsafe impl core::marker::Send for I2C0 {}
unsafe impl core::marker::Sync for I2C0 {}
impl I2C0 {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(65548usize)) }
    }

    #[doc = "Address Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn addrcfg(&self) -> crate::common::Reg<self::Addrcfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "Bus Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn busstat(&self) -> crate::common::Reg<self::Busstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(65536usize)) }
    }

    #[doc = "Clock Control 1 Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc1(&self) -> crate::common::Reg<self::Clc1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "DMA Enable Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dmae(&self) -> crate::common::Reg<self::Dmae_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(148usize)) }
    }

    #[doc = "End Data Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn enddctrl(&self) -> crate::common::Reg<self::Enddctrl_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Error Interrupt Request Source Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn errirqsc(&self) -> crate::common::Reg<self::Errirqsc_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(104usize)) }
    }

    #[doc = "Error Interrupt Request Source Mask Register\n resetvalue={Application Reset:0x0F}"]
    #[inline(always)]
    pub const fn errirqsm(&self) -> crate::common::Reg<self::Errirqsm_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }

    #[doc = "Error Interrupt Request Source Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn errirqss(&self) -> crate::common::Reg<self::Errirqss_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(100usize)) }
    }

    #[doc = "Fractional Divider Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fdivcfg(&self) -> crate::common::Reg<self::Fdivcfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }

    #[doc = "Fractional Divider High speed Mode Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fdivhighcfg(
        &self,
    ) -> crate::common::Reg<self::Fdivhighcfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "Filled FIFO Stages Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ffsstat(&self) -> crate::common::Reg<self::Ffsstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }

    #[doc = "FIFO Configuration Register\n resetvalue={Application Reset:0x22}"]
    #[inline(always)]
    pub const fn fifocfg(&self) -> crate::common::Reg<self::Fifocfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }

    #[doc = "General Purpose Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gpctl(&self) -> crate::common::Reg<self::Gpctl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(65544usize)) }
    }

    #[doc = "General Purpose Control Register\n resetvalue={Application Reset:0x0F0}"]
    #[inline(always)]
    pub const fn gpctrl(&self) -> crate::common::Reg<self::Gpctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }

    #[doc = "Interrupt Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn icr(&self) -> crate::common::Reg<self::Icr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(140usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x5703}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Interrupt Mask Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn imsc(&self) -> crate::common::Reg<self::Imsc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }

    #[doc = "Interrupt Set Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn isr(&self) -> crate::common::Reg<self::Isr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }

    #[doc = "Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst0(&self) -> crate::common::Reg<self::Krst0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(65556usize)) }
    }

    #[doc = "Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst1(&self) -> crate::common::Reg<self::Krst1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(65560usize)) }
    }

    #[doc = "Kernel Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krstclr(&self) -> crate::common::Reg<self::Krstclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(65564usize)) }
    }

    #[doc = "Masked Interrupt Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mis(&self) -> crate::common::Reg<self::Mis_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0C2C000}"]
    #[inline(always)]
    pub const fn modid(&self) -> crate::common::Reg<self::Modid_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(65540usize)) }
    }

    #[doc = "Maximum Received Packet Size Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mrpsctrl(&self) -> crate::common::Reg<self::Mrpsctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }

    #[doc = "Protocol Interrupt Request Source Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pirqsc(&self) -> crate::common::Reg<self::Pirqsc_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(120usize)) }
    }

    #[doc = "Protocol Interrupt Request Source Mask Register\n resetvalue={Application Reset:0x7F}"]
    #[inline(always)]
    pub const fn pirqsm(&self) -> crate::common::Reg<self::Pirqsm_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(112usize)) }
    }

    #[doc = "Protocol Interrupt Request Source Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pirqss(&self) -> crate::common::Reg<self::Pirqss_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(116usize)) }
    }

    #[doc = "Raw Interrupt Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ris(&self) -> crate::common::Reg<self::Ris_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "Received Packet Size Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rpsstat(&self) -> crate::common::Reg<self::Rpsstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }

    #[doc = "RUN Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn runctrl(&self) -> crate::common::Reg<self::Runctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "Reception Data Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rxd(&self) -> crate::common::Reg<self::Rxd_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(49152usize)) }
    }

    #[doc = "Timing Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn timcfg(&self) -> crate::common::Reg<self::Timcfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "Transmit Packet Size Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tpsctrl(&self) -> crate::common::Reg<self::Tpsctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "Transmission Data Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn txd(&self) -> crate::common::Reg<self::Txd_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32768usize)) }
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
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, accen0::En0, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,accen0::En0, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, accen0::En1, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,accen0::En1, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, accen0::En2, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,accen0::En2, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, accen0::En3, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,accen0::En3, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, accen0::En4, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,accen0::En4, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, accen0::En5, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,accen0::En5, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, accen0::En6, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,accen0::En6, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, accen0::En7, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,accen0::En7, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, accen0::En8, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,accen0::En8, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, accen0::En9, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,accen0::En9, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, accen0::En10, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,accen0::En10, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, accen0::En11, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,accen0::En11, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, accen0::En12, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,accen0::En12, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, accen0::En13, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,accen0::En13, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, accen0::En14, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,accen0::En14, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, accen0::En15, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,accen0::En15, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, accen0::En16, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,accen0::En16, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, accen0::En17, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,accen0::En17, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, accen0::En18, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,accen0::En18, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, accen0::En19, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,accen0::En19, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, accen0::En20, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,accen0::En20, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, accen0::En21, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,accen0::En21, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, accen0::En22, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,accen0::En22, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, accen0::En23, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,accen0::En23, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, accen0::En24, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,accen0::En24, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, accen0::En25, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,accen0::En25, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, accen0::En26, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,accen0::En26, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, accen0::En27, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,accen0::En27, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, accen0::En28, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,accen0::En28, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, accen0::En29, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,accen0::En29, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, accen0::En30, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,accen0::En30, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for        transactions with the Master TAG ID j"]
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
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En1_SPEC;
    pub type En1 = crate::EnumBitfieldStruct<u8, En1_SPEC>;
    impl En1 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En2_SPEC;
    pub type En2 = crate::EnumBitfieldStruct<u8, En2_SPEC>;
    impl En2 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En3_SPEC;
    pub type En3 = crate::EnumBitfieldStruct<u8, En3_SPEC>;
    impl En3 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En4_SPEC;
    pub type En4 = crate::EnumBitfieldStruct<u8, En4_SPEC>;
    impl En4 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En5_SPEC;
    pub type En5 = crate::EnumBitfieldStruct<u8, En5_SPEC>;
    impl En5 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En6_SPEC;
    pub type En6 = crate::EnumBitfieldStruct<u8, En6_SPEC>;
    impl En6 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En7_SPEC;
    pub type En7 = crate::EnumBitfieldStruct<u8, En7_SPEC>;
    impl En7 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En8_SPEC;
    pub type En8 = crate::EnumBitfieldStruct<u8, En8_SPEC>;
    impl En8 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En9_SPEC;
    pub type En9 = crate::EnumBitfieldStruct<u8, En9_SPEC>;
    impl En9 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En10_SPEC;
    pub type En10 = crate::EnumBitfieldStruct<u8, En10_SPEC>;
    impl En10 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En11_SPEC;
    pub type En11 = crate::EnumBitfieldStruct<u8, En11_SPEC>;
    impl En11 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En12_SPEC;
    pub type En12 = crate::EnumBitfieldStruct<u8, En12_SPEC>;
    impl En12 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En13_SPEC;
    pub type En13 = crate::EnumBitfieldStruct<u8, En13_SPEC>;
    impl En13 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En14_SPEC;
    pub type En14 = crate::EnumBitfieldStruct<u8, En14_SPEC>;
    impl En14 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En15_SPEC;
    pub type En15 = crate::EnumBitfieldStruct<u8, En15_SPEC>;
    impl En15 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En16_SPEC;
    pub type En16 = crate::EnumBitfieldStruct<u8, En16_SPEC>;
    impl En16 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En17_SPEC;
    pub type En17 = crate::EnumBitfieldStruct<u8, En17_SPEC>;
    impl En17 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En18_SPEC;
    pub type En18 = crate::EnumBitfieldStruct<u8, En18_SPEC>;
    impl En18 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En19_SPEC;
    pub type En19 = crate::EnumBitfieldStruct<u8, En19_SPEC>;
    impl En19 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En20_SPEC;
    pub type En20 = crate::EnumBitfieldStruct<u8, En20_SPEC>;
    impl En20 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En21_SPEC;
    pub type En21 = crate::EnumBitfieldStruct<u8, En21_SPEC>;
    impl En21 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En22_SPEC;
    pub type En22 = crate::EnumBitfieldStruct<u8, En22_SPEC>;
    impl En22 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En23_SPEC;
    pub type En23 = crate::EnumBitfieldStruct<u8, En23_SPEC>;
    impl En23 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En24_SPEC;
    pub type En24 = crate::EnumBitfieldStruct<u8, En24_SPEC>;
    impl En24 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En25_SPEC;
    pub type En25 = crate::EnumBitfieldStruct<u8, En25_SPEC>;
    impl En25 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En26_SPEC;
    pub type En26 = crate::EnumBitfieldStruct<u8, En26_SPEC>;
    impl En26 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En27_SPEC;
    pub type En27 = crate::EnumBitfieldStruct<u8, En27_SPEC>;
    impl En27 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En28_SPEC;
    pub type En28 = crate::EnumBitfieldStruct<u8, En28_SPEC>;
    impl En28 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En29_SPEC;
    pub type En29 = crate::EnumBitfieldStruct<u8, En29_SPEC>;
    impl En29 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En30_SPEC;
    pub type En30 = crate::EnumBitfieldStruct<u8, En30_SPEC>;
    impl En30 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En31_SPEC;
    pub type En31 = crate::EnumBitfieldStruct<u8, En31_SPEC>;
    impl En31 {
        #[doc = "0 Write access        will not be executed. Read accesses will be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write and read        accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addrcfg_SPEC;
impl crate::sealed::RegSpec for Addrcfg_SPEC {
    type DataType = u32;
}
#[doc = "Address Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Addrcfg = crate::RegValueT<Addrcfg_SPEC>;

impl Addrcfg {
    #[doc = "Ten Bit Address Mode   TBAM. When this bit is zero  only bits 7 down to 1 of the ADR field are          valid."]
    #[inline(always)]
    pub fn tbam(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, addrcfg::Tbam, Addrcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,addrcfg::Tbam, Addrcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "General Call Enable   GCE"]
    #[inline(always)]
    pub fn gce(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, addrcfg::Gce, Addrcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,addrcfg::Gce, Addrcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Master Code Enable   MCE"]
    #[inline(always)]
    pub fn mce(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, addrcfg::Mce, Addrcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,addrcfg::Mce, Addrcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Master   not Slave   MnS"]
    #[inline(always)]
    pub fn mns(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, addrcfg::MnS, Addrcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,addrcfg::MnS, Addrcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Stop on Not acknowledge   SONA. After successful transmission of a master code  during high speed          mode  SONA is not considered till a stop condition is manually          generated by SETEND."]
    #[inline(always)]
    pub fn sona(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, addrcfg::Sona, Addrcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,addrcfg::Sona, Addrcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Stop on Packet End   SOPE. This bit field should be used only in Master Mode. In slave mode         should always be 0. If device works as receiver a not acknowledge is always generated         on package end. After successful transmission of a master code  during high speed         mode  SOPE is not considered till a stop condition is manually generated by SETEND."]
    #[inline(always)]
    pub fn sope(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, addrcfg::Sope, Addrcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,addrcfg::Sope, Addrcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Addrcfg {
    #[inline(always)]
    fn default() -> Addrcfg {
        <crate::RegValueT<Addrcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod addrcfg {
    pub struct Tbam_SPEC;
    pub type Tbam = crate::EnumBitfieldStruct<u8, Tbam_SPEC>;
    impl Tbam {
        #[doc = "0 7 bit address mode enabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 10 bit address mode enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Gce_SPEC;
    pub type Gce = crate::EnumBitfieldStruct<u8, Gce_SPEC>;
    impl Gce {
        #[doc = "0 Ignore general        call occurrence."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enable general        call detection  when detected  an acknowledge will be put on the bus"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mce_SPEC;
    pub type Mce = crate::EnumBitfieldStruct<u8, Mce_SPEC>;
    impl Mce {
        #[doc = "0 Device is not able to get along with high speed mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Device is able to handle master code"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct MnS_SPEC;
    pub type MnS = crate::EnumBitfieldStruct<u8, MnS_SPEC>;
    impl MnS {
        #[doc = "0 Peripheral is configured as slave"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Peripheral is configured as master"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sona_SPEC;
    pub type Sona = crate::EnumBitfieldStruct<u8, Sona_SPEC>;
    impl Sona {
        #[doc = "0 Device changes to MASTER RESTART state."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Device puts a stop condition on the bus and changes to LISTENING state."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sope_SPEC;
    pub type Sope = crate::EnumBitfieldStruct<u8, Sope_SPEC>;
    impl Sope {
        #[doc = "0 Device enters MASTER RESTART state when the data packet end is indicated by the FIFO."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Device puts a stop condition on the bus when the data packet end is indicated by the FIFO and changes to MASTER LISTENING state."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busstat_SPEC;
impl crate::sealed::RegSpec for Busstat_SPEC {
    type DataType = u32;
}
#[doc = "Bus Status Register\n resetvalue={Application Reset:0x0}"]
pub type Busstat = crate::RegValueT<Busstat_SPEC>;

impl Busstat {
    #[doc = "Bus Status   BS. Shows the current status on the I2C bus."]
    #[inline(always)]
    pub fn bs(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, busstat::Bs, Busstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3,1,0,busstat::Bs, Busstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Read not Write   RnW. Set by hardware automatically after address byte has been sent received."]
    #[inline(always)]
    pub fn rnw(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, busstat::RnW, Busstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1,1,0,busstat::RnW, Busstat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Busstat {
    #[inline(always)]
    fn default() -> Busstat {
        <crate::RegValueT<Busstat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod busstat {
    pub struct Bs_SPEC;
    pub type Bs = crate::EnumBitfieldStruct<u8, Bs_SPEC>;
    impl Bs {
        #[doc = "00 I2C bus is free  no start condition detected ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 A start condition has been detected on the bus  bus busy ."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 The device is working as master and has claimed the control on the I2C bus  busy master ."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 A remote master has accessed this device as slave."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct RnW_SPEC;
    pub type RnW = crate::EnumBitfieldStruct<u8, RnW_SPEC>;
    impl RnW {
        #[doc = "0 Working as transmitter  Write to I2C bus ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Working as receiver  Read from I2C bus ."]
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
}
impl core::default::Default for Clc {
    #[inline(always)]
    fn default() -> Clc {
        <crate::RegValueT<Clc_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clc1_SPEC;
impl crate::sealed::RegSpec for Clc1_SPEC {
    type DataType = u32;
}
#[doc = "Clock Control 1 Register\n resetvalue={Application Reset:0x3}"]
pub type Clc1 = crate::RegValueT<Clc1_SPEC>;

impl Clc1 {
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control of the module."]
    #[inline(always)]
    pub fn disr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, clc1::Disr, Clc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,clc1::Disr, Clc1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Module Disable Status Bit   DISS. Bit indicates the current status of the module."]
    #[inline(always)]
    pub fn diss(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, clc1::Diss, Clc1_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,clc1::Diss, Clc1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Module Suspend Enable Bit for OCDS   SPEN"]
    #[inline(always)]
    pub fn spen(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, clc1::Spen, Clc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,clc1::Spen, Clc1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "External Request Disable   EDIS. This bit is not used in TC39x and should always be written with 0."]
    #[inline(always)]
    pub fn edis(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, clc1::Edis, Clc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,clc1::Edis, Clc1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Module Suspend Bit Write Enable for OCDS   SBWE. This bit is always read as 0."]
    #[inline(always)]
    pub fn sbwe(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, clc1::Sbwe, Clc1_SPEC, crate::common::W> {
        crate::common::RegisterField::<4,0x1,1,0,clc1::Sbwe, Clc1_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Fast Switch Off Enable   FSOE"]
    #[inline(always)]
    pub fn fsoe(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, clc1::Fsoe, Clc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,clc1::Fsoe, Clc1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Divider for Standard Run Mode   RMC. Max. 8 bit divider value If RMC is set to 0 the module is disabled. As long as the new divider value RMC is not valid  reading register          returns 0000 00XX H"]
    #[inline(always)]
    pub fn rmc(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Clc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Clc1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Divider for Optional Run Mode  AHB peripherals    ORMC. This functionality is hidden for Aurix Family"]
    #[inline(always)]
    pub fn ormc(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Clc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Clc1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Clc1 {
    #[inline(always)]
    fn default() -> Clc1 {
        <crate::RegValueT<Clc1_SPEC> as RegisterValue<_>>::new(3)
    }
}
pub mod clc1 {
    pub struct Disr_SPEC;
    pub type Disr = crate::EnumBitfieldStruct<u8, Disr_SPEC>;
    impl Disr {
        #[doc = "0 Module disable        not requested"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module disable        requested"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Diss_SPEC;
    pub type Diss = crate::EnumBitfieldStruct<u8, Diss_SPEC>;
    impl Diss {
        #[doc = "0 Module enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Spen_SPEC;
    pub type Spen = crate::EnumBitfieldStruct<u8, Spen_SPEC>;
    impl Spen {
        #[doc = "0 Module suspend disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module suspend enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Edis_SPEC;
    pub type Edis = crate::EnumBitfieldStruct<u8, Edis_SPEC>;
    impl Edis {
        #[doc = "0 External clock disable request is enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 External clock disable request is disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sbwe_SPEC;
    pub type Sbwe = crate::EnumBitfieldStruct<u8, Sbwe_SPEC>;
    impl Sbwe {
        #[doc = "0 Bits SPEN and FSOE are write protected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bits SPEN and FSOE are overwritten by respective value of SPEN or FSOE"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fsoe_SPEC;
    pub type Fsoe = crate::EnumBitfieldStruct<u8, Fsoe_SPEC>;
    impl Fsoe {
        #[doc = "0 FSOE Clock switch off in OCDS suspend mode via Disable Control Feature  Secure Clock Switch Off"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Fast clock switch off in OCDS suspend mode"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmae_SPEC;
impl crate::sealed::RegSpec for Dmae_SPEC {
    type DataType = u32;
}
#[doc = "DMA Enable Register\n resetvalue={Application Reset:0x0}"]
pub type Dmae = crate::RegValueT<Dmae_SPEC>;

impl Dmae {
    #[doc = "Last Single Request Interrupt   LSREQ INT"]
    #[inline(always)]
    pub fn lsreq_int(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, dmae::LsreqInt, Dmae_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,dmae::LsreqInt, Dmae_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Single Request Interrupt   SREQ INT"]
    #[inline(always)]
    pub fn sreq_int(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, dmae::SreqInt, Dmae_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,dmae::SreqInt, Dmae_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Last Burst Request Interrupt   LBREQ INT"]
    #[inline(always)]
    pub fn lbreq_int(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, dmae::LbreqInt, Dmae_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,dmae::LbreqInt, Dmae_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Burst Request Interrupt   BREQ INT"]
    #[inline(always)]
    pub fn breq_int(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, dmae::BreqInt, Dmae_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,dmae::BreqInt, Dmae_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dmae {
    #[inline(always)]
    fn default() -> Dmae {
        <crate::RegValueT<Dmae_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dmae {
    pub struct LsreqInt_SPEC;
    pub type LsreqInt = crate::EnumBitfieldStruct<u8, LsreqInt_SPEC>;
    impl LsreqInt {
        #[doc = "0 DMA request disabled   Interrupt request enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 DMA request enabled  Interrupt request disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct SreqInt_SPEC;
    pub type SreqInt = crate::EnumBitfieldStruct<u8, SreqInt_SPEC>;
    impl SreqInt {
        #[doc = "0 DMA request disabled   Interrupt request enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 DMA request enabled  Interrupt request disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct LbreqInt_SPEC;
    pub type LbreqInt = crate::EnumBitfieldStruct<u8, LbreqInt_SPEC>;
    impl LbreqInt {
        #[doc = "0 DMA request disabled   Interrupt request enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 DMA request enabled  Interrupt request disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct BreqInt_SPEC;
    pub type BreqInt = crate::EnumBitfieldStruct<u8, BreqInt_SPEC>;
    impl BreqInt {
        #[doc = "0 DMA request disabled   Interrupt request enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 DMA request enabled  Interrupt request disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enddctrl_SPEC;
impl crate::sealed::RegSpec for Enddctrl_SPEC {
    type DataType = u32;
}
#[doc = "End Data Control Register\n resetvalue={Application Reset:0x0}"]
pub type Enddctrl = crate::RegValueT<Enddctrl_SPEC>;

impl Enddctrl {
    #[doc = "Set Restart Condition   SETRSC. This bit is always read as 0."]
    #[inline(always)]
    pub fn setrsc(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, enddctrl::Setrsc, Enddctrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            enddctrl::Setrsc,
            Enddctrl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Set End of Transmission   SETEND. This bit is always read as 0. Do not write 1 to this bit when bus is free. This will cause an abort          after the first byte when a new transfer is started.  This            8217 command  8217  is buffered internally and gets valid after the next byte is          transferred."]
    #[inline(always)]
    pub fn setend(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, enddctrl::Setend, Enddctrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            enddctrl::Setend,
            Enddctrl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Enddctrl {
    #[inline(always)]
    fn default() -> Enddctrl {
        <crate::RegValueT<Enddctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod enddctrl {
    pub struct Setrsc_SPEC;
    pub type Setrsc = crate::EnumBitfieldStruct<u8, Setrsc_SPEC>;
    impl Setrsc {
        #[doc = "0 Has no effect."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The master wants to restart a data transmission  changing slave direction . The effect depends on the current state. MASTER RECEIVES BYTES  The master puts a not acknowledge on the bus and switches to MASTER RESTART state. MASTER TRANSMITS BYTES  After the current byte has been sent  the master switches to MASTER RESTART state."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Setend_SPEC;
    pub type Setend = crate::EnumBitfieldStruct<u8, Setend_SPEC>;
    impl Setend {
        #[doc = "0 Has no effect."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The effect        depends on the current state. MASTER RECEIVES BYTES  After receiving the        current byte  the master puts a not acknowledge on the bus to indicate        the transmission end to the slave transmitter. Next it produces a stop        condition on the bus and changes its state to LISTENING. MASTER        TRANSMITS BYTES  After sending the current byte and receiving an        acknowledge or not acknowledge from the slave receiver  the master puts        a stop condition on the bus to close the data transmission and changes        its state to LISTENING. MASTER RESTART  The master puts a stop condition        on the bus to close the data transmission and changes its state to        LISTENING. SLAVE RECEIVES BYTES  The slave receiver puts a        not acknowledge on the bus after the received byte and changes its state        to TRANSMISSION FINISHED."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errirqsc_SPEC;
impl crate::sealed::RegSpec for Errirqsc_SPEC {
    type DataType = u32;
}
#[doc = "Error Interrupt Request Source Clear Register\n resetvalue={Application Reset:0x0}"]
pub type Errirqsc = crate::RegValueT<Errirqsc_SPEC>;

impl Errirqsc {
    #[doc = "RX FIFO Underflow   RXF UFL"]
    #[inline(always)]
    pub fn rxf_ufl(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, errirqsc::RxfUfl, Errirqsc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            errirqsc::RxfUfl,
            Errirqsc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "RX FIFO Overflow   RXF OFL"]
    #[inline(always)]
    pub fn rxf_ofl(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, errirqsc::RxfOfl, Errirqsc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            errirqsc::RxfOfl,
            Errirqsc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "TX FIFO Underflow   TXF UFL"]
    #[inline(always)]
    pub fn txf_ufl(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, errirqsc::TxfUfl, Errirqsc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            errirqsc::TxfUfl,
            Errirqsc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "TX FIFO Overflow   TXF OFL"]
    #[inline(always)]
    pub fn txf_ofl(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, errirqsc::TxfOfl, Errirqsc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            errirqsc::TxfOfl,
            Errirqsc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Errirqsc {
    #[inline(always)]
    fn default() -> Errirqsc {
        <crate::RegValueT<Errirqsc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod errirqsc {
    pub struct RxfUfl_SPEC;
    pub type RxfUfl = crate::EnumBitfieldStruct<u8, RxfUfl_SPEC>;
    impl RxfUfl {
        #[doc = "0 No change"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear interrupt request source"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct RxfOfl_SPEC;
    pub type RxfOfl = crate::EnumBitfieldStruct<u8, RxfOfl_SPEC>;
    impl RxfOfl {
        #[doc = "0 No change"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear interrupt request source"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TxfUfl_SPEC;
    pub type TxfUfl = crate::EnumBitfieldStruct<u8, TxfUfl_SPEC>;
    impl TxfUfl {
        #[doc = "0 No change"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear interrupt request source"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TxfOfl_SPEC;
    pub type TxfOfl = crate::EnumBitfieldStruct<u8, TxfOfl_SPEC>;
    impl TxfOfl {
        #[doc = "0 No change"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear interrupt request source"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errirqsm_SPEC;
impl crate::sealed::RegSpec for Errirqsm_SPEC {
    type DataType = u32;
}
#[doc = "Error Interrupt Request Source Mask Register\n resetvalue={Application Reset:0x0F}"]
pub type Errirqsm = crate::RegValueT<Errirqsm_SPEC>;

impl Errirqsm {
    #[doc = "RX FIFO Underflow   RXF UFL"]
    #[inline(always)]
    pub fn rxf_ufl(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        errirqsm::RxfUfl,
        Errirqsm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            errirqsm::RxfUfl,
            Errirqsm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "RX FIFO Overflow   RXF OFL"]
    #[inline(always)]
    pub fn rxf_ofl(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        errirqsm::RxfOfl,
        Errirqsm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            errirqsm::RxfOfl,
            Errirqsm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "TX FIFO Underflow   TXF UFL"]
    #[inline(always)]
    pub fn txf_ufl(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        errirqsm::TxfUfl,
        Errirqsm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            errirqsm::TxfUfl,
            Errirqsm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "TX FIFO Overflow   TXF OFL"]
    #[inline(always)]
    pub fn txf_ofl(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        errirqsm::TxfOfl,
        Errirqsm_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            errirqsm::TxfOfl,
            Errirqsm_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Errirqsm {
    #[inline(always)]
    fn default() -> Errirqsm {
        <crate::RegValueT<Errirqsm_SPEC> as RegisterValue<_>>::new(15)
    }
}
pub mod errirqsm {
    pub struct RxfUfl_SPEC;
    pub type RxfUfl = crate::EnumBitfieldStruct<u8, RxfUfl_SPEC>;
    impl RxfUfl {
        #[doc = "0 Interrupt request source disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request source enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct RxfOfl_SPEC;
    pub type RxfOfl = crate::EnumBitfieldStruct<u8, RxfOfl_SPEC>;
    impl RxfOfl {
        #[doc = "0 Interrupt request source disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request source enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TxfUfl_SPEC;
    pub type TxfUfl = crate::EnumBitfieldStruct<u8, TxfUfl_SPEC>;
    impl TxfUfl {
        #[doc = "0 Interrupt request source disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request source enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TxfOfl_SPEC;
    pub type TxfOfl = crate::EnumBitfieldStruct<u8, TxfOfl_SPEC>;
    impl TxfOfl {
        #[doc = "0 Interrupt request source disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request source enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errirqss_SPEC;
impl crate::sealed::RegSpec for Errirqss_SPEC {
    type DataType = u32;
}
#[doc = "Error Interrupt Request Source Status Register\n resetvalue={Application Reset:0x0}"]
pub type Errirqss = crate::RegValueT<Errirqss_SPEC>;

impl Errirqss {
    #[doc = "RX FIFO Underflow   RXF UFL. The FIFO has detected an RX FIFO underflow."]
    #[inline(always)]
    pub fn rxf_ufl(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, errirqss::RxfUfl, Errirqss_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            errirqss::RxfUfl,
            Errirqss_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "RX FIFO Overflow   RXF OFL. The I2C kernel has detected a RX FIFO overflow"]
    #[inline(always)]
    pub fn rxf_ofl(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, errirqss::RxfOfl, Errirqss_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            errirqss::RxfOfl,
            Errirqss_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "TX FIFO Underflow   TXF UFL. The I2C kernel has detected a TX FIFO underflow."]
    #[inline(always)]
    pub fn txf_ufl(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, errirqss::TxfUfl, Errirqss_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            errirqss::TxfUfl,
            Errirqss_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "TX FIFO Overflow   TXF OFL. The FIFO has detected a TX FIFO overflow."]
    #[inline(always)]
    pub fn txf_ofl(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, errirqss::TxfOfl, Errirqss_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            errirqss::TxfOfl,
            Errirqss_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Errirqss {
    #[inline(always)]
    fn default() -> Errirqss {
        <crate::RegValueT<Errirqss_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod errirqss {
    pub struct RxfUfl_SPEC;
    pub type RxfUfl = crate::EnumBitfieldStruct<u8, RxfUfl_SPEC>;
    impl RxfUfl {
        #[doc = "0 No interrupt request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct RxfOfl_SPEC;
    pub type RxfOfl = crate::EnumBitfieldStruct<u8, RxfOfl_SPEC>;
    impl RxfOfl {
        #[doc = "0 No interrupt request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TxfUfl_SPEC;
    pub type TxfUfl = crate::EnumBitfieldStruct<u8, TxfUfl_SPEC>;
    impl TxfUfl {
        #[doc = "0 No interrupt request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TxfOfl_SPEC;
    pub type TxfOfl = crate::EnumBitfieldStruct<u8, TxfOfl_SPEC>;
    impl TxfOfl {
        #[doc = "0 No interrupt request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdivcfg_SPEC;
impl crate::sealed::RegSpec for Fdivcfg_SPEC {
    type DataType = u32;
}
#[doc = "Fractional Divider Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Fdivcfg = crate::RegValueT<Fdivcfg_SPEC>;

impl Fdivcfg {
    #[doc = "Decrement Value of Fractional Divider   DEC. For standard fast mode  see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn dec(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Fdivcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Fdivcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Increment Value of Fractional Divider   INC. For standard fast mode  see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn inc(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Fdivcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Fdivcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Fdivcfg {
    #[inline(always)]
    fn default() -> Fdivcfg {
        <crate::RegValueT<Fdivcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdivhighcfg_SPEC;
impl crate::sealed::RegSpec for Fdivhighcfg_SPEC {
    type DataType = u32;
}
#[doc = "Fractional Divider High speed Mode Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Fdivhighcfg = crate::RegValueT<Fdivhighcfg_SPEC>;

impl Fdivhighcfg {
    #[doc = "Decrement Value of Fractional Divider   DEC. For high speed mode  see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn dec(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Fdivhighcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Fdivhighcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Increment Value of Fractional Divider   INC. For high speed mode  see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn inc(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Fdivhighcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Fdivhighcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disable SCL Synchronization   DIS SCL SYN. This bit is internal only  Disable I2C SCL synchronization to allow characterization of the I2C        master mode  standard and fast mode with the specified I2C kernel clock        and high speed mode with 3.4 Mbit s ."]
    #[inline(always)]
    pub fn dis_scl_syn(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Fdivhighcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,Fdivhighcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Fdivhighcfg {
    #[inline(always)]
    fn default() -> Fdivhighcfg {
        <crate::RegValueT<Fdivhighcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ffsstat_SPEC;
impl crate::sealed::RegSpec for Ffsstat_SPEC {
    type DataType = u32;
}
#[doc = "Filled FIFO Stages Status Register\n resetvalue={Application Reset:0x0}"]
pub type Ffsstat = crate::RegValueT<Ffsstat_SPEC>;

impl Ffsstat {
    #[doc = "Filled FIFO Stages   FFS. Number of filled FIFO stages  0 to 8"]
    #[inline(always)]
    pub fn ffs(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Ffsstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Ffsstat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ffsstat {
    #[inline(always)]
    fn default() -> Ffsstat {
        <crate::RegValueT<Ffsstat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifocfg_SPEC;
impl crate::sealed::RegSpec for Fifocfg_SPEC {
    type DataType = u32;
}
#[doc = "FIFO Configuration Register\n resetvalue={Application Reset:0x22}"]
pub type Fifocfg = crate::RegValueT<Fifocfg_SPEC>;

impl Fifocfg {
    #[doc = "RX Burst Size   RXBS. Some DMA controllers do not support a burst size of 2 words."]
    #[inline(always)]
    pub fn rxbs(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, fifocfg::Rxbs, Fifocfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,fifocfg::Rxbs, Fifocfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX Burst Size   TXBS. Some DMA controllers do not support a burst size of 2 words."]
    #[inline(always)]
    pub fn txbs(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, fifocfg::Txbs, Fifocfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,fifocfg::Txbs, Fifocfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RX FIFO Alignment   RXFA. Use byte alignment wherever it is possible."]
    #[inline(always)]
    pub fn rxfa(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, fifocfg::Rxfa, Fifocfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,fifocfg::Rxfa, Fifocfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX FIFO Alignment   TXFA. Use byte alignment wherever it is possible."]
    #[inline(always)]
    pub fn txfa(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, fifocfg::Txfa, Fifocfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,fifocfg::Txfa, Fifocfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RX FIFO Flow Control   RXFC"]
    #[inline(always)]
    pub fn rxfc(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, fifocfg::Rxfc, Fifocfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,fifocfg::Rxfc, Fifocfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX FIFO Flow Control   TXFC"]
    #[inline(always)]
    pub fn txfc(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, fifocfg::Txfc, Fifocfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,fifocfg::Txfc, Fifocfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear Request Behavior Configuration   CRBC. Used to configure the clear request behavior for the FIFO data request.        Can only be used for single request and must be set to   8220 0  8221  when burst        accesses are used in the system  eg. when TX RXBS  gt  0"]
    #[inline(always)]
    pub fn crbc(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, fifocfg::Crbc, Fifocfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,fifocfg::Crbc, Fifocfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Fifocfg {
    #[inline(always)]
    fn default() -> Fifocfg {
        <crate::RegValueT<Fifocfg_SPEC> as RegisterValue<_>>::new(34)
    }
}
pub mod fifocfg {
    pub struct Rxbs_SPEC;
    pub type Rxbs = crate::EnumBitfieldStruct<u8, Rxbs_SPEC>;
    impl Rxbs {
        #[doc = "00 1 word"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 2 words"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 4 words"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Do not use this combination"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Txbs_SPEC;
    pub type Txbs = crate::EnumBitfieldStruct<u8, Txbs_SPEC>;
    impl Txbs {
        #[doc = "00 1 word"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 2 words"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 4 words"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Do not use this combination"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Rxfa_SPEC;
    pub type Rxfa = crate::EnumBitfieldStruct<u8, Rxfa_SPEC>;
    impl Rxfa {
        #[doc = "00 Byte aligned  character alignment"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Half word aligned  character alignment of two characters"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Word aligned  character alignment of four characters"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Do not use this combination"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Txfa_SPEC;
    pub type Txfa = crate::EnumBitfieldStruct<u8, Txfa_SPEC>;
    impl Txfa {
        #[doc = "00 Byte aligned  character alignment"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Half word aligned  character alignment of two characters"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Word aligned  character alignment of four characters"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Do not use this combination"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Rxfc_SPEC;
    pub type Rxfc = crate::EnumBitfieldStruct<u8, Rxfc_SPEC>;
    impl Rxfc {
        #[doc = "0 RX FIFO not as flow controller"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 RX FIFO as flow controller"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Txfc_SPEC;
    pub type Txfc = crate::EnumBitfieldStruct<u8, Txfc_SPEC>;
    impl Txfc {
        #[doc = "0 TX FIFO not as flow controller"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 TX FIFO as flow controller"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Crbc_SPEC;
    pub type Crbc = crate::EnumBitfieldStruct<u8, Crbc_SPEC>;
    impl Crbc {
        #[doc = "0 Data request is cleared by Software."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Data request is cleared automatically when Write Read access to FIFO occurs."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpctl_SPEC;
impl crate::sealed::RegSpec for Gpctl_SPEC {
    type DataType = u32;
}
#[doc = "General Purpose Control Register\n resetvalue={Application Reset:0x0}"]
pub type Gpctl = crate::RegValueT<Gpctl_SPEC>;

impl Gpctl {
    #[doc = "Port Input Select   PISEL. Used to select the input pins providing the serial data and clock input        signals  in the range of 0 to 7. In TC39x   not all PISEL          options are available. See Data Sheet."]
    #[inline(always)]
    pub fn pisel(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, gpctl::Pisel, Gpctl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,gpctl::Pisel, Gpctl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gpctl {
    #[inline(always)]
    fn default() -> Gpctl {
        <crate::RegValueT<Gpctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gpctl {
    pub struct Pisel_SPEC;
    pub type Pisel = crate::EnumBitfieldStruct<u8, Pisel_SPEC>;
    impl Pisel {
        #[doc = "000 SDA0A and SCL0A are selected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 SDA0B and SCL0B are selected."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 SDA0C and SCL0C are selected."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 SDA0D and SCL0D are selected."]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 SDA0E and SCL0E are selected."]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 SDA0F and SCL0F are selected."]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 SDA0G and SCL0G are selected."]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "111 SDA0H and SCL0H are selected."]
        pub const CONST_77: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpctrl_SPEC;
impl crate::sealed::RegSpec for Gpctrl_SPEC {
    type DataType = u32;
}
#[doc = "General Purpose Control Register\n resetvalue={Application Reset:0x0F0}"]
pub type Gpctrl = crate::RegValueT<Gpctrl_SPEC>;

impl Gpctrl {
    #[doc = "General Purpose Control Bits   GPCTRL. These register bits are directly connected to the respective general        purpose outputs of the I2C 11110000"]
    #[inline(always)]
    pub fn gpctrl(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Gpctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Gpctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gpctrl {
    #[inline(always)]
    fn default() -> Gpctrl {
        <crate::RegValueT<Gpctrl_SPEC> as RegisterValue<_>>::new(240)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr_SPEC;
impl crate::sealed::RegSpec for Icr_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Clear Register\n resetvalue={Application Reset:0x0}"]
pub type Icr = crate::RegValueT<Icr_SPEC>;

impl Icr {
    #[doc = "Last Single Request Interrupt   LSREQ INT"]
    #[inline(always)]
    pub fn lsreq_int(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, icr::LsreqInt, Icr_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0x1,1,0,icr::LsreqInt, Icr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Single Request Interrupt   SREQ INT"]
    #[inline(always)]
    pub fn sreq_int(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, icr::SreqInt, Icr_SPEC, crate::common::W> {
        crate::common::RegisterField::<1,0x1,1,0,icr::SreqInt, Icr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Last Burst Request Interrupt   LBREQ INT"]
    #[inline(always)]
    pub fn lbreq_int(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, icr::LbreqInt, Icr_SPEC, crate::common::W> {
        crate::common::RegisterField::<2,0x1,1,0,icr::LbreqInt, Icr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Burst Request Interrupt   BREQ INT"]
    #[inline(always)]
    pub fn breq_int(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, icr::BreqInt, Icr_SPEC, crate::common::W> {
        crate::common::RegisterField::<3,0x1,1,0,icr::BreqInt, Icr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        <crate::RegValueT<Icr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icr {
    pub struct LsreqInt_SPEC;
    pub type LsreqInt = crate::EnumBitfieldStruct<u8, LsreqInt_SPEC>;
    impl LsreqInt {
        #[doc = "0 No change"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear interrupt request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct SreqInt_SPEC;
    pub type SreqInt = crate::EnumBitfieldStruct<u8, SreqInt_SPEC>;
    impl SreqInt {
        #[doc = "0 No change"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear interrupt request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct LbreqInt_SPEC;
    pub type LbreqInt = crate::EnumBitfieldStruct<u8, LbreqInt_SPEC>;
    impl LbreqInt {
        #[doc = "0 No change"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear interrupt request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct BreqInt_SPEC;
    pub type BreqInt = crate::EnumBitfieldStruct<u8, BreqInt_SPEC>;
    impl BreqInt {
        #[doc = "0 No change"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear interrupt request"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x5703}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MOD REV. This bit field defines the revision number."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, id::ModRev, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,id::ModRev, Id_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Module Number   MOD NUMBER. This bit field defines the module identification number."]
    #[inline(always)]
    pub fn mod_number(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Id {
    #[inline(always)]
    fn default() -> Id {
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(22275)
    }
}
pub mod id {
    pub struct ModRev_SPEC;
    pub type ModRev = crate::EnumBitfieldStruct<u8, ModRev_SPEC>;
    impl ModRev {
        #[doc = "3 v1.8.0  version        used for TC39x"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "4 v1.9.0"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "5 v1.10.0"]
        pub const CONST_55: Self = Self::new(5);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imsc_SPEC;
impl crate::sealed::RegSpec for Imsc_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Mask Control Register\n resetvalue={Application Reset:0x0}"]
pub type Imsc = crate::RegValueT<Imsc_SPEC>;

impl Imsc {
    #[doc = "Last Single Request Interrupt   LSREQ INT"]
    #[inline(always)]
    pub fn lsreq_int(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, imsc::LsreqInt, Imsc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,imsc::LsreqInt, Imsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Single Request Interrupt   SREQ INT"]
    #[inline(always)]
    pub fn sreq_int(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, imsc::SreqInt, Imsc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,imsc::SreqInt, Imsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Last Burst Request Interrupt   LBREQ INT"]
    #[inline(always)]
    pub fn lbreq_int(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, imsc::LbreqInt, Imsc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,imsc::LbreqInt, Imsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Burst Request Interrupt   BREQ INT"]
    #[inline(always)]
    pub fn breq_int(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, imsc::BreqInt, Imsc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,imsc::BreqInt, Imsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "I2C Error Interrupt   I2C ERR INT"]
    #[inline(always)]
    pub fn i2c_err_int(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, imsc::I2CErrInt, Imsc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,imsc::I2CErrInt, Imsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "I2C Protocol Interrupt   I2C P INT"]
    #[inline(always)]
    pub fn i2c_p_int(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, imsc::I2CPInt, Imsc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,imsc::I2CPInt, Imsc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Imsc {
    #[inline(always)]
    fn default() -> Imsc {
        <crate::RegValueT<Imsc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod imsc {
    pub struct LsreqInt_SPEC;
    pub type LsreqInt = crate::EnumBitfieldStruct<u8, LsreqInt_SPEC>;
    impl LsreqInt {
        #[doc = "0 Interrupt request disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct SreqInt_SPEC;
    pub type SreqInt = crate::EnumBitfieldStruct<u8, SreqInt_SPEC>;
    impl SreqInt {
        #[doc = "0 Interrupt request disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct LbreqInt_SPEC;
    pub type LbreqInt = crate::EnumBitfieldStruct<u8, LbreqInt_SPEC>;
    impl LbreqInt {
        #[doc = "0 Interrupt request disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct BreqInt_SPEC;
    pub type BreqInt = crate::EnumBitfieldStruct<u8, BreqInt_SPEC>;
    impl BreqInt {
        #[doc = "0 Interrupt request disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct I2CErrInt_SPEC;
    pub type I2CErrInt = crate::EnumBitfieldStruct<u8, I2CErrInt_SPEC>;
    impl I2CErrInt {
        #[doc = "0 Interrupt request disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct I2CPInt_SPEC;
    pub type I2CPInt = crate::EnumBitfieldStruct<u8, I2CPInt_SPEC>;
    impl I2CPInt {
        #[doc = "0 Interrupt request disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr_SPEC;
impl crate::sealed::RegSpec for Isr_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Set Register\n resetvalue={Application Reset:0x0}"]
pub type Isr = crate::RegValueT<Isr_SPEC>;

impl Isr {
    #[doc = "Last Single Request Interrupt   LSREQ INT"]
    #[inline(always)]
    pub fn lsreq_int(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, isr::LsreqInt, Isr_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0x1,1,0,isr::LsreqInt, Isr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Single Request Interrupt   SREQ INT"]
    #[inline(always)]
    pub fn sreq_int(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, isr::SreqInt, Isr_SPEC, crate::common::W> {
        crate::common::RegisterField::<1,0x1,1,0,isr::SreqInt, Isr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Last Burst Request Interrupt   LBREQ INT"]
    #[inline(always)]
    pub fn lbreq_int(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, isr::LbreqInt, Isr_SPEC, crate::common::W> {
        crate::common::RegisterField::<2,0x1,1,0,isr::LbreqInt, Isr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Burst Request Interrupt   BREQ INT"]
    #[inline(always)]
    pub fn breq_int(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, isr::BreqInt, Isr_SPEC, crate::common::W> {
        crate::common::RegisterField::<3,0x1,1,0,isr::BreqInt, Isr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "I2C Error Interrupt   I2C ERR INT"]
    #[inline(always)]
    pub fn i2c_err_int(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, isr::I2CErrInt, Isr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<4,0x1,1,0,isr::I2CErrInt, Isr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "I2C Protocol Interrupt   I2C P INT"]
    #[inline(always)]
    pub fn i2c_p_int(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, isr::I2CPInt, Isr_SPEC, crate::common::W> {
        crate::common::RegisterField::<5,0x1,1,0,isr::I2CPInt, Isr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Isr {
    #[inline(always)]
    fn default() -> Isr {
        <crate::RegValueT<Isr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod isr {
    pub struct LsreqInt_SPEC;
    pub type LsreqInt = crate::EnumBitfieldStruct<u8, LsreqInt_SPEC>;
    impl LsreqInt {
        #[doc = "0 No change"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set interrupt request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct SreqInt_SPEC;
    pub type SreqInt = crate::EnumBitfieldStruct<u8, SreqInt_SPEC>;
    impl SreqInt {
        #[doc = "0 No change"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set interrupt request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct LbreqInt_SPEC;
    pub type LbreqInt = crate::EnumBitfieldStruct<u8, LbreqInt_SPEC>;
    impl LbreqInt {
        #[doc = "0 No change"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set interrupt request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct BreqInt_SPEC;
    pub type BreqInt = crate::EnumBitfieldStruct<u8, BreqInt_SPEC>;
    impl BreqInt {
        #[doc = "0 No change"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set interrupt request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct I2CErrInt_SPEC;
    pub type I2CErrInt = crate::EnumBitfieldStruct<u8, I2CErrInt_SPEC>;
    impl I2CErrInt {
        #[doc = "0 No change"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set interrupt request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct I2CPInt_SPEC;
    pub type I2CPInt = crate::EnumBitfieldStruct<u8, I2CPInt_SPEC>;
    impl I2CPInt {
        #[doc = "0 No change"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set interrupt request"]
        pub const CONST_11: Self = Self::new(1);
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel registers are set. The RST bit will be cleared  reset to 0   b after the kernel reset was executed."]
    #[inline(always)]
    pub fn rst(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, krst0::Rst, Krst0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,krst0::Rst, Krst0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Kernel Reset Status   RSTSTAT. This bit indicates whether a kernel reset was executed or not. This bit is set after the execution of a kernel reset in the same clock cycle both reset bits. This bit can be cleared by writing with  1  to the CLR bit in the related KRSTCLR register."]
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel reset registers is set. The RST bit will be cleared  re set to 0   after the kernel reset was executed."]
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
    #[doc = "Kernel Reset Status Clear   CLR. Read always as 0 ."]
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
pub struct Mis_SPEC;
impl crate::sealed::RegSpec for Mis_SPEC {
    type DataType = u32;
}
#[doc = "Masked Interrupt Status Register\n resetvalue={Application Reset:0x0}"]
pub type Mis = crate::RegValueT<Mis_SPEC>;

impl Mis {
    #[doc = "Last Single Request Interrupt   LSREQ INT"]
    #[inline(always)]
    pub fn lsreq_int(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mis::LsreqInt, Mis_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,mis::LsreqInt, Mis_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Single Request Interrupt   SREQ INT"]
    #[inline(always)]
    pub fn sreq_int(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, mis::SreqInt, Mis_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,mis::SreqInt, Mis_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Last Burst Request Interrupt   LBREQ INT"]
    #[inline(always)]
    pub fn lbreq_int(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, mis::LbreqInt, Mis_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,mis::LbreqInt, Mis_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Burst Request Interrupt   BREQ INT"]
    #[inline(always)]
    pub fn breq_int(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, mis::BreqInt, Mis_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1,1,0,mis::BreqInt, Mis_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "I2C Error Interrupt   I2C ERR INT. This is the combined bit for indication of FIFO errors due to overflow and underflow."]
    #[inline(always)]
    pub fn i2c_err_int(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, mis::I2CErrInt, Mis_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,mis::I2CErrInt, Mis_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "I2C Protocol Interrupt   I2C P INT. This is the combined bit for indication of a protocol event in the I2C kernel."]
    #[inline(always)]
    pub fn i2c_p_int(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, mis::I2CPInt, Mis_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x1,1,0,mis::I2CPInt, Mis_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Mis {
    #[inline(always)]
    fn default() -> Mis {
        <crate::RegValueT<Mis_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mis {
    pub struct LsreqInt_SPEC;
    pub type LsreqInt = crate::EnumBitfieldStruct<u8, LsreqInt_SPEC>;
    impl LsreqInt {
        #[doc = "0 No interrupt request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct SreqInt_SPEC;
    pub type SreqInt = crate::EnumBitfieldStruct<u8, SreqInt_SPEC>;
    impl SreqInt {
        #[doc = "0 No interrupt request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct LbreqInt_SPEC;
    pub type LbreqInt = crate::EnumBitfieldStruct<u8, LbreqInt_SPEC>;
    impl LbreqInt {
        #[doc = "0 No interrupt request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct BreqInt_SPEC;
    pub type BreqInt = crate::EnumBitfieldStruct<u8, BreqInt_SPEC>;
    impl BreqInt {
        #[doc = "0 No interrupt request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct I2CErrInt_SPEC;
    pub type I2CErrInt = crate::EnumBitfieldStruct<u8, I2CErrInt_SPEC>;
    impl I2CErrInt {
        #[doc = "0 No interrupt request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct I2CPInt_SPEC;
    pub type I2CPInt = crate::EnumBitfieldStruct<u8, I2CPInt_SPEC>;
    impl I2CPInt {
        #[doc = "0 No interrupt request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modid_SPEC;
impl crate::sealed::RegSpec for Modid_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0C2C000}"]
pub type Modid = crate::RegValueT<Modid_SPEC>;

impl Modid {
    #[doc = "Module Revision Number   MOD REV. This bit field defines the module revision number. The value of a module revision starts with 01H  first revision ."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Modid_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Modid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Module Type   MOD TYPE. The bit field is set to C0H which defines the module as a 32 bit module."]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Modid_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Modid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Module Number Value   MOD NUMBER. This bit field defines a module identification number."]
    #[inline(always)]
    pub fn mod_number(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Modid_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Modid_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Modid {
    #[inline(always)]
    fn default() -> Modid {
        <crate::RegValueT<Modid_SPEC> as RegisterValue<_>>::new(12763136)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrpsctrl_SPEC;
impl crate::sealed::RegSpec for Mrpsctrl_SPEC {
    type DataType = u32;
}
#[doc = "Maximum Received Packet Size Control Register\n resetvalue={Application Reset:0x0}"]
pub type Mrpsctrl = crate::RegValueT<Mrpsctrl_SPEC>;

impl Mrpsctrl {
    #[doc = "Maximum Received Packet Size   MRPS. Length in characters of packet to be received  write value range  0         unlimited size  to 16383 Reading returns the written value as long as the previous packet has not        been read completely from the FIFO. After that  MRPS is loaded to an        internal register  reading returns 0 and a new value can be written."]
    #[inline(always)]
    pub fn mrps(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Mrpsctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Mrpsctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Mrpsctrl {
    #[inline(always)]
    fn default() -> Mrpsctrl {
        <crate::RegValueT<Mrpsctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pirqsc_SPEC;
impl crate::sealed::RegSpec for Pirqsc_SPEC {
    type DataType = u32;
}
#[doc = "Protocol Interrupt Request Source Clear Register\n resetvalue={Application Reset:0x0}"]
pub type Pirqsc = crate::RegValueT<Pirqsc_SPEC>;

impl Pirqsc {
    #[doc = "Address Match   AM"]
    #[inline(always)]
    pub fn am(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, pirqsc::Am, Pirqsc_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0x1,1,0,pirqsc::Am, Pirqsc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "General Call   GC"]
    #[inline(always)]
    pub fn gc(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, pirqsc::Gc, Pirqsc_SPEC, crate::common::W> {
        crate::common::RegisterField::<1,0x1,1,0,pirqsc::Gc, Pirqsc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Master Code   MC"]
    #[inline(always)]
    pub fn mc(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, pirqsc::Mc, Pirqsc_SPEC, crate::common::W> {
        crate::common::RegisterField::<2,0x1,1,0,pirqsc::Mc, Pirqsc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Arbitration Lost   AL"]
    #[inline(always)]
    pub fn al(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, pirqsc::Al, Pirqsc_SPEC, crate::common::W> {
        crate::common::RegisterField::<3,0x1,1,0,pirqsc::Al, Pirqsc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Not acknowledge Received   NACK"]
    #[inline(always)]
    pub fn nack(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, pirqsc::Nack, Pirqsc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<4,0x1,1,0,pirqsc::Nack, Pirqsc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Transmission End   TX END"]
    #[inline(always)]
    pub fn tx_end(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, pirqsc::TxEnd, Pirqsc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<5,0x1,1,0,pirqsc::TxEnd, Pirqsc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Receive Mode   RX"]
    #[inline(always)]
    pub fn rx(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, pirqsc::Rx, Pirqsc_SPEC, crate::common::W> {
        crate::common::RegisterField::<6,0x1,1,0,pirqsc::Rx, Pirqsc_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Pirqsc {
    #[inline(always)]
    fn default() -> Pirqsc {
        <crate::RegValueT<Pirqsc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pirqsc {
    pub struct Am_SPEC;
    pub type Am = crate::EnumBitfieldStruct<u8, Am_SPEC>;
    impl Am {
        #[doc = "0 No change"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear Interrupt source"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Gc_SPEC;
    pub type Gc = crate::EnumBitfieldStruct<u8, Gc_SPEC>;
    impl Gc {
        #[doc = "0 No change"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear Interrupt source"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mc_SPEC;
    pub type Mc = crate::EnumBitfieldStruct<u8, Mc_SPEC>;
    impl Mc {
        #[doc = "0 No change"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear Interrupt source"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Al_SPEC;
    pub type Al = crate::EnumBitfieldStruct<u8, Al_SPEC>;
    impl Al {
        #[doc = "0 No change"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear Interrupt source"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Nack_SPEC;
    pub type Nack = crate::EnumBitfieldStruct<u8, Nack_SPEC>;
    impl Nack {
        #[doc = "0 No change"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear Interrupt source"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TxEnd_SPEC;
    pub type TxEnd = crate::EnumBitfieldStruct<u8, TxEnd_SPEC>;
    impl TxEnd {
        #[doc = "0 No change"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear Interrupt source"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rx_SPEC;
    pub type Rx = crate::EnumBitfieldStruct<u8, Rx_SPEC>;
    impl Rx {
        #[doc = "0 No change"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear Interrupt source"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pirqsm_SPEC;
impl crate::sealed::RegSpec for Pirqsm_SPEC {
    type DataType = u32;
}
#[doc = "Protocol Interrupt Request Source Mask Register\n resetvalue={Application Reset:0x7F}"]
pub type Pirqsm = crate::RegValueT<Pirqsm_SPEC>;

impl Pirqsm {
    #[doc = "Address Match   AM"]
    #[inline(always)]
    pub fn am(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, pirqsm::Am, Pirqsm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,pirqsm::Am, Pirqsm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "General Call   GC"]
    #[inline(always)]
    pub fn gc(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, pirqsm::Gc, Pirqsm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,pirqsm::Gc, Pirqsm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Master Code   MC"]
    #[inline(always)]
    pub fn mc(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, pirqsm::Mc, Pirqsm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,pirqsm::Mc, Pirqsm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Arbitration Lost   AL"]
    #[inline(always)]
    pub fn al(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, pirqsm::Al, Pirqsm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,pirqsm::Al, Pirqsm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Not acknowledge Received   NACK"]
    #[inline(always)]
    pub fn nack(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, pirqsm::Nack, Pirqsm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,pirqsm::Nack, Pirqsm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmission End   TX END"]
    #[inline(always)]
    pub fn tx_end(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, pirqsm::TxEnd, Pirqsm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,pirqsm::TxEnd, Pirqsm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive Mode   RX"]
    #[inline(always)]
    pub fn rx(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, pirqsm::Rx, Pirqsm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,pirqsm::Rx, Pirqsm_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pirqsm {
    #[inline(always)]
    fn default() -> Pirqsm {
        <crate::RegValueT<Pirqsm_SPEC> as RegisterValue<_>>::new(127)
    }
}
pub mod pirqsm {
    pub struct Am_SPEC;
    pub type Am = crate::EnumBitfieldStruct<u8, Am_SPEC>;
    impl Am {
        #[doc = "0 Interrupt request source disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request source enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Gc_SPEC;
    pub type Gc = crate::EnumBitfieldStruct<u8, Gc_SPEC>;
    impl Gc {
        #[doc = "0 Interrupt request source disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request source enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mc_SPEC;
    pub type Mc = crate::EnumBitfieldStruct<u8, Mc_SPEC>;
    impl Mc {
        #[doc = "0 Interrupt request source disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request source enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Al_SPEC;
    pub type Al = crate::EnumBitfieldStruct<u8, Al_SPEC>;
    impl Al {
        #[doc = "0 Interrupt request source disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request source enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Nack_SPEC;
    pub type Nack = crate::EnumBitfieldStruct<u8, Nack_SPEC>;
    impl Nack {
        #[doc = "0 Interrupt request source disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request source enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TxEnd_SPEC;
    pub type TxEnd = crate::EnumBitfieldStruct<u8, TxEnd_SPEC>;
    impl TxEnd {
        #[doc = "0 Interrupt request source disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request source enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rx_SPEC;
    pub type Rx = crate::EnumBitfieldStruct<u8, Rx_SPEC>;
    impl Rx {
        #[doc = "0 Interrupt request source disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request source enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pirqss_SPEC;
impl crate::sealed::RegSpec for Pirqss_SPEC {
    type DataType = u32;
}
#[doc = "Protocol Interrupt Request Source Status Register\n resetvalue={Application Reset:0x0}"]
pub type Pirqss = crate::RegValueT<Pirqss_SPEC>;

impl Pirqss {
    #[doc = "Address Match   AM. Device  in master or slave mode  is addressed by remote master  matching        device address . Accordingly  bit field BS in register BUSSTAT is set to 11 ."]
    #[inline(always)]
    pub fn am(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, pirqss::Am, Pirqss_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,pirqss::Am, Pirqss_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "General Call   GC. Remote master has transmitted a general call."]
    #[inline(always)]
    pub fn gc(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, pirqss::Gc, Pirqss_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,pirqss::Gc, Pirqss_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Master Code   MC. Remote master has transmitted a master call."]
    #[inline(always)]
    pub fn mc(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, pirqss::Mc, Pirqss_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,pirqss::Mc, Pirqss_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Arbitration Lost   AL. Device  master mode  lost the control on the I2C bus due to losing        arbitration procedure. Accordingly  bit field BS in register BUSSTAT is set to 01 ."]
    #[inline(always)]
    pub fn al(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, pirqss::Al, Pirqss_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1,1,0,pirqss::Al, Pirqss_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Not acknowledge Received   NACK. When working as transmitter this interrupt indicates a not acknowledge        from the remote receiver. The SW has to decide what further steps have        to be taken."]
    #[inline(always)]
    pub fn nack(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, pirqss::Nack, Pirqss_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,pirqss::Nack, Pirqss_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmission End   TX END. The device has ended the data transfer properly  after stop condition        has been put on the bus or the MASTER RESTART state has been entered."]
    #[inline(always)]
    pub fn tx_end(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, pirqss::TxEnd, Pirqss_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x1,1,0,pirqss::TxEnd, Pirqss_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive Mode   RX. I2C kernel indicates switching from transmitting data to receiving data."]
    #[inline(always)]
    pub fn rx(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, pirqss::Rx, Pirqss_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x1,1,0,pirqss::Rx, Pirqss_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pirqss {
    #[inline(always)]
    fn default() -> Pirqss {
        <crate::RegValueT<Pirqss_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pirqss {
    pub struct Am_SPEC;
    pub type Am = crate::EnumBitfieldStruct<u8, Am_SPEC>;
    impl Am {
        #[doc = "0 No interrupt request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Gc_SPEC;
    pub type Gc = crate::EnumBitfieldStruct<u8, Gc_SPEC>;
    impl Gc {
        #[doc = "0 No interrupt request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mc_SPEC;
    pub type Mc = crate::EnumBitfieldStruct<u8, Mc_SPEC>;
    impl Mc {
        #[doc = "0 No interrupt request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Al_SPEC;
    pub type Al = crate::EnumBitfieldStruct<u8, Al_SPEC>;
    impl Al {
        #[doc = "0 No interrupt request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Nack_SPEC;
    pub type Nack = crate::EnumBitfieldStruct<u8, Nack_SPEC>;
    impl Nack {
        #[doc = "0 No interrupt request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TxEnd_SPEC;
    pub type TxEnd = crate::EnumBitfieldStruct<u8, TxEnd_SPEC>;
    impl TxEnd {
        #[doc = "0 No interrupt request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rx_SPEC;
    pub type Rx = crate::EnumBitfieldStruct<u8, Rx_SPEC>;
    impl Rx {
        #[doc = "0 No interrupt request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ris_SPEC;
impl crate::sealed::RegSpec for Ris_SPEC {
    type DataType = u32;
}
#[doc = "Raw Interrupt Status Register\n resetvalue={Application Reset:0x0}"]
pub type Ris = crate::RegValueT<Ris_SPEC>;

impl Ris {
    #[doc = "Last Single Request Interrupt   LSREQ INT"]
    #[inline(always)]
    pub fn lsreq_int(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ris::LsreqInt, Ris_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,ris::LsreqInt, Ris_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Single Request Interrupt   SREQ INT"]
    #[inline(always)]
    pub fn sreq_int(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ris::SreqInt, Ris_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,ris::SreqInt, Ris_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Last Burst Request Interrupt   LBREQ INT"]
    #[inline(always)]
    pub fn lbreq_int(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ris::LbreqInt, Ris_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,ris::LbreqInt, Ris_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Burst Request Interrupt   BREQ INT"]
    #[inline(always)]
    pub fn breq_int(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ris::BreqInt, Ris_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1,1,0,ris::BreqInt, Ris_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "I2C Error Interrupt   I2C ERR INT. This is the combined bit for indication of FIFO errors due to overflow        and underflow."]
    #[inline(always)]
    pub fn i2c_err_int(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ris::I2CErrInt, Ris_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,ris::I2CErrInt, Ris_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "I2C Protocol Interrupt   I2C P INT. This is the combined bit for indication of a protocol event in the I2C        kernel."]
    #[inline(always)]
    pub fn i2c_p_int(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ris::I2CPInt, Ris_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x1,1,0,ris::I2CPInt, Ris_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ris {
    #[inline(always)]
    fn default() -> Ris {
        <crate::RegValueT<Ris_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ris {
    pub struct LsreqInt_SPEC;
    pub type LsreqInt = crate::EnumBitfieldStruct<u8, LsreqInt_SPEC>;
    impl LsreqInt {
        #[doc = "0 No interrupt request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct SreqInt_SPEC;
    pub type SreqInt = crate::EnumBitfieldStruct<u8, SreqInt_SPEC>;
    impl SreqInt {
        #[doc = "0 No interrupt request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct LbreqInt_SPEC;
    pub type LbreqInt = crate::EnumBitfieldStruct<u8, LbreqInt_SPEC>;
    impl LbreqInt {
        #[doc = "0 No interrupt request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct BreqInt_SPEC;
    pub type BreqInt = crate::EnumBitfieldStruct<u8, BreqInt_SPEC>;
    impl BreqInt {
        #[doc = "0 No interrupt request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct I2CErrInt_SPEC;
    pub type I2CErrInt = crate::EnumBitfieldStruct<u8, I2CErrInt_SPEC>;
    impl I2CErrInt {
        #[doc = "0 No interrupt request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct I2CPInt_SPEC;
    pub type I2CPInt = crate::EnumBitfieldStruct<u8, I2CPInt_SPEC>;
    impl I2CPInt {
        #[doc = "0 No interrupt request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt request pending"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rpsstat_SPEC;
impl crate::sealed::RegSpec for Rpsstat_SPEC {
    type DataType = u32;
}
#[doc = "Received Packet Size Status Register\n resetvalue={Application Reset:0x0}"]
pub type Rpsstat = crate::RegValueT<Rpsstat_SPEC>;

impl Rpsstat {
    #[doc = "Received Packet Size   RPS. Length in characters of the received packet  0 to 16383"]
    #[inline(always)]
    pub fn rps(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Rpsstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Rpsstat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Rpsstat {
    #[inline(always)]
    fn default() -> Rpsstat {
        <crate::RegValueT<Rpsstat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Runctrl_SPEC;
impl crate::sealed::RegSpec for Runctrl_SPEC {
    type DataType = u32;
}
#[doc = "RUN Control Register\n resetvalue={Application Reset:0x0}"]
pub type Runctrl = crate::RegValueT<Runctrl_SPEC>;

impl Runctrl {
    #[doc = "Enable I2C bus Interface   RUN"]
    #[inline(always)]
    pub fn run(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, runctrl::Run, Runctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,runctrl::Run, Runctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Runctrl {
    #[inline(always)]
    fn default() -> Runctrl {
        <crate::RegValueT<Runctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod runctrl {
    pub struct Run_SPEC;
    pub type Run = crate::EnumBitfieldStruct<u8, Run_SPEC>;
    impl Run {
        #[doc = "0 I2C bus        interface disabled  write access to configuration registers enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Participation        in I2C bus communication enabled  if properly configured   write access        to configuration registers disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxd_SPEC;
impl crate::sealed::RegSpec for Rxd_SPEC {
    type DataType = u32;
}
#[doc = "Reception Data Register\n resetvalue={Application Reset:0x0}"]
pub type Rxd = crate::RegValueT<Rxd_SPEC>;

impl Rxd {
    #[doc = "Reception Data   RXD. Received characters"]
    #[inline(always)]
    pub fn rxd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Rxd_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Rxd_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Rxd {
    #[inline(always)]
    fn default() -> Rxd {
        <crate::RegValueT<Rxd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timcfg_SPEC;
impl crate::sealed::RegSpec for Timcfg_SPEC {
    type DataType = u32;
}
#[doc = "Timing Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Timcfg = crate::RegValueT<Timcfg_SPEC>;

impl Timcfg {
    #[doc = "SDA Delay Stages for Data Hold Time in Standard and Fast modes   SDA DEL HD DAT. SDA delay stages for data hold time in standard and fast modes. SDA delay from SCL falling edge but will also affect SDA Setup time          relative to next SCL rising edge"]
    #[inline(always)]
    pub fn sda_del_hd_dat(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Timcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Timcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SDA Delay Stages for Data Hold Time in High speed Mode   HS SDA DEL HD DAT. SDA delay stages for data hold time in HS mode. SDA delay from SCL falling edge but will also affect SDA Setup time          relative to next SCL rising edge"]
    #[inline(always)]
    pub fn hs_sda_del_hd_dat(
        self,
    ) -> crate::common::RegisterField<6, 0x7, 1, 0, u8, Timcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x7,1,0,u8, Timcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SCL Delay Stages for Hold Time Start  Restart  Bit   SCL DEL HD STA"]
    #[inline(always)]
    pub fn scl_del_hd_sta(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, Timcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7,1,0,u8, Timcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Direct Configuration of SCL Low Period Length in Fast Mode   EN SCL LOW LEN"]
    #[inline(always)]
    pub fn en_scl_low_len(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        timcfg::EnSclLowLen,
        Timcfg_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            timcfg::EnSclLowLen,
            Timcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Set Fast Mode SCL Low Period Timing   FS SCL LOW. The internal duration of the SCL low time with respect to the period        length as defined by the baudrate setting  can be enlarged for the Fast        Speed Mode  in order to meet the asymmetric duty cycle requirements from        the standard. The detailed formulas are given in the functional specification."]
    #[inline(always)]
    pub fn fs_scl_low(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, timcfg::FsSclLow, Timcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            timcfg::FsSclLow,
            Timcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "SDA Delay Stages for Start Stop bit in High speed Mode   HS SDA DEL"]
    #[inline(always)]
    pub fn hs_sda_del(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Timcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Timcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SCL Low Length in Fast Mode   SCL LOW LEN. If enabled by EN SCL LOW LEN setting  this field determines the        extension of the SCL low time. In case of INC  160    160 1  the low time is        extended by the number of kernel clk cycles. In general  there is a more        complex formula  as given in the functional specification. The total period time is not changed  i.e.  the SCL high period is        reduced accordingly. Setting SCL low time to period length or higher is        not supported and would lead to unpredictable results. 00"]
    #[inline(always)]
    pub fn scl_low_len(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Timcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Timcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Timcfg {
    #[inline(always)]
    fn default() -> Timcfg {
        <crate::RegValueT<Timcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod timcfg {
    pub struct EnSclLowLen_SPEC;
    pub type EnSclLowLen = crate::EnumBitfieldStruct<u8, EnSclLowLen_SPEC>;
    impl EnSclLowLen {
        #[doc = "0 SCL low period is a fixed part of the whole period  as defined by FS SCL LOW"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SCL low period is determined by the setting of SCL LOW LEN"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct FsSclLow_SPEC;
    pub type FsSclLow = crate::EnumBitfieldStruct<u8, FsSclLow_SPEC>;
    impl FsSclLow {
        #[doc = "0 Standard mode        SCL low period timing. For INC   1 it is 5 8 of period."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Fast mode SCL low period timing. For INC   1 it is 6 8 of period."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tpsctrl_SPEC;
impl crate::sealed::RegSpec for Tpsctrl_SPEC {
    type DataType = u32;
}
#[doc = "Transmit Packet Size Control Register\n resetvalue={Application Reset:0x0}"]
pub type Tpsctrl = crate::RegValueT<Tpsctrl_SPEC>;

impl Tpsctrl {
    #[doc = "Transmit Packet Size   TPS. Length in characters of the transmit packet  write value range  1 to        16383 Reading returns the written value as long as it is not loaded to an        internal counter. After that  reading returns 0 and a new value can be        written."]
    #[inline(always)]
    pub fn tps(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Tpsctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Tpsctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Tpsctrl {
    #[inline(always)]
    fn default() -> Tpsctrl {
        <crate::RegValueT<Tpsctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txd_SPEC;
impl crate::sealed::RegSpec for Txd_SPEC {
    type DataType = u32;
}
#[doc = "Transmission Data Register\n resetvalue={Application Reset:0x0}"]
pub type Txd = crate::RegValueT<Txd_SPEC>;

impl Txd {
    #[doc = "Transmission Data   TXD. Characters to be transmitted"]
    #[inline(always)]
    pub fn txd(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Txd_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Txd_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Txd {
    #[inline(always)]
    fn default() -> Txd {
        <crate::RegValueT<Txd_SPEC> as RegisterValue<_>>::new(0)
    }
}
