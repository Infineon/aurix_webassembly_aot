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
#[doc = r"ERAY"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eray0(pub(super) *mut u8);
unsafe impl core::marker::Send for Eray0 {}
unsafe impl core::marker::Sync for Eray0 {}
impl Eray0 {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2300usize)) }
    }

    #[doc = "Aggregated Channel Status\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn acs(&self) -> crate::common::Reg<self::Acs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(296usize)) }
    }

    #[doc = "Communication Controller Error Vector\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ccev(&self) -> crate::common::Reg<self::Ccev_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
    }

    #[doc = "Communication Controller Status Vector\n resetvalue={Application Reset:0x104000}"]
    #[inline(always)]
    pub const fn ccsv(&self) -> crate::common::Reg<self::Ccsv_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Core Release Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn crel(&self) -> crate::common::Reg<self::Crel_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1008usize)) }
    }

    #[doc = "Busy and Input Buffer Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cust1(&self) -> crate::common::Reg<self::Cust1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }

    #[doc = "Customer Interface Timeout Counter Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cust3(&self) -> crate::common::Reg<self::Cust3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }

    #[doc = "Error Service Request Enable Reset\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn eier(&self) -> crate::common::Reg<self::Eier_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "Error Service Request Enable Set\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn eies(&self) -> crate::common::Reg<self::Eies_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }

    #[doc = "Error Service Request Line Select\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn eils(&self) -> crate::common::Reg<self::Eils_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }

    #[doc = "Error Service Request Select Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn eir(&self) -> crate::common::Reg<self::Eir_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "Endian Register\n resetvalue={Application Reset:0x087654321}"]
    #[inline(always)]
    pub const fn endn(&self) -> crate::common::Reg<self::Endn_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1012usize)) }
    }

    #[doc = "Even Sync ID Symbol Window 01\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn esidn(&self) -> [crate::common::Reg<self::EsiDn_SPEC, crate::common::R>; 15] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x130usize + 0x38usize)),
            ]
        }
    }

    #[doc = "FIFO Critical Level\n resetvalue={Application Reset:0x080}"]
    #[inline(always)]
    pub const fn fcl(&self) -> crate::common::Reg<self::Fcl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(780usize)) }
    }

    #[doc = "FIFO Rejection Filter\n resetvalue={Application Reset:0x1800000}"]
    #[inline(always)]
    pub const fn frf(&self) -> crate::common::Reg<self::Frf_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(772usize)) }
    }

    #[doc = "FIFO Rejection Filter Mask\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn frfm(&self) -> crate::common::Reg<self::Frfm_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(776usize)) }
    }

    #[doc = "FIFO Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fsr(&self) -> crate::common::Reg<self::Fsr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(792usize)) }
    }

    #[doc = "GTU Configuration Register 1\n resetvalue={Application Reset:0x280}"]
    #[inline(always)]
    pub const fn gtuc01(&self) -> crate::common::Reg<self::Gtuc01_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }

    #[doc = "GTU Configuration Register 2\n resetvalue={Application Reset:0x2000A}"]
    #[inline(always)]
    pub const fn gtuc02(&self) -> crate::common::Reg<self::Gtuc02_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(164usize)) }
    }

    #[doc = "GTU Configuration Register 3\n resetvalue={Application Reset:0x2020000}"]
    #[inline(always)]
    pub const fn gtuc03(&self) -> crate::common::Reg<self::Gtuc03_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(168usize)) }
    }

    #[doc = "GTU Configuration Register 4\n resetvalue={Application Reset:0x080007}"]
    #[inline(always)]
    pub const fn gtuc04(&self) -> crate::common::Reg<self::Gtuc04_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(172usize)) }
    }

    #[doc = "GTU Configuration Register 5\n resetvalue={Application Reset:0x0E000000}"]
    #[inline(always)]
    pub const fn gtuc05(&self) -> crate::common::Reg<self::Gtuc05_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(176usize)) }
    }

    #[doc = "GTU Configuration Register 6\n resetvalue={Application Reset:0x20000}"]
    #[inline(always)]
    pub const fn gtuc06(&self) -> crate::common::Reg<self::Gtuc06_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(180usize)) }
    }

    #[doc = "GTU Configuration Register 7\n resetvalue={Application Reset:0x20004}"]
    #[inline(always)]
    pub const fn gtuc07(&self) -> crate::common::Reg<self::Gtuc07_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(184usize)) }
    }

    #[doc = "GTU Configuration Register 8\n resetvalue={Application Reset:0x2}"]
    #[inline(always)]
    pub const fn gtuc08(&self) -> crate::common::Reg<self::Gtuc08_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(188usize)) }
    }

    #[doc = "GTU Configuration Register 9\n resetvalue={Application Reset:0x101}"]
    #[inline(always)]
    pub const fn gtuc09(&self) -> crate::common::Reg<self::Gtuc09_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(192usize)) }
    }

    #[doc = "GTU Configuration Register 10\n resetvalue={Application Reset:0x20005}"]
    #[inline(always)]
    pub const fn gtuc10(&self) -> crate::common::Reg<self::Gtuc10_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(196usize)) }
    }

    #[doc = "GTU Configuration Register 11\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gtuc11(&self) -> crate::common::Reg<self::Gtuc11_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(200usize)) }
    }

    #[doc = "Input Buffer Command Mask\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ibcm(&self) -> crate::common::Reg<self::Ibcm_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1296usize)) }
    }

    #[doc = "Input Buffer Command Request\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ibcr(&self) -> crate::common::Reg<self::Ibcr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1300usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x44C000}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Service Request Line Enable\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ile(&self) -> crate::common::Reg<self::Ile_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst0(&self) -> crate::common::Reg<self::Krst0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2292usize)) }
    }

    #[doc = "Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst1(&self) -> crate::common::Reg<self::Krst1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2288usize)) }
    }

    #[doc = "Kernel Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krstclr(&self) -> crate::common::Reg<self::Krstclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2284usize)) }
    }

    #[doc = "Lock Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn lck(&self) -> crate::common::Reg<self::Lck_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "Last Dynamic Transmit Slot\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ldts(&self) -> crate::common::Reg<self::Ldts_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(788usize)) }
    }

    #[doc = "Message Buffer Status\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mbs(&self) -> crate::common::Reg<self::Mbs_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1804usize)) }
    }

    #[doc = "Message Buffer Status Changed 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mbsc1(&self) -> crate::common::Reg<self::Mbsc1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(832usize)) }
    }

    #[doc = "Message Buffer Status Changed 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mbsc2(&self) -> crate::common::Reg<self::Mbsc2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(836usize)) }
    }

    #[doc = "Message Buffer Status Changed 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mbsc3(&self) -> crate::common::Reg<self::Mbsc3_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(840usize)) }
    }

    #[doc = "Message Buffer Status Changed 4\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mbsc4(&self) -> crate::common::Reg<self::Mbsc4_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(844usize)) }
    }

    #[doc = "MHD Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mhdc(&self) -> crate::common::Reg<self::Mhdc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(152usize)) }
    }

    #[doc = "Message Handler Constraints Flags\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mhdf(&self) -> crate::common::Reg<self::Mhdf_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(796usize)) }
    }

    #[doc = "Message Handler Status\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mhds(&self) -> crate::common::Reg<self::Mhds_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(784usize)) }
    }

    #[doc = "Message RAM Configuration\n resetvalue={Application Reset:0x1800000}"]
    #[inline(always)]
    pub const fn mrc(&self) -> crate::common::Reg<self::Mrc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(768usize)) }
    }

    #[doc = "Message Buffer Status Changed Interrupt Control 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn msic1(&self) -> crate::common::Reg<self::Msic1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(952usize)) }
    }

    #[doc = "Message Buffer Status Changed Interrupt Control 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn msic2(&self) -> crate::common::Reg<self::Msic2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(956usize)) }
    }

    #[doc = "Message Buffer Status Changed Interrupt Control 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn msic3(&self) -> crate::common::Reg<self::Msic3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(960usize)) }
    }

    #[doc = "Message Buffer Status Changed Interrupt Control 4\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn msic4(&self) -> crate::common::Reg<self::Msic4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(964usize)) }
    }

    #[doc = "Macrotick and Cycle Counter Value\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mtccv(&self) -> crate::common::Reg<self::Mtccv_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(276usize)) }
    }

    #[doc = "New Data Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ndat1(&self) -> crate::common::Reg<self::Ndat1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(816usize)) }
    }

    #[doc = "New Data Register 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ndat2(&self) -> crate::common::Reg<self::Ndat2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(820usize)) }
    }

    #[doc = "New Data Register 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ndat3(&self) -> crate::common::Reg<self::Ndat3_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(824usize)) }
    }

    #[doc = "New Data Register 4\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ndat4(&self) -> crate::common::Reg<self::Ndat4_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(828usize)) }
    }

    #[doc = "New Data Interrupt Control 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ndic1(&self) -> crate::common::Reg<self::Ndic1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(936usize)) }
    }

    #[doc = "New Data Interrupt Control 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ndic2(&self) -> crate::common::Reg<self::Ndic2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(940usize)) }
    }

    #[doc = "New Data Interrupt Control 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ndic3(&self) -> crate::common::Reg<self::Ndic3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(944usize)) }
    }

    #[doc = "New Data Interrupt Control 4\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ndic4(&self) -> crate::common::Reg<self::Ndic4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(948usize)) }
    }

    #[doc = "NEM Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn nemc(&self) -> crate::common::Reg<self::Nemc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(140usize)) }
    }

    #[doc = "Network Management Vector 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn nmvx(&self) -> [crate::common::Reg<self::NmVx_SPEC, crate::common::R>; 3] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x1b0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1b0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1b0usize + 0x8usize)),
            ]
        }
    }

    #[doc = "Output Buffer Command Mask\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn obcm(&self) -> crate::common::Reg<self::Obcm_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1808usize)) }
    }

    #[doc = "Output Buffer Command Request\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn obcr(&self) -> crate::common::Reg<self::Obcr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1812usize)) }
    }

    #[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2280usize)) }
    }

    #[doc = "Offset Correction Value\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ocv(&self) -> crate::common::Reg<self::Ocv_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(284usize)) }
    }

    #[doc = "Odd Sync ID Symbol Window 01\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn osidn(&self) -> [crate::common::Reg<self::OsiDn_SPEC, crate::common::R>; 15] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x170usize + 0x38usize)),
            ]
        }
    }

    #[doc = "OCDS Trigger Set Select\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn otss(&self) -> crate::common::Reg<self::Otss_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(2160usize)) }
    }

    #[doc = "PRT Configuration Register 1\n resetvalue={Application Reset:0x084C0633}"]
    #[inline(always)]
    pub const fn prtc1(&self) -> crate::common::Reg<self::Prtc1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }

    #[doc = "PRT Configuration Register 2\n resetvalue={Application Reset:0x0F2D0A0E}"]
    #[inline(always)]
    pub const fn prtc2(&self) -> crate::common::Reg<self::Prtc2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(148usize)) }
    }

    #[doc = "Rate Correction Value\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rcv(&self) -> crate::common::Reg<self::Rcv_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(280usize)) }
    }

    #[doc = "Read Data Section 01\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rddsn(&self) -> [crate::common::Reg<self::RddSn_SPEC, crate::common::R>; 64] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x5cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x60usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x64usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x68usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x6cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x70usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x74usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x78usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x7cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x80usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x84usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x88usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x8cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x90usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x94usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x98usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0x9cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xa0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xa4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xa8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xacusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xb0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xb4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xb8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xbcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xc0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xc4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xc8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xccusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xd0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xd4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xd8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xdcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xe0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xe4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xe8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xecusize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xf0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xf4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xf8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x600usize + 0xfcusize)),
            ]
        }
    }

    #[doc = "Read Header Section 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rdhs1(&self) -> crate::common::Reg<self::Rdhs1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1792usize)) }
    }

    #[doc = "Read Header Section 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rdhs2(&self) -> crate::common::Reg<self::Rdhs2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1796usize)) }
    }

    #[doc = "Read Header Section 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rdhs3(&self) -> crate::common::Reg<self::Rdhs3_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1800usize)) }
    }

    #[doc = "Slot Counter Value\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn scv(&self) -> crate::common::Reg<self::Scv_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(272usize)) }
    }

    #[doc = "SYNC Frame Status\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sfs(&self) -> crate::common::Reg<self::Sfs_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(288usize)) }
    }

    #[doc = "Status Service Request Enable Reset\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sier(&self) -> crate::common::Reg<self::Sier_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "Status Service Request Enable Set\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sies(&self) -> crate::common::Reg<self::Sies_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }

    #[doc = "Status Service Request Line Select\n resetvalue={Application Reset:0x303FFFF}"]
    #[inline(always)]
    pub const fn sils(&self) -> crate::common::Reg<self::Sils_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }

    #[doc = "Status Service Request Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sir(&self) -> crate::common::Reg<self::Sir_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "Stop Watch Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn stpw1(&self) -> crate::common::Reg<self::Stpw1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(76usize)) }
    }

    #[doc = "Stop Watch Register 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn stpw2(&self) -> crate::common::Reg<self::Stpw2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }

    #[doc = "SUC Configuration Register 1\n resetvalue={Application Reset:0x0C401000}"]
    #[inline(always)]
    pub const fn succ1(&self) -> crate::common::Reg<self::Succ1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "SUC Configuration Register 2\n resetvalue={Application Reset:0x1000504}"]
    #[inline(always)]
    pub const fn succ2(&self) -> crate::common::Reg<self::Succ2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }

    #[doc = "SUC Configuration Register 3\n resetvalue={Application Reset:0x11}"]
    #[inline(always)]
    pub const fn succ3(&self) -> crate::common::Reg<self::Succ3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }

    #[doc = "Symbol Window and Network Idle Time Status\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn swnit(&self) -> crate::common::Reg<self::Swnit_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(292usize)) }
    }

    #[doc = "Timer 0 Configuration\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t0c(&self) -> crate::common::Reg<self::T0C_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }

    #[doc = "Timer 1 Configuration\n resetvalue={Application Reset:0x20000}"]
    #[inline(always)]
    pub const fn t1c(&self) -> crate::common::Reg<self::T1C_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }

    #[doc = "Test Register 1\n resetvalue={Application Reset:0x300}"]
    #[inline(always)]
    pub const fn test1(&self) -> crate::common::Reg<self::Test1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "Test Register 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn test2(&self) -> crate::common::Reg<self::Test2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Transmission Request Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn txrq1(&self) -> crate::common::Reg<self::Txrq1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(800usize)) }
    }

    #[doc = "Transmission Request Register 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn txrq2(&self) -> crate::common::Reg<self::Txrq2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(804usize)) }
    }

    #[doc = "Transmission Request Register 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn txrq3(&self) -> crate::common::Reg<self::Txrq3_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(808usize)) }
    }

    #[doc = "Transmission Request Register 4\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn txrq4(&self) -> crate::common::Reg<self::Txrq4_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(812usize)) }
    }

    #[doc = "Write Data Section 01\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn wrdsn(&self) -> [crate::common::Reg<self::WrdSn_SPEC, crate::common::RW>; 64] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x5cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x60usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x64usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x68usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x6cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x70usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x74usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x78usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x7cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x80usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x84usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x88usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x8cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x90usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x94usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x98usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0x9cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xa0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xa4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xa8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xacusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xb0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xb4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xb8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xbcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xc0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xc4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xc8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xccusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xd0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xd4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xd8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xdcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xe0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xe4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xe8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xecusize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xf0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xf4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xf8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x400usize + 0xfcusize)),
            ]
        }
    }

    #[doc = "Write Header Section 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn wrhs1(&self) -> crate::common::Reg<self::Wrhs1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1280usize)) }
    }

    #[doc = "Write Header Section 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn wrhs2(&self) -> crate::common::Reg<self::Wrhs2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1284usize)) }
    }

    #[doc = "Write Header Section 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn wrhs3(&self) -> crate::common::Reg<self::Wrhs3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1288usize)) }
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
pub struct Acs_SPEC;
impl crate::sealed::RegSpec for Acs_SPEC {
    type DataType = u32;
}
#[doc = "Aggregated Channel Status\n resetvalue={Application Reset:0x0}"]
pub type Acs = crate::RegValueT<Acs_SPEC>;

impl Acs {
    #[doc = "Valid Frame Received on Channel A vSS ValidFrameA    VFRA. One or more valid Frames were received on channel A in any static or dynamic slot during the observation period."]
    #[inline(always)]
    pub fn vfra(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, acs::Vfra, Acs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,acs::Vfra, Acs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Syntax Error Detected on Channel A vSS SyntaxErrorA    SEDA. One or more syntax errors in static or dynamic slots  symbol window  and network idle time  NIT  were observed on channel A."]
    #[inline(always)]
    pub fn seda(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, acs::Seda, Acs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,acs::Seda, Acs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Content Error Detected on Channel A vSS ContentErrorA    CEDA. One or more Frames with a content error were received on channel A in any static or dynamic slot during the observation period."]
    #[inline(always)]
    pub fn ceda(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, acs::Ceda, Acs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,acs::Ceda, Acs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Communication Indicator Channel A   CIA. One or more valid Frames were received on channel A in slots that also contained any additional communication during the observation period  i.e. one or more slots received a valid Frame AND had any combination of either syntax error OR content error OR slot boundary violation."]
    #[inline(always)]
    pub fn cia(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, acs::Cia, Acs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,acs::Cia, Acs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slot Boundary Violation on Channel A vSS BViolationA    SBVA. One or more slot boundary violations were observed on channel A at any time during the observation period  static or dynamic slots  symbol window  and network idle time NIT ."]
    #[inline(always)]
    pub fn sbva(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, acs::Sbva, Acs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,acs::Sbva, Acs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Valid Frame Received on Channel B vSS ValidFrameB    VFRB. One or more valid Frames were received on channel B in any static or dynamic slot during the observation period."]
    #[inline(always)]
    pub fn vfrb(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, acs::Vfrb, Acs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,acs::Vfrb, Acs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Syntax Error Detected on Channel B vSS SyntaxErrorB    SEDB. One or more syntax errors in static or dynamic slots  symbol window  and network idle time  NIT  were observed on channel B."]
    #[inline(always)]
    pub fn sedb(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, acs::Sedb, Acs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,acs::Sedb, Acs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Content Error Detected on Channel B vSS ContentErrorB    CEDB. One or more Frames with a content error were received on channel B in any static or dynamic slot during the observation period."]
    #[inline(always)]
    pub fn cedb(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, acs::Cedb, Acs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x1,1,0,acs::Cedb, Acs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Communication Indicator Channel B   CIB. One or more valid Frames were received on channel B in slots that also contained any additional communication during the observation period  i.e. one or more slots received a valid Frame AND had any combination of either syntax error OR content error OR slot boundary violation."]
    #[inline(always)]
    pub fn cib(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, acs::Cib, Acs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x1,1,0,acs::Cib, Acs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slot Boundary Violation on Channel B vSS BViolationB    SBVB. One or more slot boundary violations were observed on channel B at any time during the observation period  static or dynamic slots  symbol window  and network idle time NIT ."]
    #[inline(always)]
    pub fn sbvb(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, acs::Sbvb, Acs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x1,1,0,acs::Sbvb, Acs_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Acs {
    #[inline(always)]
    fn default() -> Acs {
        <crate::RegValueT<Acs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod acs {
    pub struct Vfra_SPEC;
    pub type Vfra = crate::EnumBitfieldStruct<u8, Vfra_SPEC>;
    impl Vfra {
        #[doc = "0 No valid Frame received"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Valid Frame s  received on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Seda_SPEC;
    pub type Seda = crate::EnumBitfieldStruct<u8, Seda_SPEC>;
    impl Seda {
        #[doc = "0 No syntax error observed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Syntax error s  observed on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ceda_SPEC;
    pub type Ceda = crate::EnumBitfieldStruct<u8, Ceda_SPEC>;
    impl Ceda {
        #[doc = "0 No Frame with content error received"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Frame s  with content error received on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cia_SPEC;
    pub type Cia = crate::EnumBitfieldStruct<u8, Cia_SPEC>;
    impl Cia {
        #[doc = "0 No valid Frame s  received in slots containing any additional communication"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Valid Frame s  received on channel A in slots containing any additional communication"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sbva_SPEC;
    pub type Sbva = crate::EnumBitfieldStruct<u8, Sbva_SPEC>;
    impl Sbva {
        #[doc = "0 No slot boundary violation observed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Slot boundary violation s  observed on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Vfrb_SPEC;
    pub type Vfrb = crate::EnumBitfieldStruct<u8, Vfrb_SPEC>;
    impl Vfrb {
        #[doc = "0 No valid Frame received"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Valid Frame s  received on channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sedb_SPEC;
    pub type Sedb = crate::EnumBitfieldStruct<u8, Sedb_SPEC>;
    impl Sedb {
        #[doc = "0 No syntax error observed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Syntax error s  observed on channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cedb_SPEC;
    pub type Cedb = crate::EnumBitfieldStruct<u8, Cedb_SPEC>;
    impl Cedb {
        #[doc = "0 No Frame with content error received"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Frame s  with content error received on channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cib_SPEC;
    pub type Cib = crate::EnumBitfieldStruct<u8, Cib_SPEC>;
    impl Cib {
        #[doc = "0 No valid Frame s  received in slots containing any additional communication"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Valid Frame s  received on channel B in slots containing any additional communication"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sbvb_SPEC;
    pub type Sbvb = crate::EnumBitfieldStruct<u8, Sbvb_SPEC>;
    impl Sbvb {
        #[doc = "0 No slot boundary violation observed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Slot boundary violation s  observed on channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccev_SPEC;
impl crate::sealed::RegSpec for Ccev_SPEC {
    type DataType = u32;
}
#[doc = "Communication Controller Error Vector\n resetvalue={Application Reset:0x0}"]
pub type Ccev = crate::RegValueT<Ccev_SPEC>;

impl Ccev {
    #[doc = "Clock Correction Failed Counter vClockCorrectionFailed    CCFC. The Clock Correction Failed Counter is incremented by one at the end of        any odd communication cycle where either the missing offset correction        error or missing rate correction error are active. The Clock Correction        Failed Counter is reset to 0 at the end of an odd communication cycle if        neither the offset correction failed nor the rate correction failed        errors are active. The Clock Correction Failed Counter stops at 15."]
    #[inline(always)]
    pub fn ccfc(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Ccev_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xf, 1, 0, u8, Ccev_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Error Mode vPOC ErrorMode    ERRM. Indicates the actual error mode of the POC."]
    #[inline(always)]
    pub fn errm(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, ccev::Errm, Ccev_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x3,1,0,ccev::Errm, Ccev_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Passive to Active Count vAllowPassiveToActive    PTAC. Indicates the number of consecutive even   odd cycle pairs that have passed with valid rate and offset correction terms  while the node is waiting to transit from  NORMAL PASSIVE  state to  NORMAL ACTIVE  state. The transition takes place when PTAC equals SUCC1.PTA."]
    #[inline(always)]
    pub fn ptac(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, Ccev_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, Ccev_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ccev {
    #[inline(always)]
    fn default() -> Ccev {
        <crate::RegValueT<Ccev_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccev {
    pub struct Errm_SPEC;
    pub type Errm = crate::EnumBitfieldStruct<u8, Errm_SPEC>;
    impl Errm {
        #[doc = "00  ACTIVE   green"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01  PASSIVE   yellow"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10  COMM HALT   red"]
        pub const CONST_22: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccsv_SPEC;
impl crate::sealed::RegSpec for Ccsv_SPEC {
    type DataType = u32;
}
#[doc = "Communication Controller Status Vector\n resetvalue={Application Reset:0x104000}"]
pub type Ccsv = crate::RegValueT<Ccsv_SPEC>;

impl Ccsv {
    #[doc = "Freeze Status Indicator vPOC Freeze    FSI. Indicates that the POC has entered the   8220 HALT  8221  state due to CHI command          8220 FREEZE  8221  or due to an error condition requiring an immediate POC halt.        Reset by transition from   8220 HALT  8221  to   8220 DEFAULT CONFIG  8221  state."]
    #[inline(always)]
    pub fn fsi(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ccsv_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ccsv_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Halt Request vPOC CHIHaltRequest    HRQ. Indicates that a request from the Host has been received to halt the POC        at the end of the communication cycle. Reset by transition from   8220 HALT  8221         to   8220 DEFAULT CONFIG  8221  state or when entering   8220 READY  8221  state."]
    #[inline(always)]
    pub fn hrq(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ccsv_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ccsv_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Slot Mode vPOC SlotMode    SLM. Indicates the actual slot mode of the POC in states READY  WAKEUP         STARTUP  NORMAL ACTIVE  and NORMAL PASSIVE. Default is   8220 SINGLE  8221 . Changes        to   8220 ALL  8221   depending on configuration bit SUCC1.TSM. In   8220 NORMAL ACTIVE  8221         or   8220 NORMAL PASSIVE  8221  state the CHI command   8220 ALL SLOTS  8221  will change the        slot mode from   8220 SINGLE  8221  over   8220 ALL PENDING  8221  to   8220 ALL  8221 . Set to SINGLE in        all other states."]
    #[inline(always)]
    pub fn slm(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, ccsv::Slm, Ccsv_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3,1,0,ccsv::Slm, Ccsv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Coldstart Noise Indicator vPOC ColdstartNoise    CSNI. Indicates that the cold start procedure occurred under noisy conditions.        Reset by CHI command   8220 RESET STATUS INDICATORS  8221  or by transition from          8220 HALT  8221  to   8220 DEFAULT CONFIG  8221  state or from   8220 READY  8221  to   8220 STARTUP  8221  state."]
    #[inline(always)]
    pub fn csni(self) -> crate::common::RegisterFieldBool<12, 1, 0, Ccsv_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ccsv_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Coldstart Abort Indicator   CSAI. Coldstart aborted. Reset by CHI command   8220 RESET STATUS INDICATORS  8221  or by        transition from   8220 HALT  8221  to   8220 DEFAULT CONFIG  8221  state or from   8220 READY  8221  to          8220 STARTUP  8221  state."]
    #[inline(always)]
    pub fn csai(self) -> crate::common::RegisterFieldBool<13, 1, 0, Ccsv_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ccsv_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Cold Start Inhibit vColdStartInhibit    CSI. Indicates that the node is disabled from cold starting. The flag is set        whenever the POC enters   8220 READY  8221  state due to CHI command   8220 READY  8221 . The        flag has to be reset under control of the Host by CHI command          8220 ALLOW COLDSTART  8221   SUCC1.CMD  160    160 1001 B  ."]
    #[inline(always)]
    pub fn csi(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, ccsv::Csi, Ccsv_SPEC, crate::common::R> {
        crate::common::RegisterField::<14,0x1,1,0,ccsv::Csi, Ccsv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Wakeup Status vPOC WakeupStatus    WSV. Indicates the status of the current wakeup attempt. Reset by CHI command          8220 RESET STATUS INDICATORS  8221  or by transition from   8220 HALT  8221  to          8220 DEFAULT CONFIG  8221  state."]
    #[inline(always)]
    pub fn wsv(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, ccsv::Wsv, Ccsv_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7,1,0,ccsv::Wsv, Ccsv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Remaining Coldstart Attempts vRemainingColdstartAttempts    RCA. Indicates the number of remaining coldstart attempts. The RUN command        resets this counter to the maximum number of coldstart attempts as        configured by SUCC1.CSA."]
    #[inline(always)]
    pub fn rca(
        self,
    ) -> crate::common::RegisterField<19, 0x1f, 1, 0, u8, Ccsv_SPEC, crate::common::R> {
        crate::common::RegisterField::<19,0x1f,1,0,u8, Ccsv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "POC Status Log   PSL. Status of CCSV.POCS immediately before entering   8220 HALT  8221  state. Set when        entering   8220 HALT  8221  state. Set to   8220 HALT  8221  when FREEZE command is applied        during   8220 HALT  8221  state. Reset to 000000 B when leaving   8220 HALT  8221  state."]
    #[inline(always)]
    pub fn psl(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, Ccsv_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x3f,1,0,u8, Ccsv_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ccsv {
    #[inline(always)]
    fn default() -> Ccsv {
        <crate::RegValueT<Ccsv_SPEC> as RegisterValue<_>>::new(1064960)
    }
}
pub mod ccsv {
    pub struct Slm_SPEC;
    pub type Slm = crate::EnumBitfieldStruct<u8, Slm_SPEC>;
    impl Slm {
        #[doc = "00 SINGLE"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 ALL PENDING"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 ALL"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Csi_SPEC;
    pub type Csi = crate::EnumBitfieldStruct<u8, Csi_SPEC>;
    impl Csi {
        #[doc = "0 Cold starting of node enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Cold starting of node disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wsv_SPEC;
    pub type Wsv = crate::EnumBitfieldStruct<u8, Wsv_SPEC>;
    impl Wsv {
        #[doc = "000 UNDEFINED. Wakeup not yet executed by the Communication Controller."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 RECEIVED HEADER. Set when the Communication Controller finishes wakeup due to the reception of a Frame Header without coding violation on either channel in  WAKEUP LISTEN  state."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 RECEIVED WUP. Set when the Communication Controller finishes wakeup due to the reception of a valid wakeup pattern on the configured wakeup channel in  WAKEUP LISTEN  state."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 COLLISION HEADER. Set when the Communication Controller stops wakeup due to a detected collision during wakeup pattern transmission by receiving a valid Header on either channel."]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 COLLISION WUP. Set when the Communication Controller stops wakeup due to a detected collision during wakeup pattern transmission by receiving a valid wakeup pattern on the configured wakeup channel."]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 COLLISION UNKNOWN. Set when the Communication Controller stops wakeup by leaving  WAKEUP DETECT  state after expiration of the wakeup timer without receiving a valid wakeup pattern or a valid Frame Header."]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 TRANSMITTED. Set when the Communication Controller has successfully completed the transmission of the wakeup pattern."]
        pub const CONST_66: Self = Self::new(6);
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
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control of the module. This bit disables the kernel clocks f CLC ERAY and the sampling clock f SCLK ."]
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
    #[doc = "External Sleep Mode Request Disable Bit   EDIS. Used to control module  8217 s sleep mode. If this bit is cleared the kernel clock f CLC ERAY and the sampling clock f SCLK are disabled during System Sleep Mode."]
    #[inline(always)]
    pub fn edis(self) -> crate::common::RegisterFieldBool<3, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clock Divider in Run Mode   RMC. This bit field is not affected by an application reset. This bit field only controls the kernel clock f CLC ERAY and not the sampling clock f SCLK ."]
    #[inline(always)]
    pub fn rmc(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, clc::Rmc, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,clc::Rmc, Clc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Clc {
    #[inline(always)]
    fn default() -> Clc {
        <crate::RegValueT<Clc_SPEC> as RegisterValue<_>>::new(3)
    }
}
pub mod clc {
    pub struct Rmc_SPEC;
    pub type Rmc = crate::EnumBitfieldStruct<u8, Rmc_SPEC>;
    impl Rmc {
        #[doc = "000 No clock signal f CLC ERAY generated  default after reset"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 Clock f CLC ERAY   f SPB selected"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 Clock f CLC ERAY   f SPB  2 selected"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "100 Clock f CLC ERAY   f SPB  4 selected"]
        pub const CONST_44: Self = Self::new(4);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crel_SPEC;
impl crate::sealed::RegSpec for Crel_SPEC {
    type DataType = u32;
}
#[doc = "Core Release Register\n resetvalue={Application Reset:0x0}"]
pub type Crel = crate::RegValueT<Crel_SPEC>;

impl Crel {
    #[doc = "Design Time Stamp  Day   DAY. Two digits  BCD coded."]
    #[inline(always)]
    pub fn day(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Crel_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Crel_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Design Time Stamp  Month   MON. Two digits  BCD coded."]
    #[inline(always)]
    pub fn mon(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Crel_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Crel_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Design Time Stamp  Year   YEAR. One digit  BCD coded."]
    #[inline(always)]
    pub fn year(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Crel_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Crel_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Sub Step of Core Release   SUBSTEP. One digits  BCD coded."]
    #[inline(always)]
    pub fn substep(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, crel::Substep, Crel_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<20,0xf,1,0,crel::Substep, Crel_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Step of Core Release   STEP. One digits  BCD coded."]
    #[inline(always)]
    pub fn step(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, crel::Step, Crel_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xf,1,0,crel::Step, Crel_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Core Release   REL. One digit  BCD coded."]
    #[inline(always)]
    pub fn rel(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, crel::Rel, Crel_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0xf,1,0,crel::Rel, Crel_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Crel {
    #[inline(always)]
    fn default() -> Crel {
        <crate::RegValueT<Crel_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod crel {
    pub struct Substep_SPEC;
    pub type Substep = crate::EnumBitfieldStruct<u8, Substep_SPEC>;
    impl Substep {
        #[doc = "0 Alpha         pre Beta  pre Beta update  pre Beta2  pre Beta2 update  Beta  Beta2         Revision 1.0.0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Beta ct         Beta ct fix1  Revision 1.0.1"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Revision1.0RC1 Beta ct fix2  REVISION 1.0RC1"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Step_SPEC;
    pub type Step = crate::EnumBitfieldStruct<u8, Step_SPEC>;
    impl Step {
        #[doc = "0 Revision 1.0.0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Alpha"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 pre Beta"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 pre Beta update"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "4 pre Beta2"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "5 pre Beta2 update"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "6 Beta"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "7 Beta2"]
        pub const CONST_77: Self = Self::new(7);
    }
    pub struct Rel_SPEC;
    pub type Rel = crate::EnumBitfieldStruct<u8, Rel_SPEC>;
    impl Rel {
        #[doc = "0 alpha beta2ct"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Revision 1.0"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cust1_SPEC;
impl crate::sealed::RegSpec for Cust1_SPEC {
    type DataType = u32;
}
#[doc = "Busy and Input Buffer Control Register\n resetvalue={Application Reset:0x0}"]
pub type Cust1 = crate::RegValueT<Cust1_SPEC>;

impl Cust1 {
    #[doc = "CIF Timeout Service Request Status   INT0. INT0 will be set if a timeout has occurred during the auto delay scheme        and must be reset by writing zero to INT0. Software can also set this        bit field. In case hardware sets INT0 and at the same point of time software          clears INT0  INT0 is cleared."]
    #[inline(always)]
    pub fn int0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cust1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cust1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable auto delay scheme for Output Buffer Control Register  OBCR    OEN. This control bit controls the delay scheme for Output Buffer Control        Register  OBCR  read accesses."]
    #[inline(always)]
    pub fn oen(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, cust1::Oen, Cust1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,cust1::Oen, Cust1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable auto delay scheme for Input Buffer Control Register  IBCR    IEN. This control bit controls the auto delay scheme for Input Buffer Control Register  IBCR  read accesses."]
    #[inline(always)]
    pub fn ien(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, cust1::Ien, Cust1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,cust1::Ien, Cust1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Buffer Status Register   IBFS. This status bit  eray mhd signal  ifb ifb1 2n  indicates which of the two Input Buffer RAMs  IBF  is accessible by the host  via CIF  as Input Buffer. The other non accessible buffer RAM is currently used as shadow buffer RAM by the ERAY message handler and therefore not accessible by the host. After reset  it is set by hardware."]
    #[inline(always)]
    pub fn ibfs(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, cust1::Ibfs, Cust1_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1,1,0,cust1::Ibfs, Cust1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Input Buffer 1 Page Select Register   IBF1PAG. This control bit selects if the upper page or lower page of Input Buffer 1  IBF1  currently active. Write is only possible  if Input Buffer RAM 1 is currently accessible by the host  via CIF  and therefore IBFS set."]
    #[inline(always)]
    pub fn ibf1pag(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, cust1::Ibf1Pag, Cust1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,cust1::Ibf1Pag, Cust1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Buffer 2 Page Select Register   IBF2PAG. This control bit selects if the upper page or lower page of Input Buffer 2  IBF2  currently active. Write is only possible  if Input Buffer RAM 2 is currently accessible by the host  via CIF  and therefore IBFS cleared."]
    #[inline(always)]
    pub fn ibf2pag(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, cust1::Ibf2Pag, Cust1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,cust1::Ibf2Pag, Cust1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive Input Select Channel A   RISA"]
    #[inline(always)]
    pub fn risa(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, cust1::Risa, Cust1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,cust1::Risa, Cust1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive Input Select Channel B   RISB"]
    #[inline(always)]
    pub fn risb(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, cust1::Risb, Cust1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,cust1::Risb, Cust1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Stop Watch Trigger Input Select   STPWTS"]
    #[inline(always)]
    pub fn stpwts(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, cust1::Stpwts, Cust1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,cust1::Stpwts, Cust1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Cust1 {
    #[inline(always)]
    fn default() -> Cust1 {
        <crate::RegValueT<Cust1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cust1 {
    pub struct Oen_SPEC;
    pub type Oen = crate::EnumBitfieldStruct<u8, Oen_SPEC>;
    impl Oen {
        #[doc = "0 Disable auto delay scheme for Output Buffer Control Register  OBCR"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enable auto delay scheme for Output Buffer Control Register  OBCR"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ien_SPEC;
    pub type Ien = crate::EnumBitfieldStruct<u8, Ien_SPEC>;
    impl Ien {
        #[doc = "0 Disable auto delay scheme for Input Buffer Control Register  IBCR"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enable auto delay scheme for Input Buffer Control Register  IBCR"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ibfs_SPEC;
    pub type Ibfs = crate::EnumBitfieldStruct<u8, Ibfs_SPEC>;
    impl Ibfs {
        #[doc = "0 Input Buffer RAM 2  IBF2  is accessible as Input Buffer by the host  CIF"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Input Buffer RAM 1  IBF1  is accessible as Input Buffer by the host  CIF"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ibf1Pag_SPEC;
    pub type Ibf1Pag = crate::EnumBitfieldStruct<u8, Ibf1Pag_SPEC>;
    impl Ibf1Pag {
        #[doc = "0 Read  Lower Page  256 Bytes  of Input Buffer RAM 1 selected Write  Select Lower Page  256 Bytes  of Input Buffer RAM 1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Upper Page  256 Bytes  of Input Buffer RAM 1 selected Write  Select Upper Page  256 Bytes  of Input Buffer RAM 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ibf2Pag_SPEC;
    pub type Ibf2Pag = crate::EnumBitfieldStruct<u8, Ibf2Pag_SPEC>;
    impl Ibf2Pag {
        #[doc = "0 Read  Lower Page  256 Bytes  of Input Buffer RAM 2 selected Write  Select Lower Page  256 Bytes  of Input Buffer RAM 2"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Upper Page  256 Bytes  of Input Buffer RAM 2 selected Write  Select Upper Page  256 Byte  of Input Buffer RAM 2"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Risa_SPEC;
    pub type Risa = crate::EnumBitfieldStruct<u8, Risa_SPEC>;
    impl Risa {
        #[doc = "00 Channel A receiver input RXDA0 selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Channel A receiver input RXDA1 selected"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Channel A receiver input RXDA2 selected"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Channel A receiver input RXDA3 selected"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Risb_SPEC;
    pub type Risb = crate::EnumBitfieldStruct<u8, Risb_SPEC>;
    impl Risb {
        #[doc = "00 Channel B receiver input RXDB0 selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Channel B receiver input RXDB1 selected"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Channel B receiver input RXDB2 selected"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Channel B receiver input RXDB3 selected"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Stpwts_SPEC;
    pub type Stpwts = crate::EnumBitfieldStruct<u8, Stpwts_SPEC>;
    impl Stpwts {
        #[doc = "00 Stop Watch Trigger input STPWT0 selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Stop Watch Trigger input STPWT1 selected"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Stop Watch Trigger input STPWT2 selected"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Stop Watch Trigger input STPWT3 selected"]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cust3_SPEC;
impl crate::sealed::RegSpec for Cust3_SPEC {
    type DataType = u32;
}
#[doc = "Customer Interface Timeout Counter Register\n resetvalue={Application Reset:0x0}"]
pub type Cust3 = crate::RegValueT<Cust3_SPEC>;

impl Cust3 {
    #[doc = "CIF Timeout Reload Value   TO. The 32 bit down counter reload  start up  value must be setup for the automatic delay scheme."]
    #[inline(always)]
    pub fn to(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Cust3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Cust3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Cust3 {
    #[inline(always)]
    fn default() -> Cust3 {
        <crate::RegValueT<Cust3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eier_SPEC;
impl crate::sealed::RegSpec for Eier_SPEC {
    type DataType = u32;
}
#[doc = "Error Service Request Enable Reset\n resetvalue={Application Reset:0x0}"]
pub type Eier = crate::RegValueT<Eier_SPEC>;

impl Eier {
    #[doc = "POC Error Mode Changed Service Request Enable   PEMCE"]
    #[inline(always)]
    pub fn pemce(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, eier::Pemce, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,eier::Pemce, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Command Not Accepted Service Request Enable   CNAE"]
    #[inline(always)]
    pub fn cnae(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, eier::Cnae, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,eier::Cnae, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SYNC Frames Below Minimum Service Request Enable   SFBME"]
    #[inline(always)]
    pub fn sfbme(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, eier::Sfbme, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,eier::Sfbme, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SYNC Frame Overflow Service Request Enable   SFOE"]
    #[inline(always)]
    pub fn sfoe(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, eier::Sfoe, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,eier::Sfoe, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Correction Failure Service Request Enable   CCFE"]
    #[inline(always)]
    pub fn ccfe(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, eier::Ccfe, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,eier::Ccfe, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CHI Command Locked Service Request Enable   CCLE"]
    #[inline(always)]
    pub fn ccle(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, eier::Ccle, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,eier::Ccle, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ECC Error Service Request Enable   EERRE"]
    #[inline(always)]
    pub fn eerre(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, eier::Eerre, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,eier::Eerre, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive FIFO Overrun Service Request Enable   RFOE"]
    #[inline(always)]
    pub fn rfoe(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, eier::Rfoe, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,eier::Rfoe, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Empty FIFO Access Service Request Enable   EFAE"]
    #[inline(always)]
    pub fn efae(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, eier::Efae, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,eier::Efae, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Illegal Input Buffer Access Service Request Enable   IIBAE"]
    #[inline(always)]
    pub fn iibae(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, eier::Iibae, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,eier::Iibae, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Illegal Output Buffer Access Service Request Enable   IOBAE"]
    #[inline(always)]
    pub fn iobae(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, eier::Iobae, Eier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,eier::Iobae, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Handler Constraints Flag Service Request Enable   MHFE"]
    #[inline(always)]
    pub fn mhfe(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, eier::Mhfe, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x1,1,0,eier::Mhfe, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Detected on Channel A Service Request Enable   EDAE"]
    #[inline(always)]
    pub fn edae(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, eier::Edae, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1,1,0,eier::Edae, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Latest Transmit Violation Channel A Service Request Enable   LTVAE"]
    #[inline(always)]
    pub fn ltvae(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, eier::Ltvae, Eier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,eier::Ltvae, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmission Across Boundary Channel A Service Request Enable   TABAE"]
    #[inline(always)]
    pub fn tabae(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, eier::Tabae, Eier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,eier::Tabae, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Detected on Channel B Service Request Enable   EDBE"]
    #[inline(always)]
    pub fn edbe(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, eier::Edbe, Eier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1,1,0,eier::Edbe, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Latest Transmit Violation Channel B Service Request Enable   LTVBE"]
    #[inline(always)]
    pub fn ltvbe(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, eier::Ltvbe, Eier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,eier::Ltvbe, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmission Across Boundary Channel B Service Request Enable   TABBE"]
    #[inline(always)]
    pub fn tabbe(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, eier::Tabbe, Eier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,eier::Tabbe, Eier_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Eier {
    #[inline(always)]
    fn default() -> Eier {
        <crate::RegValueT<Eier_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eier {
    pub struct Pemce_SPEC;
    pub type Pemce = crate::EnumBitfieldStruct<u8, Pemce_SPEC>;
    impl Pemce {
        #[doc = "0 Read  Protocol Error Mode Changed Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Protocol Error Mode Changed Service Request enabled Write  Disable Protocol Error Mode Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cnae_SPEC;
    pub type Cnae = crate::EnumBitfieldStruct<u8, Cnae_SPEC>;
    impl Cnae {
        #[doc = "0 Read  Command Not Accepted Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Command Not Accepted Service Request enabled Write  Disable Command Not Accepted Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sfbme_SPEC;
    pub type Sfbme = crate::EnumBitfieldStruct<u8, Sfbme_SPEC>;
    impl Sfbme {
        #[doc = "0 Read  SYNC Frames Below Minimum Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  SYNC Frames Below Minimum Service Request enabled Write  Disable SYNC Frames Below Minimum Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sfoe_SPEC;
    pub type Sfoe = crate::EnumBitfieldStruct<u8, Sfoe_SPEC>;
    impl Sfoe {
        #[doc = "0 Read  SYNC Frame Overflow Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  SYNC Frame Overflow Service Request enabled Write  Disable Protocol Error Mode Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ccfe_SPEC;
    pub type Ccfe = crate::EnumBitfieldStruct<u8, Ccfe_SPEC>;
    impl Ccfe {
        #[doc = "0 Read  Clock Correction Failure Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Clock Correction Failure Service Request enabled Write  Disable Clock Correction Failure Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ccle_SPEC;
    pub type Ccle = crate::EnumBitfieldStruct<u8, Ccle_SPEC>;
    impl Ccle {
        #[doc = "0 Read  CHI Command Locked Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  CHI Command Locked Service Request enabled Write  Disable CHI Command Locked Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eerre_SPEC;
    pub type Eerre = crate::EnumBitfieldStruct<u8, Eerre_SPEC>;
    impl Eerre {
        #[doc = "0 Read  ECC Error Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ERead  CC Error Service Request enabled Write  Disable ECC Error Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfoe_SPEC;
    pub type Rfoe = crate::EnumBitfieldStruct<u8, Rfoe_SPEC>;
    impl Rfoe {
        #[doc = "0 Read  Receive FIFO Overrun Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Receive FIFO Overrun Service Request enabled Write  Disable Receive FIFO Overrun Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Efae_SPEC;
    pub type Efae = crate::EnumBitfieldStruct<u8, Efae_SPEC>;
    impl Efae {
        #[doc = "0 Read  Empty FIFO Access Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Empty FIFO Access Service Request enabled Write  Disable Empty FIFO Access Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Iibae_SPEC;
    pub type Iibae = crate::EnumBitfieldStruct<u8, Iibae_SPEC>;
    impl Iibae {
        #[doc = "0 Read  Illegal Input Buffer Access Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Illegal Input Buffer Access Service Request enabled Write  Disable Illegal Input Buffer Access Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Iobae_SPEC;
    pub type Iobae = crate::EnumBitfieldStruct<u8, Iobae_SPEC>;
    impl Iobae {
        #[doc = "0 Read  Illegal Output Buffer Access Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Illegal Output Buffer Access Service Request enabled Write  Disable Illegal Output Buffer Access Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mhfe_SPEC;
    pub type Mhfe = crate::EnumBitfieldStruct<u8, Mhfe_SPEC>;
    impl Mhfe {
        #[doc = "0 Read  Message Handler Constraints Flag Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Message Handler Constraints Flag Service Request enabled Write  Disable Message Handler Constraints Flag Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Edae_SPEC;
    pub type Edae = crate::EnumBitfieldStruct<u8, Edae_SPEC>;
    impl Edae {
        #[doc = "0 Read  Error Detected on Channel A Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Error Detected on Channel A Service Request enabled Write  Disable Error Detected on Channel A Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ltvae_SPEC;
    pub type Ltvae = crate::EnumBitfieldStruct<u8, Ltvae_SPEC>;
    impl Ltvae {
        #[doc = "0 Read  Latest Transmit Violation Channel A Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Latest Transmit Violation Channel A Service Request enabled Write  Disable Latest Transmit Violation Channel A Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tabae_SPEC;
    pub type Tabae = crate::EnumBitfieldStruct<u8, Tabae_SPEC>;
    impl Tabae {
        #[doc = "0 Read  Transmission Across Boundary Channel A Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Transmission Across Boundary Channel A Service Request enabled Write  Enable Transmission Across Boundary Channel A Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Edbe_SPEC;
    pub type Edbe = crate::EnumBitfieldStruct<u8, Edbe_SPEC>;
    impl Edbe {
        #[doc = "0 Read  Error Detected on Channel B Service Request disabled Write  Unchange"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Error Detected on Channel B Service Request enabled Write  Disable Error Detected on Channel B Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ltvbe_SPEC;
    pub type Ltvbe = crate::EnumBitfieldStruct<u8, Ltvbe_SPEC>;
    impl Ltvbe {
        #[doc = "0 Read  Latest Transmit Violation Channel B Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Latest Transmit Violation Channel B Service Request enabled Write  Disable Latest Transmit Violation Channel B Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tabbe_SPEC;
    pub type Tabbe = crate::EnumBitfieldStruct<u8, Tabbe_SPEC>;
    impl Tabbe {
        #[doc = "0 Read  Transmission Across Boundary Channel B Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Transmission Across Boundary Channel B Service Request enabled Write  Disable Transmission Across Boundary Channel B Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eies_SPEC;
impl crate::sealed::RegSpec for Eies_SPEC {
    type DataType = u32;
}
#[doc = "Error Service Request Enable Set\n resetvalue={Application Reset:0x0}"]
pub type Eies = crate::RegValueT<Eies_SPEC>;

impl Eies {
    #[doc = "POC Error Mode Changed Service Request Enable   PEMCE"]
    #[inline(always)]
    pub fn pemce(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, eies::Pemce, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,eies::Pemce, Eies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Command Not Accepted Service Request Enable   CNAE"]
    #[inline(always)]
    pub fn cnae(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, eies::Cnae, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,eies::Cnae, Eies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SYNC Frames Below Minimum Service Request Enable   SFBME"]
    #[inline(always)]
    pub fn sfbme(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, eies::Sfbme, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,eies::Sfbme, Eies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SYNC Frame Overflow Service Request Enable   SFOE"]
    #[inline(always)]
    pub fn sfoe(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, eies::Sfoe, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,eies::Sfoe, Eies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Correction Failure Service Request Enable   CCFE"]
    #[inline(always)]
    pub fn ccfe(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, eies::Ccfe, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,eies::Ccfe, Eies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CHI Command Locked Service Request Enable   CCLE"]
    #[inline(always)]
    pub fn ccle(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, eies::Ccle, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,eies::Ccle, Eies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ECC Error Service Request Enable   EERRE"]
    #[inline(always)]
    pub fn eerre(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, eies::Eerre, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,eies::Eerre, Eies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive FIFO Overrun Service Request Enable   RFOE"]
    #[inline(always)]
    pub fn rfoe(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, eies::Rfoe, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,eies::Rfoe, Eies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Empty FIFO Access Service Request Enable   EFAE"]
    #[inline(always)]
    pub fn efae(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, eies::Efae, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,eies::Efae, Eies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Illegal Input Buffer Access Service Request Enable   IIBAE"]
    #[inline(always)]
    pub fn iibae(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, eies::Iibae, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,eies::Iibae, Eies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Illegal Output Buffer Access Service Request Enable   IOBAE"]
    #[inline(always)]
    pub fn iobae(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, eies::Iobae, Eies_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,eies::Iobae, Eies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Handler Constraints Flag Service Request Enable   MHFE"]
    #[inline(always)]
    pub fn mhfe(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, eies::Mhfe, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x1,1,0,eies::Mhfe, Eies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Detected on Channel A Service Request Enable   EDAE"]
    #[inline(always)]
    pub fn edae(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, eies::Edae, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1,1,0,eies::Edae, Eies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Latest Transmit Violation Channel A Service Request Enable   LTVAE"]
    #[inline(always)]
    pub fn ltvae(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, eies::Ltvae, Eies_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,eies::Ltvae, Eies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmission Across Boundary Channel A Service Request Enable   TABAE"]
    #[inline(always)]
    pub fn tabae(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, eies::Tabae, Eies_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,eies::Tabae, Eies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Detected on Channel B Service Request Enable   EDBE"]
    #[inline(always)]
    pub fn edbe(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, eies::Edbe, Eies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1,1,0,eies::Edbe, Eies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Latest Transmit Violation Channel B Service Request Enable   LTVBE"]
    #[inline(always)]
    pub fn ltvbe(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, eies::Ltvbe, Eies_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,eies::Ltvbe, Eies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmission Across Boundary Channel B Service Request Enable   TABBE"]
    #[inline(always)]
    pub fn tabbe(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, eies::Tabbe, Eies_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,eies::Tabbe, Eies_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Eies {
    #[inline(always)]
    fn default() -> Eies {
        <crate::RegValueT<Eies_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eies {
    pub struct Pemce_SPEC;
    pub type Pemce = crate::EnumBitfieldStruct<u8, Pemce_SPEC>;
    impl Pemce {
        #[doc = "0 Read  Protocol Error Mode Changed Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Protocol Error Mode Changed Service Request enabled Write  Enable Protocol Error Mode Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cnae_SPEC;
    pub type Cnae = crate::EnumBitfieldStruct<u8, Cnae_SPEC>;
    impl Cnae {
        #[doc = "0 Read  Command Not Valid Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Command Not Valid Service Request enabled Write  Enable Command Not Valid Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sfbme_SPEC;
    pub type Sfbme = crate::EnumBitfieldStruct<u8, Sfbme_SPEC>;
    impl Sfbme {
        #[doc = "0 Read  SYNC Frames Below Minimum Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  SYNC Frames Below Minimum Service Request enabled Write  Enable SYNC Frames Below Minimum Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sfoe_SPEC;
    pub type Sfoe = crate::EnumBitfieldStruct<u8, Sfoe_SPEC>;
    impl Sfoe {
        #[doc = "0 Read  SYNC Frame Overflow Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  SYNC Frame Overflow Service Request enabled Write  Enable Protocol Error Mode Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ccfe_SPEC;
    pub type Ccfe = crate::EnumBitfieldStruct<u8, Ccfe_SPEC>;
    impl Ccfe {
        #[doc = "0 Read  Clock Correction Failure Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Clock Correction Failure Service Request enabled Write  Enable Clock Correction Failure Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ccle_SPEC;
    pub type Ccle = crate::EnumBitfieldStruct<u8, Ccle_SPEC>;
    impl Ccle {
        #[doc = "0 Read  CHI Command Locked Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  CHI Command Locked Service Request enabled Write  Enable CHI Command Locked Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eerre_SPEC;
    pub type Eerre = crate::EnumBitfieldStruct<u8, Eerre_SPEC>;
    impl Eerre {
        #[doc = "0 Read  ECC Error Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  ECC Error Service Request enabled Enable ECC Error Service Request Write  Unchanged"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfoe_SPEC;
    pub type Rfoe = crate::EnumBitfieldStruct<u8, Rfoe_SPEC>;
    impl Rfoe {
        #[doc = "0 Read  Receive FIFO Overrun Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Receive FIFO Overrun Service Request enabled Write  Enable Receive FIFO Overrun Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Efae_SPEC;
    pub type Efae = crate::EnumBitfieldStruct<u8, Efae_SPEC>;
    impl Efae {
        #[doc = "0 Read  Empty FIFO Access Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Empty FIFO Access Service Request enabled Write  Enable Empty FIFO Access Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Iibae_SPEC;
    pub type Iibae = crate::EnumBitfieldStruct<u8, Iibae_SPEC>;
    impl Iibae {
        #[doc = "0 Read  Illegal Input Buffer Access Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Illegal Input Buffer Access Service Request enabled Write  Enable Illegal Input Buffer Access Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Iobae_SPEC;
    pub type Iobae = crate::EnumBitfieldStruct<u8, Iobae_SPEC>;
    impl Iobae {
        #[doc = "0 Read  Illegal Output Buffer Access Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Illegal Output Buffer Access Service Request enabled Write  Enable Illegal Output Buffer Access Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mhfe_SPEC;
    pub type Mhfe = crate::EnumBitfieldStruct<u8, Mhfe_SPEC>;
    impl Mhfe {
        #[doc = "0 Read  Message Handler Constraints Flag Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Message Handler Constraints Flag Service Request enabled Write  Enable Message Handler Constraints Flag Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Edae_SPEC;
    pub type Edae = crate::EnumBitfieldStruct<u8, Edae_SPEC>;
    impl Edae {
        #[doc = "0 Read  Error Detected on Channel A Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Error Detected on Channel A Service Request enabled Write  Enable Error Detected on Channel A Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ltvae_SPEC;
    pub type Ltvae = crate::EnumBitfieldStruct<u8, Ltvae_SPEC>;
    impl Ltvae {
        #[doc = "0 Read  Latest Transmit Violation Channel A Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Latest Transmit Violation Channel A Service Request enabled  Write  Enable Latest Transmit Violation Channel A Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tabae_SPEC;
    pub type Tabae = crate::EnumBitfieldStruct<u8, Tabae_SPEC>;
    impl Tabae {
        #[doc = "0 Read  Transmission Across Boundary Channel A Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Transmission Across Boundary Channel A Service Request enabled Write  Enable Transmission Across Boundary Channel A Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Edbe_SPEC;
    pub type Edbe = crate::EnumBitfieldStruct<u8, Edbe_SPEC>;
    impl Edbe {
        #[doc = "0 Read  Error Detected on Channel B Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Error Detected on Channel B Service Request enabled Write  Enable Error Detected on Channel B Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ltvbe_SPEC;
    pub type Ltvbe = crate::EnumBitfieldStruct<u8, Ltvbe_SPEC>;
    impl Ltvbe {
        #[doc = "0 Read  Latest Transmit Violation Channel B Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Latest Transmit Violation Channel B Service Request enabled Write  Enable Latest Transmit Violation Channel B Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tabbe_SPEC;
    pub type Tabbe = crate::EnumBitfieldStruct<u8, Tabbe_SPEC>;
    impl Tabbe {
        #[doc = "0 Read  Transmission Across Boundary Channel B Service Request disabled Write  Enable Transmission Across Boundary Channel B Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Transmission Across Boundary Channel B Service Request enabled Write  Enable Transmission Across Boundary Channel B Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eils_SPEC;
impl crate::sealed::RegSpec for Eils_SPEC {
    type DataType = u32;
}
#[doc = "Error Service Request Line Select\n resetvalue={Application Reset:0x0}"]
pub type Eils = crate::RegValueT<Eils_SPEC>;

impl Eils {
    #[doc = "POC Error Mode Changed Service Request Line   PEMCL"]
    #[inline(always)]
    pub fn pemcl(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, eils::Pemcl, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,eils::Pemcl, Eils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Command Not Accepted Service Request Line   CNAL"]
    #[inline(always)]
    pub fn cnal(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, eils::Cnal, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,eils::Cnal, Eils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SYNC Frames Below Minimum Service Request Line   SFBML"]
    #[inline(always)]
    pub fn sfbml(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, eils::Sfbml, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,eils::Sfbml, Eils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SYNC Frame Overflow Service Request Line   SFOL"]
    #[inline(always)]
    pub fn sfol(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, eils::Sfol, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,eils::Sfol, Eils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Correction Failure Service Request Line   CCFL"]
    #[inline(always)]
    pub fn ccfl(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, eils::Ccfl, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,eils::Ccfl, Eils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CHI Command Locked Service Request Line   CCLL"]
    #[inline(always)]
    pub fn ccll(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, eils::Ccll, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,eils::Ccll, Eils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ECC Error Service Request Line   EERRL"]
    #[inline(always)]
    pub fn eerrl(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, eils::Eerrl, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,eils::Eerrl, Eils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive FIFO Overrun Service Request Line   RFOL"]
    #[inline(always)]
    pub fn rfol(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, eils::Rfol, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,eils::Rfol, Eils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Empty FIFO Access Service Request Line   EFAL"]
    #[inline(always)]
    pub fn efal(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, eils::Efal, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,eils::Efal, Eils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Illegal Input Buffer Access Service Request Line   IIBAL"]
    #[inline(always)]
    pub fn iibal(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, eils::Iibal, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,eils::Iibal, Eils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Illegal Output Buffer Access Service Request Line   IOBAL"]
    #[inline(always)]
    pub fn iobal(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, eils::Iobal, Eils_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,eils::Iobal, Eils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Handler Constrains Flag Service Request Line   MHFL"]
    #[inline(always)]
    pub fn mhfl(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, eils::Mhfl, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x1,1,0,eils::Mhfl, Eils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Detected on Channel A Service Request Line   EDAL"]
    #[inline(always)]
    pub fn edal(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, eils::Edal, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1,1,0,eils::Edal, Eils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Latest Transmit Violation Channel A Service Request Line   LTVAL"]
    #[inline(always)]
    pub fn ltval(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, eils::Ltval, Eils_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,eils::Ltval, Eils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmission Across Boundary Channel A Service Request Line   TABAL"]
    #[inline(always)]
    pub fn tabal(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, eils::Tabal, Eils_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,eils::Tabal, Eils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Detected on Channel B Service Request Line   EDBL"]
    #[inline(always)]
    pub fn edbl(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, eils::Edbl, Eils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1,1,0,eils::Edbl, Eils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Latest Transmit Violation Channel B Service Request Line   LTVBL"]
    #[inline(always)]
    pub fn ltvbl(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, eils::Ltvbl, Eils_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,eils::Ltvbl, Eils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmission Across Boundary Channel A Service Request Line   TABBL"]
    #[inline(always)]
    pub fn tabbl(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, eils::Tabbl, Eils_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,eils::Tabbl, Eils_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Eils {
    #[inline(always)]
    fn default() -> Eils {
        <crate::RegValueT<Eils_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eils {
    pub struct Pemcl_SPEC;
    pub type Pemcl = crate::EnumBitfieldStruct<u8, Pemcl_SPEC>;
    impl Pemcl {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cnal_SPEC;
    pub type Cnal = crate::EnumBitfieldStruct<u8, Cnal_SPEC>;
    impl Cnal {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sfbml_SPEC;
    pub type Sfbml = crate::EnumBitfieldStruct<u8, Sfbml_SPEC>;
    impl Sfbml {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sfol_SPEC;
    pub type Sfol = crate::EnumBitfieldStruct<u8, Sfol_SPEC>;
    impl Sfol {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ccfl_SPEC;
    pub type Ccfl = crate::EnumBitfieldStruct<u8, Ccfl_SPEC>;
    impl Ccfl {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ccll_SPEC;
    pub type Ccll = crate::EnumBitfieldStruct<u8, Ccll_SPEC>;
    impl Ccll {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eerrl_SPEC;
    pub type Eerrl = crate::EnumBitfieldStruct<u8, Eerrl_SPEC>;
    impl Eerrl {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfol_SPEC;
    pub type Rfol = crate::EnumBitfieldStruct<u8, Rfol_SPEC>;
    impl Rfol {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Efal_SPEC;
    pub type Efal = crate::EnumBitfieldStruct<u8, Efal_SPEC>;
    impl Efal {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Iibal_SPEC;
    pub type Iibal = crate::EnumBitfieldStruct<u8, Iibal_SPEC>;
    impl Iibal {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Iobal_SPEC;
    pub type Iobal = crate::EnumBitfieldStruct<u8, Iobal_SPEC>;
    impl Iobal {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mhfl_SPEC;
    pub type Mhfl = crate::EnumBitfieldStruct<u8, Mhfl_SPEC>;
    impl Mhfl {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Edal_SPEC;
    pub type Edal = crate::EnumBitfieldStruct<u8, Edal_SPEC>;
    impl Edal {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ltval_SPEC;
    pub type Ltval = crate::EnumBitfieldStruct<u8, Ltval_SPEC>;
    impl Ltval {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tabal_SPEC;
    pub type Tabal = crate::EnumBitfieldStruct<u8, Tabal_SPEC>;
    impl Tabal {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Edbl_SPEC;
    pub type Edbl = crate::EnumBitfieldStruct<u8, Edbl_SPEC>;
    impl Edbl {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ltvbl_SPEC;
    pub type Ltvbl = crate::EnumBitfieldStruct<u8, Ltvbl_SPEC>;
    impl Ltvbl {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tabbl_SPEC;
    pub type Tabbl = crate::EnumBitfieldStruct<u8, Tabbl_SPEC>;
    impl Tabbl {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eir_SPEC;
impl crate::sealed::RegSpec for Eir_SPEC {
    type DataType = u32;
}
#[doc = "Error Service Request Select Register\n resetvalue={Application Reset:0x0}"]
pub type Eir = crate::RegValueT<Eir_SPEC>;

impl Eir {
    #[doc = "POC Error Mode Changed   PEMC. This flag is set whenever the error mode signalled by CCEV.ERRM in the        Communication Controller Error Vector register has changed. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn pemc(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, eir::Pemc, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,eir::Pemc, Eir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Command Not Accepted   CNA. The flag signals that the write access to the CHI command vector        SUCC1.CMD in the SUC Configuration Register 1 was not successful because        the requested command was not valid in the actual POC state  or because        the CHI command was locked  CCL   1 . This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn cna(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, eir::Cna, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,eir::Cna, Eir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SYNC Frames Below Minimum   SFBM. This flag signals that the number of SYNC Frames received during the        last communication cycle was below the limit required by the FlexRay  8482         protocol. May be set during startup and therefore should be cleared by        the Host after the Communication Controller entered   8220 NORMAL ACTIVE  8221         state. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn sfbm(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, eir::Sfbm, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,eir::Sfbm, Eir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SYNC Frame Overflow   SFO. Set when either the number of SYNC Frames received during the last        communication cycle or the total number of SYNC Frames received during        the last double cycle exceeds the maximum number of SYNC Frames as        defined by GTUC02.SNM in the GTU Configuration Register  160 2. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn sfo(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, eir::Sfo, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,eir::Sfo, Eir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Correction Failure   CCF. This flag is set at the end of the cycle whenever one of the following        errors occurred  Missing offset and   or rate correction Clock Correction limit reached The clock correction status is monitored in registers CCEV and SFS. A        failure may occur during startup  therefore bit CCF should be cleared by        the Host after the Communication Controller entered   8220 NORMAL ACTIVE  8221         state. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn ccf(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, eir::Ccf, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,eir::Ccf, Eir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CHI Command Locked   CCL. The flag signals that the write access to the CHI command vector        SUCC1.CMD was not successful because the execution of the previous CHI        command has not yet completed. In this case bit EIR.CNA is also set to 1. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn ccl(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, eir::Ccl, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,eir::Ccl, Eir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ECC Error   EERR. The flag signals an ECC error to the Host. It is set whenever one of the        flags MHDS.EIBF  MHDS.EOBF  MHDS.EMR  MHDS.ETBF1  MHDS.ETBF2 changes        from 0 to 1. See also   8220 Message Handler Status  8221 . This bit must be cleared at        initialization of the module"]
    #[inline(always)]
    pub fn eerr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, eir::Eerr, Eir_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x1,1,0,eir::Eerr, Eir_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive FIFO Overrun   RFO. The flag is set by the Communication Controller when a receive FIFO        overrun is detected. When a receive FIFO overrun occurs  the oldest        message is overwritten with the actual received message. The actual        state of the FIFO is monitored in register FSR."]
    #[inline(always)]
    pub fn rfo(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, eir::Rfo, Eir_SPEC, crate::common::R> {
        crate::common::RegisterField::<7,0x1,1,0,eir::Rfo, Eir_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Empty FIFO Access   EFA. This flag is set by the Communication Controller when the Host requests        the transfer of a message from the receive FIFO via Output Buffer while        the receive FIFO is empty."]
    #[inline(always)]
    pub fn efa(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, eir::Efa, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,eir::Efa, Eir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Illegal Input Buffer Access   IIBA. This flag is set by the Communication Controller when the Host wants to        modify a Message Buffer via Input Buffer while the Communication        Controller is not in   8220 CONFIG  8221  or   8220 DEFAULT CONFIG  8221  state and one of the        following conditions applies  The Host writes to the Input Buffer Command Request register to modify          the  Header Section of Message Buffer 0  1 if configured for transmission            in key slot Header Section of static Message Buffers with buffer number  lt             MRC.FDB while MRC.SEC   01 B Header Section of any static or dynamic Message Buffer while MRC.SEC              1x B Header and   or Data Section of any message buffer belonging to the            receive FIFO The Host writes to any register of the Input Buffer while IBCR.IBSYS          is set."]
    #[inline(always)]
    pub fn iiba(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, eir::Iiba, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,eir::Iiba, Eir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Illegal Output Buffer Access   IOBA. This flag is set by the Communication Controller when the Host requests        the transfer of a Message Buffer from the Message RAM to the Output        Buffer while OBCR.OBSYS is set to 1."]
    #[inline(always)]
    pub fn ioba(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, eir::Ioba, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x1,1,0,eir::Ioba, Eir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Handler Constraints Flag   MHF. The flag signals a Message Handler constraints violation condition. It        is set whenever one of the flags MHDF.SNUA  MHDF.SNUB  MHDF.FNFA         MHDF.FNFB  MHDF.TBFA  MHDF.TBFB  MHDF.TNSA  MHDF.TNSB  MHDF.WAHP changes        from 0 to 1."]
    #[inline(always)]
    pub fn mhf(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, eir::Mhf, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x1,1,0,eir::Mhf, Eir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Detected on Channel A   EDA. This bit is set whenever one of the flags ACS.SEDA  ACS.CEDA  ACS.CIA         ACS.SBVA changes from 0 to 1. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn eda(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, eir::Eda, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1,1,0,eir::Eda, Eir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Latest Transmit Violation Channel A   LTVA. The flag signals a latest transmit violation on channel A to the Host. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn ltva(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, eir::Ltva, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<17,0x1,1,0,eir::Ltva, Eir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmission Across Boundary Channel A   TABA. The flag signals to the Host that a transmission across a slot boundary        occurred for channel A. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn taba(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, eir::Taba, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x1,1,0,eir::Taba, Eir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Detected on Channel B   EDB. This bit is set whenever one of the flags ACS.SEDB  ACS.CEDB  ACS.CIB         ACS.SBVB changes from 0 to 1. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn edb(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, eir::Edb, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1,1,0,eir::Edb, Eir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Latest Transmit Violation Channel B   LTVB. The flag signals a latest transmit violation on channel B to the Host. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn ltvb(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, eir::Ltvb, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<25,0x1,1,0,eir::Ltvb, Eir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmission Across Boundary Channel B   TABB. The flag signals to the Host that a transmission across a slot boundary        occurred for channel B. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn tabb(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, eir::Tabb, Eir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x1,1,0,eir::Tabb, Eir_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Eir {
    #[inline(always)]
    fn default() -> Eir {
        <crate::RegValueT<Eir_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eir {
    pub struct Pemc_SPEC;
    pub type Pemc = crate::EnumBitfieldStruct<u8, Pemc_SPEC>;
    impl Pemc {
        #[doc = "0 Error mode has not changed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error mode has changed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cna_SPEC;
    pub type Cna = crate::EnumBitfieldStruct<u8, Cna_SPEC>;
    impl Cna {
        #[doc = "0 CHI command accepted"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 CHI command not accepted"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sfbm_SPEC;
    pub type Sfbm = crate::EnumBitfieldStruct<u8, Sfbm_SPEC>;
    impl Sfbm {
        #[doc = "0 Sync node  1 or more SYNC Frames received Non sync node  2 or more SYNC Frames received"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Less than the required minimum of SYNC Frames received"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sfo_SPEC;
    pub type Sfo = crate::EnumBitfieldStruct<u8, Sfo_SPEC>;
    impl Sfo {
        #[doc = "0 Number of received SYNC Frames   GTUC02.SNM"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 More SYNC Frames received than configured by GTUC02.SNM"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ccf_SPEC;
    pub type Ccf = crate::EnumBitfieldStruct<u8, Ccf_SPEC>;
    impl Ccf {
        #[doc = "0 Clock correction successful so far"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clock correction failed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ccl_SPEC;
    pub type Ccl = crate::EnumBitfieldStruct<u8, Ccl_SPEC>;
    impl Ccl {
        #[doc = "0 CHI command accepted"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 CHI command not accepted"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eerr_SPEC;
    pub type Eerr = crate::EnumBitfieldStruct<u8, Eerr_SPEC>;
    impl Eerr {
        #[doc = "0 No error detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error detected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfo_SPEC;
    pub type Rfo = crate::EnumBitfieldStruct<u8, Rfo_SPEC>;
    impl Rfo {
        #[doc = "0 No receive FIFO overrun detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A receive FIFO overrun has been detected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Efa_SPEC;
    pub type Efa = crate::EnumBitfieldStruct<u8, Efa_SPEC>;
    impl Efa {
        #[doc = "0 No Host access to empty FIFO occurred"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Host access to empty FIFO occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Iiba_SPEC;
    pub type Iiba = crate::EnumBitfieldStruct<u8, Iiba_SPEC>;
    impl Iiba {
        #[doc = "0 No illegal Host access to Input Buffer occurred"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Illegal Host access to Input Buffer occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ioba_SPEC;
    pub type Ioba = crate::EnumBitfieldStruct<u8, Ioba_SPEC>;
    impl Ioba {
        #[doc = "0 No illegal Host access to Output Buffer occurred"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Illegal Host access to Output Buffer occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mhf_SPEC;
    pub type Mhf = crate::EnumBitfieldStruct<u8, Mhf_SPEC>;
    impl Mhf {
        #[doc = "0 No Message Handler failure detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Message Handler failure detected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eda_SPEC;
    pub type Eda = crate::EnumBitfieldStruct<u8, Eda_SPEC>;
    impl Eda {
        #[doc = "0 No error detected on channel A"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error detected on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ltva_SPEC;
    pub type Ltva = crate::EnumBitfieldStruct<u8, Ltva_SPEC>;
    impl Ltva {
        #[doc = "0 No latest transmit violation detected on channel A"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Latest transmit violation detected on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Taba_SPEC;
    pub type Taba = crate::EnumBitfieldStruct<u8, Taba_SPEC>;
    impl Taba {
        #[doc = "0 No transmission across slot boundary detected on channel A"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transmission across slot boundary detected on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Edb_SPEC;
    pub type Edb = crate::EnumBitfieldStruct<u8, Edb_SPEC>;
    impl Edb {
        #[doc = "0 No error detected on channel B"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error detected on channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ltvb_SPEC;
    pub type Ltvb = crate::EnumBitfieldStruct<u8, Ltvb_SPEC>;
    impl Ltvb {
        #[doc = "0 No latest transmit violation detected on channel B"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Latest transmit violation detected on channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tabb_SPEC;
    pub type Tabb = crate::EnumBitfieldStruct<u8, Tabb_SPEC>;
    impl Tabb {
        #[doc = "0 No transmission across slot boundary detected on channel B"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transmission across slot boundary detected on channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Endn_SPEC;
impl crate::sealed::RegSpec for Endn_SPEC {
    type DataType = u32;
}
#[doc = "Endian Register\n resetvalue={Application Reset:0x087654321}"]
pub type Endn = crate::RegValueT<Endn_SPEC>;

impl Endn {
    #[doc = "Endianness Test Value   ETV. The endianness test value."]
    #[inline(always)]
    pub fn etv(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Endn_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Endn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Endn {
    #[inline(always)]
    fn default() -> Endn {
        <crate::RegValueT<Endn_SPEC> as RegisterValue<_>>::new(2271560481)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EsiDn_SPEC;
impl crate::sealed::RegSpec for EsiDn_SPEC {
    type DataType = u32;
}
#[doc = "Even Sync ID Symbol Window 01\n resetvalue={Application Reset:0x0}"]
pub type EsiDn = crate::RegValueT<EsiDn_SPEC>;

impl EsiDn {
    #[doc = "Even Sync ID vsSyncIDListA B even    EID. SYNC Frame ID even communication cycle."]
    #[inline(always)]
    pub fn eid(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, EsiDn_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, EsiDn_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Received Configured Even Sync ID on Channel A   RXEA. Signals that a SYNC Frame corresponding to the stored even sync ID was received on channel A or that the node is configured to be a sync node with key slot   EID  ESID1 only ."]
    #[inline(always)]
    pub fn rxea(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, esidn::Rxea, EsiDn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<14,0x1,1,0,esidn::Rxea, EsiDn_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Received Configured Even Sync ID on Channel B   RXEB. Signals that a SYNC Frame corresponding to the stored even sync ID was received on channel B or that the node is configured to be a sync node with key slot   EID  ESID1 only ."]
    #[inline(always)]
    pub fn rxeb(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, esidn::Rxeb, EsiDn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<15,0x1,1,0,esidn::Rxeb, EsiDn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for EsiDn {
    #[inline(always)]
    fn default() -> EsiDn {
        <crate::RegValueT<EsiDn_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod esidn {
    pub struct Rxea_SPEC;
    pub type Rxea = crate::EnumBitfieldStruct<u8, Rxea_SPEC>;
    impl Rxea {
        #[doc = "0 SYNC Frame not received on channel A   node configured to transmit SYNC Frames"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SYNC Frame received on channel A  node not configured to transmit SYNC Frames"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rxeb_SPEC;
    pub type Rxeb = crate::EnumBitfieldStruct<u8, Rxeb_SPEC>;
    impl Rxeb {
        #[doc = "0 SYNC Frame not received on channel B   node configured to transmit SYNC Frames"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SYNC Frame received on channel B   node not configured to transmit SYNC Frames"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcl_SPEC;
impl crate::sealed::RegSpec for Fcl_SPEC {
    type DataType = u32;
}
#[doc = "FIFO Critical Level\n resetvalue={Application Reset:0x080}"]
pub type Fcl = crate::RegValueT<Fcl_SPEC>;

impl Fcl {
    #[doc = "Critical Level   CL. When the receive FIFO fill level FSR.RFFL is equal or greater than the critical level configured by CL  the receive FIFO critical level flag FSR.RFCL is set. If CL is programmed to values  gt  128  bit FSR.RFCL is never set. When FSR.RFCL changes from 0 to 1 bit SIR.RFCL is set to 1  and if enabled  a service request is generated."]
    #[inline(always)]
    pub fn cl(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Fcl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Fcl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Fcl {
    #[inline(always)]
    fn default() -> Fcl {
        <crate::RegValueT<Fcl_SPEC> as RegisterValue<_>>::new(128)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frf_SPEC;
impl crate::sealed::RegSpec for Frf_SPEC {
    type DataType = u32;
}
#[doc = "FIFO Rejection Filter\n resetvalue={Application Reset:0x1800000}"]
pub type Frf = crate::RegValueT<Frf_SPEC>;

impl Frf {
    #[doc = "Channel Filter   CH. May be modified in  DEFAULT CONFIG  or  CONFIG  state only."]
    #[inline(always)]
    pub fn ch(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, frf::Ch, Frf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,frf::Ch, Frf_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Frame ID Filter   FID. Determines the Frame ID to be rejected by the FIFO. With the additional configuration of register FRFM  the corresponding Frame ID filter bits are ignored  which results in further rejected Frame IDs. When FRFM.MFID is zero  a Frame ID filter value of zero means that no Frame ID is rejected."]
    #[inline(always)]
    pub fn fid(
        self,
    ) -> crate::common::RegisterField<2, 0x7ff, 1, 0, frf::Fid, Frf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x7ff,1,0,frf::Fid, Frf_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Cycle Counter Filter   CYF. The 7 bit cycle counter filter determines the cycle set to which Frame        ID and channel rejection filter are applied. In cycles not belonging to the cycle set specified by CYF  all Frames are        rejected. For details about the configuration of the cycle counter        filter see   8220 Cycle Counter Filtering  8221 . May be modified in          8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221  state only."]
    #[inline(always)]
    pub fn cyf(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Frf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Frf_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reject in Static Segment   RSS. If this bit is set  the FIFO is used only be used in dynamic segment. May be modified in  DEFAULT CONFIG  or  CONFIG  state only."]
    #[inline(always)]
    pub fn rss(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, frf::Rss, Frf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<23,0x1,1,0,frf::Rss, Frf_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reject NULL Frames   RNF. If this bit is set  received NULL Frames are not stored in the FIFO. May be modified in  DEFAULT CONFIG  or  CONFIG  state only."]
    #[inline(always)]
    pub fn rnf(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, frf::Rnf, Frf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1,1,0,frf::Rnf, Frf_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Frf {
    #[inline(always)]
    fn default() -> Frf {
        <crate::RegValueT<Frf_SPEC> as RegisterValue<_>>::new(25165824)
    }
}
pub mod frf {
    pub struct Ch_SPEC;
    pub type Ch = crate::EnumBitfieldStruct<u8, Ch_SPEC>;
    impl Ch {
        #[doc = "00 receive on both channels If reception on both channels is configured  also in static segment always both Frames  from channel A and B  are stored in the FIFO  even if they are identical."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 receive only on channel B"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 receive only on channel A"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 no reception"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Fid_SPEC;
    pub type Fid = crate::EnumBitfieldStruct<u8, Fid_SPEC>;
    impl Fid {
        #[doc = "000  7FF H Frame ID filter values"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Rss_SPEC;
    pub type Rss = crate::EnumBitfieldStruct<u8, Rss_SPEC>;
    impl Rss {
        #[doc = "0 FIFO also used in static segment"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Reject messages for static segment"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rnf_SPEC;
    pub type Rnf = crate::EnumBitfieldStruct<u8, Rnf_SPEC>;
    impl Rnf {
        #[doc = "0 NULL Frames are stored in the FIFO"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Reject all NULL Frames"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Frfm_SPEC;
impl crate::sealed::RegSpec for Frfm_SPEC {
    type DataType = u32;
}
#[doc = "FIFO Rejection Filter Mask\n resetvalue={Application Reset:0x0}"]
pub type Frfm = crate::RegValueT<Frfm_SPEC>;

impl Frfm {
    #[doc = "Mask Frame ID Filter   MFID. May be modified in   8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221  state only. When  0         written in a bit position  the corresponding Frame ID filter bit is used        for rejection filtering. When  1  written in a bit position  the        corresponding Frame ID filter bit is ignored. Valid values are from        0x000   0x3FF."]
    #[inline(always)]
    pub fn mfid(
        self,
    ) -> crate::common::RegisterField<2, 0x7ff, 1, 0, u16, Frfm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x7ff,1,0,u16, Frfm_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Frfm {
    #[inline(always)]
    fn default() -> Frfm {
        <crate::RegValueT<Frfm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsr_SPEC;
impl crate::sealed::RegSpec for Fsr_SPEC {
    type DataType = u32;
}
#[doc = "FIFO Status Register\n resetvalue={Application Reset:0x0}"]
pub type Fsr = crate::RegValueT<Fsr_SPEC>;

impl Fsr {
    #[doc = "Receive FIFO Not Empty   RFNE. This flag is set by the Communication Controller when a received valid Frame  data or NULL Frame depending on rejection mask  was stored in the FIFO. In addition  service request flag SIR.RFNE is set. The bit is reset after the Host has read all message from the FIFO."]
    #[inline(always)]
    pub fn rfne(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, fsr::Rfne, Fsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,fsr::Rfne, Fsr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive FIFO Critical Level   RFCL. This flag is set when the receive FIFO fill level RFFL is equal or greater than the critical level as configured by FCL.CL. The flag is cleared by the Communication Controller as soon as RFFL drops below FCL.CL. When RFCL changes from 0 to 1 bit SIR.RFCL is set to 1  and if enabled  an service request is generated."]
    #[inline(always)]
    pub fn rfcl(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, fsr::Rfcl, Fsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,fsr::Rfcl, Fsr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive FIFO Overrun   RFO. The flag is set by the Communication Controller when a receive FIFO overrun is detected. When a receive FIFO overrun occurs  the oldest message is overwritten with the actual received message. In addition  service request flag EIR.RFO is set.The flag is cleared by the next FIFO read access issued by the Host."]
    #[inline(always)]
    pub fn rfo(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, fsr::Rfo, Fsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,fsr::Rfo, Fsr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive FIFO Fill Level   RFFL. Number of FIFO buffers filled up with new data not yet read by the Host. Maximum value is 128."]
    #[inline(always)]
    pub fn rffl(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Fsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Fsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Fsr {
    #[inline(always)]
    fn default() -> Fsr {
        <crate::RegValueT<Fsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fsr {
    pub struct Rfne_SPEC;
    pub type Rfne = crate::EnumBitfieldStruct<u8, Rfne_SPEC>;
    impl Rfne {
        #[doc = "0 Receive FIFO is empty"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Receive FIFO is not empty"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfcl_SPEC;
    pub type Rfcl = crate::EnumBitfieldStruct<u8, Rfcl_SPEC>;
    impl Rfcl {
        #[doc = "0 Receive FIFO below critical level"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Receive FIFO critical level reached"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfo_SPEC;
    pub type Rfo = crate::EnumBitfieldStruct<u8, Rfo_SPEC>;
    impl Rfo {
        #[doc = "0 No receive FIFO overrun detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A receive FIFO overrun has been detected"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuc01_SPEC;
impl crate::sealed::RegSpec for Gtuc01_SPEC {
    type DataType = u32;
}
#[doc = "GTU Configuration Register 1\n resetvalue={Application Reset:0x280}"]
pub type Gtuc01 = crate::RegValueT<Gtuc01_SPEC>;

impl Gtuc01 {
    #[doc = "Microtick per Cycle pMicroPerCycle    UT. Configures the duration of the communication cycle in Microticks. Valid        values are 640 to 640000  280 H to 9C400 H          Microticks."]
    #[inline(always)]
    pub fn ut(
        self,
    ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, Gtuc01_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfffff,1,0,u32, Gtuc01_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gtuc01 {
    #[inline(always)]
    fn default() -> Gtuc01 {
        <crate::RegValueT<Gtuc01_SPEC> as RegisterValue<_>>::new(640)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuc02_SPEC;
impl crate::sealed::RegSpec for Gtuc02_SPEC {
    type DataType = u32;
}
#[doc = "GTU Configuration Register 2\n resetvalue={Application Reset:0x2000A}"]
pub type Gtuc02 = crate::RegValueT<Gtuc02_SPEC>;

impl Gtuc02 {
    #[doc = "Macrotick Per Cycle gMacroPerCycle    MPC. Configures the duration of one communication cycle in Macroticks. The        cycle length must be identical in all nodes of a cluster. Valid values        are 10 to 16000  A H to 3E80 H          Macroticks."]
    #[inline(always)]
    pub fn mpc(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Gtuc02_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Gtuc02_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sync Node Max gSyncNodeMax    SNM. Maximum number of Frames within a cluster with SYNC Frame indicator bit        SYN set to 1. Must be identical in all nodes of a cluster. Valid values        are 2 to 15  2 H to F H  ."]
    #[inline(always)]
    pub fn snm(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Gtuc02_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Gtuc02_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gtuc02 {
    #[inline(always)]
    fn default() -> Gtuc02 {
        <crate::RegValueT<Gtuc02_SPEC> as RegisterValue<_>>::new(131082)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuc03_SPEC;
impl crate::sealed::RegSpec for Gtuc03_SPEC {
    type DataType = u32;
}
#[doc = "GTU Configuration Register 3\n resetvalue={Application Reset:0x2020000}"]
pub type Gtuc03 = crate::RegValueT<Gtuc03_SPEC>;

impl Gtuc03 {
    #[doc = "Microtick Initial Offset Channel A pMicroInitialOffset A     UIOA. Configures the number of Microticks between the actual time reference        point on channel A and the subsequent Macrotick boundary of the        secondary time reference point. The parameter depends on        pDelayCompensation A  and therefore has to be set for each channel        independently. Valid values are 0 to 240  0 H to F0 H   Microticks."]
    #[inline(always)]
    pub fn uioa(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Gtuc03_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Gtuc03_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Microtick Initial Offset Channel B  pMicroInitialOffset B     UIOB. Configures the number of Microticks between the actual time reference        point on channel B and the subsequent Macrotick boundary of the        secondary time reference point. The parameter depends on        pDelayCompensation B  and therefore has to be set for each channel        independently. Valid values are 0 to 240  0 H to F0 H   Microticks."]
    #[inline(always)]
    pub fn uiob(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Gtuc03_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Gtuc03_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Macrotick Initial Offset Channel A gMacroInitialOffset A     MIOA. Configures the number of Macroticks between the static slot boundary and        the subsequent Macrotick boundary of the secondary time reference point        based on the nominal Macrotick duration. Must be identical in all nodes        of a cluster. Valid values are 2 to 72  2 H to 48 H   Macroticks."]
    #[inline(always)]
    pub fn mioa(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Gtuc03_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Gtuc03_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Macrotick Initial Offset Channel B gMacroInitialOffset B     MIOB. Configures the number of Macroticks between the static slot boundary and        the subsequent Macrotick boundary of the secondary time reference point        based on the nominal Macrotick duration. Must be identical in all nodes        of a cluster. Valid values are 2 to 72  2 H to 48 H   Macroticks."]
    #[inline(always)]
    pub fn miob(
        self,
    ) -> crate::common::RegisterField<24, 0x7f, 1, 0, u8, Gtuc03_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7f,1,0,u8, Gtuc03_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gtuc03 {
    #[inline(always)]
    fn default() -> Gtuc03 {
        <crate::RegValueT<Gtuc03_SPEC> as RegisterValue<_>>::new(33685504)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuc04_SPEC;
impl crate::sealed::RegSpec for Gtuc04_SPEC {
    type DataType = u32;
}
#[doc = "GTU Configuration Register 4\n resetvalue={Application Reset:0x080007}"]
pub type Gtuc04 = crate::RegValueT<Gtuc04_SPEC>;

impl Gtuc04 {
    #[doc = "Network Idle Time Start gMacroPerCycle   gdNIT   1    NIT. Configures the starting point of the Network Idle Time  NIT  at the end        of the communication cycle expressed in terms of Macroticks from the        beginning of the cycle. The start of network idle time  NIT  is        recognized if Macrotick   gMacroPerCycle   gdNIT  1 and the increment        pulse of Macrotick is set. Must be identical in all nodes of a cluster.        Valid values are 7 to 15997  7 H to 3E7D H          Macroticks."]
    #[inline(always)]
    pub fn nit(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Gtuc04_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Gtuc04_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Offset Correction Start  gOffsetCorrectionStart   1    OCS. Determines the start of the offset correction within the network idle        time  NIT  phase  calculated from start of cycle. Must be identical in        all nodes of a cluster. For cluster consisting of E Ray implementations only  it is sufficient to program OCS   NIT   1. Valid        values are 8 to 15998  8 H to 3E7E H          Macroticks."]
    #[inline(always)]
    pub fn ocs(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, Gtuc04_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3fff,1,0,u16, Gtuc04_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gtuc04 {
    #[inline(always)]
    fn default() -> Gtuc04 {
        <crate::RegValueT<Gtuc04_SPEC> as RegisterValue<_>>::new(524295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuc05_SPEC;
impl crate::sealed::RegSpec for Gtuc05_SPEC {
    type DataType = u32;
}
#[doc = "GTU Configuration Register 5\n resetvalue={Application Reset:0x0E000000}"]
pub type Gtuc05 = crate::RegValueT<Gtuc05_SPEC>;

impl Gtuc05 {
    #[doc = "Delay Compensation Channel A pDelayCompensation A     DCA. Used to compensate for reception delays on channel A. This covers        assumed propagation delay up to cPropagationDelayMax for Microticks in        the range of 0.0125  160   181 s to 0.05  160   181 s. In practice  the minimum of the        propagation delays of all sync nodes should be applied. Valid values are 0 to 200  0 H to C8 H          Microticks."]
    #[inline(always)]
    pub fn dca(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Gtuc05_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Gtuc05_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Delay Compensation Channel B  pDelayCompensation B     DCB. Used to compensate for reception delays on channel B. This covers        assumed propagation delay up to cPropagationDelayMax for Microticks in        the range of 0.0125 to 0.05  160   181 s. In practice  the minimum of the        propagation delays of all sync nodes should be applied. Valid values are 0 to 200  0 H to C8 H          Microticks."]
    #[inline(always)]
    pub fn dcb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Gtuc05_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Gtuc05_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Cluster Drift Damping pClusterDriftDamping    CDD. Configures the cluster drift damping value used in clock synchronization        to minimize accumulation of rounding errors. Valid values are 0 to 20  0 H to 14 H   Microticks."]
    #[inline(always)]
    pub fn cdd(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Gtuc05_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Gtuc05_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Decoding Correction pDecodingCorrection    DEC. Configures the decoding correction value used to determine the primary        time reference point. Valid values are 14 to 143  E H to 8F H   Microticks."]
    #[inline(always)]
    pub fn dec(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Gtuc05_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Gtuc05_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gtuc05 {
    #[inline(always)]
    fn default() -> Gtuc05 {
        <crate::RegValueT<Gtuc05_SPEC> as RegisterValue<_>>::new(234881024)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuc06_SPEC;
impl crate::sealed::RegSpec for Gtuc06_SPEC {
    type DataType = u32;
}
#[doc = "GTU Configuration Register 6\n resetvalue={Application Reset:0x20000}"]
pub type Gtuc06 = crate::RegValueT<Gtuc06_SPEC>;

impl Gtuc06 {
    #[doc = "Accepted Startup Range pdAcceptedStartupRange    ASR. Number of Microticks constituting the expanded range of measured deviation for startup Frames during integration. Valid values are 0 to 1875  0 H to 753 H   Microticks."]
    #[inline(always)]
    pub fn asr(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Gtuc06_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Gtuc06_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Maximum Oscillator Drift pdMaxDrift 1    MOD. Maximum drift offset between two nodes that operate with unsynchronized clocks over one communication cycle in Microticks. Valid values are 2 to 1923  2 H to 783 H   Microticks."]
    #[inline(always)]
    pub fn r#mod(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, Gtuc06_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7ff,1,0,u16, Gtuc06_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gtuc06 {
    #[inline(always)]
    fn default() -> Gtuc06 {
        <crate::RegValueT<Gtuc06_SPEC> as RegisterValue<_>>::new(131072)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuc07_SPEC;
impl crate::sealed::RegSpec for Gtuc07_SPEC {
    type DataType = u32;
}
#[doc = "GTU Configuration Register 7\n resetvalue={Application Reset:0x20004}"]
pub type Gtuc07 = crate::RegValueT<Gtuc07_SPEC>;

impl Gtuc07 {
    #[doc = "Static Slot Length gdStaticSlot    SSL. Configures the duration of a static slot in Macroticks. The static slot        length must be identical in all nodes of a cluster. Valid values are 4        to 659  4 H to 293 H          Macroticks."]
    #[inline(always)]
    pub fn ssl(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Gtuc07_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Gtuc07_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Number of Static Slots gNumberOfStaticSlots    NSS. Configures the number of static slots in a cycle. At least 2 coldstart        nodes must be configured to startup a FlexRay  8482  network. The number of        static slots must be identical in all nodes of a cluster. Valid values        are 2 to 1023  2 H to 3FF H  ."]
    #[inline(always)]
    pub fn nss(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Gtuc07_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Gtuc07_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gtuc07 {
    #[inline(always)]
    fn default() -> Gtuc07 {
        <crate::RegValueT<Gtuc07_SPEC> as RegisterValue<_>>::new(131076)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuc08_SPEC;
impl crate::sealed::RegSpec for Gtuc08_SPEC {
    type DataType = u32;
}
#[doc = "GTU Configuration Register 8\n resetvalue={Application Reset:0x2}"]
pub type Gtuc08 = crate::RegValueT<Gtuc08_SPEC>;

impl Gtuc08 {
    #[doc = "Minislot Length gdMinislot    MSL. Configures the duration of a minislot in Macroticks. The minislot length        must be identical in all nodes of a cluster. Valid values are 2 to 63  2 H to 3F H   Macroticks."]
    #[inline(always)]
    pub fn msl(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Gtuc08_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Gtuc08_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Number of Minislots gNumberOfMinislots    NMS. Configures the number of minislots within the dynamic segment of a        cycle. The number of minislots must be identical in all nodes of a        cluster. Valid values are 0 to 7986  0 H to 1F32 H  ."]
    #[inline(always)]
    pub fn nms(
        self,
    ) -> crate::common::RegisterField<16, 0x1fff, 1, 0, u16, Gtuc08_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1fff,1,0,u16, Gtuc08_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gtuc08 {
    #[inline(always)]
    fn default() -> Gtuc08 {
        <crate::RegValueT<Gtuc08_SPEC> as RegisterValue<_>>::new(2)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuc09_SPEC;
impl crate::sealed::RegSpec for Gtuc09_SPEC {
    type DataType = u32;
}
#[doc = "GTU Configuration Register 9\n resetvalue={Application Reset:0x101}"]
pub type Gtuc09 = crate::RegValueT<Gtuc09_SPEC>;

impl Gtuc09 {
    #[doc = "Action Point Offset gdActionPointOffset    APO. Configures the action point offset in Macroticks within static slots and        symbol window. Must be identical in all nodes of a cluster. Valid values        are 1 to 63  1 H to 3F H          Macroticks."]
    #[inline(always)]
    pub fn apo(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Gtuc09_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Gtuc09_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Minislot Action Point Offset  gd Minislot Action Point Offset    MAPO. Configures the action point offset in Macroticks within the minislots of        the dynamic segment. Must be identical in all nodes of a cluster. Valid        values are 1 to 31  1 H to 1F H          Macroticks."]
    #[inline(always)]
    pub fn mapo(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, Gtuc09_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, Gtuc09_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Dynamic Slot Idle Phase  gdDynamicSlotIdlePhase    DSI. The duration of the dynamic slot idle phase has to be greater or equal        than the idle detection time. Must be identical in all nodes of a        cluster. Valid values are 0 to 2 Minislot."]
    #[inline(always)]
    pub fn dsi(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Gtuc09_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Gtuc09_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gtuc09 {
    #[inline(always)]
    fn default() -> Gtuc09 {
        <crate::RegValueT<Gtuc09_SPEC> as RegisterValue<_>>::new(257)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuc10_SPEC;
impl crate::sealed::RegSpec for Gtuc10_SPEC {
    type DataType = u32;
}
#[doc = "GTU Configuration Register 10\n resetvalue={Application Reset:0x20005}"]
pub type Gtuc10 = crate::RegValueT<Gtuc10_SPEC>;

impl Gtuc10 {
    #[doc = "Maximum Offset Correction pOffsetCorrectionOut    MOC. Holds the maximum permitted offset correction value to be applied by the        internal clock synchronization algorithm  absolute value . The        Communication Controller checks only the internal offset correction        value against the maximum offset correction value. Valid values are 5 to        15266  5 H to 3BA2 H          Microticks."]
    #[inline(always)]
    pub fn moc(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Gtuc10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Gtuc10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Maximum Rate Correction  pRateCorrectionOut    MRC. Holds the maximum permitted rate correction value to be applied by the        internal clock synchronization algorithm. The communication controller        checks only the internal rate correction value against the maximum rate        correction value  absolute value . Valid values are 2 to 1923  2 H to 783 H   Microticks."]
    #[inline(always)]
    pub fn mrc(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, Gtuc10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7ff,1,0,u16, Gtuc10_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gtuc10 {
    #[inline(always)]
    fn default() -> Gtuc10 {
        <crate::RegValueT<Gtuc10_SPEC> as RegisterValue<_>>::new(131077)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtuc11_SPEC;
impl crate::sealed::RegSpec for Gtuc11_SPEC {
    type DataType = u32;
}
#[doc = "GTU Configuration Register 11\n resetvalue={Application Reset:0x0}"]
pub type Gtuc11 = crate::RegValueT<Gtuc11_SPEC>;

impl Gtuc11 {
    #[doc = "External Offset Correction Control pExternOffsetControl    EOCC. By writing to EOCC the external offset correction is enabled as        specified below. Should be modified only outside network idle time  NIT ."]
    #[inline(always)]
    pub fn eocc(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, gtuc11::Eocc, Gtuc11_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,gtuc11::Eocc, Gtuc11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "External Rate Correction Control pExternRateControl    ERCC. By writing to ERCC the external rate correction is enabled as specified        below. Should be modified only outside network idle time  NIT ."]
    #[inline(always)]
    pub fn ercc(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, gtuc11::Ercc, Gtuc11_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,gtuc11::Ercc, Gtuc11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "External Offset Correction pExternOffsetCorrection    EOC. Holds the external clock offset correction value in Microticks to be        applied by the internal synchronization algorithm. The value is        subtracted   added from   to the calculated offset correction value. The        value is applied during network idle time  NIT . May be modified in          8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221  state only. Valid values are 0 to 7        Microticks."]
    #[inline(always)]
    pub fn eoc(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Gtuc11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Gtuc11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "External Rate Correction  pExternRateCorrection    ERC. Holds the external rate correction value in Microticks to be applied by        the internal clock synchronization algorithm. The value is subtracted          added from   to the calculated rate correction value. The value is        applied during network idle time  NIT . Can be modified in          8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221  state only. Valid values are 0 to 7        Microticks."]
    #[inline(always)]
    pub fn erc(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Gtuc11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Gtuc11_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Gtuc11 {
    #[inline(always)]
    fn default() -> Gtuc11 {
        <crate::RegValueT<Gtuc11_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod gtuc11 {
    pub struct Eocc_SPEC;
    pub type Eocc = crate::EnumBitfieldStruct<u8, Eocc_SPEC>;
    impl Eocc {
        #[doc = "00 No external clock correction"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 No external clock correction"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 External offset correction value subtracted from calculated offset correction value"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 External offset correction value added to calculated offset correction value"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Ercc_SPEC;
    pub type Ercc = crate::EnumBitfieldStruct<u8, Ercc_SPEC>;
    impl Ercc {
        #[doc = "00 No external rate correction"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 No external rate correction"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 External rate correction value subtracted from calculated rate correction value"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 External rate correction value added to calculated rate correction value"]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ibcm_SPEC;
impl crate::sealed::RegSpec for Ibcm_SPEC {
    type DataType = u32;
}
#[doc = "Input Buffer Command Mask\n resetvalue={Application Reset:0x0}"]
pub type Ibcm = crate::RegValueT<Ibcm_SPEC>;

impl Ibcm {
    #[doc = "Load Header Section Host   LHSH"]
    #[inline(always)]
    pub fn lhsh(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ibcm::Lhsh, Ibcm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,ibcm::Lhsh, Ibcm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Load Data Section Host   LDSH"]
    #[inline(always)]
    pub fn ldsh(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ibcm::Ldsh, Ibcm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,ibcm::Ldsh, Ibcm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Set Transmission Request Host   STXRH. If this bit is set to 1  the Transmission Request flag TXRQ1.TXRn  n   0 31  to TXRQ4.TXRn  n   0 31  for the selected Message Buffer is set in the Transmission Request Registers to release the Message Buffer for transmission. In single shot mode the flag is cleared by the Communication Controller after transmission has completed. TXRQ1.TXRn  n   0 31  to TXRQ4.TXRn  n   0 31  are evaluated for transmit buffer only."]
    #[inline(always)]
    pub fn stxrh(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ibcm::Stxrh, Ibcm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,ibcm::Stxrh, Ibcm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Load Header Section Shadow   LHSS"]
    #[inline(always)]
    pub fn lhss(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, ibcm::Lhss, Ibcm_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x1,1,0,ibcm::Lhss, Ibcm_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Load Data Section Shadow   LDSS"]
    #[inline(always)]
    pub fn ldss(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, ibcm::Ldss, Ibcm_SPEC, crate::common::R> {
        crate::common::RegisterField::<17,0x1,1,0,ibcm::Ldss, Ibcm_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmission Request Shadow   STXRS. If this bit is set to 1  the Transmission Request flag TXRQ1.TXRn  n   0 31  to TXRQ4.TXRn  n   0 31  for the selected Message Buffer is set in the Transmission Request Registers to release the Message Buffer for transmission. In single shot mode the flag is cleared by the Communication Controller after transmission has completed. TXRQ1.TXRn  n   0 31  to TXRQ4.TXRn  n   0 31  are evaluated for transmit buffer only."]
    #[inline(always)]
    pub fn stxrs(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, ibcm::Stxrs, Ibcm_SPEC, crate::common::R> {
        crate::common::RegisterField::<18,0x1,1,0,ibcm::Stxrs, Ibcm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ibcm {
    #[inline(always)]
    fn default() -> Ibcm {
        <crate::RegValueT<Ibcm_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ibcm {
    pub struct Lhsh_SPEC;
    pub type Lhsh = crate::EnumBitfieldStruct<u8, Lhsh_SPEC>;
    impl Lhsh {
        #[doc = "0 Header Section is not updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Header Section selected for transfer from Input Buffer to the Message RAM"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ldsh_SPEC;
    pub type Ldsh = crate::EnumBitfieldStruct<u8, Ldsh_SPEC>;
    impl Ldsh {
        #[doc = "0 Data Section is not updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Data Section selected for transfer from Input Buffer to the Message RAM"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Stxrh_SPEC;
    pub type Stxrh = crate::EnumBitfieldStruct<u8, Stxrh_SPEC>;
    impl Stxrh {
        #[doc = "0 Reset Transmission Request flag"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set Transmission Request flag  transmit buffer released for transmission"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lhss_SPEC;
    pub type Lhss = crate::EnumBitfieldStruct<u8, Lhss_SPEC>;
    impl Lhss {
        #[doc = "0 Header Section is not updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Header Section selected for transfer from Input Buffer to the Message RAM  transfer is ongoing of finalized"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ldss_SPEC;
    pub type Ldss = crate::EnumBitfieldStruct<u8, Ldss_SPEC>;
    impl Ldss {
        #[doc = "0 Data Section is not updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Data Section selected for transfer from Input Buffer to the Message RAM  transfer is ongoing of finalized"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Stxrs_SPEC;
    pub type Stxrs = crate::EnumBitfieldStruct<u8, Stxrs_SPEC>;
    impl Stxrs {
        #[doc = "0 Reset Transmission Request flag"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set Transmission Request flag  transmit buffer released for transmission  operation is ongoing of finalized"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ibcr_SPEC;
impl crate::sealed::RegSpec for Ibcr_SPEC {
    type DataType = u32;
}
#[doc = "Input Buffer Command Request\n resetvalue={Application Reset:0x0}"]
pub type Ibcr = crate::RegValueT<Ibcr_SPEC>;

impl Ibcr {
    #[doc = "Input Buffer Request Host   IBRH. Selects the target Message Buffer in the Message RAM for data transfer from Input Buffer. Valid values are 00 H to 7F H  0 127 ."]
    #[inline(always)]
    pub fn ibrh(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Ibcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8, Ibcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Buffer Busy Host   IBSYH. Set to 1 by writing IBRH while IBSYS is still 1. After the ongoing transfer between IBF Shadow and the Message RAM has completed  the IBSYH is set back to 0."]
    #[inline(always)]
    pub fn ibsyh(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, ibcr::Ibsyh, Ibcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<15,0x1,1,0,ibcr::Ibsyh, Ibcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Input Buffer Request Shadow   IBRS. Number of the target Message Buffer actually updated lately updated. Valid values are 00 H to 7F H  0 127 ."]
    #[inline(always)]
    pub fn ibrs(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Ibcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Ibcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Input Buffer Busy Shadow   IBSYS. Set to 1 after writing IBRH. When the transfer between IBF Shadow and the Message RAM has completed  IBSYS is set back to 0."]
    #[inline(always)]
    pub fn ibsys(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, ibcr::Ibsys, Ibcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<31,0x1,1,0,ibcr::Ibsys, Ibcr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ibcr {
    #[inline(always)]
    fn default() -> Ibcr {
        <crate::RegValueT<Ibcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ibcr {
    pub struct Ibsyh_SPEC;
    pub type Ibsyh = crate::EnumBitfieldStruct<u8, Ibsyh_SPEC>;
    impl Ibsyh {
        #[doc = "0 No request pending"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Request while transfer between IBF Shadow and Message RAM in progress"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ibsys_SPEC;
    pub type Ibsys = crate::EnumBitfieldStruct<u8, Ibsys_SPEC>;
    impl Ibsys {
        #[doc = "0 Transfer between IBF Shadow and Message RAM completed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transfer between IBF Shadow and Message RAM in progress"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x44C000}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MOD REV. MOD REV defines the module revision number. The value of a module revision starts with 01 H  first revision ."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MOD TYPE. The value of this bit field is C0 H . It defines the module as a 32 bit module."]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MOD NUMBER. This bit field defines a module identification number. For the E Ray module the module identification number is 44 H ."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(4505600)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ile_SPEC;
impl crate::sealed::RegSpec for Ile_SPEC {
    type DataType = u32;
}
#[doc = "Service Request Line Enable\n resetvalue={Application Reset:0x0}"]
pub type Ile = crate::RegValueT<Ile_SPEC>;

impl Ile {
    #[doc = "Enable Service Request Line 0 INT0SRC    EINT0"]
    #[inline(always)]
    pub fn eint0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ile::Eint0, Ile_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,ile::Eint0, Ile_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Service Request Line 1 INT1SRC    EINT1"]
    #[inline(always)]
    pub fn eint1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ile::Eint1, Ile_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,ile::Eint1, Ile_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ile {
    #[inline(always)]
    fn default() -> Ile {
        <crate::RegValueT<Ile_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ile {
    pub struct Eint0_SPEC;
    pub type Eint0 = crate::EnumBitfieldStruct<u8, Eint0_SPEC>;
    impl Eint0 {
        #[doc = "0 Service Request line disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request line enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eint1_SPEC;
    pub type Eint1 = crate::EnumBitfieldStruct<u8, Eint1_SPEC>;
    impl Eint1 {
        #[doc = "0 Service Request line disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request line enabled"]
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel reset registers is set. The RST bit will be cleared  re set to  0   by the BPI FPI after the kernel reset was executed."]
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
pub struct Lck_SPEC;
impl crate::sealed::RegSpec for Lck_SPEC {
    type DataType = u32;
}
#[doc = "Lock Register\n resetvalue={Application Reset:0x0}"]
pub type Lck = crate::RegValueT<Lck_SPEC>;

impl Lck {
    #[doc = "Configuration Lock Key   CLK. To leave  CONFIG  state by writing to SUCC1.CMD commands READY  MONITOR MODE  ATM  LOOP BACK  in the SUC Configuration Register 1  the write operation has to be directly preceded by two consecutive write accesses to the Configuration Lock Key  unlock sequence . If the write sequence below is interrupted by other write accesses between the second write to the Configuration Lock Key and the write access to the SUCC1 register  the Communication Controller remains in  CONFIG  state and the sequence has to be repeated. First write  LCK.CLK   CE H   1100 1110 B Second write  LCK.CLK   31 H   0011 0001 B Third write  SUCC1.CMD Returns 0 if read"]
    #[inline(always)]
    pub fn clk(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Lck_SPEC, crate::common::W> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Lck_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Test Mode Key   TMK. To set bit TEST1.WRTEN the write operation has to be directly preceded by two consecutive write accesses to the Test Mode Key. If the write sequence is interrupted by other write accesses between the second write to the Test Mode Key and the write access to the Test1 register  bit TEST1.WRTEN is not set to 1 and the sequence has to be repeated. First write  LCK.TMK   75 H   0111 0101 B Second write  LCK.TMK   8A H   1000 1010 B Second write  TEST1.WRTEN   1 Returns 0 if read"]
    #[inline(always)]
    pub fn tmk(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Lck_SPEC, crate::common::W> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Lck_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Lck {
    #[inline(always)]
    fn default() -> Lck {
        <crate::RegValueT<Lck_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldts_SPEC;
impl crate::sealed::RegSpec for Ldts_SPEC {
    type DataType = u32;
}
#[doc = "Last Dynamic Transmit Slot\n resetvalue={Application Reset:0x0}"]
pub type Ldts = crate::RegValueT<Ldts_SPEC>;

impl Ldts {
    #[doc = "Last Dynamic Transmission Channel A   LDTA. Value of  vSlotCounter A   at the time of the last Frame transmission on channel A in the dynamic segment of this node. It is updated at the end of the dynamic segment and is reset to zero if no Frame was transmitted during the dynamic segment."]
    #[inline(always)]
    pub fn ldta(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Ldts_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Ldts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Last Dynamic Transmission Channel B   LDTB. Value of  vSlotCounter B   at the time of the last Frame transmission on channel B in the dynamic segment of this node. It is updated at the end of the dynamic segment and is reset to zero if no Frame was transmitted during the dynamic segment."]
    #[inline(always)]
    pub fn ldtb(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, Ldts_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7ff,1,0,u16, Ldts_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ldts {
    #[inline(always)]
    fn default() -> Ldts {
        <crate::RegValueT<Ldts_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbs_SPEC;
impl crate::sealed::RegSpec for Mbs_SPEC {
    type DataType = u32;
}
#[doc = "Message Buffer Status\n resetvalue={Application Reset:0x0}"]
pub type Mbs = crate::RegValueT<Mbs_SPEC>;

impl Mbs {
    #[doc = "Valid Frame Received on Channel A vSS ValidFrameA    VFRA. A valid Frame indication is set if a valid Frame was received on channel A."]
    #[inline(always)]
    pub fn vfra(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mbs::Vfra, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,mbs::Vfra, Mbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Valid Frame Received on Channel B vSS ValidFrameB    VFRB. A valid Frame indication is set if a valid Frame was received on channel B."]
    #[inline(always)]
    pub fn vfrb(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, mbs::Vfrb, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,mbs::Vfrb, Mbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Syntax Error Observed on Channel A vSS SyntaxErrorA    SEOA. A syntax error was observed in the assigned slot on channel A."]
    #[inline(always)]
    pub fn seoa(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, mbs::Seoa, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,mbs::Seoa, Mbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Syntax Error Observed on Channel B vSS SyntaxErrorB    SEOB. A syntax error was observed in the assigned slot on channel B."]
    #[inline(always)]
    pub fn seob(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, mbs::Seob, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1,1,0,mbs::Seob, Mbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Content Error Observed on Channel A vSS ContentErrorA    CEOA. A content error was observed in the assigned slot on channel A."]
    #[inline(always)]
    pub fn ceoa(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, mbs::Ceoa, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x1,1,0,mbs::Ceoa, Mbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Content Error Observed on Channel B vSS ContentErrorB    CEOB. A content error was observed in the assigned slot on channel B."]
    #[inline(always)]
    pub fn ceob(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, mbs::Ceob, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x1,1,0,mbs::Ceob, Mbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Slot Boundary Violation Observed on Channel A vSS BViolationA    SVOA. A slot boundary violation  channel active at the start or at the end of the assigned slot  was observed on channel A."]
    #[inline(always)]
    pub fn svoa(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, mbs::Svoa, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x1,1,0,mbs::Svoa, Mbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Slot Boundary Violation Observed on Channel B vSS BViolationB    SVOB. A slot boundary violation  channel active at the start or at the end of the assigned slot  was observed on channel B."]
    #[inline(always)]
    pub fn svob(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, mbs::Svob, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterField::<7,0x1,1,0,mbs::Svob, Mbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmission Conflict Indication Channel A vSS TxConflictA    TCIA. A transmission conflict indication is set if a transmission conflict has occurred on channel A."]
    #[inline(always)]
    pub fn tcia(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, mbs::Tcia, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x1,1,0,mbs::Tcia, Mbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmission Conflict Indication Channel B vSS TxConflictB    TCIB. A transmission conflict indication is set if a transmission conflict has occurred on channel B."]
    #[inline(always)]
    pub fn tcib(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, mbs::Tcib, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterField::<9,0x1,1,0,mbs::Tcib, Mbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Empty Slot Channel A   ESA. In an empty slot there is no activity detected on the bus. The condition is checked in static and dynamic slots."]
    #[inline(always)]
    pub fn esa(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, mbs::Esa, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterField::<10,0x1,1,0,mbs::Esa, Mbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Empty Slot Channel B   ESB. In an empty slot there is no activity detected on the bus. The condition is checked in static and dynamic slots."]
    #[inline(always)]
    pub fn esb(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, mbs::Esb, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterField::<11,0x1,1,0,mbs::Esb, Mbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Message Lost   MLST. The flag is set in case the Host did not read the message before the Message Buffer was updated from a received Data Frame. Not affected by reception of NULL Frames except for Message Buffers belonging to the receive FIFO. The flag is reset by a Host write to the Message Buffer via IBF or when a new message is stored into the Message Buffer after the Message Buffers ND flag was reset by reading out the Message Buffer via OBF."]
    #[inline(always)]
    pub fn mlst(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, mbs::Mlst, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x1,1,0,mbs::Mlst, Mbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Frame Transmitted on Channel A   FTA. Indicates that this node has transmitted a Data Frame in the assigned slot on channel A. The FlexRay  protocol specification requires that FTA can only be reset by the Host. Therefore the Cycle Count Status CCS for these bits is only valid for the cycle where the bits are set to 1"]
    #[inline(always)]
    pub fn fta(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, mbs::Fta, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterField::<14,0x1,1,0,mbs::Fta, Mbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Frame Transmitted on Channel B   FTB. Indicates that this node has transmitted a Data Frame in the assigned slot on channel B. The FlexRay  protocol specification requires that FTB can only be reset by the Host. Therefore the Cycle Count Status CCS for these bits is only valid for the cycle where the bits are set to 1"]
    #[inline(always)]
    pub fn ftb(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, mbs::Ftb, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterField::<15,0x1,1,0,mbs::Ftb, Mbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Cycle Count Status   CCS. Cycle Count when status  MBS register  has been updated."]
    #[inline(always)]
    pub fn ccs(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Mbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Received on Channel Indicator Status vSS Channel    RCIS. Indicates the channel on which the Frame was received. For receive buffers  CFG   0  the RCIS is updated from both valid data and NULL Frames. If no valid Frame was received  the previous value is maintained. For transmit buffers the flags have no meaning and should be ignored."]
    #[inline(always)]
    pub fn rcis(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, mbs::Rcis, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x1,1,0,mbs::Rcis, Mbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Startup Frame Indicator Status vRF Header SuFIndicator    SFIS. A Startup Frame is marked by the Startup Frame indicator. For receive buffers  CFG   0  the SFIS is updated from both valid data and NULL Frames. If no valid Frame was received  the previous value is maintained. For transmit buffers the flags have no meaning and should be ignored."]
    #[inline(always)]
    pub fn sfis(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, mbs::Sfis, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterField::<25,0x1,1,0,mbs::Sfis, Mbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SYNC Frame Indicator Status vRF Header SyFIndicator    SYNS. A Startup Frame is marked by the Startup Frame indicator. For receive buffers  CFG   0  the SYNS is updated from both valid data and NULL Frames. If no valid Frame was received  the previous value is maintained. For transmit buffers the flags have no meaning and should be ignored."]
    #[inline(always)]
    pub fn syns(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, mbs::Syns, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterField::<26,0x1,1,0,mbs::Syns, Mbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "NULL Frame Indicator Status vRF Header NFIndicator    NFIS. If reset to 0 the Payload Segment of the received Frame contains no usable data. For receive buffers  CFG   0  the NFIS is updated from both valid data and NULL Frames. If no valid Frame was received  the previous value is maintained. For transmit buffers the flags have no meaning and should be ignored."]
    #[inline(always)]
    pub fn nfis(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, mbs::Nfis, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterField::<27,0x1,1,0,mbs::Nfis, Mbs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Payload Preamble Indictor Status vRF Header PPIndicator    PPIS. The payload preamble indicator defines whether a Network Management vector or message ID is contained within the Payload Segment of the received Frame. For receive buffers  CFG   0  the PPIS is updated from both valid data and NULL Frames. If no valid Frame was received  the previous value is maintained. For transmit buffers the flags have no meaning and should be ignored."]
    #[inline(always)]
    pub fn ppis(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, mbs::Ppis, Mbs_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0x1,1,0,mbs::Ppis, Mbs_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Mbs {
    #[inline(always)]
    fn default() -> Mbs {
        <crate::RegValueT<Mbs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mbs {
    pub struct Vfra_SPEC;
    pub type Vfra = crate::EnumBitfieldStruct<u8, Vfra_SPEC>;
    impl Vfra {
        #[doc = "0 No valid Frame received on channel A"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Valid Frame received on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Vfrb_SPEC;
    pub type Vfrb = crate::EnumBitfieldStruct<u8, Vfrb_SPEC>;
    impl Vfrb {
        #[doc = "0 No valid Frame received on channel B"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Valid Frame received on channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Seoa_SPEC;
    pub type Seoa = crate::EnumBitfieldStruct<u8, Seoa_SPEC>;
    impl Seoa {
        #[doc = "0 No syntax error observed on channel A"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Syntax error observed on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Seob_SPEC;
    pub type Seob = crate::EnumBitfieldStruct<u8, Seob_SPEC>;
    impl Seob {
        #[doc = "0 No syntax error observed on channel B"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Syntax error observed on channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ceoa_SPEC;
    pub type Ceoa = crate::EnumBitfieldStruct<u8, Ceoa_SPEC>;
    impl Ceoa {
        #[doc = "0 No content error observed on channel A"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Content error observed on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ceob_SPEC;
    pub type Ceob = crate::EnumBitfieldStruct<u8, Ceob_SPEC>;
    impl Ceob {
        #[doc = "0 No content error observed on channel B"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Content error observed on channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Svoa_SPEC;
    pub type Svoa = crate::EnumBitfieldStruct<u8, Svoa_SPEC>;
    impl Svoa {
        #[doc = "0 No slot boundary violation observed on channel A"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Slot boundary violation observed on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Svob_SPEC;
    pub type Svob = crate::EnumBitfieldStruct<u8, Svob_SPEC>;
    impl Svob {
        #[doc = "0 No slot boundary violation observed on channel B"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Slot boundary violation observed on channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tcia_SPEC;
    pub type Tcia = crate::EnumBitfieldStruct<u8, Tcia_SPEC>;
    impl Tcia {
        #[doc = "0 No transmission conflict occurred on channel A"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transmission conflict occurred on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tcib_SPEC;
    pub type Tcib = crate::EnumBitfieldStruct<u8, Tcib_SPEC>;
    impl Tcib {
        #[doc = "0 No transmission conflict occurred on channel B"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transmission conflict occurred on channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esa_SPEC;
    pub type Esa = crate::EnumBitfieldStruct<u8, Esa_SPEC>;
    impl Esa {
        #[doc = "0 Bus activity detected in the assigned slot on channel A"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No bus activity detected in the assigned slot on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esb_SPEC;
    pub type Esb = crate::EnumBitfieldStruct<u8, Esb_SPEC>;
    impl Esb {
        #[doc = "0 Bus activity detected in the assigned slot on channel B"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No bus activity detected in the assigned slot on channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mlst_SPEC;
    pub type Mlst = crate::EnumBitfieldStruct<u8, Mlst_SPEC>;
    impl Mlst {
        #[doc = "0 No message lost"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Unprocessed message was overwritten"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fta_SPEC;
    pub type Fta = crate::EnumBitfieldStruct<u8, Fta_SPEC>;
    impl Fta {
        #[doc = "0 No transmission transmitted on channel A"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Data Frame transmitted on channel A in cycle defined by CCS bit field"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ftb_SPEC;
    pub type Ftb = crate::EnumBitfieldStruct<u8, Ftb_SPEC>;
    impl Ftb {
        #[doc = "0 No transmission transmitted on channel B"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Data Frame transmitted on channel B in cycle defined by CCS bit field"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rcis_SPEC;
    pub type Rcis = crate::EnumBitfieldStruct<u8, Rcis_SPEC>;
    impl Rcis {
        #[doc = "0 Frame received on channel B"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Frame received on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sfis_SPEC;
    pub type Sfis = crate::EnumBitfieldStruct<u8, Sfis_SPEC>;
    impl Sfis {
        #[doc = "0 No Startup Frame received"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The received Frame is a startup Frame"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Syns_SPEC;
    pub type Syns = crate::EnumBitfieldStruct<u8, Syns_SPEC>;
    impl Syns {
        #[doc = "0 No SYNC Frame received"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The received Frame is a SYNC Frame"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Nfis_SPEC;
    pub type Nfis = crate::EnumBitfieldStruct<u8, Nfis_SPEC>;
    impl Nfis {
        #[doc = "0 Received Frame is a NULL Frame"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Received Frame is not a NULL Frame"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ppis_SPEC;
    pub type Ppis = crate::EnumBitfieldStruct<u8, Ppis_SPEC>;
    impl Ppis {
        #[doc = "0 Static Segment  The Payload Segment of the received Frame does not contain a Network Management vector or a message ID Dynamic Segment  The Payload Segment of the received Frame does not contain a Network Management vector or a message ID"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Static Segment  Network Management vector at the beginning of the payload Dynamic Segment  Message ID at the beginning of the payload"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbsc1_SPEC;
impl crate::sealed::RegSpec for Mbsc1_SPEC {
    type DataType = u32;
}
#[doc = "Message Buffer Status Changed 1\n resetvalue={Application Reset:0x0}"]
pub type Mbsc1 = crate::RegValueT<Mbsc1_SPEC>;

impl Mbsc1 {
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc16(self) -> crate::common::RegisterFieldBool<16, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc17(self) -> crate::common::RegisterFieldBool<17, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc18(self) -> crate::common::RegisterFieldBool<18, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc19(self) -> crate::common::RegisterFieldBool<19, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc20(self) -> crate::common::RegisterFieldBool<20, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc21(self) -> crate::common::RegisterFieldBool<21, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc22(self) -> crate::common::RegisterFieldBool<22, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc23(self) -> crate::common::RegisterFieldBool<23, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc24(self) -> crate::common::RegisterFieldBool<24, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc25(self) -> crate::common::RegisterFieldBool<25, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc26(self) -> crate::common::RegisterFieldBool<26, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc27(self) -> crate::common::RegisterFieldBool<27, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc28(self) -> crate::common::RegisterFieldBool<28, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc29(self) -> crate::common::RegisterFieldBool<29, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc30(self) -> crate::common::RegisterFieldBool<30, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 31  n   0 31    MBC31. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer 0 to Message Buffer  160 31. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc31(self) -> crate::common::RegisterFieldBool<31, 1, 0, Mbsc1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Mbsc1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Mbsc1 {
    #[inline(always)]
    fn default() -> Mbsc1 {
        <crate::RegValueT<Mbsc1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbsc2_SPEC;
impl crate::sealed::RegSpec for Mbsc2_SPEC {
    type DataType = u32;
}
#[doc = "Message Buffer Status Changed 2\n resetvalue={Application Reset:0x0}"]
pub type Mbsc2 = crate::RegValueT<Mbsc2_SPEC>;

impl Mbsc2 {
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc32(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc33(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc34(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc35(self) -> crate::common::RegisterFieldBool<3, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc36(self) -> crate::common::RegisterFieldBool<4, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc37(self) -> crate::common::RegisterFieldBool<5, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc38(self) -> crate::common::RegisterFieldBool<6, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc39(self) -> crate::common::RegisterFieldBool<7, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc40(self) -> crate::common::RegisterFieldBool<8, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc41(self) -> crate::common::RegisterFieldBool<9, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc42(self) -> crate::common::RegisterFieldBool<10, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc43(self) -> crate::common::RegisterFieldBool<11, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc44(self) -> crate::common::RegisterFieldBool<12, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc45(self) -> crate::common::RegisterFieldBool<13, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc46(self) -> crate::common::RegisterFieldBool<14, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc47(self) -> crate::common::RegisterFieldBool<15, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc48(self) -> crate::common::RegisterFieldBool<16, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc49(self) -> crate::common::RegisterFieldBool<17, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc50(self) -> crate::common::RegisterFieldBool<18, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc51(self) -> crate::common::RegisterFieldBool<19, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc52(self) -> crate::common::RegisterFieldBool<20, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc53(self) -> crate::common::RegisterFieldBool<21, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc54(self) -> crate::common::RegisterFieldBool<22, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc55(self) -> crate::common::RegisterFieldBool<23, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc56(self) -> crate::common::RegisterFieldBool<24, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc57(self) -> crate::common::RegisterFieldBool<25, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc58(self) -> crate::common::RegisterFieldBool<26, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc59(self) -> crate::common::RegisterFieldBool<27, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc60(self) -> crate::common::RegisterFieldBool<28, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc61(self) -> crate::common::RegisterFieldBool<29, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc62(self) -> crate::common::RegisterFieldBool<30, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 63  n   32 63    MBC63. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 32 to Message Buffer  160 63. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc63(self) -> crate::common::RegisterFieldBool<31, 1, 0, Mbsc2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Mbsc2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Mbsc2 {
    #[inline(always)]
    fn default() -> Mbsc2 {
        <crate::RegValueT<Mbsc2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbsc3_SPEC;
impl crate::sealed::RegSpec for Mbsc3_SPEC {
    type DataType = u32;
}
#[doc = "Message Buffer Status Changed 3\n resetvalue={Application Reset:0x0}"]
pub type Mbsc3 = crate::RegValueT<Mbsc3_SPEC>;

impl Mbsc3 {
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc64(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc65(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc66(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc67(self) -> crate::common::RegisterFieldBool<3, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc68(self) -> crate::common::RegisterFieldBool<4, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc69(self) -> crate::common::RegisterFieldBool<5, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc70(self) -> crate::common::RegisterFieldBool<6, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc71(self) -> crate::common::RegisterFieldBool<7, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc72(self) -> crate::common::RegisterFieldBool<8, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc73(self) -> crate::common::RegisterFieldBool<9, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc74(self) -> crate::common::RegisterFieldBool<10, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc75(self) -> crate::common::RegisterFieldBool<11, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc76(self) -> crate::common::RegisterFieldBool<12, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc77(self) -> crate::common::RegisterFieldBool<13, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc78(self) -> crate::common::RegisterFieldBool<14, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc79(self) -> crate::common::RegisterFieldBool<15, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc80(self) -> crate::common::RegisterFieldBool<16, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc81(self) -> crate::common::RegisterFieldBool<17, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc82(self) -> crate::common::RegisterFieldBool<18, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc83(self) -> crate::common::RegisterFieldBool<19, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc84(self) -> crate::common::RegisterFieldBool<20, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc85(self) -> crate::common::RegisterFieldBool<21, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc86(self) -> crate::common::RegisterFieldBool<22, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc87(self) -> crate::common::RegisterFieldBool<23, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc88(self) -> crate::common::RegisterFieldBool<24, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc89(self) -> crate::common::RegisterFieldBool<25, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc90(self) -> crate::common::RegisterFieldBool<26, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc91(self) -> crate::common::RegisterFieldBool<27, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc92(self) -> crate::common::RegisterFieldBool<28, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc93(self) -> crate::common::RegisterFieldBool<29, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc94(self) -> crate::common::RegisterFieldBool<30, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 95  n   64 95    MBC95. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 64 to Message Buffer  160 95. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc95(self) -> crate::common::RegisterFieldBool<31, 1, 0, Mbsc3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Mbsc3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Mbsc3 {
    #[inline(always)]
    fn default() -> Mbsc3 {
        <crate::RegValueT<Mbsc3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mbsc4_SPEC;
impl crate::sealed::RegSpec for Mbsc4_SPEC {
    type DataType = u32;
}
#[doc = "Message Buffer Status Changed 4\n resetvalue={Application Reset:0x0}"]
pub type Mbsc4 = crate::RegValueT<Mbsc4_SPEC>;

impl Mbsc4 {
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc96(self) -> crate::common::RegisterFieldBool<0, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc97(self) -> crate::common::RegisterFieldBool<1, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc98(self) -> crate::common::RegisterFieldBool<2, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc99(self) -> crate::common::RegisterFieldBool<3, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc100(self) -> crate::common::RegisterFieldBool<4, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc101(self) -> crate::common::RegisterFieldBool<5, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc102(self) -> crate::common::RegisterFieldBool<6, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc103(self) -> crate::common::RegisterFieldBool<7, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc104(self) -> crate::common::RegisterFieldBool<8, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc105(self) -> crate::common::RegisterFieldBool<9, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc106(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc107(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc108(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc109(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc110(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc111(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc112(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc113(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc114(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc115(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc116(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc117(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc118(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc119(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc120(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc121(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc122(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc123(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc124(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc125(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc126(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Status Changed 127  n   96 127    MBC127. An MBC flags is set whenever the Message Handler changes on of the        status flags VFRA  VFRB  SEOA  SEOB  CEOA  CEOB  SVOA  SVOB  TCIA  TCIB         ESA  ESB  MLST  FTA  FTB in the Header Section  see   8220 Message Buffer        Status  8221   of the respective Message Buffer  160 96 to Message Buffer  160 127. The        flags are reset when the Header Section of the Message Buffer is        reconfigured or when it has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn mbc127(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Mbsc4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Mbsc4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Mbsc4 {
    #[inline(always)]
    fn default() -> Mbsc4 {
        <crate::RegValueT<Mbsc4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mhdc_SPEC;
impl crate::sealed::RegSpec for Mhdc_SPEC {
    type DataType = u32;
}
#[doc = "MHD Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Mhdc = crate::RegValueT<Mhdc_SPEC>;

impl Mhdc {
    #[doc = "Static Frame Data Length gPayloadLengthStatic    SFDL. Configures the cluster wide payload length for all Frames sent in the        static segment in double byte. The payload length must be identical in        all nodes of a cluster. Valid values are 0 to 127  0 to 7F H  ."]
    #[inline(always)]
    pub fn sfdl(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Mhdc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8, Mhdc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Start of Latest Transmit pLatestTx     SLT. Configures the maximum minislot value allowed before inhibiting Frame        transmission in the dynamic segment of the cycle. There is no        transmission dynamic segment if SLT is reset to zero. Valid values are 0        to 7981  0 to 1F2D H   minislots."]
    #[inline(always)]
    pub fn slt(
        self,
    ) -> crate::common::RegisterField<16, 0x1fff, 1, 0, u16, Mhdc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1fff,1,0,u16, Mhdc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Mhdc {
    #[inline(always)]
    fn default() -> Mhdc {
        <crate::RegValueT<Mhdc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mhdf_SPEC;
impl crate::sealed::RegSpec for Mhdf_SPEC {
    type DataType = u32;
}
#[doc = "Message Handler Constraints Flags\n resetvalue={Application Reset:0x0}"]
pub type Mhdf = crate::RegValueT<Mhdf_SPEC>;

impl Mhdf {
    #[doc = "Status Not Updated Channel A   SNUA. This flag is set by the Communication Controller when the Message Handler  due to overload condition  was not able to update a Message Buffer s status MBS with respect to channel A."]
    #[inline(always)]
    pub fn snua(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mhdf::Snua, Mhdf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,mhdf::Snua, Mhdf_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Status Not Updated Channel B   SNUB. This flag is set by the Communication Controller when the Message Handler  due to overload condition  was not able to update a Message Buffer s status MBS with respect to channel B."]
    #[inline(always)]
    pub fn snub(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, mhdf::Snub, Mhdf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,mhdf::Snub, Mhdf_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Find Sequence Not Finished Channel A   FNFA. This flag is set by the Communication Controller when the Message Handler  due to overload condition  was not able to finish a find sequence  scan of Message RAM for matching Message Buffer  with respect to channel A."]
    #[inline(always)]
    pub fn fnfa(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, mhdf::Fnfa, Mhdf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,mhdf::Fnfa, Mhdf_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Find Sequence Not Finished Channel B   FNFB. This flag is set by the Communication Controller when the Message Handler  due to overload condition  was not able to finish a find sequence  scan of Message RAM for matching Message Buffer  with respect to channel B."]
    #[inline(always)]
    pub fn fnfb(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, mhdf::Fnfb, Mhdf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,mhdf::Fnfb, Mhdf_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transient Buffer Access Failure A   TBFA. This flag is set by the Communication Controller when a read or write access to Transient Buffer A requested by PRT A could not complete within the available time."]
    #[inline(always)]
    pub fn tbfa(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, mhdf::Tbfa, Mhdf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,mhdf::Tbfa, Mhdf_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transient Buffer Access Failure B   TBFB. This flag is set by the Communication Controller when a read or write access to Transient Buffer B requested by PRT B could not complete within the available time."]
    #[inline(always)]
    pub fn tbfb(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, mhdf::Tbfb, Mhdf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,mhdf::Tbfb, Mhdf_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmission Not Started Channel A   TNSA. This flag is set by the CC when the Message Handler was not ready to start a scheduled transmission on channel A at the action point of the configured slot."]
    #[inline(always)]
    pub fn tnsa(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, mhdf::Tnsa, Mhdf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,mhdf::Tnsa, Mhdf_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmission Not Started Channel B   TNSB. This flag is set by the CC when the Message Handler was not ready to start a scheduled transmission on channel B at the action point of the configured slot."]
    #[inline(always)]
    pub fn tnsb(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, mhdf::Tnsb, Mhdf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,mhdf::Tnsb, Mhdf_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Write Attempt to Header Partition   WAHP. Outside  DEFAULT CONFIG  and  CONFIG  state this flag is set by the Communication Controller when the message handler tries to write message data into the Header Partition of the Message RAM due to faulty configuration of a Message Buffer. The write attempt is not executed  to protect the Header Partition from unintended write accesses."]
    #[inline(always)]
    pub fn wahp(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, mhdf::Wahp, Mhdf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,mhdf::Wahp, Mhdf_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Mhdf {
    #[inline(always)]
    fn default() -> Mhdf {
        <crate::RegValueT<Mhdf_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mhdf {
    pub struct Snua_SPEC;
    pub type Snua = crate::EnumBitfieldStruct<u8, Snua_SPEC>;
    impl Snua {
        #[doc = "0 No overload condition occurred when updating MBS for channel A"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBS for channel A not updated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Snub_SPEC;
    pub type Snub = crate::EnumBitfieldStruct<u8, Snub_SPEC>;
    impl Snub {
        #[doc = "0 No overload condition occurred when updating MBS for channel B"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBS for channel B not updated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fnfa_SPEC;
    pub type Fnfa = crate::EnumBitfieldStruct<u8, Fnfa_SPEC>;
    impl Fnfa {
        #[doc = "0 No find sequence not finished for channel A"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Find sequence not finished for channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fnfb_SPEC;
    pub type Fnfb = crate::EnumBitfieldStruct<u8, Fnfb_SPEC>;
    impl Fnfb {
        #[doc = "0 No find sequence not finished for channel B"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Find sequence not finished for channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tbfa_SPEC;
    pub type Tbfa = crate::EnumBitfieldStruct<u8, Tbfa_SPEC>;
    impl Tbfa {
        #[doc = "0 No TBF A access failure"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 TBF A access failure"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tbfb_SPEC;
    pub type Tbfb = crate::EnumBitfieldStruct<u8, Tbfb_SPEC>;
    impl Tbfb {
        #[doc = "0 No Transient Buffer B access failure"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transient Buffer B access failure"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tnsa_SPEC;
    pub type Tnsa = crate::EnumBitfieldStruct<u8, Tnsa_SPEC>;
    impl Tnsa {
        #[doc = "0 No transmission not started on channel A"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transmission not started on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tnsb_SPEC;
    pub type Tnsb = crate::EnumBitfieldStruct<u8, Tnsb_SPEC>;
    impl Tnsb {
        #[doc = "0 No transmission not started on channel B"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transmission not started on channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wahp_SPEC;
    pub type Wahp = crate::EnumBitfieldStruct<u8, Wahp_SPEC>;
    impl Wahp {
        #[doc = "0 No write attempt to Header Partition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write attempt to Header Partition"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mhds_SPEC;
impl crate::sealed::RegSpec for Mhds_SPEC {
    type DataType = u32;
}
#[doc = "Message Handler Status\n resetvalue={Application Reset:0x0}"]
pub type Mhds = crate::RegValueT<Mhds_SPEC>;

impl Mhds {
    #[doc = "ECC Error Input Buffer RAM 1 2   EIBF"]
    #[inline(always)]
    pub fn eibf(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, mhds::Eibf, Mhds_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,mhds::Eibf, Mhds_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ECC Error Output Buffer RAM 1 2   EOBF"]
    #[inline(always)]
    pub fn eobf(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, mhds::Eobf, Mhds_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,mhds::Eobf, Mhds_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ECC Error Message RAM   EMR"]
    #[inline(always)]
    pub fn emr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, mhds::Emr, Mhds_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,mhds::Emr, Mhds_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ECC Error Transient Buffer RAM A   ETBF1"]
    #[inline(always)]
    pub fn etbf1(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, mhds::Etbf1, Mhds_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,mhds::Etbf1, Mhds_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ECC Error Transient Buffer RAM B   ETBF2"]
    #[inline(always)]
    pub fn etbf2(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, mhds::Etbf2, Mhds_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,mhds::Etbf2, Mhds_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Faulty Message Buffer Detected   FMBD"]
    #[inline(always)]
    pub fn fmbd(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, mhds::Fmbd, Mhds_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,mhds::Fmbd, Mhds_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Multiple Faulty Message Buffers detected   MFMB"]
    #[inline(always)]
    pub fn mfmb(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, mhds::Mfmb, Mhds_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,mhds::Mfmb, Mhds_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear all internal RAM s   CRAM. Signals that execution of the CHI command CLEAR RAMS is ongoing  all bits of all internal RAM blocks are written to 0 . The bit is set by CHI command CLEAR RAMS."]
    #[inline(always)]
    pub fn cram(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, mhds::Cram, Mhds_SPEC, crate::common::R> {
        crate::common::RegisterField::<7,0x1,1,0,mhds::Cram, Mhds_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Faulty Message Buffer   FMB. ECC error occurred when reading from the Message Buffer or when transferring data from Input Buffer or Transient Buffer A or Transient Buffer B to the Message Buffer referenced by MHDS.FMB. Value only valid when one of the flags MHDS.EIBF  MHDS.EMR  MHDS.ETBF1  MHDS.ETBF2  and flag MHDS.FMBD is set. Updated only after the Host has reset flag MHDS.FMBD."]
    #[inline(always)]
    pub fn fmb(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, Mhds_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x7f,1,0,u8, Mhds_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Message Buffer Transmitted   MBT. Number of last successfully transmitted Message Buffer. If the Message Buffer is configured for single shot mode  the respective TXR flag in the Transmission Request Registers TXRQ1 to TXRQ4 was reset. MBT is reset when the Communication Controller leaves  CONFIG  state or enters  STARTUP  state."]
    #[inline(always)]
    pub fn mbt(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Mhds_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Mhds_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Message Buffer Updated   MBU. Number of Message Buffer that was updated last. For this Message Buffer the respective NDn  n   0 31  to NDn  n   96 127 and   or MBCn  n   0 31  to MBCn  n   96 127  flag in the New Data Registers NDAT1 to NDAT4 and the Message Buffer Status Changed MBSC1 to MBSC4 registers are also set. MBU is reset when the Communication Controller leaves  CONFIG  state or enters  STARTUP  state."]
    #[inline(always)]
    pub fn mbu(
        self,
    ) -> crate::common::RegisterField<24, 0x7f, 1, 0, u8, Mhds_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x7f,1,0,u8, Mhds_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Mhds {
    #[inline(always)]
    fn default() -> Mhds {
        <crate::RegValueT<Mhds_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mhds {
    pub struct Eibf_SPEC;
    pub type Eibf = crate::EnumBitfieldStruct<u8, Eibf_SPEC>;
    impl Eibf {
        #[doc = "0 No error"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error occurred when reading Input Buffer RAM 1 or Input Buffer RAM 2"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eobf_SPEC;
    pub type Eobf = crate::EnumBitfieldStruct<u8, Eobf_SPEC>;
    impl Eobf {
        #[doc = "0 No error"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error occurred when reading Output Buffer RAM 1 or Output Buffer RAM 2"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Emr_SPEC;
    pub type Emr = crate::EnumBitfieldStruct<u8, Emr_SPEC>;
    impl Emr {
        #[doc = "0 No error"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error occurred when reading the Message RAM"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etbf1_SPEC;
    pub type Etbf1 = crate::EnumBitfieldStruct<u8, Etbf1_SPEC>;
    impl Etbf1 {
        #[doc = "0 No error"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error occurred when reading Transient Buffer RAM A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Etbf2_SPEC;
    pub type Etbf2 = crate::EnumBitfieldStruct<u8, Etbf2_SPEC>;
    impl Etbf2 {
        #[doc = "0 No error"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error occurred when reading Transient Buffer RAM B"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fmbd_SPEC;
    pub type Fmbd = crate::EnumBitfieldStruct<u8, Fmbd_SPEC>;
    impl Fmbd {
        #[doc = "0 No faulty Message Buffer"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Message Buffer referenced by MHDS.FMB holds faulty data due to a ECC error"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mfmb_SPEC;
    pub type Mfmb = crate::EnumBitfieldStruct<u8, Mfmb_SPEC>;
    impl Mfmb {
        #[doc = "0 No additional faulty Message Buffer"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Another faulty Message Buffer was detected while flag MHDS.FMBD is set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cram_SPEC;
    pub type Cram = crate::EnumBitfieldStruct<u8, Cram_SPEC>;
    impl Cram {
        #[doc = "0 No execution of the CHI command CLEAR RAMS"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Execution of the CHI command CLEAR RAMS ongoing"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrc_SPEC;
impl crate::sealed::RegSpec for Mrc_SPEC {
    type DataType = u32;
}
#[doc = "Message RAM Configuration\n resetvalue={Application Reset:0x1800000}"]
pub type Mrc = crate::RegValueT<Mrc_SPEC>;

impl Mrc {
    #[doc = "First Dynamic Buffer   FDB. May be modified in  DEFAULT CONFIG  or  CONFIG  state only."]
    #[inline(always)]
    pub fn fdb(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, mrc::Fdb, Mrc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,mrc::Fdb, Mrc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "First Buffer of FIFO   FFB. May be modified in  DEFAULT CONFIG  or  CONFIG  state only."]
    #[inline(always)]
    pub fn ffb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, mrc::Ffb, Mrc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,mrc::Ffb, Mrc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Last Configured Buffer   LCB. May be only modified in  DEFAULT CONFIG  or  CONFIG  state."]
    #[inline(always)]
    pub fn lcb(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, mrc::Lcb, Mrc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,mrc::Lcb, Mrc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Secure Buffers   SEC. Not evaluated when the Communication Controller is in   8220 DEFAULT CONFIG  8221         or   8220 CONFIG  8221  state. For temporary unlocking see   8220 Host Handling of Errors  8221 . In nodes configured for SYNC Frame transmission or for single slot          mode operation Message Buffer 0  and if SPLM   1  also Message Buffer          1  Reconfiguration of all Message Buffers is always locked"]
    #[inline(always)]
    pub fn sec(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, mrc::Sec, Mrc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,mrc::Sec, Mrc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SYNC Frame Payload Multiplex   SPLM. This bit is only evaluated if the node is configured as sync node  SUCC1.TXSY   1  or for single slot mode operation  SUCC1.TSM   1 . When this bit is set to 1 Message Buffers 0 and 1 are dedicated for SYNC Frame transmission with different payload data on channel A and B. When this bit is reset to 0  SYNC Frames are transmitted from Message Buffer 0 with the same payload data on both channels. Note that the channel filter configuration for Message Buffer 0 resp. Message Buffer 1 has to be chosen accordingly."]
    #[inline(always)]
    pub fn splm(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, mrc::Splm, Mrc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x1,1,0,mrc::Splm, Mrc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Mrc {
    #[inline(always)]
    fn default() -> Mrc {
        <crate::RegValueT<Mrc_SPEC> as RegisterValue<_>>::new(25165824)
    }
}
pub mod mrc {
    pub struct Fdb_SPEC;
    pub type Fdb = crate::EnumBitfieldStruct<u8, Fdb_SPEC>;
    impl Fdb {
        #[doc = "00 No group of Message Buffers exclusively for the static segment configured"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "80  FF No dynamic Message Buffers configured"]
        pub const CONST_128128: Self = Self::new(128);
    }
    pub struct Ffb_SPEC;
    pub type Ffb = crate::EnumBitfieldStruct<u8, Ffb_SPEC>;
    impl Ffb {
        #[doc = "00  7E H Message Buffers from FFB to LCB assigned to the FIFO"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "7F All Message Buffers assigned to the FIFO"]
        pub const CONST_127127: Self = Self::new(127);
        #[doc = "80  FF No Message Buffers assigned to the FIFO"]
        pub const CONST_128128: Self = Self::new(128);
    }
    pub struct Lcb_SPEC;
    pub type Lcb = crate::EnumBitfieldStruct<u8, Lcb_SPEC>;
    impl Lcb {
        #[doc = "01  7F H Number of Message Buffers is LCB   1"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "80  FF H No Message Buffer configured"]
        pub const CONST_128128: Self = Self::new(128);
    }
    pub struct Sec_SPEC;
    pub type Sec = crate::EnumBitfieldStruct<u8, Sec_SPEC>;
    impl Sec {
        #[doc = "00 Reconfiguration of Message Buffers enabled with numbers  lt  FFB enabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Reconfiguration of Message Buffers with numbers  lt  FDB and with numbers   FFB locked and transmission of Message Buffers for static segment with numbers   FDB disabled"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Reconfiguration of all Message Buffers locked"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Reconfiguration of all Message Buffers locked and transmission of Message Buffers for static segment with numbers   FDB disabled"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Splm_SPEC;
    pub type Splm = crate::EnumBitfieldStruct<u8, Splm_SPEC>;
    impl Splm {
        #[doc = "0 Only Message Buffer 0 locked against reconfiguration"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Both Message Buffers 0 and 1 are locked against reconfiguration"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msic1_SPEC;
impl crate::sealed::RegSpec for Msic1_SPEC {
    type DataType = u32;
}
#[doc = "Message Buffer Status Changed Interrupt Control 1\n resetvalue={Application Reset:0x0}"]
pub type Msic1 = crate::RegValueT<Msic1_SPEC>;

impl Msic1 {
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, msic1::Msip0, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,msic1::Msip0, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, msic1::Msip1, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,msic1::Msip1, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, msic1::Msip2, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,msic1::Msip2, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, msic1::Msip3, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,msic1::Msip3, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, msic1::Msip4, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,msic1::Msip4, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, msic1::Msip5, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,msic1::Msip5, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, msic1::Msip6, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,msic1::Msip6, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, msic1::Msip7, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,msic1::Msip7, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, msic1::Msip8, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,msic1::Msip8, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, msic1::Msip9, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,msic1::Msip9, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, msic1::Msip10, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,msic1::Msip10, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, msic1::Msip11, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,msic1::Msip11, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, msic1::Msip12, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,msic1::Msip12, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, msic1::Msip13, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,msic1::Msip13, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, msic1::Msip14, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,msic1::Msip14, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, msic1::Msip15, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,msic1::Msip15, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip16(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, msic1::Msip16, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,msic1::Msip16, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, msic1::Msip17, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,msic1::Msip17, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip18(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, msic1::Msip18, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,msic1::Msip18, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip19(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, msic1::Msip19, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,msic1::Msip19, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip20(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, msic1::Msip20, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,msic1::Msip20, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip21(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, msic1::Msip21, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,msic1::Msip21, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, msic1::Msip22, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,msic1::Msip22, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, msic1::Msip23, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,msic1::Msip23, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, msic1::Msip24, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,msic1::Msip24, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip25(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, msic1::Msip25, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,msic1::Msip25, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip26(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, msic1::Msip26, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,msic1::Msip26, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip27(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, msic1::Msip27, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,msic1::Msip27, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip28(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, msic1::Msip28, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,msic1::Msip28, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip29(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, msic1::Msip29, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,msic1::Msip29, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, msic1::Msip30, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,msic1::Msip30, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 31  n   0 31    MSIP31. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip31(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, msic1::Msip31, Msic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,msic1::Msip31, Msic1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Msic1 {
    #[inline(always)]
    fn default() -> Msic1 {
        <crate::RegValueT<Msic1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod msic1 {
    pub struct Msip0_SPEC;
    pub type Msip0 = crate::EnumBitfieldStruct<u8, Msip0_SPEC>;
    impl Msip0 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip1_SPEC;
    pub type Msip1 = crate::EnumBitfieldStruct<u8, Msip1_SPEC>;
    impl Msip1 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip2_SPEC;
    pub type Msip2 = crate::EnumBitfieldStruct<u8, Msip2_SPEC>;
    impl Msip2 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip3_SPEC;
    pub type Msip3 = crate::EnumBitfieldStruct<u8, Msip3_SPEC>;
    impl Msip3 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip4_SPEC;
    pub type Msip4 = crate::EnumBitfieldStruct<u8, Msip4_SPEC>;
    impl Msip4 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip5_SPEC;
    pub type Msip5 = crate::EnumBitfieldStruct<u8, Msip5_SPEC>;
    impl Msip5 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip6_SPEC;
    pub type Msip6 = crate::EnumBitfieldStruct<u8, Msip6_SPEC>;
    impl Msip6 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip7_SPEC;
    pub type Msip7 = crate::EnumBitfieldStruct<u8, Msip7_SPEC>;
    impl Msip7 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip8_SPEC;
    pub type Msip8 = crate::EnumBitfieldStruct<u8, Msip8_SPEC>;
    impl Msip8 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip9_SPEC;
    pub type Msip9 = crate::EnumBitfieldStruct<u8, Msip9_SPEC>;
    impl Msip9 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip10_SPEC;
    pub type Msip10 = crate::EnumBitfieldStruct<u8, Msip10_SPEC>;
    impl Msip10 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip11_SPEC;
    pub type Msip11 = crate::EnumBitfieldStruct<u8, Msip11_SPEC>;
    impl Msip11 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip12_SPEC;
    pub type Msip12 = crate::EnumBitfieldStruct<u8, Msip12_SPEC>;
    impl Msip12 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip13_SPEC;
    pub type Msip13 = crate::EnumBitfieldStruct<u8, Msip13_SPEC>;
    impl Msip13 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip14_SPEC;
    pub type Msip14 = crate::EnumBitfieldStruct<u8, Msip14_SPEC>;
    impl Msip14 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip15_SPEC;
    pub type Msip15 = crate::EnumBitfieldStruct<u8, Msip15_SPEC>;
    impl Msip15 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip16_SPEC;
    pub type Msip16 = crate::EnumBitfieldStruct<u8, Msip16_SPEC>;
    impl Msip16 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip17_SPEC;
    pub type Msip17 = crate::EnumBitfieldStruct<u8, Msip17_SPEC>;
    impl Msip17 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip18_SPEC;
    pub type Msip18 = crate::EnumBitfieldStruct<u8, Msip18_SPEC>;
    impl Msip18 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip19_SPEC;
    pub type Msip19 = crate::EnumBitfieldStruct<u8, Msip19_SPEC>;
    impl Msip19 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip20_SPEC;
    pub type Msip20 = crate::EnumBitfieldStruct<u8, Msip20_SPEC>;
    impl Msip20 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip21_SPEC;
    pub type Msip21 = crate::EnumBitfieldStruct<u8, Msip21_SPEC>;
    impl Msip21 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip22_SPEC;
    pub type Msip22 = crate::EnumBitfieldStruct<u8, Msip22_SPEC>;
    impl Msip22 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip23_SPEC;
    pub type Msip23 = crate::EnumBitfieldStruct<u8, Msip23_SPEC>;
    impl Msip23 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip24_SPEC;
    pub type Msip24 = crate::EnumBitfieldStruct<u8, Msip24_SPEC>;
    impl Msip24 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip25_SPEC;
    pub type Msip25 = crate::EnumBitfieldStruct<u8, Msip25_SPEC>;
    impl Msip25 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip26_SPEC;
    pub type Msip26 = crate::EnumBitfieldStruct<u8, Msip26_SPEC>;
    impl Msip26 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip27_SPEC;
    pub type Msip27 = crate::EnumBitfieldStruct<u8, Msip27_SPEC>;
    impl Msip27 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip28_SPEC;
    pub type Msip28 = crate::EnumBitfieldStruct<u8, Msip28_SPEC>;
    impl Msip28 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip29_SPEC;
    pub type Msip29 = crate::EnumBitfieldStruct<u8, Msip29_SPEC>;
    impl Msip29 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip30_SPEC;
    pub type Msip30 = crate::EnumBitfieldStruct<u8, Msip30_SPEC>;
    impl Msip30 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip31_SPEC;
    pub type Msip31 = crate::EnumBitfieldStruct<u8, Msip31_SPEC>;
    impl Msip31 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msic2_SPEC;
impl crate::sealed::RegSpec for Msic2_SPEC {
    type DataType = u32;
}
#[doc = "Message Buffer Status Changed Interrupt Control 2\n resetvalue={Application Reset:0x0}"]
pub type Msic2 = crate::RegValueT<Msic2_SPEC>;

impl Msic2 {
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip32(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, msic2::Msip32, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,msic2::Msip32, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip33(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, msic2::Msip33, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,msic2::Msip33, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip34(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, msic2::Msip34, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,msic2::Msip34, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip35(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, msic2::Msip35, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,msic2::Msip35, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip36(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, msic2::Msip36, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,msic2::Msip36, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip37(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, msic2::Msip37, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,msic2::Msip37, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip38(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, msic2::Msip38, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,msic2::Msip38, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip39(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, msic2::Msip39, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,msic2::Msip39, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip40(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, msic2::Msip40, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,msic2::Msip40, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip41(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, msic2::Msip41, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,msic2::Msip41, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip42(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, msic2::Msip42, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,msic2::Msip42, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip43(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, msic2::Msip43, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,msic2::Msip43, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip44(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, msic2::Msip44, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,msic2::Msip44, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip45(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, msic2::Msip45, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,msic2::Msip45, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip46(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, msic2::Msip46, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,msic2::Msip46, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip47(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, msic2::Msip47, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,msic2::Msip47, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip48(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, msic2::Msip48, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,msic2::Msip48, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip49(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, msic2::Msip49, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,msic2::Msip49, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip50(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, msic2::Msip50, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,msic2::Msip50, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip51(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, msic2::Msip51, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,msic2::Msip51, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip52(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, msic2::Msip52, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,msic2::Msip52, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip53(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, msic2::Msip53, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,msic2::Msip53, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip54(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, msic2::Msip54, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,msic2::Msip54, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip55(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, msic2::Msip55, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,msic2::Msip55, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip56(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, msic2::Msip56, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,msic2::Msip56, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip57(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, msic2::Msip57, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,msic2::Msip57, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip58(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, msic2::Msip58, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,msic2::Msip58, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip59(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, msic2::Msip59, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,msic2::Msip59, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip60(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, msic2::Msip60, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,msic2::Msip60, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip61(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, msic2::Msip61, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,msic2::Msip61, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip62(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, msic2::Msip62, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,msic2::Msip62, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 63  n   32 63    MSIP63. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service        request output that becomes active on a Message Buffer Status Changed        Flag becoming active."]
    #[inline(always)]
    pub fn msip63(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, msic2::Msip63, Msic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,msic2::Msip63, Msic2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Msic2 {
    #[inline(always)]
    fn default() -> Msic2 {
        <crate::RegValueT<Msic2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod msic2 {
    pub struct Msip32_SPEC;
    pub type Msip32 = crate::EnumBitfieldStruct<u8, Msip32_SPEC>;
    impl Msip32 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip33_SPEC;
    pub type Msip33 = crate::EnumBitfieldStruct<u8, Msip33_SPEC>;
    impl Msip33 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip34_SPEC;
    pub type Msip34 = crate::EnumBitfieldStruct<u8, Msip34_SPEC>;
    impl Msip34 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip35_SPEC;
    pub type Msip35 = crate::EnumBitfieldStruct<u8, Msip35_SPEC>;
    impl Msip35 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip36_SPEC;
    pub type Msip36 = crate::EnumBitfieldStruct<u8, Msip36_SPEC>;
    impl Msip36 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip37_SPEC;
    pub type Msip37 = crate::EnumBitfieldStruct<u8, Msip37_SPEC>;
    impl Msip37 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip38_SPEC;
    pub type Msip38 = crate::EnumBitfieldStruct<u8, Msip38_SPEC>;
    impl Msip38 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip39_SPEC;
    pub type Msip39 = crate::EnumBitfieldStruct<u8, Msip39_SPEC>;
    impl Msip39 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip40_SPEC;
    pub type Msip40 = crate::EnumBitfieldStruct<u8, Msip40_SPEC>;
    impl Msip40 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip41_SPEC;
    pub type Msip41 = crate::EnumBitfieldStruct<u8, Msip41_SPEC>;
    impl Msip41 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip42_SPEC;
    pub type Msip42 = crate::EnumBitfieldStruct<u8, Msip42_SPEC>;
    impl Msip42 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip43_SPEC;
    pub type Msip43 = crate::EnumBitfieldStruct<u8, Msip43_SPEC>;
    impl Msip43 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip44_SPEC;
    pub type Msip44 = crate::EnumBitfieldStruct<u8, Msip44_SPEC>;
    impl Msip44 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip45_SPEC;
    pub type Msip45 = crate::EnumBitfieldStruct<u8, Msip45_SPEC>;
    impl Msip45 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip46_SPEC;
    pub type Msip46 = crate::EnumBitfieldStruct<u8, Msip46_SPEC>;
    impl Msip46 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip47_SPEC;
    pub type Msip47 = crate::EnumBitfieldStruct<u8, Msip47_SPEC>;
    impl Msip47 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip48_SPEC;
    pub type Msip48 = crate::EnumBitfieldStruct<u8, Msip48_SPEC>;
    impl Msip48 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip49_SPEC;
    pub type Msip49 = crate::EnumBitfieldStruct<u8, Msip49_SPEC>;
    impl Msip49 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip50_SPEC;
    pub type Msip50 = crate::EnumBitfieldStruct<u8, Msip50_SPEC>;
    impl Msip50 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip51_SPEC;
    pub type Msip51 = crate::EnumBitfieldStruct<u8, Msip51_SPEC>;
    impl Msip51 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip52_SPEC;
    pub type Msip52 = crate::EnumBitfieldStruct<u8, Msip52_SPEC>;
    impl Msip52 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip53_SPEC;
    pub type Msip53 = crate::EnumBitfieldStruct<u8, Msip53_SPEC>;
    impl Msip53 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip54_SPEC;
    pub type Msip54 = crate::EnumBitfieldStruct<u8, Msip54_SPEC>;
    impl Msip54 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip55_SPEC;
    pub type Msip55 = crate::EnumBitfieldStruct<u8, Msip55_SPEC>;
    impl Msip55 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip56_SPEC;
    pub type Msip56 = crate::EnumBitfieldStruct<u8, Msip56_SPEC>;
    impl Msip56 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip57_SPEC;
    pub type Msip57 = crate::EnumBitfieldStruct<u8, Msip57_SPEC>;
    impl Msip57 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip58_SPEC;
    pub type Msip58 = crate::EnumBitfieldStruct<u8, Msip58_SPEC>;
    impl Msip58 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip59_SPEC;
    pub type Msip59 = crate::EnumBitfieldStruct<u8, Msip59_SPEC>;
    impl Msip59 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip60_SPEC;
    pub type Msip60 = crate::EnumBitfieldStruct<u8, Msip60_SPEC>;
    impl Msip60 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip61_SPEC;
    pub type Msip61 = crate::EnumBitfieldStruct<u8, Msip61_SPEC>;
    impl Msip61 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip62_SPEC;
    pub type Msip62 = crate::EnumBitfieldStruct<u8, Msip62_SPEC>;
    impl Msip62 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip63_SPEC;
    pub type Msip63 = crate::EnumBitfieldStruct<u8, Msip63_SPEC>;
    impl Msip63 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msic3_SPEC;
impl crate::sealed::RegSpec for Msic3_SPEC {
    type DataType = u32;
}
#[doc = "Message Buffer Status Changed Interrupt Control 3\n resetvalue={Application Reset:0x0}"]
pub type Msic3 = crate::RegValueT<Msic3_SPEC>;

impl Msic3 {
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip64(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, msic3::Msip64, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,msic3::Msip64, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip65(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, msic3::Msip65, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,msic3::Msip65, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip66(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, msic3::Msip66, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,msic3::Msip66, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip67(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, msic3::Msip67, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,msic3::Msip67, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip68(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, msic3::Msip68, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,msic3::Msip68, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip69(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, msic3::Msip69, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,msic3::Msip69, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip70(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, msic3::Msip70, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,msic3::Msip70, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip71(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, msic3::Msip71, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,msic3::Msip71, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip72(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, msic3::Msip72, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,msic3::Msip72, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip73(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, msic3::Msip73, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,msic3::Msip73, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip74(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, msic3::Msip74, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,msic3::Msip74, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip75(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, msic3::Msip75, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,msic3::Msip75, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip76(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, msic3::Msip76, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,msic3::Msip76, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip77(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, msic3::Msip77, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,msic3::Msip77, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip78(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, msic3::Msip78, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,msic3::Msip78, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip79(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, msic3::Msip79, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,msic3::Msip79, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip80(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, msic3::Msip80, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,msic3::Msip80, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip81(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, msic3::Msip81, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,msic3::Msip81, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip82(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, msic3::Msip82, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,msic3::Msip82, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip83(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, msic3::Msip83, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,msic3::Msip83, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip84(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, msic3::Msip84, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,msic3::Msip84, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip85(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, msic3::Msip85, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,msic3::Msip85, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip86(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, msic3::Msip86, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,msic3::Msip86, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip87(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, msic3::Msip87, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,msic3::Msip87, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip88(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, msic3::Msip88, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,msic3::Msip88, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip89(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, msic3::Msip89, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,msic3::Msip89, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip90(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, msic3::Msip90, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,msic3::Msip90, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip91(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, msic3::Msip91, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,msic3::Msip91, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip92(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, msic3::Msip92, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,msic3::Msip92, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip93(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, msic3::Msip93, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,msic3::Msip93, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip94(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, msic3::Msip94, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,msic3::Msip94, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 95  n   64 95    MSIP95. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip95(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, msic3::Msip95, Msic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,msic3::Msip95, Msic3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Msic3 {
    #[inline(always)]
    fn default() -> Msic3 {
        <crate::RegValueT<Msic3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod msic3 {
    pub struct Msip64_SPEC;
    pub type Msip64 = crate::EnumBitfieldStruct<u8, Msip64_SPEC>;
    impl Msip64 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip65_SPEC;
    pub type Msip65 = crate::EnumBitfieldStruct<u8, Msip65_SPEC>;
    impl Msip65 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip66_SPEC;
    pub type Msip66 = crate::EnumBitfieldStruct<u8, Msip66_SPEC>;
    impl Msip66 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip67_SPEC;
    pub type Msip67 = crate::EnumBitfieldStruct<u8, Msip67_SPEC>;
    impl Msip67 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip68_SPEC;
    pub type Msip68 = crate::EnumBitfieldStruct<u8, Msip68_SPEC>;
    impl Msip68 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip69_SPEC;
    pub type Msip69 = crate::EnumBitfieldStruct<u8, Msip69_SPEC>;
    impl Msip69 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip70_SPEC;
    pub type Msip70 = crate::EnumBitfieldStruct<u8, Msip70_SPEC>;
    impl Msip70 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip71_SPEC;
    pub type Msip71 = crate::EnumBitfieldStruct<u8, Msip71_SPEC>;
    impl Msip71 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip72_SPEC;
    pub type Msip72 = crate::EnumBitfieldStruct<u8, Msip72_SPEC>;
    impl Msip72 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip73_SPEC;
    pub type Msip73 = crate::EnumBitfieldStruct<u8, Msip73_SPEC>;
    impl Msip73 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip74_SPEC;
    pub type Msip74 = crate::EnumBitfieldStruct<u8, Msip74_SPEC>;
    impl Msip74 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip75_SPEC;
    pub type Msip75 = crate::EnumBitfieldStruct<u8, Msip75_SPEC>;
    impl Msip75 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip76_SPEC;
    pub type Msip76 = crate::EnumBitfieldStruct<u8, Msip76_SPEC>;
    impl Msip76 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip77_SPEC;
    pub type Msip77 = crate::EnumBitfieldStruct<u8, Msip77_SPEC>;
    impl Msip77 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip78_SPEC;
    pub type Msip78 = crate::EnumBitfieldStruct<u8, Msip78_SPEC>;
    impl Msip78 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip79_SPEC;
    pub type Msip79 = crate::EnumBitfieldStruct<u8, Msip79_SPEC>;
    impl Msip79 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip80_SPEC;
    pub type Msip80 = crate::EnumBitfieldStruct<u8, Msip80_SPEC>;
    impl Msip80 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip81_SPEC;
    pub type Msip81 = crate::EnumBitfieldStruct<u8, Msip81_SPEC>;
    impl Msip81 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip82_SPEC;
    pub type Msip82 = crate::EnumBitfieldStruct<u8, Msip82_SPEC>;
    impl Msip82 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip83_SPEC;
    pub type Msip83 = crate::EnumBitfieldStruct<u8, Msip83_SPEC>;
    impl Msip83 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip84_SPEC;
    pub type Msip84 = crate::EnumBitfieldStruct<u8, Msip84_SPEC>;
    impl Msip84 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip85_SPEC;
    pub type Msip85 = crate::EnumBitfieldStruct<u8, Msip85_SPEC>;
    impl Msip85 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip86_SPEC;
    pub type Msip86 = crate::EnumBitfieldStruct<u8, Msip86_SPEC>;
    impl Msip86 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip87_SPEC;
    pub type Msip87 = crate::EnumBitfieldStruct<u8, Msip87_SPEC>;
    impl Msip87 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip88_SPEC;
    pub type Msip88 = crate::EnumBitfieldStruct<u8, Msip88_SPEC>;
    impl Msip88 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip89_SPEC;
    pub type Msip89 = crate::EnumBitfieldStruct<u8, Msip89_SPEC>;
    impl Msip89 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip90_SPEC;
    pub type Msip90 = crate::EnumBitfieldStruct<u8, Msip90_SPEC>;
    impl Msip90 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip91_SPEC;
    pub type Msip91 = crate::EnumBitfieldStruct<u8, Msip91_SPEC>;
    impl Msip91 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip92_SPEC;
    pub type Msip92 = crate::EnumBitfieldStruct<u8, Msip92_SPEC>;
    impl Msip92 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip93_SPEC;
    pub type Msip93 = crate::EnumBitfieldStruct<u8, Msip93_SPEC>;
    impl Msip93 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip94_SPEC;
    pub type Msip94 = crate::EnumBitfieldStruct<u8, Msip94_SPEC>;
    impl Msip94 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip95_SPEC;
    pub type Msip95 = crate::EnumBitfieldStruct<u8, Msip95_SPEC>;
    impl Msip95 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msic4_SPEC;
impl crate::sealed::RegSpec for Msic4_SPEC {
    type DataType = u32;
}
#[doc = "Message Buffer Status Changed Interrupt Control 4\n resetvalue={Application Reset:0x0}"]
pub type Msic4 = crate::RegValueT<Msic4_SPEC>;

impl Msic4 {
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip96(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, msic4::Msip96, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,msic4::Msip96, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip97(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, msic4::Msip97, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,msic4::Msip97, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip98(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, msic4::Msip98, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,msic4::Msip98, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip99(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, msic4::Msip99, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,msic4::Msip99, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip100(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, msic4::Msip100, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,msic4::Msip100, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip101(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, msic4::Msip101, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,msic4::Msip101, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip102(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, msic4::Msip102, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,msic4::Msip102, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip103(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, msic4::Msip103, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,msic4::Msip103, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip104(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, msic4::Msip104, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,msic4::Msip104, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip105(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, msic4::Msip105, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,msic4::Msip105, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip106(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, msic4::Msip106, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,msic4::Msip106, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip107(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, msic4::Msip107, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,msic4::Msip107, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip108(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, msic4::Msip108, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,msic4::Msip108, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip109(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, msic4::Msip109, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,msic4::Msip109, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip110(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, msic4::Msip110, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,msic4::Msip110, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip111(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, msic4::Msip111, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,msic4::Msip111, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip112(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, msic4::Msip112, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,msic4::Msip112, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip113(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, msic4::Msip113, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,msic4::Msip113, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip114(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, msic4::Msip114, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,msic4::Msip114, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip115(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, msic4::Msip115, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,msic4::Msip115, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip116(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, msic4::Msip116, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,msic4::Msip116, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip117(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, msic4::Msip117, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,msic4::Msip117, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip118(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, msic4::Msip118, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,msic4::Msip118, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip119(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, msic4::Msip119, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,msic4::Msip119, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip120(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, msic4::Msip120, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,msic4::Msip120, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip121(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, msic4::Msip121, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,msic4::Msip121, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip122(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, msic4::Msip122, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,msic4::Msip122, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip123(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, msic4::Msip123, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,msic4::Msip123, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip124(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, msic4::Msip124, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,msic4::Msip124, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip125(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, msic4::Msip125, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,msic4::Msip125, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip126(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, msic4::Msip126, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,msic4::Msip126, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Changed Interrupt Pointer 127  n   96 127    MSIP127. MSIPn determines the interrupt  MBSC0SRC or MBSC1SRC  of the service request output that becomes active on a Message Buffer Status Changed Flag becoming active."]
    #[inline(always)]
    pub fn msip127(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, msic4::Msip127, Msic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,msic4::Msip127, Msic4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Msic4 {
    #[inline(always)]
    fn default() -> Msic4 {
        <crate::RegValueT<Msic4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod msic4 {
    pub struct Msip96_SPEC;
    pub type Msip96 = crate::EnumBitfieldStruct<u8, Msip96_SPEC>;
    impl Msip96 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip97_SPEC;
    pub type Msip97 = crate::EnumBitfieldStruct<u8, Msip97_SPEC>;
    impl Msip97 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip98_SPEC;
    pub type Msip98 = crate::EnumBitfieldStruct<u8, Msip98_SPEC>;
    impl Msip98 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip99_SPEC;
    pub type Msip99 = crate::EnumBitfieldStruct<u8, Msip99_SPEC>;
    impl Msip99 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip100_SPEC;
    pub type Msip100 = crate::EnumBitfieldStruct<u8, Msip100_SPEC>;
    impl Msip100 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip101_SPEC;
    pub type Msip101 = crate::EnumBitfieldStruct<u8, Msip101_SPEC>;
    impl Msip101 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip102_SPEC;
    pub type Msip102 = crate::EnumBitfieldStruct<u8, Msip102_SPEC>;
    impl Msip102 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip103_SPEC;
    pub type Msip103 = crate::EnumBitfieldStruct<u8, Msip103_SPEC>;
    impl Msip103 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip104_SPEC;
    pub type Msip104 = crate::EnumBitfieldStruct<u8, Msip104_SPEC>;
    impl Msip104 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip105_SPEC;
    pub type Msip105 = crate::EnumBitfieldStruct<u8, Msip105_SPEC>;
    impl Msip105 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip106_SPEC;
    pub type Msip106 = crate::EnumBitfieldStruct<u8, Msip106_SPEC>;
    impl Msip106 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip107_SPEC;
    pub type Msip107 = crate::EnumBitfieldStruct<u8, Msip107_SPEC>;
    impl Msip107 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip108_SPEC;
    pub type Msip108 = crate::EnumBitfieldStruct<u8, Msip108_SPEC>;
    impl Msip108 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip109_SPEC;
    pub type Msip109 = crate::EnumBitfieldStruct<u8, Msip109_SPEC>;
    impl Msip109 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip110_SPEC;
    pub type Msip110 = crate::EnumBitfieldStruct<u8, Msip110_SPEC>;
    impl Msip110 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip111_SPEC;
    pub type Msip111 = crate::EnumBitfieldStruct<u8, Msip111_SPEC>;
    impl Msip111 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip112_SPEC;
    pub type Msip112 = crate::EnumBitfieldStruct<u8, Msip112_SPEC>;
    impl Msip112 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip113_SPEC;
    pub type Msip113 = crate::EnumBitfieldStruct<u8, Msip113_SPEC>;
    impl Msip113 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip114_SPEC;
    pub type Msip114 = crate::EnumBitfieldStruct<u8, Msip114_SPEC>;
    impl Msip114 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip115_SPEC;
    pub type Msip115 = crate::EnumBitfieldStruct<u8, Msip115_SPEC>;
    impl Msip115 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip116_SPEC;
    pub type Msip116 = crate::EnumBitfieldStruct<u8, Msip116_SPEC>;
    impl Msip116 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip117_SPEC;
    pub type Msip117 = crate::EnumBitfieldStruct<u8, Msip117_SPEC>;
    impl Msip117 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip118_SPEC;
    pub type Msip118 = crate::EnumBitfieldStruct<u8, Msip118_SPEC>;
    impl Msip118 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip119_SPEC;
    pub type Msip119 = crate::EnumBitfieldStruct<u8, Msip119_SPEC>;
    impl Msip119 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip120_SPEC;
    pub type Msip120 = crate::EnumBitfieldStruct<u8, Msip120_SPEC>;
    impl Msip120 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip121_SPEC;
    pub type Msip121 = crate::EnumBitfieldStruct<u8, Msip121_SPEC>;
    impl Msip121 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip122_SPEC;
    pub type Msip122 = crate::EnumBitfieldStruct<u8, Msip122_SPEC>;
    impl Msip122 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip123_SPEC;
    pub type Msip123 = crate::EnumBitfieldStruct<u8, Msip123_SPEC>;
    impl Msip123 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip124_SPEC;
    pub type Msip124 = crate::EnumBitfieldStruct<u8, Msip124_SPEC>;
    impl Msip124 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip125_SPEC;
    pub type Msip125 = crate::EnumBitfieldStruct<u8, Msip125_SPEC>;
    impl Msip125 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip126_SPEC;
    pub type Msip126 = crate::EnumBitfieldStruct<u8, Msip126_SPEC>;
    impl Msip126 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msip127_SPEC;
    pub type Msip127 = crate::EnumBitfieldStruct<u8, Msip127_SPEC>;
    impl Msip127 {
        #[doc = "0 MBSC0SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MBSC1SRC selected for Message Buffer Status Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtccv_SPEC;
impl crate::sealed::RegSpec for Mtccv_SPEC {
    type DataType = u32;
}
#[doc = "Macrotick and Cycle Counter Value\n resetvalue={Application Reset:0x0}"]
pub type Mtccv = crate::RegValueT<Mtccv_SPEC>;

impl Mtccv {
    #[doc = "Macrotick Value vMacrotick    MTV. Current Macrotick value. The value is incremented by the Communication Controller and reset at the start of a communication cycle. Valid values are 0 to 16000  0 H to 3E80 H  ."]
    #[inline(always)]
    pub fn mtv(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Mtccv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Mtccv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Cycle Counter Value vCycleCounter    CCV. Current cycle counter value. The value is incremented by the Communication Controller at the start of a communication cycle. Valid values are 0 to 63  0 H to 3F H  ."]
    #[inline(always)]
    pub fn ccv(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Mtccv_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Mtccv_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Mtccv {
    #[inline(always)]
    fn default() -> Mtccv {
        <crate::RegValueT<Mtccv_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ndat1_SPEC;
impl crate::sealed::RegSpec for Ndat1_SPEC {
    type DataType = u32;
}
#[doc = "New Data Register 1\n resetvalue={Application Reset:0x0}"]
pub type Ndat1 = crate::RegValueT<Ndat1_SPEC>;

impl Ndat1 {
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd16(self) -> crate::common::RegisterFieldBool<16, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd17(self) -> crate::common::RegisterFieldBool<17, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd18(self) -> crate::common::RegisterFieldBool<18, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd19(self) -> crate::common::RegisterFieldBool<19, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd20(self) -> crate::common::RegisterFieldBool<20, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd21(self) -> crate::common::RegisterFieldBool<21, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd22(self) -> crate::common::RegisterFieldBool<22, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd23(self) -> crate::common::RegisterFieldBool<23, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd24(self) -> crate::common::RegisterFieldBool<24, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd25(self) -> crate::common::RegisterFieldBool<25, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd26(self) -> crate::common::RegisterFieldBool<26, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd27(self) -> crate::common::RegisterFieldBool<27, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd28(self) -> crate::common::RegisterFieldBool<28, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd29(self) -> crate::common::RegisterFieldBool<29, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd30(self) -> crate::common::RegisterFieldBool<30, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 31  n   0 31    ND31. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd31(self) -> crate::common::RegisterFieldBool<31, 1, 0, Ndat1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ndat1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ndat1 {
    #[inline(always)]
    fn default() -> Ndat1 {
        <crate::RegValueT<Ndat1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ndat2_SPEC;
impl crate::sealed::RegSpec for Ndat2_SPEC {
    type DataType = u32;
}
#[doc = "New Data Register 2\n resetvalue={Application Reset:0x0}"]
pub type Ndat2 = crate::RegValueT<Ndat2_SPEC>;

impl Ndat2 {
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd32(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd33(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd34(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd35(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd36(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd37(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd38(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd39(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd40(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd41(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd42(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd43(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd44(self) -> crate::common::RegisterFieldBool<12, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd45(self) -> crate::common::RegisterFieldBool<13, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd46(self) -> crate::common::RegisterFieldBool<14, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd47(self) -> crate::common::RegisterFieldBool<15, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd48(self) -> crate::common::RegisterFieldBool<16, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd49(self) -> crate::common::RegisterFieldBool<17, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd50(self) -> crate::common::RegisterFieldBool<18, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd51(self) -> crate::common::RegisterFieldBool<19, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd52(self) -> crate::common::RegisterFieldBool<20, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd53(self) -> crate::common::RegisterFieldBool<21, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd54(self) -> crate::common::RegisterFieldBool<22, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd55(self) -> crate::common::RegisterFieldBool<23, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd56(self) -> crate::common::RegisterFieldBool<24, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd57(self) -> crate::common::RegisterFieldBool<25, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd58(self) -> crate::common::RegisterFieldBool<26, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd59(self) -> crate::common::RegisterFieldBool<27, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd60(self) -> crate::common::RegisterFieldBool<28, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd61(self) -> crate::common::RegisterFieldBool<29, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd62(self) -> crate::common::RegisterFieldBool<30, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 63  n   32 63    ND63. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd63(self) -> crate::common::RegisterFieldBool<31, 1, 0, Ndat2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ndat2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ndat2 {
    #[inline(always)]
    fn default() -> Ndat2 {
        <crate::RegValueT<Ndat2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ndat3_SPEC;
impl crate::sealed::RegSpec for Ndat3_SPEC {
    type DataType = u32;
}
#[doc = "New Data Register 3\n resetvalue={Application Reset:0x0}"]
pub type Ndat3 = crate::RegValueT<Ndat3_SPEC>;

impl Ndat3 {
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd64(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd65(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd66(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd67(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd68(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd69(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd70(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd71(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd72(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd73(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd74(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd75(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd76(self) -> crate::common::RegisterFieldBool<12, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd77(self) -> crate::common::RegisterFieldBool<13, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd78(self) -> crate::common::RegisterFieldBool<14, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd79(self) -> crate::common::RegisterFieldBool<15, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd80(self) -> crate::common::RegisterFieldBool<16, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd81(self) -> crate::common::RegisterFieldBool<17, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd82(self) -> crate::common::RegisterFieldBool<18, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd83(self) -> crate::common::RegisterFieldBool<19, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd84(self) -> crate::common::RegisterFieldBool<20, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd85(self) -> crate::common::RegisterFieldBool<21, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd86(self) -> crate::common::RegisterFieldBool<22, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd87(self) -> crate::common::RegisterFieldBool<23, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd88(self) -> crate::common::RegisterFieldBool<24, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd89(self) -> crate::common::RegisterFieldBool<25, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd90(self) -> crate::common::RegisterFieldBool<26, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd91(self) -> crate::common::RegisterFieldBool<27, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd92(self) -> crate::common::RegisterFieldBool<28, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd93(self) -> crate::common::RegisterFieldBool<29, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd94(self) -> crate::common::RegisterFieldBool<30, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 95  n   64 95    ND95. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd95(self) -> crate::common::RegisterFieldBool<31, 1, 0, Ndat3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ndat3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ndat3 {
    #[inline(always)]
    fn default() -> Ndat3 {
        <crate::RegValueT<Ndat3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ndat4_SPEC;
impl crate::sealed::RegSpec for Ndat4_SPEC {
    type DataType = u32;
}
#[doc = "New Data Register 4\n resetvalue={Application Reset:0x0}"]
pub type Ndat4 = crate::RegValueT<Ndat4_SPEC>;

impl Ndat4 {
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd96(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd97(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd98(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd99(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd100(self) -> crate::common::RegisterFieldBool<4, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd101(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd102(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd103(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd104(self) -> crate::common::RegisterFieldBool<8, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd105(self) -> crate::common::RegisterFieldBool<9, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd106(self) -> crate::common::RegisterFieldBool<10, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd107(self) -> crate::common::RegisterFieldBool<11, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd108(self) -> crate::common::RegisterFieldBool<12, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd109(self) -> crate::common::RegisterFieldBool<13, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd110(self) -> crate::common::RegisterFieldBool<14, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd111(self) -> crate::common::RegisterFieldBool<15, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd112(self) -> crate::common::RegisterFieldBool<16, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd113(self) -> crate::common::RegisterFieldBool<17, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd114(self) -> crate::common::RegisterFieldBool<18, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd115(self) -> crate::common::RegisterFieldBool<19, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd116(self) -> crate::common::RegisterFieldBool<20, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd117(self) -> crate::common::RegisterFieldBool<21, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd118(self) -> crate::common::RegisterFieldBool<22, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd119(self) -> crate::common::RegisterFieldBool<23, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd120(self) -> crate::common::RegisterFieldBool<24, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd121(self) -> crate::common::RegisterFieldBool<25, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd122(self) -> crate::common::RegisterFieldBool<26, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd123(self) -> crate::common::RegisterFieldBool<27, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd124(self) -> crate::common::RegisterFieldBool<28, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd125(self) -> crate::common::RegisterFieldBool<29, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd126(self) -> crate::common::RegisterFieldBool<30, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "New Data 127  n   96 127    ND127. The flags are set when a valid received Data Frame matches the Message Buffer s filter configuration  independent of the payload length received or the payload length configured for that Message Buffer. The flags are not set after reception of NULL Frames except for Message Buffers belonging to the receive FIFO. An ND flag is reset when the Header Section of the corresponding Message Buffer is reconfigured or when the Data Section has been transferred to the Output Buffer."]
    #[inline(always)]
    pub fn nd127(self) -> crate::common::RegisterFieldBool<31, 1, 0, Ndat4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ndat4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ndat4 {
    #[inline(always)]
    fn default() -> Ndat4 {
        <crate::RegValueT<Ndat4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ndic1_SPEC;
impl crate::sealed::RegSpec for Ndic1_SPEC {
    type DataType = u32;
}
#[doc = "New Data Interrupt Control 1\n resetvalue={Application Reset:0x0}"]
pub type Ndic1 = crate::RegValueT<Ndic1_SPEC>;

impl Ndic1 {
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ndic1::Ndip0, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,ndic1::Ndip0, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ndic1::Ndip1, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,ndic1::Ndip1, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ndic1::Ndip2, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,ndic1::Ndip2, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ndic1::Ndip3, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,ndic1::Ndip3, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ndic1::Ndip4, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,ndic1::Ndip4, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ndic1::Ndip5, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,ndic1::Ndip5, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, ndic1::Ndip6, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,ndic1::Ndip6, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, ndic1::Ndip7, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,ndic1::Ndip7, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, ndic1::Ndip8, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,ndic1::Ndip8, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, ndic1::Ndip9, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,ndic1::Ndip9, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, ndic1::Ndip10, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,ndic1::Ndip10, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, ndic1::Ndip11, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,ndic1::Ndip11, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, ndic1::Ndip12, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,ndic1::Ndip12, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, ndic1::Ndip13, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,ndic1::Ndip13, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, ndic1::Ndip14, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,ndic1::Ndip14, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, ndic1::Ndip15, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,ndic1::Ndip15, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip16(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, ndic1::Ndip16, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,ndic1::Ndip16, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, ndic1::Ndip17, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,ndic1::Ndip17, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip18(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, ndic1::Ndip18, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,ndic1::Ndip18, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip19(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, ndic1::Ndip19, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,ndic1::Ndip19, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip20(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, ndic1::Ndip20, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,ndic1::Ndip20, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip21(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, ndic1::Ndip21, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,ndic1::Ndip21, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, ndic1::Ndip22, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,ndic1::Ndip22, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, ndic1::Ndip23, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,ndic1::Ndip23, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, ndic1::Ndip24, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,ndic1::Ndip24, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip25(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, ndic1::Ndip25, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,ndic1::Ndip25, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip26(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, ndic1::Ndip26, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,ndic1::Ndip26, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip27(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, ndic1::Ndip27, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,ndic1::Ndip27, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip28(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, ndic1::Ndip28, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,ndic1::Ndip28, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip29(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, ndic1::Ndip29, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,ndic1::Ndip29, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, ndic1::Ndip30, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,ndic1::Ndip30, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 31  n   0 31    NDIP31. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip31(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, ndic1::Ndip31, Ndic1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,ndic1::Ndip31, Ndic1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ndic1 {
    #[inline(always)]
    fn default() -> Ndic1 {
        <crate::RegValueT<Ndic1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ndic1 {
    pub struct Ndip0_SPEC;
    pub type Ndip0 = crate::EnumBitfieldStruct<u8, Ndip0_SPEC>;
    impl Ndip0 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip1_SPEC;
    pub type Ndip1 = crate::EnumBitfieldStruct<u8, Ndip1_SPEC>;
    impl Ndip1 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip2_SPEC;
    pub type Ndip2 = crate::EnumBitfieldStruct<u8, Ndip2_SPEC>;
    impl Ndip2 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip3_SPEC;
    pub type Ndip3 = crate::EnumBitfieldStruct<u8, Ndip3_SPEC>;
    impl Ndip3 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip4_SPEC;
    pub type Ndip4 = crate::EnumBitfieldStruct<u8, Ndip4_SPEC>;
    impl Ndip4 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip5_SPEC;
    pub type Ndip5 = crate::EnumBitfieldStruct<u8, Ndip5_SPEC>;
    impl Ndip5 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip6_SPEC;
    pub type Ndip6 = crate::EnumBitfieldStruct<u8, Ndip6_SPEC>;
    impl Ndip6 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip7_SPEC;
    pub type Ndip7 = crate::EnumBitfieldStruct<u8, Ndip7_SPEC>;
    impl Ndip7 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip8_SPEC;
    pub type Ndip8 = crate::EnumBitfieldStruct<u8, Ndip8_SPEC>;
    impl Ndip8 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip9_SPEC;
    pub type Ndip9 = crate::EnumBitfieldStruct<u8, Ndip9_SPEC>;
    impl Ndip9 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip10_SPEC;
    pub type Ndip10 = crate::EnumBitfieldStruct<u8, Ndip10_SPEC>;
    impl Ndip10 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip11_SPEC;
    pub type Ndip11 = crate::EnumBitfieldStruct<u8, Ndip11_SPEC>;
    impl Ndip11 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip12_SPEC;
    pub type Ndip12 = crate::EnumBitfieldStruct<u8, Ndip12_SPEC>;
    impl Ndip12 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip13_SPEC;
    pub type Ndip13 = crate::EnumBitfieldStruct<u8, Ndip13_SPEC>;
    impl Ndip13 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip14_SPEC;
    pub type Ndip14 = crate::EnumBitfieldStruct<u8, Ndip14_SPEC>;
    impl Ndip14 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip15_SPEC;
    pub type Ndip15 = crate::EnumBitfieldStruct<u8, Ndip15_SPEC>;
    impl Ndip15 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip16_SPEC;
    pub type Ndip16 = crate::EnumBitfieldStruct<u8, Ndip16_SPEC>;
    impl Ndip16 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip17_SPEC;
    pub type Ndip17 = crate::EnumBitfieldStruct<u8, Ndip17_SPEC>;
    impl Ndip17 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip18_SPEC;
    pub type Ndip18 = crate::EnumBitfieldStruct<u8, Ndip18_SPEC>;
    impl Ndip18 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip19_SPEC;
    pub type Ndip19 = crate::EnumBitfieldStruct<u8, Ndip19_SPEC>;
    impl Ndip19 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip20_SPEC;
    pub type Ndip20 = crate::EnumBitfieldStruct<u8, Ndip20_SPEC>;
    impl Ndip20 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip21_SPEC;
    pub type Ndip21 = crate::EnumBitfieldStruct<u8, Ndip21_SPEC>;
    impl Ndip21 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip22_SPEC;
    pub type Ndip22 = crate::EnumBitfieldStruct<u8, Ndip22_SPEC>;
    impl Ndip22 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip23_SPEC;
    pub type Ndip23 = crate::EnumBitfieldStruct<u8, Ndip23_SPEC>;
    impl Ndip23 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip24_SPEC;
    pub type Ndip24 = crate::EnumBitfieldStruct<u8, Ndip24_SPEC>;
    impl Ndip24 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip25_SPEC;
    pub type Ndip25 = crate::EnumBitfieldStruct<u8, Ndip25_SPEC>;
    impl Ndip25 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip26_SPEC;
    pub type Ndip26 = crate::EnumBitfieldStruct<u8, Ndip26_SPEC>;
    impl Ndip26 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip27_SPEC;
    pub type Ndip27 = crate::EnumBitfieldStruct<u8, Ndip27_SPEC>;
    impl Ndip27 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip28_SPEC;
    pub type Ndip28 = crate::EnumBitfieldStruct<u8, Ndip28_SPEC>;
    impl Ndip28 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip29_SPEC;
    pub type Ndip29 = crate::EnumBitfieldStruct<u8, Ndip29_SPEC>;
    impl Ndip29 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip30_SPEC;
    pub type Ndip30 = crate::EnumBitfieldStruct<u8, Ndip30_SPEC>;
    impl Ndip30 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip31_SPEC;
    pub type Ndip31 = crate::EnumBitfieldStruct<u8, Ndip31_SPEC>;
    impl Ndip31 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ndic2_SPEC;
impl crate::sealed::RegSpec for Ndic2_SPEC {
    type DataType = u32;
}
#[doc = "New Data Interrupt Control 2\n resetvalue={Application Reset:0x0}"]
pub type Ndic2 = crate::RegValueT<Ndic2_SPEC>;

impl Ndic2 {
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip32(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ndic2::Ndip32, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,ndic2::Ndip32, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip33(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ndic2::Ndip33, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,ndic2::Ndip33, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip34(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ndic2::Ndip34, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,ndic2::Ndip34, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip35(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ndic2::Ndip35, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,ndic2::Ndip35, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip36(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ndic2::Ndip36, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,ndic2::Ndip36, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip37(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ndic2::Ndip37, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,ndic2::Ndip37, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip38(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, ndic2::Ndip38, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,ndic2::Ndip38, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip39(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, ndic2::Ndip39, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,ndic2::Ndip39, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip40(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, ndic2::Ndip40, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,ndic2::Ndip40, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip41(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, ndic2::Ndip41, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,ndic2::Ndip41, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip42(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, ndic2::Ndip42, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,ndic2::Ndip42, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip43(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, ndic2::Ndip43, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,ndic2::Ndip43, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip44(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, ndic2::Ndip44, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,ndic2::Ndip44, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip45(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, ndic2::Ndip45, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,ndic2::Ndip45, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip46(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, ndic2::Ndip46, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,ndic2::Ndip46, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip47(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, ndic2::Ndip47, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,ndic2::Ndip47, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip48(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, ndic2::Ndip48, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,ndic2::Ndip48, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip49(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, ndic2::Ndip49, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,ndic2::Ndip49, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip50(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, ndic2::Ndip50, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,ndic2::Ndip50, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip51(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, ndic2::Ndip51, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,ndic2::Ndip51, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip52(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, ndic2::Ndip52, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,ndic2::Ndip52, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip53(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, ndic2::Ndip53, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,ndic2::Ndip53, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip54(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, ndic2::Ndip54, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,ndic2::Ndip54, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip55(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, ndic2::Ndip55, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,ndic2::Ndip55, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip56(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, ndic2::Ndip56, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,ndic2::Ndip56, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip57(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, ndic2::Ndip57, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,ndic2::Ndip57, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip58(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, ndic2::Ndip58, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,ndic2::Ndip58, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip59(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, ndic2::Ndip59, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,ndic2::Ndip59, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip60(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, ndic2::Ndip60, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,ndic2::Ndip60, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip61(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, ndic2::Ndip61, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,ndic2::Ndip61, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip62(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, ndic2::Ndip62, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,ndic2::Ndip62, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 63  n   32 63    NDIP63. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip63(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, ndic2::Ndip63, Ndic2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,ndic2::Ndip63, Ndic2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ndic2 {
    #[inline(always)]
    fn default() -> Ndic2 {
        <crate::RegValueT<Ndic2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ndic2 {
    pub struct Ndip32_SPEC;
    pub type Ndip32 = crate::EnumBitfieldStruct<u8, Ndip32_SPEC>;
    impl Ndip32 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip33_SPEC;
    pub type Ndip33 = crate::EnumBitfieldStruct<u8, Ndip33_SPEC>;
    impl Ndip33 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip34_SPEC;
    pub type Ndip34 = crate::EnumBitfieldStruct<u8, Ndip34_SPEC>;
    impl Ndip34 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip35_SPEC;
    pub type Ndip35 = crate::EnumBitfieldStruct<u8, Ndip35_SPEC>;
    impl Ndip35 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip36_SPEC;
    pub type Ndip36 = crate::EnumBitfieldStruct<u8, Ndip36_SPEC>;
    impl Ndip36 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip37_SPEC;
    pub type Ndip37 = crate::EnumBitfieldStruct<u8, Ndip37_SPEC>;
    impl Ndip37 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip38_SPEC;
    pub type Ndip38 = crate::EnumBitfieldStruct<u8, Ndip38_SPEC>;
    impl Ndip38 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip39_SPEC;
    pub type Ndip39 = crate::EnumBitfieldStruct<u8, Ndip39_SPEC>;
    impl Ndip39 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip40_SPEC;
    pub type Ndip40 = crate::EnumBitfieldStruct<u8, Ndip40_SPEC>;
    impl Ndip40 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip41_SPEC;
    pub type Ndip41 = crate::EnumBitfieldStruct<u8, Ndip41_SPEC>;
    impl Ndip41 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip42_SPEC;
    pub type Ndip42 = crate::EnumBitfieldStruct<u8, Ndip42_SPEC>;
    impl Ndip42 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip43_SPEC;
    pub type Ndip43 = crate::EnumBitfieldStruct<u8, Ndip43_SPEC>;
    impl Ndip43 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip44_SPEC;
    pub type Ndip44 = crate::EnumBitfieldStruct<u8, Ndip44_SPEC>;
    impl Ndip44 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip45_SPEC;
    pub type Ndip45 = crate::EnumBitfieldStruct<u8, Ndip45_SPEC>;
    impl Ndip45 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip46_SPEC;
    pub type Ndip46 = crate::EnumBitfieldStruct<u8, Ndip46_SPEC>;
    impl Ndip46 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip47_SPEC;
    pub type Ndip47 = crate::EnumBitfieldStruct<u8, Ndip47_SPEC>;
    impl Ndip47 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip48_SPEC;
    pub type Ndip48 = crate::EnumBitfieldStruct<u8, Ndip48_SPEC>;
    impl Ndip48 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip49_SPEC;
    pub type Ndip49 = crate::EnumBitfieldStruct<u8, Ndip49_SPEC>;
    impl Ndip49 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip50_SPEC;
    pub type Ndip50 = crate::EnumBitfieldStruct<u8, Ndip50_SPEC>;
    impl Ndip50 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip51_SPEC;
    pub type Ndip51 = crate::EnumBitfieldStruct<u8, Ndip51_SPEC>;
    impl Ndip51 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip52_SPEC;
    pub type Ndip52 = crate::EnumBitfieldStruct<u8, Ndip52_SPEC>;
    impl Ndip52 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip53_SPEC;
    pub type Ndip53 = crate::EnumBitfieldStruct<u8, Ndip53_SPEC>;
    impl Ndip53 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip54_SPEC;
    pub type Ndip54 = crate::EnumBitfieldStruct<u8, Ndip54_SPEC>;
    impl Ndip54 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip55_SPEC;
    pub type Ndip55 = crate::EnumBitfieldStruct<u8, Ndip55_SPEC>;
    impl Ndip55 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip56_SPEC;
    pub type Ndip56 = crate::EnumBitfieldStruct<u8, Ndip56_SPEC>;
    impl Ndip56 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip57_SPEC;
    pub type Ndip57 = crate::EnumBitfieldStruct<u8, Ndip57_SPEC>;
    impl Ndip57 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip58_SPEC;
    pub type Ndip58 = crate::EnumBitfieldStruct<u8, Ndip58_SPEC>;
    impl Ndip58 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip59_SPEC;
    pub type Ndip59 = crate::EnumBitfieldStruct<u8, Ndip59_SPEC>;
    impl Ndip59 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip60_SPEC;
    pub type Ndip60 = crate::EnumBitfieldStruct<u8, Ndip60_SPEC>;
    impl Ndip60 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip61_SPEC;
    pub type Ndip61 = crate::EnumBitfieldStruct<u8, Ndip61_SPEC>;
    impl Ndip61 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip62_SPEC;
    pub type Ndip62 = crate::EnumBitfieldStruct<u8, Ndip62_SPEC>;
    impl Ndip62 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip63_SPEC;
    pub type Ndip63 = crate::EnumBitfieldStruct<u8, Ndip63_SPEC>;
    impl Ndip63 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ndic3_SPEC;
impl crate::sealed::RegSpec for Ndic3_SPEC {
    type DataType = u32;
}
#[doc = "New Data Interrupt Control 3\n resetvalue={Application Reset:0x0}"]
pub type Ndic3 = crate::RegValueT<Ndic3_SPEC>;

impl Ndic3 {
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip64(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ndic3::Ndip64, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,ndic3::Ndip64, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip65(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ndic3::Ndip65, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,ndic3::Ndip65, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip66(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ndic3::Ndip66, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,ndic3::Ndip66, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip67(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ndic3::Ndip67, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,ndic3::Ndip67, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip68(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ndic3::Ndip68, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,ndic3::Ndip68, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip69(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ndic3::Ndip69, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,ndic3::Ndip69, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip70(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, ndic3::Ndip70, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,ndic3::Ndip70, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip71(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, ndic3::Ndip71, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,ndic3::Ndip71, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip72(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, ndic3::Ndip72, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,ndic3::Ndip72, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip73(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, ndic3::Ndip73, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,ndic3::Ndip73, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip74(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, ndic3::Ndip74, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,ndic3::Ndip74, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip75(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, ndic3::Ndip75, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,ndic3::Ndip75, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip76(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, ndic3::Ndip76, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,ndic3::Ndip76, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip77(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, ndic3::Ndip77, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,ndic3::Ndip77, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip78(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, ndic3::Ndip78, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,ndic3::Ndip78, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip79(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, ndic3::Ndip79, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,ndic3::Ndip79, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip80(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, ndic3::Ndip80, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,ndic3::Ndip80, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip81(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, ndic3::Ndip81, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,ndic3::Ndip81, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip82(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, ndic3::Ndip82, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,ndic3::Ndip82, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip83(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, ndic3::Ndip83, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,ndic3::Ndip83, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip84(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, ndic3::Ndip84, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,ndic3::Ndip84, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip85(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, ndic3::Ndip85, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,ndic3::Ndip85, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip86(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, ndic3::Ndip86, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,ndic3::Ndip86, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip87(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, ndic3::Ndip87, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,ndic3::Ndip87, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip88(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, ndic3::Ndip88, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,ndic3::Ndip88, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip89(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, ndic3::Ndip89, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,ndic3::Ndip89, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip90(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, ndic3::Ndip90, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,ndic3::Ndip90, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip91(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, ndic3::Ndip91, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,ndic3::Ndip91, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip92(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, ndic3::Ndip92, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,ndic3::Ndip92, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip93(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, ndic3::Ndip93, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,ndic3::Ndip93, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip94(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, ndic3::Ndip94, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,ndic3::Ndip94, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 95  n   64 95    NDIP95. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip95(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, ndic3::Ndip95, Ndic3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,ndic3::Ndip95, Ndic3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ndic3 {
    #[inline(always)]
    fn default() -> Ndic3 {
        <crate::RegValueT<Ndic3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ndic3 {
    pub struct Ndip64_SPEC;
    pub type Ndip64 = crate::EnumBitfieldStruct<u8, Ndip64_SPEC>;
    impl Ndip64 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip65_SPEC;
    pub type Ndip65 = crate::EnumBitfieldStruct<u8, Ndip65_SPEC>;
    impl Ndip65 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip66_SPEC;
    pub type Ndip66 = crate::EnumBitfieldStruct<u8, Ndip66_SPEC>;
    impl Ndip66 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip67_SPEC;
    pub type Ndip67 = crate::EnumBitfieldStruct<u8, Ndip67_SPEC>;
    impl Ndip67 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip68_SPEC;
    pub type Ndip68 = crate::EnumBitfieldStruct<u8, Ndip68_SPEC>;
    impl Ndip68 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip69_SPEC;
    pub type Ndip69 = crate::EnumBitfieldStruct<u8, Ndip69_SPEC>;
    impl Ndip69 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip70_SPEC;
    pub type Ndip70 = crate::EnumBitfieldStruct<u8, Ndip70_SPEC>;
    impl Ndip70 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip71_SPEC;
    pub type Ndip71 = crate::EnumBitfieldStruct<u8, Ndip71_SPEC>;
    impl Ndip71 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip72_SPEC;
    pub type Ndip72 = crate::EnumBitfieldStruct<u8, Ndip72_SPEC>;
    impl Ndip72 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip73_SPEC;
    pub type Ndip73 = crate::EnumBitfieldStruct<u8, Ndip73_SPEC>;
    impl Ndip73 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip74_SPEC;
    pub type Ndip74 = crate::EnumBitfieldStruct<u8, Ndip74_SPEC>;
    impl Ndip74 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip75_SPEC;
    pub type Ndip75 = crate::EnumBitfieldStruct<u8, Ndip75_SPEC>;
    impl Ndip75 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip76_SPEC;
    pub type Ndip76 = crate::EnumBitfieldStruct<u8, Ndip76_SPEC>;
    impl Ndip76 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip77_SPEC;
    pub type Ndip77 = crate::EnumBitfieldStruct<u8, Ndip77_SPEC>;
    impl Ndip77 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip78_SPEC;
    pub type Ndip78 = crate::EnumBitfieldStruct<u8, Ndip78_SPEC>;
    impl Ndip78 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip79_SPEC;
    pub type Ndip79 = crate::EnumBitfieldStruct<u8, Ndip79_SPEC>;
    impl Ndip79 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip80_SPEC;
    pub type Ndip80 = crate::EnumBitfieldStruct<u8, Ndip80_SPEC>;
    impl Ndip80 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip81_SPEC;
    pub type Ndip81 = crate::EnumBitfieldStruct<u8, Ndip81_SPEC>;
    impl Ndip81 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip82_SPEC;
    pub type Ndip82 = crate::EnumBitfieldStruct<u8, Ndip82_SPEC>;
    impl Ndip82 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip83_SPEC;
    pub type Ndip83 = crate::EnumBitfieldStruct<u8, Ndip83_SPEC>;
    impl Ndip83 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip84_SPEC;
    pub type Ndip84 = crate::EnumBitfieldStruct<u8, Ndip84_SPEC>;
    impl Ndip84 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip85_SPEC;
    pub type Ndip85 = crate::EnumBitfieldStruct<u8, Ndip85_SPEC>;
    impl Ndip85 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip86_SPEC;
    pub type Ndip86 = crate::EnumBitfieldStruct<u8, Ndip86_SPEC>;
    impl Ndip86 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip87_SPEC;
    pub type Ndip87 = crate::EnumBitfieldStruct<u8, Ndip87_SPEC>;
    impl Ndip87 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip88_SPEC;
    pub type Ndip88 = crate::EnumBitfieldStruct<u8, Ndip88_SPEC>;
    impl Ndip88 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip89_SPEC;
    pub type Ndip89 = crate::EnumBitfieldStruct<u8, Ndip89_SPEC>;
    impl Ndip89 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip90_SPEC;
    pub type Ndip90 = crate::EnumBitfieldStruct<u8, Ndip90_SPEC>;
    impl Ndip90 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip91_SPEC;
    pub type Ndip91 = crate::EnumBitfieldStruct<u8, Ndip91_SPEC>;
    impl Ndip91 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip92_SPEC;
    pub type Ndip92 = crate::EnumBitfieldStruct<u8, Ndip92_SPEC>;
    impl Ndip92 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip93_SPEC;
    pub type Ndip93 = crate::EnumBitfieldStruct<u8, Ndip93_SPEC>;
    impl Ndip93 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip94_SPEC;
    pub type Ndip94 = crate::EnumBitfieldStruct<u8, Ndip94_SPEC>;
    impl Ndip94 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip95_SPEC;
    pub type Ndip95 = crate::EnumBitfieldStruct<u8, Ndip95_SPEC>;
    impl Ndip95 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ndic4_SPEC;
impl crate::sealed::RegSpec for Ndic4_SPEC {
    type DataType = u32;
}
#[doc = "New Data Interrupt Control 4\n resetvalue={Application Reset:0x0}"]
pub type Ndic4 = crate::RegValueT<Ndic4_SPEC>;

impl Ndic4 {
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip96(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ndic4::Ndip96, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,ndic4::Ndip96, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip97(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ndic4::Ndip97, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,ndic4::Ndip97, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip98(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ndic4::Ndip98, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,ndic4::Ndip98, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip99(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ndic4::Ndip99, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,ndic4::Ndip99, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip100(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, ndic4::Ndip100, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,ndic4::Ndip100, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip101(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ndic4::Ndip101, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,ndic4::Ndip101, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip102(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, ndic4::Ndip102, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,ndic4::Ndip102, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip103(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, ndic4::Ndip103, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,ndic4::Ndip103, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip104(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, ndic4::Ndip104, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,ndic4::Ndip104, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip105(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, ndic4::Ndip105, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,ndic4::Ndip105, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip106(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, ndic4::Ndip106, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,ndic4::Ndip106, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip107(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, ndic4::Ndip107, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,ndic4::Ndip107, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip108(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, ndic4::Ndip108, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,ndic4::Ndip108, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip109(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, ndic4::Ndip109, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,ndic4::Ndip109, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip110(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, ndic4::Ndip110, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,ndic4::Ndip110, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip111(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, ndic4::Ndip111, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,ndic4::Ndip111, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip112(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, ndic4::Ndip112, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,ndic4::Ndip112, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip113(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, ndic4::Ndip113, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,ndic4::Ndip113, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip114(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, ndic4::Ndip114, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,ndic4::Ndip114, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip115(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, ndic4::Ndip115, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,ndic4::Ndip115, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip116(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, ndic4::Ndip116, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,ndic4::Ndip116, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip117(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, ndic4::Ndip117, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,ndic4::Ndip117, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip118(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, ndic4::Ndip118, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,ndic4::Ndip118, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip119(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, ndic4::Ndip119, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,ndic4::Ndip119, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip120(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, ndic4::Ndip120, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,ndic4::Ndip120, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip121(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, ndic4::Ndip121, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,ndic4::Ndip121, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip122(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, ndic4::Ndip122, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,ndic4::Ndip122, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip123(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, ndic4::Ndip123, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,ndic4::Ndip123, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip124(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, ndic4::Ndip124, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,ndic4::Ndip124, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip125(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, ndic4::Ndip125, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,ndic4::Ndip125, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip126(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, ndic4::Ndip126, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,ndic4::Ndip126, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "New Data Interrupt Pointer 127  n   96 127    NDIP127. NDIPn determines the interrupt  NDAT0SRC or NDAT1SRC  of the service request output that becomes active on a New Data Flag becoming active."]
    #[inline(always)]
    pub fn ndip127(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, ndic4::Ndip127, Ndic4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,ndic4::Ndip127, Ndic4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ndic4 {
    #[inline(always)]
    fn default() -> Ndic4 {
        <crate::RegValueT<Ndic4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ndic4 {
    pub struct Ndip96_SPEC;
    pub type Ndip96 = crate::EnumBitfieldStruct<u8, Ndip96_SPEC>;
    impl Ndip96 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip97_SPEC;
    pub type Ndip97 = crate::EnumBitfieldStruct<u8, Ndip97_SPEC>;
    impl Ndip97 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip98_SPEC;
    pub type Ndip98 = crate::EnumBitfieldStruct<u8, Ndip98_SPEC>;
    impl Ndip98 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip99_SPEC;
    pub type Ndip99 = crate::EnumBitfieldStruct<u8, Ndip99_SPEC>;
    impl Ndip99 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip100_SPEC;
    pub type Ndip100 = crate::EnumBitfieldStruct<u8, Ndip100_SPEC>;
    impl Ndip100 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip101_SPEC;
    pub type Ndip101 = crate::EnumBitfieldStruct<u8, Ndip101_SPEC>;
    impl Ndip101 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip102_SPEC;
    pub type Ndip102 = crate::EnumBitfieldStruct<u8, Ndip102_SPEC>;
    impl Ndip102 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip103_SPEC;
    pub type Ndip103 = crate::EnumBitfieldStruct<u8, Ndip103_SPEC>;
    impl Ndip103 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip104_SPEC;
    pub type Ndip104 = crate::EnumBitfieldStruct<u8, Ndip104_SPEC>;
    impl Ndip104 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip105_SPEC;
    pub type Ndip105 = crate::EnumBitfieldStruct<u8, Ndip105_SPEC>;
    impl Ndip105 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip106_SPEC;
    pub type Ndip106 = crate::EnumBitfieldStruct<u8, Ndip106_SPEC>;
    impl Ndip106 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip107_SPEC;
    pub type Ndip107 = crate::EnumBitfieldStruct<u8, Ndip107_SPEC>;
    impl Ndip107 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip108_SPEC;
    pub type Ndip108 = crate::EnumBitfieldStruct<u8, Ndip108_SPEC>;
    impl Ndip108 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip109_SPEC;
    pub type Ndip109 = crate::EnumBitfieldStruct<u8, Ndip109_SPEC>;
    impl Ndip109 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip110_SPEC;
    pub type Ndip110 = crate::EnumBitfieldStruct<u8, Ndip110_SPEC>;
    impl Ndip110 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip111_SPEC;
    pub type Ndip111 = crate::EnumBitfieldStruct<u8, Ndip111_SPEC>;
    impl Ndip111 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip112_SPEC;
    pub type Ndip112 = crate::EnumBitfieldStruct<u8, Ndip112_SPEC>;
    impl Ndip112 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip113_SPEC;
    pub type Ndip113 = crate::EnumBitfieldStruct<u8, Ndip113_SPEC>;
    impl Ndip113 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip114_SPEC;
    pub type Ndip114 = crate::EnumBitfieldStruct<u8, Ndip114_SPEC>;
    impl Ndip114 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip115_SPEC;
    pub type Ndip115 = crate::EnumBitfieldStruct<u8, Ndip115_SPEC>;
    impl Ndip115 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip116_SPEC;
    pub type Ndip116 = crate::EnumBitfieldStruct<u8, Ndip116_SPEC>;
    impl Ndip116 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip117_SPEC;
    pub type Ndip117 = crate::EnumBitfieldStruct<u8, Ndip117_SPEC>;
    impl Ndip117 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip118_SPEC;
    pub type Ndip118 = crate::EnumBitfieldStruct<u8, Ndip118_SPEC>;
    impl Ndip118 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip119_SPEC;
    pub type Ndip119 = crate::EnumBitfieldStruct<u8, Ndip119_SPEC>;
    impl Ndip119 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip120_SPEC;
    pub type Ndip120 = crate::EnumBitfieldStruct<u8, Ndip120_SPEC>;
    impl Ndip120 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip121_SPEC;
    pub type Ndip121 = crate::EnumBitfieldStruct<u8, Ndip121_SPEC>;
    impl Ndip121 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip122_SPEC;
    pub type Ndip122 = crate::EnumBitfieldStruct<u8, Ndip122_SPEC>;
    impl Ndip122 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip123_SPEC;
    pub type Ndip123 = crate::EnumBitfieldStruct<u8, Ndip123_SPEC>;
    impl Ndip123 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip124_SPEC;
    pub type Ndip124 = crate::EnumBitfieldStruct<u8, Ndip124_SPEC>;
    impl Ndip124 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip125_SPEC;
    pub type Ndip125 = crate::EnumBitfieldStruct<u8, Ndip125_SPEC>;
    impl Ndip125 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip126_SPEC;
    pub type Ndip126 = crate::EnumBitfieldStruct<u8, Ndip126_SPEC>;
    impl Ndip126 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ndip127_SPEC;
    pub type Ndip127 = crate::EnumBitfieldStruct<u8, Ndip127_SPEC>;
    impl Ndip127 {
        #[doc = "0 NDAT0SRC selected for New Data Service Request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 NDAT1SRC selected for New Data Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nemc_SPEC;
impl crate::sealed::RegSpec for Nemc_SPEC {
    type DataType = u32;
}
#[doc = "NEM Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Nemc = crate::RegValueT<Nemc_SPEC>;

impl Nemc {
    #[doc = "Network Management Vector Length gNetworkManagementVectorLength    NML. These bits configure the length of the NM Vector. The configured length must be identical in all nodes of a cluster. Valid values are 0 to 12  0 H to C H   bytes."]
    #[inline(always)]
    pub fn nml(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Nemc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Nemc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Nemc {
    #[inline(always)]
    fn default() -> Nemc {
        <crate::RegValueT<Nemc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NmVx_SPEC;
impl crate::sealed::RegSpec for NmVx_SPEC {
    type DataType = u32;
}
#[doc = "Network Management Vector 1\n resetvalue={Application Reset:0x0}"]
pub type NmVx = crate::RegValueT<NmVx_SPEC>;

impl NmVx {
    #[doc = "Network Management Vector   NM"]
    #[inline(always)]
    pub fn nm(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, NmVx_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, NmVx_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for NmVx {
    #[inline(always)]
    fn default() -> NmVx {
        <crate::RegValueT<NmVx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Obcm_SPEC;
impl crate::sealed::RegSpec for Obcm_SPEC {
    type DataType = u32;
}
#[doc = "Output Buffer Command Mask\n resetvalue={Application Reset:0x0}"]
pub type Obcm = crate::RegValueT<Obcm_SPEC>;

impl Obcm {
    #[doc = "Read Header Section Shadow   RHSS"]
    #[inline(always)]
    pub fn rhss(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, obcm::Rhss, Obcm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,obcm::Rhss, Obcm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Read Data Section Shadow   RDSS"]
    #[inline(always)]
    pub fn rdss(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, obcm::Rdss, Obcm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,obcm::Rdss, Obcm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Read Header Section Host   RHSH"]
    #[inline(always)]
    pub fn rhsh(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, obcm::Rhsh, Obcm_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x1,1,0,obcm::Rhsh, Obcm_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Read Data Section Host   RDSH"]
    #[inline(always)]
    pub fn rdsh(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, obcm::Rdsh, Obcm_SPEC, crate::common::R> {
        crate::common::RegisterField::<17,0x1,1,0,obcm::Rdsh, Obcm_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Obcm {
    #[inline(always)]
    fn default() -> Obcm {
        <crate::RegValueT<Obcm_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod obcm {
    pub struct Rhss_SPEC;
    pub type Rhss = crate::EnumBitfieldStruct<u8, Rhss_SPEC>;
    impl Rhss {
        #[doc = "0 Header Section is not read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Header Section selected for transfer from Message RAM to Output Buffer"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rdss_SPEC;
    pub type Rdss = crate::EnumBitfieldStruct<u8, Rdss_SPEC>;
    impl Rdss {
        #[doc = "0 Data Section is not read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Data Section selected for transfer from Message RAM to Output Buffer"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rhsh_SPEC;
    pub type Rhsh = crate::EnumBitfieldStruct<u8, Rhsh_SPEC>;
    impl Rhsh {
        #[doc = "0 Header Section is not read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Header Section selected for transfer from Message RAM to Output Buffer"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rdsh_SPEC;
    pub type Rdsh = crate::EnumBitfieldStruct<u8, Rdsh_SPEC>;
    impl Rdsh {
        #[doc = "0 Data Section is not read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Data Section selected for transfer from Message RAM to Output Buffer"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Obcr_SPEC;
impl crate::sealed::RegSpec for Obcr_SPEC {
    type DataType = u32;
}
#[doc = "Output Buffer Command Request\n resetvalue={Application Reset:0x0}"]
pub type Obcr = crate::RegValueT<Obcr_SPEC>;

impl Obcr {
    #[doc = "Output Buffer Request Shadow   OBRS. Number of source Message Buffer to be transferred from the Message RAM        to OBF Shadow. Valid values are 00 H to 7F H  0 to 127 . If the number of the first Message Buffer of the receive FIFO is written        to this register the Message Handler transfers the Message Buffer        addressed by the GET Index Register  GIDX    8220 FIFO Function  8221   to OBF        Shadow."]
    #[inline(always)]
    pub fn obrs(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Obcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8, Obcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "View Shadow Buffer   VIEW. Toggles between OBF Shadow and OBF Host. Only writeable while OBCR.OBSYS   0."]
    #[inline(always)]
    pub fn view(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, obcr::View, Obcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,obcr::View, Obcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Request Message RAM Transfer   REQ. Requests transfer of Message Buffer addressed by OBCR.OBRS from Message RAM to OBF Shadow. Only writeable while OBCR.OBSYS   0."]
    #[inline(always)]
    pub fn req(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, obcr::Req, Obcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,obcr::Req, Obcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Output Buffer Busy Shadow   OBSYS. Set to 1 after setting bit OBCR.REQ. When the transfer between the        Message RAM and OBF Shadow has completed  OBCR.OBSYS is cleared again."]
    #[inline(always)]
    pub fn obsys(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, obcr::Obsys, Obcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<15,0x1,1,0,obcr::Obsys, Obcr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Output Buffer Request Host   OBRH. Number of Message Buffer currently accessible by the Host via RDHS1 to        RDHS3  MBS  and RDDSnn  nn   01 64 . By setting OBCR.VIEW OBF Shadow and        OBF Host are swapped and the transferred Message Buffer is accessible by        the Host. Valid values are 00 H to 7F H  01 to 127 ."]
    #[inline(always)]
    pub fn obrh(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Obcr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Obcr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Obcr {
    #[inline(always)]
    fn default() -> Obcr {
        <crate::RegValueT<Obcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod obcr {
    pub struct View_SPEC;
    pub type View = crate::EnumBitfieldStruct<u8, View_SPEC>;
    impl View {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Swap OBF Shadow and OBF Host"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Req_SPEC;
    pub type Req = crate::EnumBitfieldStruct<u8, Req_SPEC>;
    impl Req {
        #[doc = "0 No request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transfer to OBF Shadow requested"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Obsys_SPEC;
    pub type Obsys = crate::EnumBitfieldStruct<u8, Obsys_SPEC>;
    impl Obsys {
        #[doc = "0 No transfer in progress"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transfer between Message RAM and OBF Shadow in progress"]
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
        #[doc = "1 Hard suspend. Clocks f CLC ERAY and the sampling clock f SCLK are switched off immediately. No read or write access to any registers."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Soft suspend. This bit forces the module into freeze state."]
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
pub struct Ocv_SPEC;
impl crate::sealed::RegSpec for Ocv_SPEC {
    type DataType = u32;
}
#[doc = "Offset Correction Value\n resetvalue={Application Reset:0x0}"]
pub type Ocv = crate::RegValueT<Ocv_SPEC>;

impl Ocv {
    #[doc = "Offset Correction Value vOffsetCorrection    OCV. Offset correction value  two s complement . Calculated internal offset correction value before limitation. If the OCV value exceeds the limits defined by GTUC10.MOC flag SFS.OCLR is set to 1."]
    #[inline(always)]
    pub fn ocv(
        self,
    ) -> crate::common::RegisterField<0, 0x7ffff, 1, 0, u32, Ocv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7ffff,1,0,u32, Ocv_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ocv {
    #[inline(always)]
    fn default() -> Ocv {
        <crate::RegValueT<Ocv_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OsiDn_SPEC;
impl crate::sealed::RegSpec for OsiDn_SPEC {
    type DataType = u32;
}
#[doc = "Odd Sync ID Symbol Window 01\n resetvalue={Application Reset:0x0}"]
pub type OsiDn = crate::RegValueT<OsiDn_SPEC>;

impl OsiDn {
    #[doc = "Odd Sync ID vsSyncIDListA B odd    OID. SYNC Frame ID even communication cycle."]
    #[inline(always)]
    pub fn oid(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, OsiDn_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, OsiDn_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Received Odd Sync ID on Channel A   RXOA. Signals that a SYNC Frame corresponding to the stored odd sync ID was received on channel A or that the node is configured to be a sync node with key slot   OID  OSID1 only ."]
    #[inline(always)]
    pub fn rxoa(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, osidn::Rxoa, OsiDn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<14,0x1,1,0,osidn::Rxoa, OsiDn_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Received Odd Sync ID on Channel B   RXOB. Signals that a SYNC Frame corresponding to the stored odd sync ID was received on channel B or that the node is configured to be a sync node with key slot   OID  OSID1 only"]
    #[inline(always)]
    pub fn rxob(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, osidn::Rxob, OsiDn_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<15,0x1,1,0,osidn::Rxob, OsiDn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for OsiDn {
    #[inline(always)]
    fn default() -> OsiDn {
        <crate::RegValueT<OsiDn_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod osidn {
    pub struct Rxoa_SPEC;
    pub type Rxoa = crate::EnumBitfieldStruct<u8, Rxoa_SPEC>;
    impl Rxoa {
        #[doc = "0 SYNC Frame not received on channel A  node configured to transmit SYNC Frames"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SYNC Frame received on channel A  node not configured to transmit SYNC Frames"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rxob_SPEC;
    pub type Rxob = crate::EnumBitfieldStruct<u8, Rxob_SPEC>;
    impl Rxob {
        #[doc = "0 SYNC Frame not received on channel B  node configured to transmit SYNC Frames"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SYNC Frame received on channel B  node not configured to transmit SYNC Frames"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otss_SPEC;
impl crate::sealed::RegSpec for Otss_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Trigger Set Select\n resetvalue={Application Reset:0x0}"]
pub type Otss = crate::RegValueT<Otss_SPEC>;

impl Otss {
    #[doc = "Trigger Set for OTGB0   OTGB0"]
    #[inline(always)]
    pub fn otgb0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, otss::Otgb0, Otss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,otss::Otgb0, Otss_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Set for OTGB1   OTGB1"]
    #[inline(always)]
    pub fn otgb1(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, otss::Otgb1, Otss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,otss::Otgb1, Otss_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Set for OTGB2   OTGB2"]
    #[inline(always)]
    pub fn otgb2(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, otss::Otgb2, Otss_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,otss::Otgb2, Otss_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Otss {
    #[inline(always)]
    fn default() -> Otss {
        <crate::RegValueT<Otss_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod otss {
    pub struct Otgb0_SPEC;
    pub type Otgb0 = crate::EnumBitfieldStruct<u8, Otgb0_SPEC>;
    impl Otgb0 {
        #[doc = "0 No Trigger Set        selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Trigger Set        TS16 SEP"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Trigger Set        TS16 MC"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Otgb1_SPEC;
    pub type Otgb1 = crate::EnumBitfieldStruct<u8, Otgb1_SPEC>;
    impl Otgb1 {
        #[doc = "0 No Trigger Set        selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Trigger Set        TS16 SEP"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Trigger Set        TS16 MC"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Otgb2_SPEC;
    pub type Otgb2 = crate::EnumBitfieldStruct<u8, Otgb2_SPEC>;
    impl Otgb2 {
        #[doc = "0 No Trigger Set selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Trigger Set        TS32 SCSC"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prtc1_SPEC;
impl crate::sealed::RegSpec for Prtc1_SPEC {
    type DataType = u32;
}
#[doc = "PRT Configuration Register 1\n resetvalue={Application Reset:0x084C0633}"]
pub type Prtc1 = crate::RegValueT<Prtc1_SPEC>;

impl Prtc1 {
    #[doc = "Transmission Start Sequence Transmitter gdTSSTransmitter    TSST. Configures the duration of the Transmission Start Sequence  TSS  in        terms of Bit Times  1 bit time  160   4  160 Microticks  160   100ns at 10Mbps . Must        be identical in all nodes of a cluster. Valid values are 3 to 15  3 H to F H   Bit Times."]
    #[inline(always)]
    pub fn tsst(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Prtc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Prtc1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Collision Avoidance Symbol Maximum  gdCASRxLowMax    CASM. Configures the upper limit of the acceptance window for a collision        avoidance symbol  CAS . Valid values are 67 to 99  43 H to 63 H  . Most significant bit of CASM        is hard wired to 1 and can not be modified."]
    #[inline(always)]
    pub fn casm(
        self,
    ) -> crate::common::RegisterField<4, 0x7f, 1, 0, u8, Prtc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7f,1,0,u8, Prtc1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Strobe Point Position   SPP. Defines the sample count value for strobing. The strobed bit value is        set to the voted value when the sample count is incremented to the value        configured by SPP. The current revision 2.1 of the FlexRay  8482  protocol requires that SPP            00 B . The alternate strobe point          positions could be used to compensate for asymmetries in the physical          layer."]
    #[inline(always)]
    pub fn spp(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, prtc1::Spp, Prtc1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,prtc1::Spp, Prtc1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Baud Rate Prescaler  gdSampleClockPeriod  pSamplePerMicrotick    BRP. The baud rate prescaler configures the baud rate on the FlexRay  8482  bus.        The baud rates listed below are valid with a sample clock f SCLK          80  160 MHz. One bit time always consists of 8 samples independent of the        configured baud rate."]
    #[inline(always)]
    pub fn brp(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, prtc1::Brp, Prtc1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,prtc1::Brp, Prtc1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wakeup Symbol Receive Window Length  gdWakeupSymbolRxWindow    RXW. Configures the number of Bit Times used by the node to test the duration        of the received wakeup pattern. Must be identical in all nodes of a        cluster. Valid values are 76 to 301  4C H to 12D H   Bit Times."]
    #[inline(always)]
    pub fn rxw(
        self,
    ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, Prtc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1ff,1,0,u16, Prtc1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Repetitions of Tx Wakeup Pattern  pWakeupPattern    RWP. Configures the number of repetitions  sequences  of the Tx wakeup        symbol. Valid values are 2 to 63  2 H to 3F H  ."]
    #[inline(always)]
    pub fn rwp(
        self,
    ) -> crate::common::RegisterField<26, 0x3f, 1, 0, u8, Prtc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3f,1,0,u8, Prtc1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Prtc1 {
    #[inline(always)]
    fn default() -> Prtc1 {
        <crate::RegValueT<Prtc1_SPEC> as RegisterValue<_>>::new(139200051)
    }
}
pub mod prtc1 {
    pub struct Spp_SPEC;
    pub type Spp = crate::EnumBitfieldStruct<u8, Spp_SPEC>;
    impl Spp {
        #[doc = "00 Sample 5  default"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Sample 4"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Sample 6"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Brp_SPEC;
    pub type Brp = crate::EnumBitfieldStruct<u8, Brp_SPEC>;
    impl Brp {
        #[doc = "00 10 Mbit s  1 Microtick  25 ns  twice sampled with f SCLK   gdSampleClockPeriod   12.5 ns   1   f SCLK pSamplesPerMicrotick   2"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 5 Mbit s  1 Microtick  25ns  single sampled with f SCLK   2  gdSampleClockPeriod   25 ns   2   f SCLK pSamplesPerMicrotick   1"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 2.5 Mbit s  1 Microtick   50ns  single sampled with f SCLK   4  gdSampleClockPeriod   50 ns   4   f SCLK pSamplesPerMicrotick   1"]
        pub const CONST_22: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prtc2_SPEC;
impl crate::sealed::RegSpec for Prtc2_SPEC {
    type DataType = u32;
}
#[doc = "PRT Configuration Register 2\n resetvalue={Application Reset:0x0F2D0A0E}"]
pub type Prtc2 = crate::RegValueT<Prtc2_SPEC>;

impl Prtc2 {
    #[doc = "Wakeup Symbol Receive Idle gdWakeupSymbolRxIdle    RXI. Configures the number of Bit Times used by the node to test the duration        of the idle phase of the received wakeup symbol. Must be identical in        all nodes of a cluster. Valid values are 14 to 59  E H to 3B H   Bit Times."]
    #[inline(always)]
    pub fn rxi(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Prtc2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Prtc2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wakeup Symbol Receive Low  gdWakeupSymbolRxLow    RXL. Configures the number of Bit Times used by the node to test the duration        of the low phase of the received wakeup symbol. Must be identical in all        nodes of a cluster. Valid values are 10 to 55  A H to 37 H   Bit Times."]
    #[inline(always)]
    pub fn rxl(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, Prtc2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3f,1,0,u8, Prtc2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wakeup Symbol Transmit Idle  gdWakeupSymbolTxIdle    TXI. Configures the number of Bit Times used by the node to transmit the idle        phase of the wakeup symbol. Must be identical in all nodes of a cluster.        Valid values are 45 to 180  2D H to B4 H  Bit        Times."]
    #[inline(always)]
    pub fn txi(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Prtc2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Prtc2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wakeup Symbol Transmit Low  gdWakeupSymbolTxLow    TXL. Configures the number of Bit Times used by the node to transmit the low        phase of the wakeup symbol. Must be identical in all nodes of a cluster.        Valid values are 15 to 60  F H to 3C H          Bit Times."]
    #[inline(always)]
    pub fn txl(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, Prtc2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3f,1,0,u8, Prtc2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Prtc2 {
    #[inline(always)]
    fn default() -> Prtc2 {
        <crate::RegValueT<Prtc2_SPEC> as RegisterValue<_>>::new(254609934)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcv_SPEC;
impl crate::sealed::RegSpec for Rcv_SPEC {
    type DataType = u32;
}
#[doc = "Rate Correction Value\n resetvalue={Application Reset:0x0}"]
pub type Rcv = crate::RegValueT<Rcv_SPEC>;

impl Rcv {
    #[doc = "Rate Correction Value vRateCorrection    RCV. Rate correction value  two s complement . Calculated internal rate correction value before limitation. If the RCV value exceeds the limits defined by GTUC10.MRC  flag SFS.RCLR is set to 1."]
    #[inline(always)]
    pub fn rcv(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, Rcv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xfff,1,0,u16, Rcv_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Rcv {
    #[inline(always)]
    fn default() -> Rcv {
        <crate::RegValueT<Rcv_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RddSn_SPEC;
impl crate::sealed::RegSpec for RddSn_SPEC {
    type DataType = u32;
}
#[doc = "Read Data Section 01\n resetvalue={Application Reset:0x0}"]
pub type RddSn = crate::RegValueT<RddSn_SPEC>;

impl RddSn {
    #[doc = "32 Bit Word nn  Byte 0   MDRB0"]
    #[inline(always)]
    pub fn mdrb0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, RddSn_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, RddSn_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "32 Bit Word nn  Byte 1   MDRB1"]
    #[inline(always)]
    pub fn mdrb1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, RddSn_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, RddSn_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "32 Bit Word nn  Byte 2   MDRB2"]
    #[inline(always)]
    pub fn mdrb2(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, RddSn_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, RddSn_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "32 Bit Word nn  Byte 3   MDRB3"]
    #[inline(always)]
    pub fn mdrb3(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, RddSn_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, RddSn_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for RddSn {
    #[inline(always)]
    fn default() -> RddSn {
        <crate::RegValueT<RddSn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdhs1_SPEC;
impl crate::sealed::RegSpec for Rdhs1_SPEC {
    type DataType = u32;
}
#[doc = "Read Header Section 1\n resetvalue={Application Reset:0x0}"]
pub type Rdhs1 = crate::RegValueT<Rdhs1_SPEC>;

impl Rdhs1 {
    #[doc = "Frame ID   FID"]
    #[inline(always)]
    pub fn fid(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Rdhs1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Rdhs1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Cycle Code   CYC"]
    #[inline(always)]
    pub fn cyc(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Rdhs1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Rdhs1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Channel Filter Control A   CHA"]
    #[inline(always)]
    pub fn cha(self) -> crate::common::RegisterFieldBool<24, 1, 0, Rdhs1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Rdhs1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel Filter Control B   CHB"]
    #[inline(always)]
    pub fn chb(self) -> crate::common::RegisterFieldBool<25, 1, 0, Rdhs1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Rdhs1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Direction Configuration Bit   CFG"]
    #[inline(always)]
    pub fn cfg(self) -> crate::common::RegisterFieldBool<26, 1, 0, Rdhs1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Rdhs1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Payload Preamble Indicator Transmit   PPIT"]
    #[inline(always)]
    pub fn ppit(self) -> crate::common::RegisterFieldBool<27, 1, 0, Rdhs1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Rdhs1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Mode   TXM"]
    #[inline(always)]
    pub fn txm(self) -> crate::common::RegisterFieldBool<28, 1, 0, Rdhs1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Rdhs1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Service Request   MBI"]
    #[inline(always)]
    pub fn mbi(self) -> crate::common::RegisterFieldBool<29, 1, 0, Rdhs1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Rdhs1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Rdhs1 {
    #[inline(always)]
    fn default() -> Rdhs1 {
        <crate::RegValueT<Rdhs1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdhs2_SPEC;
impl crate::sealed::RegSpec for Rdhs2_SPEC {
    type DataType = u32;
}
#[doc = "Read Header Section 2\n resetvalue={Application Reset:0x0}"]
pub type Rdhs2 = crate::RegValueT<Rdhs2_SPEC>;

impl Rdhs2 {
    #[doc = "Header CRC vRF Header HeaderCRC    CRC. Receive Buffer  Configuration not required. Header CRC updated from receive Data Frames. Transmit Buffer  Header CRC calculated and configured by the Host"]
    #[inline(always)]
    pub fn crc(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Rdhs2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Rdhs2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Payload Length Configured   PLC. Length of Data Section  number of 2 byte words  as configured by the Host."]
    #[inline(always)]
    pub fn plc(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Rdhs2_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Rdhs2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Payload Length Received vRF Header Length    PLR. Payload length value updated from received Data Frame  exception  if Message Buffer belongs to the receive FIFO PLR is also updated from received NULL Frames . When a message is stored into a Message Buffer the following behavior with respect to payload length received and payload length configured is implemented  PLR  gt  PLC  The payload data stored in the Message Buffer is truncated to the payload length configured for even PLC or else truncated to PLC   1. PLR   PLC  The received payload data is stored into the Message Buffers Data Section. The remaining data bytes of the Data Section as configured by PLC are filled with undefined data. PLR   0  The Message Buffer s Data Section is filled with undefined data. PLC   0  Message Buffer has no Data Section configured. No data is stored into the  Message Buffer s Data Section."]
    #[inline(always)]
    pub fn plr(
        self,
    ) -> crate::common::RegisterField<24, 0x7f, 1, 0, u8, Rdhs2_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x7f,1,0,u8, Rdhs2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Rdhs2 {
    #[inline(always)]
    fn default() -> Rdhs2 {
        <crate::RegValueT<Rdhs2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdhs3_SPEC;
impl crate::sealed::RegSpec for Rdhs3_SPEC {
    type DataType = u32;
}
#[doc = "Read Header Section 3\n resetvalue={Application Reset:0x0}"]
pub type Rdhs3 = crate::RegValueT<Rdhs3_SPEC>;

impl Rdhs3 {
    #[doc = "Data Pointer   DP. Pointer to the first 32 bit word of the Data Section of the addressed Message Buffer in the Message RAM."]
    #[inline(always)]
    pub fn dp(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Rdhs3_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Rdhs3_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive Cycle Count vRF Header CycleCount    RCC. Cycle counter value updated from received Data Frame."]
    #[inline(always)]
    pub fn rcc(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Rdhs3_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Rdhs3_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Received on Channel Indicator vSS Channel    RCI. Indicates the channel from which the received Data Frame was taken to update the respective receive buffer."]
    #[inline(always)]
    pub fn rci(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, rdhs3::Rci, Rdhs3_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x1,1,0,rdhs3::Rci, Rdhs3_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Startup Frame Indicator vRF Header SuFIndicator    SFI. A Startup Frame is marked by the Startup Frame indicator."]
    #[inline(always)]
    pub fn sfi(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, rdhs3::Sfi, Rdhs3_SPEC, crate::common::R> {
        crate::common::RegisterField::<25,0x1,1,0,rdhs3::Sfi, Rdhs3_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SYNC Frame Indicator vRF Header SyFIndicator    SYN. A SYNC Frame is marked by the SYNC Frame indicator."]
    #[inline(always)]
    pub fn syn(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, rdhs3::Syn, Rdhs3_SPEC, crate::common::R> {
        crate::common::RegisterField::<26,0x1,1,0,rdhs3::Syn, Rdhs3_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "NULL Frame Indicator vRF Header NFIndicator    NFI. Is set to 1 after storage of the first received Data Frame."]
    #[inline(always)]
    pub fn nfi(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, rdhs3::Nfi, Rdhs3_SPEC, crate::common::R> {
        crate::common::RegisterField::<27,0x1,1,0,rdhs3::Nfi, Rdhs3_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Payload Preamble Indicator vRF Header PPIndicator    PPI. The payload preamble indicator defines whether a Network Management vector or message ID is contained within the Payload Segment of the received Frame."]
    #[inline(always)]
    pub fn ppi(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, rdhs3::Ppi, Rdhs3_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0x1,1,0,rdhs3::Ppi, Rdhs3_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Rdhs3 {
    #[inline(always)]
    fn default() -> Rdhs3 {
        <crate::RegValueT<Rdhs3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rdhs3 {
    pub struct Rci_SPEC;
    pub type Rci = crate::EnumBitfieldStruct<u8, Rci_SPEC>;
    impl Rci {
        #[doc = "0 Frame received on channel B"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Frame received on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sfi_SPEC;
    pub type Sfi = crate::EnumBitfieldStruct<u8, Sfi_SPEC>;
    impl Sfi {
        #[doc = "0 The received Frame is not a startup Frame"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The received Frame is a startup Frame"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Syn_SPEC;
    pub type Syn = crate::EnumBitfieldStruct<u8, Syn_SPEC>;
    impl Syn {
        #[doc = "0 The received Frame is not a SYNC Frame"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The received Frame is a SYNC Frame"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Nfi_SPEC;
    pub type Nfi = crate::EnumBitfieldStruct<u8, Nfi_SPEC>;
    impl Nfi {
        #[doc = "0 Up to now no Data Frame has been stored into the respective Message Buffer"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 At least one Data Frame has been stored into the respective Message Buffer"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ppi_SPEC;
    pub type Ppi = crate::EnumBitfieldStruct<u8, Ppi_SPEC>;
    impl Ppi {
        #[doc = "0 The Payload Segment of the received Frame does not contain a Network Management vector nor a message ID"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Static segment  Network Management vector in the first part of the payload Dynamic segment  Message ID in the first part of the payload"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scv_SPEC;
impl crate::sealed::RegSpec for Scv_SPEC {
    type DataType = u32;
}
#[doc = "Slot Counter Value\n resetvalue={Application Reset:0x0}"]
pub type Scv = crate::RegValueT<Scv_SPEC>;

impl Scv {
    #[doc = "Slot Counter Channel A vSlotCounter A     SCCA. Current slot counter value on channel A. The value is incremented by the        Communication Controller and reset at the start of a communication        cycle. Valid values are 0 to 2047  0 H to 7FF H  ."]
    #[inline(always)]
    pub fn scca(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Scv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Scv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Slot Counter Channel B vSlotCounter B     SCCB. Current slot counter value on channel B. The value is incremented by the        Communication Controller and reset at the start of a communication        cycle. Valid values are 0 to 2047  0 H to 7FF H  ."]
    #[inline(always)]
    pub fn sccb(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, Scv_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7ff,1,0,u16, Scv_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Scv {
    #[inline(always)]
    fn default() -> Scv {
        <crate::RegValueT<Scv_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfs_SPEC;
impl crate::sealed::RegSpec for Sfs_SPEC {
    type DataType = u32;
}
#[doc = "SYNC Frame Status\n resetvalue={Application Reset:0x0}"]
pub type Sfs = crate::RegValueT<Sfs_SPEC>;

impl Sfs {
    #[doc = "Valid SYNC Frames Channel A  even communication cycle   VSAE. Holds the number of valid SYNC Frames received on channel A in the even communication cycle. If transmission of SYNC Frames is enabled by SUCC1.TXSY the value is incremented by one. The value is updated during the network idle time  NIT  of each even communication cycle. This bit field is only valid if the channel A is assigned to the Communication Controller by SUCC1.CCHA."]
    #[inline(always)]
    pub fn vsae(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Sfs_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xf, 1, 0, u8, Sfs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Valid SYNC Frames Channel A  odd communication cycle   VSAO. Holds the number of valid SYNC Frames received on channel A in the odd communication cycle. If transmission of SYNC Frames is enabled by SUCC1.TXSY the value is incremented by one. The value is updated during the network idle time  NIT  of each odd communication cycle. This bit field is only valid if the channel A is assigned to the Communication Controller by SUCC1.CCHA."]
    #[inline(always)]
    pub fn vsao(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Sfs_SPEC, crate::common::R> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, Sfs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Valid SYNC Frames Channel B  even communication cycle   VSBE. Holds the number of valid SYNC Frames received on channel B in the even communication cycle. If transmission of SYNC Frames is enabled by SUCC1.TXSY the value is incremented by one. The value is updated during the network idle time  NIT  of each even communication cycle. This bit field is only valid if the channel B is assigned to the Communication Controller by SUCC1.CCHB."]
    #[inline(always)]
    pub fn vsbe(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Sfs_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xf, 1, 0, u8, Sfs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Valid SYNC Frames Channel B  odd communication cycle   VSBO. Holds the number of valid SYNC Frames received on channel B in the odd communication cycle. If transmission of SYNC Frames is enabled by SUCC1.TXSY the value is incremented by one. The value is updated during the network idle time  NIT  of each odd communication cycle. This bit field is only valid if the channel B is assigned to the Communication Controller by SUCC1.CCHB."]
    #[inline(always)]
    pub fn vsbo(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Sfs_SPEC, crate::common::R> {
        crate::common::RegisterField::<12, 0xf, 1, 0, u8, Sfs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Missing Offset Correction Signal   MOCS. The Missing Offset Correction flag signals to the Host  that no offset correction calculation can be performed because no SYNC Frames were received. The flag is updated by the Communication Controller at start of offset correction phase."]
    #[inline(always)]
    pub fn mocs(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, sfs::Mocs, Sfs_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x1,1,0,sfs::Mocs, Sfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Offset Correction Limit Reached   OCLR. The Offset Correction Limit Reached flag signals to the Host  that the offset correction value has exceeded its limit as defined by GTUC10.MOC. The flag is updated by the Communication Controller at start of offset correction phase."]
    #[inline(always)]
    pub fn oclr(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, sfs::Oclr, Sfs_SPEC, crate::common::R> {
        crate::common::RegisterField::<17,0x1,1,0,sfs::Oclr, Sfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Missing Rate Correction Signal   MRCS. The Missing Rate Correction Flag signals to the Host  that no rate correction calculation can be performed because no pairs of even   odd SYNC Frames were received. The flag is updated by the Communication Controller at start of offset correction phase."]
    #[inline(always)]
    pub fn mrcs(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, sfs::Mrcs, Sfs_SPEC, crate::common::R> {
        crate::common::RegisterField::<18,0x1,1,0,sfs::Mrcs, Sfs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Rate Correction Limit Reached   RCLR. The Rate Correction Limit Reached flag signals to the Host  that the rate correction value has exceeded its limit.as defined by GTUC10.MRC. The flag is updated by the Communication Controller at start of offset correction phase."]
    #[inline(always)]
    pub fn rclr(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, sfs::Rclr, Sfs_SPEC, crate::common::R> {
        crate::common::RegisterField::<19,0x1,1,0,sfs::Rclr, Sfs_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Sfs {
    #[inline(always)]
    fn default() -> Sfs {
        <crate::RegValueT<Sfs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sfs {
    pub struct Mocs_SPEC;
    pub type Mocs = crate::EnumBitfieldStruct<u8, Mocs_SPEC>;
    impl Mocs {
        #[doc = "0 Offset correction signal valid"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Missing offset correction signal"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oclr_SPEC;
    pub type Oclr = crate::EnumBitfieldStruct<u8, Oclr_SPEC>;
    impl Oclr {
        #[doc = "0 Offset correction below limit"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Offset correction limit reached"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mrcs_SPEC;
    pub type Mrcs = crate::EnumBitfieldStruct<u8, Mrcs_SPEC>;
    impl Mrcs {
        #[doc = "0 Rate correction signal valid"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Missing rate correction signal"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rclr_SPEC;
    pub type Rclr = crate::EnumBitfieldStruct<u8, Rclr_SPEC>;
    impl Rclr {
        #[doc = "0 Rate correction below limit"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Rate correction limit reached"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sier_SPEC;
impl crate::sealed::RegSpec for Sier_SPEC {
    type DataType = u32;
}
#[doc = "Status Service Request Enable Reset\n resetvalue={Application Reset:0x0}"]
pub type Sier = crate::RegValueT<Sier_SPEC>;

impl Sier {
    #[doc = "Wakeup Status Service Request Enable   WSTE"]
    #[inline(always)]
    pub fn wste(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, sier::Wste, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,sier::Wste, Sier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Collision Avoidance Symbol Service Request Enable   CASE"]
    #[inline(always)]
    pub fn case(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, sier::Case, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,sier::Case, Sier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Cycle Start Service Request Enable   CYCSE"]
    #[inline(always)]
    pub fn cycse(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, sier::Cycse, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,sier::Cycse, Sier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmit Service Request Enable   TXIE"]
    #[inline(always)]
    pub fn txie(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, sier::Txie, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,sier::Txie, Sier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive Service Request Enable   RXIE"]
    #[inline(always)]
    pub fn rxie(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, sier::Rxie, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,sier::Rxie, Sier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive FIFO Not Empty Service Request Enable   RFNEE"]
    #[inline(always)]
    pub fn rfnee(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, sier::Rfnee, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,sier::Rfnee, Sier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive FIFO Critical Level Service Request Enable   RFCLE"]
    #[inline(always)]
    pub fn rfcle(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, sier::Rfcle, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,sier::Rfcle, Sier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Network Management Vector Changed Service Request Enable   NMVCE"]
    #[inline(always)]
    pub fn nmvce(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, sier::Nmvce, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,sier::Nmvce, Sier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer Service Request 0 Enable   TI0E"]
    #[inline(always)]
    pub fn ti0e(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, sier::Ti0E, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,sier::Ti0E, Sier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer Service Request 1 Enable   TI1E"]
    #[inline(always)]
    pub fn ti1e(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, sier::Ti1E, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,sier::Ti1E, Sier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transfer Input Buffer Completed Service Request Enable   TIBCE"]
    #[inline(always)]
    pub fn tibce(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, sier::Tibce, Sier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,sier::Tibce, Sier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transfer Output Buffer Completed Service Request Enable   TOBCE"]
    #[inline(always)]
    pub fn tobce(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, sier::Tobce, Sier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,sier::Tobce, Sier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Stop Watch Event Service Request Enable   SWEE"]
    #[inline(always)]
    pub fn swee(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, sier::Swee, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x1,1,0,sier::Swee, Sier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Startup Completed Successfully Service Request Enable   SUCSE"]
    #[inline(always)]
    pub fn sucse(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, sier::Sucse, Sier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,sier::Sucse, Sier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Service Request Enable   MBSIE"]
    #[inline(always)]
    pub fn mbsie(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, sier::Mbsie, Sier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,sier::Mbsie, Sier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Start of Dynamic Segment Service Request Enable   SDSE"]
    #[inline(always)]
    pub fn sdse(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, sier::Sdse, Sier_SPEC, crate::common::RW> {
        crate::common::RegisterField::<15,0x1,1,0,sier::Sdse, Sier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wakeup Pattern Channel A Service Request Enable   WUPAE"]
    #[inline(always)]
    pub fn wupae(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, sier::Wupae, Sier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,sier::Wupae, Sier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Media Access Test Symbol Channel A Service Request Enable   MTSAE"]
    #[inline(always)]
    pub fn mtsae(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, sier::Mtsae, Sier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,sier::Mtsae, Sier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wakeup Pattern Channel B Service Request Enable   WUPBE"]
    #[inline(always)]
    pub fn wupbe(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, sier::Wupbe, Sier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,sier::Wupbe, Sier_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Media Access Test Symbol Channel B Service Request Enable   MTSBE"]
    #[inline(always)]
    pub fn mtsbe(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, sier::Mtsbe, Sier_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,sier::Mtsbe, Sier_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Sier {
    #[inline(always)]
    fn default() -> Sier {
        <crate::RegValueT<Sier_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sier {
    pub struct Wste_SPEC;
    pub type Wste = crate::EnumBitfieldStruct<u8, Wste_SPEC>;
    impl Wste {
        #[doc = "0 Read  Wakeup Status Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Wakeup Status Service Request enabled Write  Disable Wakeup Status Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Case_SPEC;
    pub type Case = crate::EnumBitfieldStruct<u8, Case_SPEC>;
    impl Case {
        #[doc = "0 Read  Collision Avoidance Symbol Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Collision Avoidance Symbol Service Request enabled Write  Disable Collision Avoidance Symbol Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cycse_SPEC;
    pub type Cycse = crate::EnumBitfieldStruct<u8, Cycse_SPEC>;
    impl Cycse {
        #[doc = "0 Read  Cycle Start Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Cycle Start Service Request enabled Write  Disable Cycle Start Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Txie_SPEC;
    pub type Txie = crate::EnumBitfieldStruct<u8, Txie_SPEC>;
    impl Txie {
        #[doc = "0 Read  Transmit Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Transmit Service Request enabled Write  Disable Transmit Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rxie_SPEC;
    pub type Rxie = crate::EnumBitfieldStruct<u8, Rxie_SPEC>;
    impl Rxie {
        #[doc = "0 Read  Receive Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Receive Service Request enabled Write  Disable Receive Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfnee_SPEC;
    pub type Rfnee = crate::EnumBitfieldStruct<u8, Rfnee_SPEC>;
    impl Rfnee {
        #[doc = "0 Read  Receive FIFO Not Empty Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Receive FIFO Not Empty Service Request enabled Write  Disable Receive FIFO Not Empty Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfcle_SPEC;
    pub type Rfcle = crate::EnumBitfieldStruct<u8, Rfcle_SPEC>;
    impl Rfcle {
        #[doc = "0 Read  Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Receive FIFO Critical Level Service Request enabled Write  Disable Receive FIFO Critical Level Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Nmvce_SPEC;
    pub type Nmvce = crate::EnumBitfieldStruct<u8, Nmvce_SPEC>;
    impl Nmvce {
        #[doc = "0 Read  Network Management Vector Changed Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Network Management Vector Changed Service Request enabled Write  Disable Network Management Vector Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ti0E_SPEC;
    pub type Ti0E = crate::EnumBitfieldStruct<u8, Ti0E_SPEC>;
    impl Ti0E {
        #[doc = "0 Read  Timer Service Request 0 disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Timer Service Request 0 enabled Write  Disable Service Request 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ti1E_SPEC;
    pub type Ti1E = crate::EnumBitfieldStruct<u8, Ti1E_SPEC>;
    impl Ti1E {
        #[doc = "0 Read  Timer Service Request 1 disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Timer Service Request 1 enabled Write  Disable Timer Service Request 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tibce_SPEC;
    pub type Tibce = crate::EnumBitfieldStruct<u8, Tibce_SPEC>;
    impl Tibce {
        #[doc = "0 Read  Wakeup Status Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Wakeup Status Service Request enabled Write  Disable Wakeup Status Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tobce_SPEC;
    pub type Tobce = crate::EnumBitfieldStruct<u8, Tobce_SPEC>;
    impl Tobce {
        #[doc = "0 Read  Transfer Input Buffer Completed Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Transfer Input Buffer Completed Service Request enabled Write  Disable Transfer Input Buffer Completed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Swee_SPEC;
    pub type Swee = crate::EnumBitfieldStruct<u8, Swee_SPEC>;
    impl Swee {
        #[doc = "0 Read  Stop Watch Event Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Stop Watch Event Service Request enabled Write  Disable Stop Watch Event Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sucse_SPEC;
    pub type Sucse = crate::EnumBitfieldStruct<u8, Sucse_SPEC>;
    impl Sucse {
        #[doc = "0 Read  Startup Completed Successfully Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Startup Completed Successfully Service Request enabled Write  Disable Startup Completed Successfully Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mbsie_SPEC;
    pub type Mbsie = crate::EnumBitfieldStruct<u8, Mbsie_SPEC>;
    impl Mbsie {
        #[doc = "0 Read  Message Buffer Status Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Message Buffer Status Service Request enabled Write  Disable Message Buffer Status Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sdse_SPEC;
    pub type Sdse = crate::EnumBitfieldStruct<u8, Sdse_SPEC>;
    impl Sdse {
        #[doc = "0 Read  Start of Dynamic Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Start of Dynamic Service Request enabled Write  Disable Start of Dynamic Service Reques"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wupae_SPEC;
    pub type Wupae = crate::EnumBitfieldStruct<u8, Wupae_SPEC>;
    impl Wupae {
        #[doc = "0 Read  Wakeup Pattern Channel A Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Wakeup Pattern Channel A Service Request enabled Write  Disable Wakeup Pattern Channel A Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mtsae_SPEC;
    pub type Mtsae = crate::EnumBitfieldStruct<u8, Mtsae_SPEC>;
    impl Mtsae {
        #[doc = "0 Read  Media Access Test Symbol Channel A Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Media Access Test Symbol Channel A Service Request enabled Write  Disable Media Access Test Symbol Channel A Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wupbe_SPEC;
    pub type Wupbe = crate::EnumBitfieldStruct<u8, Wupbe_SPEC>;
    impl Wupbe {
        #[doc = "0 Read  Wakeup Pattern Channel B Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Wakeup Pattern Channel B Service Request enabled Write  Disable Wakeup Pattern Channel B Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mtsbe_SPEC;
    pub type Mtsbe = crate::EnumBitfieldStruct<u8, Mtsbe_SPEC>;
    impl Mtsbe {
        #[doc = "0 Read  Media Access Test Symbol Channel B Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Media Access Test Symbol Channel B Service Request enabled Write  Disable Media Access Test Symbol Channel B Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sies_SPEC;
impl crate::sealed::RegSpec for Sies_SPEC {
    type DataType = u32;
}
#[doc = "Status Service Request Enable Set\n resetvalue={Application Reset:0x0}"]
pub type Sies = crate::RegValueT<Sies_SPEC>;

impl Sies {
    #[doc = "Wakeup Status Service Request Enable   WSTE"]
    #[inline(always)]
    pub fn wste(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, sies::Wste, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,sies::Wste, Sies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Collision Avoidance Symbol Service Request Enable   CASE"]
    #[inline(always)]
    pub fn case(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, sies::Case, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,sies::Case, Sies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Cycle Start Service Request Enable   CYCSE"]
    #[inline(always)]
    pub fn cycse(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, sies::Cycse, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,sies::Cycse, Sies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmit Service Request Enable   TXIE"]
    #[inline(always)]
    pub fn txie(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, sies::Txie, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,sies::Txie, Sies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive Service Request Enable   RXIE"]
    #[inline(always)]
    pub fn rxie(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, sies::Rxie, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,sies::Rxie, Sies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive FIFO Not Empty Service Request Enable   RFNEE"]
    #[inline(always)]
    pub fn rfnee(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, sies::Rfnee, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,sies::Rfnee, Sies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive FIFO Critical Level Service Request Enable   RFCLE"]
    #[inline(always)]
    pub fn rfcle(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, sies::Rfcle, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,sies::Rfcle, Sies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Network Management Vector Changed Service Request Enable   NMVCE"]
    #[inline(always)]
    pub fn nmvce(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, sies::Nmvce, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,sies::Nmvce, Sies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer Service Request 0 Enable   TI0E"]
    #[inline(always)]
    pub fn ti0e(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, sies::Ti0E, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,sies::Ti0E, Sies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer Service Request 1 Enable   TI1E"]
    #[inline(always)]
    pub fn ti1e(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, sies::Ti1E, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,sies::Ti1E, Sies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transfer Input Buffer Completed Service Request Enable   TIBCE"]
    #[inline(always)]
    pub fn tibce(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, sies::Tibce, Sies_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,sies::Tibce, Sies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transfer Output Buffer Completed Service Request Enable   TOBCE"]
    #[inline(always)]
    pub fn tobce(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, sies::Tobce, Sies_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,sies::Tobce, Sies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Stop Watch Event Service Request Enable   SWEE"]
    #[inline(always)]
    pub fn swee(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, sies::Swee, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x1,1,0,sies::Swee, Sies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Startup Completed Successfully Service Request Enable   SUCSE"]
    #[inline(always)]
    pub fn sucse(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, sies::Sucse, Sies_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,sies::Sucse, Sies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Service Request Enable   MBSIE"]
    #[inline(always)]
    pub fn mbsie(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, sies::Mbsie, Sies_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,sies::Mbsie, Sies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Start of Dynamic Segment Service Request Enable   SDSE"]
    #[inline(always)]
    pub fn sdse(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, sies::Sdse, Sies_SPEC, crate::common::RW> {
        crate::common::RegisterField::<15,0x1,1,0,sies::Sdse, Sies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wakeup Pattern Channel A Service Request Enable   WUPAE"]
    #[inline(always)]
    pub fn wupae(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, sies::Wupae, Sies_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,sies::Wupae, Sies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Media Access Test Symbol Channel A Service Request Enable   MTSAE"]
    #[inline(always)]
    pub fn mtsae(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, sies::Mtsae, Sies_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,sies::Mtsae, Sies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wakeup Pattern Channel B Service Request Enable   WUPBE"]
    #[inline(always)]
    pub fn wupbe(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, sies::Wupbe, Sies_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,sies::Wupbe, Sies_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Media Access Test Symbol Channel B Service Request Enable   MTSBE"]
    #[inline(always)]
    pub fn mtsbe(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, sies::Mtsbe, Sies_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,sies::Mtsbe, Sies_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Sies {
    #[inline(always)]
    fn default() -> Sies {
        <crate::RegValueT<Sies_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sies {
    pub struct Wste_SPEC;
    pub type Wste = crate::EnumBitfieldStruct<u8, Wste_SPEC>;
    impl Wste {
        #[doc = "0 Read  Wake up Status Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read Wake up Status Service Request enabled Write  Enable Wakeup Status Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Case_SPEC;
    pub type Case = crate::EnumBitfieldStruct<u8, Case_SPEC>;
    impl Case {
        #[doc = "0 Read  Collision Avoidance Symbol Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Collision Avoidance Symbol Service Request enabled Write  Enable Collision Avoidance Symbol Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cycse_SPEC;
    pub type Cycse = crate::EnumBitfieldStruct<u8, Cycse_SPEC>;
    impl Cycse {
        #[doc = "0 Read  Cycle Start Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Cycle Start Service Request enabled Write  Enable Cycle Start Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Txie_SPEC;
    pub type Txie = crate::EnumBitfieldStruct<u8, Txie_SPEC>;
    impl Txie {
        #[doc = "0 Read  Transmit Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transmit Service Request enabled Write  Enable Transmit Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rxie_SPEC;
    pub type Rxie = crate::EnumBitfieldStruct<u8, Rxie_SPEC>;
    impl Rxie {
        #[doc = "0 Read  Receive Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Receive Service Request enabled Write  Enable Receive Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfnee_SPEC;
    pub type Rfnee = crate::EnumBitfieldStruct<u8, Rfnee_SPEC>;
    impl Rfnee {
        #[doc = "0 Read  Receive FIFO Not Empty Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Receive FIFO Not Empty Service Request enabled Write  Enable Receive FIFO Not Empty Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfcle_SPEC;
    pub type Rfcle = crate::EnumBitfieldStruct<u8, Rfcle_SPEC>;
    impl Rfcle {
        #[doc = "0 Read  Receive FIFO Critical Level Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Receive FIFO Critical Level Service Request enabled Write  Enable Receive FIFO Critical Level Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Nmvce_SPEC;
    pub type Nmvce = crate::EnumBitfieldStruct<u8, Nmvce_SPEC>;
    impl Nmvce {
        #[doc = "0 Read  Network Management Vector Changed Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Network Management Vector Changed Service Request enabled Write  Enable Network Management Vector Changed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ti0E_SPEC;
    pub type Ti0E = crate::EnumBitfieldStruct<u8, Ti0E_SPEC>;
    impl Ti0E {
        #[doc = "0 Read  Timer Service Request 0disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Timer Service Request 0 enabled Write  Enable Timer Service Request 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ti1E_SPEC;
    pub type Ti1E = crate::EnumBitfieldStruct<u8, Ti1E_SPEC>;
    impl Ti1E {
        #[doc = "0 Read  Timer Service Request 1disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Timer Service Request 1 enabled Write  Enable Timer Service Request 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tibce_SPEC;
    pub type Tibce = crate::EnumBitfieldStruct<u8, Tibce_SPEC>;
    impl Tibce {
        #[doc = "0 Read  Wakeup Status Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Wakeup Status Service Request enabled Write  Enable Wakeup Status Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tobce_SPEC;
    pub type Tobce = crate::EnumBitfieldStruct<u8, Tobce_SPEC>;
    impl Tobce {
        #[doc = "0 Read  Transfer Input Buffer Completed Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Transfer Input Buffer Completed Service Request enabled Write  Enable Transfer Input Buffer Completed Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Swee_SPEC;
    pub type Swee = crate::EnumBitfieldStruct<u8, Swee_SPEC>;
    impl Swee {
        #[doc = "0 Read  Stop Watch Event Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Stop Watch Event Service Request enabled Write  Enable Stop Watch Event Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sucse_SPEC;
    pub type Sucse = crate::EnumBitfieldStruct<u8, Sucse_SPEC>;
    impl Sucse {
        #[doc = "0 Read  Startup Completed Successfully Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Startup Completed Successfully Service Request enabled Write  Enable Startup Completed Successfully Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mbsie_SPEC;
    pub type Mbsie = crate::EnumBitfieldStruct<u8, Mbsie_SPEC>;
    impl Mbsie {
        #[doc = "0 Read  Message Buffer Status Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Message Buffer Status Service Request enabled Write  Enable Message Buffer Status Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sdse_SPEC;
    pub type Sdse = crate::EnumBitfieldStruct<u8, Sdse_SPEC>;
    impl Sdse {
        #[doc = "0 Read  Start of Dynamic Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Start of Dynamic Service Request enabled Write  Enable Start of Dynamic Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wupae_SPEC;
    pub type Wupae = crate::EnumBitfieldStruct<u8, Wupae_SPEC>;
    impl Wupae {
        #[doc = "0 Read  Wakeup Pattern Channel A Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Wakeup Pattern Channel A Service Request enabled Write  Enable Wakeup Pattern Channel A Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mtsae_SPEC;
    pub type Mtsae = crate::EnumBitfieldStruct<u8, Mtsae_SPEC>;
    impl Mtsae {
        #[doc = "0 Read  Media Access Test Symbol Channel A Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Media Access Test Symbol Channel A Service Request enabled Write  Enable Media Access Test Symbol Channel A Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wupbe_SPEC;
    pub type Wupbe = crate::EnumBitfieldStruct<u8, Wupbe_SPEC>;
    impl Wupbe {
        #[doc = "0 Read  Wakeup Pattern Channel B Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Wakeup Pattern Channel B Service Request enabled Write  Enable Wakeup Pattern Channel B Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mtsbe_SPEC;
    pub type Mtsbe = crate::EnumBitfieldStruct<u8, Mtsbe_SPEC>;
    impl Mtsbe {
        #[doc = "0 Read  Media Access Test Symbol Channel B Service Request disabled Write  Unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read  Media Access Test Symbol Channel B Service Request enabled Write  Enable Media Access Test Symbol Channel B Service Request"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sils_SPEC;
impl crate::sealed::RegSpec for Sils_SPEC {
    type DataType = u32;
}
#[doc = "Status Service Request Line Select\n resetvalue={Application Reset:0x303FFFF}"]
pub type Sils = crate::RegValueT<Sils_SPEC>;

impl Sils {
    #[doc = "Wakeup Status Service Request Line   WSTL"]
    #[inline(always)]
    pub fn wstl(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, sils::Wstl, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,sils::Wstl, Sils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Collision Avoidance Symbol Service Request Line   CASL"]
    #[inline(always)]
    pub fn casl(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, sils::Casl, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,sils::Casl, Sils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Cycle Start Service Request Line   CYCSL"]
    #[inline(always)]
    pub fn cycsl(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, sils::Cycsl, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,sils::Cycsl, Sils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmit Service Request Line   TXIL"]
    #[inline(always)]
    pub fn txil(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, sils::Txil, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,sils::Txil, Sils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive Service Request Line   RXIL"]
    #[inline(always)]
    pub fn rxil(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, sils::Rxil, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,sils::Rxil, Sils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive FIFO Not Empty Service Request Line   RFNEL"]
    #[inline(always)]
    pub fn rfnel(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, sils::Rfnel, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,sils::Rfnel, Sils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive FIFO Critical Level Service Request Line   RFCLL"]
    #[inline(always)]
    pub fn rfcll(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, sils::Rfcll, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,sils::Rfcll, Sils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Network Management Vector Changed Service Request Line   NMVCL"]
    #[inline(always)]
    pub fn nmvcl(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, sils::Nmvcl, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,sils::Nmvcl, Sils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer Service Request 0 Line   TI0L"]
    #[inline(always)]
    pub fn ti0l(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, sils::Ti0L, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,sils::Ti0L, Sils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer Service Request 1 Line   TI1L"]
    #[inline(always)]
    pub fn ti1l(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, sils::Ti1L, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,sils::Ti1L, Sils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transfer Input Buffer Completed Service Request Line   TIBCL"]
    #[inline(always)]
    pub fn tibcl(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, sils::Tibcl, Sils_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,sils::Tibcl, Sils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transfer Output Buffer Completed Service Request Line   TOBCL"]
    #[inline(always)]
    pub fn tobcl(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, sils::Tobcl, Sils_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,sils::Tobcl, Sils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Stop Watch Event Service Request Line   SWEL"]
    #[inline(always)]
    pub fn swel(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, sils::Swel, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x1,1,0,sils::Swel, Sils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Startup Completed Successfully Service Request Line   SUCSL"]
    #[inline(always)]
    pub fn sucsl(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, sils::Sucsl, Sils_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,sils::Sucsl, Sils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Service Request Line   MBSIL"]
    #[inline(always)]
    pub fn mbsil(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, sils::Mbsil, Sils_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,sils::Mbsil, Sils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Start of Dynamic Segment Service Request Line   SDSL"]
    #[inline(always)]
    pub fn sdsl(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, sils::Sdsl, Sils_SPEC, crate::common::RW> {
        crate::common::RegisterField::<15,0x1,1,0,sils::Sdsl, Sils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wakeup Pattern Channel A Service Request Line   WUPAL"]
    #[inline(always)]
    pub fn wupal(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, sils::Wupal, Sils_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,sils::Wupal, Sils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Media Access Test Symbol Channel A Service Request Line   MTSAL"]
    #[inline(always)]
    pub fn mtsal(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, sils::Mtsal, Sils_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,sils::Mtsal, Sils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wakeup Pattern Channel B Service Request Line   WUPBL"]
    #[inline(always)]
    pub fn wupbl(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, sils::Wupbl, Sils_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,sils::Wupbl, Sils_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Media Access Test Symbol Channel B Service Request Line   MTSBL"]
    #[inline(always)]
    pub fn mtsbl(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, sils::Mtsbl, Sils_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,sils::Mtsbl, Sils_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Sils {
    #[inline(always)]
    fn default() -> Sils {
        <crate::RegValueT<Sils_SPEC> as RegisterValue<_>>::new(50593791)
    }
}
pub mod sils {
    pub struct Wstl_SPEC;
    pub type Wstl = crate::EnumBitfieldStruct<u8, Wstl_SPEC>;
    impl Wstl {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Casl_SPEC;
    pub type Casl = crate::EnumBitfieldStruct<u8, Casl_SPEC>;
    impl Casl {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cycsl_SPEC;
    pub type Cycsl = crate::EnumBitfieldStruct<u8, Cycsl_SPEC>;
    impl Cycsl {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Txil_SPEC;
    pub type Txil = crate::EnumBitfieldStruct<u8, Txil_SPEC>;
    impl Txil {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rxil_SPEC;
    pub type Rxil = crate::EnumBitfieldStruct<u8, Rxil_SPEC>;
    impl Rxil {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfnel_SPEC;
    pub type Rfnel = crate::EnumBitfieldStruct<u8, Rfnel_SPEC>;
    impl Rfnel {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfcll_SPEC;
    pub type Rfcll = crate::EnumBitfieldStruct<u8, Rfcll_SPEC>;
    impl Rfcll {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Nmvcl_SPEC;
    pub type Nmvcl = crate::EnumBitfieldStruct<u8, Nmvcl_SPEC>;
    impl Nmvcl {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ti0L_SPEC;
    pub type Ti0L = crate::EnumBitfieldStruct<u8, Ti0L_SPEC>;
    impl Ti0L {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ti1L_SPEC;
    pub type Ti1L = crate::EnumBitfieldStruct<u8, Ti1L_SPEC>;
    impl Ti1L {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tibcl_SPEC;
    pub type Tibcl = crate::EnumBitfieldStruct<u8, Tibcl_SPEC>;
    impl Tibcl {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tobcl_SPEC;
    pub type Tobcl = crate::EnumBitfieldStruct<u8, Tobcl_SPEC>;
    impl Tobcl {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Swel_SPEC;
    pub type Swel = crate::EnumBitfieldStruct<u8, Swel_SPEC>;
    impl Swel {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sucsl_SPEC;
    pub type Sucsl = crate::EnumBitfieldStruct<u8, Sucsl_SPEC>;
    impl Sucsl {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mbsil_SPEC;
    pub type Mbsil = crate::EnumBitfieldStruct<u8, Mbsil_SPEC>;
    impl Mbsil {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sdsl_SPEC;
    pub type Sdsl = crate::EnumBitfieldStruct<u8, Sdsl_SPEC>;
    impl Sdsl {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wupal_SPEC;
    pub type Wupal = crate::EnumBitfieldStruct<u8, Wupal_SPEC>;
    impl Wupal {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mtsal_SPEC;
    pub type Mtsal = crate::EnumBitfieldStruct<u8, Mtsal_SPEC>;
    impl Mtsal {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wupbl_SPEC;
    pub type Wupbl = crate::EnumBitfieldStruct<u8, Wupbl_SPEC>;
    impl Wupbl {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mtsbl_SPEC;
    pub type Mtsbl = crate::EnumBitfieldStruct<u8, Mtsbl_SPEC>;
    impl Mtsbl {
        #[doc = "0 Service Request assigned to service request line eray int0 INT0SRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request assigned to service request line eray int1 INT1SRC"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sir_SPEC;
impl crate::sealed::RegSpec for Sir_SPEC {
    type DataType = u32;
}
#[doc = "Status Service Request Register\n resetvalue={Application Reset:0x0}"]
pub type Sir = crate::RegValueT<Sir_SPEC>;

impl Sir {
    #[doc = "Wakeup Status   WST. This flag is set when the wakeup status vector CCSV.WSV in the Communication Controller Status Vector register changes to a value other than UNDEFINED. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn wst(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, sir::Wst, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,sir::Wst, Sir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Collision Avoidance Symbol   CAS. This flag is set by the Communication Controller during STARTUP state when a CAS or potential CAS was received. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn cas(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, sir::Cas, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,sir::Cas, Sir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Cycle Start Service Request   CYCS. This flag is set by the Communication Controller when a communication        cycle starts This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn cycs(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, sir::Cycs, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,sir::Cycs, Sir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmit Service Request   TXI. This flag is set by the Communication Controller at the end of Frame        transmission if bit WRHS1.MBI in the respective Message Buffer is set. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn txi(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, sir::Txi, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,sir::Txi, Sir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive Service Request   RXI. This flag is set by the Communication Controller whenever the set        condition of a Message Buffer ND flag is fulfilled and if bit WRHS1.MBI        of that Message Buffer is set to 1. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn rxi(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, sir::Rxi, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,sir::Rxi, Sir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive FIFO Not Empty   RFNE. This flag is set by the Communication Controller when a received valid        Frame was stored into the empty receive FIFO.m The actual state of the        receive FIFO is monitored in register FSR"]
    #[inline(always)]
    pub fn rfne(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, sir::Rfne, Sir_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x1,1,0,sir::Rfne, Sir_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive FIFO Critical Level   RFCL. This flag is set when a valid receive FIFO fill level FSR.RFFL is equal or greater than the critical level as configured by FCL.CL."]
    #[inline(always)]
    pub fn rfcl(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, sir::Rfcl, Sir_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x1,1,0,sir::Rfcl, Sir_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Network Management Vector Changed   NMVC. This service request flag signals a change in the Network Management Vector visible to the Host. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn nmvc(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, sir::Nmvc, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,sir::Nmvc, Sir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer Service Request 0   TI0. This flag is set whenever timer 0 matches the conditions configured in the Timer Service Request 0 Configuration Register T0C. A Timer Service Request 0 is also signalled by TINT0SRC. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn ti0(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, sir::Ti0, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,sir::Ti0, Sir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer Service Request 1   TI1. This flag is set whenever the conditions programmed in the Timer Service Request 1 Configuration Register T1C are met. A Timer Service Request 1 is also signalled by TINT1SRC. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn ti1(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, sir::Ti1, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,sir::Ti1, Sir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transfer Input Buffer Completed   TIBC. This flag is set whenever a transfer from Input Buffer to the Message RAM has completed and bit IBCR.IBSYS in the Input Buffer Command Request register has been reset by the Message Handler. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn tibc(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, sir::Tibc, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x1,1,0,sir::Tibc, Sir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transfer Output Buffer Completed   TOBC. This flag is set whenever a transfer from Message RAM to the Output Buffer has completed and bit OBCR.OBSYS in the Output Buffer Command Request register has been reset by the Message Handler. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn tobc(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, sir::Tobc, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x1,1,0,sir::Tobc, Sir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Stop Watch Event   SWE. This flag is set after a stop watch activation when the current cycle counter and Macrotick value are stored in the Stop Watch Register 1  STPW1 . This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn swe(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, sir::Swe, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x1,1,0,sir::Swe, Sir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Startup Completed Successfully   SUCS. This flag is set whenever a startup completed successfully and the Communication Controller entered  NORMAL ACTIVE  state. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn sucs(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, sir::Sucs, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x1,1,0,sir::Sucs, Sir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Status Service Request   MBSI. This flag is set by the Communication Controller when the Message Buffer        status MBS has changed and if bit RDHS1.MBI of that Message Buffer is        set. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn mbsi(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, sir::Mbsi, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x1,1,0,sir::Mbsi, Sir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Start of Dynamic Segment   SDS. This flag is set by the Communication Controller when the dynamic        segment starts."]
    #[inline(always)]
    pub fn sds(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, sir::Sds, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<15,0x1,1,0,sir::Sds, Sir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wakeup Pattern Channel A   WUPA. This flag is set by the Communication Controller when a wakeup pattern was received on channel A. Only set when the Communication Controller is in  WAKEUP    READY   or  STARTUP  state  or when in Monitor mode. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn wupa(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, sir::Wupa, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1,1,0,sir::Wupa, Sir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "MTS Received on Channel A vSS ValidMTSA    MTSA. Media Access Test symbol received on channel A during the proceeding symbol window. Updated by the Communication Controller for each channel at the end of the symbol window. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn mtsa(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, sir::Mtsa, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<17,0x1,1,0,sir::Mtsa, Sir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wakeup Pattern Channel B   WUPB. This flag is set by the Communication Controller when a wakeup pattern        was received on channel B. Only set when the Communication Controller is        in   8220 WAKEUP  8221     8220 READY  8221   or   8220 STARTUP  8221  state  or when in Monitor mode. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn wupb(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, sir::Wupb, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1,1,0,sir::Wupb, Sir_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "MTS Received on Channel B   MTSB. Media Access Test symbol received on channel B during the proceeding        symbol window. Updated by the Communication Controller for each channel        at the end of the symbol window. This flag is cleared by writing a 1."]
    #[inline(always)]
    pub fn mtsb(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, sir::Mtsb, Sir_SPEC, crate::common::RW> {
        crate::common::RegisterField::<25,0x1,1,0,sir::Mtsb, Sir_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Sir {
    #[inline(always)]
    fn default() -> Sir {
        <crate::RegValueT<Sir_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod sir {
    pub struct Wst_SPEC;
    pub type Wst = crate::EnumBitfieldStruct<u8, Wst_SPEC>;
    impl Wst {
        #[doc = "0 Wake up status unmodified"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Wake up status modified  and not UNDEFINED"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cas_SPEC;
    pub type Cas = crate::EnumBitfieldStruct<u8, Cas_SPEC>;
    impl Cas {
        #[doc = "0 No bit pattern matching the CAS symbol received"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit pattern matching the CAS symbol received"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cycs_SPEC;
    pub type Cycs = crate::EnumBitfieldStruct<u8, Cycs_SPEC>;
    impl Cycs {
        #[doc = "0 No communication cycle started"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Communication cycle started"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Txi_SPEC;
    pub type Txi = crate::EnumBitfieldStruct<u8, Txi_SPEC>;
    impl Txi {
        #[doc = "0 No Frame transmitted from a transmit buffer with WRHS1.MBI   1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 At least one Frame was transmitted from a transmit buffer with WRHS1.MBI   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rxi_SPEC;
    pub type Rxi = crate::EnumBitfieldStruct<u8, Rxi_SPEC>;
    impl Rxi {
        #[doc = "0 No ND flag of a receive buffer with WRHS1.MBI   1 has been set to 1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 At least one ND flag of a receive buffer with WRHS1.MBI   1 has been set to 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfne_SPEC;
    pub type Rfne = crate::EnumBitfieldStruct<u8, Rfne_SPEC>;
    impl Rfne {
        #[doc = "0 Receive FIFO is empty"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Receive FIFO is not empty"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfcl_SPEC;
    pub type Rfcl = crate::EnumBitfieldStruct<u8, Rfcl_SPEC>;
    impl Rfcl {
        #[doc = "0 Receive FIFO below critical level"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Receive FIFO critical level reached"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Nmvc_SPEC;
    pub type Nmvc = crate::EnumBitfieldStruct<u8, Nmvc_SPEC>;
    impl Nmvc {
        #[doc = "0 No change in the Network Management vector"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Network Management vector changed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ti0_SPEC;
    pub type Ti0 = crate::EnumBitfieldStruct<u8, Ti0_SPEC>;
    impl Ti0 {
        #[doc = "0 No Timer Service Request 0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer Service Request 0 occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ti1_SPEC;
    pub type Ti1 = crate::EnumBitfieldStruct<u8, Ti1_SPEC>;
    impl Ti1 {
        #[doc = "0 No Timer Service Request 1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer Service Request 1 occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tibc_SPEC;
    pub type Tibc = crate::EnumBitfieldStruct<u8, Tibc_SPEC>;
    impl Tibc {
        #[doc = "0 No transfer completed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transfer between Input Buffer and Message RAM completed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tobc_SPEC;
    pub type Tobc = crate::EnumBitfieldStruct<u8, Tobc_SPEC>;
    impl Tobc {
        #[doc = "0 No transfer completed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transfer between Message RAM and the Output Buffer completed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Swe_SPEC;
    pub type Swe = crate::EnumBitfieldStruct<u8, Swe_SPEC>;
    impl Swe {
        #[doc = "0 No Stop Watch Event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Stop Watch Event occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sucs_SPEC;
    pub type Sucs = crate::EnumBitfieldStruct<u8, Sucs_SPEC>;
    impl Sucs {
        #[doc = "0 No startup completed successfully"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Startup completed successfully"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mbsi_SPEC;
    pub type Mbsi = crate::EnumBitfieldStruct<u8, Mbsi_SPEC>;
    impl Mbsi {
        #[doc = "0 No Message Buffer status change of Message Buffer with RDHS1.MBI  1 has changed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Message Buffer status of at least one Message Buffer with RDHS1.MBI  1 has changed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sds_SPEC;
    pub type Sds = crate::EnumBitfieldStruct<u8, Sds_SPEC>;
    impl Sds {
        #[doc = "0 Dynamic segment not yet started"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Dynamic segment started"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wupa_SPEC;
    pub type Wupa = crate::EnumBitfieldStruct<u8, Wupa_SPEC>;
    impl Wupa {
        #[doc = "0 No wake up pattern received on channel A"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Wake up pattern received on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mtsa_SPEC;
    pub type Mtsa = crate::EnumBitfieldStruct<u8, Mtsa_SPEC>;
    impl Mtsa {
        #[doc = "0 No MTS symbol received on channel A"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MTS symbol received on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wupb_SPEC;
    pub type Wupb = crate::EnumBitfieldStruct<u8, Wupb_SPEC>;
    impl Wupb {
        #[doc = "0 No wake up pattern received on channel B"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Wake up pattern received on channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mtsb_SPEC;
    pub type Mtsb = crate::EnumBitfieldStruct<u8, Mtsb_SPEC>;
    impl Mtsb {
        #[doc = "0 No MTS symbol received on channel B"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MTS symbol received on channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stpw1_SPEC;
impl crate::sealed::RegSpec for Stpw1_SPEC {
    type DataType = u32;
}
#[doc = "Stop Watch Register 1\n resetvalue={Application Reset:0x0}"]
pub type Stpw1 = crate::RegValueT<Stpw1_SPEC>;

impl Stpw1 {
    #[doc = "Enable Stop Watch Trigger   ESWT. If enabled an edge on input STPW  pin eray stpwt   if embedded  eray stpwt0  eray stpwt1  eray stpwt2  or eray stpwt3  or a service request 0 or 1 event  rising edge on signal INT0SR or INT1SR   eray int0 or eray int 1  activates the stop watch. In single shot mode this bit is reset to 0 after the actual cycle counter and Macrotick value are stored in the Stop Watch register."]
    #[inline(always)]
    pub fn eswt(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, stpw1::Eswt, Stpw1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,stpw1::Eswt, Stpw1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Stop Watch Mode Select   SWMS. It is not possible to change the Stop Watch Mode during enabled stop watch trigger  STPW1.ESWT"]
    #[inline(always)]
    pub fn swms(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, stpw1::Swms, Stpw1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,stpw1::Swms, Stpw1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Stop Watch Trigger Edge Select   EDGE"]
    #[inline(always)]
    pub fn edge(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, stpw1::Edge, Stpw1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,stpw1::Edge, Stpw1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Software Stop Watch Trigger   SSWT. When the Host writes this bit to 1 the stop watch is activated. After the actual cycle counter and Macrotick value are stored in the Stop Watch register this bit is reset to 0. The bit is only writeable while ESWT   0."]
    #[inline(always)]
    pub fn sswt(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, stpw1::Sswt, Stpw1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,stpw1::Sswt, Stpw1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable External Trigger Pin   EETP. Enables stop watch trigger event via signal STPW  eray stpwt  if ESWT   1."]
    #[inline(always)]
    pub fn eetp(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, stpw1::Eetp, Stpw1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,stpw1::Eetp, Stpw1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Service Request 0 Trigger   EINT0. Enables stop watch trigger by service request 0 event if ESWT   1."]
    #[inline(always)]
    pub fn eint0(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, stpw1::Eint0, Stpw1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,stpw1::Eint0, Stpw1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Service Request 1 Trigger   EINT1. Enables stop watch trigger by service request 1 event if ESWT   1."]
    #[inline(always)]
    pub fn eint1(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, stpw1::Eint1, Stpw1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,stpw1::Eint1, Stpw1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Stopped Cycle Counter Value   SCCV. State of the cycle counter when the stop watch event occurred. Valid values are  0 3F H Valid Values"]
    #[inline(always)]
    pub fn sccv(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, Stpw1_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3f,1,0,u8, Stpw1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Stopped Macrotick Value   SMTV. State of the Macrotick counter when the stop watch event occurred. Valid values are  0 3F H Valid Values"]
    #[inline(always)]
    pub fn smtv(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, Stpw1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3fff,1,0,u16, Stpw1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Stpw1 {
    #[inline(always)]
    fn default() -> Stpw1 {
        <crate::RegValueT<Stpw1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod stpw1 {
    pub struct Eswt_SPEC;
    pub type Eswt = crate::EnumBitfieldStruct<u8, Eswt_SPEC>;
    impl Eswt {
        #[doc = "0 Stop watch trigger disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Stop watch trigger enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Swms_SPEC;
    pub type Swms = crate::EnumBitfieldStruct<u8, Swms_SPEC>;
    impl Swms {
        #[doc = "0 Single shot mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Continuous mode"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Edge_SPEC;
    pub type Edge = crate::EnumBitfieldStruct<u8, Edge_SPEC>;
    impl Edge {
        #[doc = "0 Falling Edge"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Rising Edge"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sswt_SPEC;
    pub type Sswt = crate::EnumBitfieldStruct<u8, Sswt_SPEC>;
    impl Sswt {
        #[doc = "0 Software trigger reset"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Stop watch activated by software trigger"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eetp_SPEC;
    pub type Eetp = crate::EnumBitfieldStruct<u8, Eetp_SPEC>;
    impl Eetp {
        #[doc = "0 Stop watch trigger via signal STPW  eray stpwt  disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Edge on signal STPW  eray stpwt  triggers stop watch"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eint0_SPEC;
    pub type Eint0 = crate::EnumBitfieldStruct<u8, Eint0_SPEC>;
    impl Eint0 {
        #[doc = "0 Stop watch trigger by service request 0 disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request 0 event triggers stop watch"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eint1_SPEC;
    pub type Eint1 = crate::EnumBitfieldStruct<u8, Eint1_SPEC>;
    impl Eint1 {
        #[doc = "0 Stop watch trigger by service request 1 disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Service Request 1 event triggers stop watch"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stpw2_SPEC;
impl crate::sealed::RegSpec for Stpw2_SPEC {
    type DataType = u32;
}
#[doc = "Stop Watch Register 2\n resetvalue={Application Reset:0x0}"]
pub type Stpw2 = crate::RegValueT<Stpw2_SPEC>;

impl Stpw2 {
    #[doc = "Stop Watch Captured Slot Counter Value Channel A   SSCVA. State of the slot counter for channel A when the stop watch event occurred. Valid values are 0 to 2047  0 H to 7FF H  ."]
    #[inline(always)]
    pub fn sscva(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Stpw2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Stpw2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Stop Watch Captured Slot Counter Value Channel B   SSCVB. State of the slot counter for channel B when the stop watch event occurred. Valid values are 0 to 2047  0 H to 7FF H  ."]
    #[inline(always)]
    pub fn sscvb(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, Stpw2_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7ff,1,0,u16, Stpw2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Stpw2 {
    #[inline(always)]
    fn default() -> Stpw2 {
        <crate::RegValueT<Stpw2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Succ1_SPEC;
impl crate::sealed::RegSpec for Succ1_SPEC {
    type DataType = u32;
}
#[doc = "SUC Configuration Register 1\n resetvalue={Application Reset:0x0C401000}"]
pub type Succ1 = crate::RegValueT<Succ1_SPEC>;

impl Succ1 {
    #[doc = "POC Busy   PBSY. Signals that the POC is busy and cannot accept a command from the Host.        SUCC1.CMD is locked against write accesses. Set to 1 after hard reset        during initialization of internal RAM blocks."]
    #[inline(always)]
    pub fn pbsy(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, succ1::Pbsy, Succ1_SPEC, crate::common::R> {
        crate::common::RegisterField::<7,0x1,1,0,succ1::Pbsy, Succ1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit Startup Frame in Key Slot   pKeySlotUsedForStartup    TXST. Defines whether the key slot is used to transmit startup Frames. The bit        can be modified in   8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221  state only. 1         2"]
    #[inline(always)]
    pub fn txst(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, succ1::Txst, Succ1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,succ1::Txst, Succ1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmit SYNC Frame in Key Slot  pKeySlotUsedForSync    TXSY. Defines whether the key slot is used to transmit SYNC Frames. The bit        can be modified in   8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221  state only. 1         2"]
    #[inline(always)]
    pub fn txsy(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, succ1::Txsy, Succ1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,succ1::Txsy, Succ1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Cold Start Attempts  gColdStartAttempts    CSA. Configures the maximum number of attempts that a cold starting node is        permitted to try to start up the network without receiving any valid        response from another node. It can be modified in   8220 DEFAULT CONFIG  8221  or          8220 CONFIG  8221  state only. Must be identical in all nodes of a cluster. Valid        values are 2 to 31. 1"]
    #[inline(always)]
    pub fn csa(
        self,
    ) -> crate::common::RegisterField<11, 0x1f, 1, 0, u8, Succ1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x1f,1,0,u8, Succ1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Passive to Active  pAllowPassiveToActive    PTA. Defines the number of consecutive even   odd cycle pairs that must have        valid clock correction terms before the Communication Controller is        allowed to transit from   8220 NORMAL PASSIVE  8221  to   8220 NORMAL ACTIVE  8221  state. If        set to 00000 B the Communication        Controller is not allowed to transit from   8220 NORMAL PASSIVE  8221  to          8220 NORMAL ACTIVE  8221  state. It can be modified in   8220 DEFAULT CONFIG  8221  or          8220 CONFIG  8221  state only. Valid values are 0 to 31 even   odd cycle pairs. 1"]
    #[inline(always)]
    pub fn pta(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Succ1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Succ1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Wakeup Channel Select  pWakeupChannel    WUCS. With this bit the Host selects the channel on which the Communication        Controller sends the Wakeup pattern. The Communication Controller        ignores any attempt to change the status of this bit when not in          8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221  state. 1"]
    #[inline(always)]
    pub fn wucs(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, succ1::Wucs, Succ1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,succ1::Wucs, Succ1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmission Slot Mode  pSingleSlotEnabled    TSM. Selects the initial transmission slot mode. In SINGLE slot mode the        Communication Controller may only transmit in the preconfigured key        slot. The key slot ID is configured in the Header Section of Message        Buffer 0 respectively Message Buffers 0 and 1 depending on bit MRC.SPLM.        In case SUCC1.TSM   1  Message Buffer 0 respectively Message Buffers 0 1        can be  re configured in   8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221  state only. In ALL        slot mode the Communication Controller may transmit in all slots. The        bit can be written in   8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221  state only. The        communication controller changes to ALL slot mode when the Host        successfully applied the ALL SLOTS command by writing SUCC1.CMD  160    160 0101 B in POC states   8220 NORMAL ACTIVE  8221  or   8220 NORMAL PASSIVE  8221 . The actual slot mode        is monitored by CCSV.SLM. 1"]
    #[inline(always)]
    pub fn tsm(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, succ1::Tsm, Succ1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,succ1::Tsm, Succ1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Halt due to Clock Sync Error  pAllowHaltDueToClock    HCSE. Controls the transition to   8220 HALT  8221  state due to a clock synchronization        error. The bit can be modified in   8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221  state        only. 1"]
    #[inline(always)]
    pub fn hcse(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, succ1::Hcse, Succ1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,succ1::Hcse, Succ1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Channel A for MTS Transmission   MTSA. The bit selects channel A for MTS symbol transmission. The flag is reset        by default and may be modified only in   8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221         state. 1  3"]
    #[inline(always)]
    pub fn mtsa(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, succ1::Mtsa, Succ1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,succ1::Mtsa, Succ1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Channel B for MTS Transmission   MTSB. The bit selects channel B for MTS symbol transmission. The flag is reset        by default and may be modified only in   8220 DEFAULT CONFIG  8221  or   8220 CONFIG  8221         state. 2"]
    #[inline(always)]
    pub fn mtsb(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, succ1::Mtsb, Succ1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,succ1::Mtsb, Succ1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Connected to Channel A  pChannels    CCHA. Configures whether the node is connected to channel A. 1"]
    #[inline(always)]
    pub fn ccha(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, succ1::Ccha, Succ1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,succ1::Ccha, Succ1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Connected to Channel B  pChannels    CCHB. Configures whether the node is connected to channel B. 1"]
    #[inline(always)]
    pub fn cchb(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, succ1::Cchb, Succ1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,succ1::Cchb, Succ1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Succ1 {
    #[inline(always)]
    fn default() -> Succ1 {
        <crate::RegValueT<Succ1_SPEC> as RegisterValue<_>>::new(205524992)
    }
}
pub mod succ1 {
    pub struct Pbsy_SPEC;
    pub type Pbsy = crate::EnumBitfieldStruct<u8, Pbsy_SPEC>;
    impl Pbsy {
        #[doc = "0 POC not busy  SUCC1.CMD writable"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 POC is busy  SUCC1.CMD locked"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Txst_SPEC;
    pub type Txst = crate::EnumBitfieldStruct<u8, Txst_SPEC>;
    impl Txst {
        #[doc = "0 No Startup Frame transmission in key slot  node is non coldstarter"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Key slot used to transmit startup Frame  node is leading or following coldstarter"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Txsy_SPEC;
    pub type Txsy = crate::EnumBitfieldStruct<u8, Txsy_SPEC>;
    impl Txsy {
        #[doc = "0 No SYNC Frame transmission in key slot  node is neither sync nor coldstart node"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Key slot used to transmit SYNC Frames  node is sync node"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wucs_SPEC;
    pub type Wucs = crate::EnumBitfieldStruct<u8, Wucs_SPEC>;
    impl Wucs {
        #[doc = "0 Send wakeup pattern on channel A"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Send wakeup pattern on channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tsm_SPEC;
    pub type Tsm = crate::EnumBitfieldStruct<u8, Tsm_SPEC>;
    impl Tsm {
        #[doc = "0 ALL Slot Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SINGLE Slot Mode  default after application reset"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hcse_SPEC;
    pub type Hcse = crate::EnumBitfieldStruct<u8, Hcse_SPEC>;
    impl Hcse {
        #[doc = "0 Communication Controller will enter   remain in  NORMAL PASSIVE"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Communication Controller will enter  HALT  state"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mtsa_SPEC;
    pub type Mtsa = crate::EnumBitfieldStruct<u8, Mtsa_SPEC>;
    impl Mtsa {
        #[doc = "0 Channel A disabled for MTS transmission"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Channel A selected for MTS transmission"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mtsb_SPEC;
    pub type Mtsb = crate::EnumBitfieldStruct<u8, Mtsb_SPEC>;
    impl Mtsb {
        #[doc = "0 Channel B disabled for MTS transmission"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Channel B selected for MTS transmission"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ccha_SPEC;
    pub type Ccha = crate::EnumBitfieldStruct<u8, Ccha_SPEC>;
    impl Ccha {
        #[doc = "0 Not connected to channel A"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Node connected to channel A  default after application reset"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cchb_SPEC;
    pub type Cchb = crate::EnumBitfieldStruct<u8, Cchb_SPEC>;
    impl Cchb {
        #[doc = "0 Not connected        to channel B"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Node connected        to channel B  default after application reset"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Succ2_SPEC;
impl crate::sealed::RegSpec for Succ2_SPEC {
    type DataType = u32;
}
#[doc = "SUC Configuration Register 2\n resetvalue={Application Reset:0x1000504}"]
pub type Succ2 = crate::RegValueT<Succ2_SPEC>;

impl Succ2 {
    #[doc = "Listen Timeout pdListenTimeout    LT. Configures wakeup   startup listen timeout in Microticks. The range for wakeup   startup listen timeout  pdListenTimeout  is 1284 to 1283846  504 H to 139706 H   Microticks"]
    #[inline(always)]
    pub fn lt(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffff, 1, 0, u32, Succ2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1fffff,1,0,u32, Succ2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Listen Time out Noise  gListenNoise   1     LTN. Configures the upper limit for startup and wakeup listen timeout in the        presence of noise expressed as a multiple of the cluster constant        pdListenTimeout. The range of pdListenTimeout 2 to 16. LTN must be        configured identical in all nodes of a cluster. This bit can be updated        in  quot DEFAULT CONFIG quot  or  quot CONFIG quot  state only."]
    #[inline(always)]
    pub fn ltn(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, succ2::Ltn, Succ2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xf,1,0,succ2::Ltn, Succ2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Succ2 {
    #[inline(always)]
    fn default() -> Succ2 {
        <crate::RegValueT<Succ2_SPEC> as RegisterValue<_>>::new(16778500)
    }
}
pub mod succ2 {
    pub struct Ltn_SPEC;
    pub type Ltn = crate::EnumBitfieldStruct<u8, Ltn_SPEC>;
    impl Ltn {
        #[doc = "1 Listen Time out        Noise is equal 2"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Listen Time out        Noise is equal 3"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3   8230  E H Listen Time out Noise is equal 3   8230  15"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "F Listen Time out Noise is equal 16"]
        pub const CONST_1515: Self = Self::new(15);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Succ3_SPEC;
impl crate::sealed::RegSpec for Succ3_SPEC {
    type DataType = u32;
}
#[doc = "SUC Configuration Register 3\n resetvalue={Application Reset:0x11}"]
pub type Succ3 = crate::RegValueT<Succ3_SPEC>;

impl Succ3 {
    #[doc = "Maximum Without Clock Correction Passive gMaxWithoutClockCorrectionPassive    WCP. Defines the number of consecutive even   odd cycle pairs with missing        clock correction terms that will cause a transition from   8220 NORMAL ACTIVE  8221         to   8220 NORMAL PASSIVE  8221  state. Must be identical in all nodes of a cluster.        Valid values are 1 to 15  1 H to F H          cycle pairs."]
    #[inline(always)]
    pub fn wcp(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Succ3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Succ3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Maximum Without Clock Correction Fatal  gMaxWithoutClockCorrecti on Fatal    WCF. Defines the number of consecutive even   odd cycle pairs with missing        clock correction terms that will cause a transition from   8220 NORMAL ACTIVE  8221         or   8220 NORMAL PASSIVE  8221  to   8220 HALT  8221  state. Must be identical in all nodes of a        cluster. Valid values are 1 to 15  1 H to F H  cycle pairs."]
    #[inline(always)]
    pub fn wcf(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Succ3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Succ3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Succ3 {
    #[inline(always)]
    fn default() -> Succ3 {
        <crate::RegValueT<Succ3_SPEC> as RegisterValue<_>>::new(17)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swnit_SPEC;
impl crate::sealed::RegSpec for Swnit_SPEC {
    type DataType = u32;
}
#[doc = "Symbol Window and Network Idle Time Status\n resetvalue={Application Reset:0x0}"]
pub type Swnit = crate::RegValueT<Swnit_SPEC>;

impl Swnit {
    #[doc = "Syntax Error in Symbol Window Channel A vSS SyntaxErrorA    SESA"]
    #[inline(always)]
    pub fn sesa(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, swnit::Sesa, Swnit_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,swnit::Sesa, Swnit_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Slot Boundary Violation in Symbol Window Channel A vSS BViolationA    SBSA"]
    #[inline(always)]
    pub fn sbsa(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, swnit::Sbsa, Swnit_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,swnit::Sbsa, Swnit_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmission Conflict in Symbol Window Channel A vSS TxConflictA    TCSA"]
    #[inline(always)]
    pub fn tcsa(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, swnit::Tcsa, Swnit_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,swnit::Tcsa, Swnit_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Syntax Error in Symbol Window Channel B vSS SyntaxErrorB    SESB"]
    #[inline(always)]
    pub fn sesb(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, swnit::Sesb, Swnit_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1,1,0,swnit::Sesb, Swnit_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Slot Boundary Violation in Symbol Window Channel B vSS BViolationB    SBSB"]
    #[inline(always)]
    pub fn sbsb(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, swnit::Sbsb, Swnit_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x1,1,0,swnit::Sbsb, Swnit_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmission Conflict in Symbol Window Channel B vSS TxConflictB    TCSB"]
    #[inline(always)]
    pub fn tcsb(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, swnit::Tcsb, Swnit_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x1,1,0,swnit::Tcsb, Swnit_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "MTS Received on Channel A vSS ValidMTSA    MTSA. Media Access Test symbol received on channel A during the proceeding        symbol window. Updated by the Communication Controller for each channel        at the end of the symbol window. When this bit is set to 1  also        interrupt flag SIR.MTSA is set to 1."]
    #[inline(always)]
    pub fn mtsa(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, swnit::Mtsa, Swnit_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x1,1,0,swnit::Mtsa, Swnit_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "MTS Received on Channel B vSS ValidMTSB    MTSB. Media Access Test symbol received on channel B during the proceeding        symbol window. Updated by the Communication Controller for each channel        at the end of the symbol window. When this bit is set to 1  also        interrupt flag SIR.MTSB is set to 1."]
    #[inline(always)]
    pub fn mtsb(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, swnit::Mtsb, Swnit_SPEC, crate::common::R> {
        crate::common::RegisterField::<7,0x1,1,0,swnit::Mtsb, Swnit_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Syntax Error during network idle time  NIT  Channel A vSS SyntaxErrorA    SENA. Updated by the Communication Controller channel A at the end of the NIT."]
    #[inline(always)]
    pub fn sena(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, swnit::Sena, Swnit_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x1,1,0,swnit::Sena, Swnit_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Slot Boundary Violation during network idle time  NIT  Channel A vSS BViolationA    SBNA. Updated by the Communication Controller channel A at the end of the NIT."]
    #[inline(always)]
    pub fn sbna(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, swnit::Sbna, Swnit_SPEC, crate::common::R> {
        crate::common::RegisterField::<9,0x1,1,0,swnit::Sbna, Swnit_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Syntax Error during network idle time  NIT  Channel B vSS SyntaxErrorB    SENB. Updated by the Communication Controller channel B at the end of the NIT."]
    #[inline(always)]
    pub fn senb(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, swnit::Senb, Swnit_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<10,0x1,1,0,swnit::Senb, Swnit_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Slot Boundary Violation during network idle time  NIT  Channel B vSS BViolationB    SBNB. Updated by the Communication Controller channel B at the end of the NIT."]
    #[inline(always)]
    pub fn sbnb(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, swnit::Sbnb, Swnit_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<11,0x1,1,0,swnit::Sbnb, Swnit_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Swnit {
    #[inline(always)]
    fn default() -> Swnit {
        <crate::RegValueT<Swnit_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod swnit {
    pub struct Sesa_SPEC;
    pub type Sesa = crate::EnumBitfieldStruct<u8, Sesa_SPEC>;
    impl Sesa {
        #[doc = "0 No syntax error detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Syntax error during symbol window detected on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sbsa_SPEC;
    pub type Sbsa = crate::EnumBitfieldStruct<u8, Sbsa_SPEC>;
    impl Sbsa {
        #[doc = "0 No slot boundary violation detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Slot boundary violation during symbol window detected on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tcsa_SPEC;
    pub type Tcsa = crate::EnumBitfieldStruct<u8, Tcsa_SPEC>;
    impl Tcsa {
        #[doc = "0 No transmission conflict detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transmission conflict in symbol window detected on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sesb_SPEC;
    pub type Sesb = crate::EnumBitfieldStruct<u8, Sesb_SPEC>;
    impl Sesb {
        #[doc = "0 No syntax error detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Syntax error during symbol window detected on channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sbsb_SPEC;
    pub type Sbsb = crate::EnumBitfieldStruct<u8, Sbsb_SPEC>;
    impl Sbsb {
        #[doc = "0 No slot boundary violation detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Slot boundary violation during symbol window detected on channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tcsb_SPEC;
    pub type Tcsb = crate::EnumBitfieldStruct<u8, Tcsb_SPEC>;
    impl Tcsb {
        #[doc = "0 No transmission conflict detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transmission conflict in symbol window detected on channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mtsa_SPEC;
    pub type Mtsa = crate::EnumBitfieldStruct<u8, Mtsa_SPEC>;
    impl Mtsa {
        #[doc = "0 No MTS symbol received on channel A"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MTS symbol received on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mtsb_SPEC;
    pub type Mtsb = crate::EnumBitfieldStruct<u8, Mtsb_SPEC>;
    impl Mtsb {
        #[doc = "0 No MTS symbol        received on channel B"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MTS symbol received on channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sena_SPEC;
    pub type Sena = crate::EnumBitfieldStruct<u8, Sena_SPEC>;
    impl Sena {
        #[doc = "0 No syntax error detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Syntax error during network idle time  NIT  detected on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sbna_SPEC;
    pub type Sbna = crate::EnumBitfieldStruct<u8, Sbna_SPEC>;
    impl Sbna {
        #[doc = "0 No slot boundary violation detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Slot boundary violation during network idle time  NIT  detected on channel A"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Senb_SPEC;
    pub type Senb = crate::EnumBitfieldStruct<u8, Senb_SPEC>;
    impl Senb {
        #[doc = "0 No syntax error detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Syntax error during network idle time  NIT  detected on channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sbnb_SPEC;
    pub type Sbnb = crate::EnumBitfieldStruct<u8, Sbnb_SPEC>;
    impl Sbnb {
        #[doc = "0 No slot boundary violation detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Slot boundary violation during network idle time  NIT  detected on channel B"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T0C_SPEC;
impl crate::sealed::RegSpec for T0C_SPEC {
    type DataType = u32;
}
#[doc = "Timer 0 Configuration\n resetvalue={Application Reset:0x0}"]
pub type T0C = crate::RegValueT<T0C_SPEC>;

impl T0C {
    #[doc = "Timer 0 Run Control   T0RC"]
    #[inline(always)]
    pub fn t0rc(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, t0c::T0Rc, T0C_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,t0c::T0Rc, T0C_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer 0 Mode Select   T0MS"]
    #[inline(always)]
    pub fn t0ms(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, t0c::T0Ms, T0C_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,t0c::T0Ms, T0C_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer 0 Cycle Code   T0CC. The 7 bit timer 0 cycle code determines the cycle set used for        generation of the timer 0 service request. For details about the        configuration of the cycle code see   8220 Cycle Counter Filtering  8221 ."]
    #[inline(always)]
    pub fn t0cc(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, T0C_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7f,1,0,u8, T0C_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer 0 Macrotick Offset   T0MO. Configures the Macrotick offset from the beginning of the cycle where the service request is to occur. The Timer 0 Service Request occurs at this offset for each cycle of the cycle set."]
    #[inline(always)]
    pub fn t0mo(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, T0C_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3fff,1,0,u16, T0C_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T0C {
    #[inline(always)]
    fn default() -> T0C {
        <crate::RegValueT<T0C_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod t0c {
    pub struct T0Rc_SPEC;
    pub type T0Rc = crate::EnumBitfieldStruct<u8, T0Rc_SPEC>;
    impl T0Rc {
        #[doc = "0 Timer 0 halted"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer 0 running"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T0Ms_SPEC;
    pub type T0Ms = crate::EnumBitfieldStruct<u8, T0Ms_SPEC>;
    impl T0Ms {
        #[doc = "0 Single shot mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Continuous mode"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T1C_SPEC;
impl crate::sealed::RegSpec for T1C_SPEC {
    type DataType = u32;
}
#[doc = "Timer 1 Configuration\n resetvalue={Application Reset:0x20000}"]
pub type T1C = crate::RegValueT<T1C_SPEC>;

impl T1C {
    #[doc = "Timer 1 Run Control   T1RC"]
    #[inline(always)]
    pub fn t1rc(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, t1c::T1Rc, T1C_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,t1c::T1Rc, T1C_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer 1 Mode Select   T1MS"]
    #[inline(always)]
    pub fn t1ms(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, t1c::T1Ms, T1C_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,t1c::T1Ms, T1C_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer 1 Macrotick Count   T1MC. When the configured Macrotick count is reached the timer 1 service request is generated. Valid values are  2 H  3FFF H Macroticks in continuous mode 1 H  3FFF H Macroticks in single shot mode"]
    #[inline(always)]
    pub fn t1mc(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, T1C_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3fff,1,0,u16, T1C_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T1C {
    #[inline(always)]
    fn default() -> T1C {
        <crate::RegValueT<T1C_SPEC> as RegisterValue<_>>::new(131072)
    }
}
pub mod t1c {
    pub struct T1Rc_SPEC;
    pub type T1Rc = crate::EnumBitfieldStruct<u8, T1Rc_SPEC>;
    impl T1Rc {
        #[doc = "0 Timer 1 halted"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer 1 running"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T1Ms_SPEC;
    pub type T1Ms = crate::EnumBitfieldStruct<u8, T1Ms_SPEC>;
    impl T1Ms {
        #[doc = "0 Single shot mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Continuous mode"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Test1_SPEC;
impl crate::sealed::RegSpec for Test1_SPEC {
    type DataType = u32;
}
#[doc = "Test Register 1\n resetvalue={Application Reset:0x300}"]
pub type Test1 = crate::RegValueT<Test1_SPEC>;

impl Test1 {
    #[doc = "Write Test Register Enable   WRTEN. Enables write access to the test registers. To set the bit from 0 to 1        the test mode key has to be written as defined on   8220 Lock Register  8221 . The        unlock sequence is not required when TEST1.WRTEN is kept at 1 while        other bits of the register are changed. The bit can be reset to 0 at any        time."]
    #[inline(always)]
    pub fn wrten(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, test1::Wrten, Test1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,test1::Wrten, Test1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "External Loop Back Enable   ELBE. There are two possibilities to perform a loop back test. External loop back via physical layer or internal loop back for in system self test  default . In case of an internal loop back pins TXENA and TXENB are in their inactive state  pins TXDA and TXDB are set to HIGH  pins RXDA and RXDB are not evaluated. Bit ELBE is evaluated only when POC is in loop back mode and test multiplexer control is in non multiplexed mode TMC   00."]
    #[inline(always)]
    pub fn elbe(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, test1::Elbe, Test1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,test1::Elbe, Test1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Test Multiplexer Control   TMC"]
    #[inline(always)]
    pub fn tmc(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, test1::Tmc, Test1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,test1::Tmc, Test1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Activity on A   AOA. The channel idle condition is specified in the FlexRay  protocol spec v2.1  chapter 3  BITSTRB process  zChannelIdle ."]
    #[inline(always)]
    pub fn aoa(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, test1::Aoa, Test1_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x1,1,0,test1::Aoa, Test1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Activity on B   AOB. The channel idle condition is specified in the FlexRay  8482  protocol spec        v2.1  chapter 3  BITSTRB process  zChannelIdle ."]
    #[inline(always)]
    pub fn aob(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, test1::Aob, Test1_SPEC, crate::common::R> {
        crate::common::RegisterField::<9,0x1,1,0,test1::Aob, Test1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Read Channel A Receive Pin   RXA. This bit field shows the current logic state of RXDA."]
    #[inline(always)]
    pub fn rxa(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, test1::Rxa, Test1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x1,1,0,test1::Rxa, Test1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Read Channel B Receive Pin   RXB. This bit field shows the current logic state of RXDB."]
    #[inline(always)]
    pub fn rxb(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, test1::Rxb, Test1_SPEC, crate::common::R> {
        crate::common::RegisterField::<17,0x1,1,0,test1::Rxb, Test1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Read or Write to Channel A Transmit Pin   TXA. A write to this bit field sets the TXDA to corresponding logic state. A        read from this bit field shows the current logic state of TXDA."]
    #[inline(always)]
    pub fn txa(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, test1::Txa, Test1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,test1::Txa, Test1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Read or Write to Channel B Transmit Pin   TXB. A write to this bit field sets the TXDB to corresponding logic state. A        read from this bit field shows the current logic state of TXDB."]
    #[inline(always)]
    pub fn txb(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, test1::Txb, Test1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,test1::Txb, Test1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Read or Write to Channel A Transmit Enable Pin   TXENA. A write to this bit field sets the TXENA to corresponding logic state. A        read from this bit field shows the current logic state of TXENA."]
    #[inline(always)]
    pub fn txena(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, test1::Txena, Test1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,test1::Txena, Test1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Read or Write to Channel B Transmit Enable Pin   TXENB. A write to this bit field sets the TXENB to corresponding logic state. A        read from this bit field shows the current logic state of TXENB."]
    #[inline(always)]
    pub fn txenb(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, test1::Txenb, Test1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,test1::Txenb, Test1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Test1 {
    #[inline(always)]
    fn default() -> Test1 {
        <crate::RegValueT<Test1_SPEC> as RegisterValue<_>>::new(768)
    }
}
pub mod test1 {
    pub struct Wrten_SPEC;
    pub type Wrten = crate::EnumBitfieldStruct<u8, Wrten_SPEC>;
    impl Wrten {
        #[doc = "0 Write access to test registers disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access to test registers enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Elbe_SPEC;
    pub type Elbe = crate::EnumBitfieldStruct<u8, Elbe_SPEC>;
    impl Elbe {
        #[doc = "0 Internal loop back  default"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 External loop back"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tmc_SPEC;
    pub type Tmc = crate::EnumBitfieldStruct<u8, Tmc_SPEC>;
    impl Tmc {
        #[doc = "00 Normal signal path  default ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 RAM Test Mode  Internal busses are multiplexed to make all RAM blocks of the E Ray module directly accessible by the Host. This mode is intended to enable testing of the embedded RAM blocks during production testing."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 I O Test Mode  Output pins eray txd1  eray txd2  eray txen1  eray txen2 are driven to the values defined by bits TXA  TXB  TXENA   TXENB . The values applied to the input pins eray rxd1  eray rxd2 can be read from register bits RXA and RXB."]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Aoa_SPEC;
    pub type Aoa = crate::EnumBitfieldStruct<u8, Aoa_SPEC>;
    impl Aoa {
        #[doc = "0 No activity detected  channel A idle"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Activity detected  channel A not idle"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Aob_SPEC;
    pub type Aob = crate::EnumBitfieldStruct<u8, Aob_SPEC>;
    impl Aob {
        #[doc = "0 No activity detected  channel B idle"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Activity detected  channel B not idle"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rxa_SPEC;
    pub type Rxa = crate::EnumBitfieldStruct<u8, Rxa_SPEC>;
    impl Rxa {
        #[doc = "0  eray rxda  RXDA   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1  eray rxda  RXDA   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rxb_SPEC;
    pub type Rxb = crate::EnumBitfieldStruct<u8, Rxb_SPEC>;
    impl Rxb {
        #[doc = "0  eray rxdb  RXDB   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1  eray rxdb  RXDB   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Txa_SPEC;
    pub type Txa = crate::EnumBitfieldStruct<u8, Txa_SPEC>;
    impl Txa {
        #[doc = "0  eray txda  TXDA   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1  eray txda  TXDA   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Txb_SPEC;
    pub type Txb = crate::EnumBitfieldStruct<u8, Txb_SPEC>;
    impl Txb {
        #[doc = "0  eray txdb  TXDB   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1  eray txdb  TXDB   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Txena_SPEC;
    pub type Txena = crate::EnumBitfieldStruct<u8, Txena_SPEC>;
    impl Txena {
        #[doc = "0  eray txena  TXENA   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1  eray txena  TXENA   1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Txenb_SPEC;
    pub type Txenb = crate::EnumBitfieldStruct<u8, Txenb_SPEC>;
    impl Txenb {
        #[doc = "0  eray txenb  TXENB   0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1  eray txenb  TXENB   1"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Test2_SPEC;
impl crate::sealed::RegSpec for Test2_SPEC {
    type DataType = u32;
}
#[doc = "Test Register 2\n resetvalue={Application Reset:0x0}"]
pub type Test2 = crate::RegValueT<Test2_SPEC>;

impl Test2 {
    #[doc = "RAM Select   RS. In RAM Test mode the RAM blocks selected by RS are mapped to module        address 0000  160 0400 H to 0000  160 07FF H  1024 byte addresses ."]
    #[inline(always)]
    pub fn rs(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, test2::Rs, Test2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,test2::Rs, Test2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Segment Select   SSEL. To enable access to the complete Message RAM  8192 byte addresses  the        Message RAM is segmented."]
    #[inline(always)]
    pub fn ssel(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, test2::Ssel, Test2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7,1,0,test2::Ssel, Test2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Write ECC Data Enable   WRECC. Content of ECCW is transferred to the RAM  Test mode must be entered. See   8220 Test Register 1  8221"]
    #[inline(always)]
    pub fn wrecc(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, test2::Wrecc, Test2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,test2::Wrecc, Test2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Test2 {
    #[inline(always)]
    fn default() -> Test2 {
        <crate::RegValueT<Test2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod test2 {
    pub struct Rs_SPEC;
    pub type Rs = crate::EnumBitfieldStruct<u8, Rs_SPEC>;
    impl Rs {
        #[doc = "000 Input Buffer RAM 1  IBF1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 Input Buffer RAM 2  IBF2"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 Output Buffer RAM 1  OBF1"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 Output Buffer RAM 2  OBF2"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 Transient Buffer RAM A  TBF1"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 Transient Buffer RAM B  TBF2"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 Message RAM  MBF"]
        pub const CONST_66: Self = Self::new(6);
    }
    pub struct Ssel_SPEC;
    pub type Ssel = crate::EnumBitfieldStruct<u8, Ssel_SPEC>;
    impl Ssel {
        #[doc = "000 access to RAM byte 0000 H to 03FF H enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 access to RAM byte 0400 H to 07FF H enabled"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 access to RAM byte 0800 H to 0BFF H enabled"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 access to RAM byte 0C00 H to 0FFF H enabled"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 access to RAM        byte 1000 H to 13FF H enabled"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 access to RAM        byte 1400 H to 17FF H enabled"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 access to RAM byte 1800 H to 1BFF H enabled"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "111 access to RAM byte 1C00 H to 1FFF H enabled"]
        pub const CONST_77: Self = Self::new(7);
    }
    pub struct Wrecc_SPEC;
    pub type Wrecc = crate::EnumBitfieldStruct<u8, Wrecc_SPEC>;
    impl Wrecc {
        #[doc = "0 disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txrq1_SPEC;
impl crate::sealed::RegSpec for Txrq1_SPEC {
    type DataType = u32;
}
#[doc = "Transmission Request Register 1\n resetvalue={Application Reset:0x0}"]
pub type Txrq1 = crate::RegValueT<Txrq1_SPEC>;

impl Txrq1 {
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr12(self) -> crate::common::RegisterFieldBool<12, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr13(self) -> crate::common::RegisterFieldBool<13, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr14(self) -> crate::common::RegisterFieldBool<14, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr15(self) -> crate::common::RegisterFieldBool<15, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr16(self) -> crate::common::RegisterFieldBool<16, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr17(self) -> crate::common::RegisterFieldBool<17, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr18(self) -> crate::common::RegisterFieldBool<18, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr19(self) -> crate::common::RegisterFieldBool<19, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr20(self) -> crate::common::RegisterFieldBool<20, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr21(self) -> crate::common::RegisterFieldBool<21, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr22(self) -> crate::common::RegisterFieldBool<22, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr23(self) -> crate::common::RegisterFieldBool<23, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr24(self) -> crate::common::RegisterFieldBool<24, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr25(self) -> crate::common::RegisterFieldBool<25, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr26(self) -> crate::common::RegisterFieldBool<26, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr27(self) -> crate::common::RegisterFieldBool<27, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr28(self) -> crate::common::RegisterFieldBool<28, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr29(self) -> crate::common::RegisterFieldBool<29, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr30(self) -> crate::common::RegisterFieldBool<30, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 31  n   0 31    TXR31. If the flag is set  the respective Message Buffer 0 to 31 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr31(self) -> crate::common::RegisterFieldBool<31, 1, 0, Txrq1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Txrq1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Txrq1 {
    #[inline(always)]
    fn default() -> Txrq1 {
        <crate::RegValueT<Txrq1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txrq2_SPEC;
impl crate::sealed::RegSpec for Txrq2_SPEC {
    type DataType = u32;
}
#[doc = "Transmission Request Register 2\n resetvalue={Application Reset:0x0}"]
pub type Txrq2 = crate::RegValueT<Txrq2_SPEC>;

impl Txrq2 {
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr32(self) -> crate::common::RegisterFieldBool<0, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr33(self) -> crate::common::RegisterFieldBool<1, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr34(self) -> crate::common::RegisterFieldBool<2, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr35(self) -> crate::common::RegisterFieldBool<3, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr36(self) -> crate::common::RegisterFieldBool<4, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr37(self) -> crate::common::RegisterFieldBool<5, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr38(self) -> crate::common::RegisterFieldBool<6, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr39(self) -> crate::common::RegisterFieldBool<7, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr40(self) -> crate::common::RegisterFieldBool<8, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr41(self) -> crate::common::RegisterFieldBool<9, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr42(self) -> crate::common::RegisterFieldBool<10, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr43(self) -> crate::common::RegisterFieldBool<11, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr44(self) -> crate::common::RegisterFieldBool<12, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr45(self) -> crate::common::RegisterFieldBool<13, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr46(self) -> crate::common::RegisterFieldBool<14, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr47(self) -> crate::common::RegisterFieldBool<15, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr48(self) -> crate::common::RegisterFieldBool<16, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr49(self) -> crate::common::RegisterFieldBool<17, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr50(self) -> crate::common::RegisterFieldBool<18, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr51(self) -> crate::common::RegisterFieldBool<19, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr52(self) -> crate::common::RegisterFieldBool<20, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr53(self) -> crate::common::RegisterFieldBool<21, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr54(self) -> crate::common::RegisterFieldBool<22, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr55(self) -> crate::common::RegisterFieldBool<23, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr56(self) -> crate::common::RegisterFieldBool<24, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr57(self) -> crate::common::RegisterFieldBool<25, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr58(self) -> crate::common::RegisterFieldBool<26, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr59(self) -> crate::common::RegisterFieldBool<27, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr60(self) -> crate::common::RegisterFieldBool<28, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr61(self) -> crate::common::RegisterFieldBool<29, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr62(self) -> crate::common::RegisterFieldBool<30, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 63  n   32 63    TXR63. If the flag is set  the respective Message Buffer 32 to 63 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr63(self) -> crate::common::RegisterFieldBool<31, 1, 0, Txrq2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Txrq2_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Txrq2 {
    #[inline(always)]
    fn default() -> Txrq2 {
        <crate::RegValueT<Txrq2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txrq3_SPEC;
impl crate::sealed::RegSpec for Txrq3_SPEC {
    type DataType = u32;
}
#[doc = "Transmission Request Register 3\n resetvalue={Application Reset:0x0}"]
pub type Txrq3 = crate::RegValueT<Txrq3_SPEC>;

impl Txrq3 {
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr64(self) -> crate::common::RegisterFieldBool<0, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr65(self) -> crate::common::RegisterFieldBool<1, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr66(self) -> crate::common::RegisterFieldBool<2, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr67(self) -> crate::common::RegisterFieldBool<3, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr68(self) -> crate::common::RegisterFieldBool<4, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr69(self) -> crate::common::RegisterFieldBool<5, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr70(self) -> crate::common::RegisterFieldBool<6, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr71(self) -> crate::common::RegisterFieldBool<7, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr72(self) -> crate::common::RegisterFieldBool<8, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr73(self) -> crate::common::RegisterFieldBool<9, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr74(self) -> crate::common::RegisterFieldBool<10, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr75(self) -> crate::common::RegisterFieldBool<11, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr76(self) -> crate::common::RegisterFieldBool<12, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr77(self) -> crate::common::RegisterFieldBool<13, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr78(self) -> crate::common::RegisterFieldBool<14, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr79(self) -> crate::common::RegisterFieldBool<15, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr80(self) -> crate::common::RegisterFieldBool<16, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr81(self) -> crate::common::RegisterFieldBool<17, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr82(self) -> crate::common::RegisterFieldBool<18, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr83(self) -> crate::common::RegisterFieldBool<19, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr84(self) -> crate::common::RegisterFieldBool<20, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr85(self) -> crate::common::RegisterFieldBool<21, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr86(self) -> crate::common::RegisterFieldBool<22, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr87(self) -> crate::common::RegisterFieldBool<23, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr88(self) -> crate::common::RegisterFieldBool<24, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr89(self) -> crate::common::RegisterFieldBool<25, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr90(self) -> crate::common::RegisterFieldBool<26, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr91(self) -> crate::common::RegisterFieldBool<27, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr92(self) -> crate::common::RegisterFieldBool<28, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr93(self) -> crate::common::RegisterFieldBool<29, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr94(self) -> crate::common::RegisterFieldBool<30, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 95  n   64 95    TXR95. If the flag is set  the respective Message Buffer 64 to 95 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr95(self) -> crate::common::RegisterFieldBool<31, 1, 0, Txrq3_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Txrq3_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Txrq3 {
    #[inline(always)]
    fn default() -> Txrq3 {
        <crate::RegValueT<Txrq3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txrq4_SPEC;
impl crate::sealed::RegSpec for Txrq4_SPEC {
    type DataType = u32;
}
#[doc = "Transmission Request Register 4\n resetvalue={Application Reset:0x0}"]
pub type Txrq4 = crate::RegValueT<Txrq4_SPEC>;

impl Txrq4 {
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr96(self) -> crate::common::RegisterFieldBool<0, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr97(self) -> crate::common::RegisterFieldBool<1, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr98(self) -> crate::common::RegisterFieldBool<2, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr99(self) -> crate::common::RegisterFieldBool<3, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr100(self) -> crate::common::RegisterFieldBool<4, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr101(self) -> crate::common::RegisterFieldBool<5, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr102(self) -> crate::common::RegisterFieldBool<6, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr103(self) -> crate::common::RegisterFieldBool<7, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr104(self) -> crate::common::RegisterFieldBool<8, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr105(self) -> crate::common::RegisterFieldBool<9, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr106(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr107(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr108(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr109(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr110(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr111(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr112(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr113(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr114(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr115(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr116(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr117(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr118(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr119(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr120(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr121(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr122(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr123(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr124(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr125(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr126(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Transmission Request 127  n   96 127    TXR127. If the flag is set  the respective Message Buffer 96 to 127 is ready for transmission respectively transmission of this Message Buffer is in progress. In single shot mode the flags are reset after transmission has completed."]
    #[inline(always)]
    pub fn txr127(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Txrq4_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Txrq4_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Txrq4 {
    #[inline(always)]
    fn default() -> Txrq4 {
        <crate::RegValueT<Txrq4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WrdSn_SPEC;
impl crate::sealed::RegSpec for WrdSn_SPEC {
    type DataType = u32;
}
#[doc = "Write Data Section 01\n resetvalue={Application Reset:0x0}"]
pub type WrdSn = crate::RegValueT<WrdSn_SPEC>;

impl WrdSn {
    #[doc = "32 Bit Word nn  Byte 0   MDWB0"]
    #[inline(always)]
    pub fn mdwb0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, WrdSn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, WrdSn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "32 Bit Word nn  Byte 1   MDWB1"]
    #[inline(always)]
    pub fn mdwb1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, WrdSn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, WrdSn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "32 Bit Word nn  Byte 2   MDWB2"]
    #[inline(always)]
    pub fn mdwb2(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, WrdSn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, WrdSn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "32 Bit Word nn  Byte 3   MDWB3"]
    #[inline(always)]
    pub fn mdwb3(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, WrdSn_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, WrdSn_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for WrdSn {
    #[inline(always)]
    fn default() -> WrdSn {
        <crate::RegValueT<WrdSn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wrhs1_SPEC;
impl crate::sealed::RegSpec for Wrhs1_SPEC {
    type DataType = u32;
}
#[doc = "Write Header Section 1\n resetvalue={Application Reset:0x0}"]
pub type Wrhs1 = crate::RegValueT<Wrhs1_SPEC>;

impl Wrhs1 {
    #[doc = "Frame ID   FID. Frame ID of the selected Message Buffer. The Frame ID defines the slot        number for transmission   reception of the respective message. Message        Buffers with Frame ID   0 are considered as not valid."]
    #[inline(always)]
    pub fn fid(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Wrhs1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Wrhs1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Cycle Code   CYC. The 7 bit cycle code determines the cycle set used for cycle counter        filtering. For details about the configuration of the cycle code see        Section  160  quot Cycle Counter Filtering quot ."]
    #[inline(always)]
    pub fn cyc(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Wrhs1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Wrhs1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel Filter Control A   CHA. The channel filtering field A associated with the buffer serves of        channel A as a filter for receive buffers  and as a control field for        transmit buffers"]
    #[inline(always)]
    pub fn cha(self) -> crate::common::RegisterFieldBool<24, 1, 0, Wrhs1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Wrhs1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel Filter Control B   CHB. The channel filtering field B associated with the buffer serves of        channel B as a filter for receive buffers  and as a control field for        transmit buffers"]
    #[inline(always)]
    pub fn chb(self) -> crate::common::RegisterFieldBool<25, 1, 0, Wrhs1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Wrhs1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Message Buffer Direction Configuration Bit   CFG. This bit is used to configure the corresponding buffer as a transmit        buffer or as a receive buffer. For Message Buffers belonging to the        receive FIFO the bit is not evaluated."]
    #[inline(always)]
    pub fn cfg(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, wrhs1::Cfg, Wrhs1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,wrhs1::Cfg, Wrhs1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Payload Preamble Indicator Transmit   PPIT. This bit is used to control the state of the Payload Preamble Indicator        in transmit Frames. If the bit is set in a static Message Buffer  the        respective Message Buffer holds Network Management information. If the        bit is set in a dynamic Message Buffer the first two byte of the Payload        Segment may be used for message ID filtering by the receiver. Message ID        filtering of received FlexRay  8482  Frames is not supported by the E Ray module  but can be done by the Host."]
    #[inline(always)]
    pub fn ppit(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, wrhs1::Ppit, Wrhs1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,wrhs1::Ppit, Wrhs1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmission Mode   TXM. This bit is used to select the transmission mode  see   8220 Transmit        Buffers  8221  ."]
    #[inline(always)]
    pub fn txm(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, wrhs1::Txm, Wrhs1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,wrhs1::Txm, Wrhs1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Message Buffer Service Request   MBI. This bit enables the receive   transmit service request for the        corresponding Message Buffer. After a dedicated receive buffer has been        updated by the Message Handler  flag SIR.RXI and  or SIR.MBSI in the        Status Service Request register are set. After a transmission has        completed flag SIR.TXI is set."]
    #[inline(always)]
    pub fn mbi(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, wrhs1::Mbi, Wrhs1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,wrhs1::Mbi, Wrhs1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Wrhs1 {
    #[inline(always)]
    fn default() -> Wrhs1 {
        <crate::RegValueT<Wrhs1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod wrhs1 {
    pub struct Cfg_SPEC;
    pub type Cfg = crate::EnumBitfieldStruct<u8, Cfg_SPEC>;
    impl Cfg {
        #[doc = "0 The corresponding buffer is configured as Receive Buffer"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The corresponding buffer is configured as Transmit Buffer"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ppit_SPEC;
    pub type Ppit = crate::EnumBitfieldStruct<u8, Ppit_SPEC>;
    impl Ppit {
        #[doc = "0 Payload Preamble Indicator not set"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Payload Preamble Indicator set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Txm_SPEC;
    pub type Txm = crate::EnumBitfieldStruct<u8, Txm_SPEC>;
    impl Txm {
        #[doc = "0 Continuous mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Single shot mode"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mbi_SPEC;
    pub type Mbi = crate::EnumBitfieldStruct<u8, Mbi_SPEC>;
    impl Mbi {
        #[doc = "0 The corresponding Message Buffer service request is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The corresponding Message Buffer service request is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wrhs2_SPEC;
impl crate::sealed::RegSpec for Wrhs2_SPEC {
    type DataType = u32;
}
#[doc = "Write Header Section 2\n resetvalue={Application Reset:0x0}"]
pub type Wrhs2 = crate::RegValueT<Wrhs2_SPEC>;

impl Wrhs2 {
    #[doc = "Header CRC vRF Header HeaderCRC    CRC. Receive Buffer  Configuration not required Transmit Buffer  Header CRC calculated and configured by the Host. For calculation of the Header CRC the payload length of the Frame send on the bus has to be considered. In static segment the payload length of all Frames is configured by MHDC.SFDL."]
    #[inline(always)]
    pub fn crc(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Wrhs2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Wrhs2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Payload Length Configured   PLC. Length of Data Section  number of 2 byte words  as configured by the Host. During static segment the static Frame payload length as configured by MHDC.SFDL in the MHD Configuration Register defines the payload length for all static Frames. If the payload length configured by PLC is shorter than this value padding byte are inserted to ensure that Frames have proper physical length. The padding pattern is logical zero."]
    #[inline(always)]
    pub fn plc(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Wrhs2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Wrhs2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Wrhs2 {
    #[inline(always)]
    fn default() -> Wrhs2 {
        <crate::RegValueT<Wrhs2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wrhs3_SPEC;
impl crate::sealed::RegSpec for Wrhs3_SPEC {
    type DataType = u32;
}
#[doc = "Write Header Section 3\n resetvalue={Application Reset:0x0}"]
pub type Wrhs3 = crate::RegValueT<Wrhs3_SPEC>;

impl Wrhs3 {
    #[doc = "Data Pointer   DP. Pointer to the first 32 bit word of the Data Section of the addressed Message Buffer in the Message RAM."]
    #[inline(always)]
    pub fn dp(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Wrhs3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Wrhs3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Wrhs3 {
    #[inline(always)]
    fn default() -> Wrhs3 {
        <crate::RegValueT<Wrhs3_SPEC> as RegisterValue<_>>::new(0)
    }
}
