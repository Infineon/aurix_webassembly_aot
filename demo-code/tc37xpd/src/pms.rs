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
#[doc = r"PMS"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pms(pub(super) *mut u8);
unsafe impl core::marker::Send for Pms {}
unsafe impl core::marker::Sync for Pms {}
impl Pms {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(508usize)) }
    }

    #[doc = "SMU stdby FSP Configuration Register\n resetvalue={LVD Reset:0x0,Warm PORST:0x0}"]
    #[inline(always)]
    pub const fn ag2ifsp_stdby(
        &self,
    ) -> [crate::common::Reg<self::Ag2IFspStdby_SPEC, crate::common::RW>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x1a4usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1a4usize + 0x4usize)),
            ]
        }
    }

    #[doc = "Alarm Status Register\n resetvalue={LVD Reset:0x0}"]
    #[inline(always)]
    pub const fn ag2i_stdby(
        &self,
    ) -> [crate::common::Reg<self::Ag2IStdby_SPEC, crate::common::RW>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x188usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x188usize + 0x4usize)),
            ]
        }
    }

    #[doc = "SMU stdby Command Register\n resetvalue={LVD Reset:0x0,Warm PORST:0x0}"]
    #[inline(always)]
    pub const fn cmd_stdby(&self) -> crate::common::Reg<self::CmdStdby_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(412usize)) }
    }

    #[doc = "Die Temperature Sensor Control Register\n resetvalue={LVD Reset:0x200,Cold PORST:0x200,After SSW execution:0x200}"]
    #[inline(always)]
    pub const fn dtscon(&self) -> crate::common::Reg<self::Dtscon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(452usize)) }
    }

    #[doc = "Die Temperature Sensor Limit Register\n resetvalue={LVD Reset:0x0CD806D6,Cold PORST:0x0CD806D6}"]
    #[inline(always)]
    pub const fn dtslim(&self) -> crate::common::Reg<self::Dtslim_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(456usize)) }
    }

    #[doc = "Die Temperature Sensor Status Register\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
    #[inline(always)]
    pub const fn dtsstat(&self) -> crate::common::Reg<self::Dtsstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(448usize)) }
    }

    #[doc = "EVR33 Control Register\n resetvalue={LVD Reset:0x407ED,Cold PORST:0x407ED,After SSW execution:0x407ED}"]
    #[inline(always)]
    pub const fn evr33con(&self) -> crate::common::Reg<self::Evr33Con_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }

    #[doc = "EVR Primary ADC Status Register\n resetvalue={LVD Reset:0x0}"]
    #[inline(always)]
    pub const fn evradcstat(&self) -> crate::common::Reg<self::Evradcstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "EVR Analog Control Register\n resetvalue={LVD Reset:0x0,After SSW execution:0x0}"]
    #[inline(always)]
    pub const fn evranactrl(&self) -> crate::common::Reg<self::Evranactrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(152usize)) }
    }

    #[doc = "EVR Bandgap and Oscillator Control Register\n resetvalue={LVD Reset:0x080109340,After SSW execution:0x080108140}"]
    #[inline(always)]
    pub const fn evrbgoctrl(&self) -> crate::common::Reg<self::Evrbgoctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(156usize)) }
    }

    #[doc = "EVR Bandgap and Oscillator Control Register 2\n resetvalue={LVD Reset:0x09,After SSW execution:0x09}"]
    #[inline(always)]
    pub const fn evrbgoctrl2(
        &self,
    ) -> crate::common::Reg<self::Evrbgoctrl2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(168usize)) }
    }

    #[doc = "EVRC Control Register\n resetvalue={LVD Reset:0x0,Cold PORST:0x0,After SSW execution:0x0}"]
    #[inline(always)]
    pub const fn evrccon(&self) -> crate::common::Reg<self::Evrccon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(148usize)) }
    }

    #[doc = "EVR Secondary Monitor Control Register\n resetvalue={LVD Reset:0x0A5A5A5,Cold PORST:0x0A5A5A5,After SSW execution:0x0A5A5A5}"]
    #[inline(always)]
    pub const fn evrmonctrl(&self) -> crate::common::Reg<self::Evrmonctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(104usize)) }
    }

    #[doc = "EVR Secondary Monitor Filter Register\n resetvalue={LVD Reset:0x300,Cold PORST:0x300,After SSW execution:0x10301}"]
    #[inline(always)]
    pub const fn evrmonfilt(&self) -> crate::common::Reg<self::Evrmonfilt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(112usize)) }
    }

    #[doc = "EVR Secondary ADC Status Register 1\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
    #[inline(always)]
    pub const fn evrmonstat1(
        &self,
    ) -> crate::common::Reg<self::Evrmonstat1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }

    #[doc = "EVR Secondary ADC Status Register 2\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
    #[inline(always)]
    pub const fn evrmonstat2(
        &self,
    ) -> crate::common::Reg<self::Evrmonstat2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(100usize)) }
    }

    #[doc = "EVR Oscillator Control Register\n resetvalue={LVD Reset:0x1F,After SSW execution:0x2000001F}"]
    #[inline(always)]
    pub const fn evroscctrl(&self) -> crate::common::Reg<self::Evroscctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }

    #[doc = "EVR Oscillator Control Register 2\n resetvalue={LVD Reset:0x0B1A05C,After SSW execution:0x0FF4440}"]
    #[inline(always)]
    pub const fn evroscctrl2(
        &self,
    ) -> crate::common::Reg<self::Evroscctrl2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(164usize)) }
    }

    #[doc = "EVR Secondary Over voltage Monitor Register\n resetvalue={LVD Reset:0x0FEFEFE,Cold PORST:0x0FEFEFE,After SSW execution:0x0FEFEFE}"]
    #[inline(always)]
    pub const fn evrovmon(&self) -> crate::common::Reg<self::Evrovmon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(124usize)) }
    }

    #[doc = "EVR Secondary Over voltage Monitor Register 2\n resetvalue={LVD Reset:0x0FEFEFE,Cold PORST:0x0FEFEFE,After SSW execution:0x0FEFEFE}"]
    #[inline(always)]
    pub const fn evrovmon2(&self) -> crate::common::Reg<self::Evrovmon2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }

    #[doc = "EVR Reset Control Register\n resetvalue={LVD Reset:0x597F4A,Cold PORST:0x597F4A,After SSW execution:0x5C834B}"]
    #[inline(always)]
    pub const fn evrrstcon(&self) -> crate::common::Reg<self::Evrrstcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "EVR Reset Control Register 2\n resetvalue={LVD Reset:0x0,Cold PORST:0x0,After SSW execution:0x0}"]
    #[inline(always)]
    pub const fn evrrstcon2(&self) -> crate::common::Reg<self::Evrrstcon2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "EVR Reset Hysteresis Control Register\n resetvalue={LVD Reset:0x15040404,Cold PORST:0x15040404,After SSW execution:0x15040404}"]
    #[inline(always)]
    pub const fn evrrsthys(&self) -> crate::common::Reg<self::Evrrsthys_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }

    #[doc = "EVR Reset Status Register\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
    #[inline(always)]
    pub const fn evrrststat(&self) -> crate::common::Reg<self::Evrrststat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }

    #[doc = "EVRC SD Coefficient Register 0\n resetvalue={LVD Reset:0x0B50873B6,Cold PORST:0x0B50873B6,After SSW execution:0x0B50873B6}"]
    #[inline(always)]
    pub const fn evrsdcoeff0(
        &self,
    ) -> crate::common::Reg<self::Evrsdcoeff0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(328usize)) }
    }

    #[doc = "EVRC SD Coefficient Register 1\n resetvalue={LVD Reset:0x0A2946C46,Cold PORST:0x0A2946C46,After SSW execution:0x0A2946C46}"]
    #[inline(always)]
    pub const fn evrsdcoeff1(
        &self,
    ) -> crate::common::Reg<self::Evrsdcoeff1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(332usize)) }
    }

    #[doc = "EVRC SD Coefficient Register 2\n resetvalue={LVD Reset:0x3408710E,Cold PORST:0x3408710E,After SSW execution:0x3408710E}"]
    #[inline(always)]
    pub const fn evrsdcoeff2(
        &self,
    ) -> crate::common::Reg<self::Evrsdcoeff2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(336usize)) }
    }

    #[doc = "EVRC SD Coefficient Register 3\n resetvalue={LVD Reset:0x2946C44,Cold PORST:0x2946C44,After SSW execution:0x2946C44}"]
    #[inline(always)]
    pub const fn evrsdcoeff3(
        &self,
    ) -> crate::common::Reg<self::Evrsdcoeff3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(340usize)) }
    }

    #[doc = "EVRC SD Coefficient Register 4\n resetvalue={LVD Reset:0x1B0822B6,Cold PORST:0x1B0822B6,After SSW execution:0x1B0822B6}"]
    #[inline(always)]
    pub const fn evrsdcoeff4(
        &self,
    ) -> crate::common::Reg<self::Evrsdcoeff4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(344usize)) }
    }

    #[doc = "EVRC SD Coefficient Register 5\n resetvalue={LVD Reset:0x2946C46,Cold PORST:0x2946C46,After SSW execution:0x2946C46}"]
    #[inline(always)]
    pub const fn evrsdcoeff5(
        &self,
    ) -> crate::common::Reg<self::Evrsdcoeff5_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(348usize)) }
    }

    #[doc = "EVRC SD Coefficient Register 6\n resetvalue={LVD Reset:0x080971802,Cold PORST:0x080971802,After SSW execution:0x080971802}"]
    #[inline(always)]
    pub const fn evrsdcoeff6(
        &self,
    ) -> crate::common::Reg<self::Evrsdcoeff6_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(352usize)) }
    }

    #[doc = "EVRC SD Coefficient Register 7\n resetvalue={LVD Reset:0x08000D8F7,Cold PORST:0x08000D8F7,After SSW execution:0x08000D8F7}"]
    #[inline(always)]
    pub const fn evrsdcoeff7(
        &self,
    ) -> crate::common::Reg<self::Evrsdcoeff7_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(356usize)) }
    }

    #[doc = "EVRC SD Coefficient Register 8\n resetvalue={LVD Reset:0x080171002,Cold PORST:0x080171002,After SSW execution:0x080171002}"]
    #[inline(always)]
    pub const fn evrsdcoeff8(
        &self,
    ) -> crate::common::Reg<self::Evrsdcoeff8_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(360usize)) }
    }

    #[doc = "EVRC SD Coefficient Register 9\n resetvalue={LVD Reset:0x08000A0AF,Cold PORST:0x08000A0AF,After SSW execution:0x08000A0AF}"]
    #[inline(always)]
    pub const fn evrsdcoeff9(
        &self,
    ) -> crate::common::Reg<self::Evrsdcoeff9_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(364usize)) }
    }

    #[doc = "EVRC SD Control Register 0\n resetvalue={LVD Reset:0x0F0390001,Cold PORST:0x0F0390001,After SSW execution:0x0F0390001}"]
    #[inline(always)]
    pub const fn evrsdctrl0(&self) -> crate::common::Reg<self::Evrsdctrl0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(264usize)) }
    }

    #[doc = "EVRC SD Control Register 1\n resetvalue={LVD Reset:0x086690708,Cold PORST:0x086690708,After SSW execution:0x086690708}"]
    #[inline(always)]
    pub const fn evrsdctrl1(&self) -> crate::common::Reg<self::Evrsdctrl1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(268usize)) }
    }

    #[doc = "EVRC SD Control Register 10\n resetvalue={LVD Reset:0x5A82,Cold PORST:0x5A82,After SSW execution:0x5A82}"]
    #[inline(always)]
    pub const fn evrsdctrl10(
        &self,
    ) -> crate::common::Reg<self::Evrsdctrl10_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(304usize)) }
    }

    #[doc = "EVRC SD Control Register 11\n resetvalue={LVD Reset:0x092070909,Cold PORST:0x092070909,After SSW execution:0x092070909}"]
    #[inline(always)]
    pub const fn evrsdctrl11(
        &self,
    ) -> crate::common::Reg<self::Evrsdctrl11_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(308usize)) }
    }

    #[doc = "EVRC SD Control Register 2\n resetvalue={LVD Reset:0x36033B,Cold PORST:0x36033B,After SSW execution:0x36033B}"]
    #[inline(always)]
    pub const fn evrsdctrl2(&self) -> crate::common::Reg<self::Evrsdctrl2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(272usize)) }
    }

    #[doc = "EVRC SD Control Register 3\n resetvalue={LVD Reset:0x0B690810,Cold PORST:0x0B690810,After SSW execution:0x0B690810}"]
    #[inline(always)]
    pub const fn evrsdctrl3(&self) -> crate::common::Reg<self::Evrsdctrl3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(276usize)) }
    }

    #[doc = "EVRC SD Control Register 4\n resetvalue={LVD Reset:0x360009,Cold PORST:0x360009,After SSW execution:0x360009}"]
    #[inline(always)]
    pub const fn evrsdctrl4(&self) -> crate::common::Reg<self::Evrsdctrl4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(280usize)) }
    }

    #[doc = "EVRC SD Control Register 5\n resetvalue={LVD Reset:0x0B690808,Cold PORST:0x0B690808,After SSW execution:0x0B690808}"]
    #[inline(always)]
    pub const fn evrsdctrl5(&self) -> crate::common::Reg<self::Evrsdctrl5_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(284usize)) }
    }

    #[doc = "EVRC SD Control Register 6\n resetvalue={LVD Reset:0x080231C94,Cold PORST:0x080231C94,After SSW execution:0x080231C94}"]
    #[inline(always)]
    pub const fn evrsdctrl6(&self) -> crate::common::Reg<self::Evrsdctrl6_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(288usize)) }
    }

    #[doc = "EVRC SD Control Register 7\n resetvalue={LVD Reset:0x0800000FE,Cold PORST:0x0800000FE,After SSW execution:0x0800000FE}"]
    #[inline(always)]
    pub const fn evrsdctrl7(&self) -> crate::common::Reg<self::Evrsdctrl7_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(292usize)) }
    }

    #[doc = "EVRC SD Control Register 8\n resetvalue={LVD Reset:0x09121048E,Cold PORST:0x09121048E,After SSW execution:0x09121048E}"]
    #[inline(always)]
    pub const fn evrsdctrl8(&self) -> crate::common::Reg<self::Evrsdctrl8_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(296usize)) }
    }

    #[doc = "EVRC SD Control Register 9\n resetvalue={LVD Reset:0x080000434,Cold PORST:0x080000434,After SSW execution:0x080000434}"]
    #[inline(always)]
    pub const fn evrsdctrl9(&self) -> crate::common::Reg<self::Evrsdctrl9_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(300usize)) }
    }

    #[doc = "EVR SD Status Register 0\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
    #[inline(always)]
    pub const fn evrsdstat0(&self) -> crate::common::Reg<self::Evrsdstat0_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(252usize)) }
    }

    #[doc = "EVR Status Register\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
    #[inline(always)]
    pub const fn evrstat(&self) -> crate::common::Reg<self::Evrstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }

    #[doc = "EVR Trim Control Register\n resetvalue={LVD Reset:0x080006C9E,Cold PORST:0x080006C9E,After SSW execution:0x6C9E}"]
    #[inline(always)]
    pub const fn evrtrim(&self) -> crate::common::Reg<self::Evrtrim_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(76usize)) }
    }

    #[doc = "EVR Trim Control Register 2\n resetvalue={LVD Reset:0x2C0C2FF,Cold PORST:0x2C0C2FF,After SSW execution:0x2C0C2FF}"]
    #[inline(always)]
    pub const fn evrtrim2(&self) -> crate::common::Reg<self::Evrtrim2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(84usize)) }
    }

    #[doc = "EVR Trim Status Register\n resetvalue={LVD Reset:0x6C9E,Cold PORST:0x6C9E}"]
    #[inline(always)]
    pub const fn evrtrimstat(
        &self,
    ) -> crate::common::Reg<self::Evrtrimstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }

    #[doc = "EVR Secondary Under voltage Monitor Register\n resetvalue={LVD Reset:0x75A7B8,Cold PORST:0x75A7B8,After SSW execution:0x75A7B8}"]
    #[inline(always)]
    pub const fn evruvmon(&self) -> crate::common::Reg<self::Evruvmon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(120usize)) }
    }

    #[doc = "EVR Secondary Under voltage Monitor Register 2\n resetvalue={LVD Reset:0x2A7000BC,Cold PORST:0x2A7000BC,After SSW execution:0x2A7000BC}"]
    #[inline(always)]
    pub const fn evruvmon2(&self) -> crate::common::Reg<self::Evruvmon2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "EVR Primary HSM Over voltage Monitor Register\n resetvalue={LVD Reset:0x0E1B586,Cold PORST:0x0E1B586,After SSW execution:0x0E1B586}"]
    #[inline(always)]
    pub const fn hsmovmon(&self) -> crate::common::Reg<self::Hsmovmon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(140usize)) }
    }

    #[doc = "EVR Primary HSM Under voltage Monitor Register\n resetvalue={LVD Reset:0x5C824D,Cold PORST:0x5C824D,After SSW execution:0x5C824D}"]
    #[inline(always)]
    pub const fn hsmuvmon(&self) -> crate::common::Reg<self::Hsmuvmon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }

    #[doc = "Identification Register\n resetvalue={Application Reset:0x0E8C001}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "SMU stdby BIST Control Register\n resetvalue={LVD Reset:0x0,Warm PORST:0x0}"]
    #[inline(always)]
    pub const fn monbistctrl(
        &self,
    ) -> crate::common::Reg<self::Monbistctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(408usize)) }
    }

    #[doc = "SMU stdby BIST Status Register\n resetvalue={LVD Reset:0x0,Warm PORST:0x0}"]
    #[inline(always)]
    pub const fn monbiststat(
        &self,
    ) -> crate::common::Reg<self::Monbiststat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(400usize)) }
    }

    #[doc = "OCDS Trigger Set Control 0 Register\n resetvalue={LVD Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn otsc0(&self) -> crate::common::Reg<self::Otsc0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(484usize)) }
    }

    #[doc = "OCDS Trigger Set Control 1 Register\n resetvalue={LVD Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn otsc1(&self) -> crate::common::Reg<self::Otsc1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(488usize)) }
    }

    #[doc = "OCDS Trigger Set Select Register\n resetvalue={LVD Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn otss(&self) -> crate::common::Reg<self::Otss_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(480usize)) }
    }

    #[doc = "PMS Interrupt Enable Register\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
    #[inline(always)]
    pub const fn pmsien(&self) -> crate::common::Reg<self::Pmsien_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(116usize)) }
    }

    #[doc = "Standby and Wake up Control Register 0\n resetvalue={LVD Reset:0x1002D0}"]
    #[inline(always)]
    pub const fn pmswcr0(&self) -> crate::common::Reg<self::Pmswcr0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(180usize)) }
    }

    #[doc = "Standby and Wake up Control Register 2\n resetvalue={LVD Reset:0x4000000}"]
    #[inline(always)]
    pub const fn pmswcr2(&self) -> crate::common::Reg<self::Pmswcr2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(184usize)) }
    }

    #[doc = "Standby and Wake up Control Register 3\n resetvalue={LVD Reset:0x0}"]
    #[inline(always)]
    pub const fn pmswcr3(&self) -> crate::common::Reg<self::Pmswcr3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(192usize)) }
    }

    #[doc = "Standby and Wake up Control Register 4\n resetvalue={LVD Reset:0x20}"]
    #[inline(always)]
    pub const fn pmswcr4(&self) -> crate::common::Reg<self::Pmswcr4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(196usize)) }
    }

    #[doc = "Standby and Wake up Control Register 5\n resetvalue={LVD Reset:0x0}"]
    #[inline(always)]
    pub const fn pmswcr5(&self) -> crate::common::Reg<self::Pmswcr5_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(200usize)) }
    }

    #[doc = "Standby and Wake up Status Register\n resetvalue={LVD Reset:0x0A0000}"]
    #[inline(always)]
    pub const fn pmswstat(&self) -> crate::common::Reg<self::Pmswstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(212usize)) }
    }

    #[doc = "Standby and Wake up Status Register 2\n resetvalue={LVD Reset:0x100000}"]
    #[inline(always)]
    pub const fn pmswstat2(&self) -> crate::common::Reg<self::Pmswstat2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(216usize)) }
    }

    #[doc = "Standby and Wake up Status Clear Register\n resetvalue={LVD Reset:0x0}"]
    #[inline(always)]
    pub const fn pmswstatclr(
        &self,
    ) -> crate::common::Reg<self::Pmswstatclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(232usize)) }
    }

    #[doc = "Standby WUT Counter Register\n resetvalue={LVD Reset:0x0}"]
    #[inline(always)]
    pub const fn pmswutcnt(&self) -> crate::common::Reg<self::Pmswutcnt_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(220usize)) }
    }

    #[doc = "Test Control Register 0\n resetvalue={LVD Reset:0x0}"]
    #[inline(always)]
    pub const fn tstctrl0(&self) -> crate::common::Reg<self::Tstctrl0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(496usize)) }
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
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, accen0::En0, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,accen0::En0, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, accen0::En1, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,accen0::En1, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, accen0::En2, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,accen0::En2, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, accen0::En3, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,accen0::En3, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, accen0::En4, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,accen0::En4, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, accen0::En5, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,accen0::En5, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, accen0::En6, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,accen0::En6, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, accen0::En7, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,accen0::En7, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, accen0::En8, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,accen0::En8, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, accen0::En9, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,accen0::En9, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, accen0::En10, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,accen0::En10, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, accen0::En11, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,accen0::En11, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, accen0::En12, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,accen0::En12, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, accen0::En13, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,accen0::En13, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, accen0::En14, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,accen0::En14, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, accen0::En15, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,accen0::En15, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, accen0::En16, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,accen0::En16, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, accen0::En17, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,accen0::En17, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, accen0::En18, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,accen0::En18, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, accen0::En19, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,accen0::En19, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, accen0::En20, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,accen0::En20, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, accen0::En21, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,accen0::En21, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, accen0::En22, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,accen0::En22, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, accen0::En23, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,accen0::En23, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, accen0::En24, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,accen0::En24, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, accen0::En25, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,accen0::En25, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, accen0::En26, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,accen0::En26, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, accen0::En27, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,accen0::En27, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, accen0::En28, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,accen0::En28, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, accen0::En29, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,accen0::En29, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, accen0::En30, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,accen0::En30, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the PMS kernel addresses for        transactions with the Master TAG ID n"]
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
pub struct Ag2IFspStdby_SPEC;
impl crate::sealed::RegSpec for Ag2IFspStdby_SPEC {
    type DataType = u32;
}
#[doc = "SMU stdby FSP Configuration Register\n resetvalue={LVD Reset:0x0,Warm PORST:0x0}"]
pub type Ag2IFspStdby = crate::RegValueT<Ag2IFspStdby_SPEC>;

impl Ag2IFspStdby {
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ag2ifsp_stdby::Fe4,
        Ag2IFspStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ag2ifsp_stdby::Fe4,
            Ag2IFspStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ag2ifsp_stdby::Fe5,
        Ag2IFspStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ag2ifsp_stdby::Fe5,
            Ag2IFspStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ag2ifsp_stdby::Fe6,
        Ag2IFspStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ag2ifsp_stdby::Fe6,
            Ag2IFspStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ag2ifsp_stdby::Fe7,
        Ag2IFspStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ag2ifsp_stdby::Fe7,
            Ag2IFspStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ag2ifsp_stdby::Fe8,
        Ag2IFspStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ag2ifsp_stdby::Fe8,
            Ag2IFspStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ag2ifsp_stdby::Fe9,
        Ag2IFspStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ag2ifsp_stdby::Fe9,
            Ag2IFspStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        ag2ifsp_stdby::Fe10,
        Ag2IFspStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            ag2ifsp_stdby::Fe10,
            Ag2IFspStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        ag2ifsp_stdby::Fe11,
        Ag2IFspStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            ag2ifsp_stdby::Fe11,
            Ag2IFspStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ag2ifsp_stdby::Fe12,
        Ag2IFspStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ag2ifsp_stdby::Fe12,
            Ag2IFspStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        ag2ifsp_stdby::Fe13,
        Ag2IFspStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            ag2ifsp_stdby::Fe13,
            Ag2IFspStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        ag2ifsp_stdby::Fe14,
        Ag2IFspStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            ag2ifsp_stdby::Fe14,
            Ag2IFspStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Fault signaling configuration flag for alarm 16 belonging to alarm group i.   FE16"]
    #[inline(always)]
    pub fn fe15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        ag2ifsp_stdby::Fe15,
        Ag2IFspStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            ag2ifsp_stdby::Fe15,
            Ag2IFspStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "AG2iFSP STDBY register bits protection. Setting this bit enables that bits FE z  can be changed in this write        operation. This bit is read as zero."]
    #[inline(always)]
    pub fn bitprot(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Ag2IFspStdby_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30,1,0,Ag2IFspStdby_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Ag2IFspStdby {
    #[inline(always)]
    fn default() -> Ag2IFspStdby {
        <crate::RegValueT<Ag2IFspStdby_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ag2ifsp_stdby {
    pub struct Fe4_SPEC;
    pub type Fe4 = crate::EnumBitfieldStruct<u8, Fe4_SPEC>;
    impl Fe4 {
        #[doc = "0 FSP        disabled for this alarm event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP        enabled for this alarm event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe5_SPEC;
    pub type Fe5 = crate::EnumBitfieldStruct<u8, Fe5_SPEC>;
    impl Fe5 {
        #[doc = "0 FSP        disabled for this alarm event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP        enabled for this alarm event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe6_SPEC;
    pub type Fe6 = crate::EnumBitfieldStruct<u8, Fe6_SPEC>;
    impl Fe6 {
        #[doc = "0 FSP        disabled for this alarm event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP        enabled for this alarm event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe7_SPEC;
    pub type Fe7 = crate::EnumBitfieldStruct<u8, Fe7_SPEC>;
    impl Fe7 {
        #[doc = "0 FSP        disabled for this alarm event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP        enabled for this alarm event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe8_SPEC;
    pub type Fe8 = crate::EnumBitfieldStruct<u8, Fe8_SPEC>;
    impl Fe8 {
        #[doc = "0 FSP        disabled for this alarm event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP        enabled for this alarm event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe9_SPEC;
    pub type Fe9 = crate::EnumBitfieldStruct<u8, Fe9_SPEC>;
    impl Fe9 {
        #[doc = "0 FSP        disabled for this alarm event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP        enabled for this alarm event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe10_SPEC;
    pub type Fe10 = crate::EnumBitfieldStruct<u8, Fe10_SPEC>;
    impl Fe10 {
        #[doc = "0 FSP        disabled for this alarm event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP        enabled for this alarm event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe11_SPEC;
    pub type Fe11 = crate::EnumBitfieldStruct<u8, Fe11_SPEC>;
    impl Fe11 {
        #[doc = "0 FSP        disabled for this alarm event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP        enabled for this alarm event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe12_SPEC;
    pub type Fe12 = crate::EnumBitfieldStruct<u8, Fe12_SPEC>;
    impl Fe12 {
        #[doc = "0 FSP        disabled for this alarm event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP        enabled for this alarm event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe13_SPEC;
    pub type Fe13 = crate::EnumBitfieldStruct<u8, Fe13_SPEC>;
    impl Fe13 {
        #[doc = "0 FSP        disabled for this alarm event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP        enabled for this alarm event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe14_SPEC;
    pub type Fe14 = crate::EnumBitfieldStruct<u8, Fe14_SPEC>;
    impl Fe14 {
        #[doc = "0 FSP        disabled for this alarm event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP        enabled for this alarm event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe15_SPEC;
    pub type Fe15 = crate::EnumBitfieldStruct<u8, Fe15_SPEC>;
    impl Fe15 {
        #[doc = "0 FSP        disabled for this alarm event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FSP        enabled for this alarm event."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ag2IStdby_SPEC;
impl crate::sealed::RegSpec for Ag2IStdby_SPEC {
    type DataType = u32;
}
#[doc = "Alarm Status Register\n resetvalue={LVD Reset:0x0}"]
pub type Ag2IStdby = crate::RegValueT<Ag2IStdby_SPEC>;

impl Ag2IStdby {
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        ag2i_stdby::Sf4,
        Ag2IStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            ag2i_stdby::Sf4,
            Ag2IStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        ag2i_stdby::Sf5,
        Ag2IStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            ag2i_stdby::Sf5,
            Ag2IStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf6(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        ag2i_stdby::Sf6,
        Ag2IStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            ag2i_stdby::Sf6,
            Ag2IStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf7(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        ag2i_stdby::Sf7,
        Ag2IStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            ag2i_stdby::Sf7,
            Ag2IStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf8(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        ag2i_stdby::Sf8,
        Ag2IStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            ag2i_stdby::Sf8,
            Ag2IStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf9(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        ag2i_stdby::Sf9,
        Ag2IStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            ag2i_stdby::Sf9,
            Ag2IStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        ag2i_stdby::Sf10,
        Ag2IStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            ag2i_stdby::Sf10,
            Ag2IStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        ag2i_stdby::Sf11,
        Ag2IStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            ag2i_stdby::Sf11,
            Ag2IStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        ag2i_stdby::Sf12,
        Ag2IStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            ag2i_stdby::Sf12,
            Ag2IStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        ag2i_stdby::Sf13,
        Ag2IStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            ag2i_stdby::Sf13,
            Ag2IStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        ag2i_stdby::Sf14,
        Ag2IStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            ag2i_stdby::Sf14,
            Ag2IStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Status flag for alarm 16 belonging to alarm group i.   SF16"]
    #[inline(always)]
    pub fn sf15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        ag2i_stdby::Sf15,
        Ag2IStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            ag2i_stdby::Sf15,
            Ag2IStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Error Pin Fault State Status Bit   FSPERR. The bit indicates that Error pin was set into fault state by the SMU stdby. Reset by setting to 0. If the Error Pins were set in fault state by the SMU stdby  reseting this bit sets the Error Pins back in fault free state"]
    #[inline(always)]
    pub fn fsperr(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        ag2i_stdby::Fsperr,
        Ag2IStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            ag2i_stdby::Fsperr,
            Ag2IStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Ag2IStdby {
    #[inline(always)]
    fn default() -> Ag2IStdby {
        <crate::RegValueT<Ag2IStdby_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ag2i_stdby {
    pub struct Sf4_SPEC;
    pub type Sf4 = crate::EnumBitfieldStruct<u8, Sf4_SPEC>;
    impl Sf4 {
        #[doc = "Status flag z does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Status flag z reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf5_SPEC;
    pub type Sf5 = crate::EnumBitfieldStruct<u8, Sf5_SPEC>;
    impl Sf5 {
        #[doc = "Status flag z does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Status flag z reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf6_SPEC;
    pub type Sf6 = crate::EnumBitfieldStruct<u8, Sf6_SPEC>;
    impl Sf6 {
        #[doc = "Status flag z does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Status flag z reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf7_SPEC;
    pub type Sf7 = crate::EnumBitfieldStruct<u8, Sf7_SPEC>;
    impl Sf7 {
        #[doc = "Status flag z does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Status flag z reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf8_SPEC;
    pub type Sf8 = crate::EnumBitfieldStruct<u8, Sf8_SPEC>;
    impl Sf8 {
        #[doc = "Status flag z does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Status flag z reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf9_SPEC;
    pub type Sf9 = crate::EnumBitfieldStruct<u8, Sf9_SPEC>;
    impl Sf9 {
        #[doc = "Status flag z does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Status flag z reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf10_SPEC;
    pub type Sf10 = crate::EnumBitfieldStruct<u8, Sf10_SPEC>;
    impl Sf10 {
        #[doc = "Status flag z does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Status flag z reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf11_SPEC;
    pub type Sf11 = crate::EnumBitfieldStruct<u8, Sf11_SPEC>;
    impl Sf11 {
        #[doc = "Status flag z does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Status flag z reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf12_SPEC;
    pub type Sf12 = crate::EnumBitfieldStruct<u8, Sf12_SPEC>;
    impl Sf12 {
        #[doc = "Status flag z does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Status flag z reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf13_SPEC;
    pub type Sf13 = crate::EnumBitfieldStruct<u8, Sf13_SPEC>;
    impl Sf13 {
        #[doc = "Status flag z does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Status flag z reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf14_SPEC;
    pub type Sf14 = crate::EnumBitfieldStruct<u8, Sf14_SPEC>;
    impl Sf14 {
        #[doc = "Status flag z does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Status flag z reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sf15_SPEC;
    pub type Sf15 = crate::EnumBitfieldStruct<u8, Sf15_SPEC>;
    impl Sf15 {
        #[doc = "Status flag z does not report a fault condition"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Status flag z reports a fault condition"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fsperr_SPEC;
    pub type Fsperr = crate::EnumBitfieldStruct<u8, Fsperr_SPEC>;
    impl Fsperr {
        #[doc = "Error pin was not set into fault state"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The Error pin        was set into fault state."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmdStdby_SPEC;
impl crate::sealed::RegSpec for CmdStdby_SPEC {
    type DataType = u32;
}
#[doc = "SMU stdby Command Register\n resetvalue={LVD Reset:0x0,Warm PORST:0x0}"]
pub type CmdStdby = crate::RegValueT<CmdStdby_SPEC>;

impl CmdStdby {
    #[doc = "SMU stdby Module Enable   SMUEN. This bit enables SMU stdby to issue a FSP reaction when an alarm is received. Also  SMUEN needs to be set to enter the SMU stdby BIST mode."]
    #[inline(always)]
    pub fn smuen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        cmd_stdby::Smuen,
        CmdStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            cmd_stdby::Smuen,
            CmdStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "SMU stdby FSP0 Error pin enable   FSP0EN. This bit enables SMU stdby Error pin function to be able set P33.8 to        fault state."]
    #[inline(always)]
    pub fn fsp0en(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        cmd_stdby::Fsp0En,
        CmdStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            cmd_stdby::Fsp0En,
            CmdStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "SMU stdby FSP1 Error pin enable   FSP1EN. This bit enables SMU stdby Error pin function to be able set P33.10 to        fault state."]
    #[inline(always)]
    pub fn fsp1en(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        cmd_stdby::Fsp1En,
        CmdStdby_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            cmd_stdby::Fsp1En,
            CmdStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "SMU stdby alarm status clear enable   ASCE. This bit controls if a status flag set in an AGx register upon detection        of the alarm event can be cleared by software or not. When ASCE is        enabled  software shall write a 1 to bit position in AGx to clear the        bit  W1C . When a W1C action takes place the ASCE bit is automatically        cleared to 0 by hardware and software shall set the ASCE bit again."]
    #[inline(always)]
    pub fn asce(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, cmd_stdby::Asce, CmdStdby_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            cmd_stdby::Asce,
            CmdStdby_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "CMD STDBY register bits protection. Setting this bit enables that bits SMUEN  FSP0EN  FSP1EN or and ASCE can        be changed in this write operation. This bit is read as zero."]
    #[inline(always)]
    pub fn bitprot(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, CmdStdby_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, CmdStdby_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for CmdStdby {
    #[inline(always)]
    fn default() -> CmdStdby {
        <crate::RegValueT<CmdStdby_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cmd_stdby {
    pub struct Smuen_SPEC;
    pub type Smuen = crate::EnumBitfieldStruct<u8, Smuen_SPEC>;
    impl Smuen {
        #[doc = "0 SMU stdby        disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SMU stdby        enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fsp0En_SPEC;
    pub type Fsp0En = crate::EnumBitfieldStruct<u8, Fsp0En_SPEC>;
    impl Fsp0En {
        #[doc = "0 SMU stdby Error Pin fault indication function on P33.8 inactive."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SMU stdby Error Pin fault indication function on P33.8 active."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fsp1En_SPEC;
    pub type Fsp1En = crate::EnumBitfieldStruct<u8, Fsp1En_SPEC>;
    impl Fsp1En {
        #[doc = "0 SMU stdby Error Pin fault indication function on P33.10 inactive."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SMU stdby Error Pin fault indication function on P33.10 active."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Asce_SPEC;
    pub type Asce = crate::EnumBitfieldStruct<u8, Asce_SPEC>;
    impl Asce {
        #[doc = "0 SMU stdby alarm        status bits in AG2i STDBY cannot be cleared."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SMU stdby alarm          status bits in AG2i STDBY can be cleared"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtscon_SPEC;
impl crate::sealed::RegSpec for Dtscon_SPEC {
    type DataType = u32;
}
#[doc = "Die Temperature Sensor Control Register\n resetvalue={LVD Reset:0x200,Cold PORST:0x200,After SSW execution:0x200}"]
pub type Dtscon = crate::RegValueT<Dtscon_SPEC>;

impl Dtscon {
    #[doc = "DTS Gain Trim Value  gtrim . This bit field contains information about gain trimming of the die        temperature sensor. GAINTRIM influences the length of integration time        where DTS pulses are counted. A settling time of x  160 us is required after        update."]
    #[inline(always)]
    pub fn gaintrim(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Dtscon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Dtscon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DTS Offset Trim Value  otrim . This bit field contains information about trimming of the die        temperature sensor. DTSRESULT   DTSRESULTANA   OFFSETTRIM"]
    #[inline(always)]
    pub fn offsettrim(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, Dtscon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xfff,1,0,u16, Dtscon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, dtscon::Slck, Dtscon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,dtscon::Slck, Dtscon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dtscon {
    #[inline(always)]
    fn default() -> Dtscon {
        <crate::RegValueT<Dtscon_SPEC> as RegisterValue<_>>::new(512)
    }
}
pub mod dtscon {
    pub struct Slck_SPEC;
    pub type Slck = crate::EnumBitfieldStruct<u8, Slck_SPEC>;
    impl Slck {
        #[doc = "0 No lock active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Lock is active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dtslim_SPEC;
impl crate::sealed::RegSpec for Dtslim_SPEC {
    type DataType = u32;
}
#[doc = "Die Temperature Sensor Limit Register\n resetvalue={LVD Reset:0x0CD806D6,Cold PORST:0x0CD806D6}"]
pub type Dtslim = crate::RegValueT<Dtslim_SPEC>;

impl Dtslim {
    #[doc = "Lower Limit   LOWER. This bit field defines the lower limit of the DTS temperature check. The        DTS measurement result is compared against this value and if the        measurement result is less than or equal to the configured LOWER        bitfield value  flag LLU is set."]
    #[inline(always)]
    pub fn lower(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, Dtslim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16, Dtslim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lower Limit Underflow   LLU. When this bit is set  a HSM temperature underflow trigger is generated.        When this bit is set the related SMU DTS alarm trigger is generated.        This bit has to be written with zero in order to clear it. Writing a one        has no effect. This bit is set when a DTS measurement is finished and        the result is below the lower limit  i.e. DTSLIM.LOWER ."]
    #[inline(always)]
    pub fn llu(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, dtslim::Llu, Dtslim_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,dtslim::Llu, Dtslim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Upper Limit   UPPER. This bit field defines the upper limit of the DTS temperature check. The        DTS measurement result is compared against this value and if the        measurement result is greater than or equal to the configured UPPER        bitfield value  flag UOF is set."]
    #[inline(always)]
    pub fn upper(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, Dtslim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xfff,1,0,u16, Dtslim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, dtslim::Slck, Dtslim_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,dtslim::Slck, Dtslim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Upper Limit Overflow   UOF. When this bit is set  a HSM temperature overflow trigger is generated. When this bit is set  the related SMU DTS alarm trigger is generated. This bit has to be written with zero in order to clear it. Writing a one        has no effect. This bit is set when a DTS measurement is finished and the result is        exceeding the upper limit  i.e. DTSLIM.UPPER ."]
    #[inline(always)]
    pub fn uof(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, dtslim::Uof, Dtslim_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,dtslim::Uof, Dtslim_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dtslim {
    #[inline(always)]
    fn default() -> Dtslim {
        <crate::RegValueT<Dtslim_SPEC> as RegisterValue<_>>::new(215484118)
    }
}
pub mod dtslim {
    pub struct Llu_SPEC;
    pub type Llu = crate::EnumBitfieldStruct<u8, Llu_SPEC>;
    impl Llu {
        #[doc = "0 No temperature        underflow was detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A temperature        underflow was detected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Slck_SPEC;
    pub type Slck = crate::EnumBitfieldStruct<u8, Slck_SPEC>;
    impl Slck {
        #[doc = "0 No lock active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Lock is active"]
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
pub struct Dtsstat_SPEC;
impl crate::sealed::RegSpec for Dtsstat_SPEC {
    type DataType = u32;
}
#[doc = "Die Temperature Sensor Status Register\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
pub type Dtsstat = crate::RegValueT<Dtsstat_SPEC>;

impl Dtsstat {
    #[doc = "Result of the DTS Measurement   RESULT. This bit field shows the result of the DTS measurement. The value given        is directly related to the die temperature and can be evaluated using        the following formula. T    176 C     RESULT   Gnom    273.15 T   176 K   RESULT  G nom RESULT  G nom    T    176 C    273.15    G nom   T    176 K  G nom   7.505"]
    #[inline(always)]
    pub fn result(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, Dtsstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xfff,1,0,u16, Dtsstat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Dtsstat {
    #[inline(always)]
    fn default() -> Dtsstat {
        <crate::RegValueT<Dtsstat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evr33Con_SPEC;
impl crate::sealed::RegSpec for Evr33Con_SPEC {
    type DataType = u32;
}
#[doc = "EVR33 Control Register\n resetvalue={LVD Reset:0x407ED,Cold PORST:0x407ED,After SSW execution:0x407ED}"]
pub type Evr33Con = crate::RegValueT<Evr33Con_SPEC>;

impl Evr33Con {
    #[doc = "Short to Supply Voltage Threshold x i    SHVH33. This field defines the upper threshold level VDDP3 supply. EVRC short to        supply alarm has the nominal values of SHVH33   4.5V and t33SHHV   3ms. SHVH33     VDDx   937.5 mV    LSB  LSB   15 mV"]
    #[inline(always)]
    pub fn shvh33(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evr33Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evr33Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "KD Value   KD. This bit field defines KD value of EVR33 regulator with internal pass        devices. Bit Coding    lt exponent.1 gt  lt exponent.0 gt  lt mantissa.1 gt  lt mantissa.0 gt  Differential PID component  160    160    160 4  160    160 mantissa  160    160    160    160 2  160    160    160 exponent  160    160 2  160    160"]
    #[inline(always)]
    pub fn kd(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Evr33Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Evr33Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Short to High Detection Enable   SHHVEN"]
    #[inline(always)]
    pub fn shhven(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        evr33con::Shhven,
        Evr33Con_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            evr33con::Shhven,
            Evr33Con_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Short to Low Detection Enable   SHLVEN"]
    #[inline(always)]
    pub fn shlven(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        evr33con::Shlven,
        Evr33Con_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            evr33con::Shlven,
            Evr33Con_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "3.3 V Voltage Primary ADC Counter LPF Coefficient for EVR33   EVR33LPF. This bitfield configures the coefficient of the Low Pass Filter of the        VDDP3  160    160 EVR33 Primary Monitor ADC counter value for EVR33. y  160  k   160    160    160 y  160  k 1   160    160  1 a   160    160    160    160 x  160  k   160    160 a   160   y  160  k  is filter output         x  160  k  is ADC output a  160    160  1  160    160  2  160    160 LPF  .If LPF  160    160 0  the filter output is the same as ADC        output."]
    #[inline(always)]
    pub fn evr33lpf(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Evr33Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Evr33Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Short to Ground Voltage Threshold x i    SHVL33. This field defines the lower threshold level VDDP3 supply. EVR33 short        to ground alarm has the nominal values of SHVL33   1V and t33SHLV   3ms. SHVL33     VDDx   937.5 mV    LSB  LSB   15 mV"]
    #[inline(always)]
    pub fn shvl33(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evr33Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evr33Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "KP Value   KP. This bit field defines KP value of EVR33 regulator with internal pass        devices. Bit Coding    lt exponent.1 gt  lt exponent.0 gt  lt mantissa.1 gt  lt mantissa.0 gt  Proportional PID component  160    160    160 4  160    160 mantissa  160    160    160    160 2  160    160    160 exponent  160    160 4  160    160"]
    #[inline(always)]
    pub fn kp(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Evr33Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Evr33Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "EVR33 Regulator Enable   EVR33OFF. This bit can only be changed if bit BPEVR33OFF is set in parallel. EVR33        regulator can only be disabled via EVR33OFF and cannot be consequently        enabled after disabling. EVR33OFF is intended to be used only for        internal test purposes and is not used in customer application."]
    #[inline(always)]
    pub fn evr33off(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        evr33con::Evr33Off,
        Evr33Con_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            evr33con::Evr33Off,
            Evr33Con_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Bit Protection EVR33OFF   BPEVR33OFF. Setting this bit enables that bit EVR33OFF can be changed in this write        operation. This bit is read as zero."]
    #[inline(always)]
    pub fn bpevr33off(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Evr33Con_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, Evr33Con_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, evr33con::Slck, Evr33Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            evr33con::Slck,
            Evr33Con_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evr33Con {
    #[inline(always)]
    fn default() -> Evr33Con {
        <crate::RegValueT<Evr33Con_SPEC> as RegisterValue<_>>::new(264173)
    }
}
pub mod evr33con {
    pub struct Shhven_SPEC;
    pub type Shhven = crate::EnumBitfieldStruct<u8, Shhven_SPEC>;
    impl Shhven {
        #[doc = "0 Short to High Detection is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Short to High Detection is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Shlven_SPEC;
    pub type Shlven = crate::EnumBitfieldStruct<u8, Shlven_SPEC>;
    impl Shlven {
        #[doc = "0 Short to Low Detection is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Short to Low Detection is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Evr33Off_SPEC;
    pub type Evr33Off = crate::EnumBitfieldStruct<u8, Evr33Off_SPEC>;
    impl Evr33Off {
        #[doc = "0 The EVR33        regulator module is enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The EVR33        regulator module is disabled  switched        off. The device may be operating in standby or with external 3.3V        supply."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Slck_SPEC;
    pub type Slck = crate::EnumBitfieldStruct<u8, Slck_SPEC>;
    impl Slck {
        #[doc = "0 No lock active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Lock is active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evradcstat_SPEC;
impl crate::sealed::RegSpec for Evradcstat_SPEC {
    type DataType = u32;
}
#[doc = "EVR Primary ADC Status Register\n resetvalue={LVD Reset:0x0}"]
pub type Evradcstat = crate::RegValueT<Evradcstat_SPEC>;

impl Evradcstat {
    #[doc = "ADC VDD Core Voltage Conversion Result   ADCCV. This bit field contains the last filtered conversion result of the ADC        measurement of the VDD  160    160 EVRC supply by the Primary Monitor. The        tracking ADC offset trim value to reach target offset is defined in        EVRTRIM2.ADCCOFFS  signed value .The value is updated every cycle   40ns. VIN  160    160  0.7125    ADCCV   EVRTRIM2.ADCCOFFS    LSB   160 V LSB   5  160 mV Eg. 1.25 V   6C"]
    #[inline(always)]
    pub fn adccv(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evradcstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evradcstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ADC VDDP3 Voltage Conversion Result   ADC33V. This bit field contains the last filtered conversion result of the ADC        measurement of the VDDP3  160    160 EVR33 supply by the Primary Monitor. The        tracking ADC offset trim value to reach target offset is defined in        EVRTRIM2.ADC33OFFS  signed value . VIN  160    160  0.9375    ADC33V   EVRTRIM2.ADC33OFFS    LSB   160 V LSB   15  160 mV Eg. 3.3 V   9E"]
    #[inline(always)]
    pub fn adc33v(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evradcstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evradcstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ADC VEXT Supply Conversion Result   ADCSWDV. This bit field contains the last filtered conversion result of the ADC        measurement of the external VEXT  3.3V  160    160 5V  supply by the Primary        Monitor. The tracking ADC        offset trim value to reach target offset is defined in        EVRTRIM.ADCSWDOFFS  signed value . VIN  160    160  1.050    ADCSWDV   EVRTRIM2.ADCSWDOFFS    LSB   160 V  160  LSB   20  160 mV Eg. 5 V   C6"]
    #[inline(always)]
    pub fn adcswdv(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evradcstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evradcstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EVRC Regulator or VDD Over voltage event flag   OVC. This bit is set if VDD primary voltage monitor recognizes a over voltage        event. An alarm is raised to the HSM and SMU."]
    #[inline(always)]
    pub fn ovc(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        evradcstat::Ovc,
        Evradcstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            evradcstat::Ovc,
            Evradcstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "EVR33 Regulator or VDDP3 Over voltage event flag   OV33. This bit is set if VDDP3 primary voltage monitor recognizes a        over voltage event. An alarm is raised to the HSM and SMU."]
    #[inline(always)]
    pub fn ov33(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        evradcstat::Ov33,
        Evradcstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            evradcstat::Ov33,
            Evradcstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Supply Watchdog  SWD  or VEXT Over voltage event flag   OVSWD. This bit is set if VEXT primary voltage monitor recognizes an        over voltage event. An alarm is raised to the HSM and SMU."]
    #[inline(always)]
    pub fn ovswd(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        evradcstat::Ovswd,
        Evradcstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            evradcstat::Ovswd,
            Evradcstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "EVRC Regulator or VDD Under voltage event flag   UVC. This bit is set if VDD primary voltage monitor recognizes a        under voltage event. An alarm is raised to the HSM and SMU."]
    #[inline(always)]
    pub fn uvc(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        evradcstat::Uvc,
        Evradcstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            evradcstat::Uvc,
            Evradcstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "EVR33 Regulator or VDDP3 Under voltage event flag   UV33. This bit is set if VDDP3 primary voltage monitor recognizes a        under voltage event. An alarm is raised to the HSM and SMU."]
    #[inline(always)]
    pub fn uv33(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        evradcstat::Uv33,
        Evradcstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            evradcstat::Uv33,
            Evradcstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Supply Watchdog  SWD  or VEXT Under voltage event flag   UVSWD. This bit is set if VEXT primary voltage monitor recognizes an        under voltage event. An alarm is raised to the HSM and SMU."]
    #[inline(always)]
    pub fn uvswd(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        evradcstat::Uvswd,
        Evradcstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            evradcstat::Uvswd,
            Evradcstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evradcstat {
    #[inline(always)]
    fn default() -> Evradcstat {
        <crate::RegValueT<Evradcstat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod evradcstat {
    pub struct Ovc_SPEC;
    pub type Ovc = crate::EnumBitfieldStruct<u8, Ovc_SPEC>;
    impl Ovc {
        #[doc = "0 No over voltage        condition happened."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 VDD        Over voltage event indication as configured in HSMOVMON register."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ov33_SPEC;
    pub type Ov33 = crate::EnumBitfieldStruct<u8, Ov33_SPEC>;
    impl Ov33 {
        #[doc = "0 No over voltage        condition happened."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 VDDP3        Over voltage event indication as configured in HSMOVMON register."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ovswd_SPEC;
    pub type Ovswd = crate::EnumBitfieldStruct<u8, Ovswd_SPEC>;
    impl Ovswd {
        #[doc = "0 No over voltage        condition happened."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 VEXT        Over voltage event indication as configured in HSMOVMON register."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Uvc_SPEC;
    pub type Uvc = crate::EnumBitfieldStruct<u8, Uvc_SPEC>;
    impl Uvc {
        #[doc = "0 No        under voltage condition happened."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 VDD        Under voltage event indication as configured in HSMUVMON register."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Uv33_SPEC;
    pub type Uv33 = crate::EnumBitfieldStruct<u8, Uv33_SPEC>;
    impl Uv33 {
        #[doc = "0 No        under voltage condition happened."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 VDDP3        Under voltage event indication as configured in HSMUVMON register."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Uvswd_SPEC;
    pub type Uvswd = crate::EnumBitfieldStruct<u8, Uvswd_SPEC>;
    impl Uvswd {
        #[doc = "0 No        under voltage condition happened."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 VEXT        Under voltage event indication as configured in HSMUVMON register."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evranactrl_SPEC;
impl crate::sealed::RegSpec for Evranactrl_SPEC {
    type DataType = u32;
}
#[doc = "EVR Analog Control Register\n resetvalue={LVD Reset:0x0,After SSW execution:0x0}"]
pub type Evranactrl = crate::RegValueT<Evranactrl_SPEC>;

impl Evranactrl {
    #[doc = "PLACEHOLDER   CONTROL. This register bit field contains definitions for EVR control in general.        Layout of the bit field is to be defined later."]
    #[inline(always)]
    pub fn control(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Evranactrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Evranactrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Evranactrl {
    #[inline(always)]
    fn default() -> Evranactrl {
        <crate::RegValueT<Evranactrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrbgoctrl_SPEC;
impl crate::sealed::RegSpec for Evrbgoctrl_SPEC {
    type DataType = u32;
}
#[doc = "EVR Bandgap and Oscillator Control Register\n resetvalue={LVD Reset:0x080109340,After SSW execution:0x080108140}"]
pub type Evrbgoctrl = crate::RegValueT<Evrbgoctrl_SPEC>;

impl Evrbgoctrl {
    #[doc = "SHPBG Bandgap Trim Value   VBGTRIM. This bit field contains information about trimming of the high precision        bandgap.The Bandgap voltage is measured via Analog Test Bus at 125  176 C and        the value is trimmed so that the measurement corresponds to 1.13V. A        settling time of   30  160 us need to be waited after a new trim value is        programmed.  trimvpbg  trimbg i    VBGTRIM   VBGPTRIM  signed value"]
    #[inline(always)]
    pub fn vbgtrim(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Evrbgoctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8, Evrbgoctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Back up Clock Coarse Trim Value   OSCCTRIM. This binary coded bit field contains information about the 100MHz OSC        coarse trimming.  osc trim ct 1v2  fBACK  160    160  fOFFSET    OSCCTRIM  160    160 LSBCT      OSCFTRIM   OSCFPTRIM           160    160 LSBFT    160 MHz  LSBCT   323 kHz   LSBFT   110kHz  TEMPSTEP   1.4MHz. SIGN    1 if OSCTEMPSIGN   0   1 if OSCTEMPSIGN   1. fOFFSET   55 MHz    SIGN  160    160 OSCTEMPTRIM   TEMPSTEP     OFFSTEN   OFFST . OFFST   4.7 MHz  OFFSTEN  1 if OSCTEMPOFFS   0  OFFSTEN  0 if        OSCTEMPOFFS   1. Back up Clock accuracy is documented in datasheet. When coarse trimming is done  fine trimming should be kept to 1F. The        actual coarse trimming value is trimmed in production and may deviate        from the reset value. LSBCT trimming error      323 kHz  LSBCT    temp drift"]
    #[inline(always)]
    pub fn oscctrim(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xff,
        1,
        0,
        evrbgoctrl::Oscctrim,
        Evrbgoctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xff,
            1,
            0,
            evrbgoctrl::Oscctrim,
            Evrbgoctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "IREF Trim Value trimiref trimcur i    IREFTRIM. This bit field contains information about trimming of the reference        current cicuitry. A settling time of x  160 us is required after update.        After HPBG trimming  the reference current is measured via Analog Test        Bus at 125  176 C against 1.2V and the value is trimmed so that the        measurement corresponds to 20uA."]
    #[inline(always)]
    pub fn ireftrim(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Evrbgoctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Evrbgoctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SHPBG Bandgap Signed Trim Value   VBGPTRIM. This bit field allows device individual trimming of the bandgap trim        value in production. The first bit is sign information. The range is  16        to 15."]
    #[inline(always)]
    pub fn vbgptrim(
        self,
    ) -> crate::common::RegisterField<21, 0x1f, 1, 0, u8, Evrbgoctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x1f,1,0,u8, Evrbgoctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OSC Temperature Coefficient Selection   OSCTEMPTRIM. This bit field selects the temperature coefficient trimming bits for 100        MHz OSC configuration.  osc tc trim sel 1v2"]
    #[inline(always)]
    pub fn osctemptrim(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x7,
        1,
        0,
        evrbgoctrl::Osctemptrim,
        Evrbgoctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x7,
            1,
            0,
            evrbgoctrl::Osctemptrim,
            Evrbgoctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "OSC Temperature Coefficient Sign   OSCTEMPSIGN. This bit field selects the sign of temperature coefficient trimming bits        for 100 MHz OSC configuration.  osc tc trim sign 1v2"]
    #[inline(always)]
    pub fn osctempsign(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        evrbgoctrl::Osctempsign,
        Evrbgoctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            evrbgoctrl::Osctempsign,
            Evrbgoctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        evrbgoctrl::Slck,
        Evrbgoctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            evrbgoctrl::Slck,
            Evrbgoctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "HPBG VI Chopping Enable   ENVICHOP. This bit field enables chopping for V I converter"]
    #[inline(always)]
    pub fn envichop(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        evrbgoctrl::Envichop,
        Evrbgoctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            evrbgoctrl::Envichop,
            Evrbgoctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrbgoctrl {
    #[inline(always)]
    fn default() -> Evrbgoctrl {
        <crate::RegValueT<Evrbgoctrl_SPEC> as RegisterValue<_>>::new(2148565312)
    }
}
pub mod evrbgoctrl {
    pub struct Oscctrim_SPEC;
    pub type Oscctrim = crate::EnumBitfieldStruct<u8, Oscctrim_SPEC>;
    impl Oscctrim {
        #[doc = "00000000   160 55 MHz"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "11111111   160 145 MHz"]
        pub const CONST_255255: Self = Self::new(255);
    }
    pub struct Osctemptrim_SPEC;
    pub type Osctemptrim = crate::EnumBitfieldStruct<u8, Osctemptrim_SPEC>;
    impl Osctemptrim {
        #[doc = "000 Lowest temperature coefficient"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "111 Highest temperature coefficient"]
        pub const CONST_77: Self = Self::new(7);
    }
    pub struct Osctempsign_SPEC;
    pub type Osctempsign = crate::EnumBitfieldStruct<u8, Osctempsign_SPEC>;
    impl Osctempsign {
        #[doc = "0 PTAT negative temperature coefficient"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 CTAT positive temperature coefficient"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Slck_SPEC;
    pub type Slck = crate::EnumBitfieldStruct<u8, Slck_SPEC>;
    impl Slck {
        #[doc = "0 No lock active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Lock is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Envichop_SPEC;
    pub type Envichop = crate::EnumBitfieldStruct<u8, Envichop_SPEC>;
    impl Envichop {
        #[doc = "0 VI Chopping disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 VI Chopping enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrbgoctrl2_SPEC;
impl crate::sealed::RegSpec for Evrbgoctrl2_SPEC {
    type DataType = u32;
}
#[doc = "EVR Bandgap and Oscillator Control Register 2\n resetvalue={LVD Reset:0x09,After SSW execution:0x09}"]
pub type Evrbgoctrl2 = crate::RegValueT<Evrbgoctrl2_SPEC>;

impl Evrbgoctrl2 {
    #[doc = "IREF Dynamic Trim Offset Value for Temperature Range 0.   IREFTR0. This bit field contains trim offset information for dynamic trimming of        the reference current for the temperature range.         0  160   8804   160 DTSSTAT.RESULT  160  lt   160 2048"]
    #[inline(always)]
    pub fn ireftr0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Evrbgoctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Evrbgoctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IREF Dynamic Trim Offset Value for Temperature Range 1.   IREFTR1. This bit field contains trim offset information for dynamic trimming of        the reference current for the temperature range.         2048  160   8804   160 DTSSTAT.RESULT  160  lt   160 2304"]
    #[inline(always)]
    pub fn ireftr1(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, Evrbgoctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x7,1,0,u8, Evrbgoctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IREF Dynamic Trim Offset Value for Temperature Range 2.   IREFTR2. This bit field contains trim offset information for dynamic trimming of        the reference current for the temperature range.         2304  160   8804   160 DTSSTAT.RESULT  160  lt   160 2560"]
    #[inline(always)]
    pub fn ireftr2(
        self,
    ) -> crate::common::RegisterField<6, 0x7, 1, 0, u8, Evrbgoctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x7,1,0,u8, Evrbgoctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IREF Dynamic Trim Offset Value for Temperature Range 3.   IREFTR3. This bit field contains trim offset information for dynamic trimming of        the reference current for the temperature range.         2560  160   8804   160 DTSSTAT.RESULT  160  lt   160 2816"]
    #[inline(always)]
    pub fn ireftr3(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, Evrbgoctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7,1,0,u8, Evrbgoctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IREF Dynamic Trim Offset Value for Temperature Range 4.   IREFTR4. This bit field contains trim offset information for dynamic trimming of        the reference current for the temperature range.         2816  160   8804   160 DTSSTAT.RESULT  160  lt   160 3072"]
    #[inline(always)]
    pub fn ireftr4(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, Evrbgoctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8, Evrbgoctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IREF Dynamic Trim Offset Value for Temperature Range 5.   IREFTR5. This bit field contains trim offset information for dynamic trimming of        the reference current for the temperature range.         3072  160   8804   160 DTSSTAT.RESULT  160  lt   160 3328"]
    #[inline(always)]
    pub fn ireftr5(
        self,
    ) -> crate::common::RegisterField<15, 0x7, 1, 0, u8, Evrbgoctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<15,0x7,1,0,u8, Evrbgoctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IREF Dynamic Trim Offset Value for Temperature Range 6.   IREFTR6. This bit field contains trim offset information for dynamic trimming of        the reference current for the temperature range.         3328  160   8804   160 DTSSTAT.RESULT  160  lt   160 3584"]
    #[inline(always)]
    pub fn ireftr6(
        self,
    ) -> crate::common::RegisterField<18, 0x7, 1, 0, u8, Evrbgoctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x7,1,0,u8, Evrbgoctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IREF Dynamic Trim Offset Value for Temperature Range 7.   IREFTR7. This bit field contains trim offset information for dynamic trimming of        the reference current for the temperature range.         3584  160   8804   160 DTSSTAT.RESULT  160   163   160 4096"]
    #[inline(always)]
    pub fn ireftr7(
        self,
    ) -> crate::common::RegisterField<21, 0x7, 1, 0, u8, Evrbgoctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x7,1,0,u8, Evrbgoctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        evrbgoctrl2::Slck,
        Evrbgoctrl2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            evrbgoctrl2::Slck,
            Evrbgoctrl2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Dynamic IREF Trim Enable   IREFTRIMEN. Based on temperature  Iref can be trimmed."]
    #[inline(always)]
    pub fn ireftrimen(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        evrbgoctrl2::Ireftrimen,
        Evrbgoctrl2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            evrbgoctrl2::Ireftrimen,
            Evrbgoctrl2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrbgoctrl2 {
    #[inline(always)]
    fn default() -> Evrbgoctrl2 {
        <crate::RegValueT<Evrbgoctrl2_SPEC> as RegisterValue<_>>::new(9)
    }
}
pub mod evrbgoctrl2 {
    pub struct Slck_SPEC;
    pub type Slck = crate::EnumBitfieldStruct<u8, Slck_SPEC>;
    impl Slck {
        #[doc = "0 No lock active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Lock is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ireftrimen_SPEC;
    pub type Ireftrimen = crate::EnumBitfieldStruct<u8, Ireftrimen_SPEC>;
    impl Ireftrimen {
        #[doc = "0 The Dynamic Iref Trim function is disabled  switched off."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The Dynamic Iref Trim function is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrccon_SPEC;
impl crate::sealed::RegSpec for Evrccon_SPEC {
    type DataType = u32;
}
#[doc = "EVRC Control Register\n resetvalue={LVD Reset:0x0,Cold PORST:0x0,After SSW execution:0x0}"]
pub type Evrccon = crate::RegValueT<Evrccon_SPEC>;

impl Evrccon {
    #[doc = "EVRC Regulator Enable   EVRCOFF. This bit can only be changed if bit BPEVRCOFF is set in parallel. After        EVRCOFF bit is set  a minimum disable time 50 fBACK clock cycles is        awaited to ensure proper EVRC regulator switch off. Consequently a        re enabling of EVRC cannot be done during this disable time of 50 fBACK        clock cycles. EVRCOFF is intended to be used only for internal test        purposes and is not used in customer application."]
    #[inline(always)]
    pub fn evrcoff(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        evrccon::Evrcoff,
        Evrccon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            evrccon::Evrcoff,
            Evrccon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Bit Protection EVRCOFF   BPEVRCOFF. Setting this bit enables that bit EVRCOFF can be changed in this write        operation.This bit is read as zero. EVRC regulator can only be disabled        via EVRCOFF and cannot be consequently enabled after disabling."]
    #[inline(always)]
    pub fn bpevrcoff(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Evrccon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, Evrccon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, evrccon::Slck, Evrccon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,evrccon::Slck, Evrccon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Evrccon {
    #[inline(always)]
    fn default() -> Evrccon {
        <crate::RegValueT<Evrccon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod evrccon {
    pub struct Evrcoff_SPEC;
    pub type Evrcoff = crate::EnumBitfieldStruct<u8, Evrcoff_SPEC>;
    impl Evrcoff {
        #[doc = "0 The EVRC regulator module is enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The EVRC        regulator module is disabled  switched        off. The device may be operating in standby or with external VDD supply."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Slck_SPEC;
    pub type Slck = crate::EnumBitfieldStruct<u8, Slck_SPEC>;
    impl Slck {
        #[doc = "0 No lock active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Lock is active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrmonctrl_SPEC;
impl crate::sealed::RegSpec for Evrmonctrl_SPEC {
    type DataType = u32;
}
#[doc = "EVR Secondary Monitor Control Register\n resetvalue={LVD Reset:0x0A5A5A5,Cold PORST:0x0A5A5A5,After SSW execution:0x0A5A5A5}"]
pub type Evrmonctrl = crate::RegValueT<Evrmonctrl_SPEC>;

impl Evrmonctrl {
    #[doc = "VDD Over voltage monitoring mode   EVRCOVMOD. Incase both EVRCOVMOD  160    160  00  amp         EVRCUVMOD  160    160  00   then ADC        conversion for the respective supply rail does not take place."]
    #[inline(always)]
    pub fn evrcovmod(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        evrmonctrl::Evrcovmod,
        Evrmonctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            evrmonctrl::Evrcovmod,
            Evrmonctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "EVRPR or VDDPD Over voltage monitoring mode   PREOVMOD. Incase both PREOVMOD  160    160  00  amp         PREUVMOD  160    160  00   then ADC        conversion for the respective supply rail does not take place."]
    #[inline(always)]
    pub fn preovmod(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        evrmonctrl::Preovmod,
        Evrmonctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            evrmonctrl::Preovmod,
            Evrmonctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VDD Under voltage monitoring mode   EVRCUVMOD. Incase both EVRCOVMOD  160    160  00  amp         EVRCUVMOD  160    160  00   then ADC        conversion for the respective supply rail does not take place."]
    #[inline(always)]
    pub fn evrcuvmod(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        evrmonctrl::Evrcuvmod,
        Evrmonctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            evrmonctrl::Evrcuvmod,
            Evrmonctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "EVRPR or VDDPD Under voltage monitoring mode   PREUVMOD. Incase both PREOVMOD  160    160  00  amp         PREUVMOD  160    160  00   then ADC        conversion for the respective supply rail does not take place."]
    #[inline(always)]
    pub fn preuvmod(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        evrmonctrl::Preuvmod,
        Evrmonctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            evrmonctrl::Preuvmod,
            Evrmonctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VDDP3 Supply Over voltage monitoring mode   EVR33OVMOD. Incase both EVR33OVMOD  160    160  00  amp         EVR33UVMOD  160    160  00   then ADC        conversion for the respective supply rail does not take place."]
    #[inline(always)]
    pub fn evr33ovmod(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        evrmonctrl::Evr33Ovmod,
        Evrmonctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            evrmonctrl::Evr33Ovmod,
            Evrmonctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VDDM ADC Supply Over voltage monitoring mode   VDDMOVMOD. Incase both VDDMOVMOD  160    160  00  amp         VDDMUVMOD  160    160  00   then ADC        conversion for the VDDM supply rail continues to run as used for ADC        function."]
    #[inline(always)]
    pub fn vddmovmod(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        evrmonctrl::Vddmovmod,
        Evrmonctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            evrmonctrl::Vddmovmod,
            Evrmonctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VDDP3 Supply Under voltage monitoring mode   EVR33UVMOD. Incase both EVR33OVMOD  160    160  00  amp         EVR33UVMOD  160    160  00   then ADC        conversion for the respective supply rail does not take place."]
    #[inline(always)]
    pub fn evr33uvmod(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x3,
        1,
        0,
        evrmonctrl::Evr33Uvmod,
        Evrmonctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x3,
            1,
            0,
            evrmonctrl::Evr33Uvmod,
            Evrmonctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VDDM ADC Supply Under voltage monitoring mode   VDDMUVMOD. Incase both VDDMOVMOD  160    160  00  amp         VDDMUVMOD  160    160  00   then ADC        conversion for the VDDM supply rail continues to run as used for ADC        function."]
    #[inline(always)]
    pub fn vddmuvmod(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        evrmonctrl::Vddmuvmod,
        Evrmonctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            evrmonctrl::Vddmuvmod,
            Evrmonctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VEXT Over voltage monitoring mode   SWDOVMOD. Incase both SWDOVMOD  160    160  00  amp         SWDUVMOD  160    160  00   then ADC        conversion for the respective supply rail does not take place."]
    #[inline(always)]
    pub fn swdovmod(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        evrmonctrl::Swdovmod,
        Evrmonctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            evrmonctrl::Swdovmod,
            Evrmonctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "EVR Standby Supply or VEVRSB Over voltage monitoring mode   SBOVMOD. Incase both SBOVMOD  160    160  00  amp         SBUVMOD  160    160  00   then ADC        conversion for the respective supply rail does not take place."]
    #[inline(always)]
    pub fn sbovmod(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x3,
        1,
        0,
        evrmonctrl::Sbovmod,
        Evrmonctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x3,
            1,
            0,
            evrmonctrl::Sbovmod,
            Evrmonctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VEXT Under voltage monitoring mode   SWDUVMOD. Incase both SWDOVMOD  160    160  00  amp         SWDUVMOD  160    160  00   then ADC        conversion for the respective supply rail does not take place."]
    #[inline(always)]
    pub fn swduvmod(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        evrmonctrl::Swduvmod,
        Evrmonctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            evrmonctrl::Swduvmod,
            Evrmonctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "EVR Standby Supply or VEVRSB Under voltage monitoring mode   SBUVMOD. Incase both SBOVMOD  160    160  00  amp         SBUVMOD  160    160  00   then ADC        conversion for the respective supply rail does not take place."]
    #[inline(always)]
    pub fn sbuvmod(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x3,
        1,
        0,
        evrmonctrl::Sbuvmod,
        Evrmonctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x3,
            1,
            0,
            evrmonctrl::Sbuvmod,
            Evrmonctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        evrmonctrl::Slck,
        Evrmonctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            evrmonctrl::Slck,
            Evrmonctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrmonctrl {
    #[inline(always)]
    fn default() -> Evrmonctrl {
        <crate::RegValueT<Evrmonctrl_SPEC> as RegisterValue<_>>::new(10855845)
    }
}
pub mod evrmonctrl {
    pub struct Evrcovmod_SPEC;
    pub type Evrcovmod = crate::EnumBitfieldStruct<u8, Evrcovmod_SPEC>;
    impl Evrcovmod {
        #[doc = "00 Over voltage        monitoring inactive .        This results in a complete reset of the comparator unit  status bits and        filter values and alarm is deasserted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 An        over voltage event is triggered when the threshold is crossed in a lower        to higher voltage transition .        Greater than or equal compare is used."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 An        over voltage event is triggered when the threshold is crossed in a        higher to lower voltage transition .        Less than or equal compare is used."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 An        over voltage event is triggered when the threshold is crossed in either        direction . Greater        than or equal compare is used."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Preovmod_SPEC;
    pub type Preovmod = crate::EnumBitfieldStruct<u8, Preovmod_SPEC>;
    impl Preovmod {
        #[doc = "00 Over voltage        monitoring inactive .        This results in a complete reset of the comparator unit  status bits and        filter values and alarm is deasserted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 An        over voltage event is triggered when the threshold is crossed in a lower        to higher voltage transition .        Greater than or equal compare is used."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 An        over voltage event is triggered when the threshold is crossed in a        higher to lower voltage transition .        Less than or equal compare is used."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 An        over voltage event is triggered when the threshold is crossed in either        direction . Greater        than or equal compare is used."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Evrcuvmod_SPEC;
    pub type Evrcuvmod = crate::EnumBitfieldStruct<u8, Evrcuvmod_SPEC>;
    impl Evrcuvmod {
        #[doc = "00 Under voltage        monitoring inactive .        This results in a complete reset of the comparator unit  status bits and        filter values and alarm is deasserted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 An        under voltage event is triggered when the threshold is crossed in a        lower to higher voltage transition. Greater than or equal compare is        used."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 An        under voltage event is triggered when the threshold is crossed in a        higher to lower voltage transition .        Less than or equal compare is used."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 An        under voltage event is triggered when the threshold is crossed in either        direction. Less than or equal compare is used."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Preuvmod_SPEC;
    pub type Preuvmod = crate::EnumBitfieldStruct<u8, Preuvmod_SPEC>;
    impl Preuvmod {
        #[doc = "00 Under voltage        monitoring inactive .        This results in a complete reset of the comparator unit  status bits and        filter values and alarm is deasserted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 An        under voltage event is triggered when the threshold is crossed in a        lower to higher voltage transition. Greater than or equal compare is        used."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 An        under voltage event is triggered when the threshold is crossed in a        higher to lower voltage transition .        Less than or equal compare is used."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 An        under voltage event is triggered when the threshold is crossed in either        direction. Less than or equal compare is used."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Evr33Ovmod_SPEC;
    pub type Evr33Ovmod = crate::EnumBitfieldStruct<u8, Evr33Ovmod_SPEC>;
    impl Evr33Ovmod {
        #[doc = "00 Over voltage        monitoring inactive .        This results in a complete reset of the comparator unit  status bits and        filter values and alarm is deasserted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 An        over voltage event is triggered when the threshold is crossed in a lower        to higher voltage transition .        Greater than or equal compare is used."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 An        over voltage event is triggered when the threshold is crossed in a        higher to lower voltage transition .        Less than or equal compare is used."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 An        over voltage event is triggered when the threshold is crossed in either        direction . Greater        than or equal compare is used."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Vddmovmod_SPEC;
    pub type Vddmovmod = crate::EnumBitfieldStruct<u8, Vddmovmod_SPEC>;
    impl Vddmovmod {
        #[doc = "00 Over voltage        monitoring inactive .        This results in a complete reset of the comparator unit  status bits and        filter values and alarm is deasserted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 An        over voltage event is triggered when the threshold is crossed in a lower        to higher voltage transition .        Greater than or equal compare is used."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 An        over voltage event is triggered when the threshold is crossed in a        higher to lower voltage transition .Less        than or equal compare is used."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 An        over voltage event is triggered when the threshold is crossed in either        direction . Greater        than or equal compare is used."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Evr33Uvmod_SPEC;
    pub type Evr33Uvmod = crate::EnumBitfieldStruct<u8, Evr33Uvmod_SPEC>;
    impl Evr33Uvmod {
        #[doc = "00 Under voltage        monitoring inactive .        This results in a complete reset of the comparator unit  status bits and        filter values and alarm is deasserted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 An        under voltage event is triggered when the threshold is crossed in a        lower to higher voltage transition. Greater than or equal compare is        used."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 An        under voltage event is triggered when the threshold is crossed in a        higher to lower voltage transition .        Less than or equal compare is used."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 An        under voltage event is triggered when the threshold is crossed in either        direction. Less than or equal compare is used."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Vddmuvmod_SPEC;
    pub type Vddmuvmod = crate::EnumBitfieldStruct<u8, Vddmuvmod_SPEC>;
    impl Vddmuvmod {
        #[doc = "00 Under voltage        monitoring inactive .        This results in a complete reset of the comparator unit  status bits and        filter values and alarm is deasserted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 An        under voltage event is triggered when the threshold is crossed in a        lower to higher voltage transition. Greater than or equal compare is        used."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 An        under voltage event is triggered when the threshold is crossed in a        higher to lower voltage transition .        Less than or equal compare is used."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 An        under voltage event is triggered when the threshold is crossed in either        direction. Less than or equal compare is used."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Swdovmod_SPEC;
    pub type Swdovmod = crate::EnumBitfieldStruct<u8, Swdovmod_SPEC>;
    impl Swdovmod {
        #[doc = "00 Over voltage        monitoring inactive .        This results in a complete reset of the comparator unit  status bits and        filter values and alarm is deasserted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 An        over voltage event is triggered when the threshold is crossed in a lower        to higher voltage transition .        Greater than or equal compare is used."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 An        over voltage event is triggered when the threshold is crossed in a        higher to lower voltage transition .        Less than or equal compare is used."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 An        over voltage event is triggered when the threshold is crossed in either        direction . Greater        than or equal compare is used."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sbovmod_SPEC;
    pub type Sbovmod = crate::EnumBitfieldStruct<u8, Sbovmod_SPEC>;
    impl Sbovmod {
        #[doc = "00 Over voltage        monitoring inactive .        This results in a complete reset of the comparator unit  status bits and        filter values and alarm is deasserted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 An        over voltage event is triggered when the threshold is crossed in a lower        to higher voltage transition .        Greater than or equal compare is used."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 An        over voltage event is triggered when the threshold is crossed in a        higher to lower voltage transition .Less        than or equal compare is used."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 An        over voltage event is triggered when the threshold is crossed in either        direction . Greater        than or equal compare is used."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Swduvmod_SPEC;
    pub type Swduvmod = crate::EnumBitfieldStruct<u8, Swduvmod_SPEC>;
    impl Swduvmod {
        #[doc = "00 Under voltage        monitoring inactive .        This results in a complete reset of the comparator unit  status bits and        filter values and alarm is deasserted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 An        under voltage event is triggered when the threshold is crossed in a        lower to higher voltage transition. Greater than or equal compare is        used."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 An        under voltage event is triggered when the threshold is crossed in a        higher to lower voltage transition .        Less than or equal compare is used."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 An        under voltage event is triggered when the threshold is crossed in either        direction. Less than or equal compare is used."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sbuvmod_SPEC;
    pub type Sbuvmod = crate::EnumBitfieldStruct<u8, Sbuvmod_SPEC>;
    impl Sbuvmod {
        #[doc = "00 Under voltage        monitoring inactive .        This results in a complete reset of the comparator unit  status bits and        filter values and alarm is deasserted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 An        under voltage event is triggered when the threshold is crossed in a        lower to higher voltage transition. Greater than or equal compare is        used."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 An        under voltage event is triggered when the threshold is crossed in a        higher to lower voltage transition .        Less than or equal compare is used."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 An        under voltage event is triggered when the threshold is crossed in either        direction. Less than or equal compare is used."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Slck_SPEC;
    pub type Slck = crate::EnumBitfieldStruct<u8, Slck_SPEC>;
    impl Slck {
        #[doc = "0 No lock active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Lock is active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrmonfilt_SPEC;
impl crate::sealed::RegSpec for Evrmonfilt_SPEC {
    type DataType = u32;
}
#[doc = "EVR Secondary Monitor Filter Register\n resetvalue={LVD Reset:0x300,Cold PORST:0x300,After SSW execution:0x10301}"]
pub type Evrmonfilt = crate::RegValueT<Evrmonfilt_SPEC>;

impl Evrmonfilt {
    #[doc = "VDD Secondary ADC Supply Filter   EVRCFIL"]
    #[inline(always)]
    pub fn evrcfil(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xf,
        1,
        0,
        evrmonfilt::Evrcfil,
        Evrmonfilt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xf,
            1,
            0,
            evrmonfilt::Evrcfil,
            Evrmonfilt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VDDPD Secondary ADC Supply Filter   PREFIL"]
    #[inline(always)]
    pub fn prefil(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        evrmonfilt::Prefil,
        Evrmonfilt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            evrmonfilt::Prefil,
            Evrmonfilt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VDDP3 Secondary ADC Supply Filter   EVR33FIL"]
    #[inline(always)]
    pub fn evr33fil(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        evrmonfilt::Evr33Fil,
        Evrmonfilt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            evrmonfilt::Evr33Fil,
            Evrmonfilt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VDDM Secondary ADC Supply Filter   VDDMFIL"]
    #[inline(always)]
    pub fn vddmfil(
        self,
    ) -> crate::common::RegisterField<
        12,
        0xf,
        1,
        0,
        evrmonfilt::Vddmfil,
        Evrmonfilt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0xf,
            1,
            0,
            evrmonfilt::Vddmfil,
            Evrmonfilt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VEXT Secondary ADC Supply Filter   SWDFIL"]
    #[inline(always)]
    pub fn swdfil(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        evrmonfilt::Swdfil,
        Evrmonfilt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            evrmonfilt::Swdfil,
            Evrmonfilt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VEVRSB Secondary ADC Supply Filter   SBFIL"]
    #[inline(always)]
    pub fn sbfil(
        self,
    ) -> crate::common::RegisterField<
        20,
        0xf,
        1,
        0,
        evrmonfilt::Sbfil,
        Evrmonfilt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0xf,
            1,
            0,
            evrmonfilt::Sbfil,
            Evrmonfilt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Clear all Spike Filters   CLRFIL. To avoid spurious alarms during change of configuration or start up         CLRFIL shall be set followed by alarm reconfiguration followed by        activation of filter logic by clearing CLRFIL register bit."]
    #[inline(always)]
    pub fn clrfil(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        evrmonfilt::Clrfil,
        Evrmonfilt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            evrmonfilt::Clrfil,
            Evrmonfilt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        evrmonfilt::Slck,
        Evrmonfilt_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            evrmonfilt::Slck,
            Evrmonfilt_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrmonfilt {
    #[inline(always)]
    fn default() -> Evrmonfilt {
        <crate::RegValueT<Evrmonfilt_SPEC> as RegisterValue<_>>::new(768)
    }
}
pub mod evrmonfilt {
    pub struct Evrcfil_SPEC;
    pub type Evrcfil = crate::EnumBitfieldStruct<u8, Evrcfil_SPEC>;
    impl Evrcfil {
        #[doc = "0 Each        conversion result is compared with threshold to generate alarm"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "F A spike filter        of consecutive 16 ADC results are used to generate alarm to SMU."]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Prefil_SPEC;
    pub type Prefil = crate::EnumBitfieldStruct<u8, Prefil_SPEC>;
    impl Prefil {
        #[doc = "0 Each        conversion result is compared with threshold to generate alarm"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "F A spike filter        of consecutive 16 ADC results are used to generate alarm to SMU."]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Evr33Fil_SPEC;
    pub type Evr33Fil = crate::EnumBitfieldStruct<u8, Evr33Fil_SPEC>;
    impl Evr33Fil {
        #[doc = "0 Each        conversion result is compared with threshold to generate alarm"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "F A spike filter        of consecutive 16 ADC results are used to generate alarm to SMU."]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Vddmfil_SPEC;
    pub type Vddmfil = crate::EnumBitfieldStruct<u8, Vddmfil_SPEC>;
    impl Vddmfil {
        #[doc = "0 Each        conversion result is compared with threshold to generate alarm"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "F A spike filter        of consecutive 16 ADC results are used to generate alarm to SMU."]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Swdfil_SPEC;
    pub type Swdfil = crate::EnumBitfieldStruct<u8, Swdfil_SPEC>;
    impl Swdfil {
        #[doc = "0 Each        conversion result is compared with threshold to generate alarm"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "F A spike filter        of consecutive 16 ADC results are used to generate alarm to SMU."]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Sbfil_SPEC;
    pub type Sbfil = crate::EnumBitfieldStruct<u8, Sbfil_SPEC>;
    impl Sbfil {
        #[doc = "0 Each        conversion result is compared with threshold to generate alarm"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "F A spike filter        of consecutive 16 ADC results are used to generate alarm to SMU."]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Clrfil_SPEC;
    pub type Clrfil = crate::EnumBitfieldStruct<u8, Clrfil_SPEC>;
    impl Clrfil {
        #[doc = "0 No effect"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 All spike        filters configured in EVRMONFILT register are reset. The xFIL        configuration value remains as configured and continue to be used for        adc filtration."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Slck_SPEC;
    pub type Slck = crate::EnumBitfieldStruct<u8, Slck_SPEC>;
    impl Slck {
        #[doc = "0 No lock active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Lock is active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrmonstat1_SPEC;
impl crate::sealed::RegSpec for Evrmonstat1_SPEC {
    type DataType = u32;
}
#[doc = "EVR Secondary ADC Status Register 1\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
pub type Evrmonstat1 = crate::RegValueT<Evrmonstat1_SPEC>;

impl Evrmonstat1 {
    #[doc = "VDD Supply Secondary ADC Conversion Result   ADCCV. This bit field contains the last conversion result of the ADC        measurement of the VDD  160    160 EVRC supply by the Secondary Monitor. This        bitfield is updated if secondary over  or under voltage monitoring is        activated via EVRMONCTRL.EVRCxxMOD. The        update of the result is taking place at   6  160    160    160 12  160    160 active channels  160                160 fBACK   4  160    clock cycles. VIN  160    160  LSB  160    160  ADCx 1    160    160 Ideal LSB   5.7692  160 mV Full Range  160    160 1465  160 mV E.g. 1.25  160 V   DA"]
    #[inline(always)]
    pub fn adccv(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrmonstat1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrmonstat1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VDDP3 Supply Secondary ADC Conversion Result   ADC33V. This bit field contains the last conversion result of the ADC        measurement of the VDDP3  160    160 EVR33 supply by the Secondary Monitor. This        bitfield is updated if secondary over  or under voltage monitoring is        activated via EVRMONCTRL.EVR33xxMOD. The        update of the result is taking place at   6  160    160    160 12  160    160 active channels  160                160 fBACK   4  160    clock cycles. VIN  160    160  LSB  160    160  ADCx 1    160    160 Ideal LSB   15.00  160 mV Full Range  160    160 3810  160 mV E.g. 3.30  160 V   DD"]
    #[inline(always)]
    pub fn adc33v(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrmonstat1_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrmonstat1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VEXT Supply Secondary ADC Conversion Result   ADCSWDV. This bit field contains the last conversion result of the ADC        measurement of the external VEXT  3.3V  160    160 5V  supply by the Secondary        Monitor. This bitfield is updated if secondary over  or under voltage        monitoring is activated via EVRMONCTRL.SWDxxMOD. The        update of the result is taking place at   6  160    160    160 12  160    160 active channels  160                160 fBACK   4  160    clock cycles. VIN  160    160  LSB  160    160  ADCx 1    160    160 LSB   23.077  160 mV Full Range  160    160 5861  160 mV E.g. 5.01  160 V   DA   160   160   160   160   160    160 3.3  160 V   90"]
    #[inline(always)]
    pub fn adcswdv(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evrmonstat1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evrmonstat1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Secondary Monitor Activity Counter   ACTVCNT. This bit field cumulatively counts the end of conversion signals in a        single Secondary Monitor Background Scan over all channels and        respective filter configurations. The total number of conversions ConvTot     8721   ChX  ChXFIL . The counter is reset to 0 on a ConvTot overflow."]
    #[inline(always)]
    pub fn actvcnt(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, Evrmonstat1_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x3f,1,0,u8, Evrmonstat1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrmonstat1 {
    #[inline(always)]
    fn default() -> Evrmonstat1 {
        <crate::RegValueT<Evrmonstat1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrmonstat2_SPEC;
impl crate::sealed::RegSpec for Evrmonstat2_SPEC {
    type DataType = u32;
}
#[doc = "EVR Secondary ADC Status Register 2\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
pub type Evrmonstat2 = crate::RegValueT<Evrmonstat2_SPEC>;

impl Evrmonstat2 {
    #[doc = "VDDPD Supply Secondary ADC Conversion Result   ADCPRE. This bit field contains the last conversion result of the ADC        measurement of the VDDPD supply by the Secondary Monitor. This bitfield        is updated if secondary over  or under voltage monitoring is activated        via EVRMONCTRL.PRExxMOD. The        update of the result is taking place at   6  160    160    160 12  160    160 active channels  160                160 fBACK   4  160    clock cycles. VIN  160    160  LSB  160    160  ADCx 1    160    160 Ideal LSB   5.7692  160 mV Full Range  160    160 1465  160 mV E.g. 1.25  160 V   DA"]
    #[inline(always)]
    pub fn adcpre(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrmonstat2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrmonstat2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VEVRSB Supply Secondary ADC Conversion Result   ADCSB. This bit field contains the last conversion result of the ADC        measurement of the external VEVRSB  3.3V  160    160 5V  standby supply by the        Secondary Monitor. This bitfield is updated if secondary over  or        under voltage monitoring is activated via EVRMONCTRL.SBxxMOD. The        update of the result is taking place at   6  160    160    160 12  160    160 active channels  160                160 fBACK   4  160    clock cycles. VIN  160    160  LSB  160    160  ADCx 1    160    160 Ideal LSB   23.077  160 mV Full Range  160    160 5861  160 mV E.g. 5.01  160 V   DA   160   160   160   160   160   160 3.0  160 V   90"]
    #[inline(always)]
    pub fn adcsb(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrmonstat2_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrmonstat2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VDDM Supply Secondary ADC Conversion Result   ADCVDDM. This bit field contains the last conversion result of the ADC        measurement of the VDDM ADC supply by the Secondary Monitor. This        bitfield is updated if secondary over  or under voltage monitoring is        activated via EVRMONCTRL.VDDMxxMOD. The        update of the result is taking place at   6  160    160    160 12  160    160 active channels  160                160 fBACK   4  160    clock cycles. VIN  160    160  LSB  160    160  ADCx 1    160    160 Ideal LSB   23.077  160 mV Full Range  160    160 5861  160 mV E.g. 5.01  160 V   DA   160   160   160   160   160   160 3.0  160 V   90"]
    #[inline(always)]
    pub fn adcvddm(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evrmonstat2_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evrmonstat2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrmonstat2 {
    #[inline(always)]
    fn default() -> Evrmonstat2 {
        <crate::RegValueT<Evrmonstat2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evroscctrl_SPEC;
impl crate::sealed::RegSpec for Evroscctrl_SPEC {
    type DataType = u32;
}
#[doc = "EVR Oscillator Control Register\n resetvalue={LVD Reset:0x1F,After SSW execution:0x2000001F}"]
pub type Evroscctrl = crate::RegValueT<Evroscctrl_SPEC>;

impl Evroscctrl {
    #[doc = "Back up Clock Fine Trim Value   OSCFTRIM. This thermometer coded bit field contains information about the 100MHz        OSC fine trimming. fBACK ftrim     OSCFTRIM    OSCFPTRIM           OSCTRx if OSCTRIMEN 1       LSBFT  MHz  LSBFT   110kHz Back up Clock accuracy is documented in datasheet. It is recommended to        wait 1  160 us after every fine trim step so that the clock source settles at        the new frequency. fBACK ftrim        value is saturated to range of 64."]
    #[inline(always)]
    pub fn oscftrim(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3f,
        1,
        0,
        evroscctrl::Oscftrim,
        Evroscctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3f,
            1,
            0,
            evroscctrl::Oscftrim,
            Evroscctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "OSC Fine Trim Signed Value   OSCFPTRIM. This bit field allows device individual trimming of the oscillator trim        value during application. After updating the trim value  a waiting time        of 1  160 us is required for the change to take effect. OSCTRx        is implicitly added to OSCFPTRIM and saturated to range of  32 to 31"]
    #[inline(always)]
    pub fn oscfptrim(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Evroscctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Evroscctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Oscillator Temperature Offset Coefficient   OSCTEMPOFFS. This bitfield enables the centering function of the HPOSC temperature        coefficient to compensate for technology variations."]
    #[inline(always)]
    pub fn osctempoffs(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        evroscctrl::Osctempoffs,
        Evroscctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            evroscctrl::Osctempoffs,
            Evroscctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Dynamic Oscillator Trim Enable   OSCTRIMEN. Based on temperature  Oscillator can be trimmed."]
    #[inline(always)]
    pub fn osctrimen(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        evroscctrl::Osctrimen,
        Evroscctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            evroscctrl::Osctrimen,
            Evroscctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evroscctrl {
    #[inline(always)]
    fn default() -> Evroscctrl {
        <crate::RegValueT<Evroscctrl_SPEC> as RegisterValue<_>>::new(31)
    }
}
pub mod evroscctrl {
    pub struct Oscftrim_SPEC;
    pub type Oscftrim = crate::EnumBitfieldStruct<u8, Oscftrim_SPEC>;
    impl Oscftrim {
        #[doc = "000000 0 MHz"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "011111 3.65 MHz"]
        pub const CONST_3131: Self = Self::new(31);
        #[doc = "111111 7.3 MHz"]
        pub const CONST_6363: Self = Self::new(63);
    }
    pub struct Osctempoffs_SPEC;
    pub type Osctempoffs = crate::EnumBitfieldStruct<u8, Osctempoffs_SPEC>;
    impl Osctempoffs {
        #[doc = "0 Centering on."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Centering off."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Osctrimen_SPEC;
    pub type Osctrimen = crate::EnumBitfieldStruct<u8, Osctrimen_SPEC>;
    impl Osctrimen {
        #[doc = "0 The Dynamic        Oscillator Trim function is disabled  switched        off."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The Dynamic        Oscillator Trim function is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evroscctrl2_SPEC;
impl crate::sealed::RegSpec for Evroscctrl2_SPEC {
    type DataType = u32;
}
#[doc = "EVR Oscillator Control Register 2\n resetvalue={LVD Reset:0x0B1A05C,After SSW execution:0x0FF4440}"]
pub type Evroscctrl2 = crate::RegValueT<Evroscctrl2_SPEC>;

impl Evroscctrl2 {
    #[doc = "Back up Clock Dynamic Trim Offset Value for Temperature Range 0.   OSCTR0. This bit field contains trim offset information for dynamic trimming of        the oscillator for the temperature range.  0  160   8804   160 DTSSTAT.RESULT  160  lt   160 2048"]
    #[inline(always)]
    pub fn osctr0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Evroscctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Evroscctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Back up Clock Dynamic Trim Offset Value for Temperature Range 1.   OSCTR1. This bit field contains trim offset information for dynamic trimming of        the oscillator for the temperature range.  2048  160   8804   160 DTSSTAT.RESULT  160  lt   160 2304"]
    #[inline(always)]
    pub fn osctr1(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, u8, Evroscctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x7,1,0,u8, Evroscctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Back up Clock Dynamic Trim Offset Value for Temperature Range 2.   OSCTR2. This bit field contains trim offset information for dynamic trimming of        the oscillator for the temperature range.  2304  160   8804   160 DTSSTAT.RESULT  160  lt   160 2560"]
    #[inline(always)]
    pub fn osctr2(
        self,
    ) -> crate::common::RegisterField<6, 0x7, 1, 0, u8, Evroscctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x7,1,0,u8, Evroscctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Back up Clock Dynamic Trim Offset Value for Temperature Range 3.   OSCTR3. This bit field contains trim offset information for dynamic trimming of        the oscillator for the temperature range.  2560  160   8804   160 DTSSTAT.RESULT  160  lt   160 2816"]
    #[inline(always)]
    pub fn osctr3(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, u8, Evroscctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x7,1,0,u8, Evroscctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Back up Clock Dynamic Trim Offset Value for Temperature Range 4.   OSCTR4. This bit field contains trim offset information for dynamic trimming of        the oscillator for the temperature range.  2816  160   8804   160 DTSSTAT.RESULT  160  lt   160 3072"]
    #[inline(always)]
    pub fn osctr4(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, Evroscctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8, Evroscctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Back up Clock Dynamic Trim Offset Value for Temperature Range 5.   OSCTR5. This bit field contains trim offset information for dynamic trimming of        the oscillator for the temperature range.  3072  160   8804   160 DTSSTAT.RESULT  160  lt   160 3328"]
    #[inline(always)]
    pub fn osctr5(
        self,
    ) -> crate::common::RegisterField<15, 0x7, 1, 0, u8, Evroscctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<15,0x7,1,0,u8, Evroscctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Back up Clock Dynamic Trim Offset Value for Temperature Range 6.   OSCTR6. This bit field contains trim offset information for dynamic trimming of        the oscillator for the temperature range.  3328  160   8804   160 DTSSTAT.RESULT  160  lt   160 3584"]
    #[inline(always)]
    pub fn osctr6(
        self,
    ) -> crate::common::RegisterField<18, 0x7, 1, 0, u8, Evroscctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x7,1,0,u8, Evroscctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Back up Clock Dynamic Trim Offset Value for Temperature Range 7.   OSCTR7. This bit field contains trim offset information for dynamic trimming of        the oscillator for the temperature range.  3584  160   8804   160 DTSSTAT.RESULT  160  lt   160 4096"]
    #[inline(always)]
    pub fn osctr7(
        self,
    ) -> crate::common::RegisterField<21, 0x7, 1, 0, u8, Evroscctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x7,1,0,u8, Evroscctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        evroscctrl2::Slck,
        Evroscctrl2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            evroscctrl2::Slck,
            Evroscctrl2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evroscctrl2 {
    #[inline(always)]
    fn default() -> Evroscctrl2 {
        <crate::RegValueT<Evroscctrl2_SPEC> as RegisterValue<_>>::new(11599936)
    }
}
pub mod evroscctrl2 {
    pub struct Slck_SPEC;
    pub type Slck = crate::EnumBitfieldStruct<u8, Slck_SPEC>;
    impl Slck {
        #[doc = "0 No lock active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Lock is active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrovmon_SPEC;
impl crate::sealed::RegSpec for Evrovmon_SPEC {
    type DataType = u32;
}
#[doc = "EVR Secondary Over voltage Monitor Register\n resetvalue={LVD Reset:0x0FEFEFE,Cold PORST:0x0FEFEFE,After SSW execution:0x0FEFEFE}"]
pub type Evrovmon = crate::RegValueT<Evrovmon_SPEC>;

impl Evrovmon {
    #[doc = "VDD Supply Secondary Monitor Over voltage threshold   EVRCOVVAL. This field defines the over voltage monitoring threshold level of the        EVRC regulator output or VDD supply. Ideal Threshold     VIN  160    160 LSB   160    160 1  Ideal LSB   5.7692  160 mV"]
    #[inline(always)]
    pub fn evrcovval(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrovmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrovmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDP3 Supply Secondary Monitor Over voltage threshold   EVR33OVVAL. This field defines the over voltage monitoring threshold level of the        EVR33 regulator output or VDDP3 supply. Ideal Threshold     VIN  160    160 LSB   160    160 1  Ideal LSB   15.00  160 mV"]
    #[inline(always)]
    pub fn evr33ovval(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrovmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrovmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEXT Supply Secondary Monitor Over voltage threshold   SWDOVVAL. This field defines the over voltage threshold level of the external VEXT        supply monitor. Ideal Threshold     VIN  160    160 LSB   160    160 1  Ideal LSB   23.077  160 mV."]
    #[inline(always)]
    pub fn swdovval(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evrovmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evrovmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, evrovmon::Slck, Evrovmon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            evrovmon::Slck,
            Evrovmon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrovmon {
    #[inline(always)]
    fn default() -> Evrovmon {
        <crate::RegValueT<Evrovmon_SPEC> as RegisterValue<_>>::new(16711422)
    }
}
pub mod evrovmon {
    pub struct Slck_SPEC;
    pub type Slck = crate::EnumBitfieldStruct<u8, Slck_SPEC>;
    impl Slck {
        #[doc = "0 No lock active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Lock is active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrovmon2_SPEC;
impl crate::sealed::RegSpec for Evrovmon2_SPEC {
    type DataType = u32;
}
#[doc = "EVR Secondary Over voltage Monitor Register 2\n resetvalue={LVD Reset:0x0FEFEFE,Cold PORST:0x0FEFEFE,After SSW execution:0x0FEFEFE}"]
pub type Evrovmon2 = crate::RegValueT<Evrovmon2_SPEC>;

impl Evrovmon2 {
    #[doc = "VDDPD Supply Secondary Monitor Over voltage threshold   PREOVVAL. This field defines the over voltage monitoring threshold level of the        VDDPD supply or EVRPR output. Ideal Threshold     VIN  160    160 LSB   160    160 1  Ideal LSB   5.7692  160 mV"]
    #[inline(always)]
    pub fn preovval(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrovmon2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrovmon2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDM Supply Secondary Monitor Over voltage threshold   VDDMOVVAL. This field defines the over voltage monitoring threshold level of the        VDDM ADC supply Ideal Threshold     VIN  160    160 LSB   160    160 1  Ideal LSB   23.077  160 mV"]
    #[inline(always)]
    pub fn vddmovval(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrovmon2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrovmon2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEVRSB Supply Secondary Monitor Over voltage threshold   SBOVVAL. This field defines the over voltage threshold level of the external        VEVRSB  3.3V  160    160 5V  standby supply monitor. Ideal Threshold     VIN  160    160 LSB   160    160 1  Ideal LSB   23.077  160 mV"]
    #[inline(always)]
    pub fn sbovval(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evrovmon2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evrovmon2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        evrovmon2::Slck,
        Evrovmon2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            evrovmon2::Slck,
            Evrovmon2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrovmon2 {
    #[inline(always)]
    fn default() -> Evrovmon2 {
        <crate::RegValueT<Evrovmon2_SPEC> as RegisterValue<_>>::new(16711422)
    }
}
pub mod evrovmon2 {
    pub struct Slck_SPEC;
    pub type Slck = crate::EnumBitfieldStruct<u8, Slck_SPEC>;
    impl Slck {
        #[doc = "0 No lock active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Lock is active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrrstcon_SPEC;
impl crate::sealed::RegSpec for Evrrstcon_SPEC {
    type DataType = u32;
}
#[doc = "EVR Reset Control Register\n resetvalue={LVD Reset:0x597F4A,Cold PORST:0x597F4A,After SSW execution:0x5C834B}"]
pub type Evrrstcon = crate::RegValueT<Evrrstcon_SPEC>;

impl Evrrstcon {
    #[doc = "VDD Supply Reset Trim Value   RSTCTRIM. This bit field selects the hard reset generation level of VDD supply        rail. This bit field is trimmed by Firmware. RSTCTRIM  160    160   VDDx   712.5 mV    LSB   if        RSTCPTRIM 0  VDDPRIUV  160    160 712.5 mV   LSB   RSTCTRIM          LSB   RSTCPTRIM  signed value  LSB   5 mV"]
    #[inline(always)]
    pub fn rstctrim(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrrstcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrrstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDP3 Supply Reset Trim Value   RST33TRIM. This bit field selects the hard reset generation level of VDDP3 supply        rail. This bit field is trimmed by Firmware. RST33TRIM     VDDx   937.5 mV    LSB   if        RST33PTRIM 0  VDDP3PRIUV  160    160 937.5 mV   LSB   RST33TRIM   LSB   RST33PTRIM  signed        value  LSB   15 mV"]
    #[inline(always)]
    pub fn rst33trim(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrrstcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrrstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEXT Supply Reset Trim Value   RSTSWDTRIM. This bit field selects the hard reset generation level of the external        VEXT supply rail. This bitfield is trimmed by Firmware. RSTSWDTRIM     VDDx   1050 mV    LSB   if RSTSWDPTRIM 0  VEXTPRIUV   1050 mV  LSB   RSTSWDTRIM          LSB   RSTSWDPTRIM  signed value  LSB   20 mV"]
    #[inline(always)]
    pub fn rstswdtrim(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evrrstcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evrrstcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDD Reset Enable   RSTCOFF. This bit can only be changed if bit BPRSTCOFF is set in parallel.        RSTCOFF is intended to be used only for internal test purposes and the        primary reset generation is not to be disabled in customer application."]
    #[inline(always)]
    pub fn rstcoff(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        evrrstcon::Rstcoff,
        Evrrstcon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            evrrstcon::Rstcoff,
            Evrrstcon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Bit Protection RSTCOFF   BPRSTCOFF. Setting this bit enables that bit RSTCOFF can be changed in this write        operation. This bit is read as zero."]
    #[inline(always)]
    pub fn bprstcoff(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Evrrstcon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25,1,0,Evrrstcon_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "VDDP3 Reset Enable   RST33OFF. This bit can only be changed if bit BPRST33OFF is set in parallel. The        VDDP3 reset is disabled by application to support voltage drop up to        nominal 3.0  160 V during cranking. RST33OFF is intended to be used only for        internal test purposes and the primary reset generation is not to be        disabled in customer application."]
    #[inline(always)]
    pub fn rst33off(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        evrrstcon::Rst33Off,
        Evrrstcon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            evrrstcon::Rst33Off,
            Evrrstcon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Bit Protection RST33OFF   BPRST33OFF. Setting this bit enables that bit RST33OFF can be changed in this write        operation. This bit read also as zero."]
    #[inline(always)]
    pub fn bprst33off(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Evrrstcon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27,1,0,Evrrstcon_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "VEXT Reset Enable   RSTSWDOFF. This bit can only be changed if bit BPRSTSWDOFF is set in parallel.        RSTSWDOFF is intended to be used only for internal test purposes and the        primary reset generation is not to be disabled in customer application."]
    #[inline(always)]
    pub fn rstswdoff(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        evrrstcon::Rstswdoff,
        Evrrstcon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            evrrstcon::Rstswdoff,
            Evrrstcon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Bit Protection RSTSWDOFF   BPRSTSWDOFF. Setting this bit enables that bit RSTSWDOFF can be changed in this write        operation. This bit is read as zero."]
    #[inline(always)]
    pub fn bprstswdoff(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Evrrstcon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29,1,0,Evrrstcon_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        evrrstcon::Slck,
        Evrrstcon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            evrrstcon::Slck,
            Evrrstcon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrrstcon {
    #[inline(always)]
    fn default() -> Evrrstcon {
        <crate::RegValueT<Evrrstcon_SPEC> as RegisterValue<_>>::new(5768010)
    }
}
pub mod evrrstcon {
    pub struct Rstcoff_SPEC;
    pub type Rstcoff = crate::EnumBitfieldStruct<u8, Rstcoff_SPEC>;
    impl Rstcoff {
        #[doc = "0 A reset trigger        signal is generated and forwarded to the SCU by primary monitor        depending on the selected reset trim value ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No reset        trigger signal is generated and forwarded to the SCU by primary monitor        depending on the selected reset trim value ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst33Off_SPEC;
    pub type Rst33Off = crate::EnumBitfieldStruct<u8, Rst33Off_SPEC>;
    impl Rst33Off {
        #[doc = "0 A reset trigger        signal is generated and forwarded to the SCU by primary monitor        depending on the selected reset trim value ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No reset        trigger signal is generated and forwarded to the SCU by primary monitor        depending on the selected reset trim value ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rstswdoff_SPEC;
    pub type Rstswdoff = crate::EnumBitfieldStruct<u8, Rstswdoff_SPEC>;
    impl Rstswdoff {
        #[doc = "0 A reset trigger        signal is generated and forwarded to the SCU by primary monitor        depending on the selected reset trim value ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No reset        trigger signal is generated and forwarded to the SCU by primary        monitordepending on the selected reset trim value ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Slck_SPEC;
    pub type Slck = crate::EnumBitfieldStruct<u8, Slck_SPEC>;
    impl Slck {
        #[doc = "0 No lock active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Lock is active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrrstcon2_SPEC;
impl crate::sealed::RegSpec for Evrrstcon2_SPEC {
    type DataType = u32;
}
#[doc = "EVR Reset Control Register 2\n resetvalue={LVD Reset:0x0,Cold PORST:0x0,After SSW execution:0x0}"]
pub type Evrrstcon2 = crate::RegValueT<Evrrstcon2_SPEC>;

impl Evrrstcon2 {
    #[doc = "VDD Signed Reset Trim Value   RSTCPTRIM. This bit field allows fine trimming of the reset threshold. The first        bit is sign information. The range is  16 to 15 LSB."]
    #[inline(always)]
    pub fn rstcptrim(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, Evrrstcon2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, Evrrstcon2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDP3 Signed Reset Trim Value   RST33PTRIM. This bit field allows fine trimming of the reset threshold. The first        bit is sign information. The range is  16 to 15 LSB."]
    #[inline(always)]
    pub fn rst33ptrim(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, Evrrstcon2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, Evrrstcon2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEXT Signed Reset Trim Value   RSTSWDPTRIM. This bit field allows fine trimming of the reset threshold. The first        bit is sign information. The range is  16 to 15 LSB."]
    #[inline(always)]
    pub fn rstswdptrim(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Evrrstcon2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Evrrstcon2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        evrrstcon2::Slck,
        Evrrstcon2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            evrrstcon2::Slck,
            Evrrstcon2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrrstcon2 {
    #[inline(always)]
    fn default() -> Evrrstcon2 {
        <crate::RegValueT<Evrrstcon2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod evrrstcon2 {
    pub struct Slck_SPEC;
    pub type Slck = crate::EnumBitfieldStruct<u8, Slck_SPEC>;
    impl Slck {
        #[doc = "0 No lock active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Lock is active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrrsthys_SPEC;
impl crate::sealed::RegSpec for Evrrsthys_SPEC {
    type DataType = u32;
}
#[doc = "EVR Reset Hysteresis Control Register\n resetvalue={LVD Reset:0x15040404,Cold PORST:0x15040404,After SSW execution:0x15040404}"]
pub type Evrrsthys = crate::RegValueT<Evrrsthys_SPEC>;

impl Evrrsthys {
    #[doc = "VDD Supply Reset Hysteresis Trim Value   RSTCHYS. This bit field selects the hysteresis offset level of the VDD primary        monitor. cold PORST release level  160    160  EVRRSTCON.RSTCTRIM  160     160 EVRRSTHYS.RSTCHYS          EVRRSTCON2.RSTCPTRIM  signed value  cold PORST assertion level  160    160 EVRRSTCON.RSTCTRIM   EVRRSTCON2.RSTCPTRIM         signed value"]
    #[inline(always)]
    pub fn rstchys(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrrsthys_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrrsthys_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VDDP3 Supply Reset Hysteresis Trim Value   RST33HYS. This bit field selects the hysteresis offset level of the VDDP3 primary        monitor. cold PORST release level  160    160  EVRRSTCON.RST33TRIM  160     160 EVRRSTHYS.RST33HYS          EVRRSTCON2.RST33PTRIM  signed value  cold PORST assertion level  160    160 EVRRSTCON.RST33TRIM   EVRRSTCON2.RST33PTRIM         signed value"]
    #[inline(always)]
    pub fn rst33hys(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrrsthys_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrrsthys_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VEXT Supply Hysteresis Trim Value   RSTSWDHYS. This bit field selects the hysteresis offset level of the external VEXT        primary monitor. cold PORST release level  160    160  EVRRSTCON.RSTSWDTRIM  160     160 EVRRSTHYS.RSTSWDHYS          EVRRSTCON2.RSTSWDPTRIM  signed value  cold PORST assertion level  160    160 EVRRSTCON.RSTSWDTRIM          EVRRSTCON2.RSTSWDPTRIM  signed value"]
    #[inline(always)]
    pub fn rstswdhys(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evrrsthys_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evrrsthys_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VDD Reset Release Hysteresis Enable   RSTCHYSEN"]
    #[inline(always)]
    pub fn rstchysen(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        evrrsthys::Rstchysen,
        Evrrsthys_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            evrrsthys::Rstchysen,
            Evrrsthys_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "VDDP3 Reset Release Hysteresis Enable   RST33HYSEN"]
    #[inline(always)]
    pub fn rst33hysen(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        evrrsthys::Rst33Hysen,
        Evrrsthys_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            evrrsthys::Rst33Hysen,
            Evrrsthys_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "VEXT Reset Release Hysteresis Enable   RSTSWDHYSEN"]
    #[inline(always)]
    pub fn rstswdhysen(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        evrrsthys::Rstswdhysen,
        Evrrsthys_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            evrrsthys::Rstswdhysen,
            Evrrsthys_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrrsthys {
    #[inline(always)]
    fn default() -> Evrrsthys {
        <crate::RegValueT<Evrrsthys_SPEC> as RegisterValue<_>>::new(352584708)
    }
}
pub mod evrrsthys {
    pub struct Rstchysen_SPEC;
    pub type Rstchysen = crate::EnumBitfieldStruct<u8, Rstchysen_SPEC>;
    impl Rstchysen {
        #[doc = "0 Hysteresis for        reset release is not active for VDD rail"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Hysteresis for        reset release is active for VDD rail"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst33Hysen_SPEC;
    pub type Rst33Hysen = crate::EnumBitfieldStruct<u8, Rst33Hysen_SPEC>;
    impl Rst33Hysen {
        #[doc = "0 Hysteresis for        reset release is not active for VDDP3 rail"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Hysteresis for        reset release is active for VDDP3 rail"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rstswdhysen_SPEC;
    pub type Rstswdhysen = crate::EnumBitfieldStruct<u8, Rstswdhysen_SPEC>;
    impl Rstswdhysen {
        #[doc = "0 Hysteresis for        reset release is not active for VEXT rail"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Hysteresis for        reset release is active for VEXT rail"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrrststat_SPEC;
impl crate::sealed::RegSpec for Evrrststat_SPEC {
    type DataType = u32;
}
#[doc = "EVR Reset Status Register\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
pub type Evrrststat = crate::RegValueT<Evrrststat_SPEC>;

impl Evrrststat {
    #[doc = "VDD Supply Reset Value Status   RSTC. This bit field indicates the actual cold PORST reset trim setpoint for        core voltage supply rail used by the Primary monitors. The value is        updated via PMS EVRRSTCON .RSTCTRIM and PMS EVRRSTCON2 .RSTCPTRIM register. RSTC   RSTCTRIM   RSTCPTRIM  signed value  RSTC range  160    160 0 up to 255 VDDPRIUV   712.5 mV   LSB   RSTC LSB   5 mV"]
    #[inline(always)]
    pub fn rstc(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrrststat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrrststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VDDP3 Supply Reset Value Status   RST33. This bit field indicates the actual cold PORST reset trim setpoint for        3.3  160 V supply rail used by the Primary monitors. The value is updated via PMS EVRRSTCON .RST33TRIM and PMS EVRRSTCON2 .RST33PTRIM register. RST33   RST33TRIM   RST33PTRIM  signed value  RST33 range  160    160 0 up to 255 VDDP3PRIUV   937.5 mV   LSB   RST33 LSB   15 mV"]
    #[inline(always)]
    pub fn rst33(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrrststat_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrrststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VEXT Supply Reset Value Status   RSTSWD. This bit field indicates the actual cold PORST reset trim setpoint for        5  160 V supply rail used by the Primary monitors. The value is updated via PMS EVRRSTCON .RSTSWDTRIM and PMS EVRRSTCON2 .        RSTSWDPTRIM register. RSTSWD   RSTSWDTRIM          RSTSWDPTRIM  signed value  RSTSWD range  160    160 0 up to 255 VEXTPRIUV   1050 mV  LSB   RSTSWD LSB   20 mV"]
    #[inline(always)]
    pub fn rstswd(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evrrststat_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evrrststat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EVRC Reset Enable Status   RSTCOFF. The value is updated via PMS EVRRSTCON .RSTCOFF        register bit."]
    #[inline(always)]
    pub fn rstcoff(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        evrrststat::Rstcoff,
        Evrrststat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            evrrststat::Rstcoff,
            Evrrststat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "EVR33 Reset Enable Status   RST33OFF. The value is updated via PMS EVRRSTCON .RST33OFF        register bit."]
    #[inline(always)]
    pub fn rst33off(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        evrrststat::Rst33Off,
        Evrrststat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            evrrststat::Rst33Off,
            Evrrststat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "EVR SWD Reset Enable   RSTSWDOFF. The value is updated via PMS EVRRSTCON .RSTSWDOFF        register bit."]
    #[inline(always)]
    pub fn rstswdoff(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        evrrststat::Rstswdoff,
        Evrrststat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            evrrststat::Rstswdoff,
            Evrrststat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrrststat {
    #[inline(always)]
    fn default() -> Evrrststat {
        <crate::RegValueT<Evrrststat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod evrrststat {
    pub struct Rstcoff_SPEC;
    pub type Rstcoff = crate::EnumBitfieldStruct<u8, Rstcoff_SPEC>;
    impl Rstcoff {
        #[doc = "0 A cold PORST is        triggered incase of VDD primary under voltage event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No cold PORST        is generated incase of a primary under voltage event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst33Off_SPEC;
    pub type Rst33Off = crate::EnumBitfieldStruct<u8, Rst33Off_SPEC>;
    impl Rst33Off {
        #[doc = "0 A cold PORST is        triggered incase of VDDP3 primary under voltage event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No cold PORST        is generated incase of a primary under voltage event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rstswdoff_SPEC;
    pub type Rstswdoff = crate::EnumBitfieldStruct<u8, Rstswdoff_SPEC>;
    impl Rstswdoff {
        #[doc = "0 A cold PORST is        triggered incase of VEXT primary under voltage event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No cold PORST        is generated incase of a primary under voltage event."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdcoeff0_SPEC;
impl crate::sealed::RegSpec for Evrsdcoeff0_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Coefficient Register 0\n resetvalue={LVD Reset:0x0B50873B6,Cold PORST:0x0B50873B6,After SSW execution:0x0B50873B6}"]
pub type Evrsdcoeff0 = crate::RegValueT<Evrsdcoeff0_SPEC>;

impl Evrsdcoeff0 {
    #[doc = "S0 Enable m0en s0en i    M0S0EN. This bitfield enables the fast forward error term."]
    #[inline(always)]
    pub fn m0s0en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Enable m0en s2en i    M0S2EN. This bitfield enables the digital reconstruction of the inductor current."]
    #[inline(always)]
    pub fn m0s2en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Enable m0en s3en i    M0S3EN. This bitfield enables the integrator."]
    #[inline(always)]
    pub fn m0s3en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Clip m0en s3clip i    M0S3CLIP. This bitfield specifies the clipping of the integrator state to negative        values."]
    #[inline(always)]
    pub fn m0s3clip(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S4 Enable m0en s4en i    M0S4EN. This bitfield enables the double integrator branch."]
    #[inline(always)]
    pub fn m0s4en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Ramp Enable m0en rampen i    M0RAMPEN. This bitfield enables the artificial ramp in order to avoid        instabilities at high duty cycles."]
    #[inline(always)]
    pub fn m0rampen(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SFRGET m0en sfrget i    M0SFRGET. This bitfield enables the compensation of parasitic effects in the        inductor current reconstruction."]
    #[inline(always)]
    pub fn m0sfrget(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Skip Enable m0en skipen i    M0SKIPEN. This bitfield enables the skip pulse logic."]
    #[inline(always)]
    pub fn m0skipen(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Coefficient m0s3 coeff i    M0S3COEFF. Configuration register of S3   integrator coefficient."]
    #[inline(always)]
    pub fn m0s3coeff(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S4 Coefficient m0s4 coeff i    M0S4COEFF. Configuration register of S4   double integrator coefficient."]
    #[inline(always)]
    pub fn m0s4coeff(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S Ramp Coefficient m0srmp coeff i    M0SRMPCOEFF. Configuration register of S Ramp   artificial ramp coefficient."]
    #[inline(always)]
    pub fn m0srmpcoeff(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Forgetting Factor m0fget coeff i    M0FGETCOEFF. This bitfield specifies the forgetting factor for compensation of        parasitic effects."]
    #[inline(always)]
    pub fn m0fgetcoeff(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8, Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Coefficient m0s2 coeff i    M0S2COEFF. Inductor current reconstruction coefficient."]
    #[inline(always)]
    pub fn m0s2coeff(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Evrsdcoeff0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Evrsdcoeff0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Vin Source m0s2 vinsrc i    M0S2VINSRC. This bitfield specifies the source of the input voltage used for the        inductor current reconstruction."]
    #[inline(always)]
    pub fn m0s2vinsrc(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        evrsdcoeff0::M0S2Vinsrc,
        Evrsdcoeff0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            evrsdcoeff0::M0S2Vinsrc,
            Evrsdcoeff0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "S2 Vout Source m0s2 vosrc i    M0S2VOSRC. This bitfield specifies the source of the output voltage used for the        inductor current reconstruction."]
    #[inline(always)]
    pub fn m0s2vosrc(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        evrsdcoeff0::M0S2Vosrc,
        Evrsdcoeff0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            evrsdcoeff0::M0S2Vosrc,
            Evrsdcoeff0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "S Ramp Fractional Coefficient. This bitfield specifies the S Ramp fractional coefficient."]
    #[inline(always)]
    pub fn m0srmpcoefffrac(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        evrsdcoeff0::M0Srmpcoefffrac,
        Evrsdcoeff0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            evrsdcoeff0::M0Srmpcoefffrac,
            Evrsdcoeff0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        evrsdcoeff0::Lck,
        Evrsdcoeff0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            evrsdcoeff0::Lck,
            Evrsdcoeff0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrsdcoeff0 {
    #[inline(always)]
    fn default() -> Evrsdcoeff0 {
        <crate::RegValueT<Evrsdcoeff0_SPEC> as RegisterValue<_>>::new(3037230006)
    }
}
pub mod evrsdcoeff0 {
    pub struct M0S2Vinsrc_SPEC;
    pub type M0S2Vinsrc = crate::EnumBitfieldStruct<u8, M0S2Vinsrc_SPEC>;
    impl M0S2Vinsrc {
        #[doc = "0 The        register value M0VIN is used."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The FF ADC        counter value is used"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct M0S2Vosrc_SPEC;
    pub type M0S2Vosrc = crate::EnumBitfieldStruct<u8, M0S2Vosrc_SPEC>;
    impl M0S2Vosrc {
        #[doc = "0 The        register value M0VO is used."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The FB ADC        counter value is used"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct M0Srmpcoefffrac_SPEC;
    pub type M0Srmpcoefffrac = crate::EnumBitfieldStruct<u8, M0Srmpcoefffrac_SPEC>;
    impl M0Srmpcoefffrac {
        #[doc = "0 no        fractional coefficient used."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 fractional        coefficient 1 2 used  SRMP   0.5 ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lck_SPEC;
    pub type Lck = crate::EnumBitfieldStruct<u8, Lck_SPEC>;
    impl Lck {
        #[doc = "0 The        register is unlocked and can be updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The register is        locked and cannot be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdcoeff1_SPEC;
impl crate::sealed::RegSpec for Evrsdcoeff1_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Coefficient Register 1\n resetvalue={LVD Reset:0x0A2946C46,Cold PORST:0x0A2946C46,After SSW execution:0x0A2946C46}"]
pub type Evrsdcoeff1 = crate::RegValueT<Evrsdcoeff1_SPEC>;

impl Evrsdcoeff1 {
    #[doc = "LPF Coefficient m0vocf lpf i    M0VOCFLPF. This bitfield reflects LPF coefficient used in the LPF applied to the        FB ADC counter value or the programmed register value. y  160  k   160    160    160 y  160  k 1   160    160  1 a   160    160    160    160 x  160  k   160    160 a   160   y  160  k  is filter output         x  160  k  is ADC output a  160    160  1  160    160  2  160    160 LPF  .If LPF  160    160 0  the filter output is the same as ADC        output."]
    #[inline(always)]
    pub fn m0vocflpf(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Evrsdcoeff1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Evrsdcoeff1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Output Voltage Ramp Coefficient m0vocf inc i    M0VOCFINC. This bitfield reflects increment for the output voltage ramp used in the        inductor current reconstruction. Step applied to ramp   2  160    160 M0VOCFINC."]
    #[inline(always)]
    pub fn m0vocfinc(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Evrsdcoeff1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Evrsdcoeff1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Digital representation of the target voltage m0vo lb i    M0VOUT. This bitfield can be used for the inductor current reconstruction        instead of the FBADC value."]
    #[inline(always)]
    pub fn m0vout(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdcoeff1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdcoeff1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Digital representation of the input voltage m0vinh vin i m0vinl vin i     M0VIN. This bitfield is used for the inductor current reconstruction instead of        the FFADC value. Absolute value including ADC offset."]
    #[inline(always)]
    pub fn m0vin(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, Evrsdcoeff1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16, Evrsdcoeff1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Fractional Coefficient. This bitfield specifies the S3 fractional integrator coefficient. 00   no fractional coefficient used 01 ... fractional coefficient 1 4 used  S3   0.25  10 ... fractional coefficient 1 2 used  S3   0.5  11 ... fractional coefficient 3 4 used  S3   0.75"]
    #[inline(always)]
    pub fn m0s3coefffrac(
        self,
    ) -> crate::common::RegisterField<27, 0x3, 1, 0, u8, Evrsdcoeff1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x3,1,0,u8, Evrsdcoeff1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Fractional Coefficient. This bitfield specifies the S2 fractional coefficient of the inductor        current reconstruction coefficient. 00   no fractional coefficient used 01 ... fractional coefficient 1 4 used  S2   0.25  10 ... fractional coefficient 1 2 used  S2   0.5  11 ... fractional coefficient 3 4 used  S2   0.75"]
    #[inline(always)]
    pub fn m0s2coefffrac(
        self,
    ) -> crate::common::RegisterField<29, 0x3, 1, 0, u8, Evrsdcoeff1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x3,1,0,u8, Evrsdcoeff1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        evrsdcoeff1::Lck,
        Evrsdcoeff1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            evrsdcoeff1::Lck,
            Evrsdcoeff1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrsdcoeff1 {
    #[inline(always)]
    fn default() -> Evrsdcoeff1 {
        <crate::RegValueT<Evrsdcoeff1_SPEC> as RegisterValue<_>>::new(2727636038)
    }
}
pub mod evrsdcoeff1 {
    pub struct Lck_SPEC;
    pub type Lck = crate::EnumBitfieldStruct<u8, Lck_SPEC>;
    impl Lck {
        #[doc = "0 The        register is unlocked and can be updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The register is        locked and cannot be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdcoeff2_SPEC;
impl crate::sealed::RegSpec for Evrsdcoeff2_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Coefficient Register 2\n resetvalue={LVD Reset:0x3408710E,Cold PORST:0x3408710E,After SSW execution:0x3408710E}"]
pub type Evrsdcoeff2 = crate::RegValueT<Evrsdcoeff2_SPEC>;

impl Evrsdcoeff2 {
    #[doc = "S0 Enable m1en s0en i    M1S0EN. This bitfield enables the fast forward error term."]
    #[inline(always)]
    pub fn m1s0en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Enable m1en s2en i    M1S2EN. This bitfield enables the digital reconstruction of the inductor current."]
    #[inline(always)]
    pub fn m1s2en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Enable m1en s3en i    M1S3EN. This bitfield enables the integrator."]
    #[inline(always)]
    pub fn m1s3en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Clip m1en s3clip i    M1S3CLIP. This bitfield specifies the clipping of the integrator state to negative        values."]
    #[inline(always)]
    pub fn m1s3clip(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S4 Enable m1en s4en i    M1S4EN. This bitfield enables the double integrator branch."]
    #[inline(always)]
    pub fn m1s4en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Ramp Enable m1en rampen i    M1RAMPEN. This bitfield enables the artificial ramp in order to avoid        instabilities at high duty cycles."]
    #[inline(always)]
    pub fn m1rampen(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SFRGET m1en sfrget i    M1SFRGET. This bitfield enables the compensation of parasitic effects in the        inductor current reconstruction."]
    #[inline(always)]
    pub fn m1sfrget(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Skip Enable m1en skipen i    M1SKIPEN. This bitfield enables the skip pulse logic."]
    #[inline(always)]
    pub fn m1skipen(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Coefficient m1s3 coeff i    M1S3COEFF. Configuration register of S3   integrator coefficient."]
    #[inline(always)]
    pub fn m1s3coeff(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S4 Coefficient m1s4 coeff i    M1S4COEFF. Configuration register of S4   double integrator coefficient."]
    #[inline(always)]
    pub fn m1s4coeff(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S Ramp Coefficient m1srmp coeff i    M1SRMPCOEFF. Configuration register of S Ramp   artificial ramp coefficient."]
    #[inline(always)]
    pub fn m1srmpcoeff(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Forgetting Factor m1fget coeff i    M1FGETCOEFF. This bitfield specifies the forgetting factor for compensation of        parasitic effects."]
    #[inline(always)]
    pub fn m1fgetcoeff(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8, Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Coefficient m1s2 coeff i    M1S2COEFF. Inductor current reconstruction coefficient."]
    #[inline(always)]
    pub fn m1s2coeff(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Evrsdcoeff2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Evrsdcoeff2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Vin Source m1s2 vinsrc i    M1S2VINSRC. This bitfield specifies the source of the input voltage used for the        inductor current reconstruction."]
    #[inline(always)]
    pub fn m1s2vinsrc(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        evrsdcoeff2::M1S2Vinsrc,
        Evrsdcoeff2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            evrsdcoeff2::M1S2Vinsrc,
            Evrsdcoeff2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "S2 Vout Source m1s2 vosrc i    M1S2VOSRC. This bitfield specifies the source of the output voltage used for the        inductor current reconstruction."]
    #[inline(always)]
    pub fn m1s2vosrc(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        evrsdcoeff2::M1S2Vosrc,
        Evrsdcoeff2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            evrsdcoeff2::M1S2Vosrc,
            Evrsdcoeff2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrsdcoeff2 {
    #[inline(always)]
    fn default() -> Evrsdcoeff2 {
        <crate::RegValueT<Evrsdcoeff2_SPEC> as RegisterValue<_>>::new(872968462)
    }
}
pub mod evrsdcoeff2 {
    pub struct M1S2Vinsrc_SPEC;
    pub type M1S2Vinsrc = crate::EnumBitfieldStruct<u8, M1S2Vinsrc_SPEC>;
    impl M1S2Vinsrc {
        #[doc = "0 The        register value M1VIN is used."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The FF ADC        counter value is used"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct M1S2Vosrc_SPEC;
    pub type M1S2Vosrc = crate::EnumBitfieldStruct<u8, M1S2Vosrc_SPEC>;
    impl M1S2Vosrc {
        #[doc = "0 The        register value M1VO is used."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The FB ADC        counter value is used"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdcoeff3_SPEC;
impl crate::sealed::RegSpec for Evrsdcoeff3_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Coefficient Register 3\n resetvalue={LVD Reset:0x2946C44,Cold PORST:0x2946C44,After SSW execution:0x2946C44}"]
pub type Evrsdcoeff3 = crate::RegValueT<Evrsdcoeff3_SPEC>;

impl Evrsdcoeff3 {
    #[doc = "LPF Coefficient m1vocf lpf i    M1VOCFLPF. This bitfield reflects LPF coefficient used in the LPF applied to the        FB ADC counter value or the programmed register value. y  160  k   160    160    160 y  160  k 1   160    160  1 a   160    160    160    160 x  160  k   160    160 a   160   y  160  k  is filter output         x  160  k  is ADC output a  160    160  1  160    160  2  160    160 LPF  .If LPF  160    160 0  the filter output is the same as ADC        output."]
    #[inline(always)]
    pub fn m1vocflpf(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Evrsdcoeff3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Evrsdcoeff3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Output Voltage Ramp Coefficient m1vocf inc i    M1VOCFINC. This bitfield reflects increment for the output voltage ramp used in the        inductor current reconstruction. Step applied to ramp   2  160    160 M1VOCFINC."]
    #[inline(always)]
    pub fn m1vocfinc(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Evrsdcoeff3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Evrsdcoeff3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Digital representation of the target voltage m1vo lb i    M1VOUT. This bitfield can be used for the inductor current reconstruction        instead of the FBADC value."]
    #[inline(always)]
    pub fn m1vout(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdcoeff3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdcoeff3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Digital representation of the input voltage m1vinh vin i m1vinl vin i    M1VIN. This bitfield can be used for the inductor current reconstruction        instead of the FFADC value. Absolute value including ADC offset."]
    #[inline(always)]
    pub fn m1vin(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, Evrsdcoeff3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16, Evrsdcoeff3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Fractional Coefficient. This bitfield specifies the S3 fractional integrator coefficient. 00   no fractional coefficient used 01 ... fractional coefficient 1 4 used  S3   0.25  10 ... fractional coefficient 1 2 used  S3   0.5  11 ... fractional coefficient 3 4 used  S3   0.75"]
    #[inline(always)]
    pub fn m1s3coefffrac(
        self,
    ) -> crate::common::RegisterField<27, 0x3, 1, 0, u8, Evrsdcoeff3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x3,1,0,u8, Evrsdcoeff3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Fractional Coefficient. This bitfield specifies the S2 fractional coefficient of the inductor        current reconstruction coefficient. 00   no fractional coefficient used 01 ... fractional coefficient 1 4 used  S2   0.25  10 ... fractional coefficient 1 2 used  S2   0.5  11 ... fractional coefficient 3 4 used  S2   0.75"]
    #[inline(always)]
    pub fn m1s2coefffrac(
        self,
    ) -> crate::common::RegisterField<29, 0x3, 1, 0, u8, Evrsdcoeff3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x3,1,0,u8, Evrsdcoeff3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S Ramp Fractional Coefficient. This bitfield specifies the S Ramp fractional coefficient."]
    #[inline(always)]
    pub fn m1srmpcoefffrac(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        evrsdcoeff3::M1Srmpcoefffrac,
        Evrsdcoeff3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            evrsdcoeff3::M1Srmpcoefffrac,
            Evrsdcoeff3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrsdcoeff3 {
    #[inline(always)]
    fn default() -> Evrsdcoeff3 {
        <crate::RegValueT<Evrsdcoeff3_SPEC> as RegisterValue<_>>::new(43281476)
    }
}
pub mod evrsdcoeff3 {
    pub struct M1Srmpcoefffrac_SPEC;
    pub type M1Srmpcoefffrac = crate::EnumBitfieldStruct<u8, M1Srmpcoefffrac_SPEC>;
    impl M1Srmpcoefffrac {
        #[doc = "0 no        fractional coefficient used"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 fractional        coefficient 1 2 used  SRMP   0.5 ."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdcoeff4_SPEC;
impl crate::sealed::RegSpec for Evrsdcoeff4_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Coefficient Register 4\n resetvalue={LVD Reset:0x1B0822B6,Cold PORST:0x1B0822B6,After SSW execution:0x1B0822B6}"]
pub type Evrsdcoeff4 = crate::RegValueT<Evrsdcoeff4_SPEC>;

impl Evrsdcoeff4 {
    #[doc = "S0 Enable m2en s0en i    M2S0EN. This bitfield enables the fast forward error term."]
    #[inline(always)]
    pub fn m2s0en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Enable m2en s2en i    M2S2EN. This bitfield enables the digital reconstruction of the inductor current."]
    #[inline(always)]
    pub fn m2s2en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Enable m2en s3en i    M2S3EN. This bitfield enables the integrator."]
    #[inline(always)]
    pub fn m2s3en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Clip m2en s3clip i    M2S3CLIP. This bitfield specifies the clipping of the integrator state to negative        values."]
    #[inline(always)]
    pub fn m2s3clip(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S4 Enable m2en s4en i    M2S4EN. This bitfield enables the double integrator branch."]
    #[inline(always)]
    pub fn m2s4en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Ramp Enable m2en rampen i    M2RAMPEN. This bitfield enables the artificial ramp in order to avoid        instabilities at high duty cycles."]
    #[inline(always)]
    pub fn m2rampen(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SFRGET m2en sfrget i    M2SFRGET. This bitfield enables the compensation of parasitic effects in the        inductor current reconstruction."]
    #[inline(always)]
    pub fn m2sfrget(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Skip Enable m2en skipen i    M2SKIPEN. This bitfield enables the skip pulse logic."]
    #[inline(always)]
    pub fn m2skipen(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Coefficient m2s3 coeff i    M2S3COEFF. Configuration register of S3   integrator coefficient."]
    #[inline(always)]
    pub fn m2s3coeff(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S4 Coefficient m2s4 coeff i    M2S4COEFF. Configuration register of S4   double integrator coefficient."]
    #[inline(always)]
    pub fn m2s4coeff(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S Ramp Coefficient m2srmp coeff i    M2SRMPCOEFF. Configuration register of S Ramp   artificial ramp coefficient."]
    #[inline(always)]
    pub fn m2srmpcoeff(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Forgetting Factor m2fget coeff i    M2FGETCOEFF. This bitfield specifies the forgetting factor for compensation of        parasitic effects."]
    #[inline(always)]
    pub fn m2fgetcoeff(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8, Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Coefficient m2s2 coeff i    M2S2COEFF. Inductor current reconstruction coefficient."]
    #[inline(always)]
    pub fn m2s2coeff(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Evrsdcoeff4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Evrsdcoeff4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Vin Source m2s2 vinsrc i    M2S2VINSRC. This bitfield specifies the source of the input voltage used for the        inductor current reconstruction."]
    #[inline(always)]
    pub fn m2s2vinsrc(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        evrsdcoeff4::M2S2Vinsrc,
        Evrsdcoeff4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            evrsdcoeff4::M2S2Vinsrc,
            Evrsdcoeff4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "S2 Vout Source m2s2 vosrc i    M2S2VOSRC. This bitfield specifies the source of the output voltage used for the        inductor current reconstruction."]
    #[inline(always)]
    pub fn m2s2vosrc(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        evrsdcoeff4::M2S2Vosrc,
        Evrsdcoeff4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            evrsdcoeff4::M2S2Vosrc,
            Evrsdcoeff4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrsdcoeff4 {
    #[inline(always)]
    fn default() -> Evrsdcoeff4 {
        <crate::RegValueT<Evrsdcoeff4_SPEC> as RegisterValue<_>>::new(453518006)
    }
}
pub mod evrsdcoeff4 {
    pub struct M2S2Vinsrc_SPEC;
    pub type M2S2Vinsrc = crate::EnumBitfieldStruct<u8, M2S2Vinsrc_SPEC>;
    impl M2S2Vinsrc {
        #[doc = "0 The        register value M2VIN is used."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The FF ADC        counter value is used"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct M2S2Vosrc_SPEC;
    pub type M2S2Vosrc = crate::EnumBitfieldStruct<u8, M2S2Vosrc_SPEC>;
    impl M2S2Vosrc {
        #[doc = "0 The        register value M2VO is used."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The FB ADC        counter value is used"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdcoeff5_SPEC;
impl crate::sealed::RegSpec for Evrsdcoeff5_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Coefficient Register 5\n resetvalue={LVD Reset:0x2946C46,Cold PORST:0x2946C46,After SSW execution:0x2946C46}"]
pub type Evrsdcoeff5 = crate::RegValueT<Evrsdcoeff5_SPEC>;

impl Evrsdcoeff5 {
    #[doc = "LPF Coefficient m2vocf lpf i    M2VOCFLPF. This bitfield reflects LPF coefficient used in the LPF applied to the        FB ADC counter value or the programmed register value. y  160  k   160    160    160 y  160  k 1   160    160  1 a   160    160    160    160 x  160  k   160    160 a   160   y  160  k  is filter output         x  160  k  is ADC output a  160    160  1  160    160  2  160    160 LPF  .If LPF  160    160 0  the filter output is the same as ADC        output."]
    #[inline(always)]
    pub fn m2vocflpf(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Evrsdcoeff5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Evrsdcoeff5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Output Voltage Ramp Coefficient m2vocf inc i    M2VOCFINC. This bitfield reflects the increment for the output voltage ramp used in        the inductor current reconstruction. Step applied to ramp   2  160    160 M2VOCFINC."]
    #[inline(always)]
    pub fn m2vocfinc(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Evrsdcoeff5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Evrsdcoeff5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Digital representation of the target voltage m2vo lb i    M2VOUT. This bitfield can be used for the inductor current reconstruction        instead of the FBADC value."]
    #[inline(always)]
    pub fn m2vout(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdcoeff5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdcoeff5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Digital representation of the input voltage m2vinh vin i m2vinl vin i    M2VIN. This bitfield can be used for the inductor current reconstruction        instead of the FFADC value. Absolute value including ADC offset."]
    #[inline(always)]
    pub fn m2vin(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, Evrsdcoeff5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x7ff,1,0,u16, Evrsdcoeff5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S3 Fractional Coefficient. This bitfield specifies the S3 fractional integrator coefficient. 00   no fractional coefficient used 01 ... fractional coefficient 1 4 used  S3   0.25  10 ... fractional coefficient 1 2 used  S3   0.5  11 ... fractional coefficient 3 4 used  S3   0.75"]
    #[inline(always)]
    pub fn m2s3coefffrac(
        self,
    ) -> crate::common::RegisterField<27, 0x3, 1, 0, u8, Evrsdcoeff5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x3,1,0,u8, Evrsdcoeff5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S2 Fractional Coefficient. This bitfield specifies the S2 fractional coefficient of the inductor        current reconstruction coefficient. 00   no fractional coefficient used 01 ... fractional coefficient 1 4 used  S2   0.25  10 ... fractional coefficient 1 2 used  S2   0.5  11 ... fractional coefficient 3 4 used  S2   0.75"]
    #[inline(always)]
    pub fn m2s2coefffrac(
        self,
    ) -> crate::common::RegisterField<29, 0x3, 1, 0, u8, Evrsdcoeff5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x3,1,0,u8, Evrsdcoeff5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S Ramp Fractional Coefficient. This bitfield specifies the S Ramp fractional coefficient."]
    #[inline(always)]
    pub fn m2srmpcoefffrac(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        evrsdcoeff5::M2Srmpcoefffrac,
        Evrsdcoeff5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            evrsdcoeff5::M2Srmpcoefffrac,
            Evrsdcoeff5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrsdcoeff5 {
    #[inline(always)]
    fn default() -> Evrsdcoeff5 {
        <crate::RegValueT<Evrsdcoeff5_SPEC> as RegisterValue<_>>::new(43281478)
    }
}
pub mod evrsdcoeff5 {
    pub struct M2Srmpcoefffrac_SPEC;
    pub type M2Srmpcoefffrac = crate::EnumBitfieldStruct<u8, M2Srmpcoefffrac_SPEC>;
    impl M2Srmpcoefffrac {
        #[doc = "0 no        fractional coefficient used"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 fractional        coefficient 1 2 used  SRMP   0.5 ."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdcoeff6_SPEC;
impl crate::sealed::RegSpec for Evrsdcoeff6_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Coefficient Register 6\n resetvalue={LVD Reset:0x080971802,Cold PORST:0x080971802,After SSW execution:0x080971802}"]
pub type Evrsdcoeff6 = crate::RegValueT<Evrsdcoeff6_SPEC>;

impl Evrsdcoeff6 {
    #[doc = "Commutation trimming and Slope Control drv5v0 trim i    CT5REG0. Trimming of the commutation parameters of the external driver  5V ."]
    #[inline(always)]
    pub fn ct5reg0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdcoeff6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdcoeff6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Commutation trimming drv5v1 trim i    CT5REG1. Trimming of the commutation parameters of the external driver  5V ."]
    #[inline(always)]
    pub fn ct5reg1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdcoeff6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdcoeff6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Commutation trimming drv5v2 trim i    CT5REG2. Trimming of the commutation parameters of the external driver  5V ."]
    #[inline(always)]
    pub fn ct5reg2(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evrsdcoeff6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evrsdcoeff6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        evrsdcoeff6::Lck,
        Evrsdcoeff6_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            evrsdcoeff6::Lck,
            Evrsdcoeff6_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrsdcoeff6 {
    #[inline(always)]
    fn default() -> Evrsdcoeff6 {
        <crate::RegValueT<Evrsdcoeff6_SPEC> as RegisterValue<_>>::new(2157385730)
    }
}
pub mod evrsdcoeff6 {
    pub struct Lck_SPEC;
    pub type Lck = crate::EnumBitfieldStruct<u8, Lck_SPEC>;
    impl Lck {
        #[doc = "0 The        register is unlocked and can be updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The register is        locked and cannot be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdcoeff7_SPEC;
impl crate::sealed::RegSpec for Evrsdcoeff7_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Coefficient Register 7\n resetvalue={LVD Reset:0x08000D8F7,Cold PORST:0x08000D8F7,After SSW execution:0x08000D8F7}"]
pub type Evrsdcoeff7 = crate::RegValueT<Evrsdcoeff7_SPEC>;

impl Evrsdcoeff7 {
    #[doc = "Commutation trimming drv5v3 trim i    CT5REG3. Trimming of the commutation parameters of the external driver  5V ."]
    #[inline(always)]
    pub fn ct5reg3(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdcoeff7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdcoeff7_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Commutation trimming drv5v4 trim i    CT5REG4. Trimming of the commutation parameters of the external driver  5V ."]
    #[inline(always)]
    pub fn ct5reg4(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdcoeff7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdcoeff7_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        evrsdcoeff7::Lck,
        Evrsdcoeff7_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            evrsdcoeff7::Lck,
            Evrsdcoeff7_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrsdcoeff7 {
    #[inline(always)]
    fn default() -> Evrsdcoeff7 {
        <crate::RegValueT<Evrsdcoeff7_SPEC> as RegisterValue<_>>::new(2147539191)
    }
}
pub mod evrsdcoeff7 {
    pub struct Lck_SPEC;
    pub type Lck = crate::EnumBitfieldStruct<u8, Lck_SPEC>;
    impl Lck {
        #[doc = "0 The        register is unlocked and can be updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The register is        locked and cannot be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdcoeff8_SPEC;
impl crate::sealed::RegSpec for Evrsdcoeff8_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Coefficient Register 8\n resetvalue={LVD Reset:0x080171002,Cold PORST:0x080171002,After SSW execution:0x080171002}"]
pub type Evrsdcoeff8 = crate::RegValueT<Evrsdcoeff8_SPEC>;

impl Evrsdcoeff8 {
    #[doc = "Commutation trimming drv3v0 trim i    CT33REG0. Trimming of the commutation parameters of the external driver  3.3V ."]
    #[inline(always)]
    pub fn ct33reg0(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdcoeff8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdcoeff8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Commutation trimming drv3v1 trim i    CT33REG1. Trimming of the commutation parameters of the external driver  3.3V ."]
    #[inline(always)]
    pub fn ct33reg1(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdcoeff8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdcoeff8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Commutation trimming drv3v2 trim i    CT33REG2. Trimming of the commutation parameters of the external driver  3.3V ."]
    #[inline(always)]
    pub fn ct33reg2(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evrsdcoeff8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evrsdcoeff8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        evrsdcoeff8::Lck,
        Evrsdcoeff8_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            evrsdcoeff8::Lck,
            Evrsdcoeff8_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrsdcoeff8 {
    #[inline(always)]
    fn default() -> Evrsdcoeff8 {
        <crate::RegValueT<Evrsdcoeff8_SPEC> as RegisterValue<_>>::new(2148995074)
    }
}
pub mod evrsdcoeff8 {
    pub struct Lck_SPEC;
    pub type Lck = crate::EnumBitfieldStruct<u8, Lck_SPEC>;
    impl Lck {
        #[doc = "0 The        register is unlocked and can be updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The register is        locked and cannot be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdcoeff9_SPEC;
impl crate::sealed::RegSpec for Evrsdcoeff9_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Coefficient Register 9\n resetvalue={LVD Reset:0x08000A0AF,Cold PORST:0x08000A0AF,After SSW execution:0x08000A0AF}"]
pub type Evrsdcoeff9 = crate::RegValueT<Evrsdcoeff9_SPEC>;

impl Evrsdcoeff9 {
    #[doc = "Commutation trimming drv3v3 trim i    CT33REG3. Trimming of the commutation parameters of the external driver  3.3V ."]
    #[inline(always)]
    pub fn ct33reg3(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdcoeff9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdcoeff9_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Commutation trimming drv3v4 trim i    CT33REG4. Trimming of the commutation parameters of the external driver  3.3V ."]
    #[inline(always)]
    pub fn ct33reg4(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdcoeff9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdcoeff9_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        evrsdcoeff9::Lck,
        Evrsdcoeff9_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            evrsdcoeff9::Lck,
            Evrsdcoeff9_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrsdcoeff9 {
    #[inline(always)]
    fn default() -> Evrsdcoeff9 {
        <crate::RegValueT<Evrsdcoeff9_SPEC> as RegisterValue<_>>::new(2147524783)
    }
}
pub mod evrsdcoeff9 {
    pub struct Lck_SPEC;
    pub type Lck = crate::EnumBitfieldStruct<u8, Lck_SPEC>;
    impl Lck {
        #[doc = "0 The        register is unlocked and can be updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The register is        locked and cannot be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl0_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl0_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 0\n resetvalue={LVD Reset:0x0F0390001,Cold PORST:0x0F0390001,After SSW execution:0x0F0390001}"]
pub type Evrsdctrl0 = crate::RegValueT<Evrsdctrl0_SPEC>;

impl Evrsdctrl0 {
    #[doc = "Frequency Spread Threshold freqsp coeff i    SDFREQSPRD. This bit field defines the additional frequency spread to the nominal        EVRC regulator switching frequency during operation"]
    #[inline(always)]
    pub fn sdfreqsprd(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Evrsdctrl0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Evrsdctrl0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Regulator Switching Frequency or Over sampling Factor m0osfl fact i m0osfh fact i    SDFREQ. This bit field configures the EVRC regulator switching frequency during        closed loop operation.The switching frequency is equal to  100  160 MHz           SDFREQ 1   value. SDFREQ represents the corresponding over sampling        factor or clock cycles in a period."]
    #[inline(always)]
    pub fn sdfreq(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xfff,
        1,
        0,
        evrsdctrl0::Sdfreq,
        Evrsdctrl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xfff,
            1,
            0,
            evrsdctrl0::Sdfreq,
            Evrsdctrl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "NMOS level during OFF state drvslo ngoff i    NGOFF. This bit field configures the state of N ch. MOSFET driver during        start up and shut down phases."]
    #[inline(always)]
    pub fn ngoff(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        evrsdctrl0::Ngoff,
        Evrsdctrl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            evrsdctrl0::Ngoff,
            Evrsdctrl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "PMOS level during OFF state drvslo pgoff i    PGOFF. This bitfield configures the state of Pch. MOSFET driver during start up        and shut down phases."]
    #[inline(always)]
    pub fn pgoff(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        evrsdctrl0::Pgoff,
        Evrsdctrl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            evrsdctrl0::Pgoff,
            Evrsdctrl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Update request for SMPS register values   UP. This bitfield triggers the update of the current register values from        PMS FPI EVRC registers to the local SMPS module registers. It shall be ensured that ALL EVRSDCTRLx and EVRSDCOEFFx registers have        correct and coherent values across the various registers before the        update request is issued. Incase of singular register update  the other        register values should match and be consistent. After a cold PORST  the        UP bit is set as default reset value to ensure that the complete SMPS        regulator parameter set is set back to its reset state. Consequently         the UP bit is reset and a read delivers 0. The parameter update via UP        bit is not allowed in start up and low power mode."]
    #[inline(always)]
    pub fn up(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        evrsdctrl0::Up,
        Evrsdctrl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            evrsdctrl0::Up,
            Evrsdctrl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When UP bit is set to 1  register is locked for shadow register update        and LCK bit is set to 1. Once shadow register update is done  the lock        is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        evrsdctrl0::Lck,
        Evrsdctrl0_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            evrsdctrl0::Lck,
            Evrsdctrl0_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrsdctrl0 {
    #[inline(always)]
    fn default() -> Evrsdctrl0 {
        <crate::RegValueT<Evrsdctrl0_SPEC> as RegisterValue<_>>::new(4030267393)
    }
}
pub mod evrsdctrl0 {
    pub struct Sdfreq_SPEC;
    pub type Sdfreq = crate::EnumBitfieldStruct<u8, Sdfreq_SPEC>;
    impl Sdfreq {
        #[doc = "7C   160   160 0.8  160 MHz         100 MHz  124 1   SMPS switching frequency"]
        pub const CONST_125125: Self = Self::new(125);
        #[doc = "36   160   160 1.82  160 MHz         100 MHz  54 1   SMPS switching frequency"]
        pub const CONST_5555: Self = Self::new(55);
    }
    pub struct Ngoff_SPEC;
    pub type Ngoff = crate::EnumBitfieldStruct<u8, Ngoff_SPEC>;
    impl Ngoff {
        #[doc = "0 TRISTATE"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 LOW"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pgoff_SPEC;
    pub type Pgoff = crate::EnumBitfieldStruct<u8, Pgoff_SPEC>;
    impl Pgoff {
        #[doc = "0 HIGH"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 TRISTATE"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Up_SPEC;
    pub type Up = crate::EnumBitfieldStruct<u8, Up_SPEC>;
    impl Up {
        #[doc = "0 No        action is undertaken."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A new complete        EVRC parameter set is transferred to the SMPS module. All EVRSDCTRLx and        EVRSDCOEFFx register contents are transferred."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lck_SPEC;
    pub type Lck = crate::EnumBitfieldStruct<u8, Lck_SPEC>;
    impl Lck {
        #[doc = "0 The        register is unlocked and can be updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The register is        locked and cannot be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl1_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl1_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 1\n resetvalue={LVD Reset:0x086690708,Cold PORST:0x086690708,After SSW execution:0x086690708}"]
pub type Evrsdctrl1 = crate::RegValueT<Evrsdctrl1_SPEC>;

impl Evrsdctrl1 {
    #[doc = "Minimum Off Time m0toff mintof i    M0TOFF. This bitfield configures the minimum off time within one period in        100MHz clock cycle periods during closed loop operation."]
    #[inline(always)]
    pub fn m0toff(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdctrl1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdctrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Minimum On Time m0ton minton i    M0TON. This bitfield configures the minimum on time within one period in 100MHz        clock cycle periods during closed loop operation."]
    #[inline(always)]
    pub fn m0ton(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdctrl1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdctrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S0 coefficient m0s0 coeff i    M0S0COEFF. This bitfield indicates the S0 coefficient during closed loop operation."]
    #[inline(always)]
    pub fn m0s0coeff(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Evrsdctrl1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Evrsdctrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Dead Band m0s0 deadbd i    M0DEADBD. This bitfield specifies the dead band to block the ADC ripple during        closed loop operation."]
    #[inline(always)]
    pub fn m0deadbd(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Evrsdctrl1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Evrsdctrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ADC Zero Bin m0fcfg adczb i    M0ADCZB. This bitfield specifies the zero error bin during closed loop operation."]
    #[inline(always)]
    pub fn m0adczb(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x3,
        1,
        0,
        evrsdctrl1::M0Adczb,
        Evrsdctrl1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x3,
            1,
            0,
            evrsdctrl1::M0Adczb,
            Evrsdctrl1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Skip Pulse Threshold m0skip thres i    M0SKIP. This bitfield specifies the threshold to detect a skip pulse condition        during closed loop operation.  N channel MOSFET ."]
    #[inline(always)]
    pub fn m0skip(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Evrsdctrl1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Evrsdctrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "EVRC Synchronization input enable synci0 en i    SYNCEN. This bitfield enables the input synchronization logic of EVRC SMPS        regulator. When set to 1  the DCDC will start to lock to the external        synchronization input signal. This EVRC Synchronization status is indicated in EVRSTAT.SYNCLCK status        bits."]
    #[inline(always)]
    pub fn syncen(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        evrsdctrl1::Syncen,
        Evrsdctrl1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            evrsdctrl1::Syncen,
            Evrsdctrl1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        evrsdctrl1::Lck,
        Evrsdctrl1_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            evrsdctrl1::Lck,
            Evrsdctrl1_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrsdctrl1 {
    #[inline(always)]
    fn default() -> Evrsdctrl1 {
        <crate::RegValueT<Evrsdctrl1_SPEC> as RegisterValue<_>>::new(2255030024)
    }
}
pub mod evrsdctrl1 {
    pub struct M0Adczb_SPEC;
    pub type M0Adczb = crate::EnumBitfieldStruct<u8, M0Adczb_SPEC>;
    impl M0Adczb {
        #[doc = "00 No        compensation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 1 8"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 1 4"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 3 8"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Syncen_SPEC;
    pub type Syncen = crate::EnumBitfieldStruct<u8, Syncen_SPEC>;
    impl Syncen {
        #[doc = "0 Synchronization        of EVRC switching gate outputs to external input signal is disabled ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Synchronization        of EVRC switching gate outputs to external input signal is enabled ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lck_SPEC;
    pub type Lck = crate::EnumBitfieldStruct<u8, Lck_SPEC>;
    impl Lck {
        #[doc = "0 The        register is unlocked and can be updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The register is        locked and cannot be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl10_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl10_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 10\n resetvalue={LVD Reset:0x5A82,Cold PORST:0x5A82,After SSW execution:0x5A82}"]
pub type Evrsdctrl10 = crate::RegValueT<Evrsdctrl10_SPEC>;

impl Evrsdctrl10 {
    #[doc = "Short to High Voltage Threshold shrth1 shvh i    SHVH. High Voltage Threshold    SDVOUTSEL  160    160 SHVH  160 x  160 5  160 mV . EVRC short to supply        alarm has the nominal values of SHVH of 1.9V and tCSHHV of 3ms."]
    #[inline(always)]
    pub fn shvh(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdctrl10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdctrl10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Short to Low Voltage Threshold shrtl1 shvl i    SHVL. Low Voltage Threshold    SDVOUTSEL  160    160 SHVL  160 x  160 5  160 mV . EVRC short to ground        alarm has the nominal values of SHVL of 0.8V and tCSHLV of 3ms."]
    #[inline(always)]
    pub fn shvl(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdctrl10_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdctrl10_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Short to High Detection Enable shrth0 shhven i    SHHVEN"]
    #[inline(always)]
    pub fn shhven(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        evrsdctrl10::Shhven,
        Evrsdctrl10_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            evrsdctrl10::Shhven,
            Evrsdctrl10_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Short to Low Detection Enable shrtl0 shlven i    SHLVEN"]
    #[inline(always)]
    pub fn shlven(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        evrsdctrl10::Shlven,
        Evrsdctrl10_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            evrsdctrl10::Shlven,
            Evrsdctrl10_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrsdctrl10 {
    #[inline(always)]
    fn default() -> Evrsdctrl10 {
        <crate::RegValueT<Evrsdctrl10_SPEC> as RegisterValue<_>>::new(23170)
    }
}
pub mod evrsdctrl10 {
    pub struct Shhven_SPEC;
    pub type Shhven = crate::EnumBitfieldStruct<u8, Shhven_SPEC>;
    impl Shhven {
        #[doc = "0 Short        to High Detection is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Short to High        Detection is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Shlven_SPEC;
    pub type Shlven = crate::EnumBitfieldStruct<u8, Shlven_SPEC>;
    impl Shlven {
        #[doc = "0 Short        to Low Detection is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Short to Low        Detection is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl11_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl11_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 11\n resetvalue={LVD Reset:0x092070909,Cold PORST:0x092070909,After SSW execution:0x092070909}"]
pub type Evrsdctrl11 = crate::RegValueT<Evrsdctrl11_SPEC>;

impl Evrsdctrl11 {
    #[doc = "High VDD Limit for Droop request droopvh thres i    DROOPVH. This bitfield defines the VDD high voltage limit above which a positive        droop request on VDD voltage shall be ignored. VDD Droop High Limit   712.5 mV   LSB    SDVOUTSEL  SDVOUTTRIM         DROOPVH   LSB   5 mV"]
    #[inline(always)]
    pub fn droopvh(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, Evrsdctrl11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, Evrsdctrl11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Low VDD Limit for Droop request droopvl thres i    DROOPVL. This bitfield defines the VDD low voltage limit below which a negative        droop request on VDD voltage shall be ignored. VDD Droop Low Limit   712.5 mV   LSB    SDVOUTSEL  SDVOUTTRIM  DROOPVL          LSB   5 mV"]
    #[inline(always)]
    pub fn droopvl(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, Evrsdctrl11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, Evrsdctrl11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Maximum Deviation of the Synchronization Input Frequency synci1 maxdev i    SYNCMAXDEV. This bitfield defines the maximum allowed frequency deviation of the        synchronization input signal frequency from the programmed nominal DCDC        switching frequency  EVRSDCTRL0.SDFREQ . For locking         EVRSDCTRL11.SYNCMAXDEV has to be chosen to be greater or equal to the        value of EVRSDCTRL11.SYNCHYST  and unequal to zero. Violation of limit        leads to loss of synchronization.The frequency window is defined as        follows d  160  f MAXDEV   160   100 MHz   2 SYNCMAXDEV            SDFREQ 2 SYNCMAXDEV 2  SYNCMAXDEV  160   round    100 MHz   d  160  f MAXDEV     sqrt   100 MHz   d  160  f MAXDEV   2          SDFREQ 2"]
    #[inline(always)]
    pub fn syncmaxdev(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Evrsdctrl11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Evrsdctrl11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Unlock Hysteresis Window synci0 hyst i    SYNCHYST. This bitfield defines the hysteresis window for synchronization locking        and unlocking. For locking  EVRSDCTRL11.SYNCHYST has to be chosen to be        lower or equal to the value of EVRSDCTRL11.SYNCMAXDEV  and unequal to        zero. The limit is applied to the period counter running at 100 MHz. Upper unlock condition  SDFREQ          SYNCMAXDEV Upper lock condition  SDFREQ          SYNCMAXDEV   SYNCHYST Lower unlock condition   SDFREQ          SYNCMAXDEV Lower lock condition   SDFREQ          SYNCMAXDEV   SYNCHYST SYNCHYST   round   d  160  f HYST    SDFREQ   177  SYNCMAXDEV  2       d  160  f HYST    SDFREQ   177  SYNCMAXDEV    100 MHz"]
    #[inline(always)]
    pub fn synchyst(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Evrsdctrl11_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Evrsdctrl11_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Synchronisation Input Multiplexer   SYNCMUXSEL. This bitfield selects synchronisation input either from CCU6 or GTM        inputs to be forwarded to EVRC SMPS regulator."]
    #[inline(always)]
    pub fn syncmuxsel(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x3,
        1,
        0,
        evrsdctrl11::Syncmuxsel,
        Evrsdctrl11_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x3,
            1,
            0,
            evrsdctrl11::Syncmuxsel,
            Evrsdctrl11_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        evrsdctrl11::Lck,
        Evrsdctrl11_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            evrsdctrl11::Lck,
            Evrsdctrl11_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrsdctrl11 {
    #[inline(always)]
    fn default() -> Evrsdctrl11 {
        <crate::RegValueT<Evrsdctrl11_SPEC> as RegisterValue<_>>::new(2449934601)
    }
}
pub mod evrsdctrl11 {
    pub struct Syncmuxsel_SPEC;
    pub type Syncmuxsel = crate::EnumBitfieldStruct<u8, Syncmuxsel_SPEC>;
    impl Syncmuxsel {
        #[doc = "00 Synchronization        input open or unconnected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 CCU60 COUT63"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 GTM"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Lck_SPEC;
    pub type Lck = crate::EnumBitfieldStruct<u8, Lck_SPEC>;
    impl Lck {
        #[doc = "0 The        register is unlocked and can be updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The register is        locked and cannot be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl2_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl2_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 2\n resetvalue={LVD Reset:0x36033B,Cold PORST:0x36033B,After SSW execution:0x36033B}"]
pub type Evrsdctrl2 = crate::RegValueT<Evrsdctrl2_SPEC>;

impl Evrsdctrl2 {
    #[doc = "Low Power Mode Hysteresis OFFSET lpbnd offset i    LPBNDOFFSET. This bitfield defines the turn on threshold in LP mode"]
    #[inline(always)]
    pub fn lpbndoffset(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Evrsdctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Evrsdctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Low Power Mode Hysteresis Band Width lpbnd width i    LPBNDWIDTH. This bitfield defines the turn on threshold in LP mode."]
    #[inline(always)]
    pub fn lpbndwidth(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Evrsdctrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Evrsdctrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Low Pass Filter Coefficient lplpf coeff i    LPLPFCOEFF. This bit field configures the low pass filter coefficient for the        setting of the turn on threshold of the Sliding function."]
    #[inline(always)]
    pub fn lplpfcoeff(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        evrsdctrl2::Lplpfcoeff,
        Evrsdctrl2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            evrsdctrl2::Lplpfcoeff,
            Evrsdctrl2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Regulator Over sampling Factor m1osfl fact i m1osfh fact i    SDFREQLP. This bitfield configures the EVRC regulator FB ADC sampling period        during low power mode. The switching frequency is not constant."]
    #[inline(always)]
    pub fn sdfreqlp(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xfff,
        1,
        0,
        evrsdctrl2::Sdfreqlp,
        Evrsdctrl2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xfff,
            1,
            0,
            evrsdctrl2::Sdfreqlp,
            Evrsdctrl2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "LPM or PWM EVRC Mode Activation   EVRCMOD. This bit switches operation mode between PWM and LPM mode."]
    #[inline(always)]
    pub fn evrcmod(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        evrsdctrl2::Evrcmod,
        Evrsdctrl2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            evrsdctrl2::Evrcmod,
            Evrsdctrl2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        evrsdctrl2::Lck,
        Evrsdctrl2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            evrsdctrl2::Lck,
            Evrsdctrl2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrsdctrl2 {
    #[inline(always)]
    fn default() -> Evrsdctrl2 {
        <crate::RegValueT<Evrsdctrl2_SPEC> as RegisterValue<_>>::new(3539771)
    }
}
pub mod evrsdctrl2 {
    pub struct Lplpfcoeff_SPEC;
    pub type Lplpfcoeff = crate::EnumBitfieldStruct<u8, Lplpfcoeff_SPEC>;
    impl Lplpfcoeff {
        #[doc = "0   160 Fast Filter"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "F   160 Slow Filter"]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Sdfreqlp_SPEC;
    pub type Sdfreqlp = crate::EnumBitfieldStruct<u8, Sdfreqlp_SPEC>;
    impl Sdfreqlp {
        #[doc = "C8   160   160 0.5  160 MHz         100 MHz 200"]
        pub const CONST_200200: Self = Self::new(200);
        #[doc = "7D   160   160 0.8  160 MHz         100 MHz 125"]
        pub const CONST_125125: Self = Self::new(125);
        #[doc = "37   160   160 1.82  160 MHz         100 MHz 55"]
        pub const CONST_5555: Self = Self::new(55);
    }
    pub struct Evrcmod_SPEC;
    pub type Evrcmod = crate::EnumBitfieldStruct<u8, Evrcmod_SPEC>;
    impl Evrcmod {
        #[doc = "0 The step down        converter is in normal operational closed loop state  PWM . Both Pch.        MOSFET and Nch. MOSFET are being switched."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The step down        converter is in low power mode  LPM . Only Pch. MOSFET is being switched and Nch. MOSFET behaves like a        diode."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lck_SPEC;
    pub type Lck = crate::EnumBitfieldStruct<u8, Lck_SPEC>;
    impl Lck {
        #[doc = "0 The        register is unlocked and can be updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The register is        locked and cannot be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl3_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl3_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 3\n resetvalue={LVD Reset:0x0B690810,Cold PORST:0x0B690810,After SSW execution:0x0B690810}"]
pub type Evrsdctrl3 = crate::RegValueT<Evrsdctrl3_SPEC>;

impl Evrsdctrl3 {
    #[doc = "Minimum Off Time m1toff mintof i    M1TOFF. This bitfield configures the minimum off time within one period in        100MHz clock cycle periods during LP mode."]
    #[inline(always)]
    pub fn m1toff(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdctrl3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdctrl3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Minimum On Time m1ton minton i    M1TON. This bitfield configures the minimum on time within one period in 100MHz        clock cycle periods during LP mode."]
    #[inline(always)]
    pub fn m1ton(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdctrl3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdctrl3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S0 coefficient m1s0 coeff i    M1S0COEFF. This bitfield indicates the S0 coefficient during LP mode."]
    #[inline(always)]
    pub fn m1s0coeff(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Evrsdctrl3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Evrsdctrl3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Dead Band m1s0 deadbd i    M1DEADBD. This bitfield specifies the dead band to block the ADC ripple during LP        mode."]
    #[inline(always)]
    pub fn m1deadbd(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Evrsdctrl3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Evrsdctrl3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ADC Zero Bin m1fcfg adczb i    M1ADCZB. This bitfield specifies the zero error bin during LP mode."]
    #[inline(always)]
    pub fn m1adczb(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x3,
        1,
        0,
        evrsdctrl3::M1Adczb,
        Evrsdctrl3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x3,
            1,
            0,
            evrsdctrl3::M1Adczb,
            Evrsdctrl3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Skip Pulse Threshold m1skip thres i    M1SKIP. This bitfield is disabled in LPM mode as PFM applied by control itself."]
    #[inline(always)]
    pub fn m1skip(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Evrsdctrl3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Evrsdctrl3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdctrl3 {
    #[inline(always)]
    fn default() -> Evrsdctrl3 {
        <crate::RegValueT<Evrsdctrl3_SPEC> as RegisterValue<_>>::new(191432720)
    }
}
pub mod evrsdctrl3 {
    pub struct M1Adczb_SPEC;
    pub type M1Adczb = crate::EnumBitfieldStruct<u8, M1Adczb_SPEC>;
    impl M1Adczb {
        #[doc = "00 No        compensation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 1 8"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 1 4"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 3 8"]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl4_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl4_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 4\n resetvalue={LVD Reset:0x360009,Cold PORST:0x360009,After SSW execution:0x360009}"]
pub type Evrsdctrl4 = crate::RegValueT<Evrsdctrl4_SPEC>;

impl Evrsdctrl4 {
    #[doc = "Voltage OK Circuit Configuration vokcfg config i    VOKCFG. t.b.d."]
    #[inline(always)]
    pub fn vokcfg(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Evrsdctrl4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Evrsdctrl4_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Regulator Switching Frequency or Over sampling Factor m2osfl fact i m2osfh fact i    SDFREQST. This bit field configures the EVRC regulator switching frequency during        closed loop start up.The switching frequency is equal to  100  160 MHz          SDFREQ  value. SDFREQ represents the corresponding over sampling factor."]
    #[inline(always)]
    pub fn sdfreqst(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xfff,
        1,
        0,
        evrsdctrl4::Sdfreqst,
        Evrsdctrl4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xfff,
            1,
            0,
            evrsdctrl4::Sdfreqst,
            Evrsdctrl4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrsdctrl4 {
    #[inline(always)]
    fn default() -> Evrsdctrl4 {
        <crate::RegValueT<Evrsdctrl4_SPEC> as RegisterValue<_>>::new(3538953)
    }
}
pub mod evrsdctrl4 {
    pub struct Sdfreqst_SPEC;
    pub type Sdfreqst = crate::EnumBitfieldStruct<u8, Sdfreqst_SPEC>;
    impl Sdfreqst {
        #[doc = "C8   160   160 0.5  160 MHz         100 MHz 200  SMPS switching frequency"]
        pub const CONST_200200: Self = Self::new(200);
        #[doc = "7D   160   160 0.8  160 MHz         100 MHz 125  SMPS switching frequency"]
        pub const CONST_125125: Self = Self::new(125);
        #[doc = "37   160   160 1.82  160 MHz         100 MHz 55  SMPS switching frequency"]
        pub const CONST_5555: Self = Self::new(55);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl5_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl5_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 5\n resetvalue={LVD Reset:0x0B690808,Cold PORST:0x0B690808,After SSW execution:0x0B690808}"]
pub type Evrsdctrl5 = crate::RegValueT<Evrsdctrl5_SPEC>;

impl Evrsdctrl5 {
    #[doc = "Minimum Off Time m2toff mintof i    M2TOFF. This bitfield configures the minimum off time within one period in        100MHz clock cycle periods during closed loop operation."]
    #[inline(always)]
    pub fn m2toff(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdctrl5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdctrl5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Minimum On Time m2ton minton i    M2TON. This bitfield configures the minimum on time within one period in 100MHz        clock cycle periods during closed loop operation."]
    #[inline(always)]
    pub fn m2ton(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdctrl5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdctrl5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "S0 coefficient m2s0 coeff i    M2S0COEFF. This bitfield indicates the S0 coefficient during closed loop operation."]
    #[inline(always)]
    pub fn m2s0coeff(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Evrsdctrl5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Evrsdctrl5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Dead Band m2s0 deadbd i    M2DEADBD. This bitfield specifies the dead band to block the ADC ripple during        closed loop operation."]
    #[inline(always)]
    pub fn m2deadbd(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Evrsdctrl5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Evrsdctrl5_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ADC Zero Bin m2fcfg adczb i    M2ADCZB. This bitfield specifies the zero error bin during closed loop operation."]
    #[inline(always)]
    pub fn m2adczb(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x3,
        1,
        0,
        evrsdctrl5::M2Adczb,
        Evrsdctrl5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x3,
            1,
            0,
            evrsdctrl5::M2Adczb,
            Evrsdctrl5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Skip Pulse Threshold m2skip thres i    M2SKIP. This bitfield specifies the threshold to detect a skip pulse condition        during closed loop operation.  N channel MOSFET ."]
    #[inline(always)]
    pub fn m2skip(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Evrsdctrl5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Evrsdctrl5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdctrl5 {
    #[inline(always)]
    fn default() -> Evrsdctrl5 {
        <crate::RegValueT<Evrsdctrl5_SPEC> as RegisterValue<_>>::new(191432712)
    }
}
pub mod evrsdctrl5 {
    pub struct M2Adczb_SPEC;
    pub type M2Adczb = crate::EnumBitfieldStruct<u8, M2Adczb_SPEC>;
    impl M2Adczb {
        #[doc = "00 No        compensation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 1 8"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 1 4"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 3 8"]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl6_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl6_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 6\n resetvalue={LVD Reset:0x080231C94,Cold PORST:0x080231C94,After SSW execution:0x080231C94}"]
pub type Evrsdctrl6 = crate::RegValueT<Evrsdctrl6_SPEC>;

impl Evrsdctrl6 {
    #[doc = "Vin threshold to switch between SINCLO or SINCHI. svinth thres i    SVINTH. This bit field specifies the threshold to decide on the ramp up        increment during startup. If Vin is below the threshold  SINCLO is taken        as ramp up increment  else if Vin is equal or above the threshold         SINCHI is taken as ramp up increment.The threshold is compared to the        FF ADC counter value  without offset."]
    #[inline(always)]
    pub fn svinth(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdctrl6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdctrl6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Vout threshold to switch from open loop start up to closed loop mode. svoth thres i    SVOTH. This bit field specifies the threshold to decide when to switch from        open loop mode to closed loop mode during startup. If Vout is below the        threshold  open loop ramp up is executed. if Vout is equal or above the        threshold  closed loop PWM in start up configuration is executed. The        threshold is compared to the low pass filtered FB ADC counter value         without offset. The switch happens only in one direction during startup        and the system does not switch back into start up mode even if threshold        is crossed in other direction."]
    #[inline(always)]
    pub fn svoth(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrsdctrl6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrsdctrl6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Increment for low input voltage. sinc sinclo i    SINCLO. This bitfield specifies the increment of the on time during open loop        ramp up during startup. If Vin is below the threshold  SVINTH   SINCLO        is taken as ramp up increment. if Vin is equal or above the threshold         SVINTH   SINCHI is taken as ramp up increment"]
    #[inline(always)]
    pub fn sinclo(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Evrsdctrl6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Evrsdctrl6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Increment for high input voltage. sinc sinchi i    SINCHI. This bitfield specifies the increment of the on time during open loop        ramp up during startup. If Vin is below the threshold  SVINTH   SINCLO        is taken as ramp up increment. if Vin is equal or above the threshold         SVINTH   SINCHI is taken as ramp up increment"]
    #[inline(always)]
    pub fn sinchi(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, Evrsdctrl6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x7,1,0,u8, Evrsdctrl6_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        evrsdctrl6::Lck,
        Evrsdctrl6_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            evrsdctrl6::Lck,
            Evrsdctrl6_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrsdctrl6 {
    #[inline(always)]
    fn default() -> Evrsdctrl6 {
        <crate::RegValueT<Evrsdctrl6_SPEC> as RegisterValue<_>>::new(2149784724)
    }
}
pub mod evrsdctrl6 {
    pub struct Lck_SPEC;
    pub type Lck = crate::EnumBitfieldStruct<u8, Lck_SPEC>;
    impl Lck {
        #[doc = "0 The        register is unlocked and can be updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The register is        locked and cannot be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl7_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl7_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 7\n resetvalue={LVD Reset:0x0800000FE,Cold PORST:0x0800000FE,After SSW execution:0x0800000FE}"]
pub type Evrsdctrl7 = crate::RegValueT<Evrsdctrl7_SPEC>;

impl Evrsdctrl7 {
    #[doc = "Selection of N driver current   DRVNI. Adjustable driver strength of the N driver current"]
    #[inline(always)]
    pub fn drvni(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        evrsdctrl7::Drvni,
        Evrsdctrl7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            evrsdctrl7::Drvni,
            Evrsdctrl7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "P Driver Current Boost Factor drvp strgth i    DRVPCBF. Adjustable boost factor for the P driver current"]
    #[inline(always)]
    pub fn drvpcbf(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        evrsdctrl7::Drvpcbf,
        Evrsdctrl7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            evrsdctrl7::Drvpcbf,
            Evrsdctrl7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "P Driver Current drvp strgth i    DRVP. Base drive current of the P channel MOSFET when driven with 3.3V  160    160  5V."]
    #[inline(always)]
    pub fn drvp(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        evrsdctrl7::Drvp,
        Evrsdctrl7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            evrsdctrl7::Drvp,
            Evrsdctrl7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Switching Configuration drvslo mode i    DRVSLOMODE. This bitfield configure the type of switching."]
    #[inline(always)]
    pub fn drvslomode(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        evrsdctrl7::Drvslomode,
        Evrsdctrl7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            evrsdctrl7::Drvslomode,
            Evrsdctrl7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Spare bits drvspr x i    DRVSPR"]
    #[inline(always)]
    pub fn drvspr(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evrsdctrl7_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evrsdctrl7_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        evrsdctrl7::Lck,
        Evrsdctrl7_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            evrsdctrl7::Lck,
            Evrsdctrl7_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrsdctrl7 {
    #[inline(always)]
    fn default() -> Evrsdctrl7 {
        <crate::RegValueT<Evrsdctrl7_SPEC> as RegisterValue<_>>::new(2147483902)
    }
}
pub mod evrsdctrl7 {
    pub struct Drvni_SPEC;
    pub type Drvni = crate::EnumBitfieldStruct<u8, Drvni_SPEC>;
    impl Drvni {
        #[doc = "00 1 4"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 1 2"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 3 4"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 1"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Drvpcbf_SPEC;
    pub type Drvpcbf = crate::EnumBitfieldStruct<u8, Drvpcbf_SPEC>;
    impl Drvpcbf {
        #[doc = "00 9  160    160 7"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 9  160    160 5"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 9  160    160 4"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 9  160    160 3"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Drvp_SPEC;
    pub type Drvp = crate::EnumBitfieldStruct<u8, Drvp_SPEC>;
    impl Drvp {
        #[doc = "0000 5 3  160 mA  160    160 7 8  160 mA"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "0001 6 3  160 mA  160    160 9 4  160 mA"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "0010 7 4  160 mA  160    160 11  160 mA"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "0011 8 4  160 mA  160    160 12 5  160 mA"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "0100 10 5  160 mA  160    160 15 6  160 mA"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "0101 12 6  160 mA  160    160 18 7  160 mA"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "0110 14 7  160 mA  160    160 21 8  160 mA"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "0111 17 8  160 mA  160    160 26 4  160 mA"]
        pub const CONST_77: Self = Self::new(7);
        #[doc = "1000 20 9  160 mA  160    160 31  160 mA"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "1001 25  160 mA  160    160 37 1  160 mA"]
        pub const CONST_99: Self = Self::new(9);
        #[doc = "1010 29 1  160 mA  160    160 43 2  160 mA"]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "1011 35 3  160 mA  160    160 52 3  160 mA"]
        pub const CONST_1111: Self = Self::new(11);
        #[doc = "1100 41 4  160 mA  160    160 61 4  160 mA"]
        pub const CONST_1212: Self = Self::new(12);
        #[doc = "1101 49 6  160 mA  160    160 73 4  160 mA"]
        pub const CONST_1313: Self = Self::new(13);
        #[doc = "1110 58 8  160 mA  160    160 87  160 mA"]
        pub const CONST_1414: Self = Self::new(14);
        #[doc = "1111 69 9  160 mA  160    160 103 5  160 mA"]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Drvslomode_SPEC;
    pub type Drvslomode = crate::EnumBitfieldStruct<u8, Drvslomode_SPEC>;
    impl Drvslomode {
        #[doc = "00 Nominal mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 B C mode"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Hard Switching        mode"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Lck_SPEC;
    pub type Lck = crate::EnumBitfieldStruct<u8, Lck_SPEC>;
    impl Lck {
        #[doc = "0 The        register is unlocked and can be updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The register is        locked and cannot be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl8_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl8_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 8\n resetvalue={LVD Reset:0x09121048E,Cold PORST:0x09121048E,After SSW execution:0x09121048E}"]
pub type Evrsdctrl8 = crate::RegValueT<Evrsdctrl8_SPEC>;

impl Evrsdctrl8 {
    #[doc = "Feedback Converted Counter Value Offset fbadc2 offset i    FBADCOFFS. This bitfield configures the offset of the converted counter value of        the feedback ADC measuring the core voltage."]
    #[inline(always)]
    pub fn fbadcoffs(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdctrl8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdctrl8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "FB ADC Sampling period fbadc1 smpthr i    FBADCSMP. This bitfield configures the sampling period in 100 MHz clock cycles for        the feedback ADC measuring the core voltage."]
    #[inline(always)]
    pub fn fbadcsmp(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, Evrsdctrl8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3f,1,0,u8, Evrsdctrl8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "FB ADC Blanked Samples Number fbadc0 blank i    FBADCBLNK. This bitfield configures the number of feedback ADC samples that are        blanked in case of a transition of the PWM drive output to minimise        switching noise influence."]
    #[inline(always)]
    pub fn fbadcblnk(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Evrsdctrl8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Evrsdctrl8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "FB ADC Counter LPF Coefficient fbadc0 lpfcnt i    FBADCLPF. This bit field configures the coefficient of the Low Pass Filter of the        feedback ADC counter value measuring the core voltage. y  160  k   160    160    160 y  160  k 1   160    160  1 a   160    160    160    160 x  160  k   160    160 a   160   y  160  k  is filter output         x  160  k  is ADC output a  160    160  1  160    160  2  160    160 LPF  .If LPF  160    160 0  the filter output is the same as ADC        output."]
    #[inline(always)]
    pub fn fbadclpf(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Evrsdctrl8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Evrsdctrl8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "FB ADC Error LPF Coefficient fbadc3 lpferr i    FBADCERR. This bitfield configures the coefficient of the Low Pass Filter of the        output voltage error signal of the feedback ADC."]
    #[inline(always)]
    pub fn fbadcerr(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, Evrsdctrl8_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, Evrsdctrl8_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "FB ADC LSB for Error Computation fbadc3 lsb i    FBADCLSB. This bitfield configures the LSB of the feedback ADC counter value used        for the error computation."]
    #[inline(always)]
    pub fn fbadclsb(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        evrsdctrl8::Fbadclsb,
        Evrsdctrl8_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            evrsdctrl8::Fbadclsb,
            Evrsdctrl8_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        evrsdctrl8::Lck,
        Evrsdctrl8_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            evrsdctrl8::Lck,
            Evrsdctrl8_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrsdctrl8 {
    #[inline(always)]
    fn default() -> Evrsdctrl8 {
        <crate::RegValueT<Evrsdctrl8_SPEC> as RegisterValue<_>>::new(2434860174)
    }
}
pub mod evrsdctrl8 {
    pub struct Fbadclsb_SPEC;
    pub type Fbadclsb = crate::EnumBitfieldStruct<u8, Fbadclsb_SPEC>;
    impl Fbadclsb {
        #[doc = "0 5  160 mV"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 10  160 mV"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lck_SPEC;
    pub type Lck = crate::EnumBitfieldStruct<u8, Lck_SPEC>;
    impl Lck {
        #[doc = "0 The        register is unlocked and can be updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The register is        locked and cannot be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdctrl9_SPEC;
impl crate::sealed::RegSpec for Evrsdctrl9_SPEC {
    type DataType = u32;
}
#[doc = "EVRC SD Control Register 9\n resetvalue={LVD Reset:0x080000434,Cold PORST:0x080000434,After SSW execution:0x080000434}"]
pub type Evrsdctrl9 = crate::RegValueT<Evrsdctrl9_SPEC>;

impl Evrsdctrl9 {
    #[doc = "Feed Forward Converted Counter Value Offset ffadc1 offset i    FFADCOFFS. This bit field configures the offset of the converted counter value of        the feed forward ADC measuring the input VEXT voltage."]
    #[inline(always)]
    pub fn ffadcoffs(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdctrl9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdctrl9_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "FF ADC Counter LPF Coefficient ffadc0 lpfcnt i    FFADCLPF. This bit field configures the coefficient of the Low Pass Filter of the        feed forward ADC counter value measuring the input VEXT voltage. y  160  k   160    160    160 y  160  k 1   160    160  1 a   160    160    160    160 x  160  k   160    160 a   160   y  160  k  is filter output         x  160  k  is ADC output a  160    160  1  160    160  2  160    160 LPF  .If LPF  160    160 0  the filter output is the same as ADC        output."]
    #[inline(always)]
    pub fn ffadclpf(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Evrsdctrl9_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Evrsdctrl9_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        evrsdctrl9::Lck,
        Evrsdctrl9_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            evrsdctrl9::Lck,
            Evrsdctrl9_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrsdctrl9 {
    #[inline(always)]
    fn default() -> Evrsdctrl9 {
        <crate::RegValueT<Evrsdctrl9_SPEC> as RegisterValue<_>>::new(2147484724)
    }
}
pub mod evrsdctrl9 {
    pub struct Lck_SPEC;
    pub type Lck = crate::EnumBitfieldStruct<u8, Lck_SPEC>;
    impl Lck {
        #[doc = "0 The        register is unlocked and can be updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The register is        locked and cannot be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrsdstat0_SPEC;
impl crate::sealed::RegSpec for Evrsdstat0_SPEC {
    type DataType = u32;
}
#[doc = "EVR SD Status Register 0\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
pub type Evrsdstat0 = crate::RegValueT<Evrsdstat0_SPEC>;

impl Evrsdstat0 {
    #[doc = "Step Down Converter Core Voltage Feedback ADC Conversion Result   ADCFBCV. This bit field indicates the last ADC conversion result of the step down        converter feedback ADC measuring VDD core voltage. VIN  160    160  LSB  160    160 ADCFBCV  0.7125   160 V  160    160 LSB   5  160 mV E.g. 1.20  160 V   62   98"]
    #[inline(always)]
    pub fn adcfbcv(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrsdstat0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrsdstat0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DPWM Control Output Status   DPWMOUT. This bit field reflects the actual PWM output of the controller provided        to the external MOSFET switches."]
    #[inline(always)]
    pub fn dpwmout(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, Evrsdstat0_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xfff,1,0,u16, Evrsdstat0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrsdstat0 {
    #[inline(always)]
    fn default() -> Evrsdstat0 {
        <crate::RegValueT<Evrsdstat0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrstat_SPEC;
impl crate::sealed::RegSpec for Evrstat_SPEC {
    type DataType = u32;
}
#[doc = "EVR Status Register\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
pub type Evrstat = crate::RegValueT<Evrstat_SPEC>;

impl Evrstat {
    #[doc = "EVRC status   EVRC. This bit is set if the internal EVRC regulator is currently active. EVRC        is activated if HWCFG 2  pin level is latched high during start up        phase. The bit reflects the        DCDC regulator enable bit provided from PMS to DCDC."]
    #[inline(always)]
    pub fn evrc(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, evrstat::Evrc, Evrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,evrstat::Evrc, Evrstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VDD Over voltage event flag   OVC. This bit is set if VDD secondary voltage monitor recognizes a        over voltage event. An alarm is raised to the SMU and the status bit        remains set until violation disappears."]
    #[inline(always)]
    pub fn ovc(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, evrstat::Ovc, Evrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,evrstat::Ovc, Evrstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EVR33 status   EVR33. This bit is set if the internal EVR33 LDO regulator is active. EVR33 is        activated if HWCFG 1  pin level is latched high during start up phase. The bit reflects the EVR33 enable bit provided from PMS to PID."]
    #[inline(always)]
    pub fn evr33(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, evrstat::Evr33, Evrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x1,1,0,evrstat::Evr33, Evrstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VDDP3 Over voltage event flag   OV33. This bit is set if VDDP3 secondary voltage monitor recognizes a        over voltage event. An alarm is raised to the SMU and the status bit        remains set until violation disappears."]
    #[inline(always)]
    pub fn ov33(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, evrstat::Ov33, Evrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x1,1,0,evrstat::Ov33, Evrstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VEXT Over voltage event flag   OVSWD. This bit is set if VEXT secondary voltage monitor recognizes an        over voltage event. An alarm is raised to the SMU and the status bit        remains set until violation disappears."]
    #[inline(always)]
    pub fn ovswd(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, evrstat::Ovswd, Evrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,evrstat::Ovswd, Evrstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VDD Under voltage event flag   UVC. This bit is set if VDD secondary voltage monitor recognizes a        under voltage event. An alarm is raised to the SMU and the status bit        remains set until violation disappears."]
    #[inline(always)]
    pub fn uvc(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, evrstat::Uvc, Evrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x1,1,0,evrstat::Uvc, Evrstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VDDP3 Under voltage event flag   UV33. This bit is set if VDDP3 secondary voltage monitor recognizes a        under voltage event. An alarm is raised to the SMU and the status bit        remains set until violation disappears."]
    #[inline(always)]
    pub fn uv33(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, evrstat::Uv33, Evrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x1,1,0,evrstat::Uv33, Evrstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "VEXT Under voltage event flag   UVSWD. This bit is set if VEXT secondary voltage monitor recognizes an        under voltage event. An alarm is raised to the SMU and the status bit        remains set until violation disappears."]
    #[inline(always)]
    pub fn uvswd(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, evrstat::Uvswd, Evrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<7,0x1,1,0,evrstat::Uvswd, Evrstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EVRC Synchronization Input Locked status sd sync in locked o    SYNCLCK. This bitfield indicates the current synchronization status of EVRC SMPS        regulator to external DCDCSYNCI input signal. When the EVRC switching        frequency  edge is locked to the synchronization input  the SYNCLCK bit        is set to HIGH indicating the locked state. When the synchronization is        lost owing to frequency deviations beyond MAXDEV or the feature is        disabled via SYNCEN  the SYNCLCK bit is set to LOW. This EVRC Synchronization status is indicated in EVRSDSTAT0.SYNCLCK        status bits."]
    #[inline(always)]
    pub fn synclck(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, evrstat::Synclck, Evrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x1,1,0,evrstat::Synclck, Evrstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EVR33 Regulator Voltage OK status   EVR33VOK. This bit is set after the soft ramp up time of the EVR33 voltage OK ramp        detector has elapsed and is not based on the measured VDDP3 voltage at        the end of ramp phase.."]
    #[inline(always)]
    pub fn evr33vok(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, evrstat::Evr33Vok, Evrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            evrstat::Evr33Vok,
            Evrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "EVRC Reset Trigger   RSTC"]
    #[inline(always)]
    pub fn rstc(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, evrstat::Rstc, Evrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<13,0x1,1,0,evrstat::Rstc, Evrstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EVR33 Reset Trigger   RST33"]
    #[inline(always)]
    pub fn rst33(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, evrstat::Rst33, Evrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<14,0x1,1,0,evrstat::Rst33, Evrstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EVR SWD Reset Trigger   RSTSWD"]
    #[inline(always)]
    pub fn rstswd(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, evrstat::Rstswd, Evrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<15,0x1,1,0,evrstat::Rstswd, Evrstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Short to ground   EVRCSHLV. This bit is set if a short condition to ground has been detected. The        measured EVRC output is below the operational supply range and the upper        controller limits are reached.The feature is supported only during        closed loop operation or EVRCMOD  160    160 00b.  evr sd short to lv"]
    #[inline(always)]
    pub fn evrcshlv(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        evrstat::Evrcshlv,
        Evrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            evrstat::Evrcshlv,
            Evrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Short to supply   EVRCSHHV. This bit is set if a short condition to supply has been detected. The        measured EVRC output exceeds the allowed supply range and the lower        controller limits are reached. The feature is supported only during        closed loop operation or EVRCMOD  160    160 00b.  evr sd short to hv"]
    #[inline(always)]
    pub fn evrcshhv(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        evrstat::Evrcshhv,
        Evrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            evrstat::Evrcshhv,
            Evrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Short to ground   EVR33SHLV. This bit is set if a short condition to ground has been detected. The        measured EVR33 output is below the operational supply range and the        lower gate drive threshold voltage driving P ch. MOSFET  VGATEL 0 1V  is reached."]
    #[inline(always)]
    pub fn evr33shlv(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        evrstat::Evr33Shlv,
        Evrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            evrstat::Evr33Shlv,
            Evrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Short to supply   EVR33SHHV. This bit is set if a short condition to supply has been detected. The        measured EVR33 output exceeds the allowed supply range and the upper        gate drive threshold voltage driving P ch. MOSFET  VGATEL 4 9V  is reached."]
    #[inline(always)]
    pub fn evr33shhv(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        evrstat::Evr33Shhv,
        Evrstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            evrstat::Evr33Shhv,
            Evrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "VEXT External Supply Level Status   SWDLVL. This bit indicates that the VEXT voltage has dropped below  4  160 V to        indicate EVRC parameter switch to differentiate 5V or 3.3V external        supply. A hysteresis of  120  160 mV is implemented on this detector.  drv low vdd . The primary SWD ADC monitor value is used for EVRC SWDLVL        comparator."]
    #[inline(always)]
    pub fn swdlvl(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, evrstat::Swdlvl, Evrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<20,0x1,1,0,evrstat::Swdlvl, Evrstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EVRC Regulator Voltage OK status   SDVOK. This bit is set by the EVRC voltage OK detector to indicate that the new        regulator output value has been reached. This bit is reset incase        EVRTRIM. SDVOUTSEL or SDVOUTTRIM values are adapted to scale core        voltage and is set when the new output setpoint is reached. This bit is        also reset incase droop compensation is requested before a load jump        event. A time out period of x  160 us shall be waited when polling SDVOK bit.  status voltok o"]
    #[inline(always)]
    pub fn sdvok(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, evrstat::Sdvok, Evrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<21,0x1,1,0,evrstat::Sdvok, Evrstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EVRC Mode   EVRCMOD. This bit indicates the current operation mode of LC   PWM  LPM  STRT. Depending        on whether EVRC is activated the status bit is consequently updated.         status mode o"]
    #[inline(always)]
    pub fn evrcmod(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, evrstat::Evrcmod, Evrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            22,
            0x3,
            1,
            0,
            evrstat::Evrcmod,
            Evrstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Pre Regulator VDDPD Over voltage event flag   OVPRE. This bit is set if VDDPD supply secondary voltage monitor recognizes an        over voltage event. An alarm is raised to the SMU."]
    #[inline(always)]
    pub fn ovpre(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, evrstat::Ovpre, Evrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<24,0x1,1,0,evrstat::Ovpre, Evrstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Standby Supply or VEVRSB Over voltage event flag   OVSB. This bit is set if VEVRSB supply secondary voltage monitor recognizes an        over voltage event. An alarm is raised to the SMU."]
    #[inline(always)]
    pub fn ovsb(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, evrstat::Ovsb, Evrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<25,0x1,1,0,evrstat::Ovsb, Evrstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ADC VDDM Supply Over voltage event flag   OVDDM. This bit is set if VDDM ADC supply secondary voltage monitor recognizes        an over voltage event. An alarm is raised to the SMU."]
    #[inline(always)]
    pub fn ovddm(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, evrstat::Ovddm, Evrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<26,0x1,1,0,evrstat::Ovddm, Evrstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Pre Regulator VDDPD Under voltage event flag   UVPRE. This bit is set if VDDPD supply secondary voltage monitor recognizes an        under voltage event. An alarm is raised to the SMU."]
    #[inline(always)]
    pub fn uvpre(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, evrstat::Uvpre, Evrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<27,0x1,1,0,evrstat::Uvpre, Evrstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Standby Supply or VEVRSB Under voltage event flag   UVSB. This bit is set if VEVRSB supply secondary voltage monitor recognizes an        under voltage event. An alarm is raised to the SMU."]
    #[inline(always)]
    pub fn uvsb(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, evrstat::Uvsb, Evrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<28,0x1,1,0,evrstat::Uvsb, Evrstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ADC VDDM Supply Under voltage event flag   UVDDM. This bit is set if VDDM ADC supply secondary voltage monitor recognizes        an under voltage event. An alarm is raised to the SMU."]
    #[inline(always)]
    pub fn uvddm(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, evrstat::Uvddm, Evrstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<29,0x1,1,0,evrstat::Uvddm, Evrstat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrstat {
    #[inline(always)]
    fn default() -> Evrstat {
        <crate::RegValueT<Evrstat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod evrstat {
    pub struct Evrc_SPEC;
    pub type Evrc = crate::EnumBitfieldStruct<u8, Evrc_SPEC>;
    impl Evrc {
        #[doc = "0 EVRC is        inactive."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 EVRC is active."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ovc_SPEC;
    pub type Ovc = crate::EnumBitfieldStruct<u8, Ovc_SPEC>;
    impl Ovc {
        #[doc = "0 No Over voltage        condition or event active."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 VDD        Over voltage condition event indication as configured in        EVROVMON  160    160 EVRMONCTRL register."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Evr33_SPEC;
    pub type Evr33 = crate::EnumBitfieldStruct<u8, Evr33_SPEC>;
    impl Evr33 {
        #[doc = "0 EVR33 is        inactive."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 EVR33 is        active."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ov33_SPEC;
    pub type Ov33 = crate::EnumBitfieldStruct<u8, Ov33_SPEC>;
    impl Ov33 {
        #[doc = "0 No over voltage        condition or event active."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 VDDP3        Over voltage event indication as configured in EVROVMON  160    160 EVRMONCTRL        register."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ovswd_SPEC;
    pub type Ovswd = crate::EnumBitfieldStruct<u8, Ovswd_SPEC>;
    impl Ovswd {
        #[doc = "0 No over voltage        condition or event active."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 VEXT        Over voltage event indication as configured in EVROVMON  160    160 EVRMONCTRL        register."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Uvc_SPEC;
    pub type Uvc = crate::EnumBitfieldStruct<u8, Uvc_SPEC>;
    impl Uvc {
        #[doc = "0 No        under voltage condition or event active."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 VDD        Under voltage event indication as configured in EVRUVMON  160    160 EVRMONCTRL        register."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Uv33_SPEC;
    pub type Uv33 = crate::EnumBitfieldStruct<u8, Uv33_SPEC>;
    impl Uv33 {
        #[doc = "0 No        under voltage condition or event active."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 VDDP3        Under voltage event indication as configured in EVRUVMON  160    160 EVRMONCTRL        register."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Uvswd_SPEC;
    pub type Uvswd = crate::EnumBitfieldStruct<u8, Uvswd_SPEC>;
    impl Uvswd {
        #[doc = "0 No        under voltage condition or event active."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 VEXT        Under voltage event indication as configured in EVRUVMON  160    160 EVRMONCTRL        register."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Synclck_SPEC;
    pub type Synclck = crate::EnumBitfieldStruct<u8, Synclck_SPEC>;
    impl Synclck {
        #[doc = "0 EVRC regulator        runs on internal configured switching frequency and is not currently        synchronized to external DCDCSYNCI input signal."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 EVRC regulator        switching frequency and VGATE output edge is currently synchronized to        external DCDCSYNCI input signal."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Evr33Vok_SPEC;
    pub type Evr33Vok = crate::EnumBitfieldStruct<u8, Evr33Vok_SPEC>;
    impl Evr33Vok {
        #[doc = "0 EVR33        ramp up time has not elapsed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 EVR33 ramp up        time has elapsed."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rstc_SPEC;
    pub type Rstc = crate::EnumBitfieldStruct<u8, Rstc_SPEC>;
    impl Rstc {
        #[doc = "0 No cold reset        trigger signal is active after spike filter and core VDD voltage output        is above the selected reset trim value ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A cold reset        trigger signal is active after spike filter and core VDD voltage output        is below the selected reset trim value ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst33_SPEC;
    pub type Rst33 = crate::EnumBitfieldStruct<u8, Rst33_SPEC>;
    impl Rst33 {
        #[doc = "0 No cold reset        trigger signal is active after spike filter and 3.3  160 V VDDP3 voltage        output is above the selected reset trim value ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A cold reset        trigger signal is active after spike filter and 3.3  160 V VDDP3 voltage        output is below the selected reset trim value ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rstswd_SPEC;
    pub type Rstswd = crate::EnumBitfieldStruct<u8, Rstswd_SPEC>;
    impl Rstswd {
        #[doc = "0 No cold reset        trigger signal is active after spike filter and VEXT voltage input is        above the selected reset trim value ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A cold reset        trigger signal is active after spike filter and VEXT voltage input is        below the selected reset trim value ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Evrcshlv_SPEC;
    pub type Evrcshlv = crate::EnumBitfieldStruct<u8, Evrcshlv_SPEC>;
    impl Evrcshlv {
        #[doc = "0 No short to        ground detected on VDD rail."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Short to ground        detected on VDD rail."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Evrcshhv_SPEC;
    pub type Evrcshhv = crate::EnumBitfieldStruct<u8, Evrcshhv_SPEC>;
    impl Evrcshhv {
        #[doc = "0 No short to        supply detected on VDD rail."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Short to supply        detected on VDD rail."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Evr33Shlv_SPEC;
    pub type Evr33Shlv = crate::EnumBitfieldStruct<u8, Evr33Shlv_SPEC>;
    impl Evr33Shlv {
        #[doc = "0 No short to        ground detected on VDDP3 rail."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Short to ground        detected on VDDP3 rail."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Evr33Shhv_SPEC;
    pub type Evr33Shhv = crate::EnumBitfieldStruct<u8, Evr33Shhv_SPEC>;
    impl Evr33Shhv {
        #[doc = "0 No short to        supply detected on VDDP3 rail."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Short to supply        detected on VDDP3 rail."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Swdlvl_SPEC;
    pub type Swdlvl = crate::EnumBitfieldStruct<u8, Swdlvl_SPEC>;
    impl Swdlvl {
        #[doc = "0 VEXT        external supply is above the threshold."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 VEXT external        supply is below the threshold."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sdvok_SPEC;
    pub type Sdvok = crate::EnumBitfieldStruct<u8, Sdvok_SPEC>;
    impl Sdvok {
        #[doc = "0 EVRC        regulator setpoint voltage has not been reached."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 EVRC regulator        setpoint voltage is reached and VDD voltage is ok."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Evrcmod_SPEC;
    pub type Evrcmod = crate::EnumBitfieldStruct<u8, Evrcmod_SPEC>;
    impl Evrcmod {
        #[doc = "00 SMPS Normal        PWM Mode  PWM   The step down converter is in normal operational closed        loop state. Both Pch. MOSFET and Nch. MOSFET are being switched."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 SMPS Low Power        Mode  LPM   The step down converter is in low power state .        Only Pch. MOSFET is being switched and Nch. MOSFET behaves like a diode."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 SMPS Start up        Mode  STRT   The step down converter is in start up phase. Only Pch.        MOSFET is being switched and Nch. MOSFET behaves like a diode."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 EVRC is        disabled."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Ovpre_SPEC;
    pub type Ovpre = crate::EnumBitfieldStruct<u8, Ovpre_SPEC>;
    impl Ovpre {
        #[doc = "0 No over voltage        condition happened."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 VDDPD        Over voltage event indication as configured in EVROVMON2 register."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ovsb_SPEC;
    pub type Ovsb = crate::EnumBitfieldStruct<u8, Ovsb_SPEC>;
    impl Ovsb {
        #[doc = "0 No over voltage        condition happened."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 VEVRSB        Over voltage event indication as configured in EVROVMON2 register."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ovddm_SPEC;
    pub type Ovddm = crate::EnumBitfieldStruct<u8, Ovddm_SPEC>;
    impl Ovddm {
        #[doc = "0 No over voltage        condition happened."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 VDDM        Over voltage event indication as configured in EVROVMON2 register."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Uvpre_SPEC;
    pub type Uvpre = crate::EnumBitfieldStruct<u8, Uvpre_SPEC>;
    impl Uvpre {
        #[doc = "0 No        under voltage condition happened."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 VDDPD        Under voltage event indication as configured in EVRUVMON2 register."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Uvsb_SPEC;
    pub type Uvsb = crate::EnumBitfieldStruct<u8, Uvsb_SPEC>;
    impl Uvsb {
        #[doc = "0 No        under voltage condition happened."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 VEVRSB        Under voltage event indication as configured in EVRUVMON2 register."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Uvddm_SPEC;
    pub type Uvddm = crate::EnumBitfieldStruct<u8, Uvddm_SPEC>;
    impl Uvddm {
        #[doc = "0 No        under voltage condition happened."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 VDDM        Under voltage event indication as configured in EVRUVMON2 register."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrtrim_SPEC;
impl crate::sealed::RegSpec for Evrtrim_SPEC {
    type DataType = u32;
}
#[doc = "EVR Trim Control Register\n resetvalue={LVD Reset:0x080006C9E,Cold PORST:0x080006C9E,After SSW execution:0x6C9E}"]
pub type Evrtrim = crate::RegValueT<Evrtrim_SPEC>;

impl Evrtrim {
    #[doc = "EVR33 Regulator Output Voltage Target Value   EVR33VOUTSEL. The VDDP3 output level of the EVR33 LDO regulator. The ramp up        completion to the new target value is indicated via EVRSTAT.EVR33VOK        bit. The  EVR33VOUTSEL   EVR33VOUTTRIM  setpoint value shall be        programmed between 0x24 and 0xDA for valid closed loop PID regulator        function. 3.3 V   9E   158 EVR33VOUTSEL     VDDP3   937.5 mV    LSB  VDDP3   937.5 mV   LSB   EVR33VOUTSEL LSB   15 mV"]
    #[inline(always)]
    pub fn evr33voutsel(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrtrim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrtrim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "EVRC Regulator Output Voltage Target Value   SDVOUTSEL. The VDD output level of the Step down regulator.  vosel target i  1.25 V   6C   108 SDVOUTSEL     VDD   712.5 mV    LSB   VDD   712.5 mV   LSB   SDVOUTSEL  LSB   5 mV. This register bitfield requires a parameter update via EVRSDCTRL0.UP for        transfer to EVRC SMPS shadow register. The reaching of the new target        value is indicated via EVRSTAT.SDVOK bit."]
    #[inline(always)]
    pub fn sdvoutsel(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrtrim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrtrim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "EVR33 Regulator Output Voltage Trim Value   EVR33VOUTTRIM. The 6 bit ADC BIST trimming value offset added to the EVR33 output level        value installed by firmware from the flash. This        field is meant for trimming during production tests. VDDP3 Setpoint   EVR33VOUTSEL   EVR33VOUTTRIM  signed value  EVR33OUTTRIM RANGE    32 to 31 LSB LSB   15 mV"]
    #[inline(always)]
    pub fn evr33vouttrim(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Evrtrim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Evrtrim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "EVRC Regulator Output Voltage Trim Value vtrim trim i    SDVOUTTRIM. The 6 bit ADC BIST trimming value offset added to the EVRC output level        value installed by firmware from the flash. The reaching of the new        setpoint is indicated via EVRSTAT.SDVOK bits VDD Setpoint   SDVOUTSEL   SDVOUTTRIM  signed value  SDVOUTTRIM RANGE    32 to 31 LSB LSB   5 mV This register bitfield requires a parameter update via EVRSDCTRL0.UP for        transfer to SMPS shadow register."]
    #[inline(always)]
    pub fn sdvouttrim(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, Evrtrim_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3f,1,0,u8, Evrtrim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, evrtrim::Slck, Evrtrim_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,evrtrim::Slck, Evrtrim_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates if the register can be updated with a new value or if        the register is locked and a write action from the bus side has no        effect. When EVRSDCTRL0.UP bit is set to 1  register is locked for shadow        register update and LCK bit is set to 1. Once shadow register update is        done  the lock is released and LCK bit is set to 0."]
    #[inline(always)]
    pub fn lck(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, evrtrim::Lck, Evrtrim_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<31,0x1,1,0,evrtrim::Lck, Evrtrim_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrtrim {
    #[inline(always)]
    fn default() -> Evrtrim {
        <crate::RegValueT<Evrtrim_SPEC> as RegisterValue<_>>::new(27806)
    }
}
pub mod evrtrim {
    pub struct Slck_SPEC;
    pub type Slck = crate::EnumBitfieldStruct<u8, Slck_SPEC>;
    impl Slck {
        #[doc = "0 No lock active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Lock is active"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lck_SPEC;
    pub type Lck = crate::EnumBitfieldStruct<u8, Lck_SPEC>;
    impl Lck {
        #[doc = "0 The        register is unlocked and can be updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The register is        locked and cannot be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrtrim2_SPEC;
impl crate::sealed::RegSpec for Evrtrim2_SPEC {
    type DataType = u32;
}
#[doc = "EVR Trim Control Register 2\n resetvalue={LVD Reset:0x2C0C2FF,Cold PORST:0x2C0C2FF,After SSW execution:0x2C0C2FF}"]
pub type Evrtrim2 = crate::RegValueT<Evrtrim2_SPEC>;

impl Evrtrim2 {
    #[doc = "VDD Supply Primary ADC Counter LPF Coefficient   ADCCLPF. This bitfield configures the coefficient of the Low Pass Filter of the        VDD  160    160 EVRC Primary Monitor tracking ADC counter value. The Low Pass        filter is used for cold PORST generation and EVRADCSTAT value. y  160  k   160    160    160 y  160  k 1   160    160  1 a   160    160    160    160 x  160  k   160    160 a   160   y  160  k  is filter output         x  160  k  is ADC output a  160    160  1  160    160  2  160    160 LPF  .If LPF  160    160 0  the filter output is the same as ADC        output."]
    #[inline(always)]
    pub fn adcclpf(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Evrtrim2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Evrtrim2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDP3 Supply Primary ADC Counter LPF Coefficient   ADC33LPF. This bitfield configures the coefficient of the Low Pass Filter of the        VDDP3  160    160 EVR33 Primary Monitor tracking ADC counter value. The Low Pass        filter is used for cold PORST generation and EVRADCSTAT value. y  160  k   160    160    160 y  160  k 1   160    160  1 a   160    160    160    160 x  160  k   160    160 a   160   y  160  k  is filter output         x  160  k  is ADC output a  160    160  1  160    160  2  160    160 LPF  .If LPF  160    160 0  the filter output is the same as ADC        output."]
    #[inline(always)]
    pub fn adc33lpf(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Evrtrim2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Evrtrim2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEXT Supply Primary ADC Counter LPF Coefficient   ADCSWDLPF. This bitfield configures the coefficient of the Low Pass Filter of the        external VEXT  3.3V  160    160 5V  Primary Monitor tracking ADC counter value.        The Low Pass filter is used for cold PORST generation and EVRADCSTAT        value. y  160  k   160    160    160 y  160  k 1   160    160  1 a   160    160    160    160 x  160  k   160    160 a   160   y  160  k  is filter output         x  160  k  is ADC output a  160    160  1  160    160  2  160    160 LPF  .If LPF  160    160 0  the filter output is the same as ADC        output."]
    #[inline(always)]
    pub fn adcswdlpf(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Evrtrim2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Evrtrim2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDP3 Supply Primary ADC Counter LPF Coefficient for HSM   HSM33LPF. This bitfield configures the coefficient of the Low Pass Filter of the        VDDP3  160    160 EVR33 Primary Monitor tracking ADC counter value for HSM. y  160  k   160    160    160 y  160  k 1   160    160  1 a   160    160    160    160 x  160  k   160    160 a   160   y  160  k  is filter output         x  160  k  is ADC output a  160    160  1  160    160  2  160    160 LPF  .If LPF  160    160 0  the filter output is the same as ADC        output. 00   2 clock delay   80 ns nominal 01   3 clock delay   120 ns nominal 10   5 clock delay   200 ns nominal 11   9 clock delay   360 ns nominal"]
    #[inline(always)]
    pub fn hsm33lpf(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Evrtrim2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Evrtrim2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDP3 Supply Primary ADC Offset Trim Value   ADC33OFFS. The 6 bit ADC BIST trimming value for offset compensation added to the        VDDP3 output level value installed by firmware from the flash. The range        is from  32 to 31 LSB."]
    #[inline(always)]
    pub fn adc33offs(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, Evrtrim2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3f,1,0,u8, Evrtrim2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEXT Supply Primary ADC Counter LPF Coefficient for HSM   HSMSWDLPF. This bitfield configures the coefficient of the Low Pass Filter of the        external VEXT  3.3V  160    160 5V  Primary Monitor tracking ADC counter value for        HSM. y  160  k   160    160    160 y  160  k 1   160    160  1 a   160    160    160    160 x  160  k   160    160 a   160   y  160  k  is filter output         x  160  k  is ADC output a  160    160  1  160    160  2  160    160 LPF  .If LPF  160    160 0  the filter output is the same as ADC        output. 00   2 clock delay   80 ns nominal 01   3 clock delay   120 ns nominal 10   5 clock delay   200 ns nominal 11   9 clock delay   360 ns nominal"]
    #[inline(always)]
    pub fn hsmswdlpf(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Evrtrim2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Evrtrim2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEXT Supply Primary ADC Offset Trim Value   ADCSWDOFFS. The 6 bit ADC BIST trimming value for offset compensation added to the        VEXT output level value installed by firmware from the flash. The range        is from  32 to 31 LSB."]
    #[inline(always)]
    pub fn adcswdoffs(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Evrtrim2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Evrtrim2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDD Supply Primary ADC Counter LPF Coefficient for HSM   HSMCLPF. This bitfield configures the coefficient of the Low Pass Filter of the        VDD  160    160 EVRC Primary Monitor tracking ADC counter value for HSM. y  160  k   160    160    160 y  160  k 1   160    160  1 a   160    160    160    160 x  160  k   160    160 a   160   y  160  k  is filter output         x  160  k  is ADC output a  160    160  1  160    160  2  160    160 LPF  .If LPF  160    160 0  the filter output is the same as ADC        output. 00   2 clock delay   80 ns nominal 01   3 clock delay   120 ns nominal 10   5 clock delay   200 ns nominal 11   9 clock delay   360 ns nominal"]
    #[inline(always)]
    pub fn hsmclpf(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, Evrtrim2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8, Evrtrim2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDD Supply Primary ADC Offset Trim Value   ADCCOFFS. The 6 bit ADC BIST trimming value for offset compensation added to the        VDD output level value installed by firmware from the flash. The range        is from  32 to 31 LSB."]
    #[inline(always)]
    pub fn adccoffs(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, Evrtrim2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3f,1,0,u8, Evrtrim2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, evrtrim2::Slck, Evrtrim2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            evrtrim2::Slck,
            Evrtrim2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evrtrim2 {
    #[inline(always)]
    fn default() -> Evrtrim2 {
        <crate::RegValueT<Evrtrim2_SPEC> as RegisterValue<_>>::new(46187263)
    }
}
pub mod evrtrim2 {
    pub struct Slck_SPEC;
    pub type Slck = crate::EnumBitfieldStruct<u8, Slck_SPEC>;
    impl Slck {
        #[doc = "0 No lock active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Lock is active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evrtrimstat_SPEC;
impl crate::sealed::RegSpec for Evrtrimstat_SPEC {
    type DataType = u32;
}
#[doc = "EVR Trim Status Register\n resetvalue={LVD Reset:0x6C9E,Cold PORST:0x6C9E}"]
pub type Evrtrimstat = crate::RegValueT<Evrtrimstat_SPEC>;

impl Evrtrimstat {
    #[doc = "EVR33 Regulator Output Voltage Target Value   EVR33VOUTSEL. This bitfield indicates EVR33 output target value as configured in        EVTRIM.EVR33VOUTSEL."]
    #[inline(always)]
    pub fn evr33voutsel(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evrtrimstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evrtrimstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EVRC Regulator Output Voltage Target Value   SDVOUTSEL. This bit field indicates the EVRC output level of the Step down        regulator as configured in EVTRIM.SDVOUTSEL.  vosel target o"]
    #[inline(always)]
    pub fn sdvoutsel(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evrtrimstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evrtrimstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EVR33 Regulator Output Voltage Trim Value   EVR33VOUTTRIM. This bit field indicates the 6 bit ADC BIST trimming value offset added        to the EVR33 output level value installed by firmware from flash        configuration sector if production trimming is required."]
    #[inline(always)]
    pub fn evr33vouttrim(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Evrtrimstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Evrtrimstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "EVRC Regulator Output Voltage Trim Value vtrim trim o    SDVOUTTRIM. This bit field indicates the 5 bit ADC BIST trimming value offset added        to the EVRC output level value installed by firmware from flash        configuration sector as configured in EVTRIM.SDVOUTTRIM."]
    #[inline(always)]
    pub fn sdvouttrim(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, Evrtrimstat_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x3f,1,0,u8, Evrtrimstat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Evrtrimstat {
    #[inline(always)]
    fn default() -> Evrtrimstat {
        <crate::RegValueT<Evrtrimstat_SPEC> as RegisterValue<_>>::new(27806)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evruvmon_SPEC;
impl crate::sealed::RegSpec for Evruvmon_SPEC {
    type DataType = u32;
}
#[doc = "EVR Secondary Under voltage Monitor Register\n resetvalue={LVD Reset:0x75A7B8,Cold PORST:0x75A7B8,After SSW execution:0x75A7B8}"]
pub type Evruvmon = crate::RegValueT<Evruvmon_SPEC>;

impl Evruvmon {
    #[doc = "VDD Supply Secondary Monitor Under voltage threshold   EVRCUVVAL. This field defines the under voltage monitoring threshold level of the        EVRC regulator output or VDD supply. Ideal Threshold     VIN  160    160 LSB   160    160 1  Ideal LSB   5.7692  160 mV"]
    #[inline(always)]
    pub fn evrcuvval(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evruvmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evruvmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDP3 Supply Secondary Monitor Under voltage threshold   EVR33UVVAL. This field defines the under voltage monitoring threshold level of the        EVR33 regulator output or VDDP3 supply. Ideal Threshold     VIN  160    160 LSB   160    160 1 . Ideal LSB   15.00  160 mV"]
    #[inline(always)]
    pub fn evr33uvval(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evruvmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evruvmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEXT Supply Secondary Monitor Under voltage threshold   SWDUVVAL. This field defines the under voltage threshold level of the external        VEXT supply monitor. Ideal Threshold     VIN  160    160 LSB   160    160 1  Ideal LSB   23.077  160 mV."]
    #[inline(always)]
    pub fn swduvval(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evruvmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evruvmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, evruvmon::Slck, Evruvmon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            evruvmon::Slck,
            Evruvmon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evruvmon {
    #[inline(always)]
    fn default() -> Evruvmon {
        <crate::RegValueT<Evruvmon_SPEC> as RegisterValue<_>>::new(7710648)
    }
}
pub mod evruvmon {
    pub struct Slck_SPEC;
    pub type Slck = crate::EnumBitfieldStruct<u8, Slck_SPEC>;
    impl Slck {
        #[doc = "0 No lock active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Lock is active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evruvmon2_SPEC;
impl crate::sealed::RegSpec for Evruvmon2_SPEC {
    type DataType = u32;
}
#[doc = "EVR Secondary Under voltage Monitor Register 2\n resetvalue={LVD Reset:0x2A7000BC,Cold PORST:0x2A7000BC,After SSW execution:0x2A7000BC}"]
pub type Evruvmon2 = crate::RegValueT<Evruvmon2_SPEC>;

impl Evruvmon2 {
    #[doc = "VDDPD Supply Secondary Monitor Under voltage threshold   PREUVVAL. This field defines the under voltage monitoring threshold level of the        VDDPD supply or EVRPR output. Ideal Threshold     VIN  160    160 LSB   160    160 1  Ideal LSB   5.7692  160 mV"]
    #[inline(always)]
    pub fn preuvval(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Evruvmon2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Evruvmon2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDM Supply Secondary Monitor Under voltage threshold   VDDMUVVAL. This field defines the under voltage monitoring threshold level of the        VDDM ADC supply. Ideal Threshold     VIN  160    160 LSB   160    160 1  Ideal LSB   23.077  160 mV"]
    #[inline(always)]
    pub fn vddmuvval(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Evruvmon2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Evruvmon2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEVRSB Supply Secondary Monitor Under voltage threshold   SBUVVAL. This field defines the under voltage threshold level of the external        VEVRSB  3.3V  160    160 5V  standby supply monitor. Ideal Threshold     VIN  160    160 LSB   160    160 1  Ideal LSB   23.077  160 mV."]
    #[inline(always)]
    pub fn sbuvval(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Evruvmon2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Evruvmon2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDM Level Select   VDDMLVLSEL. This field defines the under voltage monitoring threshold level required        by EVADC  160    160 EDSADC modules to differentiate between 5  160 V or 3.3  160 V VDDM        supply level to adjust analog behavior to the actual voltage level. The        6 MSB bits of the ADC result is compared against VDDMLVLSEL with 4 LSB        hysteresis. Ideal Threshold     VIN   LSB    1  Ideal LSB   92.308 mV"]
    #[inline(always)]
    pub fn vddmlvlsel(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, Evruvmon2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3f,1,0,u8, Evruvmon2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        evruvmon2::Slck,
        Evruvmon2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            evruvmon2::Slck,
            Evruvmon2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evruvmon2 {
    #[inline(always)]
    fn default() -> Evruvmon2 {
        <crate::RegValueT<Evruvmon2_SPEC> as RegisterValue<_>>::new(711983292)
    }
}
pub mod evruvmon2 {
    pub struct Slck_SPEC;
    pub type Slck = crate::EnumBitfieldStruct<u8, Slck_SPEC>;
    impl Slck {
        #[doc = "0 No lock active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Lock is active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsmovmon_SPEC;
impl crate::sealed::RegSpec for Hsmovmon_SPEC {
    type DataType = u32;
}
#[doc = "EVR Primary HSM Over voltage Monitor Register\n resetvalue={LVD Reset:0x0E1B586,Cold PORST:0x0E1B586,After SSW execution:0x0E1B586}"]
pub type Hsmovmon = crate::RegValueT<Hsmovmon_SPEC>;

impl Hsmovmon {
    #[doc = "VDD Supply Primary Monitor Alarm Over voltage threshold   EVRCOVVAL. This field defines the over voltage monitoring threshold level of the        EVRC regulator output or VDD supply. EVRCOVVAL     VDDx   712.5 mV    LSB  LSB   5 mV"]
    #[inline(always)]
    pub fn evrcovval(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Hsmovmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Hsmovmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDP3 Supply Primary Monitor Alarm Over voltage threshold   EVR33OVVAL. This field defines the over voltage monitoring threshold level of the        EVR33 regulator output or VDDP3 supply. EVR33OVVAL     VDDx   937.5 mV    LSB  LSB   15 mV"]
    #[inline(always)]
    pub fn evr33ovval(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Hsmovmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Hsmovmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEXT Supply Primary Monitor Alarm Over voltage threshold   SWDOVVAL. This field defines the over voltage threshold level of the external VEXT        supply monitor. SWDOVVAL     VDDx   1050 mV    LSB  LSB   20 mV"]
    #[inline(always)]
    pub fn swdovval(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Hsmovmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Hsmovmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDD Primary Monitor OV Alarm Disable   EVRCOFF"]
    #[inline(always)]
    pub fn evrcoff(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        hsmovmon::Evrcoff,
        Hsmovmon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            hsmovmon::Evrcoff,
            Hsmovmon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VDDP3 Primary Monitor OV Alarm Disable   EVR33OFF"]
    #[inline(always)]
    pub fn evr33off(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        hsmovmon::Evr33Off,
        Hsmovmon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            hsmovmon::Evr33Off,
            Hsmovmon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VEXT Primary Monitor OV Alarm Disable   SWDOFF"]
    #[inline(always)]
    pub fn swdoff(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        hsmovmon::Swdoff,
        Hsmovmon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            hsmovmon::Swdoff,
            Hsmovmon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, hsmovmon::Slck, Hsmovmon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            hsmovmon::Slck,
            Hsmovmon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Hsmovmon {
    #[inline(always)]
    fn default() -> Hsmovmon {
        <crate::RegValueT<Hsmovmon_SPEC> as RegisterValue<_>>::new(14792070)
    }
}
pub mod hsmovmon {
    pub struct Evrcoff_SPEC;
    pub type Evrcoff = crate::EnumBitfieldStruct<u8, Evrcoff_SPEC>;
    impl Evrcoff {
        #[doc = "0 A alarm trigger signal is generated and forwarded to the HSM by the EVRC block depending on the EVRCOVVAL configured value ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No alarm trigger signal is generated and forwarded to the HSM by the EVRC block depending on the selected reset trim value ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Evr33Off_SPEC;
    pub type Evr33Off = crate::EnumBitfieldStruct<u8, Evr33Off_SPEC>;
    impl Evr33Off {
        #[doc = "0 A alarm trigger signal is generated and forwarded to the HSM by the EVR33 block depending on the EVR33OVVAL configured value ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No alarm trigger signal is generated and forwarded to the HSM by the EVR33 block depending on the selected reset trim value ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Swdoff_SPEC;
    pub type Swdoff = crate::EnumBitfieldStruct<u8, Swdoff_SPEC>;
    impl Swdoff {
        #[doc = "0 A alarm trigger signal is generated and forwarded to the HSM by the SWD block depending on the SWDOVVAL configured value ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No alarm trigger signal is generated and forwarded to the HSM by the SWD block depending on the selected reset trim value ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Slck_SPEC;
    pub type Slck = crate::EnumBitfieldStruct<u8, Slck_SPEC>;
    impl Slck {
        #[doc = "0 No lock active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Lock is active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsmuvmon_SPEC;
impl crate::sealed::RegSpec for Hsmuvmon_SPEC {
    type DataType = u32;
}
#[doc = "EVR Primary HSM Under voltage Monitor Register\n resetvalue={LVD Reset:0x5C824D,Cold PORST:0x5C824D,After SSW execution:0x5C824D}"]
pub type Hsmuvmon = crate::RegValueT<Hsmuvmon_SPEC>;

impl Hsmuvmon {
    #[doc = "VDD Supply Primary Monitor Alarm Under voltage threshold   EVRCUVVAL. This field defines the under voltage monitoring threshold level of the        EVRC regulator output or VDD supply.EVRCUVVAL     VDDx   712.5 mV    LSB  LSB   5 mV"]
    #[inline(always)]
    pub fn evrcuvval(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Hsmuvmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Hsmuvmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDDP3 Supply Primary Monitor Alarm Under voltage threshold   EVR33UVVAL. This field defines the under voltage monitoring threshold level of the        EVR33 regulator output or VDDP3 supply. EVR33UVVAL     VDDx   937.5 mV    LSB  LSB   15 mV"]
    #[inline(always)]
    pub fn evr33uvval(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Hsmuvmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Hsmuvmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VEXT Supply Primary Monitor Alarm Under voltage threshold   SWDUVVAL. This field defines the under voltage threshold level of the external        VEXT supply monitor. SWDUVVAL     VDDx   1050 mV    LSB  LSB   20 mV"]
    #[inline(always)]
    pub fn swduvval(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Hsmuvmon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Hsmuvmon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "VDD Primary Monitor UV Alarm Disable   EVRCOFF"]
    #[inline(always)]
    pub fn evrcoff(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        hsmuvmon::Evrcoff,
        Hsmuvmon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            hsmuvmon::Evrcoff,
            Hsmuvmon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VDDP3 Primary Monitor UV Alarm Disable   EVR33OFF"]
    #[inline(always)]
    pub fn evr33off(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        hsmuvmon::Evr33Off,
        Hsmuvmon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            hsmuvmon::Evr33Off,
            Hsmuvmon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "VEXT Primary Monitor UV Alarm Disable   SWDOFF"]
    #[inline(always)]
    pub fn swdoff(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        hsmuvmon::Swdoff,
        Hsmuvmon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            hsmuvmon::Swdoff,
            Hsmuvmon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "HSM Voltage Filter   HSMFIL"]
    #[inline(always)]
    pub fn hsmfil(
        self,
    ) -> crate::common::RegisterField<
        27,
        0xf,
        1,
        0,
        hsmuvmon::Hsmfil,
        Hsmuvmon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0xf,
            1,
            0,
            hsmuvmon::Hsmfil,
            Hsmuvmon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "HSM Security Lock   SLCK. If this bit is set  all other bits in this register can no longer be        written. Write requests to other bits when SLCK is set will trigger an        SLCK access error alarm.This bit can not by cleared by software. SLCK        bit can only be set by an access from the HSM master  TAG  160    160 000011 B  .        A set operation performed by any other master or software is ignored and        the bit is kept as cleared."]
    #[inline(always)]
    pub fn slck(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, hsmuvmon::Slck, Hsmuvmon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            hsmuvmon::Slck,
            Hsmuvmon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Hsmuvmon {
    #[inline(always)]
    fn default() -> Hsmuvmon {
        <crate::RegValueT<Hsmuvmon_SPEC> as RegisterValue<_>>::new(6062669)
    }
}
pub mod hsmuvmon {
    pub struct Evrcoff_SPEC;
    pub type Evrcoff = crate::EnumBitfieldStruct<u8, Evrcoff_SPEC>;
    impl Evrcoff {
        #[doc = "0 A alarm trigger        signal is generated and forwarded to the HSM by the EVRC block depending        on the EVRCUVVAL configured value ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No alarm trigger signal is generated and forwarded to the HSM by the EVRC block depending on the selected reset trim value ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Evr33Off_SPEC;
    pub type Evr33Off = crate::EnumBitfieldStruct<u8, Evr33Off_SPEC>;
    impl Evr33Off {
        #[doc = "0 A alarm trigger signal is generated and forwarded to the HSM by the EVR33 block depending on the EVR33UVVAL configured value ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No alarm trigger signal is generated and forwarded to the HSM by the EVR33 block depending on the selected reset trim value ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Swdoff_SPEC;
    pub type Swdoff = crate::EnumBitfieldStruct<u8, Swdoff_SPEC>;
    impl Swdoff {
        #[doc = "0 A alarm trigger signal is generated and forwarded to the HSM by the SWD block depending on the SWDUVVAL configured value ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No alarm trigger signal is generated and forwarded to the HSM by the SWD block depending on the selected reset trim value ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hsmfil_SPEC;
    pub type Hsmfil = crate::EnumBitfieldStruct<u8, Hsmfil_SPEC>;
    impl Hsmfil {
        #[doc = "0 Each conversion result is compared with threshold to generate alarm"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "F A spike filter        of consecutive 16 ADC results are used to generate alarm to HSM."]
        pub const CONST_1515: Self = Self::new(15);
    }
    pub struct Slck_SPEC;
    pub type Slck = crate::EnumBitfieldStruct<u8, Slck_SPEC>;
    impl Slck {
        #[doc = "0 No lock active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Lock is active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Identification Register\n resetvalue={Application Reset:0x0E8C001}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MODREV. This bit field indicates the revision number of the PMS module. This is intended as a continuous numbering scheme defined by Design."]
    #[inline(always)]
    pub fn modrev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, id::Modrev, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,id::Modrev, Id_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Module Type   MODTYPE. This bit field is fixed coded as C0 H .        It defines a 32 bit module."]
    #[inline(always)]
    pub fn modtype(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number   MODNUMBER. This bit field defines the module identification number. The identification number for the PMS is 00E8 H ."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(15253505)
    }
}
pub mod id {
    pub struct Modrev_SPEC;
    pub type Modrev = crate::EnumBitfieldStruct<u8, Modrev_SPEC>;
    impl Modrev {
        #[doc = "00000001 TA step        version"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "000000XX TB step        version"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "000000XX TB step        version"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "000000XX TB step        version"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "00000XXX TC step        version"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "00000XXX TC step        version"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "00000XXX TC step        version"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "00000XXX TC step        version"]
        pub const CONST_77: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Monbistctrl_SPEC;
impl crate::sealed::RegSpec for Monbistctrl_SPEC {
    type DataType = u32;
}
#[doc = "SMU stdby BIST Control Register\n resetvalue={LVD Reset:0x0,Warm PORST:0x0}"]
pub type Monbistctrl = crate::RegValueT<Monbistctrl_SPEC>;

impl Monbistctrl {
    #[doc = "SMU stdby alarm BIST enable   TSTEN. If SMUEN in the CMD stdby register is set to 1  setting the TSTEN bit triggers SMU stdby BIST tests to test the alarm path and PMS components. This bit can only be changed if bit BITPROT is set in parallel. This bit is cleared by hardware at the end of the SMU stdby BIST operation."]
    #[inline(always)]
    pub fn tsten(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        monbistctrl::Tsten,
        Monbistctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            monbistctrl::Tsten,
            Monbistctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "SMU stdby BIST flag clear   TSTCLR. Setting this bit enables the clearing of the bitfields TSTOK  TSTDONE  TSTRUN  SMUERR and PMSERR of register MONBISTSTAT."]
    #[inline(always)]
    pub fn tstclr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        monbistctrl::Tstclr,
        Monbistctrl_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            monbistctrl::Tstclr,
            Monbistctrl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Bit Protection TSTEN   BITPROT. Setting this bit enables that bit TSTEN can be changed in this write        operation. This bit is read as zero."]
    #[inline(always)]
    pub fn bitprot(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Monbistctrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30,1,0,Monbistctrl_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Monbistctrl {
    #[inline(always)]
    fn default() -> Monbistctrl {
        <crate::RegValueT<Monbistctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod monbistctrl {
    pub struct Tsten_SPEC;
    pub type Tsten = crate::EnumBitfieldStruct<u8, Tsten_SPEC>;
    impl Tsten {
        #[doc = "0 SMU stdby BIST        test is inactive"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SMU stdby BIST test triggered. When the SMU stdby BIST is activated  the alarm signals tested by the bist are not propagated to the SMU core."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tstclr_SPEC;
    pub type Tstclr = crate::EnumBitfieldStruct<u8, Tstclr_SPEC>;
    impl Tstclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The bitfields TSTOK  TSTDONE  TSTRUN  SMUERR and PMSERR of register MONBISTSTAT are cleared. Before triggering SMU stdby BIST mechanism via TSTEN  previous test results need to be cleared via TSTCLR."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Monbiststat_SPEC;
impl crate::sealed::RegSpec for Monbiststat_SPEC {
    type DataType = u32;
}
#[doc = "SMU stdby BIST Status Register\n resetvalue={LVD Reset:0x0,Warm PORST:0x0}"]
pub type Monbiststat = crate::RegValueT<Monbiststat_SPEC>;

impl Monbiststat {
    #[doc = "SMU stdby BIST ok bit   TSTOK. This status bit indicates that MONBIST test was successful and the        result is as expected."]
    #[inline(always)]
    pub fn tstok(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        monbiststat::Tstok,
        Monbiststat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            monbiststat::Tstok,
            Monbiststat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "SMU stdby BIST run bit   TSTRUN. The status bit indicates that MONBIST test is ongoing."]
    #[inline(always)]
    pub fn tstrun(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        monbiststat::Tstrun,
        Monbiststat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            monbiststat::Tstrun,
            Monbiststat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "SMU stdby BIST done bit   TSTDONE. The status bit indicates that MONBIST test is completed."]
    #[inline(always)]
    pub fn tstdone(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        monbiststat::Tstdone,
        Monbiststat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            monbiststat::Tstdone,
            Monbiststat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Error found in SMU stdby found by SMU stdby BIST. This status bit indicates that the MONBIST test found an error in the        SMU stdby."]
    #[inline(always)]
    pub fn smuerr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        monbiststat::Smuerr,
        Monbiststat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            monbiststat::Smuerr,
            Monbiststat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Error found in PMS SARADC by SMU stdby BIST. This status bit indicates that SMU stdby BIST found an error in the PMS        SARADC."]
    #[inline(always)]
    pub fn pmserr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        monbiststat::Pmserr,
        Monbiststat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            monbiststat::Pmserr,
            Monbiststat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Monbiststat {
    #[inline(always)]
    fn default() -> Monbiststat {
        <crate::RegValueT<Monbiststat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod monbiststat {
    pub struct Tstok_SPEC;
    pub type Tstok = crate::EnumBitfieldStruct<u8, Tstok_SPEC>;
    impl Tstok {
        #[doc = "0 The MONBIST        test result is not ok."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The MONBIST        test result is ok."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tstrun_SPEC;
    pub type Tstrun = crate::EnumBitfieldStruct<u8, Tstrun_SPEC>;
    impl Tstrun {
        #[doc = "0 The MONBIST        test is currently inactive."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The MONBIST        test is active."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tstdone_SPEC;
    pub type Tstdone = crate::EnumBitfieldStruct<u8, Tstdone_SPEC>;
    impl Tstdone {
        #[doc = "The MONBIST test is not started."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "The MONBIST test is done."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Smuerr_SPEC;
    pub type Smuerr = crate::EnumBitfieldStruct<u8, Smuerr_SPEC>;
    impl Smuerr {
        #[doc = "No error happened in SMU STDBY module during MONBIST test."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 This bit is set        if MONBIST test failed and the error occurred in SMU STDBY module and        alarm processing path. This status bit is cleared by setting TSTCLR        bit.."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pmserr_SPEC;
    pub type Pmserr = crate::EnumBitfieldStruct<u8, Pmserr_SPEC>;
    impl Pmserr {
        #[doc = "No error happened in PMS module during MONBIST test."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 This bit is set        if MONBIST test failed and the error occurred in Secondary voltage        monitor and alarm generation in PMS. This status bit is cleared by        setting TSTCLR bit."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otsc0_SPEC;
impl crate::sealed::RegSpec for Otsc0_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Trigger Set Control 0 Register\n resetvalue={LVD Reset:0x0,Debug Reset:0x0}"]
pub type Otsc0 = crate::RegValueT<Otsc0_SPEC>;

impl Otsc0 {
    #[doc = "OTGB0 TS16 ADCMON Low Byte   B0LAM"]
    #[inline(always)]
    pub fn b0lam(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, otsc0::B0Lam, Otsc0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,otsc0::B0Lam, Otsc0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OTGB0 TS16 ADCMON High Byte   B0HAM"]
    #[inline(always)]
    pub fn b0ham(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, otsc0::B0Ham, Otsc0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,otsc0::B0Ham, Otsc0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OTGB1 TS16 ADCMON Low Byte   B1LAM"]
    #[inline(always)]
    pub fn b1lam(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, otsc0::B1Lam, Otsc0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xf,1,0,otsc0::B1Lam, Otsc0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OTGB1 TS16 ADCMON High Byte   B1HAM"]
    #[inline(always)]
    pub fn b1ham(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, otsc0::B1Ham, Otsc0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0xf,1,0,otsc0::B1Ham, Otsc0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Otsc0 {
    #[inline(always)]
    fn default() -> Otsc0 {
        <crate::RegValueT<Otsc0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod otsc0 {
    pub struct B0Lam_SPEC;
    pub type B0Lam = crate::EnumBitfieldStruct<u8, B0Lam_SPEC>;
    impl B0Lam {
        #[doc = "0 No Module        selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PRADCCV"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 PRADC33V"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 PRADCSWDV"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "4 PRADCFBCV"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "5 SECADCCV"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "6 SECADC33V"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "7 SECADCSWDV"]
        pub const CONST_77: Self = Self::new(7);
        #[doc = "8 SECADCPRE"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "9 SECADCSB"]
        pub const CONST_99: Self = Self::new(9);
        #[doc = "A SECADCVDDM"]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "B DTSRESULTL"]
        pub const CONST_1111: Self = Self::new(11);
        #[doc = "C DTSRESULTH"]
        pub const CONST_1212: Self = Self::new(12);
    }
    pub struct B0Ham_SPEC;
    pub type B0Ham = crate::EnumBitfieldStruct<u8, B0Ham_SPEC>;
    impl B0Ham {
        #[doc = "0 No Module        selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PRADCCV"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 PRADC33V"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 PRADCSWDV"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "4 PRADCFBCV"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "5 SECADCCV"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "6 SECADC33V"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "7 SECADCSWDV"]
        pub const CONST_77: Self = Self::new(7);
        #[doc = "8 SECADCPRE"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "9 SECADCSB"]
        pub const CONST_99: Self = Self::new(9);
        #[doc = "A SECADCVDDM"]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "B DTSRESULTL"]
        pub const CONST_1111: Self = Self::new(11);
        #[doc = "C DTSRESULTH"]
        pub const CONST_1212: Self = Self::new(12);
    }
    pub struct B1Lam_SPEC;
    pub type B1Lam = crate::EnumBitfieldStruct<u8, B1Lam_SPEC>;
    impl B1Lam {
        #[doc = "0 No Module selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PRADCCV"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 PRADC33V"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 PRADCSWDV"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "4 PRADCFBCV"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "5 SECADCCV"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "6 SECADC33V"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "7 SECADCSWDV"]
        pub const CONST_77: Self = Self::new(7);
        #[doc = "8 SECADCPRE"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "9 SECADCSB"]
        pub const CONST_99: Self = Self::new(9);
        #[doc = "A SECADCVDDM"]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "B DTSRESULTL"]
        pub const CONST_1111: Self = Self::new(11);
        #[doc = "C DTSRESULTH"]
        pub const CONST_1212: Self = Self::new(12);
    }
    pub struct B1Ham_SPEC;
    pub type B1Ham = crate::EnumBitfieldStruct<u8, B1Ham_SPEC>;
    impl B1Ham {
        #[doc = "0 No Module selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PRADCCV"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 PRADC33V"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 PRADCSWDV"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "4 PRADCFBCV"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "5 SECADCCV"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "6 SECADC33V"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "7 SECADCSWDV"]
        pub const CONST_77: Self = Self::new(7);
        #[doc = "8 SECADCPRE"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "9 SECADCSB"]
        pub const CONST_99: Self = Self::new(9);
        #[doc = "A SECADCVDDM"]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "B DTSRESULTL"]
        pub const CONST_1111: Self = Self::new(11);
        #[doc = "C DTSRESULTH"]
        pub const CONST_1212: Self = Self::new(12);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otsc1_SPEC;
impl crate::sealed::RegSpec for Otsc1_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Trigger Set Control 1 Register\n resetvalue={LVD Reset:0x0,Debug Reset:0x0}"]
pub type Otsc1 = crate::RegValueT<Otsc1_SPEC>;

impl Otsc1 {
    #[doc = "OTGB0 TS16 EVRCON   B0EC"]
    #[inline(always)]
    pub fn b0ec(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, otsc1::B0Ec, Otsc1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xf,1,0,otsc1::B0Ec, Otsc1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OTGB1 TS16 EVRCON   B1EC"]
    #[inline(always)]
    pub fn b1ec(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, otsc1::B1Ec, Otsc1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0xf,1,0,otsc1::B1Ec, Otsc1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OTGB0 TS16 EVRCON DMONAD   DMONAD. The multiplexer signal selection documented in DMONAD coding table."]
    #[inline(always)]
    pub fn dmonad(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Otsc1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Otsc1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Otsc1 {
    #[inline(always)]
    fn default() -> Otsc1 {
        <crate::RegValueT<Otsc1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod otsc1 {
    pub struct B0Ec_SPEC;
    pub type B0Ec = crate::EnumBitfieldStruct<u8, B0Ec_SPEC>;
    impl B0Ec {
        #[doc = "0 No Module        selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 EVRCDPWM"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 EVRCOUT"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 EVR33OUT"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "4 WUTCNT"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "5 TCSCRINT"]
        pub const CONST_55: Self = Self::new(5);
    }
    pub struct B1Ec_SPEC;
    pub type B1Ec = crate::EnumBitfieldStruct<u8, B1Ec_SPEC>;
    impl B1Ec {
        #[doc = "0 No Module        selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 EVRCDPWM"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 EVRCOUT"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "3 EVR33OUT"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "4 WUTCNT"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "5 TCSCRINT"]
        pub const CONST_55: Self = Self::new(5);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otss_SPEC;
impl crate::sealed::RegSpec for Otss_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Trigger Set Select Register\n resetvalue={LVD Reset:0x0,Debug Reset:0x0}"]
pub type Otss = crate::RegValueT<Otss_SPEC>;

impl Otss {
    #[doc = "Trigger Set for OTGB0   OTGB0"]
    #[inline(always)]
    pub fn otgb0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, otss::Otgb0, Otss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,otss::Otgb0, Otss_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger Set for OTGB1   OTGB1"]
    #[inline(always)]
    pub fn otgb1(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, otss::Otgb1, Otss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,otss::Otgb1, Otss_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "1 Trigger Set        TS16 ADCMON"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Trigger Set        TS16 EVRCON"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Otgb1_SPEC;
    pub type Otgb1 = crate::EnumBitfieldStruct<u8, Otgb1_SPEC>;
    impl Otgb1 {
        #[doc = "0 No Trigger Set        selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Trigger Set        TS16 ADCMON"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Trigger Set        TS16 EVRCON"]
        pub const CONST_22: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmsien_SPEC;
impl crate::sealed::RegSpec for Pmsien_SPEC {
    type DataType = u32;
}
#[doc = "PMS Interrupt Enable Register\n resetvalue={LVD Reset:0x0,Cold PORST:0x0}"]
pub type Pmsien = crate::RegValueT<Pmsien_SPEC>;

impl Pmsien {
    #[doc = "OVSWD Interrupt enable   OVSWD. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn ovswd(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, pmsien::Ovswd, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,pmsien::Ovswd, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "UVSWD Interrupt enable   UVSWD. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn uvswd(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, pmsien::Uvswd, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,pmsien::Uvswd, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OV33 Interrupt enable   OV33. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn ov33(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, pmsien::Ov33, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,pmsien::Ov33, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "UV33 Interrupt enable   UV33. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn uv33(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, pmsien::Uv33, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,pmsien::Uv33, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OVC Interrupt enable   OVC. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn ovc(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, pmsien::Ovc, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,pmsien::Ovc, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "UVC Interrupt enable   UVC. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn uvc(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, pmsien::Uvc, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,pmsien::Uvc, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OVPRE Interrupt enable   OVPRE. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn ovpre(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, pmsien::Ovpre, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,pmsien::Ovpre, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "UVPRE Interrupt enable   UVPRE. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn uvpre(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, pmsien::Uvpre, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,pmsien::Uvpre, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OVDDM Interrupt enable   OVDDM. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn ovddm(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, pmsien::Ovddm, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,pmsien::Ovddm, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "UVDDM Interrupt enable   UVDDM. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn uvddm(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, pmsien::Uvddm, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,pmsien::Uvddm, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OVSB Interrupt enable   OVSB. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn ovsb(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, pmsien::Ovsb, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,pmsien::Ovsb, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "UVSB Interrupt enable   UVSB. Interrupt triggered on event as configured in EVRMONCTRL register."]
    #[inline(always)]
    pub fn uvsb(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, pmsien::Uvsb, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,pmsien::Uvsb, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "EVRCMOD Interrupt enable   EVRCMOD. Interrupt triggered on a state change of EVRSTAT.EVRCMOD 0  bitfield."]
    #[inline(always)]
    pub fn evrcmod(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, pmsien::Evrcmod, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,pmsien::Evrcmod, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SDVOK Interrupt enable   SDVOK. Interrupt triggered on EVRSTAT.SDVOK rising edge event."]
    #[inline(always)]
    pub fn sdvok(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, pmsien::Sdvok, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,pmsien::Sdvok, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SD SYNCLCK Interrupt enable   SYNCLCK. Interrupt triggered on a state change of EVRSTAT.SYNCLCK bitfield."]
    #[inline(always)]
    pub fn synclck(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, pmsien::Synclck, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,pmsien::Synclck, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SWDLVL Interrupt enable   SWDLVL. Interrupt triggered on a state change of EVRSTAT.SWDLVL bitfield."]
    #[inline(always)]
    pub fn swdlvl(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, pmsien::Swdlvl, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,pmsien::Swdlvl, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "WUTWKP Interrupt enable   WUTWKP. Interrupt triggered on a WUTCNT underflow event."]
    #[inline(always)]
    pub fn wutwkp(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, pmsien::Wutwkp, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,pmsien::Wutwkp, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ESR0WKP Interrupt enable   ESR0WKP. Interrupt triggered on a ESR0WKP event."]
    #[inline(always)]
    pub fn esr0wkp(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, pmsien::Esr0Wkp, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,pmsien::Esr0Wkp, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ESR1WKP Interrupt enable   ESR1WKP. Interrupt triggered on a ESR1WKP event."]
    #[inline(always)]
    pub fn esr1wkp(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, pmsien::Esr1Wkp, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,pmsien::Esr1Wkp, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PINAWKP Interrupt enable   PINAWKP. Interrupt triggered on a PINAWKP event."]
    #[inline(always)]
    pub fn pinawkp(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, pmsien::Pinawkp, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,pmsien::Pinawkp, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PINBWKP Interrupt enable   PINBWKP. Interrupt triggered on a PINBWKP event."]
    #[inline(always)]
    pub fn pinbwkp(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, pmsien::Pinbwkp, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,pmsien::Pinbwkp, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SCRINT Interrupt enable   SCRINT. Interrupt triggered on a SCRINT event triggered by SCR to PMS to decode        information in PMSWCR2.SCRINT register."]
    #[inline(always)]
    pub fn scrint(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, pmsien::Scrint, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,pmsien::Scrint, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SCRRST Interrupt enable   SCRRST. Interrupt triggered by SCR to PMS on an internal SCR software reset."]
    #[inline(always)]
    pub fn scrrst(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, pmsien::Scrrst, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,pmsien::Scrrst, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SCRECC Interrupt enable   SCRECC. Interrupt triggered by SCR to PMS on an internal RAM double bit ECC        error."]
    #[inline(always)]
    pub fn screcc(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, pmsien::Screcc, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,pmsien::Screcc, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SCRWDT Interrupt enable   SCRWDT. Interrupt triggered by SCR to PMS on an internal SCR watchdog timeout        error."]
    #[inline(always)]
    pub fn scrwdt(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, pmsien::Scrwdt, Pmsien_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,pmsien::Scrwdt, Pmsien_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pmsien {
    #[inline(always)]
    fn default() -> Pmsien {
        <crate::RegValueT<Pmsien_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pmsien {
    pub struct Ovswd_SPEC;
    pub type Ovswd = crate::EnumBitfieldStruct<u8, Ovswd_SPEC>;
    impl Ovswd {
        #[doc = "0 Interrupt        is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is        enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Uvswd_SPEC;
    pub type Uvswd = crate::EnumBitfieldStruct<u8, Uvswd_SPEC>;
    impl Uvswd {
        #[doc = "0 Interrupt        is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is        enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ov33_SPEC;
    pub type Ov33 = crate::EnumBitfieldStruct<u8, Ov33_SPEC>;
    impl Ov33 {
        #[doc = "0 Interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Uv33_SPEC;
    pub type Uv33 = crate::EnumBitfieldStruct<u8, Uv33_SPEC>;
    impl Uv33 {
        #[doc = "0 Interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ovc_SPEC;
    pub type Ovc = crate::EnumBitfieldStruct<u8, Ovc_SPEC>;
    impl Ovc {
        #[doc = "0 Interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Uvc_SPEC;
    pub type Uvc = crate::EnumBitfieldStruct<u8, Uvc_SPEC>;
    impl Uvc {
        #[doc = "0 Interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ovpre_SPEC;
    pub type Ovpre = crate::EnumBitfieldStruct<u8, Ovpre_SPEC>;
    impl Ovpre {
        #[doc = "0 Interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Uvpre_SPEC;
    pub type Uvpre = crate::EnumBitfieldStruct<u8, Uvpre_SPEC>;
    impl Uvpre {
        #[doc = "0 Interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ovddm_SPEC;
    pub type Ovddm = crate::EnumBitfieldStruct<u8, Ovddm_SPEC>;
    impl Ovddm {
        #[doc = "0 Interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Uvddm_SPEC;
    pub type Uvddm = crate::EnumBitfieldStruct<u8, Uvddm_SPEC>;
    impl Uvddm {
        #[doc = "0 Interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ovsb_SPEC;
    pub type Ovsb = crate::EnumBitfieldStruct<u8, Ovsb_SPEC>;
    impl Ovsb {
        #[doc = "0 Interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Uvsb_SPEC;
    pub type Uvsb = crate::EnumBitfieldStruct<u8, Uvsb_SPEC>;
    impl Uvsb {
        #[doc = "0 Interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Evrcmod_SPEC;
    pub type Evrcmod = crate::EnumBitfieldStruct<u8, Evrcmod_SPEC>;
    impl Evrcmod {
        #[doc = "0 Interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sdvok_SPEC;
    pub type Sdvok = crate::EnumBitfieldStruct<u8, Sdvok_SPEC>;
    impl Sdvok {
        #[doc = "0 Interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Synclck_SPEC;
    pub type Synclck = crate::EnumBitfieldStruct<u8, Synclck_SPEC>;
    impl Synclck {
        #[doc = "0 Interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Swdlvl_SPEC;
    pub type Swdlvl = crate::EnumBitfieldStruct<u8, Swdlvl_SPEC>;
    impl Swdlvl {
        #[doc = "0 Interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wutwkp_SPEC;
    pub type Wutwkp = crate::EnumBitfieldStruct<u8, Wutwkp_SPEC>;
    impl Wutwkp {
        #[doc = "0 Interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esr0Wkp_SPEC;
    pub type Esr0Wkp = crate::EnumBitfieldStruct<u8, Esr0Wkp_SPEC>;
    impl Esr0Wkp {
        #[doc = "0 Interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esr1Wkp_SPEC;
    pub type Esr1Wkp = crate::EnumBitfieldStruct<u8, Esr1Wkp_SPEC>;
    impl Esr1Wkp {
        #[doc = "0 Interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pinawkp_SPEC;
    pub type Pinawkp = crate::EnumBitfieldStruct<u8, Pinawkp_SPEC>;
    impl Pinawkp {
        #[doc = "0 Interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pinbwkp_SPEC;
    pub type Pinbwkp = crate::EnumBitfieldStruct<u8, Pinbwkp_SPEC>;
    impl Pinbwkp {
        #[doc = "0 Interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scrint_SPEC;
    pub type Scrint = crate::EnumBitfieldStruct<u8, Scrint_SPEC>;
    impl Scrint {
        #[doc = "0 Interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scrrst_SPEC;
    pub type Scrrst = crate::EnumBitfieldStruct<u8, Scrrst_SPEC>;
    impl Scrrst {
        #[doc = "0 Interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Screcc_SPEC;
    pub type Screcc = crate::EnumBitfieldStruct<u8, Screcc_SPEC>;
    impl Screcc {
        #[doc = "0 Interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scrwdt_SPEC;
    pub type Scrwdt = crate::EnumBitfieldStruct<u8, Scrwdt_SPEC>;
    impl Scrwdt {
        #[doc = "0 Interrupt is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmswcr0_SPEC;
impl crate::sealed::RegSpec for Pmswcr0_SPEC {
    type DataType = u32;
}
#[doc = "Standby and Wake up Control Register 0\n resetvalue={LVD Reset:0x1002D0}"]
pub type Pmswcr0 = crate::RegValueT<Pmswcr0_SPEC>;

impl Pmswcr0 {
    #[doc = "Standby Entry on VEXT Supply ramp down   VEXTSTBYEN. This bit field enables Standby Entry on VEXT supply ramp down. This is        supported only in case Standby domain is supplied separately via VEVRSB        supply pin and VEXT rail is switched off during Standby. The voltage        threshold for entry is configured in EVRUVMON register. Current        configuration is reflected in PMSWSTAT2.VEXTSTBYEN register bit."]
    #[inline(always)]
    pub fn vextstbyen(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pmswcr0::Vextstbyen,
        Pmswcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pmswcr0::Vextstbyen,
            Pmswcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Standby Entry on VDD Supply ramp down   VDDSTBYEN. This bit field enables Standby Entry on VDD supply ramp down. This is        supported only in case Standby domain is supplied separately via VEVRSB        supply pin and VDD rail is switched off during Standby. The voltage        threshold for entry is configured in EVRUVMON register. Current        configuration is reflected in PMSWSTAT2.VDDSTBYEN register bit."]
    #[inline(always)]
    pub fn vddstbyen(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pmswcr0::Vddstbyen,
        Pmswcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pmswcr0::Vddstbyen,
            Pmswcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ESR0 Digital Filter Enable   ESR0DFEN. This bit activates digital spike filter. If the digital filter  majority        filter of 3 consecutive values  is enabled during normal RUN mode  then        pulses less than 30ns are suppressed and pulses longer than 100ns will        always result in a trigger. If the back up clock is disabled in Standby        mode and filter is running on 70  160 KHz Standby clock  then pulses less        than 5 us are suppressed and pulses longer than 50 us will always result        in a trigger."]
    #[inline(always)]
    pub fn esr0dfen(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pmswcr0::Esr0Dfen,
        Pmswcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pmswcr0::Esr0Dfen,
            Pmswcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ESR0 Edge Detection Control   ESR0EDCON. This bit field defines the edge of a ESR0 wake up trigger"]
    #[inline(always)]
    pub fn esr0edcon(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x3,
        1,
        0,
        pmswcr0::Esr0Edcon,
        Pmswcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x3,
            1,
            0,
            pmswcr0::Esr0Edcon,
            Pmswcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ESR1 Digital Filter Enable   ESR1DFEN. This bit activates digital spike filter. If the digital filter  majority        filter of 3 consecutive values  is enabled during normal RUN mode  then        pulses less than 30ns are suppressed and pulses longer than 100ns will        always result in a trigger. If the back up clock is disabled in Standby        mode and filter is running on 70 KHz Standby clock  then pulses less        than 5 us are suppressed and pulses longer than 50 us will always result        in a trigger."]
    #[inline(always)]
    pub fn esr1dfen(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pmswcr0::Esr1Dfen,
        Pmswcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pmswcr0::Esr1Dfen,
            Pmswcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ESR1 Edge Detection Control   ESR1EDCON. This bit field defines the edge of a ESR1 wake up trigger"]
    #[inline(always)]
    pub fn esr1edcon(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        pmswcr0::Esr1Edcon,
        Pmswcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            pmswcr0::Esr1Edcon,
            Pmswcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "PINA Digital Filter Enable   PINADFEN. This bit activates digital spike filter. If the digital filter  majority        filter of 3 consecutive values  is enabled during normal RUN mode  then        pulses less than 40ns are suppressed and pulses longer than 100ns will        always result in a trigger. If the back up clock is disabled in Standby        mode and filter is running on 70 KHz Standby clock  then pulses less        than 5 us are suppressed and pulses longer than 50 us will always result        in a trigger."]
    #[inline(always)]
    pub fn pinadfen(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pmswcr0::Pinadfen,
        Pmswcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pmswcr0::Pinadfen,
            Pmswcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "PINA Edge Detection Control   PINAEDCON. This bit field defines the edge of a Pin A wake up trigger"]
    #[inline(always)]
    pub fn pinaedcon(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x3,
        1,
        0,
        pmswcr0::Pinaedcon,
        Pmswcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x3,
            1,
            0,
            pmswcr0::Pinaedcon,
            Pmswcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "PINB Digital Filter Enable   PINBDFEN. This bit activates digital spike filter. If the digital filter  majority        filter of 3 consecutive values  is enabled during normal RUN mode  then        pulses less than 40ns are suppressed and pulses longer than 100ns will        always result in a trigger. If the back up clock is disabled in Standby        mode and filter is running on 70 KHz Standby clock  then pulses less        than 5 us are suppressed and pulses longer than 50 us will always result        in a trigger."]
    #[inline(always)]
    pub fn pinbdfen(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pmswcr0::Pinbdfen,
        Pmswcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pmswcr0::Pinbdfen,
            Pmswcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "PINB Edge Detection Control   PINBEDCON. This bit field defines the edge of a Pin B wake up trigger"]
    #[inline(always)]
    pub fn pinbedcon(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x3,
        1,
        0,
        pmswcr0::Pinbedcon,
        Pmswcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x3,
            1,
            0,
            pmswcr0::Pinbedcon,
            Pmswcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ESR0 Wake up enable from Standby   ESR0WKEN. This bit configures wake up via ESR0 pin from STANDBY mode and current        configuration is reflected in PMSWSTAT2.ESR0WKEN register bit."]
    #[inline(always)]
    pub fn esr0wken(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        pmswcr0::Esr0Wken,
        Pmswcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            pmswcr0::Esr0Wken,
            Pmswcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ESR1 Wake up enable from Standby   ESR1WKEN. This bit configures wake up via ESR1 pin from STANDBY mode and current        configuration is reflected in PMSWSTAT2.ESR1WKEN register bit."]
    #[inline(always)]
    pub fn esr1wken(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        pmswcr0::Esr1Wken,
        Pmswcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            pmswcr0::Esr1Wken,
            Pmswcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Pin A Wake up enable from Standby   PINAWKEN. This bit configures wake up via PINA pin from STANDBY mode and current        configuration is reflected in PMSWSTAT2.PINAWKEN register bit."]
    #[inline(always)]
    pub fn pinawken(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        pmswcr0::Pinawken,
        Pmswcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            pmswcr0::Pinawken,
            Pmswcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Pin B Wake up enable from Standby   PINBWKEN. This bit configures wake up via PINB pin from STANDBY mode and current        configuration is reflected in PMSWSTAT2.PINBWKEN register bit."]
    #[inline(always)]
    pub fn pinbwken(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        pmswcr0::Pinbwken,
        Pmswcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            pmswcr0::Pinbwken,
            Pmswcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Standby Wake up Enable on VEXT Supply ramp up   PWRWKEN. This bit field enables wake up on VEXT supply ramp up after blanking        filter time has expired. This is supported only in case Standby domain        is supplied separately via VEVRSB supply pin and VEXT rail is switched        off during Standby. Current configuration is reflected in        PMSWSTAT2.PWRWKEN register bit."]
    #[inline(always)]
    pub fn pwrwken(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        pmswcr0::Pwrwken,
        Pmswcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            pmswcr0::Pwrwken,
            Pmswcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Standby Controller Wake up enable from Standby   SCRWKEN. This bit configures wake up via SCR from STANDBY mode and current        configuration is reflected in PMSWSTAT2.SCRWKEN register bit."]
    #[inline(always)]
    pub fn scrwken(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        pmswcr0::Scrwken,
        Pmswcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            pmswcr0::Scrwken,
            Pmswcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "PORST pin Wake up enable from Standby   PORSTWKEN. This bit configures wake up via PORST pin from STANDBY mode and current        configuration is reflected in PMSWSTAT2.PORSTWKEN register bit."]
    #[inline(always)]
    pub fn porstwken(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        pmswcr0::Porstwken,
        Pmswcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            pmswcr0::Porstwken,
            Pmswcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "WUT Wake up enable from Standby   WUTWKEN. This bit configures wake up via WUT from STANDBY mode and current        configuration is reflected in PMSWSTAT2.WUTWKEN register bit."]
    #[inline(always)]
    pub fn wutwken(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        pmswcr0::Wutwken,
        Pmswcr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            pmswcr0::Wutwken,
            Pmswcr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Pmswcr0 {
    #[inline(always)]
    fn default() -> Pmswcr0 {
        <crate::RegValueT<Pmswcr0_SPEC> as RegisterValue<_>>::new(1049296)
    }
}
pub mod pmswcr0 {
    pub struct Vextstbyen_SPEC;
    pub type Vextstbyen = crate::EnumBitfieldStruct<u8, Vextstbyen_SPEC>;
    impl Vextstbyen {
        #[doc = "0 Standby        Entry on VEXT supply ramp down is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Standby Entry        triggered on a VEXT Supply undervoltage event  SWDUV . Blanking filter        active on Standby mode entry."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Vddstbyen_SPEC;
    pub type Vddstbyen = crate::EnumBitfieldStruct<u8, Vddstbyen_SPEC>;
    impl Vddstbyen {
        #[doc = "0 Standby        Entry on VDD supply ramp down is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Standby Entry        triggered on a VDD Supply undervoltage event  VDDUV . Blanking filter        active on Standby mode entry."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esr0Dfen_SPEC;
    pub type Esr0Dfen = crate::EnumBitfieldStruct<u8, Esr0Dfen_SPEC>;
    impl Esr0Dfen {
        #[doc = "0 The filter is bypassed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The filter is        used"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esr0Edcon_SPEC;
    pub type Esr0Edcon = crate::EnumBitfieldStruct<u8, Esr0Edcon_SPEC>;
    impl Esr0Edcon {
        #[doc = "00 No trigger is        generated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 A trigger is        generated upon a rising edge"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 A trigger is generated upon a falling edge"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 A trigger is generated upon a rising OR falling edge"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Esr1Dfen_SPEC;
    pub type Esr1Dfen = crate::EnumBitfieldStruct<u8, Esr1Dfen_SPEC>;
    impl Esr1Dfen {
        #[doc = "0 The filter is bypassed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The filter is used"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esr1Edcon_SPEC;
    pub type Esr1Edcon = crate::EnumBitfieldStruct<u8, Esr1Edcon_SPEC>;
    impl Esr1Edcon {
        #[doc = "00 No trigger is generated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 A trigger is generated upon a rising edge"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 A trigger is generated upon a falling edge"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 A trigger is generated upon a rising OR falling edge"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Pinadfen_SPEC;
    pub type Pinadfen = crate::EnumBitfieldStruct<u8, Pinadfen_SPEC>;
    impl Pinadfen {
        #[doc = "0 The filter is bypassed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The filter is used"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pinaedcon_SPEC;
    pub type Pinaedcon = crate::EnumBitfieldStruct<u8, Pinaedcon_SPEC>;
    impl Pinaedcon {
        #[doc = "00 No trigger is generated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 A trigger is generated upon a rising edge"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 A trigger is generated upon a falling edge"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 A trigger is generated upon a rising OR falling edge"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Pinbdfen_SPEC;
    pub type Pinbdfen = crate::EnumBitfieldStruct<u8, Pinbdfen_SPEC>;
    impl Pinbdfen {
        #[doc = "0 The filter is bypassed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The filter is used"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pinbedcon_SPEC;
    pub type Pinbedcon = crate::EnumBitfieldStruct<u8, Pinbedcon_SPEC>;
    impl Pinbedcon {
        #[doc = "00 No trigger is generated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 A trigger is generated upon a rising edge"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 A trigger is generated upon a falling edge"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 A trigger is generated upon a rising OR falling edge"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Esr0Wken_SPEC;
    pub type Esr0Wken = crate::EnumBitfieldStruct<u8, Esr0Wken_SPEC>;
    impl Esr0Wken {
        #[doc = "0 System wake up via ESR0 pin is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 System wake up is enabled via ESR0 pin."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esr1Wken_SPEC;
    pub type Esr1Wken = crate::EnumBitfieldStruct<u8, Esr1Wken_SPEC>;
    impl Esr1Wken {
        #[doc = "0 System wake up via ESR1 pin is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 System wake up is enabled via ESR1 pin."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pinawken_SPEC;
    pub type Pinawken = crate::EnumBitfieldStruct<u8, Pinawken_SPEC>;
    impl Pinawken {
        #[doc = "0 System wake up via Pin A is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 System wake up is enabled via Pin A."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pinbwken_SPEC;
    pub type Pinbwken = crate::EnumBitfieldStruct<u8, Pinbwken_SPEC>;
    impl Pinbwken {
        #[doc = "0 System wake up via Pin B is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 System wake up is enabled via Pin B."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pwrwken_SPEC;
    pub type Pwrwken = crate::EnumBitfieldStruct<u8, Pwrwken_SPEC>;
    impl Pwrwken {
        #[doc = "0 Wake up        on VEXT supply ramp down is disabled. Blanking filter configuration has        no effect."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Wake up from standby on VEXT supply ramp up is enabled        after blanking filter time expiry."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scrwken_SPEC;
    pub type Scrwken = crate::EnumBitfieldStruct<u8, Scrwken_SPEC>;
    impl Scrwken {
        #[doc = "0 System wake up via 8 bit Standby Controller is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 System wake up is enabled via 8 bit Standby Controller."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Porstwken_SPEC;
    pub type Porstwken = crate::EnumBitfieldStruct<u8, Porstwken_SPEC>;
    impl Porstwken {
        #[doc = "0 System wake up via PORST pin is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 System wake up via PORST pin is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wutwken_SPEC;
    pub type Wutwken = crate::EnumBitfieldStruct<u8, Wutwken_SPEC>;
    impl Wutwken {
        #[doc = "0 System        wake up via Wake up Timer is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 System wake up is enabled via Wake up Timer."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmswcr2_SPEC;
impl crate::sealed::RegSpec for Pmswcr2_SPEC {
    type DataType = u32;
}
#[doc = "Standby and Wake up Control Register 2\n resetvalue={LVD Reset:0x4000000}"]
pub type Pmswcr2 = crate::RegValueT<Pmswcr2_SPEC>;

impl Pmswcr2 {
    #[doc = "Data exchange from Standby Controller to PMS main domain.   SCRINT. This bit field allows fast data exchange from SCR to PMS CPUx. The data        maybe read by CPUx consequent to an interrupt from the SCR to decode the        interrupt. Incase SCR is enabled  at the end of the SCR Firmware        routine  a value of 80H is set in SCRINT register to indicate that SCR        has finished executing the startup code."]
    #[inline(always)]
    pub fn scrint(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Pmswcr2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Pmswcr2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SCR RAM ECC error   reset flag   SCRECC. The flag is set by SCR and cleared by explicit write to the register          bit. The flag is not cleared by SCR.While the SCR is being reset          triggered by SCR RAM ECC error  this flag is set and clearing the flag          is not possible for that duration."]
    #[inline(always)]
    pub fn screcc(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, pmswcr2::Screcc, Pmswcr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,pmswcr2::Screcc, Pmswcr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SCR Watchdog Timer error   reset flag   SCRWDT. The flag is set by SCR and cleared by explicit write to the register          bit. The flag is not cleared by SCR.While the SCR is being reset          triggered by SCR watchdog  this flag is set and clearing the flag is          not possible for that duration."]
    #[inline(always)]
    pub fn scrwdt(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, pmswcr2::Scrwdt, Pmswcr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pmswcr2::Scrwdt,
            Pmswcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "SCR Software reset flag   SCRRST. The flag is set by SCR and cleared by explicit write to the register          bit. The flag is not cleared by SCR. While the SCR is being reset          triggered by SCR software  this flag is set and clearing  the flag is not possible for that duration."]
    #[inline(always)]
    pub fn scrrst(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, pmswcr2::Scrrst, Pmswcr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            pmswcr2::Scrrst,
            Pmswcr2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Data exchange from PMS main domain to Standby Controller.   TCINT. This bit field allows fast data exchange from PMS to SCR. The data may        be read by SCR consequent to an interrupt request  TCINTREQ  from        PMS CPUx to SCR to decode the interrupt."]
    #[inline(always)]
    pub fn tcint(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Pmswcr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Pmswcr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SW Interrupt request from PMS to Standby Controller.   TCINTREQ. Setting this bit triggers an interrupt to the 8 bit Standby controller. The TCINTREQ bit is cleared after SCR has latched the TCINT value and          SCR NMI is issued."]
    #[inline(always)]
    pub fn tcintreq(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Pmswcr2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Pmswcr2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SMU Reset indication flag   SMURST. SCR is explicitly informed in case reset issued by SMU. The flag is          cleared after SCR has latched the signal and SCR NMI is issued."]
    #[inline(always)]
    pub fn smurst(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, pmswcr2::Smurst, Pmswcr2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<25,0x1,1,0,pmswcr2::Smurst, Pmswcr2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Application or System Reset indication flag   RST. SCR is explicitly informed in case of an application or system reset.          The flag is cleared after SCR has latched the signal and SCR NMI is          issued."]
    #[inline(always)]
    pub fn rst(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, pmswcr2::Rst, Pmswcr2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<26,0x1,1,0,pmswcr2::Rst, Pmswcr2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pmswcr2 {
    #[inline(always)]
    fn default() -> Pmswcr2 {
        <crate::RegValueT<Pmswcr2_SPEC> as RegisterValue<_>>::new(67108864)
    }
}
pub mod pmswcr2 {
    pub struct Screcc_SPEC;
    pub type Screcc = crate::EnumBitfieldStruct<u8, Screcc_SPEC>;
    impl Screcc {
        #[doc = "0 No ECC error   reset reported by SCR."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ECC error   reset was detected in SCR RAM."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scrwdt_SPEC;
    pub type Scrwdt = crate::EnumBitfieldStruct<u8, Scrwdt_SPEC>;
    impl Scrwdt {
        #[doc = "0 No WDT error   reset reported by SCR."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 WDT timer error   reset reported by SCR."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scrrst_SPEC;
    pub type Scrrst = crate::EnumBitfieldStruct<u8, Scrrst_SPEC>;
    impl Scrrst {
        #[doc = "0 No reset occurred in SCR."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A reset has occurred in SCR."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Smurst_SPEC;
    pub type Smurst = crate::EnumBitfieldStruct<u8, Smurst_SPEC>;
    impl Smurst {
        #[doc = "0 No reset was issued by SMU."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SMU issued an application or system reset."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rst_SPEC;
    pub type Rst = crate::EnumBitfieldStruct<u8, Rst_SPEC>;
    impl Rst {
        #[doc = "0 No application or system reset occurred."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An application or system reset has occurred."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmswcr3_SPEC;
impl crate::sealed::RegSpec for Pmswcr3_SPEC {
    type DataType = u32;
}
#[doc = "Standby and Wake up Control Register 3\n resetvalue={LVD Reset:0x0}"]
pub type Pmswcr3 = crate::RegValueT<Pmswcr3_SPEC>;

impl Pmswcr3 {
    #[doc = "WUT reload value.   WUTREL. The counter starts counting down from WUTREL value. The current value of        counter is indicated in WUTCNT. On WUTCNT underflow  a reload        WUTCNT  160    160 WUTREL takes place in auto reload mode."]
    #[inline(always)]
    pub fn wutrel(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, Pmswcr3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffff,1,0,u32, Pmswcr3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "WUT enable   WUTEN. This bit enables the Wake up Timer. The status bit PMSWSTAT.WUTEN is set        once Wake up Timer is enabled."]
    #[inline(always)]
    pub fn wuten(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, pmswcr3::Wuten, Pmswcr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,pmswcr3::Wuten, Pmswcr3_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Status   LCK. This bit indicates that the register is busy owing to ongoing bus        access. The register can be updated with a new value when BUSY bit is        cleared. The register requires synchronization to the 70kHz clock domain        on a register update."]
    #[inline(always)]
    pub fn busy(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, pmswcr3::Busy, Pmswcr3_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<28,0x1,1,0,pmswcr3::Busy, Pmswcr3_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "WUT clock divider   WUTDIV. A write to this register bitfield may trigger immediate update        irrespective of the status of BUSY bit."]
    #[inline(always)]
    pub fn wutdiv(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, pmswcr3::Wutdiv, Pmswcr3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            pmswcr3::Wutdiv,
            Pmswcr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "WUT mode selection   WUTMODE. This bit configures the Wake up Timer mode. The status bit        PMSWSTAT.WUTMODE is respectively updated. A write to this register        bitfield may trigger immediate update irrespective of the status of BUSY        bit."]
    #[inline(always)]
    pub fn wutmode(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        pmswcr3::Wutmode,
        Pmswcr3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            pmswcr3::Wutmode,
            Pmswcr3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Pmswcr3 {
    #[inline(always)]
    fn default() -> Pmswcr3 {
        <crate::RegValueT<Pmswcr3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pmswcr3 {
    pub struct Wuten_SPEC;
    pub type Wuten = crate::EnumBitfieldStruct<u8, Wuten_SPEC>;
    impl Wuten {
        #[doc = "0 Wake up timer  WUT  disable request"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Wake up timer  WUT  enable request."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Busy_SPEC;
    pub type Busy = crate::EnumBitfieldStruct<u8, Busy_SPEC>;
    impl Busy {
        #[doc = "0 The        register can be updated."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The register        update is ongoing. A write action may stall bus access for the time        duration BUSY bit is set."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wutdiv_SPEC;
    pub type Wutdiv = crate::EnumBitfieldStruct<u8, Wutdiv_SPEC>;
    impl Wutdiv {
        #[doc = "0 Wake up timer  WUT  clock   fSB   70 KHz clock."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Wake up timer         WUT  clock   fSB  70  160 KHz    210."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wutmode_SPEC;
    pub type Wutmode = crate::EnumBitfieldStruct<u8, Wutmode_SPEC>;
    impl Wutmode {
        #[doc = "0 Wake up timer  WUT  auto reload mode selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Wake up timer         WUT  auto stop mode selected."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmswcr4_SPEC;
impl crate::sealed::RegSpec for Pmswcr4_SPEC {
    type DataType = u32;
}
#[doc = "Standby and Wake up Control Register 4\n resetvalue={LVD Reset:0x20}"]
pub type Pmswcr4 = crate::RegValueT<Pmswcr4_SPEC>;

impl Pmswcr4 {
    #[doc = "Standby Controller Reset request enable   SCRSTEN"]
    #[inline(always)]
    pub fn bpscrstreq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pmswcr4::Bpscrstreq,
        Pmswcr4_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pmswcr4::Bpscrstreq,
            Pmswcr4_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Standby Controller Reset request   SCRSTREQ"]
    #[inline(always)]
    pub fn scrstreq(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pmswcr4::Scrstreq,
        Pmswcr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pmswcr4::Scrstreq,
            Pmswcr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Bit Protection for PORSTREQ   PORSTEN"]
    #[inline(always)]
    pub fn bpporstreq(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pmswcr4::Bpporstreq,
        Pmswcr4_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pmswcr4::Bpporstreq,
            Pmswcr4_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "SCR Reset behavior on warm PORST in Normal RUN   SLEEP mode   PORSTREQ"]
    #[inline(always)]
    pub fn porstreq(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        pmswcr4::Porstreq,
        Pmswcr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            pmswcr4::Porstreq,
            Pmswcr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Default Clock selection on Standby Mode Entry   SCRCLKSEL"]
    #[inline(always)]
    pub fn scrclksel(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pmswcr4::Scrclksel,
        Pmswcr4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pmswcr4::Scrclksel,
            Pmswcr4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Standby Controller Reset request enable   BPSCREN"]
    #[inline(always)]
    pub fn bpscren(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, pmswcr4::Bpscren, Pmswcr4_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            pmswcr4::Bpscren,
            Pmswcr4_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Standby Controller Enable request   SCREN. SCR MBIST maybe activated independent of this bit."]
    #[inline(always)]
    pub fn scren(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, pmswcr4::Scren, Pmswcr4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,pmswcr4::Scren, Pmswcr4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pmswcr4 {
    #[inline(always)]
    fn default() -> Pmswcr4 {
        <crate::RegValueT<Pmswcr4_SPEC> as RegisterValue<_>>::new(32)
    }
}
pub mod pmswcr4 {
    pub struct Bpscrstreq_SPEC;
    pub type Bpscrstreq = crate::EnumBitfieldStruct<u8, Bpscrstreq_SPEC>;
    impl Bpscrstreq {
        #[doc = "0 Bit SCRSTREQ is not updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit SCRSTREQ can be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scrstreq_SPEC;
    pub type Scrstreq = crate::EnumBitfieldStruct<u8, Scrstreq_SPEC>;
    impl Scrstreq {
        #[doc = "0 No        request for main reset of the 8 bit Standby Controller.         evr scr rst req i"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 8 bit Standby        Controller reset request."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Bpporstreq_SPEC;
    pub type Bpporstreq = crate::EnumBitfieldStruct<u8, Bpporstreq_SPEC>;
    impl Bpporstreq {
        #[doc = "0 Bit PORSTREQ is not updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit PORSTREQ can be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Porstreq_SPEC;
    pub type Porstreq = crate::EnumBitfieldStruct<u8, Porstreq_SPEC>;
    impl Porstreq {
        #[doc = "0 8 bit Standby Controller is not reset when warm PORST pin is asserted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 8 bit Standby Controller is reset when warm PORST pin is asserted. warm PORST usage in normal and standby mode."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scrclksel_SPEC;
    pub type Scrclksel = crate::EnumBitfieldStruct<u8, Scrclksel_SPEC>;
    impl Scrclksel {
        #[doc = "0 100MHz        oscillator can be enabled or disabled based on request from Standby        Controller. By default 100  160 MHz Oscillator is requested by SCR in Standby        Mode. Later  the Standby        Controller may request deactivation of the 100 MHz clock after switching        operation to 70  160 KHz Oscillator clock."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 100MHz        oscillator is always active irrespective of SCR requests. Thus both        70  160 KHz Oscillator and 100  160 MHz oscillator are active in Standby Mode. Standby        Controller receives both the clocks and clock selection happens in the        Standby Controller."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Bpscren_SPEC;
    pub type Bpscren = crate::EnumBitfieldStruct<u8, Bpscren_SPEC>;
    impl Bpscren {
        #[doc = "0 Bit SCREN is not updated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit SCREN can be updated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scren_SPEC;
    pub type Scren = crate::EnumBitfieldStruct<u8, Scren_SPEC>;
    impl Scren {
        #[doc = "0 8 bit Standby Controller is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 8 bit Standby Controller is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmswcr5_SPEC;
impl crate::sealed::RegSpec for Pmswcr5_SPEC {
    type DataType = u32;
}
#[doc = "Standby and Wake up Control Register 5\n resetvalue={LVD Reset:0x0}"]
pub type Pmswcr5 = crate::RegValueT<Pmswcr5_SPEC>;

impl Pmswcr5 {
    #[doc = "Bit protection for Tristate request bit  TRISTREQ    BPTRISTREQ. Setting this bit enables that bit TRISTREQ can be changed by a write        operation."]
    #[inline(always)]
    pub fn bptristreq(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pmswcr5::Bptristreq,
        Pmswcr5_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pmswcr5::Bptristreq,
            Pmswcr5_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Tristate enable   TRISTREQ. This bit decides whether pads behave as inputs with weak pull up or        tristate on reset assertion de assertion or Standby  Wake up transition.        After supply ramp up or LVD reset  TRISTREQ  160    160    160 HWCFG6."]
    #[inline(always)]
    pub fn tristreq(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pmswcr5::Tristreq,
        Pmswcr5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pmswcr5::Tristreq,
            Pmswcr5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ESR0 Tristate enable   ESR0TRIST. This bit configures ESR0 pin behavior either as reset output or tristate        during Standby mode if VEXT is supplied."]
    #[inline(always)]
    pub fn esr0trist(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pmswcr5::Esr0Trist,
        Pmswcr5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pmswcr5::Esr0Trist,
            Pmswcr5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "PORST Digital Filter enable   PORSTDF. This bit field enables additional PORST digital filter    160 tPORSTDF        parameter  160   to provide enhanced immunity against spurious spikes. The filter is implemented over 20 cycles x 40 ns clock which results in        a total delay between 800ns    2   trimmed fback clock  and is active        only in operational mode. In Standby mode  the filter is implemented        over 2 x 70kHz clock such that pulses less than 8 us are suppressed and        pulses longer than 70 us will result in a wake up trigger."]
    #[inline(always)]
    pub fn porstdf(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, pmswcr5::Porstdf, Pmswcr5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pmswcr5::Porstdf,
            Pmswcr5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "DC DC Synchronisation Output. This bitfield enables the synchronisation output to synchronize the external SMPS regulator with respect to the internal EVRC regulator. This enable bit controls the ENPS signal to the port pin  so that the synchronisation signal is also available during active warm PORST phase. If enabled  this signal need to be also routed to HW DIR  amp  HWEN of the port pin."]
    #[inline(always)]
    pub fn dcdcsynco(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pmswcr5::Dcdcsynco,
        Pmswcr5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pmswcr5::Dcdcsynco,
            Pmswcr5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Pmswcr5 {
    #[inline(always)]
    fn default() -> Pmswcr5 {
        <crate::RegValueT<Pmswcr5_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pmswcr5 {
    pub struct Bptristreq_SPEC;
    pub type Bptristreq = crate::EnumBitfieldStruct<u8, Bptristreq_SPEC>;
    impl Bptristreq {
        #[doc = "0 TRISTREQ keeps the previous state and cannot be changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 TRISTREQ bit can be changed with a write operation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tristreq_SPEC;
    pub type Tristreq = crate::EnumBitfieldStruct<u8, Tristreq_SPEC>;
    impl Tristreq {
        #[doc = "0 No request to switch the input pad state of all the pads to tristate from pull up  default reset state"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Pad domain in        tristate. VGATE1P pull up remains active if VEXT available and EVRC SMPS        selected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esr0Trist_SPEC;
    pub type Esr0Trist = crate::EnumBitfieldStruct<u8, Esr0Trist_SPEC>;
    impl Esr0Trist {
        #[doc = "0 ESR0 configured as reset output and is held low during Standby state  default reset state"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ESR0 in tristate during Standby state."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Porstdf_SPEC;
    pub type Porstdf = crate::EnumBitfieldStruct<u8, Porstdf_SPEC>;
    impl Porstdf {
        #[doc = "0 PORST recognition delay   Analog PORST pad filter delay  default reset state ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PORST recognition delay   Analog PORST pad filter delay   Digital filter delay."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Dcdcsynco_SPEC;
    pub type Dcdcsynco = crate::EnumBitfieldStruct<u8, Dcdcsynco_SPEC>;
    impl Dcdcsynco {
        #[doc = "0 DC DC Synchronisation signal not available."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 DC DC Synchronisation signal available."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmswstat_SPEC;
impl crate::sealed::RegSpec for Pmswstat_SPEC {
    type DataType = u32;
}
#[doc = "Standby and Wake up Status Register\n resetvalue={LVD Reset:0x0A0000}"]
pub type Pmswstat = crate::RegValueT<Pmswstat_SPEC>;

impl Pmswstat {
    #[doc = "EVR Hardware Configuration status   HWCFGEVR. This bit field indicates the supply configuration latched by the EVR        from HWCFG 2 1  during a cold startup based on which EVRx regulators are        consequently started. The latched configuration is used during        STANDBY RUN transition to reselect EVR mode. The HWCFG pins are latched continuously during a period of 60us as        measure to improve robustness owing to noise injection."]
    #[inline(always)]
    pub fn hwcfgevr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x3,
        1,
        0,
        pmswstat::Hwcfgevr,
        Pmswstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x3,
            1,
            0,
            pmswstat::Hwcfgevr,
            Pmswstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Hardware Configuration Pin 4 status   HWCFG4. This bit field indicates the latched level of HWCFG 4  during a cold        startup. The HWCFG pin is latched continuously during a period of 60us as measure        to improve robustness owing to noise injection."]
    #[inline(always)]
    pub fn hwcfg4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Pmswstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Pmswstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Hardware Configuration Pin 5 status   HWCFG5. This bit field indicates the latched level of HWCFG 5  during a cold        startup. The HWCFG pin is latched continuously during a period of 60us as measure        to improve robustness owing to noise injection."]
    #[inline(always)]
    pub fn hwcfg5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Pmswstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Pmswstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Pad Tristate   Pull up status   TRIST. This bit indicates whether pads are configured as inputs with weak        pull up or as tristate during after reset or after wake up. At start up         the value latched from HWCFG 6  pin decides the default state and is        reflected in TRIST status bit. This bit may be later updated when        PMSWCR5.TRISTREQ is set to override initial latched status from HWCFG 6 ."]
    #[inline(always)]
    pub fn trist(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, pmswstat::Trist, Pmswstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x1,1,0,pmswstat::Trist, Pmswstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "TESTMODE Pin status   TESTMODE. This bit field indicates the latched level of TESTMODE pin during a cold        startup. The HWCFG pin is latched continuously during a period of 60us as measure        to improve robustness owing to noise injection."]
    #[inline(always)]
    pub fn testmode(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Pmswstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Pmswstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ESR0 pin status during Standby   ESR0TRIST. This bit indicates if ESR0 pin is configured as reset output or tristate        during Standby mode  amp  transitions if VEXT is supplied.This bit is        updated when PMSWCR5. ESR0TRIST is set."]
    #[inline(always)]
    pub fn esr0trist(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pmswstat::Esr0Trist,
        Pmswstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pmswstat::Esr0Trist,
            Pmswstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "PORST Digital Filter status   PORSTDF. This bit field indicates whether additional PORST digital filter is        activated. This bit is updated when PMSWCR5.PORSTDF is set."]
    #[inline(always)]
    pub fn porstdf(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        pmswstat::Porstdf,
        Pmswstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            pmswstat::Porstdf,
            Pmswstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Standby Controller status   SCR. This bit indicates whether SCR is enabled. This bit is updated when        PMSWCR4.SCREN bit is set."]
    #[inline(always)]
    pub fn scr(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, pmswstat::Scr, Pmswstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1,1,0,pmswstat::Scr, Pmswstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Standby Controller Reset Indication flag   SCRST. This bit is set after a power on reset as SCR is in reset state. This bit is consequently set when a reset is issued via PMSWCR4.SCRSTREQ bit. This status flag is set on every SCR reset caused by any SCR reset source."]
    #[inline(always)]
    pub fn scrst(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, pmswstat::Scrst, Pmswstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            pmswstat::Scrst,
            Pmswstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Current Clock configuration for SCR before Standby Mode Entry   SCRCLK. This bit is updated when PMSWCR4.SCRCLKSEL bit is set."]
    #[inline(always)]
    pub fn scrclk(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        pmswstat::Scrclk,
        Pmswstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            pmswstat::Scrclk,
            Pmswstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Standby Controller Reset on warm PORST   PORSTREQ. This bit is updated when PMSWCR4.PORSTREQ bit is set."]
    #[inline(always)]
    pub fn porstreq(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        pmswstat::Porstreq,
        Pmswstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            pmswstat::Porstreq,
            Pmswstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "WUT Enable status   WUTEN. This bit indicates whether WUT is enabled. This bit is updated when        PMSWCR3.WUTEN bit is updated."]
    #[inline(always)]
    pub fn wuten(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, pmswstat::Wuten, Pmswstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            pmswstat::Wuten,
            Pmswstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "WUT Run status   WUTRUN. This bit indicates whether WUT is currently running. Due to        synchronization to 70 KHz    160 fSB  160   WUT clock  setting of flag after        enable may take up to 55  160 us."]
    #[inline(always)]
    pub fn wutrun(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        pmswstat::Wutrun,
        Pmswstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            pmswstat::Wutrun,
            Pmswstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "WUT Mode status   WUTMODE. This bit indicates the current WUT mode. This bit is updated when        PMSWCR3.WUTMODE bit is updated."]
    #[inline(always)]
    pub fn wutmode(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        pmswstat::Wutmode,
        Pmswstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            pmswstat::Wutmode,
            Pmswstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "ESR0 Interrupt flag   ESR0INT. In case interrupt was triggered by ESR0 pin event during RUN mode  this        flag is set. The bit shall be cleared explicitly via        PMSWSTATCLR.ESR0INTCLR bit after interrupt is serviced."]
    #[inline(always)]
    pub fn esr0int(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        pmswstat::Esr0Int,
        Pmswstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            pmswstat::Esr0Int,
            Pmswstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "ESR1 Interrupt flag   ESR1INT. In case interrupt was triggered by ESR1 pin event during RUN mode  this        flag is set. The bit shall be cleared explicitly via        PMSWSTATCLR.ESR1INTCLR bit after interrupt is serviced."]
    #[inline(always)]
    pub fn esr1int(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        pmswstat::Esr1Int,
        Pmswstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            pmswstat::Esr1Int,
            Pmswstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Pin A Interrupt flag   PINAINT. In case interrupt was triggered by PINA pin event during RUN mode  this        flag is set. The bit shall be cleared explicitly via        PMSWSTATCLR.PINAINTCLR bit after interrupt is serviced."]
    #[inline(always)]
    pub fn pinaint(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        pmswstat::Pinaint,
        Pmswstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            pmswstat::Pinaint,
            Pmswstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Pin B Interrupt flag   PINBINT. In case interrupt was triggered by PINB pin event during RUN mode  this        flag is set. The bit shall be cleared explicitly via        PMSWSTATCLR.PINBINTCLR bit after interrupt is serviced."]
    #[inline(always)]
    pub fn pinbint(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        pmswstat::Pinbint,
        Pmswstat_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            pmswstat::Pinbint,
            Pmswstat_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Pmswstat {
    #[inline(always)]
    fn default() -> Pmswstat {
        <crate::RegValueT<Pmswstat_SPEC> as RegisterValue<_>>::new(655360)
    }
}
pub mod pmswstat {
    pub struct Hwcfgevr_SPEC;
    pub type Hwcfgevr = crate::EnumBitfieldStruct<u8, Hwcfgevr_SPEC>;
    impl Hwcfgevr {
        #[doc = "00 EVRC        inactive  EVR33 inactive."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 EVRC inactive         EVR33 active."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 EVRC active         EVR33 inactive."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "010 EVRC active         EVR33 active."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Trist_SPEC;
    pub type Trist = crate::EnumBitfieldStruct<u8, Trist_SPEC>;
    impl Trist {
        #[doc = "0 Pads        configured as inputs with weak pull up."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Pads are in        tristate."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esr0Trist_SPEC;
    pub type Esr0Trist = crate::EnumBitfieldStruct<u8, Esr0Trist_SPEC>;
    impl Esr0Trist {
        #[doc = "0 ESR0 configured as reset output and is held low during Standby state  default reset state"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 ESR0 in tristate during Standby state."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Porstdf_SPEC;
    pub type Porstdf = crate::EnumBitfieldStruct<u8, Porstdf_SPEC>;
    impl Porstdf {
        #[doc = "0 PORST        recognition delay   Analog PORST pad filter delay  default reset state ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PORST recognition delay   Analog PORST pad filter delay   Digital filter delay."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scr_SPEC;
    pub type Scr = crate::EnumBitfieldStruct<u8, Scr_SPEC>;
    impl Scr {
        #[doc = "0 8 bit Standby Controller is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 8 bit Standby Controller is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scrst_SPEC;
    pub type Scrst = crate::EnumBitfieldStruct<u8, Scrst_SPEC>;
    impl Scrst {
        #[doc = "0 No reset of        Standby controller took place."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Reset of        Standby controller took place.  evr scr rst o"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scrclk_SPEC;
    pub type Scrclk = crate::EnumBitfieldStruct<u8, Scrclk_SPEC>;
    impl Scrclk {
        #[doc = "0 Only        70  160 KHz Oscillator is active in Standby Mode. Standby        Controller receives the 70KHz clock. Later  the Standby Controller may        request EVR to activate the 100 MHz clock."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Both 70  160 KHz        Oscillator and 100  160 MHz oscillator are active in Standby Mode. Standby        Controller receives both the clocks and clock selection happens in the        Standby Controller."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Porstreq_SPEC;
    pub type Porstreq = crate::EnumBitfieldStruct<u8, Porstreq_SPEC>;
    impl Porstreq {
        #[doc = "0 8 bit Standby Controller clock is not reset when warm PORST pin is asserted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 8 bit Standby Controller is reset when warm PORST pin is asserted."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wuten_SPEC;
    pub type Wuten = crate::EnumBitfieldStruct<u8, Wuten_SPEC>;
    impl Wuten {
        #[doc = "0 Wake up timer  WUT  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Wake up timer         WUT  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wutrun_SPEC;
    pub type Wutrun = crate::EnumBitfieldStruct<u8, Wutrun_SPEC>;
    impl Wutrun {
        #[doc = "0 Wake up        timer  WUT  is inactive."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Wake up timer         WUT  is active."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wutmode_SPEC;
    pub type Wutmode = crate::EnumBitfieldStruct<u8, Wutmode_SPEC>;
    impl Wutmode {
        #[doc = "0 Wake up timer  WUT  auto reload mode is selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Wake up timer         WUT  auto stop mode is selected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esr0Int_SPEC;
    pub type Esr0Int = crate::EnumBitfieldStruct<u8, Esr0Int_SPEC>;
    impl Esr0Int {
        #[doc = "0 No interrupt        event detected on ESR0 input."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An interrupt        event as defined by PMSWCR0. ESR0EDCON detected on ESR0 input."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esr1Int_SPEC;
    pub type Esr1Int = crate::EnumBitfieldStruct<u8, Esr1Int_SPEC>;
    impl Esr1Int {
        #[doc = "0 No interrupt        event detected on ESR1 input."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An interrupt        event as defined by PMSWCR0. ESR1EDCON detected on ESR1 input."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pinaint_SPEC;
    pub type Pinaint = crate::EnumBitfieldStruct<u8, Pinaint_SPEC>;
    impl Pinaint {
        #[doc = "0 No interrupt        event detected on Pin A input."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An interrupt        event as defined by PMSWCR0. PINAEDCON detected on Pin A input."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pinbint_SPEC;
    pub type Pinbint = crate::EnumBitfieldStruct<u8, Pinbint_SPEC>;
    impl Pinbint {
        #[doc = "0 No interrupt        event detected on the Pin B input."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An interrupt        event as defined by PMSWCR0. PINBEDCON detected on Pin B input."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmswstat2_SPEC;
impl crate::sealed::RegSpec for Pmswstat2_SPEC {
    type DataType = u32;
}
#[doc = "Standby and Wake up Status Register 2\n resetvalue={LVD Reset:0x100000}"]
pub type Pmswstat2 = crate::RegValueT<Pmswstat2_SPEC>;

impl Pmswstat2 {
    #[doc = "ESR0 Wake up flag   ESR0WKP. In case wake up was triggered by ESR0 pin during STANDBY  this flag is        set. The bit shall be cleared explicitly after wakeup via        PMSWSTATCLR.ESR0WKPCLR bit before next STANDBY entry."]
    #[inline(always)]
    pub fn esr0wkp(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pmswstat2::Esr0Wkp,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pmswstat2::Esr0Wkp,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "ESR1 Wake up flag   ESR1WKP. In case wake up was triggered by ESR1 pin during STANDBY  this flag is        set. The bit shall be cleared explicitly after wakeup via        PMSWSTATCLR.ESR1WKPCLR bit before next STANDBY entry."]
    #[inline(always)]
    pub fn esr1wkp(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pmswstat2::Esr1Wkp,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pmswstat2::Esr1Wkp,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Pin Wake up flag   PINAWKP. In case wake up was triggered by PINA pin during STANDBY  this flag is        set. The bit shall be cleared explicitly after wakeup via        PMSWSTATCLR.PINAWKPCLR bit before next STANDBY entry."]
    #[inline(always)]
    pub fn pinawkp(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pmswstat2::Pinawkp,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pmswstat2::Pinawkp,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Pin B Wake up flag   PINBWKP. In case wake up was triggered by PINB pin during STANDBY  this flag is        set. The bit shall be cleared explicitly after wakeup via        PMSWSTATCLR.PINBWKPCLR bit before next STANDBY entry."]
    #[inline(always)]
    pub fn pinbwkp(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pmswstat2::Pinbwkp,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pmswstat2::Pinbwkp,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Wake up event on VEXT Supply ramp up   PWRWKP. In case wake up was triggered by VEXT ramp up pin during STANDBY  this        flag is set. The bit shall be cleared explicitly after wakeup via        PMSWSTATCLR.PWRWKPCLR bit before next STANDBY entry."]
    #[inline(always)]
    pub fn pwrwkp(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pmswstat2::Pwrwkp,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pmswstat2::Pwrwkp,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "SCR Wake up flag   SCRWKP. In case wake up is triggered by SCR to the main controller during        STANDBY  this flag is set. The bit shall be cleared explicitly after        wakeup via PMSWSTATCLR.SCRWKPCLR bit before next STANDBY entry."]
    #[inline(always)]
    pub fn scrwkp(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        pmswstat2::Scrwkp,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            pmswstat2::Scrwkp,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "PORST Wake up flag   PORSTWKP. In case wake up was triggered by PORST pin during STANDBY  this flag is        set. The bit shall be cleared explicitly after wakeup via        PMSWSTATCLR.PORSTWKPCLR bit before next STANDBY entry."]
    #[inline(always)]
    pub fn porstwkp(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pmswstat2::Porstwkp,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pmswstat2::Porstwkp,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "WUT Wake up flag   WUTWKP. In case wake up was triggered by Wake up timer during STANDBY  this flag        is set. The bit shall be cleared explicitly after wakeup via        PMSWSTATCLR.WUTWKPCLR bit before next STANDBY entry."]
    #[inline(always)]
    pub fn wutwkp(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pmswstat2::Wutwkp,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pmswstat2::Wutwkp,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "ESR0 Overrun status flag   ESR0OVRUN. This flag indicates that a consecutive ESR0 wake up event occurred while        ESR0WKP flag was already set during STANDBY. The bit shall be cleared        explicitly after wakeup via PMSWSTATCLR.ESR0OVRUNCLR bit before next        STANDBY entry."]
    #[inline(always)]
    pub fn esr0ovrun(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pmswstat2::Esr0Ovrun,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pmswstat2::Esr0Ovrun,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "ESR1 Overrun status flag   ESR1OVRUN. This flag indicates that a consecutive ESR1 wake up event occurred while        ESR1WKP flag was already set during STANDBY. The bit shall be cleared        explicitly after wakeup via PMSWSTATCLR.ESR1OVRUNCLR bit before next        STANDBY entry."]
    #[inline(always)]
    pub fn esr1ovrun(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pmswstat2::Esr1Ovrun,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pmswstat2::Esr1Ovrun,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Pin A Overrun status flag   PINAOVRUN. This flag indicates that a consecutive PINA wake up event occurred while        PINAWKP flag was already set during STANDBY. The bit shall be cleared        explicitly after wakeup via PMSWSTATCLR.PINAOVRUNCLR bit before next        STANDBY entry."]
    #[inline(always)]
    pub fn pinaovrun(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pmswstat2::Pinaovrun,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pmswstat2::Pinaovrun,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Pin B Overrun status flag   PINBOVRUN. This flag indicates that a consecutive PINB wake up event occurred while        PINBWKP flag was already set during STANDBY. The bit shall be cleared        explicitly after wakeup via PMSWSTATCLR.PINBOVRUNCLR bit before next        STANDBY entry."]
    #[inline(always)]
    pub fn pinbovrun(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        pmswstat2::Pinbovrun,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            pmswstat2::Pinbovrun,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Standby Entry Enable status on VDD Supply ramp down   VDDSTBYWKEN. This bit indicates that Standby Entry may be triggered on a VDD Supply        undervoltage event  VDDUV . This is supported only when Standby domain        is supplied separately by VEVRSB Standby supply pin.This bit is updated        when PMSWCR0.VDDSTBYWKEN bit is updated."]
    #[inline(always)]
    pub fn vddstbyen(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        pmswstat2::Vddstbyen,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            pmswstat2::Vddstbyen,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "SCR Overrun status flag   SCROVRUN. This flag indicates that a consecutive SCR wake up event occurred while        SCRWKP flag was already set during STANDBY. The bit shall be cleared        explicitly after wakeup via PMSWSTATCLR.SCROVRUNCLR bit before next        STANDBY entry."]
    #[inline(always)]
    pub fn scrovrun(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pmswstat2::Scrovrun,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pmswstat2::Scrovrun,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "PORST Overrun status flag   PORSTOVRUN. This flag indicates that a consecutive PORST wake up event occurred        while PORSTWKP flag was already set during STANDBY. The bit shall be        cleared explicitly after wakeup via PMSWSTATCLR.PORSTOVRUNCLR bit before        next STANDBY entry."]
    #[inline(always)]
    pub fn porstovrun(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pmswstat2::Porstovrun,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pmswstat2::Porstovrun,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "WUT Overrun status flag   WUTOVRUN. This flag indicates that a consecutive WUT wake up event occurred while        WUTWKP flag was already set during STANDBY. The bit shall be cleared        explicitly after wakeup via PMSWSTATCLR.WUTOVRUNCLR bit before next        STANDBY entry. WUTREL need to be greater than 10 during Standby mode to        be able to latch consecutive WUT underflow events and update the        WUTOVRRUN register bitfield."]
    #[inline(always)]
    pub fn wutovrun(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        pmswstat2::Wutovrun,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pmswstat2::Wutovrun,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Standby Entry Enable status on VEXT Supply ramp down   VEXTSTBYWKEN. This bit indicates that Standby Entry may be triggered on a VEXT Supply        undervoltage event  SWDUV . This is supported only when Standby domain        is supplied separately by VEVRSB Standby supply pin.This bit is updated        when PMSWCR0.VEXTSTBYWKEN bit is updated."]
    #[inline(always)]
    pub fn vextstbyen(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        pmswstat2::Vextstbyen,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            pmswstat2::Vextstbyen,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "ESR0 Wake up enable status   ESR0WKEN. This bit indicates that ESR0 is enabled to trigger wake up from Standby.        This bit is updated when PMSWCR0.ESR0WKEN bit is updated."]
    #[inline(always)]
    pub fn esr0wken(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        pmswstat2::Esr0Wken,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            pmswstat2::Esr0Wken,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "ESR1 Wake up enable status   ESR1WKEN. This bit indicates that ESR1 is enabled to trigger wake up from Standby.        This bit is updated when PMSWCR0.ESR1WKEN bit is updated."]
    #[inline(always)]
    pub fn esr1wken(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        pmswstat2::Esr1Wken,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            pmswstat2::Esr1Wken,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Pin A Wake up enable status   PINAWKEN. This bit indicates that PINA is enabled to trigger wake up from Standby.        This bit is updated when PMSWCR0.PINAWKEN bit is updated."]
    #[inline(always)]
    pub fn pinawken(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        pmswstat2::Pinawken,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            pmswstat2::Pinawken,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Pin B Wake up enable status   PINBWKEN. This bit indicates that PINB is enabled to trigger wake up from Standby.        This bit is updated when PMSWCR0.PINBWKEN bit is updated."]
    #[inline(always)]
    pub fn pinbwken(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        pmswstat2::Pinbwken,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            pmswstat2::Pinbwken,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Standby Wake up Enable status on VEXT Supply ramp up   PWRWKEN. This bit indicates that VEXT detector is enabled to trigger wake up from        Standby during VEXT supply ramp up after blanking filter time has        expired. This is supported only when Standby domain is supplied        separately by VEVRSB Standby supply pin.This bit is updated when        PMSWCR0.PWRWKEN bit is updated."]
    #[inline(always)]
    pub fn pwrwken(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        pmswstat2::Pwrwken,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            pmswstat2::Pwrwken,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Standby Controller Wake up Enable status   SCRWKEN. This bit indicates that SCR is enabled to trigger wake up from Standby.        This bit is updated when PMSWCR0.SCRWKEN bit is updated."]
    #[inline(always)]
    pub fn scrwken(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        pmswstat2::Scrwken,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            pmswstat2::Scrwken,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "PORST pin Wake up enable status from Standby   PORSTWKEN. This bit indicates that wake up via PORST pin is enabled during STANDBY        mode. This bit is updated when PMSWCR0. PORSTWKEN bit is updated."]
    #[inline(always)]
    pub fn porstwken(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        pmswstat2::Porstwken,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            pmswstat2::Porstwken,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "WUT Wake up enable status   WUTWKEN. This bit indicates that WUT is enabled to trigger wake up from Standby.        This bit is updated when PMSWCR0.WUTWKEN bit is updated."]
    #[inline(always)]
    pub fn wutwken(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        pmswstat2::Wutwken,
        Pmswstat2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            pmswstat2::Wutwken,
            Pmswstat2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Pmswstat2 {
    #[inline(always)]
    fn default() -> Pmswstat2 {
        <crate::RegValueT<Pmswstat2_SPEC> as RegisterValue<_>>::new(1048576)
    }
}
pub mod pmswstat2 {
    pub struct Esr0Wkp_SPEC;
    pub type Esr0Wkp = crate::EnumBitfieldStruct<u8, Esr0Wkp_SPEC>;
    impl Esr0Wkp {
        #[doc = "0 No wake up        event detected on ESR0 input during STANDBY."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An event as        defined by PMSWCR0. ESR0EDCON detected on ESR0 input.  evr esr0 wkp o"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esr1Wkp_SPEC;
    pub type Esr1Wkp = crate::EnumBitfieldStruct<u8, Esr1Wkp_SPEC>;
    impl Esr1Wkp {
        #[doc = "0 No wake up event detected on ESR1 input during STANDBY."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An event as        defined by PMSWCR0. ESR1EDCON detected on ESR1 input.  evr esr1 wkp o"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pinawkp_SPEC;
    pub type Pinawkp = crate::EnumBitfieldStruct<u8, Pinawkp_SPEC>;
    impl Pinawkp {
        #[doc = "0 No wake up event detected on Pin A input during STANDBY."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An event as        defined by PMSWCR0. PINAEDCON detected on Pin A input.  evr pina wkp o"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pinbwkp_SPEC;
    pub type Pinbwkp = crate::EnumBitfieldStruct<u8, Pinbwkp_SPEC>;
    impl Pinbwkp {
        #[doc = "0 No wake up event occurred on the Pin B input during STANDBY."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An event as        defined by PMSWCR0. PINBEDCON detected on Pin B input.  evr pinb wkp o"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pwrwkp_SPEC;
    pub type Pwrwkp = crate::EnumBitfieldStruct<u8, Pwrwkp_SPEC>;
    impl Pwrwkp {
        #[doc = "0 No VEXT supply wake up event detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 VEXT Monitor        threshold exceeded on VEXT supply ramp up leading to System Wake up from        STANDBY."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scrwkp_SPEC;
    pub type Scrwkp = crate::EnumBitfieldStruct<u8, Scrwkp_SPEC>;
    impl Scrwkp {
        #[doc = "0 No SCR wake up event detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A SCR wake up        event occurred.  evr scr wkp o"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Porstwkp_SPEC;
    pub type Porstwkp = crate::EnumBitfieldStruct<u8, Porstwkp_SPEC>;
    impl Porstwkp {
        #[doc = "0 No wake up event detected on PORST input during STANDBY if enabled via PMSWCR0.PORSTWKEN bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A wake up event        detected on PORST input if enabled via PMSWCR0.PORSTWKEN bit."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wutwkp_SPEC;
    pub type Wutwkp = crate::EnumBitfieldStruct<u8, Wutwkp_SPEC>;
    impl Wutwkp {
        #[doc = "0 No wake up event detected due to WUT underflow."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A wake up event        from STANDBY was detected due to WUT underflow.  evr wut wkp o"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esr0Ovrun_SPEC;
    pub type Esr0Ovrun = crate::EnumBitfieldStruct<u8, Esr0Ovrun_SPEC>;
    impl Esr0Ovrun {
        #[doc = "0 No overrun condition detected on ESR0 input."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An overrun        condition detected on ESR0 input.  evr esr0 ovr o"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esr1Ovrun_SPEC;
    pub type Esr1Ovrun = crate::EnumBitfieldStruct<u8, Esr1Ovrun_SPEC>;
    impl Esr1Ovrun {
        #[doc = "0 No overrun condition detected on ESR1 input."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An overrun        condition detected on ESR1 input.  evr esr1 ovr o"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pinaovrun_SPEC;
    pub type Pinaovrun = crate::EnumBitfieldStruct<u8, Pinaovrun_SPEC>;
    impl Pinaovrun {
        #[doc = "0 No overrun condition detected on Pin A input."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An overrun        condition detected on Pin A input.  evr pina ovr o"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pinbovrun_SPEC;
    pub type Pinbovrun = crate::EnumBitfieldStruct<u8, Pinbovrun_SPEC>;
    impl Pinbovrun {
        #[doc = "0 No overrun condition detected on Pin B input."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An overrun        condition detected on Pin B input.  evr pinb ovr o"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Vddstbyen_SPEC;
    pub type Vddstbyen = crate::EnumBitfieldStruct<u8, Vddstbyen_SPEC>;
    impl Vddstbyen {
        #[doc = "0 Standby Entry on VDD supply ramp down is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Standby Entry is enabled on a VDD Supply undervoltage event  SWDUV .        Blanking filter active on Standby mode entry."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scrovrun_SPEC;
    pub type Scrovrun = crate::EnumBitfieldStruct<u8, Scrovrun_SPEC>;
    impl Scrovrun {
        #[doc = "0 No overrun condition detected of SCR wake up event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An overrun condition detected of SCR wake up event."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Porstovrun_SPEC;
    pub type Porstovrun = crate::EnumBitfieldStruct<u8, Porstovrun_SPEC>;
    impl Porstovrun {
        #[doc = "0 No overrun condition detected on PORST input if enabled via PMSWCR0.PORSTWKEN bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An overrun condition detected on PORST input if enabled via PMSWCR0.PORSTWKEN bit."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wutovrun_SPEC;
    pub type Wutovrun = crate::EnumBitfieldStruct<u8, Wutovrun_SPEC>;
    impl Wutovrun {
        #[doc = "0 No overrun condition detected of WUT events."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An overrun        condition detected of WUT events.  evr wut ovr o"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Vextstbyen_SPEC;
    pub type Vextstbyen = crate::EnumBitfieldStruct<u8, Vextstbyen_SPEC>;
    impl Vextstbyen {
        #[doc = "0 Standby Entry on VEXT supply ramp down is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Standby Entry is enabled on a VEXT Supply undervoltage event  SWDUV .        Blanking filter active on Standby mode entry."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esr0Wken_SPEC;
    pub type Esr0Wken = crate::EnumBitfieldStruct<u8, Esr0Wken_SPEC>;
    impl Esr0Wken {
        #[doc = "0 Wake up from Standby via ESR0 is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Wake up from Standby via ESR0 is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esr1Wken_SPEC;
    pub type Esr1Wken = crate::EnumBitfieldStruct<u8, Esr1Wken_SPEC>;
    impl Esr1Wken {
        #[doc = "0 Wake up from Standby via ESR1 is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Wake up from Standby via ESR1 is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pinawken_SPEC;
    pub type Pinawken = crate::EnumBitfieldStruct<u8, Pinawken_SPEC>;
    impl Pinawken {
        #[doc = "0 Wake up from Standby via PINA is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Wake up from Standby via PINA is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pinbwken_SPEC;
    pub type Pinbwken = crate::EnumBitfieldStruct<u8, Pinbwken_SPEC>;
    impl Pinbwken {
        #[doc = "0 Wake up from Standby via PINB is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Wake up from Standby via PINB is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pwrwken_SPEC;
    pub type Pwrwken = crate::EnumBitfieldStruct<u8, Pwrwken_SPEC>;
    impl Pwrwken {
        #[doc = "0 Wake up        on VEXT supply ramp down disabled. Blanking filter configuration has no        effect."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Standby Wake up        on VEXT supply ramp up is enabled after blanking filter expiry."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scrwken_SPEC;
    pub type Scrwken = crate::EnumBitfieldStruct<u8, Scrwken_SPEC>;
    impl Scrwken {
        #[doc = "0 Wake up from Standby via SCR is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Wake up from Standby via SCR is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Porstwken_SPEC;
    pub type Porstwken = crate::EnumBitfieldStruct<u8, Porstwken_SPEC>;
    impl Porstwken {
        #[doc = "0 System wake up via PORST pin is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 System wake up via PORST pin is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wutwken_SPEC;
    pub type Wutwken = crate::EnumBitfieldStruct<u8, Wutwken_SPEC>;
    impl Wutwken {
        #[doc = "0 Wake up from Standby via WUT is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Wake up from Standby via WUT is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmswstatclr_SPEC;
impl crate::sealed::RegSpec for Pmswstatclr_SPEC {
    type DataType = u32;
}
#[doc = "Standby and Wake up Status Clear Register\n resetvalue={LVD Reset:0x0}"]
pub type Pmswstatclr = crate::RegValueT<Pmswstatclr_SPEC>;

impl Pmswstatclr {
    #[doc = "ESR0 Wake up indication flag clear   ESR0WKPCLR"]
    #[inline(always)]
    pub fn esr0wkpclr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        pmswstatclr::Esr0Wkpclr,
        Pmswstatclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            pmswstatclr::Esr0Wkpclr,
            Pmswstatclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "ESR1 Wake up indication flag clear   ESR1WKPCLR"]
    #[inline(always)]
    pub fn esr1wkpclr(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        pmswstatclr::Esr1Wkpclr,
        Pmswstatclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            pmswstatclr::Esr1Wkpclr,
            Pmswstatclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "PINA Wake up indication flag clear   PINAWKPCLR"]
    #[inline(always)]
    pub fn pinawkpclr(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        pmswstatclr::Pinawkpclr,
        Pmswstatclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            pmswstatclr::Pinawkpclr,
            Pmswstatclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "PINB Wake up indication flag clear   PINBWKPCLR"]
    #[inline(always)]
    pub fn pinbwkpclr(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        pmswstatclr::Pinbwkpclr,
        Pmswstatclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            pmswstatclr::Pinbwkpclr,
            Pmswstatclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "PWRWKP Wake up indication flag clear   PWRWKPCLR"]
    #[inline(always)]
    pub fn pwrwkpclr(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        pmswstatclr::Pwrwkpclr,
        Pmswstatclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            pmswstatclr::Pwrwkpclr,
            Pmswstatclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "SCR Wake up indication flag clear   SCRWKPCLR"]
    #[inline(always)]
    pub fn scrwkpclr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        pmswstatclr::Scrwkpclr,
        Pmswstatclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            pmswstatclr::Scrwkpclr,
            Pmswstatclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "PORST Wake up indication flag clear   PORSTWKPCLR"]
    #[inline(always)]
    pub fn porstwkpclr(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        pmswstatclr::Porstwkpclr,
        Pmswstatclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            pmswstatclr::Porstwkpclr,
            Pmswstatclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "WUT Wake up indication flag clear   WUTWKPCLR"]
    #[inline(always)]
    pub fn wutwkpclr(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        pmswstatclr::Wutwkpclr,
        Pmswstatclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            pmswstatclr::Wutwkpclr,
            Pmswstatclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "ESR0 Overrun status indication flag clear   ESR0OVRUNCLR"]
    #[inline(always)]
    pub fn esr0ovrunclr(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        pmswstatclr::Esr0Ovrunclr,
        Pmswstatclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            pmswstatclr::Esr0Ovrunclr,
            Pmswstatclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "ESR1 Overrun status indication flag clear   ESR1OVRUNCLR"]
    #[inline(always)]
    pub fn esr1ovrunclr(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        pmswstatclr::Esr1Ovrunclr,
        Pmswstatclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            pmswstatclr::Esr1Ovrunclr,
            Pmswstatclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "PINA Overrun status indication flag clear   PINAOVRUNCLR"]
    #[inline(always)]
    pub fn pinaovrunclr(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        pmswstatclr::Pinaovrunclr,
        Pmswstatclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            pmswstatclr::Pinaovrunclr,
            Pmswstatclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "PINB Overrun status indication flag clear   PINBOVRUNCLR"]
    #[inline(always)]
    pub fn pinbovrunclr(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        pmswstatclr::Pinbovrunclr,
        Pmswstatclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            pmswstatclr::Pinbovrunclr,
            Pmswstatclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "SCR Overrun status indication flag clear   SCROVRUNCLR"]
    #[inline(always)]
    pub fn scrovrunclr(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        pmswstatclr::Scrovrunclr,
        Pmswstatclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            pmswstatclr::Scrovrunclr,
            Pmswstatclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "PORST Overrun status indication flag clear   PORSTOVRUNCLR"]
    #[inline(always)]
    pub fn porstovrunclr(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        pmswstatclr::Porstovrunclr,
        Pmswstatclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            pmswstatclr::Porstovrunclr,
            Pmswstatclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "WUT Overrun status indication flag clear   WUTOVRUNCLR"]
    #[inline(always)]
    pub fn wutovrunclr(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        pmswstatclr::Wutovrunclr,
        Pmswstatclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            pmswstatclr::Wutovrunclr,
            Pmswstatclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Standby controller SCRST indication flag clear   SCRSTCLR"]
    #[inline(always)]
    pub fn scrstclr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        pmswstatclr::Scrstclr,
        Pmswstatclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            pmswstatclr::Scrstclr,
            Pmswstatclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "ESR0 Interrupt indication flag clear   ESR0INTCLR"]
    #[inline(always)]
    pub fn esr0intclr(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        pmswstatclr::Esr0Intclr,
        Pmswstatclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            pmswstatclr::Esr0Intclr,
            Pmswstatclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "ESR1 Interrupt indication flag clear   ESR1INTCLR"]
    #[inline(always)]
    pub fn esr1intclr(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        pmswstatclr::Esr1Intclr,
        Pmswstatclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            pmswstatclr::Esr1Intclr,
            Pmswstatclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "PINA Interrupt indication flag clear   PINAINTCLR"]
    #[inline(always)]
    pub fn pinaintclr(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        pmswstatclr::Pinaintclr,
        Pmswstatclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            pmswstatclr::Pinaintclr,
            Pmswstatclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "PINB Interrupt indication flag clear   PINBINTCLR"]
    #[inline(always)]
    pub fn pinbintclr(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        pmswstatclr::Pinbintclr,
        Pmswstatclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            pmswstatclr::Pinbintclr,
            Pmswstatclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Pmswstatclr {
    #[inline(always)]
    fn default() -> Pmswstatclr {
        <crate::RegValueT<Pmswstatclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pmswstatclr {
    pub struct Esr0Wkpclr_SPEC;
    pub type Esr0Wkpclr = crate::EnumBitfieldStruct<u8, Esr0Wkpclr_SPEC>;
    impl Esr0Wkpclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PMSWSTAT2.ESR0WKP        bit cleared.  evr esr0 wkp clr i"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esr1Wkpclr_SPEC;
    pub type Esr1Wkpclr = crate::EnumBitfieldStruct<u8, Esr1Wkpclr_SPEC>;
    impl Esr1Wkpclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PMSWSTAT2.ESR1WKP        bit cleared.  evr esr1 wkp clr i"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pinawkpclr_SPEC;
    pub type Pinawkpclr = crate::EnumBitfieldStruct<u8, Pinawkpclr_SPEC>;
    impl Pinawkpclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PMSWSTAT2.PINAWKP        bit cleared.  evr pina wkp clr i"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pinbwkpclr_SPEC;
    pub type Pinbwkpclr = crate::EnumBitfieldStruct<u8, Pinbwkpclr_SPEC>;
    impl Pinbwkpclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PMSWSTAT2.PINBWKP        bit cleared.  evr pinb wkp clr i"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pwrwkpclr_SPEC;
    pub type Pwrwkpclr = crate::EnumBitfieldStruct<u8, Pwrwkpclr_SPEC>;
    impl Pwrwkpclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PMSWSTAT2.PWRWKP        bit cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scrwkpclr_SPEC;
    pub type Scrwkpclr = crate::EnumBitfieldStruct<u8, Scrwkpclr_SPEC>;
    impl Scrwkpclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PMSWSTAT2.SCRWKP        bit cleared.  evr scr wkp clr i"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Porstwkpclr_SPEC;
    pub type Porstwkpclr = crate::EnumBitfieldStruct<u8, Porstwkpclr_SPEC>;
    impl Porstwkpclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PMSWSTAT2.PORSTWKP        bit cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wutwkpclr_SPEC;
    pub type Wutwkpclr = crate::EnumBitfieldStruct<u8, Wutwkpclr_SPEC>;
    impl Wutwkpclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PMSWSTAT2.WUTWKP        bit cleared.  evr wut wkp clr i"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esr0Ovrunclr_SPEC;
    pub type Esr0Ovrunclr = crate::EnumBitfieldStruct<u8, Esr0Ovrunclr_SPEC>;
    impl Esr0Ovrunclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PMSWSTAT2.ESR0OVRUN        bit cleared.  evr esr0 ovr clr i"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esr1Ovrunclr_SPEC;
    pub type Esr1Ovrunclr = crate::EnumBitfieldStruct<u8, Esr1Ovrunclr_SPEC>;
    impl Esr1Ovrunclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PMSWSTAT2.ESR1OVRUN        bit cleared.  evr esr1 ovr clr i"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pinaovrunclr_SPEC;
    pub type Pinaovrunclr = crate::EnumBitfieldStruct<u8, Pinaovrunclr_SPEC>;
    impl Pinaovrunclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PMSWSTAT2.PINAOVRUN        bit cleared.  evr pina ovr clr i"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pinbovrunclr_SPEC;
    pub type Pinbovrunclr = crate::EnumBitfieldStruct<u8, Pinbovrunclr_SPEC>;
    impl Pinbovrunclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PMSWSTAT2.PINBOVRUN        bit cleared.  evr pinb ovr clr i"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scrovrunclr_SPEC;
    pub type Scrovrunclr = crate::EnumBitfieldStruct<u8, Scrovrunclr_SPEC>;
    impl Scrovrunclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PMSWSTAT2.SCROVRUN        bit cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Porstovrunclr_SPEC;
    pub type Porstovrunclr = crate::EnumBitfieldStruct<u8, Porstovrunclr_SPEC>;
    impl Porstovrunclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PMSWSTAT2.PORSTOVRUN        bit cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wutovrunclr_SPEC;
    pub type Wutovrunclr = crate::EnumBitfieldStruct<u8, Wutovrunclr_SPEC>;
    impl Wutovrunclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PMSWSTAT2.WUTOVRUN        bit cleared.  evr wut ovr clr i"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scrstclr_SPEC;
    pub type Scrstclr = crate::EnumBitfieldStruct<u8, Scrstclr_SPEC>;
    impl Scrstclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PMSWSTAT.SCRST        bit cleared.  evr scr rst clr i"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esr0Intclr_SPEC;
    pub type Esr0Intclr = crate::EnumBitfieldStruct<u8, Esr0Intclr_SPEC>;
    impl Esr0Intclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PMSWSTAT.ESR0INT        bit cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Esr1Intclr_SPEC;
    pub type Esr1Intclr = crate::EnumBitfieldStruct<u8, Esr1Intclr_SPEC>;
    impl Esr1Intclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PMSWSTAT.ESR1INT        bit cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pinaintclr_SPEC;
    pub type Pinaintclr = crate::EnumBitfieldStruct<u8, Pinaintclr_SPEC>;
    impl Pinaintclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PMSWSTAT.PINAINT        bit cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pinbintclr_SPEC;
    pub type Pinbintclr = crate::EnumBitfieldStruct<u8, Pinbintclr_SPEC>;
    impl Pinbintclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PMSWSTAT.PINBINT        bit cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmswutcnt_SPEC;
impl crate::sealed::RegSpec for Pmswutcnt_SPEC {
    type DataType = u32;
}
#[doc = "Standby WUT Counter Register\n resetvalue={LVD Reset:0x0}"]
pub type Pmswutcnt = crate::RegValueT<Pmswutcnt_SPEC>;

impl Pmswutcnt {
    #[doc = "WUT counter value.   WUTCNT. The current WUT counter value is indicated in this register bitfield.        The WUTCNT value may have a deviation of 3 additional clock cycles to        the expected counter value owing to synchronization overheads. The WUT        clock is based on standby 70 kHz clock with       160 30  variation. The        counter depending on the mode can run through a RUN to STANDBY to RUN        mode transition without interruption."]
    #[inline(always)]
    pub fn wutcnt(
        self,
    ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, Pmswutcnt_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffff,1,0,u32, Pmswutcnt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pmswutcnt {
    #[inline(always)]
    fn default() -> Pmswutcnt {
        <crate::RegValueT<Pmswutcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tstctrl0_SPEC;
impl crate::sealed::RegSpec for Tstctrl0_SPEC {
    type DataType = u32;
}
#[doc = "Test Control Register 0\n resetvalue={LVD Reset:0x0}"]
pub type Tstctrl0 = crate::RegValueT<Tstctrl0_SPEC>;

impl Tstctrl0 {
    #[doc = "PMS or SMPS select   TESTSEL. This field specifies if SMPS or PMS tests are carried out."]
    #[inline(always)]
    pub fn testsel(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        tstctrl0::Testsel,
        Tstctrl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            tstctrl0::Testsel,
            Tstctrl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Test Mode Select   TESTMODSEL. This field selects the SMPS   PMS tests to be executed."]
    #[inline(always)]
    pub fn testmodsel(
        self,
    ) -> crate::common::RegisterField<
        4,
        0xf,
        1,
        0,
        tstctrl0::Testmodsel,
        Tstctrl0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0xf,
            1,
            0,
            tstctrl0::Testmodsel,
            Tstctrl0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Tstctrl0 {
    #[inline(always)]
    fn default() -> Tstctrl0 {
        <crate::RegValueT<Tstctrl0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tstctrl0 {
    pub struct Testsel_SPEC;
    pub type Testsel = crate::EnumBitfieldStruct<u8, Testsel_SPEC>;
    impl Testsel {
        #[doc = "00 Test disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 PMS  amp  EVR33 tests enabled"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 SMPS tests enabled"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Testmodsel_SPEC;
    pub type Testmodsel = crate::EnumBitfieldStruct<u8, Testmodsel_SPEC>;
    impl Testmodsel {
        #[doc = "0000 Test disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "0001 Test FB ADC Direct   adc swd bist start"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "0010 Test Gate Control Direct   adc 3v3 bist start"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "0011 Feedback ADC BIST   adc 1v2 bist start"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "0100 Test ADC   Gate Control Direct   evr33 bist start"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "1000 Leakage Test"]
        pub const CONST_88: Self = Self::new(8);
        #[doc = "1010 DLL BIST"]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "1011 OSCILLATOR BIST"]
        pub const CONST_1111: Self = Self::new(11);
    }
}
