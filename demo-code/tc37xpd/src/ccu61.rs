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
#[doc = r"CCU6"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccu61(pub(super) *mut u8);
unsafe impl core::marker::Send for Ccu61 {}
unsafe impl core::marker::Sync for Ccu61 {}
impl Ccu61 {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(252usize)) }
    }

    #[doc = "Compare Register for T13\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cc63r(&self) -> crate::common::Reg<self::Cc63R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(88usize)) }
    }

    #[doc = "Compare Shadow Register for T13\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cc63sr(&self) -> crate::common::Reg<self::Cc63Sr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(92usize)) }
    }

    #[doc = "Capture Compare Register for Channel CC60\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cc6xr(&self) -> [crate::common::Reg<self::Cc6XR_SPEC, crate::common::R>; 3] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x8usize)),
            ]
        }
    }

    #[doc = "Capture Compare Shadow Reg. for Channel CC60\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cc6xsr(&self) -> [crate::common::Reg<self::Cc6XSr_SPEC, crate::common::RW>; 3] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x40usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x40usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x40usize + 0x8usize)),
            ]
        }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Compare State Modification Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cmpmodif(&self) -> crate::common::Reg<self::Cmpmodif_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(100usize)) }
    }

    #[doc = "Compare State Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cmpstat(&self) -> crate::common::Reg<self::Cmpstat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x5400,Application Reset:0x5409}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Interrupt Enable Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ien(&self) -> crate::common::Reg<self::Ien_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(176usize)) }
    }

    #[doc = "Input Monitoring Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn imon(&self) -> crate::common::Reg<self::Imon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(152usize)) }
    }

    #[doc = "Interrupt Node Pointer Register\n resetvalue={Application Reset:0x3940}"]
    #[inline(always)]
    pub const fn inp(&self) -> crate::common::Reg<self::Inp_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(172usize)) }
    }

    #[doc = "Interrupt Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn is(&self) -> crate::common::Reg<self::Is_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }

    #[doc = "Interrupt Status Reset Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn isr(&self) -> crate::common::Reg<self::Isr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(168usize)) }
    }

    #[doc = "Interrupt Status Set Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn iss(&self) -> crate::common::Reg<self::Iss_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(164usize)) }
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

    #[doc = "Kernel State Control Sensitivity Register\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn kscsr(&self) -> crate::common::Reg<self::Kscsr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "Lost Indicator Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn li(&self) -> crate::common::Reg<self::Li_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(156usize)) }
    }

    #[doc = "Module Configuration Register\n resetvalue={Application Reset:0x08007,Application Reset:0x7}"]
    #[inline(always)]
    pub const fn mcfg(&self) -> crate::common::Reg<self::Mcfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }

    #[doc = "Multi Channel Mode Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mcmctr(&self) -> crate::common::Reg<self::Mcmctr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(148usize)) }
    }

    #[doc = "Multi Channel Mode Output Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mcmout(&self) -> crate::common::Reg<self::Mcmout_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }

    #[doc = "Multi Channel Mode Output Shadow Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mcmouts(&self) -> crate::common::Reg<self::Mcmouts_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(140usize)) }
    }

    #[doc = "Modulation Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn modctr(&self) -> crate::common::Reg<self::Modctr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "OCDS Control and Status Register\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(232usize)) }
    }

    #[doc = "Port Input Select Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pisel0(&self) -> crate::common::Reg<self::Pisel0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "Port Input Select Register 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pisel2(&self) -> crate::common::Reg<self::Pisel2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Passive State Level Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pslr(&self) -> crate::common::Reg<self::Pslr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }

    #[doc = "Timer T12 Counter Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t12(&self) -> crate::common::Reg<self::T12_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "Dead Time Control Register for Timer12\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t12dtc(&self) -> crate::common::Reg<self::T12Dtc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }

    #[doc = "T12 Mode Select Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t12msel(&self) -> crate::common::Reg<self::T12Msel_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(104usize)) }
    }

    #[doc = "Timer 12 Period Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t12pr(&self) -> crate::common::Reg<self::T12Pr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "Timer T13 Counter Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t13(&self) -> crate::common::Reg<self::T13_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }

    #[doc = "Timer 13 Period Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t13pr(&self) -> crate::common::Reg<self::T13Pr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(84usize)) }
    }

    #[doc = "Timer Control Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tctr0(&self) -> crate::common::Reg<self::Tctr0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(112usize)) }
    }

    #[doc = "Timer Control Register 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tctr2(&self) -> crate::common::Reg<self::Tctr2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(116usize)) }
    }

    #[doc = "Timer Control Register 4\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tctr4(&self) -> crate::common::Reg<self::Tctr4_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(120usize)) }
    }

    #[doc = "Trap Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn trpctr(&self) -> crate::common::Reg<self::Trpctr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
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
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En1_SPEC;
    pub type En1 = crate::EnumBitfieldStruct<u8, En1_SPEC>;
    impl En1 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En2_SPEC;
    pub type En2 = crate::EnumBitfieldStruct<u8, En2_SPEC>;
    impl En2 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En3_SPEC;
    pub type En3 = crate::EnumBitfieldStruct<u8, En3_SPEC>;
    impl En3 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En4_SPEC;
    pub type En4 = crate::EnumBitfieldStruct<u8, En4_SPEC>;
    impl En4 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En5_SPEC;
    pub type En5 = crate::EnumBitfieldStruct<u8, En5_SPEC>;
    impl En5 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En6_SPEC;
    pub type En6 = crate::EnumBitfieldStruct<u8, En6_SPEC>;
    impl En6 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En7_SPEC;
    pub type En7 = crate::EnumBitfieldStruct<u8, En7_SPEC>;
    impl En7 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En8_SPEC;
    pub type En8 = crate::EnumBitfieldStruct<u8, En8_SPEC>;
    impl En8 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En9_SPEC;
    pub type En9 = crate::EnumBitfieldStruct<u8, En9_SPEC>;
    impl En9 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En10_SPEC;
    pub type En10 = crate::EnumBitfieldStruct<u8, En10_SPEC>;
    impl En10 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En11_SPEC;
    pub type En11 = crate::EnumBitfieldStruct<u8, En11_SPEC>;
    impl En11 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En12_SPEC;
    pub type En12 = crate::EnumBitfieldStruct<u8, En12_SPEC>;
    impl En12 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En13_SPEC;
    pub type En13 = crate::EnumBitfieldStruct<u8, En13_SPEC>;
    impl En13 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En14_SPEC;
    pub type En14 = crate::EnumBitfieldStruct<u8, En14_SPEC>;
    impl En14 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En15_SPEC;
    pub type En15 = crate::EnumBitfieldStruct<u8, En15_SPEC>;
    impl En15 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En16_SPEC;
    pub type En16 = crate::EnumBitfieldStruct<u8, En16_SPEC>;
    impl En16 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En17_SPEC;
    pub type En17 = crate::EnumBitfieldStruct<u8, En17_SPEC>;
    impl En17 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En18_SPEC;
    pub type En18 = crate::EnumBitfieldStruct<u8, En18_SPEC>;
    impl En18 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En19_SPEC;
    pub type En19 = crate::EnumBitfieldStruct<u8, En19_SPEC>;
    impl En19 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En20_SPEC;
    pub type En20 = crate::EnumBitfieldStruct<u8, En20_SPEC>;
    impl En20 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En21_SPEC;
    pub type En21 = crate::EnumBitfieldStruct<u8, En21_SPEC>;
    impl En21 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En22_SPEC;
    pub type En22 = crate::EnumBitfieldStruct<u8, En22_SPEC>;
    impl En22 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En23_SPEC;
    pub type En23 = crate::EnumBitfieldStruct<u8, En23_SPEC>;
    impl En23 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En24_SPEC;
    pub type En24 = crate::EnumBitfieldStruct<u8, En24_SPEC>;
    impl En24 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En25_SPEC;
    pub type En25 = crate::EnumBitfieldStruct<u8, En25_SPEC>;
    impl En25 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En26_SPEC;
    pub type En26 = crate::EnumBitfieldStruct<u8, En26_SPEC>;
    impl En26 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En27_SPEC;
    pub type En27 = crate::EnumBitfieldStruct<u8, En27_SPEC>;
    impl En27 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En28_SPEC;
    pub type En28 = crate::EnumBitfieldStruct<u8, En28_SPEC>;
    impl En28 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En29_SPEC;
    pub type En29 = crate::EnumBitfieldStruct<u8, En29_SPEC>;
    impl En29 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En30_SPEC;
    pub type En30 = crate::EnumBitfieldStruct<u8, En30_SPEC>;
    impl En30 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En31_SPEC;
    pub type En31 = crate::EnumBitfieldStruct<u8, En31_SPEC>;
    impl En31 {
        #[doc = "No write access. 0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cc63R_SPEC;
impl crate::sealed::RegSpec for Cc63R_SPEC {
    type DataType = u32;
}
#[doc = "Compare Register for T13\n resetvalue={Application Reset:0x0}"]
pub type Cc63R = crate::RegValueT<Cc63R_SPEC>;

impl Cc63R {
    #[doc = "Channel CC63 Compare Value   CCV. The bit field CCV contains the value  that is compared to the T13 counter value."]
    #[inline(always)]
    pub fn ccv(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Cc63R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Cc63R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Cc63R {
    #[inline(always)]
    fn default() -> Cc63R {
        <crate::RegValueT<Cc63R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cc63Sr_SPEC;
impl crate::sealed::RegSpec for Cc63Sr_SPEC {
    type DataType = u32;
}
#[doc = "Compare Shadow Register for T13\n resetvalue={Application Reset:0x0}"]
pub type Cc63Sr = crate::RegValueT<Cc63Sr_SPEC>;

impl Cc63Sr {
    #[doc = "Shadow Register for Channel CC63 Compare Value   CCS. The bit field contents of CCS is transferred to the bit field CCV during a shadow transfer."]
    #[inline(always)]
    pub fn ccs(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Cc63Sr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Cc63Sr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Cc63Sr {
    #[inline(always)]
    fn default() -> Cc63Sr {
        <crate::RegValueT<Cc63Sr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cc6XR_SPEC;
impl crate::sealed::RegSpec for Cc6XR_SPEC {
    type DataType = u32;
}
#[doc = "Capture Compare Register for Channel CC60\n resetvalue={Application Reset:0x0}"]
pub type Cc6XR = crate::RegValueT<Cc6XR_SPEC>;

impl Cc6XR {
    #[doc = "Capture Compare Value   CCV. In compare mode  the bit fields CCV contain the values  that are compared to the T12 counter value. In capture mode  the captured value of T12 can be read from these registers."]
    #[inline(always)]
    pub fn ccv(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Cc6XR_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Cc6XR_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Cc6XR {
    #[inline(always)]
    fn default() -> Cc6XR {
        <crate::RegValueT<Cc6XR_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cc6XSr_SPEC;
impl crate::sealed::RegSpec for Cc6XSr_SPEC {
    type DataType = u32;
}
#[doc = "Capture Compare Shadow Reg. for Channel CC60\n resetvalue={Application Reset:0x0}"]
pub type Cc6XSr = crate::RegValueT<Cc6XSr_SPEC>;

impl Cc6XSr {
    #[doc = "Shadow Register for Channel x Capture Compare Value   CCS. In compare mode  the bit fields contents of CCS are transferred to the bit fields CCV for the corresponding channel during a shadow transfer. In capture mode  the captured value of T12 can be read from these registers."]
    #[inline(always)]
    pub fn ccs(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Cc6XSr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Cc6XSr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Cc6XSr {
    #[inline(always)]
    fn default() -> Cc6XSr {
        <crate::RegValueT<Cc6XSr_SPEC> as RegisterValue<_>>::new(0)
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
        #[doc = "0 Module disable is not requested."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module disable is requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Diss_SPEC;
    pub type Diss = crate::EnumBitfieldStruct<u8, Diss_SPEC>;
    impl Diss {
        #[doc = "0 Module is enabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module is disabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Edis_SPEC;
    pub type Edis = crate::EnumBitfieldStruct<u8, Edis_SPEC>;
    impl Edis {
        #[doc = "0 Sleep Mode        request is regarded. Module is enabled to go into Sleep Mode."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Sleep Mode        request is disregarded  Sleep Mode cannot be entered upon a request."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpmodif_SPEC;
impl crate::sealed::RegSpec for Cmpmodif_SPEC {
    type DataType = u32;
}
#[doc = "Compare State Modification Register\n resetvalue={Application Reset:0x0}"]
pub type Cmpmodif = crate::RegValueT<Cmpmodif_SPEC>;

impl Cmpmodif {
    #[doc = "Capture Compare Status Modification Bits MCC62S  x   0  1  2    MCC62S. These bits are used to set the corresponding bits CC6xST by SW. The functionality of a write access to bits concerning the same        capture compare state bit is shown in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn mcc60s(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Cmpmodif_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cmpmodif_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture Compare Status Modification Bits MCC62S  x   0  1  2    MCC62S. These bits are used to set the corresponding bits CC6xST by SW. The functionality of a write access to bits concerning the same        capture compare state bit is shown in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn mcc61s(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Cmpmodif_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cmpmodif_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture Compare Status Modification Bits MCC62S  x   0  1  2    MCC62S. These bits are used to set the corresponding bits CC6xST by SW. The functionality of a write access to bits concerning the same        capture compare state bit is shown in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn mcc62s(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Cmpmodif_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cmpmodif_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture Compare Status Modification Bit MCC63S   MCC63S. This bit is used to set the corresponding bit CC63ST by SW. The functionality of a write access to bits concerning the same capture compare state bit is shown in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn mcc63s(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Cmpmodif_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Cmpmodif_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture Compare Status Modification Bits MCC62R  x   0  1  2    MCC62R. These bits are used to clear the corresponding bits CC6xST by SW. The functionality of a write access to bits concerning the same        capture compare state bit is shown in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn mcc60r(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Cmpmodif_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Cmpmodif_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture Compare Status Modification Bits MCC62R  x   0  1  2    MCC62R. These bits are used to clear the corresponding bits CC6xST by SW. The functionality of a write access to bits concerning the same        capture compare state bit is shown in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn mcc61r(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Cmpmodif_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Cmpmodif_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture Compare Status Modification Bits MCC62R  x   0  1  2    MCC62R. These bits are used to clear the corresponding bits CC6xST by SW. The functionality of a write access to bits concerning the same        capture compare state bit is shown in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn mcc62r(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Cmpmodif_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Cmpmodif_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Capture Compare Status Modification Bits MCC63R   MCC63R. This bit is used to clear the corresponding bit CC63ST by SW. The functionality of a write access to bits concerning the same capture compare state bit is shown in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn mcc63r(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Cmpmodif_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, Cmpmodif_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Cmpmodif {
    #[inline(always)]
    fn default() -> Cmpmodif {
        <crate::RegValueT<Cmpmodif_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmpstat_SPEC;
impl crate::sealed::RegSpec for Cmpstat_SPEC {
    type DataType = u32;
}
#[doc = "Compare State Register\n resetvalue={Application Reset:0x0}"]
pub type Cmpstat = crate::RegValueT<Cmpstat_SPEC>;

impl Cmpstat {
    #[doc = "Capture Compare State Bits for CC62  x   0  1  2    CC62ST. Bits CC6xST monitor the state of the capture compare channels. Bits        CC6xST  x  160    160 0  1  2  are related to T12 and are set and cleared        according to the T12 switching rules."]
    #[inline(always)]
    pub fn cc60st(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, cmpstat::Cc60St, Cmpstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,cmpstat::Cc60St, Cmpstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture Compare State Bits for CC62  x   0  1  2    CC62ST. Bits CC6xST monitor the state of the capture compare channels. Bits        CC6xST  x  160    160 0  1  2  are related to T12 and are set and cleared        according to the T12 switching rules."]
    #[inline(always)]
    pub fn cc61st(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, cmpstat::Cc61St, Cmpstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,cmpstat::Cc61St, Cmpstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture Compare State Bits for CC62  x   0  1  2    CC62ST. Bits CC6xST monitor the state of the capture compare channels. Bits        CC6xST  x  160    160 0  1  2  are related to T12 and are set and cleared        according to the T12 switching rules."]
    #[inline(always)]
    pub fn cc62st(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, cmpstat::Cc62St, Cmpstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1,1,0,cmpstat::Cc62St, Cmpstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Sampled Hall Pattern Bits   CCPOS62. Bits CCPOS6x  x   0  1  2  are indicating the value of the input Hall pattern that has been compared to the current and expected value. The value is sampled when the event HCRDY  Hall Compare Ready  occurs."]
    #[inline(always)]
    pub fn ccpos60(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, cmpstat::Ccpos60, Cmpstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x1,1,0,cmpstat::Ccpos60, Cmpstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Sampled Hall Pattern Bits   CCPOS62. Bits CCPOS6x  x   0  1  2  are indicating the value of the input Hall pattern that has been compared to the current and expected value. The value is sampled when the event HCRDY  Hall Compare Ready  occurs."]
    #[inline(always)]
    pub fn ccpos61(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, cmpstat::Ccpos61, Cmpstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,cmpstat::Ccpos61, Cmpstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Sampled Hall Pattern Bits   CCPOS62. Bits CCPOS6x  x   0  1  2  are indicating the value of the input Hall pattern that has been compared to the current and expected value. The value is sampled when the event HCRDY  Hall Compare Ready  occurs."]
    #[inline(always)]
    pub fn ccpos62(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, cmpstat::Ccpos62, Cmpstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x1,1,0,cmpstat::Ccpos62, Cmpstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture Compare State Bit for CC63   CC63ST. Bit CC63ST monitors the state of the compare channel. Bit CC63ST is related to T13 and is set and cleared according to the T13 switching rules."]
    #[inline(always)]
    pub fn cc63st(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, cmpstat::Cc63St, Cmpstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x1,1,0,cmpstat::Cc63St, Cmpstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Passive State Select for Compare Outputs CC62  x   0  1  2    CC62PS. Bits CC6xPS select the state of the corresponding compare channel  that is considered to be the passive state. During the passive state  the passive level  defined in register PSLR  is driven by the output pin. Bits CC6xPS  x   0  1  2  are related to T12. In capture mode  these bits are not used."]
    #[inline(always)]
    pub fn cc60ps(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, cmpstat::Cc60Ps, Cmpstat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,cmpstat::Cc60Ps, Cmpstat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Passive State Select for Compare Outputs CC62  x   0  1  2    CC62PS. Bits CC6xPS select the state of the corresponding compare channel  that is considered to be the passive state. During the passive state  the passive level  defined in register PSLR  is driven by the output pin. Bits CC6xPS  x   0  1  2  are related to T12. In capture mode  these bits are not used."]
    #[inline(always)]
    pub fn cc61ps(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, cmpstat::Cc61Ps, Cmpstat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            cmpstat::Cc61Ps,
            Cmpstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Passive State Select for Compare Outputs CC62  x   0  1  2    CC62PS. Bits CC6xPS select the state of the corresponding compare channel  that is considered to be the passive state. During the passive state  the passive level  defined in register PSLR  is driven by the output pin. Bits CC6xPS  x   0  1  2  are related to T12. In capture mode  these bits are not used."]
    #[inline(always)]
    pub fn cc62ps(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, cmpstat::Cc62Ps, Cmpstat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            cmpstat::Cc62Ps,
            Cmpstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Passive State Select for Compare Outputs COUT62  x   0  1  2    COUT62PS. Bits COUT6xPS select the state of the corresponding compare channel  that is considered to be the passive state. During the passive state  the passive level  defined in register PSLR  is driven by the output pin. Bits COUT6xPS  x   0  1  2  are related to T12. In capture mode  these bits are not used."]
    #[inline(always)]
    pub fn cout60ps(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        cmpstat::Cout60Ps,
        Cmpstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            cmpstat::Cout60Ps,
            Cmpstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Passive State Select for Compare Outputs COUT62  x   0  1  2    COUT62PS. Bits COUT6xPS select the state of the corresponding compare channel  that is considered to be the passive state. During the passive state  the passive level  defined in register PSLR  is driven by the output pin. Bits COUT6xPS  x   0  1  2  are related to T12. In capture mode  these bits are not used."]
    #[inline(always)]
    pub fn cout61ps(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        cmpstat::Cout61Ps,
        Cmpstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            cmpstat::Cout61Ps,
            Cmpstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Passive State Select for Compare Outputs COUT62  x   0  1  2    COUT62PS. Bits COUT6xPS select the state of the corresponding compare channel  that is considered to be the passive state. During the passive state  the passive level  defined in register PSLR  is driven by the output pin. Bits COUT6xPS  x   0  1  2  are related to T12. In capture mode  these bits are not used."]
    #[inline(always)]
    pub fn cout62ps(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        cmpstat::Cout62Ps,
        Cmpstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            cmpstat::Cout62Ps,
            Cmpstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Passive State Select for Compare Output COUT63   COUT63PS. Bit COUT63PS selects the state of the corresponding compare channel  that is considered to be the passive state. During the passive state  the passive level  defined in register PSLR  is driven by the output pin. Bit COUT63PS is related to T13."]
    #[inline(always)]
    pub fn cout63ps(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        cmpstat::Cout63Ps,
        Cmpstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            cmpstat::Cout63Ps,
            Cmpstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "T13 Inverted Modulation   T13IM. Bit T13IM inverts the T13 signal for the modulation of the CC6x and COUT6x  x   0  1  2  signals."]
    #[inline(always)]
    pub fn t13im(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, cmpstat::T13Im, Cmpstat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,cmpstat::T13Im, Cmpstat_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Cmpstat {
    #[inline(always)]
    fn default() -> Cmpstat {
        <crate::RegValueT<Cmpstat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cmpstat {
    pub struct Cc60St_SPEC;
    pub type Cc60St = crate::EnumBitfieldStruct<u8, Cc60St_SPEC>;
    impl Cc60St {
        #[doc = "0 In compare mode  the timer count is less than the compare value. In capture mode  the selected edge has not yet been detected since the bit has been cleared by SW the last time."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 In compare mode  the counter value is greater than or equal to the compare value. In capture mode  the selected edge has been detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cc61St_SPEC;
    pub type Cc61St = crate::EnumBitfieldStruct<u8, Cc61St_SPEC>;
    impl Cc61St {
        #[doc = "0 In compare mode  the timer count is less than the compare value. In capture mode  the selected edge has not yet been detected since the bit has been cleared by SW the last time."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 In compare mode  the counter value is greater than or equal to the compare value. In capture mode  the selected edge has been detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cc62St_SPEC;
    pub type Cc62St = crate::EnumBitfieldStruct<u8, Cc62St_SPEC>;
    impl Cc62St {
        #[doc = "0 In compare mode  the timer count is less than the compare value. In capture mode  the selected edge has not yet been detected since the bit has been cleared by SW the last time."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 In compare mode  the counter value is greater than or equal to the compare value. In capture mode  the selected edge has been detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ccpos60_SPEC;
    pub type Ccpos60 = crate::EnumBitfieldStruct<u8, Ccpos60_SPEC>;
    impl Ccpos60 {
        #[doc = "0 The input CCPOSx has been sampled as 0."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The input CCPOSx has been sampled as 1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ccpos61_SPEC;
    pub type Ccpos61 = crate::EnumBitfieldStruct<u8, Ccpos61_SPEC>;
    impl Ccpos61 {
        #[doc = "0 The input CCPOSx has been sampled as 0."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The input CCPOSx has been sampled as 1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ccpos62_SPEC;
    pub type Ccpos62 = crate::EnumBitfieldStruct<u8, Ccpos62_SPEC>;
    impl Ccpos62 {
        #[doc = "0 The input CCPOSx has been sampled as 0."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The input CCPOSx has been sampled as 1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cc63St_SPEC;
    pub type Cc63St = crate::EnumBitfieldStruct<u8, Cc63St_SPEC>;
    impl Cc63St {
        #[doc = "0 In compare        mode  the timer count is less than the compare value."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 In compare        mode  the counter value is greater than or equal to the compare value."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cc60Ps_SPEC;
    pub type Cc60Ps = crate::EnumBitfieldStruct<u8, Cc60Ps_SPEC>;
    impl Cc60Ps {
        #[doc = "0 The corresponding compare signal is in passive state while CC6xST is 0."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The corresponding compare signal is in passive state while CC6xST is 1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cc61Ps_SPEC;
    pub type Cc61Ps = crate::EnumBitfieldStruct<u8, Cc61Ps_SPEC>;
    impl Cc61Ps {
        #[doc = "0 The corresponding compare signal is in passive state while CC6xST is 0."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The corresponding compare signal is in passive state while CC6xST is 1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cc62Ps_SPEC;
    pub type Cc62Ps = crate::EnumBitfieldStruct<u8, Cc62Ps_SPEC>;
    impl Cc62Ps {
        #[doc = "0 The corresponding compare signal is in passive state while CC6xST is 0."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The corresponding compare signal is in passive state while CC6xST is 1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cout60Ps_SPEC;
    pub type Cout60Ps = crate::EnumBitfieldStruct<u8, Cout60Ps_SPEC>;
    impl Cout60Ps {
        #[doc = "0 The corresponding compare signal is in passive state while CC6xST is 0."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The corresponding compare signal is in passive state while CC6xST is 1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cout61Ps_SPEC;
    pub type Cout61Ps = crate::EnumBitfieldStruct<u8, Cout61Ps_SPEC>;
    impl Cout61Ps {
        #[doc = "0 The corresponding compare signal is in passive state while CC6xST is 0."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The corresponding compare signal is in passive state while CC6xST is 1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cout62Ps_SPEC;
    pub type Cout62Ps = crate::EnumBitfieldStruct<u8, Cout62Ps_SPEC>;
    impl Cout62Ps {
        #[doc = "0 The corresponding compare signal is in passive state while CC6xST is 0."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The corresponding compare signal is in passive state while CC6xST is 1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cout63Ps_SPEC;
    pub type Cout63Ps = crate::EnumBitfieldStruct<u8, Cout63Ps_SPEC>;
    impl Cout63Ps {
        #[doc = "0 The corresponding compare signal is in passive state while CC63ST is 0."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The corresponding compare signal is in passive state while CC63ST is 1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T13Im_SPEC;
    pub type T13Im = crate::EnumBitfieldStruct<u8, T13Im_SPEC>;
    impl T13Im {
        #[doc = "0 T13 output CC63 O is equal to CC63ST."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 T13 output CC63 O is equal to CC63ST ."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x5400,Application Reset:0x5409}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MODREV. MODREV defines the module revision number. The value of a module        revision starts with 01 H  first        revision   02 H          03 H    8230 up        to FF H ."]
    #[inline(always)]
    pub fn modrev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MODNUM. This bit field defines the module identification number for the CCU6  54 H"]
    #[inline(always)]
    pub fn modnum(
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(21504)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ien_SPEC;
impl crate::sealed::RegSpec for Ien_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Enable Register\n resetvalue={Application Reset:0x0}"]
pub type Ien = crate::RegValueT<Ien_SPEC>;

impl Ien {
    #[doc = "Capture  Compare Match Rising Edge Interrupt Enable for Channel CC6x ENCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn encc60r(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ien::Encc60R, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,ien::Encc60R, Ien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Capture  Compare Match Falling Edge Interrupt Enable for Channel CC6x ENCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn encc60f(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ien::Encc60F, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,ien::Encc60F, Ien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Capture  Compare Match Rising Edge Interrupt Enable for Channel CC6x ENCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn encc61r(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ien::Encc61R, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,ien::Encc61R, Ien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Capture  Compare Match Falling Edge Interrupt Enable for Channel CC6x ENCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn encc61f(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ien::Encc61F, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,ien::Encc61F, Ien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Capture  Compare Match Rising Edge Interrupt Enable for Channel CC6x ENCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn encc62r(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ien::Encc62R, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,ien::Encc62R, Ien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Capture  Compare Match Falling Edge Interrupt Enable for Channel CC6x ENCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn encc62f(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ien::Encc62F, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,ien::Encc62F, Ien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Interrupt for T12 One Match   ENT12OM"]
    #[inline(always)]
    pub fn ent12om(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, ien::Ent12Om, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,ien::Ent12Om, Ien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Interrupt for T12 Period Match   ENT12PM"]
    #[inline(always)]
    pub fn ent12pm(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, ien::Ent12Pm, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,ien::Ent12Pm, Ien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Interrupt for T13 Compare Match   ENT13CM"]
    #[inline(always)]
    pub fn ent13cm(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, ien::Ent13Cm, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,ien::Ent13Cm, Ien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Interrupt for T13 Period Match   ENT13PM"]
    #[inline(always)]
    pub fn ent13pm(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, ien::Ent13Pm, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,ien::Ent13Pm, Ien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Interrupt for Trap Flag   ENTRPF"]
    #[inline(always)]
    pub fn entrpf(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, ien::Entrpf, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x1,1,0,ien::Entrpf, Ien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Interrupt for Correct Hall Event   ENCHE"]
    #[inline(always)]
    pub fn enche(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, ien::Enche, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x1,1,0,ien::Enche, Ien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Interrupt for Wrong Hall Event   ENWHE"]
    #[inline(always)]
    pub fn enwhe(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, ien::Enwhe, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x1,1,0,ien::Enwhe, Ien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Idle   ENIDLE. This bit enables the automatic entering of the idle state  bit IDLE will be set  after a wrong hall event has been detected  bit WHE is set . During the idle state  the bit field MCMP is automatically cleared."]
    #[inline(always)]
    pub fn enidle(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, ien::Enidle, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x1,1,0,ien::Enidle, Ien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Multi Channel Mode Shadow Transfer Interrupt   ENSTR"]
    #[inline(always)]
    pub fn enstr(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, ien::Enstr, Ien_SPEC, crate::common::RW> {
        crate::common::RegisterField::<15,0x1,1,0,ien::Enstr, Ien_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ien {
    #[inline(always)]
    fn default() -> Ien {
        <crate::RegValueT<Ien_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ien {
    pub struct Encc60R_SPEC;
    pub type Encc60R = crate::EnumBitfieldStruct<u8, Encc60R_SPEC>;
    impl Encc60R {
        #[doc = "0 No interrupt        will be generated if the set condition for bit CC6xR in register IS        occurs."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An interrupt        will be generated if the set condition for bit CC6xR in register IS        occurs. The service request output that will be activated is selected by        bit field INPCC6x."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Encc60F_SPEC;
    pub type Encc60F = crate::EnumBitfieldStruct<u8, Encc60F_SPEC>;
    impl Encc60F {
        #[doc = "0 No interrupt        will be generated if the set condition for bit CC6xF in register IS        occurs."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An interrupt        will be generated if the set condition for bit CC6xF in register IS        occurs. The service request output that will be activated is selected by        bit field INPCC6x."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Encc61R_SPEC;
    pub type Encc61R = crate::EnumBitfieldStruct<u8, Encc61R_SPEC>;
    impl Encc61R {
        #[doc = "0 No interrupt        will be generated if the set condition for bit CC6xR in register IS        occurs."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An interrupt        will be generated if the set condition for bit CC6xR in register IS        occurs. The service request output that will be activated is selected by        bit field INPCC6x."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Encc61F_SPEC;
    pub type Encc61F = crate::EnumBitfieldStruct<u8, Encc61F_SPEC>;
    impl Encc61F {
        #[doc = "0 No interrupt        will be generated if the set condition for bit CC6xF in register IS        occurs."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An interrupt        will be generated if the set condition for bit CC6xF in register IS        occurs. The service request output that will be activated is selected by        bit field INPCC6x."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Encc62R_SPEC;
    pub type Encc62R = crate::EnumBitfieldStruct<u8, Encc62R_SPEC>;
    impl Encc62R {
        #[doc = "0 No interrupt        will be generated if the set condition for bit CC6xR in register IS        occurs."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An interrupt        will be generated if the set condition for bit CC6xR in register IS        occurs. The service request output that will be activated is selected by        bit field INPCC6x."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Encc62F_SPEC;
    pub type Encc62F = crate::EnumBitfieldStruct<u8, Encc62F_SPEC>;
    impl Encc62F {
        #[doc = "0 No interrupt        will be generated if the set condition for bit CC6xF in register IS        occurs."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An interrupt        will be generated if the set condition for bit CC6xF in register IS        occurs. The service request output that will be activated is selected by        bit field INPCC6x."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ent12Om_SPEC;
    pub type Ent12Om = crate::EnumBitfieldStruct<u8, Ent12Om_SPEC>;
    impl Ent12Om {
        #[doc = "0 No interrupt will be generated if the set condition for bit T12OM in register IS occurs."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An interrupt will be generated if the set condition for bit T12OM in register IS occurs. The service request output that will be activated is selected by bit field INPT12."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ent12Pm_SPEC;
    pub type Ent12Pm = crate::EnumBitfieldStruct<u8, Ent12Pm_SPEC>;
    impl Ent12Pm {
        #[doc = "0 No interrupt will be generated if the set condition for bit T12PM in register IS occurs."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An interrupt will be generated if the set condition for bit T12PM in register IS occurs. The service request output that will be activated is selected by bit field INPT12."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ent13Cm_SPEC;
    pub type Ent13Cm = crate::EnumBitfieldStruct<u8, Ent13Cm_SPEC>;
    impl Ent13Cm {
        #[doc = "0 No interrupt will be generated if the set condition for bit T13CM in register IS occurs."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An interrupt will be generated if the set condition for bit T13CM in register IS occurs. The service request output that will be activated is selected by bit field INPT13."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ent13Pm_SPEC;
    pub type Ent13Pm = crate::EnumBitfieldStruct<u8, Ent13Pm_SPEC>;
    impl Ent13Pm {
        #[doc = "0 No interrupt will be generated if the set condition for bit T13PM in register IS occurs."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An interrupt will be generated if the set condition for bit T13PM in register IS occurs. The service request output that will be activated is selected by bit field INPT13."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Entrpf_SPEC;
    pub type Entrpf = crate::EnumBitfieldStruct<u8, Entrpf_SPEC>;
    impl Entrpf {
        #[doc = "0 No interrupt will be generated if the set condition for bit TRPF in register IS occurs."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An interrupt will be generated if the set condition for bit TRPF in register IS occurs. The service request output that will be activated is selected by bit field INPERR."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enche_SPEC;
    pub type Enche = crate::EnumBitfieldStruct<u8, Enche_SPEC>;
    impl Enche {
        #[doc = "0 No interrupt will be generated if the set condition for bit CHE in register IS occurs."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An interrupt will be generated if the set condition for bit CHE in register IS occurs. The service request output that will be activated is selected by bit field INPCHE."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enwhe_SPEC;
    pub type Enwhe = crate::EnumBitfieldStruct<u8, Enwhe_SPEC>;
    impl Enwhe {
        #[doc = "0 No interrupt will be generated if the set condition for bit WHE in register IS occurs."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An interrupt will be generated if the set condition for bit WHE in register IS occurs. The service request output that will be activated is selected by bit field INPERR."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enidle_SPEC;
    pub type Enidle = crate::EnumBitfieldStruct<u8, Enidle_SPEC>;
    impl Enidle {
        #[doc = "0 The bit IDLE is not automatically set when a wrong hall event is detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit IDLE is automatically set when a wrong hall event is detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enstr_SPEC;
    pub type Enstr = crate::EnumBitfieldStruct<u8, Enstr_SPEC>;
    impl Enstr {
        #[doc = "0 No interrupt will be generated if the set condition for bit STR in register IS occurs."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An interrupt will be generated if the set condition for bit STR in register IS occurs. The service request output that will be activated is selected by bit field INPCHE."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imon_SPEC;
impl crate::sealed::RegSpec for Imon_SPEC {
    type DataType = u32;
}
#[doc = "Input Monitoring Register\n resetvalue={Application Reset:0x0}"]
pub type Imon = crate::RegValueT<Imon_SPEC>;

impl Imon {
    #[doc = "Lost Bit Event   LBE. This bit determines if a lost bit event has occurred. A lost bit event        occurs when a selected event occurs again with the previous event        captured  IMON.x remains set  and its lost indicator is enabled  for at        least one of the monitored input signals. The bit can be cleared by        writing a 1 to the same bit position  while writing a 0 has no effect."]
    #[inline(always)]
    pub fn lbe(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, imon::Lbe, Imon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,imon::Lbe, Imon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event indication for input signal CCPOS2   CCPOS2I. The bit determines if the selected event has occurred via an edge        detection. The bit can be cleared by writing a 1 to the same bit        position  while writing a 0 has no effect. The dedicated edge is indicated for a selected event if          Hysteretic like Control or Capture modes are initialized in          T12MSEL.MSEL6x. If these modes are not selected  then all edges will          be indicated as an event for the inputs."]
    #[inline(always)]
    pub fn ccpos0i(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, imon::Ccpos0I, Imon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,imon::Ccpos0I, Imon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event indication for input signal CCPOS2   CCPOS2I. The bit determines if the selected event has occurred via an edge        detection. The bit can be cleared by writing a 1 to the same bit        position  while writing a 0 has no effect. The dedicated edge is indicated for a selected event if          Hysteretic like Control or Capture modes are initialized in          T12MSEL.MSEL6x. If these modes are not selected  then all edges will          be indicated as an event for the inputs."]
    #[inline(always)]
    pub fn ccpos1i(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, imon::Ccpos1I, Imon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,imon::Ccpos1I, Imon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event indication for input signal CCPOS2   CCPOS2I. The bit determines if the selected event has occurred via an edge        detection. The bit can be cleared by writing a 1 to the same bit        position  while writing a 0 has no effect. The dedicated edge is indicated for a selected event if          Hysteretic like Control or Capture modes are initialized in          T12MSEL.MSEL6x. If these modes are not selected  then all edges will          be indicated as an event for the inputs."]
    #[inline(always)]
    pub fn ccpos2i(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, imon::Ccpos2I, Imon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,imon::Ccpos2I, Imon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event indication for input signal CC62IN   CC62INI. The bit determines if the selected event has occurred via an edge        detection. The bit can be cleared by writing a 1 to the same bit        position  while writing a 0 has no effect."]
    #[inline(always)]
    pub fn cc60ini(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, imon::Cc60Ini, Imon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,imon::Cc60Ini, Imon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event indication for input signal CC62IN   CC62INI. The bit determines if the selected event has occurred via an edge        detection. The bit can be cleared by writing a 1 to the same bit        position  while writing a 0 has no effect."]
    #[inline(always)]
    pub fn cc61ini(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, imon::Cc61Ini, Imon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,imon::Cc61Ini, Imon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event indication for input signal CC62IN   CC62INI. The bit determines if the selected event has occurred via an edge        detection. The bit can be cleared by writing a 1 to the same bit        position  while writing a 0 has no effect."]
    #[inline(always)]
    pub fn cc62ini(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, imon::Cc62Ini, Imon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,imon::Cc62Ini, Imon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event indication for input signal CTRAP   CTRAPI. The bit determines if the selected event has occurred via an edge        detection. The bit can be cleared by writing a 1 to the same bit        position  while writing a 0 has no effect."]
    #[inline(always)]
    pub fn ctrapi(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, imon::Ctrapi, Imon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,imon::Ctrapi, Imon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event indication for input signal T12HR   T12HRI. The bit determines if the selected event has occurred via an edge        detection. The bit can be cleared by writing a 1 to the same bit        position  while writing a 0 has no effect."]
    #[inline(always)]
    pub fn t12hri(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, imon::T12Hri, Imon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,imon::T12Hri, Imon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event indication for input signal T13HR   T13HRI. The bit determines if the selected event has occurred via an edge        detection. The bit can be cleared by writing a 1 to the same bit        position  while writing a 0 has no effect."]
    #[inline(always)]
    pub fn t13hri(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, imon::T13Hri, Imon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,imon::T13Hri, Imon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Imon {
    #[inline(always)]
    fn default() -> Imon {
        <crate::RegValueT<Imon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod imon {
    pub struct Lbe_SPEC;
    pub type Lbe = crate::EnumBitfieldStruct<u8, Lbe_SPEC>;
    impl Lbe {
        #[doc = "0 The lost bit        event has not occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The lost bit        event has occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ccpos0I_SPEC;
    pub type Ccpos0I = crate::EnumBitfieldStruct<u8, Ccpos0I_SPEC>;
    impl Ccpos0I {
        #[doc = "0 A selected event has not occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Edge detection indicates a selected event has occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ccpos1I_SPEC;
    pub type Ccpos1I = crate::EnumBitfieldStruct<u8, Ccpos1I_SPEC>;
    impl Ccpos1I {
        #[doc = "0 A selected event has not occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Edge detection indicates a selected event has occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ccpos2I_SPEC;
    pub type Ccpos2I = crate::EnumBitfieldStruct<u8, Ccpos2I_SPEC>;
    impl Ccpos2I {
        #[doc = "0 A selected event has not occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Edge detection indicates a selected event has occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cc60Ini_SPEC;
    pub type Cc60Ini = crate::EnumBitfieldStruct<u8, Cc60Ini_SPEC>;
    impl Cc60Ini {
        #[doc = "0 A selected event has not occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Edge detection indicates a selected event has occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cc61Ini_SPEC;
    pub type Cc61Ini = crate::EnumBitfieldStruct<u8, Cc61Ini_SPEC>;
    impl Cc61Ini {
        #[doc = "0 A selected event has not occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Edge detection indicates a selected event has occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cc62Ini_SPEC;
    pub type Cc62Ini = crate::EnumBitfieldStruct<u8, Cc62Ini_SPEC>;
    impl Cc62Ini {
        #[doc = "0 A selected event has not occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Edge detection indicates a selected event has occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ctrapi_SPEC;
    pub type Ctrapi = crate::EnumBitfieldStruct<u8, Ctrapi_SPEC>;
    impl Ctrapi {
        #[doc = "0 An event has not occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Edge detection indicates an event has occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T12Hri_SPEC;
    pub type T12Hri = crate::EnumBitfieldStruct<u8, T12Hri_SPEC>;
    impl T12Hri {
        #[doc = "0 An event has not occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Edge detection indicates an event has occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T13Hri_SPEC;
    pub type T13Hri = crate::EnumBitfieldStruct<u8, T13Hri_SPEC>;
    impl T13Hri {
        #[doc = "0 An event has        not occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Edge detection indicates an event has occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inp_SPEC;
impl crate::sealed::RegSpec for Inp_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Node Pointer Register\n resetvalue={Application Reset:0x3940}"]
pub type Inp = crate::RegValueT<Inp_SPEC>;

impl Inp {
    #[doc = "Interrupt Node Pointer for Channel CC6x Interrupts  INPCC6x  x 0 1 2 . This bit field defines the service request output activated due to a set        condition for bit CC6xR  if enabled by bit ENCC6xR  or for bit CC6xF  if        enabled by bit ENCC6xF ."]
    #[inline(always)]
    pub fn inpcc60(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, inp::Inpcc60, Inp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,inp::Inpcc60, Inp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for Channel CC6x Interrupts  INPCC6x  x 0 1 2 . This bit field defines the service request output activated due to a set        condition for bit CC6xR  if enabled by bit ENCC6xR  or for bit CC6xF  if        enabled by bit ENCC6xF ."]
    #[inline(always)]
    pub fn inpcc61(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, inp::Inpcc61, Inp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,inp::Inpcc61, Inp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for Channel CC6x Interrupts  INPCC6x  x 0 1 2 . This bit field defines the service request output activated due to a set        condition for bit CC6xR  if enabled by bit ENCC6xR  or for bit CC6xF  if        enabled by bit ENCC6xF ."]
    #[inline(always)]
    pub fn inpcc62(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, inp::Inpcc62, Inp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,inp::Inpcc62, Inp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for the CHE Interrupt   INPCHE. This bit field defines the service request output activated due to a set condition for bit CHE  if enabled by bit ENCHE  of for bit STR  if enabled by bit ENSTR . Coding see INPCC6x."]
    #[inline(always)]
    pub fn inpche(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Inp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6, 0x3, 1, 0, u8, Inp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Node Pointer for Error Interrupts   INPERR. This bit field defines the service request output activated due to a set        condition for bit TRPF  if enabled by bit ENTRPF  or for bit WHE  if        enabled by bit ENWHE . Coding see INPCC6x."]
    #[inline(always)]
    pub fn inperr(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Inp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8, 0x3, 1, 0, u8, Inp_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Node Pointer for Timer12 Interrupts   INPT12. This bit field defines the service request output activated due to a set condition for bit T12OM  if enabled by bit ENT12OM  or for bit T12PM  if enabled by bit ENT12PM . Coding see INPCC6x."]
    #[inline(always)]
    pub fn inpt12(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Inp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Inp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for Timer13 Interrupt   INPT13. This bit field defines the service request output activated due to a set condition for bit T13CM  if enabled by bit ENT13CM  or for bit T13PM  if enabled by bit ENT13PM . Coding see INPCC6x."]
    #[inline(always)]
    pub fn inpt13(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Inp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Inp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Inp {
    #[inline(always)]
    fn default() -> Inp {
        <crate::RegValueT<Inp_SPEC> as RegisterValue<_>>::new(14656)
    }
}
pub mod inp {
    pub struct Inpcc60_SPEC;
    pub type Inpcc60 = crate::EnumBitfieldStruct<u8, Inpcc60_SPEC>;
    impl Inpcc60 {
        #[doc = "00 Service request output SR0 is selected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Service request output SR1 is selected."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Service request output SR2 is selected."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Service request output SR3 is selected."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Inpcc61_SPEC;
    pub type Inpcc61 = crate::EnumBitfieldStruct<u8, Inpcc61_SPEC>;
    impl Inpcc61 {
        #[doc = "00 Service request output SR0 is selected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Service request output SR1 is selected."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Service request output SR2 is selected."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Service request output SR3 is selected."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Inpcc62_SPEC;
    pub type Inpcc62 = crate::EnumBitfieldStruct<u8, Inpcc62_SPEC>;
    impl Inpcc62 {
        #[doc = "00 Service request output SR0 is selected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Service request output SR1 is selected."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Service request output SR2 is selected."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Service request output SR3 is selected."]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Is_SPEC;
impl crate::sealed::RegSpec for Is_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Status Register\n resetvalue={Application Reset:0x0}"]
pub type Is = crate::RegValueT<Is_SPEC>;

impl Is {
    #[doc = "Capture  Compare Match Rising Edge Flag ICC6xR  x 0 1 2 . This bit indicates that event CC6x R has been detected. This event        occurs in compare mode when a compare match is detected while T12 is        counting up  CM 6x and CDIR  160    160 0  and in capture mode when a rising edge        is detected at the related input CC6xIN."]
    #[inline(always)]
    pub fn icc60r(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, is::Icc60R, Is_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,is::Icc60R, Is_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture  Compare Match Falling Edge Flag ICC6xF  x 0 1 2  . This bit indicates that event CC6x F has been detected. This event        occurs in compare mode when a compare match is detected while T12 is        counting down  CM 6x and CDIR  160    160 1  and in capture mode when a falling        edge is detected at the related input CC6xIN."]
    #[inline(always)]
    pub fn icc60f(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, is::Icc60F, Is_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,is::Icc60F, Is_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture  Compare Match Rising Edge Flag ICC6xR  x 0 1 2 . This bit indicates that event CC6x R has been detected. This event        occurs in compare mode when a compare match is detected while T12 is        counting up  CM 6x and CDIR  160    160 0  and in capture mode when a rising edge        is detected at the related input CC6xIN."]
    #[inline(always)]
    pub fn icc61r(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, is::Icc61R, Is_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,is::Icc61R, Is_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture  Compare Match Falling Edge Flag ICC6xF  x 0 1 2  . This bit indicates that event CC6x F has been detected. This event        occurs in compare mode when a compare match is detected while T12 is        counting down  CM 6x and CDIR  160    160 1  and in capture mode when a falling        edge is detected at the related input CC6xIN."]
    #[inline(always)]
    pub fn icc61f(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, is::Icc61F, Is_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1,1,0,is::Icc61F, Is_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture  Compare Match Rising Edge Flag ICC6xR  x 0 1 2 . This bit indicates that event CC6x R has been detected. This event        occurs in compare mode when a compare match is detected while T12 is        counting up  CM 6x and CDIR  160    160 0  and in capture mode when a rising edge        is detected at the related input CC6xIN."]
    #[inline(always)]
    pub fn icc62r(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, is::Icc62R, Is_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x1,1,0,is::Icc62R, Is_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture  Compare Match Falling Edge Flag ICC6xF  x 0 1 2  . This bit indicates that event CC6x F has been detected. This event        occurs in compare mode when a compare match is detected while T12 is        counting down  CM 6x and CDIR  160    160 1  and in capture mode when a falling        edge is detected at the related input CC6xIN."]
    #[inline(always)]
    pub fn icc62f(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, is::Icc62F, Is_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x1,1,0,is::Icc62F, Is_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Timer T12 One Match Flag   T12OM. This bit indicates that a timer T12 one match while counting down  T12 OM and CDIR   1  has been detected."]
    #[inline(always)]
    pub fn t12om(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, is::T12Om, Is_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x1,1,0,is::T12Om, Is_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Timer T12 Period Match Flag   T12PM. This bit indicates that a timer T12 period match while counting up  T12 PM and CDIR   0  has been detected."]
    #[inline(always)]
    pub fn t12pm(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, is::T12Pm, Is_SPEC, crate::common::R> {
        crate::common::RegisterField::<7,0x1,1,0,is::T12Pm, Is_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Timer T13 Compare Match Flag   T13CM. This bit indicates that a timer T13 compare match  CM 63  has been detected."]
    #[inline(always)]
    pub fn t13cm(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, is::T13Cm, Is_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x1,1,0,is::T13Cm, Is_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Timer T13 Period Match Flag   T13PM. This bit indicates that a timer T13 period match  T13 PM  has been detected."]
    #[inline(always)]
    pub fn t13pm(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, is::T13Pm, Is_SPEC, crate::common::R> {
        crate::common::RegisterField::<9,0x1,1,0,is::T13Pm, Is_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Trap Flag   TRPF. This bit indicates if a trap condition  input CTRAP   0 or by SW  is   has been detected. If TRPM2  0  it becomes cleared automatically if CTRAP   1 or TRPPEN   0  whereas if TRPM2   1  it has to be cleared by writing RTRPF   1."]
    #[inline(always)]
    pub fn trpf(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, is::Trpf, Is_SPEC, crate::common::R> {
        crate::common::RegisterField::<10,0x1,1,0,is::Trpf, Is_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Trap State   TRPS. This bit indicates the actual trap state. It is set if TRPF   1 and becomes cleared according to the mode selected in register TRPCTR."]
    #[inline(always)]
    pub fn trps(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, is::Trps, Is_SPEC, crate::common::R> {
        crate::common::RegisterField::<11,0x1,1,0,is::Trps, Is_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Correct Hall Event   CHE. This bit indicates that a correct Hall event  CM CHE  has been detected."]
    #[inline(always)]
    pub fn che(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, is::Che, Is_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x1,1,0,is::Che, Is_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Wrong Hall Event   WHE. This bit indicates that a wrong Hall event  CM WHE  has been detected."]
    #[inline(always)]
    pub fn whe(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, is::Whe, Is_SPEC, crate::common::R> {
        crate::common::RegisterField::<13,0x1,1,0,is::Whe, Is_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "IDLE State   IDLE. If enabled by ENIDLE   1  this bit is set together with bit WHE and it has to be cleared by SW."]
    #[inline(always)]
    pub fn idle(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, is::Idle, Is_SPEC, crate::common::R> {
        crate::common::RegisterField::<14,0x1,1,0,is::Idle, Is_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Multi Channel Mode Shadow Transfer Request   STR. This bit indicates that a shadow transfer from MCMPS to MCMP  MCM ST  has taken place."]
    #[inline(always)]
    pub fn str(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, is::Str, Is_SPEC, crate::common::R> {
        crate::common::RegisterField::<15,0x1,1,0,is::Str, Is_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Is {
    #[inline(always)]
    fn default() -> Is {
        <crate::RegValueT<Is_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod is {
    pub struct Icc60R_SPEC;
    pub type Icc60R = crate::EnumBitfieldStruct<u8, Icc60R_SPEC>;
    impl Icc60R {
        #[doc = "0 The event has not yet been detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The event has been detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Icc60F_SPEC;
    pub type Icc60F = crate::EnumBitfieldStruct<u8, Icc60F_SPEC>;
    impl Icc60F {
        #[doc = "0 The event has not yet been detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The event has been detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Icc61R_SPEC;
    pub type Icc61R = crate::EnumBitfieldStruct<u8, Icc61R_SPEC>;
    impl Icc61R {
        #[doc = "0 The event has not yet been detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The event has been detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Icc61F_SPEC;
    pub type Icc61F = crate::EnumBitfieldStruct<u8, Icc61F_SPEC>;
    impl Icc61F {
        #[doc = "0 The event has not yet been detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The event has been detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Icc62R_SPEC;
    pub type Icc62R = crate::EnumBitfieldStruct<u8, Icc62R_SPEC>;
    impl Icc62R {
        #[doc = "0 The event has not yet been detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The event has been detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Icc62F_SPEC;
    pub type Icc62F = crate::EnumBitfieldStruct<u8, Icc62F_SPEC>;
    impl Icc62F {
        #[doc = "0 The event has not yet been detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The event has been detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T12Om_SPEC;
    pub type T12Om = crate::EnumBitfieldStruct<u8, T12Om_SPEC>;
    impl T12Om {
        #[doc = "0 The event has not yet been detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The event has been detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T12Pm_SPEC;
    pub type T12Pm = crate::EnumBitfieldStruct<u8, T12Pm_SPEC>;
    impl T12Pm {
        #[doc = "0 The event has not yet been detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The event has been detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T13Cm_SPEC;
    pub type T13Cm = crate::EnumBitfieldStruct<u8, T13Cm_SPEC>;
    impl T13Cm {
        #[doc = "0 The event has not yet been detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The event has been detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T13Pm_SPEC;
    pub type T13Pm = crate::EnumBitfieldStruct<u8, T13Pm_SPEC>;
    impl T13Pm {
        #[doc = "0 The event has not yet been detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The event has been detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trpf_SPEC;
    pub type Trpf = crate::EnumBitfieldStruct<u8, Trpf_SPEC>;
    impl Trpf {
        #[doc = "0 The trap condition has not been detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The trap condition is   has been detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trps_SPEC;
    pub type Trps = crate::EnumBitfieldStruct<u8, Trps_SPEC>;
    impl Trps {
        #[doc = "0 The trap state is not active."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The trap state is active."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Che_SPEC;
    pub type Che = crate::EnumBitfieldStruct<u8, Che_SPEC>;
    impl Che {
        #[doc = "0 The event has not yet been detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The event has been detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Whe_SPEC;
    pub type Whe = crate::EnumBitfieldStruct<u8, Whe_SPEC>;
    impl Whe {
        #[doc = "0 The event has not yet been detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The event has been detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Idle_SPEC;
    pub type Idle = crate::EnumBitfieldStruct<u8, Idle_SPEC>;
    impl Idle {
        #[doc = "0 No action."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit field MCMP is cleared  the selected outputs are set to passive state."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Str_SPEC;
    pub type Str = crate::EnumBitfieldStruct<u8, Str_SPEC>;
    impl Str {
        #[doc = "0 The event has not yet been detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The event has been detected."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr_SPEC;
impl crate::sealed::RegSpec for Isr_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Status Reset Register\n resetvalue={Application Reset:0x0}"]
pub type Isr = crate::RegValueT<Isr_SPEC>;

impl Isr {
    #[doc = "Reset Capture  Compare Match Rising Edge Flag   RCC6xR  x 0 1 2"]
    #[inline(always)]
    pub fn rcc60r(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, isr::Rcc60R, Isr_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0x1,1,0,isr::Rcc60R, Isr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Reset Capture  Compare Match Falling Edge Flag   RCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn rcc60f(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, isr::Rcc60F, Isr_SPEC, crate::common::W> {
        crate::common::RegisterField::<1,0x1,1,0,isr::Rcc60F, Isr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Reset Capture  Compare Match Rising Edge Flag   RCC6xR  x 0 1 2"]
    #[inline(always)]
    pub fn rcc61r(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, isr::Rcc61R, Isr_SPEC, crate::common::W> {
        crate::common::RegisterField::<2,0x1,1,0,isr::Rcc61R, Isr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Reset Capture  Compare Match Falling Edge Flag   RCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn rcc61f(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, isr::Rcc61F, Isr_SPEC, crate::common::W> {
        crate::common::RegisterField::<3,0x1,1,0,isr::Rcc61F, Isr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Reset Capture  Compare Match Rising Edge Flag   RCC6xR  x 0 1 2"]
    #[inline(always)]
    pub fn rcc62r(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, isr::Rcc62R, Isr_SPEC, crate::common::W> {
        crate::common::RegisterField::<4,0x1,1,0,isr::Rcc62R, Isr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Reset Capture  Compare Match Falling Edge Flag   RCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn rcc62f(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, isr::Rcc62F, Isr_SPEC, crate::common::W> {
        crate::common::RegisterField::<5,0x1,1,0,isr::Rcc62F, Isr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Reset Timer T12 One Match Flag   RT12OM"]
    #[inline(always)]
    pub fn rt12om(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, isr::Rt12Om, Isr_SPEC, crate::common::W> {
        crate::common::RegisterField::<6,0x1,1,0,isr::Rt12Om, Isr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Reset Timer T12 Period Match Flag   RT12PM"]
    #[inline(always)]
    pub fn rt12pm(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, isr::Rt12Pm, Isr_SPEC, crate::common::W> {
        crate::common::RegisterField::<7,0x1,1,0,isr::Rt12Pm, Isr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Reset Timer T13 Compare Match Flag   RT13CM"]
    #[inline(always)]
    pub fn rt13cm(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, isr::Rt13Cm, Isr_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0x1,1,0,isr::Rt13Cm, Isr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Reset Timer T13 Period Match Flag   RT13PM"]
    #[inline(always)]
    pub fn rt13pm(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, isr::Rt13Pm, Isr_SPEC, crate::common::W> {
        crate::common::RegisterField::<9,0x1,1,0,isr::Rt13Pm, Isr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Reset Trap Flag   RTRPF"]
    #[inline(always)]
    pub fn rtrpf(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, isr::Rtrpf, Isr_SPEC, crate::common::W> {
        crate::common::RegisterField::<10,0x1,1,0,isr::Rtrpf, Isr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Reset Correct Hall Event Flag   RCHE"]
    #[inline(always)]
    pub fn rche(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, isr::Rche, Isr_SPEC, crate::common::W> {
        crate::common::RegisterField::<12,0x1,1,0,isr::Rche, Isr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Reset Wrong Hall Event Flag   RWHE"]
    #[inline(always)]
    pub fn rwhe(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, isr::Rwhe, Isr_SPEC, crate::common::W> {
        crate::common::RegisterField::<13,0x1,1,0,isr::Rwhe, Isr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Reset IDLE Flag   RIDLE"]
    #[inline(always)]
    pub fn ridle(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, isr::Ridle, Isr_SPEC, crate::common::W> {
        crate::common::RegisterField::<14,0x1,1,0,isr::Ridle, Isr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Reset STR Flag   RSTR"]
    #[inline(always)]
    pub fn rstr(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, isr::Rstr, Isr_SPEC, crate::common::W> {
        crate::common::RegisterField::<15,0x1,1,0,isr::Rstr, Isr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Isr {
    #[inline(always)]
    fn default() -> Isr {
        <crate::RegValueT<Isr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod isr {
    pub struct Rcc60R_SPEC;
    pub type Rcc60R = crate::EnumBitfieldStruct<u8, Rcc60R_SPEC>;
    impl Rcc60R {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit CC6xR will be cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rcc60F_SPEC;
    pub type Rcc60F = crate::EnumBitfieldStruct<u8, Rcc60F_SPEC>;
    impl Rcc60F {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit CC6xF will be cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rcc61R_SPEC;
    pub type Rcc61R = crate::EnumBitfieldStruct<u8, Rcc61R_SPEC>;
    impl Rcc61R {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit CC6xR will be cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rcc61F_SPEC;
    pub type Rcc61F = crate::EnumBitfieldStruct<u8, Rcc61F_SPEC>;
    impl Rcc61F {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit CC6xF will be cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rcc62R_SPEC;
    pub type Rcc62R = crate::EnumBitfieldStruct<u8, Rcc62R_SPEC>;
    impl Rcc62R {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit CC6xR will be cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rcc62F_SPEC;
    pub type Rcc62F = crate::EnumBitfieldStruct<u8, Rcc62F_SPEC>;
    impl Rcc62F {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit CC6xF will be cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rt12Om_SPEC;
    pub type Rt12Om = crate::EnumBitfieldStruct<u8, Rt12Om_SPEC>;
    impl Rt12Om {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit T12OM will be cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rt12Pm_SPEC;
    pub type Rt12Pm = crate::EnumBitfieldStruct<u8, Rt12Pm_SPEC>;
    impl Rt12Pm {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit T12PM IS will be cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rt13Cm_SPEC;
    pub type Rt13Cm = crate::EnumBitfieldStruct<u8, Rt13Cm_SPEC>;
    impl Rt13Cm {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit T13CM will be cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rt13Pm_SPEC;
    pub type Rt13Pm = crate::EnumBitfieldStruct<u8, Rt13Pm_SPEC>;
    impl Rt13Pm {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit T13PM will be cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rtrpf_SPEC;
    pub type Rtrpf = crate::EnumBitfieldStruct<u8, Rtrpf_SPEC>;
    impl Rtrpf {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit TRPF will be cleared  not taken into account while input CTRAP  0 and TRPPEN 1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rche_SPEC;
    pub type Rche = crate::EnumBitfieldStruct<u8, Rche_SPEC>;
    impl Rche {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit CHE will be cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rwhe_SPEC;
    pub type Rwhe = crate::EnumBitfieldStruct<u8, Rwhe_SPEC>;
    impl Rwhe {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit WHE will be cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ridle_SPEC;
    pub type Ridle = crate::EnumBitfieldStruct<u8, Ridle_SPEC>;
    impl Ridle {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit IDLE will be cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rstr_SPEC;
    pub type Rstr = crate::EnumBitfieldStruct<u8, Rstr_SPEC>;
    impl Rstr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit STR will be cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iss_SPEC;
impl crate::sealed::RegSpec for Iss_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Status Set Register\n resetvalue={Application Reset:0x0}"]
pub type Iss = crate::RegValueT<Iss_SPEC>;

impl Iss {
    #[doc = "Set Capture  Compare Match Rising Edge Flag   SCC6xR  x 0 1 2"]
    #[inline(always)]
    pub fn scc60r(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, iss::Scc60R, Iss_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0x1,1,0,iss::Scc60R, Iss_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Capture  Compare Match Falling Edge Flag   SCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn scc60f(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, iss::Scc60F, Iss_SPEC, crate::common::W> {
        crate::common::RegisterField::<1,0x1,1,0,iss::Scc60F, Iss_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Capture  Compare Match Rising Edge Flag   SCC6xR  x 0 1 2"]
    #[inline(always)]
    pub fn scc61r(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, iss::Scc61R, Iss_SPEC, crate::common::W> {
        crate::common::RegisterField::<2,0x1,1,0,iss::Scc61R, Iss_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Capture  Compare Match Falling Edge Flag   SCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn scc61f(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, iss::Scc61F, Iss_SPEC, crate::common::W> {
        crate::common::RegisterField::<3,0x1,1,0,iss::Scc61F, Iss_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Capture  Compare Match Rising Edge Flag   SCC6xR  x 0 1 2"]
    #[inline(always)]
    pub fn scc62r(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, iss::Scc62R, Iss_SPEC, crate::common::W> {
        crate::common::RegisterField::<4,0x1,1,0,iss::Scc62R, Iss_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Capture  Compare Match Falling Edge Flag   SCC6xF  x 0 1 2"]
    #[inline(always)]
    pub fn scc62f(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, iss::Scc62F, Iss_SPEC, crate::common::W> {
        crate::common::RegisterField::<5,0x1,1,0,iss::Scc62F, Iss_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Timer T12 One Match Flag   ST12OM"]
    #[inline(always)]
    pub fn st12om(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, iss::St12Om, Iss_SPEC, crate::common::W> {
        crate::common::RegisterField::<6,0x1,1,0,iss::St12Om, Iss_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Timer T12 Period Match Flag   ST12PM"]
    #[inline(always)]
    pub fn st12pm(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, iss::St12Pm, Iss_SPEC, crate::common::W> {
        crate::common::RegisterField::<7,0x1,1,0,iss::St12Pm, Iss_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Timer T13 Compare Match Flag   ST13CM"]
    #[inline(always)]
    pub fn st13cm(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, iss::St13Cm, Iss_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0x1,1,0,iss::St13Cm, Iss_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Timer T13 Period Match Flag   ST13PM"]
    #[inline(always)]
    pub fn st13pm(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, iss::St13Pm, Iss_SPEC, crate::common::W> {
        crate::common::RegisterField::<9,0x1,1,0,iss::St13Pm, Iss_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Trap Flag   STRPF"]
    #[inline(always)]
    pub fn strpf(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, iss::Strpf, Iss_SPEC, crate::common::W> {
        crate::common::RegisterField::<10,0x1,1,0,iss::Strpf, Iss_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Software Hall Compare   SWHC"]
    #[inline(always)]
    pub fn swhc(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, iss::Swhc, Iss_SPEC, crate::common::W> {
        crate::common::RegisterField::<11,0x1,1,0,iss::Swhc, Iss_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Correct Hall Event Flag   SCHE"]
    #[inline(always)]
    pub fn sche(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, iss::Sche, Iss_SPEC, crate::common::W> {
        crate::common::RegisterField::<12,0x1,1,0,iss::Sche, Iss_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Wrong Hall Event Flag   SWHE"]
    #[inline(always)]
    pub fn swhe(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, iss::Swhe, Iss_SPEC, crate::common::W> {
        crate::common::RegisterField::<13,0x1,1,0,iss::Swhe, Iss_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set IDLE Flag   SIDLE"]
    #[inline(always)]
    pub fn sidle(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, iss::Sidle, Iss_SPEC, crate::common::W> {
        crate::common::RegisterField::<14,0x1,1,0,iss::Sidle, Iss_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set STR Flag   SSTR"]
    #[inline(always)]
    pub fn sstr(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, iss::Sstr, Iss_SPEC, crate::common::W> {
        crate::common::RegisterField::<15,0x1,1,0,iss::Sstr, Iss_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Iss {
    #[inline(always)]
    fn default() -> Iss {
        <crate::RegValueT<Iss_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iss {
    pub struct Scc60R_SPEC;
    pub type Scc60R = crate::EnumBitfieldStruct<u8, Scc60R_SPEC>;
    impl Scc60R {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit CC6xR will        be set."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scc60F_SPEC;
    pub type Scc60F = crate::EnumBitfieldStruct<u8, Scc60F_SPEC>;
    impl Scc60F {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit CC6xF will        be set."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scc61R_SPEC;
    pub type Scc61R = crate::EnumBitfieldStruct<u8, Scc61R_SPEC>;
    impl Scc61R {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit CC6xR will        be set."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scc61F_SPEC;
    pub type Scc61F = crate::EnumBitfieldStruct<u8, Scc61F_SPEC>;
    impl Scc61F {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit CC6xF will        be set."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scc62R_SPEC;
    pub type Scc62R = crate::EnumBitfieldStruct<u8, Scc62R_SPEC>;
    impl Scc62R {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit CC6xR will        be set."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scc62F_SPEC;
    pub type Scc62F = crate::EnumBitfieldStruct<u8, Scc62F_SPEC>;
    impl Scc62F {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit CC6xF will        be set."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct St12Om_SPEC;
    pub type St12Om = crate::EnumBitfieldStruct<u8, St12Om_SPEC>;
    impl St12Om {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit T12OM will be set."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct St12Pm_SPEC;
    pub type St12Pm = crate::EnumBitfieldStruct<u8, St12Pm_SPEC>;
    impl St12Pm {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit T12PM will be set."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct St13Cm_SPEC;
    pub type St13Cm = crate::EnumBitfieldStruct<u8, St13Cm_SPEC>;
    impl St13Cm {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit T13CM will be set."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct St13Pm_SPEC;
    pub type St13Pm = crate::EnumBitfieldStruct<u8, St13Pm_SPEC>;
    impl St13Pm {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit T13PM will be set."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Strpf_SPEC;
    pub type Strpf = crate::EnumBitfieldStruct<u8, Strpf_SPEC>;
    impl Strpf {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bits TRPF and TRPS will be set."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Swhc_SPEC;
    pub type Swhc = crate::EnumBitfieldStruct<u8, Swhc_SPEC>;
    impl Swhc {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The Hall compare action is triggered."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sche_SPEC;
    pub type Sche = crate::EnumBitfieldStruct<u8, Sche_SPEC>;
    impl Sche {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit CHE will be set."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Swhe_SPEC;
    pub type Swhe = crate::EnumBitfieldStruct<u8, Swhe_SPEC>;
    impl Swhe {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit WHE will be set."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sidle_SPEC;
    pub type Sidle = crate::EnumBitfieldStruct<u8, Sidle_SPEC>;
    impl Sidle {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit IDLE will be set."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sstr_SPEC;
    pub type Sstr = crate::EnumBitfieldStruct<u8, Sstr_SPEC>;
    impl Sstr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit STR will be set."]
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel registers are set. The RST bit will be cleared  re set to  0   by the BPI after the kernel reset was executed."]
    #[inline(always)]
    pub fn rst(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, krst0::Rst, Krst0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,krst0::Rst, Krst0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Kernel Reset Status   RSTSTAT. This bit indicates wether a kernel reset was executed or not. This bit        is set after the execution of a kernel reset in the same clock cycle in        which the reset bits are cleared. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related KRSTCLR register."]
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel reset registers are set. The RST bit will be cleared  re set to  0   by the BPI after the kernel reset was executed."]
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
pub struct Kscsr_SPEC;
impl crate::sealed::RegSpec for Kscsr_SPEC {
    type DataType = u32;
}
#[doc = "Kernel State Control Sensitivity Register\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
pub type Kscsr = crate::RegValueT<Kscsr_SPEC>;

impl Kscsr {
    #[doc = "Sensitivity Block x  SBx  x 0 1 2 3 . This bit defines if block x of the CCU6 kernel is sensitive to Stop Mode 0 or Stop Mode 1. The functional        definition of the blocks is given in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn sb0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, kscsr::Sb0, Kscsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,kscsr::Sb0, Kscsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sensitivity Block x  SBx  x 0 1 2 3 . This bit defines if block x of the CCU6 kernel is sensitive to Stop Mode 0 or Stop Mode 1. The functional        definition of the blocks is given in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn sb1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, kscsr::Sb1, Kscsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,kscsr::Sb1, Kscsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sensitivity Block x  SBx  x 0 1 2 3 . This bit defines if block x of the CCU6 kernel is sensitive to Stop Mode 0 or Stop Mode 1. The functional        definition of the blocks is given in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn sb2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, kscsr::Sb2, Kscsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,kscsr::Sb2, Kscsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sensitivity Block x  SBx  x 0 1 2 3 . This bit defines if block x of the CCU6 kernel is sensitive to Stop Mode 0 or Stop Mode 1. The functional        definition of the blocks is given in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn sb3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, kscsr::Sb3, Kscsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,kscsr::Sb3, Kscsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Kscsr {
    #[inline(always)]
    fn default() -> Kscsr {
        <crate::RegValueT<Kscsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod kscsr {
    pub struct Sb0_SPEC;
    pub type Sb0 = crate::EnumBitfieldStruct<u8, Sb0_SPEC>;
    impl Sb0 {
        #[doc = "0 Block x is not sensitive to Stop Mode 0 or Stop Mode 1. It continues normal operation without respecting the defined stop condition."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Block x is sensitive to Stop Mode 0 or Stop Mode 1. It is respecting the defined stop condition."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sb1_SPEC;
    pub type Sb1 = crate::EnumBitfieldStruct<u8, Sb1_SPEC>;
    impl Sb1 {
        #[doc = "0 Block x is not sensitive to Stop Mode 0 or Stop Mode 1. It continues normal operation without respecting the defined stop condition."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Block x is sensitive to Stop Mode 0 or Stop Mode 1. It is respecting the defined stop condition."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sb2_SPEC;
    pub type Sb2 = crate::EnumBitfieldStruct<u8, Sb2_SPEC>;
    impl Sb2 {
        #[doc = "0 Block x is not sensitive to Stop Mode 0 or Stop Mode 1. It continues normal operation without respecting the defined stop condition."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Block x is sensitive to Stop Mode 0 or Stop Mode 1. It is respecting the defined stop condition."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sb3_SPEC;
    pub type Sb3 = crate::EnumBitfieldStruct<u8, Sb3_SPEC>;
    impl Sb3 {
        #[doc = "0 Block x is not sensitive to Stop Mode 0 or Stop Mode 1. It continues normal operation without respecting the defined stop condition."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Block x is sensitive to Stop Mode 0 or Stop Mode 1. It is respecting the defined stop condition."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Li_SPEC;
impl crate::sealed::RegSpec for Li_SPEC {
    type DataType = u32;
}
#[doc = "Lost Indicator Register\n resetvalue={Application Reset:0x0}"]
pub type Li = crate::RegValueT<Li_SPEC>;

impl Li {
    #[doc = "Lost Indicator Enable for input signal CCPOS2   CCPOS2EN. This bit determines if the monitored event at the input signal is enabled for the detection of a lost bit event."]
    #[inline(always)]
    pub fn ccpos0en(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, li::Ccpos0En, Li_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,li::Ccpos0En, Li_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lost Indicator Enable for input signal CCPOS2   CCPOS2EN. This bit determines if the monitored event at the input signal is enabled for the detection of a lost bit event."]
    #[inline(always)]
    pub fn ccpos1en(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, li::Ccpos1En, Li_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,li::Ccpos1En, Li_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lost Indicator Enable for input signal CCPOS2   CCPOS2EN. This bit determines if the monitored event at the input signal is enabled for the detection of a lost bit event."]
    #[inline(always)]
    pub fn ccpos2en(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, li::Ccpos2En, Li_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,li::Ccpos2En, Li_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lost Indicator Enable for input signal CC62IN   CC62INEN. This bit determines if the monitored event at the input signal is enabled for the detection of a lost bit event."]
    #[inline(always)]
    pub fn cc60inen(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, li::Cc60Inen, Li_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,li::Cc60Inen, Li_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lost Indicator Enable for input signal CC62IN   CC62INEN. This bit determines if the monitored event at the input signal is enabled for the detection of a lost bit event."]
    #[inline(always)]
    pub fn cc61inen(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, li::Cc61Inen, Li_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,li::Cc61Inen, Li_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lost Indicator Enable for input signal CC62IN   CC62INEN. This bit determines if the monitored event at the input signal is enabled for the detection of a lost bit event."]
    #[inline(always)]
    pub fn cc62inen(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, li::Cc62Inen, Li_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,li::Cc62Inen, Li_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lost Indicator Enable for input signal CTRAP   CTRAPEN. This bit determines if the monitored event at the input signal is enabled for the detection of a lost bit event."]
    #[inline(always)]
    pub fn ctrapen(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, li::Ctrapen, Li_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,li::Ctrapen, Li_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lost Indicator Enable for input signal T12HR   T12HREN. This bit determines if the monitored event at the input signal is enabled for the detection of a lost bit event."]
    #[inline(always)]
    pub fn t12hren(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, li::T12Hren, Li_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,li::T12Hren, Li_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lost Indicator Enable for input signal T13HR   T13HREN. This bit determines if the monitored event at the input signal is enabled for the detection of a lost bit event."]
    #[inline(always)]
    pub fn t13hren(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, li::T13Hren, Li_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,li::T13Hren, Li_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Enable for Lost Bit Event   LBEEN. This bit determines if a SRx line is activated if lost bit event is detected."]
    #[inline(always)]
    pub fn lbeen(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, li::Lbeen, Li_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x1,1,0,li::Lbeen, Li_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer for lost bit event   INPLBE. This bit field defines which service request output line is selected to output an lost event alert for an enabled lost bit event."]
    #[inline(always)]
    pub fn inplbe(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, li::Inplbe, Li_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,li::Inplbe, Li_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Li {
    #[inline(always)]
    fn default() -> Li {
        <crate::RegValueT<Li_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod li {
    pub struct Ccpos0En_SPEC;
    pub type Ccpos0En = crate::EnumBitfieldStruct<u8, Ccpos0En_SPEC>;
    impl Ccpos0En {
        #[doc = "0 Input signal is disabled for a lost bit event detection."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Input signal is enabled for a lost bit event detection."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ccpos1En_SPEC;
    pub type Ccpos1En = crate::EnumBitfieldStruct<u8, Ccpos1En_SPEC>;
    impl Ccpos1En {
        #[doc = "0 Input signal is disabled for a lost bit event detection."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Input signal is enabled for a lost bit event detection."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ccpos2En_SPEC;
    pub type Ccpos2En = crate::EnumBitfieldStruct<u8, Ccpos2En_SPEC>;
    impl Ccpos2En {
        #[doc = "0 Input signal is disabled for a lost bit event detection."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Input signal is enabled for a lost bit event detection."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cc60Inen_SPEC;
    pub type Cc60Inen = crate::EnumBitfieldStruct<u8, Cc60Inen_SPEC>;
    impl Cc60Inen {
        #[doc = "0 Input signal is disabled for a lost bit event detection."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Input signal is enabled for a lost bit event detection."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cc61Inen_SPEC;
    pub type Cc61Inen = crate::EnumBitfieldStruct<u8, Cc61Inen_SPEC>;
    impl Cc61Inen {
        #[doc = "0 Input signal is disabled for a lost bit event detection."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Input signal is enabled for a lost bit event detection."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cc62Inen_SPEC;
    pub type Cc62Inen = crate::EnumBitfieldStruct<u8, Cc62Inen_SPEC>;
    impl Cc62Inen {
        #[doc = "0 Input signal is disabled for a lost bit event detection."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Input signal is enabled for a lost bit event detection."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ctrapen_SPEC;
    pub type Ctrapen = crate::EnumBitfieldStruct<u8, Ctrapen_SPEC>;
    impl Ctrapen {
        #[doc = "0 Input signal is disabled for a lost bit event detection."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Input signal is enabled for a lost bit event detection."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T12Hren_SPEC;
    pub type T12Hren = crate::EnumBitfieldStruct<u8, T12Hren_SPEC>;
    impl T12Hren {
        #[doc = "0 Input signal is disabled for a lost bit event detection."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Input signal is enabled for a lost bit event detection."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T13Hren_SPEC;
    pub type T13Hren = crate::EnumBitfieldStruct<u8, T13Hren_SPEC>;
    impl T13Hren {
        #[doc = "0 Input signal is disabled for a lost bit event detection."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Input signal is enabled for a lost bit event detection."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lbeen_SPEC;
    pub type Lbeen = crate::EnumBitfieldStruct<u8, Lbeen_SPEC>;
    impl Lbeen {
        #[doc = "0 Lost bit event is disabled for the activation of a SRx line."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Lost bit event is enabled for the activation of a SRx line."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Inplbe_SPEC;
    pub type Inplbe = crate::EnumBitfieldStruct<u8, Inplbe_SPEC>;
    impl Inplbe {
        #[doc = "00 Service request output SR0 is selected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Service request output SR1 is selected."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Service request output SR2 is selected."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Service request output SR3 is selected."]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcfg_SPEC;
impl crate::sealed::RegSpec for Mcfg_SPEC {
    type DataType = u32;
}
#[doc = "Module Configuration Register\n resetvalue={Application Reset:0x08007,Application Reset:0x7}"]
pub type Mcfg = crate::RegValueT<Mcfg_SPEC>;

impl Mcfg {
    #[doc = "T12 Available   T12. This bit indicates if the T12 block is available."]
    #[inline(always)]
    pub fn t12(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mcfg::T12, Mcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,mcfg::T12, Mcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "T13 Available   T13. This bit indicates if the T13 block is available."]
    #[inline(always)]
    pub fn t13(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, mcfg::T13, Mcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,mcfg::T13, Mcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Multi Channel Mode Available   MCM. This bit indicates if the multi channel mode functionality is available."]
    #[inline(always)]
    pub fn mcm(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, mcfg::Mcm, Mcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,mcfg::Mcm, Mcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Write Enable   WREN. This bit enables disabled the write capability of this register."]
    #[inline(always)]
    pub fn wren(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, mcfg::Wren, Mcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<15,0x1,1,0,mcfg::Wren, Mcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Mcfg {
    #[inline(always)]
    fn default() -> Mcfg {
        <crate::RegValueT<Mcfg_SPEC> as RegisterValue<_>>::new(7)
    }
}
pub mod mcfg {
    pub struct T12_SPEC;
    pub type T12 = crate::EnumBitfieldStruct<u8, T12_SPEC>;
    impl T12 {
        #[doc = "0 The T12 block is not available. A write access to T12PR is ignored."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The T12 block is available. A write access to T12PR is executed."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T13_SPEC;
    pub type T13 = crate::EnumBitfieldStruct<u8, T13_SPEC>;
    impl T13 {
        #[doc = "0 The T13 block is not available. A write access to T13PR is ignored."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The T13 block is available. A write access to T13PR is executed."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mcm_SPEC;
    pub type Mcm = crate::EnumBitfieldStruct<u8, Mcm_SPEC>;
    impl Mcm {
        #[doc = "0 The multi channel mode functionality is not available. A write access to MCMOUTS is ignored."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The multi channel mode functionality is available. A write access to MCMOUTS is executed."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wren_SPEC;
    pub type Wren = crate::EnumBitfieldStruct<u8, Wren_SPEC>;
    impl Wren {
        #[doc = "0 Write accesses to this register are ignored."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 This register can be written  default ."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcmctr_SPEC;
impl crate::sealed::RegSpec for Mcmctr_SPEC {
    type DataType = u32;
}
#[doc = "Multi Channel Mode Control Register\n resetvalue={Application Reset:0x0}"]
pub type Mcmctr = crate::RegValueT<Mcmctr_SPEC>;

impl Mcmctr {
    #[doc = "Switching Selection   SWSEL. Bit field SWSEL selects one of the following trigger request sources  next multi channel event  for the shadow transfer MCM ST from MCMPS to MCMP. The trigger request is stored in the reminder flag R until the shadow transfer is done and flag R is cleared automatically with the shadow transfer. The shadow transfer takes place synchronously with an event selected in bit field SWSYN."]
    #[inline(always)]
    pub fn swsel(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, mcmctr::Swsel, Mcmctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,mcmctr::Swsel, Mcmctr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Switching Synchronization   SWSYN. Bit field SWSYN defines the synchronization mechanism of the shadow transfer event MCM ST if it has been requested before  flag R set by an event selected by SWSEL  and if MCMEN   1. This feature permits the synchronization of the outputs to the PWM source  that is used for modulation  T12 or T13 ."]
    #[inline(always)]
    pub fn swsyn(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, mcmctr::Swsyn, Mcmctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,mcmctr::Swsyn, Mcmctr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Shadow Transfer Enable for T12 Upcounting   STE12U. This bit enables the shadow transfer T12 ST if flag MCMOUT.R is set or becomes set while a T12 period match is detected while counting up."]
    #[inline(always)]
    pub fn ste12u(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, mcmctr::Ste12U, Mcmctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,mcmctr::Ste12U, Mcmctr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Shadow Transfer Enable for T12 Downcounting   STE12D. This bit enables the shadow transfer T12 ST if flag MCMOUT.R is set or becomes set while a T12 one match is detected while counting down."]
    #[inline(always)]
    pub fn ste12d(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, mcmctr::Ste12D, Mcmctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,mcmctr::Ste12D, Mcmctr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Shadow Transfer Enable for T13 Upcounting   STE13U. This bit enables the shadow transfer T13 ST if flag MCMOUT.R is set or becomes set while a T13 period match is detected."]
    #[inline(always)]
    pub fn ste13u(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, mcmctr::Ste13U, Mcmctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,mcmctr::Ste13U, Mcmctr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Mcmctr {
    #[inline(always)]
    fn default() -> Mcmctr {
        <crate::RegValueT<Mcmctr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mcmctr {
    pub struct Swsel_SPEC;
    pub type Swsel = crate::EnumBitfieldStruct<u8, Swsel_SPEC>;
    impl Swsel {
        #[doc = "000 No trigger request will be generated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 Correct Hall pattern detected  CM CHE"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 T13 period match detected  while counting up"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 T12 one match  while counting down"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 T12 channel 1 compare match detected  phase delay function"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 T12 period match detected  while counting up"]
        pub const CONST_55: Self = Self::new(5);
    }
    pub struct Swsyn_SPEC;
    pub type Swsyn = crate::EnumBitfieldStruct<u8, Swsyn_SPEC>;
    impl Swsyn {
        #[doc = "00 Direct  the trigger event immediately leads to the shadow transfer"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 A T13 zero match triggers the shadow transfer"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 A T12 zero match  while counting up  triggers the shadow transfer"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Ste12U_SPEC;
    pub type Ste12U = crate::EnumBitfieldStruct<u8, Ste12U_SPEC>;
    impl Ste12U {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The T12 ST shadow transfer mechanism is enabled if MCMEN   1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ste12D_SPEC;
    pub type Ste12D = crate::EnumBitfieldStruct<u8, Ste12D_SPEC>;
    impl Ste12D {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The T12 ST shadow transfer mechanism is enabled if MCMEN   1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ste13U_SPEC;
    pub type Ste13U = crate::EnumBitfieldStruct<u8, Ste13U_SPEC>;
    impl Ste13U {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The T13 ST shadow transfer mechanism is enabled if MCMEN   1."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcmout_SPEC;
impl crate::sealed::RegSpec for Mcmout_SPEC {
    type DataType = u32;
}
#[doc = "Multi Channel Mode Output Register\n resetvalue={Application Reset:0x0}"]
pub type Mcmout = crate::RegValueT<Mcmout_SPEC>;

impl Mcmout {
    #[doc = "Multi Channel PWM Pattern   MCMP. Bit field MCMP defines the output pattern for the multi channel mode. If this mode is enabled by MODCTR.MCMEN   1  the output state of all T12 related PWM outputs can be modified. This bit field is 0 while IS.IDLE   1. MCMP0   MCMOUT.0 for output CC60 MCMP1   MCMOUT.1 for output COUT60 MCMP2   MCMOUT.2 for output CC61 MCMP3   MCMOUT.3 for output COUT61 MCMP4   MCMOUT.4 for output CC62 MCMP5   MCMOUT.5 for output COUT62"]
    #[inline(always)]
    pub fn mcmp(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, mcmout::Mcmp, Mcmout_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,mcmout::Mcmp, Mcmout_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reminder Flag   R. This flag indicates that the shadow transfer from MCMPS to MCMP has been requested by the selected trigger source. It is cleared when the shadow transfer takes place or while MCMEN 0."]
    #[inline(always)]
    pub fn r(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, mcmout::R, Mcmout_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x1,1,0,mcmout::R, Mcmout_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Expected Hall Pattern   EXPH. Bit field EXPH is updated by a shadow transfer HP ST from bit field EXPHS. If HCRDY   1  EXPH is compared to the sampled CCPOSx inputs in order to detect the occurrence of the next desired   expected  hall pattern or a wrong pattern. If the sampled hall pattern at the hall input pins is equal to bit field EXPH  a correct Hall event has been detected  CM CHE ."]
    #[inline(always)]
    pub fn exph(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Mcmout_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Mcmout_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Current Hall Pattern   CURH. Bit field CURH is updated by a shadow transfer HP ST from bit field CURHS. If HCRDY   1  CURH is compared to the sampled CCPOSx inputs in order to detect a spike. If the sampled Hall pattern at the Hall input pins is equal to bit field CURH  no Hall event has been detected. If the sampled Hall input pattern is neither equal to CURH nor equal to EXPH  the Hall event was not the desired one and may be due to a fatal error  e.g. blocked rotor  etc. . In this case  a wrong Hall event has been detected  CM WHE ."]
    #[inline(always)]
    pub fn curh(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Mcmout_SPEC, crate::common::R> {
        crate::common::RegisterField::<11,0x7,1,0,u8, Mcmout_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Mcmout {
    #[inline(always)]
    fn default() -> Mcmout {
        <crate::RegValueT<Mcmout_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mcmout {
    pub struct Mcmp_SPEC;
    pub type Mcmp = crate::EnumBitfieldStruct<u8, Mcmp_SPEC>;
    impl Mcmp {
        #[doc = "0 The output is set to the passive state. A PWM generated by T12 or T13 are not taken into account."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The output can be in the active state  depending on the enabled PWM modulation signals generated by T12  T13 and the trap state."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct R_SPEC;
    pub type R = crate::EnumBitfieldStruct<u8, R_SPEC>;
    impl R {
        #[doc = "0 A shadow transfer MCM ST is not requested."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A shadow transfer MCM ST is requested  but has not yet been executed  because the selected synchronization condition has not yet occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcmouts_SPEC;
impl crate::sealed::RegSpec for Mcmouts_SPEC {
    type DataType = u32;
}
#[doc = "Multi Channel Mode Output Shadow Register\n resetvalue={Application Reset:0x0}"]
pub type Mcmouts = crate::RegValueT<Mcmouts_SPEC>;

impl Mcmouts {
    #[doc = "Multi Channel PWM Pattern Shadow   MCMPS. Bit field MCMPS is the shadow bit field for bit field MCMP. The multi channel shadow transfer is triggered by MCM ST according to the transfer conditions defined by register MCMCTR."]
    #[inline(always)]
    pub fn mcmps(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Mcmouts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Mcmouts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Shadow Transfer Request for MCMPS   STRMCM. Writing STRMCM  160    160 1 leads to an immediate activation of MCM ST to update        bit field MCMP by the value of MCMPS. When read  this bit always        delivers 0."]
    #[inline(always)]
    pub fn strmcm(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, mcmouts::Strmcm, Mcmouts_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<7,0x1,1,0,mcmouts::Strmcm, Mcmouts_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Expected Hall Pattern Shadow   EXPHS. Bit field EXPHS is the shadow bit field for bit field EXPH. The shadow transfer takes place when a correct Hall event is detected  CM CHE ."]
    #[inline(always)]
    pub fn exphs(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Mcmouts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Mcmouts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Current Hall Pattern Shadow   CURHS. Bit field CURHS is the shadow bit field for bit field CURH. The shadow transfer takes place when a correct Hall event is detected  CM CHE ."]
    #[inline(always)]
    pub fn curhs(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Mcmouts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x7,1,0,u8, Mcmouts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Shadow Transfer Request for the Hall Pattern   STRHP. Writing STRHP   1 leads to an immediate activation of HP ST to update bit fields EXPH and CURH by EXPHS and CURHS. When read  this bit always delivers 0."]
    #[inline(always)]
    pub fn strhp(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, mcmouts::Strhp, Mcmouts_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<15,0x1,1,0,mcmouts::Strhp, Mcmouts_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Mcmouts {
    #[inline(always)]
    fn default() -> Mcmouts {
        <crate::RegValueT<Mcmouts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mcmouts {
    pub struct Strmcm_SPEC;
    pub type Strmcm = crate::EnumBitfieldStruct<u8, Strmcm_SPEC>;
    impl Strmcm {
        #[doc = "0 No action."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit field MCMP        is updated."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Strhp_SPEC;
    pub type Strhp = crate::EnumBitfieldStruct<u8, Strhp_SPEC>;
    impl Strhp {
        #[doc = "0 No action."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit fields EXPH        and CURH are updated."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modctr_SPEC;
impl crate::sealed::RegSpec for Modctr_SPEC {
    type DataType = u32;
}
#[doc = "Modulation Control Register\n resetvalue={Application Reset:0x0}"]
pub type Modctr = crate::RegValueT<Modctr_SPEC>;

impl Modctr {
    #[doc = "T12 Modulation Enable   T12MODEN. These bits enable the modulation of the corresponding output signal by a PWM pattern generated by timer T12. T12MODEN0   MODCTR.0 for output CC60 T12MODEN1   MODCTR.1 for output COUT60 T12MODEN2   MODCTR.2 for output CC61 T12MODEN3   MODCTR.3 for output COUT61 T12MODEN4   MODCTR.4 for output CC62 T12MODEN5   MODCTR.5 for output COUT62"]
    #[inline(always)]
    pub fn t12moden(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, modctr::T12Moden, Modctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            modctr::T12Moden,
            Modctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Multi Channel Mode Enable   MCMEN"]
    #[inline(always)]
    pub fn mcmen(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, modctr::Mcmen, Modctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,modctr::Mcmen, Modctr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "T13 Modulation Enable   T13MODEN. These bits enable the modulation of the corresponding output signal by the PWM pattern CC63 O generated by timer T13. T13MODEN0   MODCTR.8 for output CC60 T13MODEN1   MODCTR.9 for output COUT60 T13MODEN2   MODCTR.10 for output CC61 T13MODEN3   MODCTR.11 for output COUT61 T13MODEN4   MODCTR.12 for output CC62 T13MODEN5   MODCTR.13 for output COUT62"]
    #[inline(always)]
    pub fn t13moden(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, modctr::T13Moden, Modctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0x3f,
            1,
            0,
            modctr::T13Moden,
            Modctr_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Enable Compare Timer T13 Output   ECT13O"]
    #[inline(always)]
    pub fn ect13o(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, modctr::Ect13O, Modctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,modctr::Ect13O, Modctr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Modctr {
    #[inline(always)]
    fn default() -> Modctr {
        <crate::RegValueT<Modctr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod modctr {
    pub struct T12Moden_SPEC;
    pub type T12Moden = crate::EnumBitfieldStruct<u8, T12Moden_SPEC>;
    impl T12Moden {
        #[doc = "0 The modulation of the corresponding output signal by a T12 PWM pattern is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The modulation of the corresponding output signal by a T12 PWM pattern is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mcmen_SPEC;
    pub type Mcmen = crate::EnumBitfieldStruct<u8, Mcmen_SPEC>;
    impl Mcmen {
        #[doc = "0 The modulation of the corresponding output signal by a multi channel pattern according to bit field MCMOUT is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The modulation of the corresponding output signal by a multi channel pattern according to bit field MCMOUT is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T13Moden_SPEC;
    pub type T13Moden = crate::EnumBitfieldStruct<u8, T13Moden_SPEC>;
    impl T13Moden {
        #[doc = "0 The modulation of the corresponding output signal by a T13 PWM pattern is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The modulation of the corresponding output signal by a T13 PWM pattern is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ect13O_SPEC;
    pub type Ect13O = crate::EnumBitfieldStruct<u8, Ect13O_SPEC>;
    impl Ect13O {
        #[doc = "0 The output COUT63 is in the passive state."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The output COUT63 is enabled for the PWM signal generated by T13."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ocs_SPEC;
impl crate::sealed::RegSpec for Ocs_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Control and Status Register\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
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
    #[doc = "OCDS Suspend Control   SUS. Controls the sensitivity to the suspend signal coming from the OCDS        Trigger Switch  OTGS  Effects of soft suspend options on CCU6 Functional Blocks are described        in section CROSSREFERENCE"]
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
        #[doc = "0 No Trigger Set        output"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Trigger Set        TS16 CCU6  see CROSSREFERENCE"]
        pub const CONST_11: Self = Self::new(1);
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
        #[doc = "0 Will not        suspend"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Hard suspend.        Clock is switched off immediately."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Soft suspend         Stop Mode 0"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 Soft suspend         Stop Mode 1"]
        pub const CONST_33: Self = Self::new(3);
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
pub struct Pisel0_SPEC;
impl crate::sealed::RegSpec for Pisel0_SPEC {
    type DataType = u32;
}
#[doc = "Port Input Select Register 0\n resetvalue={Application Reset:0x0}"]
pub type Pisel0 = crate::RegValueT<Pisel0_SPEC>;

impl Pisel0 {
    #[doc = "Input Select for CC60 ISCC6x  x 0 1 2 . This bit field defines the input signal used as CC6x capture input."]
    #[inline(always)]
    pub fn iscc60(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, pisel0::Iscc60, Pisel0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,pisel0::Iscc60, Pisel0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for CC60 ISCC6x  x 0 1 2 . This bit field defines the input signal used as CC6x capture input."]
    #[inline(always)]
    pub fn iscc61(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, pisel0::Iscc61, Pisel0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,pisel0::Iscc61, Pisel0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for CC60 ISCC6x  x 0 1 2 . This bit field defines the input signal used as CC6x capture input."]
    #[inline(always)]
    pub fn iscc62(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, pisel0::Iscc62, Pisel0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,pisel0::Iscc62, Pisel0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for CTRAP   ISTRP. This bit field defines the input signal used as CTRAP input."]
    #[inline(always)]
    pub fn istrp(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, pisel0::Istrp, Pisel0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,pisel0::Istrp, Pisel0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for CCPOS0 ISPOSx  x 0 1 2 . This bit field defines the input signal used as CCPOSx input."]
    #[inline(always)]
    pub fn ispos0(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, pisel0::Ispos0, Pisel0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,pisel0::Ispos0, Pisel0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for CCPOS0 ISPOSx  x 0 1 2 . This bit field defines the input signal used as CCPOSx input."]
    #[inline(always)]
    pub fn ispos1(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, pisel0::Ispos1, Pisel0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,pisel0::Ispos1, Pisel0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for CCPOS0 ISPOSx  x 0 1 2 . This bit field defines the input signal used as CCPOSx input."]
    #[inline(always)]
    pub fn ispos2(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, pisel0::Ispos2, Pisel0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,pisel0::Ispos2, Pisel0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for T12HR   IST12HR. This bit field defines the input signal used as T12HR input."]
    #[inline(always)]
    pub fn ist12hr(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, pisel0::Ist12Hr, Pisel0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,pisel0::Ist12Hr, Pisel0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pisel0 {
    #[inline(always)]
    fn default() -> Pisel0 {
        <crate::RegValueT<Pisel0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pisel0 {
    pub struct Iscc60_SPEC;
    pub type Iscc60 = crate::EnumBitfieldStruct<u8, Iscc60_SPEC>;
    impl Iscc60 {
        #[doc = "00 The signal        CC6xINA is selected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 The signal        CC6xINB is selected."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 The signal        CC6xINC is selected."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 The signal        CC6xIND is selected."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Iscc61_SPEC;
    pub type Iscc61 = crate::EnumBitfieldStruct<u8, Iscc61_SPEC>;
    impl Iscc61 {
        #[doc = "00 The signal        CC6xINA is selected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 The signal        CC6xINB is selected."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 The signal        CC6xINC is selected."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 The signal        CC6xIND is selected."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Iscc62_SPEC;
    pub type Iscc62 = crate::EnumBitfieldStruct<u8, Iscc62_SPEC>;
    impl Iscc62 {
        #[doc = "00 The signal        CC6xINA is selected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 The signal        CC6xINB is selected."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 The signal        CC6xINC is selected."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 The signal        CC6xIND is selected."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Istrp_SPEC;
    pub type Istrp = crate::EnumBitfieldStruct<u8, Istrp_SPEC>;
    impl Istrp {
        #[doc = "00 The signal CTRAPA is selected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 The signal CTRAPB is selected."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 The signal CTRAPC is selected."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 The signal CTRAPD is selected."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Ispos0_SPEC;
    pub type Ispos0 = crate::EnumBitfieldStruct<u8, Ispos0_SPEC>;
    impl Ispos0 {
        #[doc = "00 The signal        CCPOSxA is selected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 The signal        CCPOSxB is selected."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 The signal        CCPOSxC is selected."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 The signal        CCPOSxD is selected."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Ispos1_SPEC;
    pub type Ispos1 = crate::EnumBitfieldStruct<u8, Ispos1_SPEC>;
    impl Ispos1 {
        #[doc = "00 The signal        CCPOSxA is selected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 The signal        CCPOSxB is selected."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 The signal        CCPOSxC is selected."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 The signal        CCPOSxD is selected."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Ispos2_SPEC;
    pub type Ispos2 = crate::EnumBitfieldStruct<u8, Ispos2_SPEC>;
    impl Ispos2 {
        #[doc = "00 The signal        CCPOSxA is selected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 The signal        CCPOSxB is selected."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 The signal        CCPOSxC is selected."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 The signal        CCPOSxD is selected."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Ist12Hr_SPEC;
    pub type Ist12Hr = crate::EnumBitfieldStruct<u8, Ist12Hr_SPEC>;
    impl Ist12Hr {
        #[doc = "00 Either signal T12HRA  if T12EXT   0  or T12HRE  if T12EXT   1  is selected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Either signal T12HRB  if T12EXT   0  or T12HRF  if T12EXT   1  is selected."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Either signal T12HRC  if T12EXT   0  or T12HRG  if T12EXT   1  is selected."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Either signal T12HRD  if T12EXT   0  or T12HRH  if T12EXT   1  is selected."]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pisel2_SPEC;
impl crate::sealed::RegSpec for Pisel2_SPEC {
    type DataType = u32;
}
#[doc = "Port Input Select Register 2\n resetvalue={Application Reset:0x0}"]
pub type Pisel2 = crate::RegValueT<Pisel2_SPEC>;

impl Pisel2 {
    #[doc = "Input Select for T13HR   IST13HR. This bit field defines the input signal used as T13HR input."]
    #[inline(always)]
    pub fn ist13hr(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, pisel2::Ist13Hr, Pisel2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,pisel2::Ist13Hr, Pisel2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for T12 Counting Input   ISCNT12. This bit field defines the input event leading to a counting action of T12."]
    #[inline(always)]
    pub fn iscnt12(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, pisel2::Iscnt12, Pisel2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,pisel2::Iscnt12, Pisel2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for T13 Counting Input   ISCNT13. This bit field defines the input event leading to a counting action of T13."]
    #[inline(always)]
    pub fn iscnt13(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, pisel2::Iscnt13, Pisel2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,pisel2::Iscnt13, Pisel2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extension for T12HR Inputs   T12EXT. This bit extends the 2 bit field IST12HR."]
    #[inline(always)]
    pub fn t12ext(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, pisel2::T12Ext, Pisel2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,pisel2::T12Ext, Pisel2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extension for T13HR Inputs   T13EXT. This bit extends the 2 bit field IST13HR."]
    #[inline(always)]
    pub fn t13ext(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, pisel2::T13Ext, Pisel2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,pisel2::T13Ext, Pisel2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pisel2 {
    #[inline(always)]
    fn default() -> Pisel2 {
        <crate::RegValueT<Pisel2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pisel2 {
    pub struct Ist13Hr_SPEC;
    pub type Ist13Hr = crate::EnumBitfieldStruct<u8, Ist13Hr_SPEC>;
    impl Ist13Hr {
        #[doc = "00 Either signal T13HRA  if T13EXT   0  or T13HRE  if T13EXT   1  is selected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Either signal T13HRB  if T13EXT   0  or T13HRF  if T13EXT   1  is selected."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Either signal T13HRC  if T13EXT   0  or T13HRG  if T13EXT   1  is selected."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Either signal T13HRD  if T13EXT   0  or T13HRH  if T13EXT   1  is selected."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Iscnt12_SPEC;
    pub type Iscnt12 = crate::EnumBitfieldStruct<u8, Iscnt12_SPEC>;
    impl Iscnt12 {
        #[doc = "00 The T12 prescaler generates the counting events. Bit TCTR4.T12CNT is not taken into account."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Bit TCTR4.T12CNT written with 1 is a counting event. The T12 prescaler is not taken into account."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 The timer T12 is counting each rising edge detected in the selected T12HR signal."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 The timer T12 is counting each falling edge detected in the selected T12HR signal."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Iscnt13_SPEC;
    pub type Iscnt13 = crate::EnumBitfieldStruct<u8, Iscnt13_SPEC>;
    impl Iscnt13 {
        #[doc = "00 The T13 prescaler generates the counting events. Bit TCTR4.T13CNT is not taken into account."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Bit TCTR4.T13CNT written with 1 is a counting event. The T13 prescaler is not taken into account."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 The timer T13 is counting each rising edge detected in the selected T13HR signal."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 The timer T13 is counting each falling edge detected in the selected T13HR signal."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct T12Ext_SPEC;
    pub type T12Ext = crate::EnumBitfieldStruct<u8, T12Ext_SPEC>;
    impl T12Ext {
        #[doc = "0 One of the signals T12HR D A  is selected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 One of the signals T12HR H E  is selected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T13Ext_SPEC;
    pub type T13Ext = crate::EnumBitfieldStruct<u8, T13Ext_SPEC>;
    impl T13Ext {
        #[doc = "0 One of the signals T13HR D A  is selected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 One of the signals T13HR H E  is selected."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pslr_SPEC;
impl crate::sealed::RegSpec for Pslr_SPEC {
    type DataType = u32;
}
#[doc = "Passive State Level Register\n resetvalue={Application Reset:0x0}"]
pub type Pslr = crate::RegValueT<Pslr_SPEC>;

impl Pslr {
    #[doc = "Compare Outputs Passive State Level   PSL. These bits define the passive level driven by the module outputs during the passive state. PSL0   PSLR.0 for output CC60 PSL1   PSLR.1 for output COUT60 PSL2   PSLR.2 for output CC61 PSL3   PSLR.3 for output COUT61 PSL4   PSLR.4 for output CC62 PSL5   PSLR.5 for output COUT62"]
    #[inline(always)]
    pub fn psl(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, pslr::Psl, Pslr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,pslr::Psl, Pslr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Passive State Level of Output COUT63   PSL63. This bit defines the passive level driven by the module output COUT63 during the passive state."]
    #[inline(always)]
    pub fn psl63(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, pslr::Psl63, Pslr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,pslr::Psl63, Pslr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pslr {
    #[inline(always)]
    fn default() -> Pslr {
        <crate::RegValueT<Pslr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pslr {
    pub struct Psl_SPEC;
    pub type Psl = crate::EnumBitfieldStruct<u8, Psl_SPEC>;
    impl Psl {
        #[doc = "0 The passive level is 0."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The passive level is 1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Psl63_SPEC;
    pub type Psl63 = crate::EnumBitfieldStruct<u8, Psl63_SPEC>;
    impl Psl63 {
        #[doc = "0 The passive level is 0."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The passive level is 1."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T12_SPEC;
impl crate::sealed::RegSpec for T12_SPEC {
    type DataType = u32;
}
#[doc = "Timer T12 Counter Register\n resetvalue={Application Reset:0x0}"]
pub type T12 = crate::RegValueT<T12_SPEC>;

impl T12 {
    #[doc = "Timer 12 Counter Value   T12CV. This register represents the 16 bit counter value of Timer12."]
    #[inline(always)]
    pub fn t12cv(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, T12_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, T12_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T12 {
    #[inline(always)]
    fn default() -> T12 {
        <crate::RegValueT<T12_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T12Dtc_SPEC;
impl crate::sealed::RegSpec for T12Dtc_SPEC {
    type DataType = u32;
}
#[doc = "Dead Time Control Register for Timer12\n resetvalue={Application Reset:0x0}"]
pub type T12Dtc = crate::RegValueT<T12Dtc_SPEC>;

impl T12Dtc {
    #[doc = "Dead Time   DTM. Bit field DTM determines the programmable delay between switching from        the passive state to the active state of the selected outputs. The        switching from the active state to the passive state is not delayed."]
    #[inline(always)]
    pub fn dtm(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, T12Dtc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, T12Dtc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Dead Time Enable Bits DTEx  x 0 1 2  . Bits DTE0  8230 DTE2 enable and disable the dead time generation for each        compare channel  0  1  2  of timer T12."]
    #[inline(always)]
    pub fn dte0(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, t12dtc::Dte0, T12Dtc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,t12dtc::Dte0, T12Dtc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Dead Time Enable Bits DTEx  x 0 1 2  . Bits DTE0  8230 DTE2 enable and disable the dead time generation for each        compare channel  0  1  2  of timer T12."]
    #[inline(always)]
    pub fn dte1(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, t12dtc::Dte1, T12Dtc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,t12dtc::Dte1, T12Dtc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Dead Time Enable Bits DTEx  x 0 1 2  . Bits DTE0  8230 DTE2 enable and disable the dead time generation for each        compare channel  0  1  2  of timer T12."]
    #[inline(always)]
    pub fn dte2(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, t12dtc::Dte2, T12Dtc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,t12dtc::Dte2, T12Dtc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Dead Time Run Indication Bits DTRx  x 1 2 3 . Bits DTR0  8230 DTR2 indicate the status of the dead time generation for each        compare channel  0  1  2  of timer T12."]
    #[inline(always)]
    pub fn dtr0(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, t12dtc::Dtr0, T12Dtc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<12,0x1,1,0,t12dtc::Dtr0, T12Dtc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Dead Time Run Indication Bits DTRx  x 1 2 3 . Bits DTR0  8230 DTR2 indicate the status of the dead time generation for each        compare channel  0  1  2  of timer T12."]
    #[inline(always)]
    pub fn dtr1(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, t12dtc::Dtr1, T12Dtc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<13,0x1,1,0,t12dtc::Dtr1, T12Dtc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Dead Time Run Indication Bits DTRx  x 1 2 3 . Bits DTR0  8230 DTR2 indicate the status of the dead time generation for each        compare channel  0  1  2  of timer T12."]
    #[inline(always)]
    pub fn dtr2(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, t12dtc::Dtr2, T12Dtc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<14,0x1,1,0,t12dtc::Dtr2, T12Dtc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for T12Dtc {
    #[inline(always)]
    fn default() -> T12Dtc {
        <crate::RegValueT<T12Dtc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod t12dtc {
    pub struct Dte0_SPEC;
    pub type Dte0 = crate::EnumBitfieldStruct<u8, Dte0_SPEC>;
    impl Dte0 {
        #[doc = "0 Dead Time        Counter x is disabled. The corresponding outputs switch from the passive        state to the active state  according to the actual compare status         without any delay."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Dead Time        Counter x is enabled. The corresponding outputs switch from the passive        state to the active state  according to the compare status  with the        delay programmed in bit field DTM."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Dte1_SPEC;
    pub type Dte1 = crate::EnumBitfieldStruct<u8, Dte1_SPEC>;
    impl Dte1 {
        #[doc = "0 Dead Time        Counter x is disabled. The corresponding outputs switch from the passive        state to the active state  according to the actual compare status         without any delay."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Dead Time        Counter x is enabled. The corresponding outputs switch from the passive        state to the active state  according to the compare status  with the        delay programmed in bit field DTM."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Dte2_SPEC;
    pub type Dte2 = crate::EnumBitfieldStruct<u8, Dte2_SPEC>;
    impl Dte2 {
        #[doc = "0 Dead Time        Counter x is disabled. The corresponding outputs switch from the passive        state to the active state  according to the actual compare status         without any delay."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Dead Time        Counter x is enabled. The corresponding outputs switch from the passive        state to the active state  according to the compare status  with the        delay programmed in bit field DTM."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Dtr0_SPEC;
    pub type Dtr0 = crate::EnumBitfieldStruct<u8, Dtr0_SPEC>;
    impl Dtr0 {
        #[doc = "0 Dead Time        Counter x is currently in the passive state."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Dead Time        Counter x is currently in the active state."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Dtr1_SPEC;
    pub type Dtr1 = crate::EnumBitfieldStruct<u8, Dtr1_SPEC>;
    impl Dtr1 {
        #[doc = "0 Dead Time        Counter x is currently in the passive state."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Dead Time        Counter x is currently in the active state."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Dtr2_SPEC;
    pub type Dtr2 = crate::EnumBitfieldStruct<u8, Dtr2_SPEC>;
    impl Dtr2 {
        #[doc = "0 Dead Time        Counter x is currently in the passive state."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Dead Time        Counter x is currently in the active state."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T12Msel_SPEC;
impl crate::sealed::RegSpec for T12Msel_SPEC {
    type DataType = u32;
}
#[doc = "T12 Mode Select Register\n resetvalue={Application Reset:0x0}"]
pub type T12Msel = crate::RegValueT<T12Msel_SPEC>;

impl T12Msel {
    #[doc = "Capture Compare Mode Selection MSEL6x  x 0 1 2 . These bit fields select the operating mode of the three T12        capture compare channels. Each channel  x   0  1  2  can be programmed        individually for one of these modes  except for Hall Sensor Mode .        Coding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn msel60(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, T12Msel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, T12Msel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Capture Compare Mode Selection MSEL6x  x 0 1 2 . These bit fields select the operating mode of the three T12        capture compare channels. Each channel  x   0  1  2  can be programmed        individually for one of these modes  except for Hall Sensor Mode .        Coding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn msel61(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, T12Msel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, T12Msel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Capture Compare Mode Selection MSEL6x  x 0 1 2 . These bit fields select the operating mode of the three T12        capture compare channels. Each channel  x   0  1  2  can be programmed        individually for one of these modes  except for Hall Sensor Mode .        Coding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn msel62(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, T12Msel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, T12Msel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Hall Synchronization   HSYNC. Bit field HSYNC defines the source for the sampling of the Hall input        pattern and the comparison to the current and the expected Hall pattern        bit fields. Coding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn hsync(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, T12Msel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8, T12Msel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Delay Bypass   DBYP. DBYP controls whether the source signal for the sampling of the Hall        input pattern  selected by HSYNC  is delayed by the Dead Time Counter 0."]
    #[inline(always)]
    pub fn dbyp(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, t12msel::Dbyp, T12Msel_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,t12msel::Dbyp, T12Msel_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T12Msel {
    #[inline(always)]
    fn default() -> T12Msel {
        <crate::RegValueT<T12Msel_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod t12msel {
    pub struct Dbyp_SPEC;
    pub type Dbyp = crate::EnumBitfieldStruct<u8, Dbyp_SPEC>;
    impl Dbyp {
        #[doc = "0 The bypass is        not active. Dead Time Counter  160 0 is generating a delay after the source        signal becomes active."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bypass is        active. Dead Time Counter  160 0 is not used for a delay."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T12Pr_SPEC;
impl crate::sealed::RegSpec for T12Pr_SPEC {
    type DataType = u32;
}
#[doc = "Timer 12 Period Register\n resetvalue={Application Reset:0x0}"]
pub type T12Pr = crate::RegValueT<T12Pr_SPEC>;

impl T12Pr {
    #[doc = "T12 Period Value   T12PV. The value T12PV defines the counter value for T12 leading to a        period match. When reaching this value  the timer T12 is set to zero         edge aligned mode  or changes its count direction to down counting         center aligned mode ."]
    #[inline(always)]
    pub fn t12pv(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, T12Pr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, T12Pr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T12Pr {
    #[inline(always)]
    fn default() -> T12Pr {
        <crate::RegValueT<T12Pr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T13_SPEC;
impl crate::sealed::RegSpec for T13_SPEC {
    type DataType = u32;
}
#[doc = "Timer T13 Counter Register\n resetvalue={Application Reset:0x0}"]
pub type T13 = crate::RegValueT<T13_SPEC>;

impl T13 {
    #[doc = "Timer 13 Counter Value   T13CV. This register represents the 16 bit counter value of Timer13."]
    #[inline(always)]
    pub fn t13cv(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, T13_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, T13_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T13 {
    #[inline(always)]
    fn default() -> T13 {
        <crate::RegValueT<T13_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T13Pr_SPEC;
impl crate::sealed::RegSpec for T13Pr_SPEC {
    type DataType = u32;
}
#[doc = "Timer 13 Period Register\n resetvalue={Application Reset:0x0}"]
pub type T13Pr = crate::RegValueT<T13Pr_SPEC>;

impl T13Pr {
    #[doc = "T13 Period Value   T13PV. The value T13PV defines the counter value for T13 leading to a period match. When reaching this value  the timer T13 is set to zero."]
    #[inline(always)]
    pub fn t13pv(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, T13Pr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, T13Pr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T13Pr {
    #[inline(always)]
    fn default() -> T13Pr {
        <crate::RegValueT<T13Pr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tctr0_SPEC;
impl crate::sealed::RegSpec for Tctr0_SPEC {
    type DataType = u32;
}
#[doc = "Timer Control Register 0\n resetvalue={Application Reset:0x0}"]
pub type Tctr0 = crate::RegValueT<Tctr0_SPEC>;

impl Tctr0 {
    #[doc = "Timer T12 Input Clock Select   T12CLK. Selects the input clock for timer T12 that is derived from the peripheral clock according to the equation f T12   f CC6   2  lt T12CLK gt  ."]
    #[inline(always)]
    pub fn t12clk(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, tctr0::T12Clk, Tctr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,tctr0::T12Clk, Tctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T12 Prescaler Bit   T12PRE. In order to support higher clock frequencies  an additional prescaler factor of 1 256 can be enabled for the prescaler for T12."]
    #[inline(always)]
    pub fn t12pre(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, tctr0::T12Pre, Tctr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,tctr0::T12Pre, Tctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T12 Run Bit   T12R. T12R starts and stops timer T12. It is set cleared by SW by setting bits T12RR or T12RS or it is cleared by HW according to the function defined by bit field T12SSC."]
    #[inline(always)]
    pub fn t12r(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, tctr0::T12R, Tctr0_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x1,1,0,tctr0::T12R, Tctr0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Timer T12 Shadow Transfer Enable   STE12. Bit STE12 enables or disables the shadow transfer of the T12 period value  the compare values and passive state select bits and levels from their shadow registers to the actual registers if a T12 shadow transfer event is detected. Bit STE12 is cleared by hardware after the shadow transfer. A T12 shadow transfer event is a period match while counting up or a one match while counting down."]
    #[inline(always)]
    pub fn ste12(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, tctr0::Ste12, Tctr0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x1,1,0,tctr0::Ste12, Tctr0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Count Direction of Timer T12   CDIR. This bit is set cleared according to the counting rules of T12."]
    #[inline(always)]
    pub fn cdir(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, tctr0::Cdir, Tctr0_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x1,1,0,tctr0::Cdir, Tctr0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "T12 Operating Mode   CTM"]
    #[inline(always)]
    pub fn ctm(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, tctr0::Ctm, Tctr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,tctr0::Ctm, Tctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T13 Input Clock Select   T13CLK. Selects the input clock for timer T13 that is derived from the peripheral clock according to the equation f T13   f CC6   2  lt T13CLK gt  ."]
    #[inline(always)]
    pub fn t13clk(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, tctr0::T13Clk, Tctr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x7,1,0,tctr0::T13Clk, Tctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T13 Prescaler Bit   T13PRE. In order to support higher clock frequencies  an additional prescaler factor of 1 256 can be enabled for the prescaler for T13."]
    #[inline(always)]
    pub fn t13pre(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, tctr0::T13Pre, Tctr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,tctr0::T13Pre, Tctr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T13 Run Bit   T13R. T13R starts and stops timer T13. It is set cleared by SW by setting bits T13RR orT13RS or it is set cleared by HW according to the function defined by bit fields T13SSC  T13TEC and T13TED."]
    #[inline(always)]
    pub fn t13r(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, tctr0::T13R, Tctr0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<12,0x1,1,0,tctr0::T13R, Tctr0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Timer T13 Shadow Transfer Enable   STE13. Bit STE13 enables or disables the shadow transfer of the T13 period value  the compare value and passive state select bit and level from their shadow registers to the actual registers if a T13 shadow transfer event is detected. Bit STE13 is cleared by hardware after the shadow transfer. A T13 shadow transfer event is a period match."]
    #[inline(always)]
    pub fn ste13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, tctr0::Ste13, Tctr0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<13,0x1,1,0,tctr0::Ste13, Tctr0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Tctr0 {
    #[inline(always)]
    fn default() -> Tctr0 {
        <crate::RegValueT<Tctr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tctr0 {
    pub struct T12Clk_SPEC;
    pub type T12Clk = crate::EnumBitfieldStruct<u8, T12Clk_SPEC>;
    impl T12Clk {
        #[doc = "000 f T12   f CC6"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 f T12   f CC6   2"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 f T12   f CC6   4"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 f T12   f CC6   8"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 f T12   f CC6   16"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 f T12   f CC6   32"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 f T12   f CC6   64"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "111 f T12   f CC6   128"]
        pub const CONST_77: Self = Self::new(7);
    }
    pub struct T12Pre_SPEC;
    pub type T12Pre = crate::EnumBitfieldStruct<u8, T12Pre_SPEC>;
    impl T12Pre {
        #[doc = "0 The additional prescaler for T12 is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The additional prescaler for T12 is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T12R_SPEC;
    pub type T12R = crate::EnumBitfieldStruct<u8, T12R_SPEC>;
    impl T12R {
        #[doc = "0 Timer T12 is stopped."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer T12 is running."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ste12_SPEC;
    pub type Ste12 = crate::EnumBitfieldStruct<u8, Ste12_SPEC>;
    impl Ste12 {
        #[doc = "0 The shadow register transfer is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The shadow register transfer is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cdir_SPEC;
    pub type Cdir = crate::EnumBitfieldStruct<u8, Cdir_SPEC>;
    impl Cdir {
        #[doc = "0 T12 counts up."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 T12 counts down."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ctm_SPEC;
    pub type Ctm = crate::EnumBitfieldStruct<u8, Ctm_SPEC>;
    impl Ctm {
        #[doc = "0 Edge aligned Mode  T12 always counts up and continues counting from zero after reaching the period value."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Center aligned Mode  T12 counts down after detecting a period match and counts up after detecting a one match."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T13Clk_SPEC;
    pub type T13Clk = crate::EnumBitfieldStruct<u8, T13Clk_SPEC>;
    impl T13Clk {
        #[doc = "000 f T13   f CC6"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 f T13   f CC6   2"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 f T13   f CC6   4"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 f T13   f CC6   8"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 f T13   f CC6   16"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 f T13   f CC6   32"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 f T13   f CC6   64"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "111 f T13   f CC6   128"]
        pub const CONST_77: Self = Self::new(7);
    }
    pub struct T13Pre_SPEC;
    pub type T13Pre = crate::EnumBitfieldStruct<u8, T13Pre_SPEC>;
    impl T13Pre {
        #[doc = "0 The additional prescaler for T13 is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The additional prescaler for T13 is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T13R_SPEC;
    pub type T13R = crate::EnumBitfieldStruct<u8, T13R_SPEC>;
    impl T13R {
        #[doc = "0 Timer T13 is stopped."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer T13 is running."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ste13_SPEC;
    pub type Ste13 = crate::EnumBitfieldStruct<u8, Ste13_SPEC>;
    impl Ste13 {
        #[doc = "0 The shadow register transfer is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The shadow register transfer is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tctr2_SPEC;
impl crate::sealed::RegSpec for Tctr2_SPEC {
    type DataType = u32;
}
#[doc = "Timer Control Register 2\n resetvalue={Application Reset:0x0}"]
pub type Tctr2 = crate::RegValueT<Tctr2_SPEC>;

impl Tctr2 {
    #[doc = "Timer T12 Single Shot Control   T12SSC. This bit controls the single shot mode of T12."]
    #[inline(always)]
    pub fn t12ssc(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, tctr2::T12Ssc, Tctr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,tctr2::T12Ssc, Tctr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T13 Single Shot Control   T13SSC. This bit controls the single shot mode of T13."]
    #[inline(always)]
    pub fn t13ssc(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, tctr2::T13Ssc, Tctr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,tctr2::T13Ssc, Tctr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "T13 Trigger Event Control   T13TEC. Bit field T13TEC selects the trigger event to start T13  automatic set of T13R for synchronization to T12 compare signals  according to following combinations"]
    #[inline(always)]
    pub fn t13tec(
        self,
    ) -> crate::common::RegisterField<2, 0x7, 1, 0, tctr2::T13Tec, Tctr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x7,1,0,tctr2::T13Tec, Tctr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T13 Trigger Event Direction   T13TED. Bit field T13TED delivers additional information to control the automatic set of bit T13R in the case that the trigger action defined by T13TEC is detected."]
    #[inline(always)]
    pub fn t13ted(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, tctr2::T13Ted, Tctr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,tctr2::T13Ted, Tctr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T12 External Run Selection   T12RSEL. Bit field T12RSEL defines the event of signal T12HR that can set the run bit T12R by HW."]
    #[inline(always)]
    pub fn t12rsel(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, tctr2::T12Rsel, Tctr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,tctr2::T12Rsel, Tctr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T13 External Run Selection   T13RSEL. Bit field T13RSEL defines the event of signal T13HR that can set the run bit T13R by HW."]
    #[inline(always)]
    pub fn t13rsel(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, tctr2::T13Rsel, Tctr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,tctr2::T13Rsel, Tctr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Tctr2 {
    #[inline(always)]
    fn default() -> Tctr2 {
        <crate::RegValueT<Tctr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tctr2 {
    pub struct T12Ssc_SPEC;
    pub type T12Ssc = crate::EnumBitfieldStruct<u8, T12Ssc_SPEC>;
    impl T12Ssc {
        #[doc = "0 The single shot mode is disabled  no HW action on T12R."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The single shot mode is enabled  the bit T12R is cleared by HW if   T12 reaches its period value in edge aligned mode   T12 reaches the value 1 while down counting in center aligned mode. In parallel to the clear action of bit T12R  the bits CC6xST  x 0  1  2  are cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T13Ssc_SPEC;
    pub type T13Ssc = crate::EnumBitfieldStruct<u8, T13Ssc_SPEC>;
    impl T13Ssc {
        #[doc = "0 No HW action on T13R"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The single shot mode is enabled  the bit T13R is cleared by HW if T13 reaches its period value. In parallel to the clear action of bit T13R  the bit CC63ST is cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T13Tec_SPEC;
    pub type T13Tec = crate::EnumBitfieldStruct<u8, T13Tec_SPEC>;
    impl T13Tec {
        #[doc = "000 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 Set T13R on a T12 compare event on channel 0"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 Set T13R on a T12 compare event on channel 1"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 Set T13R on a T12 compare event on channel 2"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 Set T13R on any T12 compare event  ch. 0  1  2"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 Set T13R upon a period match of T12"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 Set T13R upon a zero match of T12  while counting up"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "111 Set T13R on any edge of inputs CCPOSx"]
        pub const CONST_77: Self = Self::new(7);
    }
    pub struct T13Ted_SPEC;
    pub type T13Ted = crate::EnumBitfieldStruct<u8, T13Ted_SPEC>;
    impl T13Ted {
        #[doc = "01 While T12 is counting up"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 While T12 is counting down"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Independent on the count direction of T12"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct T12Rsel_SPEC;
    pub type T12Rsel = crate::EnumBitfieldStruct<u8, T12Rsel_SPEC>;
    impl T12Rsel {
        #[doc = "00 The external setting of T12R is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Bit T12R is set if a rising edge of signal T12HR is detected."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Bit T12R is set if a falling edge of signal T12HR is detected."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Bit T12R is set if an edge of signal T12HR is detected."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct T13Rsel_SPEC;
    pub type T13Rsel = crate::EnumBitfieldStruct<u8, T13Rsel_SPEC>;
    impl T13Rsel {
        #[doc = "00 The external setting of T13R is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Bit T13R is set if a rising edge of signal T13HR is detected."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Bit T13R is set if a falling edge of signal T13HR is detected."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Bit T13R is set if an edge of signal T13HR is detected."]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tctr4_SPEC;
impl crate::sealed::RegSpec for Tctr4_SPEC {
    type DataType = u32;
}
#[doc = "Timer Control Register 4\n resetvalue={Application Reset:0x0}"]
pub type Tctr4 = crate::RegValueT<Tctr4_SPEC>;

impl Tctr4 {
    #[doc = "Timer T12 Run Reset   T12RR. Setting this bit clears the T12R bit."]
    #[inline(always)]
    pub fn t12rr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, tctr4::T12Rr, Tctr4_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x1,1,0,tctr4::T12Rr, Tctr4_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Timer T12 Run Set   T12RS. Setting this bit sets the T12R bit."]
    #[inline(always)]
    pub fn t12rs(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, tctr4::T12Rs, Tctr4_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<1,0x1,1,0,tctr4::T12Rs, Tctr4_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Timer T12 Reset   T12RES"]
    #[inline(always)]
    pub fn t12res(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, tctr4::T12Res, Tctr4_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<2,0x1,1,0,tctr4::T12Res, Tctr4_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Dead Time Counter Reset   DTRES"]
    #[inline(always)]
    pub fn dtres(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, tctr4::Dtres, Tctr4_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<3,0x1,1,0,tctr4::Dtres, Tctr4_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Timer T12 Count Event   T12CNT"]
    #[inline(always)]
    pub fn t12cnt(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, tctr4::T12Cnt, Tctr4_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<5,0x1,1,0,tctr4::T12Cnt, Tctr4_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Timer T12 Shadow Transfer Request   T12STR"]
    #[inline(always)]
    pub fn t12str(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, tctr4::T12Str, Tctr4_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<6,0x1,1,0,tctr4::T12Str, Tctr4_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Timer T12 Shadow Transfer Disable   T12STD"]
    #[inline(always)]
    pub fn t12std(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, tctr4::T12Std, Tctr4_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<7,0x1,1,0,tctr4::T12Std, Tctr4_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Timer T13 Run Reset   T13RR. Setting this bit clears the T13R bit."]
    #[inline(always)]
    pub fn t13rr(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, tctr4::T13Rr, Tctr4_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0x1,1,0,tctr4::T13Rr, Tctr4_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Timer T13 Run Set   T13RS. Setting this bit sets the T13R bit."]
    #[inline(always)]
    pub fn t13rs(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, tctr4::T13Rs, Tctr4_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<9,0x1,1,0,tctr4::T13Rs, Tctr4_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Timer T13 Reset   T13RES"]
    #[inline(always)]
    pub fn t13res(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, tctr4::T13Res, Tctr4_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<10,0x1,1,0,tctr4::T13Res, Tctr4_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Timer T13 Count Event   T13CNT"]
    #[inline(always)]
    pub fn t13cnt(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, tctr4::T13Cnt, Tctr4_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<13,0x1,1,0,tctr4::T13Cnt, Tctr4_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Timer T13 Shadow Transfer Request   T13STR"]
    #[inline(always)]
    pub fn t13str(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, tctr4::T13Str, Tctr4_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<14,0x1,1,0,tctr4::T13Str, Tctr4_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Timer T13 Shadow Transfer Disable   T13STD"]
    #[inline(always)]
    pub fn t13std(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, tctr4::T13Std, Tctr4_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<15,0x1,1,0,tctr4::T13Std, Tctr4_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Tctr4 {
    #[inline(always)]
    fn default() -> Tctr4 {
        <crate::RegValueT<Tctr4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tctr4 {
    pub struct T12Rr_SPEC;
    pub type T12Rr = crate::EnumBitfieldStruct<u8, T12Rr_SPEC>;
    impl T12Rr {
        #[doc = "0 T12R is not influenced."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 T12R is cleared  T12 stops counting."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T12Rs_SPEC;
    pub type T12Rs = crate::EnumBitfieldStruct<u8, T12Rs_SPEC>;
    impl T12Rs {
        #[doc = "0 T12R is not influenced."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 T12R is set  T12 starts counting."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T12Res_SPEC;
    pub type T12Res = crate::EnumBitfieldStruct<u8, T12Res_SPEC>;
    impl T12Res {
        #[doc = "0 No effect on T12."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The T12 counter register is cleared to zero. The switching of the output signals is according to the switching rules. Setting of T12RES has no impact on bit T12R."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Dtres_SPEC;
    pub type Dtres = crate::EnumBitfieldStruct<u8, Dtres_SPEC>;
    impl Dtres {
        #[doc = "0 No effect on the dead time counters."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The three dead time counter channels are cleared to zero."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T12Cnt_SPEC;
    pub type T12Cnt = crate::EnumBitfieldStruct<u8, T12Cnt_SPEC>;
    impl T12Cnt {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 If enabled  PISEL2   timer T12 counts one step."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T12Str_SPEC;
    pub type T12Str = crate::EnumBitfieldStruct<u8, T12Str_SPEC>;
    impl T12Str {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 STE12 is set  enabling the shadow transfer."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T12Std_SPEC;
    pub type T12Std = crate::EnumBitfieldStruct<u8, T12Std_SPEC>;
    impl T12Std {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 STE12 is cleared without triggering the shadow transfer."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T13Rr_SPEC;
    pub type T13Rr = crate::EnumBitfieldStruct<u8, T13Rr_SPEC>;
    impl T13Rr {
        #[doc = "0 T13R is not influenced."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 T13R is cleared  T13 stops counting."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T13Rs_SPEC;
    pub type T13Rs = crate::EnumBitfieldStruct<u8, T13Rs_SPEC>;
    impl T13Rs {
        #[doc = "0 T13R is not influenced."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 T13R is set  T13 starts counting."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T13Res_SPEC;
    pub type T13Res = crate::EnumBitfieldStruct<u8, T13Res_SPEC>;
    impl T13Res {
        #[doc = "0 No effect on T13."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The T13 counter register is cleared to zero. The switching of the output signals is according to the switching rules. Setting of T13RES has no impact on bit T13R."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T13Cnt_SPEC;
    pub type T13Cnt = crate::EnumBitfieldStruct<u8, T13Cnt_SPEC>;
    impl T13Cnt {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 If enabled  PISEL2   timer T13 counts one step."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T13Str_SPEC;
    pub type T13Str = crate::EnumBitfieldStruct<u8, T13Str_SPEC>;
    impl T13Str {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 STE13 is set  enabling the shadow transfer."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T13Std_SPEC;
    pub type T13Std = crate::EnumBitfieldStruct<u8, T13Std_SPEC>;
    impl T13Std {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 STE13 is cleared without triggering the shadow transfer."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trpctr_SPEC;
impl crate::sealed::RegSpec for Trpctr_SPEC {
    type DataType = u32;
}
#[doc = "Trap Control Register\n resetvalue={Application Reset:0x0}"]
pub type Trpctr = crate::RegValueT<Trpctr_SPEC>;

impl Trpctr {
    #[doc = "Trap Mode Control Bit 0   TRPM0. Together with bit TRPM1  these two bits define the behavior of the        selected outputs when leaving the trap state after the trap condition        has become inactive again. A synchronization to the timer driving the        PWM pattern avoids unintended pulses when leaving the trap state. The        behavior resulting from the combination  TRPM1  TRPM0  is described in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn trpm0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Trpctr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Trpctr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Trap Mode Control Bit 1   TRPM1. Together with bit TRPM0  these two bits define the behavior of the        selected outputs when leaving the trap state after the trap condition        has become inactive again. A synchronization to the timer driving the        PWM pattern avoids unintended pulses when leaving the trap state. The        behavior resulting from the combination  TRPM1  TRPM0  is described in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn trpm1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Trpctr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Trpctr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Trap Mode Control Bit 2   TRPM2. This bit defines how the trap flag TRPF can be cleared after the trap        input condition   CTRAP   160    160 0        and TRPPEN  160    160 1  is no longer valid  either by CTRAP   160    160 1        or by TRPPEN  160    160 0 ."]
    #[inline(always)]
    pub fn trpm2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, trpctr::Trpm2, Trpctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,trpctr::Trpm2, Trpctr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trap Enable Control   TRPEN. Setting a bit enables the trap functionality for the following corresponding output signals  TRPEN0   TRPCTR.8 for output CC60 TRPEN1   TRPCTR.9 for output COUT60 TRPEN2   TRPCTR.10 for output CC61 TRPEN3   TRPCTR.11 for output COUT61 TRPEN4   TRPCTR.12 for output CC62 TRPEN5   TRPCTR.13 for output COUT62"]
    #[inline(always)]
    pub fn trpen(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, trpctr::Trpen, Trpctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3f,1,0,trpctr::Trpen, Trpctr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trap Enable Control for Timer T13   TRPEN13"]
    #[inline(always)]
    pub fn trpen13(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, trpctr::Trpen13, Trpctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,trpctr::Trpen13, Trpctr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trap Pin Enable   TRPPEN. This bit enables the input  pin  function for the trap generation. An interrupt can only be generated if a falling edge is detected at pin CTRAP while TRPPEN   1."]
    #[inline(always)]
    pub fn trppen(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, trpctr::Trppen, Trpctr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,trpctr::Trppen, Trpctr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Trpctr {
    #[inline(always)]
    fn default() -> Trpctr {
        <crate::RegValueT<Trpctr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod trpctr {
    pub struct Trpm2_SPEC;
    pub type Trpm2 = crate::EnumBitfieldStruct<u8, Trpm2_SPEC>;
    impl Trpm2 {
        #[doc = "0 Automatic Mode  Bit TRPF is cleared by HW if the trap input condition is no longer valid."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Manual Mode  Bit TRPF stays 1 after the trap input condition is no longer valid. It has to be cleared by SW by writing ISR.RTRPF   1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trpen_SPEC;
    pub type Trpen = crate::EnumBitfieldStruct<u8, Trpen_SPEC>;
    impl Trpen {
        #[doc = "0 The trap functionality of the corresponding output signal is disabled. The output state is independent from bit IS.TRPS."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The trap functionality of the corresponding output signal is enabled. The output state is set to the passive while IS.TRPS 1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trpen13_SPEC;
    pub type Trpen13 = crate::EnumBitfieldStruct<u8, Trpen13_SPEC>;
    impl Trpen13 {
        #[doc = "0 The trap functionality for output COUT63 is disabled. The output state is independent from bit IS.TRPS."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The trap functionality for output COUT63 is enabled. The output state is set to the passive while IS.TRPS 1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trppen_SPEC;
    pub type Trppen = crate::EnumBitfieldStruct<u8, Trppen_SPEC>;
    impl Trppen {
        #[doc = "0 The CCU6 trap functionality based on the input CTRAP is disabled. A CCU6 trap can only be generated by SW by setting bit TRPF."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The CCU6 trap functionality based on the input CTRAP is enabled. A CCU6 trap can be generated by SW by setting bit TRPF or by CTRAP  0."]
        pub const CONST_11: Self = Self::new(1);
    }
}
