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
#[doc = r"FPI Bus Control Unit"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbcu(pub(super) *mut u8);
unsafe impl core::marker::Send for Sbcu {}
unsafe impl core::marker::Sync for Sbcu {}
impl Sbcu {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(252usize)) }
    }

    #[doc = "BCU EDC Alarm Clear Register 0\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn alclrx(&self) -> [crate::common::Reg<self::AlclRx_SPEC, crate::common::W>; 4] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x70usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x70usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x70usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x70usize + 0xcusize)),
            ]
        }
    }

    #[doc = "BCU EDC Alarm Control Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn alctrl(&self) -> crate::common::Reg<self::Alctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "BCU EDC Alarm Status Register 0\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn alstatx(&self) -> [crate::common::Reg<self::AlstaTx_SPEC, crate::common::R>; 4] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x60usize + 0xcusize)),
            ]
        }
    }

    #[doc = "BCU Control Register\n resetvalue={Application Reset:0x0FF09FFFF}"]
    #[inline(always)]
    pub const fn con(&self) -> crate::common::Reg<self::Con_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "BCU Debug Address 1 Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn dbadr1(&self) -> crate::common::Reg<self::Dbadr1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }

    #[doc = "BCU Debug Address 2 Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn dbadr2(&self) -> crate::common::Reg<self::Dbadr2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "BCU Debug Trapped Address Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn dbadrt(&self) -> crate::common::Reg<self::Dbadrt_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }

    #[doc = "BCU Debug Bus Operation Signals Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn dbbos(&self) -> crate::common::Reg<self::Dbbos_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "BCU Debug Trapped Bus Operation Signals Register\n resetvalue={Debug Reset:0x3180}"]
    #[inline(always)]
    pub const fn dbbost(&self) -> crate::common::Reg<self::Dbbost_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(76usize)) }
    }

    #[doc = "BCU Debug Control Register\n resetvalue={Debug Reset:0x7003}"]
    #[inline(always)]
    pub const fn dbcntl(&self) -> crate::common::Reg<self::Dbcntl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }

    #[doc = "BCU Debug Data Status Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn dbdat(&self) -> crate::common::Reg<self::Dbdat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }

    #[doc = "SBCU Debug Trapped Master Register\n resetvalue={Debug Reset:0x0FFFF}"]
    #[inline(always)]
    pub const fn dbgntt(&self) -> crate::common::Reg<self::Dbgntt_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }

    #[doc = "SBCU Debug Grant Mask Register\n resetvalue={Debug Reset:0x0FFFF}"]
    #[inline(always)]
    pub const fn dbgrnt(&self) -> crate::common::Reg<self::Dbgrnt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "BCU Error Address Capture Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn eadd(&self) -> crate::common::Reg<self::Eadd_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "BCU Error Control Capture Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn econ(&self) -> crate::common::Reg<self::Econ_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "BCU Error Data Capture Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn edat(&self) -> crate::common::Reg<self::Edat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x6A00}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Arbiter Priority Register High\n resetvalue={Application Reset:0x0FEDCBA98,Application Reset:0x0FEDC8888,Application Reset:0x0FEDCBA98}"]
    #[inline(always)]
    pub const fn prioh(&self) -> crate::common::Reg<self::Prioh_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Arbiter Priority Register Low\n resetvalue={Application Reset:0x76543210,Application Reset:0x088543210,Application Reset:0x76588210}"]
    #[inline(always)]
    pub const fn priol(&self) -> crate::common::Reg<self::Priol_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
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
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, accen0::En0, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,accen0::En0, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, accen0::En1, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,accen0::En1, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, accen0::En2, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,accen0::En2, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, accen0::En3, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,accen0::En3, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, accen0::En4, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,accen0::En4, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, accen0::En5, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,accen0::En5, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, accen0::En6, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,accen0::En6, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, accen0::En7, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,accen0::En7, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, accen0::En8, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,accen0::En8, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, accen0::En9, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,accen0::En9, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, accen0::En10, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,accen0::En10, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, accen0::En11, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,accen0::En11, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, accen0::En12, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,accen0::En12, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, accen0::En13, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,accen0::En13, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, accen0::En14, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,accen0::En14, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, accen0::En15, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,accen0::En15, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, accen0::En16, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,accen0::En16, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, accen0::En17, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,accen0::En17, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, accen0::En18, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,accen0::En18, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, accen0::En19, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,accen0::En19, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, accen0::En20, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,accen0::En20, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, accen0::En21, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,accen0::En21, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, accen0::En22, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,accen0::En22, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, accen0::En23, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,accen0::En23, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, accen0::En24, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,accen0::En24, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, accen0::En25, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,accen0::En25, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, accen0::En26, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,accen0::En26, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, accen0::En27, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,accen0::En27, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, accen0::En28, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,accen0::En28, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, accen0::En29, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,accen0::En29, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, accen0::En30, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,accen0::En30, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
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
pub struct AlclRx_SPEC;
impl crate::sealed::RegSpec for AlclRx_SPEC {
    type DataType = u32;
}
#[doc = "BCU EDC Alarm Clear Register 0\n resetvalue={Debug Reset:0x0}"]
pub type AlclRx = crate::RegValueT<AlclRx_SPEC>;

impl AlclRx {
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr00(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, alclrx::Clr00, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x1,1,0,alclrx::Clr00, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr01(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, alclrx::Clr01, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<1,0x1,1,0,alclrx::Clr01, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr02(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, alclrx::Clr02, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<2,0x1,1,0,alclrx::Clr02, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr03(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, alclrx::Clr03, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<3,0x1,1,0,alclrx::Clr03, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr04(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, alclrx::Clr04, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<4,0x1,1,0,alclrx::Clr04, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr05(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, alclrx::Clr05, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<5,0x1,1,0,alclrx::Clr05, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr06(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, alclrx::Clr06, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<6,0x1,1,0,alclrx::Clr06, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr07(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, alclrx::Clr07, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<7,0x1,1,0,alclrx::Clr07, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr08(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, alclrx::Clr08, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0x1,1,0,alclrx::Clr08, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr09(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, alclrx::Clr09, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<9,0x1,1,0,alclrx::Clr09, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, alclrx::Clr10, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<10,0x1,1,0,alclrx::Clr10, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, alclrx::Clr11, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<11,0x1,1,0,alclrx::Clr11, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, alclrx::Clr12, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<12,0x1,1,0,alclrx::Clr12, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, alclrx::Clr13, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<13,0x1,1,0,alclrx::Clr13, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, alclrx::Clr14, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<14,0x1,1,0,alclrx::Clr14, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, alclrx::Clr15, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<15,0x1,1,0,alclrx::Clr15, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr16(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, alclrx::Clr16, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0x1,1,0,alclrx::Clr16, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, alclrx::Clr17, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<17,0x1,1,0,alclrx::Clr17, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr18(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, alclrx::Clr18, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<18,0x1,1,0,alclrx::Clr18, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr19(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, alclrx::Clr19, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<19,0x1,1,0,alclrx::Clr19, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr20(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, alclrx::Clr20, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<20,0x1,1,0,alclrx::Clr20, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr21(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, alclrx::Clr21, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<21,0x1,1,0,alclrx::Clr21, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, alclrx::Clr22, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<22,0x1,1,0,alclrx::Clr22, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, alclrx::Clr23, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<23,0x1,1,0,alclrx::Clr23, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, alclrx::Clr24, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<24,0x1,1,0,alclrx::Clr24, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr25(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, alclrx::Clr25, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<25,0x1,1,0,alclrx::Clr25, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr26(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, alclrx::Clr26, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<26,0x1,1,0,alclrx::Clr26, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr27(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, alclrx::Clr27, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<27,0x1,1,0,alclrx::Clr27, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr28(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, alclrx::Clr28, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<28,0x1,1,0,alclrx::Clr28, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr29(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, alclrx::Clr29, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<29,0x1,1,0,alclrx::Clr29, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, alclrx::Clr30, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<30,0x1,1,0,alclrx::Clr30, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn clr31(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, alclrx::Clr31, AlclRx_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<31,0x1,1,0,alclrx::Clr31, AlclRx_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for AlclRx {
    #[inline(always)]
    fn default() -> AlclRx {
        <crate::RegValueT<AlclRx_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod alclrx {
    pub struct Clr00_SPEC;
    pub type Clr00 = crate::EnumBitfieldStruct<u8, Clr00_SPEC>;
    impl Clr00 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr01_SPEC;
    pub type Clr01 = crate::EnumBitfieldStruct<u8, Clr01_SPEC>;
    impl Clr01 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr02_SPEC;
    pub type Clr02 = crate::EnumBitfieldStruct<u8, Clr02_SPEC>;
    impl Clr02 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr03_SPEC;
    pub type Clr03 = crate::EnumBitfieldStruct<u8, Clr03_SPEC>;
    impl Clr03 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr04_SPEC;
    pub type Clr04 = crate::EnumBitfieldStruct<u8, Clr04_SPEC>;
    impl Clr04 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr05_SPEC;
    pub type Clr05 = crate::EnumBitfieldStruct<u8, Clr05_SPEC>;
    impl Clr05 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr06_SPEC;
    pub type Clr06 = crate::EnumBitfieldStruct<u8, Clr06_SPEC>;
    impl Clr06 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr07_SPEC;
    pub type Clr07 = crate::EnumBitfieldStruct<u8, Clr07_SPEC>;
    impl Clr07 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr08_SPEC;
    pub type Clr08 = crate::EnumBitfieldStruct<u8, Clr08_SPEC>;
    impl Clr08 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr09_SPEC;
    pub type Clr09 = crate::EnumBitfieldStruct<u8, Clr09_SPEC>;
    impl Clr09 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr10_SPEC;
    pub type Clr10 = crate::EnumBitfieldStruct<u8, Clr10_SPEC>;
    impl Clr10 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr11_SPEC;
    pub type Clr11 = crate::EnumBitfieldStruct<u8, Clr11_SPEC>;
    impl Clr11 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr12_SPEC;
    pub type Clr12 = crate::EnumBitfieldStruct<u8, Clr12_SPEC>;
    impl Clr12 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr13_SPEC;
    pub type Clr13 = crate::EnumBitfieldStruct<u8, Clr13_SPEC>;
    impl Clr13 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr14_SPEC;
    pub type Clr14 = crate::EnumBitfieldStruct<u8, Clr14_SPEC>;
    impl Clr14 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr15_SPEC;
    pub type Clr15 = crate::EnumBitfieldStruct<u8, Clr15_SPEC>;
    impl Clr15 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr16_SPEC;
    pub type Clr16 = crate::EnumBitfieldStruct<u8, Clr16_SPEC>;
    impl Clr16 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr17_SPEC;
    pub type Clr17 = crate::EnumBitfieldStruct<u8, Clr17_SPEC>;
    impl Clr17 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr18_SPEC;
    pub type Clr18 = crate::EnumBitfieldStruct<u8, Clr18_SPEC>;
    impl Clr18 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr19_SPEC;
    pub type Clr19 = crate::EnumBitfieldStruct<u8, Clr19_SPEC>;
    impl Clr19 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr20_SPEC;
    pub type Clr20 = crate::EnumBitfieldStruct<u8, Clr20_SPEC>;
    impl Clr20 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr21_SPEC;
    pub type Clr21 = crate::EnumBitfieldStruct<u8, Clr21_SPEC>;
    impl Clr21 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr22_SPEC;
    pub type Clr22 = crate::EnumBitfieldStruct<u8, Clr22_SPEC>;
    impl Clr22 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr23_SPEC;
    pub type Clr23 = crate::EnumBitfieldStruct<u8, Clr23_SPEC>;
    impl Clr23 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr24_SPEC;
    pub type Clr24 = crate::EnumBitfieldStruct<u8, Clr24_SPEC>;
    impl Clr24 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr25_SPEC;
    pub type Clr25 = crate::EnumBitfieldStruct<u8, Clr25_SPEC>;
    impl Clr25 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr26_SPEC;
    pub type Clr26 = crate::EnumBitfieldStruct<u8, Clr26_SPEC>;
    impl Clr26 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr27_SPEC;
    pub type Clr27 = crate::EnumBitfieldStruct<u8, Clr27_SPEC>;
    impl Clr27 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr28_SPEC;
    pub type Clr28 = crate::EnumBitfieldStruct<u8, Clr28_SPEC>;
    impl Clr28 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr29_SPEC;
    pub type Clr29 = crate::EnumBitfieldStruct<u8, Clr29_SPEC>;
    impl Clr29 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr30_SPEC;
    pub type Clr30 = crate::EnumBitfieldStruct<u8, Clr30_SPEC>;
    impl Clr30 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr31_SPEC;
    pub type Clr31 = crate::EnumBitfieldStruct<u8, Clr31_SPEC>;
    impl Clr31 {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears related SBC ALSTATx. y   clear   set to  0    read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Alctrl_SPEC;
impl crate::sealed::RegSpec for Alctrl_SPEC {
    type DataType = u32;
}
#[doc = "BCU EDC Alarm Control Register\n resetvalue={Debug Reset:0x0}"]
pub type Alctrl = crate::RegValueT<Alctrl_SPEC>;

impl Alctrl {
    #[doc = "Alarm Overflow. The ALOV bit is set if multiple FPI EDC alarms for the same FPI slave or        the same FPI master were detected while the related ALSTATx y  bit was        still set. Some errors result in a static fault situation  for example address          phase  data phase or data enable signal faults. Static faults do not          generate multiple alarms and will will not set the ALOV bit."]
    #[inline(always)]
    pub fn alov(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, alctrl::Alov, Alctrl_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,alctrl::Alov, Alctrl_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Alarm Overflow Clear. The ALOVCLR bit is required to reset the ALOV bit."]
    #[inline(always)]
    pub fn alovclr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, alctrl::Alovclr, Alctrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<1,0x1,1,0,alctrl::Alovclr, Alctrl_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Alctrl {
    #[inline(always)]
    fn default() -> Alctrl {
        <crate::RegValueT<Alctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod alctrl {
    pub struct Alov_SPEC;
    pub type Alov = crate::EnumBitfieldStruct<u8, Alov_SPEC>;
    impl Alov {
        #[doc = "0 No Alarm Overflow for any FPI EDC alarm detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Alarm Overflow detected for at least one of the set ALSTATx y  bits"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Alovclr_SPEC;
    pub type Alovclr = crate::EnumBitfieldStruct<u8, Alovclr_SPEC>;
    impl Alovclr {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear ALOV  clear   set to 0   bit value is not stored  read always returns 0"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlstaTx_SPEC;
impl crate::sealed::RegSpec for AlstaTx_SPEC {
    type DataType = u32;
}
#[doc = "BCU EDC Alarm Status Register 0\n resetvalue={Debug Reset:0x0}"]
pub type AlstaTx = crate::RegValueT<AlstaTx_SPEC>;

impl AlstaTx {
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al0(self) -> crate::common::RegisterFieldBool<0, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al1(self) -> crate::common::RegisterFieldBool<1, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al2(self) -> crate::common::RegisterFieldBool<2, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al3(self) -> crate::common::RegisterFieldBool<3, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al4(self) -> crate::common::RegisterFieldBool<4, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al5(self) -> crate::common::RegisterFieldBool<5, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al6(self) -> crate::common::RegisterFieldBool<6, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al7(self) -> crate::common::RegisterFieldBool<7, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al8(self) -> crate::common::RegisterFieldBool<8, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al9(self) -> crate::common::RegisterFieldBool<9, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Alarm 31. The Alarm bit shows if an EDC error was detected in an active phase of        the related FPI Slave   Master interface."]
    #[inline(always)]
    pub fn al31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, AlstaTx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, AlstaTx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for AlstaTx {
    #[inline(always)]
    fn default() -> AlstaTx {
        <crate::RegValueT<AlstaTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Con_SPEC;
impl crate::sealed::RegSpec for Con_SPEC {
    type DataType = u32;
}
#[doc = "BCU Control Register\n resetvalue={Application Reset:0x0FF09FFFF}"]
pub type Con = crate::RegValueT<Con_SPEC>;

impl Con {
    #[doc = "BCU Bus Time Out Value. The bit field determines the number of System Peripheral Bus time out        cycles. Default after reset is FFFF H    65536 bus cycles . TOUT value must be  gt   5."]
    #[inline(always)]
    pub fn tout(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "BCU Debug Trace Enable. The bit enables disables the error capture mechanism for the registers        BCU ECON  BCU EADD  BCU EDAT. The bit does not affect the SMU alarm or the BCU interrupt that are send        on detection case of an error condition."]
    #[inline(always)]
    pub fn dbg(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, con::Dbg, Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1,1,0,con::Dbg, Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Starvation Period Control. Determines the sample period for the starvation counter. Must be larger        than the number of masters. The reset value is FF H ."]
    #[inline(always)]
    pub fn spc(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Con_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Con {
    #[inline(always)]
    fn default() -> Con {
        <crate::RegValueT<Con_SPEC> as RegisterValue<_>>::new(4278845439)
    }
}
pub mod con {
    pub struct Dbg_SPEC;
    pub type Dbg = crate::EnumBitfieldStruct<u8, Dbg_SPEC>;
    impl Dbg {
        #[doc = "0 SBCU debug trace disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SBCU debug trace enabled  default after reset"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbadr1_SPEC;
impl crate::sealed::RegSpec for Dbadr1_SPEC {
    type DataType = u32;
}
#[doc = "BCU Debug Address 1 Register\n resetvalue={Debug Reset:0x0}"]
pub type Dbadr1 = crate::RegValueT<Dbadr1_SPEC>;

impl Dbadr1 {
    #[doc = "Debug Trigger Address 1. This register contains the address for the address 1 trigger event        generation."]
    #[inline(always)]
    pub fn adr1(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Dbadr1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Dbadr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dbadr1 {
    #[inline(always)]
    fn default() -> Dbadr1 {
        <crate::RegValueT<Dbadr1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbadr2_SPEC;
impl crate::sealed::RegSpec for Dbadr2_SPEC {
    type DataType = u32;
}
#[doc = "BCU Debug Address 2 Register\n resetvalue={Debug Reset:0x0}"]
pub type Dbadr2 = crate::RegValueT<Dbadr2_SPEC>;

impl Dbadr2 {
    #[doc = "Debug Trigger Address 2. This register contains the address for the address 2 trigger event        generation."]
    #[inline(always)]
    pub fn adr2(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Dbadr2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Dbadr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dbadr2 {
    #[inline(always)]
    fn default() -> Dbadr2 {
        <crate::RegValueT<Dbadr2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbadrt_SPEC;
impl crate::sealed::RegSpec for Dbadrt_SPEC {
    type DataType = u32;
}
#[doc = "BCU Debug Trapped Address Register\n resetvalue={Debug Reset:0x0}"]
pub type Dbadrt = crate::RegValueT<Dbadrt_SPEC>;

impl Dbadrt {
    #[doc = "FPI Bus Address Status. This register contains the FPI  160 Bus address that was captured when the        OCDS break trigger event occurred."]
    #[inline(always)]
    pub fn fpiadr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Dbadrt_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Dbadrt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Dbadrt {
    #[inline(always)]
    fn default() -> Dbadrt {
        <crate::RegValueT<Dbadrt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbbos_SPEC;
impl crate::sealed::RegSpec for Dbbos_SPEC {
    type DataType = u32;
}
#[doc = "BCU Debug Bus Operation Signals Register\n resetvalue={Debug Reset:0x0}"]
pub type Dbbos = crate::RegValueT<Dbbos_SPEC>;

impl Dbbos {
    #[doc = "SVM Signal for Status Debug Trigger. This bit determines the mode of an FPI  160 Bus transaction for which a        signal status debug trigger event is generated  if enabled by        DBCNTL.ONBOS1  160    160 1 ."]
    #[inline(always)]
    pub fn svm(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, dbbos::Svm, Dbbos_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,dbbos::Svm, Dbbos_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Write Signal for Status Debug Trigger. This bit determines the state of the WR signal of an FPI  160 Bus transaction for which a signal status debug trigger        event is generated  if enabled by DBCNTL.ONBOS2  160    160 1 ."]
    #[inline(always)]
    pub fn wr(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, dbbos::Wr, Dbbos_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,dbbos::Wr, Dbbos_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Write Signal for Status Debug Trigger. This bit determines the state of the RD signal of an FPI  160 Bus transaction for which a signal status debug trigger        event is generated  if enabled by DBCNTL.ONBOS3  160    160 1 ."]
    #[inline(always)]
    pub fn rd(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, dbbos::Rd, Dbbos_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x1,1,0,dbbos::Rd, Dbbos_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dbbos {
    #[inline(always)]
    fn default() -> Dbbos {
        <crate::RegValueT<Dbbos_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dbbos {
    pub struct Svm_SPEC;
    pub type Svm = crate::EnumBitfieldStruct<u8, Svm_SPEC>;
    impl Svm {
        #[doc = "0 Trigger on User        Mode selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Trigger on        Supervisor Mode selected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Wr_SPEC;
    pub type Wr = crate::EnumBitfieldStruct<u8, Wr_SPEC>;
    impl Wr {
        #[doc = "0 Trigger on a single write transfer or write cycle of an atomic transfer selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No operation or read transaction selected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rd_SPEC;
    pub type Rd = crate::EnumBitfieldStruct<u8, Rd_SPEC>;
    impl Rd {
        #[doc = "0 Trigger on a single read transfer or read cycle of an atomic transfer selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No operation or write transfer selected"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbbost_SPEC;
impl crate::sealed::RegSpec for Dbbost_SPEC {
    type DataType = u32;
}
#[doc = "BCU Debug Trapped Bus Operation Signals Register\n resetvalue={Debug Reset:0x3180}"]
pub type Dbbost = crate::RegValueT<Dbbost_SPEC>;

impl Dbbost {
    #[doc = "FPI Bus Supervisor Mode Status. This bit indicates the state of the Supervisor Mode signal captured from        the FPI  160 Bus signal lines when the BCU break trigger event occurred."]
    #[inline(always)]
    pub fn fpisvm(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, dbbost::Fpisvm, Dbbost_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x1,1,0,dbbost::Fpisvm, Dbbost_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "FPI Bus Acknowledge Status. This bit field indicates the acknowledge signal status captured from the        FPI  160 Bus signal lines when the BCU break trigger event occurred. Coding        see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn fpiack(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, dbbost::Fpiack, Dbbost_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x3,1,0,dbbost::Fpiack, Dbbost_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "FPI Bus Ready Status. This bit indicates the ready signal status captured from the FPI  160 Bus        signal lines when the BCU break trigger event occurred."]
    #[inline(always)]
    pub fn fpirdy(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, dbbost::Fpirdy, Dbbost_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<7,0x1,1,0,dbbost::Fpirdy, Dbbost_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "FPI Bus Write Indication Status. This bit indicates the write signal status captured from the FPI  160 Bus        signal lines when the BCU break trigger event occurred."]
    #[inline(always)]
    pub fn fpiwr(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, dbbost::Fpiwr, Dbbost_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x1,1,0,dbbost::Fpiwr, Dbbost_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "FPI Bus OCDS Suspend Status. This bit indicates the OCDS suspend signal status captured from the        FPI  160 Bus signal lines when the BCU break trigger event occurred."]
    #[inline(always)]
    pub fn fpiops(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, dbbost::Fpiops, Dbbost_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<11,0x1,1,0,dbbost::Fpiops, Dbbost_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "FPI Bus Read Indication Status. This bit indicates the read signal status captured from the FPI  160 Bus        signal lines when the BCU break trigger event occurred."]
    #[inline(always)]
    pub fn fpird(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, dbbost::Fpird, Dbbost_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<12,0x1,1,0,dbbost::Fpird, Dbbost_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "FPI Bus Abort Status. This bit indicates the abort signal status captured from the FPI  160 Bus        signal lines when the BCU break trigger event occurred."]
    #[inline(always)]
    pub fn fpiabort(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, dbbost::Fpiabort, Dbbost_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<13,0x1,1,0,dbbost::Fpiabort, Dbbost_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "FPI Bus Time out Status. This bit indicates the time out signal status captured from the FPI  160 Bus        signal lines when the BCU break trigger event occurred."]
    #[inline(always)]
    pub fn fpitout(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, dbbost::Fpitout, Dbbost_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<14,0x1,1,0,dbbost::Fpitout, Dbbost_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "FPI Bus Endinit Status. This bit indicates the ENDINIT signal status captured from the FPI  160 Bus        signal lines when the BCU break trigger event occurred."]
    #[inline(always)]
    pub fn endinit(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, dbbost::Endinit, Dbbost_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<15,0x1,1,0,dbbost::Endinit, Dbbost_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "FPI Bus Master TAG Status. This bit field indicates the master TAG captured from the FPI  160 Bus signal        lines when the BCU break trigger event occurred  see CROSSREFERENCE  .        The master TAG identifies the master of the transfer which generated BCU        break trigger event."]
    #[inline(always)]
    pub fn fpitag(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Dbbost_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Dbbost_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Dbbost {
    #[inline(always)]
    fn default() -> Dbbost {
        <crate::RegValueT<Dbbost_SPEC> as RegisterValue<_>>::new(12672)
    }
}
pub mod dbbost {
    pub struct Fpisvm_SPEC;
    pub type Fpisvm = crate::EnumBitfieldStruct<u8, Fpisvm_SPEC>;
    impl Fpisvm {
        #[doc = "0 User mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Supervisor mode"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fpiack_SPEC;
    pub type Fpiack = crate::EnumBitfieldStruct<u8, Fpiack_SPEC>;
    impl Fpiack {
        #[doc = "00 NSC  No        special case"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 RTY  Retry"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 ERR  Bus Error"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Fpirdy_SPEC;
    pub type Fpirdy = crate::EnumBitfieldStruct<u8, Fpirdy_SPEC>;
    impl Fpirdy {
        #[doc = "0 Last cycle of transfer"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Not last cycle of transfer"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fpiwr_SPEC;
    pub type Fpiwr = crate::EnumBitfieldStruct<u8, Fpiwr_SPEC>;
    impl Fpiwr {
        #[doc = "0 Single write transfer or write cycle of an atomic transfer"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No operation or read transfer"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fpiops_SPEC;
    pub type Fpiops = crate::EnumBitfieldStruct<u8, Fpiops_SPEC>;
    impl Fpiops {
        #[doc = "0 No OCDS suspend request is pending"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An OCDS suspend request is pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fpird_SPEC;
    pub type Fpird = crate::EnumBitfieldStruct<u8, Fpird_SPEC>;
    impl Fpird {
        #[doc = "0 Single read transfer or read cycle of an atomic transfer"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No operation or write transfer"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fpiabort_SPEC;
    pub type Fpiabort = crate::EnumBitfieldStruct<u8, Fpiabort_SPEC>;
    impl Fpiabort {
        #[doc = "0 A transfer that has already started was aborted"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Normal operation"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fpitout_SPEC;
    pub type Fpitout = crate::EnumBitfieldStruct<u8, Fpitout_SPEC>;
    impl Fpitout {
        #[doc = "0 Normal operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A time out event was generated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Endinit_SPEC;
    pub type Endinit = crate::EnumBitfieldStruct<u8, Endinit_SPEC>;
    impl Endinit {
        #[doc = "0 Normal operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 System was in ENDINIT state"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbcntl_SPEC;
impl crate::sealed::RegSpec for Dbcntl_SPEC {
    type DataType = u32;
}
#[doc = "BCU Debug Control Register\n resetvalue={Debug Reset:0x7003}"]
pub type Dbcntl = crate::RegValueT<Dbcntl_SPEC>;

impl Dbcntl {
    #[doc = "Status of BCU Debug Support Enable. This bit is controlled by the Cerberus and enables the BCU debug support."]
    #[inline(always)]
    pub fn eo(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, dbcntl::Eo, Dbcntl_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,dbcntl::Eo, Dbcntl_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Status of BCU Breakpoint Logic. The OA bit is set by writing a 1 to bit RA. When OA is set  registers        DBGNTT  DBADRT and DBDAT are reset. Also DBBOST is reset with the        exception of the bit field FPIRST."]
    #[inline(always)]
    pub fn oa(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, dbcntl::Oa, Dbcntl_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,dbcntl::Oa, Dbcntl_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Rearm BCU Breakpoint Logic. Writing a 1 to this bit rearms BCU breakpoint logic and sets bit OA  160    160 1.        RA is always reads as 0."]
    #[inline(always)]
    pub fn ra(self) -> crate::common::RegisterFieldBool<4, 1, 0, Dbcntl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Dbcntl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Status of HSM Transaction Trace Logic"]
    #[inline(always)]
    pub fn hsmtrtren(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, dbcntl::Hsmtrtren, Dbcntl_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x3,1,0,dbcntl::Hsmtrtren, Dbcntl_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Status of HSM Debug Mode"]
    #[inline(always)]
    pub fn hsmdbgen(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, dbcntl::Hsmdbgen, Dbcntl_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<7,0x1,1,0,dbcntl::Hsmdbgen, Dbcntl_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Grant and Address Trigger Relation"]
    #[inline(always)]
    pub fn concom0(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, dbcntl::Concom0, Dbcntl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,dbcntl::Concom0, Dbcntl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Address 1 and Address 2 Trigger Relation"]
    #[inline(always)]
    pub fn concom1(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, dbcntl::Concom1, Dbcntl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,dbcntl::Concom1, Dbcntl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Address and Signal Trigger Relation"]
    #[inline(always)]
    pub fn concom2(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, dbcntl::Concom2, Dbcntl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,dbcntl::Concom2, Dbcntl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Grant Trigger Enable"]
    #[inline(always)]
    pub fn ong(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, dbcntl::Ong, Dbcntl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,dbcntl::Ong, Dbcntl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Address 1 Trigger Control. See also CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ona1(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, dbcntl::Ona1, Dbcntl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3,1,0,dbcntl::Ona1, Dbcntl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Address 2 Trigger Control. See also CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ona2(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, dbcntl::Ona2, Dbcntl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x3,1,0,dbcntl::Ona2, Dbcntl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Op code Signal Status Trigger Condition"]
    #[inline(always)]
    pub fn onbos0(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, dbcntl::Onbos0, Dbcntl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,dbcntl::Onbos0, Dbcntl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Supervisor Mode Signal Trigger Condition"]
    #[inline(always)]
    pub fn onbos1(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, dbcntl::Onbos1, Dbcntl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,dbcntl::Onbos1, Dbcntl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Write Signal Trigger Condition"]
    #[inline(always)]
    pub fn onbos2(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, dbcntl::Onbos2, Dbcntl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,dbcntl::Onbos2, Dbcntl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Read Signal Trigger Condition"]
    #[inline(always)]
    pub fn onbos3(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, dbcntl::Onbos3, Dbcntl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,dbcntl::Onbos3, Dbcntl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dbcntl {
    #[inline(always)]
    fn default() -> Dbcntl {
        <crate::RegValueT<Dbcntl_SPEC> as RegisterValue<_>>::new(28675)
    }
}
pub mod dbcntl {
    pub struct Eo_SPEC;
    pub type Eo = crate::EnumBitfieldStruct<u8, Eo_SPEC>;
    impl Eo {
        #[doc = "BCU debug support is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "BCU debug support is enabled  default after reset"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Oa_SPEC;
    pub type Oa = crate::EnumBitfieldStruct<u8, Oa_SPEC>;
    impl Oa {
        #[doc = "The BCU breakpoint logic is disarmed. Any further breakpoint activation is discarded"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "The BCU breakpoint logic is armed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hsmtrtren_SPEC;
    pub type Hsmtrtren = crate::EnumBitfieldStruct<u8, Hsmtrtren_SPEC>;
    impl Hsmtrtren {
        #[doc = "00 HSM        transaction tracing and capturing of HSM transactions in the BCU is        disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 HSM        transaction tracing and capturing of HSM transactions in the BCU is        disabled"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 HSM        transaction address can be traced  capturing of HSM transactions in the        BCU is allowed"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 HSM        transaction address and data can be traced  capturing of HSM        transactions in the BCU is allowed"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Hsmdbgen_SPEC;
    pub type Hsmdbgen = crate::EnumBitfieldStruct<u8, Hsmdbgen_SPEC>;
    impl Hsmdbgen {
        #[doc = "0 HSM module is not in debug mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 HSM module is in debug mode"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Concom0_SPEC;
    pub type Concom0 = crate::EnumBitfieldStruct<u8, Concom0_SPEC>;
    impl Concom0 {
        #[doc = "0 The grant phase trigger condition and the address trigger condition  see CONCOM1  are combined with a logical OR for further control"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "The grant phase trigger condition and the address trigger condition  see CONCOM1  are combined with a logical AND for further control..  see CROSSREFERENCE"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Concom1_SPEC;
    pub type Concom1 = crate::EnumBitfieldStruct<u8, Concom1_SPEC>;
    impl Concom1 {
        #[doc = "0 Address 1 trigger condition and address 2 trigger condition are combined with a logical OR to the address trigger condition for further control"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Address 1 trigger condition and address 2 trigger condition are combined with a logical AND to the address trigger condition for further control.  see CROSSREFERENCE"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Concom2_SPEC;
    pub type Concom2 = crate::EnumBitfieldStruct<u8, Concom2_SPEC>;
    impl Concom2 {
        #[doc = "0 Address trigger condition  see CONCOM1  and signal status trigger conditions are combined with a logical OR for further control"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Address phase trigger condition  see CONCOM1  and the signal status trigger conditions are combined with a logical AND for further control.  see CROSSREFERENCE"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ong_SPEC;
    pub type Ong = crate::EnumBitfieldStruct<u8, Ong_SPEC>;
    impl Ong {
        #[doc = "No grant debug event trigger is generated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "The grant debug event trigger is enabled and generated according the settings of register DBGRNT.  see CROSSREFERENCE"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ona1_SPEC;
    pub type Ona1 = crate::EnumBitfieldStruct<u8, Ona1_SPEC>;
    impl Ona1 {
        #[doc = "No address 1 trigger is generated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "An address 1 trigger event is generated if the FPI Bus address is equal to DBADR1"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "An address 1 trigger event is generated if FPI Bus address is greater or equal to DBADR1"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 same as 00 B"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Ona2_SPEC;
    pub type Ona2 = crate::EnumBitfieldStruct<u8, Ona2_SPEC>;
    impl Ona2 {
        #[doc = "No address 2 trigger is generated."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "An address 2 trigger event is generated if the FPI Bus address is equal to DBADR2"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "An address 2 trigger event is generated if FPI Bus address is less or equal to DBADR2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 same as 00 B"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Onbos0_SPEC;
    pub type Onbos0 = crate::EnumBitfieldStruct<u8, Onbos0_SPEC>;
    impl Onbos0 {
        #[doc = "0 A signal status        trigger is generated for all FPI  160 Bus op codes except a   8220 no operation  8221         op code"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "A signal status trigger is generated if the FPI Bus op code matches the op code as defined in DBBOS.OPC.  see CROSSREFERENCE"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Onbos1_SPEC;
    pub type Onbos1 = crate::EnumBitfieldStruct<u8, Onbos1_SPEC>;
    impl Onbos1 {
        #[doc = "The signal status trigger generation for the FPI Bus Supervisor Mode signal is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "A signal status trigger is generated if the FPI Bus Supervisor Mode signal state is equal to the value of DBBOS.SVM.  see CROSSREFERENCE"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Onbos2_SPEC;
    pub type Onbos2 = crate::EnumBitfieldStruct<u8, Onbos2_SPEC>;
    impl Onbos2 {
        #[doc = "The signal status trigger generation for the FPI Bus write signal is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "A signal status trigger is generated if the FPI Bus write signal state is equal to the value of DBBOS.WR.  see CROSSREFERENCE"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Onbos3_SPEC;
    pub type Onbos3 = crate::EnumBitfieldStruct<u8, Onbos3_SPEC>;
    impl Onbos3 {
        #[doc = "The signal status trigger generation for the FPI Bus read signal is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "A signal status trigger is generated if the FPI Bus read signal state is equal to the value of DBBOS.RD.  see CROSSREFERENCE"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbdat_SPEC;
impl crate::sealed::RegSpec for Dbdat_SPEC {
    type DataType = u32;
}
#[doc = "BCU Debug Data Status Register\n resetvalue={Debug Reset:0x0}"]
pub type Dbdat = crate::RegValueT<Dbdat_SPEC>;

impl Dbdat {
    #[doc = "FPI Bus Data Status. This register contains the FPI  160 Bus data that was captured when the OCDS        break trigger event occurred."]
    #[inline(always)]
    pub fn fpidata(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Dbdat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Dbdat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Dbdat {
    #[inline(always)]
    fn default() -> Dbdat {
        <crate::RegValueT<Dbdat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgntt_SPEC;
impl crate::sealed::RegSpec for Dbgntt_SPEC {
    type DataType = u32;
}
#[doc = "SBCU Debug Trapped Master Register\n resetvalue={Debug Reset:0x0FFFF}"]
pub type Dbgntt = crate::RegValueT<Dbgntt_SPEC>;

impl Dbgntt {
    #[doc = "DMA   Cerberus FPI Bus Master Status. This bit indicates whether the DMA or Cerberus was FPI  160 Bus master when        the break trigger event occurred."]
    #[inline(always)]
    pub fn dma(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, dbgntt::Dma, Dbgntt_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,dbgntt::Dma, Dbgntt_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "HSSL 0 FPI Bus Master Status. This bit indicates whether the HSSL 0 was FPI  160 Bus master when the break        trigger event occurred."]
    #[inline(always)]
    pub fn hssl0(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, dbgntt::Hssl0, Dbgntt_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x1,1,0,dbgntt::Hssl0, Dbgntt_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CPU0 FPI Bus Master Status. This bit indicates whether the CPU0 was FPI  160 Bus master when the break        trigger event occurred. Position   180 6  180  of the Req Lock Grant BCU input vector set is used in the          EDC implementation for Req Lock error injection. Therefore this          position shall be used on the SPB for CPU  on BBB for the Bridge           SRI2BBB ."]
    #[inline(always)]
    pub fn cpu0(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, dbgntt::Cpu0, Dbgntt_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x1,1,0,dbgntt::Cpu0, Dbgntt_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CPU1 FPI Bus Master Status. This bit indicates whether the CPU1 was FPI  160 Bus master when the break        trigger event occurred."]
    #[inline(always)]
    pub fn cpu1(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, dbgntt::Cpu1, Dbgntt_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<7,0x1,1,0,dbgntt::Cpu1, Dbgntt_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CPU2 Grant Trigger Enable"]
    #[inline(always)]
    pub fn cpu2(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, dbgntt::Cpu2, Dbgntt_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x1,1,0,dbgntt::Cpu2, Dbgntt_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "HSM Register FPI Bus Master Interface Status. This bit indicates whether the HSM was FPI  160 Bus master when the break        trigger event occurred."]
    #[inline(always)]
    pub fn hsmrmi(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, dbgntt::Hsmrmi, Dbgntt_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<12,0x1,1,0,dbgntt::Hsmrmi, Dbgntt_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "HSM Cache FPI Bus Master Interface Status. This bit indicates whether the HSM was FPI  160 Bus master when the break        trigger event occurred."]
    #[inline(always)]
    pub fn hsmcmi(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, dbgntt::Hsmcmi, Dbgntt_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<13,0x1,1,0,dbgntt::Hsmcmi, Dbgntt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Dbgntt {
    #[inline(always)]
    fn default() -> Dbgntt {
        <crate::RegValueT<Dbgntt_SPEC> as RegisterValue<_>>::new(65535)
    }
}
pub mod dbgntt {
    pub struct Dma_SPEC;
    pub type Dma = crate::EnumBitfieldStruct<u8, Dma_SPEC>;
    impl Dma {
        #[doc = "The DMA or Cerberus was the FPI bus master."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Neither DMA nor Cerberus was the FPI Bus master."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hssl0_SPEC;
    pub type Hssl0 = crate::EnumBitfieldStruct<u8, Hssl0_SPEC>;
    impl Hssl0 {
        #[doc = "0 The HSSL0 was        the FPI  160 bus master."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The HSSL0 was        not the FPI  160 Bus master."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0_SPEC;
    pub type Cpu0 = crate::EnumBitfieldStruct<u8, Cpu0_SPEC>;
    impl Cpu0 {
        #[doc = "0 The CPU0 was        the FPI  160 Bus master."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The CPU0 was        not the FPI  160 Bus master."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1_SPEC;
    pub type Cpu1 = crate::EnumBitfieldStruct<u8, Cpu1_SPEC>;
    impl Cpu1 {
        #[doc = "0 The CPU1 was        the FPI  160 Bus master."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The CPU1 was        not the FPI  160 Bus master."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2_SPEC;
    pub type Cpu2 = crate::EnumBitfieldStruct<u8, Cpu2_SPEC>;
    impl Cpu2 {
        #[doc = "0 FPI Bus transactions with CPU2 as bus master are enabled for grant trigger event generation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FPI Bus transactions with CPU2 as bus master are disabled for grant trigger event generation"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hsmrmi_SPEC;
    pub type Hsmrmi = crate::EnumBitfieldStruct<u8, Hsmrmi_SPEC>;
    impl Hsmrmi {
        #[doc = "0 HSMRMI was the FPI bus master."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 HSMRMI was not the FPI Bus master."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hsmcmi_SPEC;
    pub type Hsmcmi = crate::EnumBitfieldStruct<u8, Hsmcmi_SPEC>;
    impl Hsmcmi {
        #[doc = "0 HSMCMI was the FPI bus master."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 HSMCMI was not the FPI Bus master."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgrnt_SPEC;
impl crate::sealed::RegSpec for Dbgrnt_SPEC {
    type DataType = u32;
}
#[doc = "SBCU Debug Grant Mask Register\n resetvalue={Debug Reset:0x0FFFF}"]
pub type Dbgrnt = crate::RegValueT<Dbgrnt_SPEC>;

impl Dbgrnt {
    #[doc = "DMA   Cerberus Trigger Enable. TC39xA  DMAH"]
    #[inline(always)]
    pub fn dma(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, dbgrnt::Dma, Dbgrnt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,dbgrnt::Dma, Dbgrnt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSSL0 Trigger Enable"]
    #[inline(always)]
    pub fn hssl0(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, dbgrnt::Hssl0, Dbgrnt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,dbgrnt::Hssl0, Dbgrnt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU0 Grant Trigger Enable"]
    #[inline(always)]
    pub fn cpu0(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, dbgrnt::Cpu0, Dbgrnt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,dbgrnt::Cpu0, Dbgrnt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU1 Grant Trigger Enable"]
    #[inline(always)]
    pub fn cpu1(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, dbgrnt::Cpu1, Dbgrnt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,dbgrnt::Cpu1, Dbgrnt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU2 Grant Trigger Enable"]
    #[inline(always)]
    pub fn cpu2(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, dbgrnt::Cpu2, Dbgrnt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,dbgrnt::Cpu2, Dbgrnt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Register Master Interface Grant Trigger Enable"]
    #[inline(always)]
    pub fn hsmrmi(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, dbgrnt::Hsmrmi, Dbgrnt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,dbgrnt::Hsmrmi, Dbgrnt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSM Cache Master Interface Grant Trigger Enable"]
    #[inline(always)]
    pub fn hsmcmi(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, dbgrnt::Hsmcmi, Dbgrnt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,dbgrnt::Hsmcmi, Dbgrnt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dbgrnt {
    #[inline(always)]
    fn default() -> Dbgrnt {
        <crate::RegValueT<Dbgrnt_SPEC> as RegisterValue<_>>::new(65535)
    }
}
pub mod dbgrnt {
    pub struct Dma_SPEC;
    pub type Dma = crate::EnumBitfieldStruct<u8, Dma_SPEC>;
    impl Dma {
        #[doc = "FPI Bus transactions with DMA   Cerberus as bus master are enabled for grant trigger event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "FPI Bus transactions with DMA   Cerberus as bus master are disabled for grant trigger event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hssl0_SPEC;
    pub type Hssl0 = crate::EnumBitfieldStruct<u8, Hssl0_SPEC>;
    impl Hssl0 {
        #[doc = "FPI Bus transactions with HSSL0 as bus master are enabled for grant trigger event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "FPI Bus transactions with HSSL0 as bus master are disabled for grant trigger event generation.. 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu0_SPEC;
    pub type Cpu0 = crate::EnumBitfieldStruct<u8, Cpu0_SPEC>;
    impl Cpu0 {
        #[doc = "FPI Bus transactions with CPU0 as bus master are enabled for grant trigger event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "FPI Bus transactions with CPU as bus master are disabled for grant trigger event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu1_SPEC;
    pub type Cpu1 = crate::EnumBitfieldStruct<u8, Cpu1_SPEC>;
    impl Cpu1 {
        #[doc = "FPI Bus transactions with CPU1 as bus master are enabled for grant trigger event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "FPI Bus transactions with CPU1 as bus master are disabled for grant trigger event generation.. 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpu2_SPEC;
    pub type Cpu2 = crate::EnumBitfieldStruct<u8, Cpu2_SPEC>;
    impl Cpu2 {
        #[doc = "FPI Bus transactions with CPU2 as bus master are enabled for grant trigger event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "FPI Bus transactions with CPU2 as bus master are disabled for grant trigger event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hsmrmi_SPEC;
    pub type Hsmrmi = crate::EnumBitfieldStruct<u8, Hsmrmi_SPEC>;
    impl Hsmrmi {
        #[doc = "FPI Bus transactions requested by the HSM bus master are enabled for grant trigger event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "FPI Bus transactions requested by the HSM bus master are disabled for grant trigger event generation.. 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hsmcmi_SPEC;
    pub type Hsmcmi = crate::EnumBitfieldStruct<u8, Hsmcmi_SPEC>;
    impl Hsmcmi {
        #[doc = "FPI Bus transactions requested by the HSM bus master are enabled for grant trigger event generation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "FPI Bus transactions requested by the HSM bus master are disabled for grant trigger event generation."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eadd_SPEC;
impl crate::sealed::RegSpec for Eadd_SPEC {
    type DataType = u32;
}
#[doc = "BCU Error Address Capture Register\n resetvalue={Application Reset:0x0}"]
pub type Eadd = crate::RegValueT<Eadd_SPEC>;

impl Eadd {
    #[doc = "Captured FPI Bus Address   FPIADR. This bit field holds the 32 bit FPI Bus address that has been captured at an FPI Bus error. Note that if multiple bus errors occurred  only the address of the first bus error is captured."]
    #[inline(always)]
    pub fn fpiadr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Eadd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Eadd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Eadd {
    #[inline(always)]
    fn default() -> Eadd {
        <crate::RegValueT<Eadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Econ_SPEC;
impl crate::sealed::RegSpec for Econ_SPEC {
    type DataType = u32;
}
#[doc = "BCU Error Control Capture Register\n resetvalue={Application Reset:0x0}"]
pub type Econ = crate::RegValueT<Econ_SPEC>;

impl Econ {
    #[doc = "FPI Bus Error Counter. ERRCNT is incremented on every occurrence of an FPI  160 Bus error. ERRCNT is        reset to 0 after the ECON register is read. Aborted        accesses to a 0 wait state SPB slave may also increment ERRCNT when the        slave generates an error acknowledge."]
    #[inline(always)]
    pub fn errcnt(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Econ_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Econ_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "State of FPI Bus Time Out Signal. This bit indicates the state of the time out signal at an FBI  160 Bus error."]
    #[inline(always)]
    pub fn tout(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, econ::Tout, Econ_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x1,1,0,econ::Tout, Econ_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "State of FPI Bus Ready Signal. This bit indicates the state of the ready signal at an FBI  160 Bus error."]
    #[inline(always)]
    pub fn rdy(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, econ::Rdy, Econ_SPEC, crate::common::RW> {
        crate::common::RegisterField::<15,0x1,1,0,econ::Rdy, Econ_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "State of FPI Bus Abort Signal. This bit indicates the state of the abort signal at an FBI  160 Bus error."]
    #[inline(always)]
    pub fn abt(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, econ::Abt, Econ_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1,1,0,econ::Abt, Econ_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "State of FPI Bus Acknowledge Signals. This bit field indicates the acknowledge code that has been output by        the selected slave at an FPI  160 Bus error. Coding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ack(
        self,
    ) -> crate::common::RegisterField<17, 0x3, 1, 0, econ::Ack, Econ_SPEC, crate::common::RW> {
        crate::common::RegisterField::<17,0x3,1,0,econ::Ack, Econ_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "State of FPI Bus Supervisor Mode Signal. This bit indicates whether the FPI  160 Bus error occurred in Supervisor Mode        or in User Mode."]
    #[inline(always)]
    pub fn svm(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, econ::Svm, Econ_SPEC, crate::common::RW> {
        crate::common::RegisterField::<19,0x1,1,0,econ::Svm, Econ_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "State of FPI Bus Write Signal. This bit indicates whether the FPI  160 Bus error occurred at a write cycle         see CROSSREFERENCE  ."]
    #[inline(always)]
    pub fn wrn(self) -> crate::common::RegisterFieldBool<20, 1, 0, Econ_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Econ_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "State of FPI Bus Read Signal. This bit indicates whether the FPI  160 Bus error occurred at a read cycle         see CROSSREFERENCE  ."]
    #[inline(always)]
    pub fn rdn(self) -> crate::common::RegisterFieldBool<21, 1, 0, Econ_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Econ_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "FPI Bus Master Tag Number Signals. This bit field indicates the FPI  160 Bus master TAG number  definitions see CROSSREFERENCE  ."]
    #[inline(always)]
    pub fn tag(
        self,
    ) -> crate::common::RegisterField<22, 0x3f, 1, 0, u8, Econ_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3f,1,0,u8, Econ_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "FPI Bus Operation Code Signals. The FPI  160 Bus operation codes are defined in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn opc(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Econ_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Econ_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Econ {
    #[inline(always)]
    fn default() -> Econ {
        <crate::RegValueT<Econ_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod econ {
    pub struct Tout_SPEC;
    pub type Tout = crate::EnumBitfieldStruct<u8, Tout_SPEC>;
    impl Tout {
        #[doc = "0 No time out occurred"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Time out has occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rdy_SPEC;
    pub type Rdy = crate::EnumBitfieldStruct<u8, Rdy_SPEC>;
    impl Rdy {
        #[doc = "0 Wait state s  have been inserted. Ready signal was active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Ready signal was inactive"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Abt_SPEC;
    pub type Abt = crate::EnumBitfieldStruct<u8, Abt_SPEC>;
    impl Abt {
        #[doc = "0 Master has aborted an FPI Bus transfer. Abort signal was active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Abort signal was inactive"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ack_SPEC;
    pub type Ack = crate::EnumBitfieldStruct<u8, Ack_SPEC>;
    impl Ack {
        #[doc = "00 NSC  No        special case"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 RTY  Retry"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 ERR  Bus Error"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Svm_SPEC;
    pub type Svm = crate::EnumBitfieldStruct<u8, Svm_SPEC>;
    impl Svm {
        #[doc = "0 Transfer was initiated in User Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transfer was initiated in Supervisor Mode"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edat_SPEC;
impl crate::sealed::RegSpec for Edat_SPEC {
    type DataType = u32;
}
#[doc = "BCU Error Data Capture Register\n resetvalue={Application Reset:0x0}"]
pub type Edat = crate::RegValueT<Edat_SPEC>;

impl Edat {
    #[doc = "Captured FPI Bus Data   FPIDAT. This bit field holds the 32 bit FPI Bus data that has been captured at an FPI Bus error. Note that if multiple bus errors occurred  only the data of the first bus error is captured. No data are captured from transactions generated by the SHE module."]
    #[inline(always)]
    pub fn fpidat(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Edat_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Edat_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Edat {
    #[inline(always)]
    fn default() -> Edat {
        <crate::RegValueT<Edat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x6A00}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MOD REV. This bit field defines the module revision number. The value of a module revision starts with 01H  first revision ."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MOD NUMBER. This bit field defines a module identification number. The value for the LBCU module is 006AH."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(27136)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prioh_SPEC;
impl crate::sealed::RegSpec for Prioh_SPEC {
    type DataType = u32;
}
#[doc = "Arbiter Priority Register High\n resetvalue={Application Reset:0x0FEDCBA98,Application Reset:0x0FEDC8888,Application Reset:0x0FEDCBA98}"]
pub type Prioh = crate::RegValueT<Prioh_SPEC>;

impl Prioh {
    #[doc = "CPU2 Priority  Index 8 . This bit field defines the priority on the SPB for CPU2 access to the        SPB."]
    #[inline(always)]
    pub fn cpu2(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Prioh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Prioh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSMRMI Priority  Index 12 . This bit field defines the priority on the SPB for HSMRMI access to the        SPB."]
    #[inline(always)]
    pub fn hsmrmi(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Prioh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Prioh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSMCMI Priority  Index 13 . This bit field defines the priority on the SPB for HSMCMI access to the        SPB."]
    #[inline(always)]
    pub fn hsmcmi(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Prioh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8, Prioh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Prioh {
    #[inline(always)]
    fn default() -> Prioh {
        <crate::RegValueT<Prioh_SPEC> as RegisterValue<_>>::new(4275865736)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Priol_SPEC;
impl crate::sealed::RegSpec for Priol_SPEC {
    type DataType = u32;
}
#[doc = "Arbiter Priority Register Low\n resetvalue={Application Reset:0x76543210,Application Reset:0x088543210,Application Reset:0x76588210}"]
pub type Priol = crate::RegValueT<Priol_SPEC>;

impl Priol {
    #[doc = "DMA   Cerberus Priority  Index 0 . This bit field defines the priority on the SPB for DMA and Cerberus        access to the SPB."]
    #[inline(always)]
    pub fn dma(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Priol_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Priol_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSSL0 Priority  Index 3 . This bit field defines the priority on the SPB for HSSL0 access to the        SPB."]
    #[inline(always)]
    pub fn hssl0(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Priol_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Priol_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU0 Priority  Index 6 . This bit field defines the priority on the SPB for CPU0 access to the        SPB. Position   180 6  180  of the Req Lock Grant BCU input vector set is used in the          EDC implementation for Req Lock error injection. Therefore this          position shall be used for a CPU."]
    #[inline(always)]
    pub fn cpu0(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Priol_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Priol_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPU1 Priority  Index 7 . This bit field defines the priority on the SPB for CPU1 access to the        SPB. This bit field contains the master priority for master connected to BCU        request input 7."]
    #[inline(always)]
    pub fn cpu1(
        self,
    ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Priol_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0xf,1,0,u8, Priol_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Priol {
    #[inline(always)]
    fn default() -> Priol {
        <crate::RegValueT<Priol_SPEC> as RegisterValue<_>>::new(5243408)
    }
}
