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
#[doc = r"STM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stm1(pub(super) *mut u8);
unsafe impl core::marker::Send for Stm1 {}
unsafe impl core::marker::Sync for Stm1 {}
impl Stm1 {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(252usize)) }
    }

    #[doc = "Timer Capture Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cap(&self) -> crate::common::Reg<self::Cap_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }

    #[doc = "Timer Capture Register Second View\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn capsv(&self) -> crate::common::Reg<self::Capsv_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(84usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Compare Match Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cmcon(&self) -> crate::common::Reg<self::Cmcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }

    #[doc = "Compare Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cmpx(&self) -> [crate::common::Reg<self::CmPx_SPEC, crate::common::RW>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x4usize)),
            ]
        }
    }

    #[doc = "Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn icr(&self) -> crate::common::Reg<self::Icr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0C007}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Interrupt Set Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn iscr(&self) -> crate::common::Reg<self::Iscr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
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

    #[doc = "OCDS Control and Status Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(232usize)) }
    }

    #[doc = "Timer Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tim0(&self) -> crate::common::Reg<self::Tim0_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "Timer Register 0 Second View\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tim0sv(&self) -> crate::common::Reg<self::Tim0Sv_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }

    #[doc = "Timer Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tim1(&self) -> crate::common::Reg<self::Tim1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Timer Register 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tim2(&self) -> crate::common::Reg<self::Tim2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }

    #[doc = "Timer Register 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tim3(&self) -> crate::common::Reg<self::Tim3_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "Timer Register 4\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tim4(&self) -> crate::common::Reg<self::Tim4_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "Timer Register 5\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tim5(&self) -> crate::common::Reg<self::Tim5_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "Timer Register 6\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tim6(&self) -> crate::common::Reg<self::Tim6_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
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
pub struct Cap_SPEC;
impl crate::sealed::RegSpec for Cap_SPEC {
    type DataType = u32;
}
#[doc = "Timer Capture Register\n resetvalue={Application Reset:0x0}"]
pub type Cap = crate::RegValueT<Cap_SPEC>;

impl Cap {
    #[doc = "Captured System Timer Bits  63 32    STMCAP 63 32 . The capture register STMCAP always captures the STM bits               63 32  when one of the registers TIM0 to TIM6 or TIM0SV is read. This capture operation              is performed in order to enable software to operate with a coherent value of all the 64              STM bits at one time stamp.This bit field contains bits  63 32  of the 64 bit STM. Reading register TIM0SV captures also the read value for                  register TIM6. In this way reading TIM0SV followed by CAPSV delivers the timer                  values for the first read request."]
    #[inline(always)]
    pub fn stmcap_63_32(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Cap_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Cap_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Cap {
    #[inline(always)]
    fn default() -> Cap {
        <crate::RegValueT<Cap_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capsv_SPEC;
impl crate::sealed::RegSpec for Capsv_SPEC {
    type DataType = u32;
}
#[doc = "Timer Capture Register Second View\n resetvalue={Application Reset:0x0}"]
pub type Capsv = crate::RegValueT<Capsv_SPEC>;

impl Capsv {
    #[doc = "Captured System Timer Bits  63 32    STMCAP 63 32 . The capture register STMCAP always captures the STM bits               63 32  when one of the registers TIM0 to TIM6 or TIM0SV is read. This capture operation              is performed in order to enable software to operate with a coherent value of all the 64              STM bits at one time stamp.This bit field contains bits  63 32  of the 64 bit STM. Reading register TIM0SV captures also the read value for                  register TIM6. In this way reading TIM0SV followed by CAPSV delivers the timer                  values for the first read request."]
    #[inline(always)]
    pub fn stmcap_63_32(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Capsv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Capsv_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Capsv {
    #[inline(always)]
    fn default() -> Capsv {
        <crate::RegValueT<Capsv_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control of the STM module. f STM is generated by the CCU."]
    #[inline(always)]
    pub fn disr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, clc::Disr, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,clc::Disr, Clc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Module Disable Status Bit   DISS. Bit indicates the current status of the STM module."]
    #[inline(always)]
    pub fn diss(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, clc::Diss, Clc_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,clc::Diss, Clc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Sleep Mode Enable Control   EDIS. Used for module sleep mode control."]
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
        #[doc = "0 No disable requested"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Disable requested"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Diss_SPEC;
    pub type Diss = crate::EnumBitfieldStruct<u8, Diss_SPEC>;
    impl Diss {
        #[doc = "0 STM module is enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 STM module is disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmcon_SPEC;
impl crate::sealed::RegSpec for Cmcon_SPEC {
    type DataType = u32;
}
#[doc = "Compare Match Control Register\n resetvalue={Application Reset:0x0}"]
pub type Cmcon = crate::RegValueT<Cmcon_SPEC>;

impl Cmcon {
    #[doc = "Compare Register Size for CMP0   MSIZE0. This bit field determines the number of bits in register CMP0  starting from bit 0  that are used for the compare operation with the System Timer. ..."]
    #[inline(always)]
    pub fn msize0(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, cmcon::Msize0, Cmcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1f,1,0,cmcon::Msize0, Cmcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Start Bit Location for CMP0   MSTART0. This bit field determines the lowest bit number of the 64 bit STM that is compared with the content of register CMP0 bit 0. The number of bits to be compared is defined by bit field MSIZE0. ..."]
    #[inline(always)]
    pub fn mstart0(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, cmcon::Mstart0, Cmcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1f,1,0,cmcon::Mstart0, Cmcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Compare Register Size for CMP1   MSIZE1. This bit field determines the number of bits in register CMP1  starting from bit 0  that are used for the compare operation with the System Timer. ..."]
    #[inline(always)]
    pub fn msize1(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, cmcon::Msize1, Cmcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1f,1,0,cmcon::Msize1, Cmcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Start Bit Location for CMP1   MSTART1. This bit field determines the lowest bit number of the 64 bit STM that is compared with the content of register CMP1 bit 0. The number of bits to be compared is defined by bit field MSIZE1. ..."]
    #[inline(always)]
    pub fn mstart1(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, cmcon::Mstart1, Cmcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1f,1,0,cmcon::Mstart1, Cmcon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Cmcon {
    #[inline(always)]
    fn default() -> Cmcon {
        <crate::RegValueT<Cmcon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cmcon {
    pub struct Msize0_SPEC;
    pub type Msize0 = crate::EnumBitfieldStruct<u8, Msize0_SPEC>;
    impl Msize0 {
        #[doc = "00000 CMP0 0  used for compare operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "00001 CMP0 1 0  used for compare operation"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "11110 CMP0 30 0  used for compare operation"]
        pub const CONST_3030: Self = Self::new(30);
        #[doc = "11111 CMP0 31 0  used for compare operation"]
        pub const CONST_3131: Self = Self::new(31);
    }
    pub struct Mstart0_SPEC;
    pub type Mstart0 = crate::EnumBitfieldStruct<u8, Mstart0_SPEC>;
    impl Mstart0 {
        #[doc = "00000 STM 0  is the lowest bit number"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "00001 STM 1  is the lowest bit number"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "11110 STM 30  is the lowest bit number"]
        pub const CONST_3030: Self = Self::new(30);
        #[doc = "11111 STM 31  is the lowest bit number"]
        pub const CONST_3131: Self = Self::new(31);
    }
    pub struct Msize1_SPEC;
    pub type Msize1 = crate::EnumBitfieldStruct<u8, Msize1_SPEC>;
    impl Msize1 {
        #[doc = "00000 CMP1 0  used for compare operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "00001 CMP1 1 0  used for compare operation"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "11110 CMP1 30 0  used for compare operation"]
        pub const CONST_3030: Self = Self::new(30);
        #[doc = "11111 CMP1 31 0  used for compare operation"]
        pub const CONST_3131: Self = Self::new(31);
    }
    pub struct Mstart1_SPEC;
    pub type Mstart1 = crate::EnumBitfieldStruct<u8, Mstart1_SPEC>;
    impl Mstart1 {
        #[doc = "00000 STM 0  is the lowest bit number"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "00001 STM 1  is the lowest bit number"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "11110 STM 30  is the lowest bit number"]
        pub const CONST_3030: Self = Self::new(30);
        #[doc = "11111 STM 31  is the lowest bit number"]
        pub const CONST_3131: Self = Self::new(31);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CmPx_SPEC;
impl crate::sealed::RegSpec for CmPx_SPEC {
    type DataType = u32;
}
#[doc = "Compare Register 0\n resetvalue={Application Reset:0x0}"]
pub type CmPx = crate::RegValueT<CmPx_SPEC>;

impl CmPx {
    #[doc = "Compare Value of Compare Register x   CMPVAL. This bit field holds up to 32 bits of the compare value  right adjusted ."]
    #[inline(always)]
    pub fn cmpval(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, CmPx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, CmPx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for CmPx {
    #[inline(always)]
    fn default() -> CmPx {
        <crate::RegValueT<CmPx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr_SPEC;
impl crate::sealed::RegSpec for Icr_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
pub type Icr = crate::RegValueT<Icr_SPEC>;

impl Icr {
    #[doc = "Compare Register CMP0 Interrupt Enable Control   CMP0EN. This bit enables the compare match interrupt with compare register CMP0."]
    #[inline(always)]
    pub fn cmp0en(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, icr::Cmp0En, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,icr::Cmp0En, Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Compare Register CMP0 Interrupt Request Flag   CMP0IR. This bit indicates whether or not a compare match event of compare register CMP0 has occured. CMP0IR must be cleared by software. CMP0IR must be cleared by software and can be set by software  too  see ISCR register . After a STM reset operation  CMP0IR is immediately set as a result of a compare match event with the reset values of the STM and the compare registers CMP0."]
    #[inline(always)]
    pub fn cmp0ir(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, icr::Cmp0Ir, Icr_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,icr::Cmp0Ir, Icr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Compare Register CMP0 Interrupt Output Selection   CMP0OS. This bit determines the interrupt output that is activated on a compare match event of compare register CMP0."]
    #[inline(always)]
    pub fn cmp0os(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, icr::Cmp0Os, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,icr::Cmp0Os, Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Compare Register CMP1 Interrupt Enable Control   CMP1EN. This bit enables the compare match interrupt with compare register CMP1."]
    #[inline(always)]
    pub fn cmp1en(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, icr::Cmp1En, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,icr::Cmp1En, Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Compare Register CMP1 Interrupt Request Flag   CMP1IR. This bit indicates whether or not a compare match event of compare register CMP1 has occured. CMP1IR must be cleared by software. CMP1IR must be cleared by software and can be set by software  too  see ISCR register . After a STM reset  CMP1IR is immediately set as a result of a compare match event with the reset values of the STM and the compare register CMP1."]
    #[inline(always)]
    pub fn cmp1ir(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, icr::Cmp1Ir, Icr_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x1,1,0,icr::Cmp1Ir, Icr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Compare Register CMP1 Interrupt Output Selection   CMP1OS. This bit determines the interrupt output that is activated on a compare match event of compare register CMP1."]
    #[inline(always)]
    pub fn cmp1os(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, icr::Cmp1Os, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,icr::Cmp1Os, Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        <crate::RegValueT<Icr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icr {
    pub struct Cmp0En_SPEC;
    pub type Cmp0En = crate::EnumBitfieldStruct<u8, Cmp0En_SPEC>;
    impl Cmp0En {
        #[doc = "0 Interrupt on compare match with CMP0 disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt on compare match with CMP0 enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cmp0Ir_SPEC;
    pub type Cmp0Ir = crate::EnumBitfieldStruct<u8, Cmp0Ir_SPEC>;
    impl Cmp0Ir {
        #[doc = "0 A compare match interrupt has not been detected since the bit has been cleared for the last time."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A compare match interrupt has been detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cmp0Os_SPEC;
    pub type Cmp0Os = crate::EnumBitfieldStruct<u8, Cmp0Os_SPEC>;
    impl Cmp0Os {
        #[doc = "0 Interrupt output STMIR0 selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt output STMIR1 selected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cmp1En_SPEC;
    pub type Cmp1En = crate::EnumBitfieldStruct<u8, Cmp1En_SPEC>;
    impl Cmp1En {
        #[doc = "0 Interrupt on compare match with CMP1 disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt on compare match with CMP1 enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cmp1Ir_SPEC;
    pub type Cmp1Ir = crate::EnumBitfieldStruct<u8, Cmp1Ir_SPEC>;
    impl Cmp1Ir {
        #[doc = "0 A compare match interrupt has not been detected since the bit has been cleared for the last time."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A compare match interrupt has been detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cmp1Os_SPEC;
    pub type Cmp1Os = crate::EnumBitfieldStruct<u8, Cmp1Os_SPEC>;
    impl Cmp1Os {
        #[doc = "0 Interrupt output STMIR0 selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt output STMIR1 selected"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0C007}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MODREV. MODREV defines the module revision number. The value of a              module revision starts with 01 H  first revision .              Current revision is 0x7."]
    #[inline(always)]
    pub fn modrev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MODTYPE. This bit field defines the module as a 32 bit module  C0 H"]
    #[inline(always)]
    pub fn modtype(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MODNUM. This bit field defines the module identification number for the              STM  0068 H"]
    #[inline(always)]
    pub fn modnum(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Id_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Id {
    #[inline(always)]
    fn default() -> Id {
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(49159)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iscr_SPEC;
impl crate::sealed::RegSpec for Iscr_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Set Clear Register\n resetvalue={Application Reset:0x0}"]
pub type Iscr = crate::RegValueT<Iscr_SPEC>;

impl Iscr {
    #[doc = "Reset Compare Register CMP0 Interrupt Flag   CMP0IRR"]
    #[inline(always)]
    pub fn cmp0irr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, iscr::Cmp0Irr, Iscr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x1,1,0,iscr::Cmp0Irr, Iscr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Compare Register CMP0 Interrupt Flag   CMP0IRS"]
    #[inline(always)]
    pub fn cmp0irs(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, iscr::Cmp0Irs, Iscr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<1,0x1,1,0,iscr::Cmp0Irs, Iscr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Reset Compare Register CMP1 Interrupt Flag   CMP1IRR"]
    #[inline(always)]
    pub fn cmp1irr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, iscr::Cmp1Irr, Iscr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<2,0x1,1,0,iscr::Cmp1Irr, Iscr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Compare Register CMP1 Interrupt Flag   CMP1IRS"]
    #[inline(always)]
    pub fn cmp1irs(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, iscr::Cmp1Irs, Iscr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<3,0x1,1,0,iscr::Cmp1Irs, Iscr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Iscr {
    #[inline(always)]
    fn default() -> Iscr {
        <crate::RegValueT<Iscr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iscr {
    pub struct Cmp0Irr_SPEC;
    pub type Cmp0Irr = crate::EnumBitfieldStruct<u8, Cmp0Irr_SPEC>;
    impl Cmp0Irr {
        #[doc = "0 Bit ICR.CMP0IR is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit ICR.CMP0IR is cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cmp0Irs_SPEC;
    pub type Cmp0Irs = crate::EnumBitfieldStruct<u8, Cmp0Irs_SPEC>;
    impl Cmp0Irs {
        #[doc = "0 Bit ICR.CMP0IR is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit ICR.CMP0IR is set. The state of bit CMP0IRR is  don t care  in this case."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cmp1Irr_SPEC;
    pub type Cmp1Irr = crate::EnumBitfieldStruct<u8, Cmp1Irr_SPEC>;
    impl Cmp1Irr {
        #[doc = "0 Bit ICR.CMP1IR is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit ICR.CMP1IR is cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cmp1Irs_SPEC;
    pub type Cmp1Irs = crate::EnumBitfieldStruct<u8, Cmp1Irs_SPEC>;
    impl Cmp1Irs {
        #[doc = "0 Bit ICR.CMP1IR is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit ICR.CMP1IR is set. The state of bit CMP1IRR is  don t care  in this case."]
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel registers are set. The RST bit will be cleared  re set to  0   by the BPI FPI after the kernel reset was executed."]
    #[inline(always)]
    pub fn rst(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, krst0::Rst, Krst0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,krst0::Rst, Krst0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Kernel Reset Status   RSTSTAT. This bit indicates wether a kernel reset was executed or not. This bit is set by the BPI FPI after the execution of a kernel reset in the same clock cycle both reset bits. This bit can be cleared by writing with  1  to the CLR bit in the related KRSTCLR register."]
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel reset registers is set. The RST bit will be cleared  re set to  0   after the kernel reset was executed."]
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
pub struct Ocs_SPEC;
impl crate::sealed::RegSpec for Ocs_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Control and Status Register\n resetvalue={Debug Reset:0x0}"]
pub type Ocs = crate::RegValueT<Ocs_SPEC>;

impl Ocs {
    #[doc = "OCDS Suspend Control   SUS. Controls the sensitivity to the suspend signal coming from the OCDS Trigger Switch  OTGS  For details see the OCDS chapter."]
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
        #[doc = "2 64 bit counter will be stopped"]
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
pub struct Tim0_SPEC;
impl crate::sealed::RegSpec for Tim0_SPEC {
    type DataType = u32;
}
#[doc = "Timer Register 0\n resetvalue={Application Reset:0x0}"]
pub type Tim0 = crate::RegValueT<Tim0_SPEC>;

impl Tim0 {
    #[doc = "System Timer Bits  31 0    STM 31 0 . This bit field contains bits  31 0  of the 64 bit STM."]
    #[inline(always)]
    pub fn stm_31_0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Tim0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Tim0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Tim0 {
    #[inline(always)]
    fn default() -> Tim0 {
        <crate::RegValueT<Tim0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tim0Sv_SPEC;
impl crate::sealed::RegSpec for Tim0Sv_SPEC {
    type DataType = u32;
}
#[doc = "Timer Register 0 Second View\n resetvalue={Application Reset:0x0}"]
pub type Tim0Sv = crate::RegValueT<Tim0Sv_SPEC>;

impl Tim0Sv {
    #[doc = "System Timer Bits  31 0    STM 31 0 . This bit field contains bits  31 0  of the 64 bit STM."]
    #[inline(always)]
    pub fn stm_31_0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Tim0Sv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Tim0Sv_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Tim0Sv {
    #[inline(always)]
    fn default() -> Tim0Sv {
        <crate::RegValueT<Tim0Sv_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tim1_SPEC;
impl crate::sealed::RegSpec for Tim1_SPEC {
    type DataType = u32;
}
#[doc = "Timer Register 1\n resetvalue={Application Reset:0x0}"]
pub type Tim1 = crate::RegValueT<Tim1_SPEC>;

impl Tim1 {
    #[doc = "System Timer Bits  35 4    STM 35 4 . This bit field contains bits  35 4  of the 64 bit STM."]
    #[inline(always)]
    pub fn stm_35_4(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Tim1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Tim1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Tim1 {
    #[inline(always)]
    fn default() -> Tim1 {
        <crate::RegValueT<Tim1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tim2_SPEC;
impl crate::sealed::RegSpec for Tim2_SPEC {
    type DataType = u32;
}
#[doc = "Timer Register 2\n resetvalue={Application Reset:0x0}"]
pub type Tim2 = crate::RegValueT<Tim2_SPEC>;

impl Tim2 {
    #[doc = "System Timer Bits  39 8    STM 39 8 . This bit field contains bits  39 8  of the 64 bit STM."]
    #[inline(always)]
    pub fn stm_39_8(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Tim2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Tim2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Tim2 {
    #[inline(always)]
    fn default() -> Tim2 {
        <crate::RegValueT<Tim2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tim3_SPEC;
impl crate::sealed::RegSpec for Tim3_SPEC {
    type DataType = u32;
}
#[doc = "Timer Register 3\n resetvalue={Application Reset:0x0}"]
pub type Tim3 = crate::RegValueT<Tim3_SPEC>;

impl Tim3 {
    #[doc = "System Timer Bits  43 12    STM 43 12 . This bit field contains bits  43 12  of the 64 bit STM."]
    #[inline(always)]
    pub fn stm_43_12(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Tim3_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Tim3_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Tim3 {
    #[inline(always)]
    fn default() -> Tim3 {
        <crate::RegValueT<Tim3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tim4_SPEC;
impl crate::sealed::RegSpec for Tim4_SPEC {
    type DataType = u32;
}
#[doc = "Timer Register 4\n resetvalue={Application Reset:0x0}"]
pub type Tim4 = crate::RegValueT<Tim4_SPEC>;

impl Tim4 {
    #[doc = "System Timer Bits  47 16    STM 47 16 . This bit field contains bits  47 16  of the 64 bit STM."]
    #[inline(always)]
    pub fn stm_47_16(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Tim4_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Tim4_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Tim4 {
    #[inline(always)]
    fn default() -> Tim4 {
        <crate::RegValueT<Tim4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tim5_SPEC;
impl crate::sealed::RegSpec for Tim5_SPEC {
    type DataType = u32;
}
#[doc = "Timer Register 5\n resetvalue={Application Reset:0x0}"]
pub type Tim5 = crate::RegValueT<Tim5_SPEC>;

impl Tim5 {
    #[doc = "System Timer Bits  51 20    STM 51 20 . This bit field contains bits  51 20  of the 64 bit STM."]
    #[inline(always)]
    pub fn stm_51_20(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Tim5_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Tim5_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Tim5 {
    #[inline(always)]
    fn default() -> Tim5 {
        <crate::RegValueT<Tim5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tim6_SPEC;
impl crate::sealed::RegSpec for Tim6_SPEC {
    type DataType = u32;
}
#[doc = "Timer Register 6\n resetvalue={Application Reset:0x0}"]
pub type Tim6 = crate::RegValueT<Tim6_SPEC>;

impl Tim6 {
    #[doc = "System Timer Bits  63 32    STM 63 32 . This bit field contains bits  63 32  of the 64 bit STM."]
    #[inline(always)]
    pub fn stm_63_32(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Tim6_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Tim6_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Tim6 {
    #[inline(always)]
    fn default() -> Tim6 {
        <crate::RegValueT<Tim6_SPEC> as RegisterValue<_>>::new(0)
    }
}
