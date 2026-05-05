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
#[doc = r"INT"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int(pub(super) *mut u8);
unsafe impl core::marker::Send for Int {}
unsafe impl core::marker::Sync for Int {}
impl Int {
    #[doc = "Access Enable covering all INT ECRx and all SRCy 15 0   Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen_config0(
        &self,
    ) -> crate::common::Reg<self::AccenConfig0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(240usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0B9C000}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "OTGM IRQ Trace\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn oit(&self) -> crate::common::Reg<self::Oit_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }

    #[doc = "OTGM IRQ MUX Missed IRQ Select\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn oixms(&self) -> crate::common::Reg<self::Oixms_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(140usize)) }
    }

    #[doc = "OTGM IRQ MUX Select 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn oixs0(&self) -> crate::common::Reg<self::Oixs0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }

    #[doc = "OTGM IRQ MUX Select 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn oixs1(&self) -> crate::common::Reg<self::Oixs1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(148usize)) }
    }

    #[doc = "OTGM IRQ MUX Trigger Set Select\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn oixts(&self) -> crate::common::Reg<self::Oixts_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }

    #[doc = "OTGM MCDS I F Sensitivity Negedge\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn omisn(&self) -> crate::common::Reg<self::Omisn_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(168usize)) }
    }

    #[doc = "OTGM MCDS I F Sensitivity Posedge\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn omisp(&self) -> crate::common::Reg<self::Omisp_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(164usize)) }
    }

    #[doc = "OTGM OTGB0 1 Status\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn oobs(&self) -> crate::common::Reg<self::Oobs_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "OTGM SSI Control\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ossic(&self) -> crate::common::Reg<self::Ossic_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }

    #[doc = "Service Request Broadcast Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn srb(&self) -> [crate::common::Reg<self::Srb_SPEC, crate::common::W>; 3] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x10usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x10usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x10usize + 0x8usize)),
            ]
        }
    }
    #[doc = "ACCEN"]
    #[inline(always)]
    pub fn accen(self) -> [self::Accen; 3] {
        unsafe {
            [
                self::Accen(self.0.add(0x100usize + 0x0usize)),
                self::Accen(self.0.add(0x100usize + 0x8usize)),
                self::Accen(self.0.add(0x100usize + 0x10usize)),
            ]
        }
    }
    #[doc = "ACCEN SRC"]
    #[inline(always)]
    pub fn accen_src(self) -> [self::AccenSrc; 4] {
        unsafe {
            [
                self::AccenSrc(self.0.add(0x180usize + 0x0usize)),
                self::AccenSrc(self.0.add(0x180usize + 0x8usize)),
                self::AccenSrc(self.0.add(0x180usize + 0x10usize)),
                self::AccenSrc(self.0.add(0x180usize + 0x18usize)),
            ]
        }
    }
    #[doc = "CH"]
    #[inline(always)]
    pub fn ch(self) -> [self::Ch; 4] {
        unsafe {
            [
                self::Ch(self.0.add(0x200usize + 0x0usize)),
                self::Ch(self.0.add(0x200usize + 0x10usize)),
                self::Ch(self.0.add(0x200usize + 0x20usize)),
                self::Ch(self.0.add(0x200usize + 0x30usize)),
            ]
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AccenConfig0_SPEC;
impl crate::sealed::RegSpec for AccenConfig0_SPEC {
    type DataType = u32;
}
#[doc = "Access Enable covering all INT ECRx and all SRCy 15 0   Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type AccenConfig0 = crate::RegValueT<AccenConfig0_SPEC>;

impl AccenConfig0 {
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        accen_config0::En0,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            accen_config0::En0,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        accen_config0::En1,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            accen_config0::En1,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        accen_config0::En2,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            accen_config0::En2,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        accen_config0::En3,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            accen_config0::En3,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        accen_config0::En4,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            accen_config0::En4,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        accen_config0::En5,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            accen_config0::En5,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        accen_config0::En6,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            accen_config0::En6,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        accen_config0::En7,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            accen_config0::En7,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        accen_config0::En8,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            accen_config0::En8,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        accen_config0::En9,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            accen_config0::En9,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        accen_config0::En10,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            accen_config0::En10,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        accen_config0::En11,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            accen_config0::En11,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        accen_config0::En12,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            accen_config0::En12,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        accen_config0::En13,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            accen_config0::En13,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        accen_config0::En14,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            accen_config0::En14,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        accen_config0::En15,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            accen_config0::En15,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        accen_config0::En16,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            accen_config0::En16,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        accen_config0::En17,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            accen_config0::En17,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        accen_config0::En18,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            accen_config0::En18,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        accen_config0::En19,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            accen_config0::En19,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        accen_config0::En20,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            accen_config0::En20,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        accen_config0::En21,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            accen_config0::En21,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        accen_config0::En22,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            accen_config0::En22,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        accen_config0::En23,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            accen_config0::En23,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        accen_config0::En24,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            accen_config0::En24,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        accen_config0::En25,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            accen_config0::En25,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        accen_config0::En26,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            accen_config0::En26,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        accen_config0::En27,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            accen_config0::En27,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        accen_config0::En28,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            accen_config0::En28,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        accen_config0::En29,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            accen_config0::En29,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        accen_config0::En30,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            accen_config0::En30,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        accen_config0::En31,
        AccenConfig0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            accen_config0::En31,
            AccenConfig0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for AccenConfig0 {
    #[inline(always)]
    fn default() -> AccenConfig0 {
        <crate::RegValueT<AccenConfig0_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod accen_config0 {
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
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0B9C000}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number. This bit field defines the module revision number. The value of a module        revision starts with 01  first revision ."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type. The bit field is set to C0 which defines        the module as a 32 bit module."]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value. This bit field defines a module identification number. The value for the        Interrupt Router module is 009Bh."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(12173312)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oit_SPEC;
impl crate::sealed::RegSpec for Oit_SPEC {
    type DataType = u32;
}
#[doc = "OTGM IRQ Trace\n resetvalue={Application Reset:0x0}"]
pub type Oit = crate::RegValueT<Oit_SPEC>;

impl Oit {
    #[doc = "Output Enable for OTGB0"]
    #[inline(always)]
    pub fn oe0(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, oit::Oe0, Oit_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,oit::Oe0, Oit_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Output Enable for OTGB1"]
    #[inline(always)]
    pub fn oe1(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, oit::Oe1, Oit_SPEC, crate::common::RW> {
        crate::common::RegisterField::<15,0x1,1,0,oit::Oe1, Oit_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Oit {
    #[inline(always)]
    fn default() -> Oit {
        <crate::RegValueT<Oit_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod oit {
    pub struct Oe0_SPEC;
    pub type Oe0 = crate::EnumBitfieldStruct<u8, Oe0_SPEC>;
    impl Oe0 {
        #[doc = "Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oe1_SPEC;
    pub type Oe1 = crate::EnumBitfieldStruct<u8, Oe1_SPEC>;
    impl Oe1 {
        #[doc = "Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oixms_SPEC;
impl crate::sealed::RegSpec for Oixms_SPEC {
    type DataType = u32;
}
#[doc = "OTGM IRQ MUX Missed IRQ Select\n resetvalue={Application Reset:0x0}"]
pub type Oixms = crate::RegValueT<Oixms_SPEC>;

impl Oixms {
    #[doc = "SRN Index for Missed Interrupt Trigger"]
    #[inline(always)]
    pub fn mirq(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Oixms_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Oixms_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Oixms {
    #[inline(always)]
    fn default() -> Oixms {
        <crate::RegValueT<Oixms_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oixs0_SPEC;
impl crate::sealed::RegSpec for Oixs0_SPEC {
    type DataType = u32;
}
#[doc = "OTGM IRQ MUX Select 0\n resetvalue={Application Reset:0x0}"]
pub type Oixs0 = crate::RegValueT<Oixs0_SPEC>;

impl Oixs0 {
    #[doc = "SRN Index for Interrupt Trigger 0"]
    #[inline(always)]
    pub fn irq0(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Oixs0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Oixs0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SRN Index for Interrupt Trigger 1"]
    #[inline(always)]
    pub fn irq1(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Oixs0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Oixs0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Oixs0 {
    #[inline(always)]
    fn default() -> Oixs0 {
        <crate::RegValueT<Oixs0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oixs1_SPEC;
impl crate::sealed::RegSpec for Oixs1_SPEC {
    type DataType = u32;
}
#[doc = "OTGM IRQ MUX Select 1\n resetvalue={Application Reset:0x0}"]
pub type Oixs1 = crate::RegValueT<Oixs1_SPEC>;

impl Oixs1 {
    #[doc = "SRN Index for Interrupt Trigger 2"]
    #[inline(always)]
    pub fn irq2(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Oixs1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Oixs1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SRN Index for Interrupt Trigger 3"]
    #[inline(always)]
    pub fn irq3(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Oixs1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Oixs1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Oixs1 {
    #[inline(always)]
    fn default() -> Oixs1 {
        <crate::RegValueT<Oixs1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oixts_SPEC;
impl crate::sealed::RegSpec for Oixts_SPEC {
    type DataType = u32;
}
#[doc = "OTGM IRQ MUX Trigger Set Select\n resetvalue={Application Reset:0x0}"]
pub type Oixts = crate::RegValueT<Oixts_SPEC>;

impl Oixts {
    #[doc = "Trigger Set Select for OTGB0 1 Overlay"]
    #[inline(always)]
    pub fn tgs(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, oixts::Tgs, Oixts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,oixts::Tgs, Oixts_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Overlay Byte Select"]
    #[inline(always)]
    pub fn obs(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, oixts::Obs, Oixts_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,oixts::Obs, Oixts_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Oixts {
    #[inline(always)]
    fn default() -> Oixts {
        <crate::RegValueT<Oixts_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod oixts {
    pub struct Tgs_SPEC;
    pub type Tgs = crate::EnumBitfieldStruct<u8, Tgs_SPEC>;
    impl Tgs {
        #[doc = "No overlay"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Trigger Set TS8 IS"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "Trigger Set TS8 SPA"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Obs_SPEC;
    pub type Obs = crate::EnumBitfieldStruct<u8, Obs_SPEC>;
    impl Obs {
        #[doc = "OTGB0  7 0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "OTGB0  15 8"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "OTGB1  7 0"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "OTGB1  15 8"]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Omisn_SPEC;
impl crate::sealed::RegSpec for Omisn_SPEC {
    type DataType = u32;
}
#[doc = "OTGM MCDS I F Sensitivity Negedge\n resetvalue={Application Reset:0x0}"]
pub type Omisn = crate::RegValueT<Omisn_SPEC>;

impl Omisn {
    #[doc = "Bitwise Negedge Sensitivity for OTGB0. If a bit is set an OTGB value will be written to MCDS on a falling edge        of the associated OTGB0 bit."]
    #[inline(always)]
    pub fn otgb0(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Omisn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Omisn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bitwise Negedge Sensitivity for OTGB1. If a bit is set an OTGB value will be written to MCDS on a falling edge        of the associated OTGB1 bit."]
    #[inline(always)]
    pub fn otgb1(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Omisn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Omisn_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Omisn {
    #[inline(always)]
    fn default() -> Omisn {
        <crate::RegValueT<Omisn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Omisp_SPEC;
impl crate::sealed::RegSpec for Omisp_SPEC {
    type DataType = u32;
}
#[doc = "OTGM MCDS I F Sensitivity Posedge\n resetvalue={Application Reset:0x0}"]
pub type Omisp = crate::RegValueT<Omisp_SPEC>;

impl Omisp {
    #[doc = "Bitwise Posedge Sensitivity for OTGB0. If a bit is set an OTGB value will be written to MCDS on a rising edge        of the associated OTGB0 bit."]
    #[inline(always)]
    pub fn otgb0(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Omisp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Omisp_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bitwise Posedge Sensitivity for OTGB1. If a bit is set an OTGB value will be written to MCDS on a rising edge        of the associated OTGB1 bit."]
    #[inline(always)]
    pub fn otgb1(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Omisp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Omisp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Omisp {
    #[inline(always)]
    fn default() -> Omisp {
        <crate::RegValueT<Omisp_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oobs_SPEC;
impl crate::sealed::RegSpec for Oobs_SPEC {
    type DataType = u32;
}
#[doc = "OTGM OTGB0 1 Status\n resetvalue={Application Reset:0x0}"]
pub type Oobs = crate::RegValueT<Oobs_SPEC>;

impl Oobs {
    #[doc = "Status of OTGB0"]
    #[inline(always)]
    pub fn otgb0(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Oobs_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Oobs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Status of OTGB1"]
    #[inline(always)]
    pub fn otgb1(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Oobs_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Oobs_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Oobs {
    #[inline(always)]
    fn default() -> Oobs {
        <crate::RegValueT<Oobs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ossic_SPEC;
impl crate::sealed::RegSpec for Ossic_SPEC {
    type DataType = u32;
}
#[doc = "OTGM SSI Control\n resetvalue={Application Reset:0x0}"]
pub type Ossic = crate::RegValueT<Ossic_SPEC>;

impl Ossic {
    #[doc = "Trigger Set for OTGB0 1"]
    #[inline(always)]
    pub fn tgs(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, ossic::Tgs, Ossic_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,ossic::Tgs, Ossic_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OTGB0 1 Bus Select"]
    #[inline(always)]
    pub fn tgb(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ossic::Tgb, Ossic_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,ossic::Tgb, Ossic_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ossic {
    #[inline(always)]
    fn default() -> Ossic {
        <crate::RegValueT<Ossic_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ossic {
    pub struct Tgs_SPEC;
    pub type Tgs = crate::EnumBitfieldStruct<u8, Tgs_SPEC>;
    impl Tgs {
        #[doc = "No Trigger Set output"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Trigger Set TS16 SSI"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tgb_SPEC;
    pub type Tgb = crate::EnumBitfieldStruct<u8, Tgb_SPEC>;
    impl Tgb {
        #[doc = "Trigger Set is output on OTGB0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Trigger Set is output on OTGB1"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srb_SPEC;
impl crate::sealed::RegSpec for Srb_SPEC {
    type DataType = u32;
}
#[doc = "Service Request Broadcast Register 0\n resetvalue={Application Reset:0x0}"]
pub type Srb = crate::RegValueT<Srb_SPEC>;

impl Srb {
    #[doc = "General Purpose Service Request Trigger 7. This bit is always read as 0."]
    #[inline(always)]
    pub fn trig0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, srb::Trig0, Srb_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0x1,1,0,srb::Trig0, Srb_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "General Purpose Service Request Trigger 7. This bit is always read as 0."]
    #[inline(always)]
    pub fn trig1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, srb::Trig1, Srb_SPEC, crate::common::W> {
        crate::common::RegisterField::<1,0x1,1,0,srb::Trig1, Srb_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "General Purpose Service Request Trigger 7. This bit is always read as 0."]
    #[inline(always)]
    pub fn trig2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, srb::Trig2, Srb_SPEC, crate::common::W> {
        crate::common::RegisterField::<2,0x1,1,0,srb::Trig2, Srb_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "General Purpose Service Request Trigger 7. This bit is always read as 0."]
    #[inline(always)]
    pub fn trig3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, srb::Trig3, Srb_SPEC, crate::common::W> {
        crate::common::RegisterField::<3,0x1,1,0,srb::Trig3, Srb_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "General Purpose Service Request Trigger 7. This bit is always read as 0."]
    #[inline(always)]
    pub fn trig4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, srb::Trig4, Srb_SPEC, crate::common::W> {
        crate::common::RegisterField::<4,0x1,1,0,srb::Trig4, Srb_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "General Purpose Service Request Trigger 7. This bit is always read as 0."]
    #[inline(always)]
    pub fn trig5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, srb::Trig5, Srb_SPEC, crate::common::W> {
        crate::common::RegisterField::<5,0x1,1,0,srb::Trig5, Srb_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "General Purpose Service Request Trigger 7. This bit is always read as 0."]
    #[inline(always)]
    pub fn trig6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, srb::Trig6, Srb_SPEC, crate::common::W> {
        crate::common::RegisterField::<6,0x1,1,0,srb::Trig6, Srb_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "General Purpose Service Request Trigger 7. This bit is always read as 0."]
    #[inline(always)]
    pub fn trig7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, srb::Trig7, Srb_SPEC, crate::common::W> {
        crate::common::RegisterField::<7,0x1,1,0,srb::Trig7, Srb_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Srb {
    #[inline(always)]
    fn default() -> Srb {
        <crate::RegValueT<Srb_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod srb {
    pub struct Trig0_SPEC;
    pub type Trig0 = crate::EnumBitfieldStruct<u8, Trig0_SPEC>;
    impl Trig0 {
        #[doc = "No effect"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Trigger the General Purpose Service Request 7 in group x  GPSRx7"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trig1_SPEC;
    pub type Trig1 = crate::EnumBitfieldStruct<u8, Trig1_SPEC>;
    impl Trig1 {
        #[doc = "No effect"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Trigger the General Purpose Service Request 7 in group x  GPSRx7"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trig2_SPEC;
    pub type Trig2 = crate::EnumBitfieldStruct<u8, Trig2_SPEC>;
    impl Trig2 {
        #[doc = "No effect"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Trigger the General Purpose Service Request 7 in group x  GPSRx7"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trig3_SPEC;
    pub type Trig3 = crate::EnumBitfieldStruct<u8, Trig3_SPEC>;
    impl Trig3 {
        #[doc = "No effect"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Trigger the General Purpose Service Request 7 in group x  GPSRx7"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trig4_SPEC;
    pub type Trig4 = crate::EnumBitfieldStruct<u8, Trig4_SPEC>;
    impl Trig4 {
        #[doc = "No effect"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Trigger the General Purpose Service Request 7 in group x  GPSRx7"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trig5_SPEC;
    pub type Trig5 = crate::EnumBitfieldStruct<u8, Trig5_SPEC>;
    impl Trig5 {
        #[doc = "No effect"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Trigger the General Purpose Service Request 7 in group x  GPSRx7"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trig6_SPEC;
    pub type Trig6 = crate::EnumBitfieldStruct<u8, Trig6_SPEC>;
    impl Trig6 {
        #[doc = "No effect"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Trigger the General Purpose Service Request 7 in group x  GPSRx7"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trig7_SPEC;
    pub type Trig7 = crate::EnumBitfieldStruct<u8, Trig7_SPEC>;
    impl Trig7 {
        #[doc = "No effect"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Trigger the General Purpose Service Request 7 in group x  GPSRx7"]
        pub const CONST_11: Self = Self::new(1);
    }
}

#[doc = "ACCEN"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accen(pub(super) *mut u8);
unsafe impl core::marker::Send for Accen {}
unsafe impl core::marker::Sync for Accen {}
impl Accen {
    #[doc = "Access Enable covering SRB0  Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen_srbx0(
        &self,
    ) -> crate::common::Reg<accen::AccenSrBx0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod accen {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AccenSrBx0_SPEC;
    impl crate::sealed::RegSpec for AccenSrBx0_SPEC {
        type DataType = u32;
    }
    #[doc = "Access Enable covering SRB0  Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type AccenSrBx0 = crate::RegValueT<AccenSrBx0_SPEC>;

    impl AccenSrBx0 {
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en0(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            accen_srbx0::En0,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                accen_srbx0::En0,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en1(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            accen_srbx0::En1,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                accen_srbx0::En1,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en2(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            accen_srbx0::En2,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                accen_srbx0::En2,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en3(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            accen_srbx0::En3,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                accen_srbx0::En3,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en4(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            accen_srbx0::En4,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                accen_srbx0::En4,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en5(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            accen_srbx0::En5,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                accen_srbx0::En5,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en6(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x1,
            1,
            0,
            accen_srbx0::En6,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                6,
                0x1,
                1,
                0,
                accen_srbx0::En6,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en7(
            self,
        ) -> crate::common::RegisterField<
            7,
            0x1,
            1,
            0,
            accen_srbx0::En7,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                accen_srbx0::En7,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en8(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1,
            1,
            0,
            accen_srbx0::En8,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                accen_srbx0::En8,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en9(
            self,
        ) -> crate::common::RegisterField<
            9,
            0x1,
            1,
            0,
            accen_srbx0::En9,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                9,
                0x1,
                1,
                0,
                accen_srbx0::En9,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en10(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            accen_srbx0::En10,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                accen_srbx0::En10,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en11(
            self,
        ) -> crate::common::RegisterField<
            11,
            0x1,
            1,
            0,
            accen_srbx0::En11,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                accen_srbx0::En11,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en12(
            self,
        ) -> crate::common::RegisterField<
            12,
            0x1,
            1,
            0,
            accen_srbx0::En12,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                12,
                0x1,
                1,
                0,
                accen_srbx0::En12,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en13(
            self,
        ) -> crate::common::RegisterField<
            13,
            0x1,
            1,
            0,
            accen_srbx0::En13,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                13,
                0x1,
                1,
                0,
                accen_srbx0::En13,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en14(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x1,
            1,
            0,
            accen_srbx0::En14,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                accen_srbx0::En14,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en15(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            accen_srbx0::En15,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                accen_srbx0::En15,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en16(
            self,
        ) -> crate::common::RegisterField<
            16,
            0x1,
            1,
            0,
            accen_srbx0::En16,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0x1,
                1,
                0,
                accen_srbx0::En16,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en17(
            self,
        ) -> crate::common::RegisterField<
            17,
            0x1,
            1,
            0,
            accen_srbx0::En17,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                17,
                0x1,
                1,
                0,
                accen_srbx0::En17,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en18(
            self,
        ) -> crate::common::RegisterField<
            18,
            0x1,
            1,
            0,
            accen_srbx0::En18,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                18,
                0x1,
                1,
                0,
                accen_srbx0::En18,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en19(
            self,
        ) -> crate::common::RegisterField<
            19,
            0x1,
            1,
            0,
            accen_srbx0::En19,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                19,
                0x1,
                1,
                0,
                accen_srbx0::En19,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en20(
            self,
        ) -> crate::common::RegisterField<
            20,
            0x1,
            1,
            0,
            accen_srbx0::En20,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0x1,
                1,
                0,
                accen_srbx0::En20,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en21(
            self,
        ) -> crate::common::RegisterField<
            21,
            0x1,
            1,
            0,
            accen_srbx0::En21,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                21,
                0x1,
                1,
                0,
                accen_srbx0::En21,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en22(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x1,
            1,
            0,
            accen_srbx0::En22,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                22,
                0x1,
                1,
                0,
                accen_srbx0::En22,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en23(
            self,
        ) -> crate::common::RegisterField<
            23,
            0x1,
            1,
            0,
            accen_srbx0::En23,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                23,
                0x1,
                1,
                0,
                accen_srbx0::En23,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en24(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x1,
            1,
            0,
            accen_srbx0::En24,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0x1,
                1,
                0,
                accen_srbx0::En24,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en25(
            self,
        ) -> crate::common::RegisterField<
            25,
            0x1,
            1,
            0,
            accen_srbx0::En25,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                25,
                0x1,
                1,
                0,
                accen_srbx0::En25,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en26(
            self,
        ) -> crate::common::RegisterField<
            26,
            0x1,
            1,
            0,
            accen_srbx0::En26,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                26,
                0x1,
                1,
                0,
                accen_srbx0::En26,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en27(
            self,
        ) -> crate::common::RegisterField<
            27,
            0x1,
            1,
            0,
            accen_srbx0::En27,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                27,
                0x1,
                1,
                0,
                accen_srbx0::En27,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en28(
            self,
        ) -> crate::common::RegisterField<
            28,
            0x1,
            1,
            0,
            accen_srbx0::En28,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                28,
                0x1,
                1,
                0,
                accen_srbx0::En28,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en29(
            self,
        ) -> crate::common::RegisterField<
            29,
            0x1,
            1,
            0,
            accen_srbx0::En29,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                29,
                0x1,
                1,
                0,
                accen_srbx0::En29,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en30(
            self,
        ) -> crate::common::RegisterField<
            30,
            0x1,
            1,
            0,
            accen_srbx0::En30,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                30,
                0x1,
                1,
                0,
                accen_srbx0::En30,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en31(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            accen_srbx0::En31,
            AccenSrBx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                accen_srbx0::En31,
                AccenSrBx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for AccenSrBx0 {
        #[inline(always)]
        fn default() -> AccenSrBx0 {
            <crate::RegValueT<AccenSrBx0_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
    pub mod accen_srbx0 {
        pub struct En0_SPEC;
        pub type En0 = crate::EnumBitfieldStruct<u8, En0_SPEC>;
        impl En0 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En1_SPEC;
        pub type En1 = crate::EnumBitfieldStruct<u8, En1_SPEC>;
        impl En1 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En2_SPEC;
        pub type En2 = crate::EnumBitfieldStruct<u8, En2_SPEC>;
        impl En2 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En3_SPEC;
        pub type En3 = crate::EnumBitfieldStruct<u8, En3_SPEC>;
        impl En3 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En4_SPEC;
        pub type En4 = crate::EnumBitfieldStruct<u8, En4_SPEC>;
        impl En4 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En5_SPEC;
        pub type En5 = crate::EnumBitfieldStruct<u8, En5_SPEC>;
        impl En5 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En6_SPEC;
        pub type En6 = crate::EnumBitfieldStruct<u8, En6_SPEC>;
        impl En6 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En7_SPEC;
        pub type En7 = crate::EnumBitfieldStruct<u8, En7_SPEC>;
        impl En7 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En8_SPEC;
        pub type En8 = crate::EnumBitfieldStruct<u8, En8_SPEC>;
        impl En8 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En9_SPEC;
        pub type En9 = crate::EnumBitfieldStruct<u8, En9_SPEC>;
        impl En9 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En10_SPEC;
        pub type En10 = crate::EnumBitfieldStruct<u8, En10_SPEC>;
        impl En10 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En11_SPEC;
        pub type En11 = crate::EnumBitfieldStruct<u8, En11_SPEC>;
        impl En11 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En12_SPEC;
        pub type En12 = crate::EnumBitfieldStruct<u8, En12_SPEC>;
        impl En12 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En13_SPEC;
        pub type En13 = crate::EnumBitfieldStruct<u8, En13_SPEC>;
        impl En13 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En14_SPEC;
        pub type En14 = crate::EnumBitfieldStruct<u8, En14_SPEC>;
        impl En14 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En15_SPEC;
        pub type En15 = crate::EnumBitfieldStruct<u8, En15_SPEC>;
        impl En15 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En16_SPEC;
        pub type En16 = crate::EnumBitfieldStruct<u8, En16_SPEC>;
        impl En16 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En17_SPEC;
        pub type En17 = crate::EnumBitfieldStruct<u8, En17_SPEC>;
        impl En17 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En18_SPEC;
        pub type En18 = crate::EnumBitfieldStruct<u8, En18_SPEC>;
        impl En18 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En19_SPEC;
        pub type En19 = crate::EnumBitfieldStruct<u8, En19_SPEC>;
        impl En19 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En20_SPEC;
        pub type En20 = crate::EnumBitfieldStruct<u8, En20_SPEC>;
        impl En20 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En21_SPEC;
        pub type En21 = crate::EnumBitfieldStruct<u8, En21_SPEC>;
        impl En21 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En22_SPEC;
        pub type En22 = crate::EnumBitfieldStruct<u8, En22_SPEC>;
        impl En22 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En23_SPEC;
        pub type En23 = crate::EnumBitfieldStruct<u8, En23_SPEC>;
        impl En23 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En24_SPEC;
        pub type En24 = crate::EnumBitfieldStruct<u8, En24_SPEC>;
        impl En24 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En25_SPEC;
        pub type En25 = crate::EnumBitfieldStruct<u8, En25_SPEC>;
        impl En25 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En26_SPEC;
        pub type En26 = crate::EnumBitfieldStruct<u8, En26_SPEC>;
        impl En26 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En27_SPEC;
        pub type En27 = crate::EnumBitfieldStruct<u8, En27_SPEC>;
        impl En27 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En28_SPEC;
        pub type En28 = crate::EnumBitfieldStruct<u8, En28_SPEC>;
        impl En28 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En29_SPEC;
        pub type En29 = crate::EnumBitfieldStruct<u8, En29_SPEC>;
        impl En29 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En30_SPEC;
        pub type En30 = crate::EnumBitfieldStruct<u8, En30_SPEC>;
        impl En30 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En31_SPEC;
        pub type En31 = crate::EnumBitfieldStruct<u8, En31_SPEC>;
        impl En31 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
}
#[doc = "ACCEN SRC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AccenSrc(pub(super) *mut u8);
unsafe impl core::marker::Send for AccenSrc {}
unsafe impl core::marker::Sync for AccenSrc {}
impl AccenSrc {
    #[doc = "Access Enable covering all SRCx 31 16  mapped to ICU0  Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen_src_tosx0(
        &self,
    ) -> crate::common::Reg<accen_src::AccenSrcToSx0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod accen_src {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AccenSrcToSx0_SPEC;
    impl crate::sealed::RegSpec for AccenSrcToSx0_SPEC {
        type DataType = u32;
    }
    #[doc = "Access Enable covering all SRCx 31 16  mapped to ICU0  Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type AccenSrcToSx0 = crate::RegValueT<AccenSrcToSx0_SPEC>;

    impl AccenSrcToSx0 {
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en0(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            accen_src_tosx0::En0,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                accen_src_tosx0::En0,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en1(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            accen_src_tosx0::En1,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                accen_src_tosx0::En1,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en2(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            accen_src_tosx0::En2,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                accen_src_tosx0::En2,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en3(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            accen_src_tosx0::En3,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                accen_src_tosx0::En3,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en4(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            accen_src_tosx0::En4,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                accen_src_tosx0::En4,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en5(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            accen_src_tosx0::En5,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                accen_src_tosx0::En5,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en6(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x1,
            1,
            0,
            accen_src_tosx0::En6,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                6,
                0x1,
                1,
                0,
                accen_src_tosx0::En6,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en7(
            self,
        ) -> crate::common::RegisterField<
            7,
            0x1,
            1,
            0,
            accen_src_tosx0::En7,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                accen_src_tosx0::En7,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en8(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1,
            1,
            0,
            accen_src_tosx0::En8,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                accen_src_tosx0::En8,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en9(
            self,
        ) -> crate::common::RegisterField<
            9,
            0x1,
            1,
            0,
            accen_src_tosx0::En9,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                9,
                0x1,
                1,
                0,
                accen_src_tosx0::En9,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en10(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            accen_src_tosx0::En10,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                accen_src_tosx0::En10,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en11(
            self,
        ) -> crate::common::RegisterField<
            11,
            0x1,
            1,
            0,
            accen_src_tosx0::En11,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                accen_src_tosx0::En11,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en12(
            self,
        ) -> crate::common::RegisterField<
            12,
            0x1,
            1,
            0,
            accen_src_tosx0::En12,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                12,
                0x1,
                1,
                0,
                accen_src_tosx0::En12,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en13(
            self,
        ) -> crate::common::RegisterField<
            13,
            0x1,
            1,
            0,
            accen_src_tosx0::En13,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                13,
                0x1,
                1,
                0,
                accen_src_tosx0::En13,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en14(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x1,
            1,
            0,
            accen_src_tosx0::En14,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                accen_src_tosx0::En14,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en15(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            accen_src_tosx0::En15,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                accen_src_tosx0::En15,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en16(
            self,
        ) -> crate::common::RegisterField<
            16,
            0x1,
            1,
            0,
            accen_src_tosx0::En16,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0x1,
                1,
                0,
                accen_src_tosx0::En16,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en17(
            self,
        ) -> crate::common::RegisterField<
            17,
            0x1,
            1,
            0,
            accen_src_tosx0::En17,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                17,
                0x1,
                1,
                0,
                accen_src_tosx0::En17,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en18(
            self,
        ) -> crate::common::RegisterField<
            18,
            0x1,
            1,
            0,
            accen_src_tosx0::En18,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                18,
                0x1,
                1,
                0,
                accen_src_tosx0::En18,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en19(
            self,
        ) -> crate::common::RegisterField<
            19,
            0x1,
            1,
            0,
            accen_src_tosx0::En19,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                19,
                0x1,
                1,
                0,
                accen_src_tosx0::En19,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en20(
            self,
        ) -> crate::common::RegisterField<
            20,
            0x1,
            1,
            0,
            accen_src_tosx0::En20,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0x1,
                1,
                0,
                accen_src_tosx0::En20,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en21(
            self,
        ) -> crate::common::RegisterField<
            21,
            0x1,
            1,
            0,
            accen_src_tosx0::En21,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                21,
                0x1,
                1,
                0,
                accen_src_tosx0::En21,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en22(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x1,
            1,
            0,
            accen_src_tosx0::En22,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                22,
                0x1,
                1,
                0,
                accen_src_tosx0::En22,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en23(
            self,
        ) -> crate::common::RegisterField<
            23,
            0x1,
            1,
            0,
            accen_src_tosx0::En23,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                23,
                0x1,
                1,
                0,
                accen_src_tosx0::En23,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en24(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x1,
            1,
            0,
            accen_src_tosx0::En24,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0x1,
                1,
                0,
                accen_src_tosx0::En24,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en25(
            self,
        ) -> crate::common::RegisterField<
            25,
            0x1,
            1,
            0,
            accen_src_tosx0::En25,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                25,
                0x1,
                1,
                0,
                accen_src_tosx0::En25,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en26(
            self,
        ) -> crate::common::RegisterField<
            26,
            0x1,
            1,
            0,
            accen_src_tosx0::En26,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                26,
                0x1,
                1,
                0,
                accen_src_tosx0::En26,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en27(
            self,
        ) -> crate::common::RegisterField<
            27,
            0x1,
            1,
            0,
            accen_src_tosx0::En27,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                27,
                0x1,
                1,
                0,
                accen_src_tosx0::En27,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en28(
            self,
        ) -> crate::common::RegisterField<
            28,
            0x1,
            1,
            0,
            accen_src_tosx0::En28,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                28,
                0x1,
                1,
                0,
                accen_src_tosx0::En28,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en29(
            self,
        ) -> crate::common::RegisterField<
            29,
            0x1,
            1,
            0,
            accen_src_tosx0::En29,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                29,
                0x1,
                1,
                0,
                accen_src_tosx0::En29,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en30(
            self,
        ) -> crate::common::RegisterField<
            30,
            0x1,
            1,
            0,
            accen_src_tosx0::En30,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                30,
                0x1,
                1,
                0,
                accen_src_tosx0::En30,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID i"]
        #[inline(always)]
        pub fn en31(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            accen_src_tosx0::En31,
            AccenSrcToSx0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                accen_src_tosx0::En31,
                AccenSrcToSx0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for AccenSrcToSx0 {
        #[inline(always)]
        fn default() -> AccenSrcToSx0 {
            <crate::RegValueT<AccenSrcToSx0_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
    pub mod accen_src_tosx0 {
        pub struct En0_SPEC;
        pub type En0 = crate::EnumBitfieldStruct<u8, En0_SPEC>;
        impl En0 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En1_SPEC;
        pub type En1 = crate::EnumBitfieldStruct<u8, En1_SPEC>;
        impl En1 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En2_SPEC;
        pub type En2 = crate::EnumBitfieldStruct<u8, En2_SPEC>;
        impl En2 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En3_SPEC;
        pub type En3 = crate::EnumBitfieldStruct<u8, En3_SPEC>;
        impl En3 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En4_SPEC;
        pub type En4 = crate::EnumBitfieldStruct<u8, En4_SPEC>;
        impl En4 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En5_SPEC;
        pub type En5 = crate::EnumBitfieldStruct<u8, En5_SPEC>;
        impl En5 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En6_SPEC;
        pub type En6 = crate::EnumBitfieldStruct<u8, En6_SPEC>;
        impl En6 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En7_SPEC;
        pub type En7 = crate::EnumBitfieldStruct<u8, En7_SPEC>;
        impl En7 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En8_SPEC;
        pub type En8 = crate::EnumBitfieldStruct<u8, En8_SPEC>;
        impl En8 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En9_SPEC;
        pub type En9 = crate::EnumBitfieldStruct<u8, En9_SPEC>;
        impl En9 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En10_SPEC;
        pub type En10 = crate::EnumBitfieldStruct<u8, En10_SPEC>;
        impl En10 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En11_SPEC;
        pub type En11 = crate::EnumBitfieldStruct<u8, En11_SPEC>;
        impl En11 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En12_SPEC;
        pub type En12 = crate::EnumBitfieldStruct<u8, En12_SPEC>;
        impl En12 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En13_SPEC;
        pub type En13 = crate::EnumBitfieldStruct<u8, En13_SPEC>;
        impl En13 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En14_SPEC;
        pub type En14 = crate::EnumBitfieldStruct<u8, En14_SPEC>;
        impl En14 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En15_SPEC;
        pub type En15 = crate::EnumBitfieldStruct<u8, En15_SPEC>;
        impl En15 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En16_SPEC;
        pub type En16 = crate::EnumBitfieldStruct<u8, En16_SPEC>;
        impl En16 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En17_SPEC;
        pub type En17 = crate::EnumBitfieldStruct<u8, En17_SPEC>;
        impl En17 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En18_SPEC;
        pub type En18 = crate::EnumBitfieldStruct<u8, En18_SPEC>;
        impl En18 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En19_SPEC;
        pub type En19 = crate::EnumBitfieldStruct<u8, En19_SPEC>;
        impl En19 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En20_SPEC;
        pub type En20 = crate::EnumBitfieldStruct<u8, En20_SPEC>;
        impl En20 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En21_SPEC;
        pub type En21 = crate::EnumBitfieldStruct<u8, En21_SPEC>;
        impl En21 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En22_SPEC;
        pub type En22 = crate::EnumBitfieldStruct<u8, En22_SPEC>;
        impl En22 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En23_SPEC;
        pub type En23 = crate::EnumBitfieldStruct<u8, En23_SPEC>;
        impl En23 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En24_SPEC;
        pub type En24 = crate::EnumBitfieldStruct<u8, En24_SPEC>;
        impl En24 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En25_SPEC;
        pub type En25 = crate::EnumBitfieldStruct<u8, En25_SPEC>;
        impl En25 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En26_SPEC;
        pub type En26 = crate::EnumBitfieldStruct<u8, En26_SPEC>;
        impl En26 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En27_SPEC;
        pub type En27 = crate::EnumBitfieldStruct<u8, En27_SPEC>;
        impl En27 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En28_SPEC;
        pub type En28 = crate::EnumBitfieldStruct<u8, En28_SPEC>;
        impl En28 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En29_SPEC;
        pub type En29 = crate::EnumBitfieldStruct<u8, En29_SPEC>;
        impl En29 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En30_SPEC;
        pub type En30 = crate::EnumBitfieldStruct<u8, En30_SPEC>;
        impl En30 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En31_SPEC;
        pub type En31 = crate::EnumBitfieldStruct<u8, En31_SPEC>;
        impl En31 {
            #[doc = "Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
}
#[doc = "CH"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch(pub(super) *mut u8);
unsafe impl core::marker::Send for Ch {}
unsafe impl core::marker::Sync for Ch {}
impl Ch {
    #[doc = "Error Capture Register 0  related to ICU0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ecrx(&self) -> crate::common::Reg<ch::EcRx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Last Acknowledged Service Request Register 0  related to ICU0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn lasrx(&self) -> crate::common::Reg<ch::LasRx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Latest Winning Service Request Register 0  related to ICU0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn lwsrx(&self) -> crate::common::Reg<ch::LwsRx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod ch {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EcRx_SPEC;
    impl crate::sealed::RegSpec for EcRx_SPEC {
        type DataType = u32;
    }
    #[doc = "Error Capture Register 0  related to ICU0\n resetvalue={Application Reset:0x0}"]
    pub type EcRx = crate::RegValueT<EcRx_SPEC>;

    impl EcRx {
        #[doc = "Service Request Priority Number. This bit field shows the priority number of the last service request        where an error was detected. Bit field can be modified by writing to it."]
        #[inline(always)]
        pub fn pn(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, EcRx_SPEC, crate::common::RW> {
            crate::common::RegisterField::<0,0xff,1,0,u8, EcRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request ECC. This bit field shows the ECC of the last service request where an error        was detected. Bit field can be modified by writing to it. This bit field can be modified by  Writing to SRC 23 16   byte write  Writing to SRC 31 16   16 bit write"]
        #[inline(always)]
        pub fn ecc(
            self,
        ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, EcRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<10,0x1f,1,0,u8, EcRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Node ID. This bit field shows the ID of the last service request where an error        was detected. Bit field can be modified by writing to it"]
        #[inline(always)]
        pub fn id(
            self,
        ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, EcRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x3ff,1,0,u16, EcRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Error Overflow Bit. The EOVCLR bit is used to clear the EOV bit. The EOV bit must be cleared        togehter with the STAT bit."]
        #[inline(always)]
        pub fn eovclr(
            self,
        ) -> crate::common::RegisterField<28, 0x1, 1, 0, ecrx::Eovclr, EcRx_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<28,0x1,1,0,ecrx::Eovclr, EcRx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Error Status Bit. The STATCLR bit is used to clear the STAT bit."]
        #[inline(always)]
        pub fn statclr(
            self,
        ) -> crate::common::RegisterField<29, 0x1, 1, 0, ecrx::Statclr, EcRx_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<29,0x1,1,0,ecrx::Statclr, EcRx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Error Overflow Bit. The bit is set if an ECC error was detected by the ICU while        ECR.STAT   180 1  180   Error Overflow situation ."]
        #[inline(always)]
        pub fn eov(
            self,
        ) -> crate::common::RegisterField<30, 0x1, 1, 0, ecrx::Eov, EcRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<30,0x1,1,0,ecrx::Eov, EcRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Error Status Bit. The Error Status Bit is set whenever an ECC was detected by the ICU."]
        #[inline(always)]
        pub fn stat(
            self,
        ) -> crate::common::RegisterField<31, 0x1, 1, 0, ecrx::Stat, EcRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<31,0x1,1,0,ecrx::Stat, EcRx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for EcRx {
        #[inline(always)]
        fn default() -> EcRx {
            <crate::RegValueT<EcRx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod ecrx {
        pub struct Eovclr_SPEC;
        pub type Eovclr = crate::EnumBitfieldStruct<u8, Eovclr_SPEC>;
        impl Eovclr {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear EOV bit  bit value is not stored  read always returns 0  no action if EOV bit is set in parallel."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Statclr_SPEC;
        pub type Statclr = crate::EnumBitfieldStruct<u8, Statclr_SPEC>;
        impl Statclr {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear STAT bit  bit value is not stored  read always returns 0  no action if STAT bit is set in parallel."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Eov_SPEC;
        pub type Eov = crate::EnumBitfieldStruct<u8, Eov_SPEC>;
        impl Eov {
            #[doc = "No Error Overflow situation detected"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Error Overflow situation detected"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Stat_SPEC;
        pub type Stat = crate::EnumBitfieldStruct<u8, Stat_SPEC>;
        impl Stat {
            #[doc = "No ECC error detected"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "ECC error detected"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LasRx_SPEC;
    impl crate::sealed::RegSpec for LasRx_SPEC {
        type DataType = u32;
    }
    #[doc = "Last Acknowledged Service Request Register 0  related to ICU0\n resetvalue={Application Reset:0x0}"]
    pub type LasRx = crate::RegValueT<LasRx_SPEC>;

    impl LasRx {
        #[doc = "Last Acknowledged Service Request Priority Number. This bit field shows the Priority Number of the last acknowledged        service request"]
        #[inline(always)]
        pub fn pn(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, LasRx_SPEC, crate::common::R> {
            crate::common::RegisterField::<0,0xff,1,0,u8, LasRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Last Acknowledged Interrupt ECC. This bit field shows the ECC value of the last acknowledged service        request  as send by the service provider with acknowledge In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
        #[inline(always)]
        pub fn ecc(
            self,
        ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, LasRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<10,0x1f,1,0,u8, LasRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Last Acknowledged Interrupt SRN ID. This bit field shows the ID number of the last acknowledged service        request  as sent by the service provider with acknowledge"]
        #[inline(always)]
        pub fn id(
            self,
        ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, LasRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0x3ff,1,0,u16, LasRx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for LasRx {
        #[inline(always)]
        fn default() -> LasRx {
            <crate::RegValueT<LasRx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LwsRx_SPEC;
    impl crate::sealed::RegSpec for LwsRx_SPEC {
        type DataType = u32;
    }
    #[doc = "Latest Winning Service Request Register 0  related to ICU0\n resetvalue={Application Reset:0x0}"]
    pub type LwsRx = crate::RegValueT<LwsRx_SPEC>;

    impl LwsRx {
        #[doc = "Latest Winner Priority Number. This bit field shows the Priority Number of a pending service request        that won the last arbitration round. This bit field is only valid if        STAT is set to 1"]
        #[inline(always)]
        pub fn pn(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, LwsRx_SPEC, crate::common::R> {
            crate::common::RegisterField::<0,0xff,1,0,u8, LwsRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Latest Winner ECC. This bit field shows the ECC field  SRN.ECC  that was transferred from        the last winning SRN to the ICU. This bit field is only valid if STAT is        set to 1. In the current implementation the ECC code is only used for error          detection. Detected errors are reported to the SMU but not corrected."]
        #[inline(always)]
        pub fn ecc(
            self,
        ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, LwsRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<10,0x1f,1,0,u8, LwsRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Latest Winner Index Number. This bit field shows the ID number of the last winning SRN. This bit        field is only valid if STAT is set to 1"]
        #[inline(always)]
        pub fn id(
            self,
        ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, LwsRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0x3ff,1,0,u16, LwsRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "LWSR Register Status. The STAT register indicates if the PN  ECC and ID bit fields are still        valid. They are still valid if the interrupt from the SRN identified by        ID is still pending. If the ICU does not have an winner because no        interrupt is pending or not yet arbitrated then it clears the STAT bit."]
        #[inline(always)]
        pub fn stat(
            self,
        ) -> crate::common::RegisterField<31, 0x1, 1, 0, lwsrx::Stat, LwsRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<31,0x1,1,0,lwsrx::Stat, LwsRx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for LwsRx {
        #[inline(always)]
        fn default() -> LwsRx {
            <crate::RegValueT<LwsRx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod lwsrx {
        pub struct Stat_SPEC;
        pub type Stat = crate::EnumBitfieldStruct<u8, Stat_SPEC>;
        impl Stat {
            #[doc = "LWSR bit fields are not valid"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "LWSR bit fields are valid"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
}
