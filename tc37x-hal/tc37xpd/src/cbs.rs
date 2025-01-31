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
#[doc = r"CBS"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cbs(pub(super) *mut u8);
unsafe impl core::marker::Send for Cbs {}
unsafe impl core::marker::Sync for Cbs {}
impl Cbs {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(508usize)) }
    }

    #[doc = "Communication Mode Data Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn comdata(&self) -> crate::common::Reg<self::Comdata_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(104usize)) }
    }

    #[doc = "Internally Controlled Trace Source Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn ictsa(&self) -> crate::common::Reg<self::Ictsa_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }

    #[doc = "Internally Controlled Trace Destination Register\n resetvalue={PowerOn Reset:0x10F068,Application Reset:0x10F068,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn ictta(&self) -> crate::common::Reg<self::Ictta_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(140usize)) }
    }

    #[doc = "IFS Address Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ifsa(&self) -> crate::common::Reg<self::Ifsa_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(240usize)) }
    }

    #[doc = "IFS Control Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ifsc(&self) -> crate::common::Reg<self::Ifsc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(244usize)) }
    }

    #[doc = "Internal Mode Status and Control Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn intmod(&self) -> crate::common::Reg<self::Intmod_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }

    #[doc = "IOClientStatus and Control Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn iosr(&self) -> crate::common::Reg<self::Iosr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(108usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={PowerOn Reset:0x6360,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn jdpid(&self) -> crate::common::Reg<self::Jdpid_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "JTAGDevice Identification Register\n resetvalue={PowerOn Reset:0x0,CFS Value:0x10207083,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn jtagid(&self) -> crate::common::Reg<self::Jtagid_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(100usize)) }
    }

    #[doc = "OSCU Control Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn ocntrl(&self) -> crate::common::Reg<self::Ocntrl_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(124usize)) }
    }

    #[doc = "OCDS Enable Control Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn oec(&self) -> crate::common::Reg<self::Oec_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(120usize)) }
    }

    #[doc = "OCDS Interface Mode Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn oifm(&self) -> crate::common::Reg<self::Oifm_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }

    #[doc = "OSCUStatus Register\n resetvalue={PowerOn Reset:0x10000,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn ostate(&self) -> crate::common::Reg<self::Ostate_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "TG Capture for Cores   BRKOUT\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tccb(&self) -> crate::common::Reg<self::Tccb_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(176usize)) }
    }

    #[doc = "TG Capture for Cores   HALT\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tcch(&self) -> crate::common::Reg<self::Tcch_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(180usize)) }
    }

    #[doc = "TG Capture for TG Input Pins\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tcip(&self) -> crate::common::Reg<self::Tcip_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "TG Capture for MCDS\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tcm(&self) -> crate::common::Reg<self::Tcm_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(188usize)) }
    }

    #[doc = "TG Capture for OTGB0 1\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tctgb(&self) -> crate::common::Reg<self::Tctgb_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(184usize)) }
    }

    #[doc = "TG Capture for TG Lines\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tctl(&self) -> crate::common::Reg<self::Tctl_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(116usize)) }
    }

    #[doc = "TG Input Pins Routing\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tipr(&self) -> crate::common::Reg<self::Tipr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "TG Line 1 Suspend Targets\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tl1st(&self) -> crate::common::Reg<self::Tl1St_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(148usize)) }
    }

    #[doc = "TG Line Control\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tlc(&self) -> crate::common::Reg<self::Tlc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }

    #[doc = "TG Line Counter Control\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tlccx(&self) -> [crate::common::Reg<self::TlcCx_SPEC, crate::common::RW>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x40usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x40usize + 0x4usize)),
            ]
        }
    }

    #[doc = "TG Line Capture and Hold Enable\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tlche(&self) -> crate::common::Reg<self::Tlche_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(152usize)) }
    }

    #[doc = "TG Line Capture and Hold Clear\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tlchs(&self) -> crate::common::Reg<self::Tlchs_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(156usize)) }
    }

    #[doc = "TG Line Counter Value\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tlcvx(&self) -> [crate::common::Reg<self::TlcVx_SPEC, crate::common::R>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x50usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x50usize + 0x4usize)),
            ]
        }
    }

    #[doc = "TG Line State\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tls(&self) -> crate::common::Reg<self::Tls_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(112usize)) }
    }

    #[doc = "TG Line Timer\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tlt(&self) -> crate::common::Reg<self::Tlt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(168usize)) }
    }

    #[doc = "TG Lines for Trigger to Host\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn tltth(&self) -> crate::common::Reg<self::Tltth_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(172usize)) }
    }

    #[doc = "TG Output Pins Pulse Stretcher\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn topps(&self) -> crate::common::Reg<self::Topps_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }

    #[doc = "TG Output Pins Routing\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn topr(&self) -> crate::common::Reg<self::Topr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "TG Routing for CPU0\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x2,Debug Reset:0x0,Application Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn trcx(&self) -> [crate::common::Reg<self::TrCx_SPEC, crate::common::RW>; 6] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x20usize + 0x14usize)),
            ]
        }
    }

    #[doc = "TG Routing Events of CPU0\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn trecx(&self) -> [crate::common::Reg<self::TreCx_SPEC, crate::common::RW>; 6] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0xc0usize + 0x14usize)),
            ]
        }
    }

    #[doc = "TG Routing for HSMControl\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn trhsm(&self) -> crate::common::Reg<self::Trhsm_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }

    #[doc = "Clear Trigger to Host Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn trigc(&self) -> crate::common::Reg<self::Trigc_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(164usize)) }
    }

    #[doc = "Set Trigger to Host Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn trigs(&self) -> crate::common::Reg<self::Trigs_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }

    #[doc = "Trigger to Host Register 0\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
    #[inline(always)]
    pub const fn trigx(&self) -> [crate::common::Reg<self::TriGx_SPEC, crate::common::R>; 6] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x100usize + 0x14usize)),
            ]
        }
    }

    #[doc = "TG Routing for MCDS Control\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn trmc(&self) -> crate::common::Reg<self::Trmc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "TG Routing for MCDS Triggers\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn trmt(&self) -> crate::common::Reg<self::Trmt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(220usize)) }
    }

    #[doc = "TG Routing for Special Signals\n resetvalue={Debug Reset:0x0EF0000}"]
    #[inline(always)]
    pub const fn trss(&self) -> crate::common::Reg<self::Trss_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }
    #[doc = "TRTGB"]
    #[inline(always)]
    pub fn trtgb(self) -> [self::Trtgb; 2] {
        unsafe {
            [
                self::Trtgb(self.0.add(0xe0usize + 0x0usize)),
                self::Trtgb(self.0.add(0xe0usize + 0x8usize)),
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
pub struct Comdata_SPEC;
impl crate::sealed::RegSpec for Comdata_SPEC {
    type DataType = u32;
}
#[doc = "Communication Mode Data Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Comdata = crate::RegValueT<Comdata_SPEC>;

impl Comdata {
    #[doc = "Read Write Data   DATA. Data transferred by read write access executed by Cerberus in Communication mode."]
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Comdata_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Comdata_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Comdata {
    #[inline(always)]
    fn default() -> Comdata {
        <crate::RegValueT<Comdata_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ictsa_SPEC;
impl crate::sealed::RegSpec for Ictsa_SPEC {
    type DataType = u32;
}
#[doc = "Internally Controlled Trace Source Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Ictsa = crate::RegValueT<Ictsa_SPEC>;

impl Ictsa {
    #[doc = "Source Address   ADDR. This address is used by Cerberus to read data  size depending on INTMOD . TRC  MOD          when a triggered transfer takes place in internal mode."]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Ictsa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Ictsa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ictsa {
    #[inline(always)]
    fn default() -> Ictsa {
        <crate::RegValueT<Ictsa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ictta_SPEC;
impl crate::sealed::RegSpec for Ictta_SPEC {
    type DataType = u32;
}
#[doc = "Internally Controlled Trace Destination Register\n resetvalue={PowerOn Reset:0x10F068,Application Reset:0x10F068,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Ictta = crate::RegValueT<Ictta_SPEC>;

impl Ictta {
    #[doc = "Destination Address   ADDR. This address is used by Cerberus to write data  size depending on INTMOD . TRC  MOD   when a triggered transfer takes place in internal mode."]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Ictta_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Ictta_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ictta {
    #[inline(always)]
    fn default() -> Ictta {
        <crate::RegValueT<Ictta_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifsa_SPEC;
impl crate::sealed::RegSpec for Ifsa_SPEC {
    type DataType = u32;
}
#[doc = "IFS Address Register\n resetvalue={Debug Reset:0x0}"]
pub type Ifsa = crate::RegValueT<Ifsa_SPEC>;

impl Ifsa {
    #[doc = "Address for FI SI Accesses   ADDR. In case of FI the lowest two bits address the byte within the 32  160 bit        word which is used for the RMW access. In case of SI the lowest two bits        are ignored."]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Ifsa_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Ifsa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ifsa {
    #[inline(always)]
    fn default() -> Ifsa {
        <crate::RegValueT<Ifsa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifsc_SPEC;
impl crate::sealed::RegSpec for Ifsc_SPEC {
    type DataType = u32;
}
#[doc = "IFS Control Register\n resetvalue={Debug Reset:0x0}"]
pub type Ifsc = crate::RegValueT<Ifsc_SPEC>;

impl Ifsc {
    #[doc = "Injection Trigger   GO. This bit shall be gated by OCDS enable."]
    #[inline(always)]
    pub fn go(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ifsc::Go, Ifsc_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0x1,1,0,ifsc::Go, Ifsc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Injection Trigger by OTGS   OTGS"]
    #[inline(always)]
    pub fn otgs(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ifsc::Otgs, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,ifsc::Otgs, Ifsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Stress or Fault Injection Mode   MODE"]
    #[inline(always)]
    pub fn mode(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ifsc::Mode, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,ifsc::Mode, Ifsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bus Master Priority   PRIO"]
    #[inline(always)]
    pub fn prio(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ifsc::Prio, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,ifsc::Prio, Ifsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Read Stress Repetitions. The value 0 means just one SI read  the value 15 means 16 reads overall"]
    #[inline(always)]
    pub fn rsrep(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Ifsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault Injection Bit Manipulation   BM7. Controls the manipulation of bit x within the byte addressed by IFSA . The lowest two bits of IFSA determine the byte within the 32 bit word of the RMW operation. All other bytes of the word are unchanged."]
    #[inline(always)]
    pub fn bm0(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, ifsc::Bm0, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,ifsc::Bm0, Ifsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault Injection Bit Manipulation   BM7. Controls the manipulation of bit x within the byte addressed by IFSA . The lowest two bits of IFSA determine the byte within the 32 bit word of the RMW operation. All other bytes of the word are unchanged."]
    #[inline(always)]
    pub fn bm1(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, ifsc::Bm1, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,ifsc::Bm1, Ifsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault Injection Bit Manipulation   BM7. Controls the manipulation of bit x within the byte addressed by IFSA . The lowest two bits of IFSA determine the byte within the 32 bit word of the RMW operation. All other bytes of the word are unchanged."]
    #[inline(always)]
    pub fn bm2(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, ifsc::Bm2, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,ifsc::Bm2, Ifsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault Injection Bit Manipulation   BM7. Controls the manipulation of bit x within the byte addressed by IFSA . The lowest two bits of IFSA determine the byte within the 32 bit word of the RMW operation. All other bytes of the word are unchanged."]
    #[inline(always)]
    pub fn bm3(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, ifsc::Bm3, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,ifsc::Bm3, Ifsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault Injection Bit Manipulation   BM7. Controls the manipulation of bit x within the byte addressed by IFSA . The lowest two bits of IFSA determine the byte within the 32 bit word of the RMW operation. All other bytes of the word are unchanged."]
    #[inline(always)]
    pub fn bm4(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, ifsc::Bm4, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,ifsc::Bm4, Ifsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault Injection Bit Manipulation   BM7. Controls the manipulation of bit x within the byte addressed by IFSA . The lowest two bits of IFSA determine the byte within the 32 bit word of the RMW operation. All other bytes of the word are unchanged."]
    #[inline(always)]
    pub fn bm5(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, ifsc::Bm5, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3,1,0,ifsc::Bm5, Ifsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault Injection Bit Manipulation   BM7. Controls the manipulation of bit x within the byte addressed by IFSA . The lowest two bits of IFSA determine the byte within the 32 bit word of the RMW operation. All other bytes of the word are unchanged."]
    #[inline(always)]
    pub fn bm6(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, ifsc::Bm6, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,ifsc::Bm6, Ifsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fault Injection Bit Manipulation   BM7. Controls the manipulation of bit x within the byte addressed by IFSA . The lowest two bits of IFSA determine the byte within the 32 bit word of the RMW operation. All other bytes of the word are unchanged."]
    #[inline(always)]
    pub fn bm7(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, ifsc::Bm7, Ifsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,ifsc::Bm7, Ifsc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ifsc {
    #[inline(always)]
    fn default() -> Ifsc {
        <crate::RegValueT<Ifsc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ifsc {
    pub struct Go_SPEC;
    pub type Go = crate::EnumBitfieldStruct<u8, Go_SPEC>;
    impl Go {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Single shot"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Otgs_SPEC;
    pub type Otgs = crate::EnumBitfieldStruct<u8, Otgs_SPEC>;
    impl Otgs {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Injection is triggered also by OTGS   TRSS .IFS"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mode_SPEC;
    pub type Mode = crate::EnumBitfieldStruct<u8, Mode_SPEC>;
    impl Mode {
        #[doc = "0 Stress        Injection  SI"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Fault Injection  FI"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Prio_SPEC;
    pub type Prio = crate::EnumBitfieldStruct<u8, Prio_SPEC>;
    impl Prio {
        #[doc = "0 Lowest bus master priority"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Highest priority  recommended for FI"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Bm0_SPEC;
    pub type Bm0 = crate::EnumBitfieldStruct<u8, Bm0_SPEC>;
    impl Bm0 {
        #[doc = "0 Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Inverted"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Set to 0"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 Set to 1"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Bm1_SPEC;
    pub type Bm1 = crate::EnumBitfieldStruct<u8, Bm1_SPEC>;
    impl Bm1 {
        #[doc = "0 Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Inverted"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Set to 0"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 Set to 1"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Bm2_SPEC;
    pub type Bm2 = crate::EnumBitfieldStruct<u8, Bm2_SPEC>;
    impl Bm2 {
        #[doc = "0 Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Inverted"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Set to 0"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 Set to 1"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Bm3_SPEC;
    pub type Bm3 = crate::EnumBitfieldStruct<u8, Bm3_SPEC>;
    impl Bm3 {
        #[doc = "0 Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Inverted"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Set to 0"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 Set to 1"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Bm4_SPEC;
    pub type Bm4 = crate::EnumBitfieldStruct<u8, Bm4_SPEC>;
    impl Bm4 {
        #[doc = "0 Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Inverted"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Set to 0"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 Set to 1"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Bm5_SPEC;
    pub type Bm5 = crate::EnumBitfieldStruct<u8, Bm5_SPEC>;
    impl Bm5 {
        #[doc = "0 Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Inverted"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Set to 0"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 Set to 1"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Bm6_SPEC;
    pub type Bm6 = crate::EnumBitfieldStruct<u8, Bm6_SPEC>;
    impl Bm6 {
        #[doc = "0 Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Inverted"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Set to 0"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 Set to 1"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Bm7_SPEC;
    pub type Bm7 = crate::EnumBitfieldStruct<u8, Bm7_SPEC>;
    impl Bm7 {
        #[doc = "0 Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Inverted"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Set to 0"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 Set to 1"]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intmod_SPEC;
impl crate::sealed::RegSpec for Intmod_SPEC {
    type DataType = u32;
}
#[doc = "Internal Mode Status and Control Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Intmod = crate::RegValueT<Intmod_SPEC>;

impl Intmod {
    #[doc = "Set Read Sync Flag   SET CRS. Used by the monitor program to set the Read Sync Flag in Internal Mode."]
    #[inline(always)]
    pub fn set_crs(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, intmod::SetCrs, Intmod_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x1,1,0,intmod::SetCrs, Intmod_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Write Sync Flag   SET CWS. Used by the monitor program to set the Write Sync Flag in Internal Mode."]
    #[inline(always)]
    pub fn set_cws(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, intmod::SetCws, Intmod_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<1,0x1,1,0,intmod::SetCws, Intmod_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Communication Synchronization Flag   SET CS. Used by the monitor program to set the higher level Communication Mode sync bit in Internal Mode."]
    #[inline(always)]
    pub fn set_cs(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, intmod::SetCs, Intmod_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<2,0x1,1,0,intmod::SetCs, Intmod_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear Communication Synchronization Flag   CLR CS. Used by the monitor program to clear the higher level Communication Mode sync bit in Internal Mode."]
    #[inline(always)]
    pub fn clr_cs(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, intmod::ClrCs, Intmod_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<3,0x1,1,0,intmod::ClrCs, Intmod_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "CHANNEL Write Protection   CHANNEL P. Protect CHA NNEL against unintended changes."]
    #[inline(always)]
    pub fn channel_p(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, intmod::ChannelP, Intmod_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<4,0x1,1,0,intmod::ChannelP, Intmod_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Channel Indication   CHANNEL. These bits are just written into the IOSR . CHAN NEL bit field in Internal Mode when CHANNEL P is held  160   8217 1  8217  concurrently 1  . Upon reading the current setting of IOSR . CHAN NEL is returned. If INT MOD is de asserted this is the value present in CROSSREFERENCE . CHAN NEL"]
    #[inline(always)]
    pub fn channel(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, Intmod_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8, Intmod_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enter Internal Mode   SET INT MOD. This bit is the only way to enter Internal Mode."]
    #[inline(always)]
    pub fn set_int_mod(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, intmod::SetIntMod, Intmod_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            intmod::SetIntMod,
            Intmod_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Enable Internally Controlled Triggered Transfer   SET INT TRC. See CROSSREFERENCE for a description of the Triggered Transfer modes."]
    #[inline(always)]
    pub fn set_int_trc(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, intmod::SetIntTrc, Intmod_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            intmod::SetIntTrc,
            Intmod_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Disable Internally Controlled Triggered Transfer   CLR INT TRC"]
    #[inline(always)]
    pub fn clr_int_trc(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, intmod::ClrIntTrc, Intmod_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            intmod::ClrIntTrc,
            Intmod_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "TRC MOD Write Protection   TRC MOD P. Protect TRC MOD against unintended changes."]
    #[inline(always)]
    pub fn trc_mod_p(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, intmod::TrcModP, Intmod_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<20,0x1,1,0,intmod::TrcModP, Intmod_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Data Size Definition for Triggered Transfer   TRC MOD. The kind of bus access used during an Internally Controlled Triggered Transfer is defined by this bit field 1    If INT MOD is de asserted this bit field is set to 00 . This value will also be present whenever Internal Mode is entered."]
    #[inline(always)]
    pub fn trc_mod(
        self,
    ) -> crate::common::RegisterField<21, 0x3, 1, 0, intmod::TrcMod, Intmod_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x3,1,0,intmod::TrcMod, Intmod_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Internal Mode Enabled Flag   INT MOD. This bit reflects whether the Internal Mode is currently active. Set by SET IN T MOD   Cleared by Application Reset   OCDS disable or any IOClient access"]
    #[inline(always)]
    pub fn int_mod(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, intmod::IntMod, Intmod_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0x1,1,0,intmod::IntMod, Intmod_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Internally Controlled Triggered Transfer Enable   INT TRC. This bit tells whether an Internally Controlled Triggered Transfer is currently enabled. Set by SET INT TRC  Cleared by CLR INT TRC. This bit is just CROSSREFERENCE . EX B US TRC if INT MOD is de asserted."]
    #[inline(always)]
    pub fn int_trc(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, intmod::IntTrc, Intmod_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<25,0x1,1,0,intmod::IntTrc, Intmod_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Intmod {
    #[inline(always)]
    fn default() -> Intmod {
        <crate::RegValueT<Intmod_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod intmod {
    pub struct SetCrs_SPEC;
    pub type SetCrs = crate::EnumBitfieldStruct<u8, SetCrs_SPEC>;
    impl SetCrs {
        #[doc = "0 No action."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set IOSR .CRSYNC Can        be done concurrently with SET INT MOD in one write access to INTMOD."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct SetCws_SPEC;
    pub type SetCws = crate::EnumBitfieldStruct<u8, SetCws_SPEC>;
    impl SetCws {
        #[doc = "0 No action."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set IOSR .CWSYNC 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct SetCs_SPEC;
    pub type SetCs = crate::EnumBitfieldStruct<u8, SetCs_SPEC>;
    impl SetCs {
        #[doc = "0 No action."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set IOSR .COM SYNC 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct ClrCs_SPEC;
    pub type ClrCs = crate::EnumBitfieldStruct<u8, ClrCs_SPEC>;
    impl ClrCs {
        #[doc = "0 No action."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear IOSR .CWSYNC 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct ChannelP_SPEC;
    pub type ChannelP = crate::EnumBitfieldStruct<u8, ChannelP_SPEC>;
    impl ChannelP {
        #[doc = "0 CHANNEL is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 CHANNEL is updated by this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct SetIntMod_SPEC;
    pub type SetIntMod = crate::EnumBitfieldStruct<u8, SetIntMod_SPEC>;
    impl SetIntMod {
        #[doc = "0 No action."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enter Internal Mode if the other preconditions are met  see CROSSREFERENCE  ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct SetIntTrc_SPEC;
    pub type SetIntTrc = crate::EnumBitfieldStruct<u8, SetIntTrc_SPEC>;
    impl SetIntTrc {
        #[doc = "0 No action."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enable        Internally Controlled Triggered Transfer if the other preconditions are        met 1  ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct ClrIntTrc_SPEC;
    pub type ClrIntTrc = crate::EnumBitfieldStruct<u8, ClrIntTrc_SPEC>;
    impl ClrIntTrc {
        #[doc = "0 no action."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Disable Internally Controlled Triggered Transfer 1  ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrcModP_SPEC;
    pub type TrcModP = crate::EnumBitfieldStruct<u8, TrcModP_SPEC>;
    impl TrcModP {
        #[doc = "0 TRC MOD is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 TRC MOD is updated by this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrcMod_SPEC;
    pub type TrcMod = crate::EnumBitfieldStruct<u8, TrcMod_SPEC>;
    impl TrcMod {
        #[doc = "00 Word  32 bit ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Half word  16 bit ."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Byte  8 bit ."]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct IntMod_SPEC;
    pub type IntMod = crate::EnumBitfieldStruct<u8, IntMod_SPEC>;
    impl IntMod {
        #[doc = "0 Internal Mode not active."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Internal Mode active."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct IntTrc_SPEC;
    pub type IntTrc = crate::EnumBitfieldStruct<u8, IntTrc_SPEC>;
    impl IntTrc {
        #[doc = "0 No Internally Controlled Triggered Transfer."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Internally Controlled Triggered Transfer is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iosr_SPEC;
impl crate::sealed::RegSpec for Iosr_SPEC {
    type DataType = u32;
}
#[doc = "IOClientStatus and Control Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Iosr = crate::RegValueT<Iosr_SPEC>;

impl Iosr {
    #[doc = "Communication Mode Read Sync Flag   CRSYNC. Reflects the protocol state  namely whether the host is waiting for data to be sent. While INT MOD is de asserted this bit is set by reading the COMDATA register via the IO READ  WORD instruction. While INT MOD is asserted this bit is set by INTMOD . SET  CRS . It is cleared by writing to COMDATA through the bus slave interface or by CROSSREFERENCE . COM  RST The flag seen when INT MOD is asserted is not affected by CROSSREFERENCE . COM  RST . . Whenever this bit is set  independent of INT MOD   the OTGS is notified to optionally request an interrupt. This bit is not affected by changes of INT MOD itself."]
    #[inline(always)]
    pub fn crsync(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, iosr::Crsync, Iosr_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x1,1,0,iosr::Crsync, Iosr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Communication Mode Write Sync Flag   CWSYNC. Reflects the protocol state  namely whether the host has posted data to be fetched. While INT MOD is de asserted this bit is set by writing the COMDATA register via the IO WRI TE WORD instruction. While INT MOD is asserted this bit is set by INTMOD . SET  CWS . It is cleared by writing a  1  to CW  ACK or by CROSSREFERENCE . COM  RST 1  . Whenever this bit is set  independent of INT MOD   the OTGS is notified to optionally request an interrupt. This bit is not affected by changes of INT MOD itself."]
    #[inline(always)]
    pub fn cwsync(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, iosr::Cwsync, Iosr_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x1,1,0,iosr::Cwsync, Iosr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Communication Mode Write Acknowledge   CW ACK. The on chip software uses this bit to clear CW SYNC after the data was taken from COMDATA . There is no automatic clear of CWS YNC as each read may potentially be cancelled by pipelining or bus re arbitration   speculative  reads ."]
    #[inline(always)]
    pub fn cw_ack(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, iosr::CwAck, Iosr_SPEC, crate::common::W> {
        crate::common::RegisterField::<6,0x1,1,0,iosr::CwAck, Iosr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Communication Mode Synchronization Flag   COM SYNC. Can be used by the monitor program for higher level synchronization. While INT MOD is asserted this bit is set by INTMOD . SET  CS and cleared by INTMOD . CLR  CS . If INT MOD is de asserted this bit represents CROSSREFERENCE . COM  SYNC directly."]
    #[inline(always)]
    pub fn com_sync(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Iosr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Iosr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Tool Interface in Use   HOSTED. This bit is active when the DAP interface has received a CROSSREFERENCE ."]
    #[inline(always)]
    pub fn hosted(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, iosr::Hosted, Iosr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x1,1,0,iosr::Hosted, Iosr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Channel Indication   CHANNEL. These bits can be used by the tool software to facilitate multiple users of the tool interface in Communication Mode While INT MOD is asserted this represents INTMOD . CHAN NEL   CROSSREFERENCE . CHAN NEL otherwise."]
    #[inline(always)]
    pub fn channel(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, Iosr_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x7,1,0,u8, Iosr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Iosr {
    #[inline(always)]
    fn default() -> Iosr {
        <crate::RegValueT<Iosr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iosr {
    pub struct Crsync_SPEC;
    pub type Crsync = crate::EnumBitfieldStruct<u8, Crsync_SPEC>;
    impl Crsync {
        #[doc = "0 No receive request pending."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Data required to be written into COMDATA ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cwsync_SPEC;
    pub type Cwsync = crate::EnumBitfieldStruct<u8, Cwsync_SPEC>;
    impl Cwsync {
        #[doc = "0 No send request pending."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 New data available in COMDATA ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct CwAck_SPEC;
    pub type CwAck = crate::EnumBitfieldStruct<u8, CwAck_SPEC>;
    impl CwAck {
        #[doc = "0 No action."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear the CWSYNC bit."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hosted_SPEC;
    pub type Hosted = crate::EnumBitfieldStruct<u8, Hosted_SPEC>;
    impl Hosted {
        #[doc = "0 DAP was not initialized by tool."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 DAP is initialized."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jdpid_SPEC;
impl crate::sealed::RegSpec for Jdpid_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={PowerOn Reset:0x6360,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Jdpid = crate::RegValueT<Jdpid_SPEC>;

impl Jdpid {
    #[doc = "Module Revision   MOD REV. This bit field indicates the revision number of the module implementation. It is just CROSSREFERENCE  7 0 ."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Jdpid_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Jdpid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Module Type   MOD TYPE"]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Jdpid_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Jdpid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Module Number   MOD NUMBER"]
    #[inline(always)]
    pub fn mod_number(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Jdpid_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Jdpid_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Jdpid {
    #[inline(always)]
    fn default() -> Jdpid {
        <crate::RegValueT<Jdpid_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jtagid_SPEC;
impl crate::sealed::RegSpec for Jtagid_SPEC {
    type DataType = u32;
}
#[doc = "JTAGDevice Identification Register\n resetvalue={PowerOn Reset:0x0,CFS Value:0x10207083,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Jtagid = crate::RegValueT<Jtagid_SPEC>;

impl Jtagid {
    #[doc = "JTAGDevice ID   JTAG ID"]
    #[inline(always)]
    pub fn jtag_id(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Jtagid_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Jtagid_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Jtagid {
    #[inline(always)]
    fn default() -> Jtagid {
        <crate::RegValueT<Jtagid_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ocntrl_SPEC;
impl crate::sealed::RegSpec for Ocntrl_SPEC {
    type DataType = u32;
}
#[doc = "OSCU Control Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Ocntrl = crate::RegValueT<Ocntrl_SPEC>;

impl Ocntrl {
    #[doc = "OC5 Write Protection   OC5 P. Protect OSTATE .OCx against unintended changes."]
    #[inline(always)]
    pub fn oc0_p(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ocntrl::Oc0P, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x1,1,0,ocntrl::Oc0P, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "OC5 Write Protection   OC5 P. Protect OSTATE .OCx against unintended changes."]
    #[inline(always)]
    pub fn oc1_p(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ocntrl::Oc1P, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<2,0x1,1,0,ocntrl::Oc1P, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "OC5 Write Protection   OC5 P. Protect OSTATE .OCx against unintended changes."]
    #[inline(always)]
    pub fn oc2_p(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ocntrl::Oc2P, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<4,0x1,1,0,ocntrl::Oc2P, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "OC5 Write Protection   OC5 P. Protect OSTATE .OCx against unintended changes."]
    #[inline(always)]
    pub fn oc3_p(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, ocntrl::Oc3P, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<6,0x1,1,0,ocntrl::Oc3P, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "OC5 Write Protection   OC5 P. Protect OSTATE .OCx against unintended changes."]
    #[inline(always)]
    pub fn oc4_p(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, ocntrl::Oc4P, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0x1,1,0,ocntrl::Oc4P, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "OC5 Write Protection   OC5 P. Protect OSTATE .OCx against unintended changes."]
    #[inline(always)]
    pub fn oc5_p(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, ocntrl::Oc5P, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<10,0x1,1,0,ocntrl::Oc5P, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Clear OCDS Control Bits Bus Domain   OC5. These bits allow to change the associated OSTATE .OCx bits by software. OSTATE .OC3 is also called OSTATE . ENIDIS  OEN protected . OSTATE .OC4 is also called OSTATE . EECTRC  OEN protected . OSTATE .OC5 is also called OSTATE . EECDIS  OEN protected ."]
    #[inline(always)]
    pub fn oc0(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ocntrl::Oc0, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<1,0x1,1,0,ocntrl::Oc0, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Clear OCDS Control Bits Bus Domain   OC5. These bits allow to change the associated OSTATE .OCx bits by software. OSTATE .OC3 is also called OSTATE . ENIDIS  OEN protected . OSTATE .OC4 is also called OSTATE . EECTRC  OEN protected . OSTATE .OC5 is also called OSTATE . EECDIS  OEN protected ."]
    #[inline(always)]
    pub fn oc1(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ocntrl::Oc1, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<3,0x1,1,0,ocntrl::Oc1, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Clear OCDS Control Bits Bus Domain   OC5. These bits allow to change the associated OSTATE .OCx bits by software. OSTATE .OC3 is also called OSTATE . ENIDIS  OEN protected . OSTATE .OC4 is also called OSTATE . EECTRC  OEN protected . OSTATE .OC5 is also called OSTATE . EECDIS  OEN protected ."]
    #[inline(always)]
    pub fn oc2(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ocntrl::Oc2, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<5,0x1,1,0,ocntrl::Oc2, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Clear OCDS Control Bits Bus Domain   OC5. These bits allow to change the associated OSTATE .OCx bits by software. OSTATE .OC3 is also called OSTATE . ENIDIS  OEN protected . OSTATE .OC4 is also called OSTATE . EECTRC  OEN protected . OSTATE .OC5 is also called OSTATE . EECDIS  OEN protected ."]
    #[inline(always)]
    pub fn oc3(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, ocntrl::Oc3, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<7,0x1,1,0,ocntrl::Oc3, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Clear OCDS Control Bits Bus Domain   OC5. These bits allow to change the associated OSTATE .OCx bits by software. OSTATE .OC3 is also called OSTATE . ENIDIS  OEN protected . OSTATE .OC4 is also called OSTATE . EECTRC  OEN protected . OSTATE .OC5 is also called OSTATE . EECDIS  OEN protected ."]
    #[inline(always)]
    pub fn oc4(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, ocntrl::Oc4, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<9,0x1,1,0,ocntrl::Oc4, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Clear OCDS Control Bits Bus Domain   OC5. These bits allow to change the associated OSTATE .OCx bits by software. OSTATE .OC3 is also called OSTATE . ENIDIS  OEN protected . OSTATE .OC4 is also called OSTATE . EECTRC  OEN protected . OSTATE .OC5 is also called OSTATE . EECDIS  OEN protected ."]
    #[inline(always)]
    pub fn oc5(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, ocntrl::Oc5, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<11,0x1,1,0,ocntrl::Oc5, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "WDTSUS Write Protection   WDTSUS P. Protect OSTATE . W DTSUS against unintended changes."]
    #[inline(always)]
    pub fn wdtsus_p(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, ocntrl::WdtsusP, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<12,0x1,1,0,ocntrl::WdtsusP, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Clear Watchdog Timer Suspension Control   WDTSUS. This bit  OEN protected  is the only way to change the OSTATE . WD TSUS by software."]
    #[inline(always)]
    pub fn wdtsus(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, ocntrl::Wdtsus, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<13,0x1,1,0,ocntrl::Wdtsus, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "STABLE Write Protection   STABLE P. Protect OSTATE . STA BLE against unintended changes."]
    #[inline(always)]
    pub fn stable_p(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, ocntrl::StableP, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<14,0x1,1,0,ocntrl::StableP, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "InitializeApplication ResetIndication   STABLE. This bit is the only way to change the OSTATE . STA BLE bit by software."]
    #[inline(always)]
    pub fn stable(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, ocntrl::Stable, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<15,0x1,1,0,ocntrl::Stable, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "OJC7 Write Protection   OJC7 P. Protect OSTATE .OJCx against unintended changes."]
    #[inline(always)]
    pub fn ojc0_p(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, ocntrl::Ojc0P, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0x1,1,0,ocntrl::Ojc0P, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "OJC7 Write Protection   OJC7 P. Protect OSTATE .OJCx against unintended changes."]
    #[inline(always)]
    pub fn ojc1_p(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, ocntrl::Ojc1P, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<18,0x1,1,0,ocntrl::Ojc1P, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "OJC7 Write Protection   OJC7 P. Protect OSTATE .OJCx against unintended changes."]
    #[inline(always)]
    pub fn ojc2_p(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, ocntrl::Ojc2P, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<20,0x1,1,0,ocntrl::Ojc2P, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "OJC7 Write Protection   OJC7 P. Protect OSTATE .OJCx against unintended changes."]
    #[inline(always)]
    pub fn ojc3_p(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, ocntrl::Ojc3P, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<22,0x1,1,0,ocntrl::Ojc3P, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "OJC7 Write Protection   OJC7 P. Protect OSTATE .OJCx against unintended changes."]
    #[inline(always)]
    pub fn ojc4_p(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, ocntrl::Ojc4P, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<24,0x1,1,0,ocntrl::Ojc4P, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "OJC7 Write Protection   OJC7 P. Protect OSTATE .OJCx against unintended changes."]
    #[inline(always)]
    pub fn ojc5_p(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, ocntrl::Ojc5P, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<26,0x1,1,0,ocntrl::Ojc5P, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "OJC7 Write Protection   OJC7 P. Protect OSTATE .OJCx against unintended changes."]
    #[inline(always)]
    pub fn ojc6_p(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, ocntrl::Ojc6P, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<28,0x1,1,0,ocntrl::Ojc6P, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "OJC7 Write Protection   OJC7 P. Protect OSTATE .OJCx against unintended changes."]
    #[inline(always)]
    pub fn ojc7_p(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, ocntrl::Ojc7P, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<30,0x1,1,0,ocntrl::Ojc7P, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Clear OCDS Control Bits IOClient Domain   OJC7. These bits allow to change the associated OSTATE .OJCx bits by software. OSTATE .OJC0 is also called OSTATE . H ARR  OEN protected  . OSTATE .OJC4 is also called OSTATE . RSTCL0  OEN protected  . OSTATE .OJC5 is also called OSTATE . RSTCL1 . OSTATE .OJC7 is also called OSTATE . RSTCL3  OEN protected  . OSTATE .OJCx can also be set cleared by the IOClient via the CROSSREFERENCE register. The IOClient has the higher priority."]
    #[inline(always)]
    pub fn ojc0(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, ocntrl::Ojc0, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<17,0x1,1,0,ocntrl::Ojc0, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Clear OCDS Control Bits IOClient Domain   OJC7. These bits allow to change the associated OSTATE .OJCx bits by software. OSTATE .OJC0 is also called OSTATE . H ARR  OEN protected  . OSTATE .OJC4 is also called OSTATE . RSTCL0  OEN protected  . OSTATE .OJC5 is also called OSTATE . RSTCL1 . OSTATE .OJC7 is also called OSTATE . RSTCL3  OEN protected  . OSTATE .OJCx can also be set cleared by the IOClient via the CROSSREFERENCE register. The IOClient has the higher priority."]
    #[inline(always)]
    pub fn ojc1(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, ocntrl::Ojc1, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<19,0x1,1,0,ocntrl::Ojc1, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Clear OCDS Control Bits IOClient Domain   OJC7. These bits allow to change the associated OSTATE .OJCx bits by software. OSTATE .OJC0 is also called OSTATE . H ARR  OEN protected  . OSTATE .OJC4 is also called OSTATE . RSTCL0  OEN protected  . OSTATE .OJC5 is also called OSTATE . RSTCL1 . OSTATE .OJC7 is also called OSTATE . RSTCL3  OEN protected  . OSTATE .OJCx can also be set cleared by the IOClient via the CROSSREFERENCE register. The IOClient has the higher priority."]
    #[inline(always)]
    pub fn ojc2(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, ocntrl::Ojc2, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<21,0x1,1,0,ocntrl::Ojc2, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Clear OCDS Control Bits IOClient Domain   OJC7. These bits allow to change the associated OSTATE .OJCx bits by software. OSTATE .OJC0 is also called OSTATE . H ARR  OEN protected  . OSTATE .OJC4 is also called OSTATE . RSTCL0  OEN protected  . OSTATE .OJC5 is also called OSTATE . RSTCL1 . OSTATE .OJC7 is also called OSTATE . RSTCL3  OEN protected  . OSTATE .OJCx can also be set cleared by the IOClient via the CROSSREFERENCE register. The IOClient has the higher priority."]
    #[inline(always)]
    pub fn ojc3(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, ocntrl::Ojc3, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<23,0x1,1,0,ocntrl::Ojc3, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Clear OCDS Control Bits IOClient Domain   OJC7. These bits allow to change the associated OSTATE .OJCx bits by software. OSTATE .OJC0 is also called OSTATE . H ARR  OEN protected  . OSTATE .OJC4 is also called OSTATE . RSTCL0  OEN protected  . OSTATE .OJC5 is also called OSTATE . RSTCL1 . OSTATE .OJC7 is also called OSTATE . RSTCL3  OEN protected  . OSTATE .OJCx can also be set cleared by the IOClient via the CROSSREFERENCE register. The IOClient has the higher priority."]
    #[inline(always)]
    pub fn ojc4(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, ocntrl::Ojc4, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<25,0x1,1,0,ocntrl::Ojc4, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Clear OCDS Control Bits IOClient Domain   OJC7. These bits allow to change the associated OSTATE .OJCx bits by software. OSTATE .OJC0 is also called OSTATE . H ARR  OEN protected  . OSTATE .OJC4 is also called OSTATE . RSTCL0  OEN protected  . OSTATE .OJC5 is also called OSTATE . RSTCL1 . OSTATE .OJC7 is also called OSTATE . RSTCL3  OEN protected  . OSTATE .OJCx can also be set cleared by the IOClient via the CROSSREFERENCE register. The IOClient has the higher priority."]
    #[inline(always)]
    pub fn ojc5(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, ocntrl::Ojc5, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<27,0x1,1,0,ocntrl::Ojc5, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Clear OCDS Control Bits IOClient Domain   OJC7. These bits allow to change the associated OSTATE .OJCx bits by software. OSTATE .OJC0 is also called OSTATE . H ARR  OEN protected  . OSTATE .OJC4 is also called OSTATE . RSTCL0  OEN protected  . OSTATE .OJC5 is also called OSTATE . RSTCL1 . OSTATE .OJC7 is also called OSTATE . RSTCL3  OEN protected  . OSTATE .OJCx can also be set cleared by the IOClient via the CROSSREFERENCE register. The IOClient has the higher priority."]
    #[inline(always)]
    pub fn ojc6(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, ocntrl::Ojc6, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<29,0x1,1,0,ocntrl::Ojc6, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Clear OCDS Control Bits IOClient Domain   OJC7. These bits allow to change the associated OSTATE .OJCx bits by software. OSTATE .OJC0 is also called OSTATE . H ARR  OEN protected  . OSTATE .OJC4 is also called OSTATE . RSTCL0  OEN protected  . OSTATE .OJC5 is also called OSTATE . RSTCL1 . OSTATE .OJC7 is also called OSTATE . RSTCL3  OEN protected  . OSTATE .OJCx can also be set cleared by the IOClient via the CROSSREFERENCE register. The IOClient has the higher priority."]
    #[inline(always)]
    pub fn ojc7(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, ocntrl::Ojc7, Ocntrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<31,0x1,1,0,ocntrl::Ojc7, Ocntrl_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Ocntrl {
    #[inline(always)]
    fn default() -> Ocntrl {
        <crate::RegValueT<Ocntrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ocntrl {
    pub struct Oc0P_SPEC;
    pub type Oc0P = crate::EnumBitfieldStruct<u8, Oc0P_SPEC>;
    impl Oc0P {
        #[doc = "0 OSTATE .OCx is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 OSTATE .OCx is updated by this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oc1P_SPEC;
    pub type Oc1P = crate::EnumBitfieldStruct<u8, Oc1P_SPEC>;
    impl Oc1P {
        #[doc = "0 OSTATE .OCx is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 OSTATE .OCx is updated by this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oc2P_SPEC;
    pub type Oc2P = crate::EnumBitfieldStruct<u8, Oc2P_SPEC>;
    impl Oc2P {
        #[doc = "0 OSTATE .OCx is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 OSTATE .OCx is updated by this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oc3P_SPEC;
    pub type Oc3P = crate::EnumBitfieldStruct<u8, Oc3P_SPEC>;
    impl Oc3P {
        #[doc = "0 OSTATE .OCx is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 OSTATE .OCx is updated by this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oc4P_SPEC;
    pub type Oc4P = crate::EnumBitfieldStruct<u8, Oc4P_SPEC>;
    impl Oc4P {
        #[doc = "0 OSTATE .OCx is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 OSTATE .OCx is updated by this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oc5P_SPEC;
    pub type Oc5P = crate::EnumBitfieldStruct<u8, Oc5P_SPEC>;
    impl Oc5P {
        #[doc = "0 OSTATE .OCx is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 OSTATE .OCx is updated by this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oc0_SPEC;
    pub type Oc0 = crate::EnumBitfieldStruct<u8, Oc0_SPEC>;
    impl Oc0 {
        #[doc = "0 Clear OSTATE .OCx if OCx P is  1  concurrently in this write access."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set OSTATE .OCx if OCx P is  1  concurrently in this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oc1_SPEC;
    pub type Oc1 = crate::EnumBitfieldStruct<u8, Oc1_SPEC>;
    impl Oc1 {
        #[doc = "0 Clear OSTATE .OCx if OCx P is  1  concurrently in this write access."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set OSTATE .OCx if OCx P is  1  concurrently in this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oc2_SPEC;
    pub type Oc2 = crate::EnumBitfieldStruct<u8, Oc2_SPEC>;
    impl Oc2 {
        #[doc = "0 Clear OSTATE .OCx if OCx P is  1  concurrently in this write access."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set OSTATE .OCx if OCx P is  1  concurrently in this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oc3_SPEC;
    pub type Oc3 = crate::EnumBitfieldStruct<u8, Oc3_SPEC>;
    impl Oc3 {
        #[doc = "0 Clear OSTATE .OCx if OCx P is  1  concurrently in this write access."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set OSTATE .OCx if OCx P is  1  concurrently in this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oc4_SPEC;
    pub type Oc4 = crate::EnumBitfieldStruct<u8, Oc4_SPEC>;
    impl Oc4 {
        #[doc = "0 Clear OSTATE .OCx if OCx P is  1  concurrently in this write access."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set OSTATE .OCx if OCx P is  1  concurrently in this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oc5_SPEC;
    pub type Oc5 = crate::EnumBitfieldStruct<u8, Oc5_SPEC>;
    impl Oc5 {
        #[doc = "0 Clear OSTATE .OCx if OCx P is  1  concurrently in this write access."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set OSTATE .OCx if OCx P is  1  concurrently in this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct WdtsusP_SPEC;
    pub type WdtsusP = crate::EnumBitfieldStruct<u8, WdtsusP_SPEC>;
    impl WdtsusP {
        #[doc = "0 OSTATE .WDTSUS is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 OSTATE .WDTSUS is updated by this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wdtsus_SPEC;
    pub type Wdtsus = crate::EnumBitfieldStruct<u8, Wdtsus_SPEC>;
    impl Wdtsus {
        #[doc = "0 Clear OSTATE .WDTSUS if WDTSUS P is  1  concurrently in this write access."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set OSTATE .WDTSUS if WDTSUS P is  1  concurrently in this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct StableP_SPEC;
    pub type StableP = crate::EnumBitfieldStruct<u8, StableP_SPEC>;
    impl StableP {
        #[doc = "0 OSTATE .STABLE is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 OSTATE .STABLE is updated by this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Stable_SPEC;
    pub type Stable = crate::EnumBitfieldStruct<u8, Stable_SPEC>;
    impl Stable {
        #[doc = "0 Clear OSTATE .STABLE if STABLE P is  1  concurrently in this write access."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set OSTATE .STABLE if STABLE P is  1  concurrently in this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ojc0P_SPEC;
    pub type Ojc0P = crate::EnumBitfieldStruct<u8, Ojc0P_SPEC>;
    impl Ojc0P {
        #[doc = "0 OSTATE .OJCx is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 OSTATE .OJCx is updated by this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ojc1P_SPEC;
    pub type Ojc1P = crate::EnumBitfieldStruct<u8, Ojc1P_SPEC>;
    impl Ojc1P {
        #[doc = "0 OSTATE .OJCx is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 OSTATE .OJCx is updated by this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ojc2P_SPEC;
    pub type Ojc2P = crate::EnumBitfieldStruct<u8, Ojc2P_SPEC>;
    impl Ojc2P {
        #[doc = "0 OSTATE .OJCx is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 OSTATE .OJCx is updated by this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ojc3P_SPEC;
    pub type Ojc3P = crate::EnumBitfieldStruct<u8, Ojc3P_SPEC>;
    impl Ojc3P {
        #[doc = "0 OSTATE .OJCx is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 OSTATE .OJCx is updated by this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ojc4P_SPEC;
    pub type Ojc4P = crate::EnumBitfieldStruct<u8, Ojc4P_SPEC>;
    impl Ojc4P {
        #[doc = "0 OSTATE .OJCx is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 OSTATE .OJCx is updated by this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ojc5P_SPEC;
    pub type Ojc5P = crate::EnumBitfieldStruct<u8, Ojc5P_SPEC>;
    impl Ojc5P {
        #[doc = "0 OSTATE .OJCx is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 OSTATE .OJCx is updated by this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ojc6P_SPEC;
    pub type Ojc6P = crate::EnumBitfieldStruct<u8, Ojc6P_SPEC>;
    impl Ojc6P {
        #[doc = "0 OSTATE .OJCx is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 OSTATE .OJCx is updated by this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ojc7P_SPEC;
    pub type Ojc7P = crate::EnumBitfieldStruct<u8, Ojc7P_SPEC>;
    impl Ojc7P {
        #[doc = "0 OSTATE .OJCx is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 OSTATE .OJCx is updated by this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ojc0_SPEC;
    pub type Ojc0 = crate::EnumBitfieldStruct<u8, Ojc0_SPEC>;
    impl Ojc0 {
        #[doc = "0 Clear OSTATE .OJCx if OJCx P is  1  concurrently in this write access."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set OSTATE .OJCx if OJCx P is  1  concurrently in this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ojc1_SPEC;
    pub type Ojc1 = crate::EnumBitfieldStruct<u8, Ojc1_SPEC>;
    impl Ojc1 {
        #[doc = "0 Clear OSTATE .OJCx if OJCx P is  1  concurrently in this write access."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set OSTATE .OJCx if OJCx P is  1  concurrently in this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ojc2_SPEC;
    pub type Ojc2 = crate::EnumBitfieldStruct<u8, Ojc2_SPEC>;
    impl Ojc2 {
        #[doc = "0 Clear OSTATE .OJCx if OJCx P is  1  concurrently in this write access."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set OSTATE .OJCx if OJCx P is  1  concurrently in this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ojc3_SPEC;
    pub type Ojc3 = crate::EnumBitfieldStruct<u8, Ojc3_SPEC>;
    impl Ojc3 {
        #[doc = "0 Clear OSTATE .OJCx if OJCx P is  1  concurrently in this write access."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set OSTATE .OJCx if OJCx P is  1  concurrently in this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ojc4_SPEC;
    pub type Ojc4 = crate::EnumBitfieldStruct<u8, Ojc4_SPEC>;
    impl Ojc4 {
        #[doc = "0 Clear OSTATE .OJCx if OJCx P is  1  concurrently in this write access."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set OSTATE .OJCx if OJCx P is  1  concurrently in this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ojc5_SPEC;
    pub type Ojc5 = crate::EnumBitfieldStruct<u8, Ojc5_SPEC>;
    impl Ojc5 {
        #[doc = "0 Clear OSTATE .OJCx if OJCx P is  1  concurrently in this write access."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set OSTATE .OJCx if OJCx P is  1  concurrently in this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ojc6_SPEC;
    pub type Ojc6 = crate::EnumBitfieldStruct<u8, Ojc6_SPEC>;
    impl Ojc6 {
        #[doc = "0 Clear OSTATE .OJCx if OJCx P is  1  concurrently in this write access."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set OSTATE .OJCx if OJCx P is  1  concurrently in this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ojc7_SPEC;
    pub type Ojc7 = crate::EnumBitfieldStruct<u8, Ojc7_SPEC>;
    impl Ojc7 {
        #[doc = "0 Clear OSTATE .OJCx if OJCx P is  1  concurrently in this write access."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set OSTATE .OJCx if OJCx P is  1  concurrently in this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oec_SPEC;
impl crate::sealed::RegSpec for Oec_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Enable Control Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Oec = crate::RegValueT<Oec_SPEC>;

impl Oec {
    #[doc = "OCDS Enabling Pattern   PAT. The byte sequence described in CROSSREFERENCE must be written to this bit field sequentially to enable the OCDS subsystem."]
    #[inline(always)]
    pub fn pat(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Oec_SPEC, crate::common::W> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Oec_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable OCDS   DS. This bit allows the on chip software to disable the OCDS subsystem at any time. This is also good practice to do at the end of any debug session. To enable OCDS again the tool needs to repeat writing the enabling sequence into P AT ."]
    #[inline(always)]
    pub fn ds(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, oec::Ds, Oec_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0x1,1,0,oec::Ds, Oec_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "OCDS Clock Off   OCO. If OCDS is disabled again  the OCDS clock can be optional switched off as well for power saving reasons  e.g. for power measurements . The OCDS clock is automatically enabled with the OCDS. OSTATE . OCO shows the status. The OCDS clock can only be disabled  if OCDS was already disabled before by another OEC register write."]
    #[inline(always)]
    pub fn oco(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, oec::Oco, Oec_SPEC, crate::common::W> {
        crate::common::RegisterField::<9,0x1,1,0,oec::Oco, Oec_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "IF LCK Write Protection   IF LCK P. Protect IF  LCK against unintended changes."]
    #[inline(always)]
    pub fn if_lck_p(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, oec::IfLckP, Oec_SPEC, crate::common::W> {
        crate::common::RegisterField::<16,0x1,1,0,oec::IfLckP, Oec_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Clear Interface Locked Indication   IF LCK. This bit is the only way to change the OSTATE . IF  LCK by software."]
    #[inline(always)]
    pub fn if_lck(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, oec::IfLck, Oec_SPEC, crate::common::W> {
        crate::common::RegisterField::<17,0x1,1,0,oec::IfLck, Oec_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "AUT OK Write Protection   AUT OK P. Protect AUT  OK against unintended changes."]
    #[inline(always)]
    pub fn aut_ok_p(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, oec::AutOkP, Oec_SPEC, crate::common::W> {
        crate::common::RegisterField::<18,0x1,1,0,oec::AutOkP, Oec_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Clear the Authorization OK Indication   AUT OK. This bit is the only way to change the OSTATE . AUT  OK by software."]
    #[inline(always)]
    pub fn aut_ok(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, oec::AutOk, Oec_SPEC, crate::common::W> {
        crate::common::RegisterField::<19,0x1,1,0,oec::AutOk, Oec_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Oec {
    #[inline(always)]
    fn default() -> Oec {
        <crate::RegValueT<Oec_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod oec {
    pub struct Ds_SPEC;
    pub type Ds = crate::EnumBitfieldStruct<u8, Ds_SPEC>;
    impl Ds {
        #[doc = "0 No action."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear OSTATE .OEN."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oco_SPEC;
    pub type Oco = crate::EnumBitfieldStruct<u8, Oco_SPEC>;
    impl Oco {
        #[doc = "0 Writing zero has no effect."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Disable OCDS clock."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct IfLckP_SPEC;
    pub type IfLckP = crate::EnumBitfieldStruct<u8, IfLckP_SPEC>;
    impl IfLckP {
        #[doc = "0 IF LCK is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 IF LCK is updated by this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct IfLck_SPEC;
    pub type IfLck = crate::EnumBitfieldStruct<u8, IfLck_SPEC>;
    impl IfLck {
        #[doc = "0 Clear OSTATE .IF LCK if IF LCK P is  1  concurrently in this write access. If OSTATE .IF LCK is changed from  0  to  1  by this write OSTATE .OEN is automatically cleared."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set OSTATE .IF LCK if IF LCK P is  1  concurrently in this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct AutOkP_SPEC;
    pub type AutOkP = crate::EnumBitfieldStruct<u8, AutOkP_SPEC>;
    impl AutOkP {
        #[doc = "0 AUT OK is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 AUT OK is updated by this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct AutOk_SPEC;
    pub type AutOk = crate::EnumBitfieldStruct<u8, AutOk_SPEC>;
    impl AutOk {
        #[doc = "0 Clear OSTATE .AUT OK if AUT OK P is  1  concurrently in this write access."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set OSTATE .AUT OK if AUT OK P is  1  concurrently in this write access."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oifm_SPEC;
impl crate::sealed::RegSpec for Oifm_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Interface Mode Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Oifm = crate::RegValueT<Oifm_SPEC>;

impl Oifm {
    #[doc = "DAPInterface Mode   DAPMODE. DAPMODE mainly determines the number of pins allocated for the DAP        Interface. The selected mode becomes active with the next received CROSSREFERENCE .        The minimum number of dedicated DAP pins is device specific. If two pins        are dedicated  no SPD  Single Pin DAP  mode is needed offered. Please        request DAP RST in parallel only if it is really needed and intended to force the        selected mode and cut by this an already established tool connection.        The underlying assumption is that the tool user is in control and the        device should not sabotage tool access."]
    #[inline(always)]
    pub fn dapmode(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, oifm::Dapmode, Oifm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,oifm::Dapmode, Oifm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DAPProtocol Clear   DAPRST. Synchronous clear for the DAP state machine. See CROSSREFERENCE . Once set this bit is cleared by the hardware as soon as the DAP module has processed the reset request. As this is done with the DAP0 clock the time it takes is not predictable."]
    #[inline(always)]
    pub fn daprst(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, oifm::Daprst, Oifm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,oifm::Daprst, Oifm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Forced JTAG Mode.  dap bypass o"]
    #[inline(always)]
    pub fn f_jtag(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, oifm::FJtag, Oifm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,oifm::FJtag, Oifm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "No Switch to JTAG.  jtag disable o"]
    #[inline(always)]
    pub fn n_jtag(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, oifm::NJtag, Oifm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,oifm::NJtag, Oifm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Control for Debug Interface Pins.  padctl o  Please consult the Data Sheet for the proper setting relative to the        data rate. Selection valid for DAP and DAPE pads."]
    #[inline(always)]
    pub fn padctl(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, oifm::Padctl, Oifm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,oifm::Padctl, Oifm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad input threshold control for Debug Interface Pins. DAP pins always use TTL levels. This bits allows to set correct TTL        levels even for 3.3V supply. Selection valid for DAP and DAPE pads."]
    #[inline(always)]
    pub fn padctli(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, oifm::Padctli, Oifm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,oifm::Padctli, Oifm_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Oifm {
    #[inline(always)]
    fn default() -> Oifm {
        <crate::RegValueT<Oifm_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod oifm {
    pub struct Dapmode_SPEC;
    pub type Dapmode = crate::EnumBitfieldStruct<u8, Dapmode_SPEC>;
    impl Dapmode {
        #[doc = "000 Three Pins.        SPD  two pin DAP and all three pin DAP configurations including CROSSREFERENCE are possible. This reset value indicates that DAPMODE was not set by        software or tool."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 One Pin. Only SPD is possible."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 Two Pins. SPD and two pin DAP are possible."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 Three Pins.        SPD  two pin DAP and all three pin DAP configurations including CROSSREFERENCE are possible."]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 Four Pins. All three pin DAP configurations plus CROSSREFERENCE"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 Five Pins. All four pin DAP configurations plus DAPE  only applicable to ED devices  configurations"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "111 Disable the DAP module completely. DAP input pins are being ignored  no output pin is driven. In case of two or more dedicated DAP pins please note that after Power on Reset CROSSREFERENCE s are still being answered until a CROSSREFERENCE is received. This allows a tool to distinguish this situation from a device being unpowered or in reset state. It is recommended to disable the DAP module without forcing  so that an established tool connection is not being cut. Due to the signature in the CROSSREFERENCE it is safe to assume that every tool connection is intended and not caused by EMI."]
        pub const CONST_77: Self = Self::new(7);
    }
    pub struct Daprst_SPEC;
    pub type Daprst = crate::EnumBitfieldStruct<u8, Daprst_SPEC>;
    impl Daprst {
        #[doc = "0 No effect."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Force DAP module s  back to Enabled state."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct FJtag_SPEC;
    pub type FJtag = crate::EnumBitfieldStruct<u8, FJtag_SPEC>;
    impl FJtag {
        #[doc = "0 All interfaces         JTAG  DAP   DXCPL  DXCM         allowed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Only JTAG        possible"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct NJtag_SPEC;
    pub type NJtag = crate::EnumBitfieldStruct<u8, NJtag_SPEC>;
    impl NJtag {
        #[doc = "0 CROSSREFERENCE allowed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 CROSSREFERENCE disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Padctl_SPEC;
    pub type Padctl = crate::EnumBitfieldStruct<u8, Padctl_SPEC>;
    impl Padctl {
        #[doc = "00 Strong        drivers  sharp edges"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Strong        drivers  medium edges"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Medium drivers"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Padctli_SPEC;
    pub type Padctli = crate::EnumBitfieldStruct<u8, Padctli_SPEC>;
    impl Padctli {
        #[doc = "00 TTL levels        when using 5V supply"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 TTL levels        when using 3.3V supply"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ostate_SPEC;
impl crate::sealed::RegSpec for Ostate_SPEC {
    type DataType = u32;
}
#[doc = "OSCUStatus Register\n resetvalue={PowerOn Reset:0x10000,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Ostate = crate::RegValueT<Ostate_SPEC>;

impl Ostate {
    #[doc = "OCDS Enabled Flag   OEN. This bit enables the OCDS functionality of the complete device. Set and clear is done through the OEC register. This flag does not show any overruling by the HSM ."]
    #[inline(always)]
    pub fn oen(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ostate::Oen, Ostate_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,ostate::Oen, Ostate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "OCDS Control Bits System Bus Domain   OC2. Are used for device specific OCDS configuration purposes. They can be set and cleared with the associated OCNTRL .OCx bits."]
    #[inline(always)]
    pub fn oc0(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ostate::Oc0, Ostate_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,ostate::Oc0, Ostate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "OCDS Control Bits System Bus Domain   OC2. Are used for device specific OCDS configuration purposes. They can be set and cleared with the associated OCNTRL .OCx bits."]
    #[inline(always)]
    pub fn oc1(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ostate::Oc1, Ostate_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1,1,0,ostate::Oc1, Ostate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "OCDS Control Bits System Bus Domain   OC2. Are used for device specific OCDS configuration purposes. They can be set and cleared with the associated OCNTRL .OCx bits."]
    #[inline(always)]
    pub fn oc2(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ostate::Oc2, Ostate_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x1,1,0,ostate::Oc2, Ostate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "OCDS ENDINIT Protection Override   ENIDIS. To modify ENDINIT protected register bits via Cerberus it would be dangerous to tamper with the protection logic controlled by a watchdog timer. This control bit allows to overrule the SCU.  Can be set and cleared with OCNTRL .OC3 only."]
    #[inline(always)]
    pub fn enidis(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ostate::Enidis, Ostate_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,ostate::Enidis, Ostate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "On Chip Trace Enable   EECTRC. The trace ports of all cores are normally turned off for security and power consumption reasons. Only if tracing is desired the ports are globally enabled with this control bit.  Can be set and cleared with OCNTRL .OC4 only. Specific enables for single trace interfaces will be added inside MCDS."]
    #[inline(always)]
    pub fn eectrc(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ostate::Eectrc, Ostate_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x1,1,0,ostate::Eectrc, Ostate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Emulation Logic Disable   EECDIS. If the features of the EEC are not needed  the complete EEC can be switched off with this control bit.  Can be set and cleared with OCNTRL .OC5 only. Both tracing and calibration  overlay  require the EEC to be turned on."]
    #[inline(always)]
    pub fn eecdis(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, ostate::Eecdis, Ostate_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x1,1,0,ostate::Eecdis, Ostate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Control of Watchdog Timer  WDT  Suspension   WDTSUS. This bit determines the behavior of the watchdog timers when the system is running with OCDS enabled. Set and clear is done by OCNTRL . WD TSUS ."]
    #[inline(always)]
    pub fn wdtsus(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, ostate::Wdtsus, Ostate_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<7,0x1,1,0,ostate::Wdtsus, Ostate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Halt after Reset Request   HARR. This bit tells the Startup Software to stop prior to entering the application code.  It can be set cleared with either OCNTRL .OJC0  via the Bus Slave Interface  or CROSSREFERENCE .OJC0  via the IOClient  . The IOClient however has the higher priority. The Kernel clock must be running to change this bit. A request from the IOClient however is stored until this is the case."]
    #[inline(always)]
    pub fn harr(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, ostate::Harr, Ostate_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x1,1,0,ostate::Harr, Ostate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "OCDS Control Bits IOClient Domain OJC1. Are used for device specific OCDS configuration purposes. As these bits can be set by the IOClient regardless of IF  LCK and O EN make sure not to create a security hole in usage. They can be set and cleared either with the associated OCNTRL .OJCx         via the system bus  or CROSSREFERENCE .OJCx        bits. The IOClient however has the higher priority. The Kernel clock must be running to change these bits. A request from          the IOClient however is stored until this is the case."]
    #[inline(always)]
    pub fn ojc1(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, ostate::Ojc1, Ostate_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<9,0x1,1,0,ostate::Ojc1, Ostate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "OCDS Control Bits IOClient Domain OJC2. Are used for device specific OCDS configuration purposes. As these bits can be set by the IOClient regardless of IF  LCK and O EN make sure not to create a security hole in usage. They can be set and cleared either with the associated OCNTRL .OJCx         via the system bus  or CROSSREFERENCE .OJCx        bits. The IOClient however has the higher priority. The Kernel clock must be running to change these bits. A request from          the IOClient however is stored until this is the case."]
    #[inline(always)]
    pub fn ojc2(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, ostate::Ojc2, Ostate_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<10,0x1,1,0,ostate::Ojc2, Ostate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "OCDS Control Bits IOClient Domain OJC3. Are used for device specific OCDS configuration purposes. As these bits can be set by the IOClient regardless of IF  LCK and O EN make sure not to create a security hole in usage. They can be set and cleared either with the associated OCNTRL .OJCx         via the system bus  or CROSSREFERENCE .OJCx        bits. The IOClient however has the higher priority. The Kernel clock must be running to change these bits. A request from          the IOClient however is stored until this is the case."]
    #[inline(always)]
    pub fn ojc3(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, ostate::Ojc3, Ostate_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<11,0x1,1,0,ostate::Ojc3, Ostate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "OCDS System Reset Request   RSTCL0. Can be set either with the associated OCNTRL .OJC4  via the system bus  or CROSSREFERENCE .OJC4 bits. The IOClient has the higher priority. The Kernel clock must be running to change this bit. A request from the IOClient however is stored until this is the case. This bit is automatically cleared by the next System Reset."]
    #[inline(always)]
    pub fn rstcl0(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, ostate::Rstcl0, Ostate_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<12,0x1,1,0,ostate::Rstcl0, Ostate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "OCDS Debug Reset Request   RSTCL1. Can be set and cleared either with the associated OCNTRL .OJC5  via the system bus  or CROSSREFERENCE .OJC5 bits. The IOClient has the higher priority. The Kernel clock must be running to change this bit. A request from the IOClient however is stored until this is the case. If the last set operation was done via OCNTRL this bit is automatically cleared by the next Application Reset . The requested Debug Reset will not clear this request automatically  a second write to OCNTRL is required to terminate the Debug Reset ."]
    #[inline(always)]
    pub fn rstcl1(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, ostate::Rstcl1, Ostate_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<13,0x1,1,0,ostate::Rstcl1, Ostate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "OCDS Control Bits IOClient Domain OJC6. Are used for device specific OCDS configuration purposes. As these bits can be set by the IOClient regardless of IF  LCK and O EN make sure not to create a security hole in usage. They can be set and cleared either with the associated OCNTRL .OJCx         via the system bus  or CROSSREFERENCE .OJCx        bits. The IOClient however has the higher priority. The Kernel clock must be running to change these bits. A request from          the IOClient however is stored until this is the case."]
    #[inline(always)]
    pub fn ojc6(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, ostate::Ojc6, Ostate_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<14,0x1,1,0,ostate::Ojc6, Ostate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "OCDS Application Reset Request   RSTCL3. Can be set and cleared either with the associated OCNTRL .OJC7  via the system bus  or CROSSREFERENCE .OJC7 bits. The IOClient has the higher priority. The Kernel clock must be running to change this bit. A request from the IOClient however is stored until this is the case. If the last set operation was done via OCNTRL this bit is automatically cleared by the next Application Reset ."]
    #[inline(always)]
    pub fn rstcl3(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, ostate::Rstcl3, Ostate_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<15,0x1,1,0,ostate::Rstcl3, Ostate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Interface Locked Indication   IF LCK. This bit is used by the software inside the device to control the access right of the IOClient. Set and clear is done through the OEC register. Overruling by HSM is not visible in this bit."]
    #[inline(always)]
    pub fn if_lck(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, ostate::IfLck, Ostate_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1,1,0,ostate::IfLck, Ostate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Authorization OK Indication   AUT OK. This bit is used by the software inside the device to store the higher        level   CROSSREFERENCE          access right status of the IOClient through Application        Reset s. It has no direct hardware effects. Set and clear is done through the OEC register."]
    #[inline(always)]
    pub fn aut_ok(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, ostate::AutOk, Ostate_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<17,0x1,1,0,ostate::AutOk, Ostate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Application Reset Indication   STABLE. This bit is used by the tool to detect Application        Reset s. It has no direct hardware effects. Set and clear is done through the OCNTRL register."]
    #[inline(always)]
    pub fn stable(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, ostate::Stable, Ostate_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<18,0x1,1,0,ostate::Stable, Ostate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "OCDS debug resource Clock On Indication   OCO. This bit is used by the tool to detect the status of the OCDS debug        resource clock on control register. Set and clear is done through the OEC . OCO register."]
    #[inline(always)]
    pub fn oco(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, ostate::Oco, Ostate_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<19,0x1,1,0,ostate::Oco, Ostate_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ostate {
    #[inline(always)]
    fn default() -> Ostate {
        <crate::RegValueT<Ostate_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ostate {
    pub struct Oen_SPEC;
    pub type Oen = crate::EnumBitfieldStruct<u8, Oen_SPEC>;
    impl Oen {
        #[doc = "0 OCDS is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 OCDS is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oc0_SPEC;
    pub type Oc0 = crate::EnumBitfieldStruct<u8, Oc0_SPEC>;
    impl Oc0 {
        #[doc = "0 Default behavior."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Special behavior."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oc1_SPEC;
    pub type Oc1 = crate::EnumBitfieldStruct<u8, Oc1_SPEC>;
    impl Oc1 {
        #[doc = "0 Default behavior."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Special behavior."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oc2_SPEC;
    pub type Oc2 = crate::EnumBitfieldStruct<u8, Oc2_SPEC>;
    impl Oc2 {
        #[doc = "0 Default behavior."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Special behavior."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enidis_SPEC;
    pub type Enidis = crate::EnumBitfieldStruct<u8, Enidis_SPEC>;
    impl Enidis {
        #[doc = "0 ENDINIT protection is controlled by SCU."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ENDINIT protection is disabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eectrc_SPEC;
    pub type Eectrc = crate::EnumBitfieldStruct<u8, Eectrc_SPEC>;
    impl Eectrc {
        #[doc = "0 All trace ports are turned off."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 All trace ports towards MCDS are turned on."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eecdis_SPEC;
    pub type Eecdis = crate::EnumBitfieldStruct<u8, Eecdis_SPEC>;
    impl Eecdis {
        #[doc = "0 EEC is enabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 EEC is disabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wdtsus_SPEC;
    pub type Wdtsus = crate::EnumBitfieldStruct<u8, Wdtsus_SPEC>;
    impl Wdtsus {
        #[doc = "0 All watchdog timers suspended as long as OCDS is enabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A watchdog timer is only suspended when the SUSOUT signal of the associated CPU is asserted."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Harr_SPEC;
    pub type Harr = crate::EnumBitfieldStruct<u8, Harr_SPEC>;
    impl Harr {
        #[doc = "0 After Application Reset the application code  according to boot option  is started."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 After the next Application Reset the processor is sent to HALT state prior to executing any application code."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ojc1_SPEC;
    pub type Ojc1 = crate::EnumBitfieldStruct<u8, Ojc1_SPEC>;
    impl Ojc1 {
        #[doc = "0 Default behavior."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Special behavior."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ojc2_SPEC;
    pub type Ojc2 = crate::EnumBitfieldStruct<u8, Ojc2_SPEC>;
    impl Ojc2 {
        #[doc = "0 Default        behavior."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Special behavior."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ojc3_SPEC;
    pub type Ojc3 = crate::EnumBitfieldStruct<u8, Ojc3_SPEC>;
    impl Ojc3 {
        #[doc = "0 Default behavior."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Special behavior."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rstcl0_SPEC;
    pub type Rstcl0 = crate::EnumBitfieldStruct<u8, Rstcl0_SPEC>;
    impl Rstcl0 {
        #[doc = "0 No effect."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A System Reset request is sent to the SCU."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rstcl1_SPEC;
    pub type Rstcl1 = crate::EnumBitfieldStruct<u8, Rstcl1_SPEC>;
    impl Rstcl1 {
        #[doc = "0 No effect."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A Debug Reset request is sent to the SCU."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ojc6_SPEC;
    pub type Ojc6 = crate::EnumBitfieldStruct<u8, Ojc6_SPEC>;
    impl Ojc6 {
        #[doc = "0 Default behavior."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Special behavior."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rstcl3_SPEC;
    pub type Rstcl3 = crate::EnumBitfieldStruct<u8, Rstcl3_SPEC>;
    impl Rstcl3 {
        #[doc = "0 No effect."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An Application Reset request is sent to the SCU."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct IfLck_SPEC;
    pub type IfLck = crate::EnumBitfieldStruct<u8, IfLck_SPEC>;
    impl IfLck {
        #[doc = "0 The IOClient is allowed to gain access."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The IOClient is not allowed to enter RW Mode."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct AutOk_SPEC;
    pub type AutOk = crate::EnumBitfieldStruct<u8, AutOk_SPEC>;
    impl AutOk {
        #[doc = "0 Tool was not authorized before."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Tool was authorized already."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Stable_SPEC;
    pub type Stable = crate::EnumBitfieldStruct<u8, Stable_SPEC>;
    impl Stable {
        #[doc = "0 At least one reset occurred since last set operation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No Application Reset since last set operation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oco_SPEC;
    pub type Oco = crate::EnumBitfieldStruct<u8, Oco_SPEC>;
    impl Oco {
        #[doc = "0 OCDS resource clock OFF  default ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 OCDS related peripheral clock ON."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tccb_SPEC;
impl crate::sealed::RegSpec for Tccb_SPEC {
    type DataType = u32;
}
#[doc = "TG Capture for Cores   BRKOUT\n resetvalue={Debug Reset:0x0}"]
pub type Tccb = crate::RegValueT<Tccb_SPEC>;

impl Tccb {
    #[doc = "Capture of BRKOUT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, tccb::C0, Tccb_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,tccb::C0, Tccb_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of BRKOUT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, tccb::C1, Tccb_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,tccb::C1, Tccb_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of BRKOUT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, tccb::C2, Tccb_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,tccb::C2, Tccb_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of BRKOUT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, tccb::C3, Tccb_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1,1,0,tccb::C3, Tccb_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of BRKOUT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, tccb::C4, Tccb_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x1,1,0,tccb::C4, Tccb_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of BRKOUT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, tccb::C5, Tccb_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x1,1,0,tccb::C5, Tccb_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of BRKOUT Signal ofHSM   HSM"]
    #[inline(always)]
    pub fn hsm(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, tccb::Hsm, Tccb_SPEC, crate::common::R> {
        crate::common::RegisterField::<31,0x1,1,0,tccb::Hsm, Tccb_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Tccb {
    #[inline(always)]
    fn default() -> Tccb {
        <crate::RegValueT<Tccb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tccb {
    pub struct C0_SPEC;
    pub type C0 = crate::EnumBitfieldStruct<u8, C0_SPEC>;
    impl C0 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct C1_SPEC;
    pub type C1 = crate::EnumBitfieldStruct<u8, C1_SPEC>;
    impl C1 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct C2_SPEC;
    pub type C2 = crate::EnumBitfieldStruct<u8, C2_SPEC>;
    impl C2 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct C3_SPEC;
    pub type C3 = crate::EnumBitfieldStruct<u8, C3_SPEC>;
    impl C3 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct C4_SPEC;
    pub type C4 = crate::EnumBitfieldStruct<u8, C4_SPEC>;
    impl C4 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct C5_SPEC;
    pub type C5 = crate::EnumBitfieldStruct<u8, C5_SPEC>;
    impl C5 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hsm_SPEC;
    pub type Hsm = crate::EnumBitfieldStruct<u8, Hsm_SPEC>;
    impl Hsm {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcch_SPEC;
impl crate::sealed::RegSpec for Tcch_SPEC {
    type DataType = u32;
}
#[doc = "TG Capture for Cores   HALT\n resetvalue={Debug Reset:0x0}"]
pub type Tcch = crate::RegValueT<Tcch_SPEC>;

impl Tcch {
    #[doc = "Capture of HALT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, tcch::C0, Tcch_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,tcch::C0, Tcch_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of HALT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, tcch::C1, Tcch_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,tcch::C1, Tcch_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of HALT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, tcch::C2, Tcch_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,tcch::C2, Tcch_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of HALT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, tcch::C3, Tcch_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1,1,0,tcch::C3, Tcch_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of HALT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, tcch::C4, Tcch_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x1,1,0,tcch::C4, Tcch_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of HALT Signal of CPU5   C5"]
    #[inline(always)]
    pub fn c5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, tcch::C5, Tcch_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x1,1,0,tcch::C5, Tcch_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of HALT Signal ofHSM   HSM"]
    #[inline(always)]
    pub fn hsm(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, tcch::Hsm, Tcch_SPEC, crate::common::R> {
        crate::common::RegisterField::<31,0x1,1,0,tcch::Hsm, Tcch_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Tcch {
    #[inline(always)]
    fn default() -> Tcch {
        <crate::RegValueT<Tcch_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcch {
    pub struct C0_SPEC;
    pub type C0 = crate::EnumBitfieldStruct<u8, C0_SPEC>;
    impl C0 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct C1_SPEC;
    pub type C1 = crate::EnumBitfieldStruct<u8, C1_SPEC>;
    impl C1 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct C2_SPEC;
    pub type C2 = crate::EnumBitfieldStruct<u8, C2_SPEC>;
    impl C2 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct C3_SPEC;
    pub type C3 = crate::EnumBitfieldStruct<u8, C3_SPEC>;
    impl C3 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct C4_SPEC;
    pub type C4 = crate::EnumBitfieldStruct<u8, C4_SPEC>;
    impl C4 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct C5_SPEC;
    pub type C5 = crate::EnumBitfieldStruct<u8, C5_SPEC>;
    impl C5 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hsm_SPEC;
    pub type Hsm = crate::EnumBitfieldStruct<u8, Hsm_SPEC>;
    impl Hsm {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcip_SPEC;
impl crate::sealed::RegSpec for Tcip_SPEC {
    type DataType = u32;
}
#[doc = "TG Capture for TG Input Pins\n resetvalue={Debug Reset:0x0}"]
pub type Tcip = crate::RegValueT<Tcip_SPEC>;

impl Tcip {
    #[doc = "Capture of Trigger Input Pin 7   P7"]
    #[inline(always)]
    pub fn p0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, tcip::P0, Tcip_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,tcip::P0, Tcip_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of Trigger Input Pin 7   P7"]
    #[inline(always)]
    pub fn p1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, tcip::P1, Tcip_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,tcip::P1, Tcip_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of Trigger Input Pin 7   P7"]
    #[inline(always)]
    pub fn p2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, tcip::P2, Tcip_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,tcip::P2, Tcip_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of Trigger Input Pin 7   P7"]
    #[inline(always)]
    pub fn p3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, tcip::P3, Tcip_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1,1,0,tcip::P3, Tcip_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of Trigger Input Pin 7   P7"]
    #[inline(always)]
    pub fn p4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, tcip::P4, Tcip_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x1,1,0,tcip::P4, Tcip_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of Trigger Input Pin 7   P7"]
    #[inline(always)]
    pub fn p5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, tcip::P5, Tcip_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x1,1,0,tcip::P5, Tcip_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of Trigger Input Pin 7   P7"]
    #[inline(always)]
    pub fn p6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, tcip::P6, Tcip_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x1,1,0,tcip::P6, Tcip_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of Trigger Input Pin 7   P7"]
    #[inline(always)]
    pub fn p7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, tcip::P7, Tcip_SPEC, crate::common::R> {
        crate::common::RegisterField::<7,0x1,1,0,tcip::P7, Tcip_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Tcip {
    #[inline(always)]
    fn default() -> Tcip {
        <crate::RegValueT<Tcip_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcip {
    pub struct P0_SPEC;
    pub type P0 = crate::EnumBitfieldStruct<u8, P0_SPEC>;
    impl P0 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct P1_SPEC;
    pub type P1 = crate::EnumBitfieldStruct<u8, P1_SPEC>;
    impl P1 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct P2_SPEC;
    pub type P2 = crate::EnumBitfieldStruct<u8, P2_SPEC>;
    impl P2 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct P3_SPEC;
    pub type P3 = crate::EnumBitfieldStruct<u8, P3_SPEC>;
    impl P3 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct P4_SPEC;
    pub type P4 = crate::EnumBitfieldStruct<u8, P4_SPEC>;
    impl P4 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct P5_SPEC;
    pub type P5 = crate::EnumBitfieldStruct<u8, P5_SPEC>;
    impl P5 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct P6_SPEC;
    pub type P6 = crate::EnumBitfieldStruct<u8, P6_SPEC>;
    impl P6 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct P7_SPEC;
    pub type P7 = crate::EnumBitfieldStruct<u8, P7_SPEC>;
    impl P7 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcm_SPEC;
impl crate::sealed::RegSpec for Tcm_SPEC {
    type DataType = u32;
}
#[doc = "TG Capture for MCDS\n resetvalue={Debug Reset:0x0}"]
pub type Tcm = crate::RegValueT<Tcm_SPEC>;

impl Tcm {
    #[doc = "Capture of MCDS break out   BRK"]
    #[inline(always)]
    pub fn brk(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, tcm::Brk, Tcm_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,tcm::Brk, Tcm_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of MCDS suspend out   SUS"]
    #[inline(always)]
    pub fn sus(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, tcm::Sus, Tcm_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,tcm::Sus, Tcm_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of MCDS trig out 3   T3"]
    #[inline(always)]
    pub fn t0(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, tcm::T0, Tcm_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x1,1,0,tcm::T0, Tcm_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of MCDS trig out 3   T3"]
    #[inline(always)]
    pub fn t1(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, tcm::T1, Tcm_SPEC, crate::common::R> {
        crate::common::RegisterField::<9,0x1,1,0,tcm::T1, Tcm_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of MCDS trig out 3   T3"]
    #[inline(always)]
    pub fn t2(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, tcm::T2, Tcm_SPEC, crate::common::R> {
        crate::common::RegisterField::<10,0x1,1,0,tcm::T2, Tcm_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of MCDS trig out 3   T3"]
    #[inline(always)]
    pub fn t3(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, tcm::T3, Tcm_SPEC, crate::common::R> {
        crate::common::RegisterField::<11,0x1,1,0,tcm::T3, Tcm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Tcm {
    #[inline(always)]
    fn default() -> Tcm {
        <crate::RegValueT<Tcm_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tcm {
    pub struct Brk_SPEC;
    pub type Brk = crate::EnumBitfieldStruct<u8, Brk_SPEC>;
    impl Brk {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sus_SPEC;
    pub type Sus = crate::EnumBitfieldStruct<u8, Sus_SPEC>;
    impl Sus {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T0_SPEC;
    pub type T0 = crate::EnumBitfieldStruct<u8, T0_SPEC>;
    impl T0 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T1_SPEC;
    pub type T1 = crate::EnumBitfieldStruct<u8, T1_SPEC>;
    impl T1 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T2_SPEC;
    pub type T2 = crate::EnumBitfieldStruct<u8, T2_SPEC>;
    impl T2 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T3_SPEC;
    pub type T3 = crate::EnumBitfieldStruct<u8, T3_SPEC>;
    impl T3 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tctgb_SPEC;
impl crate::sealed::RegSpec for Tctgb_SPEC {
    type DataType = u32;
}
#[doc = "TG Capture for OTGB0 1\n resetvalue={Debug Reset:0x0}"]
pub type Tctgb = crate::RegValueT<Tctgb_SPEC>;

impl Tctgb {
    #[doc = "Capture Bits for OTGB0   OTGB0. If a bit is set  the associated OTGB0 signal was active high since last read."]
    #[inline(always)]
    pub fn otgb0(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Tctgb_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Tctgb_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture Bits for OTGB1   OTGB1. If a bit is set  the associated OTGB1 signal was active high since last read."]
    #[inline(always)]
    pub fn otgb1(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Tctgb_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Tctgb_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Tctgb {
    #[inline(always)]
    fn default() -> Tctgb {
        <crate::RegValueT<Tctgb_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tctl_SPEC;
impl crate::sealed::RegSpec for Tctl_SPEC {
    type DataType = u32;
}
#[doc = "TG Capture for TG Lines\n resetvalue={Debug Reset:0x0}"]
pub type Tctl = crate::RegValueT<Tctl_SPEC>;

impl Tctl {
    #[doc = "Capture of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, tctl::Tl1, Tctl_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,tctl::Tl1, Tctl_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, tctl::Tl2, Tctl_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,tctl::Tl2, Tctl_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, tctl::Tl3, Tctl_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1,1,0,tctl::Tl3, Tctl_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, tctl::Tl4, Tctl_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x1,1,0,tctl::Tl4, Tctl_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, tctl::Tl5, Tctl_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x1,1,0,tctl::Tl5, Tctl_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, tctl::Tl6, Tctl_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x1,1,0,tctl::Tl6, Tctl_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, tctl::Tl7, Tctl_SPEC, crate::common::R> {
        crate::common::RegisterField::<7,0x1,1,0,tctl::Tl7, Tctl_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Tctl {
    #[inline(always)]
    fn default() -> Tctl {
        <crate::RegValueT<Tctl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tctl {
    pub struct Tl1_SPEC;
    pub type Tl1 = crate::EnumBitfieldStruct<u8, Tl1_SPEC>;
    impl Tl1 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tl2_SPEC;
    pub type Tl2 = crate::EnumBitfieldStruct<u8, Tl2_SPEC>;
    impl Tl2 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tl3_SPEC;
    pub type Tl3 = crate::EnumBitfieldStruct<u8, Tl3_SPEC>;
    impl Tl3 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tl4_SPEC;
    pub type Tl4 = crate::EnumBitfieldStruct<u8, Tl4_SPEC>;
    impl Tl4 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tl5_SPEC;
    pub type Tl5 = crate::EnumBitfieldStruct<u8, Tl5_SPEC>;
    impl Tl5 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tl6_SPEC;
    pub type Tl6 = crate::EnumBitfieldStruct<u8, Tl6_SPEC>;
    impl Tl6 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tl7_SPEC;
    pub type Tl7 = crate::EnumBitfieldStruct<u8, Tl7_SPEC>;
    impl Tl7 {
        #[doc = "0 Signal was not active since last read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal was active since last read or is active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tipr_SPEC;
impl crate::sealed::RegSpec for Tipr_SPEC {
    type DataType = u32;
}
#[doc = "TG Input Pins Routing\n resetvalue={Debug Reset:0x0}"]
pub type Tipr = crate::RegValueT<Tipr_SPEC>;

impl Tipr {
    #[doc = "Trigger Pin 7 to Trigger Line Routing"]
    #[inline(always)]
    pub fn pin0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, tipr::Pin0, Tipr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,tipr::Pin0, Tipr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Pin 7 to Trigger Line Routing"]
    #[inline(always)]
    pub fn pin1(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, tipr::Pin1, Tipr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,tipr::Pin1, Tipr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Pin 7 to Trigger Line Routing"]
    #[inline(always)]
    pub fn pin2(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, tipr::Pin2, Tipr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,tipr::Pin2, Tipr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Pin 7 to Trigger Line Routing"]
    #[inline(always)]
    pub fn pin3(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, tipr::Pin3, Tipr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,tipr::Pin3, Tipr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Pin 7 to Trigger Line Routing"]
    #[inline(always)]
    pub fn pin4(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, tipr::Pin4, Tipr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,tipr::Pin4, Tipr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Pin 7 to Trigger Line Routing"]
    #[inline(always)]
    pub fn pin5(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, tipr::Pin5, Tipr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,tipr::Pin5, Tipr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Pin 7 to Trigger Line Routing"]
    #[inline(always)]
    pub fn pin6(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, tipr::Pin6, Tipr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,tipr::Pin6, Tipr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Pin 7 to Trigger Line Routing"]
    #[inline(always)]
    pub fn pin7(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, tipr::Pin7, Tipr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,tipr::Pin7, Tipr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Tipr {
    #[inline(always)]
    fn default() -> Tipr {
        <crate::RegValueT<Tipr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tipr {
    pub struct Pin0_SPEC;
    pub type Pin0 = crate::EnumBitfieldStruct<u8, Pin0_SPEC>;
    impl Pin0 {
        #[doc = "Trigger Input Pin signal is ignored"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Pin1_SPEC;
    pub type Pin1 = crate::EnumBitfieldStruct<u8, Pin1_SPEC>;
    impl Pin1 {
        #[doc = "Trigger Input Pin signal is ignored"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Pin2_SPEC;
    pub type Pin2 = crate::EnumBitfieldStruct<u8, Pin2_SPEC>;
    impl Pin2 {
        #[doc = "Trigger Input Pin signal is ignored"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Pin3_SPEC;
    pub type Pin3 = crate::EnumBitfieldStruct<u8, Pin3_SPEC>;
    impl Pin3 {
        #[doc = "Trigger Input Pin signal is ignored"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Pin4_SPEC;
    pub type Pin4 = crate::EnumBitfieldStruct<u8, Pin4_SPEC>;
    impl Pin4 {
        #[doc = "Trigger Input Pin signal is ignored"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Pin5_SPEC;
    pub type Pin5 = crate::EnumBitfieldStruct<u8, Pin5_SPEC>;
    impl Pin5 {
        #[doc = "Trigger Input Pin signal is ignored"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Pin6_SPEC;
    pub type Pin6 = crate::EnumBitfieldStruct<u8, Pin6_SPEC>;
    impl Pin6 {
        #[doc = "Trigger Input Pin signal is ignored"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Pin7_SPEC;
    pub type Pin7 = crate::EnumBitfieldStruct<u8, Pin7_SPEC>;
    impl Pin7 {
        #[doc = "Trigger Input Pin signal is ignored"]
        pub const CONST_00: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tl1St_SPEC;
impl crate::sealed::RegSpec for Tl1St_SPEC {
    type DataType = u32;
}
#[doc = "TG Line 1 Suspend Targets\n resetvalue={Debug Reset:0x0}"]
pub type Tl1St = crate::RegValueT<Tl1St_SPEC>;

impl Tl1St {
    #[doc = "CPU5 as Suspend Target   C5"]
    #[inline(always)]
    pub fn c0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, tl1st::C0, Tl1St_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,tl1st::C0, Tl1St_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU5 as Suspend Target   C5"]
    #[inline(always)]
    pub fn c1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, tl1st::C1, Tl1St_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,tl1st::C1, Tl1St_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU5 as Suspend Target   C5"]
    #[inline(always)]
    pub fn c2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, tl1st::C2, Tl1St_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,tl1st::C2, Tl1St_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU5 as Suspend Target   C5"]
    #[inline(always)]
    pub fn c3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, tl1st::C3, Tl1St_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,tl1st::C3, Tl1St_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU5 as Suspend Target   C5"]
    #[inline(always)]
    pub fn c4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, tl1st::C4, Tl1St_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,tl1st::C4, Tl1St_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU5 as Suspend Target   C5"]
    #[inline(always)]
    pub fn c5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, tl1st::C5, Tl1St_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,tl1st::C5, Tl1St_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Master 0 as Suspend Target. Prepared for not yet known bus masters on future SoCs  which need        suspend control. Once used in the future the bit will get the name of        the master."]
    #[inline(always)]
    pub fn m0(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, tl1st::M0, Tl1St_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1,1,0,tl1st::M0, Tl1St_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Master 1 as Suspend Target. Prepared for not yet known bus masters on future SoCs  which need        suspend control. Once used in the future the bit will get the name of        the master."]
    #[inline(always)]
    pub fn m1(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, tl1st::M1, Tl1St_SPEC, crate::common::RW> {
        crate::common::RegisterField::<25,0x1,1,0,tl1st::M1, Tl1St_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Master 2 as Suspend Target. Prepared for not yet known bus masters on future SoCs  which need        suspend control. Once used in the future the bit will get the name of        the master."]
    #[inline(always)]
    pub fn m2(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, tl1st::M2, Tl1St_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x1,1,0,tl1st::M2, Tl1St_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSSL1 as Suspend Target   HSS1"]
    #[inline(always)]
    pub fn hss1(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, tl1st::Hss1, Tl1St_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,tl1st::Hss1, Tl1St_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSSL0 as Suspend Target   HSS0"]
    #[inline(always)]
    pub fn hss0(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, tl1st::Hss0, Tl1St_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,tl1st::Hss0, Tl1St_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DMA as Suspend Target   DMA"]
    #[inline(always)]
    pub fn dma(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, tl1st::Dma, Tl1St_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,tl1st::Dma, Tl1St_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM as Suspend Target   HSM"]
    #[inline(always)]
    pub fn hsm(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, tl1st::Hsm, Tl1St_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,tl1st::Hsm, Tl1St_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Tl1St {
    #[inline(always)]
    fn default() -> Tl1St {
        <crate::RegValueT<Tl1St_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tl1st {
    pub struct C0_SPEC;
    pub type C0 = crate::EnumBitfieldStruct<u8, C0_SPEC>;
    impl C0 {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Target suspended by Trigger Line 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct C1_SPEC;
    pub type C1 = crate::EnumBitfieldStruct<u8, C1_SPEC>;
    impl C1 {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Target suspended by Trigger Line 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct C2_SPEC;
    pub type C2 = crate::EnumBitfieldStruct<u8, C2_SPEC>;
    impl C2 {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Target suspended by Trigger Line 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct C3_SPEC;
    pub type C3 = crate::EnumBitfieldStruct<u8, C3_SPEC>;
    impl C3 {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Target suspended by Trigger Line 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct C4_SPEC;
    pub type C4 = crate::EnumBitfieldStruct<u8, C4_SPEC>;
    impl C4 {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Target suspended by Trigger Line 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct C5_SPEC;
    pub type C5 = crate::EnumBitfieldStruct<u8, C5_SPEC>;
    impl C5 {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Target suspended by Trigger Line 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct M0_SPEC;
    pub type M0 = crate::EnumBitfieldStruct<u8, M0_SPEC>;
    impl M0 {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Target suspended by Trigger Line 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct M1_SPEC;
    pub type M1 = crate::EnumBitfieldStruct<u8, M1_SPEC>;
    impl M1 {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Target suspended by Trigger Line 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct M2_SPEC;
    pub type M2 = crate::EnumBitfieldStruct<u8, M2_SPEC>;
    impl M2 {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Target suspended by Trigger Line 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hss1_SPEC;
    pub type Hss1 = crate::EnumBitfieldStruct<u8, Hss1_SPEC>;
    impl Hss1 {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Target suspended by Trigger Line 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hss0_SPEC;
    pub type Hss0 = crate::EnumBitfieldStruct<u8, Hss0_SPEC>;
    impl Hss0 {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Target suspended by Trigger Line 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Dma_SPEC;
    pub type Dma = crate::EnumBitfieldStruct<u8, Dma_SPEC>;
    impl Dma {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Target suspended by Trigger Line 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hsm_SPEC;
    pub type Hsm = crate::EnumBitfieldStruct<u8, Hsm_SPEC>;
    impl Hsm {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Target suspended by Trigger Line 1"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tlc_SPEC;
impl crate::sealed::RegSpec for Tlc_SPEC {
    type DataType = u32;
}
#[doc = "TG Line Control\n resetvalue={Debug Reset:0x0}"]
pub type Tlc = crate::RegValueT<Tlc_SPEC>;

impl Tlc {
    #[doc = "TG Line Signal Processing   TLSP7"]
    #[inline(always)]
    pub fn tlsp1(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, tlc::Tlsp1, Tlc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,tlc::Tlsp1, Tlc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TG Line Signal Processing   TLSP7"]
    #[inline(always)]
    pub fn tlsp2(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, tlc::Tlsp2, Tlc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,tlc::Tlsp2, Tlc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TG Line Signal Processing   TLSP7"]
    #[inline(always)]
    pub fn tlsp3(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, tlc::Tlsp3, Tlc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,tlc::Tlsp3, Tlc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TG Line Signal Processing   TLSP7"]
    #[inline(always)]
    pub fn tlsp4(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, tlc::Tlsp4, Tlc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,tlc::Tlsp4, Tlc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TG Line Signal Processing   TLSP7"]
    #[inline(always)]
    pub fn tlsp5(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, tlc::Tlsp5, Tlc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,tlc::Tlsp5, Tlc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TG Line Signal Processing   TLSP7"]
    #[inline(always)]
    pub fn tlsp6(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, tlc::Tlsp6, Tlc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,tlc::Tlsp6, Tlc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TG Line Signal Processing   TLSP7"]
    #[inline(always)]
    pub fn tlsp7(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, tlc::Tlsp7, Tlc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,tlc::Tlsp7, Tlc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Tlc {
    #[inline(always)]
    fn default() -> Tlc {
        <crate::RegValueT<Tlc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tlc {
    pub struct Tlsp1_SPEC;
    pub type Tlsp1 = crate::EnumBitfieldStruct<u8, Tlsp1_SPEC>;
    impl Tlsp1 {
        #[doc = "0 Signal is unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal is inverted"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Signal is forced inactive"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 Signal is forced active"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "4 Any signal edge is converted to a pulse"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "5 Positive signal edge is converted to a pulse"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "6 Negative signal edge is converted to a pulse"]
        pub const CONST_66: Self = Self::new(6);
    }
    pub struct Tlsp2_SPEC;
    pub type Tlsp2 = crate::EnumBitfieldStruct<u8, Tlsp2_SPEC>;
    impl Tlsp2 {
        #[doc = "0 Signal is unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal is inverted"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Signal is forced inactive"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 Signal is forced active"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "4 Any signal edge is converted to a pulse"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "5 Positive signal edge is converted to a pulse"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "6 Negative signal edge is converted to a pulse"]
        pub const CONST_66: Self = Self::new(6);
    }
    pub struct Tlsp3_SPEC;
    pub type Tlsp3 = crate::EnumBitfieldStruct<u8, Tlsp3_SPEC>;
    impl Tlsp3 {
        #[doc = "0 Signal is unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal is inverted"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Signal is forced inactive"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 Signal is forced active"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "4 Any signal edge is converted to a pulse"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "5 Positive signal edge is converted to a pulse"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "6 Negative signal edge is converted to a pulse"]
        pub const CONST_66: Self = Self::new(6);
    }
    pub struct Tlsp4_SPEC;
    pub type Tlsp4 = crate::EnumBitfieldStruct<u8, Tlsp4_SPEC>;
    impl Tlsp4 {
        #[doc = "0 Signal is unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal is inverted"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Signal is forced inactive"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 Signal is forced active"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "4 Any signal edge is converted to a pulse"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "5 Positive signal edge is converted to a pulse"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "6 Negative signal edge is converted to a pulse"]
        pub const CONST_66: Self = Self::new(6);
    }
    pub struct Tlsp5_SPEC;
    pub type Tlsp5 = crate::EnumBitfieldStruct<u8, Tlsp5_SPEC>;
    impl Tlsp5 {
        #[doc = "0 Signal is unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal is inverted"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Signal is forced inactive"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 Signal is forced active"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "4 Any signal edge is converted to a pulse"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "5 Positive signal edge is converted to a pulse"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "6 Negative signal edge is converted to a pulse"]
        pub const CONST_66: Self = Self::new(6);
    }
    pub struct Tlsp6_SPEC;
    pub type Tlsp6 = crate::EnumBitfieldStruct<u8, Tlsp6_SPEC>;
    impl Tlsp6 {
        #[doc = "0 Signal is unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal is inverted"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Signal is forced inactive"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 Signal is forced active"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "4 Any signal edge is converted to a pulse"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "5 Positive signal edge is converted to a pulse"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "6 Negative signal edge is converted to a pulse"]
        pub const CONST_66: Self = Self::new(6);
    }
    pub struct Tlsp7_SPEC;
    pub type Tlsp7 = crate::EnumBitfieldStruct<u8, Tlsp7_SPEC>;
    impl Tlsp7 {
        #[doc = "0 Signal is unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal is inverted"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Signal is forced inactive"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 Signal is forced active"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "4 Any signal edge is converted to a pulse"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "5 Positive signal edge is converted to a pulse"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "6 Negative signal edge is converted to a pulse"]
        pub const CONST_66: Self = Self::new(6);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TlcCx_SPEC;
impl crate::sealed::RegSpec for TlcCx_SPEC {
    type DataType = u32;
}
#[doc = "TG Line Counter Control\n resetvalue={Debug Reset:0x0}"]
pub type TlcCx = crate::RegValueT<TlcCx_SPEC>;

impl TlcCx {
    #[doc = "Trigger Line to Counter Routing   TGL"]
    #[inline(always)]
    pub fn tgl(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, tlccx::Tgl, TlcCx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,tlccx::Tgl, TlcCx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Level or Edge Counting   LE"]
    #[inline(always)]
    pub fn le(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, tlccx::Le, TlcCx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,tlccx::Le, TlcCx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear and Enable Counter s    CLR. Clears the counter value and enables a stopped counter. Counter reset state is stopped."]
    #[inline(always)]
    pub fn clr(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, tlccx::Clr, TlcCx_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0x3,1,0,tlccx::Clr, TlcCx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Stop Counter s    STOP"]
    #[inline(always)]
    pub fn stop(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, tlccx::Stop, TlcCx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<12,0x3,1,0,tlccx::Stop, TlcCx_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for TlcCx {
    #[inline(always)]
    fn default() -> TlcCx {
        <crate::RegValueT<TlcCx_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tlccx {
    pub struct Tgl_SPEC;
    pub type Tgl = crate::EnumBitfieldStruct<u8, Tgl_SPEC>;
    impl Tgl {
        #[doc = "Counter disabled"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Le_SPEC;
    pub type Le = crate::EnumBitfieldStruct<u8, Le_SPEC>;
    impl Le {
        #[doc = "0 Single shot  Count while TG Line is active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Single shot  Count while TG Line is inactive"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Count while TG Line is active"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 Count while TG Line is inactive"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "4 Count positive edges"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "5 Count negative edges"]
        pub const CONST_55: Self = Self::new(5);
    }
    pub struct Clr_SPEC;
    pub type Clr = crate::EnumBitfieldStruct<u8, Clr_SPEC>;
    impl Clr {
        #[doc = "0 No effect"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 This TLCVx counter only"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 All TLCVx counters"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Stop_SPEC;
    pub type Stop = crate::EnumBitfieldStruct<u8, Stop_SPEC>;
    impl Stop {
        #[doc = "0 No effect"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Stop TLCVx counter"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Stop all TLCVx counters"]
        pub const CONST_22: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tlche_SPEC;
impl crate::sealed::RegSpec for Tlche_SPEC {
    type DataType = u32;
}
#[doc = "TG Line Capture and Hold Enable\n resetvalue={Debug Reset:0x0}"]
pub type Tlche = crate::RegValueT<Tlche_SPEC>;

impl Tlche {
    #[doc = "Capture and Hold Enable for Trigger Line 3   TL3"]
    #[inline(always)]
    pub fn tl1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, tlche::Tl1, Tlche_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,tlche::Tl1, Tlche_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Capture and Hold Enable for Trigger Line 3   TL3"]
    #[inline(always)]
    pub fn tl2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, tlche::Tl2, Tlche_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,tlche::Tl2, Tlche_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Capture and Hold Enable for Trigger Line 3   TL3"]
    #[inline(always)]
    pub fn tl3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, tlche::Tl3, Tlche_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,tlche::Tl3, Tlche_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Tlche {
    #[inline(always)]
    fn default() -> Tlche {
        <crate::RegValueT<Tlche_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tlche {
    pub struct Tl1_SPEC;
    pub type Tl1 = crate::EnumBitfieldStruct<u8, Tl1_SPEC>;
    impl Tl1 {
        #[doc = "0 Disabled. The captured TG Line value is cleared."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tl2_SPEC;
    pub type Tl2 = crate::EnumBitfieldStruct<u8, Tl2_SPEC>;
    impl Tl2 {
        #[doc = "0 Disabled. The captured TG Line value is cleared."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tl3_SPEC;
    pub type Tl3 = crate::EnumBitfieldStruct<u8, Tl3_SPEC>;
    impl Tl3 {
        #[doc = "0 Disabled. The captured TG Line value is cleared."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tlchs_SPEC;
impl crate::sealed::RegSpec for Tlchs_SPEC {
    type DataType = u32;
}
#[doc = "TG Line Capture and Hold Clear\n resetvalue={Debug Reset:0x0}"]
pub type Tlchs = crate::RegValueT<Tlchs_SPEC>;

impl Tlchs {
    #[doc = "Capture and Hold Clear for Trigger Line 3   TL3"]
    #[inline(always)]
    pub fn tl1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, tlchs::Tl1, Tlchs_SPEC, crate::common::W> {
        crate::common::RegisterField::<1,0x1,1,0,tlchs::Tl1, Tlchs_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Capture and Hold Clear for Trigger Line 3   TL3"]
    #[inline(always)]
    pub fn tl2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, tlchs::Tl2, Tlchs_SPEC, crate::common::W> {
        crate::common::RegisterField::<2,0x1,1,0,tlchs::Tl2, Tlchs_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Capture and Hold Clear for Trigger Line 3   TL3"]
    #[inline(always)]
    pub fn tl3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, tlchs::Tl3, Tlchs_SPEC, crate::common::W> {
        crate::common::RegisterField::<3,0x1,1,0,tlchs::Tl3, Tlchs_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Tlchs {
    #[inline(always)]
    fn default() -> Tlchs {
        <crate::RegValueT<Tlchs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tlchs {
    pub struct Tl1_SPEC;
    pub type Tl1 = crate::EnumBitfieldStruct<u8, Tl1_SPEC>;
    impl Tl1 {
        #[doc = "0 Hold bit value is unchanged for this TL"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Hold bit value is cleared"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tl2_SPEC;
    pub type Tl2 = crate::EnumBitfieldStruct<u8, Tl2_SPEC>;
    impl Tl2 {
        #[doc = "0 Hold bit value is unchanged for this TL"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Hold bit value is cleared"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tl3_SPEC;
    pub type Tl3 = crate::EnumBitfieldStruct<u8, Tl3_SPEC>;
    impl Tl3 {
        #[doc = "0 Hold bit value is unchanged for this TL"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Hold bit value is cleared"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TlcVx_SPEC;
impl crate::sealed::RegSpec for TlcVx_SPEC {
    type DataType = u32;
}
#[doc = "TG Line Counter Value\n resetvalue={Debug Reset:0x0}"]
pub type TlcVx = crate::RegValueT<TlcVx_SPEC>;

impl TlcVx {
    #[doc = "Count Value   CV"]
    #[inline(always)]
    pub fn cv(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffffff, 1, 0, u32, TlcVx_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7fffffff,1,0,u32, TlcVx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Sticky Overflow Bit   SO. This bit is set by hardware when Count Value  30 0  is 0x7FFFFFFF. It is cleared by software like CV with TLCVx.CLR."]
    #[inline(always)]
    pub fn so(self) -> crate::common::RegisterFieldBool<31, 1, 0, TlcVx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, TlcVx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for TlcVx {
    #[inline(always)]
    fn default() -> TlcVx {
        <crate::RegValueT<TlcVx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tls_SPEC;
impl crate::sealed::RegSpec for Tls_SPEC {
    type DataType = u32;
}
#[doc = "TG Line State\n resetvalue={Debug Reset:0x0}"]
pub type Tls = crate::RegValueT<Tls_SPEC>;

impl Tls {
    #[doc = "Current State of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, tls::Tl1, Tls_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,tls::Tl1, Tls_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Current State of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, tls::Tl2, Tls_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,tls::Tl2, Tls_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Current State of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, tls::Tl3, Tls_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1,1,0,tls::Tl3, Tls_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Current State of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, tls::Tl4, Tls_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x1,1,0,tls::Tl4, Tls_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Current State of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, tls::Tl5, Tls_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x1,1,0,tls::Tl5, Tls_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Current State of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, tls::Tl6, Tls_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x1,1,0,tls::Tl6, Tls_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Current State of Trigger Line 7   TL7"]
    #[inline(always)]
    pub fn tl7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, tls::Tl7, Tls_SPEC, crate::common::R> {
        crate::common::RegisterField::<7,0x1,1,0,tls::Tl7, Tls_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Tls {
    #[inline(always)]
    fn default() -> Tls {
        <crate::RegValueT<Tls_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tls {
    pub struct Tl1_SPEC;
    pub type Tl1 = crate::EnumBitfieldStruct<u8, Tl1_SPEC>;
    impl Tl1 {
        #[doc = "0 Inactive"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tl2_SPEC;
    pub type Tl2 = crate::EnumBitfieldStruct<u8, Tl2_SPEC>;
    impl Tl2 {
        #[doc = "0 Inactive"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tl3_SPEC;
    pub type Tl3 = crate::EnumBitfieldStruct<u8, Tl3_SPEC>;
    impl Tl3 {
        #[doc = "0 Inactive"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tl4_SPEC;
    pub type Tl4 = crate::EnumBitfieldStruct<u8, Tl4_SPEC>;
    impl Tl4 {
        #[doc = "0 Inactive"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tl5_SPEC;
    pub type Tl5 = crate::EnumBitfieldStruct<u8, Tl5_SPEC>;
    impl Tl5 {
        #[doc = "0 Inactive"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tl6_SPEC;
    pub type Tl6 = crate::EnumBitfieldStruct<u8, Tl6_SPEC>;
    impl Tl6 {
        #[doc = "0 Inactive"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tl7_SPEC;
    pub type Tl7 = crate::EnumBitfieldStruct<u8, Tl7_SPEC>;
    impl Tl7 {
        #[doc = "0 Inactive"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tlt_SPEC;
impl crate::sealed::RegSpec for Tlt_SPEC {
    type DataType = u32;
}
#[doc = "TG Line Timer\n resetvalue={Debug Reset:0x0}"]
pub type Tlt = crate::RegValueT<Tlt_SPEC>;

impl Tlt {
    #[doc = "Timer to Trigger Line Routing   TGL"]
    #[inline(always)]
    pub fn tgl(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, tlt::Tgl, Tlt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,tlt::Tgl, Tlt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TG Line Value when Timer Value is Zero   VTZ"]
    #[inline(always)]
    pub fn vtz(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, tlt::Vtz, Tlt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,tlt::Vtz, Tlt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reload Timer   RL"]
    #[inline(always)]
    pub fn rl(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, tlt::Rl, Tlt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,tlt::Rl, Tlt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer Value   TIM. Timer value which is automatically decremented if not 0. The timer frequency is fixed to f SPB divided by 4."]
    #[inline(always)]
    pub fn tim(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Tlt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Tlt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Tlt {
    #[inline(always)]
    fn default() -> Tlt {
        <crate::RegValueT<Tlt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tlt {
    pub struct Tgl_SPEC;
    pub type Tgl = crate::EnumBitfieldStruct<u8, Tgl_SPEC>;
    impl Tgl {
        #[doc = "Timer disabled"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Vtz_SPEC;
    pub type Vtz = crate::EnumBitfieldStruct<u8, Vtz_SPEC>;
    impl Vtz {
        #[doc = "0 TG Line value is 0 when Timer value is 0  1 when Timer value is not 0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 TG Line value is 1 when Timer value is 0  0 when Timer value is not 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rl_SPEC;
    pub type Rl = crate::EnumBitfieldStruct<u8, Rl_SPEC>;
    impl Rl {
        #[doc = "0 Timer value is not reloaded"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer value is reloaded for cyclic pulse generation  0 when Timer value is not 0"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tltth_SPEC;
impl crate::sealed::RegSpec for Tltth_SPEC {
    type DataType = u32;
}
#[doc = "TG Lines for Trigger to Host\n resetvalue={Debug Reset:0x0}"]
pub type Tltth = crate::RegValueT<Tltth_SPEC>;

impl Tltth {
    #[doc = "TG Line Enabling for Trigger to Host  TRIG    TL7"]
    #[inline(always)]
    pub fn tl1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, tltth::Tl1, Tltth_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,tltth::Tl1, Tltth_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TG Line Enabling for Trigger to Host  TRIG    TL7"]
    #[inline(always)]
    pub fn tl2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, tltth::Tl2, Tltth_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,tltth::Tl2, Tltth_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TG Line Enabling for Trigger to Host  TRIG    TL7"]
    #[inline(always)]
    pub fn tl3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, tltth::Tl3, Tltth_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,tltth::Tl3, Tltth_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TG Line Enabling for Trigger to Host  TRIG    TL7"]
    #[inline(always)]
    pub fn tl4(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, tltth::Tl4, Tltth_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,tltth::Tl4, Tltth_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TG Line Enabling for Trigger to Host  TRIG    TL7"]
    #[inline(always)]
    pub fn tl5(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, tltth::Tl5, Tltth_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,tltth::Tl5, Tltth_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TG Line Enabling for Trigger to Host  TRIG    TL7"]
    #[inline(always)]
    pub fn tl6(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, tltth::Tl6, Tltth_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,tltth::Tl6, Tltth_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TG Line Enabling for Trigger to Host  TRIG    TL7"]
    #[inline(always)]
    pub fn tl7(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, tltth::Tl7, Tltth_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,tltth::Tl7, Tltth_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Tltth {
    #[inline(always)]
    fn default() -> Tltth {
        <crate::RegValueT<Tltth_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tltth {
    pub struct Tl1_SPEC;
    pub type Tl1 = crate::EnumBitfieldStruct<u8, Tl1_SPEC>;
    impl Tl1 {
        #[doc = "0 Not enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled as low priority host trigger"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "3 Enabled as high priority host trigger"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Tl2_SPEC;
    pub type Tl2 = crate::EnumBitfieldStruct<u8, Tl2_SPEC>;
    impl Tl2 {
        #[doc = "0 Not enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled as low priority host trigger"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "3 Enabled as high priority host trigger"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Tl3_SPEC;
    pub type Tl3 = crate::EnumBitfieldStruct<u8, Tl3_SPEC>;
    impl Tl3 {
        #[doc = "0 Not enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled as low priority host trigger"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "3 Enabled as high priority host trigger"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Tl4_SPEC;
    pub type Tl4 = crate::EnumBitfieldStruct<u8, Tl4_SPEC>;
    impl Tl4 {
        #[doc = "0 Not enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled as low priority host trigger"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "3 Enabled as high priority host trigger"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Tl5_SPEC;
    pub type Tl5 = crate::EnumBitfieldStruct<u8, Tl5_SPEC>;
    impl Tl5 {
        #[doc = "0 Not enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled as low priority host trigger"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "3 Enabled as high priority host trigger"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Tl6_SPEC;
    pub type Tl6 = crate::EnumBitfieldStruct<u8, Tl6_SPEC>;
    impl Tl6 {
        #[doc = "0 Not enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled as low priority host trigger"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "3 Enabled as high priority host trigger"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Tl7_SPEC;
    pub type Tl7 = crate::EnumBitfieldStruct<u8, Tl7_SPEC>;
    impl Tl7 {
        #[doc = "0 Not enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled as low priority host trigger"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "3 Enabled as high priority host trigger"]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Topps_SPEC;
impl crate::sealed::RegSpec for Topps_SPEC {
    type DataType = u32;
}
#[doc = "TG Output Pins Pulse Stretcher\n resetvalue={Debug Reset:0x0}"]
pub type Topps = crate::RegValueT<Topps_SPEC>;

impl Topps {
    #[doc = "Pulse Stretch Control for Trigger Pin 7   PIN7"]
    #[inline(always)]
    pub fn pin0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, topps::Pin0, Topps_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,topps::Pin0, Topps_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pulse Stretch Control for Trigger Pin 7   PIN7"]
    #[inline(always)]
    pub fn pin1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, topps::Pin1, Topps_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,topps::Pin1, Topps_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pulse Stretch Control for Trigger Pin 7   PIN7"]
    #[inline(always)]
    pub fn pin2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, topps::Pin2, Topps_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,topps::Pin2, Topps_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pulse Stretch Control for Trigger Pin 7   PIN7"]
    #[inline(always)]
    pub fn pin3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, topps::Pin3, Topps_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,topps::Pin3, Topps_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pulse Stretch Control for Trigger Pin 7   PIN7"]
    #[inline(always)]
    pub fn pin4(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, topps::Pin4, Topps_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,topps::Pin4, Topps_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pulse Stretch Control for Trigger Pin 7   PIN7"]
    #[inline(always)]
    pub fn pin5(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, topps::Pin5, Topps_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,topps::Pin5, Topps_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pulse Stretch Control for Trigger Pin 7   PIN7"]
    #[inline(always)]
    pub fn pin6(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, topps::Pin6, Topps_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,topps::Pin6, Topps_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pulse Stretch Control for Trigger Pin 7   PIN7"]
    #[inline(always)]
    pub fn pin7(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, topps::Pin7, Topps_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,topps::Pin7, Topps_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Topps {
    #[inline(always)]
    fn default() -> Topps {
        <crate::RegValueT<Topps_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod topps {
    pub struct Pin0_SPEC;
    pub type Pin0 = crate::EnumBitfieldStruct<u8, Pin0_SPEC>;
    impl Pin0 {
        #[doc = "0 No pulse stretching"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Minimum pulse width is 2 SPB clocks"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Minimum pulse width is 4 SPB clocks"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Pin1_SPEC;
    pub type Pin1 = crate::EnumBitfieldStruct<u8, Pin1_SPEC>;
    impl Pin1 {
        #[doc = "0 No pulse stretching"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Minimum pulse width is 2 SPB clocks"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Minimum pulse width is 4 SPB clocks"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Pin2_SPEC;
    pub type Pin2 = crate::EnumBitfieldStruct<u8, Pin2_SPEC>;
    impl Pin2 {
        #[doc = "0 No pulse stretching"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Minimum pulse width is 2 SPB clocks"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Minimum pulse width is 4 SPB clocks"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Pin3_SPEC;
    pub type Pin3 = crate::EnumBitfieldStruct<u8, Pin3_SPEC>;
    impl Pin3 {
        #[doc = "0 No pulse stretching"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Minimum pulse width is 2 SPB clocks"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Minimum pulse width is 4 SPB clocks"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Pin4_SPEC;
    pub type Pin4 = crate::EnumBitfieldStruct<u8, Pin4_SPEC>;
    impl Pin4 {
        #[doc = "0 No pulse stretching"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Minimum pulse width is 2 SPB clocks"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Minimum pulse width is 4 SPB clocks"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Pin5_SPEC;
    pub type Pin5 = crate::EnumBitfieldStruct<u8, Pin5_SPEC>;
    impl Pin5 {
        #[doc = "0 No pulse stretching"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Minimum pulse width is 2 SPB clocks"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Minimum pulse width is 4 SPB clocks"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Pin6_SPEC;
    pub type Pin6 = crate::EnumBitfieldStruct<u8, Pin6_SPEC>;
    impl Pin6 {
        #[doc = "0 No pulse stretching"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Minimum pulse width is 2 SPB clocks"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Minimum pulse width is 4 SPB clocks"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Pin7_SPEC;
    pub type Pin7 = crate::EnumBitfieldStruct<u8, Pin7_SPEC>;
    impl Pin7 {
        #[doc = "0 No pulse stretching"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Minimum pulse width is 2 SPB clocks"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Minimum pulse width is 4 SPB clocks"]
        pub const CONST_22: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Topr_SPEC;
impl crate::sealed::RegSpec for Topr_SPEC {
    type DataType = u32;
}
#[doc = "TG Output Pins Routing\n resetvalue={Debug Reset:0x0}"]
pub type Topr = crate::RegValueT<Topr_SPEC>;

impl Topr {
    #[doc = "Trigger Line to Trigger Pin 7 Routing"]
    #[inline(always)]
    pub fn pin0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, topr::Pin0, Topr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,topr::Pin0, Topr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to Trigger Pin 7 Routing"]
    #[inline(always)]
    pub fn pin1(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, topr::Pin1, Topr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,topr::Pin1, Topr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to Trigger Pin 7 Routing"]
    #[inline(always)]
    pub fn pin2(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, topr::Pin2, Topr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,topr::Pin2, Topr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to Trigger Pin 7 Routing"]
    #[inline(always)]
    pub fn pin3(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, topr::Pin3, Topr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,topr::Pin3, Topr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to Trigger Pin 7 Routing"]
    #[inline(always)]
    pub fn pin4(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, topr::Pin4, Topr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,topr::Pin4, Topr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to Trigger Pin 7 Routing"]
    #[inline(always)]
    pub fn pin5(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, topr::Pin5, Topr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,topr::Pin5, Topr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to Trigger Pin 7 Routing"]
    #[inline(always)]
    pub fn pin6(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, topr::Pin6, Topr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,topr::Pin6, Topr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to Trigger Pin 7 Routing"]
    #[inline(always)]
    pub fn pin7(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, topr::Pin7, Topr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,topr::Pin7, Topr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Topr {
    #[inline(always)]
    fn default() -> Topr {
        <crate::RegValueT<Topr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod topr {
    pub struct Pin0_SPEC;
    pub type Pin0 = crate::EnumBitfieldStruct<u8, Pin0_SPEC>;
    impl Pin0 {
        #[doc = "Trigger Output Pin is disabled."]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Pin1_SPEC;
    pub type Pin1 = crate::EnumBitfieldStruct<u8, Pin1_SPEC>;
    impl Pin1 {
        #[doc = "Trigger Output Pin is disabled."]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Pin2_SPEC;
    pub type Pin2 = crate::EnumBitfieldStruct<u8, Pin2_SPEC>;
    impl Pin2 {
        #[doc = "Trigger Output Pin is disabled."]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Pin3_SPEC;
    pub type Pin3 = crate::EnumBitfieldStruct<u8, Pin3_SPEC>;
    impl Pin3 {
        #[doc = "Trigger Output Pin is disabled."]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Pin4_SPEC;
    pub type Pin4 = crate::EnumBitfieldStruct<u8, Pin4_SPEC>;
    impl Pin4 {
        #[doc = "Trigger Output Pin is disabled."]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Pin5_SPEC;
    pub type Pin5 = crate::EnumBitfieldStruct<u8, Pin5_SPEC>;
    impl Pin5 {
        #[doc = "Trigger Output Pin is disabled."]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Pin6_SPEC;
    pub type Pin6 = crate::EnumBitfieldStruct<u8, Pin6_SPEC>;
    impl Pin6 {
        #[doc = "Trigger Output Pin is disabled."]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Pin7_SPEC;
    pub type Pin7 = crate::EnumBitfieldStruct<u8, Pin7_SPEC>;
    impl Pin7 {
        #[doc = "Trigger Output Pin is disabled."]
        pub const CONST_00: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrCx_SPEC;
impl crate::sealed::RegSpec for TrCx_SPEC {
    type DataType = u32;
}
#[doc = "TG Routing for CPU0\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x2,Debug Reset:0x0,Application Reset:0x0,IOClient Reset:0x0}"]
pub type TrCx = crate::RegValueT<TrCx_SPEC>;

impl TrCx {
    #[doc = "HALT to Trigger Line Routing   HALT"]
    #[inline(always)]
    pub fn halt(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, trcx::Halt, TrCx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,trcx::Halt, TrCx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "BRKOUT to Trigger Line Routing   BRKOUT"]
    #[inline(always)]
    pub fn brkout(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, trcx::Brkout, TrCx_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,trcx::Brkout, TrCx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "BRKOUT to Trigger Line 1 Routing   BT1. This option is available independent of the routing with BRKOUT .        The use case is that BRKOUT is  in addition  a source for the suspend        generation with Trigger Line 1."]
    #[inline(always)]
    pub fn bt1(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, trcx::Bt1, TrCx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,trcx::Bt1, TrCx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to BRKIN Routing   BRKIN"]
    #[inline(always)]
    pub fn brkin(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, trcx::Brkin, TrCx_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0xf,1,0,trcx::Brkin, TrCx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to SUSIN Routing   SUSIN. Note  Use TL1ST in case of Trigger Line 1."]
    #[inline(always)]
    pub fn susin(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, trcx::Susin, TrCx_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xf,1,0,trcx::Susin, TrCx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for TrCx {
    #[inline(always)]
    fn default() -> TrCx {
        <crate::RegValueT<TrCx_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod trcx {
    pub struct Halt_SPEC;
    pub type Halt = crate::EnumBitfieldStruct<u8, Halt_SPEC>;
    impl Halt {
        #[doc = "HALT signal is ignored"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Brkout_SPEC;
    pub type Brkout = crate::EnumBitfieldStruct<u8, Brkout_SPEC>;
    impl Brkout {
        #[doc = "BRKOUT signal is ignored"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Routing to Trigger Line 1  or use BT1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Bt1_SPEC;
    pub type Bt1 = crate::EnumBitfieldStruct<u8, Bt1_SPEC>;
    impl Bt1 {
        #[doc = "0 No effect"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 BRKOUT is routed to Trigger Line 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Brkin_SPEC;
    pub type Brkin = crate::EnumBitfieldStruct<u8, Brkin_SPEC>;
    impl Brkin {
        #[doc = "BRKIN signal is always inactive"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Susin_SPEC;
    pub type Susin = crate::EnumBitfieldStruct<u8, Susin_SPEC>;
    impl Susin {
        #[doc = "SUSIN signal is always inactive. 0 SUSIN signal is        always inactive"]
        pub const CONST_00: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TreCx_SPEC;
impl crate::sealed::RegSpec for TreCx_SPEC {
    type DataType = u32;
}
#[doc = "TG Routing Events of CPU0\n resetvalue={Debug Reset:0x0}"]
pub type TreCx = crate::RegValueT<TreCx_SPEC>;

impl TreCx {
    #[doc = "TRxEVT to Trigger Line Routing TR0EV"]
    #[inline(always)]
    pub fn tr0ev(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, trecx::Tr0Ev, TreCx_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,trecx::Tr0Ev, TreCx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TRxEVT to Trigger Line Routing TR2EV"]
    #[inline(always)]
    pub fn tr2ev(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, trecx::Tr2Ev, TreCx_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,trecx::Tr2Ev, TreCx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TRxEVT to Trigger Line Routing TR4EV"]
    #[inline(always)]
    pub fn tr4ev(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, trecx::Tr4Ev, TreCx_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,trecx::Tr4Ev, TreCx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TRxEVT to Trigger Line Routing TR6EV"]
    #[inline(always)]
    pub fn tr6ev(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, trecx::Tr6Ev, TreCx_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xf,1,0,trecx::Tr6Ev, TreCx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for TreCx {
    #[inline(always)]
    fn default() -> TreCx {
        <crate::RegValueT<TreCx_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod trecx {
    pub struct Tr0Ev_SPEC;
    pub type Tr0Ev = crate::EnumBitfieldStruct<u8, Tr0Ev_SPEC>;
    impl Tr0Ev {
        #[doc = "Signal is ignored"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Tr2Ev_SPEC;
    pub type Tr2Ev = crate::EnumBitfieldStruct<u8, Tr2Ev_SPEC>;
    impl Tr2Ev {
        #[doc = "Signal is ignored"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Tr4Ev_SPEC;
    pub type Tr4Ev = crate::EnumBitfieldStruct<u8, Tr4Ev_SPEC>;
    impl Tr4Ev {
        #[doc = "Signal is ignored"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Tr6Ev_SPEC;
    pub type Tr6Ev = crate::EnumBitfieldStruct<u8, Tr6Ev_SPEC>;
    impl Tr6Ev {
        #[doc = "Signal is ignored"]
        pub const CONST_00: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trhsm_SPEC;
impl crate::sealed::RegSpec for Trhsm_SPEC {
    type DataType = u32;
}
#[doc = "TG Routing for HSMControl\n resetvalue={Debug Reset:0x0}"]
pub type Trhsm = crate::RegValueT<Trhsm_SPEC>;

impl Trhsm {
    #[doc = "HALT to Trigger Line Routing   HALT"]
    #[inline(always)]
    pub fn halt(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, trhsm::Halt, Trhsm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,trhsm::Halt, Trhsm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "BRKOUT to Trigger Line Routing   BRKOUT"]
    #[inline(always)]
    pub fn brkout(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, trhsm::Brkout, Trhsm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,trhsm::Brkout, Trhsm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "BRKOUT to Trigger Line 1 Routing   BT1. BRKOUT signal is used in addition for suspend generation with Trigger Line 1."]
    #[inline(always)]
    pub fn bt1(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, trhsm::Bt1, Trhsm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,trhsm::Bt1, Trhsm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to BRKIN Routing   BRKIN"]
    #[inline(always)]
    pub fn brkin(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, trhsm::Brkin, Trhsm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0xf,1,0,trhsm::Brkin, Trhsm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to SUSIN Routing   SUSIN. Note  Use TL1ST in case of Trigger Line 1."]
    #[inline(always)]
    pub fn susin(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, trhsm::Susin, Trhsm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xf,1,0,trhsm::Susin, Trhsm_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Trhsm {
    #[inline(always)]
    fn default() -> Trhsm {
        <crate::RegValueT<Trhsm_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod trhsm {
    pub struct Halt_SPEC;
    pub type Halt = crate::EnumBitfieldStruct<u8, Halt_SPEC>;
    impl Halt {
        #[doc = "HALT signal is ignored"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Brkout_SPEC;
    pub type Brkout = crate::EnumBitfieldStruct<u8, Brkout_SPEC>;
    impl Brkout {
        #[doc = "BRKOUT signal is ignored"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Routing to Trigger Line 1  or use BT1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Bt1_SPEC;
    pub type Bt1 = crate::EnumBitfieldStruct<u8, Bt1_SPEC>;
    impl Bt1 {
        #[doc = "0 No effect"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 BRKOUT is routed to Trigger Line 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Brkin_SPEC;
    pub type Brkin = crate::EnumBitfieldStruct<u8, Brkin_SPEC>;
    impl Brkin {
        #[doc = "BRKIN signal is always inactive"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Susin_SPEC;
    pub type Susin = crate::EnumBitfieldStruct<u8, Susin_SPEC>;
    impl Susin {
        #[doc = "SUSIN signal is always inactive"]
        pub const CONST_00: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trigc_SPEC;
impl crate::sealed::RegSpec for Trigc_SPEC {
    type DataType = u32;
}
#[doc = "Clear Trigger to Host Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Trigc = crate::RegValueT<Trigc_SPEC>;

impl Trigc {
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, trigc::TrGx0, Trigc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,trigc::TrGx0, Trigc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, trigc::TrGx1, Trigc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,trigc::TrGx1, Trigc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, trigc::TrGx2, Trigc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1,1,0,trigc::TrGx2, Trigc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, trigc::TrGx3, Trigc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x1,1,0,trigc::TrGx3, Trigc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, trigc::TrGx4, Trigc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,trigc::TrGx4, Trigc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, trigc::TrGx5, Trigc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x1,1,0,trigc::TrGx5, Trigc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, trigc::TrGx6, Trigc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x1,1,0,trigc::TrGx6, Trigc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, trigc::TrGx7, Trigc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<7,0x1,1,0,trigc::TrGx7, Trigc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, trigc::TrGx8, Trigc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x1,1,0,trigc::TrGx8, Trigc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, trigc::TrGx9, Trigc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<9,0x1,1,0,trigc::TrGx9, Trigc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, trigc::TrGx10, Trigc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<10,0x1,1,0,trigc::TrGx10, Trigc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, trigc::TrGx11, Trigc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<11,0x1,1,0,trigc::TrGx11, Trigc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, trigc::TrGx12, Trigc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<12,0x1,1,0,trigc::TrGx12, Trigc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, trigc::TrGx13, Trigc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<13,0x1,1,0,trigc::TrGx13, Trigc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, trigc::TrGx14, Trigc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<14,0x1,1,0,trigc::TrGx14, Trigc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Request Bits of most important register TRIGx   TRGx 15. From all TRIGx registers with any TRG  bit set the TRIGx with the lowest x is cleared. If any bit s  are set concurrently these bits will not be cleared but end up in the set state."]
    #[inline(always)]
    pub fn trgx_15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, trigc::TrGx15, Trigc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<15,0x1,1,0,trigc::TrGx15, Trigc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Index of most important register TRIGx   x. For TRIGx the highest x available x   5 ."]
    #[inline(always)]
    pub fn x(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, trigc::X, Trigc_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,trigc::X, Trigc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Trigc {
    #[inline(always)]
    fn default() -> Trigc {
        <crate::RegValueT<Trigc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod trigc {
    pub struct TrGx0_SPEC;
    pub type TrGx0 = crate::EnumBitfieldStruct<u8, TrGx0_SPEC>;
    impl TrGx0 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx1_SPEC;
    pub type TrGx1 = crate::EnumBitfieldStruct<u8, TrGx1_SPEC>;
    impl TrGx1 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx2_SPEC;
    pub type TrGx2 = crate::EnumBitfieldStruct<u8, TrGx2_SPEC>;
    impl TrGx2 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx3_SPEC;
    pub type TrGx3 = crate::EnumBitfieldStruct<u8, TrGx3_SPEC>;
    impl TrGx3 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx4_SPEC;
    pub type TrGx4 = crate::EnumBitfieldStruct<u8, TrGx4_SPEC>;
    impl TrGx4 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx5_SPEC;
    pub type TrGx5 = crate::EnumBitfieldStruct<u8, TrGx5_SPEC>;
    impl TrGx5 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx6_SPEC;
    pub type TrGx6 = crate::EnumBitfieldStruct<u8, TrGx6_SPEC>;
    impl TrGx6 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx7_SPEC;
    pub type TrGx7 = crate::EnumBitfieldStruct<u8, TrGx7_SPEC>;
    impl TrGx7 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx8_SPEC;
    pub type TrGx8 = crate::EnumBitfieldStruct<u8, TrGx8_SPEC>;
    impl TrGx8 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx9_SPEC;
    pub type TrGx9 = crate::EnumBitfieldStruct<u8, TrGx9_SPEC>;
    impl TrGx9 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx10_SPEC;
    pub type TrGx10 = crate::EnumBitfieldStruct<u8, TrGx10_SPEC>;
    impl TrGx10 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx11_SPEC;
    pub type TrGx11 = crate::EnumBitfieldStruct<u8, TrGx11_SPEC>;
    impl TrGx11 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx12_SPEC;
    pub type TrGx12 = crate::EnumBitfieldStruct<u8, TrGx12_SPEC>;
    impl TrGx12 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx13_SPEC;
    pub type TrGx13 = crate::EnumBitfieldStruct<u8, TrGx13_SPEC>;
    impl TrGx13 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx14_SPEC;
    pub type TrGx14 = crate::EnumBitfieldStruct<u8, TrGx14_SPEC>;
    impl TrGx14 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx15_SPEC;
    pub type TrGx15 = crate::EnumBitfieldStruct<u8, TrGx15_SPEC>;
    impl TrGx15 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct X_SPEC;
    pub type X = crate::EnumBitfieldStruct<u8, X_SPEC>;
    impl X {
        #[doc = "0 TRIG0 is shown"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "5 TRIG 5 is shown"]
        pub const CONST_55: Self = Self::new(5);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trigs_SPEC;
impl crate::sealed::RegSpec for Trigs_SPEC {
    type DataType = u32;
}
#[doc = "Set Trigger to Host Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type Trigs = crate::RegValueT<Trigs_SPEC>;

impl Trigs {
    #[doc = "Service Request Bit Number to Set   BITNUM. The bit TRGx y of register TRIGx is set  with  x   BITNUM div 16   and  y   BITNUM mod 16  . LOST SEQUENCE DEFINITION"]
    #[inline(always)]
    pub fn bitnum(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, trigs::Bitnum, Trigs_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xff,1,0,trigs::Bitnum, Trigs_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Trigs {
    #[inline(always)]
    fn default() -> Trigs {
        <crate::RegValueT<Trigs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod trigs {
    pub struct Bitnum_SPEC;
    pub type Bitnum = crate::EnumBitfieldStruct<u8, Bitnum_SPEC>;
    impl Bitnum {
        #[doc = "0 TRIG0.TRGx 0 is set."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "95 TRIG 5 .TRGx   95 mod 16   is set."]
        pub const CONST_9595: Self = Self::new(95);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TriGx_SPEC;
impl crate::sealed::RegSpec for TriGx_SPEC {
    type DataType = u32;
}
#[doc = "Trigger to Host Register 0\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0,Debug Reset:0x0,IOClient Reset:0x0}"]
pub type TriGx = crate::RegValueT<TriGx_SPEC>;

impl TriGx {
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, trigx::TrGx0, TriGx_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,trigx::TrGx0, TriGx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, trigx::TrGx1, TriGx_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,trigx::TrGx1, TriGx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, trigx::TrGx2, TriGx_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1,1,0,trigx::TrGx2, TriGx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, trigx::TrGx3, TriGx_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x1,1,0,trigx::TrGx3, TriGx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, trigx::TrGx4, TriGx_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,trigx::TrGx4, TriGx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, trigx::TrGx5, TriGx_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x1,1,0,trigx::TrGx5, TriGx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, trigx::TrGx6, TriGx_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x1,1,0,trigx::TrGx6, TriGx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, trigx::TrGx7, TriGx_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<7,0x1,1,0,trigx::TrGx7, TriGx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, trigx::TrGx8, TriGx_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x1,1,0,trigx::TrGx8, TriGx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, trigx::TrGx9, TriGx_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<9,0x1,1,0,trigx::TrGx9, TriGx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, trigx::TrGx10, TriGx_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<10,0x1,1,0,trigx::TrGx10, TriGx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, trigx::TrGx11, TriGx_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<11,0x1,1,0,trigx::TrGx11, TriGx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, trigx::TrGx12, TriGx_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<12,0x1,1,0,trigx::TrGx12, TriGx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, trigx::TrGx13, TriGx_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<13,0x1,1,0,trigx::TrGx13, TriGx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, trigx::TrGx14, TriGx_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<14,0x1,1,0,trigx::TrGx14, TriGx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Service Request Bits   TRGx 15. Are used by the tool software running on the SoC to request specific services  e.g. upload measurement data block  from the host. The bit is set by writing the correct bit number into the TRIGS register. To set TRGx y the number  x  16  y  must be written to TRIGS . All request bits of a specific TRIGx are cleared only when from all TRIG  registers with any TRG  bit set TRIGx happens to have the lowest x and   either TRIGC is read   or the IO READ  TRIG instruction is executed. In case of concurrent set and clear the affected bit s  will remain set. Bit numbers higher than 95  highest TRIGx register  stuck  0 ."]
    #[inline(always)]
    pub fn trgx_15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, trigx::TrGx15, TriGx_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<15,0x1,1,0,trigx::TrGx15, TriGx_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "TRIG register number   x"]
    #[inline(always)]
    pub fn x(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, TriGx_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, TriGx_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for TriGx {
    #[inline(always)]
    fn default() -> TriGx {
        <crate::RegValueT<TriGx_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod trigx {
    pub struct TrGx0_SPEC;
    pub type TrGx0 = crate::EnumBitfieldStruct<u8, TrGx0_SPEC>;
    impl TrGx0 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx1_SPEC;
    pub type TrGx1 = crate::EnumBitfieldStruct<u8, TrGx1_SPEC>;
    impl TrGx1 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx2_SPEC;
    pub type TrGx2 = crate::EnumBitfieldStruct<u8, TrGx2_SPEC>;
    impl TrGx2 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx3_SPEC;
    pub type TrGx3 = crate::EnumBitfieldStruct<u8, TrGx3_SPEC>;
    impl TrGx3 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx4_SPEC;
    pub type TrGx4 = crate::EnumBitfieldStruct<u8, TrGx4_SPEC>;
    impl TrGx4 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx5_SPEC;
    pub type TrGx5 = crate::EnumBitfieldStruct<u8, TrGx5_SPEC>;
    impl TrGx5 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx6_SPEC;
    pub type TrGx6 = crate::EnumBitfieldStruct<u8, TrGx6_SPEC>;
    impl TrGx6 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx7_SPEC;
    pub type TrGx7 = crate::EnumBitfieldStruct<u8, TrGx7_SPEC>;
    impl TrGx7 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx8_SPEC;
    pub type TrGx8 = crate::EnumBitfieldStruct<u8, TrGx8_SPEC>;
    impl TrGx8 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx9_SPEC;
    pub type TrGx9 = crate::EnumBitfieldStruct<u8, TrGx9_SPEC>;
    impl TrGx9 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx10_SPEC;
    pub type TrGx10 = crate::EnumBitfieldStruct<u8, TrGx10_SPEC>;
    impl TrGx10 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx11_SPEC;
    pub type TrGx11 = crate::EnumBitfieldStruct<u8, TrGx11_SPEC>;
    impl TrGx11 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx12_SPEC;
    pub type TrGx12 = crate::EnumBitfieldStruct<u8, TrGx12_SPEC>;
    impl TrGx12 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx13_SPEC;
    pub type TrGx13 = crate::EnumBitfieldStruct<u8, TrGx13_SPEC>;
    impl TrGx13 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx14_SPEC;
    pub type TrGx14 = crate::EnumBitfieldStruct<u8, TrGx14_SPEC>;
    impl TrGx14 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TrGx15_SPEC;
    pub type TrGx15 = crate::EnumBitfieldStruct<u8, TrGx15_SPEC>;
    impl TrGx15 {
        #[doc = "0 No request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service  x  16  y  requested."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trmc_SPEC;
impl crate::sealed::RegSpec for Trmc_SPEC {
    type DataType = u32;
}
#[doc = "TG Routing for MCDS Control\n resetvalue={Debug Reset:0x0}"]
pub type Trmc = crate::RegValueT<Trmc_SPEC>;

impl Trmc {
    #[doc = "MCDS break out to Trigger Line Routing   BRKOUT"]
    #[inline(always)]
    pub fn brkout(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, trmc::Brkout, Trmc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,trmc::Brkout, Trmc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "MCDS suspend out to Trigger Line Routing   SUSOUT"]
    #[inline(always)]
    pub fn susout(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, trmc::Susout, Trmc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,trmc::Susout, Trmc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to MCDS break in Routing   BRKIN"]
    #[inline(always)]
    pub fn brkin(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, trmc::Brkin, Trmc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0xf,1,0,trmc::Brkin, Trmc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Trmc {
    #[inline(always)]
    fn default() -> Trmc {
        <crate::RegValueT<Trmc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod trmc {
    pub struct Brkout_SPEC;
    pub type Brkout = crate::EnumBitfieldStruct<u8, Brkout_SPEC>;
    impl Brkout {
        #[doc = "Signal is ignored"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Susout_SPEC;
    pub type Susout = crate::EnumBitfieldStruct<u8, Susout_SPEC>;
    impl Susout {
        #[doc = "Signal is ignored"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Brkin_SPEC;
    pub type Brkin = crate::EnumBitfieldStruct<u8, Brkin_SPEC>;
    impl Brkin {
        #[doc = "Signal is always inactive"]
        pub const CONST_00: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trmt_SPEC;
impl crate::sealed::RegSpec for Trmt_SPEC {
    type DataType = u32;
}
#[doc = "TG Routing for MCDS Triggers\n resetvalue={Debug Reset:0x0}"]
pub type Trmt = crate::RegValueT<Trmt_SPEC>;

impl Trmt {
    #[doc = "MCDS trig out 3 to Trigger Line Routing   TG3"]
    #[inline(always)]
    pub fn tg0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, trmt::Tg0, Trmt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,trmt::Tg0, Trmt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "MCDS trig out 3 to Trigger Line Routing   TG3"]
    #[inline(always)]
    pub fn tg1(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, trmt::Tg1, Trmt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,trmt::Tg1, Trmt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "MCDS trig out 3 to Trigger Line Routing   TG3"]
    #[inline(always)]
    pub fn tg2(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, trmt::Tg2, Trmt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,trmt::Tg2, Trmt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "MCDS trig out 3 to Trigger Line Routing   TG3"]
    #[inline(always)]
    pub fn tg3(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, trmt::Tg3, Trmt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,trmt::Tg3, Trmt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Trmt {
    #[inline(always)]
    fn default() -> Trmt {
        <crate::RegValueT<Trmt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod trmt {
    pub struct Tg0_SPEC;
    pub type Tg0 = crate::EnumBitfieldStruct<u8, Tg0_SPEC>;
    impl Tg0 {
        #[doc = "Signal is ignored"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Tg1_SPEC;
    pub type Tg1 = crate::EnumBitfieldStruct<u8, Tg1_SPEC>;
    impl Tg1 {
        #[doc = "Signal is ignored"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Tg2_SPEC;
    pub type Tg2 = crate::EnumBitfieldStruct<u8, Tg2_SPEC>;
    impl Tg2 {
        #[doc = "Signal is ignored"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Tg3_SPEC;
    pub type Tg3 = crate::EnumBitfieldStruct<u8, Tg3_SPEC>;
    impl Tg3 {
        #[doc = "Signal is ignored"]
        pub const CONST_00: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trss_SPEC;
impl crate::sealed::RegSpec for Trss_SPEC {
    type DataType = u32;
}
#[doc = "TG Routing for Special Signals\n resetvalue={Debug Reset:0x0EF0000}"]
pub type Trss = crate::RegValueT<Trss_SPEC>;

impl Trss {
    #[doc = "Trigger Line to Cerberus  Triggered Transfer Routing   TT"]
    #[inline(always)]
    pub fn tt(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, trss::Tt, Trss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,trss::Tt, Trss_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to Fault and Stress Injection Routing   IFS. This bit field is not availavle in TC39x A Step."]
    #[inline(always)]
    pub fn ifs(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, trss::Ifs, Trss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,trss::Ifs, Trss_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to SRC1 Interrupt Routing   SRC1"]
    #[inline(always)]
    pub fn src0(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, trss::Src0, Trss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,trss::Src0, Trss_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Line to SRC1 Interrupt Routing   SRC1"]
    #[inline(always)]
    pub fn src1(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, trss::Src1, Trss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,trss::Src1, Trss_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Trss {
    #[inline(always)]
    fn default() -> Trss {
        <crate::RegValueT<Trss_SPEC> as RegisterValue<_>>::new(15663104)
    }
}
pub mod trss {
    pub struct Tt_SPEC;
    pub type Tt = crate::EnumBitfieldStruct<u8, Tt_SPEC>;
    impl Tt {
        #[doc = "Signal is always inactive"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Ifs_SPEC;
    pub type Ifs = crate::EnumBitfieldStruct<u8, Ifs_SPEC>;
    impl Ifs {
        #[doc = "Signal is always inactive"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Src0_SPEC;
    pub type Src0 = crate::EnumBitfieldStruct<u8, Src0_SPEC>;
    impl Src0 {
        #[doc = "Signal is always inactive"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "E DAPE IOC32E interrupt  Emulation Device only"]
        pub const CONST_1414: Self = Self::new(14);
        #[doc = "F Cerberus  IOClient COM Mode interrupt"]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Src1_SPEC;
    pub type Src1 = crate::EnumBitfieldStruct<u8, Src1_SPEC>;
    impl Src1 {
        #[doc = "Signal is always inactive"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "E DAPE IOC32E interrupt  Emulation Device only"]
        pub const CONST_1414: Self = Self::new(14);
        #[doc = "F Cerberus  IOClient COM Mode interrupt"]
        pub const CONST_1515: Self = Self::new(15);
    }
}

#[doc = "TRTGB"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trtgb(pub(super) *mut u8);
unsafe impl core::marker::Send for Trtgb {}
unsafe impl core::marker::Sync for Trtgb {}
impl Trtgb {
    #[doc = "TG Routing for OTGBi Bits  15 8 \n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn trtgbih(&self) -> crate::common::Reg<trtgb::TrtgBiH_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "TG Routing for OTGBi Bits  7 0 \n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn trtgbil(&self) -> crate::common::Reg<trtgb::TrtgBiL_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod trtgb {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrtgBiH_SPEC;
    impl crate::sealed::RegSpec for TrtgBiH_SPEC {
        type DataType = u32;
    }
    #[doc = "TG Routing for OTGBi Bits  15 8 \n resetvalue={Debug Reset:0x0}"]
    pub type TrtgBiH = crate::RegValueT<TrtgBiH_SPEC>;

    impl TrtgBiH {
        #[doc = "OTGBi Bit 15 to Trigger Line Routing   TG15"]
        #[inline(always)]
        pub fn tg8(
            self,
        ) -> crate::common::RegisterField<0, 0xf, 1, 0, trtgbih::Tg8, TrtgBiH_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0xf,
                1,
                0,
                trtgbih::Tg8,
                TrtgBiH_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "OTGBi Bit 15 to Trigger Line Routing   TG15"]
        #[inline(always)]
        pub fn tg9(
            self,
        ) -> crate::common::RegisterField<4, 0xf, 1, 0, trtgbih::Tg9, TrtgBiH_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                4,
                0xf,
                1,
                0,
                trtgbih::Tg9,
                TrtgBiH_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "OTGBi Bit 15 to Trigger Line Routing   TG15"]
        #[inline(always)]
        pub fn tg10(
            self,
        ) -> crate::common::RegisterField<
            8,
            0xf,
            1,
            0,
            trtgbih::Tg10,
            TrtgBiH_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0xf,
                1,
                0,
                trtgbih::Tg10,
                TrtgBiH_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "OTGBi Bit 15 to Trigger Line Routing   TG15"]
        #[inline(always)]
        pub fn tg11(
            self,
        ) -> crate::common::RegisterField<
            12,
            0xf,
            1,
            0,
            trtgbih::Tg11,
            TrtgBiH_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                12,
                0xf,
                1,
                0,
                trtgbih::Tg11,
                TrtgBiH_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "OTGBi Bit 15 to Trigger Line Routing   TG15"]
        #[inline(always)]
        pub fn tg12(
            self,
        ) -> crate::common::RegisterField<
            16,
            0xf,
            1,
            0,
            trtgbih::Tg12,
            TrtgBiH_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0xf,
                1,
                0,
                trtgbih::Tg12,
                TrtgBiH_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "OTGBi Bit 15 to Trigger Line Routing   TG15"]
        #[inline(always)]
        pub fn tg13(
            self,
        ) -> crate::common::RegisterField<
            20,
            0xf,
            1,
            0,
            trtgbih::Tg13,
            TrtgBiH_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0xf,
                1,
                0,
                trtgbih::Tg13,
                TrtgBiH_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "OTGBi Bit 15 to Trigger Line Routing   TG15"]
        #[inline(always)]
        pub fn tg14(
            self,
        ) -> crate::common::RegisterField<
            24,
            0xf,
            1,
            0,
            trtgbih::Tg14,
            TrtgBiH_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0xf,
                1,
                0,
                trtgbih::Tg14,
                TrtgBiH_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "OTGBi Bit 15 to Trigger Line Routing   TG15"]
        #[inline(always)]
        pub fn tg15(
            self,
        ) -> crate::common::RegisterField<
            28,
            0xf,
            1,
            0,
            trtgbih::Tg15,
            TrtgBiH_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                28,
                0xf,
                1,
                0,
                trtgbih::Tg15,
                TrtgBiH_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for TrtgBiH {
        #[inline(always)]
        fn default() -> TrtgBiH {
            <crate::RegValueT<TrtgBiH_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod trtgbih {
        pub struct Tg8_SPEC;
        pub type Tg8 = crate::EnumBitfieldStruct<u8, Tg8_SPEC>;
        impl Tg8 {
            #[doc = "Signal is ignored"]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Tg9_SPEC;
        pub type Tg9 = crate::EnumBitfieldStruct<u8, Tg9_SPEC>;
        impl Tg9 {
            #[doc = "Signal is ignored"]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Tg10_SPEC;
        pub type Tg10 = crate::EnumBitfieldStruct<u8, Tg10_SPEC>;
        impl Tg10 {
            #[doc = "Signal is ignored"]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Tg11_SPEC;
        pub type Tg11 = crate::EnumBitfieldStruct<u8, Tg11_SPEC>;
        impl Tg11 {
            #[doc = "Signal is ignored"]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Tg12_SPEC;
        pub type Tg12 = crate::EnumBitfieldStruct<u8, Tg12_SPEC>;
        impl Tg12 {
            #[doc = "Signal is ignored"]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Tg13_SPEC;
        pub type Tg13 = crate::EnumBitfieldStruct<u8, Tg13_SPEC>;
        impl Tg13 {
            #[doc = "Signal is ignored"]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Tg14_SPEC;
        pub type Tg14 = crate::EnumBitfieldStruct<u8, Tg14_SPEC>;
        impl Tg14 {
            #[doc = "Signal is ignored"]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Tg15_SPEC;
        pub type Tg15 = crate::EnumBitfieldStruct<u8, Tg15_SPEC>;
        impl Tg15 {
            #[doc = "Signal is ignored"]
            pub const CONST_00: Self = Self::new(0);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TrtgBiL_SPEC;
    impl crate::sealed::RegSpec for TrtgBiL_SPEC {
        type DataType = u32;
    }
    #[doc = "TG Routing for OTGBi Bits  7 0 \n resetvalue={Debug Reset:0x0}"]
    pub type TrtgBiL = crate::RegValueT<TrtgBiL_SPEC>;

    impl TrtgBiL {
        #[doc = "OTGBi Bit 7 to Trigger Line Routing   TG7"]
        #[inline(always)]
        pub fn tg0(
            self,
        ) -> crate::common::RegisterField<0, 0xf, 1, 0, trtgbil::Tg0, TrtgBiL_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0xf,
                1,
                0,
                trtgbil::Tg0,
                TrtgBiL_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "OTGBi Bit 7 to Trigger Line Routing   TG7"]
        #[inline(always)]
        pub fn tg1(
            self,
        ) -> crate::common::RegisterField<4, 0xf, 1, 0, trtgbil::Tg1, TrtgBiL_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                4,
                0xf,
                1,
                0,
                trtgbil::Tg1,
                TrtgBiL_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "OTGBi Bit 7 to Trigger Line Routing   TG7"]
        #[inline(always)]
        pub fn tg2(
            self,
        ) -> crate::common::RegisterField<8, 0xf, 1, 0, trtgbil::Tg2, TrtgBiL_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                8,
                0xf,
                1,
                0,
                trtgbil::Tg2,
                TrtgBiL_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "OTGBi Bit 7 to Trigger Line Routing   TG7"]
        #[inline(always)]
        pub fn tg3(
            self,
        ) -> crate::common::RegisterField<
            12,
            0xf,
            1,
            0,
            trtgbil::Tg3,
            TrtgBiL_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                12,
                0xf,
                1,
                0,
                trtgbil::Tg3,
                TrtgBiL_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "OTGBi Bit 7 to Trigger Line Routing   TG7"]
        #[inline(always)]
        pub fn tg4(
            self,
        ) -> crate::common::RegisterField<
            16,
            0xf,
            1,
            0,
            trtgbil::Tg4,
            TrtgBiL_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0xf,
                1,
                0,
                trtgbil::Tg4,
                TrtgBiL_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "OTGBi Bit 7 to Trigger Line Routing   TG7"]
        #[inline(always)]
        pub fn tg5(
            self,
        ) -> crate::common::RegisterField<
            20,
            0xf,
            1,
            0,
            trtgbil::Tg5,
            TrtgBiL_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0xf,
                1,
                0,
                trtgbil::Tg5,
                TrtgBiL_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "OTGBi Bit 7 to Trigger Line Routing   TG7"]
        #[inline(always)]
        pub fn tg6(
            self,
        ) -> crate::common::RegisterField<
            24,
            0xf,
            1,
            0,
            trtgbil::Tg6,
            TrtgBiL_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0xf,
                1,
                0,
                trtgbil::Tg6,
                TrtgBiL_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "OTGBi Bit 7 to Trigger Line Routing   TG7"]
        #[inline(always)]
        pub fn tg7(
            self,
        ) -> crate::common::RegisterField<
            28,
            0xf,
            1,
            0,
            trtgbil::Tg7,
            TrtgBiL_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                28,
                0xf,
                1,
                0,
                trtgbil::Tg7,
                TrtgBiL_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for TrtgBiL {
        #[inline(always)]
        fn default() -> TrtgBiL {
            <crate::RegValueT<TrtgBiL_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod trtgbil {
        pub struct Tg0_SPEC;
        pub type Tg0 = crate::EnumBitfieldStruct<u8, Tg0_SPEC>;
        impl Tg0 {
            #[doc = "Signal is ignored"]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Tg1_SPEC;
        pub type Tg1 = crate::EnumBitfieldStruct<u8, Tg1_SPEC>;
        impl Tg1 {
            #[doc = "Signal is ignored"]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Tg2_SPEC;
        pub type Tg2 = crate::EnumBitfieldStruct<u8, Tg2_SPEC>;
        impl Tg2 {
            #[doc = "Signal is ignored"]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Tg3_SPEC;
        pub type Tg3 = crate::EnumBitfieldStruct<u8, Tg3_SPEC>;
        impl Tg3 {
            #[doc = "Signal is ignored"]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Tg4_SPEC;
        pub type Tg4 = crate::EnumBitfieldStruct<u8, Tg4_SPEC>;
        impl Tg4 {
            #[doc = "Signal is ignored"]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Tg5_SPEC;
        pub type Tg5 = crate::EnumBitfieldStruct<u8, Tg5_SPEC>;
        impl Tg5 {
            #[doc = "Signal is ignored"]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Tg6_SPEC;
        pub type Tg6 = crate::EnumBitfieldStruct<u8, Tg6_SPEC>;
        impl Tg6 {
            #[doc = "Signal is ignored"]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Tg7_SPEC;
        pub type Tg7 = crate::EnumBitfieldStruct<u8, Tg7_SPEC>;
        impl Tg7 {
            #[doc = "Signal is ignored"]
            pub const CONST_00: Self = Self::new(0);
        }
    }
}
