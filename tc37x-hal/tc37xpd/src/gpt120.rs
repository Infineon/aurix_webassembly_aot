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
#[doc = r"GPT12"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt120(pub(super) *mut u8);
unsafe impl core::marker::Send for Gpt120 {}
unsafe impl core::marker::Sync for Gpt120 {}
impl Gpt120 {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(252usize)) }
    }

    #[doc = "Capture and Reload Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn caprel(&self) -> crate::common::Reg<self::Caprel_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Identification Register\n resetvalue={Application Reset:0x68C000,Application Reset:0x68C001}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
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

    #[doc = "OCDS Control and Status Register\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(232usize)) }
    }

    #[doc = "Port Input Select Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pisel(&self) -> crate::common::Reg<self::Pisel_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }

    #[doc = "Timer T2 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t2(&self) -> crate::common::Reg<self::T2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "Timer T2 Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t2con(&self) -> crate::common::Reg<self::T2Con_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "Timer T3 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t3(&self) -> crate::common::Reg<self::T3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }

    #[doc = "Timer T3 Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t3con(&self) -> crate::common::Reg<self::T3Con_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Timer T4 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t4(&self) -> crate::common::Reg<self::T4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "Timer T4 Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t4con(&self) -> crate::common::Reg<self::T4Con_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }

    #[doc = "Timer T5 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t5(&self) -> crate::common::Reg<self::T5_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "Timer T5 Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t5con(&self) -> crate::common::Reg<self::T5Con_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "Timer T6 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t6(&self) -> crate::common::Reg<self::T6_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }

    #[doc = "Timer T6 Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn t6con(&self) -> crate::common::Reg<self::T6Con_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
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
pub struct Caprel_SPEC;
impl crate::sealed::RegSpec for Caprel_SPEC {
    type DataType = u32;
}
#[doc = "Capture and Reload Register\n resetvalue={Application Reset:0x0}"]
pub type Caprel = crate::RegValueT<Caprel_SPEC>;

impl Caprel {
    #[doc = "Current reload value or Captured value   CAPREL"]
    #[inline(always)]
    pub fn caprel(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Caprel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Caprel_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Caprel {
    #[inline(always)]
    fn default() -> Caprel {
        <crate::RegValueT<Caprel_SPEC> as RegisterValue<_>>::new(0)
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
        #[doc = "0 Sleep Mode request is regarded. Module is enabled to go into Sleep Mode."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Sleep Mode request is disregarded  Sleep Mode cannot be entered upon a request."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Identification Register\n resetvalue={Application Reset:0x68C000,Application Reset:0x68C001}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MODREV. This bit field indicates the revision number of the module  01 H   160    160 first        revision ."]
    #[inline(always)]
    pub fn modrev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MODTYPE. This bit field is C0 H . It defines a        32 bit module"]
    #[inline(always)]
    pub fn modtype(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number   MODNUMBER. This bit field defines the module identification number. For the GPT12 module the module identification number is 68 H ."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(6864896)
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel        reset will be executed if the reset bits of both kernel registers are        set. The RST bit will be cleared  re set to   180 0  180   by the BPI after the kernel        reset was executed."]
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
        #[doc = "0 No kernel reset        was executed"]
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel reset registers is set. The RST bit will be cleared  re set to  0   by the BPI after the kernel reset was executed."]
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
#[doc = "OCDS Control and Status Register\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
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
        #[doc = "1 Hard suspend. Clock is switched off immediately."]
        pub const CONST_11: Self = Self::new(1);
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
pub struct Pisel_SPEC;
impl crate::sealed::RegSpec for Pisel_SPEC {
    type DataType = u32;
}
#[doc = "Port Input Select Register\n resetvalue={Application Reset:0x0}"]
pub type Pisel = crate::RegValueT<Pisel_SPEC>;

impl Pisel {
    #[doc = "Input Select for T2IN   IST2IN"]
    #[inline(always)]
    pub fn ist2in(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, pisel::Ist2In, Pisel_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,pisel::Ist2In, Pisel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for T2EUD   IST2EUD"]
    #[inline(always)]
    pub fn ist2eud(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, pisel::Ist2Eud, Pisel_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,pisel::Ist2Eud, Pisel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for T3IN   IST3IN"]
    #[inline(always)]
    pub fn ist3in(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, pisel::Ist3In, Pisel_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,pisel::Ist3In, Pisel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for T3EUD   IST3EUD"]
    #[inline(always)]
    pub fn ist3eud(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, pisel::Ist3Eud, Pisel_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,pisel::Ist3Eud, Pisel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for T4IN   IST4IN"]
    #[inline(always)]
    pub fn ist4in(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, pisel::Ist4In, Pisel_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,pisel::Ist4In, Pisel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for T4EUD   IST4EUD"]
    #[inline(always)]
    pub fn ist4eud(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, pisel::Ist4Eud, Pisel_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,pisel::Ist4Eud, Pisel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for T5IN   IST5IN"]
    #[inline(always)]
    pub fn ist5in(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, pisel::Ist5In, Pisel_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,pisel::Ist5In, Pisel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for T5EUD   IST5EUD"]
    #[inline(always)]
    pub fn ist5eud(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, pisel::Ist5Eud, Pisel_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,pisel::Ist5Eud, Pisel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for T6IN   IST6IN"]
    #[inline(always)]
    pub fn ist6in(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, pisel::Ist6In, Pisel_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,pisel::Ist6In, Pisel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for T6EUD   IST6EUD"]
    #[inline(always)]
    pub fn ist6eud(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, pisel::Ist6Eud, Pisel_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,pisel::Ist6Eud, Pisel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Select for CAPIN   ISCAPIN"]
    #[inline(always)]
    pub fn iscapin(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, pisel::Iscapin, Pisel_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,pisel::Iscapin, Pisel_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pisel {
    #[inline(always)]
    fn default() -> Pisel {
        <crate::RegValueT<Pisel_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pisel {
    pub struct Ist2In_SPEC;
    pub type Ist2In = crate::EnumBitfieldStruct<u8, Ist2In_SPEC>;
    impl Ist2In {
        #[doc = "0 Signal T2INA is selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal T2INB is selected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ist2Eud_SPEC;
    pub type Ist2Eud = crate::EnumBitfieldStruct<u8, Ist2Eud_SPEC>;
    impl Ist2Eud {
        #[doc = "0 Signal T2EUDA is selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal T2EUDB is selected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ist3In_SPEC;
    pub type Ist3In = crate::EnumBitfieldStruct<u8, Ist3In_SPEC>;
    impl Ist3In {
        #[doc = "00 Signal T3INA is selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Signal T3INB is selected"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Signal T3INC is selected"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Signal T3IND is selected"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Ist3Eud_SPEC;
    pub type Ist3Eud = crate::EnumBitfieldStruct<u8, Ist3Eud_SPEC>;
    impl Ist3Eud {
        #[doc = "00 Signal T3EUDA is selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Signal T3EUDB is selected"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Signal T3EUDC is selected"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Signal T3EUDD is selected"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Ist4In_SPEC;
    pub type Ist4In = crate::EnumBitfieldStruct<u8, Ist4In_SPEC>;
    impl Ist4In {
        #[doc = "00 Signal T4INA is selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Signal T4INB is selected"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Signal T4INC is selected"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Signal T4IND is selected"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Ist4Eud_SPEC;
    pub type Ist4Eud = crate::EnumBitfieldStruct<u8, Ist4Eud_SPEC>;
    impl Ist4Eud {
        #[doc = "00 Signal T4EUDA is selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Signal T4EUDB is selected"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Signal T4EUDC is selected"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Signal T4EUDD is selected"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Ist5In_SPEC;
    pub type Ist5In = crate::EnumBitfieldStruct<u8, Ist5In_SPEC>;
    impl Ist5In {
        #[doc = "0 Signal T5INA is selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal T5INB is selected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ist5Eud_SPEC;
    pub type Ist5Eud = crate::EnumBitfieldStruct<u8, Ist5Eud_SPEC>;
    impl Ist5Eud {
        #[doc = "0 Signal T5EUDA is selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal T5EUDB is selected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ist6In_SPEC;
    pub type Ist6In = crate::EnumBitfieldStruct<u8, Ist6In_SPEC>;
    impl Ist6In {
        #[doc = "0 Signal T6INA is selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal T6INB is selected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ist6Eud_SPEC;
    pub type Ist6Eud = crate::EnumBitfieldStruct<u8, Ist6Eud_SPEC>;
    impl Ist6Eud {
        #[doc = "0 Signal T6EUDA is selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Signal T6EUDB is selected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Iscapin_SPEC;
    pub type Iscapin = crate::EnumBitfieldStruct<u8, Iscapin_SPEC>;
    impl Iscapin {
        #[doc = "00 Signal CAPINA is selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Signal CAPINB is selected"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Signal CAPINC  Read trigger from T3  is selected"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Signal CAPIND  Read trigger from T2 or T3 or T4  is selected"]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T2_SPEC;
impl crate::sealed::RegSpec for T2_SPEC {
    type DataType = u32;
}
#[doc = "Timer T2 Register\n resetvalue={Application Reset:0x0}"]
pub type T2 = crate::RegValueT<T2_SPEC>;

impl T2 {
    #[doc = "Timer T2   T2. Contains the current value of Timer T2."]
    #[inline(always)]
    pub fn t2(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, T2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, T2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T2 {
    #[inline(always)]
    fn default() -> T2 {
        <crate::RegValueT<T2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T2Con_SPEC;
impl crate::sealed::RegSpec for T2Con_SPEC {
    type DataType = u32;
}
#[doc = "Timer T2 Control Register\n resetvalue={Application Reset:0x0}"]
pub type T2Con = crate::RegValueT<T2Con_SPEC>;

impl T2Con {
    #[doc = "Timer T2 Input Parameter Selection   T2I. Depends on the operating mode  see respective sections for encoding  CROSSREFERENCE for Timer Mode and Gated Timer Mode CROSSREFERENCE for Counter Mode CROSSREFERENCE for Incremental Interface Mode"]
    #[inline(always)]
    pub fn t2i(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, T2Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, T2Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T2 Mode Control  Basic Operating Mode    T2M"]
    #[inline(always)]
    pub fn t2m(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, t2con::T2M, T2Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x7,1,0,t2con::T2M, T2Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T2 Run Bit   T2R. This bit only controls timer T2 if bit T2RC   0."]
    #[inline(always)]
    pub fn t2r(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, t2con::T2R, T2Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,t2con::T2R, T2Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T2 Up Down Control   T2UD. This bit only directly controls count direction of T2 if bit T2UDE   0."]
    #[inline(always)]
    pub fn t2ud(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, t2con::T2Ud, T2Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,t2con::T2Ud, T2Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T2 External Up Down Enable   T2UDE"]
    #[inline(always)]
    pub fn t2ude(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, t2con::T2Ude, T2Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,t2con::T2Ude, T2Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T2 Remote Control   T2RC"]
    #[inline(always)]
    pub fn t2rc(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, t2con::T2Rc, T2Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,t2con::T2Rc, T2Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T2 Interrupt Disable   T2IRDIS"]
    #[inline(always)]
    pub fn t2irdis(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, t2con::T2Irdis, T2Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,t2con::T2Irdis, T2Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T2 Edge Detection   T2EDGE. The bit is set each time a count edge is detected. T2EDGE must be cleared by software."]
    #[inline(always)]
    pub fn t2edge(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, t2con::T2Edge, T2Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,t2con::T2Edge, T2Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T2 Count Direction Change   T2CHDIR. The bit is set each time the count direction of timer T2 changes. T2CHDIR must be cleared by software."]
    #[inline(always)]
    pub fn t2chdir(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, t2con::T2Chdir, T2Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,t2con::T2Chdir, T2Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T2 Rotation Direction   T2RDIR"]
    #[inline(always)]
    pub fn t2rdir(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, t2con::T2Rdir, T2Con_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<15,0x1,1,0,t2con::T2Rdir, T2Con_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for T2Con {
    #[inline(always)]
    fn default() -> T2Con {
        <crate::RegValueT<T2Con_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod t2con {
    pub struct T2M_SPEC;
    pub type T2M = crate::EnumBitfieldStruct<u8, T2M_SPEC>;
    impl T2M {
        #[doc = "000 Timer Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 Counter Mode"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 Gated Timer Mode with gate active low"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 Gated Timer Mode with gate active high"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 Reload Mode"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 Capture Mode"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 Incremental Interface Mode  Rotation Detection Mode"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "111 Incremental Interface Mode  Edge Detection Mode"]
        pub const CONST_77: Self = Self::new(7);
    }
    pub struct T2R_SPEC;
    pub type T2R = crate::EnumBitfieldStruct<u8, T2R_SPEC>;
    impl T2R {
        #[doc = "0 Timer T2 stops"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer T2 runs"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T2Ud_SPEC;
    pub type T2Ud = crate::EnumBitfieldStruct<u8, T2Ud_SPEC>;
    impl T2Ud {
        #[doc = "0 Timer T2 counts up"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer T2 counts down"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T2Ude_SPEC;
    pub type T2Ude = crate::EnumBitfieldStruct<u8, T2Ude_SPEC>;
    impl T2Ude {
        #[doc = "0 Count direction        is controlled by bit T2UD  input T2EUD is disconnected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Count direction        is controlled by input T2EUD  see also CROSSREFERENCE"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T2Rc_SPEC;
    pub type T2Rc = crate::EnumBitfieldStruct<u8, T2Rc_SPEC>;
    impl T2Rc {
        #[doc = "0 Timer T2 is controlled by its own run bit T2R"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer T2 is controlled by the run bit T3R of core timer T3  not by bit T2R"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T2Irdis_SPEC;
    pub type T2Irdis = crate::EnumBitfieldStruct<u8, T2Irdis_SPEC>;
    impl T2Irdis {
        #[doc = "0 Interrupt generation for T2CHDIR and T2EDGE interrupts in Incremental Interface Mode is enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt generation for T2CHDIR and T2EDGE interrupts in Incremental Interface Mode is disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T2Edge_SPEC;
    pub type T2Edge = crate::EnumBitfieldStruct<u8, T2Edge_SPEC>;
    impl T2Edge {
        #[doc = "0 No count edge was detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A count edge was detected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T2Chdir_SPEC;
    pub type T2Chdir = crate::EnumBitfieldStruct<u8, T2Chdir_SPEC>;
    impl T2Chdir {
        #[doc = "0 No change in count direction was detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A change in count direction was detected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T2Rdir_SPEC;
    pub type T2Rdir = crate::EnumBitfieldStruct<u8, T2Rdir_SPEC>;
    impl T2Rdir {
        #[doc = "0 Timer T2 counts up"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer T2 counts down"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T3_SPEC;
impl crate::sealed::RegSpec for T3_SPEC {
    type DataType = u32;
}
#[doc = "Timer T3 Register\n resetvalue={Application Reset:0x0}"]
pub type T3 = crate::RegValueT<T3_SPEC>;

impl T3 {
    #[doc = "Timer T3   T3. Contains the current value of Timer T3."]
    #[inline(always)]
    pub fn t3(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, T3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, T3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T3 {
    #[inline(always)]
    fn default() -> T3 {
        <crate::RegValueT<T3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T3Con_SPEC;
impl crate::sealed::RegSpec for T3Con_SPEC {
    type DataType = u32;
}
#[doc = "Timer T3 Control Register\n resetvalue={Application Reset:0x0}"]
pub type T3Con = crate::RegValueT<T3Con_SPEC>;

impl T3Con {
    #[doc = "Timer T3 Input Parameter Selection   T3I. Depends on the operating mode  see respective sections for encoding  CROSSREFERENCE for Timer Mode and Gated Timer Mode CROSSREFERENCE for Counter Mode CROSSREFERENCE for Incremental Interface Mode"]
    #[inline(always)]
    pub fn t3i(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, T3Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, T3Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T3 Mode Control   T3M"]
    #[inline(always)]
    pub fn t3m(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, t3con::T3M, T3Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x7,1,0,t3con::T3M, T3Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T3 Run Bit   T3R"]
    #[inline(always)]
    pub fn t3r(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, t3con::T3R, T3Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,t3con::T3R, T3Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T3 Up Down Control   T3UD. This bit only directly controls count direction of T3 if bit T3UDE   0."]
    #[inline(always)]
    pub fn t3ud(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, t3con::T3Ud, T3Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,t3con::T3Ud, T3Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T3 External Up Down Enable   T3UDE"]
    #[inline(always)]
    pub fn t3ude(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, t3con::T3Ude, T3Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,t3con::T3Ude, T3Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Overflow Underflow Output Enable   T3OE"]
    #[inline(always)]
    pub fn t3oe(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, t3con::T3Oe, T3Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,t3con::T3Oe, T3Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T3 Overflow Toggle Latch   T3OTL. Toggles on each overflow underflow of T3. Can be set or cleared by        software  see separate description"]
    #[inline(always)]
    pub fn t3otl(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, T3Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, T3Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GPT1 Block Prescaler Control   BPS1. Selects the basic clock for block GPT1  see also CROSSREFERENCE"]
    #[inline(always)]
    pub fn bps1(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, t3con::Bps1, T3Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x3,1,0,t3con::Bps1, T3Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T3 Edge Detection Flag   T3EDGE. The bit is set each time a count edge is detected. T3EDGE must be        cleared by software."]
    #[inline(always)]
    pub fn t3edge(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, t3con::T3Edge, T3Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,t3con::T3Edge, T3Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T3 Count Direction Change Flag   T3CHDIR. This bit is set each time the count direction of timer T3 changes.        T3CHDIR must be cleared by software."]
    #[inline(always)]
    pub fn t3chdir(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, t3con::T3Chdir, T3Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,t3con::T3Chdir, T3Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T3 Rotation Direction Flag   T3RDIR"]
    #[inline(always)]
    pub fn t3rdir(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, t3con::T3Rdir, T3Con_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<15,0x1,1,0,t3con::T3Rdir, T3Con_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for T3Con {
    #[inline(always)]
    fn default() -> T3Con {
        <crate::RegValueT<T3Con_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod t3con {
    pub struct T3M_SPEC;
    pub type T3M = crate::EnumBitfieldStruct<u8, T3M_SPEC>;
    impl T3M {
        #[doc = "000 Timer Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 Counter Mode"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 Gated Timer Mode with gate active low"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 Gated Timer Mode with gate active high"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "110 Incremental Interface Mode  Rotation Detection Mode"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "111 Incremental Interface Mode  Edge Detection Mode"]
        pub const CONST_77: Self = Self::new(7);
    }
    pub struct T3R_SPEC;
    pub type T3R = crate::EnumBitfieldStruct<u8, T3R_SPEC>;
    impl T3R {
        #[doc = "0 Timer T3 stops"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer T3 runs"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T3Ud_SPEC;
    pub type T3Ud = crate::EnumBitfieldStruct<u8, T3Ud_SPEC>;
    impl T3Ud {
        #[doc = "0 Timer T3 counts up"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer T3 counts down"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T3Ude_SPEC;
    pub type T3Ude = crate::EnumBitfieldStruct<u8, T3Ude_SPEC>;
    impl T3Ude {
        #[doc = "0 Count direction        is controlled by bit T3UD  input T3EUD is disconnected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Count direction        is controlled by input T3EUD  see also CROSSREFERENCE"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T3Oe_SPEC;
    pub type T3Oe = crate::EnumBitfieldStruct<u8, T3Oe_SPEC>;
    impl T3Oe {
        #[doc = "0 Alternate Output Function Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 State of T3 toggle latch is output on pin T3OUT"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Bps1_SPEC;
    pub type Bps1 = crate::EnumBitfieldStruct<u8, Bps1_SPEC>;
    impl Bps1 {
        #[doc = "00 f GPT  8"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 f GPT  4"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 f GPT  32"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 f GPT  16"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct T3Edge_SPEC;
    pub type T3Edge = crate::EnumBitfieldStruct<u8, T3Edge_SPEC>;
    impl T3Edge {
        #[doc = "0 No count edge was detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A count edge was detected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T3Chdir_SPEC;
    pub type T3Chdir = crate::EnumBitfieldStruct<u8, T3Chdir_SPEC>;
    impl T3Chdir {
        #[doc = "0 No change of count direction was detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A change of count direction was detected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T3Rdir_SPEC;
    pub type T3Rdir = crate::EnumBitfieldStruct<u8, T3Rdir_SPEC>;
    impl T3Rdir {
        #[doc = "0 Timer T3 counts up"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer T3 counts down"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T4_SPEC;
impl crate::sealed::RegSpec for T4_SPEC {
    type DataType = u32;
}
#[doc = "Timer T4 Register\n resetvalue={Application Reset:0x0}"]
pub type T4 = crate::RegValueT<T4_SPEC>;

impl T4 {
    #[doc = "Timer T4   T4. Contains the current value of Timer T4."]
    #[inline(always)]
    pub fn t4(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, T4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, T4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T4 {
    #[inline(always)]
    fn default() -> T4 {
        <crate::RegValueT<T4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T4Con_SPEC;
impl crate::sealed::RegSpec for T4Con_SPEC {
    type DataType = u32;
}
#[doc = "Timer T4 Control Register\n resetvalue={Application Reset:0x0}"]
pub type T4Con = crate::RegValueT<T4Con_SPEC>;

impl T4Con {
    #[doc = "Timer T4 Input Parameter Selection   T4I. Depends on the operating mode  see respective sections for encoding  CROSSREFERENCE for Timer Mode and Gated Timer Mode CROSSREFERENCE for Counter Mode CROSSREFERENCE for Incremental Interface Mode"]
    #[inline(always)]
    pub fn t4i(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, T4Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, T4Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T4 Mode Control  Basic Operating Mode    T4M"]
    #[inline(always)]
    pub fn t4m(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, t4con::T4M, T4Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x7,1,0,t4con::T4M, T4Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T4 Run Bit   T4R. This bit only controls timer T4 if bit T4RC   0."]
    #[inline(always)]
    pub fn t4r(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, t4con::T4R, T4Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,t4con::T4R, T4Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T4 Up Down Control   T4UD. This bit only directly controls count direction of T4 if bit T4UDE   0."]
    #[inline(always)]
    pub fn t4ud(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, t4con::T4Ud, T4Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,t4con::T4Ud, T4Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T4 External Up Down Enable   T4UDE"]
    #[inline(always)]
    pub fn t4ude(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, t4con::T4Ude, T4Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,t4con::T4Ude, T4Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T4 Remote Control   T4RC"]
    #[inline(always)]
    pub fn t4rc(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, t4con::T4Rc, T4Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,t4con::T4Rc, T4Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear Timer T2 Enable   CLRT2EN. Enables the automatic clearing of timer T2 upon a falling edge of the selected T4EUD input."]
    #[inline(always)]
    pub fn clrt2en(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, t4con::Clrt2En, T4Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,t4con::Clrt2En, T4Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear Timer T3 Enable   CLRT3EN. Enables the automatic clearing of timer T3 upon a falling edge of the selected T4IN input."]
    #[inline(always)]
    pub fn clrt3en(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, t4con::Clrt3En, T4Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,t4con::Clrt3En, T4Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T4 Interrupt Disable   T4IRDIS"]
    #[inline(always)]
    pub fn t4irdis(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, t4con::T4Irdis, T4Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,t4con::T4Irdis, T4Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T4 Edge Detection   T4EDGE. The bit is set each time a count edge is detected. T4EDGE has to be cleared by software."]
    #[inline(always)]
    pub fn t4edge(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, t4con::T4Edge, T4Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,t4con::T4Edge, T4Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T4 Count Direction Change   T4CHDIR. The bit is set each time the count direction of timer T4 changes. T4CHDIR must be cleared by software."]
    #[inline(always)]
    pub fn t4chdir(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, t4con::T4Chdir, T4Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,t4con::T4Chdir, T4Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T4 Rotation Direction   T4RDIR"]
    #[inline(always)]
    pub fn t4rdir(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, t4con::T4Rdir, T4Con_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<15,0x1,1,0,t4con::T4Rdir, T4Con_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for T4Con {
    #[inline(always)]
    fn default() -> T4Con {
        <crate::RegValueT<T4Con_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod t4con {
    pub struct T4M_SPEC;
    pub type T4M = crate::EnumBitfieldStruct<u8, T4M_SPEC>;
    impl T4M {
        #[doc = "000 Timer Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 Counter Mode"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 Gated Timer Mode with gate active low"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 Gated Timer Mode with gate active high"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 Reload Mode"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 Capture Mode"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 Incremental Interface Mode  Rotation Detection Mode"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "111 Incremental Interface Mode  Edge Detection Mode"]
        pub const CONST_77: Self = Self::new(7);
    }
    pub struct T4R_SPEC;
    pub type T4R = crate::EnumBitfieldStruct<u8, T4R_SPEC>;
    impl T4R {
        #[doc = "0 Timer T4 stops"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer T4 runs"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T4Ud_SPEC;
    pub type T4Ud = crate::EnumBitfieldStruct<u8, T4Ud_SPEC>;
    impl T4Ud {
        #[doc = "0 Timer T4 counts up"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer T4 counts down"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T4Ude_SPEC;
    pub type T4Ude = crate::EnumBitfieldStruct<u8, T4Ude_SPEC>;
    impl T4Ude {
        #[doc = "0 Count direction is controlled by bit T4UD   input T4EUD is disconnected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Count direction        is controlled by input T4EUD  see also CROSSREFERENCE"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T4Rc_SPEC;
    pub type T4Rc = crate::EnumBitfieldStruct<u8, T4Rc_SPEC>;
    impl T4Rc {
        #[doc = "0 Timer T4 is controlled by its own run bit T4R"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer T4 is controlled by the run bit T3R of core timer T3  but not by bit T4R"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clrt2En_SPEC;
    pub type Clrt2En = crate::EnumBitfieldStruct<u8, Clrt2En_SPEC>;
    impl Clrt2En {
        #[doc = "0 No effect of T4EUD on timer T2"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A falling edge on T4EUD clears timer T2"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clrt3En_SPEC;
    pub type Clrt3En = crate::EnumBitfieldStruct<u8, Clrt3En_SPEC>;
    impl Clrt3En {
        #[doc = "0 No effect of T4IN on timer T3"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A falling edge on T4IN clears timer T3"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T4Irdis_SPEC;
    pub type T4Irdis = crate::EnumBitfieldStruct<u8, T4Irdis_SPEC>;
    impl T4Irdis {
        #[doc = "0 Interrupt generation for T4CHDIR and T4EDGE interrupts in Incremental Interface Mode is enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt generation for T4CHDIR and T4EDGE interrupts in Incremental Interface Mode is disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T4Edge_SPEC;
    pub type T4Edge = crate::EnumBitfieldStruct<u8, T4Edge_SPEC>;
    impl T4Edge {
        #[doc = "0 No count edge was detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A count edge was detected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T4Chdir_SPEC;
    pub type T4Chdir = crate::EnumBitfieldStruct<u8, T4Chdir_SPEC>;
    impl T4Chdir {
        #[doc = "0 No change in count direction was detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A change in count direction was detected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T4Rdir_SPEC;
    pub type T4Rdir = crate::EnumBitfieldStruct<u8, T4Rdir_SPEC>;
    impl T4Rdir {
        #[doc = "0 Timer T4 counts up"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer T4 counts down"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T5_SPEC;
impl crate::sealed::RegSpec for T5_SPEC {
    type DataType = u32;
}
#[doc = "Timer T5 Register\n resetvalue={Application Reset:0x0}"]
pub type T5 = crate::RegValueT<T5_SPEC>;

impl T5 {
    #[doc = "Timer T5   T5. Contains the current value of Timer T5."]
    #[inline(always)]
    pub fn t5(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, T5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, T5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T5 {
    #[inline(always)]
    fn default() -> T5 {
        <crate::RegValueT<T5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T5Con_SPEC;
impl crate::sealed::RegSpec for T5Con_SPEC {
    type DataType = u32;
}
#[doc = "Timer T5 Control Register\n resetvalue={Application Reset:0x0}"]
pub type T5Con = crate::RegValueT<T5Con_SPEC>;

impl T5Con {
    #[doc = "Timer T5 Input Parameter Selection   T5I. Depends on the operating mode  see respective sections for encoding  CROSSREFERENCE for Timer Mode and Gated Timer Mode CROSSREFERENCE for Counter Mode"]
    #[inline(always)]
    pub fn t5i(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, T5Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, T5Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T5 Run Bit   T5R. This bit only controls timer T5 if bit T5RC   0."]
    #[inline(always)]
    pub fn t5r(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, t5con::T5R, T5Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,t5con::T5R, T5Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T5 Up Down Control   T5UD. This bit only directly controls count direction of T5 if bit T5UDE   0."]
    #[inline(always)]
    pub fn t5ud(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, t5con::T5Ud, T5Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,t5con::T5Ud, T5Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T5 External Up Down Enable   T5UDE"]
    #[inline(always)]
    pub fn t5ude(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, t5con::T5Ude, T5Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,t5con::T5Ude, T5Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T5 Remote Control   T5RC"]
    #[inline(always)]
    pub fn t5rc(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, t5con::T5Rc, T5Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,t5con::T5Rc, T5Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T3 Capture Trigger Enable   CT3"]
    #[inline(always)]
    pub fn ct3(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, t5con::Ct3, T5Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,t5con::Ct3, T5Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T5 Capture Correction   T5CC. Implementation Bug  Capture correction does not work for all          conditions  AI00039414 ."]
    #[inline(always)]
    pub fn t5cc(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, t5con::T5Cc, T5Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,t5con::T5Cc, T5Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Register CAPREL Capture Trigger Selection   CI"]
    #[inline(always)]
    pub fn ci(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, t5con::Ci, T5Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,t5con::Ci, T5Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T5 Clear Enable Bit   T5CLR"]
    #[inline(always)]
    pub fn t5clr(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, t5con::T5Clr, T5Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,t5con::T5Clr, T5Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T5 Capture Mode Enable   T5SC"]
    #[inline(always)]
    pub fn t5sc(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, t5con::T5Sc, T5Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,t5con::T5Sc, T5Con_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T5Con {
    #[inline(always)]
    fn default() -> T5Con {
        <crate::RegValueT<T5Con_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod t5con {
    pub struct T5R_SPEC;
    pub type T5R = crate::EnumBitfieldStruct<u8, T5R_SPEC>;
    impl T5R {
        #[doc = "Timer T5 stops"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Timer T5 runs"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T5Ud_SPEC;
    pub type T5Ud = crate::EnumBitfieldStruct<u8, T5Ud_SPEC>;
    impl T5Ud {
        #[doc = "0 Timer T5 counts up"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer T5 counts down"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T5Ude_SPEC;
    pub type T5Ude = crate::EnumBitfieldStruct<u8, T5Ude_SPEC>;
    impl T5Ude {
        #[doc = "0 Count direction        is controlled by bit T5UD  input T5EUD is disconnected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Count direction        is controlled by input T5EUD  see also CROSSREFERENCE"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T5Rc_SPEC;
    pub type T5Rc = crate::EnumBitfieldStruct<u8, T5Rc_SPEC>;
    impl T5Rc {
        #[doc = "0 Timer T5 is controlled by its own run bit T5R"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer T5 is controlled by the run bit T6R of core timer T6  not by bit T5R"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ct3_SPEC;
    pub type Ct3 = crate::EnumBitfieldStruct<u8, Ct3_SPEC>;
    impl Ct3 {
        #[doc = "0 Capture trigger        from input line CAPIN"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Capture trigger        from T3 input lines T3IN and or T3EUD"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T5Cc_SPEC;
    pub type T5Cc = crate::EnumBitfieldStruct<u8, T5Cc_SPEC>;
    impl T5Cc {
        #[doc = "0 T5 is just captured without any correction"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 T5 is decremented by 1 before being captured"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ci_SPEC;
    pub type Ci = crate::EnumBitfieldStruct<u8, Ci_SPEC>;
    impl Ci {
        #[doc = "00 Capture        disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Positive        transition  rising edge  on CAPIN Rising edge        must be selected for triggering the capture clear operation by the        internal GPT1 read signals  see bit field ISCAPIN in register PISEL and description in section CROSSREFERENCE  . or any transition on T3IN"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Negative        transition  falling edge  on CAPIN or any transition on T3EUD"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Any transition         rising or falling edge  on CAPIN or any transition on T3IN or T3EUD"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct T5Clr_SPEC;
    pub type T5Clr = crate::EnumBitfieldStruct<u8, T5Clr_SPEC>;
    impl T5Clr {
        #[doc = "0 Timer T5 is not        cleared on a capture event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer T5 is        cleared on a capture event"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T5Sc_SPEC;
    pub type T5Sc = crate::EnumBitfieldStruct<u8, T5Sc_SPEC>;
    impl T5Sc {
        #[doc = "0 Capture into        register CAPREL disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Capture into        register CAPREL enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T6_SPEC;
impl crate::sealed::RegSpec for T6_SPEC {
    type DataType = u32;
}
#[doc = "Timer T6 Register\n resetvalue={Application Reset:0x0}"]
pub type T6 = crate::RegValueT<T6_SPEC>;

impl T6 {
    #[doc = "Timer T6   T6. Contains the current value of Timer T6."]
    #[inline(always)]
    pub fn t6(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, T6_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, T6_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T6 {
    #[inline(always)]
    fn default() -> T6 {
        <crate::RegValueT<T6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct T6Con_SPEC;
impl crate::sealed::RegSpec for T6Con_SPEC {
    type DataType = u32;
}
#[doc = "Timer T6 Control Register\n resetvalue={Application Reset:0x0}"]
pub type T6Con = crate::RegValueT<T6Con_SPEC>;

impl T6Con {
    #[doc = "Timer T6 Input Parameter Selection   T6I. Depends on the operating mode  see respective sections for encoding  CROSSREFERENCE for Timer Mode and Gated Timer Mode CROSSREFERENCE for Counter Mode"]
    #[inline(always)]
    pub fn t6i(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, T6Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, T6Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T6 Mode Control  Basic Operating Mode    T6M"]
    #[inline(always)]
    pub fn t6m(
        self,
    ) -> crate::common::RegisterField<3, 0x7, 1, 0, t6con::T6M, T6Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x7,1,0,t6con::T6M, T6Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T6 Run Bit   T6R"]
    #[inline(always)]
    pub fn t6r(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, t6con::T6R, T6Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,t6con::T6R, T6Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T6 Up Down Control   T6UD. This bit only directly controls count direction of T6 if bit T6UDE   0."]
    #[inline(always)]
    pub fn t6ud(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, t6con::T6Ud, T6Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,t6con::T6Ud, T6Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T6 External Up Down Enable   T6UDE"]
    #[inline(always)]
    pub fn t6ude(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, t6con::T6Ude, T6Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,t6con::T6Ude, T6Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Overflow Underflow Output Enable   T6OE"]
    #[inline(always)]
    pub fn t6oe(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, t6con::T6Oe, T6Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,t6con::T6Oe, T6Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T6 Overflow Toggle Latch   T6OTL. Toggles on each overflow underflow of timer T6. Can be set or cleared by        software  see separate description"]
    #[inline(always)]
    pub fn t6otl(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, T6Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, T6Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "GPT2 Block Prescaler Control   BPS2. Selects the basic clock for block GPT2  see also CROSSREFERENCE"]
    #[inline(always)]
    pub fn bps2(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, t6con::Bps2, T6Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x3,1,0,t6con::Bps2, T6Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T6 Clear Enable Bit   T6CLR"]
    #[inline(always)]
    pub fn t6clr(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, t6con::T6Clr, T6Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,t6con::T6Clr, T6Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Timer T6 Reload Mode Enable   T6SR"]
    #[inline(always)]
    pub fn t6sr(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, t6con::T6Sr, T6Con_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,t6con::T6Sr, T6Con_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for T6Con {
    #[inline(always)]
    fn default() -> T6Con {
        <crate::RegValueT<T6Con_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod t6con {
    pub struct T6M_SPEC;
    pub type T6M = crate::EnumBitfieldStruct<u8, T6M_SPEC>;
    impl T6M {
        #[doc = "000 Timer Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 Counter Mode"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 Gated Timer Mode with gate active low"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 Gated Timer Mode with gate active high"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct T6R_SPEC;
    pub type T6R = crate::EnumBitfieldStruct<u8, T6R_SPEC>;
    impl T6R {
        #[doc = "0 Timer T6 stops"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer T6 runs"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T6Ud_SPEC;
    pub type T6Ud = crate::EnumBitfieldStruct<u8, T6Ud_SPEC>;
    impl T6Ud {
        #[doc = "0 Timer T6 counts        up"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer T6 counts        down"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T6Ude_SPEC;
    pub type T6Ude = crate::EnumBitfieldStruct<u8, T6Ude_SPEC>;
    impl T6Ude {
        #[doc = "0 Count direction        is controlled by bit T6UD  input T6EUD is disconnected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Count direction        is controlled by input T6EUD  see also CROSSREFERENCE"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T6Oe_SPEC;
    pub type T6Oe = crate::EnumBitfieldStruct<u8, T6Oe_SPEC>;
    impl T6Oe {
        #[doc = "0 Alternate Output Function Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 State of timer T6 toggle latch is output on pin T6OUT"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Bps2_SPEC;
    pub type Bps2 = crate::EnumBitfieldStruct<u8, Bps2_SPEC>;
    impl Bps2 {
        #[doc = "00 f GPT  4"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 f GPT  2"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 f GPT  16"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 f GPT  8"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct T6Clr_SPEC;
    pub type T6Clr = crate::EnumBitfieldStruct<u8, T6Clr_SPEC>;
    impl T6Clr {
        #[doc = "0 Timer T6 is not cleared on a capture event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Timer T6 is cleared on a capture event"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct T6Sr_SPEC;
    pub type T6Sr = crate::EnumBitfieldStruct<u8, T6Sr_SPEC>;
    impl T6Sr {
        #[doc = "0 Reload from register CAPREL Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Reload from register CAPREL Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
