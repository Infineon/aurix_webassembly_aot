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
#[doc = r"System Control Unit"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scu(pub(super) *mut u8);
unsafe impl core::marker::Send for Scu {}
unsafe impl core::marker::Sync for Scu {}
impl Scu {
    #[doc = "Access Enable Register 00\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen00(&self) -> crate::common::Reg<self::Accen00_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1020usize)) }
    }

    #[doc = "Access Enable Register 10\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen10(&self) -> crate::common::Reg<self::Accen10_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1012usize)) }
    }

    #[doc = "Application Reset Disable Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn arstdis(&self) -> crate::common::Reg<self::Arstdis_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(92usize)) }
    }

    #[doc = "CCU Clock Control Register 0\n resetvalue={After SSW execution:0x5120112,System Reset:0x3130113,System Reset:0x35120112}"]
    #[inline(always)]
    pub const fn ccucon0(&self) -> crate::common::Reg<self::Ccucon0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }

    #[doc = "CCU Clock Control Register 1\n resetvalue={System Reset:0x10100302}"]
    #[inline(always)]
    pub const fn ccucon1(&self) -> crate::common::Reg<self::Ccucon1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "CCU Clock Control Register 2\n resetvalue={System Reset:0x7000101}"]
    #[inline(always)]
    pub const fn ccucon2(&self) -> crate::common::Reg<self::Ccucon2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "CCU Clock Control Register 3\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn ccucon3(&self) -> crate::common::Reg<self::Ccucon3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }

    #[doc = "CCU Clock Control Register 4\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn ccucon4(&self) -> crate::common::Reg<self::Ccucon4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }

    #[doc = "CCU Clock Control Register 5\n resetvalue={System Reset:0x30,System Reset:0x30}"]
    #[inline(always)]
    pub const fn ccucon5(&self) -> crate::common::Reg<self::Ccucon5_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(76usize)) }
    }

    #[doc = "CCU Clock Control Register 6\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn ccucon6(&self) -> crate::common::Reg<self::Ccucon6_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "CCU Clock Control Register 7\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn ccucon7(&self) -> crate::common::Reg<self::Ccucon7_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }

    #[doc = "CCU Clock Control Register 8\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn ccucon8(&self) -> crate::common::Reg<self::Ccucon8_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }

    #[doc = "CCU Control Security Monitor Register\n resetvalue={System Reset:0x1F1E}"]
    #[inline(always)]
    pub const fn ccuconsm(&self) -> crate::common::Reg<self::Ccuconsm_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(400usize)) }
    }

    #[doc = "Chip Identification Register\n resetvalue={System Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn chipid(&self) -> crate::common::Reg<self::Chipid_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(320usize)) }
    }

    #[doc = "Clock Division Control Register 0\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn clkdivctrl0(
        &self,
    ) -> crate::common::Reg<self::Clkdivctrl0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(432usize)) }
    }

    #[doc = "Clock Division Control Register 1\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn clkdivctrl1(
        &self,
    ) -> crate::common::Reg<self::Clkdivctrl1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(436usize)) }
    }

    #[doc = "Core Die Temperature Sensor Bandgap Control Register\n resetvalue={Cold PowerOn Reset:0x40,CFS Value:0x40}"]
    #[inline(always)]
    pub const fn dtscbgoctrl(
        &self,
    ) -> crate::common::Reg<self::Dtscbgoctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(268usize)) }
    }

    #[doc = "Core Die Temperature Sensor Control Register\n resetvalue={Cold PowerOn Reset:0x200,CFS Value:0x200}"]
    #[inline(always)]
    pub const fn dtsccon(&self) -> crate::common::Reg<self::Dtsccon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(272usize)) }
    }

    #[doc = "Core Die Temperature Sensor Limit Register\n resetvalue={Application Reset:0x0CD806D6}"]
    #[inline(always)]
    pub const fn dtsclim(&self) -> crate::common::Reg<self::Dtsclim_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(264usize)) }
    }

    #[doc = "Core Die Temperature Sensor Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dtscstat(&self) -> crate::common::Reg<self::Dtscstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
    }

    #[doc = "ENDINIT Global Control Register 0\n resetvalue={Application Reset:0x0FFFC000E}"]
    #[inline(always)]
    pub const fn eicon0(&self) -> crate::common::Reg<self::Eicon0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(668usize)) }
    }

    #[doc = "ENDINIT Global Control Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn eicon1(&self) -> crate::common::Reg<self::Eicon1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(672usize)) }
    }

    #[doc = "External Input Channel Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn eicri(&self) -> [crate::common::Reg<self::EicRi_SPEC, crate::common::RW>; 4] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x210usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x210usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x210usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x210usize + 0xcusize)),
            ]
        }
    }

    #[doc = "External Input Filter Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn eifilt(&self) -> crate::common::Reg<self::Eifilt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(524usize)) }
    }

    #[doc = "External Input Flag Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn eifr(&self) -> crate::common::Reg<self::Eifr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(544usize)) }
    }

    #[doc = "ENDINIT Timeout Counter Status Register\n resetvalue={Application Reset:0x0FFFC0000}"]
    #[inline(always)]
    pub const fn eisr(&self) -> crate::common::Reg<self::Eisr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(676usize)) }
    }

    #[doc = "Emergency Stop Register\n resetvalue={Application Reset:0x1}"]
    #[inline(always)]
    pub const fn emsr(&self) -> crate::common::Reg<self::Emsr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(252usize)) }
    }

    #[doc = "Emergency Stop Software set and clear register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn emssw(&self) -> crate::common::Reg<self::Emssw_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize)) }
    }

    #[doc = "ESR Output Configuration Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn esrocfg(&self) -> crate::common::Reg<self::Esrocfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(120usize)) }
    }

    #[doc = "External Clock Control Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn extcon(&self) -> crate::common::Reg<self::Extcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "Fractional Divider Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn fdr(&self) -> crate::common::Reg<self::Fdr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }

    #[doc = "Flag Modification Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fmr(&self) -> crate::common::Reg<self::Fmr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(548usize)) }
    }

    #[doc = "Identification Register\n resetvalue={System Reset:0x0C4C0C1}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Flag Gating Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn igcrj(&self) -> [crate::common::Reg<self::IgcRj_SPEC, crate::common::RW>; 4] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x22cusize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x22cusize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x22cusize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x22cusize + 0xcusize)),
            ]
        }
    }

    #[doc = "ESR Input Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn r#in(&self) -> crate::common::Reg<self::In_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(172usize)) }
    }

    #[doc = "Input Output Control Register\n resetvalue={System Reset:0x0E0}"]
    #[inline(always)]
    pub const fn iocr(&self) -> crate::common::Reg<self::Iocr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }

    #[doc = "Logic BIST Control 0 Register\n resetvalue={System Reset:0x0,CFS Value:0x400}"]
    #[inline(always)]
    pub const fn lbistctrl0(&self) -> crate::common::Reg<self::Lbistctrl0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(356usize)) }
    }

    #[doc = "Logic BIST Control 1 Register\n resetvalue={System Reset:0x0,CFS Value:0x54000007}"]
    #[inline(always)]
    pub const fn lbistctrl1(&self) -> crate::common::Reg<self::Lbistctrl1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(360usize)) }
    }

    #[doc = "Logic BIST Control 2 Register\n resetvalue={System Reset:0x0,CFS Value:0x3D}"]
    #[inline(always)]
    pub const fn lbistctrl2(&self) -> crate::common::Reg<self::Lbistctrl2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(364usize)) }
    }

    #[doc = "Logic BIST Control 3 Register\n resetvalue={System Reset:0x0,CFS Value:0x0}"]
    #[inline(always)]
    pub const fn lbistctrl3(&self) -> crate::common::Reg<self::Lbistctrl3_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(368usize)) }
    }

    #[doc = "LCL CPU0 and CPU2 Control Register\n resetvalue={Cold PowerOn Reset:0x080018001,Cold PowerOn Reset:0x080018000}"]
    #[inline(always)]
    pub const fn lclcon0(&self) -> crate::common::Reg<self::Lclcon0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(308usize)) }
    }

    #[doc = "LCL CPU1 and CPU3 Control Register\n resetvalue={Cold PowerOn Reset:0x080018001,Cold PowerOn Reset:0x080018000,Cold PowerOn Reset:0x080008000}"]
    #[inline(always)]
    pub const fn lclcon1(&self) -> crate::common::Reg<self::Lclcon1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(312usize)) }
    }

    #[doc = "LCL Test Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn lcltest(&self) -> crate::common::Reg<self::Lcltest_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(316usize)) }
    }

    #[doc = "Manufacturer Identification Register\n resetvalue={System Reset:0x1820}"]
    #[inline(always)]
    pub const fn manid(&self) -> crate::common::Reg<self::Manid_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(324usize)) }
    }

    #[doc = "Modulation Trim 0 Configuration Register\n resetvalue={PowerOn Reset:0x0,After SSW execution:0x0C000000}"]
    #[inline(always)]
    pub const fn modtrimcon0(
        &self,
    ) -> crate::common::Reg<self::Modtrimcon0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(180usize)) }
    }

    #[doc = "Modulation Trim 1 Configuration Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn modtrimcon1(
        &self,
    ) -> crate::common::Reg<self::Modtrimcon1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(184usize)) }
    }

    #[doc = "Modulation Trim Status Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn modtrimstat(
        &self,
    ) -> crate::common::Reg<self::Modtrimstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(176usize)) }
    }

    #[doc = "ESR Output Modification Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn omr(&self) -> crate::common::Reg<self::Omr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(168usize)) }
    }

    #[doc = "OSC Control Register\n resetvalue={System Reset:0x258,System Reset:0x10}"]
    #[inline(always)]
    pub const fn osccon(&self) -> crate::common::Reg<self::Osccon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "ESR Output Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn out(&self) -> crate::common::Reg<self::Out_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(164usize)) }
    }

    #[doc = "Overlay Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ovccon(&self) -> crate::common::Reg<self::Ovccon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(484usize)) }
    }

    #[doc = "Overlay Enable Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ovcenable(&self) -> crate::common::Reg<self::Ovcenable_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(480usize)) }
    }

    #[doc = "Pad Disable Control Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn pdisc(&self) -> crate::common::Reg<self::Pdisc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(396usize)) }
    }

    #[doc = "ESR Pad Driver Mode Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn pdr(&self) -> crate::common::Reg<self::Pdr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(156usize)) }
    }

    #[doc = "Pattern Detection Result Register\n resetvalue={Application Reset:0x0FF}"]
    #[inline(always)]
    pub const fn pdrr(&self) -> crate::common::Reg<self::Pdrr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(552usize)) }
    }

    #[doc = "Peripheral PLL Configuration 0 Register\n resetvalue={System Reset:0x3E00}"]
    #[inline(always)]
    pub const fn perpllcon0(&self) -> crate::common::Reg<self::Perpllcon0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }

    #[doc = "Peripheral PLL Configuration 1 Register\n resetvalue={System Reset:0x1}"]
    #[inline(always)]
    pub const fn perpllcon1(&self) -> crate::common::Reg<self::Perpllcon1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }

    #[doc = "Peripheral PLL Status Register\n resetvalue={System Reset:0x2}"]
    #[inline(always)]
    pub const fn perpllstat(&self) -> crate::common::Reg<self::Perpllstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "Pad Heating Control Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn pheatctrl(&self) -> crate::common::Reg<self::Pheatctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(372usize)) }
    }

    #[doc = "PLLTrim Register\n resetvalue={PowerOn Reset:0x080008000,CFS Value:0x080008000}"]
    #[inline(always)]
    pub const fn plltrim(&self) -> crate::common::Reg<self::Plltrim_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(520usize)) }
    }

    #[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
    #[inline(always)]
    pub const fn pmcsr0(&self) -> crate::common::Reg<self::Pmcsr0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(200usize)) }
    }

    #[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
    #[inline(always)]
    pub const fn pmcsr1(&self) -> crate::common::Reg<self::Pmcsr1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(204usize)) }
    }

    #[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
    #[inline(always)]
    pub const fn pmcsr2(&self) -> crate::common::Reg<self::Pmcsr2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(208usize)) }
    }

    #[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
    #[inline(always)]
    pub const fn pmcsr3(&self) -> crate::common::Reg<self::Pmcsr3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(212usize)) }
    }

    #[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
    #[inline(always)]
    pub const fn pmcsr4(&self) -> crate::common::Reg<self::Pmcsr4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(216usize)) }
    }

    #[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
    #[inline(always)]
    pub const fn pmcsr5(&self) -> crate::common::Reg<self::Pmcsr5_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(220usize)) }
    }

    #[doc = "Power Management Status Register 0\n resetvalue={Application Reset:0x1}"]
    #[inline(always)]
    pub const fn pmstat0(&self) -> crate::common::Reg<self::Pmstat0_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(228usize)) }
    }

    #[doc = "Standby and Wake up Control Register 1\n resetvalue={Cold PowerOn Reset:0x1000000}"]
    #[inline(always)]
    pub const fn pmswcr1(&self) -> crate::common::Reg<self::Pmswcr1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(232usize)) }
    }

    #[doc = "Power Management Transition Control and Status Register 0\n resetvalue={Cold PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn pmtrcsr0(&self) -> crate::common::Reg<self::Pmtrcsr0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(408usize)) }
    }

    #[doc = "Power Management Transition Control and Status Register 1\n resetvalue={Cold PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn pmtrcsr1(&self) -> crate::common::Reg<self::Pmtrcsr1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(412usize)) }
    }

    #[doc = "Power Management Transition Control and Status Register 2\n resetvalue={Cold PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn pmtrcsr2(&self) -> crate::common::Reg<self::Pmtrcsr2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(416usize)) }
    }

    #[doc = "Power Management Transition Control and Status Register 3\n resetvalue={Cold PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn pmtrcsr3(&self) -> crate::common::Reg<self::Pmtrcsr3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(420usize)) }
    }

    #[doc = "Power Management Transition Control and Status Register 4\n resetvalue={Cold PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn pmtrcsr4(&self) -> crate::common::Reg<self::Pmtrcsr4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(424usize)) }
    }

    #[doc = "Product Configuration Register 0\n resetvalue={Application Reset:0x0FFFFFFFF,CFS Value:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn prdcfg0(&self) -> crate::common::Reg<self::Prdcfg0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(704usize)) }
    }

    #[doc = "Product Configuration Register 1\n resetvalue={Application Reset:0x0FFFFFFFF,CFS Value:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn prdcfg1(&self) -> crate::common::Reg<self::Prdcfg1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(708usize)) }
    }

    #[doc = "Product Configuration Register 2\n resetvalue={Application Reset:0x0FFFFFFFF,CFS Value:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn prdcfg2(&self) -> crate::common::Reg<self::Prdcfg2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(712usize)) }
    }

    #[doc = "Product Configuration Register 3\n resetvalue={Application Reset:0x0FFFFFFFF,CFS Value:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn prdcfg3(&self) -> crate::common::Reg<self::Prdcfg3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(716usize)) }
    }

    #[doc = "Product Configuration Register 4\n resetvalue={Application Reset:0x0FFFFFFFF,CFS Value:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn prdcfg4(&self) -> crate::common::Reg<self::Prdcfg4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(720usize)) }
    }

    #[doc = "Product Configuration Register 5\n resetvalue={Application Reset:0x0FFFFFFFF,CFS Value:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn prdcfg5(&self) -> crate::common::Reg<self::Prdcfg5_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(724usize)) }
    }

    #[doc = "Product Configuration Register 6\n resetvalue={Application Reset:0x0FFFFFFFF,CFS Value:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn prdcfg6(&self) -> crate::common::Reg<self::Prdcfg6_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(728usize)) }
    }

    #[doc = "Reset Configuration Register\n resetvalue={PowerOn Reset:0x282,PowerOn Reset:0x282}"]
    #[inline(always)]
    pub const fn rstcon(&self) -> crate::common::Reg<self::Rstcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(88usize)) }
    }

    #[doc = "Additional Reset Control Register\n resetvalue={Cold PowerOn Reset:0x0,Cold PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn rstcon2(&self) -> crate::common::Reg<self::Rstcon2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(100usize)) }
    }

    #[doc = "Reset Configuration Register 3\n resetvalue={Cold PowerOn Reset:0x0,After SSW execution:0x08FFF3400}"]
    #[inline(always)]
    pub const fn rstcon3(&self) -> crate::common::Reg<self::Rstcon3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(104usize)) }
    }

    #[doc = "Reset Status Register\n resetvalue={Cold PowerOn Reset:0x13810000,Cold PowerOn Reset:0x13810000}"]
    #[inline(always)]
    pub const fn rststat(&self) -> crate::common::Reg<self::Rststat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }

    #[doc = "Redesign Tracing Identification Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn rtid(&self) -> crate::common::Reg<self::Rtid_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(328usize)) }
    }

    #[doc = "Safety ENDINIT Control Register 0\n resetvalue={Application Reset:0x0FFFC000E}"]
    #[inline(always)]
    pub const fn seicon0(&self) -> crate::common::Reg<self::Seicon0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(692usize)) }
    }

    #[doc = "Safety ENDINIT Control Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn seicon1(&self) -> crate::common::Reg<self::Seicon1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(696usize)) }
    }

    #[doc = "Safety ENDINIT Timeout Status Register\n resetvalue={Application Reset:0x0FFFC0000}"]
    #[inline(always)]
    pub const fn seisr(&self) -> crate::common::Reg<self::Seisr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(700usize)) }
    }

    #[doc = "Spare Register 0\n resetvalue={System Reset:0x0FFFF0000,CFS Value:0x0FFFF0000}"]
    #[inline(always)]
    pub const fn spare0(&self) -> crate::common::Reg<self::Spare0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(336usize)) }
    }

    #[doc = "Start up Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn stcon(&self) -> crate::common::Reg<self::Stcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(196usize)) }
    }

    #[doc = "Start up Memory Register 0\n resetvalue={Cold PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn stmem0(&self) -> crate::common::Reg<self::Stmem0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(384usize)) }
    }

    #[doc = "Start up Memory Register 1\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn stmem1(&self) -> crate::common::Reg<self::Stmem1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(388usize)) }
    }

    #[doc = "Start up Memory Register 2\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn stmem2(&self) -> crate::common::Reg<self::Stmem2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(392usize)) }
    }

    #[doc = "Start up Memory Register 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn stmem3(&self) -> crate::common::Reg<self::Stmem3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(448usize)) }
    }

    #[doc = "Start up Memory Register 4\n resetvalue={Cold PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn stmem4(&self) -> crate::common::Reg<self::Stmem4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(452usize)) }
    }

    #[doc = "Start up Memory Register 5\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn stmem5(&self) -> crate::common::Reg<self::Stmem5_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(456usize)) }
    }

    #[doc = "Start up Memory Register 6\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn stmem6(&self) -> crate::common::Reg<self::Stmem6_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(460usize)) }
    }

    #[doc = "Start up Status Register\n resetvalue={PowerOn Reset:0x08000}"]
    #[inline(always)]
    pub const fn ststat(&self) -> crate::common::Reg<self::Ststat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(192usize)) }
    }

    #[doc = "Alternate Address Control Register\n resetvalue={System Reset:0x1}"]
    #[inline(always)]
    pub const fn swapctrl(&self) -> crate::common::Reg<self::Swapctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(332usize)) }
    }

    #[doc = "Software Reset Configuration Register\n resetvalue={PowerOn Reset:0x0,PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn swrstcon(&self) -> crate::common::Reg<self::Swrstcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }

    #[doc = "System Control Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn syscon(&self) -> crate::common::Reg<self::Syscon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(124usize)) }
    }

    #[doc = "System PLL Configuration 0 Register\n resetvalue={System Reset:0x40003A00}"]
    #[inline(always)]
    pub const fn syspllcon0(&self) -> crate::common::Reg<self::Syspllcon0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }

    #[doc = "System PLL Configuration 1 Register\n resetvalue={System Reset:0x5}"]
    #[inline(always)]
    pub const fn syspllcon1(&self) -> crate::common::Reg<self::Syspllcon1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "System PLL Configuration 2 Register\n resetvalue={System Reset:0x6000}"]
    #[inline(always)]
    pub const fn syspllcon2(&self) -> crate::common::Reg<self::Syspllcon2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "System PLL Status Register\n resetvalue={System Reset:0x2,System Reset:0x2}"]
    #[inline(always)]
    pub const fn syspllstat(&self) -> crate::common::Reg<self::Syspllstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Trap Clear Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn trapclr(&self) -> crate::common::Reg<self::Trapclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(300usize)) }
    }

    #[doc = "Trap Disable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn trapdis0(&self) -> crate::common::Reg<self::Trapdis0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(304usize)) }
    }

    #[doc = "Trap Set Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn trapset(&self) -> crate::common::Reg<self::Trapset_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(296usize)) }
    }

    #[doc = "Trap Status Register\n resetvalue={System Reset:0x0}"]
    #[inline(always)]
    pub const fn trapstat(&self) -> crate::common::Reg<self::Trapstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(292usize)) }
    }

    #[doc = "WDTCPU0CON0"]
    #[inline(always)]
    pub const fn wdtcpu0con0(
        &self,
    ) -> crate::common::Reg<self::Wdtcpu0Con0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(588usize)) }
    }

    #[doc = "WDTCPU0CON1"]
    #[inline(always)]
    pub const fn wdtcpu0con1(
        &self,
    ) -> crate::common::Reg<self::Wdtcpu0Con1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(592usize)) }
    }

    #[doc = "WDTCPU0SR"]
    #[inline(always)]
    pub const fn wdtcpu0sr(&self) -> crate::common::Reg<self::Wdtcpu0Sr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(596usize)) }
    }

    #[doc = "WDTCPU1CON0"]
    #[inline(always)]
    pub const fn wdtcpu1con0(
        &self,
    ) -> crate::common::Reg<self::Wdtcpu1Con0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(600usize)) }
    }

    #[doc = "WDTCPU1CON1"]
    #[inline(always)]
    pub const fn wdtcpu1con1(
        &self,
    ) -> crate::common::Reg<self::Wdtcpu1Con1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(604usize)) }
    }

    #[doc = "WDTCPU1SR"]
    #[inline(always)]
    pub const fn wdtcpu1sr(&self) -> crate::common::Reg<self::Wdtcpu1Sr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(608usize)) }
    }

    #[doc = "WDTCPU2CON0"]
    #[inline(always)]
    pub const fn wdtcpu2con0(
        &self,
    ) -> crate::common::Reg<self::Wdtcpu2Con0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(612usize)) }
    }

    #[doc = "WDTCPU2CON1"]
    #[inline(always)]
    pub const fn wdtcpu2con1(
        &self,
    ) -> crate::common::Reg<self::Wdtcpu2Con1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(616usize)) }
    }

    #[doc = "WDTCPU2SR"]
    #[inline(always)]
    pub const fn wdtcpu2sr(&self) -> crate::common::Reg<self::Wdtcpu2Sr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(620usize)) }
    }
    #[doc = "ESRCFGx"]
    #[inline(always)]
    pub fn esrcfgx(self) -> [self::EsrcfGx; 2] {
        unsafe {
            [
                self::EsrcfGx(self.0.add(0x70usize + 0x0usize)),
                self::EsrcfGx(self.0.add(0x70usize + 0x4usize)),
            ]
        }
    }
    #[doc = "WDTS"]
    #[inline(always)]
    pub fn wdts(self) -> self::Wdts {
        unsafe { self::Wdts(self.0.add(680usize)) }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accen00_SPEC;
impl crate::sealed::RegSpec for Accen00_SPEC {
    type DataType = u32;
}
#[doc = "Access Enable Register 00\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Accen00 = crate::RegValueT<Accen00_SPEC>;

impl Accen00 {
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, accen00::En0, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,accen00::En0, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, accen00::En1, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,accen00::En1, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, accen00::En2, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,accen00::En2, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, accen00::En3, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,accen00::En3, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, accen00::En4, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,accen00::En4, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, accen00::En5, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,accen00::En5, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, accen00::En6, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,accen00::En6, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, accen00::En7, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,accen00::En7, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, accen00::En8, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,accen00::En8, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, accen00::En9, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,accen00::En9, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, accen00::En10, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,accen00::En10, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, accen00::En11, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,accen00::En11, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, accen00::En12, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,accen00::En12, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, accen00::En13, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,accen00::En13, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, accen00::En14, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,accen00::En14, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, accen00::En15, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,accen00::En15, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, accen00::En16, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,accen00::En16, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, accen00::En17, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,accen00::En17, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, accen00::En18, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,accen00::En18, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, accen00::En19, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,accen00::En19, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, accen00::En20, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,accen00::En20, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, accen00::En21, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,accen00::En21, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, accen00::En22, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,accen00::En22, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, accen00::En23, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,accen00::En23, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, accen00::En24, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,accen00::En24, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, accen00::En25, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,accen00::En25, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, accen00::En26, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,accen00::En26, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, accen00::En27, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,accen00::En27, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, accen00::En28, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,accen00::En28, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, accen00::En29, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,accen00::En29, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, accen00::En30, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,accen00::En30, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en31(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, accen00::En31, Accen00_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,accen00::En31, Accen00_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Accen00 {
    #[inline(always)]
    fn default() -> Accen00 {
        <crate::RegValueT<Accen00_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod accen00 {
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
pub struct Accen10_SPEC;
impl crate::sealed::RegSpec for Accen10_SPEC {
    type DataType = u32;
}
#[doc = "Access Enable Register 10\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Accen10 = crate::RegValueT<Accen10_SPEC>;

impl Accen10 {
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, accen10::En0, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,accen10::En0, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, accen10::En1, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,accen10::En1, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, accen10::En2, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,accen10::En2, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, accen10::En3, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,accen10::En3, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, accen10::En4, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,accen10::En4, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, accen10::En5, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,accen10::En5, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, accen10::En6, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,accen10::En6, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, accen10::En7, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,accen10::En7, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, accen10::En8, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,accen10::En8, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, accen10::En9, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,accen10::En9, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, accen10::En10, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,accen10::En10, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, accen10::En11, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,accen10::En11, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, accen10::En12, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,accen10::En12, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, accen10::En13, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,accen10::En13, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, accen10::En14, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,accen10::En14, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, accen10::En15, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,accen10::En15, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, accen10::En16, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,accen10::En16, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, accen10::En17, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,accen10::En17, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, accen10::En18, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,accen10::En18, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, accen10::En19, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,accen10::En19, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, accen10::En20, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,accen10::En20, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, accen10::En21, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,accen10::En21, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, accen10::En22, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,accen10::En22, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, accen10::En23, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,accen10::En23, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, accen10::En24, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,accen10::En24, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, accen10::En25, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,accen10::En25, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, accen10::En26, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,accen10::En26, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, accen10::En27, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,accen10::En27, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, accen10::En28, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,accen10::En28, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, accen10::En29, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,accen10::En29, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, accen10::En30, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,accen10::En30, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the SCU kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en31(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, accen10::En31, Accen10_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,accen10::En31, Accen10_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Accen10 {
    #[inline(always)]
    fn default() -> Accen10 {
        <crate::RegValueT<Accen10_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod accen10 {
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
pub struct Arstdis_SPEC;
impl crate::sealed::RegSpec for Arstdis_SPEC {
    type DataType = u32;
}
#[doc = "Application Reset Disable Register\n resetvalue={PowerOn Reset:0x0}"]
pub type Arstdis = crate::RegValueT<Arstdis_SPEC>;

impl Arstdis {
    #[doc = "STM0 Disable Reset   STM0DIS. This bit field defines if an Application Reset leads to an reset for the        STM0."]
    #[inline(always)]
    pub fn stm0dis(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, arstdis::Stm0Dis, Arstdis_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            arstdis::Stm0Dis,
            Arstdis_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "STM1 Disable Reset   STM1DIS. This bit field defines if an Application Reset leads to an reset for the        STM1."]
    #[inline(always)]
    pub fn stm1dis(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, arstdis::Stm1Dis, Arstdis_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            arstdis::Stm1Dis,
            Arstdis_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "STM2 Disable Reset   STM2DIS. This bit field defines if an Application Reset leads to an reset for the        STM2."]
    #[inline(always)]
    pub fn stm2dis(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, arstdis::Stm2Dis, Arstdis_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            arstdis::Stm2Dis,
            Arstdis_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Arstdis {
    #[inline(always)]
    fn default() -> Arstdis {
        <crate::RegValueT<Arstdis_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod arstdis {
    pub struct Stm0Dis_SPEC;
    pub type Stm0Dis = crate::EnumBitfieldStruct<u8, Stm0Dis_SPEC>;
    impl Stm0Dis {
        #[doc = "An Application Reset resets the STM0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "An Application Reset has no effect for the STM0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Stm1Dis_SPEC;
    pub type Stm1Dis = crate::EnumBitfieldStruct<u8, Stm1Dis_SPEC>;
    impl Stm1Dis {
        #[doc = "0 An Application        Reset resets the STM1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An Application Reset has no effect for the STM1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Stm2Dis_SPEC;
    pub type Stm2Dis = crate::EnumBitfieldStruct<u8, Stm2Dis_SPEC>;
    impl Stm2Dis {
        #[doc = "0 An Application Reset resets the STM2"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An Application Reset has no effect for the STM2"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccucon0_SPEC;
impl crate::sealed::RegSpec for Ccucon0_SPEC {
    type DataType = u32;
}
#[doc = "CCU Clock Control Register 0\n resetvalue={After SSW execution:0x5120112,System Reset:0x3130113,System Reset:0x35120112}"]
pub type Ccucon0 = crate::RegValueT<Ccucon0_SPEC>;

impl Ccucon0 {
    #[doc = "STM Divider Reload Value   STMDIV. The resulting STM frequency is configured to f STM   xa0    xa0  f source0   xa0    xa0 STMDIV for the allowed configurations. For STMDIV  xa0    xa0 0000 B the clock is shut off. f source0 can be configured either to f PLL0  CLKSEL  xa0    xa0 01 B   or f BACK  CLKSEL  xa0    xa0 00 B"]
    #[inline(always)]
    pub fn stmdiv(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, ccucon0::Stmdiv, Ccucon0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,ccucon0::Stmdiv, Ccucon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GTM Divider Reload Value   GTMDIV. The resulting GTM frequency is configured to f GTM   f SOURCEGTM   GTMDIV for the allowed configurations. For GTMDIV   0000 B the clock is shut off."]
    #[inline(always)]
    pub fn gtmdiv(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, ccucon0::Gtmdiv, Ccucon0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xf,1,0,ccucon0::Gtmdiv, Ccucon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SRI Divider Reload Value   SRIDIV. The resulting SRI frequency is configured to f SRI   f source0   SRIDIV for the allowed configurations. f source0 could be configured either to f PLL0  CLKSEL   01 B   or f BACK  CLKSEL   00 B"]
    #[inline(always)]
    pub fn sridiv(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, ccucon0::Sridiv, Ccucon0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,ccucon0::Sridiv, Ccucon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Low Power Divider Reload Value   LPDIV. The selected divider is valid for all clocks derived from f XXX with XXX   SPB  SRI  BBB  FSI  GETH  GTM  ADAS."]
    #[inline(always)]
    pub fn lpdiv(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, ccucon0::Lpdiv, Ccucon0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x7,1,0,ccucon0::Lpdiv, Ccucon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SPB Divider Reload Value   SPBDIV. f source0 could be configured either to f PLL  CLKSEL   01 B   or f BACK  CLKSEL   00 B"]
    #[inline(always)]
    pub fn spbdiv(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, ccucon0::Spbdiv, Ccucon0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            ccucon0::Spbdiv,
            Ccucon0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "BBB Divider Reload Value   BBBDIV. The resulting BBB frequency is configured to f BBB   160    160  f source0   160    160 BBBDIV        for all allowed configurations. For BBBDIV  160    160 0000 B the clock is shut off. f source0 could be        configured either to f PLL0  CLKSEL  160    160 01 B   or f BACK  CLKSEL  160    160 00 B"]
    #[inline(always)]
    pub fn bbbdiv(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, ccucon0::Bbbdiv, Ccucon0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            20,
            0xf,
            1,
            0,
            ccucon0::Bbbdiv,
            Ccucon0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "FSI Divider Reload Value   FSIDIV"]
    #[inline(always)]
    pub fn fsidiv(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, ccucon0::Fsidiv, Ccucon0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            ccucon0::Fsidiv,
            Ccucon0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "FSI2 Divider Reload Value   FSI2DIV"]
    #[inline(always)]
    pub fn fsi2div(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x3,
        1,
        0,
        ccucon0::Fsi2Div,
        Ccucon0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x3,
            1,
            0,
            ccucon0::Fsi2Div,
            Ccucon0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Clock Selection for Source   CLKSEL. This bit field defines the clock source that is used for the clock        generation of f sourcex ."]
    #[inline(always)]
    pub fn clksel(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, ccucon0::Clksel, Ccucon0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            28,
            0x3,
            1,
            0,
            ccucon0::Clksel,
            Ccucon0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Update Request   UP. Setting this bit will request an update for CCUCON0 and CCUCON5. Only one UP bit must be set for either CCUCON0 or CCUCON5. This bit always reads as zero."]
    #[inline(always)]
    pub fn up(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, ccucon0::Up, Ccucon0_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<30,0x1,1,0,ccucon0::Up, Ccucon0_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if the register is locked and a write action from the bus side has no effect. The lock bit is set when an update of CCUCON0 5 has been requested  and released when the update is complete."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, ccucon0::Lck, Ccucon0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<31,0x1,1,0,ccucon0::Lck, Ccucon0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ccucon0 {
    #[inline(always)]
    fn default() -> Ccucon0 {
        <crate::RegValueT<Ccucon0_SPEC> as RegisterValue<_>>::new(17957138)
    }
}
pub mod ccucon0 {
    pub struct Stmdiv_SPEC;
    pub type Stmdiv = crate::EnumBitfieldStruct<u8, Stmdiv_SPEC>;
    impl Stmdiv {
        #[doc = "f STM   160 is stopped"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "f STM   160    160  f source0"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "f STM   160    160  f source0  2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "f STM   160    160  f source0  3"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "f STM   160    160  f source0  4"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "f STM   160    160  f source0  5"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "f STM   160    160  f source0  6  160"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "f STM   160    160  f source0  8  160"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "f STM   160    160  f source0  10  160"]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "f STM   160    160  f source0  12  160"]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "f STM   160    160  f source0  15"]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Gtmdiv_SPEC;
    pub type Gtmdiv = crate::EnumBitfieldStruct<u8, Gtmdiv_SPEC>;
    impl Gtmdiv {
        #[doc = "f GTM   xa0 is stopped"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "f GTM   xa0    xa0  f SOURCEGTM"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "f GTM   xa0    xa0  f SOURCEGTM  2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "f GTM   xa0    xa0  f SOURCEGTM  3"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "f GTM   f SOURCEGTM  4"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "f GTM   f SOURCEGTM  5"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "f GTM   f SOURCEGTM  6"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "f GTM   f SOURCEGTM  8"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "f GTM   f SOURCEGTM  10"]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "f GTM   f SOURCEGTM  12"]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "f GTM   xa0    xa0  f SOURCEGTM  15"]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Sridiv_SPEC;
    pub type Sridiv = crate::EnumBitfieldStruct<u8, Sridiv_SPEC>;
    impl Sridiv {
        #[doc = "f SRI   xa0    xa0  f source0"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "f SRI   xa0    xa0  f source0  2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "f SRI   xa0    xa0  f source0  3"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "f SRI   f source0  4"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "f SRI   xa0    xa0  f source0  5"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "f SRI   f source0  6"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "f SRI   f source0  8"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "f SRI   f source0  10"]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "f SRI   f source0  12"]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "f SRI   xa0    xa0  f source0  15"]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Lpdiv_SPEC;
    pub type Lpdiv = crate::EnumBitfieldStruct<u8, Lpdiv_SPEC>;
    impl Lpdiv {
        #[doc = "f XXX controlled by the related CCUCON0 5 bit fields"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "f XXX   f source0   30"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "f XXX   f source0   60"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "f XXX   f source0   120"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "f XXX   f source0   240"]
        pub const CONST_44: Self = Self::new(4);
    }
    pub struct Spbdiv_SPEC;
    pub type Spbdiv = crate::EnumBitfieldStruct<u8, Spbdiv_SPEC>;
    impl Spbdiv {
        #[doc = "f SPB   xa0    xa0  f source0  2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "f SPB   xa0    xa0  f source0  3"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "f SPB   f source0  4"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "f SPB   xa0    xa0  f source0  5"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "f SPB   f source0  6"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "f SPB   f source0  8"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "f SPB   f source0  10"]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "f SPB   f source0  12"]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "f SPB   f source0  15"]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Bbbdiv_SPEC;
    pub type Bbbdiv = crate::EnumBitfieldStruct<u8, Bbbdiv_SPEC>;
    impl Bbbdiv {
        #[doc = "f BBB is stopped"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "f BBB   f source0"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "f BBB   f source0  2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "f BBB   f source0  3"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "f BBB   160    160  f source0  4  160"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "f BBB   xa0    xa0  f source0  5"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "f BBB   160    160  f source0  6  160"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "f BBB   f source0  8"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "f BBB   f source0  10"]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "f BBB   f source0  12"]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "f BBB   xa0    xa0  f source0  15"]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Fsidiv_SPEC;
    pub type Fsidiv = crate::EnumBitfieldStruct<u8, Fsidiv_SPEC>;
    impl Fsidiv {
        #[doc = "f FSI   f SRI"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "f FSI   f SRI   2 for SRIDIV   0001 B or 0010 B   else f FSI   f SRI"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "f FSI   f SRI   3 for SRIDIV   0001 B or 0010 B   else f FSI   f SRI"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Fsi2Div_SPEC;
    pub type Fsi2Div = crate::EnumBitfieldStruct<u8, Fsi2Div_SPEC>;
    impl Fsi2Div {
        #[doc = "f FSI2   f SRI"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clksel_SPEC;
    pub type Clksel = crate::EnumBitfieldStruct<u8, Clksel_SPEC>;
    impl Clksel {
        #[doc = "f BACK is used as clock source f source0   f src1   and f source2"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "f PLL0 is used as clock source f source0   f PLL1 is used as clock source f src1   f PLL2 is used as clock source f source2"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Up_SPEC;
    pub type Up = crate::EnumBitfieldStruct<u8, Up_SPEC>;
    impl Up {
        #[doc = "No        action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "A new complete parameter set is transferred to the CCU defined by registers CCUCON0 and CCUCON5."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lck_SPEC;
    pub type Lck = crate::EnumBitfieldStruct<u8, Lck_SPEC>;
    impl Lck {
        #[doc = "The register is unlocked and can be updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "The register is locked and can not be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccucon1_SPEC;
impl crate::sealed::RegSpec for Ccucon1_SPEC {
    type DataType = u32;
}
#[doc = "CCU Clock Control Register 1\n resetvalue={System Reset:0x10100302}"]
pub type Ccucon1 = crate::RegValueT<Ccucon1_SPEC>;

impl Ccucon1 {
    #[doc = "MCAN Divider Reload Value   MCANDIV. The resulting MCAN frequency is configured to f MCANI   160    160  f source1   160    160 MCANDIV        for the allowed configurations. For MCANDIV  160    160 0000 B the clock is shut off."]
    #[inline(always)]
    pub fn mcandiv(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, ccucon1::Mcandiv, Ccucon1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            ccucon1::Mcandiv,
            Ccucon1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Clock Selection for MCAN   CLKSELMCAN. This bit field defines the clock source that is used for the clock generation of f SOURCEMCAN . For switching between two non zero configurations the following sequence has to be applied  First step is to switch to 00 B . Second step is to switch to the new target configuration."]
    #[inline(always)]
    pub fn clkselmcan(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        ccucon1::Clkselmcan,
        Ccucon1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            ccucon1::Clkselmcan,
            Ccucon1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Divider Disable for fPLL1   PLL1DIVDIS. Depending on CCUCON0.CLKSEL  this bit selects whether f source1 is half f pll1 ."]
    #[inline(always)]
    pub fn pll1divdis(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ccucon1::Pll1Divdis,
        Ccucon1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ccucon1::Pll1Divdis,
            Ccucon1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "I2C Divider Reload Value   I2CDIV. The resulting I2C frequency is configured to f I2C   xa0    xa0  f SOURCE2   xa0    xa0 I2CDIV for the allowed configurations. For I2CDIV  xa0    xa0 0000 B the clock is shut off."]
    #[inline(always)]
    pub fn i2cdiv(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, ccucon1::I2Cdiv, Ccucon1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,ccucon1::I2Cdiv, Ccucon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "MSC Divider Reload Value   MSCDIV. The resulting MSC frequency is configured to f MSC   160    160  f SOURCEMSC   160    160 MSCDIV        for the allowed configurations. For MSCDIV  160    160 0000 B the clock is shut off."]
    #[inline(always)]
    pub fn mscdiv(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, ccucon1::Mscdiv, Ccucon1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            ccucon1::Mscdiv,
            Ccucon1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Clock Selection for MSC   CLKSELMSC. This bit field defines the clock source that is used for the clock generation of f SOURCEMSC . For switching between two non zero configurations the following sequence has to be applied  First step is to switch to 00 B . Second step is to switch to the new target configuration."]
    #[inline(always)]
    pub fn clkselmsc(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        ccucon1::Clkselmsc,
        Ccucon1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            ccucon1::Clkselmsc,
            Ccucon1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "QSPI Divider Reload Value   QSPIDIV. The resulting QSPI frequency is configured to f QSPI   160    160  f SOURCEQSPI   160    160 QSPIDIV        for the allowed configurations. For QSPIDIV  160    160 0000 B the clock is shut off."]
    #[inline(always)]
    pub fn qspidiv(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xf,
        1,
        0,
        ccucon1::Qspidiv,
        Ccucon1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xf,
            1,
            0,
            ccucon1::Qspidiv,
            Ccucon1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Clock Selection for QSPI   CLKSELQSPI. This bit field defines the clock source that is used for the clock generation of f SOURCEQSPI . For switching between two non zero configurations the following sequence has to be applied  First step is to switch to 00 B . Second step is to switch to the new target configuration."]
    #[inline(always)]
    pub fn clkselqspi(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x3,
        1,
        0,
        ccucon1::Clkselqspi,
        Ccucon1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x3,
            1,
            0,
            ccucon1::Clkselqspi,
            Ccucon1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if the register is locked and a write action from the bus side has no effect. The lock bit is set when at least one bit field is changed  and released when this change is executed."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, ccucon1::Lck, Ccucon1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<31,0x1,1,0,ccucon1::Lck, Ccucon1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ccucon1 {
    #[inline(always)]
    fn default() -> Ccucon1 {
        <crate::RegValueT<Ccucon1_SPEC> as RegisterValue<_>>::new(269484802)
    }
}
pub mod ccucon1 {
    pub struct Mcandiv_SPEC;
    pub type Mcandiv = crate::EnumBitfieldStruct<u8, Mcandiv_SPEC>;
    impl Mcandiv {
        #[doc = "f MCANI   xa0 is stopped"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "f MCANI   xa0    xa0  f source1"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "f MCANI   xa0    xa0  f source1  2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "f MCANI   xa0    xa0  f source1  3"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "f MCANI   f source1  4"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "f MCANI   xa0    xa0  f source1  5"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "f MCANI   f source1  6"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "f MCANI   f source1  8"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "f MCANI   f source1  10"]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "f MCANI   f source1  12"]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "f MCANI   xa0    xa0  f source1  15"]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Clkselmcan_SPEC;
    pub type Clkselmcan = crate::EnumBitfieldStruct<u8, Clkselmcan_SPEC>;
    impl Clkselmcan {
        #[doc = "f MCAN clock is stopped"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "f MCANI is used as clock source f MCAN"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "f OSC0 is used as clock source f MCAN"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Pll1Divdis_SPEC;
    pub type Pll1Divdis = crate::EnumBitfieldStruct<u8, Pll1Divdis_SPEC>;
    impl Pll1Divdis {
        #[doc = "CLKSEL    01   x2192  f source1   f back CLKSEL   01   x2192  f source1   f pll1  2"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "CLKSEL    01   x2192  f source1   f back CLKSEL   01   x2192  f source1   f pll1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct I2Cdiv_SPEC;
    pub type I2Cdiv = crate::EnumBitfieldStruct<u8, I2Cdiv_SPEC>;
    impl I2Cdiv {
        #[doc = "f I2C is stopped"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "f I2C   xa0    xa0  f SOURCE2"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "f I2C   f SOURCE2  2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "f I2C   f SOURCE2  3"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "f I2C   f SOURCE2  4  xa0"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "f I2C   f SOURCE2  5"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "f I2C   xa0    xa0  f SOURCE2  6  xa0"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "f I2C   xa0    xa0  f SOURCE2  8  xa0"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "f I2C   xa0    xa0  f SOURCE2  10"]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "f I2C   xa0    xa0  f SOURCE2  12"]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "f I2C   xa0    xa0  f SOURCE0  15"]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Mscdiv_SPEC;
    pub type Mscdiv = crate::EnumBitfieldStruct<u8, Mscdiv_SPEC>;
    impl Mscdiv {
        #[doc = "f MSC   xa0 is stopped"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "f MSC   xa0    xa0  f SOURCEMSC"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "f MSC   xa0    xa0  f SOURCEMSC  2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "f MSC   xa0    xa0  f SOURCEMSC  3"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "f MSC   f SOURCEMSC  4"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "f MSC   xa0    xa0  f SOURCEMSC  5"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "f MSC   f SOURCEMSC  6"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "f MSC   f SOURCEMSC  8"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "f MSC   f SOURCEMSC  10"]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "f MSC   f SOURCEMSC  12"]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "f MSC   xa0    xa0  f SOURCEMSC  15"]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Clkselmsc_SPEC;
    pub type Clkselmsc = crate::EnumBitfieldStruct<u8, Clkselmsc_SPEC>;
    impl Clkselmsc {
        #[doc = "f MSC clock is stopped"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "f source1 is used as clock source f SOURCEMSC"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "f source2 is used as clock source f SOURCEMSC"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Qspidiv_SPEC;
    pub type Qspidiv = crate::EnumBitfieldStruct<u8, Qspidiv_SPEC>;
    impl Qspidiv {
        #[doc = "f QSPI   xa0 is stopped"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "f QSPI   xa0    xa0  f SOURCEQSPI"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "f QSPI   xa0    xa0  f SOURCEQSPI  2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "f QSPI   xa0    xa0  f SOURCEQSPI  3"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "f QSPI   f SOURCEQSPI  4"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "f QSPI   xa0    xa0  f SOURCEQSPI  5"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "f QSPI   f SOURCEQSPI  6"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "f QSPI   f SOURCEQSPI  8"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "f QSPI   f SOURCEQSPI  10"]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "f QSPI   f SOURCEQSPI  12"]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "f QSPI   xa0    xa0  f SOURCEQSPI  15"]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Clkselqspi_SPEC;
    pub type Clkselqspi = crate::EnumBitfieldStruct<u8, Clkselqspi_SPEC>;
    impl Clkselqspi {
        #[doc = "f QSPI clock is stopped"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "f source1 is used as clock source f SOURCEQSPI"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "f source2 is used as clock source f SOURCEQSPI"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Lck_SPEC;
    pub type Lck = crate::EnumBitfieldStruct<u8, Lck_SPEC>;
    impl Lck {
        #[doc = "The register is unlocked and can be updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "The register is locked and can not be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccucon2_SPEC;
impl crate::sealed::RegSpec for Ccucon2_SPEC {
    type DataType = u32;
}
#[doc = "CCU Clock Control Register 2\n resetvalue={System Reset:0x7000101}"]
pub type Ccucon2 = crate::RegValueT<Ccucon2_SPEC>;

impl Ccucon2 {
    #[doc = "ASCLIN Fast Divider Reload Value   ASCLINFDIV. The resulting ASCLIN frequency is configured to f ASCLINF   f source2   ASCLINFDIV for the allowed configurations. For ASCLINFDIV   0000 B the clock is shut off. f source2 could be configured either to f PLL2  CLKSEL   01 B   or f BACK  CLKSEL   00 B"]
    #[inline(always)]
    pub fn asclinfdiv(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        ccucon2::Asclinfdiv,
        Ccucon2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            ccucon2::Asclinfdiv,
            Ccucon2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ASCLIN Slow Divider Reload Value   ASCLINSDIV. The resulting ASCLIN frequency is configured to f ASCLINSI   f source1   ASCLINSDIV for the allowed configurations. For ASCLINSDIV   0000 B the clock is shut off. f source1 could be configured either to f PLL1  CLKSEL   01 B   or f BACK  CLKSEL   00 B"]
    #[inline(always)]
    pub fn asclinsdiv(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        ccucon2::Asclinsdiv,
        Ccucon2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            ccucon2::Asclinsdiv,
            Ccucon2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Clock Selection for ASCLINS   CLKSELASCLINS. This bit field defines the clock source that is used for the clock generation of f ASCLINS . For switching between two non zero configurations the following sequence has to be applied  First step is to switch to 00 B . Second step is to switch to the new target configuration."]
    #[inline(always)]
    pub fn clkselasclins(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        ccucon2::Clkselasclins,
        Ccucon2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            ccucon2::Clkselasclins,
            Ccucon2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Power Safe SwitchOff for ERAY Clock   ERAYPERON. This bit is used to control the ERAY peripheral clock f ERAY for power saving purposes if the logic is not used by the application."]
    #[inline(always)]
    pub fn erayperon(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        ccucon2::Erayperon,
        Ccucon2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            ccucon2::Erayperon,
            Ccucon2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if the register is locked and a write action from the bus side has no effect. The lock bit is set when at least one bit field is changed  and released when this change is executed."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, ccucon2::Lck, Ccucon2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<31,0x1,1,0,ccucon2::Lck, Ccucon2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ccucon2 {
    #[inline(always)]
    fn default() -> Ccucon2 {
        <crate::RegValueT<Ccucon2_SPEC> as RegisterValue<_>>::new(117440769)
    }
}
pub mod ccucon2 {
    pub struct Asclinfdiv_SPEC;
    pub type Asclinfdiv = crate::EnumBitfieldStruct<u8, Asclinfdiv_SPEC>;
    impl Asclinfdiv {
        #[doc = "f ASCLINF   xa0 is stopped"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "f ASCLINF   xa0    xa0  f source2"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "f ASCLINF   xa0    xa0  f source2  2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "f ASCLINF   xa0    xa0  f source2  3"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "f ASCLINF   f source2  4"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "f ASCLINF   xa0    xa0  f source2  5"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "f ASCLINF   f source2  6"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "f ASCLINF   f source2  8"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "f ASCLINF   f source2  10"]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "f ASCLINF   f source2  12"]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "f ASCLINF   xa0    xa0  f source2  15"]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Asclinsdiv_SPEC;
    pub type Asclinsdiv = crate::EnumBitfieldStruct<u8, Asclinsdiv_SPEC>;
    impl Asclinsdiv {
        #[doc = "f ASCLINSI   xa0 is stopped"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "f ASCLINSI   xa0    xa0  f source1"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "f ASCLINSI   xa0    xa0  f source1  2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "f ASCLINSI   xa0    xa0  f source1  3"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "f ASCLINSI   xa0    xa0  f source1  4  xa0"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "f ASCLINSI   xa0    xa0  f source1  5"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "f ASCLINSI   xa0    xa0  f source1  6"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "f ASCLINSI   xa0    xa0  f source1  8"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "f ASCLINSI   xa0    xa0  f source1  10"]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "f ASCLINSI   xa0    xa0  f source1  12"]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "f ASCLINSI   xa0    xa0  f source1  15"]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Clkselasclins_SPEC;
    pub type Clkselasclins = crate::EnumBitfieldStruct<u8, Clkselasclins_SPEC>;
    impl Clkselasclins {
        #[doc = "f ASCLINS clock is stopped"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "f ASCLINSI is used as clock f ASCLINS"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "f OSC0 is used as clock f ASCLINS"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Erayperon_SPEC;
    pub type Erayperon = crate::EnumBitfieldStruct<u8, Erayperon_SPEC>;
    impl Erayperon {
        #[doc = "f ERAY is stopped"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "f ERAY   f source1   2"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lck_SPEC;
    pub type Lck = crate::EnumBitfieldStruct<u8, Lck_SPEC>;
    impl Lck {
        #[doc = "The register is unlocked and can be updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "The register is locked and can not be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccucon3_SPEC;
impl crate::sealed::RegSpec for Ccucon3_SPEC {
    type DataType = u32;
}
#[doc = "CCU Clock Control Register 3\n resetvalue={System Reset:0x0}"]
pub type Ccucon3 = crate::RegValueT<Ccucon3_SPEC>;

impl Ccucon3 {
    #[doc = "PLL0 Clock Monitor Enable   PLL0MONEN"]
    #[inline(always)]
    pub fn pll0monen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ccucon3::Pll0Monen,
        Ccucon3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ccucon3::Pll0Monen,
            Ccucon3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "PLL1 Clock Monitor Enable   PLL1MONEN"]
    #[inline(always)]
    pub fn pll1monen(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ccucon3::Pll1Monen,
        Ccucon3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ccucon3::Pll1Monen,
            Ccucon3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "PLL2 Clock Monitor Enable   PLL2MONEN"]
    #[inline(always)]
    pub fn pll2monen(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ccucon3::Pll2Monen,
        Ccucon3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ccucon3::Pll2Monen,
            Ccucon3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "SPB Clock Monitor Enable   SPBMONEN"]
    #[inline(always)]
    pub fn spbmonen(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ccucon3::Spbmonen,
        Ccucon3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ccucon3::Spbmonen,
            Ccucon3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Backup Clock Monitor Enable   BACKMONEN"]
    #[inline(always)]
    pub fn backmonen(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ccucon3::Backmonen,
        Ccucon3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ccucon3::Backmonen,
            Ccucon3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "PLL0 Clock Monitor Test   PLL0MONTST. The test enable bit is not a direct trigger for the alarm  but an inhibit for the clock to be monitored. This is to test the monitoring logic itself as well."]
    #[inline(always)]
    pub fn pll0montst(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ccucon3::Pll0Montst,
        Ccucon3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ccucon3::Pll0Montst,
            Ccucon3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "PLL1 Clock Monitor Test   PLL1MONTST. The test enable bit is not a direct trigger for the alarm  but an inhibit for the clock to be monitored. This is to test the monitoring logic itself as well."]
    #[inline(always)]
    pub fn pll1montst(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ccucon3::Pll1Montst,
        Ccucon3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ccucon3::Pll1Montst,
            Ccucon3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "PLL2 Clock Monitor Test   PLL2MONTST. The test enable bit is not a direct trigger for the alarm  but an inhibit for the clock to be monitored. This is to test the monitoring logic itself as well."]
    #[inline(always)]
    pub fn pll2montst(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        ccucon3::Pll2Montst,
        Ccucon3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            ccucon3::Pll2Montst,
            Ccucon3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "SPB Clock Monitor Test   SPBMONTST. The test enable bit is not a direct trigger for the alarm  but an inhibit for the clock to be monitored. This is to test the monitoring logic itself as well."]
    #[inline(always)]
    pub fn spbmontst(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        ccucon3::Spbmontst,
        Ccucon3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            ccucon3::Spbmontst,
            Ccucon3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Backup Clock Monitor Test   BACKMONTST. The test enable bit is not a direct trigger for the alarm  but an inhibit for the clock to be monitored. This is to test the monitoring logic itself as well."]
    #[inline(always)]
    pub fn backmontst(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ccucon3::Backmontst,
        Ccucon3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ccucon3::Backmontst,
            Ccucon3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Update Request   UP. Setting this bit will request an update for CCUCON3 and CCUCON4. Only one UP bit must be set for either CCUCON3 or CCUCON4. This bit always reads as zero."]
    #[inline(always)]
    pub fn up(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, ccucon3::Up, Ccucon3_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<30,0x1,1,0,ccucon3::Up, Ccucon3_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if the register is locked and a write action from the bus side has no effect. The lock bit is set when an update of CCUCON3 4 has been requested  and released when the update is complete."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, ccucon3::Lck, Ccucon3_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<31,0x1,1,0,ccucon3::Lck, Ccucon3_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ccucon3 {
    #[inline(always)]
    fn default() -> Ccucon3 {
        <crate::RegValueT<Ccucon3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccucon3 {
    pub struct Pll0Monen_SPEC;
    pub type Pll0Monen = crate::EnumBitfieldStruct<u8, Pll0Monen_SPEC>;
    impl Pll0Monen {
        #[doc = "Monitoring is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Monitoring is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pll1Monen_SPEC;
    pub type Pll1Monen = crate::EnumBitfieldStruct<u8, Pll1Monen_SPEC>;
    impl Pll1Monen {
        #[doc = "Monitoring is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Monitoring is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pll2Monen_SPEC;
    pub type Pll2Monen = crate::EnumBitfieldStruct<u8, Pll2Monen_SPEC>;
    impl Pll2Monen {
        #[doc = "Monitoring is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Monitoring is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Spbmonen_SPEC;
    pub type Spbmonen = crate::EnumBitfieldStruct<u8, Spbmonen_SPEC>;
    impl Spbmonen {
        #[doc = "Monitoring is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Monitoring is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Backmonen_SPEC;
    pub type Backmonen = crate::EnumBitfieldStruct<u8, Backmonen_SPEC>;
    impl Backmonen {
        #[doc = "Monitoring is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Monitoring is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pll0Montst_SPEC;
    pub type Pll0Montst = crate::EnumBitfieldStruct<u8, Pll0Montst_SPEC>;
    impl Pll0Montst {
        #[doc = "normal operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Inhibit f PLL0 at monitor input. It may take a full monitor reference count period  512 f BACK cycles  until an alarm is generated. This depends on the selected PLL frequencies."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pll1Montst_SPEC;
    pub type Pll1Montst = crate::EnumBitfieldStruct<u8, Pll1Montst_SPEC>;
    impl Pll1Montst {
        #[doc = "normal operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Inhibit f PLL1 at monitor input. It may take a full monitor reference count period  512 f BACK cycles  until an alarm is generated. This depends on the selected PLL frequencies."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pll2Montst_SPEC;
    pub type Pll2Montst = crate::EnumBitfieldStruct<u8, Pll2Montst_SPEC>;
    impl Pll2Montst {
        #[doc = "normal operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Inhibit f PLL2 at monitor input. It may take a full monitor reference count period  512 f BACK cycles  until an alarm is generated. This depends on the selected PLL frequencies."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Spbmontst_SPEC;
    pub type Spbmontst = crate::EnumBitfieldStruct<u8, Spbmontst_SPEC>;
    impl Spbmontst {
        #[doc = "normal operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Inhibit f SPB at monitor input. It may take a full monitor reference count period  512 f BACK cycles  until an alarm is generated. This depends on the selected PLL frequencies."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Backmontst_SPEC;
    pub type Backmontst = crate::EnumBitfieldStruct<u8, Backmontst_SPEC>;
    impl Backmontst {
        #[doc = "normal operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Inhibit f BACK at monitor input. It may take a full monitor reference count period  512 f PLL0 cycles  until an alarm is generated. This depends on the selected PLL frequencies."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Up_SPEC;
    pub type Up = crate::EnumBitfieldStruct<u8, Up_SPEC>;
    impl Up {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "A new complete parameter set is transferred to the CCU defined by register CCUCON3 and CCUCON4."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lck_SPEC;
    pub type Lck = crate::EnumBitfieldStruct<u8, Lck_SPEC>;
    impl Lck {
        #[doc = "The register is unlocked and can be updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "The register is locked and can not be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccucon4_SPEC;
impl crate::sealed::RegSpec for Ccucon4_SPEC {
    type DataType = u32;
}
#[doc = "CCU Clock Control Register 4\n resetvalue={System Reset:0x0}"]
pub type Ccucon4 = crate::RegValueT<Ccucon4_SPEC>;

impl Ccucon4 {
    #[doc = "Backup Clock Monitor Lower Threshold   LOTHR. lower threshold   512  f PLL0   0.9   100 MHz For proper operation and to avoid false alarms  the monitor needs to be disabled via MONEN 0 before changing setting the threshold values."]
    #[inline(always)]
    pub fn lothr(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, Ccucon4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16, Ccucon4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Backup Clock Monitor Upper Threshold   UPTHR. upper threshold   512  f PLL0   1.1   100 MHz For proper operation and to avoid false alarms  the monitor needs to be disabled via MONEN 0 before changing setting the threshold values."]
    #[inline(always)]
    pub fn upthr(
        self,
    ) -> crate::common::RegisterField<12, 0xfff, 1, 0, u16, Ccucon4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xfff,1,0,u16, Ccucon4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Backup Clock Monitor Enable   MONEN"]
    #[inline(always)]
    pub fn monen(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, ccucon4::Monen, Ccucon4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,ccucon4::Monen, Ccucon4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Backup Clock Monitor Test   MONTST. Set this bit to 1 to test alarm generation. The test enable bit is a direct trigger for the alarm."]
    #[inline(always)]
    pub fn montst(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, ccucon4::Montst, Ccucon4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            ccucon4::Montst,
            Ccucon4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Update Request   UP. Setting this bit will request an update for CCUCON3 and CCUCON4. Only one UP bit must be set for either CCUCON3 or CCUCON4. This bit always reads as zero."]
    #[inline(always)]
    pub fn up(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, ccucon4::Up, Ccucon4_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<30,0x1,1,0,ccucon4::Up, Ccucon4_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if the register is locked and a write action from the bus side has no effect. The lock bit is set when an update of CCUCON3 4 has been requested  and released when the update is complete."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, ccucon4::Lck, Ccucon4_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<31,0x1,1,0,ccucon4::Lck, Ccucon4_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ccucon4 {
    #[inline(always)]
    fn default() -> Ccucon4 {
        <crate::RegValueT<Ccucon4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ccucon4 {
    pub struct Monen_SPEC;
    pub type Monen = crate::EnumBitfieldStruct<u8, Monen_SPEC>;
    impl Monen {
        #[doc = "Monitoring is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Monitoring is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Montst_SPEC;
    pub type Montst = crate::EnumBitfieldStruct<u8, Montst_SPEC>;
    impl Montst {
        #[doc = "Normal Operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Test Alarm will be generated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Up_SPEC;
    pub type Up = crate::EnumBitfieldStruct<u8, Up_SPEC>;
    impl Up {
        #[doc = "No        action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "A new complete parameter set is transferred to the CCU defined by register CCUCON3 and CCUCON4."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lck_SPEC;
    pub type Lck = crate::EnumBitfieldStruct<u8, Lck_SPEC>;
    impl Lck {
        #[doc = "The register is unlocked and can be updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "The register is locked and can not be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccucon5_SPEC;
impl crate::sealed::RegSpec for Ccucon5_SPEC {
    type DataType = u32;
}
#[doc = "CCU Clock Control Register 5\n resetvalue={System Reset:0x30,System Reset:0x30}"]
pub type Ccucon5 = crate::RegValueT<Ccucon5_SPEC>;

impl Ccucon5 {
    #[doc = "GETH Divider Reload Value   GETHDIV. The resulting GETH frequency is configured to f GETH   xa0    xa0  f source0   xa0    xa0 GETHDIV for the allowed configurations. For GETHDIV  xa0    xa0 0000 B the clock is shut off. f source0 could be configured either to f PLL0  CLKSEL  xa0    xa0 01 B   or f BACK  CLKSEL  xa0    xa0 00 B   GETHDIV must be enabled    0  during an application reset to allow firmware related installation tasks."]
    #[inline(always)]
    pub fn gethdiv(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, ccucon5::Gethdiv, Ccucon5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            ccucon5::Gethdiv,
            Ccucon5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "MCANH Divider Reload Value   MCANHDIV. The resulting MCANH frequency is configured to f MCANH   f SOURCE0   MCANHDIV for the allowed configurations. For MCANHDIV   0000 B the clock is shut off."]
    #[inline(always)]
    pub fn mcanhdiv(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        ccucon5::Mcanhdiv,
        Ccucon5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            ccucon5::Mcanhdiv,
            Ccucon5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Update Request   UP. Setting this bit will request an update for CCUCON0 and CCUCON5. Only one UP bit must be set either CCUCON0 or CCUCON5. This bit always reads as zero."]
    #[inline(always)]
    pub fn up(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, ccucon5::Up, Ccucon5_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<30,0x1,1,0,ccucon5::Up, Ccucon5_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if the register is locked and a write action from the bus side has no effect. The lock bit is set when an update of CCUCON0 5 has been requested  and released when the update is complete."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, ccucon5::Lck, Ccucon5_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<31,0x1,1,0,ccucon5::Lck, Ccucon5_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ccucon5 {
    #[inline(always)]
    fn default() -> Ccucon5 {
        <crate::RegValueT<Ccucon5_SPEC> as RegisterValue<_>>::new(48)
    }
}
pub mod ccucon5 {
    pub struct Gethdiv_SPEC;
    pub type Gethdiv = crate::EnumBitfieldStruct<u8, Gethdiv_SPEC>;
    impl Gethdiv {
        #[doc = "f GETH   xa0 is stopped"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "f GETH   xa0    xa0  f source0"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "f GETH   xa0    xa0  f source0  2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "f GETH   xa0    xa0  f source0  3"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "f GETH   160    160  f source0  4"]
        pub const CONST_44: Self = Self::new(4);
    }
    pub struct Mcanhdiv_SPEC;
    pub type Mcanhdiv = crate::EnumBitfieldStruct<u8, Mcanhdiv_SPEC>;
    impl Mcanhdiv {
        #[doc = "f MCANH   xa0 is stopped"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "f MCANH   xa0    xa0  f SOURCE0"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "f MCANH   xa0    xa0  f SOURCE0  2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "f MCANH   xa0    xa0  f SOURCE0  3"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "f MCANH   xa0    xa0  f SOURCE0  4"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "f MCANH   xa0    xa0  f SOURCE0  5"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "f MCANH   xa0    xa0  f SOURCE0  6"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "f MCANH   xa0    xa0  f SOURCE0  8  xa0"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "f MCANH   xa0    xa0  f SOURCE0  10  xa0"]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "f MCANH   xa0    xa0  f SOURCE0  12  xa0"]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "f MCANH   xa0    xa0  f SOURCE0  15"]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Up_SPEC;
    pub type Up = crate::EnumBitfieldStruct<u8, Up_SPEC>;
    impl Up {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "A new complete parameter set is transferred to the CCU defined by register CCUCON0 and CCUCON5."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lck_SPEC;
    pub type Lck = crate::EnumBitfieldStruct<u8, Lck_SPEC>;
    impl Lck {
        #[doc = "The register is unlocked and can be updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "The register is locked and can not be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccucon6_SPEC;
impl crate::sealed::RegSpec for Ccucon6_SPEC {
    type DataType = u32;
}
#[doc = "CCU Clock Control Register 6\n resetvalue={System Reset:0x0}"]
pub type Ccucon6 = crate::RegValueT<Ccucon6_SPEC>;

impl Ccucon6 {
    #[doc = "CPU0 Divider Reload Value   CPU0DIV. The resulting CPU0 frequency  performance  is configured to f CPU0   xa0    xa0  f SRI   xa0    xa0  64  xa0    xa0 CPU0DIV   xa0    xa0 64. For CPU0DIV  xa0    xa0 000000 B   f CPU0   xa0    xa0  f SRI ."]
    #[inline(always)]
    pub fn cpu0div(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Ccucon6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Ccucon6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ccucon6 {
    #[inline(always)]
    fn default() -> Ccucon6 {
        <crate::RegValueT<Ccucon6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccucon7_SPEC;
impl crate::sealed::RegSpec for Ccucon7_SPEC {
    type DataType = u32;
}
#[doc = "CCU Clock Control Register 7\n resetvalue={System Reset:0x0}"]
pub type Ccucon7 = crate::RegValueT<Ccucon7_SPEC>;

impl Ccucon7 {
    #[doc = "CPU1 Divider Reload Value   CPU1DIV. The resulting CPU1 frequency  performance  is configured to f CPU1   xa0    xa0  f SRI   xa0    xa0  64  xa0    xa0 CPU1DIV   xa0    xa0 64. For CPU1DIV  xa0    xa0 000000 B   f CPU1   xa0    xa0  f SRI ."]
    #[inline(always)]
    pub fn cpu1div(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Ccucon7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Ccucon7_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ccucon7 {
    #[inline(always)]
    fn default() -> Ccucon7 {
        <crate::RegValueT<Ccucon7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccucon8_SPEC;
impl crate::sealed::RegSpec for Ccucon8_SPEC {
    type DataType = u32;
}
#[doc = "CCU Clock Control Register 8\n resetvalue={System Reset:0x0}"]
pub type Ccucon8 = crate::RegValueT<Ccucon8_SPEC>;

impl Ccucon8 {
    #[doc = "CPU2 Divider Reload Value   CPU2DIV. The resulting CPU2 frequency  performance  is configured to f CPU2   xa0    xa0  f SRI   xa0    xa0  64  xa0    xa0 CPU2DIV   xa0    xa0 64. For CPU2DIV  xa0    xa0 000000 B   f CPU2   xa0    xa0  f SRI ."]
    #[inline(always)]
    pub fn cpu2div(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Ccucon8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Ccucon8_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ccucon8 {
    #[inline(always)]
    fn default() -> Ccucon8 {
        <crate::RegValueT<Ccucon8_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccuconsm_SPEC;
impl crate::sealed::RegSpec for Ccuconsm_SPEC {
    type DataType = u32;
}
#[doc = "CCU Control Security Monitor Register\n resetvalue={System Reset:0x1F1E}"]
pub type Ccuconsm = crate::RegValueT<Ccuconsm_SPEC>;

impl Ccuconsm {
    #[doc = "BACK UP Clock Counter Divider Reload Value   BACKREL. This bit field defines the reload value for the BACK UP Clock Counter of the fast SPB        monitor. This bit field must not be changed while bit EN is set."]
    #[inline(always)]
    pub fn backrel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, Ccuconsm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, Ccuconsm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SPB Clock Counter Divider Reload Value   SPBREL. This bit field defines the reload value for SPB Clock Counter of the fast SPB monitor. This bit field must not be changed while bit EN is set."]
    #[inline(always)]
    pub fn spbrel(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, Ccuconsm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, Ccuconsm_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable   EN"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, ccuconsm::En, Ccuconsm_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,ccuconsm::En, Ccuconsm_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ccuconsm {
    #[inline(always)]
    fn default() -> Ccuconsm {
        <crate::RegValueT<Ccuconsm_SPEC> as RegisterValue<_>>::new(7966)
    }
}
pub mod ccuconsm {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Monitor trigger generation is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Monitor trigger generation is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chipid_SPEC;
impl crate::sealed::RegSpec for Chipid_SPEC {
    type DataType = u32;
}
#[doc = "Chip Identification Register\n resetvalue={System Reset:0x0,CFS Value:0x0}"]
pub type Chipid = crate::RegValueT<Chipid_SPEC>;

impl Chipid {
    #[doc = "Chip Revision Number   CHREV. This bit field indicates the revision number of the AurixPlus        Platform device. The value of this bit field is defined in the        product Data Sheet. Bits  3 0  are connected to the layer prog inputs  3 0 . Bits         3 0  are used to indicate the steps. These updates can be done with any        metal fix or FW ROM change. Bits  5 4  are hard wired on top level. Bits  5 4  define the        major silicon design steps  A  B  C  D  ... . These bits can be changed        only with a major redesign."]
    #[inline(always)]
    pub fn chrev(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Chipid_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Chipid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Chip Family   CHTEC. These bits indicate the product family and are changed only with a redesign. Hard wired on top level."]
    #[inline(always)]
    pub fn chtec(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, chipid::Chtec, Chipid_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x3,1,0,chipid::Chtec, Chipid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Chip Package   CHPK. These bits indicate the package. For further details refer to the Product Datasheet Updated by SSW from config sector. Use future variant codes for downconfigured silicon"]
    #[inline(always)]
    pub fn chpk(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, chipid::Chpk, Chipid_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,chipid::Chpk, Chipid_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Chip Product   CHID. These bits indicate the base product. For further details refer to the Product Datasheet Updated by SSW from config sector."]
    #[inline(always)]
    pub fn chid(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, chipid::Chid, Chipid_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0xf,1,0,chipid::Chid, Chipid_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emulation or ADAS Extension Available   EEA. Indicates if the emulation or ADAS extension hardware is available or not."]
    #[inline(always)]
    pub fn eea(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, chipid::Eea, Chipid_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1,1,0,chipid::Eea, Chipid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "µCode Version   UCODE. This bit field displays the Version X.Y of the flash µCode."]
    #[inline(always)]
    pub fn ucode(
        self,
    ) -> crate::common::RegisterField<17, 0x7f, 1, 0, u8, Chipid_SPEC, crate::common::RW> {
        crate::common::RegisterField::<17,0x7f,1,0,u8, Chipid_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Program Flash Size   FSIZE. This bit field indicates available program flash size for this device. For more details see Product Datasheet."]
    #[inline(always)]
    pub fn fsize(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, chipid::Fsize, Chipid_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xf,1,0,chipid::Fsize, Chipid_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Variant   VART. This bit field is used for variant identification. It is used to identify product variants with non standard temperature profile  max frequency  package pitch or customer feature sets. Note that variants do not include HSM availability  Flash Size or Emulation availability options which are handled by the SEC  FSIZE and EEA fields respectively. For coding details see Product Datasheet"]
    #[inline(always)]
    pub fn vart(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, Chipid_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x7,1,0,u8, Chipid_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Security Device Available   SEC. This bit field is updated by SSW from config sector. This bit field indicates whether the product has a Hardware Security Module"]
    #[inline(always)]
    pub fn sec(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, chipid::Sec, Chipid_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,chipid::Sec, Chipid_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Chipid {
    #[inline(always)]
    fn default() -> Chipid {
        <crate::RegValueT<Chipid_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod chipid {
    pub struct Chtec_SPEC;
    pub type Chtec = crate::EnumBitfieldStruct<u8, Chtec_SPEC>;
    impl Chtec {
        #[doc = "01 SAx TC2xx  C65 Product"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 SAx TC3xx  C40 Product"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Chpk_SPEC;
    pub type Chpk = crate::EnumBitfieldStruct<u8, Chpk_SPEC>;
    impl Chpk {
        #[doc = "0000 Bare Die"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "0011 TQFP100"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "0100 TQFP144"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "0101 LQFP176"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "0111 LFBGA292"]
        pub const CONST_77: Self = Self::new(7);
        #[doc = "1001 LFBGA516"]
        pub const CONST_99: Self = Self::new(9);
    }
    pub struct Chid_SPEC;
    pub type Chid = crate::EnumBitfieldStruct<u8, Chid_SPEC>;
    impl Chid {
        #[doc = "0010 SAx TC32xx"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "0011 SAx TC33xx"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "0101 SAx TC35xx"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "0110 SAx TC36xx"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "0111 SAx TC37xx"]
        pub const CONST_77: Self = Self::new(7);
        #[doc = "1000 SAx TC38xx"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "1001 SAx TC39xx"]
        pub const CONST_99: Self = Self::new(9);
    }
    pub struct Eea_SPEC;
    pub type Eea = crate::EnumBitfieldStruct<u8, Eea_SPEC>;
    impl Eea {
        #[doc = "0 EEC is not available   SAK TC3xxxU or SAK TC3xxxP"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 EEC is available   SAK TC3xxxE or SAK TC3xxxF or SAK TC3xxxA or SAK TC3xxxB"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fsize_SPEC;
    pub type Fsize = crate::EnumBitfieldStruct<u8, Fsize_SPEC>;
    impl Fsize {
        #[doc = "0000 256 KByte Program Flash  SAx TC3xxx 4Fx"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "0001 0.5 MByte Program Flash  SAx TC3xxx 8Fx"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "0010 1.0 MByte Program Flash  SAx TC3xxx 16Fx"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "0011 1.5 MByte Program Flash  SAx TC3xxx 24Fx"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "0100 2.0 MByte Program Flash  SAx TC3xxx 32Fx"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "0101 2.5 MByte Program Flash  SAx TC3xxx 40Fx"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "0110 3.0 MByte Program Flash  SAx TC3xxx 48Fx"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "0111 4.0 MByte Program Flash  SAx TC3xxx 64Fx"]
        pub const CONST_77: Self = Self::new(7);
        #[doc = "1000 5.0 MByte Program Flash  SAx TC3xxx 80Fx"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "1001 6.0 MByte Program Flash  SAx TC3xxx 96Fx"]
        pub const CONST_99: Self = Self::new(9);
        #[doc = "1010 7.0 MByte Program Flash  SAx TC3xxx 112Fx"]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "1011 8.0 MByte Program Flash  SAx TC3xxx 128Fx"]
        pub const CONST_1111: Self = Self::new(11);
        #[doc = "1100 10.0MByteProgram Flash  SAx TC3xxx 160Fx"]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "1101 12.0MByteProgram Flash  SAx TC3xxx 192Fx"]
        pub const CONST_1313: Self = Self::new(13);
        #[doc = "1110 14.0MByteProgram Flash  SAx TC3xxx 224Fx"]
        pub const CONST_1414: Self = Self::new(14);
        #[doc = "1111 16.0MByteProgram Flash  SAx TC3xxx 256Fx"]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Sec_SPEC;
    pub type Sec = crate::EnumBitfieldStruct<u8, Sec_SPEC>;
    impl Sec {
        #[doc = "0 No Hardware Security Module"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Hardware Security Module is available"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkdivctrl0_SPEC;
impl crate::sealed::RegSpec for Clkdivctrl0_SPEC {
    type DataType = u32;
}
#[doc = "Clock Division Control Register 0\n resetvalue={System Reset:0x0}"]
pub type Clkdivctrl0 = crate::RegValueT<Clkdivctrl0_SPEC>;

impl Clkdivctrl0 {
    #[doc = "Counter Enable   CNTEN. This field specifies if the CCU clock counter logic for DfT   Test is enabled for internal frequency measurement."]
    #[inline(always)]
    pub fn cnten(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        clkdivctrl0::Cnten,
        Clkdivctrl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            clkdivctrl0::Cnten,
            Clkdivctrl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Trigger Selection   TRIGSEL. This field allows to select between various trigger signals  defining start and stop time for the next counter measurement period."]
    #[inline(always)]
    pub fn trigsel(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1f,
        1,
        0,
        clkdivctrl0::Trigsel,
        Clkdivctrl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1f,
            1,
            0,
            clkdivctrl0::Trigsel,
            Clkdivctrl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "SMON3 Pre Division Factor Selection   SMONDIV. This bit directly selects the pre division factor for the ring oscillation output frequency of all SMON3 macro cells  Reg VT and High VT . If TEST TRC4.SMONSEL lt 1 or 2 gt  is set to a different value than  0000  this bit has no meaning  because in this case TCU directly selects the SMON3 output division factor via TEST TRC4.SMONDIV"]
    #[inline(always)]
    pub fn smondiv(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        clkdivctrl0::Smondiv,
        Clkdivctrl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            clkdivctrl0::Smondiv,
            Clkdivctrl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Output Enabling Protection   OUTENP. Protection bit for automatic output enabling in case of TRIGSEL   11111"]
    #[inline(always)]
    pub fn outenp(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        clkdivctrl0::Outenp,
        Clkdivctrl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            clkdivctrl0::Outenp,
            Clkdivctrl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Output Select for PLL Clock Divider   CLKDIVSEL. This bit allows to select  which of PLL divider counter inside CCU will be forwarded to vtp24   TDO GPIO in case of continuous counting mode. TRIGSEL  11111 ."]
    #[inline(always)]
    pub fn clkdivsel(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x3,
        1,
        0,
        clkdivctrl0::Clkdivsel,
        Clkdivctrl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x3,
            1,
            0,
            clkdivctrl0::Clkdivsel,
            Clkdivctrl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Counter Output Selection   CNTSEL. This register selects  which counter value is readable through the CLKDIVCTRL1.COUNTVAL bitfield  Note  Please be aware  that after changing this bit field it will take some clock cycles before CLKDIVCTRL1.COUNTVAL is updated."]
    #[inline(always)]
    pub fn cntsel(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        clkdivctrl0::Cntsel,
        Clkdivctrl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            clkdivctrl0::Cntsel,
            Clkdivctrl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Start CCU Clock Counting   START. If written high this bit triggers the automatic accumulation of PLL clock pulses during a specific period  defined through TRIGSEL . After each period counter will restart until the STOP bit has been set. If START  and STOP bits are set to high concurrently  counter will run for exactly one TRIGSEL period. When written with a high value  this bit will reset automatically as soon counter active signal from CCU has been set to high. As long as CLKDIVCTRL1.BUSY is high  any write access to this bit is ignored. In case of continuous counting  TRIGSEL  11111   this bit will have no influence. To avoid transitory configuration values in CCU while counter is active  TRIGSEL should not be changed concurrently while setting this bit to  1 ."]
    #[inline(always)]
    pub fn start(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Clkdivctrl0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,Clkdivctrl0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Stop CCU Clock Counting   STOP. If written high this bit stops the automatic accumulation of PLL clock pulses after occurrence of the next trigger signal  defined through TRIGSEL. After written with a high value  this bit will be synchronously reset in case CLKDIVCTRL1.BUSY  and START bits are both low  i.e.  If START  and STOP bits are set to  1  concurrently while counter is idle  the automatic reset of STOP must not happen before counter has been active for one TRIGSEL period . In case of continuous counting  TRIGSEL  1111  this bit will have no influence."]
    #[inline(always)]
    pub fn stop(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Clkdivctrl0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Clkdivctrl0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Speedmon   SMONCTR. These registers enable and control the SMON3 Ring Oscillator macro cells for all reg VT and high VT types. If TEST TRC4.SMONSEL lt 1 or 2 gt  is set to a different value than  0000  these bits have no meaning  because TCU directly controls the SMON cells via TEST TRC4.SMONCTR in that case."]
    #[inline(always)]
    pub fn smonctr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x7,
        1,
        0,
        clkdivctrl0::Smonctr,
        Clkdivctrl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x7,
            1,
            0,
            clkdivctrl0::Smonctr,
            Clkdivctrl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Pre scaling factor   PRESCALE. This register allows to determine the pre scaling factor for the selected trigger source. Please regard that the pre scaler control vector consists of 14 bits from which only the 13 MSBs are controllable through this register. Bit  0 is not controllable and always tied to  0 .The following settings are supported"]
    #[inline(always)]
    pub fn prescale(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1fff,
        1,
        0,
        clkdivctrl0::Prescale,
        Clkdivctrl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1fff,
            1,
            0,
            clkdivctrl0::Prescale,
            Clkdivctrl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Clkdivctrl0 {
    #[inline(always)]
    fn default() -> Clkdivctrl0 {
        <crate::RegValueT<Clkdivctrl0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod clkdivctrl0 {
    pub struct Cnten_SPEC;
    pub type Cnten = crate::EnumBitfieldStruct<u8, Cnten_SPEC>;
    impl Cnten {
        #[doc = "0 Counter disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Counter enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trigsel_SPEC;
    pub type Trigsel = crate::EnumBitfieldStruct<u8, Trigsel_SPEC>;
    impl Trigsel {
        #[doc = "00000 Reg VT 40nm SPEEDMON Output clock."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "00001 Reg VT 50nm SPEEDMON Output clock."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "00010 High VT 40 nm SPEEDMON Output clock."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "00011 Reg VT 40nm SPEEDMON Output clock."]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "01001 Crack Sensor Ringo Path 1 output clock. Using this selection will automatically enable the crack sensor ringo selecting path 1."]
        pub const CONST_99: Self = Self::new(9);
        #[doc = "01010 Crack Sensor Ringo Path 2 output clock. Using this selection will automatically enable the crack sensor ringo selecting path 2."]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "01011 Fast 100 MHz EVR oscillator clock output."]
        pub const CONST_1111: Self = Self::new(11);
        #[doc = "01100 Slow 70 kHz EVR oscillator clock output."]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "01101 Flash Oscillator  10 20 Mhz at  flashosc tst o  Flash test out"]
        pub const CONST_1313: Self = Self::new(13);
        #[doc = "10000 XTAL1 input clock."]
        pub const CONST_1616: Self = Self::new(16);
        #[doc = "10001 XORed input path from XTAL1 and vtp25."]
        pub const CONST_1717: Self = Self::new(17);
        #[doc = "10010 P02.1  vtp25  input path  Slow pad"]
        pub const CONST_1818: Self = Self::new(18);
        #[doc = "10011 P2.4  vtp11  input path  Fast pad"]
        pub const CONST_1919: Self = Self::new(19);
        #[doc = "10100 P21.4 input path  LVDSH pad ."]
        pub const CONST_2020: Self = Self::new(20);
        #[doc = "11111 Continuous counting. Selecting continuous counting  will automatically make the PLL divider output visible at vtp24  TDO GPIO and switch vtp23 to input if OUTENP bit is set to high and chip is operating in test mode or ATM. This is achieved by indicating the continuous clocking mode and the active protection bit value from SCU to TCU  via  plldiv out en req  signal . Here the corresponding test functionality of vtp24 TDO GPIO and vtp23 will be directly enabled in this case."]
        pub const CONST_3131: Self = Self::new(31);
    }
    pub struct Smondiv_SPEC;
    pub type Smondiv = crate::EnumBitfieldStruct<u8, Smondiv_SPEC>;
    impl Smondiv {
        #[doc = "0 SMON3 output is divided by 1024."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SMON3 output is divided by 16."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Outenp_SPEC;
    pub type Outenp = crate::EnumBitfieldStruct<u8, Outenp_SPEC>;
    impl Outenp {
        #[doc = "0 Vtp24 TDO GPIO stays under functional GPIO control if continuous clocking mode is selected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clock output will be automatically forwarded to vtp24   TDO GPIO if continuous clocking mode is selected and chip is operating in test mode or ATM."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clkdivsel_SPEC;
    pub type Clkdivsel = crate::EnumBitfieldStruct<u8, Clkdivsel_SPEC>;
    impl Clkdivsel {
        #[doc = "00 Bit  3 is forwarded."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Bit  5 is forwarded."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Bit  9 is forwarded."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Bit  10 is forwarded."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Cntsel_SPEC;
    pub type Cntsel = crate::EnumBitfieldStruct<u8, Cntsel_SPEC>;
    impl Cntsel {
        #[doc = "00 The value of normal divider counter is forwarded."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 The min counter value register is forwarded."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 The max counter value register is forwarded."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 The value of normal divider counter is forwarded."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Smonctr_SPEC;
    pub type Smonctr = crate::EnumBitfieldStruct<u8, Smonctr_SPEC>;
    impl Smonctr {
        #[doc = "000 SMON3 output is inactive."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 SMON3 behaves as INV RingO  internal   IFX inv. PCM RingO ."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 SMON3 behaves as NAND2 RingO  internal ."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 SMON3 behaves as NOR2 RingO  internal ."]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 SMON3 activates 1 st external RingO  design monitor 1 ."]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 SMON3 activates 2 nd external RingO  design monitor 2 or TOX RingO ."]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 SMON3 activates 3 rd external RingO  voltage monitor satellite 1 ."]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "111 SMON3 activates 4 th external RingO  voltage monitor satellite 2 ."]
        pub const CONST_77: Self = Self::new(7);
    }
    pub struct Prescale_SPEC;
    pub type Prescale = crate::EnumBitfieldStruct<u16, Prescale_SPEC>;
    impl Prescale {
        #[doc = "0 0000 0000 0000 No division  trigger signal is bypassed ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "0 0000 0000 0001 Trigger signal is divided by 2."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "0 0000 0000 0010 Trigger signal is divided by 4."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "1 1111 1111 1110 Trigger signal is divided by 16380."]
        pub const CONST_81908190: Self = Self::new(8190);
        #[doc = "1 1111 1111 1111 Trigger signal is divided by 16382."]
        pub const CONST_81918191: Self = Self::new(8191);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkdivctrl1_SPEC;
impl crate::sealed::RegSpec for Clkdivctrl1_SPEC {
    type DataType = u32;
}
#[doc = "Clock Division Control Register 1\n resetvalue={System Reset:0x0}"]
pub type Clkdivctrl1 = crate::RegValueT<Clkdivctrl1_SPEC>;

impl Clkdivctrl1 {
    #[doc = "Counter Busy Indicator   BUSY. This bit indicates the actual counter status. Once a accumulation is triggered via START  this bit will remain high until counter active signal from TCU has been set to high and low again."]
    #[inline(always)]
    pub fn busy(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        clkdivctrl1::Busy,
        Clkdivctrl1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            clkdivctrl1::Busy,
            Clkdivctrl1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Counter Value   COUNTVAL. Depending on the CLKDIVCTRL0.CNTSEL programming. This field reflects either the actual  min or max value of the 20 bit CCU counter. The values can only be considered as valid  if BUSY bit is low. First PLL pulse  resp. first toggle of clock clipping indicator  within the trigger period will always be used to reset the counter. Consequently the counter value will reflect one pulse less than occurred in reality."]
    #[inline(always)]
    pub fn countval(
        self,
    ) -> crate::common::RegisterField<1, 0xfffff, 1, 0, u32, Clkdivctrl1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0xfffff,1,0,u32, Clkdivctrl1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Clkdivctrl1 {
    #[inline(always)]
    fn default() -> Clkdivctrl1 {
        <crate::RegValueT<Clkdivctrl1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod clkdivctrl1 {
    pub struct Busy_SPEC;
    pub type Busy = crate::EnumBitfieldStruct<u8, Busy_SPEC>;
    impl Busy {
        #[doc = "0 Counter is currently inactive. Values in COUNTVAL are valid."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A counting operation is currently ongoing. To avoid corrupted measurement results the counter settings  e.g. CLKDIVSEL  in CLKDIVCTRL0 register shall not be changed in this case."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtscbgoctrl_SPEC;
impl crate::sealed::RegSpec for Dtscbgoctrl_SPEC {
    type DataType = u32;
}
#[doc = "Core Die Temperature Sensor Bandgap Control Register\n resetvalue={Cold PowerOn Reset:0x40,CFS Value:0x40}"]
pub type Dtscbgoctrl = crate::RegValueT<Dtscbgoctrl_SPEC>;

impl Dtscbgoctrl {
    #[doc = "DTSC Bandgap Trim Value   VBGTRIM. This bit field contains information about trimming of the DTSC        bandgap.The Bandgap voltage is measured via Analog Test Bus at 125  176 C and        the value is trimmed so that the measurement corresponds to 1.13V. A        settling time of   30  160 us need to be waited after a new trim value is        programmed.  trimvpbg  trimbg i    VBGTRIM   VBGPTRIM  signed value"]
    #[inline(always)]
    pub fn vbgtrim(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Dtscbgoctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8, Dtscbgoctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DTSC Bandgap Signed Trim Value   VBGPTRIM. This bit field allows device individual trimming of the bandgap trim        value in production. The first bit is sign information. The range is  16        to 15."]
    #[inline(always)]
    pub fn vbgptrim(
        self,
    ) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, Dtscbgoctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x1f,1,0,u8, Dtscbgoctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dtscbgoctrl {
    #[inline(always)]
    fn default() -> Dtscbgoctrl {
        <crate::RegValueT<Dtscbgoctrl_SPEC> as RegisterValue<_>>::new(64)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtsccon_SPEC;
impl crate::sealed::RegSpec for Dtsccon_SPEC {
    type DataType = u32;
}
#[doc = "Core Die Temperature Sensor Control Register\n resetvalue={Cold PowerOn Reset:0x200,CFS Value:0x200}"]
pub type Dtsccon = crate::RegValueT<Dtsccon_SPEC>;

impl Dtsccon {
    #[doc = "DTSC Gain Trim Value  gtrim . This bit field contains information about gain trimming of the core die        temperature sensor  DTSC . GAINTRIM influences the length of integration        time where DTSC pulses are counted. A settling time of x  160 us is required        after update."]
    #[inline(always)]
    pub fn gaintrim(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Dtsccon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Dtsccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DTSC Offset Trim Value  otrim . This bit field contains information about trimming of the die        temperature sensor. DTSCRESULT   DTSCRESULTANA   OFFSETTRIM DTSC"]
    #[inline(always)]
    pub fn offsettrim(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, Dtsccon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xfff,1,0,u16, Dtsccon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dtsccon {
    #[inline(always)]
    fn default() -> Dtsccon {
        <crate::RegValueT<Dtsccon_SPEC> as RegisterValue<_>>::new(512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtsclim_SPEC;
impl crate::sealed::RegSpec for Dtsclim_SPEC {
    type DataType = u32;
}
#[doc = "Core Die Temperature Sensor Limit Register\n resetvalue={Application Reset:0x0CD806D6}"]
pub type Dtsclim = crate::RegValueT<Dtsclim_SPEC>;

impl Dtsclim {
    #[doc = "DTSC Lower Limit   LOWER. This bit field defines the lower limit of the DTSC temperature check.        The DTSC measurement result is compared against this value and if the        measurement result is less than or equal to the configured LOWER        bitfield value  flag LLU is set."]
    #[inline(always)]
    pub fn lower(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, Dtsclim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16, Dtsclim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DTSC Bandgap OK. This bitfield indicates that the bandgap reference for the Core Die        Temperature Sensor  DTSC  is available and ok."]
    #[inline(always)]
    pub fn bgpok(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, dtsclim::Bgpok, Dtsclim_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<13,0x1,1,0,dtsclim::Bgpok, Dtsclim_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DTSC Enable. This bitfield enables the Core Die Temperature Sensor  DTSC . The        bitfield is reset on an application reset."]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, dtsclim::En, Dtsclim_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,dtsclim::En, Dtsclim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DTSC Lower Limit Underflow   LLU. When this bit is set the related SMU DTSC alarm trigger is generated.        This bit has to be written with zero in order to clear it. Writing a one        has no effect. This bit is set when a DTSC measurement is finished and        the result is below the lower limit  i.e. DTSCLIM.LOWER ."]
    #[inline(always)]
    pub fn llu(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, dtsclim::Llu, Dtsclim_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,dtsclim::Llu, Dtsclim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DTSC Upper Limit   UPPER. This bit field defines the upper limit of the DTSC temperature check.        The DTSC measurement result is compared against this value and if the        measurement result is greater than or equal to the configured UPPER        bitfield value  flag UOF is set."]
    #[inline(always)]
    pub fn upper(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, Dtsclim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xfff,1,0,u16, Dtsclim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DTSC Interrupt Enable. This bitfield enables the Core Die Temperature Sensor  DTSC  interrupt.        The bitfield is reset on an application reset."]
    #[inline(always)]
    pub fn inten(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, dtsclim::Inten, Dtsclim_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,dtsclim::Inten, Dtsclim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DTSC Interrupt status flag. This bit is set when SMU DTSC interrupt is generated when a DTSC        measurement is finished. This bit is cleared by writing a zero. Writing        a one has no effect."]
    #[inline(always)]
    pub fn int(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, dtsclim::Int, Dtsclim_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,dtsclim::Int, Dtsclim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DTSC Upper Limit Overflow   UOF. When this bit is set  the related SMU DTSC alarm trigger is generated.        This bit has to be written with zero in order to clear it. Writing a one        has no effect. This bit is set when a DTSC measurement is finished and        the result is exceeding the upper limit  i.e. DTSCLIM.UPPER ."]
    #[inline(always)]
    pub fn uof(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, dtsclim::Uof, Dtsclim_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,dtsclim::Uof, Dtsclim_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dtsclim {
    #[inline(always)]
    fn default() -> Dtsclim {
        <crate::RegValueT<Dtsclim_SPEC> as RegisterValue<_>>::new(215484118)
    }
}
pub mod dtsclim {
    pub struct Bgpok_SPEC;
    pub type Bgpok = crate::EnumBitfieldStruct<u8, Bgpok_SPEC>;
    impl Bgpok {
        #[doc = "0 DTSC Bandgap is        not ok."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 DTSC Bandgap is        ok."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 DTSC is        disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 DTSC is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Llu_SPEC;
    pub type Llu = crate::EnumBitfieldStruct<u8, Llu_SPEC>;
    impl Llu {
        #[doc = "0 No temperature        underflow was detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A temperature        underflow was detected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Inten_SPEC;
    pub type Inten = crate::EnumBitfieldStruct<u8, Inten_SPEC>;
    impl Inten {
        #[doc = "0 DTSC Interrupt        is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 DTSC Interrupt        is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Int_SPEC;
    pub type Int = crate::EnumBitfieldStruct<u8, Int_SPEC>;
    impl Int {
        #[doc = "0 No DTSC        interrupt is generated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 DTSC interrupt        is generated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Uof_SPEC;
    pub type Uof = crate::EnumBitfieldStruct<u8, Uof_SPEC>;
    impl Uof {
        #[doc = "0 No temperature        overflow was detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A temperature        overflow was detected"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtscstat_SPEC;
impl crate::sealed::RegSpec for Dtscstat_SPEC {
    type DataType = u32;
}
#[doc = "Core Die Temperature Sensor Status Register\n resetvalue={Application Reset:0x0}"]
pub type Dtscstat = crate::RegValueT<Dtscstat_SPEC>;

impl Dtscstat {
    #[doc = "Result of the DTSC Measurement   RESULT. This bit field shows the result of the DTSC measurement. The value given        is directly related to the die temperature and can be evaluated using        the following formula. T    176 C     RESULT   Gnom    273.15 T   176 K   RESULT  G nom RESULT  G nom    T    176 C    273.15    G nom   T    176 K  G nom   7.505"]
    #[inline(always)]
    pub fn result(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, Dtscstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xfff,1,0,u16, Dtscstat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Dtscstat {
    #[inline(always)]
    fn default() -> Dtscstat {
        <crate::RegValueT<Dtscstat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eicon0_SPEC;
impl crate::sealed::RegSpec for Eicon0_SPEC {
    type DataType = u32;
}
#[doc = "ENDINIT Global Control Register 0\n resetvalue={Application Reset:0x0FFFC000E}"]
pub type Eicon0 = crate::RegValueT<Eicon0_SPEC>;

impl Eicon0 {
    #[doc = "End of Initialization Control Bit   ENDINIT. The current value of ENDINIT is controlled by hardware. It is cleared        after a valid EndInit Password Access to EICON0  and it is automatically        set again after a valid EndInit Modify Access to EICON0. During a write        to EICON0  the value written to this bit is only used for the        password protection mechanism and is not stored. This bit must be cleared during a Password Access to EICON0  and set        during a Modify Access to EICON0."]
    #[inline(always)]
    pub fn endinit(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, eicon0::Endinit, Eicon0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,eicon0::Endinit, Eicon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "User Definable ENDINIT Password Field   EPW. This bit field is written with an ENDINIT password value during a Modify        Access. This password is independent from the CPU WDT passwords. A read from this bitfield returns this password  but bits  7 2  are        inverted  toggled  to ensure that a simple read write is not sufficient        to service the WDT. This bit field must be written with its current contents during a        Password Access. The default ENDINIT password after Application Reset is 00000000111100 B"]
    #[inline(always)]
    pub fn epw(
        self,
    ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, Eicon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3fff,1,0,u16, Eicon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reload Value for the ENDINIT Timeout Counter   REL. The reload value for the ENDINIT Timeout Counter is fixed. This bitfield        always reads as FFFCh and cannot be changed. This bit field must be written with its current contents during a        Password Access. During a Modify Access this bitfield may contain any        value and is ignored."]
    #[inline(always)]
    pub fn rel(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Eicon0_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Eicon0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Eicon0 {
    #[inline(always)]
    fn default() -> Eicon0 {
        <crate::RegValueT<Eicon0_SPEC> as RegisterValue<_>>::new(4294705166)
    }
}
pub mod eicon0 {
    pub struct Endinit_SPEC;
    pub type Endinit = crate::EnumBitfieldStruct<u8, Endinit_SPEC>;
    impl Endinit {
        #[doc = "Access to Endinit protected registers is permitted  default        after Application Reset"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Access to Endinit protected registers is not permitted unless one of        WDTCPUyCON0.ENDINIT is 0."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eicon1_SPEC;
impl crate::sealed::RegSpec for Eicon1_SPEC {
    type DataType = u32;
}
#[doc = "ENDINIT Global Control Register 1\n resetvalue={Application Reset:0x0}"]
pub type Eicon1 = crate::RegValueT<Eicon1_SPEC>;

impl Eicon1 {
    #[doc = "Input Frequency Request Control   IR1 IR0. Bit IR0 and IR1 should be programmed together to determine the ENDINIT        Timeout Counter frequency. These bits can only be modified if system ENDINIT  E  is de asserted.        EISR.IS0 and EISR.IS1 are updated by these bits only when system ENDINIT         E  is re asserted. As long as system ENDINIT  E  is de asserted         EISR.IS0 and EISR.IS1 control the current input frequency of the ENDINIT        Timeout Timer. When System ENDINIT  E  is re asserted  EISR.IS0 and        EISR.IS1 are updated with the new values of IR0 and IR1."]
    #[inline(always)]
    pub fn ir0(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, eicon1::Ir0, Eicon1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,eicon1::Ir0, Eicon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disable Request Control Bit   DR. This bit can only be modified if the system ENDINIT  E  is de asserted.        EISR.DS is updated when system ENDINIT  E  is re asserted. As long as        system ENDINIT E  is cleared  bit EISR.DS controls the current        enable disable status of the ENDINIT Timeout Counter. When system        ENDINIT  E  is re asserted  EISR.DS is updated with the state of DR."]
    #[inline(always)]
    pub fn dr(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, eicon1::Dr, Eicon1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,eicon1::Dr, Eicon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Frequency Request Control   IR1 IR0. Bit IR0 and IR1 should be programmed together to determine the ENDINIT        Timeout Counter frequency. These bits can only be modified if system ENDINIT  E  is de asserted.        EISR.IS0 and EISR.IS1 are updated by these bits only when system ENDINIT         E  is re asserted. As long as system ENDINIT  E  is de asserted         EISR.IS0 and EISR.IS1 control the current input frequency of the ENDINIT        Timeout Timer. When System ENDINIT  E  is re asserted  EISR.IS0 and        EISR.IS1 are updated with the new values of IR0 and IR1."]
    #[inline(always)]
    pub fn ir1(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, eicon1::Ir1, Eicon1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,eicon1::Ir1, Eicon1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Eicon1 {
    #[inline(always)]
    fn default() -> Eicon1 {
        <crate::RegValueT<Eicon1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eicon1 {
    pub struct Ir0_SPEC;
    pub type Ir0 = crate::EnumBitfieldStruct<u8, Ir0_SPEC>;
    impl Ir0 {
        #[doc = "If Bit IR1 0 Request to set input frequency to f SPB  16384.        Elseif Bit IR1 1 Request to set input frequency to f SPB  64."]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Dr_SPEC;
    pub type Dr = crate::EnumBitfieldStruct<u8, Dr_SPEC>;
    impl Dr {
        #[doc = "0 Request to        enable the ENDINIT Timeout Counter"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Request to        disable the ENDINIT Timeout counter"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ir1_SPEC;
    pub type Ir1 = crate::EnumBitfieldStruct<u8, Ir1_SPEC>;
    impl Ir1 {
        #[doc = "If Bit IR0 0 Request to set input frequency to f SPB  16384.        Elseif Bit IR0 1 Request to set input frequency to f SPB  256."]
        pub const CONST_00: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EicRi_SPEC;
impl crate::sealed::RegSpec for EicRi_SPEC {
    type DataType = u32;
}
#[doc = "External Input Channel Register 0\n resetvalue={Application Reset:0x0}"]
pub type EicRi = crate::RegValueT<EicRi_SPEC>;

impl EicRi {
    #[doc = "External Input Selection 0   EXIS0. This bit field determines which input line is selected for Input Channel  2i ."]
    #[inline(always)]
    pub fn exis0(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, eicri::Exis0, EicRi_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x7,1,0,eicri::Exis0, EicRi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Falling Edge Enable 0   FEN0. This bit determines if the falling edge of Input Channel  2i  is used to set bit INTF 2i ."]
    #[inline(always)]
    pub fn fen0(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, eicri::Fen0, EicRi_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,eicri::Fen0, EicRi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Rising Edge Enable 0   REN0. This bit determines if the rising edge of Input Channel  2 i  is used to set bit INTF 2i ."]
    #[inline(always)]
    pub fn ren0(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, eicri::Ren0, EicRi_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,eicri::Ren0, EicRi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Level Detection Enable 0   LDEN0. This bit determines if bit INTF 2i  is cleared automatically if an edge of the input Input Channel  2i  is detected  which has not been selected  rising edge with REN0   0 or falling edge with FEN0   0 ."]
    #[inline(always)]
    pub fn lden0(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, eicri::Lden0, EicRi_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,eicri::Lden0, EicRi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "External Input Enable 0   EIEN0. This bit enables the generation of a trigger event for request channel  2i   e.g. for interrupt generation  when a selected edge is detected."]
    #[inline(always)]
    pub fn eien0(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, eicri::Eien0, EicRi_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,eicri::Eien0, EicRi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Node Pointer   INP0. This bit field determines the destination  output channel  for trigger event  2i   if enabled by EIEN 2i  ."]
    #[inline(always)]
    pub fn inp0(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, eicri::Inp0, EicRi_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x7,1,0,eicri::Inp0, EicRi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "External Input Selection 1   EXIS1. This bit field determines which input line is selected for Input Channel  2i 1 ."]
    #[inline(always)]
    pub fn exis1(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, eicri::Exis1, EicRi_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x7,1,0,eicri::Exis1, EicRi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Falling Edge Enable 1   FEN1. This bit determines if the falling edge of Input Channel  2i 1  is used to set bit INTF 2i 1 ."]
    #[inline(always)]
    pub fn fen1(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, eicri::Fen1, EicRi_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,eicri::Fen1, EicRi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Rising Edge Enable 1   REN1. This bit determines if the rising edge of Input Channel  2i 1  is used to set bit INTF 2i 1 ."]
    #[inline(always)]
    pub fn ren1(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, eicri::Ren1, EicRi_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,eicri::Ren1, EicRi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Level Detection Enable 1   LDEN1. This bit determines if bit INTF 2i 1  is cleared automatically if an edge of the input Input Channel  2i 1  is detected  which has not been selected  rising edge with REN1   0 or falling edge with FEN1   0 ."]
    #[inline(always)]
    pub fn lden1(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, eicri::Lden1, EicRi_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,eicri::Lden1, EicRi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "External Input Enable 1   EIEN1. This bit enables the generation of a trigger event for request channel  2i 1   e.g. for interrupt generation  when a selected edge is detected."]
    #[inline(always)]
    pub fn eien1(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, eicri::Eien1, EicRi_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,eicri::Eien1, EicRi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Node Pointer   INP1. This bit field determines the destination  output channel  for trigger event  2i 1   if enabled by EIEN 2i 1  ."]
    #[inline(always)]
    pub fn inp1(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, eicri::Inp1, EicRi_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x7,1,0,eicri::Inp1, EicRi_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for EicRi {
    #[inline(always)]
    fn default() -> EicRi {
        <crate::RegValueT<EicRi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eicri {
    pub struct Exis0_SPEC;
    pub type Exis0 = crate::EnumBitfieldStruct<u8, Exis0_SPEC>;
    impl Exis0 {
        #[doc = "000 Input  2i  A is selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 Input  2i  B is selected"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 Input  2i  Cis selected"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 Input  2i  Dis selected"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 Input  2i  E is selected"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 Input  2i  F is selected"]
        pub const CONST_55: Self = Self::new(5);
    }
    pub struct Fen0_SPEC;
    pub type Fen0 = crate::EnumBitfieldStruct<u8, Fen0_SPEC>;
    impl Fen0 {
        #[doc = "0 The falling edge is not used"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The detection of a falling edge of Input Channel 0 generates a trigger event. INTF 2i  becomes set."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ren0_SPEC;
    pub type Ren0 = crate::EnumBitfieldStruct<u8, Ren0_SPEC>;
    impl Ren0 {
        #[doc = "0 The rising edge is not used"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The detection of a rising edge of Input Channel  2 i  generates a trigger event. INTF 2 i  becomes set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lden0_SPEC;
    pub type Lden0 = crate::EnumBitfieldStruct<u8, Lden0_SPEC>;
    impl Lden0 {
        #[doc = "0 Bit INTF 2i  will not be cleared"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit INTF 2i  will be cleared"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eien0_SPEC;
    pub type Eien0 = crate::EnumBitfieldStruct<u8, Eien0_SPEC>;
    impl Eien0 {
        #[doc = "0 The trigger event is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The trigger event is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Inp0_SPEC;
    pub type Inp0 = crate::EnumBitfieldStruct<u8, Inp0_SPEC>;
    impl Inp0 {
        #[doc = "000 An event from input ETL 2i triggers output OGU0  signal TR 2i  0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 An event from input ETL 2i triggers output OGU1  signal TR 2i  1"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 An event from input ETL 2i triggers output OGU2  signal TR 2i  2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 An event from input ETL 2i triggers output OGU3  signal TR 2i  3"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 An event from input ETL 2i triggers output OGU4  signal TR 2i  0"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 An event from input ETL 2i triggers output OGU5  signal TR 2i  0"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 An event from input ETL 2i triggers output OGU6  signal TR 2i  0"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "111 An event from input ETL 2i triggers output OGU7  signal TR 2i  0"]
        pub const CONST_77: Self = Self::new(7);
    }
    pub struct Exis1_SPEC;
    pub type Exis1 = crate::EnumBitfieldStruct<u8, Exis1_SPEC>;
    impl Exis1 {
        #[doc = "000 Input  2i 1  A is selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 Input  2i 1  B is selected"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 Input  2i 1  C is selected"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 Input  2i 1  D is selected"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 Input  2i 1  E is selected"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 Input  2i 1  F is selected"]
        pub const CONST_55: Self = Self::new(5);
    }
    pub struct Fen1_SPEC;
    pub type Fen1 = crate::EnumBitfieldStruct<u8, Fen1_SPEC>;
    impl Fen1 {
        #[doc = "0 The falling edge is not used"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The detection of a falling edge of Input Channel 1 generates a trigger event. INTF 2i 1  becomes set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ren1_SPEC;
    pub type Ren1 = crate::EnumBitfieldStruct<u8, Ren1_SPEC>;
    impl Ren1 {
        #[doc = "0 The rising edge is not used"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The detection of a rising edge of Input Channel 1 generates a trigger event . INTF 2i 1  becomes set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lden1_SPEC;
    pub type Lden1 = crate::EnumBitfieldStruct<u8, Lden1_SPEC>;
    impl Lden1 {
        #[doc = "0 Bit INTF 2i 1  will not be cleared"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit INTF1 2i 1  will be cleared"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eien1_SPEC;
    pub type Eien1 = crate::EnumBitfieldStruct<u8, Eien1_SPEC>;
    impl Eien1 {
        #[doc = "0 The trigger event is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The trigger event is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Inp1_SPEC;
    pub type Inp1 = crate::EnumBitfieldStruct<u8, Inp1_SPEC>;
    impl Inp1 {
        #[doc = "000 An event from input ETL 2i 1 triggers output OGU0  signal TR 2i 1  0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 An event from input ETL 2i 1 triggers output OGU1  signal TR 2i 1   1"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 An event from input ETL 2i 1 triggers output OGU2  signal TR 2i 1   2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 An event from input ETL 2i 1 triggers output OGU3  signal TR 2i 1   3"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 An event from input ETL 2i 1 triggers output OGU4  signal TR 2i 1   0"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 An event from input ETL 2i 1 triggers output OGU5  signal TR 2i 1   0"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 An event from input ETL 2i 1 triggers output OGU6  signal TR 2i 1   0"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "111 An event from input ETL 2i 1 triggers output OGU7  signal TR 2i 1   0"]
        pub const CONST_77: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eifilt_SPEC;
impl crate::sealed::RegSpec for Eifilt_SPEC {
    type DataType = u32;
}
#[doc = "External Input Filter Register\n resetvalue={Application Reset:0x0}"]
pub type Eifilt = crate::RegValueT<Eifilt_SPEC>;

impl Eifilt {
    #[doc = "Filter Enable for REQ0A   FILRQ0A"]
    #[inline(always)]
    pub fn filrq0a(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, eifilt::Filrq0A, Eifilt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,eifilt::Filrq0A, Eifilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Filter Enable for REQ5A   FILRQ5A"]
    #[inline(always)]
    pub fn filrq5a(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, eifilt::Filrq5A, Eifilt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,eifilt::Filrq5A, Eifilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Filter Enable for REQ2A   FILRQ2A"]
    #[inline(always)]
    pub fn filrq2a(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, eifilt::Filrq2A, Eifilt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,eifilt::Filrq2A, Eifilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Filter Enable for REQ3A   FILRQ3A"]
    #[inline(always)]
    pub fn filrq3a(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, eifilt::Filrq3A, Eifilt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,eifilt::Filrq3A, Eifilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Filter Enable for REQ0C   FILRQ0C"]
    #[inline(always)]
    pub fn filrq0c(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, eifilt::Filrq0C, Eifilt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,eifilt::Filrq0C, Eifilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Filter Enable for REQ1C   FILRQ1C"]
    #[inline(always)]
    pub fn filrq1c(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, eifilt::Filrq1C, Eifilt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,eifilt::Filrq1C, Eifilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Filter Enable for REQ3C   FILRQ3C"]
    #[inline(always)]
    pub fn filrq3c(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, eifilt::Filrq3C, Eifilt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,eifilt::Filrq3C, Eifilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Filter Enable for REQ2C   FILRQ2C"]
    #[inline(always)]
    pub fn filrq2c(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, eifilt::Filrq2C, Eifilt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,eifilt::Filrq2C, Eifilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Filter Enable for REQ4A   FILRQ4A"]
    #[inline(always)]
    pub fn filrq4a(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, eifilt::Filrq4A, Eifilt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,eifilt::Filrq4A, Eifilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Filter Enable for REQ6A   FILRQ6A"]
    #[inline(always)]
    pub fn filrq6a(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, eifilt::Filrq6A, Eifilt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,eifilt::Filrq6A, Eifilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Filter Enable for REQ1A   FILRQ1A"]
    #[inline(always)]
    pub fn filrq1a(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, eifilt::Filrq1A, Eifilt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,eifilt::Filrq1A, Eifilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Filter Enable for REQ7A   FILRQ7A"]
    #[inline(always)]
    pub fn filrq7a(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, eifilt::Filrq7A, Eifilt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,eifilt::Filrq7A, Eifilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Filter Enable for REQ6D   FILRQ6D"]
    #[inline(always)]
    pub fn filrq6d(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, eifilt::Filrq6D, Eifilt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,eifilt::Filrq6D, Eifilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Filter Enable for REQ4D   FILRQ4D"]
    #[inline(always)]
    pub fn filrq4d(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, eifilt::Filrq4D, Eifilt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,eifilt::Filrq4D, Eifilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Filter Enable for REQ2B   FILRQ2B"]
    #[inline(always)]
    pub fn filrq2b(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, eifilt::Filrq2B, Eifilt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,eifilt::Filrq2B, Eifilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Filter Enable for REQ3B   FILRQ3B"]
    #[inline(always)]
    pub fn filrq3b(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, eifilt::Filrq3B, Eifilt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,eifilt::Filrq3B, Eifilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Filter Enable for REQ7C   FILRQ7C"]
    #[inline(always)]
    pub fn filrq7c(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, eifilt::Filrq7C, Eifilt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,eifilt::Filrq7C, Eifilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Digital Glitch Filter Clock Predivider   FILTDIV. This field controls a predivider to generate the digital filter sample clock T filt   T spb   FILTDIV A value of zero in this register disables all glitch filtering."]
    #[inline(always)]
    pub fn filtdiv(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Eifilt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Eifilt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Digital Glitch Filter Depth   DEPTH. DEPTH determines the number of port input samples considered in the calculation of the floating average digital filter output for all enabled FLRQ filters. A value of zero in this register disables all glitch filtering."]
    #[inline(always)]
    pub fn depth(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Eifilt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Eifilt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Eifilt {
    #[inline(always)]
    fn default() -> Eifilt {
        <crate::RegValueT<Eifilt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod eifilt {
    pub struct Filrq0A_SPEC;
    pub type Filrq0A = crate::EnumBitfieldStruct<u8, Filrq0A_SPEC>;
    impl Filrq0A {
        #[doc = "0 REQ0A is unfiltered"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 REQ0A glitch filter is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Filrq5A_SPEC;
    pub type Filrq5A = crate::EnumBitfieldStruct<u8, Filrq5A_SPEC>;
    impl Filrq5A {
        #[doc = "0 REQ5Ais unfiltered"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 REQ5A glitch filter is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Filrq2A_SPEC;
    pub type Filrq2A = crate::EnumBitfieldStruct<u8, Filrq2A_SPEC>;
    impl Filrq2A {
        #[doc = "0 REQ2A is unfiltered"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 REQ2A glitch filter is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Filrq3A_SPEC;
    pub type Filrq3A = crate::EnumBitfieldStruct<u8, Filrq3A_SPEC>;
    impl Filrq3A {
        #[doc = "0 REQ3A is unfiltered"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 REQ3A glitch filter is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Filrq0C_SPEC;
    pub type Filrq0C = crate::EnumBitfieldStruct<u8, Filrq0C_SPEC>;
    impl Filrq0C {
        #[doc = "0 REQ0C is unfiltered"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 REQ0C glitch filter is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Filrq1C_SPEC;
    pub type Filrq1C = crate::EnumBitfieldStruct<u8, Filrq1C_SPEC>;
    impl Filrq1C {
        #[doc = "0 REQ1C is unfiltered"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 REQ1C glitch filter is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Filrq3C_SPEC;
    pub type Filrq3C = crate::EnumBitfieldStruct<u8, Filrq3C_SPEC>;
    impl Filrq3C {
        #[doc = "0 REQ3C is unfiltered"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 REQ3C glitch filter is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Filrq2C_SPEC;
    pub type Filrq2C = crate::EnumBitfieldStruct<u8, Filrq2C_SPEC>;
    impl Filrq2C {
        #[doc = "0 REQ2C is unfiltered"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 REQ2C glitch filter is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Filrq4A_SPEC;
    pub type Filrq4A = crate::EnumBitfieldStruct<u8, Filrq4A_SPEC>;
    impl Filrq4A {
        #[doc = "0 REQ4A is unfiltered"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 REQ4A glitch filter is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Filrq6A_SPEC;
    pub type Filrq6A = crate::EnumBitfieldStruct<u8, Filrq6A_SPEC>;
    impl Filrq6A {
        #[doc = "0 REQ6A is unfiltered"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 REQ6A glitch filter is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Filrq1A_SPEC;
    pub type Filrq1A = crate::EnumBitfieldStruct<u8, Filrq1A_SPEC>;
    impl Filrq1A {
        #[doc = "0 REQ1A is unfiltered"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 REQ1A glitch filter is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Filrq7A_SPEC;
    pub type Filrq7A = crate::EnumBitfieldStruct<u8, Filrq7A_SPEC>;
    impl Filrq7A {
        #[doc = "0 REQ7A is unfiltered"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 REQ7A glitch filter is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Filrq6D_SPEC;
    pub type Filrq6D = crate::EnumBitfieldStruct<u8, Filrq6D_SPEC>;
    impl Filrq6D {
        #[doc = "0 REQ6D is unfiltered"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 REQ6D glitch filter is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Filrq4D_SPEC;
    pub type Filrq4D = crate::EnumBitfieldStruct<u8, Filrq4D_SPEC>;
    impl Filrq4D {
        #[doc = "0 REQ4D is unfiltered"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 REQ4D glitch filter is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Filrq2B_SPEC;
    pub type Filrq2B = crate::EnumBitfieldStruct<u8, Filrq2B_SPEC>;
    impl Filrq2B {
        #[doc = "0 REQ2B is unfiltered"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 REQ2B glitch filter is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Filrq3B_SPEC;
    pub type Filrq3B = crate::EnumBitfieldStruct<u8, Filrq3B_SPEC>;
    impl Filrq3B {
        #[doc = "0 REQ3B is unfiltered"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 REQ3B glitch filter is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Filrq7C_SPEC;
    pub type Filrq7C = crate::EnumBitfieldStruct<u8, Filrq7C_SPEC>;
    impl Filrq7C {
        #[doc = "0 REQ7C is unfiltered"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 REQ7Cglitch filter is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eifr_SPEC;
impl crate::sealed::RegSpec for Eifr_SPEC {
    type DataType = u32;
}
#[doc = "External Input Flag Register\n resetvalue={Application Reset:0x0}"]
pub type Eifr = crate::RegValueT<Eifr_SPEC>;

impl Eifr {
    #[doc = "External Event Flag of Channel 7   INTF7. This bit monitors the status flag of the event trigger condition for the input channel x. This bit is automatically cleared when the selected condition  see RENx  FENx  is no longer met  if LDENx   1  or remains set until it is cleared by software  if LDENx   0 ."]
    #[inline(always)]
    pub fn intf0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Eifr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Eifr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "External Event Flag of Channel 7   INTF7. This bit monitors the status flag of the event trigger condition for the input channel x. This bit is automatically cleared when the selected condition  see RENx  FENx  is no longer met  if LDENx   1  or remains set until it is cleared by software  if LDENx   0 ."]
    #[inline(always)]
    pub fn intf1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Eifr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Eifr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "External Event Flag of Channel 7   INTF7. This bit monitors the status flag of the event trigger condition for the input channel x. This bit is automatically cleared when the selected condition  see RENx  FENx  is no longer met  if LDENx   1  or remains set until it is cleared by software  if LDENx   0 ."]
    #[inline(always)]
    pub fn intf2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Eifr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Eifr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "External Event Flag of Channel 7   INTF7. This bit monitors the status flag of the event trigger condition for the input channel x. This bit is automatically cleared when the selected condition  see RENx  FENx  is no longer met  if LDENx   1  or remains set until it is cleared by software  if LDENx   0 ."]
    #[inline(always)]
    pub fn intf3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Eifr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Eifr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "External Event Flag of Channel 7   INTF7. This bit monitors the status flag of the event trigger condition for the input channel x. This bit is automatically cleared when the selected condition  see RENx  FENx  is no longer met  if LDENx   1  or remains set until it is cleared by software  if LDENx   0 ."]
    #[inline(always)]
    pub fn intf4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Eifr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Eifr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "External Event Flag of Channel 7   INTF7. This bit monitors the status flag of the event trigger condition for the input channel x. This bit is automatically cleared when the selected condition  see RENx  FENx  is no longer met  if LDENx   1  or remains set until it is cleared by software  if LDENx   0 ."]
    #[inline(always)]
    pub fn intf5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Eifr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Eifr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "External Event Flag of Channel 7   INTF7. This bit monitors the status flag of the event trigger condition for the input channel x. This bit is automatically cleared when the selected condition  see RENx  FENx  is no longer met  if LDENx   1  or remains set until it is cleared by software  if LDENx   0 ."]
    #[inline(always)]
    pub fn intf6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Eifr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Eifr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "External Event Flag of Channel 7   INTF7. This bit monitors the status flag of the event trigger condition for the input channel x. This bit is automatically cleared when the selected condition  see RENx  FENx  is no longer met  if LDENx   1  or remains set until it is cleared by software  if LDENx   0 ."]
    #[inline(always)]
    pub fn intf7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Eifr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Eifr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Eifr {
    #[inline(always)]
    fn default() -> Eifr {
        <crate::RegValueT<Eifr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eisr_SPEC;
impl crate::sealed::RegSpec for Eisr_SPEC {
    type DataType = u32;
}
#[doc = "ENDINIT Timeout Counter Status Register\n resetvalue={Application Reset:0x0FFFC0000}"]
pub type Eisr = crate::RegValueT<Eisr_SPEC>;

impl Eisr {
    #[doc = "EICON0 Access Error Status Flag   AE. This bit is set when an illegal Password Access or Modify Access to        register EICON0 was attempted. This bit is only cleared on a valid        EICON0.ENDINIT Modify Access"]
    #[inline(always)]
    pub fn ae(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, eisr::Ae, Eisr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,eisr::Ae, Eisr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EI Timeout Overflow Error Status Flag   OE. This bit is set when EISR.TIM overflows from FFFF H to FFFC H . This bit is only cleared on        a valid EICON0 Modify Access."]
    #[inline(always)]
    pub fn oe(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, eisr::Oe, Eisr_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,eisr::Oe, Eisr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EI Timeout Input Clock Status   IS1 IS0. Bit IS0 and IS1 should be programmed together. These bits indicate the        current ENDINIT Timeout Counter frequency."]
    #[inline(always)]
    pub fn is0(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, eisr::Is0, Eisr_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,eisr::Is0, Eisr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EI Timeout Enable Disable Status Flag   DS"]
    #[inline(always)]
    pub fn ds(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, eisr::Ds, Eisr_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1,1,0,eisr::Ds, Eisr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EI Time Out Mode Flag   TO"]
    #[inline(always)]
    pub fn to(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, eisr::To, Eisr_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x1,1,0,eisr::To, Eisr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EI Timeout Input Clock Status   IS1 IS0. Bit IS0 and IS1 should be programmed together. These bits indicate the        current ENDINIT Timeout Counter frequency."]
    #[inline(always)]
    pub fn is1(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, eisr::Is1, Eisr_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x1,1,0,eisr::Is1, Eisr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Timer Value   TIM. Reflects the current content of the ENDINIT Timeout Counter.  Only        bits 17 and 16 are implemented in EISR. Others return   8216 1  8217"]
    #[inline(always)]
    pub fn tim(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Eisr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Eisr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Eisr {
    #[inline(always)]
    fn default() -> Eisr {
        <crate::RegValueT<Eisr_SPEC> as RegisterValue<_>>::new(4294705152)
    }
}
pub mod eisr {
    pub struct Ae_SPEC;
    pub type Ae = crate::EnumBitfieldStruct<u8, Ae_SPEC>;
    impl Ae {
        #[doc = "0 No access error"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A access error        has occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oe_SPEC;
    pub type Oe = crate::EnumBitfieldStruct<u8, Oe_SPEC>;
    impl Oe {
        #[doc = "0 No timeout        overflow error"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A timeout        overflow error has occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Is0_SPEC;
    pub type Is0 = crate::EnumBitfieldStruct<u8, Is0_SPEC>;
    impl Is0 {
        #[doc = "If Bit IS1 0 ENDINIT Timeout Counter frequency is f SPB  16384.        Elseif Bit IS1 1 ENDINIT Timeout Counter frequency is f SPB  64."]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Ds_SPEC;
    pub type Ds = crate::EnumBitfieldStruct<u8, Ds_SPEC>;
    impl Ds {
        #[doc = "0 The ENDINIT        Timeout Counter is enabled  After EICON0 Password Access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The ENDINIT        Timeout Counter is disabled  After EICON0 Modify Access"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct To_SPEC;
    pub type To = crate::EnumBitfieldStruct<u8, To_SPEC>;
    impl To {
        #[doc = "0 The ENDINIT        Timeout Counter is not operating in Time Out Mode  After EICON0 Modify        Access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The ENDINIT        Timeout Counter is operating in Time Out Mode  After EICON0 Password        Access"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Is1_SPEC;
    pub type Is1 = crate::EnumBitfieldStruct<u8, Is1_SPEC>;
    impl Is1 {
        #[doc = "If Bit IS0 0 ENDINIT Timeout Counter frequency is f SPB  16384.        Elseif Bit IS0 1 ENDINIT Timeout Counter frequency is f SPB  256."]
        pub const CONST_00: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emsr_SPEC;
impl crate::sealed::RegSpec for Emsr_SPEC {
    type DataType = u32;
}
#[doc = "Emergency Stop Register\n resetvalue={Application Reset:0x1}"]
pub type Emsr = crate::RegValueT<Emsr_SPEC>;

impl Emsr {
    #[doc = "Input Polarity   POL. This bit determines the polarity of the configured Emergency Stop input."]
    #[inline(always)]
    pub fn pol(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, emsr::Pol, Emsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,emsr::Pol, Emsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mode Selection   MODE. This bit determines the operating mode of the emergency stop signal."]
    #[inline(always)]
    pub fn mode(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, emsr::Mode, Emsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,emsr::Mode, Emsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable ON   ENON. This bit enables the setting of flag EMSF by an inactive to active level transition of input signal."]
    #[inline(always)]
    pub fn enon(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, emsr::Enon, Emsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,emsr::Enon, Emsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PORT Select   PSEL. This bit selects which one of the two Emergency Stop port options is monitored."]
    #[inline(always)]
    pub fn psel(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, emsr::Psel, Emsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,emsr::Psel, Emsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Flag   EMSF. This bit indicates that a synchronous mode port triggered emergency stop        condition has occurred."]
    #[inline(always)]
    pub fn emsf(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, emsr::Emsf, Emsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x1,1,0,emsr::Emsf, Emsr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SMU Emergency Stop Flag   SEMSF. This bit indicates that an SMU Safety Alarm triggered emergency stop        condition has occurred."]
    #[inline(always)]
    pub fn semsf(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, emsr::Semsf, Emsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<17,0x1,1,0,emsr::Semsf, Emsr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Emsr {
    #[inline(always)]
    fn default() -> Emsr {
        <crate::RegValueT<Emsr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod emsr {
    pub struct Pol_SPEC;
    pub type Pol = crate::EnumBitfieldStruct<u8, Pol_SPEC>;
    impl Pol {
        #[doc = "0 Input is active high"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Input is active low"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mode_SPEC;
    pub type Mode = crate::EnumBitfieldStruct<u8, Mode_SPEC>;
    impl Mode {
        #[doc = "0 Synchronous Mode selected  emergency stop is derived from the state of flag EMSF"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Asynchronous Mode selected  emergency stop is directly derived from the state of the input signal"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enon_SPEC;
    pub type Enon = crate::EnumBitfieldStruct<u8, Enon_SPEC>;
    impl Enon {
        #[doc = "0 Setting of EMSF is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Setting of EMSF is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Psel_SPEC;
    pub type Psel = crate::EnumBitfieldStruct<u8, Psel_SPEC>;
    impl Psel {
        #[doc = "0 Port A is used as Emergency Stop input"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Port B is used as Emergency Stop input"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Emsf_SPEC;
    pub type Emsf = crate::EnumBitfieldStruct<u8, Emsf_SPEC>;
    impl Emsf {
        #[doc = "0 An emergency stop has not occurred"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An emergency stop has occurred and emergency stop state becomes active  if MODE   0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Semsf_SPEC;
    pub type Semsf = crate::EnumBitfieldStruct<u8, Semsf_SPEC>;
    impl Semsf {
        #[doc = "0 An emergency stop has not occurred"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An emergency stop has occurred and emergency stop state becomes active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emssw_SPEC;
impl crate::sealed::RegSpec for Emssw_SPEC {
    type DataType = u32;
}
#[doc = "Emergency Stop Software set and clear register\n resetvalue={Application Reset:0x0}"]
pub type Emssw = crate::RegValueT<Emssw_SPEC>;

impl Emssw {
    #[doc = "Emergency Stop Flag Modification   EMSFM. This bit field sets or clears flag EMSF via software. In case of a simultaneous hardware and software modification request         the hardware operation will be executed. EMSFM is always read as 00 B ."]
    #[inline(always)]
    pub fn emsfm(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, emssw::Emsfm, Emssw_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<24,0x3,1,0,emssw::Emsfm, Emssw_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "SMU Emergency Stop Flag Modification   SEMSFM. This bit field sets or clears flag SEMSF via software. In case of a simultaneous hardware and software modification request         the hardware operation will be executed. SEMSFM is always read as 00 B ."]
    #[inline(always)]
    pub fn semsfm(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, emssw::Semsfm, Emssw_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<26,0x3,1,0,emssw::Semsfm, Emssw_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Emssw {
    #[inline(always)]
    fn default() -> Emssw {
        <crate::RegValueT<Emssw_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod emssw {
    pub struct Emsfm_SPEC;
    pub type Emsfm = crate::EnumBitfieldStruct<u8, Emsfm_SPEC>;
    impl Emsfm {
        #[doc = "00 EMSF remains unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 EMSF becomes set"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 EMSF becomes cleared"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 EMSF remains unchanged"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Semsfm_SPEC;
    pub type Semsfm = crate::EnumBitfieldStruct<u8, Semsfm_SPEC>;
    impl Semsfm {
        #[doc = "00 SEMSF remains unchanged"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 SEMSF becomes set"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 SEMSF becomes cleared"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SEMSF remains unchanged"]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Esrocfg_SPEC;
impl crate::sealed::RegSpec for Esrocfg_SPEC {
    type DataType = u32;
}
#[doc = "ESR Output Configuration Register\n resetvalue={System Reset:0x0}"]
pub type Esrocfg = crate::RegValueT<Esrocfg_SPEC>;

impl Esrocfg {
    #[doc = "Application Reset Indicator   ARI. This bit is set when an Application Reset request trigger occurs and        cleared by writing to ARC. When the ARI bit is set and an ESR pin is configured as a reset output         the corresponding ESR input will not re trigger a reset. This prevents        feedback of the reset indication causing a new reset request. Extension of the reset by an external ESR source is handled by SSW. Observed reset value after boot will depend upon ARI mode."]
    #[inline(always)]
    pub fn ari(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, esrocfg::Ari, Esrocfg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,esrocfg::Ari, Esrocfg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Application Reset Indicator Clear   ARC. Read as 0"]
    #[inline(always)]
    pub fn arc(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, esrocfg::Arc, Esrocfg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<1,0x1,1,0,esrocfg::Arc, Esrocfg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Esrocfg {
    #[inline(always)]
    fn default() -> Esrocfg {
        <crate::RegValueT<Esrocfg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod esrocfg {
    pub struct Ari_SPEC;
    pub type Ari = crate::EnumBitfieldStruct<u8, Ari_SPEC>;
    impl Ari {
        #[doc = "0 No application        reset trigger detected  since last clear"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Application        reset trigger detected  since last clear"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Arc_SPEC;
    pub type Arc = crate::EnumBitfieldStruct<u8, Arc_SPEC>;
    impl Arc {
        #[doc = "0 No effect"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear        Application Reset Indicator  ARI"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extcon_SPEC;
impl crate::sealed::RegSpec for Extcon_SPEC {
    type DataType = u32;
}
#[doc = "External Clock Control Register\n resetvalue={System Reset:0x0}"]
pub type Extcon = crate::RegValueT<Extcon_SPEC>;

impl Extcon {
    #[doc = "External Clock Enable for EXTCLK0   EN0. If the generation of the external clock signal is disabled  the signal is tied to zero. This bit field can be overruled in Test Mode via the TCU."]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, extcon::En0, Extcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,extcon::En0, Extcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "External Clock Select for EXTCLK0   SEL0. This bit field defines the clock source that is selected as output for        pin EXTCLK0. This bit field can be overruled in Test Mode via the TCU."]
    #[inline(always)]
    pub fn sel0(
        self,
    ) -> crate::common::RegisterField<2, 0xf, 1, 0, extcon::Sel0, Extcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0xf,1,0,extcon::Sel0, Extcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "External Clock Enable for EXTCLK1   EN1. If the generation of the external clock signal is disabled  the signal is tied to zero. This bit can be overruled in Test Mode via the TCU."]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, extcon::En1, Extcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,extcon::En1, Extcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Negation Selection   NSEL. This bit can be overruled in Test Mode via the TCU."]
    #[inline(always)]
    pub fn nsel(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, extcon::Nsel, Extcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,extcon::Nsel, Extcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "External Clock Select for EXTCLK1   SEL1. This bit field defines the clock source that is selected as the output for pin EXTCLK1. This bit field can be overruled in Test Mode via the TCU."]
    #[inline(always)]
    pub fn sel1(
        self,
    ) -> crate::common::RegisterField<18, 0xf, 1, 0, extcon::Sel1, Extcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0xf,1,0,extcon::Sel1, Extcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "External Clock Divider for EXTCLK1   DIV1. This value defines the reload value of the divider that generates f OUT out of f SPB   f OUT   f SPB   DIV1 1  . The divider itself is cleared each time bit EN1 is cleared."]
    #[inline(always)]
    pub fn div1(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Extcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Extcon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Extcon {
    #[inline(always)]
    fn default() -> Extcon {
        <crate::RegValueT<Extcon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod extcon {
    pub struct En0_SPEC;
    pub type En0 = crate::EnumBitfieldStruct<u8, En0_SPEC>;
    impl En0 {
        #[doc = "No external clock is provided"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "The configured external clock is provided"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sel0_SPEC;
    pub type Sel0 = crate::EnumBitfieldStruct<u8, Sel0_SPEC>;
    impl Sel0 {
        #[doc = "f OUT is selected for the external clock signal"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "f PLL0 is selected for the external clock signal"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "f PLL1 is selected for the external clock signal"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "f OSC0 is selected for the external clock signal"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "f BACK is selected for the external clock signal"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "f PLL2 is selected for the external clock signal"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "f BBB is selected for the external clock signal"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "f SRI is selected for the external clock signal"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "f SPB is selected for the external clock signal"]
        pub const CONST_99: Self = Self::new(9);
        #[doc = "f FSI is selected for the external clock signal"]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "f STM is selected for the external clock signal"]
        pub const CONST_1111: Self = Self::new(11);
        #[doc = "f GTM is selected for the external clock signal"]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "f TCK is selected for the external clock signal"]
        pub const CONST_1313: Self = Self::new(13);
        #[doc = "f FSI2 is selected for the external clock signal"]
        pub const CONST_1414: Self = Self::new(14);
        #[doc = "f MT0 from the ERAY module is selected for the external clock signal"]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct En1_SPEC;
    pub type En1 = crate::EnumBitfieldStruct<u8, En1_SPEC>;
    impl En1 {
        #[doc = "No external clock signal is provided"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "The configured external clock signal is provided"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Nsel_SPEC;
    pub type Nsel = crate::EnumBitfieldStruct<u8, Nsel_SPEC>;
    impl Nsel {
        #[doc = "The external clock signal EXTCLK1 is inverted"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "The external clock signal EXTCLK1 is not inverted"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sel1_SPEC;
    pub type Sel1 = crate::EnumBitfieldStruct<u8, Sel1_SPEC>;
    impl Sel1 {
        #[doc = "f OUT is selected for the external clock signal"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "f PLL0 is selected for the external clock signal"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "f PLL1 is selected for the external clock signal"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "f EBU is selected for the external clock signal"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "f BACK is selected for the external clock signal"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "f MCAN is selected for the external clock signal"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "f ADC is selected for the external clock signal"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "f QSPI is selected for the external clock signal"]
        pub const CONST_77: Self = Self::new(7);
        #[doc = "f SRI is selected for the external clock signal"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "f SPB is selected for the external clock signal"]
        pub const CONST_99: Self = Self::new(9);
        #[doc = "f I2C is selected for the external clock signal"]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "f MSC is selected for the external clock signal"]
        pub const CONST_1111: Self = Self::new(11);
        #[doc = "f ERAY is selected for the external clock signal"]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "f ASCLINF is selected for the external clock signal"]
        pub const CONST_1313: Self = Self::new(13);
        #[doc = "f ASCLINS is selected for the external clock signal"]
        pub const CONST_1414: Self = Self::new(14);
        #[doc = "f OSCFL from the flash is selected for the external clock signal"]
        pub const CONST_1515: Self = Self::new(15);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdr_SPEC;
impl crate::sealed::RegSpec for Fdr_SPEC {
    type DataType = u32;
}
#[doc = "Fractional Divider Register\n resetvalue={System Reset:0x0}"]
pub type Fdr = crate::RegValueT<Fdr_SPEC>;

impl Fdr {
    #[doc = "Step Value   STEP. In Normal Divider Mode  STEP contains the reload value for RESULT. In Fractional Divider Mode  this bit field determines the 10 bit value that is added to RESULT with each input clock cycle."]
    #[inline(always)]
    pub fn step(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Fdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Fdr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Divider Mode   DM. These bit fields determine the functionality of the fractional divider block."]
    #[inline(always)]
    pub fn dm(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, fdr::Dm, Fdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,fdr::Dm, Fdr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Result Value   RESULT. In Normal Divider Mode  RESULT acts as reload counter  addition  1 . In Fractional Divider Mode  this bit field contains the result of the addition RESULT   STEP. If DM is written with 01 B or 10 B   RESULT is loaded with 3FF H ."]
    #[inline(always)]
    pub fn result(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Fdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Fdr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Disable Clock   DISCLK"]
    #[inline(always)]
    pub fn disclk(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, fdr::Disclk, Fdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<31,0x1,1,0,fdr::Disclk, Fdr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Fdr {
    #[inline(always)]
    fn default() -> Fdr {
        <crate::RegValueT<Fdr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fdr {
    pub struct Dm_SPEC;
    pub type Dm = crate::EnumBitfieldStruct<u8, Dm_SPEC>;
    impl Dm {
        #[doc = "Fractional divider is switched off  no output clock is generated. The Reset External Divider signal is 1. RESULT is not updated  default after System Reset ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Normal Divider Mode selected."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "Fractional Divider Mode selected."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "Fractional divider is switched off  no output clock is generated. RESULT is not updated."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Disclk_SPEC;
    pub type Disclk = crate::EnumBitfieldStruct<u8, Disclk_SPEC>;
    impl Disclk {
        #[doc = "Clock generation of f OUT is enabled according to the setting of bit field DM."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Fractional divider is stopped. No change except when writing bit field DM."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmr_SPEC;
impl crate::sealed::RegSpec for Fmr_SPEC {
    type DataType = u32;
}
#[doc = "Flag Modification Register\n resetvalue={Application Reset:0x0}"]
pub type Fmr = crate::RegValueT<Fmr_SPEC>;

impl Fmr {
    #[doc = "Set Flag INTFx for Channel 7   FS7. Setting this bit will set the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fs0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, fmr::Fs0, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0x1,1,0,fmr::Fs0, Fmr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Flag INTFx for Channel 7   FS7. Setting this bit will set the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fs1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, fmr::Fs1, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterField::<1,0x1,1,0,fmr::Fs1, Fmr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Flag INTFx for Channel 7   FS7. Setting this bit will set the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fs2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, fmr::Fs2, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterField::<2,0x1,1,0,fmr::Fs2, Fmr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Flag INTFx for Channel 7   FS7. Setting this bit will set the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fs3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, fmr::Fs3, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterField::<3,0x1,1,0,fmr::Fs3, Fmr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Flag INTFx for Channel 7   FS7. Setting this bit will set the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fs4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, fmr::Fs4, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterField::<4,0x1,1,0,fmr::Fs4, Fmr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Flag INTFx for Channel 7   FS7. Setting this bit will set the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fs5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, fmr::Fs5, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterField::<5,0x1,1,0,fmr::Fs5, Fmr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Flag INTFx for Channel 7   FS7. Setting this bit will set the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fs6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, fmr::Fs6, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterField::<6,0x1,1,0,fmr::Fs6, Fmr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Flag INTFx for Channel 7   FS7. Setting this bit will set the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fs7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, fmr::Fs7, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterField::<7,0x1,1,0,fmr::Fs7, Fmr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear Flag INTFx for Channel 7   FC7. Setting this bit will clear the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fc0(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, fmr::Fc0, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterField::<16,0x1,1,0,fmr::Fc0, Fmr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear Flag INTFx for Channel 7   FC7. Setting this bit will clear the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fc1(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, fmr::Fc1, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterField::<17,0x1,1,0,fmr::Fc1, Fmr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear Flag INTFx for Channel 7   FC7. Setting this bit will clear the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fc2(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, fmr::Fc2, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterField::<18,0x1,1,0,fmr::Fc2, Fmr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear Flag INTFx for Channel 7   FC7. Setting this bit will clear the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fc3(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, fmr::Fc3, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterField::<19,0x1,1,0,fmr::Fc3, Fmr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear Flag INTFx for Channel 7   FC7. Setting this bit will clear the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fc4(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, fmr::Fc4, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterField::<20,0x1,1,0,fmr::Fc4, Fmr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear Flag INTFx for Channel 7   FC7. Setting this bit will clear the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fc5(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, fmr::Fc5, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterField::<21,0x1,1,0,fmr::Fc5, Fmr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear Flag INTFx for Channel 7   FC7. Setting this bit will clear the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fc6(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, fmr::Fc6, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterField::<22,0x1,1,0,fmr::Fc6, Fmr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear Flag INTFx for Channel 7   FC7. Setting this bit will clear the corresponding bit INTFx in register EIFR. Reading this bit always delivers a 0. If both FSx and FCx are set in the same access then the bit x in register EIFR is not modified."]
    #[inline(always)]
    pub fn fc7(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, fmr::Fc7, Fmr_SPEC, crate::common::W> {
        crate::common::RegisterField::<23,0x1,1,0,fmr::Fc7, Fmr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Fmr {
    #[inline(always)]
    fn default() -> Fmr {
        <crate::RegValueT<Fmr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fmr {
    pub struct Fs0_SPEC;
    pub type Fs0 = crate::EnumBitfieldStruct<u8, Fs0_SPEC>;
    impl Fs0 {
        #[doc = "0 The bit x in register EIFR is not modified"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit x in register EIFR is set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fs1_SPEC;
    pub type Fs1 = crate::EnumBitfieldStruct<u8, Fs1_SPEC>;
    impl Fs1 {
        #[doc = "0 The bit x in register EIFR is not modified"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit x in register EIFR is set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fs2_SPEC;
    pub type Fs2 = crate::EnumBitfieldStruct<u8, Fs2_SPEC>;
    impl Fs2 {
        #[doc = "0 The bit x in register EIFR is not modified"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit x in register EIFR is set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fs3_SPEC;
    pub type Fs3 = crate::EnumBitfieldStruct<u8, Fs3_SPEC>;
    impl Fs3 {
        #[doc = "0 The bit x in register EIFR is not modified"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit x in register EIFR is set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fs4_SPEC;
    pub type Fs4 = crate::EnumBitfieldStruct<u8, Fs4_SPEC>;
    impl Fs4 {
        #[doc = "0 The bit x in register EIFR is not modified"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit x in register EIFR is set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fs5_SPEC;
    pub type Fs5 = crate::EnumBitfieldStruct<u8, Fs5_SPEC>;
    impl Fs5 {
        #[doc = "0 The bit x in register EIFR is not modified"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit x in register EIFR is set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fs6_SPEC;
    pub type Fs6 = crate::EnumBitfieldStruct<u8, Fs6_SPEC>;
    impl Fs6 {
        #[doc = "0 The bit x in register EIFR is not modified"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit x in register EIFR is set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fs7_SPEC;
    pub type Fs7 = crate::EnumBitfieldStruct<u8, Fs7_SPEC>;
    impl Fs7 {
        #[doc = "0 The bit x in register EIFR is not modified"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit x in register EIFR is set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fc0_SPEC;
    pub type Fc0 = crate::EnumBitfieldStruct<u8, Fc0_SPEC>;
    impl Fc0 {
        #[doc = "0 The bit x in register EIFR is not modified"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit x in register EIFR is cleared"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fc1_SPEC;
    pub type Fc1 = crate::EnumBitfieldStruct<u8, Fc1_SPEC>;
    impl Fc1 {
        #[doc = "0 The bit x in register EIFR is not modified"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit x in register EIFR is cleared"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fc2_SPEC;
    pub type Fc2 = crate::EnumBitfieldStruct<u8, Fc2_SPEC>;
    impl Fc2 {
        #[doc = "0 The bit x in register EIFR is not modified"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit x in register EIFR is cleared"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fc3_SPEC;
    pub type Fc3 = crate::EnumBitfieldStruct<u8, Fc3_SPEC>;
    impl Fc3 {
        #[doc = "0 The bit x in register EIFR is not modified"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit x in register EIFR is cleared"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fc4_SPEC;
    pub type Fc4 = crate::EnumBitfieldStruct<u8, Fc4_SPEC>;
    impl Fc4 {
        #[doc = "0 The bit x in register EIFR is not modified"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit x in register EIFR is cleared"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fc5_SPEC;
    pub type Fc5 = crate::EnumBitfieldStruct<u8, Fc5_SPEC>;
    impl Fc5 {
        #[doc = "0 The bit x in register EIFR is not modified"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit x in register EIFR is cleared"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fc6_SPEC;
    pub type Fc6 = crate::EnumBitfieldStruct<u8, Fc6_SPEC>;
    impl Fc6 {
        #[doc = "0 The bit x in register EIFR is not modified"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit x in register EIFR is cleared"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fc7_SPEC;
    pub type Fc7 = crate::EnumBitfieldStruct<u8, Fc7_SPEC>;
    impl Fc7 {
        #[doc = "0 The bit x in register EIFR is not modified"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit x in register EIFR is cleared"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Identification Register\n resetvalue={System Reset:0x0C4C0C1}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MODREV. This bit field indicates the revision number of the SCU module  C1 H  . Bits  5 0  defines the SCU module IP revision number. Bits  7 6  defines        the family. This allows a continuous numbering of the revision AND a        deviation of the different SCU generations. The revision number starts        with 000001 and is incremented when a SCU change is detectable by        customer software. MODREV for other variants will be defined by Design Team"]
    #[inline(always)]
    pub fn modrev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MODTYPE. This bit field is C0 H . It defines a 32 bit module"]
    #[inline(always)]
    pub fn modtype(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number   MODNUMBER. This bit field defines the module identification number. The identification number for the SCU is 00C4 H"]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(12894401)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IgcRj_SPEC;
impl crate::sealed::RegSpec for IgcRj_SPEC {
    type DataType = u32;
}
#[doc = "Flag Gating Register 0\n resetvalue={Application Reset:0x0}"]
pub type IgcRj = crate::RegValueT<IgcRj_SPEC>;

impl IgcRj {
    #[doc = "Flag Pattern Enable for Channel 0   IPEN07. Bit IPEN0x determines if the flag INTFx of channel x takes part in the pattern detection for the gating of the requests for the output signals GOUT 2 j  and IOUT 2 j ."]
    #[inline(always)]
    pub fn ipen00(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, igcrj::Ipen00, IgcRj_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,igcrj::Ipen00, IgcRj_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Flag Pattern Enable for Channel 0   IPEN07. Bit IPEN0x determines if the flag INTFx of channel x takes part in the pattern detection for the gating of the requests for the output signals GOUT 2 j  and IOUT 2 j ."]
    #[inline(always)]
    pub fn ipen01(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, igcrj::Ipen01, IgcRj_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,igcrj::Ipen01, IgcRj_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Flag Pattern Enable for Channel 0   IPEN07. Bit IPEN0x determines if the flag INTFx of channel x takes part in the pattern detection for the gating of the requests for the output signals GOUT 2 j  and IOUT 2 j ."]
    #[inline(always)]
    pub fn ipen02(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, igcrj::Ipen02, IgcRj_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,igcrj::Ipen02, IgcRj_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Flag Pattern Enable for Channel 0   IPEN07. Bit IPEN0x determines if the flag INTFx of channel x takes part in the pattern detection for the gating of the requests for the output signals GOUT 2 j  and IOUT 2 j ."]
    #[inline(always)]
    pub fn ipen03(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, igcrj::Ipen03, IgcRj_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,igcrj::Ipen03, IgcRj_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Flag Pattern Enable for Channel 0   IPEN07. Bit IPEN0x determines if the flag INTFx of channel x takes part in the pattern detection for the gating of the requests for the output signals GOUT 2 j  and IOUT 2 j ."]
    #[inline(always)]
    pub fn ipen04(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, igcrj::Ipen04, IgcRj_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,igcrj::Ipen04, IgcRj_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Flag Pattern Enable for Channel 0   IPEN07. Bit IPEN0x determines if the flag INTFx of channel x takes part in the pattern detection for the gating of the requests for the output signals GOUT 2 j  and IOUT 2 j ."]
    #[inline(always)]
    pub fn ipen05(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, igcrj::Ipen05, IgcRj_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,igcrj::Ipen05, IgcRj_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Flag Pattern Enable for Channel 0   IPEN07. Bit IPEN0x determines if the flag INTFx of channel x takes part in the pattern detection for the gating of the requests for the output signals GOUT 2 j  and IOUT 2 j ."]
    #[inline(always)]
    pub fn ipen06(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, igcrj::Ipen06, IgcRj_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,igcrj::Ipen06, IgcRj_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Flag Pattern Enable for Channel 0   IPEN07. Bit IPEN0x determines if the flag INTFx of channel x takes part in the pattern detection for the gating of the requests for the output signals GOUT 2 j  and IOUT 2 j ."]
    #[inline(always)]
    pub fn ipen07(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, igcrj::Ipen07, IgcRj_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,igcrj::Ipen07, IgcRj_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Generate Event Enable 0   GEEN0. Bit GEEN0 enables the generation of a trigger event for output channel         2 j  when the result of the pattern detection changes. When using this        feature  a trigger  e.g. for an interrupt  is generated during the first        clock cycle when a pattern is detected or when it is no longer detected."]
    #[inline(always)]
    pub fn geen0(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, igcrj::Geen0, IgcRj_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,igcrj::Geen0, IgcRj_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Gating Pattern 0   IGP0. In each register IGCRj  bit field IGP0 determines how the pattern        detection influences the output lines GOUT 2j  and IOUT 2j ."]
    #[inline(always)]
    pub fn igp0(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, igcrj::Igp0, IgcRj_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,igcrj::Igp0, IgcRj_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Pattern Enable for Channel 1   IPEN17. Bit IPEN 2j 1 x determines if the flag INTFx of channel  2j 1  takes part in the pattern detection for the gating of the requests for the output signals GOUT 2j 1  and IOUT 2j 1 ."]
    #[inline(always)]
    pub fn ipen10(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, igcrj::Ipen10, IgcRj_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,igcrj::Ipen10, IgcRj_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Pattern Enable for Channel 1   IPEN17. Bit IPEN 2j 1 x determines if the flag INTFx of channel  2j 1  takes part in the pattern detection for the gating of the requests for the output signals GOUT 2j 1  and IOUT 2j 1 ."]
    #[inline(always)]
    pub fn ipen11(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, igcrj::Ipen11, IgcRj_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,igcrj::Ipen11, IgcRj_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Pattern Enable for Channel 1   IPEN17. Bit IPEN 2j 1 x determines if the flag INTFx of channel  2j 1  takes part in the pattern detection for the gating of the requests for the output signals GOUT 2j 1  and IOUT 2j 1 ."]
    #[inline(always)]
    pub fn ipen12(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, igcrj::Ipen12, IgcRj_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,igcrj::Ipen12, IgcRj_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Pattern Enable for Channel 1   IPEN17. Bit IPEN 2j 1 x determines if the flag INTFx of channel  2j 1  takes part in the pattern detection for the gating of the requests for the output signals GOUT 2j 1  and IOUT 2j 1 ."]
    #[inline(always)]
    pub fn ipen13(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, igcrj::Ipen13, IgcRj_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,igcrj::Ipen13, IgcRj_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Pattern Enable for Channel 1   IPEN17. Bit IPEN 2j 1 x determines if the flag INTFx of channel  2j 1  takes part in the pattern detection for the gating of the requests for the output signals GOUT 2j 1  and IOUT 2j 1 ."]
    #[inline(always)]
    pub fn ipen14(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, igcrj::Ipen14, IgcRj_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,igcrj::Ipen14, IgcRj_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Pattern Enable for Channel 1   IPEN17. Bit IPEN 2j 1 x determines if the flag INTFx of channel  2j 1  takes part in the pattern detection for the gating of the requests for the output signals GOUT 2j 1  and IOUT 2j 1 ."]
    #[inline(always)]
    pub fn ipen15(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, igcrj::Ipen15, IgcRj_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,igcrj::Ipen15, IgcRj_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Pattern Enable for Channel 1   IPEN17. Bit IPEN 2j 1 x determines if the flag INTFx of channel  2j 1  takes part in the pattern detection for the gating of the requests for the output signals GOUT 2j 1  and IOUT 2j 1 ."]
    #[inline(always)]
    pub fn ipen16(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, igcrj::Ipen16, IgcRj_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,igcrj::Ipen16, IgcRj_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Pattern Enable for Channel 1   IPEN17. Bit IPEN 2j 1 x determines if the flag INTFx of channel  2j 1  takes part in the pattern detection for the gating of the requests for the output signals GOUT 2j 1  and IOUT 2j 1 ."]
    #[inline(always)]
    pub fn ipen17(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, igcrj::Ipen17, IgcRj_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,igcrj::Ipen17, IgcRj_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Generate Event Enable 1   GEEN1. Bit GEEN1 enables the generation of a trigger event for output channel         2j 1  when the result of the pattern detection changes. When using this        feature  a trigger  e.g. for an interrupt  is generated during the first        clock cycle when a pattern is detected  or when it is no longer detected."]
    #[inline(always)]
    pub fn geen1(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, igcrj::Geen1, IgcRj_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,igcrj::Geen1, IgcRj_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Gating Pattern 1   IGP1. In each register IGCRj  bit field IGP1 determines how the pattern detection influences the output lines GOUT 2j 1  and IOUT 2j 1 ."]
    #[inline(always)]
    pub fn igp1(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, igcrj::Igp1, IgcRj_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x3,1,0,igcrj::Igp1, IgcRj_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for IgcRj {
    #[inline(always)]
    fn default() -> IgcRj {
        <crate::RegValueT<IgcRj_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod igcrj {
    pub struct Ipen00_SPEC;
    pub type Ipen00 = crate::EnumBitfieldStruct<u8, Ipen00_SPEC>;
    impl Ipen00 {
        #[doc = "0 The bit INTFx does not take part in the pattern detection"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit INTFx is taken into consideration for the pattern detection"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ipen01_SPEC;
    pub type Ipen01 = crate::EnumBitfieldStruct<u8, Ipen01_SPEC>;
    impl Ipen01 {
        #[doc = "0 The bit INTFx does not take part in the pattern detection"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit INTFx is taken into consideration for the pattern detection"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ipen02_SPEC;
    pub type Ipen02 = crate::EnumBitfieldStruct<u8, Ipen02_SPEC>;
    impl Ipen02 {
        #[doc = "0 The bit INTFx does not take part in the pattern detection"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit INTFx is taken into consideration for the pattern detection"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ipen03_SPEC;
    pub type Ipen03 = crate::EnumBitfieldStruct<u8, Ipen03_SPEC>;
    impl Ipen03 {
        #[doc = "0 The bit INTFx does not take part in the pattern detection"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit INTFx is taken into consideration for the pattern detection"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ipen04_SPEC;
    pub type Ipen04 = crate::EnumBitfieldStruct<u8, Ipen04_SPEC>;
    impl Ipen04 {
        #[doc = "0 The bit INTFx does not take part in the pattern detection"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit INTFx is taken into consideration for the pattern detection"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ipen05_SPEC;
    pub type Ipen05 = crate::EnumBitfieldStruct<u8, Ipen05_SPEC>;
    impl Ipen05 {
        #[doc = "0 The bit INTFx does not take part in the pattern detection"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit INTFx is taken into consideration for the pattern detection"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ipen06_SPEC;
    pub type Ipen06 = crate::EnumBitfieldStruct<u8, Ipen06_SPEC>;
    impl Ipen06 {
        #[doc = "0 The bit INTFx does not take part in the pattern detection"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit INTFx is taken into consideration for the pattern detection"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ipen07_SPEC;
    pub type Ipen07 = crate::EnumBitfieldStruct<u8, Ipen07_SPEC>;
    impl Ipen07 {
        #[doc = "0 The bit INTFx does not take part in the pattern detection"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit INTFx is taken into consideration for the pattern detection"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Geen0_SPEC;
    pub type Geen0 = crate::EnumBitfieldStruct<u8, Geen0_SPEC>;
    impl Geen0 {
        #[doc = "0 The trigger generation at a change of the pattern detection result is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The trigger generation at a change of the pattern detection result is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Igp0_SPEC;
    pub type Igp0 = crate::EnumBitfieldStruct<u8, Igp0_SPEC>;
    impl Igp0 {
        #[doc = "00 IOUT 2j  is inactive.The pattern is not considered."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 IOUT 2j  is activated in response to a trigger event. The pattern is not considered."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 The detected pattern is considered. IOUT 2j  is activated if a trigger event occurs while the pattern is present."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 The detected pattern is considered. IOUT 2j  is activated if a trigger event occurs while the pattern is not present."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Ipen10_SPEC;
    pub type Ipen10 = crate::EnumBitfieldStruct<u8, Ipen10_SPEC>;
    impl Ipen10 {
        #[doc = "0 The bit INTFx does not take part in the pattern detection"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit INTFx is taken into consideration for the pattern detection"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ipen11_SPEC;
    pub type Ipen11 = crate::EnumBitfieldStruct<u8, Ipen11_SPEC>;
    impl Ipen11 {
        #[doc = "0 The bit INTFx does not take part in the pattern detection"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit INTFx is taken into consideration for the pattern detection"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ipen12_SPEC;
    pub type Ipen12 = crate::EnumBitfieldStruct<u8, Ipen12_SPEC>;
    impl Ipen12 {
        #[doc = "0 The bit INTFx does not take part in the pattern detection"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit INTFx is taken into consideration for the pattern detection"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ipen13_SPEC;
    pub type Ipen13 = crate::EnumBitfieldStruct<u8, Ipen13_SPEC>;
    impl Ipen13 {
        #[doc = "0 The bit INTFx does not take part in the pattern detection"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit INTFx is taken into consideration for the pattern detection"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ipen14_SPEC;
    pub type Ipen14 = crate::EnumBitfieldStruct<u8, Ipen14_SPEC>;
    impl Ipen14 {
        #[doc = "0 The bit INTFx does not take part in the pattern detection"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit INTFx is taken into consideration for the pattern detection"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ipen15_SPEC;
    pub type Ipen15 = crate::EnumBitfieldStruct<u8, Ipen15_SPEC>;
    impl Ipen15 {
        #[doc = "0 The bit INTFx does not take part in the pattern detection"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit INTFx is taken into consideration for the pattern detection"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ipen16_SPEC;
    pub type Ipen16 = crate::EnumBitfieldStruct<u8, Ipen16_SPEC>;
    impl Ipen16 {
        #[doc = "0 The bit INTFx does not take part in the pattern detection"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit INTFx is taken into consideration for the pattern detection"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ipen17_SPEC;
    pub type Ipen17 = crate::EnumBitfieldStruct<u8, Ipen17_SPEC>;
    impl Ipen17 {
        #[doc = "0 The bit INTFx does not take part in the pattern detection"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bit INTFx is taken into consideration for the pattern detection"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Geen1_SPEC;
    pub type Geen1 = crate::EnumBitfieldStruct<u8, Geen1_SPEC>;
    impl Geen1 {
        #[doc = "0 The trigger generation at a change of the pattern detection result is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The trigger generation at a change of the pattern detection result is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Igp1_SPEC;
    pub type Igp1 = crate::EnumBitfieldStruct<u8, Igp1_SPEC>;
    impl Igp1 {
        #[doc = "00 IOUT 2j 1  is inactive.The pattern is not considered."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 IOUT 2j 1  is activated in response to a trigger event. The pattern is not considered."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 The detected pattern is considered. IOUT 2j 1  is activated if a trigger event occurs while the pattern is present."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 The detected pattern is considered. IOUT 2j 1  is activated if a trigger event occurs while the pattern is not present."]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct In_SPEC;
impl crate::sealed::RegSpec for In_SPEC {
    type DataType = u32;
}
#[doc = "ESR Input Register\n resetvalue={System Reset:0x0}"]
pub type In = crate::RegValueT<In_SPEC>;

impl In {
    #[doc = "Input Bit 1   P1. This bit indicates the level at the input pin ESR x."]
    #[inline(always)]
    pub fn p0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, r#in::P0, In_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,r#in::P0, In_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Input Bit 1   P1. This bit indicates the level at the input pin ESR x."]
    #[inline(always)]
    pub fn p1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, r#in::P1, In_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,r#in::P1, In_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for In {
    #[inline(always)]
    fn default() -> In {
        <crate::RegValueT<In_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod r#in {
    pub struct P0_SPEC;
    pub type P0 = crate::EnumBitfieldStruct<u8, P0_SPEC>;
    impl P0 {
        #[doc = "0 The input level of ESRx is 0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The input level of ESRx is 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct P1_SPEC;
    pub type P1 = crate::EnumBitfieldStruct<u8, P1_SPEC>;
    impl P1 {
        #[doc = "0 The input level of ESRx is 0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The input level of ESRx is 1"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iocr_SPEC;
impl crate::sealed::RegSpec for Iocr_SPEC {
    type DataType = u32;
}
#[doc = "Input Output Control Register\n resetvalue={System Reset:0x0E0}"]
pub type Iocr = crate::RegValueT<Iocr_SPEC>;

impl Iocr {
    #[doc = "Control for ESR0 Pin   PC0. This bit field defines the ESR0 functionality according to the coding        tables."]
    #[inline(always)]
    pub fn pc0(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Iocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Iocr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Control for ESR1 Pin   PC1. This bit field defines the ESR1 functionality according to the coding tables. The reset value of SCU IOCR.PC1 is influenced by HWCFG6 and PMSWCR5.TRISTREQ. When a cold reset is activated and HWCFG6 1 then PC1 is reset to 20H and ESR1 will have input pull up mode. If HWCFG6 0 then PC1 is reset to 00H and ESR1 will have tri state mode. PC1 and the ESR1 reset state can also be configured by software with the PMSWCR5.TRISTREQ bit. PMSWCR5.TRISTREQ is not affected by warm reset or wake up from standby so the IOCR.PC1 reset value is configured as per the state of the TRISTREQ bit prior to the warm reset"]
    #[inline(always)]
    pub fn pc1(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Iocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Iocr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Iocr {
    #[inline(always)]
    fn default() -> Iocr {
        <crate::RegValueT<Iocr_SPEC> as RegisterValue<_>>::new(224)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lbistctrl0_SPEC;
impl crate::sealed::RegSpec for Lbistctrl0_SPEC {
    type DataType = u32;
}
#[doc = "Logic BIST Control 0 Register\n resetvalue={System Reset:0x0,CFS Value:0x400}"]
pub type Lbistctrl0 = crate::RegValueT<Lbistctrl0_SPEC>;

impl Lbistctrl0 {
    #[doc = "LBIST Request   LBISTREQ. If written high this bit requests the execution of an automatic        scan test procedure. The request will only be approved if LBISTCTRL0 .LBISTDONE        bit reflects a   8217 0  8217  value  i.e. no LBIST procedure was triggered since        the last power on reset or LBIST controller has been restarted through        the LBISTCTRL0 .LBISTRES bit .        If read this bit always returns a   8216 0  8217 . This bit shall be implemented in a safety relevant way to avoid        unintended activation of LBIST during application. LBIST execution time depends on the number of scan loads as defined in          the PATTERNS field."]
    #[inline(always)]
    pub fn lbistreq(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Lbistctrl0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,Lbistctrl0_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "LBIST Reset  LBISTRES. If written high this bit synchronously brings back the LBIST controller        to its initial Reset Idle state and also clears the stored        MISR signature to allow another execution from CPU side. As a        consequence the LBISTCTRL0 .LBISTDONE         and LBISTCTRL3 .SIGNATURE bits        will be set to   8217 0  8217 . If read this bit always returns a   8216 0  8217 . It is strongly recommended to not change the LBISTFREQU parameter in          LBISTCTRL1 after this bit has been set to  1   because there is no          guarantee that the new frequency parameter value will be transferred          to the LBIST controller in time before the next LBIST run is started          from user side  i.e. LBISTCTRL0.LBISTREQ is set to  1  ."]
    #[inline(always)]
    pub fn lbistres(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Lbistctrl0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,Lbistctrl0_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "LBIST Pattern Number   PATTERNS. This field defines the number of scan patterns  i.e. scan loads   which        will be executed during the LBIST procedure. Please note that the value        programmed to this field determines the number scan capture phases not        the number of scan chain load unload phases  i.e. a value of 0x00001        will result in two scan chain loads with on capture in between  a value        of 0x00002 will result in 3 scan chain loads with 2 captures  etc. .        Consequently a value of 0x00000 is not valid  because no capture would        be executed in this case."]
    #[inline(always)]
    pub fn patterns(
        self,
    ) -> crate::common::RegisterField<2, 0x3ffff, 1, 0, u32, Lbistctrl0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3ffff,1,0,u32, Lbistctrl0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LBIST Execution Indicator   LBISTDONE. This bit indicates the actual LBIST controller execution status"]
    #[inline(always)]
    pub fn lbistdone(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        lbistctrl0::Lbistdone,
        Lbistctrl0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            lbistctrl0::Lbistdone,
            Lbistctrl0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "LBIST   Test Mode Alarm Error Injection. If written high this bit trigger both  the LBIST  and the        test mode alarm. This is required to allow self testing of all        LBIST  and test mode  related safety mechanisms in the TCU. The bit will        be reset automatically once the LBIST and test mode alarm indicator        signals from TCU are asserted. From these indicator signals SCU will        also generate corresponding alarm trigger signals for SMU."]
    #[inline(always)]
    pub fn lbisterrinj(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Lbistctrl0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,Lbistctrl0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LBIST Request Redundancy. This bit represents the safety double of LBISTCTRL0.LBISTREQ. In order        to generate a new LBIST request both  LBISTREQRED and LBISTREQ bits must        be set to high due to safety reasons. The request will only be approved        if LBISTCTRL0.LBISTDONE bit reflects a  0  value. If read this bit        always returns a  0 ."]
    #[inline(always)]
    pub fn lbistreqred(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Lbistctrl0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31,1,0,Lbistctrl0_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Lbistctrl0 {
    #[inline(always)]
    fn default() -> Lbistctrl0 {
        <crate::RegValueT<Lbistctrl0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lbistctrl0 {
    pub struct Lbistdone_SPEC;
    pub type Lbistdone = crate::EnumBitfieldStruct<u8, Lbistdone_SPEC>;
    impl Lbistdone {
        #[doc = "0 No LBIST        executed since last power on reset or LBIST controller has been        restarted  via LBISTCTRL0 .LBISTRES        function . Values in LBISTCTRL3 .SIGNATURE field        are all set to   8217 0  8217 ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 At least one        LBIST procedure successfully finished since last power on reset. Values        in LBISTCTRL3 .SIGNATURE field        reflect the resulting MISR signature."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lbistctrl1_SPEC;
impl crate::sealed::RegSpec for Lbistctrl1_SPEC {
    type DataType = u32;
}
#[doc = "Logic BIST Control 1 Register\n resetvalue={System Reset:0x0,CFS Value:0x54000007}"]
pub type Lbistctrl1 = crate::RegValueT<Lbistctrl1_SPEC>;

impl Lbistctrl1 {
    #[doc = "LBIST Seed   SEED. This field determines  which pattern is applied to the EDT channel        inputs 1 19 during LBIST execution."]
    #[inline(always)]
    pub fn seed(
        self,
    ) -> crate::common::RegisterField<0, 0x7ffff, 1, 0, u32, Lbistctrl1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7ffff,1,0,u32, Lbistctrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LBIST Split Shift Selection   SPLITSH. The value of this bit will allow to run LBIST with partitioned        scan shift operation in order to reduce the power consumption."]
    #[inline(always)]
    pub fn splitsh(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x7,
        1,
        0,
        lbistctrl1::Splitsh,
        Lbistctrl1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x7,
            1,
            0,
            lbistctrl1::Splitsh,
            Lbistctrl1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Body Application Indicator   BODY. The value of this bit will determine the static reset behavior of all GPIOs during LBIST execution. If set to low GPIOs will show a weak pull up behavior  if set to high GPIOs are constrained to tri state. A high value must be written to this bit in case LBIST shall be executed for body applications."]
    #[inline(always)]
    pub fn body(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Lbistctrl1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,Lbistctrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LBIST Frequency Selection   LBISTFREQU. Through this register field a pre scaler factor between 1..16 is selectable for LBIST operation clock  derived from EVR oscillator . This will allow to determine the LBIST scan shift frequency. Value of these bits will be mirrored inside of LBIST controller and become effective if a new LBIST procedure has been successfully initiated from system side  via LBISTCTRL0 .LBISTREQ . It is strongly recommended not to change the value of this field after LBISTCTRL0.LBISTRES has been set to high  because there is no guarantee that the new frequency parameter value will be transferred to the LBIST controller in time before the next LBIST run is started from user side  i.e. LBISTCTRL0.LBISTREQ is set to  1"]
    #[inline(always)]
    pub fn lbistfrequ(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Lbistctrl1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Lbistctrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Lbistctrl1 {
    #[inline(always)]
    fn default() -> Lbistctrl1 {
        <crate::RegValueT<Lbistctrl1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lbistctrl1 {
    pub struct Splitsh_SPEC;
    pub type Splitsh = crate::EnumBitfieldStruct<u8, Splitsh_SPEC>;
    impl Splitsh {
        #[doc = "0xx Concurrent        scan shift is selected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "0xx Concurrent        scan shift is selected."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "0xx Concurrent        scan shift is selected."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "0xx Concurrent        scan shift is selected."]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "1x0 Partitioned        scan shift is selected  four scan partitions ."]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "1x0 Partitioned        scan shift is selected  four scan partitions ."]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "1x1 Partitioned        scan shift is selected  two scan partitions ."]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "1x1 Partitioned        scan shift is selected  two scan partitions ."]
        pub const CONST_77: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lbistctrl2_SPEC;
impl crate::sealed::RegSpec for Lbistctrl2_SPEC {
    type DataType = u32;
}
#[doc = "Logic BIST Control 2 Register\n resetvalue={System Reset:0x0,CFS Value:0x3D}"]
pub type Lbistctrl2 = crate::RegValueT<Lbistctrl2_SPEC>;

impl Lbistctrl2 {
    #[doc = "LBIST Maximum Scan Chain Length   LENGTH. This field defines the number of shift cycles for each LBIST scan load. It will be automatically loaded with the product specific value  stored in Flash config sector during startup software execution."]
    #[inline(always)]
    pub fn length(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, Lbistctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16, Lbistctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Lbistctrl2 {
    #[inline(always)]
    fn default() -> Lbistctrl2 {
        <crate::RegValueT<Lbistctrl2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lbistctrl3_SPEC;
impl crate::sealed::RegSpec for Lbistctrl3_SPEC {
    type DataType = u32;
}
#[doc = "Logic BIST Control 3 Register\n resetvalue={System Reset:0x0,CFS Value:0x0}"]
pub type Lbistctrl3 = crate::RegValueT<Lbistctrl3_SPEC>;

impl Lbistctrl3 {
    #[doc = "LBIST Signature   SIGNATURE. This field reflects the MISR signature from the last LBIST execution. It is mirrored from LBIST controller inside TCU and only valid if LBISTCTRL0 .LBISTDONE is read with a high value. In case of a restart of the LBIST controller  via LBISTCTRL0 .LBISTRES function   the signature value will be synchronously reset to all 0. Please address the specific device appendix document for a description on the SIGNATURE value  depending on the LBIST run configuration."]
    #[inline(always)]
    pub fn signature(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Lbistctrl3_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Lbistctrl3_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Lbistctrl3 {
    #[inline(always)]
    fn default() -> Lbistctrl3 {
        <crate::RegValueT<Lbistctrl3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lclcon0_SPEC;
impl crate::sealed::RegSpec for Lclcon0_SPEC {
    type DataType = u32;
}
#[doc = "LCL CPU0 and CPU2 Control Register\n resetvalue={Cold PowerOn Reset:0x080018001,Cold PowerOn Reset:0x080018000}"]
pub type Lclcon0 = crate::RegValueT<Lclcon0_SPEC>;

impl Lclcon0 {
    #[doc = "Lockstep Mode Status   LS0. This bit indicates whether CPU0 is currently running in lockstep monitor        mode"]
    #[inline(always)]
    pub fn ls0(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, lclcon0::Ls0, Lclcon0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1,1,0,lclcon0::Ls0, Lclcon0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Lockstep Enable   LSEN0. This bit may only be written by SSW during boot. Enable lockstep CPU monitoring for the associated processor core  CPU0. After cold reset  lockstep is enabled by default. The LSEN bit may be        cleared during the boot to disable lockstep mode. SMU lockstep fault        reporting should be disabled when lockstep is disabled."]
    #[inline(always)]
    pub fn lsen0(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, lclcon0::Lsen0, Lclcon0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,lclcon0::Lsen0, Lclcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Lclcon0 {
    #[inline(always)]
    fn default() -> Lclcon0 {
        <crate::RegValueT<Lclcon0_SPEC> as RegisterValue<_>>::new(2147581952)
    }
}
pub mod lclcon0 {
    pub struct Ls0_SPEC;
    pub type Ls0 = crate::EnumBitfieldStruct<u8, Ls0_SPEC>;
    impl Ls0 {
        #[doc = "0 Not in lockstep mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Running in lockstep mode"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lsen0_SPEC;
    pub type Lsen0 = crate::EnumBitfieldStruct<u8, Lsen0_SPEC>;
    impl Lsen0 {
        #[doc = "0 Lockstep is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Lockstep enabled  Default after Cold Power On Reset"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lclcon1_SPEC;
impl crate::sealed::RegSpec for Lclcon1_SPEC {
    type DataType = u32;
}
#[doc = "LCL CPU1 and CPU3 Control Register\n resetvalue={Cold PowerOn Reset:0x080018001,Cold PowerOn Reset:0x080018000,Cold PowerOn Reset:0x080008000}"]
pub type Lclcon1 = crate::RegValueT<Lclcon1_SPEC>;

impl Lclcon1 {
    #[doc = "Lockstep Mode Status   LS1. This bit indicates whether CPU1 is currently running in lockstep monitor        mode"]
    #[inline(always)]
    pub fn ls1(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, lclcon1::Ls1, Lclcon1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1,1,0,lclcon1::Ls1, Lclcon1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Lockstep Enable   LSEN1. This bit may only be written by SSW during boot. Enable lockstep CPU monitoring for the associated processor core  CPU1.       If the product has no lockstep capability for CPU1  then this enables only the PFLASH access monitoring for CPU1. After cold reset  lockstep is enabled by default. The LSEN bit may be        cleared during the boot to disable lockstep mode. SMU lockstep fault        reporting should be disabled when lockstep is disabled."]
    #[inline(always)]
    pub fn lsen1(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, lclcon1::Lsen1, Lclcon1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,lclcon1::Lsen1, Lclcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Lclcon1 {
    #[inline(always)]
    fn default() -> Lclcon1 {
        <crate::RegValueT<Lclcon1_SPEC> as RegisterValue<_>>::new(2147516416)
    }
}
pub mod lclcon1 {
    pub struct Ls1_SPEC;
    pub type Ls1 = crate::EnumBitfieldStruct<u8, Ls1_SPEC>;
    impl Ls1 {
        #[doc = "0 Not in lockstep mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Running in lockstep mode"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lsen1_SPEC;
    pub type Lsen1 = crate::EnumBitfieldStruct<u8, Lsen1_SPEC>;
    impl Lsen1 {
        #[doc = "0 Lockstep is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Lockstep enabled  Default after Cold Power On Reset"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcltest_SPEC;
impl crate::sealed::RegSpec for Lcltest_SPEC {
    type DataType = u32;
}
#[doc = "LCL Test Register\n resetvalue={System Reset:0x0}"]
pub type Lcltest = crate::RegValueT<Lcltest_SPEC>;

impl Lcltest {
    #[doc = "LCL0 Lockstep Test   LCLT0. Fault injection for LCL0. Reads as zero."]
    #[inline(always)]
    pub fn lclt0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, lcltest::Lclt0, Lcltest_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x1,1,0,lcltest::Lclt0, Lcltest_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "LCL1 Lockstep Test   LCLT1. Fault injection for LCL1. Reads as zero."]
    #[inline(always)]
    pub fn lclt1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, lcltest::Lclt1, Lcltest_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<1,0x1,1,0,lcltest::Lclt1, Lcltest_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "LCL2 Lockstep Test   LCLT2. Fault injection for LCL2. Reads as zero."]
    #[inline(always)]
    pub fn lclt2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, lcltest::Lclt2, Lcltest_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<2,0x1,1,0,lcltest::Lclt2, Lcltest_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "PFI0 Lockstep Test. Fault injection for PFI0 lockstep. Reads as zero."]
    #[inline(always)]
    pub fn plclt0(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, lcltest::Plclt0, Lcltest_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0x1,1,0,lcltest::Plclt0, Lcltest_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "PFI1 Lockstep Test . Fault injection for PFI1 lockstep. Reads as zero."]
    #[inline(always)]
    pub fn plclt1(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, lcltest::Plclt1, Lcltest_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<17,0x1,1,0,lcltest::Plclt1, Lcltest_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "PFI2 Lockstep Test . Fault injection for PFI2 lockstep. Reads as zero."]
    #[inline(always)]
    pub fn plclt2(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, lcltest::Plclt2, Lcltest_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<18,0x1,1,0,lcltest::Plclt2, Lcltest_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Lcltest {
    #[inline(always)]
    fn default() -> Lcltest {
        <crate::RegValueT<Lcltest_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lcltest {
    pub struct Lclt0_SPEC;
    pub type Lclt0 = crate::EnumBitfieldStruct<u8, Lclt0_SPEC>;
    impl Lclt0 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Inject single fault in LCL0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lclt1_SPEC;
    pub type Lclt1 = crate::EnumBitfieldStruct<u8, Lclt1_SPEC>;
    impl Lclt1 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Inject single fault in LCL1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lclt2_SPEC;
    pub type Lclt2 = crate::EnumBitfieldStruct<u8, Lclt2_SPEC>;
    impl Lclt2 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Inject single fault in LCL2"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Plclt0_SPEC;
    pub type Plclt0 = crate::EnumBitfieldStruct<u8, Plclt0_SPEC>;
    impl Plclt0 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Inject single fault in PFI0 lockstep"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Plclt1_SPEC;
    pub type Plclt1 = crate::EnumBitfieldStruct<u8, Plclt1_SPEC>;
    impl Plclt1 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Inject single fault in PFI1 lockstep"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Plclt2_SPEC;
    pub type Plclt2 = crate::EnumBitfieldStruct<u8, Plclt2_SPEC>;
    impl Plclt2 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Inject single fault in PFI2 lockstep"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Manid_SPEC;
impl crate::sealed::RegSpec for Manid_SPEC {
    type DataType = u32;
}
#[doc = "Manufacturer Identification Register\n resetvalue={System Reset:0x1820}"]
pub type Manid = crate::RegValueT<Manid_SPEC>;

impl Manid {
    #[doc = "Department Identification Number   DEPT.   00 H   indicates the Automotive  amp  Industrial microcontroller department within Infineon Technologies."]
    #[inline(always)]
    pub fn dept(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, Manid_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, Manid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Manufacturer Identification Number   MANUF. This is a JEDEC normalized manufacturer code. MANUF   C1 H stands for Infineon Technologies."]
    #[inline(always)]
    pub fn manuf(
        self,
    ) -> crate::common::RegisterField<5, 0x7ff, 1, 0, u16, Manid_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x7ff,1,0,u16, Manid_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Manid {
    #[inline(always)]
    fn default() -> Manid {
        <crate::RegValueT<Manid_SPEC> as RegisterValue<_>>::new(6176)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modtrimcon0_SPEC;
impl crate::sealed::RegSpec for Modtrimcon0_SPEC {
    type DataType = u32;
}
#[doc = "Modulation Trim 0 Configuration Register\n resetvalue={PowerOn Reset:0x0,After SSW execution:0x0C000000}"]
pub type Modtrimcon0 = crate::RegValueT<Modtrimcon0_SPEC>;

impl Modtrimcon0 {
    #[doc = "XCORR Trim Value   RND GAIN EXP. this bit field generates output spll rnd gain exp o ."]
    #[inline(always)]
    pub fn rnd_gain_exp(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Modtrimcon0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Modtrimcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "XCORR GAIN Trim Value   XCORR GAIN. this bit field generates output spll xcorr gain o ."]
    #[inline(always)]
    pub fn xcorr_gain(
        self,
    ) -> crate::common::RegisterField<10, 0x7, 1, 0, u8, Modtrimcon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x7,1,0,u8, Modtrimcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "XCORR TARGET Trim Value   XCORR TARGET. this bit field generates output spll xcorr target o ."]
    #[inline(always)]
    pub fn xcorr_target(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Modtrimcon0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Modtrimcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Hysteresis Calibration Value   HYS CAL. this bit field generates output spll hysteresis cal o ."]
    #[inline(always)]
    pub fn hys_cal(
        self,
    ) -> crate::common::RegisterField<26, 0xf, 1, 0, u8, Modtrimcon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0xf,1,0,u8, Modtrimcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alpha Beta Calibration enable   AB CAL EN. this bit field generates output spll en alpha beta cal o ."]
    #[inline(always)]
    pub fn ab_cal_en(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Modtrimcon0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,Modtrimcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Calibration enable   CAL EN. this bit field generates output spll cal enable o ."]
    #[inline(always)]
    pub fn cal_en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Modtrimcon0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,Modtrimcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Modtrimcon0 {
    #[inline(always)]
    fn default() -> Modtrimcon0 {
        <crate::RegValueT<Modtrimcon0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modtrimcon1_SPEC;
impl crate::sealed::RegSpec for Modtrimcon1_SPEC {
    type DataType = u32;
}
#[doc = "Modulation Trim 1 Configuration Register\n resetvalue={PowerOn Reset:0x0}"]
pub type Modtrimcon1 = crate::RegValueT<Modtrimcon1_SPEC>;

impl Modtrimcon1 {
    #[doc = "XCORR TOL Trim Value   XCORR TOL. this bit field generates output spll xcorr tol o ."]
    #[inline(always)]
    pub fn xcorr_tol(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Modtrimcon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Modtrimcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DELTA RND GAIN Trim Value   DELTA RND GAIN. this bit field generates output spll delta rnd gain o ."]
    #[inline(always)]
    pub fn delta_rnd_gain(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Modtrimcon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Modtrimcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "XCORR AVG Selection   XCORR AVG SEL. this bit field generates output spll xcorr avg sel o ."]
    #[inline(always)]
    pub fn xcorr_avg_sel(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Modtrimcon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Modtrimcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "XCORR SEL Delay   XCORR SEL DELAY. this bit field generates output spll xcorr sel delay o ."]
    #[inline(always)]
    pub fn xcorr_sel_delay(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, Modtrimcon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,u8, Modtrimcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "System PLL EMC delay enable   SYSEMCDELAY"]
    #[inline(always)]
    pub fn sysemcdelay(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, Modtrimcon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, Modtrimcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral PLL EMC delay enable   PEREMCDELAY"]
    #[inline(always)]
    pub fn peremcdelay(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, Modtrimcon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3,1,0,u8, Modtrimcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Modtrimcon1 {
    #[inline(always)]
    fn default() -> Modtrimcon1 {
        <crate::RegValueT<Modtrimcon1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modtrimstat_SPEC;
impl crate::sealed::RegSpec for Modtrimstat_SPEC {
    type DataType = u32;
}
#[doc = "Modulation Trim Status Register\n resetvalue={PowerOn Reset:0x0}"]
pub type Modtrimstat = crate::RegValueT<Modtrimstat_SPEC>;

impl Modtrimstat {
    #[doc = "XCORR Result   XCORR. this bit field displays input spll xcorr i ."]
    #[inline(always)]
    pub fn xcorr(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Modtrimstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Modtrimstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "XCORR Valid Result   XCORR VALID. this bit field displays input spll xcorr valid i ."]
    #[inline(always)]
    pub fn xcorr_valid(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Modtrimstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,Modtrimstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "XCORR to Low Result   XCORR2LOW. this bit field displays input spll xcorr2low i ."]
    #[inline(always)]
    pub fn xcorr2low(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Modtrimstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16,1,0,Modtrimstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "XCORR to High Result   XCORR2HIGH. this bit field displays input spll xcorr2high i ."]
    #[inline(always)]
    pub fn xcorr2high(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Modtrimstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17,1,0,Modtrimstat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Modtrimstat {
    #[inline(always)]
    fn default() -> Modtrimstat {
        <crate::RegValueT<Modtrimstat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Omr_SPEC;
impl crate::sealed::RegSpec for Omr_SPEC {
    type DataType = u32;
}
#[doc = "ESR Output Modification Register\n resetvalue={System Reset:0x0}"]
pub type Omr = crate::RegValueT<Omr_SPEC>;

impl Omr {
    #[doc = "ESRx Pin Set Bit 1   PS1. Setting this bit will set or toggle the corresponding bit in the output        register OUT. Reading this bit returns 0."]
    #[inline(always)]
    pub fn ps0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "ESRx Pin Set Bit 1   PS1. Setting this bit will set or toggle the corresponding bit in the output        register OUT. Reading this bit returns 0."]
    #[inline(always)]
    pub fn ps1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "ESRx Pin Clear Bit 1   PCL1. Setting this bit will clear or toggle the corresponding bit in the port        output register OUT. Reading this bit returns 0."]
    #[inline(always)]
    pub fn pcl0(self) -> crate::common::RegisterFieldBool<16, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "ESRx Pin Clear Bit 1   PCL1. Setting this bit will clear or toggle the corresponding bit in the port        output register OUT. Reading this bit returns 0."]
    #[inline(always)]
    pub fn pcl1(self) -> crate::common::RegisterFieldBool<17, 1, 0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Omr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Omr {
    #[inline(always)]
    fn default() -> Omr {
        <crate::RegValueT<Omr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osccon_SPEC;
impl crate::sealed::RegSpec for Osccon_SPEC {
    type DataType = u32;
}
#[doc = "OSC Control Register\n resetvalue={System Reset:0x258,System Reset:0x10}"]
pub type Osccon = crate::RegValueT<Osccon_SPEC>;

impl Osccon {
    #[doc = "Oscillator for PLL Valid Low Status Bit   PLLLV. This bit indicates if the frequency output f osc of the oscillator is above the lower threshold frequency f LV   i.e. usable for the DCO part of the PLL. This is checked by the Oscillator Watchdog of the PLL using f BACK . f LV   f oscref   0 96   0 31  f BACKT"]
    #[inline(always)]
    pub fn plllv(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, osccon::Plllv, Osccon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,osccon::Plllv, Osccon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Oscillator Watchdog Reset   OSCRES. Always read as zero."]
    #[inline(always)]
    pub fn oscres(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, osccon::Oscres, Osccon_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<2,0x1,1,0,osccon::Oscres, Osccon_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Oscillator Gain Selection   GAINSEL. In Normal Mode this value should not be changed from the reset value 11 B . When using Vext 3.3V  the LVDS bias distributor has to be adjusted to 3.3V supply via P21 LPCR2.PS   0 otherwise the oscillator gain can be too low for a reliable oscillator startup at cold temperature. In case of using Vext 5V  the LVDS bias distributor setting stays at the reset value P21 LPCR2.PS   1."]
    #[inline(always)]
    pub fn gainsel(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, osccon::Gainsel, Osccon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x3,1,0,osccon::Gainsel, Osccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Oscillator Mode   MODE. This bit field defines which mode can be used and if the oscillator entered the Power Saving Mode or not. In Test Mode the shaper can be bypassed even if the oscillator is in the Oscillator Power Saving Mode. This is controlled by the TCU."]
    #[inline(always)]
    pub fn mode(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, osccon::Mode, Osccon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x3,1,0,osccon::Mode, Osccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Shaper Bypass   SHBY"]
    #[inline(always)]
    pub fn shby(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, osccon::Shby, Osccon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,osccon::Shby, Osccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Oscillator for PLL Valid High Status Bit   PLLHV. This bit indicates if the frequency output f osc of the oscillator is below the upper threshold frequency f HV   i.e. usable for the DCO part of the PLL. This is checked by the Oscillator Watchdog of the PLL using f BACK . f HV   f oscref   1 04   0 29  f BACKT"]
    #[inline(always)]
    pub fn pllhv(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, osccon::Pllhv, Osccon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x1,1,0,osccon::Pllhv, Osccon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Hysteresis Enable"]
    #[inline(always)]
    pub fn hysen(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, osccon::Hysen, Osccon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,osccon::Hysen, Osccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Hysteresis Control"]
    #[inline(always)]
    pub fn hysctl(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, osccon::Hysctl, Osccon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,osccon::Hysctl, Osccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Amplitude Control"]
    #[inline(always)]
    pub fn ampctl(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, osccon::Ampctl, Osccon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,osccon::Ampctl, Osccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OSC Frequency Value   OSCVAL. This bit field defines the divider value that generates the reference        clock that is supervised by the oscillator watchdog. f OSC   160    160 OSCCON.OSCVAL  160    160 1  160    160 16  160 MHz."]
    #[inline(always)]
    pub fn oscval(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Osccon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Osccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Amplitude Regulation Enable   APREN. This bit field enables and disables Amplitude Regulation mode. When enabled  the bit field GAINSEL limits the maximum gain."]
    #[inline(always)]
    pub fn apren(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, osccon::Apren, Osccon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,osccon::Apren, Osccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Capacitance 0 Enable   CAP0EN. Total capacitance for each XTAL1 and XTAL2 is the sum of the enabled capacitance 0 to 3."]
    #[inline(always)]
    pub fn cap0en(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, osccon::Cap0En, Osccon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,osccon::Cap0En, Osccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Capacitance 1 Enable   CAP1EN. Total capacitance for each XTAL1 and XTAL2 is the sum of the enabled capacitance 0 to 3."]
    #[inline(always)]
    pub fn cap1en(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, osccon::Cap1En, Osccon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,osccon::Cap1En, Osccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Capacitance 2 Enable   CAP2EN. Total capacitance for each XTAL1 and XTAL2 is the sum of the enabled capacitance 0 to 3."]
    #[inline(always)]
    pub fn cap2en(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, osccon::Cap2En, Osccon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,osccon::Cap2En, Osccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Capacitance 3 Enable   CAP3EN. Total capacitance for each XTAL1 and XTAL2 is the sum of the enabled capacitance 0 to 3."]
    #[inline(always)]
    pub fn cap3en(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, osccon::Cap3En, Osccon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,osccon::Cap3En, Osccon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Osccon {
    #[inline(always)]
    fn default() -> Osccon {
        <crate::RegValueT<Osccon_SPEC> as RegisterValue<_>>::new(16)
    }
}
pub mod osccon {
    pub struct Plllv_SPEC;
    pub type Plllv = crate::EnumBitfieldStruct<u8, Plllv_SPEC>;
    impl Plllv {
        #[doc = "The OSC frequency is not usable. Frequency f OSC is too low."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "The OSC frequency is usable"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oscres_SPEC;
    pub type Oscres = crate::EnumBitfieldStruct<u8, Oscres_SPEC>;
    impl Oscres {
        #[doc = "The Oscillator Watchdog of the PLL is not cleared and remains active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "The Oscillator Watchdog of the PLL is cleared and restarted"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Gainsel_SPEC;
    pub type Gainsel = crate::EnumBitfieldStruct<u8, Gainsel_SPEC>;
    impl Gainsel {
        #[doc = "Maximum gain configuration"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Mode_SPEC;
    pub type Mode = crate::EnumBitfieldStruct<u8, Mode_SPEC>;
    impl Mode {
        #[doc = "External Crystal   Ceramic Resonator Mode and External Input Clock Mode. The oscillator Power Saving Mode is not entered."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "OSC is disabled. The oscillator Power Saving Mode is not entered."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "External Input Clock Mode and the oscillator Power Saving Mode is entered"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "OSC is disabled. The oscillator Power Saving Mode is entered."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Shby_SPEC;
    pub type Shby = crate::EnumBitfieldStruct<u8, Shby_SPEC>;
    impl Shby {
        #[doc = "The shaper is not bypassed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "The shaper is bypassed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pllhv_SPEC;
    pub type Pllhv = crate::EnumBitfieldStruct<u8, Pllhv_SPEC>;
    impl Pllhv {
        #[doc = "The OSC        frequency is not usable. Frequency f OSC is too high."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "The OSC frequency is usable"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hysen_SPEC;
    pub type Hysen = crate::EnumBitfieldStruct<u8, Hysen_SPEC>;
    impl Hysen {
        #[doc = "Hysteresis is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Hysteresis is enabled  recommended"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hysctl_SPEC;
    pub type Hysctl = crate::EnumBitfieldStruct<u8, Hysctl_SPEC>;
    impl Hysctl {
        #[doc = "Hysteresis setting 1  highest hysteresis  default"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Ampctl_SPEC;
    pub type Ampctl = crate::EnumBitfieldStruct<u8, Ampctl_SPEC>;
    impl Ampctl {
        #[doc = "Amplitude control setting 1  default value"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Apren_SPEC;
    pub type Apren = crate::EnumBitfieldStruct<u8, Apren_SPEC>;
    impl Apren {
        #[doc = "Amplitude Regulation is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Amplitude Regulation is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cap0En_SPEC;
    pub type Cap0En = crate::EnumBitfieldStruct<u8, Cap0En_SPEC>;
    impl Cap0En {
        #[doc = "Capacitance C L0 is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Capacitance C L0 is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cap1En_SPEC;
    pub type Cap1En = crate::EnumBitfieldStruct<u8, Cap1En_SPEC>;
    impl Cap1En {
        #[doc = "Capacitance C L1 is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Capacitance C L1 is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cap2En_SPEC;
    pub type Cap2En = crate::EnumBitfieldStruct<u8, Cap2En_SPEC>;
    impl Cap2En {
        #[doc = "Capacitance C L2 is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Capacitance C L2 is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cap3En_SPEC;
    pub type Cap3En = crate::EnumBitfieldStruct<u8, Cap3En_SPEC>;
    impl Cap3En {
        #[doc = "Capacitance C L3 is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Capacitance C L3 is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Out_SPEC;
impl crate::sealed::RegSpec for Out_SPEC {
    type DataType = u32;
}
#[doc = "ESR Output Register\n resetvalue={System Reset:0x0}"]
pub type Out = crate::RegValueT<Out_SPEC>;

impl Out {
    #[doc = "Output Bit 1   P1. This bit determines the level at the output pin ESR x if the output is selected as GPIO output. Px can also be set cleared by control bits of the OMR register."]
    #[inline(always)]
    pub fn p0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, out::P0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,out::P0, Out_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Output Bit 1   P1. This bit determines the level at the output pin ESR x if the output is selected as GPIO output. Px can also be set cleared by control bits of the OMR register."]
    #[inline(always)]
    pub fn p1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, out::P1, Out_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,out::P1, Out_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Out {
    #[inline(always)]
    fn default() -> Out {
        <crate::RegValueT<Out_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod out {
    pub struct P0_SPEC;
    pub type P0 = crate::EnumBitfieldStruct<u8, P0_SPEC>;
    impl P0 {
        #[doc = "0 The output level of ESRx is 0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The output level of ESR x is 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct P1_SPEC;
    pub type P1 = crate::EnumBitfieldStruct<u8, P1_SPEC>;
    impl P1 {
        #[doc = "0 The output level of ESRx is 0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The output level of ESR x is 1"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ovccon_SPEC;
impl crate::sealed::RegSpec for Ovccon_SPEC {
    type DataType = u32;
}
#[doc = "Overlay Control Register\n resetvalue={Application Reset:0x0}"]
pub type Ovccon = crate::RegValueT<Ovccon_SPEC>;

impl Ovccon {
    #[doc = "CPU Select 0   CSEL0. Return 0 if read."]
    #[inline(always)]
    pub fn csel0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ovccon::Csel0, Ovccon_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x1,1,0,ovccon::Csel0, Ovccon_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "CPU Select 1  If product has CPU1    CSEL1. Return 0 if read."]
    #[inline(always)]
    pub fn csel1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ovccon::Csel1, Ovccon_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<1,0x1,1,0,ovccon::Csel1, Ovccon_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "CPU Select 2  If product has CPU2    CSEL2. Return 0 if read."]
    #[inline(always)]
    pub fn csel2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ovccon::Csel2, Ovccon_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<2,0x1,1,0,ovccon::Csel2, Ovccon_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Overlay Start   OVSTRT. CPUs which are not selected are not affected. No action is taken if OVSTP is also set. Return 0 if read."]
    #[inline(always)]
    pub fn ovstrt(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, ovccon::Ovstrt, Ovccon_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0x1,1,0,ovccon::Ovstrt, Ovccon_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Overlay Stop   OVSTP. CPUs which are not selected are not affected No action is taken if OVSTRT is also set. Return 0 if read."]
    #[inline(always)]
    pub fn ovstp(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, ovccon::Ovstp, Ovccon_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<17,0x1,1,0,ovccon::Ovstp, Ovccon_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Data Cache Invalidate   DCINVAL. No function in devices without data cache in CPU. This bit  when set  generates a pulse on signal ... ovc dcinval. Data Cache is affected only in the CPUs selected with CSEL. Return 0 if read."]
    #[inline(always)]
    pub fn dcinval(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, ovccon::Dcinval, Ovccon_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<18,0x1,1,0,ovccon::Dcinval, Ovccon_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Overlay Configured   OVCONF. Overlay configured status bit This bit may be used as handshake bit between a debug device  via JTAG        interface and Cerberus  and CPU s ."]
    #[inline(always)]
    pub fn ovconf(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, ovccon::Ovconf, Ovccon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,ovccon::Ovconf, Ovccon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Write Protection for OVCONF   POVCONF. This bit enables OVCONF write during OVCCON write. Return 0 if read."]
    #[inline(always)]
    pub fn povconf(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, ovccon::Povconf, Ovccon_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<25,0x1,1,0,ovccon::Povconf, Ovccon_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Ovccon {
    #[inline(always)]
    fn default() -> Ovccon {
        <crate::RegValueT<Ovccon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ovccon {
    pub struct Csel0_SPEC;
    pub type Csel0 = crate::EnumBitfieldStruct<u8, Csel0_SPEC>;
    impl Csel0 {
        #[doc = "0 CPU0 not affected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Action selected by OVSTRT  OVSTP  DCINVAL bits  set by the same register write access  is applied to CPU0."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Csel1_SPEC;
    pub type Csel1 = crate::EnumBitfieldStruct<u8, Csel1_SPEC>;
    impl Csel1 {
        #[doc = "0 CPU1 not affected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Action selected by OVSTRT  OVSTP  DCINVAL bits  set by the same register write access  is applied to CPU1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Csel2_SPEC;
    pub type Csel2 = crate::EnumBitfieldStruct<u8, Csel2_SPEC>;
    impl Csel2 {
        #[doc = "0 CPU2 not affected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Action selected by OVSTRT  OVSTP  DCINVAL bits  set by the same register write access  is applied to CPU2."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ovstrt_SPEC;
    pub type Ovstrt = crate::EnumBitfieldStruct<u8, Ovstrt_SPEC>;
    impl Ovstrt {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 For each CPU selected with CSEL  all the blocks selected with OVCx OSEL will be activated. In the selected CPUs all the blocks deselected with OVCx OSEL will be deactivated."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ovstp_SPEC;
    pub type Ovstp = crate::EnumBitfieldStruct<u8, Ovstp_SPEC>;
    impl Ovstp {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 For CPUs selected with CSEL  all the overlay blocks are deactivated. OVCx RABRy.OVEN bits are cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Dcinval_SPEC;
    pub type Dcinval = crate::EnumBitfieldStruct<u8, Dcinval_SPEC>;
    impl Dcinval {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Data Cache Lines in DMI are invalidated Dirty  modified  cache lines are not effected by this operation. If data cache contains modified data  it is not invalidated  and has to be written back and invalidated by the user. Therefore  it is highly recommended to either  access overlaid data in read only mode  or use only non cached access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ovconf_SPEC;
    pub type Ovconf = crate::EnumBitfieldStruct<u8, Ovconf_SPEC>;
    impl Ovconf {
        #[doc = "0 Overlay is not configured or it has been already started"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Overlay block control registers are configured and ready for overlay start"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Povconf_SPEC;
    pub type Povconf = crate::EnumBitfieldStruct<u8, Povconf_SPEC>;
    impl Povconf {
        #[doc = "0 OVCONF remains unchanged."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 OVCONF can be changed with write access to register OVCCON"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ovcenable_SPEC;
impl crate::sealed::RegSpec for Ovcenable_SPEC {
    type DataType = u32;
}
#[doc = "Overlay Enable Register\n resetvalue={Application Reset:0x0}"]
pub type Ovcenable = crate::RegValueT<Ovcenable_SPEC>;

impl Ovcenable {
    #[doc = "Overlay Enable 0   OVEN0"]
    #[inline(always)]
    pub fn oven0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ovcenable::Oven0,
        Ovcenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ovcenable::Oven0,
            Ovcenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Overlay Enable 1  If product has CPU1    OVEN1"]
    #[inline(always)]
    pub fn oven1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        ovcenable::Oven1,
        Ovcenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            ovcenable::Oven1,
            Ovcenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Overlay Enable 2  If product has CPU2    OVEN2"]
    #[inline(always)]
    pub fn oven2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        ovcenable::Oven2,
        Ovcenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            ovcenable::Oven2,
            Ovcenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Ovcenable {
    #[inline(always)]
    fn default() -> Ovcenable {
        <crate::RegValueT<Ovcenable_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ovcenable {
    pub struct Oven0_SPEC;
    pub type Oven0 = crate::EnumBitfieldStruct<u8, Oven0_SPEC>;
    impl Oven0 {
        #[doc = "0 OVC is disabled on CPU0. All Overlay redirections are disabled regardless of the state of OVC0 RABRy.OVEN."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 OVC is enabled on CPU0."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oven1_SPEC;
    pub type Oven1 = crate::EnumBitfieldStruct<u8, Oven1_SPEC>;
    impl Oven1 {
        #[doc = "0 OVC is disabled on CPU1. All Overlay redirections are disabled regardless of the state of OVC1 RABRy.OVEN."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 OVC is enabled on CPU1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oven2_SPEC;
    pub type Oven2 = crate::EnumBitfieldStruct<u8, Oven2_SPEC>;
    impl Oven2 {
        #[doc = "0 OVC is disabled on CPU2. All Overlay redirections are disabled regardless of the state of OVC2 RABRy.OVEN."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 OVC is enabled on CPU2."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdisc_SPEC;
impl crate::sealed::RegSpec for Pdisc_SPEC {
    type DataType = u32;
}
#[doc = "Pad Disable Control Register\n resetvalue={System Reset:0x0}"]
pub type Pdisc = crate::RegValueT<Pdisc_SPEC>;

impl Pdisc {
    #[doc = "Pad Disable for ESR Pin 1   PDIS1. This bit disables the pad."]
    #[inline(always)]
    pub fn pdis0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, pdisc::Pdis0, Pdisc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,pdisc::Pdis0, Pdisc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Disable for ESR Pin 1   PDIS1. This bit disables the pad."]
    #[inline(always)]
    pub fn pdis1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, pdisc::Pdis1, Pdisc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,pdisc::Pdis1, Pdisc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pdisc {
    #[inline(always)]
    fn default() -> Pdisc {
        <crate::RegValueT<Pdisc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pdisc {
    pub struct Pdis0_SPEC;
    pub type Pdis0 = crate::EnumBitfieldStruct<u8, Pdis0_SPEC>;
    impl Pdis0 {
        #[doc = "0 Pad Px is enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Pad Px is disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pdis1_SPEC;
    pub type Pdis1 = crate::EnumBitfieldStruct<u8, Pdis1_SPEC>;
    impl Pdis1 {
        #[doc = "0 Pad Px is enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Pad Px is disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdr_SPEC;
impl crate::sealed::RegSpec for Pdr_SPEC {
    type DataType = u32;
}
#[doc = "ESR Pad Driver Mode Register\n resetvalue={System Reset:0x0}"]
pub type Pdr = crate::RegValueT<Pdr_SPEC>;

impl Pdr {
    #[doc = "Pad Driver Mode for ESR Pins 0   PD0.  See PDR register description in PORTs chapter"]
    #[inline(always)]
    pub fn pd0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Pdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0x3, 1, 0, u8, Pdr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pad Level Selection for ESR Pins 0   PL0.  See PDR register description in PORTs chapter"]
    #[inline(always)]
    pub fn pl0(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Pdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2, 0x3, 1, 0, u8, Pdr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pad Driver Mode for ESR Pins 1   PD1.  See PDR register description in PORTs chapter"]
    #[inline(always)]
    pub fn pd1(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Pdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4, 0x3, 1, 0, u8, Pdr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pad Level Selection for ESR Pins 1   PL1.  See PDR register description in PORTs chapter"]
    #[inline(always)]
    pub fn pl1(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Pdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6, 0x3, 1, 0, u8, Pdr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Pdr {
    #[inline(always)]
    fn default() -> Pdr {
        <crate::RegValueT<Pdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdrr_SPEC;
impl crate::sealed::RegSpec for Pdrr_SPEC {
    type DataType = u32;
}
#[doc = "Pattern Detection Result Register\n resetvalue={Application Reset:0x0FF}"]
pub type Pdrr = crate::RegValueT<Pdrr_SPEC>;

impl Pdrr {
    #[doc = "Pattern Detection Result of Channel 7   PDR7. This bit monitors the output status of the pattern detection for the output channel y."]
    #[inline(always)]
    pub fn pdr0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Pdrr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pdrr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pattern Detection Result of Channel 7   PDR7. This bit monitors the output status of the pattern detection for the output channel y."]
    #[inline(always)]
    pub fn pdr1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Pdrr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pdrr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pattern Detection Result of Channel 7   PDR7. This bit monitors the output status of the pattern detection for the output channel y."]
    #[inline(always)]
    pub fn pdr2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Pdrr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Pdrr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pattern Detection Result of Channel 7   PDR7. This bit monitors the output status of the pattern detection for the output channel y."]
    #[inline(always)]
    pub fn pdr3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Pdrr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Pdrr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pattern Detection Result of Channel 7   PDR7. This bit monitors the output status of the pattern detection for the output channel y."]
    #[inline(always)]
    pub fn pdr4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Pdrr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Pdrr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pattern Detection Result of Channel 7   PDR7. This bit monitors the output status of the pattern detection for the output channel y."]
    #[inline(always)]
    pub fn pdr5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Pdrr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Pdrr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pattern Detection Result of Channel 7   PDR7. This bit monitors the output status of the pattern detection for the output channel y."]
    #[inline(always)]
    pub fn pdr6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Pdrr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Pdrr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pattern Detection Result of Channel 7   PDR7. This bit monitors the output status of the pattern detection for the output channel y."]
    #[inline(always)]
    pub fn pdr7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Pdrr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Pdrr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Pdrr {
    #[inline(always)]
    fn default() -> Pdrr {
        <crate::RegValueT<Pdrr_SPEC> as RegisterValue<_>>::new(255)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Perpllcon0_SPEC;
impl crate::sealed::RegSpec for Perpllcon0_SPEC {
    type DataType = u32;
}
#[doc = "Peripheral PLL Configuration 0 Register\n resetvalue={System Reset:0x3E00}"]
pub type Perpllcon0 = crate::RegValueT<Perpllcon0_SPEC>;

impl Perpllcon0 {
    #[doc = "Divider Bypass   DIVBY"]
    #[inline(always)]
    pub fn divby(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        perpllcon0::Divby,
        Perpllcon0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            perpllcon0::Divby,
            Perpllcon0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "N Divider Value   NDIV. The value the N Divider operates with is NDIV 1."]
    #[inline(always)]
    pub fn ndiv(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, Perpllcon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8, Perpllcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral PLL Power Saving Mode   PLLPWD. If the PLL has been powered down and is getting re enabled via PLLPWD   1  a wait period of 1 ms has to be applied until it is stable without jitter."]
    #[inline(always)]
    pub fn pllpwd(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        perpllcon0::Pllpwd,
        Perpllcon0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            perpllcon0::Pllpwd,
            Perpllcon0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Restart DCO Lock Detection   RESLD. Setting this bit will clear bit SYSPLLSTAT.LOCK and restart the DCO lock detection. Reading this bit returns always a zero."]
    #[inline(always)]
    pub fn resld(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Perpllcon0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18,1,0,Perpllcon0_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "P Divider Value   PDIV. The value the P Divider operates with is PDIV 1."]
    #[inline(always)]
    pub fn pdiv(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Perpllcon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Perpllcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Perpllcon0 {
    #[inline(always)]
    fn default() -> Perpllcon0 {
        <crate::RegValueT<Perpllcon0_SPEC> as RegisterValue<_>>::new(15872)
    }
}
pub mod perpllcon0 {
    pub struct Divby_SPEC;
    pub type Divby = crate::EnumBitfieldStruct<u8, Divby_SPEC>;
    impl Divby {
        #[doc = "The divide by 1.6 block in front of the K3 Divider is not bypassed. The resulting        divider factor in front of the K3 Divider is f DCO   160    160 1.6."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "The divide by 1.6 block in front of the K3 Divider is bypassed. Taking into account        the fix by two divider in front the resulting divider factor in front of        the K3 Divider is f DCO   160    160 2."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pllpwd_SPEC;
    pub type Pllpwd = crate::EnumBitfieldStruct<u8, Pllpwd_SPEC>;
    impl Pllpwd {
        #[doc = "The complete Peripheral PLL block is put into a Power Saving Mode and can no longer be used. Bypass Mode remains active if previously selected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Normal behavior"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Perpllcon1_SPEC;
impl crate::sealed::RegSpec for Perpllcon1_SPEC {
    type DataType = u32;
}
#[doc = "Peripheral PLL Configuration 1 Register\n resetvalue={System Reset:0x1}"]
pub type Perpllcon1 = crate::RegValueT<Perpllcon1_SPEC>;

impl Perpllcon1 {
    #[doc = "K2 Divider Value   K2DIV. The value the K2 Divider operates with is K2DIV 1. While PERPLLSTAT.K2RDY   0  K2DIV is locked."]
    #[inline(always)]
    pub fn k2div(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Perpllcon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Perpllcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "K3 Divider Value   K3DIV. The value the K3 Divider operates with is K3DIV 1. While PERPLLSTAT.K3RDY   0  K3DIV is locked."]
    #[inline(always)]
    pub fn k3div(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Perpllcon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Perpllcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Perpllcon1 {
    #[inline(always)]
    fn default() -> Perpllcon1 {
        <crate::RegValueT<Perpllcon1_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Perpllstat_SPEC;
impl crate::sealed::RegSpec for Perpllstat_SPEC {
    type DataType = u32;
}
#[doc = "Peripheral PLL Status Register\n resetvalue={System Reset:0x2}"]
pub type Perpllstat = crate::RegValueT<Perpllstat_SPEC>;

impl Perpllstat {
    #[doc = "Peripheral PLL Power saving Mode Status   PWDSTAT"]
    #[inline(always)]
    pub fn pwdstat(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        perpllstat::Pwdstat,
        Perpllstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            perpllstat::Pwdstat,
            Perpllstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Peripheral PLL Lock Status   LOCK. In case of a loss of lock  the f DCO is kept on the previous constant frequency."]
    #[inline(always)]
    pub fn lock(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        perpllstat::Lock,
        Perpllstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            perpllstat::Lock,
            Perpllstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "K3 Divider Ready Status   K3RDY. This bit indicates whether the K3 divider operates on the configured value. This is of interest when the PERPLLCON1.K3DIV value is changed. The PLL must be enabled and clocked to set the K3RDY field."]
    #[inline(always)]
    pub fn k3rdy(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        perpllstat::K3Rdy,
        Perpllstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            perpllstat::K3Rdy,
            Perpllstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "K2 Divider Ready Status   K2RDY. This bit indicates whether the K2 divider operates on the configured value. This is of interest when the PERPLLCON1.K2DIV value is changed. The PLL must be enabled and clocked to set the K2RDY field."]
    #[inline(always)]
    pub fn k2rdy(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        perpllstat::K2Rdy,
        Perpllstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            perpllstat::K2Rdy,
            Perpllstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Bypass Mode Status   BY. The Bypass Mode can only be entered in Test Mode by the TCU. Therefore this bit is only valid in Test Mode."]
    #[inline(always)]
    pub fn by(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, perpllstat::By, Perpllstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            perpllstat::By,
            Perpllstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Perpllstat {
    #[inline(always)]
    fn default() -> Perpllstat {
        <crate::RegValueT<Perpllstat_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod perpllstat {
    pub struct Pwdstat_SPEC;
    pub type Pwdstat = crate::EnumBitfieldStruct<u8, Pwdstat_SPEC>;
    impl Pwdstat {
        #[doc = "Peripheral PLL Power saving Mode was not entered"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Peripheral PLL Power saving Mode was entered"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lock_SPEC;
    pub type Lock = crate::EnumBitfieldStruct<u8, Lock_SPEC>;
    impl Lock {
        #[doc = "The frequency of the Peripheral PLL is not stable and doesn  x2019 t enable system operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "The frequency of the Peripheral PLL is stable and enables system operation"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct K3Rdy_SPEC;
    pub type K3Rdy = crate::EnumBitfieldStruct<u8, K3Rdy_SPEC>;
    impl K3Rdy {
        #[doc = "K3 Divider does not yet operate with the new value"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "K3 Divider operating with the new value"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct K2Rdy_SPEC;
    pub type K2Rdy = crate::EnumBitfieldStruct<u8, K2Rdy_SPEC>;
    impl K2Rdy {
        #[doc = "K2 Divider does not yet operate with the new value"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "K2 Divider operating with the new value"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct By_SPEC;
    pub type By = crate::EnumBitfieldStruct<u8, By_SPEC>;
    impl By {
        #[doc = "Bypass Mode is not entered"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Bypass Mode is entered. Input f OSC is selected as output f PLL1   2 ."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pheatctrl_SPEC;
impl crate::sealed::RegSpec for Pheatctrl_SPEC {
    type DataType = u32;
}
#[doc = "Pad Heating Control Register\n resetvalue={System Reset:0x0}"]
pub type Pheatctrl = crate::RegValueT<Pheatctrl_SPEC>;

impl Pheatctrl {
    #[doc = "Pad Heating Enable for Supply Pad x   Px HEATEN. This field allows to switch the pad heating on or off for each        individual supply pad if the pad heating test function is enabled inside        TCU  i.e. TEST TRC0.PADHEAT     8217 1  8217  . A high value will establish a        low resistive connection between VDD and GND thus causing a power        dissipation of  100mW for each selected supply pad. A low value sets the        corresponding supply pad to normal operation  default behavior . If        pad heating test function in TCU is not active  i.e. TEST TRC0.PADHEAT            8217 0  8217    this bit field has no effect. In this case the heating structure        in the supply pads is always switched off."]
    #[inline(always)]
    pub fn px_heaten(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffffff, 1, 0, u32, Pheatctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7fffffff,1,0,u32, Pheatctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Heating Activity Indicator   HEATON. Due to safety reasons it must be ensured  that the pad heating function is never active for any core supply pad during functional application due to any soft defect. Therefore the pad heating enable signal of all core supply pads is connected into an OR tree  wired across the pad cells  and the result is observable through this bit. Write transactions to this bit will not have any effect."]
    #[inline(always)]
    pub fn heaton(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        pheatctrl::Heaton,
        Pheatctrl_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            pheatctrl::Heaton,
            Pheatctrl_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Pheatctrl {
    #[inline(always)]
    fn default() -> Pheatctrl {
        <crate::RegValueT<Pheatctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pheatctrl {
    pub struct Heaton_SPEC;
    pub type Heaton = crate::EnumBitfieldStruct<u8, Heaton_SPEC>;
    impl Heaton {
        #[doc = "0 The pad heating funcion is inactive in all core supply pads."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The pad heating funcion is currently activated in one or several core supply pads."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Plltrim_SPEC;
impl crate::sealed::RegSpec for Plltrim_SPEC {
    type DataType = u32;
}
#[doc = "PLLTrim Register\n resetvalue={PowerOn Reset:0x080008000,CFS Value:0x080008000}"]
pub type Plltrim = crate::RegValueT<Plltrim_SPEC>;

impl Plltrim {
    #[doc = "System PLL IVR Trim Value   SYSPLLIVR. this bit field generates output spll ivr adj o ."]
    #[inline(always)]
    pub fn syspllivr(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Plltrim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Plltrim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "System PLL Alpha Trim Value   SYSALPHA. this bit field generates output spll alpha o ."]
    #[inline(always)]
    pub fn sysalpha(
        self,
    ) -> crate::common::RegisterField<4, 0x1f, 1, 0, u8, Plltrim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1f,1,0,u8, Plltrim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "System PLL Beta Trim Value   SYSBETA. this bit field generates output spll beta o ."]
    #[inline(always)]
    pub fn sysbeta(
        self,
    ) -> crate::common::RegisterField<9, 0x1f, 1, 0, u8, Plltrim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1f,1,0,u8, Plltrim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "System PLL Alpha and Beta Trim enable   SYSABEN. this bit field generates output spll set IFparams o ."]
    #[inline(always)]
    pub fn sysaben(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Plltrim_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Plltrim_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "System PLL IVR reference voltage filter enable. this bit field generates output spll ivr filt dis o ."]
    #[inline(always)]
    pub fn sysivr_filt_dis(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        plltrim::SysivrFiltDis,
        Plltrim_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            plltrim::SysivrFiltDis,
            Plltrim_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Peripheral PLL IVR Trim Value   PERPLLIVR. this bit field generates output ppll ivr adj o ."]
    #[inline(always)]
    pub fn perpllivr(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Plltrim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Plltrim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral PLL Alpha Trim Value   PERALPHA. this bit field generates output ppll alpha o ."]
    #[inline(always)]
    pub fn peralpha(
        self,
    ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, Plltrim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x1f,1,0,u8, Plltrim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral PLL Beta Trim Value   PERBETA. this bit field generates output ppll beta o ."]
    #[inline(always)]
    pub fn perbeta(
        self,
    ) -> crate::common::RegisterField<25, 0x1f, 1, 0, u8, Plltrim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<25,0x1f,1,0,u8, Plltrim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Peripheral PLL Alpha and Beta Trim enable   PERABEN. this bit field generates output ppll set IFparams o ."]
    #[inline(always)]
    pub fn peraben(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Plltrim_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Plltrim_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Peripheral PLL IVR reference voltage filter disable. this bit field generates output ppll ivr filt dis o ."]
    #[inline(always)]
    pub fn perivr_filt_dis(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        plltrim::PerivrFiltDis,
        Plltrim_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            plltrim::PerivrFiltDis,
            Plltrim_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Plltrim {
    #[inline(always)]
    fn default() -> Plltrim {
        <crate::RegValueT<Plltrim_SPEC> as RegisterValue<_>>::new(2147516416)
    }
}
pub mod plltrim {
    pub struct SysivrFiltDis_SPEC;
    pub type SysivrFiltDis = crate::EnumBitfieldStruct<u8, SysivrFiltDis_SPEC>;
    impl SysivrFiltDis {
        #[doc = "Filter disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Filter enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct PerivrFiltDis_SPEC;
    pub type PerivrFiltDis = crate::EnumBitfieldStruct<u8, PerivrFiltDis_SPEC>;
    impl PerivrFiltDis {
        #[doc = "Filter disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Filter enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmcsr0_SPEC;
impl crate::sealed::RegSpec for Pmcsr0_SPEC {
    type DataType = u32;
}
#[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
pub type Pmcsr0 = crate::RegValueT<Pmcsr0_SPEC>;

impl Pmcsr0 {
    #[doc = "Idle Mode and Sleep Mode Request   REQSLP. In Idle Mode or Sleep Mode  these bits are cleared in response to an        interrupt for the CPU  or when bit 15 of the corresponding CPU Watchdog        Timer register  bit WDTCPUxSR.TIM 15   changes from 0 to 1. In Standby        Mode  these bits are cleared on wake up. REQSLP maybe written only when        either CPU or Safety ENDINIT bits are set to 0. CPU ENDINIT bit has to        be set back after REQSLP is written for the mode transition to take        place. In case of Safety ENDINIT  the mode transition will be issued        immediately and does not wait till Safety ENDINIT is set back to 1 again. No explicit acknowledge to SCU on System Standby request. System Standby        entry in a few cycles."]
    #[inline(always)]
    pub fn reqslp(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, pmcsr0::Reqslp, Pmcsr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,pmcsr0::Reqslp, Pmcsr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Power management Status   PMST. This bit field reflects the current status of the CPU."]
    #[inline(always)]
    pub fn pmst(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, pmcsr0::Pmst, Pmcsr0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x7,1,0,pmcsr0::Pmst, Pmcsr0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pmcsr0 {
    #[inline(always)]
    fn default() -> Pmcsr0 {
        <crate::RegValueT<Pmcsr0_SPEC> as RegisterValue<_>>::new(256)
    }
}
pub mod pmcsr0 {
    pub struct Reqslp_SPEC;
    pub type Reqslp = crate::EnumBitfieldStruct<u8, Reqslp_SPEC>;
    impl Reqslp {
        #[doc = "00 Request CPU Run Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Request CPU Idle Mode"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Request System Sleep Mode"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Request System Standby Mode"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Pmst_SPEC;
    pub type Pmst = crate::EnumBitfieldStruct<u8, Pmst_SPEC>;
    impl Pmst {
        #[doc = "001 Normal Run Mode After a reset  all CPUs are in  Normal Run Mode   but this does not mean that all CPUs are executing code. This mode also includes the CPU  halt  mode which is the start up default for all except CPU0."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 CPU Idle Mode requested"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 CPU Idle Mode acknowledged"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 Sleep Mode requested"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "110 Standby Mode requested"]
        pub const CONST_66: Self = Self::new(6);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmcsr1_SPEC;
impl crate::sealed::RegSpec for Pmcsr1_SPEC {
    type DataType = u32;
}
#[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
pub type Pmcsr1 = crate::RegValueT<Pmcsr1_SPEC>;

impl Pmcsr1 {
    #[doc = "Idle Mode and Sleep Mode Request   REQSLP. In Idle Mode or Sleep Mode  these bits are cleared in response to an interrupt for the CPU  or when bit 15 of the corresponding CPU Watchdog Timer register  bit WDTCPUxSR.TIM 15   changes from 0 to 1. In Standby Mode  these bits are cleared on wake up. REQSLP maybe written only when either CPU or Safety ENDINIT bits are set to 0. CPU ENDINIT bit has to be set back after REQSLP is written for the mode transition to take place. In case of Safety ENDINIT  the mode transition will be issued immediately and does not wait till Safety ENDINIT is set back to 1 again. No explicit acknowledge to SCU on System Standby request. System Standby entry in a few cycles."]
    #[inline(always)]
    pub fn reqslp(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, pmcsr1::Reqslp, Pmcsr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,pmcsr1::Reqslp, Pmcsr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Power management Status   PMST. This bit field reflects the current status of the CPU."]
    #[inline(always)]
    pub fn pmst(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, pmcsr1::Pmst, Pmcsr1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x7,1,0,pmcsr1::Pmst, Pmcsr1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pmcsr1 {
    #[inline(always)]
    fn default() -> Pmcsr1 {
        <crate::RegValueT<Pmcsr1_SPEC> as RegisterValue<_>>::new(256)
    }
}
pub mod pmcsr1 {
    pub struct Reqslp_SPEC;
    pub type Reqslp = crate::EnumBitfieldStruct<u8, Reqslp_SPEC>;
    impl Reqslp {
        #[doc = "00 Request CPU Run Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Request CPU Idle Mode"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Request System Sleep Mode"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Request System Standby Mode"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Pmst_SPEC;
    pub type Pmst = crate::EnumBitfieldStruct<u8, Pmst_SPEC>;
    impl Pmst {
        #[doc = "001 Normal Run Mode After a reset  all CPUs are in  Normal Run Mode   but this does not mean that all CPUs are executing code. This mode also includes the CPU  halt  mode which is the start up default for all except CPU0."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 CPU Idle Mode requested"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 CPU Idle Mode acknowledged"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 Sleep Mode requested"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "110 Standby Mode requested"]
        pub const CONST_66: Self = Self::new(6);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmcsr2_SPEC;
impl crate::sealed::RegSpec for Pmcsr2_SPEC {
    type DataType = u32;
}
#[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
pub type Pmcsr2 = crate::RegValueT<Pmcsr2_SPEC>;

impl Pmcsr2 {
    #[doc = "Idle Mode and Sleep Mode Request   REQSLP. In Idle Mode or Sleep Mode  these bits are cleared in response to an interrupt for the CPU  or when bit 15 of the corresponding CPU Watchdog Timer register  bit WDTCPUxSR.TIM 15   changes from 0 to 1. In Standby Mode  these bits are cleared on wake up. REQSLP maybe written only when either CPU or Safety ENDINIT bits are set to 0. CPU ENDINIT bit has to be set back after REQSLP is written for the mode transition to take place. In case of Safety ENDINIT  the mode transition will be issued immediately and does not wait till Safety ENDINIT is set back to 1 again. No explicit acknowledge to SCU on System Standby request. System Standby entry in a few cycles."]
    #[inline(always)]
    pub fn reqslp(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, pmcsr2::Reqslp, Pmcsr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,pmcsr2::Reqslp, Pmcsr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Power management Status   PMST. This bit field reflects the current status of the CPU."]
    #[inline(always)]
    pub fn pmst(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, pmcsr2::Pmst, Pmcsr2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x7,1,0,pmcsr2::Pmst, Pmcsr2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pmcsr2 {
    #[inline(always)]
    fn default() -> Pmcsr2 {
        <crate::RegValueT<Pmcsr2_SPEC> as RegisterValue<_>>::new(256)
    }
}
pub mod pmcsr2 {
    pub struct Reqslp_SPEC;
    pub type Reqslp = crate::EnumBitfieldStruct<u8, Reqslp_SPEC>;
    impl Reqslp {
        #[doc = "00 Request CPU Run Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Request CPU Idle Mode"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Request System Sleep Mode"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Request System Standby Mode"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Pmst_SPEC;
    pub type Pmst = crate::EnumBitfieldStruct<u8, Pmst_SPEC>;
    impl Pmst {
        #[doc = "001 Normal Run Mode After a reset  all CPUs are in  Normal Run Mode   but this does not mean that all CPUs are executing code. This mode also includes the CPU  halt  mode which is the start up default for all except CPU0."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 CPU Idle Mode requested"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 CPU Idle Mode acknowledged"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 Sleep Mode requested"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "110 Standby Mode requested"]
        pub const CONST_66: Self = Self::new(6);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmcsr3_SPEC;
impl crate::sealed::RegSpec for Pmcsr3_SPEC {
    type DataType = u32;
}
#[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
pub type Pmcsr3 = crate::RegValueT<Pmcsr3_SPEC>;

impl Pmcsr3 {
    #[doc = "Idle Mode and Sleep Mode Request   REQSLP. In Idle Mode or Sleep Mode  these bits are cleared in response to an interrupt for the CPU  or when bit 15 of the corresponding CPU Watchdog Timer register  bit WDTCPUxSR.TIM 15   changes from 0 to 1. In Standby Mode  these bits are cleared on wake up. REQSLP maybe written only when either CPU or Safety ENDINIT bits are set to 0. CPU ENDINIT bit has to be set back after REQSLP is written for the mode transition to take place. In case of Safety ENDINIT  the mode transition will be issued immediately and does not wait till Safety ENDINIT is set back to 1 again. No explicit acknowledge to SCU on System Standby request. System Standby entry in a few cycles."]
    #[inline(always)]
    pub fn reqslp(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, pmcsr3::Reqslp, Pmcsr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,pmcsr3::Reqslp, Pmcsr3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Power management Status   PMST. This bit field reflects the current status of the CPU."]
    #[inline(always)]
    pub fn pmst(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, pmcsr3::Pmst, Pmcsr3_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x7,1,0,pmcsr3::Pmst, Pmcsr3_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pmcsr3 {
    #[inline(always)]
    fn default() -> Pmcsr3 {
        <crate::RegValueT<Pmcsr3_SPEC> as RegisterValue<_>>::new(256)
    }
}
pub mod pmcsr3 {
    pub struct Reqslp_SPEC;
    pub type Reqslp = crate::EnumBitfieldStruct<u8, Reqslp_SPEC>;
    impl Reqslp {
        #[doc = "00 Request CPU Run Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Request CPU Idle Mode"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Request System Sleep Mode"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Request System Standby Mode"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Pmst_SPEC;
    pub type Pmst = crate::EnumBitfieldStruct<u8, Pmst_SPEC>;
    impl Pmst {
        #[doc = "001 Normal Run Mode After a reset  all CPUs are in  Normal Run Mode   but this does not mean that all CPUs are executing code. This mode also includes the CPU  halt  mode which is the start up default for all except CPU0."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 CPU Idle Mode requested"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 CPU Idle Mode acknowledged"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 Sleep Mode requested"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "110 Standby Mode requested"]
        pub const CONST_66: Self = Self::new(6);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmcsr4_SPEC;
impl crate::sealed::RegSpec for Pmcsr4_SPEC {
    type DataType = u32;
}
#[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
pub type Pmcsr4 = crate::RegValueT<Pmcsr4_SPEC>;

impl Pmcsr4 {
    #[doc = "Idle Mode and Sleep Mode Request   REQSLP. In Idle Mode or Sleep Mode  these bits are cleared in response to an interrupt for the CPU  or when bit 15 of the corresponding CPU Watchdog Timer register  bit WDTCPUxSR.TIM 15   changes from 0 to 1. In Standby Mode  these bits are cleared on wake up. REQSLP maybe written only when either CPU or Safety ENDINIT bits are set to 0. CPU ENDINIT bit has to be set back after REQSLP is written for the mode transition to take place. In case of Safety ENDINIT  the mode transition will be issued immediately and does not wait till Safety ENDINIT is set back to 1 again. No explicit acknowledge to SCU on System Standby request. System Standby entry in a few cycles."]
    #[inline(always)]
    pub fn reqslp(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, pmcsr4::Reqslp, Pmcsr4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,pmcsr4::Reqslp, Pmcsr4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Power management Status   PMST. This bit field reflects the current status of the CPU."]
    #[inline(always)]
    pub fn pmst(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, pmcsr4::Pmst, Pmcsr4_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x7,1,0,pmcsr4::Pmst, Pmcsr4_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pmcsr4 {
    #[inline(always)]
    fn default() -> Pmcsr4 {
        <crate::RegValueT<Pmcsr4_SPEC> as RegisterValue<_>>::new(256)
    }
}
pub mod pmcsr4 {
    pub struct Reqslp_SPEC;
    pub type Reqslp = crate::EnumBitfieldStruct<u8, Reqslp_SPEC>;
    impl Reqslp {
        #[doc = "00 Request CPU Run Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Request CPU Idle Mode"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Request System Sleep Mode"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Request System Standby Mode"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Pmst_SPEC;
    pub type Pmst = crate::EnumBitfieldStruct<u8, Pmst_SPEC>;
    impl Pmst {
        #[doc = "001 Normal Run Mode After a reset  all CPUs are in  Normal Run Mode   but this does not mean that all CPUs are executing code. This mode also includes the CPU  halt  mode which is the start up default for all except CPU0."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 CPU Idle Mode requested"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 CPU Idle Mode acknowledged"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 Sleep Mode requested"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "110 Standby Mode requested"]
        pub const CONST_66: Self = Self::new(6);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmcsr5_SPEC;
impl crate::sealed::RegSpec for Pmcsr5_SPEC {
    type DataType = u32;
}
#[doc = "Power Management Control and Status Register\n resetvalue={Application Reset:0x100}"]
pub type Pmcsr5 = crate::RegValueT<Pmcsr5_SPEC>;

impl Pmcsr5 {
    #[doc = "Idle Mode and Sleep Mode Request   REQSLP. In Idle Mode or Sleep Mode  these bits are cleared in response to an interrupt for the CPU  or when bit 15 of the corresponding CPU Watchdog Timer register  bit WDTCPUxSR.TIM 15   changes from 0 to 1. In Standby Mode  these bits are cleared on wake up. REQSLP maybe written only when either CPU or Safety ENDINIT bits are set to 0. CPU ENDINIT bit has to be set back after REQSLP is written for the mode transition to take place. In case of Safety ENDINIT  the mode transition will be issued immediately and does not wait till Safety ENDINIT is set back to 1 again. No explicit acknowledge to SCU on System Standby request. System Standby entry in a few cycles."]
    #[inline(always)]
    pub fn reqslp(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, pmcsr5::Reqslp, Pmcsr5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,pmcsr5::Reqslp, Pmcsr5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Power management Status   PMST. This bit field reflects the current status of the CPU."]
    #[inline(always)]
    pub fn pmst(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, pmcsr5::Pmst, Pmcsr5_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x7,1,0,pmcsr5::Pmst, Pmcsr5_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pmcsr5 {
    #[inline(always)]
    fn default() -> Pmcsr5 {
        <crate::RegValueT<Pmcsr5_SPEC> as RegisterValue<_>>::new(256)
    }
}
pub mod pmcsr5 {
    pub struct Reqslp_SPEC;
    pub type Reqslp = crate::EnumBitfieldStruct<u8, Reqslp_SPEC>;
    impl Reqslp {
        #[doc = "00 Request CPU Run Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Request CPU Idle Mode"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Request System Sleep Mode"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Request System Standby Mode"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Pmst_SPEC;
    pub type Pmst = crate::EnumBitfieldStruct<u8, Pmst_SPEC>;
    impl Pmst {
        #[doc = "001 Normal Run Mode After a reset  all CPUs are in  Normal Run Mode   but this does not mean that all CPUs are executing code. This mode also includes the CPU  halt  mode which is the start up default for all except CPU0."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 CPU Idle Mode requested"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 CPU Idle Mode acknowledged"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 Sleep Mode requested"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "110 Standby Mode requested"]
        pub const CONST_66: Self = Self::new(6);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmstat0_SPEC;
impl crate::sealed::RegSpec for Pmstat0_SPEC {
    type DataType = u32;
}
#[doc = "Power Management Status Register 0\n resetvalue={Application Reset:0x1}"]
pub type Pmstat0 = crate::RegValueT<Pmstat0_SPEC>;

impl Pmstat0 {
    #[doc = "CPU0 Status   CPU0. This bit field reflects the current status of CPU0."]
    #[inline(always)]
    pub fn cpu0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, pmstat0::Cpu0, Pmstat0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,pmstat0::Cpu0, Pmstat0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CPU1 Status   CPU1. This bit field reflects the current status of CPU1."]
    #[inline(always)]
    pub fn cpu1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, pmstat0::Cpu1, Pmstat0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,pmstat0::Cpu1, Pmstat0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CPU2 Status   CPU2. This bit field reflects the current status of CPU2."]
    #[inline(always)]
    pub fn cpu2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, pmstat0::Cpu2, Pmstat0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1,1,0,pmstat0::Cpu2, Pmstat0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CPU3 Status   CPU3. This bit field reflects the current status of CPU3."]
    #[inline(always)]
    pub fn cpu3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, pmstat0::Cpu3, Pmstat0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x1,1,0,pmstat0::Cpu3, Pmstat0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CPU4 Status   CPU4. This bit field reflects the current status of CPU4."]
    #[inline(always)]
    pub fn cpu4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, pmstat0::Cpu4, Pmstat0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,pmstat0::Cpu4, Pmstat0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CPU5 Status   CPU5. This bit field reflects the current status of CPU5."]
    #[inline(always)]
    pub fn cpu5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, pmstat0::Cpu5, Pmstat0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x1,1,0,pmstat0::Cpu5, Pmstat0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CPU0LS Status   CPU0LS. This bit field reflects the current status of CPU0 Lockstep Checker        Core.The activation of the Lockstep is configured in UCB BMI        configuration and determines the default reset value. The default reset        value 0 is for the case where CPU0LS is disabled in UCB BMI        configuration."]
    #[inline(always)]
    pub fn cpu0ls(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, pmstat0::Cpu0Ls, Pmstat0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1,1,0,pmstat0::Cpu0Ls, Pmstat0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CPU1LS Status   CPU1LS. This bit field reflects the current status of CPU1 Lockstep Checker Core.The activation of the Lockstep is configured in UCB BMI configuration and determines the default status. The default reset value 0 is for the case where CPU0LS is disabled in UCB BMI configuration."]
    #[inline(always)]
    pub fn cpu1ls(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, pmstat0::Cpu1Ls, Pmstat0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<17,0x1,1,0,pmstat0::Cpu1Ls, Pmstat0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CPU2LS Status   CPU2LS. This bit field reflects the current status of CPU2 Lockstep Checker Core.The activation of the Lockstep is configured in UCB BMI configuration and determines the default status. The default reset value 0 is for the case where CPU0LS is disabled in UCB BMI configuration."]
    #[inline(always)]
    pub fn cpu2ls(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, pmstat0::Cpu2Ls, Pmstat0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<18,0x1,1,0,pmstat0::Cpu2Ls, Pmstat0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CPU3LS Status   CPU3LS. This bit field reflects the current status of CPU3 Lockstep Checker Core.The activation of the Lockstep is configured in UCB BMI configuration and determines the default status. The default reset value 0 is for the case where CPU0LS is disabled in UCB BMI configuration."]
    #[inline(always)]
    pub fn cpu3ls(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, pmstat0::Cpu3Ls, Pmstat0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<19,0x1,1,0,pmstat0::Cpu3Ls, Pmstat0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pmstat0 {
    #[inline(always)]
    fn default() -> Pmstat0 {
        <crate::RegValueT<Pmstat0_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod pmstat0 {
    pub struct Cpu0_SPEC;
    pub type Cpu0 = crate::EnumBitfieldStruct<u8, Cpu0_SPEC>;
    impl Cpu0 {
        #[doc = "0 CPU0 is in Halt or Idle Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 CPU0 is in Normal Run Mode"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1_SPEC;
    pub type Cpu1 = crate::EnumBitfieldStruct<u8, Cpu1_SPEC>;
    impl Cpu1 {
        #[doc = "0 CPU1 is in Halt or Idle Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 CPU1 is in Normal Run Mode"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2_SPEC;
    pub type Cpu2 = crate::EnumBitfieldStruct<u8, Cpu2_SPEC>;
    impl Cpu2 {
        #[doc = "0 CPU2 is in Halt or Idle Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 CPU2 is in Normal Run Mode"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu3_SPEC;
    pub type Cpu3 = crate::EnumBitfieldStruct<u8, Cpu3_SPEC>;
    impl Cpu3 {
        #[doc = "0 CPU3 is in Halt or Idle Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 CPU3 is in Normal Run Mode"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu4_SPEC;
    pub type Cpu4 = crate::EnumBitfieldStruct<u8, Cpu4_SPEC>;
    impl Cpu4 {
        #[doc = "0 CPU4 is in Halt or Idle Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 CPU4 is in Normal Run Mode"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu5_SPEC;
    pub type Cpu5 = crate::EnumBitfieldStruct<u8, Cpu5_SPEC>;
    impl Cpu5 {
        #[doc = "0 CPU5 is in Halt or Idle Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 CPU5 is in Normal Run Mode"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0Ls_SPEC;
    pub type Cpu0Ls = crate::EnumBitfieldStruct<u8, Cpu0Ls_SPEC>;
    impl Cpu0Ls {
        #[doc = "0 CPU0LS is disabled or in Halt or Idle Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 CPU0LS is enabled and in Normal Run Mode"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1Ls_SPEC;
    pub type Cpu1Ls = crate::EnumBitfieldStruct<u8, Cpu1Ls_SPEC>;
    impl Cpu1Ls {
        #[doc = "0 CPU1LS is disabled or in Halt or Idle Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 CPU1LS is enabled and in Normal Run Mode"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2Ls_SPEC;
    pub type Cpu2Ls = crate::EnumBitfieldStruct<u8, Cpu2Ls_SPEC>;
    impl Cpu2Ls {
        #[doc = "0 CPU2LS is disabled or in Halt or Idle Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 CPU2LS is enabled and in Normal Run Mode"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu3Ls_SPEC;
    pub type Cpu3Ls = crate::EnumBitfieldStruct<u8, Cpu3Ls_SPEC>;
    impl Cpu3Ls {
        #[doc = "0 CPU3LS is disabled or in Halt or Idle Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 CPU3LS is enabled and in Normal Run Mode"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmswcr1_SPEC;
impl crate::sealed::RegSpec for Pmswcr1_SPEC {
    type DataType = u32;
}
#[doc = "Standby and Wake up Control Register 1\n resetvalue={Cold PowerOn Reset:0x1000000}"]
pub type Pmswcr1 = crate::RegValueT<Pmswcr1_SPEC>;

impl Pmswcr1 {
    #[doc = "Idle Request Acknowledge Sequence Disable   IRADIS. This bit enables SCU Idle Request Acknowledge sequence to all modules on Standby entry. IRADIS bit has no effect incase of Standby entry triggered via PWRWKEN register bit. This bit shall be set before Standby entry to disable Idle request acknowledge sequence so that standby request is not blocked by a pending reset idle request acknowledge sequence."]
    #[inline(always)]
    pub fn iradis(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, pmswcr1::Iradis, Pmswcr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pmswcr1::Iradis,
            Pmswcr1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Standby Entry Event configuration enable   STBYEVEN"]
    #[inline(always)]
    pub fn stbyeven(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        pmswcr1::Stbyeven,
        Pmswcr1_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            pmswcr1::Stbyeven,
            Pmswcr1_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Pmswcr1 {
    #[inline(always)]
    fn default() -> Pmswcr1 {
        <crate::RegValueT<Pmswcr1_SPEC> as RegisterValue<_>>::new(16777216)
    }
}
pub mod pmswcr1 {
    pub struct Iradis_SPEC;
    pub type Iradis = crate::EnumBitfieldStruct<u8, Iradis_SPEC>;
    impl Iradis {
        #[doc = "0 Idle Request Acknowledge Sequence issued on Standby entry."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Idle Request Acknowledge Sequence skipped on Standby entry."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Stbyeven_SPEC;
    pub type Stbyeven = crate::EnumBitfieldStruct<u8, Stbyeven_SPEC>;
    impl Stbyeven {
        #[doc = "0 Bit STBYEV is not updated."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit STBYEV can be updated."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmtrcsr0_SPEC;
impl crate::sealed::RegSpec for Pmtrcsr0_SPEC {
    type DataType = u32;
}
#[doc = "Power Management Transition Control and Status Register 0\n resetvalue={Cold PowerOn Reset:0x0}"]
pub type Pmtrcsr0 = crate::RegValueT<Pmtrcsr0_SPEC>;

impl Pmtrcsr0 {
    #[doc = "Load Jump Timer Enable   LJTEN. This bit field enables the usage of load jump timer."]
    #[inline(always)]
    pub fn ljten(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, pmtrcsr0::Ljten, Pmtrcsr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pmtrcsr0::Ljten,
            Pmtrcsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Load Jump Timer Overflow Enable   LJTOVEN. This bit field enables the update of LJTOV status bit on timer overflow        or time out."]
    #[inline(always)]
    pub fn ljtoven(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pmtrcsr0::Ljtoven,
        Pmtrcsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pmtrcsr0::Ljtoven,
            Pmtrcsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Load Jump Timer Overflow Interrupt Enable   LJTOVIEN. This bit field enables the activation of interrupt on timer overflow or        time out."]
    #[inline(always)]
    pub fn ljtovien(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pmtrcsr0::Ljtovien,
        Pmtrcsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pmtrcsr0::Ljtovien,
            Pmtrcsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Load Jump Timer Start   LJTSTRT. This bit field starts Load jump timer. This is intended for test        purposes. The LJTSTRT remains set on a write and is cleared when LJTOV        bit is set if LJTOVEN bit is enabled."]
    #[inline(always)]
    pub fn ljtstrt(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pmtrcsr0::Ljtstrt,
        Pmtrcsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pmtrcsr0::Ljtstrt,
            Pmtrcsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Load Jump Timer Stop   LJTSTP. This bit field stops Load jump timer. This is intended for test        purposes. The LJTSTP remains set on a write and is to be explicitly        cleared by software. The LJTSTP stops the counter at the current value        and timer re starts from that value when LJTSTP is cleared and LJTSTRT        is set."]
    #[inline(always)]
    pub fn ljtstp(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pmtrcsr0::Ljtstp,
        Pmtrcsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pmtrcsr0::Ljtstp,
            Pmtrcsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Load Jump Timer Clear   LJTCLR. This bit field clear Load jump timer count. This is intended for test        purposes. This bit resets LJT and clears LJTRUN if LJTEN bit is set."]
    #[inline(always)]
    pub fn ljtclr(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, pmtrcsr0::Ljtclr, Pmtrcsr0_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            pmtrcsr0::Ljtclr,
            Pmtrcsr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Droop Voltage Step vdroop step i    SDSTEP. This bit field defines the voltage offset for droop compensation on a        load jump to the EVRC setpoint value. The request is made via SCU PMTRCSR3 .VDROOPREQ  sd droop cntr i   on        an anticipated load jump with a voltage offset equal to the SDSTEP x        5  160 mV. The droop step is a positive offset if VDROOPREQ  160    160 01b and is a        negative offset if VDROOPREQ  160    160 10b and no offset is applied if        VDROOPREQ  160    160 00b. Maximum Droop   80  160 mV."]
    #[inline(always)]
    pub fn sdstep(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Pmtrcsr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Pmtrcsr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage Droop Timer Enable   VDTEN. This bit field enables the usage of Voltage Droop timer."]
    #[inline(always)]
    pub fn vdten(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        pmtrcsr0::Vdten,
        Pmtrcsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pmtrcsr0::Vdten,
            Pmtrcsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Voltage Droop Timer Overflow Enable   VDTOVEN. This bit field enables the update of VDTOV status bit on timer overflow        or time out."]
    #[inline(always)]
    pub fn vdtoven(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        pmtrcsr0::Vdtoven,
        Pmtrcsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            pmtrcsr0::Vdtoven,
            Pmtrcsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Voltage Droop Timer Overflow Interrupt Enable   VDTOVIEN. This bit field enables the activation of interrupt on timer overflow or        time out."]
    #[inline(always)]
    pub fn vdtovien(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        pmtrcsr0::Vdtovien,
        Pmtrcsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            pmtrcsr0::Vdtovien,
            Pmtrcsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Voltage Droop Timer Start   VDTSTRT. This bit field starts Voltage Droop timer. This is intended for test        purposes. The VDTSTRT remains set on a write and is cleared when VDTOV        bit is set if VDTOVEN bit is enabled."]
    #[inline(always)]
    pub fn vdtstrt(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        pmtrcsr0::Vdtstrt,
        Pmtrcsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            pmtrcsr0::Vdtstrt,
            Pmtrcsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Voltage Droop Timer Stop   VDTSTP. This bit field stops Voltage Droop timer. SCU cancels the droop request        via signal sd droop cntr i  160    160 00. This is intended for test purposes. The        VDTSTP remains set on a write and is to be explicitly cleared by        software. The VDTSTP stops the counter at the current value and timer        re starts from that value when VDTSTP is cleared and VDTSTRT is set."]
    #[inline(always)]
    pub fn vdtstp(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        pmtrcsr0::Vdtstp,
        Pmtrcsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            pmtrcsr0::Vdtstp,
            Pmtrcsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Voltage Droop Timer Clear   VDTCLR. This bit field clear Voltage Droop timer count. This is intended for        test purposes. This bit resets VDT and clears VDTRUN if VDTEN bit is set."]
    #[inline(always)]
    pub fn vdtclr(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        pmtrcsr0::Vdtclr,
        Pmtrcsr0_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            pmtrcsr0::Vdtclr,
            Pmtrcsr0_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "EVRC Low Power Mode activation on a Sleep Request   LPSLPEN. This bit field enables the activation of LPM EVRC mode on a sleep        request. If this bit is set          SCU sends lp enable signal on Sleep request to PMS which is OR ed with        EVRSDCTRL2.EVRCMOD value and is provided to EVRC regulator."]
    #[inline(always)]
    pub fn lpslpen(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        pmtrcsr0::Lpslpen,
        Pmtrcsr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            pmtrcsr0::Lpslpen,
            Pmtrcsr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Pmtrcsr0 {
    #[inline(always)]
    fn default() -> Pmtrcsr0 {
        <crate::RegValueT<Pmtrcsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pmtrcsr0 {
    pub struct Ljten_SPEC;
    pub type Ljten = crate::EnumBitfieldStruct<u8, Ljten_SPEC>;
    impl Ljten {
        #[doc = "0 Load Jump Timer inactive"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Load Jump Timer active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ljtoven_SPEC;
    pub type Ljtoven = crate::EnumBitfieldStruct<u8, Ljtoven_SPEC>;
    impl Ljtoven {
        #[doc = "0 LJTOV bit is not updated on a Load Jump Timer overflow."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 LJTOV bit is updated on a Load Jump Timer overflow."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ljtovien_SPEC;
    pub type Ljtovien = crate::EnumBitfieldStruct<u8, Ljtovien_SPEC>;
    impl Ljtovien {
        #[doc = "0 LJTOV interrupt is inactive."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 LJTOV interrupt is activated on a Load Jump Timer overflow."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ljtstrt_SPEC;
    pub type Ljtstrt = crate::EnumBitfieldStruct<u8, Ljtstrt_SPEC>;
    impl Ljtstrt {
        #[doc = "0 Load Jump Timer status not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Load Jump Timer started."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ljtstp_SPEC;
    pub type Ljtstp = crate::EnumBitfieldStruct<u8, Ljtstp_SPEC>;
    impl Ljtstp {
        #[doc = "0 Load Jump Timer status not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Load Jump Timer stopped."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ljtclr_SPEC;
    pub type Ljtclr = crate::EnumBitfieldStruct<u8, Ljtclr_SPEC>;
    impl Ljtclr {
        #[doc = "0 Load Jump Count status not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Load Jump Timer Count cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Vdten_SPEC;
    pub type Vdten = crate::EnumBitfieldStruct<u8, Vdten_SPEC>;
    impl Vdten {
        #[doc = "0 Voltage Droop Timer inactive"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Voltage Droop Timer active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Vdtoven_SPEC;
    pub type Vdtoven = crate::EnumBitfieldStruct<u8, Vdtoven_SPEC>;
    impl Vdtoven {
        #[doc = "0 VDTOV bit is not updated on a Voltage Droop Timer overflow."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 VDTOV bit is updated on a Voltage Droop Timer overflow."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Vdtovien_SPEC;
    pub type Vdtovien = crate::EnumBitfieldStruct<u8, Vdtovien_SPEC>;
    impl Vdtovien {
        #[doc = "0 VDTOV interrupt is inactive."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 VDTOV interrupt is activated on a Voltage Droop Timer overflow."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Vdtstrt_SPEC;
    pub type Vdtstrt = crate::EnumBitfieldStruct<u8, Vdtstrt_SPEC>;
    impl Vdtstrt {
        #[doc = "0 Voltage Droop Timer status not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Voltage Droop Timer started."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Vdtstp_SPEC;
    pub type Vdtstp = crate::EnumBitfieldStruct<u8, Vdtstp_SPEC>;
    impl Vdtstp {
        #[doc = "0 Voltage Droop Timer status not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Voltage Droop Timer stopped."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Vdtclr_SPEC;
    pub type Vdtclr = crate::EnumBitfieldStruct<u8, Vdtclr_SPEC>;
    impl Vdtclr {
        #[doc = "0 Voltage Droop Count status not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Voltage Droop Timer Count cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lpslpen_SPEC;
    pub type Lpslpen = crate::EnumBitfieldStruct<u8, Lpslpen_SPEC>;
    impl Lpslpen {
        #[doc = "0 EVRC remains in normal operation mode during and after a sleep request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 LPM mode activated on a sleep request"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmtrcsr1_SPEC;
impl crate::sealed::RegSpec for Pmtrcsr1_SPEC {
    type DataType = u32;
}
#[doc = "Power Management Transition Control and Status Register 1\n resetvalue={Cold PowerOn Reset:0x0}"]
pub type Pmtrcsr1 = crate::RegValueT<Pmtrcsr1_SPEC>;

impl Pmtrcsr1 {
    #[doc = "Load Jump Timer Compare Setpoint Value   LJTCV. This bit field defines the compare setpoint value of Load Jump timer.        The compare event would lead to LJTOV bit being set and LJT interrupt        being raised. The LJTRUN status bit  LDJMPREQ bit and LJTCNT value is        reset to 0 on a compare event.   160   160 X  160 us is the compare value. LSB  160  1  160 us. Total range  160    160 65.5  160 ms"]
    #[inline(always)]
    pub fn ljtcv(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Pmtrcsr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Pmtrcsr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Voltage Droop Timer Compare Setpoint Value   VDTCV. This bit field defines the compare setpoint value of Voltage Droop        timer. The compare event would lead to VDTOV bit being set and VDT        interrupt being raised. The VDTRUN status bit  VDROOPREQ bit and VDTCNT        value is reset to 0 on a compare event.   160   160 X  160 us is the compare value. LSB  160  1  160 us. Total range  160    160 1023  160 us"]
    #[inline(always)]
    pub fn vdtcv(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Pmtrcsr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Pmtrcsr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pmtrcsr1 {
    #[inline(always)]
    fn default() -> Pmtrcsr1 {
        <crate::RegValueT<Pmtrcsr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmtrcsr2_SPEC;
impl crate::sealed::RegSpec for Pmtrcsr2_SPEC {
    type DataType = u32;
}
#[doc = "Power Management Transition Control and Status Register 2\n resetvalue={Cold PowerOn Reset:0x0}"]
pub type Pmtrcsr2 = crate::RegValueT<Pmtrcsr2_SPEC>;

impl Pmtrcsr2 {
    #[doc = "Load Jump Timer Overflow Status   LJTOV. This status bit indicates that the Load Jump timer compare match has        happened. if LJTOVEN bit is enabled  then LJTOV can only be cleared        explicitly via LJTOVCLR bit. if LJTOVEN bit is disabled  LJTOV is cleared on a taken Load Jump        Request  A new Load Jump request is        taken only if both LJT  amp  VDT are not currently running and no active        Load Jump request is being processed  . LJTOV being set will lead        to an interrupt if LJTOVIEN is enabled."]
    #[inline(always)]
    pub fn ljtov(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, pmtrcsr2::Ljtov, Pmtrcsr2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x1,1,0,pmtrcsr2::Ljtov, Pmtrcsr2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Load Jump Timer Overflow Status Clear   LJTOVCLR. This bit clears LJTOV status bit and sets VDROOPREQ and LDJMPREQ to 0 if        LJTOVEN bit is enabled. This bit always reads as 0."]
    #[inline(always)]
    pub fn ljtovclr(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pmtrcsr2::Ljtovclr,
        Pmtrcsr2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pmtrcsr2::Ljtovclr,
            Pmtrcsr2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Load Jump Timer Value   LJTCNT. This bit field reflects the current Load Jump timer value. LJTCNT value is cleared on timer overflow and on a taken Load Jump Request X us is the compare value. LSB  1 us. Total range   65.5 ms"]
    #[inline(always)]
    pub fn ljtcnt(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Pmtrcsr2_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Pmtrcsr2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pmtrcsr2 {
    #[inline(always)]
    fn default() -> Pmtrcsr2 {
        <crate::RegValueT<Pmtrcsr2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pmtrcsr2 {
    pub struct Ljtov_SPEC;
    pub type Ljtov = crate::EnumBitfieldStruct<u8, Ljtov_SPEC>;
    impl Ljtov {
        #[doc = "0 Load Jump Timer compare overflow has not happened."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Load Jump Timer compare overflow has happened."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ljtovclr_SPEC;
    pub type Ljtovclr = crate::EnumBitfieldStruct<u8, Ljtovclr_SPEC>;
    impl Ljtovclr {
        #[doc = "0 This clear bit has no effect on Load Jump Timer overflow flag."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Load Jump Timer overflow flag is cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmtrcsr3_SPEC;
impl crate::sealed::RegSpec for Pmtrcsr3_SPEC {
    type DataType = u32;
}
#[doc = "Power Management Transition Control and Status Register 3\n resetvalue={Cold PowerOn Reset:0x0}"]
pub type Pmtrcsr3 = crate::RegValueT<Pmtrcsr3_SPEC>;

impl Pmtrcsr3 {
    #[doc = "Voltage Droop Request   VDROOPREQ. This bit requests a Voltage Droop consequently leading to Voltage Droop        Timer start and VDTRUN bit being set if        VDTEN 1 . The request is not taken if VDTRUN bit is already in        set state and VDT is currently running. The request is also not taken if         VDTOV bit is set AND VDTOVEN bit is enabled . The droop step is a        positive offset if sd droop cntr i  160    160 01 and is a negative offset if        sd droop cntr i  160    160 10 and no offset is applied if sd droop cntr i  160    160 00        and is applied immediately."]
    #[inline(always)]
    pub fn vdroopreq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        pmtrcsr3::Vdroopreq,
        Pmtrcsr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            pmtrcsr3::Vdroopreq,
            Pmtrcsr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Voltage Droop Timer Overflow Status   VDTOV. This status bit indicates that the Voltage Droop timer compare match has happened. if VDTOVEN bit is enabled  then VDTOV can only be cleared by explicitly via VDTOVCLR bit. if VDTOVEN bit is disabled  VDTOV is cleared on a taken Voltage Droop Request  A new Voltage Droop request is taken only if both LJT  amp  VDT are not currently running and no active Voltage Droop request is being processed  . VDTOV being set will lead to an interrupt if VDTOVIEN is enabled. Incase SDVOK is set by EVRC before VDT compare match  VDTOV bit is not set."]
    #[inline(always)]
    pub fn vdtov(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, pmtrcsr3::Vdtov, Pmtrcsr3_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x1,1,0,pmtrcsr3::Vdtov, Pmtrcsr3_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Voltage Droop Timer Overflow Status Clear   VDTOVCLR. This bit clears VDTOV status bit if VDTOVEN bit is enabled. If VDTOVEN bit is disabled  this bit has no effect. This bit always reads as 0."]
    #[inline(always)]
    pub fn vdtovclr(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pmtrcsr3::Vdtovclr,
        Pmtrcsr3_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pmtrcsr3::Vdtovclr,
            Pmtrcsr3_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Voltage Droop Timer Value   VDTCNT. This bit field reflects the current Voltage Droop timer value. VDTCNT value is cleared on timer overflow and on a taken Voltage Droop Request.   X us is the compare value. LSB  1 us. Total range   65.5 ms"]
    #[inline(always)]
    pub fn vdtcnt(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Pmtrcsr3_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Pmtrcsr3_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pmtrcsr3 {
    #[inline(always)]
    fn default() -> Pmtrcsr3 {
        <crate::RegValueT<Pmtrcsr3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pmtrcsr3 {
    pub struct Vdroopreq_SPEC;
    pub type Vdroopreq = crate::EnumBitfieldStruct<u8, Vdroopreq_SPEC>;
    impl Vdroopreq {
        #[doc = "00 Voltage Droop        and Voltage Droop Timer inactive"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 A Positive        Voltage Droop Request made and taken. Voltage Droop Timer activated."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 A Negative Voltage Droop Request made and taken. Voltage Droop Timer activated."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Voltage Droop and Voltage Droop Timer inactive"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Vdtov_SPEC;
    pub type Vdtov = crate::EnumBitfieldStruct<u8, Vdtov_SPEC>;
    impl Vdtov {
        #[doc = "0 Voltage Droop Timer compare overflow has not happened."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Voltage Droop Timer compare overflow has happened."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Vdtovclr_SPEC;
    pub type Vdtovclr = crate::EnumBitfieldStruct<u8, Vdtovclr_SPEC>;
    impl Vdtovclr {
        #[doc = "0 This clear bit has no effect on Voltage Droop Timer overflow flag."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Voltage Droop Timer overflow flag is cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmtrcsr4_SPEC;
impl crate::sealed::RegSpec for Pmtrcsr4_SPEC {
    type DataType = u32;
}
#[doc = "Power Management Transition Control and Status Register 4\n resetvalue={Cold PowerOn Reset:0x0}"]
pub type Pmtrcsr4 = crate::RegValueT<Pmtrcsr4_SPEC>;

impl Pmtrcsr4 {
    #[doc = "Warm Reset Droop Enable   RSTDROOPEN. This bit field enables negative voltage droop incase of a warm reset        request. A negative droop step is requested via sd droop cntr i   10        with Reset droop value as configured in SDSTEP register. By default        Reset Droop is disabled."]
    #[inline(always)]
    pub fn rstdroopen(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Pmtrcsr4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pmtrcsr4_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Bit Protection RSTDROOPEN   BPRSTDROOPEN. Setting this bit enables that bit RSTDROOPEN can be changed in this        write operation."]
    #[inline(always)]
    pub fn bprstdroopen(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Pmtrcsr4_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pmtrcsr4_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Voltage Droop during Reset   RSTVDROOP"]
    #[inline(always)]
    pub fn rstvdroop(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        pmtrcsr4::Rstvdroop,
        Pmtrcsr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            pmtrcsr4::Rstvdroop,
            Pmtrcsr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Droop Voltage Step vdroop step i    RSTSDSTEP. This bit field RSTSDSTEP defines the voltage offset for droop        compensation on a reset jump to the EVRC setpoint value. The request is        made via sd droop cntr i signal on an anticipated load jump with a        voltage offset equal to the RSTSDSTEP x 5  160 mV. Maximum Droop   80  160 mV."]
    #[inline(always)]
    pub fn rstsdstep(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Pmtrcsr4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Pmtrcsr4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pmtrcsr4 {
    #[inline(always)]
    fn default() -> Pmtrcsr4 {
        <crate::RegValueT<Pmtrcsr4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pmtrcsr4 {
    pub struct Rstvdroop_SPEC;
    pub type Rstvdroop = crate::EnumBitfieldStruct<u8, Rstvdroop_SPEC>;
    impl Rstvdroop {
        #[doc = "00 Voltage Droop        inactive during reset. sd droop cntr i  160    160 00 on a reset request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 A Positive Voltage Droop Request taken. sd droop cntr i   01 on a reset request."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 A Negative        Voltage Droop Request taken. Voltage Droop Timer activated.        sd droop cntr i  160    160 10 on a reset request."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Voltage Droop inactive during reset. sd droop cntr i   00 on a reset request."]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prdcfg0_SPEC;
impl crate::sealed::RegSpec for Prdcfg0_SPEC {
    type DataType = u32;
}
#[doc = "Product Configuration Register 0\n resetvalue={Application Reset:0x0FFFFFFFF,CFS Value:0x0FFFFFFFF}"]
pub type Prdcfg0 = crate::RegValueT<Prdcfg0_SPEC>;

impl Prdcfg0 {
    #[doc = "QSPI0 Availability Control   QSPI0AV"]
    #[inline(always)]
    pub fn qspi0av(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, prdcfg0::Qspi0Av, Prdcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            prdcfg0::Qspi0Av,
            Prdcfg0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "QSPI1 Availability Control   QSPI1AV"]
    #[inline(always)]
    pub fn qspi1av(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, prdcfg0::Qspi1Av, Prdcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            prdcfg0::Qspi1Av,
            Prdcfg0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "QSPI2 Availability Control   QSPI2AV"]
    #[inline(always)]
    pub fn qspi2av(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, prdcfg0::Qspi2Av, Prdcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            prdcfg0::Qspi2Av,
            Prdcfg0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "QSPI3 Availability Control   QSPI3AV"]
    #[inline(always)]
    pub fn qspi3av(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, prdcfg0::Qspi3Av, Prdcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            prdcfg0::Qspi3Av,
            Prdcfg0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "QSPI4 Availability Control   QSPI4AV"]
    #[inline(always)]
    pub fn qspi4av(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, prdcfg0::Qspi4Av, Prdcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            prdcfg0::Qspi4Av,
            Prdcfg0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "QSPI5 Availability Control   QSPI5AV"]
    #[inline(always)]
    pub fn qspi5av(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, prdcfg0::Qspi5Av, Prdcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            prdcfg0::Qspi5Av,
            Prdcfg0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "I2C0 Availability Control   I2C0AV"]
    #[inline(always)]
    pub fn i2c0av(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, prdcfg0::I2C0Av, Prdcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,prdcfg0::I2C0Av, Prdcfg0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "I2C1 Availability Control   I2C1AV"]
    #[inline(always)]
    pub fn i2c1av(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, prdcfg0::I2C1Av, Prdcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,prdcfg0::I2C1Av, Prdcfg0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "MSC0 Availability Control   MSC0AV"]
    #[inline(always)]
    pub fn msc0av(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, prdcfg0::Msc0Av, Prdcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,prdcfg0::Msc0Av, Prdcfg0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "MSC1 Availability Control   MSC1AV"]
    #[inline(always)]
    pub fn msc1av(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, prdcfg0::Msc1Av, Prdcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,prdcfg0::Msc1Av, Prdcfg0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "MSC2 Availability Control   MSC2AV"]
    #[inline(always)]
    pub fn msc2av(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, prdcfg0::Msc2Av, Prdcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            prdcfg0::Msc2Av,
            Prdcfg0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "MSC3 Availability Control   MSC3AV"]
    #[inline(always)]
    pub fn msc3av(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, prdcfg0::Msc3Av, Prdcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            prdcfg0::Msc3Av,
            Prdcfg0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "PSI5AV Availability Control   PSI5AV"]
    #[inline(always)]
    pub fn psi5av(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, prdcfg0::Psi5Av, Prdcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            prdcfg0::Psi5Av,
            Prdcfg0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "SDMMC Availability Control   SDMMCAV"]
    #[inline(always)]
    pub fn sdmmcav(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        prdcfg0::Sdmmcav,
        Prdcfg0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            prdcfg0::Sdmmcav,
            Prdcfg0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CAN0 Availability Control   CAN0AV"]
    #[inline(always)]
    pub fn can0av(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, prdcfg0::Can0Av, Prdcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            prdcfg0::Can0Av,
            Prdcfg0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CAN1 Availability Control   CAN1AV"]
    #[inline(always)]
    pub fn can1av(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, prdcfg0::Can1Av, Prdcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            prdcfg0::Can1Av,
            Prdcfg0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CAN2 Availability Control   CAN2AV"]
    #[inline(always)]
    pub fn can2av(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, prdcfg0::Can2Av, Prdcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            prdcfg0::Can2Av,
            Prdcfg0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GETH Availability Control   GETHAV"]
    #[inline(always)]
    pub fn gethav(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, prdcfg0::Gethav, Prdcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            prdcfg0::Gethav,
            Prdcfg0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "HSM Availability Control   HSMAV"]
    #[inline(always)]
    pub fn hsmav(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, prdcfg0::Hsmav, Prdcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,prdcfg0::Hsmav, Prdcfg0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ETHERMAC Availability Control   ETHAV"]
    #[inline(always)]
    pub fn ethav(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, prdcfg0::Ethav, Prdcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,prdcfg0::Ethav, Prdcfg0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SENT Availability Control   SENTAV"]
    #[inline(always)]
    pub fn sentav(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, prdcfg0::Sentav, Prdcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            prdcfg0::Sentav,
            Prdcfg0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "FCE Availability Control   FCEAV"]
    #[inline(always)]
    pub fn fceav(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, prdcfg0::Fceav, Prdcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,prdcfg0::Fceav, Prdcfg0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GETH1 Availability Control   GETH1AV"]
    #[inline(always)]
    pub fn geth1av(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        prdcfg0::Geth1Av,
        Prdcfg0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            prdcfg0::Geth1Av,
            Prdcfg0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "PSI5S Availability Control   PSI5SAV"]
    #[inline(always)]
    pub fn psi5sav(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        prdcfg0::Psi5Sav,
        Prdcfg0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            prdcfg0::Psi5Sav,
            Prdcfg0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ED Availability Control   EDAV"]
    #[inline(always)]
    pub fn edav(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, prdcfg0::Edav, Prdcfg0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,prdcfg0::Edav, Prdcfg0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Prdcfg0 {
    #[inline(always)]
    fn default() -> Prdcfg0 {
        <crate::RegValueT<Prdcfg0_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod prdcfg0 {
    pub struct Qspi0Av_SPEC;
    pub type Qspi0Av = crate::EnumBitfieldStruct<u8, Qspi0Av_SPEC>;
    impl Qspi0Av {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Qspi1Av_SPEC;
    pub type Qspi1Av = crate::EnumBitfieldStruct<u8, Qspi1Av_SPEC>;
    impl Qspi1Av {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Qspi2Av_SPEC;
    pub type Qspi2Av = crate::EnumBitfieldStruct<u8, Qspi2Av_SPEC>;
    impl Qspi2Av {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Qspi3Av_SPEC;
    pub type Qspi3Av = crate::EnumBitfieldStruct<u8, Qspi3Av_SPEC>;
    impl Qspi3Av {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Qspi4Av_SPEC;
    pub type Qspi4Av = crate::EnumBitfieldStruct<u8, Qspi4Av_SPEC>;
    impl Qspi4Av {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Qspi5Av_SPEC;
    pub type Qspi5Av = crate::EnumBitfieldStruct<u8, Qspi5Av_SPEC>;
    impl Qspi5Av {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct I2C0Av_SPEC;
    pub type I2C0Av = crate::EnumBitfieldStruct<u8, I2C0Av_SPEC>;
    impl I2C0Av {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct I2C1Av_SPEC;
    pub type I2C1Av = crate::EnumBitfieldStruct<u8, I2C1Av_SPEC>;
    impl I2C1Av {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msc0Av_SPEC;
    pub type Msc0Av = crate::EnumBitfieldStruct<u8, Msc0Av_SPEC>;
    impl Msc0Av {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msc1Av_SPEC;
    pub type Msc1Av = crate::EnumBitfieldStruct<u8, Msc1Av_SPEC>;
    impl Msc1Av {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msc2Av_SPEC;
    pub type Msc2Av = crate::EnumBitfieldStruct<u8, Msc2Av_SPEC>;
    impl Msc2Av {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msc3Av_SPEC;
    pub type Msc3Av = crate::EnumBitfieldStruct<u8, Msc3Av_SPEC>;
    impl Msc3Av {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Psi5Av_SPEC;
    pub type Psi5Av = crate::EnumBitfieldStruct<u8, Psi5Av_SPEC>;
    impl Psi5Av {
        #[doc = "0 Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sdmmcav_SPEC;
    pub type Sdmmcav = crate::EnumBitfieldStruct<u8, Sdmmcav_SPEC>;
    impl Sdmmcav {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Can0Av_SPEC;
    pub type Can0Av = crate::EnumBitfieldStruct<u8, Can0Av_SPEC>;
    impl Can0Av {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Can1Av_SPEC;
    pub type Can1Av = crate::EnumBitfieldStruct<u8, Can1Av_SPEC>;
    impl Can1Av {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Can2Av_SPEC;
    pub type Can2Av = crate::EnumBitfieldStruct<u8, Can2Av_SPEC>;
    impl Can2Av {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Gethav_SPEC;
    pub type Gethav = crate::EnumBitfieldStruct<u8, Gethav_SPEC>;
    impl Gethav {
        #[doc = "0 Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hsmav_SPEC;
    pub type Hsmav = crate::EnumBitfieldStruct<u8, Hsmav_SPEC>;
    impl Hsmav {
        #[doc = "0 Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ethav_SPEC;
    pub type Ethav = crate::EnumBitfieldStruct<u8, Ethav_SPEC>;
    impl Ethav {
        #[doc = "0 Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sentav_SPEC;
    pub type Sentav = crate::EnumBitfieldStruct<u8, Sentav_SPEC>;
    impl Sentav {
        #[doc = "0 Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fceav_SPEC;
    pub type Fceav = crate::EnumBitfieldStruct<u8, Fceav_SPEC>;
    impl Fceav {
        #[doc = "0 Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Geth1Av_SPEC;
    pub type Geth1Av = crate::EnumBitfieldStruct<u8, Geth1Av_SPEC>;
    impl Geth1Av {
        #[doc = "0 Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module cannot be enabled"]
        pub const CONST_10: Self = Self::new(0);
    }
    pub struct Psi5Sav_SPEC;
    pub type Psi5Sav = crate::EnumBitfieldStruct<u8, Psi5Sav_SPEC>;
    impl Psi5Sav {
        #[doc = "0 Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Edav_SPEC;
    pub type Edav = crate::EnumBitfieldStruct<u8, Edav_SPEC>;
    impl Edav {
        #[doc = "0 Clears CHIPID.EEA bit to indicate to software that device appears as a PD"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Sets CHIPID.EEA to indicate to software that device is an ED"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prdcfg1_SPEC;
impl crate::sealed::RegSpec for Prdcfg1_SPEC {
    type DataType = u32;
}
#[doc = "Product Configuration Register 1\n resetvalue={Application Reset:0x0FFFFFFFF,CFS Value:0x0FFFFFFFF}"]
pub type Prdcfg1 = crate::RegValueT<Prdcfg1_SPEC>;

impl Prdcfg1 {
    #[doc = "HSPDM Availability Control   HSPDMAV"]
    #[inline(always)]
    pub fn hspdmav(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, prdcfg1::Hspdmav, Prdcfg1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            prdcfg1::Hspdmav,
            Prdcfg1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "MCDS Availability Control   MCDSAV"]
    #[inline(always)]
    pub fn mcdsav(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, prdcfg1::Mcdsav, Prdcfg1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,prdcfg1::Mcdsav, Prdcfg1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "AGBT Availability Control   AGBTAV"]
    #[inline(always)]
    pub fn agbtav(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, prdcfg1::Agbtav, Prdcfg1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,prdcfg1::Agbtav, Prdcfg1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CIF Availability Control   CIFAV"]
    #[inline(always)]
    pub fn cifav(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, prdcfg1::Cifav, Prdcfg1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,prdcfg1::Cifav, Prdcfg1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "EMEM Availability Control   EMEMAV"]
    #[inline(always)]
    pub fn ememav(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, prdcfg1::Ememav, Prdcfg1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,prdcfg1::Ememav, Prdcfg1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "MiniMCDS Availability Control   MINIMCDSAV"]
    #[inline(always)]
    pub fn minimcdsav(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        prdcfg1::Minimcdsav,
        Prdcfg1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            prdcfg1::Minimcdsav,
            Prdcfg1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "EBU Availability Control   EBUAV"]
    #[inline(always)]
    pub fn ebuav(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, prdcfg1::Ebuav, Prdcfg1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,prdcfg1::Ebuav, Prdcfg1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU1 Availability Control   CPU1AV. Note  If CPUxAV 0 then CPUx associated SFR registers and dLMU are also not available but associated PFlash bank is still available"]
    #[inline(always)]
    pub fn cpu1av(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, prdcfg1::Cpu1Av, Prdcfg1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            prdcfg1::Cpu1Av,
            Prdcfg1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU2 Availability Control   CPU2AV. Note  If CPUxAV 0 then CPUx associated SFR registers and dLMU are also not available but associated PFlash bank is still available"]
    #[inline(always)]
    pub fn cpu2av(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, prdcfg1::Cpu2Av, Prdcfg1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            prdcfg1::Cpu2Av,
            Prdcfg1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU3 Availability Control   CPU3AV. Note  If CPUxAV 0 then CPUx associated SFR registers and dLMU are also not available but associated PFlash bank is still available"]
    #[inline(always)]
    pub fn cpu3av(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, prdcfg1::Cpu3Av, Prdcfg1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            prdcfg1::Cpu3Av,
            Prdcfg1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU4 Availability Control   CPU4AV. Note  If CPUxAV 0 then CPUx associated SFR registers and dLMU are also not available but associated PFlash bank is still available"]
    #[inline(always)]
    pub fn cpu4av(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, prdcfg1::Cpu4Av, Prdcfg1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            prdcfg1::Cpu4Av,
            Prdcfg1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU5 Availability Control   CPU5AV. Note  If CPUxAV 0 then CPUx associated SFR registers and dLMU are also not available but associated PFlash bank is still available"]
    #[inline(always)]
    pub fn cpu5av(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, prdcfg1::Cpu5Av, Prdcfg1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            prdcfg1::Cpu5Av,
            Prdcfg1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Overlay Availability Control   EXOVAV. This bit enables disables overlay of memory space outside the CPU s . It        does not control overlay of CPU s  local memories."]
    #[inline(always)]
    pub fn exovav(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, prdcfg1::Exovav, Prdcfg1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            prdcfg1::Exovav,
            Prdcfg1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ASCLIN 8 9 and 10 11 Availability Control   ASCLAV118"]
    #[inline(always)]
    pub fn asclav118(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        prdcfg1::Asclav118,
        Prdcfg1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            prdcfg1::Asclav118,
            Prdcfg1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ASCLIN0 1 Availability Control   ASCLAV01"]
    #[inline(always)]
    pub fn asclav01(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        prdcfg1::Asclav01,
        Prdcfg1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            prdcfg1::Asclav01,
            Prdcfg1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ASCLIN2 3 Availability Control   ASCLAV23"]
    #[inline(always)]
    pub fn asclav23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        prdcfg1::Asclav23,
        Prdcfg1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            prdcfg1::Asclav23,
            Prdcfg1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ASCLIN4 5 Availability Control   ASCLAV45"]
    #[inline(always)]
    pub fn asclav45(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        prdcfg1::Asclav45,
        Prdcfg1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            prdcfg1::Asclav45,
            Prdcfg1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ASCLIN6 7 Availability Control   ASCLAV67"]
    #[inline(always)]
    pub fn asclav67(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        prdcfg1::Asclav67,
        Prdcfg1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            prdcfg1::Asclav67,
            Prdcfg1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ASCLIN 23 22 and 13 12 Availability Control   ASCLAV2312"]
    #[inline(always)]
    pub fn asclav2312(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        prdcfg1::Asclav2312,
        Prdcfg1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            prdcfg1::Asclav2312,
            Prdcfg1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Emulation or ADAS Memory Size   EMEMKB"]
    #[inline(always)]
    pub fn ememkb(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, prdcfg1::Ememkb, Prdcfg1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            prdcfg1::Ememkb,
            Prdcfg1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Prdcfg1 {
    #[inline(always)]
    fn default() -> Prdcfg1 {
        <crate::RegValueT<Prdcfg1_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod prdcfg1 {
    pub struct Hspdmav_SPEC;
    pub type Hspdmav = crate::EnumBitfieldStruct<u8, Hspdmav_SPEC>;
    impl Hspdmav {
        #[doc = "0 Module x cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module x is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mcdsav_SPEC;
    pub type Mcdsav = crate::EnumBitfieldStruct<u8, Mcdsav_SPEC>;
    impl Mcdsav {
        #[doc = "0 Module x cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module x is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Agbtav_SPEC;
    pub type Agbtav = crate::EnumBitfieldStruct<u8, Agbtav_SPEC>;
    impl Agbtav {
        #[doc = "0 Module x cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module x is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cifav_SPEC;
    pub type Cifav = crate::EnumBitfieldStruct<u8, Cifav_SPEC>;
    impl Cifav {
        #[doc = "0 Module x cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module x is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ememav_SPEC;
    pub type Ememav = crate::EnumBitfieldStruct<u8, Ememav_SPEC>;
    impl Ememav {
        #[doc = "0 Module x cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module x is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Minimcdsav_SPEC;
    pub type Minimcdsav = crate::EnumBitfieldStruct<u8, Minimcdsav_SPEC>;
    impl Minimcdsav {
        #[doc = "0 Module x cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module x is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ebuav_SPEC;
    pub type Ebuav = crate::EnumBitfieldStruct<u8, Ebuav_SPEC>;
    impl Ebuav {
        #[doc = "0 Module x cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module x is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1Av_SPEC;
    pub type Cpu1Av = crate::EnumBitfieldStruct<u8, Cpu1Av_SPEC>;
    impl Cpu1Av {
        #[doc = "CPU x cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "CPU x is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2Av_SPEC;
    pub type Cpu2Av = crate::EnumBitfieldStruct<u8, Cpu2Av_SPEC>;
    impl Cpu2Av {
        #[doc = "CPU x cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "CPU x is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu3Av_SPEC;
    pub type Cpu3Av = crate::EnumBitfieldStruct<u8, Cpu3Av_SPEC>;
    impl Cpu3Av {
        #[doc = "CPU x cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "CPU x is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu4Av_SPEC;
    pub type Cpu4Av = crate::EnumBitfieldStruct<u8, Cpu4Av_SPEC>;
    impl Cpu4Av {
        #[doc = "CPU x cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "CPU x is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu5Av_SPEC;
    pub type Cpu5Av = crate::EnumBitfieldStruct<u8, Cpu5Av_SPEC>;
    impl Cpu5Av {
        #[doc = "CPU x cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "CPU x is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Exovav_SPEC;
    pub type Exovav = crate::EnumBitfieldStruct<u8, Exovav_SPEC>;
    impl Exovav {
        #[doc = "0 External Overlay cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 External Overlay is available and can be enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Asclav118_SPEC;
    pub type Asclav118 = crate::EnumBitfieldStruct<u8, Asclav118_SPEC>;
    impl Asclav118 {
        #[doc = "ASCLIN 8 9 and 10 11 cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "ASCLIN 8 9 and 10 11 available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Asclav01_SPEC;
    pub type Asclav01 = crate::EnumBitfieldStruct<u8, Asclav01_SPEC>;
    impl Asclav01 {
        #[doc = "0 ASCLIN 0   1        cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ASCLIN 0   1 is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Asclav23_SPEC;
    pub type Asclav23 = crate::EnumBitfieldStruct<u8, Asclav23_SPEC>;
    impl Asclav23 {
        #[doc = "0 ASCLIN 2   3 cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ASCLIN 2   3 is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Asclav45_SPEC;
    pub type Asclav45 = crate::EnumBitfieldStruct<u8, Asclav45_SPEC>;
    impl Asclav45 {
        #[doc = "0 ASCLIN 4   5 cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ASCLIN 4   5 is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Asclav67_SPEC;
    pub type Asclav67 = crate::EnumBitfieldStruct<u8, Asclav67_SPEC>;
    impl Asclav67 {
        #[doc = "0 ASCLIN 6   7        cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ASCLIN 6   7 is        available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Asclav2312_SPEC;
    pub type Asclav2312 = crate::EnumBitfieldStruct<u8, Asclav2312_SPEC>;
    impl Asclav2312 {
        #[doc = "ASCLIN23 22 to ASCLIN 13 12 cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "ASCLIN23 22 to ASCLIN 13 12 are all available and can be enabled  if        present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ememkb_SPEC;
    pub type Ememkb = crate::EnumBitfieldStruct<u8, Ememkb_SPEC>;
    impl Ememkb {
        #[doc = "00 EMEM effective size is 1024KB  if enabled by EMEMAV and present in silicon"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 EMEM effective size is 2048KB  if enabled by EMEMAV and present in silicon"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 EMEM effective size is 3072KB  if enabled by EMEMAV and present in silicon"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 EMEM effective size is 4096KB  if enabled by EMEMAV and present in silicon"]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prdcfg2_SPEC;
impl crate::sealed::RegSpec for Prdcfg2_SPEC {
    type DataType = u32;
}
#[doc = "Product Configuration Register 2\n resetvalue={Application Reset:0x0FFFFFFFF,CFS Value:0x0FFFFFFFF}"]
pub type Prdcfg2 = crate::RegValueT<Prdcfg2_SPEC>;

impl Prdcfg2 {
    #[doc = "CCU6 Availability Control   CCU6AV"]
    #[inline(always)]
    pub fn ccu6av(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, prdcfg2::Ccu6Av, Prdcfg2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,prdcfg2::Ccu6Av, Prdcfg2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GPT12 Availability Control   GPT12AV"]
    #[inline(always)]
    pub fn gpt12av(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, prdcfg2::Gpt12Av, Prdcfg2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            prdcfg2::Gpt12Av,
            Prdcfg2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "HSSL Availability Control   HSSL0AV"]
    #[inline(always)]
    pub fn hssl0av(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, prdcfg2::Hssl0Av, Prdcfg2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            prdcfg2::Hssl0Av,
            Prdcfg2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "GTM Availability Control   GTMAV"]
    #[inline(always)]
    pub fn gtmav(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, prdcfg2::Gtmav, Prdcfg2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,prdcfg2::Gtmav, Prdcfg2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "STM0 Availability Control   STM0AV"]
    #[inline(always)]
    pub fn stm0av(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, prdcfg2::Stm0Av, Prdcfg2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            prdcfg2::Stm0Av,
            Prdcfg2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "STM1 Availability Control   STM1AV"]
    #[inline(always)]
    pub fn stm1av(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, prdcfg2::Stm1Av, Prdcfg2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            prdcfg2::Stm1Av,
            Prdcfg2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "STM2 Availability Control   STM2AV"]
    #[inline(always)]
    pub fn stm2av(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, prdcfg2::Stm2Av, Prdcfg2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            prdcfg2::Stm2Av,
            Prdcfg2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "STM3 Availability Control   STM3AV"]
    #[inline(always)]
    pub fn stm3av(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, prdcfg2::Stm3Av, Prdcfg2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            prdcfg2::Stm3Av,
            Prdcfg2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "STM4 Availability Control   STM4AV"]
    #[inline(always)]
    pub fn stm4av(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, prdcfg2::Stm4Av, Prdcfg2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            prdcfg2::Stm4Av,
            Prdcfg2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "STM5 Availability Control   STM5AV"]
    #[inline(always)]
    pub fn stm5av(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, prdcfg2::Stm5Av, Prdcfg2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            prdcfg2::Stm5Av,
            Prdcfg2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ERAY0 Availability Control   ERAY0AV"]
    #[inline(always)]
    pub fn eray0av(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        prdcfg2::Eray0Av,
        Prdcfg2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            prdcfg2::Eray0Av,
            Prdcfg2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ERAY1 Availability Control   ERAY1AV"]
    #[inline(always)]
    pub fn eray1av(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        prdcfg2::Eray1Av,
        Prdcfg2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            prdcfg2::Eray1Av,
            Prdcfg2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "HSSL1 Availability Control   HSSL1AV"]
    #[inline(always)]
    pub fn hssl1av(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        prdcfg2::Hssl1Av,
        Prdcfg2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            prdcfg2::Hssl1Av,
            Prdcfg2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "SCU Bit Protection   SCUBITPRO"]
    #[inline(always)]
    pub fn scubitpro(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        prdcfg2::Scubitpro,
        Prdcfg2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            prdcfg2::Scubitpro,
            Prdcfg2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Prdcfg2 {
    #[inline(always)]
    fn default() -> Prdcfg2 {
        <crate::RegValueT<Prdcfg2_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod prdcfg2 {
    pub struct Ccu6Av_SPEC;
    pub type Ccu6Av = crate::EnumBitfieldStruct<u8, Ccu6Av_SPEC>;
    impl Ccu6Av {
        #[doc = "0 Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Gpt12Av_SPEC;
    pub type Gpt12Av = crate::EnumBitfieldStruct<u8, Gpt12Av_SPEC>;
    impl Gpt12Av {
        #[doc = "0 Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hssl0Av_SPEC;
    pub type Hssl0Av = crate::EnumBitfieldStruct<u8, Hssl0Av_SPEC>;
    impl Hssl0Av {
        #[doc = "0 Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Gtmav_SPEC;
    pub type Gtmav = crate::EnumBitfieldStruct<u8, Gtmav_SPEC>;
    impl Gtmav {
        #[doc = "0 Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Stm0Av_SPEC;
    pub type Stm0Av = crate::EnumBitfieldStruct<u8, Stm0Av_SPEC>;
    impl Stm0Av {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Stm1Av_SPEC;
    pub type Stm1Av = crate::EnumBitfieldStruct<u8, Stm1Av_SPEC>;
    impl Stm1Av {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Stm2Av_SPEC;
    pub type Stm2Av = crate::EnumBitfieldStruct<u8, Stm2Av_SPEC>;
    impl Stm2Av {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Stm3Av_SPEC;
    pub type Stm3Av = crate::EnumBitfieldStruct<u8, Stm3Av_SPEC>;
    impl Stm3Av {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Stm4Av_SPEC;
    pub type Stm4Av = crate::EnumBitfieldStruct<u8, Stm4Av_SPEC>;
    impl Stm4Av {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Stm5Av_SPEC;
    pub type Stm5Av = crate::EnumBitfieldStruct<u8, Stm5Av_SPEC>;
    impl Stm5Av {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eray0Av_SPEC;
    pub type Eray0Av = crate::EnumBitfieldStruct<u8, Eray0Av_SPEC>;
    impl Eray0Av {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eray1Av_SPEC;
    pub type Eray1Av = crate::EnumBitfieldStruct<u8, Eray1Av_SPEC>;
    impl Eray1Av {
        #[doc = "Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hssl1Av_SPEC;
    pub type Hssl1Av = crate::EnumBitfieldStruct<u8, Hssl1Av_SPEC>;
    impl Hssl1Av {
        #[doc = "0 Module cannot be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module is available and can be enabled  if present in silicon"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scubitpro_SPEC;
    pub type Scubitpro = crate::EnumBitfieldStruct<u8, Scubitpro_SPEC>;
    impl Scubitpro {
        #[doc = "0 Named bitfields in SPARE registers are unprotected and writeable"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Named bitfields in SPARE registers are protected and not writeable"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prdcfg3_SPEC;
impl crate::sealed::RegSpec for Prdcfg3_SPEC {
    type DataType = u32;
}
#[doc = "Product Configuration Register 3\n resetvalue={Application Reset:0x0FFFFFFFF,CFS Value:0x0FFFFFFFF}"]
pub type Prdcfg3 = crate::RegValueT<Prdcfg3_SPEC>;

impl Prdcfg3 {
    #[doc = "CPU0 Data Scratchpad Size   CPU0DSPRKB. Available memory size in KBytes  If present in silicon"]
    #[inline(always)]
    pub fn cpu0dsprkb(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Prdcfg3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Prdcfg3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU0 Program Scratchpad Size   CPU0PSPRKB. Available memory size in KBytes  If present in silicon"]
    #[inline(always)]
    pub fn cpu0psprkb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Prdcfg3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Prdcfg3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU1 Data Scratchpad Size   CPU1DSPRKB. Available memory size in KBytes  If present in silicon"]
    #[inline(always)]
    pub fn cpu1dsprkb(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Prdcfg3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Prdcfg3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU1 Program Scratchpad Size   CPU1PSPRKB. Available memory size in KBytes  If present in silicon"]
    #[inline(always)]
    pub fn cpu1psprkb(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Prdcfg3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Prdcfg3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Prdcfg3 {
    #[inline(always)]
    fn default() -> Prdcfg3 {
        <crate::RegValueT<Prdcfg3_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prdcfg4_SPEC;
impl crate::sealed::RegSpec for Prdcfg4_SPEC {
    type DataType = u32;
}
#[doc = "Product Configuration Register 4\n resetvalue={Application Reset:0x0FFFFFFFF,CFS Value:0x0FFFFFFFF}"]
pub type Prdcfg4 = crate::RegValueT<Prdcfg4_SPEC>;

impl Prdcfg4 {
    #[doc = "CPU2 Data Scratchpad Size   CPU2DSPRKB. Available memory size in KBytes  If present in silicon"]
    #[inline(always)]
    pub fn cpu2dsprkb(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Prdcfg4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Prdcfg4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU2 Program Scratchpad Size   CPU2PSPRKB. Available memory size in KBytes  If present in silicon"]
    #[inline(always)]
    pub fn cpu2psprkb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Prdcfg4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Prdcfg4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU3 Data Scratchpad Size   CPU3DSPRKB. Available memory size in KBytes  If present in silicon"]
    #[inline(always)]
    pub fn cpu3dsprkb(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Prdcfg4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Prdcfg4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU3 Program Scratchpad Size   CPU3PSPRKB. Available memory size in KBytes  If present in silicon"]
    #[inline(always)]
    pub fn cpu3psprkb(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Prdcfg4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Prdcfg4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Prdcfg4 {
    #[inline(always)]
    fn default() -> Prdcfg4 {
        <crate::RegValueT<Prdcfg4_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prdcfg5_SPEC;
impl crate::sealed::RegSpec for Prdcfg5_SPEC {
    type DataType = u32;
}
#[doc = "Product Configuration Register 5\n resetvalue={Application Reset:0x0FFFFFFFF,CFS Value:0x0FFFFFFFF}"]
pub type Prdcfg5 = crate::RegValueT<Prdcfg5_SPEC>;

impl Prdcfg5 {
    #[doc = "CPU4 Data Scratchpad Size   CPU4DSPRKB. Available memory size in KBytes  If present in silicon"]
    #[inline(always)]
    pub fn cpu4dsprkb(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Prdcfg5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Prdcfg5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU4 Program Scratchpad Size   CPU4PSPRKB. Available memory size in KBytes  If present in silicon"]
    #[inline(always)]
    pub fn cpu4psprkb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Prdcfg5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Prdcfg5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU5 Data Scratchpad Size   CPU5DSPRKB. Available memory size in KBytes  If present in silicon"]
    #[inline(always)]
    pub fn cpu5dsprkb(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Prdcfg5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Prdcfg5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU5 Program Scratchpad Size   CPU5PSPRKB. Available memory size in KBytes  If present in silicon"]
    #[inline(always)]
    pub fn cpu5psprkb(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Prdcfg5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Prdcfg5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Prdcfg5 {
    #[inline(always)]
    fn default() -> Prdcfg5 {
        <crate::RegValueT<Prdcfg5_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prdcfg6_SPEC;
impl crate::sealed::RegSpec for Prdcfg6_SPEC {
    type DataType = u32;
}
#[doc = "Product Configuration Register 6\n resetvalue={Application Reset:0x0FFFFFFFF,CFS Value:0x0FFFFFFFF}"]
pub type Prdcfg6 = crate::RegValueT<Prdcfg6_SPEC>;

impl Prdcfg6 {
    #[doc = "CPUx Lockstep Availability Control   LCLxAV. The combinations not listed below shall not be used.  Control via SSW not hardware sideband"]
    #[inline(always)]
    pub fn lclxav(
        self,
    ) -> crate::common::RegisterField<10, 0x7, 1, 0, prdcfg6::LcLxAv, Prdcfg6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            10,
            0x7,
            1,
            0,
            prdcfg6::LcLxAv,
            Prdcfg6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CPU0 Core ID   CPU0CID. Customer Core ID value"]
    #[inline(always)]
    pub fn cpu0cid(
        self,
    ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, Prdcfg6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x7,1,0,u8, Prdcfg6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU1 Core ID   CPU1CID. Customer Core ID value"]
    #[inline(always)]
    pub fn cpu1cid(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Prdcfg6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Prdcfg6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU2 Core ID   CPU2CID. Customer Core ID value"]
    #[inline(always)]
    pub fn cpu2cid(
        self,
    ) -> crate::common::RegisterField<19, 0x7, 1, 0, u8, Prdcfg6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<19,0x7,1,0,u8, Prdcfg6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU3 Core ID   CPU3CID. Customer Core ID value"]
    #[inline(always)]
    pub fn cpu3cid(
        self,
    ) -> crate::common::RegisterField<22, 0x7, 1, 0, u8, Prdcfg6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x7,1,0,u8, Prdcfg6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU4 Core ID   CPU4CID. Customer Core ID value"]
    #[inline(always)]
    pub fn cpu4cid(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, Prdcfg6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<25,0x7,1,0,u8, Prdcfg6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU5 Core ID   CPU5CID. Customer Core ID value"]
    #[inline(always)]
    pub fn cpu5cid(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, Prdcfg6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x7,1,0,u8, Prdcfg6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Prdcfg6 {
    #[inline(always)]
    fn default() -> Prdcfg6 {
        <crate::RegValueT<Prdcfg6_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod prdcfg6 {
    pub struct LcLxAv_SPEC;
    pub type LcLxAv = crate::EnumBitfieldStruct<u8, LcLxAv_SPEC>;
    impl LcLxAv {
        #[doc = "0 Only CPU0 lockstep is available and can be enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 CPU0 and CPU1 locksteps are available and can be enabled"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "1 CPU0  CPU1 and CPU2 locksteps are available and can be enabled"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "1 CPU0  CPU1  CPU2 and CPU3 locksteps are available and can be enabled"]
        pub const CONST_77: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstcon_SPEC;
impl crate::sealed::RegSpec for Rstcon_SPEC {
    type DataType = u32;
}
#[doc = "Reset Configuration Register\n resetvalue={PowerOn Reset:0x282,PowerOn Reset:0x282}"]
pub type Rstcon = crate::RegValueT<Rstcon_SPEC>;

impl Rstcon {
    #[doc = "ESR0 Reset Request Trigger Reset Configuration   ESR0. This bit field defines which reset is generated by a reset request        trigger from ESR0 reset."]
    #[inline(always)]
    pub fn esr0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, rstcon::Esr0, Rstcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,rstcon::Esr0, Rstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ESR1 Reset Request Trigger Reset Configuration   ESR1. This bit field defines which reset is generated by a reset request        trigger from ESR1 reset."]
    #[inline(always)]
    pub fn esr1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, rstcon::Esr1, Rstcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,rstcon::Esr1, Rstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SMU Reset Request Trigger Reset Configuration   SMU. This bit field defines which reset is generated by a reset request        trigger from SMU reset."]
    #[inline(always)]
    pub fn smu(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, rstcon::Smu, Rstcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,rstcon::Smu, Rstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SW Reset Request Trigger Reset Configuration   SW. This bit field defines which reset is generated by a reset request        trigger from software reset."]
    #[inline(always)]
    pub fn sw(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, rstcon::Sw, Rstcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,rstcon::Sw, Rstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "STM0 Reset Request Trigger Reset Configuration   STM0. This bit field defines which reset is generated by a reset request        trigger from STM0 compare match reset."]
    #[inline(always)]
    pub fn stm0(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, rstcon::Stm0, Rstcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,rstcon::Stm0, Rstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "STM1 Reset Request Trigger Reset Configuration  If Product has STM1    STM1. This bit field defines which reset is generated by a reset request        trigger from STM1 compare match reset."]
    #[inline(always)]
    pub fn stm1(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, rstcon::Stm1, Rstcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,rstcon::Stm1, Rstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "STM2 Reset Request Trigger Reset Configuration  If Product has STM2    STM2. This bit field defines which reset is generated by a reset request        trigger from STM2 compare match reset."]
    #[inline(always)]
    pub fn stm2(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, rstcon::Stm2, Rstcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,rstcon::Stm2, Rstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Rstcon {
    #[inline(always)]
    fn default() -> Rstcon {
        <crate::RegValueT<Rstcon_SPEC> as RegisterValue<_>>::new(642)
    }
}
pub mod rstcon {
    pub struct Esr0_SPEC;
    pub type Esr0 = crate::EnumBitfieldStruct<u8, Esr0_SPEC>;
    impl Esr0 {
        #[doc = "00 No reset is        generated for a trigger of ESR0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 A System Reset is generated for a trigger of ESR0 reset"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 An Application Reset is generated for a trigger of ESR0 reset"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Esr1_SPEC;
    pub type Esr1 = crate::EnumBitfieldStruct<u8, Esr1_SPEC>;
    impl Esr1 {
        #[doc = "00 No reset is generated for a trigger of ESR1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 A System Reset is generated for a trigger of ESR1 reset"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 An Application Reset is generated for a trigger of ESR1 reset"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Smu_SPEC;
    pub type Smu = crate::EnumBitfieldStruct<u8, Smu_SPEC>;
    impl Smu {
        #[doc = "00 No reset is generated for a trigger of SMU"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 A System Reset is generated for a trigger of SMU reset"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 An Application Reset is generated for a trigger of SMU reset"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Sw_SPEC;
    pub type Sw = crate::EnumBitfieldStruct<u8, Sw_SPEC>;
    impl Sw {
        #[doc = "00 No reset is generated for a trigger of software reset"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 A System Reset is generated for a trigger of Software reset"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 An Application Reset is generated for a trigger of Software reset"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Stm0_SPEC;
    pub type Stm0 = crate::EnumBitfieldStruct<u8, Stm0_SPEC>;
    impl Stm0 {
        #[doc = "00 No reset is generated for an STM0 trigger"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 A System Reset is generated for a trigger of STM0 reset"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 An Application Reset is generated for a trigger of STM0 reset"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Stm1_SPEC;
    pub type Stm1 = crate::EnumBitfieldStruct<u8, Stm1_SPEC>;
    impl Stm1 {
        #[doc = "00 No reset is generated for a trigger of STM1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 A System Reset is generated for a trigger of STM1 reset"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 An Application Reset is generated for a trigger of STM1 reset"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Stm2_SPEC;
    pub type Stm2 = crate::EnumBitfieldStruct<u8, Stm2_SPEC>;
    impl Stm2 {
        #[doc = "00 No reset is generated for a trigger of STM2"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 A System Reset is generated for a trigger of STM2 reset"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 An Application Reset is generated for a trigger of STM2 reset"]
        pub const CONST_22: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstcon2_SPEC;
impl crate::sealed::RegSpec for Rstcon2_SPEC {
    type DataType = u32;
}
#[doc = "Additional Reset Control Register\n resetvalue={Cold PowerOn Reset:0x0,Cold PowerOn Reset:0x0}"]
pub type Rstcon2 = crate::RegValueT<Rstcon2_SPEC>;

impl Rstcon2 {
    #[doc = "Force Reset Timeout   FRTO"]
    #[inline(always)]
    pub fn frto(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, rstcon2::Frto, Rstcon2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,rstcon2::Frto, Rstcon2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear Cold Reset Status   CLRC. This bit simultaneously clears the sticky status bits which may indicate any previous cold reset  i.e. RSTSTAT.STBYR  RSTSTAT.SWD  RSTSTAT.EVR33  RSTSTAT.EVRC  RSTSTAT.PORST  RSTSTAT.LBPORST and RSTSTAT.LBTERM ."]
    #[inline(always)]
    pub fn clrc(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, rstcon2::Clrc, Rstcon2_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<1,0x1,1,0,rstcon2::Clrc, Rstcon2_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Reset Timeout Counter 20us   TOUT20. This bit indicates elapsed time after the last warm reset request. This        may used by a shutdown routine to schedule its activities independently        from current clock settings"]
    #[inline(always)]
    pub fn tout20(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, rstcon2::Tout20, Rstcon2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1,1,0,rstcon2::Tout20, Rstcon2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Timeout Counter 40us   TOUT40. This bit indicates elapsed time after the last warm reset request. This        may used by a shutdown routine to schedule its activities independently        from current clock settings"]
    #[inline(always)]
    pub fn tout40(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, rstcon2::Tout40, Rstcon2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x1,1,0,rstcon2::Tout40, Rstcon2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Timeout Counter 60us   TOUT60. This bit indicates elapsed time after the last warm reset request. This        may used by a shutdown routine to schedule its activities independently        from current clock settings"]
    #[inline(always)]
    pub fn tout60(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, rstcon2::Tout60, Rstcon2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,rstcon2::Tout60, Rstcon2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Timeout Counter 80us   TOUT80. This bit indicates elapsed time after the last warm reset request. This        may used by a shutdown routine to schedule its activities independently        from current clock settings"]
    #[inline(always)]
    pub fn tout80(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, rstcon2::Tout80, Rstcon2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x1,1,0,rstcon2::Tout80, Rstcon2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Flash Shutdown State Reached   FLSS. Represents the state of the FSI before the last warm reset If this bit is zero after a System Reset  or higher  then it is possible        that Flash reliability could have been affected by the reset."]
    #[inline(always)]
    pub fn flss(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, rstcon2::Flss, Rstcon2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x1,1,0,rstcon2::Flss, Rstcon2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CPU x Shutdown State Reached   CSSx. The state of CPU x before the last warm reset If any bit is zero after an Application Reset  or higher  then it is        possible that SRAM content could have been corrupted by the reset For products with fewer CPUs  only the LSBs are active and unused upper        bits will always read   8216 1  8217"]
    #[inline(always)]
    pub fn cssx(
        self,
    ) -> crate::common::RegisterField<7, 0x3f, 1, 0, rstcon2::CsSx, Rstcon2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<7,0x3f,1,0,rstcon2::CsSx, Rstcon2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "MCDS Shutdown State Reached   MCDSSS. This bit is valid only in an Emulation Device. This bit represents the state of MCDS  or        MiniMCDS  before the last warm reset If this bit is zero after a        warm PORST then it is possible that trace content could have been        corrupted by the reset"]
    #[inline(always)]
    pub fn mcdsss(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, rstcon2::Mcdsss, Rstcon2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<15,0x1,1,0,rstcon2::Mcdsss, Rstcon2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "User Information   USRINFO. User data register  Cleared only on Cold Power on reset . This may be used by an application to store information which must        survive all warm resets"]
    #[inline(always)]
    pub fn usrinfo(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Rstcon2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Rstcon2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Rstcon2 {
    #[inline(always)]
    fn default() -> Rstcon2 {
        <crate::RegValueT<Rstcon2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rstcon2 {
    pub struct Frto_SPEC;
    pub type Frto = crate::EnumBitfieldStruct<u8, Frto_SPEC>;
    impl Frto {
        #[doc = "Start next warm reset internally as soon as Flash and CPUs are idle and        ready to reset"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Start next warm reset internally only when TOUTCNT expires"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clrc_SPEC;
    pub type Clrc = crate::EnumBitfieldStruct<u8, Clrc_SPEC>;
    impl Clrc {
        #[doc = "0 No effect"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear cold reset RSTSTAT status bits"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tout20_SPEC;
    pub type Tout20 = crate::EnumBitfieldStruct<u8, Tout20_SPEC>;
    impl Tout20 {
        #[doc = "0 TOUTCNT  lt  20 microseconds"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 TOUTCNT  gt   20 microseconds."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tout40_SPEC;
    pub type Tout40 = crate::EnumBitfieldStruct<u8, Tout40_SPEC>;
    impl Tout40 {
        #[doc = "0 TOUTCNT  lt  40 microseconds"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 TOUTCNT  gt   40 microseconds."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tout60_SPEC;
    pub type Tout60 = crate::EnumBitfieldStruct<u8, Tout60_SPEC>;
    impl Tout60 {
        #[doc = "0 TOUTCNT  lt  60 microseconds"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 TOUTCNT  gt   60 microseconds."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tout80_SPEC;
    pub type Tout80 = crate::EnumBitfieldStruct<u8, Tout80_SPEC>;
    impl Tout80 {
        #[doc = "0 TOUTCNT  lt  80 microseconds"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 TOUTCNT  gt   80 microseconds."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Flss_SPEC;
    pub type Flss = crate::EnumBitfieldStruct<u8, Flss_SPEC>;
    impl Flss {
        #[doc = "0 FSI flash        shutdown state not reached before last reset"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSI flash        shutdown state reached before last reset"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct CsSx_SPEC;
    pub type CsSx = crate::EnumBitfieldStruct<u8, CsSx_SPEC>;
    impl CsSx {
        #[doc = "0 CPU x shutdown        state not achieved prior to last reset"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 CPU x in        shutdown state at last reset"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mcdsss_SPEC;
    pub type Mcdsss = crate::EnumBitfieldStruct<u8, Mcdsss_SPEC>;
    impl Mcdsss {
        #[doc = "0 MCDS shutdown        state not achieved prior to last reset"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MCDS in        shutdown state at last reset"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rstcon3_SPEC;
impl crate::sealed::RegSpec for Rstcon3_SPEC {
    type DataType = u32;
}
#[doc = "Reset Configuration Register 3\n resetvalue={Cold PowerOn Reset:0x0,After SSW execution:0x08FFF3400}"]
pub type Rstcon3 = crate::RegValueT<Rstcon3_SPEC>;

impl Rstcon3 {
    #[doc = "Shutdown Address   SHDNADDR. This field defines the vector address for the start of the shutdown        routine which is used to ramp down the CPU power consumption in response        to a reset request."]
    #[inline(always)]
    pub fn shdnaddr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Rstcon3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Rstcon3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Rstcon3 {
    #[inline(always)]
    fn default() -> Rstcon3 {
        <crate::RegValueT<Rstcon3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rststat_SPEC;
impl crate::sealed::RegSpec for Rststat_SPEC {
    type DataType = u32;
}
#[doc = "Reset Status Register\n resetvalue={Cold PowerOn Reset:0x13810000,Cold PowerOn Reset:0x13810000}"]
pub type Rststat = crate::RegValueT<Rststat_SPEC>;

impl Rststat {
    #[doc = "Reset Request Trigger Reset Status for ESR0   ESR0"]
    #[inline(always)]
    pub fn esr0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, rststat::Esr0, Rststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,rststat::Esr0, Rststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Request Trigger Reset Status for ESR1   ESR1"]
    #[inline(always)]
    pub fn esr1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, rststat::Esr1, Rststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,rststat::Esr1, Rststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Request Trigger Reset Status for SMU   SMU.  See SMU section for SMU trigger sources  including Watchdog Timers"]
    #[inline(always)]
    pub fn smu(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, rststat::Smu, Rststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x1,1,0,rststat::Smu, Rststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Request Trigger Reset Status for SW   SW"]
    #[inline(always)]
    pub fn sw(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, rststat::Sw, Rststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,rststat::Sw, Rststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Request Trigger Reset Status for STM0 Compare Match   STM0"]
    #[inline(always)]
    pub fn stm0(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, rststat::Stm0, Rststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x1,1,0,rststat::Stm0, Rststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Request Trigger Reset Status for STM1 Compare Match  If Product has STM1    STM1"]
    #[inline(always)]
    pub fn stm1(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, rststat::Stm1, Rststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x1,1,0,rststat::Stm1, Rststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Request Trigger Reset Status for STM2 Compare Match  If Product has STM2    STM2"]
    #[inline(always)]
    pub fn stm2(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, rststat::Stm2, Rststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<7,0x1,1,0,rststat::Stm2, Rststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Request Trigger Reset Status for PORST   PORST. This bit is also set if the bits CB0  CB1  and CB3 are set in parallel."]
    #[inline(always)]
    pub fn porst(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, rststat::Porst, Rststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1,1,0,rststat::Porst, Rststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Request Trigger Reset Status for Cerberus System Reset   CB0"]
    #[inline(always)]
    pub fn cb0(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, rststat::Cb0, Rststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<18,0x1,1,0,rststat::Cb0, Rststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Request Trigger Reset Status for Cerberus Debug Reset   CB1"]
    #[inline(always)]
    pub fn cb1(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, rststat::Cb1, Rststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<19,0x1,1,0,rststat::Cb1, Rststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Request Trigger Reset Status for Cerberus Application Reset   CB3"]
    #[inline(always)]
    pub fn cb3(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, rststat::Cb3, Rststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<20,0x1,1,0,rststat::Cb3, Rststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Request Trigger Reset Status for Tuning Protection   TP"]
    #[inline(always)]
    pub fn tp(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, rststat::Tp, Rststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<21,0x1,1,0,rststat::Tp, Rststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Request Trigger Reset Status for TCU   TCU"]
    #[inline(always)]
    pub fn tcu(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, rststat::Tcu, Rststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<22,0x1,1,0,rststat::Tcu, Rststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Request Trigger Reset Status for EVRC   EVRC"]
    #[inline(always)]
    pub fn evrc(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, rststat::Evrc, Rststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<23,0x1,1,0,rststat::Evrc, Rststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Request Trigger Reset Status for EVR33   EVR33"]
    #[inline(always)]
    pub fn evr33(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, rststat::Evr33, Rststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0x1,1,0,rststat::Evr33, Rststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Request Trigger Reset Status for Supply Watchdog  SWD    SWD. The Supply Watchdog trigger is described in Power Management Controller          8220 Supply Monitoring  8221  chapter"]
    #[inline(always)]
    pub fn swd(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, rststat::Swd, Rststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<25,0x1,1,0,rststat::Swd, Rststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Request Trigger Reset Status for HSM System Reset  HSM S    HSMS"]
    #[inline(always)]
    pub fn hsms(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, rststat::Hsms, Rststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<26,0x1,1,0,rststat::Hsms, Rststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Request Trigger Reset Status for HSM Application Reset  HSM A    HSMA"]
    #[inline(always)]
    pub fn hsma(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, rststat::Hsma, Rststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<27,0x1,1,0,rststat::Hsma, Rststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reset Request Trigger Reset Status for Standby Regulator Watchdog  STBYR    STBYR"]
    #[inline(always)]
    pub fn stbyr(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, rststat::Stbyr, Rststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<28,0x1,1,0,rststat::Stbyr, Rststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "LBIST termination due to PORST. This bitfield indicates if the LBIST was early terminated due to the occurrence of a Power On Reset. If the status of this bitfield is 0  the application must still check the LBTERM to check if the LBIST was terminated properly. This bitfield is cleared when the RSTCON2.CLRC is set."]
    #[inline(always)]
    pub fn lbporst(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, rststat::Lbporst, Rststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            rststat::Lbporst,
            Rststat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "LBIST was properly terminated. This bitfield indicates if the LBIST was terminated properly. This bitfield is cleared when the RSTCON2.CLRC is set."]
    #[inline(always)]
    pub fn lbterm(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, rststat::Lbterm, Rststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<30,0x1,1,0,rststat::Lbterm, Rststat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Rststat {
    #[inline(always)]
    fn default() -> Rststat {
        <crate::RegValueT<Rststat_SPEC> as RegisterValue<_>>::new(268500992)
    }
}
pub mod rststat {
    pub struct Esr0_SPEC;
    pub type Esr0 = crate::EnumBitfieldStruct<u8, Esr0_SPEC>;
    impl Esr0 {
        #[doc = "0 The last reset        was not requested by this reset trigger"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The last reset        was requested by this reset trigger"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esr1_SPEC;
    pub type Esr1 = crate::EnumBitfieldStruct<u8, Esr1_SPEC>;
    impl Esr1 {
        #[doc = "0 The last reset was not requested by this reset trigger"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The last reset was requested by this reset trigger"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Smu_SPEC;
    pub type Smu = crate::EnumBitfieldStruct<u8, Smu_SPEC>;
    impl Smu {
        #[doc = "0 The last reset was not requested by this reset trigger"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The last reset was requested by this reset trigger"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sw_SPEC;
    pub type Sw = crate::EnumBitfieldStruct<u8, Sw_SPEC>;
    impl Sw {
        #[doc = "0 The last reset was not requested by this reset trigger"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The last reset was requested by this reset trigger"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Stm0_SPEC;
    pub type Stm0 = crate::EnumBitfieldStruct<u8, Stm0_SPEC>;
    impl Stm0 {
        #[doc = "0 The last reset        was not requested by this reset trigger"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The last reset        was requested by this reset trigger"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Stm1_SPEC;
    pub type Stm1 = crate::EnumBitfieldStruct<u8, Stm1_SPEC>;
    impl Stm1 {
        #[doc = "0 The last reset was not requested by this reset trigger"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The last reset was requested by this reset trigger"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Stm2_SPEC;
    pub type Stm2 = crate::EnumBitfieldStruct<u8, Stm2_SPEC>;
    impl Stm2 {
        #[doc = "0 The last reset was not requested by this reset trigger"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The last reset was requested by this reset trigger"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Porst_SPEC;
    pub type Porst = crate::EnumBitfieldStruct<u8, Porst_SPEC>;
    impl Porst {
        #[doc = "0 This reset trigger has not occurred since the last clear  by RSTCON2.CLRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 This reset trigger has occurred since the last clear  by RSTCON2.CLRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cb0_SPEC;
    pub type Cb0 = crate::EnumBitfieldStruct<u8, Cb0_SPEC>;
    impl Cb0 {
        #[doc = "0 The last reset was not requested by this reset trigger"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The last reset was requested by this reset trigger"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cb1_SPEC;
    pub type Cb1 = crate::EnumBitfieldStruct<u8, Cb1_SPEC>;
    impl Cb1 {
        #[doc = "0 The last reset was not requested by this reset trigger"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The last reset was requested by this reset trigger"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cb3_SPEC;
    pub type Cb3 = crate::EnumBitfieldStruct<u8, Cb3_SPEC>;
    impl Cb3 {
        #[doc = "0 The last reset was not requested by this reset trigger"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The last reset was requested by this reset trigger"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tp_SPEC;
    pub type Tp = crate::EnumBitfieldStruct<u8, Tp_SPEC>;
    impl Tp {
        #[doc = "0 The last reset was not requested by this reset trigger"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The last reset was requested by this reset trigger"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tcu_SPEC;
    pub type Tcu = crate::EnumBitfieldStruct<u8, Tcu_SPEC>;
    impl Tcu {
        #[doc = "0 The last reset was not requested by this reset trigger"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The last reset was requested by this reset trigger"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Evrc_SPEC;
    pub type Evrc = crate::EnumBitfieldStruct<u8, Evrc_SPEC>;
    impl Evrc {
        #[doc = "0 This reset trigger has not occurred since the last clear  by RSTCON2.CLRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 This reset trigger has occurred since the last clear  by RSTCON2.CLRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Evr33_SPEC;
    pub type Evr33 = crate::EnumBitfieldStruct<u8, Evr33_SPEC>;
    impl Evr33 {
        #[doc = "0 This reset        trigger has not occurred since the last clear  by RSTCON2.CLRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 This reset trigger has occurred since the last clear  by RSTCON2.CLRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Swd_SPEC;
    pub type Swd = crate::EnumBitfieldStruct<u8, Swd_SPEC>;
    impl Swd {
        #[doc = "0 This reset trigger has not occurred since the last clear  by RSTCON2.CLRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 This reset trigger has occurred since the last clear  by RSTCON2.CLRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hsms_SPEC;
    pub type Hsms = crate::EnumBitfieldStruct<u8, Hsms_SPEC>;
    impl Hsms {
        #[doc = "0 The last reset was not requested by this reset trigger"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The last reset was requested by this reset trigger"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hsma_SPEC;
    pub type Hsma = crate::EnumBitfieldStruct<u8, Hsma_SPEC>;
    impl Hsma {
        #[doc = "0 The last reset was not requested by this reset trigger"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The last reset was requested by this reset trigger"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Stbyr_SPEC;
    pub type Stbyr = crate::EnumBitfieldStruct<u8, Stbyr_SPEC>;
    impl Stbyr {
        #[doc = "0 This reset        trigger has not occurred since the last clear  by RSTCON2.CLRC"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 This reset trigger has occurred since the last clear  by RSTCON2.CLRC"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lbporst_SPEC;
    pub type Lbporst = crate::EnumBitfieldStruct<u8, Lbporst_SPEC>;
    impl Lbporst {
        #[doc = "LBIST early termination due to the occurrence of Power On Reset"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LBIST was not terminated early due to a Power On Reset"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lbterm_SPEC;
    pub type Lbterm = crate::EnumBitfieldStruct<u8, Lbterm_SPEC>;
    impl Lbterm {
        #[doc = "LBIST was not terminated properly"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "LBIST was terminated properly"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rtid_SPEC;
impl crate::sealed::RegSpec for Rtid_SPEC {
    type DataType = u32;
}
#[doc = "Redesign Tracing Identification Register\n resetvalue={System Reset:0x0}"]
pub type Rtid = crate::RegValueT<Rtid_SPEC>;

impl Rtid {
    #[doc = "Redesign Trace Bit 15   RT15. RTx can be used  e.g.  for minor redesign stepping identification purposes or enabling of features. The assignment of individual bits in this field will be defined in the TLDS and DfT Specification. Bit 15   Allow access to FSI in test mode Bit 14   Allow STP to be disabled by TCU Bit 13   Unlock Cerberus  ocds hw lock"]
    #[inline(always)]
    pub fn rt0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, rtid::Rt0, Rtid_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,rtid::Rt0, Rtid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Redesign Trace Bit 15   RT15. RTx can be used  e.g.  for minor redesign stepping identification purposes or enabling of features. The assignment of individual bits in this field will be defined in the TLDS and DfT Specification. Bit 15   Allow access to FSI in test mode Bit 14   Allow STP to be disabled by TCU Bit 13   Unlock Cerberus  ocds hw lock"]
    #[inline(always)]
    pub fn rt1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, rtid::Rt1, Rtid_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,rtid::Rt1, Rtid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Redesign Trace Bit 15   RT15. RTx can be used  e.g.  for minor redesign stepping identification purposes or enabling of features. The assignment of individual bits in this field will be defined in the TLDS and DfT Specification. Bit 15   Allow access to FSI in test mode Bit 14   Allow STP to be disabled by TCU Bit 13   Unlock Cerberus  ocds hw lock"]
    #[inline(always)]
    pub fn rt2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, rtid::Rt2, Rtid_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,rtid::Rt2, Rtid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Redesign Trace Bit 15   RT15. RTx can be used  e.g.  for minor redesign stepping identification purposes or enabling of features. The assignment of individual bits in this field will be defined in the TLDS and DfT Specification. Bit 15   Allow access to FSI in test mode Bit 14   Allow STP to be disabled by TCU Bit 13   Unlock Cerberus  ocds hw lock"]
    #[inline(always)]
    pub fn rt3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, rtid::Rt3, Rtid_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1,1,0,rtid::Rt3, Rtid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Redesign Trace Bit 15   RT15. RTx can be used  e.g.  for minor redesign stepping identification purposes or enabling of features. The assignment of individual bits in this field will be defined in the TLDS and DfT Specification. Bit 15   Allow access to FSI in test mode Bit 14   Allow STP to be disabled by TCU Bit 13   Unlock Cerberus  ocds hw lock"]
    #[inline(always)]
    pub fn rt4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, rtid::Rt4, Rtid_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x1,1,0,rtid::Rt4, Rtid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Redesign Trace Bit 15   RT15. RTx can be used  e.g.  for minor redesign stepping identification purposes or enabling of features. The assignment of individual bits in this field will be defined in the TLDS and DfT Specification. Bit 15   Allow access to FSI in test mode Bit 14   Allow STP to be disabled by TCU Bit 13   Unlock Cerberus  ocds hw lock"]
    #[inline(always)]
    pub fn rt5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, rtid::Rt5, Rtid_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x1,1,0,rtid::Rt5, Rtid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Redesign Trace Bit 15   RT15. RTx can be used  e.g.  for minor redesign stepping identification purposes or enabling of features. The assignment of individual bits in this field will be defined in the TLDS and DfT Specification. Bit 15   Allow access to FSI in test mode Bit 14   Allow STP to be disabled by TCU Bit 13   Unlock Cerberus  ocds hw lock"]
    #[inline(always)]
    pub fn rt6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, rtid::Rt6, Rtid_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x1,1,0,rtid::Rt6, Rtid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Redesign Trace Bit 15   RT15. RTx can be used  e.g.  for minor redesign stepping identification purposes or enabling of features. The assignment of individual bits in this field will be defined in the TLDS and DfT Specification. Bit 15   Allow access to FSI in test mode Bit 14   Allow STP to be disabled by TCU Bit 13   Unlock Cerberus  ocds hw lock"]
    #[inline(always)]
    pub fn rt7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, rtid::Rt7, Rtid_SPEC, crate::common::R> {
        crate::common::RegisterField::<7,0x1,1,0,rtid::Rt7, Rtid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Redesign Trace Bit 15   RT15. RTx can be used  e.g.  for minor redesign stepping identification purposes or enabling of features. The assignment of individual bits in this field will be defined in the TLDS and DfT Specification. Bit 15   Allow access to FSI in test mode Bit 14   Allow STP to be disabled by TCU Bit 13   Unlock Cerberus  ocds hw lock"]
    #[inline(always)]
    pub fn rt8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, rtid::Rt8, Rtid_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x1,1,0,rtid::Rt8, Rtid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Redesign Trace Bit 15   RT15. RTx can be used  e.g.  for minor redesign stepping identification purposes or enabling of features. The assignment of individual bits in this field will be defined in the TLDS and DfT Specification. Bit 15   Allow access to FSI in test mode Bit 14   Allow STP to be disabled by TCU Bit 13   Unlock Cerberus  ocds hw lock"]
    #[inline(always)]
    pub fn rt9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, rtid::Rt9, Rtid_SPEC, crate::common::R> {
        crate::common::RegisterField::<9,0x1,1,0,rtid::Rt9, Rtid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Redesign Trace Bit 15   RT15. RTx can be used  e.g.  for minor redesign stepping identification purposes or enabling of features. The assignment of individual bits in this field will be defined in the TLDS and DfT Specification. Bit 15   Allow access to FSI in test mode Bit 14   Allow STP to be disabled by TCU Bit 13   Unlock Cerberus  ocds hw lock"]
    #[inline(always)]
    pub fn rt10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, rtid::Rt10, Rtid_SPEC, crate::common::R> {
        crate::common::RegisterField::<10,0x1,1,0,rtid::Rt10, Rtid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Redesign Trace Bit 15   RT15. RTx can be used  e.g.  for minor redesign stepping identification purposes or enabling of features. The assignment of individual bits in this field will be defined in the TLDS and DfT Specification. Bit 15   Allow access to FSI in test mode Bit 14   Allow STP to be disabled by TCU Bit 13   Unlock Cerberus  ocds hw lock"]
    #[inline(always)]
    pub fn rt11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, rtid::Rt11, Rtid_SPEC, crate::common::R> {
        crate::common::RegisterField::<11,0x1,1,0,rtid::Rt11, Rtid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Redesign Trace Bit 15   RT15. RTx can be used  e.g.  for minor redesign stepping identification purposes or enabling of features. The assignment of individual bits in this field will be defined in the TLDS and DfT Specification. Bit 15   Allow access to FSI in test mode Bit 14   Allow STP to be disabled by TCU Bit 13   Unlock Cerberus  ocds hw lock"]
    #[inline(always)]
    pub fn rt12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, rtid::Rt12, Rtid_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x1,1,0,rtid::Rt12, Rtid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Redesign Trace Bit 15   RT15. RTx can be used  e.g.  for minor redesign stepping identification purposes or enabling of features. The assignment of individual bits in this field will be defined in the TLDS and DfT Specification. Bit 15   Allow access to FSI in test mode Bit 14   Allow STP to be disabled by TCU Bit 13   Unlock Cerberus  ocds hw lock"]
    #[inline(always)]
    pub fn rt13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, rtid::Rt13, Rtid_SPEC, crate::common::R> {
        crate::common::RegisterField::<13,0x1,1,0,rtid::Rt13, Rtid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Redesign Trace Bit 15   RT15. RTx can be used  e.g.  for minor redesign stepping identification purposes or enabling of features. The assignment of individual bits in this field will be defined in the TLDS and DfT Specification. Bit 15   Allow access to FSI in test mode Bit 14   Allow STP to be disabled by TCU Bit 13   Unlock Cerberus  ocds hw lock"]
    #[inline(always)]
    pub fn rt14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, rtid::Rt14, Rtid_SPEC, crate::common::R> {
        crate::common::RegisterField::<14,0x1,1,0,rtid::Rt14, Rtid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Redesign Trace Bit 15   RT15. RTx can be used  e.g.  for minor redesign stepping identification purposes or enabling of features. The assignment of individual bits in this field will be defined in the TLDS and DfT Specification. Bit 15   Allow access to FSI in test mode Bit 14   Allow STP to be disabled by TCU Bit 13   Unlock Cerberus  ocds hw lock"]
    #[inline(always)]
    pub fn rt15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, rtid::Rt15, Rtid_SPEC, crate::common::R> {
        crate::common::RegisterField::<15,0x1,1,0,rtid::Rt15, Rtid_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Rtid {
    #[inline(always)]
    fn default() -> Rtid {
        <crate::RegValueT<Rtid_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rtid {
    pub struct Rt0_SPEC;
    pub type Rt0 = crate::EnumBitfieldStruct<u8, Rt0_SPEC>;
    impl Rt0 {
        #[doc = "0 No change indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A change has been made  without changing bit field CHIPID.CHREV ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rt1_SPEC;
    pub type Rt1 = crate::EnumBitfieldStruct<u8, Rt1_SPEC>;
    impl Rt1 {
        #[doc = "0 No change indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A change has been made  without changing bit field CHIPID.CHREV ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rt2_SPEC;
    pub type Rt2 = crate::EnumBitfieldStruct<u8, Rt2_SPEC>;
    impl Rt2 {
        #[doc = "0 No change indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A change has been made  without changing bit field CHIPID.CHREV ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rt3_SPEC;
    pub type Rt3 = crate::EnumBitfieldStruct<u8, Rt3_SPEC>;
    impl Rt3 {
        #[doc = "0 No change indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A change has been made  without changing bit field CHIPID.CHREV ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rt4_SPEC;
    pub type Rt4 = crate::EnumBitfieldStruct<u8, Rt4_SPEC>;
    impl Rt4 {
        #[doc = "0 No change indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A change has been made  without changing bit field CHIPID.CHREV ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rt5_SPEC;
    pub type Rt5 = crate::EnumBitfieldStruct<u8, Rt5_SPEC>;
    impl Rt5 {
        #[doc = "0 No change indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A change has been made  without changing bit field CHIPID.CHREV ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rt6_SPEC;
    pub type Rt6 = crate::EnumBitfieldStruct<u8, Rt6_SPEC>;
    impl Rt6 {
        #[doc = "0 No change indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A change has been made  without changing bit field CHIPID.CHREV ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rt7_SPEC;
    pub type Rt7 = crate::EnumBitfieldStruct<u8, Rt7_SPEC>;
    impl Rt7 {
        #[doc = "0 No change indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A change has been made  without changing bit field CHIPID.CHREV ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rt8_SPEC;
    pub type Rt8 = crate::EnumBitfieldStruct<u8, Rt8_SPEC>;
    impl Rt8 {
        #[doc = "0 No change indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A change has been made  without changing bit field CHIPID.CHREV ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rt9_SPEC;
    pub type Rt9 = crate::EnumBitfieldStruct<u8, Rt9_SPEC>;
    impl Rt9 {
        #[doc = "0 No change indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A change has been made  without changing bit field CHIPID.CHREV ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rt10_SPEC;
    pub type Rt10 = crate::EnumBitfieldStruct<u8, Rt10_SPEC>;
    impl Rt10 {
        #[doc = "0 No change indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A change has been made  without changing bit field CHIPID.CHREV ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rt11_SPEC;
    pub type Rt11 = crate::EnumBitfieldStruct<u8, Rt11_SPEC>;
    impl Rt11 {
        #[doc = "0 No change indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A change has been made  without changing bit field CHIPID.CHREV ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rt12_SPEC;
    pub type Rt12 = crate::EnumBitfieldStruct<u8, Rt12_SPEC>;
    impl Rt12 {
        #[doc = "0 No change indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A change has been made  without changing bit field CHIPID.CHREV ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rt13_SPEC;
    pub type Rt13 = crate::EnumBitfieldStruct<u8, Rt13_SPEC>;
    impl Rt13 {
        #[doc = "0 No change indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A change has been made  without changing bit field CHIPID.CHREV ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rt14_SPEC;
    pub type Rt14 = crate::EnumBitfieldStruct<u8, Rt14_SPEC>;
    impl Rt14 {
        #[doc = "0 No change indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A change has been made  without changing bit field CHIPID.CHREV ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rt15_SPEC;
    pub type Rt15 = crate::EnumBitfieldStruct<u8, Rt15_SPEC>;
    impl Rt15 {
        #[doc = "0 No change indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A change has been made  without changing bit field CHIPID.CHREV ."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seicon0_SPEC;
impl crate::sealed::RegSpec for Seicon0_SPEC {
    type DataType = u32;
}
#[doc = "Safety ENDINIT Control Register 0\n resetvalue={Application Reset:0x0FFFC000E}"]
pub type Seicon0 = crate::RegValueT<Seicon0_SPEC>;

impl Seicon0 {
    #[doc = "End of Initialization Control Bit   ENDINIT. The current value of ENDINIT is controlled by hardware. It is cleared        after a valid EndInit Password Access to SEICON0  and it is        automatically set again after a valid EndInit Modify Access to SEICON0.        During a write to SEICON0  the value written to this bit is only used        for the password protection mechanism and is not stored. This bit must be cleared during a Password Access to SEICON0  and set        during a Modify Access to SEICON0."]
    #[inline(always)]
    pub fn endinit(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, seicon0::Endinit, Seicon0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            seicon0::Endinit,
            Seicon0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "User Definable Safety ENDINIT Password Field   EPW. This bit field is written with an ENDINIT password value during a Modify        Access. This password is independent from the CPU WDT or WDTS passwords. A read from this bitfield returns this password  but bits  7 2  are        inverted  toggled  to ensure that a simple read write is not sufficient        to service the Safety ENDINIT Timeout Counter. This bit field must be written with its current contents during a        Password Access. The default ENDINIT password after Application Reset is 00000000111100 B"]
    #[inline(always)]
    pub fn epw(
        self,
    ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, Seicon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3fff,1,0,u16, Seicon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reload Value for the Safety  ENDINIT Timeout Counter   REL. The reload value for the Safety ENDINIT Timeout Counter is fixed. This        bitfield always reads as FFFCh and cannot be changed. This bit field must be written with its current contents during a        Password Access. During a Modify Access this bitfield may contain any        value and is ignored."]
    #[inline(always)]
    pub fn rel(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Seicon0_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Seicon0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Seicon0 {
    #[inline(always)]
    fn default() -> Seicon0 {
        <crate::RegValueT<Seicon0_SPEC> as RegisterValue<_>>::new(4294705166)
    }
}
pub mod seicon0 {
    pub struct Endinit_SPEC;
    pub type Endinit = crate::EnumBitfieldStruct<u8, Endinit_SPEC>;
    impl Endinit {
        #[doc = "Access to Safety Endinit protected registers is permitted  default after Application Reset"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Access to Safety Endinit protected registers is not permitted unless        WDTSCON0.ENDINIT is 0."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seicon1_SPEC;
impl crate::sealed::RegSpec for Seicon1_SPEC {
    type DataType = u32;
}
#[doc = "Safety ENDINIT Control Register 1\n resetvalue={Application Reset:0x0}"]
pub type Seicon1 = crate::RegValueT<Seicon1_SPEC>;

impl Seicon1 {
    #[doc = "Input Frequency Request Control   IR1 IR0. Bit IR0 and IR1 should be programmed together to determine the Safety        ENDINIT Timeout Counter frequency. These bits can only be modified when Safety ENDINIT  SE  is de asserted.        SEISR.IS0 and SEISR.IS1 are updated by these bits only when Safety        ENDINIT  SE  is re asserted. As long as an Safety ENDINIT is cleared         SEISR.IS0 and SEISR.IS1 control the current input frequency of the        Safety ENDINIT Timeout Timer. When Safety ENDINIT SE  is re asserted         SEISR.IS0 and SEISR.IS1 are updated with the new values of IR0 and IR1."]
    #[inline(always)]
    pub fn ir0(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, seicon1::Ir0, Seicon1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,seicon1::Ir0, Seicon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disable Request Control Bit   DR. This bit can only be modified when Safety ENDINIT  SE  is de asserted.        SEISR.DS is updated when Safety ENDINIT is re asserted. As long as        Safety ENDINIT is deasserted  bit SEISR.DS controls the current        enable disable status of the WDT. When Safety ENDINIT is re asserted         SEISR.DS is updated with the state of DR."]
    #[inline(always)]
    pub fn dr(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, seicon1::Dr, Seicon1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,seicon1::Dr, Seicon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Frequency Request Control   IR1 IR0. Bit IR0 and IR1 should be programmed together to determine the Safety        ENDINIT Timeout Counter frequency. These bits can only be modified when Safety ENDINIT  SE  is de asserted.        SEISR.IS0 and SEISR.IS1 are updated by these bit only when Safety        ENDINIT  SE  is re asserted. As long as an ENDINIT is cleared  SEISR.IS0        and SEISR.IS1 control the current input frequency of the ENDINIT Timeout        Counter. When Safety ENDINIT SE  is re asserted  SEISR.IS0 and SEISR.IS1        is updated with the new values of IR0 and IR1."]
    #[inline(always)]
    pub fn ir1(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, seicon1::Ir1, Seicon1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,seicon1::Ir1, Seicon1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Seicon1 {
    #[inline(always)]
    fn default() -> Seicon1 {
        <crate::RegValueT<Seicon1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod seicon1 {
    pub struct Ir0_SPEC;
    pub type Ir0 = crate::EnumBitfieldStruct<u8, Ir0_SPEC>;
    impl Ir0 {
        #[doc = "If Bit IR1 0 Request to set input frequency to f SPB  16384.        Elseif Bit IR1 1 Request to set input frequency to f SPB  64."]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Dr_SPEC;
    pub type Dr = crate::EnumBitfieldStruct<u8, Dr_SPEC>;
    impl Dr {
        #[doc = "0 Request to        enable the Safety ENDINIT Timeout counter"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Request to        disable the Safety ENDINIT Timeout counter"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ir1_SPEC;
    pub type Ir1 = crate::EnumBitfieldStruct<u8, Ir1_SPEC>;
    impl Ir1 {
        #[doc = "If Bit IR0 0 Request to set input frequency to f SPB  16384.        Elseif Bit IR0 1 Request to set input frequency to f SPB  256."]
        pub const CONST_00: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Seisr_SPEC;
impl crate::sealed::RegSpec for Seisr_SPEC {
    type DataType = u32;
}
#[doc = "Safety ENDINIT Timeout Status Register\n resetvalue={Application Reset:0x0FFFC0000}"]
pub type Seisr = crate::RegValueT<Seisr_SPEC>;

impl Seisr {
    #[doc = "SEICON0 Access Error Status Flag   AE. This bit is set when an illegal Password Access or Modify Access to        register SEICON0 was attempted. This bit is only cleared on a valid        SEICON0.ENDINIT Modify Access"]
    #[inline(always)]
    pub fn ae(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, seisr::Ae, Seisr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,seisr::Ae, Seisr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SEI Timeout Overflow Error Status Flag   OE. This bit is set when SEISR.TIM overflows from FFFF H to FFFC H . This bit is only cleared on        a valid SEICON0 Modify Access."]
    #[inline(always)]
    pub fn oe(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, seisr::Oe, Seisr_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,seisr::Oe, Seisr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SEI Timeout Input Clock Status   IS0  IS1. Bit IS0 and IS1 should be programmed together. These bits inidicate the        current Safety ENDINIT Timeout Counter clock period. They are updated        with the state of bits SEICON1.IR0 and SEICON1.IR1 after a valid SEICON0        Modify Access."]
    #[inline(always)]
    pub fn is0(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, seisr::Is0, Seisr_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,seisr::Is0, Seisr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SEI Enable Disable Status Flag   DS"]
    #[inline(always)]
    pub fn ds(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, seisr::Ds, Seisr_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1,1,0,seisr::Ds, Seisr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SEI Time Out Mode Flag   TO"]
    #[inline(always)]
    pub fn to(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, seisr::To, Seisr_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x1,1,0,seisr::To, Seisr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SEI Timeout Input Clock Status   IS0  IS1. Bit IS0 and IS1 should be programmed together. These bits inidicate the        current Safety ENDINIT Timeout Counter clock period . They are updated        with the state of bits SEICON1.IR0 and SEICON1.IR1 after a valid SEICON0        Modify Access."]
    #[inline(always)]
    pub fn is1(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, seisr::Is1, Seisr_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x1,1,0,seisr::Is1, Seisr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Timer Value   TIM. Reflects the current content of the Safety EINDINIT Timeout Counter.  Only        bits 17 and 16 are implemented in SEISR. Others return   8216 1  8217"]
    #[inline(always)]
    pub fn tim(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Seisr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Seisr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Seisr {
    #[inline(always)]
    fn default() -> Seisr {
        <crate::RegValueT<Seisr_SPEC> as RegisterValue<_>>::new(4294705152)
    }
}
pub mod seisr {
    pub struct Ae_SPEC;
    pub type Ae = crate::EnumBitfieldStruct<u8, Ae_SPEC>;
    impl Ae {
        #[doc = "0 No access error"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An access error        has occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oe_SPEC;
    pub type Oe = crate::EnumBitfieldStruct<u8, Oe_SPEC>;
    impl Oe {
        #[doc = "0 No overflow        error"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An overflow        error has occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Is0_SPEC;
    pub type Is0 = crate::EnumBitfieldStruct<u8, Is0_SPEC>;
    impl Is0 {
        #[doc = "If Bit IS1 0 Safety ENDINIT Timeout Counter frequency is f SPB  16384.        Elseif Bit IS1 1 Safety ENDINIT Timeout Counter frequency is f SPB  64."]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Ds_SPEC;
    pub type Ds = crate::EnumBitfieldStruct<u8, Ds_SPEC>;
    impl Ds {
        #[doc = "0 The SEI Timeout        Counter is enabled  After SEICON0 Password Access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The SEI timeout        counter is disabled  After SEICON0 Modify Access"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct To_SPEC;
    pub type To = crate::EnumBitfieldStruct<u8, To_SPEC>;
    impl To {
        #[doc = "0 The SEI Timeout        Counter is not operating in Time Out Mode  After SEICON0 Modify Access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The SEI timeout        counter is operating in Time Out Mode  After SEICON0 Password Access"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Is1_SPEC;
    pub type Is1 = crate::EnumBitfieldStruct<u8, Is1_SPEC>;
    impl Is1 {
        #[doc = "If Bit IS0 0 Safety ENDINIT Timeout Counter frequency is f SPB  16384.        Elseif Bit IS0 1 Safety ENDINIT Timeout Counter frequency is f SPB  256."]
        pub const CONST_00: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spare0_SPEC;
impl crate::sealed::RegSpec for Spare0_SPEC {
    type DataType = u32;
}
#[doc = "Spare Register 0\n resetvalue={System Reset:0x0FFFF0000,CFS Value:0x0FFFF0000}"]
pub type Spare0 = crate::RegValueT<Spare0_SPEC>;

impl Spare0 {
    #[doc = "LVDS Bias trimming Value. Contains the LVDS Bias trimming value"]
    #[inline(always)]
    pub fn lbtv(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Spare0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Spare0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Spare0 {
    #[inline(always)]
    fn default() -> Spare0 {
        <crate::RegValueT<Spare0_SPEC> as RegisterValue<_>>::new(4294901760)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stcon_SPEC;
impl crate::sealed::RegSpec for Stcon_SPEC {
    type DataType = u32;
}
#[doc = "Start up Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Stcon = crate::RegValueT<Stcon_SPEC>;

impl Stcon {
    #[doc = "Set Flash Config. Sector Access Enable   SFCBAE. Setting this bit sets bit STSTAT.FCBAE. Reading this bit returns always        a zero. If bits SFCBAE and CFCBAE are both set during the same access then bit          STSTAT.FCBAE is set."]
    #[inline(always)]
    pub fn sfcbae(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Stcon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, Stcon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Flash Config. Sector Access Enable   CFCBAE. Setting this bit clears bit STSTAT.FCBAE. Reading this bit returns        always a zero."]
    #[inline(always)]
    pub fn cfcbae(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Stcon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, Stcon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Start up Protection Setting   STP. This bit is also cleared by an Application Reset. STP is automatically set when a shutdown trap occurs. The start up protection for the system generated out of this bit can be        overruled  deactivated  if bit RTID.RT14 is set AND signal tcu nostp request from the TCU is asserted."]
    #[inline(always)]
    pub fn stp(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, stcon::Stp, Stcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,stcon::Stp, Stcon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Stcon {
    #[inline(always)]
    fn default() -> Stcon {
        <crate::RegValueT<Stcon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod stcon {
    pub struct Stp_SPEC;
    pub type Stp = crate::EnumBitfieldStruct<u8, Stp_SPEC>;
    impl Stp {
        #[doc = "0 Start up code is executed. Start up protection is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Start up code protection is active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stmem0_SPEC;
impl crate::sealed::RegSpec for Stmem0_SPEC {
    type DataType = u32;
}
#[doc = "Start up Memory Register 0\n resetvalue={Cold PowerOn Reset:0x0}"]
pub type Stmem0 = crate::RegValueT<Stmem0_SPEC>;

impl Stmem0 {
    #[doc = "Memory   MEM. This register is used by the start up software as memory."]
    #[inline(always)]
    pub fn mem(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Stmem0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Stmem0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Stmem0 {
    #[inline(always)]
    fn default() -> Stmem0 {
        <crate::RegValueT<Stmem0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stmem1_SPEC;
impl crate::sealed::RegSpec for Stmem1_SPEC {
    type DataType = u32;
}
#[doc = "Start up Memory Register 1\n resetvalue={PowerOn Reset:0x0}"]
pub type Stmem1 = crate::RegValueT<Stmem1_SPEC>;

impl Stmem1 {
    #[doc = "Memory   MEM. This register is used by the start up software as memory."]
    #[inline(always)]
    pub fn mem(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Stmem1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Stmem1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Stmem1 {
    #[inline(always)]
    fn default() -> Stmem1 {
        <crate::RegValueT<Stmem1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stmem2_SPEC;
impl crate::sealed::RegSpec for Stmem2_SPEC {
    type DataType = u32;
}
#[doc = "Start up Memory Register 2\n resetvalue={System Reset:0x0}"]
pub type Stmem2 = crate::RegValueT<Stmem2_SPEC>;

impl Stmem2 {
    #[doc = "Memory   MEM. This register is used by the start up software as memory."]
    #[inline(always)]
    pub fn mem(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Stmem2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Stmem2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Stmem2 {
    #[inline(always)]
    fn default() -> Stmem2 {
        <crate::RegValueT<Stmem2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stmem3_SPEC;
impl crate::sealed::RegSpec for Stmem3_SPEC {
    type DataType = u32;
}
#[doc = "Start up Memory Register 3\n resetvalue={Application Reset:0x0}"]
pub type Stmem3 = crate::RegValueT<Stmem3_SPEC>;

impl Stmem3 {
    #[doc = "Memory   MEM. This register is used by the start up software as memory."]
    #[inline(always)]
    pub fn mem(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Stmem3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Stmem3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Stmem3 {
    #[inline(always)]
    fn default() -> Stmem3 {
        <crate::RegValueT<Stmem3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stmem4_SPEC;
impl crate::sealed::RegSpec for Stmem4_SPEC {
    type DataType = u32;
}
#[doc = "Start up Memory Register 4\n resetvalue={Cold PowerOn Reset:0x0}"]
pub type Stmem4 = crate::RegValueT<Stmem4_SPEC>;

impl Stmem4 {
    #[doc = "Memory   MEM. This register is used by the start up software as memory."]
    #[inline(always)]
    pub fn mem(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Stmem4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Stmem4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Stmem4 {
    #[inline(always)]
    fn default() -> Stmem4 {
        <crate::RegValueT<Stmem4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stmem5_SPEC;
impl crate::sealed::RegSpec for Stmem5_SPEC {
    type DataType = u32;
}
#[doc = "Start up Memory Register 5\n resetvalue={PowerOn Reset:0x0}"]
pub type Stmem5 = crate::RegValueT<Stmem5_SPEC>;

impl Stmem5 {
    #[doc = "Memory   MEM. This register is used by the start up software as memory."]
    #[inline(always)]
    pub fn mem(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Stmem5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Stmem5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Stmem5 {
    #[inline(always)]
    fn default() -> Stmem5 {
        <crate::RegValueT<Stmem5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stmem6_SPEC;
impl crate::sealed::RegSpec for Stmem6_SPEC {
    type DataType = u32;
}
#[doc = "Start up Memory Register 6\n resetvalue={System Reset:0x0}"]
pub type Stmem6 = crate::RegValueT<Stmem6_SPEC>;

impl Stmem6 {
    #[doc = "Memory   MEM. This register is used by the start up software as memory."]
    #[inline(always)]
    pub fn mem(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Stmem6_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Stmem6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Stmem6 {
    #[inline(always)]
    fn default() -> Stmem6 {
        <crate::RegValueT<Stmem6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ststat_SPEC;
impl crate::sealed::RegSpec for Ststat_SPEC {
    type DataType = u32;
}
#[doc = "Start up Status Register\n resetvalue={PowerOn Reset:0x08000}"]
pub type Ststat = crate::RegValueT<Ststat_SPEC>;

impl Ststat {
    #[doc = "Hardware Configuration Setting   HWCFG. This bit field contains the value that is used by the boot software. This bit field is updated in case of an Application Reset with the content by register SWRSTCON.SWCFG if bit SWRSTCON.SWBOOT AND RSTSTAT.SW are set. This bit field is updated in case of an Application Reset with the content of the latches of pins P14.2 P14.6  P10.5  P10.6 if bit SWRSTCON.SWBOOT OR RSTSTAT.SW are cleared and bit STSTAT.LUDIS is cleared. This bit field is left unchanged in case of an Application Reset and is not updated with the content of the latches of pins P14.2 P14.6  P10.5  P10.6 if bit SWRSTCON.SWBOOT OR RSTSTAT.SW are cleared and bit STSTAT.LUDIS is set. The observed reset value after boot depends upon the state of the HWCFG pins"]
    #[inline(always)]
    pub fn hwcfg(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Ststat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Ststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Firmware Test Setting   FTM. This bit field contains the value that is used by the boot software in        Test Mode. This Bit field is updated in Test Mode by the TCU. In        Normal Mode this bit field is updated with 0000000 B and should be ignored by the boot software."]
    #[inline(always)]
    pub fn ftm(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, Ststat_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x7f,1,0,u8, Ststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "MODE   MODE. This bit indicates if the Test Mode is entered or not."]
    #[inline(always)]
    pub fn mode(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, ststat::Mode, Ststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<15,0x1,1,0,ststat::Mode, Ststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Flash Config. Sector Access Enable   FCBAE. This bit can be cleared by setting bit STCON.CFCBAE. This bit can be set by setting bit STCON.SFCBAE. Reset value of this bit is 0"]
    #[inline(always)]
    pub fn fcbae(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, ststat::Fcbae, Ststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1,1,0,ststat::Fcbae, Ststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Latch Update Disable   LUDIS. This bit can be set by setting bit SYSCON.SETLUDIS. Reset value of this bit is 0"]
    #[inline(always)]
    pub fn ludis(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, ststat::Ludis, Ststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<17,0x1,1,0,ststat::Ludis, Ststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "FAR Fuse State   FARFUSE. All FAR fuses must be blown to enter FAR state  See DfT Spec  Reset value of this bit is 0"]
    #[inline(always)]
    pub fn farfuse(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, ststat::Farfuse, Ststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<18,0x1,1,0,ststat::Farfuse, Ststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "TRSTL Status   TRSTL. This bit simply displays the value of TRSTL."]
    #[inline(always)]
    pub fn trstl(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Ststat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ststat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Single Pin DAP Mode Enable   SPDEN"]
    #[inline(always)]
    pub fn spden(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, ststat::Spden, Ststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<20,0x1,1,0,ststat::Spden, Ststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "OTP Over ruled   OTPOFF"]
    #[inline(always)]
    pub fn otpoff(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, ststat::Otpoff, Ststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<21,0x1,1,0,ststat::Otpoff, Ststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "FSI Disabled   FSIOFF. fsi clock disable o signal from TCU"]
    #[inline(always)]
    pub fn fsioff(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, ststat::Fsioff, Ststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<22,0x1,1,0,ststat::Fsioff, Ststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Start Up Protection Over ruling Request   STPOVREQ"]
    #[inline(always)]
    pub fn stpovreq(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, ststat::Stpovreq, Ststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<23,0x1,1,0,ststat::Stpovreq, Ststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "RAM Content Security Integrity   RAMINT. In normal operation this bit can be set or cleared by the application         via SYSCON.RAMINTM . If a test boot mode is entered  the bit is        automatically cleared  and cannot be set again in test mode  because the        content may have been altered This bit is reset only by a cold power on reset."]
    #[inline(always)]
    pub fn ramint(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, ststat::Ramint, Ststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0x1,1,0,ststat::Ramint, Ststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Individual FAR Fuse State   SUBFUSE. Fuse state  latched after last power on reset  For each bit x in this field"]
    #[inline(always)]
    pub fn subfuse(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, ststat::Subfuse, Ststat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<28,0xf,1,0,ststat::Subfuse, Ststat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ststat {
    #[inline(always)]
    fn default() -> Ststat {
        <crate::RegValueT<Ststat_SPEC> as RegisterValue<_>>::new(32768)
    }
}
pub mod ststat {
    pub struct Mode_SPEC;
    pub type Mode = crate::EnumBitfieldStruct<u8, Mode_SPEC>;
    impl Mode {
        #[doc = "0 A Test Mode can be selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Normal Mode is selected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fcbae_SPEC;
    pub type Fcbae = crate::EnumBitfieldStruct<u8, Fcbae_SPEC>;
    impl Fcbae {
        #[doc = "0 Flash config sector is not accessible. Instead the flash memory area is accessed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Flash config sector is accessible. The flash memory area can not be accessed."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ludis_SPEC;
    pub type Ludis = crate::EnumBitfieldStruct<u8, Ludis_SPEC>;
    impl Ludis {
        #[doc = "0 Bit field STSTAT.HWCFG is automatically updated with the latched value of pins P14.2 P14.6  P10.5  P10.6"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit field STSTAT.HWCFG is not updated with the latched value of pins P14.2 P14.6  P10.5  P1.6 P10.6"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Farfuse_SPEC;
    pub type Farfuse = crate::EnumBitfieldStruct<u8, Farfuse_SPEC>;
    impl Farfuse {
        #[doc = "0 FAR disabled One or more FAR fuses remain virgin"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FAR enabled All FAR fuses have been blown   STSTAT.SUBFUSE   1111b"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Spden_SPEC;
    pub type Spden = crate::EnumBitfieldStruct<u8, Spden_SPEC>;
    impl Spden {
        #[doc = "0 Single Pin DAP Mode is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Single Pin DAP Mode is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Otpoff_SPEC;
    pub type Otpoff = crate::EnumBitfieldStruct<u8, Otpoff_SPEC>;
    impl Otpoff {
        #[doc = "0 OTP over ruling pad is low"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 OTP over ruling pad is high"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fsioff_SPEC;
    pub type Fsioff = crate::EnumBitfieldStruct<u8, Fsioff_SPEC>;
    impl Fsioff {
        #[doc = "0 FSI is enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSI is disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Stpovreq_SPEC;
    pub type Stpovreq = crate::EnumBitfieldStruct<u8, Stpovreq_SPEC>;
    impl Stpovreq {
        #[doc = "0 Start Up Protection Over ruling not requested"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Start Up Protection Over ruling is requested"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ramint_SPEC;
    pub type Ramint = crate::EnumBitfieldStruct<u8, Ramint_SPEC>;
    impl Ramint {
        #[doc = "0 RAM Security Integrity cannot be guaranteed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 RAM Security Integrity maintained"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Subfuse_SPEC;
    pub type Subfuse = crate::EnumBitfieldStruct<u8, Subfuse_SPEC>;
    impl Subfuse {
        #[doc = "0 FAR fuse x is virgin"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FAR fuse x has been blown"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swapctrl_SPEC;
impl crate::sealed::RegSpec for Swapctrl_SPEC {
    type DataType = u32;
}
#[doc = "Alternate Address Control Register\n resetvalue={System Reset:0x1}"]
pub type Swapctrl = crate::RegValueT<Swapctrl_SPEC>;

impl Swapctrl {
    #[doc = "Address Configuration. Configures the currently used address region  A B selection"]
    #[inline(always)]
    pub fn addrcfg(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        swapctrl::Addrcfg,
        Swapctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            swapctrl::Addrcfg,
            Swapctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Spare address configuration registers. Spare read write bits"]
    #[inline(always)]
    pub fn spare(
        self,
    ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, Swapctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3fff,1,0,u16, Swapctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Swapctrl {
    #[inline(always)]
    fn default() -> Swapctrl {
        <crate::RegValueT<Swapctrl_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod swapctrl {
    pub struct Addrcfg_SPEC;
    pub type Addrcfg = crate::EnumBitfieldStruct<u8, Addrcfg_SPEC>;
    impl Addrcfg {
        #[doc = "01 Address region A active  B inactive"]
        pub const CONST_01: Self = Self::new(1);
        #[doc = "10 Address region B active  A inactive"]
        pub const CONST_12: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swrstcon_SPEC;
impl crate::sealed::RegSpec for Swrstcon_SPEC {
    type DataType = u32;
}
#[doc = "Software Reset Configuration Register\n resetvalue={PowerOn Reset:0x0,PowerOn Reset:0x0}"]
pub type Swrstcon = crate::RegValueT<Swrstcon_SPEC>;

impl Swrstcon {
    #[doc = "Software Boot Configuration Selection   SWBOOT"]
    #[inline(always)]
    pub fn swboot(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        swrstcon::Swboot,
        Swrstcon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            swrstcon::Swboot,
            Swrstcon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Software Reset Request   SWRSTREQ. This bit is automatically cleared and read always as zero."]
    #[inline(always)]
    pub fn swrstreq(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        swrstcon::Swrstreq,
        Swrstcon_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            swrstcon::Swrstreq,
            Swrstcon_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Software Boot Configuration   SWCFG. A software boot configuration different from the external applied        hardware configuration can be specified with these bits. The configuration encoding is equal to the HWCFG encoding in register        STSTAT.  Internal only  Not communicated to customer in TS"]
    #[inline(always)]
    pub fn swcfg(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Swrstcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Swrstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Swrstcon {
    #[inline(always)]
    fn default() -> Swrstcon {
        <crate::RegValueT<Swrstcon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod swrstcon {
    pub struct Swboot_SPEC;
    pub type Swboot = crate::EnumBitfieldStruct<u8, Swboot_SPEC>;
    impl Swboot {
        #[doc = "0 Bit field        STSTAT.HWCFG is not updated with the content of SWCFG upon an        Application Reset"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit field STSTAT.HWCFG is updated with the content of SWCFG upon an Application Reset"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Swrstreq_SPEC;
    pub type Swrstreq = crate::EnumBitfieldStruct<u8, Swrstreq_SPEC>;
    impl Swrstreq {
        #[doc = "0 No SW Reset is requested"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A SW Reset request trigger is generated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscon_SPEC;
impl crate::sealed::RegSpec for Syscon_SPEC {
    type DataType = u32;
}
#[doc = "System Control Register\n resetvalue={System Reset:0x0}"]
pub type Syscon = crate::RegValueT<Syscon_SPEC>;

impl Syscon {
    #[doc = "Capture Compare Trigger 0   CCTRIG0. This bit is used to trigger the Synchronous Start feature of the CCU6."]
    #[inline(always)]
    pub fn cctrig0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Syscon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Syscon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "RAM Integrity Modify   RAMINTM"]
    #[inline(always)]
    pub fn ramintm(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, syscon::Ramintm, Syscon_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<2,0x3,1,0,syscon::Ramintm, Syscon_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Latch Update Disable   SETLUDIS. Setting this bit sets bit STSTAT.LUDIS. Clearing this bit has no effect. This bit reads always as zero."]
    #[inline(always)]
    pub fn setludis(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Syscon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Syscon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable DXCPL   DDC. Drives signal to TCU"]
    #[inline(always)]
    pub fn ddc(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, syscon::Ddc, Syscon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,syscon::Ddc, Syscon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Syscon {
    #[inline(always)]
    fn default() -> Syscon {
        <crate::RegValueT<Syscon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syscon {
    pub struct Ramintm_SPEC;
    pub type Ramintm = crate::EnumBitfieldStruct<u8, Ramintm_SPEC>;
    impl Ramintm {
        #[doc = "00 No effect"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Set STSTAT.RAMINT  No effect in test mode"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "11 No effect"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "10 Clear STSTAT.RAMINT"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Ddc_SPEC;
    pub type Ddc = crate::EnumBitfieldStruct<u8, Ddc_SPEC>;
    impl Ddc {
        #[doc = "0 DXCPL not disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 DXCPL disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syspllcon0_SPEC;
impl crate::sealed::RegSpec for Syspllcon0_SPEC {
    type DataType = u32;
}
#[doc = "System PLL Configuration 0 Register\n resetvalue={System Reset:0x40003A00}"]
pub type Syspllcon0 = crate::RegValueT<Syspllcon0_SPEC>;

impl Syspllcon0 {
    #[doc = "Modulation Enable   MODEN. This bit controls the activation of the frequency modulation of the        System PLL."]
    #[inline(always)]
    pub fn moden(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        syspllcon0::Moden,
        Syspllcon0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            syspllcon0::Moden,
            Syspllcon0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "N Divider Value   NDIV. The value the N Divider operates with is NDIV 1."]
    #[inline(always)]
    pub fn ndiv(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, Syspllcon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8, Syspllcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "System PLL Power Saving Mode   PLLPWD. If the PLL has been powered down and is getting re enabled via PLLPWD   1  a wait period of 1 ms has to be applied until it is stable without jitter."]
    #[inline(always)]
    pub fn pllpwd(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        syspllcon0::Pllpwd,
        Syspllcon0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            syspllcon0::Pllpwd,
            Syspllcon0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Restart DCO Lock Detection   RESLD. Setting this bit will clear bit SYSPLLSTAT.LOCK and restart the DCO lock detection. Reading this bit returns always a zero."]
    #[inline(always)]
    pub fn resld(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Syspllcon0_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18,1,0,Syspllcon0_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "P Divider Value   PDIV. The value the P Divider operates with is PDIV 1."]
    #[inline(always)]
    pub fn pdiv(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Syspllcon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Syspllcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Selection   INSEL. This bit field defines as clock source for the two PLLs  System       PLL and Peripheral PLL ."]
    #[inline(always)]
    pub fn insel(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x3,
        1,
        0,
        syspllcon0::Insel,
        Syspllcon0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            syspllcon0::Insel,
            Syspllcon0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Syspllcon0 {
    #[inline(always)]
    fn default() -> Syspllcon0 {
        <crate::RegValueT<Syspllcon0_SPEC> as RegisterValue<_>>::new(1073756672)
    }
}
pub mod syspllcon0 {
    pub struct Moden_SPEC;
    pub type Moden = crate::EnumBitfieldStruct<u8, Moden_SPEC>;
    impl Moden {
        #[doc = "Frequency modulation is not activated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Frequency modulation is activated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pllpwd_SPEC;
    pub type Pllpwd = crate::EnumBitfieldStruct<u8, Pllpwd_SPEC>;
    impl Pllpwd {
        #[doc = "The complete System PLL block is put into a Power Saving Mode and can no longer be used. Bypass Mode will remain active if previously selected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Normal behavior"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Insel_SPEC;
    pub type Insel = crate::EnumBitfieldStruct<u8, Insel_SPEC>;
    impl Insel {
        #[doc = "back up        clock is used as clock source"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "f OSC0 is used as clock source"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "SYSCLK        pin is used as clock source"]
        pub const CONST_22: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syspllcon1_SPEC;
impl crate::sealed::RegSpec for Syspllcon1_SPEC {
    type DataType = u32;
}
#[doc = "System PLL Configuration 1 Register\n resetvalue={System Reset:0x5}"]
pub type Syspllcon1 = crate::RegValueT<Syspllcon1_SPEC>;

impl Syspllcon1 {
    #[doc = "K2 Divider Value   K2DIV. The value the K2 Divider operates with is K2DIV 1. While SYSPLLSTAT.K2RDY   0  K2DIV is locked."]
    #[inline(always)]
    pub fn k2div(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Syspllcon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Syspllcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Syspllcon1 {
    #[inline(always)]
    fn default() -> Syspllcon1 {
        <crate::RegValueT<Syspllcon1_SPEC> as RegisterValue<_>>::new(5)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syspllcon2_SPEC;
impl crate::sealed::RegSpec for Syspllcon2_SPEC {
    type DataType = u32;
}
#[doc = "System PLL Configuration 2 Register\n resetvalue={System Reset:0x6000}"]
pub type Syspllcon2 = crate::RegValueT<Syspllcon2_SPEC>;

impl Syspllcon2 {
    #[doc = "Modulation Configuration   MODCFG. This bit field defines the modulation. MODCFG 9 0  defines the modulation amplitude. Bits MODCFG 9 5  are treated as integer part and bits MODCFG 4 0  as fractional part. Bits MODCFG 15 10  have to be configured with the following setting  0x111101 B ."]
    #[inline(always)]
    pub fn modcfg(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Syspllcon2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Syspllcon2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Syspllcon2 {
    #[inline(always)]
    fn default() -> Syspllcon2 {
        <crate::RegValueT<Syspllcon2_SPEC> as RegisterValue<_>>::new(24576)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syspllstat_SPEC;
impl crate::sealed::RegSpec for Syspllstat_SPEC {
    type DataType = u32;
}
#[doc = "System PLL Status Register\n resetvalue={System Reset:0x2,System Reset:0x2}"]
pub type Syspllstat = crate::RegValueT<Syspllstat_SPEC>;

impl Syspllstat {
    #[doc = "System PLL Power saving Mode Status   PWDSTAT"]
    #[inline(always)]
    pub fn pwdstat(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        syspllstat::Pwdstat,
        Syspllstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            syspllstat::Pwdstat,
            Syspllstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "System PLL Lock Status   LOCK. In case of a loss of lock  the f DCO is kept on the previous constant frequency."]
    #[inline(always)]
    pub fn lock(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        syspllstat::Lock,
        Syspllstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            syspllstat::Lock,
            Syspllstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "K2 Divider Ready Status   K2RDY. This bit indicates whether the K2 divider operates on the configured value. This is of interest when the SYSPLLCON1.K2DIV value is changed. The PLL must be enabled and clocked to set the K2RDY field."]
    #[inline(always)]
    pub fn k2rdy(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        syspllstat::K2Rdy,
        Syspllstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            syspllstat::K2Rdy,
            Syspllstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Bypass Mode Status   BY. Bypass Mode can only be entered in Test Mode by the TCU. Therefore this bit is only valid in Test Mode."]
    #[inline(always)]
    pub fn by(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, syspllstat::By, Syspllstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            syspllstat::By,
            Syspllstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Modulation Run   MODRUN. This bit indicates if the frequency modulation of the System PLL is activated or not."]
    #[inline(always)]
    pub fn modrun(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        syspllstat::Modrun,
        Syspllstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            syspllstat::Modrun,
            Syspllstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Syspllstat {
    #[inline(always)]
    fn default() -> Syspllstat {
        <crate::RegValueT<Syspllstat_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod syspllstat {
    pub struct Pwdstat_SPEC;
    pub type Pwdstat = crate::EnumBitfieldStruct<u8, Pwdstat_SPEC>;
    impl Pwdstat {
        #[doc = "System PLL Power saving Mode was not entered"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "System PLL Power saving Mode was entered"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lock_SPEC;
    pub type Lock = crate::EnumBitfieldStruct<u8, Lock_SPEC>;
    impl Lock {
        #[doc = "The frequency of the System PLL is not stable and doesn  x2019 t enable system operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "The frequency of the System PLL is stable and enables system operation"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct K2Rdy_SPEC;
    pub type K2Rdy = crate::EnumBitfieldStruct<u8, K2Rdy_SPEC>;
    impl K2Rdy {
        #[doc = "K2 Divider does not yet operate with the new value"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "K2 Divider operating with the new value"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct By_SPEC;
    pub type By = crate::EnumBitfieldStruct<u8, By_SPEC>;
    impl By {
        #[doc = "Bypass Mode is not active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Bypass Mode is active. Input f OSC is selected as output f PLL0 ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Modrun_SPEC;
    pub type Modrun = crate::EnumBitfieldStruct<u8, Modrun_SPEC>;
    impl Modrun {
        #[doc = "Frequency modulation is not active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Frequency modulation is active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trapclr_SPEC;
impl crate::sealed::RegSpec for Trapclr_SPEC {
    type DataType = u32;
}
#[doc = "Trap Clear Register\n resetvalue={System Reset:0x0}"]
pub type Trapclr = crate::RegValueT<Trapclr_SPEC>;

impl Trapclr {
    #[doc = "Clear Trap Request Flag ESR0T   ESR0T. Setting this bit clears bit TRAPSTAT.ESR0T. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn esr0t(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Trapclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Trapclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Trap Request Flag ESR1T   ESR1T. Setting this bit clears bit TRAPSTAT.ESR1T. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn esr1t(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Trapclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Trapclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Trap Request Flag TRAP2   TRAP2. Setting this bit clears bit TRAPSTAT.TRAP2. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn trap2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Trapclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Trapclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Trap Request Flag SMUT   SMUT. Setting this bit clears bit TRAPSTAT.SMUT. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn smut(self) -> crate::common::RegisterFieldBool<3, 1, 0, Trapclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Trapclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Trapclr {
    #[inline(always)]
    fn default() -> Trapclr {
        <crate::RegValueT<Trapclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trapdis0_SPEC;
impl crate::sealed::RegSpec for Trapdis0_SPEC {
    type DataType = u32;
}
#[doc = "Trap Disable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Trapdis0 = crate::RegValueT<Trapdis0_SPEC>;

impl Trapdis0 {
    #[doc = "Disable Trap Request ESR0T on CPU0   CPU0ESR0T"]
    #[inline(always)]
    pub fn cpu0esr0t(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        trapdis0::Cpu0Esr0T,
        Trapdis0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            trapdis0::Cpu0Esr0T,
            Trapdis0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Disable Trap Request ESR1T on CPU0   CPU0ESR1T"]
    #[inline(always)]
    pub fn cpu0esr1t(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        trapdis0::Cpu0Esr1T,
        Trapdis0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            trapdis0::Cpu0Esr1T,
            Trapdis0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Disable Trap Request TRAP2T on CPU0   CPU0TRAP2T"]
    #[inline(always)]
    pub fn cpu0trap2t(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        trapdis0::Cpu0Trap2T,
        Trapdis0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            trapdis0::Cpu0Trap2T,
            Trapdis0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Disable Trap Request SMUT on CPU0   CPU0SMUT"]
    #[inline(always)]
    pub fn cpu0smut(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        trapdis0::Cpu0Smut,
        Trapdis0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            trapdis0::Cpu0Smut,
            Trapdis0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Disable Trap Request ESR0T on CPU1  If product has CPU1    CPU1ESR0T"]
    #[inline(always)]
    pub fn cpu1esr0t(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        trapdis0::Cpu1Esr0T,
        Trapdis0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            trapdis0::Cpu1Esr0T,
            Trapdis0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Disable Trap Request ESR1T on CPU1  If product has CPU1    CPU1ESR1T"]
    #[inline(always)]
    pub fn cpu1esr1t(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        trapdis0::Cpu1Esr1T,
        Trapdis0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            trapdis0::Cpu1Esr1T,
            Trapdis0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Disable Trap Request TRAP2T on CPU1  If product has CPU1    CPU1TRAP2T"]
    #[inline(always)]
    pub fn cpu1trap2t(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        trapdis0::Cpu1Trap2T,
        Trapdis0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            trapdis0::Cpu1Trap2T,
            Trapdis0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Disable Trap Request SMUT on CPU1  If product has CPU1    CPU1SMUT"]
    #[inline(always)]
    pub fn cpu1smut(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        trapdis0::Cpu1Smut,
        Trapdis0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            trapdis0::Cpu1Smut,
            Trapdis0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Disable Trap Request ESR0T on CPU2  If product has CPU2    CPU2ESR0T"]
    #[inline(always)]
    pub fn cpu2esr0t(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        trapdis0::Cpu2Esr0T,
        Trapdis0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            trapdis0::Cpu2Esr0T,
            Trapdis0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Disable Trap Request ESR1T on CPU2  If product has CPU2    CPU2ESR1T"]
    #[inline(always)]
    pub fn cpu2esr1t(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        trapdis0::Cpu2Esr1T,
        Trapdis0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            trapdis0::Cpu2Esr1T,
            Trapdis0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Disable Trap Request TRAP2T on CPU2  If product has CPU2    CPU2TRAP2T"]
    #[inline(always)]
    pub fn cpu2trap2t(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        trapdis0::Cpu2Trap2T,
        Trapdis0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            trapdis0::Cpu2Trap2T,
            Trapdis0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Disable Trap Request SMUT on CPU2  If product has CPU2    CPU2SMUT"]
    #[inline(always)]
    pub fn cpu2smut(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        trapdis0::Cpu2Smut,
        Trapdis0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            trapdis0::Cpu2Smut,
            Trapdis0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Trapdis0 {
    #[inline(always)]
    fn default() -> Trapdis0 {
        <crate::RegValueT<Trapdis0_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod trapdis0 {
    pub struct Cpu0Esr0T_SPEC;
    pub type Cpu0Esr0T = crate::EnumBitfieldStruct<u8, Cpu0Esr0T_SPEC>;
    impl Cpu0Esr0T {
        #[doc = "0 A CPU0 trap request can be generated for this source"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No trap request can be generated for this source"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0Esr1T_SPEC;
    pub type Cpu0Esr1T = crate::EnumBitfieldStruct<u8, Cpu0Esr1T_SPEC>;
    impl Cpu0Esr1T {
        #[doc = "0 A CPU0 trap request can be generated for this source"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No trap request can be generated for this source"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0Trap2T_SPEC;
    pub type Cpu0Trap2T = crate::EnumBitfieldStruct<u8, Cpu0Trap2T_SPEC>;
    impl Cpu0Trap2T {
        #[doc = "0 A CPU0 trap request can be generated for this source"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No trap request can be generated for this source"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0Smut_SPEC;
    pub type Cpu0Smut = crate::EnumBitfieldStruct<u8, Cpu0Smut_SPEC>;
    impl Cpu0Smut {
        #[doc = "0 A CPU0 trap request can be generated for this source"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No trap request can be generated for this source"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1Esr0T_SPEC;
    pub type Cpu1Esr0T = crate::EnumBitfieldStruct<u8, Cpu1Esr0T_SPEC>;
    impl Cpu1Esr0T {
        #[doc = "0 A CPU1 trap request can be generated for this source"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No trap request can be generated for this source"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1Esr1T_SPEC;
    pub type Cpu1Esr1T = crate::EnumBitfieldStruct<u8, Cpu1Esr1T_SPEC>;
    impl Cpu1Esr1T {
        #[doc = "0 A CPU1 trap request can be generated for this source"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No trap request can be generated for this source"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1Trap2T_SPEC;
    pub type Cpu1Trap2T = crate::EnumBitfieldStruct<u8, Cpu1Trap2T_SPEC>;
    impl Cpu1Trap2T {
        #[doc = "0 A CPU1 trap request can be generated for this source"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No trap request can be generated for this source"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1Smut_SPEC;
    pub type Cpu1Smut = crate::EnumBitfieldStruct<u8, Cpu1Smut_SPEC>;
    impl Cpu1Smut {
        #[doc = "0 A CPU1 trap request can be generated for this source"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No trap request can be generated for this source"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2Esr0T_SPEC;
    pub type Cpu2Esr0T = crate::EnumBitfieldStruct<u8, Cpu2Esr0T_SPEC>;
    impl Cpu2Esr0T {
        #[doc = "0 A CPU2 trap request can be generated for this source"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No trap request can be generated for this source"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2Esr1T_SPEC;
    pub type Cpu2Esr1T = crate::EnumBitfieldStruct<u8, Cpu2Esr1T_SPEC>;
    impl Cpu2Esr1T {
        #[doc = "0 A CPU2 trap request can be generated for this source"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No trap request can be generated for this source"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2Trap2T_SPEC;
    pub type Cpu2Trap2T = crate::EnumBitfieldStruct<u8, Cpu2Trap2T_SPEC>;
    impl Cpu2Trap2T {
        #[doc = "0 A CPU2 trap request can be generated for this source"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No trap request can be generated for this source"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2Smut_SPEC;
    pub type Cpu2Smut = crate::EnumBitfieldStruct<u8, Cpu2Smut_SPEC>;
    impl Cpu2Smut {
        #[doc = "0 A CPU2 trap request can be generated for this source"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No trap request can be generated for this source"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trapset_SPEC;
impl crate::sealed::RegSpec for Trapset_SPEC {
    type DataType = u32;
}
#[doc = "Trap Set Register\n resetvalue={System Reset:0x0}"]
pub type Trapset = crate::RegValueT<Trapset_SPEC>;

impl Trapset {
    #[doc = "Set Trap Request Flag ESR0T   ESR0T. Setting this bit sets bit TRAPSTAT.ESR0T. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn esr0t(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Trapset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Trapset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Trap Request Flag ESR1T   ESR1T. Setting this bit sets bit TRAPSTAT.ESR1T. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn esr1t(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Trapset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Trapset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Trap Request Flag TRAP2   TRAP2. Setting this bit sets bit TRAPSTAT.TRAP2. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn trap2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Trapset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Trapset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set Trap Request Flag SMUT   SMUT. Setting this bit sets bit TRAPSTAT.SMUT. Clearing this bit has no effect. Reading this bit returns always zero."]
    #[inline(always)]
    pub fn smut(self) -> crate::common::RegisterFieldBool<3, 1, 0, Trapset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Trapset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Trapset {
    #[inline(always)]
    fn default() -> Trapset {
        <crate::RegValueT<Trapset_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trapstat_SPEC;
impl crate::sealed::RegSpec for Trapstat_SPEC {
    type DataType = u32;
}
#[doc = "Trap Status Register\n resetvalue={System Reset:0x0}"]
pub type Trapstat = crate::RegValueT<Trapstat_SPEC>;

impl Trapstat {
    #[doc = "ESR0 Trap Request Flag   ESR0T. This bit is set if an ESR0 event is triggered. This bit can be cleared by setting bit TRAPCLR.ESR0T. This bit can be set by setting bit TRAPSET.ESR0T. Observed reset value after boot will depend upon ARI mode because of          ESR pin transition."]
    #[inline(always)]
    pub fn esr0t(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, trapstat::Esr0T, Trapstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,trapstat::Esr0T, Trapstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ESR1 Trap Request Flag   ESR1T. This bit is set if an ESR1 event is triggered. This bit can be cleared by setting bit TRAPCLR.ESR1T. This bit can be set by setting bit TRAPSET.ESR1T. Reset value of this bit is 0"]
    #[inline(always)]
    pub fn esr1t(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, trapstat::Esr1T, Trapstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,trapstat::Esr1T, Trapstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Trap Bit 2 Request Flag   TRAP2. This bit can be cleared by setting bit TRAPCLR.TRAP2. This bit can be set by setting bit TRAPSET.TRAP2. Reset value of this bit is 0"]
    #[inline(always)]
    pub fn trap2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, trapstat::Trap2, Trapstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1,1,0,trapstat::Trap2, Trapstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SMU Alarm Trap Request Flag   SMUT. This bit is set if an SMU Alarm is indicated. This bit can be cleared by setting bit TRAPCLR.SMUT. This bit can be set by setting bit TRAPSET.SMUT. Reset value of this bit is 0"]
    #[inline(always)]
    pub fn smut(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, trapstat::Smut, Trapstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x1,1,0,trapstat::Smut, Trapstat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Trapstat {
    #[inline(always)]
    fn default() -> Trapstat {
        <crate::RegValueT<Trapstat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod trapstat {
    pub struct Esr0T_SPEC;
    pub type Esr0T = crate::EnumBitfieldStruct<u8, Esr0T_SPEC>;
    impl Esr0T {
        #[doc = "0 No trap was requested since this bit was cleared the last time"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A trap was requested since this bit was cleared the last time"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esr1T_SPEC;
    pub type Esr1T = crate::EnumBitfieldStruct<u8, Esr1T_SPEC>;
    impl Esr1T {
        #[doc = "0 No trap was requested since this bit was cleared the last time"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A trap was requested since this bit was cleared the last time"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trap2_SPEC;
    pub type Trap2 = crate::EnumBitfieldStruct<u8, Trap2_SPEC>;
    impl Trap2 {
        #[doc = "0 No trap was requested since this bit was cleared the last time"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A trap was requested since this bit was cleared the last time"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Smut_SPEC;
    pub type Smut = crate::EnumBitfieldStruct<u8, Smut_SPEC>;
    impl Smut {
        #[doc = "0 No trap was requested since this bit was cleared the last time"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A trap was requested since this bit was cleared the last time"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdtcpu0Con0_SPEC;
impl crate::sealed::RegSpec for Wdtcpu0Con0_SPEC {
    type DataType = u32;
}
#[doc = "WDTCPU0CON0"]
pub type Wdtcpu0Con0 = crate::RegValueT<Wdtcpu0Con0_SPEC>;

impl Wdtcpu0Con0 {
    #[doc = "End-of-Initialization Control Bit - ENDINIT"]
    #[inline(always)]
    pub fn endinit(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Wdtcpu0Con0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Wdtcpu0Con0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Bit to Control Access to WDTxCON0 - LCK"]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Wdtcpu0Con0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Wdtcpu0Con0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "User-Definable Password Field for Access to WDTxCON0 - PW"]
    #[inline(always)]
    pub fn pw(
        self,
    ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, Wdtcpu0Con0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3fff,1,0,u16, Wdtcpu0Con0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reload Value for the WDT (also Time Check Value) - REL"]
    #[inline(always)]
    pub fn rel(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Wdtcpu0Con0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Wdtcpu0Con0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Wdtcpu0Con0 {
    #[inline(always)]
    fn default() -> Wdtcpu0Con0 {
        <crate::RegValueT<Wdtcpu0Con0_SPEC> as RegisterValue<_>>::new(4294705166)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdtcpu0Con1_SPEC;
impl crate::sealed::RegSpec for Wdtcpu0Con1_SPEC {
    type DataType = u32;
}
#[doc = "WDTCPU0CON1"]
pub type Wdtcpu0Con1 = crate::RegValueT<Wdtcpu0Con1_SPEC>;

impl Wdtcpu0Con1 {
    #[doc = "Input Frequency Request Control - IR1,IR0"]
    #[inline(always)]
    pub fn ir0(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Wdtcpu0Con1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Wdtcpu0Con1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disable Request Control Bit - DR"]
    #[inline(always)]
    pub fn dr(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Wdtcpu0Con1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Wdtcpu0Con1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Frequency Request Control - IR1,IR0"]
    #[inline(always)]
    pub fn ir1(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Wdtcpu0Con1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Wdtcpu0Con1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Unlock Restriction Request Control Bit - UR"]
    #[inline(always)]
    pub fn ur(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Wdtcpu0Con1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Wdtcpu0Con1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Password Auto-sequence Request Bit - PAR"]
    #[inline(always)]
    pub fn par(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Wdtcpu0Con1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Wdtcpu0Con1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Counter Check Request Bit - TCR"]
    #[inline(always)]
    pub fn tcr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Wdtcpu0Con1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Wdtcpu0Con1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer Check Tolerance Request - TCTR"]
    #[inline(always)]
    pub fn tctr(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, Wdtcpu0Con1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8, Wdtcpu0Con1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Wdtcpu0Con1 {
    #[inline(always)]
    fn default() -> Wdtcpu0Con1 {
        <crate::RegValueT<Wdtcpu0Con1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdtcpu0Sr_SPEC;
impl crate::sealed::RegSpec for Wdtcpu0Sr_SPEC {
    type DataType = u32;
}
#[doc = "WDTCPU0SR"]
pub type Wdtcpu0Sr = crate::RegValueT<Wdtcpu0Sr_SPEC>;

impl Wdtcpu0Sr {
    #[doc = "Watchdog Access Error Status Flag - AE"]
    #[inline(always)]
    pub fn ae(self) -> crate::common::RegisterFieldBool<0, 1, 0, Wdtcpu0Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Wdtcpu0Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Watchdog Overflow Error Status Flag - OE"]
    #[inline(always)]
    pub fn oe(self) -> crate::common::RegisterFieldBool<1, 1, 0, Wdtcpu0Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Wdtcpu0Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Watchdog Input Clock Status - IS1,IS0"]
    #[inline(always)]
    pub fn is0(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Wdtcpu0Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Wdtcpu0Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Watchdog Enable/Disable Status Flag - DS"]
    #[inline(always)]
    pub fn ds(self) -> crate::common::RegisterFieldBool<3, 1, 0, Wdtcpu0Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Wdtcpu0Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Watchdog Time-Out Mode Flag - TO"]
    #[inline(always)]
    pub fn to(self) -> crate::common::RegisterFieldBool<4, 1, 0, Wdtcpu0Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Wdtcpu0Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Watchdog Input Clock Status - IS1,IS0"]
    #[inline(always)]
    pub fn is1(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Wdtcpu0Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Wdtcpu0Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SMU Unlock Restriction Status Flag - US"]
    #[inline(always)]
    pub fn us(self) -> crate::common::RegisterFieldBool<6, 1, 0, Wdtcpu0Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Wdtcpu0Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Password Auto-sequence Status Flag - PAS"]
    #[inline(always)]
    pub fn pas(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Wdtcpu0Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Wdtcpu0Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer Check Status Flag - TCS"]
    #[inline(always)]
    pub fn tcs(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Wdtcpu0Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Wdtcpu0Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer Check Tolerance - TCT"]
    #[inline(always)]
    pub fn tct(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, Wdtcpu0Sr_SPEC, crate::common::R> {
        crate::common::RegisterField::<9,0x7f,1,0,u8, Wdtcpu0Sr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Timer Value - TIM"]
    #[inline(always)]
    pub fn tim(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Wdtcpu0Sr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Wdtcpu0Sr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Wdtcpu0Sr {
    #[inline(always)]
    fn default() -> Wdtcpu0Sr {
        <crate::RegValueT<Wdtcpu0Sr_SPEC> as RegisterValue<_>>::new(4294705168)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdtcpu1Con0_SPEC;
impl crate::sealed::RegSpec for Wdtcpu1Con0_SPEC {
    type DataType = u32;
}
#[doc = "WDTCPU1CON0"]
pub type Wdtcpu1Con0 = crate::RegValueT<Wdtcpu1Con0_SPEC>;

impl Wdtcpu1Con0 {
    #[doc = "End-of-Initialization Control Bit - ENDINIT"]
    #[inline(always)]
    pub fn endinit(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Wdtcpu1Con0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Wdtcpu1Con0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Bit to Control Access to WDTxCON0 - LCK"]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Wdtcpu1Con0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Wdtcpu1Con0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "User-Definable Password Field for Access to WDTxCON0 - PW"]
    #[inline(always)]
    pub fn pw(
        self,
    ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, Wdtcpu1Con0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3fff,1,0,u16, Wdtcpu1Con0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reload Value for the WDT (also Time Check Value) - REL"]
    #[inline(always)]
    pub fn rel(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Wdtcpu1Con0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Wdtcpu1Con0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Wdtcpu1Con0 {
    #[inline(always)]
    fn default() -> Wdtcpu1Con0 {
        <crate::RegValueT<Wdtcpu1Con0_SPEC> as RegisterValue<_>>::new(4294705167)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdtcpu1Con1_SPEC;
impl crate::sealed::RegSpec for Wdtcpu1Con1_SPEC {
    type DataType = u32;
}
#[doc = "WDTCPU1CON1"]
pub type Wdtcpu1Con1 = crate::RegValueT<Wdtcpu1Con1_SPEC>;

impl Wdtcpu1Con1 {
    #[doc = "Input Frequency Request Control - IR1,IR0"]
    #[inline(always)]
    pub fn ir0(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Wdtcpu1Con1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Wdtcpu1Con1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disable Request Control Bit - DR"]
    #[inline(always)]
    pub fn dr(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Wdtcpu1Con1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Wdtcpu1Con1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Frequency Request Control - IR1,IR0"]
    #[inline(always)]
    pub fn ir1(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Wdtcpu1Con1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Wdtcpu1Con1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Unlock Restriction Request Control Bit - UR"]
    #[inline(always)]
    pub fn ur(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Wdtcpu1Con1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Wdtcpu1Con1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Password Auto-sequence Request Bit - PAR"]
    #[inline(always)]
    pub fn par(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Wdtcpu1Con1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Wdtcpu1Con1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Counter Check Request Bit - TCR"]
    #[inline(always)]
    pub fn tcr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Wdtcpu1Con1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Wdtcpu1Con1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer Check Tolerance Request - TCTR"]
    #[inline(always)]
    pub fn tctr(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, Wdtcpu1Con1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8, Wdtcpu1Con1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Wdtcpu1Con1 {
    #[inline(always)]
    fn default() -> Wdtcpu1Con1 {
        <crate::RegValueT<Wdtcpu1Con1_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdtcpu1Sr_SPEC;
impl crate::sealed::RegSpec for Wdtcpu1Sr_SPEC {
    type DataType = u32;
}
#[doc = "WDTCPU1SR"]
pub type Wdtcpu1Sr = crate::RegValueT<Wdtcpu1Sr_SPEC>;

impl Wdtcpu1Sr {
    #[doc = "Watchdog Access Error Status Flag - AE"]
    #[inline(always)]
    pub fn ae(self) -> crate::common::RegisterFieldBool<0, 1, 0, Wdtcpu1Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Wdtcpu1Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Watchdog Overflow Error Status Flag - OE"]
    #[inline(always)]
    pub fn oe(self) -> crate::common::RegisterFieldBool<1, 1, 0, Wdtcpu1Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Wdtcpu1Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Watchdog Input Clock Status - IS1,IS0"]
    #[inline(always)]
    pub fn is0(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Wdtcpu1Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Wdtcpu1Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Watchdog Enable/Disable Status Flag - DS"]
    #[inline(always)]
    pub fn ds(self) -> crate::common::RegisterFieldBool<3, 1, 0, Wdtcpu1Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Wdtcpu1Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Watchdog Time-Out Mode Flag - TO"]
    #[inline(always)]
    pub fn to(self) -> crate::common::RegisterFieldBool<4, 1, 0, Wdtcpu1Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Wdtcpu1Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Watchdog Input Clock Status - IS1,IS0"]
    #[inline(always)]
    pub fn is1(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Wdtcpu1Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Wdtcpu1Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SMU Unlock Restriction Status Flag - US"]
    #[inline(always)]
    pub fn us(self) -> crate::common::RegisterFieldBool<6, 1, 0, Wdtcpu1Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Wdtcpu1Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Password Auto-sequence Status Flag - PAS"]
    #[inline(always)]
    pub fn pas(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Wdtcpu1Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Wdtcpu1Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer Check Status Flag - TCS"]
    #[inline(always)]
    pub fn tcs(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Wdtcpu1Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Wdtcpu1Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer Check Tolerance - TCT"]
    #[inline(always)]
    pub fn tct(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, Wdtcpu1Sr_SPEC, crate::common::R> {
        crate::common::RegisterField::<9,0x7f,1,0,u8, Wdtcpu1Sr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Timer Value - TIM"]
    #[inline(always)]
    pub fn tim(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Wdtcpu1Sr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Wdtcpu1Sr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Wdtcpu1Sr {
    #[inline(always)]
    fn default() -> Wdtcpu1Sr {
        <crate::RegValueT<Wdtcpu1Sr_SPEC> as RegisterValue<_>>::new(4294705160)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdtcpu2Con0_SPEC;
impl crate::sealed::RegSpec for Wdtcpu2Con0_SPEC {
    type DataType = u32;
}
#[doc = "WDTCPU2CON0"]
pub type Wdtcpu2Con0 = crate::RegValueT<Wdtcpu2Con0_SPEC>;

impl Wdtcpu2Con0 {
    #[doc = "End-of-Initialization Control Bit - ENDINIT"]
    #[inline(always)]
    pub fn endinit(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Wdtcpu2Con0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Wdtcpu2Con0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Bit to Control Access to WDTxCON0 - LCK"]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Wdtcpu2Con0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Wdtcpu2Con0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "User-Definable Password Field for Access to WDTxCON0 - PW"]
    #[inline(always)]
    pub fn pw(
        self,
    ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, Wdtcpu2Con0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3fff,1,0,u16, Wdtcpu2Con0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reload Value for the WDT (also Time Check Value) - REL"]
    #[inline(always)]
    pub fn rel(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Wdtcpu2Con0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Wdtcpu2Con0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Wdtcpu2Con0 {
    #[inline(always)]
    fn default() -> Wdtcpu2Con0 {
        <crate::RegValueT<Wdtcpu2Con0_SPEC> as RegisterValue<_>>::new(4294705167)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdtcpu2Con1_SPEC;
impl crate::sealed::RegSpec for Wdtcpu2Con1_SPEC {
    type DataType = u32;
}
#[doc = "WDTCPU2CON1"]
pub type Wdtcpu2Con1 = crate::RegValueT<Wdtcpu2Con1_SPEC>;

impl Wdtcpu2Con1 {
    #[doc = "Input Frequency Request Control - IR1,IR0"]
    #[inline(always)]
    pub fn ir0(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Wdtcpu2Con1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Wdtcpu2Con1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disable Request Control Bit - DR"]
    #[inline(always)]
    pub fn dr(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Wdtcpu2Con1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Wdtcpu2Con1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Frequency Request Control - IR1,IR0"]
    #[inline(always)]
    pub fn ir1(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Wdtcpu2Con1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Wdtcpu2Con1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Unlock Restriction Request Control Bit - UR"]
    #[inline(always)]
    pub fn ur(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Wdtcpu2Con1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Wdtcpu2Con1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Password Auto-sequence Request Bit - PAR"]
    #[inline(always)]
    pub fn par(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Wdtcpu2Con1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Wdtcpu2Con1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Counter Check Request Bit - TCR"]
    #[inline(always)]
    pub fn tcr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Wdtcpu2Con1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,Wdtcpu2Con1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer Check Tolerance Request - TCTR"]
    #[inline(always)]
    pub fn tctr(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, Wdtcpu2Con1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7f,1,0,u8, Wdtcpu2Con1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Wdtcpu2Con1 {
    #[inline(always)]
    fn default() -> Wdtcpu2Con1 {
        <crate::RegValueT<Wdtcpu2Con1_SPEC> as RegisterValue<_>>::new(8)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdtcpu2Sr_SPEC;
impl crate::sealed::RegSpec for Wdtcpu2Sr_SPEC {
    type DataType = u32;
}
#[doc = "WDTCPU2SR"]
pub type Wdtcpu2Sr = crate::RegValueT<Wdtcpu2Sr_SPEC>;

impl Wdtcpu2Sr {
    #[doc = "Watchdog Access Error Status Flag - AE"]
    #[inline(always)]
    pub fn ae(self) -> crate::common::RegisterFieldBool<0, 1, 0, Wdtcpu2Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Wdtcpu2Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Watchdog Overflow Error Status Flag - OE"]
    #[inline(always)]
    pub fn oe(self) -> crate::common::RegisterFieldBool<1, 1, 0, Wdtcpu2Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Wdtcpu2Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Watchdog Input Clock Status - IS1,IS0"]
    #[inline(always)]
    pub fn is0(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Wdtcpu2Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Wdtcpu2Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Watchdog Enable/Disable Status Flag - DS"]
    #[inline(always)]
    pub fn ds(self) -> crate::common::RegisterFieldBool<3, 1, 0, Wdtcpu2Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Wdtcpu2Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Watchdog Time-Out Mode Flag - TO"]
    #[inline(always)]
    pub fn to(self) -> crate::common::RegisterFieldBool<4, 1, 0, Wdtcpu2Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Wdtcpu2Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Watchdog Input Clock Status - IS1,IS0"]
    #[inline(always)]
    pub fn is1(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Wdtcpu2Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Wdtcpu2Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SMU Unlock Restriction Status Flag - US"]
    #[inline(always)]
    pub fn us(self) -> crate::common::RegisterFieldBool<6, 1, 0, Wdtcpu2Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Wdtcpu2Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Password Auto-sequence Status Flag - PAS"]
    #[inline(always)]
    pub fn pas(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Wdtcpu2Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Wdtcpu2Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer Check Status Flag - TCS"]
    #[inline(always)]
    pub fn tcs(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Wdtcpu2Sr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Wdtcpu2Sr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Timer Check Tolerance - TCT"]
    #[inline(always)]
    pub fn tct(
        self,
    ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, Wdtcpu2Sr_SPEC, crate::common::R> {
        crate::common::RegisterField::<9,0x7f,1,0,u8, Wdtcpu2Sr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Timer Value - TIM"]
    #[inline(always)]
    pub fn tim(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Wdtcpu2Sr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Wdtcpu2Sr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Wdtcpu2Sr {
    #[inline(always)]
    fn default() -> Wdtcpu2Sr {
        <crate::RegValueT<Wdtcpu2Sr_SPEC> as RegisterValue<_>>::new(4294705160)
    }
}

#[doc = "ESRCFGx"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EsrcfGx(pub(super) *mut u8);
unsafe impl core::marker::Send for EsrcfGx {}
unsafe impl core::marker::Sync for EsrcfGx {}
impl EsrcfGx {
    #[doc = "ESR0 Input Configuration Register\n resetvalue={System Reset:0x100}"]
    #[inline(always)]
    pub const fn esrcfgx(&self) -> crate::common::Reg<esrcfgx::EsrcfGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod esrcfgx {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EsrcfGx_SPEC;
    impl crate::sealed::RegSpec for EsrcfGx_SPEC {
        type DataType = u32;
    }
    #[doc = "ESR0 Input Configuration Register\n resetvalue={System Reset:0x100}"]
    pub type EsrcfGx = crate::RegValueT<EsrcfGx_SPEC>;

    impl EsrcfGx {
        #[doc = "Edge Detection Control   EDCON. This bit field defines the edges that lead to an ESRx trigger of the        synchronous path."]
        #[inline(always)]
        pub fn edcon(
            self,
        ) -> crate::common::RegisterField<
            7,
            0x3,
            1,
            0,
            esrcfgx::Edcon,
            EsrcfGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                7,
                0x3,
                1,
                0,
                esrcfgx::Edcon,
                EsrcfGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for EsrcfGx {
        #[inline(always)]
        fn default() -> EsrcfGx {
            <crate::RegValueT<EsrcfGx_SPEC> as RegisterValue<_>>::new(256)
        }
    }
    pub mod esrcfgx {
        pub struct Edcon_SPEC;
        pub type Edcon = crate::EnumBitfieldStruct<u8, Edcon_SPEC>;
        impl Edcon {
            #[doc = "00 No trigger is generated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 A trigger is generated upon a rising edge"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 A trigger is generated upon a falling edge"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 A trigger is generated upon a rising OR falling edge"]
            pub const CONST_33: Self = Self::new(3);
        }
    }
}
#[doc = "WDTS"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdts(pub(super) *mut u8);
unsafe impl core::marker::Send for Wdts {}
unsafe impl core::marker::Sync for Wdts {}
impl Wdts {
    #[doc = "Safety WDT Control Register 0\n resetvalue={Application Reset:0x0FFFC000E}"]
    #[inline(always)]
    pub const fn wdtscon0(&self) -> crate::common::Reg<wdts::Wdtscon0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Safety WDT Control Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn wdtscon1(&self) -> crate::common::Reg<wdts::Wdtscon1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Safety WDT Status Register\n resetvalue={Application Reset:0x0FFFC0010}"]
    #[inline(always)]
    pub const fn wdtssr(&self) -> crate::common::Reg<wdts::Wdtssr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
pub mod wdts {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdtscon0_SPEC;
    impl crate::sealed::RegSpec for Wdtscon0_SPEC {
        type DataType = u32;
    }
    #[doc = "Safety WDT Control Register 0\n resetvalue={Application Reset:0x0FFFC000E}"]
    pub type Wdtscon0 = crate::RegValueT<Wdtscon0_SPEC>;

    impl Wdtscon0 {
        #[doc = "End of Initialization Control Bit   ENDINIT. This bit must be written with a   8216 1  8217  during a Password Access or Check        Access  although this write is only used for the password protection        mechanism and is not stored . This bit must be written with the required        ENDINIT update value during a Modify Access. This bit may be used to access registers with   8220 SE  8221  protection  but the        alternate register SEICON0.ENDINIT is recommended for this purpose so        that the Watchdog Timer is not affected."]
        #[inline(always)]
        pub fn endinit(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            wdtscon0::Endinit,
            Wdtscon0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                wdtscon0::Endinit,
                Wdtscon0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Lock Bit to Control Access to WDTxCON0   LCK. The current value of LCK is controlled by hardware. It is cleared after        a valid Password Access to WDTxCON0 when WDTxSR.US is 0  or when        WDTxSR.US is 1 and the SMU is in RUN mode   and it is automatically set        again after a valid Modify Access to WDTxCON0. During a write to        WDTxCON0  the value written to this bit is only used for the        password protection mechanism and is not stored. This bit must be cleared during a Password Access to WDTxCON0  and set        during a Modify Access to WDTxCON0. A Check Access does not clear LCK."]
        #[inline(always)]
        pub fn lck(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            wdtscon0::Lck,
            Wdtscon0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                wdtscon0::Lck,
                Wdtscon0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "User Definable Password Field for Access to WDTxCON0   PW. This bit field is written with an initial password value during a Modify        Access. A read from this bitfield returns this initial password  but bits  7 2         are inverted  toggled  to ensure that a simple read write is not        sufficient to service the WDTx.  This        also provides backward compatibility  If corresponding WDTxSR.PAS   0 then this bit field must be written with        its current contents during a Password Access or Check Access. If corresponding WDTxSR.PAS   1 then this bit field must be written with        the next password in the LFSR sequence during a Password Access or Check        Access The default password after Application Reset is 00000000111100 B"]
        #[inline(always)]
        pub fn pw(
            self,
        ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, Wdtscon0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x3fff,1,0,u16, Wdtscon0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Reload Value for the WDT  also Time Check Value    REL. The reload value can be changed during a Modify Access to WDTxCON0         Default after Application Reset        is FFFC H  .If the Watchdog Timer is        enabled and in Normal Timer Mode  it will start counting from this value        after a correct Watchdog service. A read from this bitfield always returns the current reload value. During a Password Access or a Check Access this bitfield may be used for        additional checks. Writes during such checks have no effect upon the        reload value. If corresponding WDTxSR.TCS 0 then this bit field must be written with        its current contents during a Password Access or Check Access. If corresponding WDTxSR.TCS 1 then this bit field must be written with        an inverted estimate of the current WDTxSR.TIM value during a Password        Access or Check Access."]
        #[inline(always)]
        pub fn rel(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Wdtscon0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, Wdtscon0_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Wdtscon0 {
        #[inline(always)]
        fn default() -> Wdtscon0 {
            <crate::RegValueT<Wdtscon0_SPEC> as RegisterValue<_>>::new(4294705166)
        }
    }
    pub mod wdtscon0 {
        pub struct Endinit_SPEC;
        pub type Endinit = crate::EnumBitfieldStruct<u8, Endinit_SPEC>;
        impl Endinit {
            #[doc = "0 Access to        Endinit protected registers is permitted  default after Application Reset"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Access to        Endinit protected registers is not permitted."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Lck_SPEC;
        pub type Lck = crate::EnumBitfieldStruct<u8, Lck_SPEC>;
        impl Lck {
            #[doc = "0 Register WDTxCON0 is unlocked"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Register WDTxCON0 is locked  default after Application Reset"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdtscon1_SPEC;
    impl crate::sealed::RegSpec for Wdtscon1_SPEC {
        type DataType = u32;
    }
    #[doc = "Safety WDT Control Register 1\n resetvalue={Application Reset:0x0}"]
    pub type Wdtscon1 = crate::RegValueT<Wdtscon1_SPEC>;

    impl Wdtscon1 {
        #[doc = "Clear Internal Reset Flag   CLRIRF. This bit is used to request a clear of the internal flag which indicates        whether a previous SMU reset has already been requested After modification  the internal flag is only cleared when Safety        Endinit  SE  is re asserted. As long as Safety ENDINIT SE  is not        asserted  the internal flag is unchanged and continues to determine the        response to a further SMU reset request. When Safety ENDINIT is        reasserted  the internal flag is cleared together with this bit."]
        #[inline(always)]
        pub fn clrirf(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            wdtscon1::Clrirf,
            Wdtscon1_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                wdtscon1::Clrirf,
                Wdtscon1_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Input Frequency Request Control   IR1 IR0. Bit IR0 and IR1 should be programmed together to determine the WDTx        timer frequency. WDTxSR.IS0 and WDTxSR.IS1 are updated by these bits only when Safety        ENDINIT  SE  is re asserted. As long as Safety ENDINIT is de asserted         WDTxSR.IS0 and WDTxSR.IS1 control the current input frequency of the        Safety Watchdog Timer. When Safety ENDINIT is reasserted  WDTxSR.IS0 and        WDTxSR.IS1 are updated with the values of IR0 and IR1."]
        #[inline(always)]
        pub fn ir0(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            wdtscon1::Ir0,
            Wdtscon1_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                wdtscon1::Ir0,
                Wdtscon1_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Disable Request Control Bit   DR. This bit can only be modified when Safety ENDINIT  SE  is de asserted.        WDTxSR.DS is updated when Safety ENDINIT is re asserted. As long as        Safety ENDINIT is de asserted  bit WDTxSR.DS controls the current        enable disable status of the WDTx. When Safety ENDINIT is reasserted         WDTxSR.DS is updated with the state of DR."]
        #[inline(always)]
        pub fn dr(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            wdtscon1::Dr,
            Wdtscon1_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                wdtscon1::Dr,
                Wdtscon1_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Input Frequency Request Control   IR1 IR0. Bit IR0 and IR1 should be programmed together to determine the WDTx        timer frequency WDTxSR.IS0 and WDTxSR.IS1 are updated by these bits only when Safety        ENDINIT  SE  is re asserted. As long as Safety ENDINIT is de asserted         WDTxSR.IS0 and WDTxSR.IS1 control the current input frequency of the        Safety Watchdog Timer. When Safety ENDINIT is reasserted  WDTxSR.IS0 and        WDTxSR.IS1 are updated with the values of IR0 and IR1."]
        #[inline(always)]
        pub fn ir1(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            wdtscon1::Ir1,
            Wdtscon1_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                wdtscon1::Ir1,
                Wdtscon1_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Unlock Restriction Request Control Bit   UR. This bit can only be modified when Safety ENDINIT  SE  is de asserted.        WDTxSR.US is updated when Safety ENDINIT is reasserted. As long as the        Safety ENDINIT is cleared  bit WDTxSR.US controls whether unlocking is        possible at all times or only when the SMU is not in the FAULT state.        When Safety ENDINIT is reasserted  WDTxSR.US is updated with the state        of UR."]
        #[inline(always)]
        pub fn ur(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x1,
            1,
            0,
            wdtscon1::Ur,
            Wdtscon1_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                6,
                0x1,
                1,
                0,
                wdtscon1::Ur,
                Wdtscon1_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Password Auto sequence Request Bit   PAR. This bit can only be modified when Safety ENDINIT is de asserted.        WDTxSR.PAS is updated when Safety ENDINIT is reasserted. As long as        Safety ENDINIT is de asserted  bit WDTxSR.PAS controls password        sequencing. When Safety ENDINIT is reasserted  WDTxSR.PAS is updated        with the state of PAR."]
        #[inline(always)]
        pub fn par(
            self,
        ) -> crate::common::RegisterField<
            7,
            0x1,
            1,
            0,
            wdtscon1::Par,
            Wdtscon1_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                wdtscon1::Par,
                Wdtscon1_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Counter Check Request Bit   TCR. This bit can only be modified when Safety ENDINIT  SE  is de asserted.        WDTxSR.TCS is updated when Safety ENDINIT is re asserted. As long as        Safety ENDINIT is de asserted  bit WDTxSR.TCS controls whether counter        check is enabled. When Safety ENDINIT is reasserted  WDTxSR.TCS is        updated with the state of TCR"]
        #[inline(always)]
        pub fn tcr(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1,
            1,
            0,
            wdtscon1::Tcr,
            Wdtscon1_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                wdtscon1::Tcr,
                Wdtscon1_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Timer Check Tolerance Request   TCTR. This bit can only be modified when Safety ENDINIT is de asserted.        WDTxSR.TCT is updated when Safety ENDINIT is reasserted. As long as        Safety ENDINIT is de asserted  bit WDTxSR.TCT controls the tolerance of        timer checks. When Safety ENDINIT is re asserted  WDTxSR.TCT is updated        with the state of TCTR."]
        #[inline(always)]
        pub fn tctr(
            self,
        ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, Wdtscon1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<9,0x7f,1,0,u8, Wdtscon1_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Wdtscon1 {
        #[inline(always)]
        fn default() -> Wdtscon1 {
            <crate::RegValueT<Wdtscon1_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod wdtscon1 {
        pub struct Clrirf_SPEC;
        pub type Clrirf = crate::EnumBitfieldStruct<u8, Clrirf_SPEC>;
        impl Clrirf {
            #[doc = "0 No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Request to        clear the internal previous SMU reset flag"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Ir0_SPEC;
        pub type Ir0 = crate::EnumBitfieldStruct<u8, Ir0_SPEC>;
        impl Ir0 {
            #[doc = "If Bit IR1 0 Request to set input frequency to f SPB  16384.        Elseif Bit IR1 1 Request to set input frequency to f SPB  64."]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Dr_SPEC;
        pub type Dr = crate::EnumBitfieldStruct<u8, Dr_SPEC>;
        impl Dr {
            #[doc = "0 Request to        enable the WDTx"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Request to        disable the WDTx"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Ir1_SPEC;
        pub type Ir1 = crate::EnumBitfieldStruct<u8, Ir1_SPEC>;
        impl Ir1 {
            #[doc = "If Bit IR0 0 Request to set input frequency to f SPB  16384.        Elseif Bit IR0 1 Request to set input frequency to f SPB  256."]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Ur_SPEC;
        pub type Ur = crate::EnumBitfieldStruct<u8, Ur_SPEC>;
        impl Ur {
            #[doc = "0 Request to        disable SMU restriction of WDTx unlock"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Request to        enable SMU restriction of WDTx unlock"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Par_SPEC;
        pub type Par = crate::EnumBitfieldStruct<u8, Par_SPEC>;
        impl Par {
            #[doc = "0 Request no        automatic change of password after each Modify Access or Check Access"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Request        automatic sequence of password after each Modify Access or Check Access"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tcr_SPEC;
        pub type Tcr = crate::EnumBitfieldStruct<u8, Tcr_SPEC>;
        impl Tcr {
            #[doc = "0 Request to        check only that REL field is written with existing REL value during        Modify Access or Check Access"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Request to        check that REL field is written with correct TIM Count  within tolerance        of WDTxSR.TCT  during Modify Access or Check Access"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdtssr_SPEC;
    impl crate::sealed::RegSpec for Wdtssr_SPEC {
        type DataType = u32;
    }
    #[doc = "Safety WDT Status Register\n resetvalue={Application Reset:0x0FFFC0010}"]
    pub type Wdtssr = crate::RegValueT<Wdtssr_SPEC>;

    impl Wdtssr {
        #[doc = "Watchdog Access Error Status Flag   AE. This bit is set when an illegal Password Access or Modify Access to        register WDTxCON0 was attempted. This bit is only cleared when        WDTxCON0.ENDINIT is set during a valid Modify Access"]
        #[inline(always)]
        pub fn ae(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, wdtssr::Ae, Wdtssr_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0x1,1,0,wdtssr::Ae, Wdtssr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Watchdog Overflow Error Status Flag   OE. This bit is set when the WDTx overflows from FFFF H to 0000 H . This bit is only cleared        when WDTxCON0.ENDINIT is set to 1 during a valid Modify Access."]
        #[inline(always)]
        pub fn oe(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, wdtssr::Oe, Wdtssr_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<1,0x1,1,0,wdtssr::Oe, Wdtssr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Watchdog Input Clock Status   IS1 IS0. Bit IS0 and IS1 should be programmed together. These bits indicate the        current WDTx clock rate. These bits are updated with the state of bits        WDTxCON1.IR0 and WDTxCON1.IR1 after WDTxCON0.ENDINIT is written with 1        during a valid Modify Access to register WDTxCON0."]
        #[inline(always)]
        pub fn is0(
            self,
        ) -> crate::common::RegisterField<2, 0x1, 1, 0, wdtssr::Is0, Wdtssr_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<2,0x1,1,0,wdtssr::Is0, Wdtssr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Watchdog Enable Disable Status Flag   DS. This bit is updated with the state of bit WDTxCON1.DR  after        WDTxCON0.ENDINIT is set during a Valid Modify Access to register        WDTxCON0  and it is cleared when Time Out mode is entered."]
        #[inline(always)]
        pub fn ds(
            self,
        ) -> crate::common::RegisterField<3, 0x1, 1, 0, wdtssr::Ds, Wdtssr_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<3,0x1,1,0,wdtssr::Ds, Wdtssr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Watchdog Time Out Mode Flag   TO. This bit is set when Time Out Mode is entered. It is automatically        cleared when Time Out Mode is left."]
        #[inline(always)]
        pub fn to(
            self,
        ) -> crate::common::RegisterField<4, 0x1, 1, 0, wdtssr::To, Wdtssr_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<4,0x1,1,0,wdtssr::To, Wdtssr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Watchdog Input Clock Status   IS1 IS0. Bit IS0 and IS1 should be programmed together. These bits indicate the        current WDTx clock rate. These bits are updated with the state of bits        WDTxCON1.IR0 and WDTxCON1.IR1 after WDTxCON0.ENDINIT is written with 1        during a valid Modify Access to register WDTxCON0."]
        #[inline(always)]
        pub fn is1(
            self,
        ) -> crate::common::RegisterField<5, 0x1, 1, 0, wdtssr::Is1, Wdtssr_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<5,0x1,1,0,wdtssr::Is1, Wdtssr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "SMU Unlock Restriction Status Flag   US. WDTxCON0.LCK will not be unlocked by a valid Password Access if this bit        is   8216 1  8217  and the SMU is not in the FAULT state"]
        #[inline(always)]
        pub fn us(
            self,
        ) -> crate::common::RegisterField<6, 0x1, 1, 0, wdtssr::Us, Wdtssr_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<6,0x1,1,0,wdtssr::Us, Wdtssr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Password Auto sequence Status Flag   PAS. This bit is updated with the state of bit WDTxCON1.PAR after        WDTxCON0.ENDINIT is written with 1 during a valid Modify Access to        register WDTxCON0."]
        #[inline(always)]
        pub fn pas(
            self,
        ) -> crate::common::RegisterField<7, 0x1, 1, 0, wdtssr::Pas, Wdtssr_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<7,0x1,1,0,wdtssr::Pas, Wdtssr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Timer Check Status Flag   TCS. This bit is updated with the state of bit WDTxCON1.TCR after        WDTxCON0.ENDINIT is written with 1 during a Valid Modify Access to        register WDTxCON0."]
        #[inline(always)]
        pub fn tcs(
            self,
        ) -> crate::common::RegisterField<8, 0x1, 1, 0, wdtssr::Tcs, Wdtssr_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<8,0x1,1,0,wdtssr::Tcs, Wdtssr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Timer Check Tolerance   TCT. This field determines the tolerance of the timer check during Password        or Check Access  See TCS .This bit is updated with the state of bit        WDTxCON1.TCTR after WDTxCON0.ENDINIT is written with 1 during a Valid        Modify Access to register WDTxCON0."]
        #[inline(always)]
        pub fn tct(
            self,
        ) -> crate::common::RegisterField<9, 0x7f, 1, 0, u8, Wdtssr_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<9,0x7f,1,0,u8, Wdtssr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Timer Value   TIM. Reflects the current content of the WDTx."]
        #[inline(always)]
        pub fn tim(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Wdtssr_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, Wdtssr_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for Wdtssr {
        #[inline(always)]
        fn default() -> Wdtssr {
            <crate::RegValueT<Wdtssr_SPEC> as RegisterValue<_>>::new(4294705168)
        }
    }
    pub mod wdtssr {
        pub struct Ae_SPEC;
        pub type Ae = crate::EnumBitfieldStruct<u8, Ae_SPEC>;
        impl Ae {
            #[doc = "0 No Watchdog        access error"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 A Watchdog        access error has occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Oe_SPEC;
        pub type Oe = crate::EnumBitfieldStruct<u8, Oe_SPEC>;
        impl Oe {
            #[doc = "0 No Watchdog        overflow error"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 A Watchdog        overflow error has occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Is0_SPEC;
        pub type Is0 = crate::EnumBitfieldStruct<u8, Is0_SPEC>;
        impl Is0 {
            #[doc = "If Bit IS1 0 WDTx counter frequency is f SPB  16384. Elseif Bit        IS1 1 WDTx counter frequency is f SPB  64."]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Ds_SPEC;
        pub type Ds = crate::EnumBitfieldStruct<u8, Ds_SPEC>;
        impl Ds {
            #[doc = "0 WDTx is enabled         default after Application Reset"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 WDTx is        disabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct To_SPEC;
        pub type To = crate::EnumBitfieldStruct<u8, To_SPEC>;
        impl To {
            #[doc = "0 The Watchdog is        not operating in Time Out Mode"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 The Watchdog is        operating in Time Out Mode  default after Application Reset"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Is1_SPEC;
        pub type Is1 = crate::EnumBitfieldStruct<u8, Is1_SPEC>;
        impl Is1 {
            #[doc = "If Bit IS0 0 WDTx counter frequency is f SPB  16384. Elseif Bit        IS0 1 WDTx counter frequency is f SPB  256."]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Us_SPEC;
        pub type Us = crate::EnumBitfieldStruct<u8, Us_SPEC>;
        impl Us {
            #[doc = "0 WDTx unlock        permitted at any time"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 WDTx unlock        only permitted when the SMU is in not the FAULT state."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Pas_SPEC;
        pub type Pas = crate::EnumBitfieldStruct<u8, Pas_SPEC>;
        impl Pas {
            #[doc = "0 No change of        password after each Modify Access or Check Access"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Automatically        sequence the password after each Modify Access or Check Access"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tcs_SPEC;
        pub type Tcs = crate::EnumBitfieldStruct<u8, Tcs_SPEC>;
        impl Tcs {
            #[doc = "0 Check only that        REL field is written with existing REL value during Modify Access or        Check Access"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Check that REL        field is written with inverted estimated TIM value  within     TCT        value  during Password Access or Check Access"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
}
