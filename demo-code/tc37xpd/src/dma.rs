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
#[doc = r"DMA"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma(pub(super) *mut u8);
unsafe impl core::marker::Send for Dma {}
unsafe impl core::marker::Sync for Dma {}
impl Dma {
    #[doc = "DMA Clock Control Register\n resetvalue={Application Reset:0x08}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "ME 1 Clear Error Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn clre0(&self) -> crate::common::Reg<self::Clre0_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(296usize)) }
    }

    #[doc = "ME 1 Clear Error Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn clre1(&self) -> crate::common::Reg<self::Clre1_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4392usize)) }
    }

    #[doc = "ME 1 Enable Error Register\n resetvalue={Application Reset:0x4030000}"]
    #[inline(always)]
    pub const fn eer0(&self) -> crate::common::Reg<self::Eer0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(288usize)) }
    }

    #[doc = "ME 1 Enable Error Register\n resetvalue={Application Reset:0x4030000}"]
    #[inline(always)]
    pub const fn eer1(&self) -> crate::common::Reg<self::Eer1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4384usize)) }
    }

    #[doc = "RP 0 Error Interrupt Set Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn errintrr(&self) -> [crate::common::Reg<self::ErrintRr_SPEC, crate::common::W>; 4] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x1320usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1320usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1320usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1320usize + 0xcusize)),
            ]
        }
    }

    #[doc = "ME 1 Error Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn errsr0(&self) -> crate::common::Reg<self::Errsr0_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(292usize)) }
    }

    #[doc = "ME 1 Error Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn errsr1(&self) -> crate::common::Reg<self::Errsr1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4388usize)) }
    }

    #[doc = "DMA Channel 000 Resource Partition Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hrrc(&self) -> [crate::common::Reg<self::HrRc_SPEC, crate::common::RW>; 128] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x5cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x60usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x64usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x68usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x6cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x70usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x74usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x78usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x7cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x80usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x84usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x88usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x8cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x90usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x94usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x98usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x9cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xa0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xa4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xa8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xacusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xb0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xb4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xb8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xbcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xc0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xc4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xc8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xccusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xd0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xd4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xd8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xdcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xe0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xe4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xe8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xecusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xf0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xf4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xf8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0xfcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x100usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x104usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x108usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x10cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x110usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x114usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x118usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x11cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x120usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x124usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x128usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x12cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x130usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x134usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x138usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x13cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x140usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x144usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x148usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x14cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x150usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x154usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x158usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x15cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x160usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x164usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x168usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x16cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x170usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x174usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x178usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x17cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x180usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x184usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x188usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x18cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x190usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x194usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x198usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x19cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1a0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1a4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1a8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1acusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1b0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1b4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1b8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1bcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1c0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1c4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1c8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1ccusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1d0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1d4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1d8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1dcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1e0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1e4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1e8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1ecusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1f0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1f4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1f8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1800usize + 0x1fcusize)),
            ]
        }
    }

    #[doc = "DMA Identification Register\n resetvalue={Application Reset:0x087C000}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "ME 1 Read Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me00r(&self) -> crate::common::Reg<self::Me00R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(320usize)) }
    }

    #[doc = "ME 1 Read Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me01r(&self) -> crate::common::Reg<self::Me01R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(324usize)) }
    }

    #[doc = "ME 1 Read Register 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me02r(&self) -> crate::common::Reg<self::Me02R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(328usize)) }
    }

    #[doc = "ME 1 Read Register 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me03r(&self) -> crate::common::Reg<self::Me03R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(332usize)) }
    }

    #[doc = "ME 1 Read Register 4\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me04r(&self) -> crate::common::Reg<self::Me04R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(336usize)) }
    }

    #[doc = "ME 1 Read Register 5\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me05r(&self) -> crate::common::Reg<self::Me05R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(340usize)) }
    }

    #[doc = "ME 1 Read Register 6\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me06r(&self) -> crate::common::Reg<self::Me06R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(344usize)) }
    }

    #[doc = "ME 1 Read Register 7\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me07r(&self) -> crate::common::Reg<self::Me07R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(348usize)) }
    }

    #[doc = "ME 1 Channel Address and Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me0adicr(&self) -> crate::common::Reg<self::Me0Adicr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(400usize)) }
    }

    #[doc = "ME 1 Channel Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me0chcr(&self) -> crate::common::Reg<self::Me0Chcr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(404usize)) }
    }

    #[doc = "ME 1 Channel Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me0chsr(&self) -> crate::common::Reg<self::Me0Chsr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(412usize)) }
    }

    #[doc = "ME 1 Channel Destination Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me0dadr(&self) -> crate::common::Reg<self::Me0Dadr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(396usize)) }
    }

    #[doc = "ME 1 Channel Read Data CRC Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me0rdcrc(&self) -> crate::common::Reg<self::Me0Rdcrc_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(384usize)) }
    }

    #[doc = "ME 1 Channel Source Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me0sadr(&self) -> crate::common::Reg<self::Me0Sadr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(392usize)) }
    }

    #[doc = "ME 1 Channel Source and Destination Address CRC Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me0sdcrc(&self) -> crate::common::Reg<self::Me0Sdcrc_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(388usize)) }
    }

    #[doc = "ME 1 Channel Shadow Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me0shadr(&self) -> crate::common::Reg<self::Me0Shadr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(408usize)) }
    }

    #[doc = "ME 1 Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me0sr(&self) -> crate::common::Reg<self::Me0Sr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(304usize)) }
    }

    #[doc = "ME 1 Read Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me10r(&self) -> crate::common::Reg<self::Me10R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4416usize)) }
    }

    #[doc = "ME 1 Read Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me11r(&self) -> crate::common::Reg<self::Me11R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4420usize)) }
    }

    #[doc = "ME 1 Read Register 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me12r(&self) -> crate::common::Reg<self::Me12R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4424usize)) }
    }

    #[doc = "ME 1 Read Register 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me13r(&self) -> crate::common::Reg<self::Me13R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4428usize)) }
    }

    #[doc = "ME 1 Read Register 4\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me14r(&self) -> crate::common::Reg<self::Me14R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4432usize)) }
    }

    #[doc = "ME 1 Read Register 5\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me15r(&self) -> crate::common::Reg<self::Me15R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4436usize)) }
    }

    #[doc = "ME 1 Read Register 6\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me16r(&self) -> crate::common::Reg<self::Me16R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4440usize)) }
    }

    #[doc = "ME 1 Read Register 7\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me17r(&self) -> crate::common::Reg<self::Me17R_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4444usize)) }
    }

    #[doc = "ME 1 Channel Address and Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me1adicr(&self) -> crate::common::Reg<self::Me1Adicr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4496usize)) }
    }

    #[doc = "ME 1 Channel Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me1chcr(&self) -> crate::common::Reg<self::Me1Chcr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4500usize)) }
    }

    #[doc = "ME 1 Channel Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me1chsr(&self) -> crate::common::Reg<self::Me1Chsr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4508usize)) }
    }

    #[doc = "ME 1 Channel Destination Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me1dadr(&self) -> crate::common::Reg<self::Me1Dadr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4492usize)) }
    }

    #[doc = "ME 1 Channel Read Data CRC Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me1rdcrc(&self) -> crate::common::Reg<self::Me1Rdcrc_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4480usize)) }
    }

    #[doc = "ME 1 Channel Source Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me1sadr(&self) -> crate::common::Reg<self::Me1Sadr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4488usize)) }
    }

    #[doc = "ME 1 Channel Source and Destination Address CRC Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me1sdcrc(&self) -> crate::common::Reg<self::Me1Sdcrc_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4484usize)) }
    }

    #[doc = "ME 1 Channel Shadow Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me1shadr(&self) -> crate::common::Reg<self::Me1Shadr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4504usize)) }
    }

    #[doc = "ME 1 Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn me1sr(&self) -> crate::common::Reg<self::Me1Sr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4400usize)) }
    }

    #[doc = "RP 0 Mode Register\n resetvalue={Application Reset:0x1}"]
    #[inline(always)]
    pub const fn moder(&self) -> [crate::common::Reg<self::ModEr_SPEC, crate::common::RW>; 4] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x1300usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1300usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1300usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1300usize + 0xcusize)),
            ]
        }
    }

    #[doc = "DMA OCDS Trigger Set Select\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn otss(&self) -> crate::common::Reg<self::Otss_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4608usize)) }
    }

    #[doc = "DMA Pattern Read Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn prr0(&self) -> crate::common::Reg<self::Prr0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4616usize)) }
    }

    #[doc = "DMA Pattern Read Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn prr1(&self) -> crate::common::Reg<self::Prr1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4620usize)) }
    }

    #[doc = "DMA Channel 000 Suspend Acknowledge Register\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn susacrc(&self) -> [crate::common::Reg<self::SusacRc_SPEC, crate::common::R>; 128] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x5cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x60usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x64usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x68usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x6cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x70usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x74usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x78usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x7cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x80usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x84usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x88usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x8cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x90usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x94usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x98usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x9cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xa0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xa4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xa8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xacusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xb0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xb4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xb8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xbcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xc0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xc4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xc8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xccusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xd0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xd4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xd8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xdcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xe0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xe4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xe8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xecusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xf0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xf4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xf8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0xfcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x100usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x104usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x108usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x10cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x110usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x114usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x118usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x11cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x120usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x124usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x128usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x12cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x130usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x134usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x138usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x13cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x140usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x144usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x148usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x14cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x150usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x154usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x158usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x15cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x160usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x164usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x168usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x16cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x170usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x174usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x178usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x17cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x180usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x184usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x188usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x18cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x190usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x194usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x198usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x19cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1a0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1a4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1a8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1acusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1b0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1b4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1b8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1bcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1c0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1c4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1c8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1ccusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1d0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1d4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1d8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1dcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1e0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1e4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1e8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1ecusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1f0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1f4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1f8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1c00usize + 0x1fcusize)),
            ]
        }
    }

    #[doc = "DMA Channel 000 Suspend Enable Register\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn susenrc(
        &self,
    ) -> [crate::common::Reg<self::SusenRc_SPEC, crate::common::RW>; 128] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x5cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x60usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x64usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x68usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x6cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x70usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x74usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x78usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x7cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x80usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x84usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x88usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x8cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x90usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x94usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x98usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x9cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xa0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xa4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xa8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xacusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xb0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xb4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xb8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xbcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xc0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xc4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xc8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xccusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xd0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xd4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xd8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xdcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xe0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xe4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xe8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xecusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xf0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xf4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xf8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0xfcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x100usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x104usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x108usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x10cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x110usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x114usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x118usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x11cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x120usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x124usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x128usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x12cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x130usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x134usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x138usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x13cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x140usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x144usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x148usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x14cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x150usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x154usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x158usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x15cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x160usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x164usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x168usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x16cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x170usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x174usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x178usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x17cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x180usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x184usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x188usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x18cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x190usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x194usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x198usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x19cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1a0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1a4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1a8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1acusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1b0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1b4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1b8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1bcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1c0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1c4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1c8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1ccusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1d0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1d4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1d8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1dcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1e0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1e4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1e8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1ecusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1f0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1f4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1f8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a00usize + 0x1fcusize)),
            ]
        }
    }

    #[doc = "DMA Time Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn time(&self) -> crate::common::Reg<self::Time_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4624usize)) }
    }

    #[doc = "DMA Channel 000 Transaction State Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tsrc(&self) -> [crate::common::Reg<self::TsRc_SPEC, crate::common::RW>; 128] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x5cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x60usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x64usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x68usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x6cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x70usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x74usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x78usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x7cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x80usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x84usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x88usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x8cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x90usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x94usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x98usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x9cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xa0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xa4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xa8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xacusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xb0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xb4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xb8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xbcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xc0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xc4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xc8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xccusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xd0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xd4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xd8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xdcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xe0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xe4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xe8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xecusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xf0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xf4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xf8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0xfcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x100usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x104usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x108usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x10cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x110usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x114usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x118usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x11cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x120usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x124usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x128usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x12cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x130usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x134usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x138usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x13cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x140usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x144usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x148usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x14cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x150usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x154usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x158usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x15cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x160usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x164usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x168usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x16cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x170usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x174usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x178usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x17cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x180usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x184usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x188usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x18cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x190usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x194usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x198usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x19cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1a0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1a4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1a8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1acusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1b0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1b4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1b8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1bcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1c0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1c4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1c8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1ccusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1d0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1d4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1d8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1dcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1e0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1e4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1e8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1ecusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1f0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1f4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1f8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1e00usize + 0x1fcusize)),
            ]
        }
    }
    #[doc = "ACCEN"]
    #[inline(always)]
    pub fn accen(self) -> [self::Accen; 4] {
        unsafe {
            [
                self::Accen(self.0.add(0x40usize + 0x0usize)),
                self::Accen(self.0.add(0x40usize + 0x8usize)),
                self::Accen(self.0.add(0x40usize + 0x10usize)),
                self::Accen(self.0.add(0x40usize + 0x18usize)),
            ]
        }
    }
    #[doc = "CH"]
    #[inline(always)]
    pub fn ch(self) -> [self::Ch; 128] {
        unsafe {
            [
                self::Ch(self.0.add(0x2000usize + 0x0usize)),
                self::Ch(self.0.add(0x2000usize + 0x20usize)),
                self::Ch(self.0.add(0x2000usize + 0x40usize)),
                self::Ch(self.0.add(0x2000usize + 0x60usize)),
                self::Ch(self.0.add(0x2000usize + 0x80usize)),
                self::Ch(self.0.add(0x2000usize + 0xa0usize)),
                self::Ch(self.0.add(0x2000usize + 0xc0usize)),
                self::Ch(self.0.add(0x2000usize + 0xe0usize)),
                self::Ch(self.0.add(0x2000usize + 0x100usize)),
                self::Ch(self.0.add(0x2000usize + 0x120usize)),
                self::Ch(self.0.add(0x2000usize + 0x140usize)),
                self::Ch(self.0.add(0x2000usize + 0x160usize)),
                self::Ch(self.0.add(0x2000usize + 0x180usize)),
                self::Ch(self.0.add(0x2000usize + 0x1a0usize)),
                self::Ch(self.0.add(0x2000usize + 0x1c0usize)),
                self::Ch(self.0.add(0x2000usize + 0x1e0usize)),
                self::Ch(self.0.add(0x2000usize + 0x200usize)),
                self::Ch(self.0.add(0x2000usize + 0x220usize)),
                self::Ch(self.0.add(0x2000usize + 0x240usize)),
                self::Ch(self.0.add(0x2000usize + 0x260usize)),
                self::Ch(self.0.add(0x2000usize + 0x280usize)),
                self::Ch(self.0.add(0x2000usize + 0x2a0usize)),
                self::Ch(self.0.add(0x2000usize + 0x2c0usize)),
                self::Ch(self.0.add(0x2000usize + 0x2e0usize)),
                self::Ch(self.0.add(0x2000usize + 0x300usize)),
                self::Ch(self.0.add(0x2000usize + 0x320usize)),
                self::Ch(self.0.add(0x2000usize + 0x340usize)),
                self::Ch(self.0.add(0x2000usize + 0x360usize)),
                self::Ch(self.0.add(0x2000usize + 0x380usize)),
                self::Ch(self.0.add(0x2000usize + 0x3a0usize)),
                self::Ch(self.0.add(0x2000usize + 0x3c0usize)),
                self::Ch(self.0.add(0x2000usize + 0x3e0usize)),
                self::Ch(self.0.add(0x2000usize + 0x400usize)),
                self::Ch(self.0.add(0x2000usize + 0x420usize)),
                self::Ch(self.0.add(0x2000usize + 0x440usize)),
                self::Ch(self.0.add(0x2000usize + 0x460usize)),
                self::Ch(self.0.add(0x2000usize + 0x480usize)),
                self::Ch(self.0.add(0x2000usize + 0x4a0usize)),
                self::Ch(self.0.add(0x2000usize + 0x4c0usize)),
                self::Ch(self.0.add(0x2000usize + 0x4e0usize)),
                self::Ch(self.0.add(0x2000usize + 0x500usize)),
                self::Ch(self.0.add(0x2000usize + 0x520usize)),
                self::Ch(self.0.add(0x2000usize + 0x540usize)),
                self::Ch(self.0.add(0x2000usize + 0x560usize)),
                self::Ch(self.0.add(0x2000usize + 0x580usize)),
                self::Ch(self.0.add(0x2000usize + 0x5a0usize)),
                self::Ch(self.0.add(0x2000usize + 0x5c0usize)),
                self::Ch(self.0.add(0x2000usize + 0x5e0usize)),
                self::Ch(self.0.add(0x2000usize + 0x600usize)),
                self::Ch(self.0.add(0x2000usize + 0x620usize)),
                self::Ch(self.0.add(0x2000usize + 0x640usize)),
                self::Ch(self.0.add(0x2000usize + 0x660usize)),
                self::Ch(self.0.add(0x2000usize + 0x680usize)),
                self::Ch(self.0.add(0x2000usize + 0x6a0usize)),
                self::Ch(self.0.add(0x2000usize + 0x6c0usize)),
                self::Ch(self.0.add(0x2000usize + 0x6e0usize)),
                self::Ch(self.0.add(0x2000usize + 0x700usize)),
                self::Ch(self.0.add(0x2000usize + 0x720usize)),
                self::Ch(self.0.add(0x2000usize + 0x740usize)),
                self::Ch(self.0.add(0x2000usize + 0x760usize)),
                self::Ch(self.0.add(0x2000usize + 0x780usize)),
                self::Ch(self.0.add(0x2000usize + 0x7a0usize)),
                self::Ch(self.0.add(0x2000usize + 0x7c0usize)),
                self::Ch(self.0.add(0x2000usize + 0x7e0usize)),
                self::Ch(self.0.add(0x2000usize + 0x800usize)),
                self::Ch(self.0.add(0x2000usize + 0x820usize)),
                self::Ch(self.0.add(0x2000usize + 0x840usize)),
                self::Ch(self.0.add(0x2000usize + 0x860usize)),
                self::Ch(self.0.add(0x2000usize + 0x880usize)),
                self::Ch(self.0.add(0x2000usize + 0x8a0usize)),
                self::Ch(self.0.add(0x2000usize + 0x8c0usize)),
                self::Ch(self.0.add(0x2000usize + 0x8e0usize)),
                self::Ch(self.0.add(0x2000usize + 0x900usize)),
                self::Ch(self.0.add(0x2000usize + 0x920usize)),
                self::Ch(self.0.add(0x2000usize + 0x940usize)),
                self::Ch(self.0.add(0x2000usize + 0x960usize)),
                self::Ch(self.0.add(0x2000usize + 0x980usize)),
                self::Ch(self.0.add(0x2000usize + 0x9a0usize)),
                self::Ch(self.0.add(0x2000usize + 0x9c0usize)),
                self::Ch(self.0.add(0x2000usize + 0x9e0usize)),
                self::Ch(self.0.add(0x2000usize + 0xa00usize)),
                self::Ch(self.0.add(0x2000usize + 0xa20usize)),
                self::Ch(self.0.add(0x2000usize + 0xa40usize)),
                self::Ch(self.0.add(0x2000usize + 0xa60usize)),
                self::Ch(self.0.add(0x2000usize + 0xa80usize)),
                self::Ch(self.0.add(0x2000usize + 0xaa0usize)),
                self::Ch(self.0.add(0x2000usize + 0xac0usize)),
                self::Ch(self.0.add(0x2000usize + 0xae0usize)),
                self::Ch(self.0.add(0x2000usize + 0xb00usize)),
                self::Ch(self.0.add(0x2000usize + 0xb20usize)),
                self::Ch(self.0.add(0x2000usize + 0xb40usize)),
                self::Ch(self.0.add(0x2000usize + 0xb60usize)),
                self::Ch(self.0.add(0x2000usize + 0xb80usize)),
                self::Ch(self.0.add(0x2000usize + 0xba0usize)),
                self::Ch(self.0.add(0x2000usize + 0xbc0usize)),
                self::Ch(self.0.add(0x2000usize + 0xbe0usize)),
                self::Ch(self.0.add(0x2000usize + 0xc00usize)),
                self::Ch(self.0.add(0x2000usize + 0xc20usize)),
                self::Ch(self.0.add(0x2000usize + 0xc40usize)),
                self::Ch(self.0.add(0x2000usize + 0xc60usize)),
                self::Ch(self.0.add(0x2000usize + 0xc80usize)),
                self::Ch(self.0.add(0x2000usize + 0xca0usize)),
                self::Ch(self.0.add(0x2000usize + 0xcc0usize)),
                self::Ch(self.0.add(0x2000usize + 0xce0usize)),
                self::Ch(self.0.add(0x2000usize + 0xd00usize)),
                self::Ch(self.0.add(0x2000usize + 0xd20usize)),
                self::Ch(self.0.add(0x2000usize + 0xd40usize)),
                self::Ch(self.0.add(0x2000usize + 0xd60usize)),
                self::Ch(self.0.add(0x2000usize + 0xd80usize)),
                self::Ch(self.0.add(0x2000usize + 0xda0usize)),
                self::Ch(self.0.add(0x2000usize + 0xdc0usize)),
                self::Ch(self.0.add(0x2000usize + 0xde0usize)),
                self::Ch(self.0.add(0x2000usize + 0xe00usize)),
                self::Ch(self.0.add(0x2000usize + 0xe20usize)),
                self::Ch(self.0.add(0x2000usize + 0xe40usize)),
                self::Ch(self.0.add(0x2000usize + 0xe60usize)),
                self::Ch(self.0.add(0x2000usize + 0xe80usize)),
                self::Ch(self.0.add(0x2000usize + 0xea0usize)),
                self::Ch(self.0.add(0x2000usize + 0xec0usize)),
                self::Ch(self.0.add(0x2000usize + 0xee0usize)),
                self::Ch(self.0.add(0x2000usize + 0xf00usize)),
                self::Ch(self.0.add(0x2000usize + 0xf20usize)),
                self::Ch(self.0.add(0x2000usize + 0xf40usize)),
                self::Ch(self.0.add(0x2000usize + 0xf60usize)),
                self::Ch(self.0.add(0x2000usize + 0xf80usize)),
                self::Ch(self.0.add(0x2000usize + 0xfa0usize)),
                self::Ch(self.0.add(0x2000usize + 0xfc0usize)),
                self::Ch(self.0.add(0x2000usize + 0xfe0usize)),
            ]
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clc_SPEC;
impl crate::sealed::RegSpec for Clc_SPEC {
    type DataType = u32;
}
#[doc = "DMA Clock Control Register\n resetvalue={Application Reset:0x08}"]
pub type Clc = crate::RegValueT<Clc_SPEC>;

impl Clc {
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control of the DMA"]
    #[inline(always)]
    pub fn disr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, clc::Disr, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,clc::Disr, Clc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Module Disable Status Bit   DISS. Bit indicates the current status of the DMA"]
    #[inline(always)]
    pub fn diss(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, clc::Diss, Clc_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,clc::Diss, Clc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Sleep Mode Enable Control   EDIS. Used for DMA sleep mode control."]
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
        <crate::RegValueT<Clc_SPEC> as RegisterValue<_>>::new(8)
    }
}
pub mod clc {
    pub struct Disr_SPEC;
    pub type Disr = crate::EnumBitfieldStruct<u8, Disr_SPEC>;
    impl Disr {
        #[doc = "0 DMA enable is        requested."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 DMA disable is        requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Diss_SPEC;
    pub type Diss = crate::EnumBitfieldStruct<u8, Diss_SPEC>;
    impl Diss {
        #[doc = "0 DMA is enabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 DMA is        disabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Edis_SPEC;
    pub type Edis = crate::EnumBitfieldStruct<u8, Edis_SPEC>;
    impl Edis {
        #[doc = "0 Sleep control        enabled. DMA may enter sleep mode."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Sleep control        disabled. DMA shall not enter sleep mode."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clre0_SPEC;
impl crate::sealed::RegSpec for Clre0_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Clear Error Register\n resetvalue={Application Reset:0x0}"]
pub type Clre0 = crate::RegValueT<Clre0_SPEC>;

impl Clre0 {
    #[doc = "Clear ME Source Error   CSER"]
    #[inline(always)]
    pub fn cser(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, clre0::Cser, Clre0_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0x1,1,0,clre0::Cser, Clre0_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear ME Destination Error   CDER"]
    #[inline(always)]
    pub fn cder(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, clre0::Cder, Clre0_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<17,0x1,1,0,clre0::Cder, Clre0_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear SPB Error   CSPBER"]
    #[inline(always)]
    pub fn cspber(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, clre0::Cspber, Clre0_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<20,0x1,1,0,clre0::Cspber, Clre0_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear SRI Error   CSRIER"]
    #[inline(always)]
    pub fn csrier(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, clre0::Csrier, Clre0_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<21,0x1,1,0,clre0::Csrier, Clre0_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear RAM Error   CRAMER"]
    #[inline(always)]
    pub fn cramer(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, clre0::Cramer, Clre0_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<24,0x1,1,0,clre0::Cramer, Clre0_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear SLL Error   CSLLER"]
    #[inline(always)]
    pub fn csller(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, clre0::Csller, Clre0_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<25,0x1,1,0,clre0::Csller, Clre0_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear DLL Error   CDLLER"]
    #[inline(always)]
    pub fn cdller(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, clre0::Cdller, Clre0_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<26,0x1,1,0,clre0::Cdller, Clre0_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Clre0 {
    #[inline(always)]
    fn default() -> Clre0 {
        <crate::RegValueT<Clre0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod clre0 {
    pub struct Cser_SPEC;
    pub type Cser = crate::EnumBitfieldStruct<u8, Cser_SPEC>;
    impl Cser {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear source        error flag DMA ERRSRm.SER."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cder_SPEC;
    pub type Cder = crate::EnumBitfieldStruct<u8, Cder_SPEC>;
    impl Cder {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear        destination error flag DMA ERRSRm.DER."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cspber_SPEC;
    pub type Cspber = crate::EnumBitfieldStruct<u8, Cspber_SPEC>;
    impl Cspber {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear error        flag DMA ERRSRm.SPBER."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Csrier_SPEC;
    pub type Csrier = crate::EnumBitfieldStruct<u8, Csrier_SPEC>;
    impl Csrier {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear error        flag DMA ERRSRm.SRIER."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cramer_SPEC;
    pub type Cramer = crate::EnumBitfieldStruct<u8, Cramer_SPEC>;
    impl Cramer {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear error        flag DMA ERRSRm.RAMER."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Csller_SPEC;
    pub type Csller = crate::EnumBitfieldStruct<u8, Csller_SPEC>;
    impl Csller {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear error        flag DMA ERRSRm.SLLER."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cdller_SPEC;
    pub type Cdller = crate::EnumBitfieldStruct<u8, Cdller_SPEC>;
    impl Cdller {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear error        flag DMA ERRSRm.DLLER."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clre1_SPEC;
impl crate::sealed::RegSpec for Clre1_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Clear Error Register\n resetvalue={Application Reset:0x0}"]
pub type Clre1 = crate::RegValueT<Clre1_SPEC>;

impl Clre1 {
    #[doc = "Clear ME Source Error   CSER"]
    #[inline(always)]
    pub fn cser(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, clre1::Cser, Clre1_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0x1,1,0,clre1::Cser, Clre1_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear ME Destination Error   CDER"]
    #[inline(always)]
    pub fn cder(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, clre1::Cder, Clre1_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<17,0x1,1,0,clre1::Cder, Clre1_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear SPB Error   CSPBER"]
    #[inline(always)]
    pub fn cspber(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, clre1::Cspber, Clre1_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<20,0x1,1,0,clre1::Cspber, Clre1_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear SRI Error   CSRIER"]
    #[inline(always)]
    pub fn csrier(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, clre1::Csrier, Clre1_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<21,0x1,1,0,clre1::Csrier, Clre1_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear RAM Error   CRAMER"]
    #[inline(always)]
    pub fn cramer(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, clre1::Cramer, Clre1_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<24,0x1,1,0,clre1::Cramer, Clre1_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear SLL Error   CSLLER"]
    #[inline(always)]
    pub fn csller(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, clre1::Csller, Clre1_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<25,0x1,1,0,clre1::Csller, Clre1_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear DLL Error   CDLLER"]
    #[inline(always)]
    pub fn cdller(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, clre1::Cdller, Clre1_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<26,0x1,1,0,clre1::Cdller, Clre1_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Clre1 {
    #[inline(always)]
    fn default() -> Clre1 {
        <crate::RegValueT<Clre1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod clre1 {
    pub struct Cser_SPEC;
    pub type Cser = crate::EnumBitfieldStruct<u8, Cser_SPEC>;
    impl Cser {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear source        error flag DMA ERRSRm.SER."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cder_SPEC;
    pub type Cder = crate::EnumBitfieldStruct<u8, Cder_SPEC>;
    impl Cder {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear        destination error flag DMA ERRSRm.DER."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cspber_SPEC;
    pub type Cspber = crate::EnumBitfieldStruct<u8, Cspber_SPEC>;
    impl Cspber {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear error        flag DMA ERRSRm.SPBER."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Csrier_SPEC;
    pub type Csrier = crate::EnumBitfieldStruct<u8, Csrier_SPEC>;
    impl Csrier {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear error        flag DMA ERRSRm.SRIER."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cramer_SPEC;
    pub type Cramer = crate::EnumBitfieldStruct<u8, Cramer_SPEC>;
    impl Cramer {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear error        flag DMA ERRSRm.RAMER."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Csller_SPEC;
    pub type Csller = crate::EnumBitfieldStruct<u8, Csller_SPEC>;
    impl Csller {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear error        flag DMA ERRSRm.SLLER."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cdller_SPEC;
    pub type Cdller = crate::EnumBitfieldStruct<u8, Cdller_SPEC>;
    impl Cdller {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear error        flag DMA ERRSRm.DLLER."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eer0_SPEC;
impl crate::sealed::RegSpec for Eer0_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Enable Error Register\n resetvalue={Application Reset:0x4030000}"]
pub type Eer0 = crate::RegValueT<Eer0_SPEC>;

impl Eer0 {
    #[doc = "Enable ME Source Error   ESER. This bit enables the generation of a ME source error interrupt."]
    #[inline(always)]
    pub fn eser(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, eer0::Eser, Eer0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1,1,0,eer0::Eser, Eer0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable ME Destination Error   EDER. This bit enables the generation of a ME destination error interrupt."]
    #[inline(always)]
    pub fn eder(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, eer0::Eder, Eer0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<17,0x1,1,0,eer0::Eder, Eer0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable ME DMA Linked List Error   ELER. This bit enables the generation of a ME DMA Linked List error interrupt."]
    #[inline(always)]
    pub fn eler(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, eer0::Eler, Eer0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x1,1,0,eer0::Eler, Eer0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Eer0 {
    #[inline(always)]
    fn default() -> Eer0 {
        <crate::RegValueT<Eer0_SPEC> as RegisterValue<_>>::new(67305472)
    }
}
pub mod eer0 {
    pub struct Eser_SPEC;
    pub type Eser = crate::EnumBitfieldStruct<u8, Eser_SPEC>;
    impl Eser {
        #[doc = "0 ME source error        interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ME source error        interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eder_SPEC;
    pub type Eder = crate::EnumBitfieldStruct<u8, Eder_SPEC>;
    impl Eder {
        #[doc = "0 ME destination        error interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ME destination        error interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eler_SPEC;
    pub type Eler = crate::EnumBitfieldStruct<u8, Eler_SPEC>;
    impl Eler {
        #[doc = "0 ME DMA Linked        List error interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ME DMA Linked        List error interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eer1_SPEC;
impl crate::sealed::RegSpec for Eer1_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Enable Error Register\n resetvalue={Application Reset:0x4030000}"]
pub type Eer1 = crate::RegValueT<Eer1_SPEC>;

impl Eer1 {
    #[doc = "Enable ME Source Error   ESER. This bit enables the generation of a ME source error interrupt."]
    #[inline(always)]
    pub fn eser(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, eer1::Eser, Eer1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1,1,0,eer1::Eser, Eer1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable ME Destination Error   EDER. This bit enables the generation of a ME destination error interrupt."]
    #[inline(always)]
    pub fn eder(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, eer1::Eder, Eer1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<17,0x1,1,0,eer1::Eder, Eer1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable ME DMA Linked List Error   ELER. This bit enables the generation of a ME DMA Linked List error interrupt."]
    #[inline(always)]
    pub fn eler(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, eer1::Eler, Eer1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x1,1,0,eer1::Eler, Eer1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Eer1 {
    #[inline(always)]
    fn default() -> Eer1 {
        <crate::RegValueT<Eer1_SPEC> as RegisterValue<_>>::new(67305472)
    }
}
pub mod eer1 {
    pub struct Eser_SPEC;
    pub type Eser = crate::EnumBitfieldStruct<u8, Eser_SPEC>;
    impl Eser {
        #[doc = "0 ME source error        interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ME source error        interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eder_SPEC;
    pub type Eder = crate::EnumBitfieldStruct<u8, Eder_SPEC>;
    impl Eder {
        #[doc = "0 ME destination        error interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ME destination        error interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eler_SPEC;
    pub type Eler = crate::EnumBitfieldStruct<u8, Eler_SPEC>;
    impl Eler {
        #[doc = "0 ME DMA Linked        List error interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ME DMA Linked        List error interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ErrintRr_SPEC;
impl crate::sealed::RegSpec for ErrintRr_SPEC {
    type DataType = u32;
}
#[doc = "RP 0 Error Interrupt Set Register\n resetvalue={Application Reset:0x0}"]
pub type ErrintRr = crate::RegValueT<ErrintRr_SPEC>;

impl ErrintRr {
    #[doc = "Set Error Interrupt Service Request   SIT. Reading this bit returns a 0."]
    #[inline(always)]
    pub fn sit(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, errintrr::Sit, ErrintRr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x1,1,0,errintrr::Sit, ErrintRr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for ErrintRr {
    #[inline(always)]
    fn default() -> ErrintRr {
        <crate::RegValueT<ErrintRr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod errintrr {
    pub struct Sit_SPEC;
    pub type Sit = crate::EnumBitfieldStruct<u8, Sit_SPEC>;
    impl Sit {
        #[doc = "0 No action."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 DMA error        interrupt service request will be activated."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errsr0_SPEC;
impl crate::sealed::RegSpec for Errsr0_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Error Status Register\n resetvalue={Application Reset:0x0}"]
pub type Errsr0 = crate::RegValueT<Errsr0_SPEC>;

impl Errsr0 {
    #[doc = "ME Last Error Channel   LEC. This bit field indicates the DMA channel number of the last DMA channel        of ME generating an error i.e. RAM error DMA ERRSRm.RAMER  Safe Linked        List error DMA ERRSRm.SLLER  DMA Linked List error DMA ERRSRm.DLLER and        all on  160 chip  160 bus errors."]
    #[inline(always)]
    pub fn lec(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Errsr0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7f,1,0,u8, Errsr0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ME Source Error   SER. This bit is set whenever a ME error occurred during a source  read  move        of a DMA transfer  or a request could not been serviced due to the        access protection."]
    #[inline(always)]
    pub fn ser(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, errsr0::Ser, Errsr0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1,1,0,errsr0::Ser, Errsr0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ME Destination Error   DER. This bit is set whenever a ME error occurred during a destination         write  move of a DMA transfer  or a request could not been serviced due        to the access protection."]
    #[inline(always)]
    pub fn der(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, errsr0::Der, Errsr0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<17,0x1,1,0,errsr0::Der, Errsr0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ME SPB Bus Error   SPBER. This bit is set when a ME DMA Move that has been started by the DMA SPB        master interface leads to an error on the SPB  160 Bus."]
    #[inline(always)]
    pub fn spber(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, errsr0::Spber, Errsr0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<20,0x1,1,0,errsr0::Spber, Errsr0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ME SRI Bus Error   SRIER. This bit is set when a ME DMA Move that has been started by the DMA SRI        master interface leads to an error on the SRI  160 Bus."]
    #[inline(always)]
    pub fn srier(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, errsr0::Srier, Errsr0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<21,0x1,1,0,errsr0::Srier, Errsr0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ME RAM Error   RAMER. This bit is set whenever a ME error occurred during the loading of a TCS        from the DMARAM to the ME channel registers."]
    #[inline(always)]
    pub fn ramer(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, errsr0::Ramer, Errsr0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0x1,1,0,errsr0::Ramer, Errsr0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ME Safe Linked List Error   SLLER. This bit is set when a ME error occurred during the comparison of a        SDCRC checksums during a safe linked list."]
    #[inline(always)]
    pub fn sller(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, errsr0::Sller, Errsr0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<25,0x1,1,0,errsr0::Sller, Errsr0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ME DMA Linked List Error   DLLER. This bit is set when a ME error occurred during a DMALL  ACCLL  SAFLL or        CONLL operation when a new TCS is loaded from anywhere in memory to        overwrite the current TCS stored in the DMARAM."]
    #[inline(always)]
    pub fn dller(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, errsr0::Dller, Errsr0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<26,0x1,1,0,errsr0::Dller, Errsr0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Errsr0 {
    #[inline(always)]
    fn default() -> Errsr0 {
        <crate::RegValueT<Errsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod errsr0 {
    pub struct Ser_SPEC;
    pub type Ser = crate::EnumBitfieldStruct<u8, Ser_SPEC>;
    impl Ser {
        #[doc = "0 No ME source        error has occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ME source error        has occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Der_SPEC;
    pub type Der = crate::EnumBitfieldStruct<u8, Der_SPEC>;
    impl Der {
        #[doc = "0 No ME        destination error has occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ME destination        error has occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Spber_SPEC;
    pub type Spber = crate::EnumBitfieldStruct<u8, Spber_SPEC>;
    impl Spber {
        #[doc = "0 No error        occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An error        occurred on SPB  160 Bus interface."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Srier_SPEC;
    pub type Srier = crate::EnumBitfieldStruct<u8, Srier_SPEC>;
    impl Srier {
        #[doc = "0 No error        occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An error        occurred on SRI  160 Bus interface."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ramer_SPEC;
    pub type Ramer = crate::EnumBitfieldStruct<u8, Ramer_SPEC>;
    impl Ramer {
        #[doc = "0 No error        occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An error        occurred during the load of a TCS."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sller_SPEC;
    pub type Sller = crate::EnumBitfieldStruct<u8, Sller_SPEC>;
    impl Sller {
        #[doc = "0 No error        occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An error        occurred during the SDCRC checksum comparison."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Dller_SPEC;
    pub type Dller = crate::EnumBitfieldStruct<u8, Dller_SPEC>;
    impl Dller {
        #[doc = "0 No error        occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An error        occurred during the loading of a new TCS."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errsr1_SPEC;
impl crate::sealed::RegSpec for Errsr1_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Error Status Register\n resetvalue={Application Reset:0x0}"]
pub type Errsr1 = crate::RegValueT<Errsr1_SPEC>;

impl Errsr1 {
    #[doc = "ME Last Error Channel   LEC. This bit field indicates the DMA channel number of the last DMA channel        of ME generating an error i.e. RAM error DMA ERRSRm.RAMER  Safe Linked        List error DMA ERRSRm.SLLER  DMA Linked List error DMA ERRSRm.DLLER and        all on  160 chip  160 bus errors."]
    #[inline(always)]
    pub fn lec(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Errsr1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7f,1,0,u8, Errsr1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ME Source Error   SER. This bit is set whenever a ME error occurred during a source  read  move        of a DMA transfer  or a request could not been serviced due to the        access protection."]
    #[inline(always)]
    pub fn ser(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, errsr1::Ser, Errsr1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1,1,0,errsr1::Ser, Errsr1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ME Destination Error   DER. This bit is set whenever a ME error occurred during a destination         write  move of a DMA transfer  or a request could not been serviced due        to the access protection."]
    #[inline(always)]
    pub fn der(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, errsr1::Der, Errsr1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<17,0x1,1,0,errsr1::Der, Errsr1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ME SPB Bus Error   SPBER. This bit is set when a ME DMA Move that has been started by the DMA SPB        master interface leads to an error on the SPB  160 Bus."]
    #[inline(always)]
    pub fn spber(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, errsr1::Spber, Errsr1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<20,0x1,1,0,errsr1::Spber, Errsr1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ME SRI Bus Error   SRIER. This bit is set when a ME DMA Move that has been started by the DMA SRI        master interface leads to an error on the SRI  160 Bus."]
    #[inline(always)]
    pub fn srier(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, errsr1::Srier, Errsr1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<21,0x1,1,0,errsr1::Srier, Errsr1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ME RAM Error   RAMER. This bit is set whenever a ME error occurred during the loading of a TCS        from the DMARAM to the ME channel registers."]
    #[inline(always)]
    pub fn ramer(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, errsr1::Ramer, Errsr1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0x1,1,0,errsr1::Ramer, Errsr1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ME Safe Linked List Error   SLLER. This bit is set when a ME error occurred during the comparison of a        SDCRC checksums during a safe linked list."]
    #[inline(always)]
    pub fn sller(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, errsr1::Sller, Errsr1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<25,0x1,1,0,errsr1::Sller, Errsr1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ME DMA Linked List Error   DLLER. This bit is set when a ME error occurred during a DMALL  ACCLL  SAFLL or        CONLL operation when a new TCS is loaded from anywhere in memory to        overwrite the current TCS stored in the DMARAM."]
    #[inline(always)]
    pub fn dller(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, errsr1::Dller, Errsr1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<26,0x1,1,0,errsr1::Dller, Errsr1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Errsr1 {
    #[inline(always)]
    fn default() -> Errsr1 {
        <crate::RegValueT<Errsr1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod errsr1 {
    pub struct Ser_SPEC;
    pub type Ser = crate::EnumBitfieldStruct<u8, Ser_SPEC>;
    impl Ser {
        #[doc = "0 No ME source        error has occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ME source error        has occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Der_SPEC;
    pub type Der = crate::EnumBitfieldStruct<u8, Der_SPEC>;
    impl Der {
        #[doc = "0 No ME        destination error has occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ME destination        error has occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Spber_SPEC;
    pub type Spber = crate::EnumBitfieldStruct<u8, Spber_SPEC>;
    impl Spber {
        #[doc = "0 No error        occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An error        occurred on SPB  160 Bus interface."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Srier_SPEC;
    pub type Srier = crate::EnumBitfieldStruct<u8, Srier_SPEC>;
    impl Srier {
        #[doc = "0 No error        occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An error        occurred on SRI  160 Bus interface."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ramer_SPEC;
    pub type Ramer = crate::EnumBitfieldStruct<u8, Ramer_SPEC>;
    impl Ramer {
        #[doc = "0 No error        occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An error        occurred during the load of a TCS."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sller_SPEC;
    pub type Sller = crate::EnumBitfieldStruct<u8, Sller_SPEC>;
    impl Sller {
        #[doc = "0 No error        occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An error        occurred during the SDCRC checksum comparison."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Dller_SPEC;
    pub type Dller = crate::EnumBitfieldStruct<u8, Dller_SPEC>;
    impl Dller {
        #[doc = "0 No error        occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An error        occurred during the loading of a new TCS."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HrRc_SPEC;
impl crate::sealed::RegSpec for HrRc_SPEC {
    type DataType = u32;
}
#[doc = "DMA Channel 000 Resource Partition Register\n resetvalue={Application Reset:0x0}"]
pub type HrRc = crate::RegValueT<HrRc_SPEC>;

impl HrRc {
    #[doc = "DMA Channel Resource Partition   HRP"]
    #[inline(always)]
    pub fn hrp(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, hrrc::Hrp, HrRc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,hrrc::Hrp, HrRc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for HrRc {
    #[inline(always)]
    fn default() -> HrRc {
        <crate::RegValueT<HrRc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod hrrc {
    pub struct Hrp_SPEC;
    pub type Hrp = crate::EnumBitfieldStruct<u8, Hrp_SPEC>;
    impl Hrp {
        #[doc = "00 Resource        Partition 0  RP0 ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Resource        Partition 1  RP1 ."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Resource        Partition 2  RP2 ."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Resource        Partition 3  RP3 ."]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "DMA Identification Register\n resetvalue={Application Reset:0x087C000}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MOD REV. This bit field defines the module revision number. See DMA Design Specification for MOD REV value."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MOD TYPE. The bit field is set to C0 H which        defines the module as a 32 bit module."]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MOD NUMBER. This bit field defines a module identification number."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(8896512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me00R_SPEC;
impl crate::sealed::RegSpec for Me00R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 0\n resetvalue={Application Reset:0x0}"]
pub type Me00R = crate::RegValueT<Me00R_SPEC>;

impl Me00R {
    #[doc = "DMA Read Move Data Byte   RD00"]
    #[inline(always)]
    pub fn rd00(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me00R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me00R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD01"]
    #[inline(always)]
    pub fn rd01(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me00R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me00R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD02"]
    #[inline(always)]
    pub fn rd02(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me00R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me00R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD03"]
    #[inline(always)]
    pub fn rd03(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me00R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me00R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me00R {
    #[inline(always)]
    fn default() -> Me00R {
        <crate::RegValueT<Me00R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me01R_SPEC;
impl crate::sealed::RegSpec for Me01R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 1\n resetvalue={Application Reset:0x0}"]
pub type Me01R = crate::RegValueT<Me01R_SPEC>;

impl Me01R {
    #[doc = "DMA Read Move Data Byte   RD10"]
    #[inline(always)]
    pub fn rd10(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me01R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me01R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD11"]
    #[inline(always)]
    pub fn rd11(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me01R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me01R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD12"]
    #[inline(always)]
    pub fn rd12(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me01R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me01R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD13"]
    #[inline(always)]
    pub fn rd13(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me01R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me01R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me01R {
    #[inline(always)]
    fn default() -> Me01R {
        <crate::RegValueT<Me01R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me02R_SPEC;
impl crate::sealed::RegSpec for Me02R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 2\n resetvalue={Application Reset:0x0}"]
pub type Me02R = crate::RegValueT<Me02R_SPEC>;

impl Me02R {
    #[doc = "DMA Read Move Data Byte   RD20"]
    #[inline(always)]
    pub fn rd20(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me02R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me02R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD21"]
    #[inline(always)]
    pub fn rd21(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me02R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me02R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD22"]
    #[inline(always)]
    pub fn rd22(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me02R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me02R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD23"]
    #[inline(always)]
    pub fn rd23(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me02R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me02R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me02R {
    #[inline(always)]
    fn default() -> Me02R {
        <crate::RegValueT<Me02R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me03R_SPEC;
impl crate::sealed::RegSpec for Me03R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 3\n resetvalue={Application Reset:0x0}"]
pub type Me03R = crate::RegValueT<Me03R_SPEC>;

impl Me03R {
    #[doc = "DMA Read Move Data Byte   RD30"]
    #[inline(always)]
    pub fn rd30(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me03R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me03R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD31"]
    #[inline(always)]
    pub fn rd31(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me03R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me03R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD32"]
    #[inline(always)]
    pub fn rd32(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me03R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me03R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD33"]
    #[inline(always)]
    pub fn rd33(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me03R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me03R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me03R {
    #[inline(always)]
    fn default() -> Me03R {
        <crate::RegValueT<Me03R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me04R_SPEC;
impl crate::sealed::RegSpec for Me04R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 4\n resetvalue={Application Reset:0x0}"]
pub type Me04R = crate::RegValueT<Me04R_SPEC>;

impl Me04R {
    #[doc = "DMA Read Move Data Byte   RD40"]
    #[inline(always)]
    pub fn rd40(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me04R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me04R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD41"]
    #[inline(always)]
    pub fn rd41(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me04R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me04R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD42"]
    #[inline(always)]
    pub fn rd42(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me04R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me04R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD43"]
    #[inline(always)]
    pub fn rd43(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me04R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me04R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me04R {
    #[inline(always)]
    fn default() -> Me04R {
        <crate::RegValueT<Me04R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me05R_SPEC;
impl crate::sealed::RegSpec for Me05R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 5\n resetvalue={Application Reset:0x0}"]
pub type Me05R = crate::RegValueT<Me05R_SPEC>;

impl Me05R {
    #[doc = "DMA Read Move Data Byte   RD50"]
    #[inline(always)]
    pub fn rd50(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me05R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me05R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD51"]
    #[inline(always)]
    pub fn rd51(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me05R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me05R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD52"]
    #[inline(always)]
    pub fn rd52(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me05R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me05R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD53"]
    #[inline(always)]
    pub fn rd53(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me05R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me05R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me05R {
    #[inline(always)]
    fn default() -> Me05R {
        <crate::RegValueT<Me05R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me06R_SPEC;
impl crate::sealed::RegSpec for Me06R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 6\n resetvalue={Application Reset:0x0}"]
pub type Me06R = crate::RegValueT<Me06R_SPEC>;

impl Me06R {
    #[doc = "DMA Read Move Data Byte   RD60"]
    #[inline(always)]
    pub fn rd60(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me06R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me06R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD61"]
    #[inline(always)]
    pub fn rd61(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me06R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me06R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD62"]
    #[inline(always)]
    pub fn rd62(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me06R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me06R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD63"]
    #[inline(always)]
    pub fn rd63(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me06R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me06R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me06R {
    #[inline(always)]
    fn default() -> Me06R {
        <crate::RegValueT<Me06R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me07R_SPEC;
impl crate::sealed::RegSpec for Me07R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 7\n resetvalue={Application Reset:0x0}"]
pub type Me07R = crate::RegValueT<Me07R_SPEC>;

impl Me07R {
    #[doc = "DMA Read Move Data Byte   RD70"]
    #[inline(always)]
    pub fn rd70(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me07R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me07R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD71"]
    #[inline(always)]
    pub fn rd71(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me07R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me07R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD72"]
    #[inline(always)]
    pub fn rd72(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me07R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me07R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD73"]
    #[inline(always)]
    pub fn rd73(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me07R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me07R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me07R {
    #[inline(always)]
    fn default() -> Me07R {
        <crate::RegValueT<Me07R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me0Adicr_SPEC;
impl crate::sealed::RegSpec for Me0Adicr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Address and Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
pub type Me0Adicr = crate::RegValueT<Me0Adicr_SPEC>;

impl Me0Adicr {
    #[doc = "Source Address Modification Factor   SMF. Active DMA channel source address modification factor."]
    #[inline(always)]
    pub fn smf(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Me0Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Increment of Source Address   INCS. Active DMA channel increment of source address control."]
    #[inline(always)]
    pub fn incs(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Me0Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Destination Address Modification Factor   DMF. Active DMA channel destination address modification factor."]
    #[inline(always)]
    pub fn dmf(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x7,1,0,u8, Me0Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Increment of Destination Address   INCD. Active DMA channel increment of destination address control."]
    #[inline(always)]
    pub fn incd(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Me0Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Circular Buffer Length Source   CBLS. Active DMA channel circular source buffer control."]
    #[inline(always)]
    pub fn cbls(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Me0Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Circular Buffer Length Destination   CBLD. Active DMA channel circular destination buffer control."]
    #[inline(always)]
    pub fn cbld(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Me0Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Shadow Control   SHCT. Active DMA channel control of shadow address register function."]
    #[inline(always)]
    pub fn shct(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Me0Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Source Circular Buffer Enable   SCBE. Active DMA channel circular source buffer enable disable."]
    #[inline(always)]
    pub fn scbe(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Me0Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Destination Circular Buffer Enable   DCBE. Active DMA channel circular destination buffer enable disable."]
    #[inline(always)]
    pub fn dcbe(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Me0Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Time Stamp   STAMP. Active DMA channel control to enable the appendage of a timestamp after        the end of the last DMA Move during a DMA transaction."]
    #[inline(always)]
    pub fn stamp(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Me0Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wrap Source Enable   WRPSE. Active DMA channel source buffer interrupt trigger enable disable."]
    #[inline(always)]
    pub fn wrpse(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Me0Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wrap Destination Enable   WRPDE. Active DMA channel destination buffer interrupt trigger enable disable."]
    #[inline(always)]
    pub fn wrpde(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Me0Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Control   INTCT. Active DMA channel interrupt service request control."]
    #[inline(always)]
    pub fn intct(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<26,0x3,1,0,u8, Me0Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Interrupt Raise Detect Value   IRDV. Active DMA channel control of the Threshold Limit of of DMA channel        CHSR.TCOUNT for triggering a channel interrupt service request."]
    #[inline(always)]
    pub fn irdv(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Me0Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Me0Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me0Adicr {
    #[inline(always)]
    fn default() -> Me0Adicr {
        <crate::RegValueT<Me0Adicr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me0Chcr_SPEC;
impl crate::sealed::RegSpec for Me0Chcr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Control Register\n resetvalue={Application Reset:0x0}"]
pub type Me0Chcr = crate::RegValueT<Me0Chcr_SPEC>;

impl Me0Chcr {
    #[doc = "Transfer Reload Value   TREL. Active DMA channel Transfer Reload Value."]
    #[inline(always)]
    pub fn trel(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Me0Chcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Me0Chcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Block Mode   BLKM. Active DMA channel Block Mode."]
    #[inline(always)]
    pub fn blkm(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Me0Chcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Me0Chcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Request Only After Transaction   RROAT. Active DMA channel Reset Request Only After Transaction."]
    #[inline(always)]
    pub fn rroat(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Me0Chcr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Me0Chcr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel Operation Mode   CHMODE. Active DMA channel Channel Operation Mode."]
    #[inline(always)]
    pub fn chmode(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Me0Chcr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Me0Chcr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel Data Width   CHDW. Active DMA channel DMA move Channel Data Width."]
    #[inline(always)]
    pub fn chdw(
        self,
    ) -> crate::common::RegisterField<21, 0x7, 1, 0, u8, Me0Chcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<21,0x7,1,0,u8, Me0Chcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Pattern Select   PATSEL. Active DMA channel Pattern Select control."]
    #[inline(always)]
    pub fn patsel(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Me0Chcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Me0Chcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Swap Data CRC byte order   SWAP. Active DMA channel swap data CRC byte order."]
    #[inline(always)]
    pub fn swap(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Me0Chcr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Me0Chcr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Peripheral Request Select   PRSEL. Active DMA channel Peripheral Request Select."]
    #[inline(always)]
    pub fn prsel(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Me0Chcr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Me0Chcr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Me0Chcr {
    #[inline(always)]
    fn default() -> Me0Chcr {
        <crate::RegValueT<Me0Chcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me0Chsr_SPEC;
impl crate::sealed::RegSpec for Me0Chsr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Status Register\n resetvalue={Application Reset:0x0}"]
pub type Me0Chsr = crate::RegValueT<Me0Chsr_SPEC>;

impl Me0Chsr {
    #[doc = "Transfer Count Status   TCOUNT. Active DMA channel count of the number of DMA transfers. TCOUNT is        loaded with the DMA channel value of CHCFGR.TREL when TSR.CH becomes set         and TCOUNT  160    160 0 . After each DMA transfer  TCOUNT is decremented by 1."]
    #[inline(always)]
    pub fn tcount(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Me0Chsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Me0Chsr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Old Value of Pattern Detection   LXO. Active DMA channel compare result of a pattern compare operation when        8 bit or 16 bit data width is selected."]
    #[inline(always)]
    pub fn lxo(self) -> crate::common::RegisterFieldBool<15, 1, 0, Me0Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Me0Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wrap Source Buffer   WRPS. Active DMA channel Wrap Source Buffer status bit."]
    #[inline(always)]
    pub fn wrps(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Me0Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Me0Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wrap Destination Buffer   WRPD. Active DMA channel Wrap Destination Buffer status bit."]
    #[inline(always)]
    pub fn wrpd(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Me0Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Me0Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt from Channel   ICH. Active DMA channel detection of channel interrupt service request."]
    #[inline(always)]
    pub fn ich(self) -> crate::common::RegisterFieldBool<18, 1, 0, Me0Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Me0Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pattern Detection from Channel   IPM. Active DMA channel detection of pattern match interrupt service request."]
    #[inline(always)]
    pub fn ipm(self) -> crate::common::RegisterFieldBool<19, 1, 0, Me0Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Me0Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "DMA Double Buffering Active Buffer   BUFFER. Active DMA channel DMA Double Buffering Active Buffer status bit."]
    #[inline(always)]
    pub fn buffer(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, me0chsr::Buffer, Me0Chsr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<22,0x1,1,0,me0chsr::Buffer, Me0Chsr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Double Buffering Frozen Buffer   FROZEN. Active DMA channel DMA Double Buffering Frozen Buffer status bit."]
    #[inline(always)]
    pub fn frozen(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Me0Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Me0Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Me0Chsr {
    #[inline(always)]
    fn default() -> Me0Chsr {
        <crate::RegValueT<Me0Chsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod me0chsr {
    pub struct Buffer_SPEC;
    pub type Buffer = crate::EnumBitfieldStruct<u8, Buffer_SPEC>;
    impl Buffer {
        #[doc = "0 Buffer 0 read or filled by DMA."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Buffer 1 read or filled by DMA."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me0Dadr_SPEC;
impl crate::sealed::RegSpec for Me0Dadr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Destination Address Register\n resetvalue={Application Reset:0x0}"]
pub type Me0Dadr = crate::RegValueT<Me0Dadr_SPEC>;

impl Me0Dadr {
    #[doc = "Destination Address   DADR. Active DMA channel 32 bit destination address used for DMA write moves."]
    #[inline(always)]
    pub fn dadr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Me0Dadr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Me0Dadr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me0Dadr {
    #[inline(always)]
    fn default() -> Me0Dadr {
        <crate::RegValueT<Me0Dadr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me0Rdcrc_SPEC;
impl crate::sealed::RegSpec for Me0Rdcrc_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Read Data CRC Register\n resetvalue={Application Reset:0x0}"]
pub type Me0Rdcrc = crate::RegValueT<Me0Rdcrc_SPEC>;

impl Me0Rdcrc {
    #[doc = "Read Data CRC   RDCRC. Active DMA channel read data CRC32 ethernet polynomial checksum."]
    #[inline(always)]
    pub fn rdcrc(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Me0Rdcrc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Me0Rdcrc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me0Rdcrc {
    #[inline(always)]
    fn default() -> Me0Rdcrc {
        <crate::RegValueT<Me0Rdcrc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me0Sadr_SPEC;
impl crate::sealed::RegSpec for Me0Sadr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Source Address Register\n resetvalue={Application Reset:0x0}"]
pub type Me0Sadr = crate::RegValueT<Me0Sadr_SPEC>;

impl Me0Sadr {
    #[doc = "Source Address   SADR. Active DMA channel 32 bit source address used for DMA read moves."]
    #[inline(always)]
    pub fn sadr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Me0Sadr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Me0Sadr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me0Sadr {
    #[inline(always)]
    fn default() -> Me0Sadr {
        <crate::RegValueT<Me0Sadr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me0Sdcrc_SPEC;
impl crate::sealed::RegSpec for Me0Sdcrc_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Source and Destination Address CRC Register\n resetvalue={Application Reset:0x0}"]
pub type Me0Sdcrc = crate::RegValueT<Me0Sdcrc_SPEC>;

impl Me0Sdcrc {
    #[doc = "Source and Destination Address CRC   SDCRC. Active DMA channel source and destination address CRC32 ethernet polynomial checksum."]
    #[inline(always)]
    pub fn sdcrc(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Me0Sdcrc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Me0Sdcrc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me0Sdcrc {
    #[inline(always)]
    fn default() -> Me0Sdcrc {
        <crate::RegValueT<Me0Sdcrc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me0Shadr_SPEC;
impl crate::sealed::RegSpec for Me0Shadr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Shadow Address Register\n resetvalue={Application Reset:0x0}"]
pub type Me0Shadr = crate::RegValueT<Me0Shadr_SPEC>;

impl Me0Shadr {
    #[doc = "Shadowed Address   SHADR. This bit field holds the 32 bit shadow address of the active DMA        channel. The function of the shadow address is set by the shadow control        settings."]
    #[inline(always)]
    pub fn shadr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Me0Shadr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Me0Shadr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me0Shadr {
    #[inline(always)]
    fn default() -> Me0Shadr {
        <crate::RegValueT<Me0Shadr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me0Sr_SPEC;
impl crate::sealed::RegSpec for Me0Sr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Status Register\n resetvalue={Application Reset:0x0}"]
pub type Me0Sr = crate::RegValueT<Me0Sr_SPEC>;

impl Me0Sr {
    #[doc = "ME Read Status   RS"]
    #[inline(always)]
    pub fn rs(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, me0sr::Rs, Me0Sr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,me0sr::Rs, Me0Sr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ME Write Status   WS"]
    #[inline(always)]
    pub fn ws(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, me0sr::Ws, Me0Sr_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x1,1,0,me0sr::Ws, Me0Sr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ME Active Channel   CH. Indicates the number of the DMA Channel currently processed by the ME."]
    #[inline(always)]
    pub fn ch(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Me0Sr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Me0Sr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me0Sr {
    #[inline(always)]
    fn default() -> Me0Sr {
        <crate::RegValueT<Me0Sr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod me0sr {
    pub struct Rs_SPEC;
    pub type Rs = crate::EnumBitfieldStruct<u8, Rs_SPEC>;
    impl Rs {
        #[doc = "0 ME is not        performing a DMA read move."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ME is        performing a DMA read move."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ws_SPEC;
    pub type Ws = crate::EnumBitfieldStruct<u8, Ws_SPEC>;
    impl Ws {
        #[doc = "0 ME is not        performing a DMA write move."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ME is        performing a DMA write move."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me10R_SPEC;
impl crate::sealed::RegSpec for Me10R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 0\n resetvalue={Application Reset:0x0}"]
pub type Me10R = crate::RegValueT<Me10R_SPEC>;

impl Me10R {
    #[doc = "DMA Read Move Data Byte   RD00"]
    #[inline(always)]
    pub fn rd00(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me10R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me10R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD01"]
    #[inline(always)]
    pub fn rd01(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me10R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me10R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD02"]
    #[inline(always)]
    pub fn rd02(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me10R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me10R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD03"]
    #[inline(always)]
    pub fn rd03(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me10R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me10R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me10R {
    #[inline(always)]
    fn default() -> Me10R {
        <crate::RegValueT<Me10R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me11R_SPEC;
impl crate::sealed::RegSpec for Me11R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 1\n resetvalue={Application Reset:0x0}"]
pub type Me11R = crate::RegValueT<Me11R_SPEC>;

impl Me11R {
    #[doc = "DMA Read Move Data Byte   RD10"]
    #[inline(always)]
    pub fn rd10(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me11R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me11R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD11"]
    #[inline(always)]
    pub fn rd11(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me11R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me11R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD12"]
    #[inline(always)]
    pub fn rd12(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me11R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me11R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD13"]
    #[inline(always)]
    pub fn rd13(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me11R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me11R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me11R {
    #[inline(always)]
    fn default() -> Me11R {
        <crate::RegValueT<Me11R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me12R_SPEC;
impl crate::sealed::RegSpec for Me12R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 2\n resetvalue={Application Reset:0x0}"]
pub type Me12R = crate::RegValueT<Me12R_SPEC>;

impl Me12R {
    #[doc = "DMA Read Move Data Byte   RD20"]
    #[inline(always)]
    pub fn rd20(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me12R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me12R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD21"]
    #[inline(always)]
    pub fn rd21(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me12R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me12R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD22"]
    #[inline(always)]
    pub fn rd22(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me12R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me12R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD23"]
    #[inline(always)]
    pub fn rd23(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me12R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me12R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me12R {
    #[inline(always)]
    fn default() -> Me12R {
        <crate::RegValueT<Me12R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me13R_SPEC;
impl crate::sealed::RegSpec for Me13R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 3\n resetvalue={Application Reset:0x0}"]
pub type Me13R = crate::RegValueT<Me13R_SPEC>;

impl Me13R {
    #[doc = "DMA Read Move Data Byte   RD30"]
    #[inline(always)]
    pub fn rd30(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me13R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me13R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD31"]
    #[inline(always)]
    pub fn rd31(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me13R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me13R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD32"]
    #[inline(always)]
    pub fn rd32(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me13R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me13R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD33"]
    #[inline(always)]
    pub fn rd33(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me13R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me13R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me13R {
    #[inline(always)]
    fn default() -> Me13R {
        <crate::RegValueT<Me13R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me14R_SPEC;
impl crate::sealed::RegSpec for Me14R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 4\n resetvalue={Application Reset:0x0}"]
pub type Me14R = crate::RegValueT<Me14R_SPEC>;

impl Me14R {
    #[doc = "DMA Read Move Data Byte   RD40"]
    #[inline(always)]
    pub fn rd40(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me14R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me14R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD41"]
    #[inline(always)]
    pub fn rd41(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me14R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me14R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD42"]
    #[inline(always)]
    pub fn rd42(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me14R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me14R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD43"]
    #[inline(always)]
    pub fn rd43(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me14R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me14R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me14R {
    #[inline(always)]
    fn default() -> Me14R {
        <crate::RegValueT<Me14R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me15R_SPEC;
impl crate::sealed::RegSpec for Me15R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 5\n resetvalue={Application Reset:0x0}"]
pub type Me15R = crate::RegValueT<Me15R_SPEC>;

impl Me15R {
    #[doc = "DMA Read Move Data Byte   RD50"]
    #[inline(always)]
    pub fn rd50(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me15R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me15R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD51"]
    #[inline(always)]
    pub fn rd51(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me15R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me15R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD52"]
    #[inline(always)]
    pub fn rd52(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me15R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me15R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD53"]
    #[inline(always)]
    pub fn rd53(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me15R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me15R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me15R {
    #[inline(always)]
    fn default() -> Me15R {
        <crate::RegValueT<Me15R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me16R_SPEC;
impl crate::sealed::RegSpec for Me16R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 6\n resetvalue={Application Reset:0x0}"]
pub type Me16R = crate::RegValueT<Me16R_SPEC>;

impl Me16R {
    #[doc = "DMA Read Move Data Byte   RD60"]
    #[inline(always)]
    pub fn rd60(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me16R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me16R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD61"]
    #[inline(always)]
    pub fn rd61(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me16R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me16R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD62"]
    #[inline(always)]
    pub fn rd62(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me16R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me16R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD63"]
    #[inline(always)]
    pub fn rd63(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me16R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me16R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me16R {
    #[inline(always)]
    fn default() -> Me16R {
        <crate::RegValueT<Me16R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me17R_SPEC;
impl crate::sealed::RegSpec for Me17R_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Read Register 7\n resetvalue={Application Reset:0x0}"]
pub type Me17R = crate::RegValueT<Me17R_SPEC>;

impl Me17R {
    #[doc = "DMA Read Move Data Byte   RD70"]
    #[inline(always)]
    pub fn rd70(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Me17R_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Me17R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD71"]
    #[inline(always)]
    pub fn rd71(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Me17R_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Me17R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD72"]
    #[inline(always)]
    pub fn rd72(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Me17R_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Me17R_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Read Move Data Byte   RD73"]
    #[inline(always)]
    pub fn rd73(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Me17R_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Me17R_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me17R {
    #[inline(always)]
    fn default() -> Me17R {
        <crate::RegValueT<Me17R_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me1Adicr_SPEC;
impl crate::sealed::RegSpec for Me1Adicr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Address and Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
pub type Me1Adicr = crate::RegValueT<Me1Adicr_SPEC>;

impl Me1Adicr {
    #[doc = "Source Address Modification Factor   SMF. Active DMA channel source address modification factor."]
    #[inline(always)]
    pub fn smf(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Me1Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Increment of Source Address   INCS. Active DMA channel increment of source address control."]
    #[inline(always)]
    pub fn incs(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Me1Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Destination Address Modification Factor   DMF. Active DMA channel destination address modification factor."]
    #[inline(always)]
    pub fn dmf(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x7,1,0,u8, Me1Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Increment of Destination Address   INCD. Active DMA channel increment of destination address control."]
    #[inline(always)]
    pub fn incd(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Me1Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Circular Buffer Length Source   CBLS. Active DMA channel circular source buffer control."]
    #[inline(always)]
    pub fn cbls(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Me1Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Circular Buffer Length Destination   CBLD. Active DMA channel circular destination buffer control."]
    #[inline(always)]
    pub fn cbld(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Me1Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Shadow Control   SHCT. Active DMA channel control of shadow address register function."]
    #[inline(always)]
    pub fn shct(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Me1Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Source Circular Buffer Enable   SCBE. Active DMA channel circular source buffer enable disable."]
    #[inline(always)]
    pub fn scbe(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Me1Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Destination Circular Buffer Enable   DCBE. Active DMA channel circular destination buffer enable disable."]
    #[inline(always)]
    pub fn dcbe(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Me1Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Time Stamp   STAMP. Active DMA channel control to enable the appendage of a timestamp after        the end of the last DMA Move during a DMA transaction."]
    #[inline(always)]
    pub fn stamp(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Me1Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wrap Source Enable   WRPSE. Active DMA channel source buffer interrupt trigger enable disable."]
    #[inline(always)]
    pub fn wrpse(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Me1Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wrap Destination Enable   WRPDE. Active DMA channel destination buffer interrupt trigger enable disable."]
    #[inline(always)]
    pub fn wrpde(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Me1Adicr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Control   INTCT. Active DMA channel interrupt service request control."]
    #[inline(always)]
    pub fn intct(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<26,0x3,1,0,u8, Me1Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Interrupt Raise Detect Value   IRDV. Active DMA channel control of the Threshold Limit of of DMA channel        CHSR.TCOUNT for triggering a channel interrupt service request."]
    #[inline(always)]
    pub fn irdv(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Me1Adicr_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Me1Adicr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me1Adicr {
    #[inline(always)]
    fn default() -> Me1Adicr {
        <crate::RegValueT<Me1Adicr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me1Chcr_SPEC;
impl crate::sealed::RegSpec for Me1Chcr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Control Register\n resetvalue={Application Reset:0x0}"]
pub type Me1Chcr = crate::RegValueT<Me1Chcr_SPEC>;

impl Me1Chcr {
    #[doc = "Transfer Reload Value   TREL. Active DMA channel Transfer Reload Value."]
    #[inline(always)]
    pub fn trel(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Me1Chcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Me1Chcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Block Mode   BLKM. Active DMA channel Block Mode."]
    #[inline(always)]
    pub fn blkm(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Me1Chcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Me1Chcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Request Only After Transaction   RROAT. Active DMA channel Reset Request Only After Transaction."]
    #[inline(always)]
    pub fn rroat(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Me1Chcr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Me1Chcr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel Operation Mode   CHMODE. Active DMA channel Channel Operation Mode."]
    #[inline(always)]
    pub fn chmode(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Me1Chcr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Me1Chcr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel Data Width   CHDW. Active DMA channel DMA move Channel Data Width."]
    #[inline(always)]
    pub fn chdw(
        self,
    ) -> crate::common::RegisterField<21, 0x7, 1, 0, u8, Me1Chcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<21,0x7,1,0,u8, Me1Chcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Pattern Select   PATSEL. Active DMA channel Pattern Select control."]
    #[inline(always)]
    pub fn patsel(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Me1Chcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Me1Chcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Swap Data CRC byte order   SWAP. Active DMA channel swap data CRC byte order."]
    #[inline(always)]
    pub fn swap(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Me1Chcr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Me1Chcr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Peripheral Request Select   PRSEL. Active DMA channel Peripheral Request Select."]
    #[inline(always)]
    pub fn prsel(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Me1Chcr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Me1Chcr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Me1Chcr {
    #[inline(always)]
    fn default() -> Me1Chcr {
        <crate::RegValueT<Me1Chcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me1Chsr_SPEC;
impl crate::sealed::RegSpec for Me1Chsr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Status Register\n resetvalue={Application Reset:0x0}"]
pub type Me1Chsr = crate::RegValueT<Me1Chsr_SPEC>;

impl Me1Chsr {
    #[doc = "Transfer Count Status   TCOUNT. Active DMA channel count of the number of DMA transfers. TCOUNT is        loaded with the DMA channel value of CHCFGR.TREL when TSR.CH becomes set         and TCOUNT  160    160 0 . After each DMA transfer  TCOUNT is decremented by 1."]
    #[inline(always)]
    pub fn tcount(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Me1Chsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Me1Chsr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Old Value of Pattern Detection   LXO. Active DMA channel compare result of a pattern compare operation when        8 bit or 16 bit data width is selected."]
    #[inline(always)]
    pub fn lxo(self) -> crate::common::RegisterFieldBool<15, 1, 0, Me1Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Me1Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wrap Source Buffer   WRPS. Active DMA channel Wrap Source Buffer status bit."]
    #[inline(always)]
    pub fn wrps(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Me1Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Me1Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Wrap Destination Buffer   WRPD. Active DMA channel Wrap Destination Buffer status bit."]
    #[inline(always)]
    pub fn wrpd(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Me1Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Me1Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt from Channel   ICH. Active DMA channel detection of channel interrupt service request."]
    #[inline(always)]
    pub fn ich(self) -> crate::common::RegisterFieldBool<18, 1, 0, Me1Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Me1Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pattern Detection from Channel   IPM. Active DMA channel detection of pattern match interrupt service request."]
    #[inline(always)]
    pub fn ipm(self) -> crate::common::RegisterFieldBool<19, 1, 0, Me1Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Me1Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "DMA Double Buffering Active Buffer   BUFFER. Active DMA channel DMA Double Buffering Active Buffer status bit."]
    #[inline(always)]
    pub fn buffer(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, me1chsr::Buffer, Me1Chsr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<22,0x1,1,0,me1chsr::Buffer, Me1Chsr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Double Buffering Frozen Buffer   FROZEN. Active DMA channel DMA Double Buffering Frozen Buffer status bit."]
    #[inline(always)]
    pub fn frozen(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Me1Chsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Me1Chsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Me1Chsr {
    #[inline(always)]
    fn default() -> Me1Chsr {
        <crate::RegValueT<Me1Chsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod me1chsr {
    pub struct Buffer_SPEC;
    pub type Buffer = crate::EnumBitfieldStruct<u8, Buffer_SPEC>;
    impl Buffer {
        #[doc = "0 Buffer 0 read or filled by DMA."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Buffer 1 read or filled by DMA."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me1Dadr_SPEC;
impl crate::sealed::RegSpec for Me1Dadr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Destination Address Register\n resetvalue={Application Reset:0x0}"]
pub type Me1Dadr = crate::RegValueT<Me1Dadr_SPEC>;

impl Me1Dadr {
    #[doc = "Destination Address   DADR. Active DMA channel 32 bit destination address used for DMA write moves."]
    #[inline(always)]
    pub fn dadr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Me1Dadr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Me1Dadr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me1Dadr {
    #[inline(always)]
    fn default() -> Me1Dadr {
        <crate::RegValueT<Me1Dadr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me1Rdcrc_SPEC;
impl crate::sealed::RegSpec for Me1Rdcrc_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Read Data CRC Register\n resetvalue={Application Reset:0x0}"]
pub type Me1Rdcrc = crate::RegValueT<Me1Rdcrc_SPEC>;

impl Me1Rdcrc {
    #[doc = "Read Data CRC   RDCRC. Active DMA channel read data CRC32 ethernet polynomial checksum."]
    #[inline(always)]
    pub fn rdcrc(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Me1Rdcrc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Me1Rdcrc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me1Rdcrc {
    #[inline(always)]
    fn default() -> Me1Rdcrc {
        <crate::RegValueT<Me1Rdcrc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me1Sadr_SPEC;
impl crate::sealed::RegSpec for Me1Sadr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Source Address Register\n resetvalue={Application Reset:0x0}"]
pub type Me1Sadr = crate::RegValueT<Me1Sadr_SPEC>;

impl Me1Sadr {
    #[doc = "Source Address   SADR. Active DMA channel 32 bit source address used for DMA read moves."]
    #[inline(always)]
    pub fn sadr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Me1Sadr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Me1Sadr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me1Sadr {
    #[inline(always)]
    fn default() -> Me1Sadr {
        <crate::RegValueT<Me1Sadr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me1Sdcrc_SPEC;
impl crate::sealed::RegSpec for Me1Sdcrc_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Source and Destination Address CRC Register\n resetvalue={Application Reset:0x0}"]
pub type Me1Sdcrc = crate::RegValueT<Me1Sdcrc_SPEC>;

impl Me1Sdcrc {
    #[doc = "Source and Destination Address CRC   SDCRC. Active DMA channel source and destination address CRC32 ethernet polynomial checksum."]
    #[inline(always)]
    pub fn sdcrc(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Me1Sdcrc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Me1Sdcrc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me1Sdcrc {
    #[inline(always)]
    fn default() -> Me1Sdcrc {
        <crate::RegValueT<Me1Sdcrc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me1Shadr_SPEC;
impl crate::sealed::RegSpec for Me1Shadr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Channel Shadow Address Register\n resetvalue={Application Reset:0x0}"]
pub type Me1Shadr = crate::RegValueT<Me1Shadr_SPEC>;

impl Me1Shadr {
    #[doc = "Shadowed Address   SHADR. This bit field holds the 32 bit shadow address of the active DMA        channel. The function of the shadow address is set by the shadow control        settings."]
    #[inline(always)]
    pub fn shadr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Me1Shadr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Me1Shadr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me1Shadr {
    #[inline(always)]
    fn default() -> Me1Shadr {
        <crate::RegValueT<Me1Shadr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Me1Sr_SPEC;
impl crate::sealed::RegSpec for Me1Sr_SPEC {
    type DataType = u32;
}
#[doc = "ME 1 Status Register\n resetvalue={Application Reset:0x0}"]
pub type Me1Sr = crate::RegValueT<Me1Sr_SPEC>;

impl Me1Sr {
    #[doc = "ME Read Status   RS"]
    #[inline(always)]
    pub fn rs(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, me1sr::Rs, Me1Sr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,me1sr::Rs, Me1Sr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ME Write Status   WS"]
    #[inline(always)]
    pub fn ws(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, me1sr::Ws, Me1Sr_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x1,1,0,me1sr::Ws, Me1Sr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ME Active Channel   CH. Indicates the number of the DMA Channel currently processed by the ME."]
    #[inline(always)]
    pub fn ch(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Me1Sr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Me1Sr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Me1Sr {
    #[inline(always)]
    fn default() -> Me1Sr {
        <crate::RegValueT<Me1Sr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod me1sr {
    pub struct Rs_SPEC;
    pub type Rs = crate::EnumBitfieldStruct<u8, Rs_SPEC>;
    impl Rs {
        #[doc = "0 ME is not        performing a DMA read move."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ME is        performing a DMA read move."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ws_SPEC;
    pub type Ws = crate::EnumBitfieldStruct<u8, Ws_SPEC>;
    impl Ws {
        #[doc = "0 ME is not        performing a DMA write move."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ME is        performing a DMA write move."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ModEr_SPEC;
impl crate::sealed::RegSpec for ModEr_SPEC {
    type DataType = u32;
}
#[doc = "RP 0 Mode Register\n resetvalue={Application Reset:0x1}"]
pub type ModEr = crate::RegValueT<ModEr_SPEC>;

impl ModEr {
    #[doc = "Resource Partition Supervisor Mode   MODE"]
    #[inline(always)]
    pub fn mode(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, moder::Mode, ModEr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,moder::Mode, ModEr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for ModEr {
    #[inline(always)]
    fn default() -> ModEr {
        <crate::RegValueT<ModEr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod moder {
    pub struct Mode_SPEC;
    pub type Mode = crate::EnumBitfieldStruct<u8, Mode_SPEC>;
    impl Mode {
        #[doc = "0 Bus master        interface accesses on chip bus in user mode."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bus master        interface accesses on chip bus in supervisor mode."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otss_SPEC;
impl crate::sealed::RegSpec for Otss_SPEC {
    type DataType = u32;
}
#[doc = "DMA OCDS Trigger Set Select\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
pub type Otss = crate::RegValueT<Otss_SPEC>;

impl Otss {
    #[doc = "Trigger Set for OTGB0 or OTGB1   TGS"]
    #[inline(always)]
    pub fn tgs(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, otss::Tgs, Otss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,otss::Tgs, Otss_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OTGB0 or OTGB1 Bus Select   BS"]
    #[inline(always)]
    pub fn bs(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, otss::Bs, Otss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,otss::Bs, Otss_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Otss {
    #[inline(always)]
    fn default() -> Otss {
        <crate::RegValueT<Otss_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod otss {
    pub struct Tgs_SPEC;
    pub type Tgs = crate::EnumBitfieldStruct<u8, Tgs_SPEC>;
    impl Tgs {
        #[doc = "0 No Trigger Set        selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Trigger Set 1"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Trigger Set 2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "8 Trigger Set 8"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "9 Trigger Set 9"]
        pub const CONST_99: Self = Self::new(9);
        #[doc = "10 Trigger Set 10"]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "11 Trigger Set 11"]
        pub const CONST_1111: Self = Self::new(11);
        #[doc = "12 Trigger Set 12"]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "13 Trigger Set 13"]
        pub const CONST_1313: Self = Self::new(13);
        #[doc = "14 Trigger Set 14"]
        pub const CONST_1414: Self = Self::new(14);
        #[doc = "15 Trigger Set 15"]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Bs_SPEC;
    pub type Bs = crate::EnumBitfieldStruct<u8, Bs_SPEC>;
    impl Bs {
        #[doc = "0 Trigger Set is        output on OTGB0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Trigger Set is        output on OTGB1"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prr0_SPEC;
impl crate::sealed::RegSpec for Prr0_SPEC {
    type DataType = u32;
}
#[doc = "DMA Pattern Read Register 0\n resetvalue={Application Reset:0x0}"]
pub type Prr0 = crate::RegValueT<Prr0_SPEC>;

impl Prr0 {
    #[doc = "Pattern Data Byte   PAT00"]
    #[inline(always)]
    pub fn pat00(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Prr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Prr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pattern Data Byte   PAT01"]
    #[inline(always)]
    pub fn pat01(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Prr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Prr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pattern Data Byte   PAT02"]
    #[inline(always)]
    pub fn pat02(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Prr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Prr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pattern Data Byte   PAT03"]
    #[inline(always)]
    pub fn pat03(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Prr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Prr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Prr0 {
    #[inline(always)]
    fn default() -> Prr0 {
        <crate::RegValueT<Prr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prr1_SPEC;
impl crate::sealed::RegSpec for Prr1_SPEC {
    type DataType = u32;
}
#[doc = "DMA Pattern Read Register 1\n resetvalue={Application Reset:0x0}"]
pub type Prr1 = crate::RegValueT<Prr1_SPEC>;

impl Prr1 {
    #[doc = "Pattern Data Byte   PAT10"]
    #[inline(always)]
    pub fn pat10(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Prr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Prr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pattern Data Byte   PAT11"]
    #[inline(always)]
    pub fn pat11(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Prr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Prr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pattern Data Byte   PAT12"]
    #[inline(always)]
    pub fn pat12(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Prr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Prr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pattern Data Byte   PAT13"]
    #[inline(always)]
    pub fn pat13(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Prr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Prr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Prr1 {
    #[inline(always)]
    fn default() -> Prr1 {
        <crate::RegValueT<Prr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SusacRc_SPEC;
impl crate::sealed::RegSpec for SusacRc_SPEC {
    type DataType = u32;
}
#[doc = "DMA Channel 000 Suspend Acknowledge Register\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
pub type SusacRc = crate::RegValueT<SusacRc_SPEC>;

impl SusacRc {
    #[doc = "DMA Channel Suspend State or Frozen State Active for DMA Channel   SUSAC. Status bit indicates whether or not a DMA channel is in channel suspend        state or in the frozen state."]
    #[inline(always)]
    pub fn susac(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, susacrc::Susac, SusacRc_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,susacrc::Susac, SusacRc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for SusacRc {
    #[inline(always)]
    fn default() -> SusacRc {
        <crate::RegValueT<SusacRc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod susacrc {
    pub struct Susac_SPEC;
    pub type Susac = crate::EnumBitfieldStruct<u8, Susac_SPEC>;
    impl Susac {
        #[doc = "0 DMA channel is        not in channel suspend state  frozen state or internal actions are not        completed after the channel suspend state or frozen state was requested."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 DMA channel is        in channel suspend state or frozen state."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SusenRc_SPEC;
impl crate::sealed::RegSpec for SusenRc_SPEC {
    type DataType = u32;
}
#[doc = "DMA Channel 000 Suspend Enable Register\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
pub type SusenRc = crate::RegValueT<SusenRc_SPEC>;

impl SusenRc {
    #[doc = "Channel Suspend Enable for DMA Channel   SUSEN. Enables the DMA channel suspend capability. Channel suspend mode shall        be terminated when SUSEN is written with 0."]
    #[inline(always)]
    pub fn susen(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, susenrc::Susen, SusenRc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,susenrc::Susen, SusenRc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for SusenRc {
    #[inline(always)]
    fn default() -> SusenRc {
        <crate::RegValueT<SusenRc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod susenrc {
    pub struct Susen_SPEC;
    pub type Susen = crate::EnumBitfieldStruct<u8, Susen_SPEC>;
    impl Susen {
        #[doc = "0 DMA channel is        disabled for DMA channel suspend. The DMA channel does not react on an        active suspend request signal SUSREQ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 DMA channel is        enabled for DMA channel suspend. If the suspend request signal SUSREQ        becomes active  a DMA transaction of the DMA channel is stopped after        the current DMA transfer has completed."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Time_SPEC;
impl crate::sealed::RegSpec for Time_SPEC {
    type DataType = u32;
}
#[doc = "DMA Time Register\n resetvalue={Application Reset:0x0}"]
pub type Time = crate::RegValueT<Time_SPEC>;

impl Time {
    #[doc = "Timestamp Count   COUNT. The count value used during the appendage of DMA timestamps."]
    #[inline(always)]
    pub fn count(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Time_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Time_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Time {
    #[inline(always)]
    fn default() -> Time {
        <crate::RegValueT<Time_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TsRc_SPEC;
impl crate::sealed::RegSpec for TsRc_SPEC {
    type DataType = u32;
}
#[doc = "DMA Channel 000 Transaction State Register\n resetvalue={Application Reset:0x0}"]
pub type TsRc = crate::RegValueT<TsRc_SPEC>;

impl TsRc {
    #[doc = "DMA Channel Reset   RST. The DMA channel reset bit is set by software  DMA channel TSR.RST   1         and cleared by hardware when the DMA channel has been reset."]
    #[inline(always)]
    pub fn rst(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, tsrc::Rst, TsRc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,tsrc::Rst, TsRc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DMA Channel Hardware Request Enable   HTRE    . See Functional Description for enable and disable. When a DMA channel          is configured for single mode  HTRE is reset when ME CHSR.TCOUNT is          decremented and ME CHSR.TCOUNT   0. When a DMA channel error is          reported or a pattern match is detected  DMA channel HTRE is reset."]
    #[inline(always)]
    pub fn htre(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, tsrc::Htre, TsRc_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,tsrc::Htre, TsRc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Channel Transaction Transfer Request Lost   TRL. This bit is reset by software clearing TRL  writing DMA channel TSR.CTL          1  or resetting the DMA channel  writing DMA channel TSR.RST  160    160 1 ."]
    #[inline(always)]
    pub fn trl(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, tsrc::Trl, TsRc_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,tsrc::Trl, TsRc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DMA Channel Transaction Request State   CH. CH is reset when a pattern match is detected."]
    #[inline(always)]
    pub fn ch(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, tsrc::Ch, TsRc_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1,1,0,tsrc::Ch, TsRc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Enable DMA Channel Transaction Transfer Request Lost Interrupt   ETRL. DMA channel control bit to enable the generation of an error interrupt        service request when DMA channel TSR.TRL is set."]
    #[inline(always)]
    pub fn etrl(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, tsrc::Etrl, TsRc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,tsrc::Etrl, TsRc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DMA Channel Halt Request   HLTREQ. The DMA channel halt request bit is set by software  writing DMA channel        TSR.HLTREQ   1  and cleared by software  writing DMA channel TSR.HLTCLR          1 ."]
    #[inline(always)]
    pub fn hltreq(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, tsrc::Hltreq, TsRc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,tsrc::Hltreq, TsRc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DMA Channel Halt Acknowledge   HLTACK"]
    #[inline(always)]
    pub fn hltack(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, tsrc::Hltack, TsRc_SPEC, crate::common::R> {
        crate::common::RegisterField::<9,0x1,1,0,tsrc::Hltack, TsRc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Enable DMA Channel Hardware Transaction Request   ECH. See Functional Description. Reading this bit returns a 0."]
    #[inline(always)]
    pub fn ech(self) -> crate::common::RegisterFieldBool<16, 1, 0, TsRc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, TsRc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable DMA Channel Hardware Transaction Request   DCH. See Functional Description. Reading this bit returns a 0."]
    #[inline(always)]
    pub fn dch(self) -> crate::common::RegisterFieldBool<17, 1, 0, TsRc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, TsRc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear DMA Channel Transaction Transfer Request Lost   CTL. Software clear of the DMA channel TRL status flag. Reading this bit returns a 0."]
    #[inline(always)]
    pub fn ctl(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, tsrc::Ctl, TsRc_SPEC, crate::common::W> {
        crate::common::RegisterField::<18,0x1,1,0,tsrc::Ctl, TsRc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear DMA Channel Halt Request and Acknowledge   HLTCLR. Reading this bit returns a 0."]
    #[inline(always)]
    pub fn hltclr(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, tsrc::Hltclr, TsRc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<24,0x1,1,0,tsrc::Hltclr, TsRc_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for TsRc {
    #[inline(always)]
    fn default() -> TsRc {
        <crate::RegValueT<TsRc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tsrc {
    pub struct Rst_SPEC;
    pub type Rst = crate::EnumBitfieldStruct<u8, Rst_SPEC>;
    impl Rst {
        #[doc = "0 After the        application of a DMA channel reset  the DMA channel is in the DMA        channel reset state. Software write to 0 has no effect."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A DMA channel        reset is pending. See Functional Description."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Htre_SPEC;
    pub type Htre = crate::EnumBitfieldStruct<u8, Htre_SPEC>;
    impl Htre {
        #[doc = "0 DMA hardware        request is disabled for DMA channel."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 DMA hardware        request is enabled for DMA channel."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trl_SPEC;
    pub type Trl = crate::EnumBitfieldStruct<u8, Trl_SPEC>;
    impl Trl {
        #[doc = "0 No TRL event        has been detected for DMA channel."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 TRL event has        been detected for DMA channel."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ch_SPEC;
    pub type Ch = crate::EnumBitfieldStruct<u8, Ch_SPEC>;
    impl Ch {
        #[doc = "0 No DMA request        is pending for DMA channel."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 DMA request is        pending for DMA channel."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etrl_SPEC;
    pub type Etrl = crate::EnumBitfieldStruct<u8, Etrl_SPEC>;
    impl Etrl {
        #[doc = "0 Interrupt        generation for DMA channel TRL event is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt        generation for DMA channel TRL event is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hltreq_SPEC;
    pub type Hltreq = crate::EnumBitfieldStruct<u8, Hltreq_SPEC>;
    impl Hltreq {
        #[doc = "0 No action."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Halt request."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hltack_SPEC;
    pub type Hltack = crate::EnumBitfieldStruct<u8, Hltack_SPEC>;
    impl Hltack {
        #[doc = "0 DMA channel is        not halted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 DMA channel is        halted."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ctl_SPEC;
    pub type Ctl = crate::EnumBitfieldStruct<u8, Ctl_SPEC>;
    impl Ctl {
        #[doc = "0 No action."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear DMA        channel TRL flag  TSR.TRL ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hltclr_SPEC;
    pub type Hltclr = crate::EnumBitfieldStruct<u8, Hltclr_SPEC>;
    impl Hltclr {
        #[doc = "0 No action."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear DMA        channel halt request  TSR.HLTREQ  and halt acknowledge  TSR.HLTACK ."]
        pub const CONST_11: Self = Self::new(1);
    }
}

#[doc = "ACCEN"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accen(pub(super) *mut u8);
unsafe impl core::marker::Send for Accen {}
unsafe impl core::marker::Sync for Accen {}
impl Accen {
    #[doc = "RP 0 Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accenr0(&self) -> crate::common::Reg<accen::AcceNr0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod accen {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AcceNr0_SPEC;
    impl crate::sealed::RegSpec for AcceNr0_SPEC {
        type DataType = u32;
    }
    #[doc = "RP 0 Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type AcceNr0 = crate::RegValueT<AcceNr0_SPEC>;

    impl AcceNr0 {
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en0(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, accenr0::En0, AcceNr0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                accenr0::En0,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en1(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, accenr0::En1, AcceNr0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                accenr0::En1,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en2(
            self,
        ) -> crate::common::RegisterField<2, 0x1, 1, 0, accenr0::En2, AcceNr0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                accenr0::En2,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en3(
            self,
        ) -> crate::common::RegisterField<3, 0x1, 1, 0, accenr0::En3, AcceNr0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                accenr0::En3,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en4(
            self,
        ) -> crate::common::RegisterField<4, 0x1, 1, 0, accenr0::En4, AcceNr0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                accenr0::En4,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en5(
            self,
        ) -> crate::common::RegisterField<5, 0x1, 1, 0, accenr0::En5, AcceNr0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                accenr0::En5,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en6(
            self,
        ) -> crate::common::RegisterField<6, 0x1, 1, 0, accenr0::En6, AcceNr0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                6,
                0x1,
                1,
                0,
                accenr0::En6,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en7(
            self,
        ) -> crate::common::RegisterField<7, 0x1, 1, 0, accenr0::En7, AcceNr0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                accenr0::En7,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en8(
            self,
        ) -> crate::common::RegisterField<8, 0x1, 1, 0, accenr0::En8, AcceNr0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                accenr0::En8,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en9(
            self,
        ) -> crate::common::RegisterField<9, 0x1, 1, 0, accenr0::En9, AcceNr0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                9,
                0x1,
                1,
                0,
                accenr0::En9,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en10(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            accenr0::En10,
            AcceNr0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                accenr0::En10,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en11(
            self,
        ) -> crate::common::RegisterField<
            11,
            0x1,
            1,
            0,
            accenr0::En11,
            AcceNr0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                accenr0::En11,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en12(
            self,
        ) -> crate::common::RegisterField<
            12,
            0x1,
            1,
            0,
            accenr0::En12,
            AcceNr0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                12,
                0x1,
                1,
                0,
                accenr0::En12,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en13(
            self,
        ) -> crate::common::RegisterField<
            13,
            0x1,
            1,
            0,
            accenr0::En13,
            AcceNr0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                13,
                0x1,
                1,
                0,
                accenr0::En13,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en14(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x1,
            1,
            0,
            accenr0::En14,
            AcceNr0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                accenr0::En14,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en15(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            accenr0::En15,
            AcceNr0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                accenr0::En15,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en16(
            self,
        ) -> crate::common::RegisterField<
            16,
            0x1,
            1,
            0,
            accenr0::En16,
            AcceNr0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0x1,
                1,
                0,
                accenr0::En16,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en17(
            self,
        ) -> crate::common::RegisterField<
            17,
            0x1,
            1,
            0,
            accenr0::En17,
            AcceNr0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                17,
                0x1,
                1,
                0,
                accenr0::En17,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en18(
            self,
        ) -> crate::common::RegisterField<
            18,
            0x1,
            1,
            0,
            accenr0::En18,
            AcceNr0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                18,
                0x1,
                1,
                0,
                accenr0::En18,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en19(
            self,
        ) -> crate::common::RegisterField<
            19,
            0x1,
            1,
            0,
            accenr0::En19,
            AcceNr0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                19,
                0x1,
                1,
                0,
                accenr0::En19,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en20(
            self,
        ) -> crate::common::RegisterField<
            20,
            0x1,
            1,
            0,
            accenr0::En20,
            AcceNr0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0x1,
                1,
                0,
                accenr0::En20,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en21(
            self,
        ) -> crate::common::RegisterField<
            21,
            0x1,
            1,
            0,
            accenr0::En21,
            AcceNr0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                21,
                0x1,
                1,
                0,
                accenr0::En21,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en22(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x1,
            1,
            0,
            accenr0::En22,
            AcceNr0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                22,
                0x1,
                1,
                0,
                accenr0::En22,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en23(
            self,
        ) -> crate::common::RegisterField<
            23,
            0x1,
            1,
            0,
            accenr0::En23,
            AcceNr0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                23,
                0x1,
                1,
                0,
                accenr0::En23,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en24(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x1,
            1,
            0,
            accenr0::En24,
            AcceNr0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0x1,
                1,
                0,
                accenr0::En24,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en25(
            self,
        ) -> crate::common::RegisterField<
            25,
            0x1,
            1,
            0,
            accenr0::En25,
            AcceNr0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                25,
                0x1,
                1,
                0,
                accenr0::En25,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en26(
            self,
        ) -> crate::common::RegisterField<
            26,
            0x1,
            1,
            0,
            accenr0::En26,
            AcceNr0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                26,
                0x1,
                1,
                0,
                accenr0::En26,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en27(
            self,
        ) -> crate::common::RegisterField<
            27,
            0x1,
            1,
            0,
            accenr0::En27,
            AcceNr0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                27,
                0x1,
                1,
                0,
                accenr0::En27,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en28(
            self,
        ) -> crate::common::RegisterField<
            28,
            0x1,
            1,
            0,
            accenr0::En28,
            AcceNr0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                28,
                0x1,
                1,
                0,
                accenr0::En28,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en29(
            self,
        ) -> crate::common::RegisterField<
            29,
            0x1,
            1,
            0,
            accenr0::En29,
            AcceNr0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                29,
                0x1,
                1,
                0,
                accenr0::En29,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en30(
            self,
        ) -> crate::common::RegisterField<
            30,
            0x1,
            1,
            0,
            accenr0::En30,
            AcceNr0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                30,
                0x1,
                1,
                0,
                accenr0::En30,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
        #[inline(always)]
        pub fn en31(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            accenr0::En31,
            AcceNr0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                accenr0::En31,
                AcceNr0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for AcceNr0 {
        #[inline(always)]
        fn default() -> AcceNr0 {
            <crate::RegValueT<AcceNr0_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
    pub mod accenr0 {
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
}
#[doc = "CH"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch(pub(super) *mut u8);
unsafe impl core::marker::Send for Ch {}
unsafe impl core::marker::Sync for Ch {}
impl Ch {
    #[doc = "DMARAM Channel 000 Address and Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn adicrc(&self) -> crate::common::Reg<ch::AdicRc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "DMARAM Channel 000 Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn chcfgrc(&self) -> crate::common::Reg<ch::ChcfgRc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "DMARAM Channel 000 Control and Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn chcsrc(&self) -> crate::common::Reg<ch::ChcsRc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "DMARAM Channel 000 Destination Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dadrc(&self) -> crate::common::Reg<ch::DadRc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "DMARAM Channel 000 Read Data CRC Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rdcrcrc(&self) -> crate::common::Reg<ch::RdcrcRc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "DMARAM Channel 000 Source Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sadrc(&self) -> crate::common::Reg<ch::SadRc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "DMARAM Channel 000 Source and Destination Address CRC Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sdcrcrc(&self) -> crate::common::Reg<ch::SdcrcRc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "DMARAM Channel 000 Shadow Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn shadrc(&self) -> crate::common::Reg<ch::ShadRc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
}
pub mod ch {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AdicRc_SPEC;
    impl crate::sealed::RegSpec for AdicRc_SPEC {
        type DataType = u32;
    }
    #[doc = "DMARAM Channel 000 Address and Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
    pub type AdicRc = crate::RegValueT<AdicRc_SPEC>;

    impl AdicRc {
        #[doc = "Source Address Modification Factor   SMF. DMA channel TCS 32 bit source address modification factor and the        channel data width CHDW determines an address offset value by which the        source address is modified after each DMA move. If SCBE   1 B and CBLS  160    160 0000 B then the        source address is not modified."]
        #[inline(always)]
        pub fn smf(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, adicrc::Smf, AdicRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7,1,0,adicrc::Smf, AdicRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Increment of Source Address   INCS. DMA channel TCS control bit to determine if the address offset selected        by SMF will be added to or subtracted from the source address after each        DMA move. If SCBE   1 B and CBLS  160    160 0000 B then the source address is not modified."]
        #[inline(always)]
        pub fn incs(
            self,
        ) -> crate::common::RegisterField<3, 0x1, 1, 0, adicrc::Incs, AdicRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<3,0x1,1,0,adicrc::Incs, AdicRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Destination Address Modification Factor   DMF. DMA channel TCS 32 bit destination address modification factor and the        channel data width CHDW determines an address offset value by which the        destination address is modified after each DMA move. If DCBE   1 B and CBLD  160    160 0000 B then the        destination address is not modified."]
        #[inline(always)]
        pub fn dmf(
            self,
        ) -> crate::common::RegisterField<4, 0x7, 1, 0, adicrc::Dmf, AdicRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x7,1,0,adicrc::Dmf, AdicRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Increment of Destination Address   INCD. DMA channel TCS control bit to determine if the address offset selected        by DMF will be added to or subtracted from the destination address after        each DMA move. If DCBE   1 B and CBLD  160    160 0000 B the destination address is not modified."]
        #[inline(always)]
        pub fn incd(
            self,
        ) -> crate::common::RegisterField<7, 0x1, 1, 0, adicrc::Incd, AdicRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<7,0x1,1,0,adicrc::Incd, AdicRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Circular Buffer Length Source   CBLS. DMA channel TCS circular buffer source address update control bit        determines which part of the 32 bit source address register remains        unchanged and is not updated after a DMA move operation. CBLS determines the size of the circular source buffer."]
        #[inline(always)]
        pub fn cbls(
            self,
        ) -> crate::common::RegisterField<8, 0xf, 1, 0, adicrc::Cbls, AdicRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xf,1,0,adicrc::Cbls, AdicRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Circular Buffer Length Destination   CBLD. DMA channel TCS circular buffer destination address update control bit        determines which part of the 32 bit destination address register remains        unchanged and is not updated after a DMA move operation. CBLD determines the size of the circular destination buffer."]
        #[inline(always)]
        pub fn cbld(
            self,
        ) -> crate::common::RegisterField<12, 0xf, 1, 0, adicrc::Cbld, AdicRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                12,
                0xf,
                1,
                0,
                adicrc::Cbld,
                AdicRc_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Shadow Control   SHCT. DMA channel TCS shadow control determines the function of the shadow        address register."]
        #[inline(always)]
        pub fn shct(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, adicrc::Shct, AdicRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                16,
                0xf,
                1,
                0,
                adicrc::Shct,
                AdicRc_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Source Circular Buffer Enable   SCBE. DMA channel TCS source circular buffer enable."]
        #[inline(always)]
        pub fn scbe(
            self,
        ) -> crate::common::RegisterField<20, 0x1, 1, 0, adicrc::Scbe, AdicRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                20,
                0x1,
                1,
                0,
                adicrc::Scbe,
                AdicRc_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Destination Circular Buffer Enable   DCBE. DMA channel TCS destination circular buffer enable."]
        #[inline(always)]
        pub fn dcbe(
            self,
        ) -> crate::common::RegisterField<21, 0x1, 1, 0, adicrc::Dcbe, AdicRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                21,
                0x1,
                1,
                0,
                adicrc::Dcbe,
                AdicRc_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Time Stamp   STAMP. DMA channel TCS control bit to enable the appendage of a timestamp after        the end of the last DMA Move during a DMA transaction."]
        #[inline(always)]
        pub fn stamp(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x1,
            1,
            0,
            adicrc::Stamp,
            AdicRc_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                22,
                0x1,
                1,
                0,
                adicrc::Stamp,
                AdicRc_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Wrap Source Enable   WRPSE. DMA channel TCS source buffer interrupt trigger enable disable."]
        #[inline(always)]
        pub fn wrpse(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x1,
            1,
            0,
            adicrc::Wrpse,
            AdicRc_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0x1,
                1,
                0,
                adicrc::Wrpse,
                AdicRc_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Wrap Destination Enable   WRPDE. DMA channel TCS destination buffer interrupt trigger enable disable."]
        #[inline(always)]
        pub fn wrpde(
            self,
        ) -> crate::common::RegisterField<
            25,
            0x1,
            1,
            0,
            adicrc::Wrpde,
            AdicRc_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                25,
                0x1,
                1,
                0,
                adicrc::Wrpde,
                AdicRc_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Interrupt Control   INTCT. DMA channel TCS interrupt control. If DMA channel CHCFGR.PRSEL   1 B for          the next lower priority channel then the channel transfer trigger          interrupt is disabled."]
        #[inline(always)]
        pub fn intct(
            self,
        ) -> crate::common::RegisterField<
            26,
            0x3,
            1,
            0,
            adicrc::Intct,
            AdicRc_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                26,
                0x3,
                1,
                0,
                adicrc::Intct,
                AdicRc_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Interrupt Raise Detect Value   IRDV. DMA channel TCS interrupt threshold value defines the Threshold Limit of        CHSR.TCOUNT for which a channel interrupt trigger will be raised."]
        #[inline(always)]
        pub fn irdv(
            self,
        ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, AdicRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0xf,1,0,u8, AdicRc_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for AdicRc {
        #[inline(always)]
        fn default() -> AdicRc {
            <crate::RegValueT<AdicRc_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod adicrc {
        pub struct Smf_SPEC;
        pub type Smf = crate::EnumBitfieldStruct<u8, Smf_SPEC>;
        impl Smf {
            #[doc = "000 Address        offset is 1 x  160 CHDW"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "001 Address        offset is 2 x  160 CHDW"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "010 Address offset is 4 x CHDW"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "011 Address offset is 8 x CHDW"]
            pub const CONST_33: Self = Self::new(3);
            #[doc = "100 Address offset is 16 x CHDW"]
            pub const CONST_44: Self = Self::new(4);
            #[doc = "101 Address offset is 32 x CHDW"]
            pub const CONST_55: Self = Self::new(5);
            #[doc = "110 Address offset is 64 x CHDW"]
            pub const CONST_66: Self = Self::new(6);
            #[doc = "111 Address offset is 128 x CHDW"]
            pub const CONST_77: Self = Self::new(7);
        }
        pub struct Incs_SPEC;
        pub type Incs = crate::EnumBitfieldStruct<u8, Incs_SPEC>;
        impl Incs {
            #[doc = "0 Address offset        will be subtracted."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Address offset        will be added."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Dmf_SPEC;
        pub type Dmf = crate::EnumBitfieldStruct<u8, Dmf_SPEC>;
        impl Dmf {
            #[doc = "000 Address        offset is 1 x  160 CHDW"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "001 Address offset is 2 x CHDW"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "010 Address offset is 4 x CHDW"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "011 Address offset is 8 x CHDW"]
            pub const CONST_33: Self = Self::new(3);
            #[doc = "100 Address offset is 16 x CHDW"]
            pub const CONST_44: Self = Self::new(4);
            #[doc = "101 Address offset is 32 x CHDW"]
            pub const CONST_55: Self = Self::new(5);
            #[doc = "110 Address offset is 64 x CHDW"]
            pub const CONST_66: Self = Self::new(6);
            #[doc = "111 Address offset is 128 x CHDW"]
            pub const CONST_77: Self = Self::new(7);
        }
        pub struct Incd_SPEC;
        pub type Incd = crate::EnumBitfieldStruct<u8, Incd_SPEC>;
        impl Incd {
            #[doc = "0 Address offset        will be subtracted."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Address offset        will be added."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cbls_SPEC;
        pub type Cbls = crate::EnumBitfieldStruct<u8, Cbls_SPEC>;
        impl Cbls {
            #[doc = "0000 Source        address SADR 31 0  is not updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "0001 Source        address SADR 31 1  is not updated"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "0010 Source        address SADR 31 2  is not updated"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "0011 Source        address SADR 31 3  is not updated"]
            pub const CONST_33: Self = Self::new(3);
            #[doc = "0010 Source        address SADR 31 4  is not updated"]
            pub const CONST_44: Self = Self::new(4);
            #[doc = "0101 Source        address SADR 31 5  is not updated"]
            pub const CONST_55: Self = Self::new(5);
            #[doc = "0110 Source        address SADR 31 6  is not updated"]
            pub const CONST_66: Self = Self::new(6);
            #[doc = "0111 Source        address SADR 31 7  is not updated"]
            pub const CONST_77: Self = Self::new(7);
            #[doc = "1000 Source        address SADR 31 8  is not updated"]
            pub const CONST_88: Self = Self::new(8);
            #[doc = "1001 Source        address SADR 31 9  is not updated"]
            pub const CONST_99: Self = Self::new(9);
            #[doc = "1100 Source        address SADR 31 10  is not updated"]
            pub const CONST_1010: Self = Self::new(10);
            #[doc = "1011 Source        address SADR 31 11  is not updated"]
            pub const CONST_1111: Self = Self::new(11);
            #[doc = "1100 Source        address SADR 31 12  is not updated"]
            pub const CONST_1212: Self = Self::new(12);
            #[doc = "1101 Source        address SADR 31 13  is not updated"]
            pub const CONST_1313: Self = Self::new(13);
            #[doc = "1110 Source        address SADR 31 14  is not updated"]
            pub const CONST_1414: Self = Self::new(14);
            #[doc = "1111 Source        address SADR 31 15  is not updated"]
            pub const CONST_1515: Self = Self::new(15);
        }
        pub struct Cbld_SPEC;
        pub type Cbld = crate::EnumBitfieldStruct<u8, Cbld_SPEC>;
        impl Cbld {
            #[doc = "0000 Destination address DADR 31 0  is not updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "0001 Destination address DADR 31 1  is not updated"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "0010 Destination        address DADR 31 2  is not updated"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "0011 Destination        address DADR 31 3  is not updated"]
            pub const CONST_33: Self = Self::new(3);
            #[doc = "0100 Destination        address DADR 31 4  is not updated"]
            pub const CONST_44: Self = Self::new(4);
            #[doc = "0101 Destination        address DADR 31 5  is not updated"]
            pub const CONST_55: Self = Self::new(5);
            #[doc = "0110 Destination        address DADR 31 6  is not updated"]
            pub const CONST_66: Self = Self::new(6);
            #[doc = "0111 Destination        address DADR 31 7  is not updated"]
            pub const CONST_77: Self = Self::new(7);
            #[doc = "1000 Destination        address DADR 31 8  is not updated"]
            pub const CONST_88: Self = Self::new(8);
            #[doc = "1001 Destination        address DADR 31 9  is not updated"]
            pub const CONST_99: Self = Self::new(9);
            #[doc = "1010 Destination        address DADR 31 10  is not updated"]
            pub const CONST_1010: Self = Self::new(10);
            #[doc = "1011 Destination        address DADR 31 11  is not updated"]
            pub const CONST_1111: Self = Self::new(11);
            #[doc = "1100 Destination        address DADR 31 12  is not updated"]
            pub const CONST_1212: Self = Self::new(12);
            #[doc = "1101 Destination        address DADR 31 13  is not updated"]
            pub const CONST_1313: Self = Self::new(13);
            #[doc = "1110 Destination        address DADR 31 14  is not updated"]
            pub const CONST_1414: Self = Self::new(14);
            #[doc = "1111 Destination        address DADR 31 15  is not updated"]
            pub const CONST_1515: Self = Self::new(15);
        }
        pub struct Shct_SPEC;
        pub type Shct = crate::EnumBitfieldStruct<u8, Shct_SPEC>;
        impl Shct {
            #[doc = "0000 Move        Operation."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "0001 Shadow        Operation Read Only Mode Source Address."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "0010 Shadow        Operation Read Only Mode Destination Address."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "0101 Shadow        Operation Direct Write Mode Source Address."]
            pub const CONST_55: Self = Self::new(5);
            #[doc = "0110 Shadow        Operation Direct Write Mode Destination Address."]
            pub const CONST_66: Self = Self::new(6);
            #[doc = "1000 DMA Double        Source Buffering with Software Switch Only."]
            pub const CONST_88: Self = Self::new(8);
            #[doc = "1001 DMA Double        Source Buffering with Software Switch and Automatic Hardware Switch."]
            pub const CONST_99: Self = Self::new(9);
            #[doc = "1010 DMA Double        Destination Buffering with Software Switch Only."]
            pub const CONST_1010: Self = Self::new(10);
            #[doc = "1011 DMA Double        Destination Buffering with Software Switch and Automatic Hardware        Switch."]
            pub const CONST_1111: Self = Self::new(11);
            #[doc = "1100 DMA Linked        List  DMALL ."]
            pub const CONST_1212: Self = Self::new(12);
            #[doc = "1101 Accumulated        Linked List  ACCLL ."]
            pub const CONST_1313: Self = Self::new(13);
            #[doc = "1110 Safe Linked        List  SAFLL ."]
            pub const CONST_1414: Self = Self::new(14);
            #[doc = "1111 Conditional        Linked List  CONLL ."]
            pub const CONST_1515: Self = Self::new(15);
        }
        pub struct Scbe_SPEC;
        pub type Scbe = crate::EnumBitfieldStruct<u8, Scbe_SPEC>;
        impl Scbe {
            #[doc = "0 Source circular        buffer disabled."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Source circular        buffer enabled."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Dcbe_SPEC;
        pub type Dcbe = crate::EnumBitfieldStruct<u8, Dcbe_SPEC>;
        impl Dcbe {
            #[doc = "0 Destination        circular buffer disabled."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Destination        circular buffer enabled."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Stamp_SPEC;
        pub type Stamp = crate::EnumBitfieldStruct<u8, Stamp_SPEC>;
        impl Stamp {
            #[doc = "0 No action."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 DMA timestamp        is appended."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Wrpse_SPEC;
        pub type Wrpse = crate::EnumBitfieldStruct<u8, Wrpse_SPEC>;
        impl Wrpse {
            #[doc = "0 Wrap source        buffer interrupt trigger disabled."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Wrap source        buffer interrupt trigger enabled."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Wrpde_SPEC;
        pub type Wrpde = crate::EnumBitfieldStruct<u8, Wrpde_SPEC>;
        impl Wrpde {
            #[doc = "0 Wrap        destination buffer interrupt trigger disabled."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Wrap        destination buffer interrupt trigger enabled."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Intct_SPEC;
        pub type Intct = crate::EnumBitfieldStruct<u8, Intct_SPEC>;
        impl Intct {
            #[doc = "00 No interrupt        trigger will be generated on changing the TCOUNT value. The DMA channel        CHSR.ICH is set when TCOUNT equals IRDV."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 No interrupt        trigger will be generated on changing the TCOUNT value. The DMA channel        CHSR.ICH is set when TCOUNT is decremented"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Interrupt        trigger is generated and DMA channel CHSR.ICH is set on changing the        TCOUNT value and TCOUNT equals IRDV"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Interrupt        trigger is generated and DMA channel CHSR.ICH is set each time TCOUNT is        decremented"]
            pub const CONST_33: Self = Self::new(3);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChcfgRc_SPEC;
    impl crate::sealed::RegSpec for ChcfgRc_SPEC {
        type DataType = u32;
    }
    #[doc = "DMARAM Channel 000 Configuration Register\n resetvalue={Application Reset:0x0}"]
    pub type ChcfgRc = crate::RegValueT<ChcfgRc_SPEC>;

    impl ChcfgRc {
        #[doc = "Transfer Reload Value   TREL. DMA channel TCS transfer reload value to control the number of DMA        transfers in a DMA transaction. The 14 bit transfer count value is        loaded into ME CHSR.TCOUNT at the start of a DMA transaction  when        TSR.CH becomes set and CHSR.TCOUNT  160    160 0 . A write to CHCFGR.TREL during a        running DMA transaction has no influence on the running DMA transaction. If CHCFGR.TREL  160    160 0 or if CHCFGR.TREL  160    160 1 then ME CHSR.TCOUNT will be        loaded with 1 when a new DMA transaction is started  at least one DMA        transfer must be executed per DMA transaction ."]
        #[inline(always)]
        pub fn trel(
            self,
        ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, ChcfgRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3fff,1,0,u16, ChcfgRc_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Block Mode   BLKM. Defines the number of DMA moves executed during one DMA transfer."]
        #[inline(always)]
        pub fn blkm(
            self,
        ) -> crate::common::RegisterField<
            16,
            0x7,
            1,
            0,
            chcfgrc::Blkm,
            ChcfgRc_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0x7,
                1,
                0,
                chcfgrc::Blkm,
                ChcfgRc_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Reset Request Only After Transaction   RROAT. DMA channel control bit to determine if the DMA request state flag  DMA        channel TSR.CH  is reset after each DMA transfer."]
        #[inline(always)]
        pub fn rroat(
            self,
        ) -> crate::common::RegisterField<
            19,
            0x1,
            1,
            0,
            chcfgrc::Rroat,
            ChcfgRc_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                19,
                0x1,
                1,
                0,
                chcfgrc::Rroat,
                ChcfgRc_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Channel Operation Mode   CHMODE. DMA channel TCS control to determine TSR.HTRE reset condition."]
        #[inline(always)]
        pub fn chmode(
            self,
        ) -> crate::common::RegisterField<
            20,
            0x1,
            1,
            0,
            chcfgrc::Chmode,
            ChcfgRc_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0x1,
                1,
                0,
                chcfgrc::Chmode,
                ChcfgRc_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Channel Data Width   CHDW. DMA channel TCS data width for DMA read moves and DMA write moves."]
        #[inline(always)]
        pub fn chdw(
            self,
        ) -> crate::common::RegisterField<
            21,
            0x7,
            1,
            0,
            chcfgrc::Chdw,
            ChcfgRc_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                21,
                0x7,
                1,
                0,
                chcfgrc::Chdw,
                ChcfgRc_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Pattern Select   PATSEL. DMA channel TCS bit field to select the pattern detection operation  see        Functional Description . If PATSEL 1 0  is not equal to 00 B then a ME pattern detection operation defined by the channel data width         CHDW  will be performed. PATSEL 2  selects the pattern read register.        If a pattern match is detected then a DMA channel interrupt shall be        triggered."]
        #[inline(always)]
        pub fn patsel(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x7,
            1,
            0,
            chcfgrc::Patsel,
            ChcfgRc_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0x7,
                1,
                0,
                chcfgrc::Patsel,
                ChcfgRc_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Swap Data CRC Byte Order   SWAP. DMA channel TCS swap data CRC byte order."]
        #[inline(always)]
        pub fn swap(
            self,
        ) -> crate::common::RegisterField<
            27,
            0x1,
            1,
            0,
            chcfgrc::Swap,
            ChcfgRc_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                27,
                0x1,
                1,
                0,
                chcfgrc::Swap,
                ChcfgRc_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Peripheral Request Select   PRSEL. DMA channel TCS control bit field to select the source of a DMA request."]
        #[inline(always)]
        pub fn prsel(
            self,
        ) -> crate::common::RegisterField<
            28,
            0x1,
            1,
            0,
            chcfgrc::Prsel,
            ChcfgRc_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                28,
                0x1,
                1,
                0,
                chcfgrc::Prsel,
                ChcfgRc_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for ChcfgRc {
        #[inline(always)]
        fn default() -> ChcfgRc {
            <crate::RegValueT<ChcfgRc_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod chcfgrc {
        pub struct Blkm_SPEC;
        pub type Blkm = crate::EnumBitfieldStruct<u8, Blkm_SPEC>;
        impl Blkm {
            #[doc = "000 One DMA        transfer has 1 DMA move"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "001 One DMA        transfer has 2 DMA moves"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "010 One DMA        transfer has 4 DMA moves"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "011 One DMA        transfer has 8 DMA moves"]
            pub const CONST_33: Self = Self::new(3);
            #[doc = "100 One DMA        transfer has 16 DMA moves"]
            pub const CONST_44: Self = Self::new(4);
            #[doc = "101 One DMA        transfer has 3 DMA moves"]
            pub const CONST_55: Self = Self::new(5);
            #[doc = "110 One DMA        transfer has 5 DMA moves"]
            pub const CONST_66: Self = Self::new(6);
            #[doc = "111 One DMA        transfer has 9 DMA moves"]
            pub const CONST_77: Self = Self::new(7);
        }
        pub struct Rroat_SPEC;
        pub type Rroat = crate::EnumBitfieldStruct<u8, Rroat_SPEC>;
        impl Rroat {
            #[doc = "0 DMA channel        TSR.CH is reset after the start of each DMA transfer. A DMA request is        required for each DMA transfer."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 DMA channel        TSR.CH is reset when CHSR.TCOUNT  160    160 0 and after the completion of the        last DMA transfer  i.e. on completion of the DMA transaction . One DMA        request starts a complete DMA transaction."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Chmode_SPEC;
        pub type Chmode = crate::EnumBitfieldStruct<u8, Chmode_SPEC>;
        impl Chmode {
            #[doc = "0 Single Mode is        selected for DMA channel. After a DMA transaction  DMA channel is        disabled for further hardware requests  TSR.HTRE is reset by hardware         TSR.HTRE must be set again by software for starting a new transaction."]
            pub const SINGLE_MODE_0: Self = Self::new(0);
            #[doc = "1 Continuous Mode is        selected for DMA channel. After a DMA transaction  bit TSR.HTRE remains        set."]
            pub const CONTINUOUS_MODE_1: Self = Self::new(1);
        }
        pub struct Chdw_SPEC;
        pub type Chdw = crate::EnumBitfieldStruct<u8, Chdw_SPEC>;
        impl Chdw {
            #[doc = "000 8 bit data        width for moves selected Single Data Transfer Byte  SDTB"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "001 16 bit data        width for moves selected Single Data Transfer Half Word  SDTH"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "010 32 bit data        width for moves selected Single Data Transfer Word  SDTW"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "011 64 bit data        width transaction selected SRI Bus  Single Data Transfer Double Word  SDTD  SPB Bus  not supported."]
            pub const CONST_33: Self = Self::new(3);
            #[doc = "100 128 bit data        width transaction selected SRI Bus  Block Transfer Request   2 Transfers  BTR2  SPB Bus  not supported."]
            pub const CONST_44: Self = Self::new(4);
            #[doc = "101 256 bit data        width transaction selected SRI Bus  Block Transfer Request   4 Transfers  BTR4  SPB Bus  not supported."]
            pub const CONST_55: Self = Self::new(5);
        }
        pub struct Patsel_SPEC;
        pub type Patsel = crate::EnumBitfieldStruct<u8, Patsel_SPEC>;
        impl Patsel {
            #[doc = "0000 No pattern        compare operation."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "001 DMA read move        data compared with PRR0."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "010 DMA read move        data compared with PRR0."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "011 DMA read move        data compared with PRR0."]
            pub const CONST_33: Self = Self::new(3);
            #[doc = "100 No pattern        compare operation."]
            pub const CONST_44: Self = Self::new(4);
            #[doc = "001 DMA read move        data compared with PRR1."]
            pub const CONST_55: Self = Self::new(5);
            #[doc = "001 DMA read move        data compared with PRR1."]
            pub const CONST_66: Self = Self::new(6);
            #[doc = "001 DMA read move        data compared with PRR1."]
            pub const CONST_77: Self = Self::new(7);
        }
        pub struct Swap_SPEC;
        pub type Swap = crate::EnumBitfieldStruct<u8, Swap_SPEC>;
        impl Swap {
            #[doc = "0 Byte order is        not swapped"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Byte order is        swapped."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Prsel_SPEC;
        pub type Prsel = crate::EnumBitfieldStruct<u8, Prsel_SPEC>;
        impl Prsel {
            #[doc = "0 DMA hardware        request selected."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 DMA daisy chain        request selected."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChcsRc_SPEC;
    impl crate::sealed::RegSpec for ChcsRc_SPEC {
        type DataType = u32;
    }
    #[doc = "DMARAM Channel 000 Control and Status Register\n resetvalue={Application Reset:0x0}"]
    pub type ChcsRc = crate::RegValueT<ChcsRc_SPEC>;

    impl ChcsRc {
        #[doc = "Transfer Count   TCOUNT. DMA channel status transfer count updated after DMARAM write back."]
        #[inline(always)]
        pub fn tcount(
            self,
        ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, ChcsRc_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0x3fff,1,0,u16, ChcsRc_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Old Value of Pattern Detection   LXO. DMA channel status bit to store the result of a pattern detection        operation when 8 bit or 16 bit data width is selected."]
        #[inline(always)]
        pub fn lxo(
            self,
        ) -> crate::common::RegisterField<15, 0x1, 1, 0, chcsrc::Lxo, ChcsRc_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<15,0x1,1,0,chcsrc::Lxo, ChcsRc_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Wrap Source Buffer   WRPS. Status bit indicates that a DMA channel has reached a wrap source buffer        boundary. Bit is reset by software  DMA channel CHCSR.CWRP   1 B or DMA channel reset TSR.RST   1 B  ."]
        #[inline(always)]
        pub fn wrps(
            self,
        ) -> crate::common::RegisterField<16, 0x1, 1, 0, chcsrc::Wrps, ChcsRc_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0x1,1,0,chcsrc::Wrps, ChcsRc_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Wrap Destination Buffer   WRPD. Status bit indicates that a DMA channel has reached a wrap destination        buffer boundary. Bit is reset by software  DMA channel CHCSR.CWRP   1 B or DMA channel reset TSR.RST   1 B  ."]
        #[inline(always)]
        pub fn wrpd(
            self,
        ) -> crate::common::RegisterField<17, 0x1, 1, 0, chcsrc::Wrpd, ChcsRc_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<17,0x1,1,0,chcsrc::Wrpd, ChcsRc_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Interrupt from Channel   ICH. As soon as ME CHSR.TCOUNT decrements  if ADICR.INTCT 0    0 and        CHSR.COUNT   IRDV then ME CHSR.ICH is set else ME CHSR.ICH is set for        each decrement of CSR.TCOUNT. Bit is reset by software  DMA channel        CHCSR.CICH   1 B or by a DMA channel reset TSR.RST   1 B  ."]
        #[inline(always)]
        pub fn ich(
            self,
        ) -> crate::common::RegisterField<18, 0x1, 1, 0, chcsrc::Ich, ChcsRc_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<18,0x1,1,0,chcsrc::Ich, ChcsRc_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Pattern Detection from Channel   IPM. Status bit indicates that a pattern match has been detected for the DMA        channel when pattern detection is enabled. This bit is reset by software         DMA channel CHCSR.CICH   1 B or DMA channel reset TSR.RST   1 B  ."]
        #[inline(always)]
        pub fn ipm(
            self,
        ) -> crate::common::RegisterField<19, 0x1, 1, 0, chcsrc::Ipm, ChcsRc_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<19,0x1,1,0,chcsrc::Ipm, ChcsRc_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "DMA Double Buffering Active Buffer   BUFFER. During a DMA double buffering operation  the status bit indicates which        buffer is read or filled."]
        #[inline(always)]
        pub fn buffer(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x1,
            1,
            0,
            chcsrc::Buffer,
            ChcsRc_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                22,
                0x1,
                1,
                0,
                chcsrc::Buffer,
                ChcsRc_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "DMA Double Buffering Frozen Buffer   FROZEN. If a DMA channel is configured for double buffering operation  the        FROZEN bit indicates that one of the buffers is frozen and available for        processing by a cyclic software task. FROZEN bit shall only be set by the DMA and shall only be cleared by          software."]
        #[inline(always)]
        pub fn frozen(
            self,
        ) -> crate::common::RegisterField<
            23,
            0x1,
            1,
            0,
            chcsrc::Frozen,
            ChcsRc_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                23,
                0x1,
                1,
                0,
                chcsrc::Frozen,
                ChcsRc_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "DMA Double Buffering Switch Buffer   SWB. When DMA double buffering is configured  the control bit is used to        re direct data from one buffer to the other buffer. If a DMALL  ACCLL  SAFLL or CONLL operation is configured then SWB          shall be 0 B ."]
        #[inline(always)]
        pub fn swb(
            self,
        ) -> crate::common::RegisterField<24, 0x1, 1, 0, chcsrc::Swb, ChcsRc_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<24,0x1,1,0,chcsrc::Swb, ChcsRc_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Wrap Buffer Interrupt   CWRP. Software clear of the DMA channel source and destination wrap buffer        flags stored at CHCSR.WRPS and CHCSR.WRPD. If the DMA channel is active        in a ME then clear ME bit fields CHSR.WRPS and CHSR.WRPD. Reading this        bit returns a 0."]
        #[inline(always)]
        pub fn cwrp(
            self,
        ) -> crate::common::RegisterField<25, 0x1, 1, 0, chcsrc::Cwrp, ChcsRc_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<25,0x1,1,0,chcsrc::Cwrp, ChcsRc_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Clear Interrupt for DMA Channel   CICH. Software clear of the DMA channel flags stored at CHCSR.ICH and        CHCSR.IPM. If the DMA channel is active in a ME then clear ME bit fields        CHSR.ICH and CHSR.IPM. Reading this bit returns a 0."]
        #[inline(always)]
        pub fn cich(
            self,
        ) -> crate::common::RegisterField<26, 0x1, 1, 0, chcsrc::Cich, ChcsRc_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<26,0x1,1,0,chcsrc::Cich, ChcsRc_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Set Interrupt Trigger for DMA Channel   SIT. Reading this bit returns a 0. If a DMALL  ACCLL  SAFLL or CONLL operation is configured then SIT          must be 0 B ."]
        #[inline(always)]
        pub fn sit(
            self,
        ) -> crate::common::RegisterField<27, 0x1, 1, 0, chcsrc::Sit, ChcsRc_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<27,0x1,1,0,chcsrc::Sit, ChcsRc_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Set Transaction Request   SCH. Reading this bit returns a 0."]
        #[inline(always)]
        pub fn sch(
            self,
        ) -> crate::common::RegisterField<31, 0x1, 1, 0, chcsrc::Sch, ChcsRc_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<31,0x1,1,0,chcsrc::Sch, ChcsRc_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for ChcsRc {
        #[inline(always)]
        fn default() -> ChcsRc {
            <crate::RegValueT<ChcsRc_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod chcsrc {
        pub struct Lxo_SPEC;
        pub type Lxo = crate::EnumBitfieldStruct<u8, Lxo_SPEC>;
        impl Lxo {
            #[doc = "0 The        corresponding pattern compare operation did not find a pattern match        during the previous DMA read move."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 The        corresponding pattern compare operation found a pattern match during the        previous DMA read move."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Wrps_SPEC;
        pub type Wrps = crate::EnumBitfieldStruct<u8, Wrps_SPEC>;
        impl Wrps {
            #[doc = "0 No wrap source        buffer occurred."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Wrap source        buffer occurred."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Wrpd_SPEC;
        pub type Wrpd = crate::EnumBitfieldStruct<u8, Wrpd_SPEC>;
        impl Wrpd {
            #[doc = "0 No wrap        destination buffer occurred."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Wrap        destination buffer occurred."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Ich_SPEC;
        pub type Ich = crate::EnumBitfieldStruct<u8, Ich_SPEC>;
        impl Ich {
            #[doc = "0 An interrupt        from channel has not been detected."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 An interrupt        from channel has been detected."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Ipm_SPEC;
        pub type Ipm = crate::EnumBitfieldStruct<u8, Ipm_SPEC>;
        impl Ipm {
            #[doc = "0 A pattern match        has not been detected."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 A pattern match        has been detected."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Buffer_SPEC;
        pub type Buffer = crate::EnumBitfieldStruct<u8, Buffer_SPEC>;
        impl Buffer {
            #[doc = "0 Buffer 0 read or filled by DMA."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Buffer 1 read or filled by DMA."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Frozen_SPEC;
        pub type Frozen = crate::EnumBitfieldStruct<u8, Frozen_SPEC>;
        impl Frozen {
            #[doc = "0 Buffer is not        frozen."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Buffer is        frozen."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Swb_SPEC;
        pub type Swb = crate::EnumBitfieldStruct<u8, Swb_SPEC>;
        impl Swb {
            #[doc = "0 No action."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Switch from        buffer."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cwrp_SPEC;
        pub type Cwrp = crate::EnumBitfieldStruct<u8, Cwrp_SPEC>;
        impl Cwrp {
            #[doc = "0 No action."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Clear DMA        channel bits CSR.WRPS and CSR.WRPD."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cich_SPEC;
        pub type Cich = crate::EnumBitfieldStruct<u8, Cich_SPEC>;
        impl Cich {
            #[doc = "0 No action."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Clear DMA        channel bits CSR.ICH and CSR.IPM."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Sit_SPEC;
        pub type Sit = crate::EnumBitfieldStruct<u8, Sit_SPEC>;
        impl Sit {
            #[doc = "0 No action."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 DMA channel        interrupt trigger will be activated."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Sch_SPEC;
        pub type Sch = crate::EnumBitfieldStruct<u8, Sch_SPEC>;
        impl Sch {
            #[doc = "0 No action."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 DMA software        request initiated for DMA channel. When SCH is set  DMA channel TSR.CH        is set to indicate a DMA request is pending."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DadRc_SPEC;
    impl crate::sealed::RegSpec for DadRc_SPEC {
        type DataType = u32;
    }
    #[doc = "DMARAM Channel 000 Destination Address Register\n resetvalue={Application Reset:0x0}"]
    pub type DadRc = crate::RegValueT<DadRc_SPEC>;

    impl DadRc {
        #[doc = "Destination Address   DADR. 32 bit destination address."]
        #[inline(always)]
        pub fn dadr(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, DadRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, DadRc_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for DadRc {
        #[inline(always)]
        fn default() -> DadRc {
            <crate::RegValueT<DadRc_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RdcrcRc_SPEC;
    impl crate::sealed::RegSpec for RdcrcRc_SPEC {
        type DataType = u32;
    }
    #[doc = "DMARAM Channel 000 Read Data CRC Register\n resetvalue={Application Reset:0x0}"]
    pub type RdcrcRc = crate::RegValueT<RdcrcRc_SPEC>;

    impl RdcrcRc {
        #[doc = "Read Data CRC   RDCRC. Checksum calculated for DMA read move data."]
        #[inline(always)]
        pub fn rdcrc(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, RdcrcRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, RdcrcRc_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for RdcrcRc {
        #[inline(always)]
        fn default() -> RdcrcRc {
            <crate::RegValueT<RdcrcRc_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SadRc_SPEC;
    impl crate::sealed::RegSpec for SadRc_SPEC {
        type DataType = u32;
    }
    #[doc = "DMARAM Channel 000 Source Address Register\n resetvalue={Application Reset:0x0}"]
    pub type SadRc = crate::RegValueT<SadRc_SPEC>;

    impl SadRc {
        #[doc = "Source Address   SADR. 32 bit source address."]
        #[inline(always)]
        pub fn sadr(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, SadRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, SadRc_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for SadRc {
        #[inline(always)]
        fn default() -> SadRc {
            <crate::RegValueT<SadRc_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SdcrcRc_SPEC;
    impl crate::sealed::RegSpec for SdcrcRc_SPEC {
        type DataType = u32;
    }
    #[doc = "DMARAM Channel 000 Source and Destination Address CRC Register\n resetvalue={Application Reset:0x0}"]
    pub type SdcrcRc = crate::RegValueT<SdcrcRc_SPEC>;

    impl SdcrcRc {
        #[doc = "Source and Destination Address CRC   SDCRC. Checksum calculated for DMA move source and destination addresses."]
        #[inline(always)]
        pub fn sdcrc(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, SdcrcRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, SdcrcRc_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for SdcrcRc {
        #[inline(always)]
        fn default() -> SdcrcRc {
            <crate::RegValueT<SdcrcRc_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ShadRc_SPEC;
    impl crate::sealed::RegSpec for ShadRc_SPEC {
        type DataType = u32;
    }
    #[doc = "DMARAM Channel 000 Shadow Address Register\n resetvalue={Application Reset:0x0}"]
    pub type ShadRc = crate::RegValueT<ShadRc_SPEC>;

    impl ShadRc {
        #[doc = "Shadowed Address   SHADR. 32 bit shadow address."]
        #[inline(always)]
        pub fn shadr(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, ShadRc_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, ShadRc_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for ShadRc {
        #[inline(always)]
        fn default() -> ShadRc {
            <crate::RegValueT<ShadRc_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
