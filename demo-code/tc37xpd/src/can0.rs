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
#[doc = r"CAN"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Can0(pub(super) *mut u8);
unsafe impl core::marker::Send for Can0 {}
unsafe impl core::marker::Sync for Can0 {}
impl Can0 {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(33020usize)) }
    }

    #[doc = "Access Enable Register Control 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accenctr0(&self) -> crate::common::Reg<self::Accenctr0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32988usize)) }
    }

    #[doc = "Buffer receive address and transmit address\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn bufadr(&self) -> crate::common::Reg<self::Bufadr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32820usize)) }
    }

    #[doc = "CAN Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32768usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0B8C000}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32776usize)) }
    }

    #[doc = "Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst0(&self) -> crate::common::Reg<self::Krst0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(33012usize)) }
    }

    #[doc = "Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst1(&self) -> crate::common::Reg<self::Krst1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(33008usize)) }
    }

    #[doc = "Kernel Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krstclr(&self) -> crate::common::Reg<self::Krstclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(33004usize)) }
    }

    #[doc = "Module Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mcr(&self) -> crate::common::Reg<self::Mcr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32816usize)) }
    }

    #[doc = "Measure Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mecr(&self) -> crate::common::Reg<self::Mecr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32832usize)) }
    }

    #[doc = "Measure Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mestat(&self) -> crate::common::Reg<self::Mestat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32836usize)) }
    }

    #[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(33000usize)) }
    }
    #[doc = "N"]
    #[inline(always)]
    pub fn n(self) -> [self::N; 4] {
        unsafe {
            [
                self::N(self.0.add(0x8100usize + 0x0usize)),
                self::N(self.0.add(0x8100usize + 0x400usize)),
                self::N(self.0.add(0x8100usize + 0x800usize)),
                self::N(self.0.add(0x8100usize + 0xc00usize)),
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
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, accen0::En0, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,accen0::En0, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, accen0::En1, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,accen0::En1, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, accen0::En2, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,accen0::En2, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, accen0::En3, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,accen0::En3, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, accen0::En4, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,accen0::En4, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, accen0::En5, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,accen0::En5, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, accen0::En6, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,accen0::En6, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, accen0::En7, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,accen0::En7, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, accen0::En8, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,accen0::En8, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, accen0::En9, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,accen0::En9, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, accen0::En10, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,accen0::En10, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, accen0::En11, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,accen0::En11, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, accen0::En12, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,accen0::En12, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, accen0::En13, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,accen0::En13, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, accen0::En14, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,accen0::En14, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, accen0::En15, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,accen0::En15, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, accen0::En16, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,accen0::En16, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, accen0::En17, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,accen0::En17, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, accen0::En18, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,accen0::En18, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, accen0::En19, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,accen0::En19, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, accen0::En20, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,accen0::En20, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, accen0::En21, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,accen0::En21, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, accen0::En22, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,accen0::En22, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, accen0::En23, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,accen0::En23, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, accen0::En24, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,accen0::En24, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, accen0::En25, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,accen0::En25, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, accen0::En26, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,accen0::En26, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, accen0::En27, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,accen0::En27, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, accen0::En28, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,accen0::En28, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, accen0::En29, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,accen0::En29, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, accen0::En30, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,accen0::En30, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y."]
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
pub struct Accenctr0_SPEC;
impl crate::sealed::RegSpec for Accenctr0_SPEC {
    type DataType = u32;
}
#[doc = "Access Enable Register Control 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Accenctr0 = crate::RegValueT<Accenctr0_SPEC>;

impl Accenctr0 {
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, accenctr0::En0, Accenctr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            accenctr0::En0,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, accenctr0::En1, Accenctr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            accenctr0::En1,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, accenctr0::En2, Accenctr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            accenctr0::En2,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, accenctr0::En3, Accenctr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            accenctr0::En3,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, accenctr0::En4, Accenctr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            accenctr0::En4,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, accenctr0::En5, Accenctr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            accenctr0::En5,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, accenctr0::En6, Accenctr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            accenctr0::En6,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, accenctr0::En7, Accenctr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            accenctr0::En7,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, accenctr0::En8, Accenctr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            accenctr0::En8,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, accenctr0::En9, Accenctr0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            accenctr0::En9,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        accenctr0::En10,
        Accenctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            accenctr0::En10,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        accenctr0::En11,
        Accenctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            accenctr0::En11,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        accenctr0::En12,
        Accenctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            accenctr0::En12,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        accenctr0::En13,
        Accenctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            accenctr0::En13,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        accenctr0::En14,
        Accenctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            accenctr0::En14,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        accenctr0::En15,
        Accenctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            accenctr0::En15,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        accenctr0::En16,
        Accenctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            accenctr0::En16,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        accenctr0::En17,
        Accenctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            accenctr0::En17,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        accenctr0::En18,
        Accenctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            accenctr0::En18,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        accenctr0::En19,
        Accenctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            accenctr0::En19,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        accenctr0::En20,
        Accenctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            accenctr0::En20,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        accenctr0::En21,
        Accenctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            accenctr0::En21,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        accenctr0::En22,
        Accenctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            accenctr0::En22,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        accenctr0::En23,
        Accenctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            accenctr0::En23,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        accenctr0::En24,
        Accenctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            accenctr0::En24,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        accenctr0::En25,
        Accenctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            accenctr0::En25,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        accenctr0::En26,
        Accenctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            accenctr0::En26,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        accenctr0::En27,
        Accenctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            accenctr0::En27,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        accenctr0::En28,
        Accenctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            accenctr0::En28,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterField<
        29,
        0x1,
        1,
        0,
        accenctr0::En29,
        Accenctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            29,
            0x1,
            1,
            0,
            accenctr0::En29,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        accenctr0::En30,
        Accenctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            accenctr0::En30,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en31(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        accenctr0::En31,
        Accenctr0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            accenctr0::En31,
            Accenctr0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Accenctr0 {
    #[inline(always)]
    fn default() -> Accenctr0 {
        <crate::RegValueT<Accenctr0_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod accenctr0 {
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
pub struct Bufadr_SPEC;
impl crate::sealed::RegSpec for Bufadr_SPEC {
    type DataType = u32;
}
#[doc = "Buffer receive address and transmit address\n resetvalue={Application Reset:0x0}"]
pub type Bufadr = crate::RegValueT<Bufadr_SPEC>;

impl Bufadr {
    #[doc = "Transmit Buffer start address   TXBUF. This is the start address of the first dedicated transmit buffer."]
    #[inline(always)]
    pub fn txbuf(
        self,
    ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Bufadr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3fff,1,0,u16, Bufadr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive Buffer start address   RXBUF. This is the start address of the first dedicated receive buffer."]
    #[inline(always)]
    pub fn rxbuf(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, Bufadr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3fff,1,0,u16, Bufadr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Bufadr {
    #[inline(always)]
    fn default() -> Bufadr {
        <crate::RegValueT<Bufadr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clc_SPEC;
impl crate::sealed::RegSpec for Clc_SPEC {
    type DataType = u32;
}
#[doc = "CAN Clock Control Register\n resetvalue={Application Reset:0x3}"]
pub type Clc = crate::RegValueT<Clc_SPEC>;

impl Clc {
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control of the module. The synchronous and        asynchronous clock is switched on off Note that no register access is        possible to any register while module is disabled. A disable request is        granted  if the M CAN clock is disabled  or all M CAN nodes acknowledge the disable request."]
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
    #[doc = "Sleep Mode Disable Control   EDIS. Used to control module s sleep mode."]
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
        <crate::RegValueT<Clc_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0B8C000}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MOD REV. MOD REV defines the revision number. The value of a module revision        starts with 01 H  first revision ."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MOD TYPE"]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, id::ModType, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,id::ModType, Id_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Module Number Value   MOD NUMBER. This bit field defines the MCMCAN module identification number 00B8 ."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(12107776)
    }
}
pub mod id {
    pub struct ModType_SPEC;
    pub type ModType = crate::EnumBitfieldStruct<u8, ModType_SPEC>;
    impl ModType {
        #[doc = "C0 Define the        module as a 32 bit module."]
        pub const CONST_192192: Self = Self::new(192);
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel        reset will be executed if the reset bits of both kernel registers are        set. The RST bit will be cleared  reset to   180 0  180   by the BPI FPI after the        kernel reset was executed."]
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel reset registers is set. The RST bit will be cleared  reset to  0   by the BPI FPI after the kernel reset was executed."]
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
pub struct Mcr_SPEC;
impl crate::sealed::RegSpec for Mcr_SPEC {
    type DataType = u32;
}
#[doc = "Module Control Register\n resetvalue={Application Reset:0x0}"]
pub type Mcr = crate::RegValueT<Mcr_SPEC>;

impl Mcr {
    #[doc = "Clock Select 0   CLKSEL0. This bitfield is MCR.CI and MCR.CCCE protected."]
    #[inline(always)]
    pub fn clksel0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, mcr::Clksel0, Mcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,mcr::Clksel0, Mcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Select 1   CLKSEL1. This bitfield is MCR.CI and MCR.CCCE protected."]
    #[inline(always)]
    pub fn clksel1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, mcr::Clksel1, Mcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,mcr::Clksel1, Mcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Select 2   CLKSEL2. This bitfield is MCR.CI and MCR.CCCE protected."]
    #[inline(always)]
    pub fn clksel2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, mcr::Clksel2, Mcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,mcr::Clksel2, Mcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Select 3   CLKSEL3. This bitfield is MCR.CI and MCR.CCCE protected."]
    #[inline(always)]
    pub fn clksel3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, mcr::Clksel3, Mcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,mcr::Clksel3, Mcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Node   NODE. This bit field determines the CAN node i which is used for debug over        CAN. This bitfield only exists on CAN0."]
    #[inline(always)]
    pub fn node(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Mcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Mcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RAM BUSY   RBUSY. This bit is not implemented in A step silicon. This bit shows that the RAM Initialization is running. This bit is set        back to 0b by hardware when the RAM intialization is completed."]
    #[inline(always)]
    pub fn rbusy(self) -> crate::common::RegisterFieldBool<28, 1, 0, Mcr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Mcr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RAM Init   RINIT. This bit is not implemented in A step silicon. This bit is MCR.CI and MCR.CCCE protected. This bit starts the initialization of the RAM block to all 0x0. The RAM initialization is started only when this bit is changed from 0b        to 1b and also RBUSY is 0b."]
    #[inline(always)]
    pub fn rinit(self) -> crate::common::RegisterFieldBool<29, 1, 0, Mcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Mcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Change Init   CI. Needs to be set to enable and disable clocks."]
    #[inline(always)]
    pub fn ci(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, mcr::Ci, Mcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x1,1,0,mcr::Ci, Mcr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock and RAM Change Enable   CCCE. Needs to be set to enable and disable the clocks."]
    #[inline(always)]
    pub fn ccce(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, mcr::Ccce, Mcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<31,0x1,1,0,mcr::Ccce, Mcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Mcr {
    #[inline(always)]
    fn default() -> Mcr {
        <crate::RegValueT<Mcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mcr {
    pub struct Clksel0_SPEC;
    pub type Clksel0 = crate::EnumBitfieldStruct<u8, Clksel0_SPEC>;
    impl Clksel0 {
        #[doc = "00 No clock        supplied"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 The        asynchronous clock source is switched on"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 The        synchronous clock source is switched on"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Both clock        sources are switched on  according        to Bosch this is possible"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Clksel1_SPEC;
    pub type Clksel1 = crate::EnumBitfieldStruct<u8, Clksel1_SPEC>;
    impl Clksel1 {
        #[doc = "00 No clock        supplied"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 The        asynchronous clock source is switched on"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 The        synchronous clock source is switched on"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Both clock        sources are switched on  according        to Bosch this is possible"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Clksel2_SPEC;
    pub type Clksel2 = crate::EnumBitfieldStruct<u8, Clksel2_SPEC>;
    impl Clksel2 {
        #[doc = "00 No clock        supplied"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 The        asynchronous clock source is switched on"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 The        synchronous clock source is switched on"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Both clock        sources are switched on  according        to Bosch this is possible"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Clksel3_SPEC;
    pub type Clksel3 = crate::EnumBitfieldStruct<u8, Clksel3_SPEC>;
    impl Clksel3 {
        #[doc = "00 No clock        supplied"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 The        asynchronous clock source is switched on"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 The        synchronous clock source is switched on"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Both clock        sources are switched on  according        to Bosch this is possible"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Ci_SPEC;
    pub type Ci = crate::EnumBitfieldStruct<u8, Ci_SPEC>;
    impl Ci {
        #[doc = "0 Change Init        disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Change Init        enabled  takes effect with CCCE  1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ccce_SPEC;
    pub type Ccce = crate::EnumBitfieldStruct<u8, Ccce_SPEC>;
    impl Ccce {
        #[doc = "0 Clock and RAM        Change disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clock and RAM        Change enabled  takes effect with CI  1"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mecr_SPEC;
impl crate::sealed::RegSpec for Mecr_SPEC {
    type DataType = u32;
}
#[doc = "Measure Control Register\n resetvalue={Application Reset:0x0}"]
pub type Mecr = crate::RegValueT<Mecr_SPEC>;

impl Mecr {
    #[doc = "Threshold   TH. This bit field contains the threshold value for the measurement timer.        If TH   0000   the timer is stopped and        the capture function is disabled."]
    #[inline(always)]
    pub fn th(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Mecr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Mecr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Node Pointer   INP. INP selects the interrupt output line INT Om  m   0 15  for a capture        event interrupt."]
    #[inline(always)]
    pub fn inp(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Mecr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Mecr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Node   NODE. This bit field determines the CAN node i whose input line RXDCANi is        used for start and capture of the measurement timer."]
    #[inline(always)]
    pub fn node(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, Mecr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x7,1,0,u8, Mecr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Any Edge   ANYED. This bit enables capture on any edge of CAN input line specified by NODE."]
    #[inline(always)]
    pub fn anyed(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, mecr::Anyed, Mecr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,mecr::Anyed, Mecr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Capture Event Interrupt Enable   CAPEIE. This bit enables the capture event interrupt. Bit field INP selects the interrupt output line which becomes activated at this type of interrupt."]
    #[inline(always)]
    pub fn capeie(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, mecr::Capeie, Mecr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,mecr::Capeie, Mecr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Digital Glitch Filter Depth   DEPTH. DEPTH determines the number of input samples clocked with f SYNi that are taken into account for the calculation of the floating average.        The higher DEPTH is chosen to be  the longer the glitches that are        suppressed and the longer the delay of the input signal introduced by        this filter."]
    #[inline(always)]
    pub fn depth(
        self,
    ) -> crate::common::RegisterField<27, 0x7, 1, 0, mecr::Depth, Mecr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x7,1,0,mecr::Depth, Mecr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Start Of Frame   SOF. This bit selects falling edge or any edge as measurement for start of        frame detection."]
    #[inline(always)]
    pub fn sof(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, mecr::Sof, Mecr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x1,1,0,mecr::Sof, Mecr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Mecr {
    #[inline(always)]
    fn default() -> Mecr {
        <crate::RegValueT<Mecr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mecr {
    pub struct Anyed_SPEC;
    pub type Anyed = crate::EnumBitfieldStruct<u8, Anyed_SPEC>;
    impl Anyed {
        #[doc = "0 Capture on        falling  dominant  edge only"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Capture on        rising  recessive  or falling  dominant  edge"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Capeie_SPEC;
    pub type Capeie = crate::EnumBitfieldStruct<u8, Capeie_SPEC>;
    impl Capeie {
        #[doc = "0 Capture event        interrupt is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Capture event        interrupt is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Depth_SPEC;
    pub type Depth = crate::EnumBitfieldStruct<u8, Depth_SPEC>;
    impl Depth {
        #[doc = "000 off  default"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 Filter depth        of 8 cycles"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 Filter depth        of 16 cycles"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 Filter depth        of 32 cycles"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 Filter depth        of 64 cycles"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 Filter depth        of 128 cycles"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 Filter depth        of 255 cycles"]
        pub const CONST_66: Self = Self::new(6);
    }
    pub struct Sof_SPEC;
    pub type Sof = crate::EnumBitfieldStruct<u8, Sof_SPEC>;
    impl Sof {
        #[doc = "0 Measurement starts with any falling edge"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Measurement starts with falling Start of Frame edge. i.e any falling edge that occurs while the CAN node is in idle state"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mestat_SPEC;
impl crate::sealed::RegSpec for Mestat_SPEC {
    type DataType = u32;
}
#[doc = "Measure Status Register\n resetvalue={Application Reset:0x0}"]
pub type Mestat = crate::RegValueT<Mestat_SPEC>;

impl Mestat {
    #[doc = "Captured Timer   CAPT. This bit field contains the captured measurement timer content. The timer itself is cleared and started by the first falling  dominant         edge of a CAN frame on the input line of the CAN node specified by        MECR.NODE. The timer is incremented by the module control clock f SYNi and will be stopped when FFFF is        reached. If MECR.TH   0000   the timer is        always stopped. A capture will take place if all the following conditions are met  MECR.TH  gt  0000 Timer is cleared and started by new frame Timer reaches MECR.TH This node is not sending and first edge  as specified by MECR.ANYED           after 3. occurs on input line Capture will be repeated for the following CAN frames until MECR.TH is        cleared."]
    #[inline(always)]
    pub fn capt(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Mestat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Mestat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Captured Rising Edge   CAPRED. This bit indicates the type of edge that caused the last capture event."]
    #[inline(always)]
    pub fn capred(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, mestat::Capred, Mestat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x1,1,0,mestat::Capred, Mestat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Capture Event   CAPE. This flag is set on a capture event. It must be reset by software. An interrupt request is generated if MECR.CAPEIE   1. If CAPE 1 then no further measurement results are posted to MESTAT.CAPT and MESTAT.CAPRED. CAPE bit has to be cleared to re enable update of MESTAT.CAPT and MESTAT.CAPRED."]
    #[inline(always)]
    pub fn cape(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, mestat::Cape, Mestat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,mestat::Cape, Mestat_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Mestat {
    #[inline(always)]
    fn default() -> Mestat {
        <crate::RegValueT<Mestat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mestat {
    pub struct Capred_SPEC;
    pub type Capred = crate::EnumBitfieldStruct<u8, Capred_SPEC>;
    impl Capred {
        #[doc = "0 Capture occurred on falling  dominant  edge"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Capture occurred on rising  recessive  edge"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cape_SPEC;
    pub type Cape = crate::EnumBitfieldStruct<u8, Cape_SPEC>;
    impl Cape {
        #[doc = "0 No capture event has occurred since last flag reset"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Capture event has occurred since last flag reset"]
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
    #[doc = "TGS  TGB Write Protection   TG P. TGS and TGB are only written when TG P is 1  otherwise unchanged. Read as 0."]
    #[inline(always)]
    pub fn tg_p(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ocs_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ocs_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "OCDS Suspend Control   SUS. Controls the sensitivity to the suspend signal coming from the OCDS Trigger Switch  OTGS"]
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
        #[doc = "0 No Trigger Set output"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 TS16 CAN"]
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
        #[doc = "0 Will not suspend"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Hard suspend. Clock is off immediately. Do not use this mode in normal CAN applications  this mode is meant for debugging the peripheral IP."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Soft suspend of CAN nodes."]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Sussta_SPEC;
    pub type Sussta = crate::EnumBitfieldStruct<u8, Sussta_SPEC>;
    impl Sussta {
        #[doc = "0 CAN nodes are not  yet  suspended"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 All CAN nodes are suspended"]
        pub const CONST_11: Self = Self::new(1);
    }
}

#[doc = "N"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct N(pub(super) *mut u8);
unsafe impl core::marker::Send for N {}
unsafe impl core::marker::Sync for N {}
impl N {
    #[doc = "Access Enable Register CAN Node 0 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accennodei0(&self) -> crate::common::Reg<n::AccennodEi0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "CC Control Register 0\n resetvalue={Application Reset:0x1}"]
    #[inline(always)]
    pub const fn cccri(&self) -> crate::common::Reg<n::CccRi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(280usize)) }
    }
    #[doc = "Core Release Register 0\n resetvalue={Application Reset:0x32150320,Application Reset:0x32150323,Application Reset:0x32150320}"]
    #[inline(always)]
    pub const fn creli(&self) -> crate::common::Reg<n::CreLi_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize)) }
    }
    #[doc = "Data Bit Timing   Prescaler Register 0\n resetvalue={Application Reset:0x0A33}"]
    #[inline(always)]
    pub const fn dbtpi(&self) -> crate::common::Reg<n::DbtPi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(268usize)) }
    }
    #[doc = "Error Counter Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ecri(&self) -> crate::common::Reg<n::EcRi_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(320usize)) }
    }
    #[doc = "End Address Node 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn endadri(&self) -> crate::common::Reg<n::EndadRi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Endian Register 0\n resetvalue={Application Reset:0x087654321}"]
    #[inline(always)]
    pub const fn endni(&self) -> crate::common::Reg<n::EndNi_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
    }
    #[doc = "Global Filter Configuration 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gfci(&self) -> crate::common::Reg<n::GfCi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(384usize)) }
    }
    #[doc = "Interrupt routing for Groups 1 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn grint1i(&self) -> crate::common::Reg<n::Grint1I_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Interrupt routing for Groups 2 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn grint2i(&self) -> crate::common::Reg<n::Grint2I_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "High Priority Message Status 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn hpmsi(&self) -> crate::common::Reg<n::HpmSi_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(404usize)) }
    }
    #[doc = "Interrupt Enable 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn iei(&self) -> crate::common::Reg<n::IEi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(340usize)) }
    }
    #[doc = "Interrupt Line Enable 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ilei(&self) -> crate::common::Reg<n::IlEi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(348usize)) }
    }
    #[doc = "Interrupt Line Select 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ilsi(&self) -> crate::common::Reg<n::IlSi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(344usize)) }
    }
    #[doc = "Interrupt Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn iri(&self) -> crate::common::Reg<n::IRi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(336usize)) }
    }
    #[doc = "Interrupt Signalling Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn isregi(&self) -> crate::common::Reg<n::IsreGi_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Nominal Bit Timing   Prescaler Register 0\n resetvalue={Application Reset:0x6000A03}"]
    #[inline(always)]
    pub const fn nbtpi(&self) -> crate::common::Reg<n::NbtPi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(284usize)) }
    }
    #[doc = "New Data 1 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ndat1i(&self) -> crate::common::Reg<n::Ndat1I_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(408usize)) }
    }
    #[doc = "New Data 2 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ndat2i(&self) -> crate::common::Reg<n::Ndat2I_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(412usize)) }
    }
    #[doc = "Node 0 Port Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn npcri(&self) -> crate::common::Reg<n::NpcRi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }
    #[doc = "Protocol Status Register 0\n resetvalue={Application Reset:0x707}"]
    #[inline(always)]
    pub const fn psri(&self) -> crate::common::Reg<n::PsRi_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(324usize)) }
    }
    #[doc = "RAM Watchdog 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rwdi(&self) -> crate::common::Reg<n::RwDi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(276usize)) }
    }
    #[doc = "Standard ID Filter Configuration 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sidfci(&self) -> crate::common::Reg<n::SidfCi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(388usize)) }
    }
    #[doc = "Start Address Node 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn startadri(&self) -> crate::common::Reg<n::StartadRi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Transmitter Delay Compensation Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tdcri(&self) -> crate::common::Reg<n::TdcRi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(328usize)) }
    }
    #[doc = "Test Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn testi(&self) -> crate::common::Reg<n::TesTi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(272usize)) }
    }
    #[doc = "Timeout Counter Configuration 0\n resetvalue={Application Reset:0x0FFFF0000}"]
    #[inline(always)]
    pub const fn tocci(&self) -> crate::common::Reg<n::TocCi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(296usize)) }
    }
    #[doc = "Timeout Counter Value 0\n resetvalue={Application Reset:0x0FFFF}"]
    #[inline(always)]
    pub const fn tocvi(&self) -> crate::common::Reg<n::TocVi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(300usize)) }
    }
    #[doc = "Timestamp Counter Configuration 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tscci(&self) -> crate::common::Reg<n::TscCi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(288usize)) }
    }
    #[doc = "Timestamp Counter Value 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tscvi(&self) -> crate::common::Reg<n::TscVi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(292usize)) }
    }
    #[doc = "Time Trigger Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ttcri(&self) -> crate::common::Reg<n::TtcRi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(240usize)) }
    }
    #[doc = "Extended ID AND Mask 0\n resetvalue={Application Reset:0x1FFFFFFF}"]
    #[inline(always)]
    pub const fn xidami(&self) -> crate::common::Reg<n::XidaMi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(400usize)) }
    }
    #[doc = "Extended ID Filter Configuration 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn xidfci(&self) -> crate::common::Reg<n::XidfCi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(392usize)) }
    }
    #[doc = "NT"]
    #[inline(always)]
    pub fn nt(self) -> n::Nt {
        unsafe { n::Nt(self.0.add(32usize)) }
    }
    #[doc = "RX"]
    #[inline(always)]
    pub fn rx(self) -> n::Rx {
        unsafe { n::Rx(self.0.add(416usize)) }
    }
    #[doc = "TT"]
    #[inline(always)]
    pub fn tt(self) -> n::Tt {
        unsafe { n::Tt(self.0.add(512usize)) }
    }
    #[doc = "TX"]
    #[inline(always)]
    pub fn tx(self) -> n::Tx {
        unsafe { n::Tx(self.0.add(448usize)) }
    }
}
pub mod n {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AccennodEi0_SPEC;
    impl crate::sealed::RegSpec for AccennodEi0_SPEC {
        type DataType = u32;
    }
    #[doc = "Access Enable Register CAN Node 0 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type AccennodEi0 = crate::RegValueT<AccennodEi0_SPEC>;

    impl AccennodEi0 {
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en0(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            accennodei0::En0,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                accennodei0::En0,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en1(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            accennodei0::En1,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                accennodei0::En1,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en2(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            accennodei0::En2,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                accennodei0::En2,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en3(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            accennodei0::En3,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                accennodei0::En3,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en4(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            accennodei0::En4,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                accennodei0::En4,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en5(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            accennodei0::En5,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                accennodei0::En5,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en6(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x1,
            1,
            0,
            accennodei0::En6,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                6,
                0x1,
                1,
                0,
                accennodei0::En6,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en7(
            self,
        ) -> crate::common::RegisterField<
            7,
            0x1,
            1,
            0,
            accennodei0::En7,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                accennodei0::En7,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en8(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1,
            1,
            0,
            accennodei0::En8,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                accennodei0::En8,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en9(
            self,
        ) -> crate::common::RegisterField<
            9,
            0x1,
            1,
            0,
            accennodei0::En9,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                9,
                0x1,
                1,
                0,
                accennodei0::En9,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en10(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            accennodei0::En10,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                accennodei0::En10,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en11(
            self,
        ) -> crate::common::RegisterField<
            11,
            0x1,
            1,
            0,
            accennodei0::En11,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                accennodei0::En11,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en12(
            self,
        ) -> crate::common::RegisterField<
            12,
            0x1,
            1,
            0,
            accennodei0::En12,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                12,
                0x1,
                1,
                0,
                accennodei0::En12,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en13(
            self,
        ) -> crate::common::RegisterField<
            13,
            0x1,
            1,
            0,
            accennodei0::En13,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                13,
                0x1,
                1,
                0,
                accennodei0::En13,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en14(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x1,
            1,
            0,
            accennodei0::En14,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                accennodei0::En14,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en15(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            accennodei0::En15,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                accennodei0::En15,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en16(
            self,
        ) -> crate::common::RegisterField<
            16,
            0x1,
            1,
            0,
            accennodei0::En16,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0x1,
                1,
                0,
                accennodei0::En16,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en17(
            self,
        ) -> crate::common::RegisterField<
            17,
            0x1,
            1,
            0,
            accennodei0::En17,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                17,
                0x1,
                1,
                0,
                accennodei0::En17,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en18(
            self,
        ) -> crate::common::RegisterField<
            18,
            0x1,
            1,
            0,
            accennodei0::En18,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                18,
                0x1,
                1,
                0,
                accennodei0::En18,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en19(
            self,
        ) -> crate::common::RegisterField<
            19,
            0x1,
            1,
            0,
            accennodei0::En19,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                19,
                0x1,
                1,
                0,
                accennodei0::En19,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en20(
            self,
        ) -> crate::common::RegisterField<
            20,
            0x1,
            1,
            0,
            accennodei0::En20,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0x1,
                1,
                0,
                accennodei0::En20,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en21(
            self,
        ) -> crate::common::RegisterField<
            21,
            0x1,
            1,
            0,
            accennodei0::En21,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                21,
                0x1,
                1,
                0,
                accennodei0::En21,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en22(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x1,
            1,
            0,
            accennodei0::En22,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                22,
                0x1,
                1,
                0,
                accennodei0::En22,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en23(
            self,
        ) -> crate::common::RegisterField<
            23,
            0x1,
            1,
            0,
            accennodei0::En23,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                23,
                0x1,
                1,
                0,
                accennodei0::En23,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en24(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x1,
            1,
            0,
            accennodei0::En24,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0x1,
                1,
                0,
                accennodei0::En24,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en25(
            self,
        ) -> crate::common::RegisterField<
            25,
            0x1,
            1,
            0,
            accennodei0::En25,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                25,
                0x1,
                1,
                0,
                accennodei0::En25,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en26(
            self,
        ) -> crate::common::RegisterField<
            26,
            0x1,
            1,
            0,
            accennodei0::En26,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                26,
                0x1,
                1,
                0,
                accennodei0::En26,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en27(
            self,
        ) -> crate::common::RegisterField<
            27,
            0x1,
            1,
            0,
            accennodei0::En27,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                27,
                0x1,
                1,
                0,
                accennodei0::En27,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en28(
            self,
        ) -> crate::common::RegisterField<
            28,
            0x1,
            1,
            0,
            accennodei0::En28,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                28,
                0x1,
                1,
                0,
                accennodei0::En28,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en29(
            self,
        ) -> crate::common::RegisterField<
            29,
            0x1,
            1,
            0,
            accennodei0::En29,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                29,
                0x1,
                1,
                0,
                accennodei0::En29,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en30(
            self,
        ) -> crate::common::RegisterField<
            30,
            0x1,
            1,
            0,
            accennodei0::En30,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                30,
                0x1,
                1,
                0,
                accennodei0::En30,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
        #[inline(always)]
        pub fn en31(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            accennodei0::En31,
            AccennodEi0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                accennodei0::En31,
                AccennodEi0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for AccennodEi0 {
        #[inline(always)]
        fn default() -> AccennodEi0 {
            <crate::RegValueT<AccennodEi0_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
    pub mod accennodei0 {
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
    pub struct CccRi_SPEC;
    impl crate::sealed::RegSpec for CccRi_SPEC {
        type DataType = u32;
    }
    #[doc = "CC Control Register 0\n resetvalue={Application Reset:0x1}"]
    pub type CccRi = crate::RegValueT<CccRi_SPEC>;

    impl CccRi {
        #[doc = "Initialization   INIT. Due to the synchronization mechanism between the two clock domains           there may be a delay until the value written to INIT can be read back.          Therefore the programmer has to assure that the previous value written          to INIT has been accepted by reading INIT before setting INIT to a new          value."]
        #[inline(always)]
        pub fn init(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, cccri::Init, CccRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1,1,0,cccri::Init, CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration Change Enable   CCE"]
        #[inline(always)]
        pub fn cce(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, cccri::Cce, CccRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0x1,1,0,cccri::Cce, CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Restricted Operation Mode   ASM. Bit ASM can only be set by the Host when both CCE and INIT are set to          8216 1  8217 . In can also be set by the M CAN .        The bit can be reset by the Host at any time. For a description of the        Restricted Operation Mode see paragraph Restricted Operation Mode."]
        #[inline(always)]
        pub fn asm(
            self,
        ) -> crate::common::RegisterField<2, 0x1, 1, 0, cccri::Asm, CccRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x1,1,0,cccri::Asm, CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Clock Stop Acknowledge   CSA"]
        #[inline(always)]
        pub fn csa(
            self,
        ) -> crate::common::RegisterField<3, 0x1, 1, 0, cccri::Csa, CccRi_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<3,0x1,1,0,cccri::Csa, CccRi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Clock Stop Request   CSR"]
        #[inline(always)]
        pub fn csr(
            self,
        ) -> crate::common::RegisterField<4, 0x1, 1, 0, cccri::Csr, CccRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x1,1,0,cccri::Csr, CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Bus Monitoring Mode   MON. Bit MON can only be set by the Host when both CCE and INIT are set to          8216 1  8217 . The bit can be reset by the Host at any time. The bus monitoring mode corresponds to the Analyzer Mode of the MultiCAN        module."]
        #[inline(always)]
        pub fn mon(
            self,
        ) -> crate::common::RegisterField<5, 0x1, 1, 0, cccri::Mon, CccRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x1,1,0,cccri::Mon, CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Disable Automatic Retransmission   DAR. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set."]
        #[inline(always)]
        pub fn dar(
            self,
        ) -> crate::common::RegisterField<6, 0x1, 1, 0, cccri::Dar, CccRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<6,0x1,1,0,cccri::Dar, CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Test Mode Enable   TEST. The TEST register can only be set  if CCE  INIT and TEST are set. Writes to test will only have effect  if all three bits are set."]
        #[inline(always)]
        pub fn test(
            self,
        ) -> crate::common::RegisterField<7, 0x1, 1, 0, cccri::Test, CccRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<7,0x1,1,0,cccri::Test, CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "FD Operation Enable   FDOE. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set."]
        #[inline(always)]
        pub fn fdoe(
            self,
        ) -> crate::common::RegisterField<8, 0x1, 1, 0, cccri::Fdoe, CccRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x1,1,0,cccri::Fdoe, CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Bit Rate Switch Enable   BRSE. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set."]
        #[inline(always)]
        pub fn brse(
            self,
        ) -> crate::common::RegisterField<9, 0x1, 1, 0, cccri::Brse, CccRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<9,0x1,1,0,cccri::Brse, CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Protocol Exception Handling Disable   PXHD. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
        #[inline(always)]
        pub fn pxhd(
            self,
        ) -> crate::common::RegisterField<12, 0x1, 1, 0, cccri::Pxhd, CccRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0x1,1,0,cccri::Pxhd, CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Edge Filtering during Bus Integration   EFBI. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
        #[inline(always)]
        pub fn efbi(
            self,
        ) -> crate::common::RegisterField<13, 0x1, 1, 0, cccri::Efbi, CccRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<13,0x1,1,0,cccri::Efbi, CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Transmit Pause   TXP. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. If this bit is set  the M CAN pauses for two CAN bit times before starting the next transmission after        itself has successfully transmitted a frame  see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn txp(
            self,
        ) -> crate::common::RegisterField<14, 0x1, 1, 0, cccri::Txp, CccRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<14,0x1,1,0,cccri::Txp, CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Non ISO Operation   NISO. If this bit is set  the M CAN uses the CAN FD frame format as specified        by the Bosch CAN FD Specification V1.0."]
        #[inline(always)]
        pub fn niso(
            self,
        ) -> crate::common::RegisterField<15, 0x1, 1, 0, cccri::Niso, CccRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<15,0x1,1,0,cccri::Niso, CccRi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for CccRi {
        #[inline(always)]
        fn default() -> CccRi {
            <crate::RegValueT<CccRi_SPEC> as RegisterValue<_>>::new(1)
        }
    }
    pub mod cccri {
        pub struct Init_SPEC;
        pub type Init = crate::EnumBitfieldStruct<u8, Init_SPEC>;
        impl Init {
            #[doc = "0 Normal Operation"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Initialization is started"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cce_SPEC;
        pub type Cce = crate::EnumBitfieldStruct<u8, Cce_SPEC>;
        impl Cce {
            #[doc = "0 The CPU has no write access to the protected configuration registers"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 The CPU has write access to the protected configuration registers  while CCCR.INIT    1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Asm_SPEC;
        pub type Asm = crate::EnumBitfieldStruct<u8, Asm_SPEC>;
        impl Asm {
            #[doc = "0 Normal CAN operation"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Restricted Operation Mode active"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Csa_SPEC;
        pub type Csa = crate::EnumBitfieldStruct<u8, Csa_SPEC>;
        impl Csa {
            #[doc = "0 No clock stop acknowledged"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 M CAN may be set in power down by stopping the synchronous and the asynchronous clock source"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Csr_SPEC;
        pub type Csr = crate::EnumBitfieldStruct<u8, Csr_SPEC>;
        impl Csr {
            #[doc = "0 No clock stop is requested"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Clock stop requested. When clock stop is requested  first INIT and then CSA will be set after all pending transfer requests have been completed and the CAN bus reached idle."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Mon_SPEC;
        pub type Mon = crate::EnumBitfieldStruct<u8, Mon_SPEC>;
        impl Mon {
            #[doc = "0 Bus Monitoring Mode is disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Bus Monitoring Mode is enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Dar_SPEC;
        pub type Dar = crate::EnumBitfieldStruct<u8, Dar_SPEC>;
        impl Dar {
            #[doc = "0 Automatic retransmission of messages not transmitted successfully enabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Automatic retransmission disabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Test_SPEC;
        pub type Test = crate::EnumBitfieldStruct<u8, Test_SPEC>;
        impl Test {
            #[doc = "0 Normal operation  register TEST holds reset values"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Test Mode  write access to register TEST enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Fdoe_SPEC;
        pub type Fdoe = crate::EnumBitfieldStruct<u8, Fdoe_SPEC>;
        impl Fdoe {
            #[doc = "0 CAN FD frame format disabled."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 CAN FD frame format enabled."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Brse_SPEC;
        pub type Brse = crate::EnumBitfieldStruct<u8, Brse_SPEC>;
        impl Brse {
            #[doc = "0 Bit rate switching for transmission disabled."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Bit rate switching for transmission enabled."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Pxhd_SPEC;
        pub type Pxhd = crate::EnumBitfieldStruct<u8, Pxhd_SPEC>;
        impl Pxhd {
            #[doc = "0 Protocol exception handling enabled."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Protocol exception handling disabled.   When protocol exception handling is disabled  the M CAN will transmit an error frame when it detects a protocol exception condition."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Efbi_SPEC;
        pub type Efbi = crate::EnumBitfieldStruct<u8, Efbi_SPEC>;
        impl Efbi {
            #[doc = "0 Edge filter        disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Two consecutive        dominant tq required to detect an edge for hard synchronization."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Txp_SPEC;
        pub type Txp = crate::EnumBitfieldStruct<u8, Txp_SPEC>;
        impl Txp {
            #[doc = "0 Transmit pause disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Transmit pause enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Niso_SPEC;
        pub type Niso = crate::EnumBitfieldStruct<u8, Niso_SPEC>;
        impl Niso {
            #[doc = "0 CAN FD frame format according to ISO11898 1"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 CAN FD frame format according to Bosch CAN FD Specification V1.0"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CreLi_SPEC;
    impl crate::sealed::RegSpec for CreLi_SPEC {
        type DataType = u32;
    }
    #[doc = "Core Release Register 0\n resetvalue={Application Reset:0x32150320,Application Reset:0x32150323,Application Reset:0x32150320}"]
    pub type CreLi = crate::RegValueT<CreLi_SPEC>;

    impl CreLi {
        #[doc = "Sub step of Core Release   SUBSTEP. One digit  BCD coded."]
        #[inline(always)]
        pub fn substep(
            self,
        ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, CreLi_SPEC, crate::common::R> {
            crate::common::RegisterField::<20,0xf,1,0,u8, CreLi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Step of Core Release   STEP. One digit  BCD coded."]
        #[inline(always)]
        pub fn step(
            self,
        ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, CreLi_SPEC, crate::common::R> {
            crate::common::RegisterField::<24,0xf,1,0,u8, CreLi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Core Release   REL. One digit  BCD coded."]
        #[inline(always)]
        pub fn rel(
            self,
        ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, CreLi_SPEC, crate::common::R> {
            crate::common::RegisterField::<28,0xf,1,0,u8, CreLi_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for CreLi {
        #[inline(always)]
        fn default() -> CreLi {
            <crate::RegValueT<CreLi_SPEC> as RegisterValue<_>>::new(840237856)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbtPi_SPEC;
    impl crate::sealed::RegSpec for DbtPi_SPEC {
        type DataType = u32;
    }
    #[doc = "Data Bit Timing   Prescaler Register 0\n resetvalue={Application Reset:0x0A33}"]
    pub type DbtPi = crate::RegValueT<DbtPi_SPEC>;

    impl DbtPi {
        #[doc = "Data  Re  Synchronization Jump Width   DSJW. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Valid values are 0 to 15. The actual interpretation by the hardware of        this value is such that one more than the value programmed here is used."]
        #[inline(always)]
        pub fn dsjw(
            self,
        ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, DbtPi_SPEC, crate::common::RW> {
            crate::common::RegisterField::<0,0xf,1,0,u8, DbtPi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Data time segment after sample point   DTSEG2. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Valid values are 0 to 15. The actual interpretation by the hardware of        this value is such that one more than the programmed value is used."]
        #[inline(always)]
        pub fn dtseg2(
            self,
        ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, DbtPi_SPEC, crate::common::RW> {
            crate::common::RegisterField::<4,0xf,1,0,u8, DbtPi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Data time segment before sample point   DTSEG1. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Valid values are 0 to 31. The actual interpretation by the hardware of        this value is such that one more than the programmed value is used."]
        #[inline(always)]
        pub fn dtseg1(
            self,
        ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, DbtPi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x1f,1,0,u8, DbtPi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Data Baud Rate Prescaler   DBRP. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. The value by which the oscillator frequency is divided for generating        the bit time quanta. The bit time is built up from a multiple of this        quanta. Valid values for the Baud Rate Prescaler are   8201 0 to 31. The        actual interpretation by the hardware of this value is such that one        more than the value programmed here is used."]
        #[inline(always)]
        pub fn dbrp(
            self,
        ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, DbtPi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1f,1,0,u8, DbtPi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Transmitter Delay Compensation   TDC. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
        #[inline(always)]
        pub fn tdc(
            self,
        ) -> crate::common::RegisterField<23, 0x1, 1, 0, dbtpi::Tdc, DbtPi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<23,0x1,1,0,dbtpi::Tdc, DbtPi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for DbtPi {
        #[inline(always)]
        fn default() -> DbtPi {
            <crate::RegValueT<DbtPi_SPEC> as RegisterValue<_>>::new(2611)
        }
    }
    pub mod dbtpi {
        pub struct Tdc_SPEC;
        pub type Tdc = crate::EnumBitfieldStruct<u8, Tdc_SPEC>;
        impl Tdc {
            #[doc = "0 Transmitter Delay Compensation disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Transmitter Delay Compensation enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EcRi_SPEC;
    impl crate::sealed::RegSpec for EcRi_SPEC {
        type DataType = u32;
    }
    #[doc = "Error Counter Register 0\n resetvalue={Application Reset:0x0}"]
    pub type EcRi = crate::RegValueT<EcRi_SPEC>;

    impl EcRi {
        #[doc = "Transmit Error Counter   TEC. Actual state of the Transmit Error Counter  values between 0 and 255 When CCCR.ASM is set  the CAN protocol controller does not increment          TEC and REC when a CAN protocol error is detected  but CEL is still          incremented."]
        #[inline(always)]
        pub fn tec(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, EcRi_SPEC, crate::common::R> {
            crate::common::RegisterField::<0,0xff,1,0,u8, EcRi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Receive Error Counter   REC. Actual state of the Receive Error Counter  values between 0 and 127 When CCCR.ASM is set  the CAN protocol controller does not increment          TEC and REC when a CAN protocol error is detected  but CEL is still          incremented."]
        #[inline(always)]
        pub fn rec(
            self,
        ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, EcRi_SPEC, crate::common::R> {
            crate::common::RegisterField::<8,0x7f,1,0,u8, EcRi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Receive Error Passive   RP"]
        #[inline(always)]
        pub fn rp(
            self,
        ) -> crate::common::RegisterField<15, 0x1, 1, 0, ecri::Rp, EcRi_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<15,0x1,1,0,ecri::Rp, EcRi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "CAN Error Logging   CEL. The counter is incremented each time when a CAN protocol error causes        the Transmit Error Counter or the Receive Error Counter to be        incremented. It is reset by read access to CEL. The counter stops at        0xFF  the next increment of TEC or REC sets interrupt flag IR.ELO. The counter is reset on read  if the bit NPCRi.DELE is set for the node."]
        #[inline(always)]
        pub fn cel(
            self,
        ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, EcRi_SPEC, crate::common::R> {
            crate::common::RegisterField::<16,0xff,1,0,u8, EcRi_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for EcRi {
        #[inline(always)]
        fn default() -> EcRi {
            <crate::RegValueT<EcRi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod ecri {
        pub struct Rp_SPEC;
        pub type Rp = crate::EnumBitfieldStruct<u8, Rp_SPEC>;
        impl Rp {
            #[doc = "0 The Receive Error Counter is below the error passive level of 128"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 The Receive Error Counter has reached the error passive level of 128"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EndadRi_SPEC;
    impl crate::sealed::RegSpec for EndadRi_SPEC {
        type DataType = u32;
    }
    #[doc = "End Address Node 0\n resetvalue={Application Reset:0x0}"]
    pub type EndadRi = crate::RegValueT<EndadRi_SPEC>;

    impl EndadRi {
        #[doc = "Message RAM end   END. The address within the RAM area of the MCMCAN          of node i  where the message RAM to be protected ends"]
        #[inline(always)]
        pub fn end(
            self,
        ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, EndadRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x3fff,1,0,u16, EndadRi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for EndadRi {
        #[inline(always)]
        fn default() -> EndadRi {
            <crate::RegValueT<EndadRi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EndNi_SPEC;
    impl crate::sealed::RegSpec for EndNi_SPEC {
        type DataType = u32;
    }
    #[doc = "Endian Register 0\n resetvalue={Application Reset:0x087654321}"]
    pub type EndNi = crate::RegValueT<EndNi_SPEC>;

    impl EndNi {
        #[doc = "Endianness Test Value   ETV. The endianness test value is 0x87654321."]
        #[inline(always)]
        pub fn etv(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, EndNi_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, EndNi_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for EndNi {
        #[inline(always)]
        fn default() -> EndNi {
            <crate::RegValueT<EndNi_SPEC> as RegisterValue<_>>::new(2271560481)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GfCi_SPEC;
    impl crate::sealed::RegSpec for GfCi_SPEC {
        type DataType = u32;
    }
    #[doc = "Global Filter Configuration 0\n resetvalue={Application Reset:0x0}"]
    pub type GfCi = crate::RegValueT<GfCi_SPEC>;

    impl GfCi {
        #[doc = "Reject Remote Frames Extended   RRFE. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set."]
        #[inline(always)]
        pub fn rrfe(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, gfci::Rrfe, GfCi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1,1,0,gfci::Rrfe, GfCi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Reject Remote Frames Standard   RRFS. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set."]
        #[inline(always)]
        pub fn rrfs(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, gfci::Rrfs, GfCi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0x1,1,0,gfci::Rrfs, GfCi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Accept Non matching Frames Extended   ANFE. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set. Defines how received messages with 29 bit IDs that do not match any element of the filter list are treated."]
        #[inline(always)]
        pub fn anfe(
            self,
        ) -> crate::common::RegisterField<2, 0x3, 1, 0, gfci::Anfe, GfCi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x3,1,0,gfci::Anfe, GfCi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Accept Non matching Frames Standard   ANFS. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set. Defines how received messages with 11 bit IDs that do not match any element of the filter list are treated."]
        #[inline(always)]
        pub fn anfs(
            self,
        ) -> crate::common::RegisterField<4, 0x3, 1, 0, gfci::Anfs, GfCi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x3,1,0,gfci::Anfs, GfCi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GfCi {
        #[inline(always)]
        fn default() -> GfCi {
            <crate::RegValueT<GfCi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod gfci {
        pub struct Rrfe_SPEC;
        pub type Rrfe = crate::EnumBitfieldStruct<u8, Rrfe_SPEC>;
        impl Rrfe {
            #[doc = "0 Filter remote frames with 29 bit extended IDs"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Reject all remote frames with 29 bit extended IDs"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rrfs_SPEC;
        pub type Rrfs = crate::EnumBitfieldStruct<u8, Rrfs_SPEC>;
        impl Rrfs {
            #[doc = "0 Filter remote frames with 11 bit standard IDs"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Reject all remote frames with 11 bit standard IDs"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Anfe_SPEC;
        pub type Anfe = crate::EnumBitfieldStruct<u8, Anfe_SPEC>;
        impl Anfe {
            #[doc = "00 Accept in Rx FIFO 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Accept in Rx FIFO 1"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Reject"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Reject"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Anfs_SPEC;
        pub type Anfs = crate::EnumBitfieldStruct<u8, Anfs_SPEC>;
        impl Anfs {
            #[doc = "00 Accept in Rx FIFO 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Accept in Rx FIFO 1"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Reject"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Reject"]
            pub const CONST_33: Self = Self::new(3);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Grint1I_SPEC;
    impl crate::sealed::RegSpec for Grint1I_SPEC {
        type DataType = u32;
    }
    #[doc = "Interrupt routing for Groups 1 0\n resetvalue={Application Reset:0x0}"]
    pub type Grint1I = crate::RegValueT<Grint1I_SPEC>;

    impl Grint1I {
        #[doc = "Transmit Event FIFO Incidents   TEFIFO. are mapped here. IR.TEFF  Transmit Event FIFO Full  and IR.TEFN         Transmit Event FIFO New Entry"]
        #[inline(always)]
        pub fn tefifo(
            self,
        ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Grint1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xf,1,0,u8, Grint1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "High Priority Events   HPE. are mapped here  giving IR.HPM an interrupt level"]
        #[inline(always)]
        pub fn hpe(
            self,
        ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Grint1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0xf,1,0,u8, Grint1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Watermark interrupts   WATI. are mapped here  IR.TEFW  Transmit FIFO warning interrupt reached          IR.RF1W  Receive FIFO 1 warning interrupt reached . IR.RF0W  Receive        FIFO 0 warning interrupt reached"]
        #[inline(always)]
        pub fn wati(
            self,
        ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Grint1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xf,1,0,u8, Grint1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "ALERTS   ALRT. All kind of alerts are mapped here. IR.EW  warning status   IR.EP  error        passive   IR.TSW  timestamp wrap around   IR.TEFL  Transmit Event FIFO        Element Lost   IR.RF0L  Receive FIFO 0 Message Lost   IR.RF1L  Receive        FIFO 1 Message Lost . The        following TTCAN error messages and warnings are also shown here  TTIR.        CER  Configuration Error   TTIR.AW Application Watchdog  TTIR.WT  Watch        Trigger   TTIR.IWT Initialization Watch Trigger  TTIR.ELC  Error Level        Changed   TTIR.SE2  Scheduling Error 2   TTIR.SE1  Scheduling Error          TTIR.TXO  Tx Count Overflow   TTIR.TXU  TX Count Underflow   TTIR.GTE         Global Time Error   TTIR.GTD  Global Time Discontinuity  and TTIR.GTW         Global Time Wrap"]
        #[inline(always)]
        pub fn alrt(
            self,
        ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Grint1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0xf,1,0,u8, Grint1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Module errors   MOER. IR.WDI  watchdog interrupt  and IR.MRAF  message RAM access failure  are        mapped here."]
        #[inline(always)]
        pub fn moer(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Grint1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, Grint1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Safety counter overflow   SAFE. The interrupt node for IR.ELO showing a safety counter overflow"]
        #[inline(always)]
        pub fn safe(
            self,
        ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Grint1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0xf,1,0,u8, Grint1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Bus Off has been reached   BOFF. Mapped to IRi.BO flag indication the change in Bus Off status. To get        out of bus off  the CCCRn.INIT bit has to be reset."]
        #[inline(always)]
        pub fn boff(
            self,
        ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Grint1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0xf,1,0,u8, Grint1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Last Error Interrupts   LOI. The interrupt sources IR.PED  Protocol Error in Data Phase  and IR.PEA         Protocol Error in Arbitration Phase  are signalled here."]
        #[inline(always)]
        pub fn loi(
            self,
        ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Grint1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0xf,1,0,u8, Grint1I_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Grint1I {
        #[inline(always)]
        fn default() -> Grint1I {
            <crate::RegValueT<Grint1I_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Grint2I_SPEC;
    impl crate::sealed::RegSpec for Grint2I_SPEC {
        type DataType = u32;
    }
    #[doc = "Interrupt routing for Groups 2 0\n resetvalue={Application Reset:0x0}"]
    pub type Grint2I = crate::RegValueT<Grint2I_SPEC>;

    impl Grint2I {
        #[doc = "Message stored in dedicated receive buffer interrupt  IR.DRX    REINT. is assigned to interrupt node."]
        #[inline(always)]
        pub fn reint(
            self,
        ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Grint2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xf,1,0,u8, Grint2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "IR.RF1F   RxF1F. Receive FIFO1 full interrupt assigned to an interrupt node"]
        #[inline(always)]
        pub fn rxf1f(
            self,
        ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Grint2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0xf,1,0,u8, Grint2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "IR.RF0F   RxF0F. Receive FIFO0 full interrupt assigned to an interrupt node"]
        #[inline(always)]
        pub fn rxf0f(
            self,
        ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Grint2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xf,1,0,u8, Grint2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "IR.RF1N   RxF1N. Receive FIFO1 new message assigned to an interrupt node"]
        #[inline(always)]
        pub fn rxf1n(
            self,
        ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Grint2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0xf,1,0,u8, Grint2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "IR.RF0N   RxF0N. Receive FIFO0 new message assigned to an interrupt node"]
        #[inline(always)]
        pub fn rxf0n(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Grint2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, Grint2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Receive Timeouts   RETI. can be assigned here. IR.TOO  time out event  and TE  Timer Event"]
        #[inline(always)]
        pub fn reti(
            self,
        ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Grint2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0xf,1,0,u8, Grint2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Transmission Queue Events   TRAQ. can be assigned here. IR.TFE Transmission FIFO Empty"]
        #[inline(always)]
        pub fn traq(
            self,
        ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Grint2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0xf,1,0,u8, Grint2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Interrupts of the transmission control   TRACO. can be assigned here. IR.TCF  Transmission Cancellation Finished  and        IR.TF  Transmission Completed . As        an additional information the copy of a local time event is shown here        with TTIR.SWT  Stop Watch Event . Further on the TTIR.TTMI Trigger Time        Event Internal  TTIR.RTMI  Register Time Mark   TTIR.SOG  Start of Gap          TTIR.CSM  Change of Synchronization Mode   TTIR.SMC  Start Matrix Cycle         and TTIR.SBC  Start of Basic Cycle  are shown here."]
        #[inline(always)]
        pub fn traco(
            self,
        ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, Grint2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0xf,1,0,u8, Grint2I_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Grint2I {
        #[inline(always)]
        fn default() -> Grint2I {
            <crate::RegValueT<Grint2I_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HpmSi_SPEC;
    impl crate::sealed::RegSpec for HpmSi_SPEC {
        type DataType = u32;
    }
    #[doc = "High Priority Message Status 0\n resetvalue={Application Reset:0x0}"]
    pub type HpmSi = crate::RegValueT<HpmSi_SPEC>;

    impl HpmSi {
        #[doc = "Buffer Index   BIDX. Index of Rx FIFO element to which the message was stored. Only valid when MSI 1     1 ."]
        #[inline(always)]
        pub fn bidx(
            self,
        ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, HpmSi_SPEC, crate::common::R> {
            crate::common::RegisterField::<0,0x3f,1,0,u8, HpmSi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Message Storage Indicator   MSI"]
        #[inline(always)]
        pub fn msi(
            self,
        ) -> crate::common::RegisterField<6, 0x3, 1, 0, hpmsi::Msi, HpmSi_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<6,0x3,1,0,hpmsi::Msi, HpmSi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Filter Index   FIDX. Index of matching filter element. Range is 0 to SIDFC.LSS  160    160 1        resp. XIDFC.LSE  160    160 1."]
        #[inline(always)]
        pub fn fidx(
            self,
        ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, HpmSi_SPEC, crate::common::R> {
            crate::common::RegisterField::<8,0x7f,1,0,u8, HpmSi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Filter List   FLST. Indicates the filter list of the matching filter element."]
        #[inline(always)]
        pub fn flst(
            self,
        ) -> crate::common::RegisterField<15, 0x1, 1, 0, hpmsi::Flst, HpmSi_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<15,0x1,1,0,hpmsi::Flst, HpmSi_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for HpmSi {
        #[inline(always)]
        fn default() -> HpmSi {
            <crate::RegValueT<HpmSi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod hpmsi {
        pub struct Msi_SPEC;
        pub type Msi = crate::EnumBitfieldStruct<u8, Msi_SPEC>;
        impl Msi {
            #[doc = "00 No FIFO selected"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 FIFO message lost"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Message stored in FIFO 0"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Message stored in FIFO 1"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Flst_SPEC;
        pub type Flst = crate::EnumBitfieldStruct<u8, Flst_SPEC>;
        impl Flst {
            #[doc = "0 Standard Filter List"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Extended Filter List"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IEi_SPEC;
    impl crate::sealed::RegSpec for IEi_SPEC {
        type DataType = u32;
    }
    #[doc = "Interrupt Enable 0\n resetvalue={Application Reset:0x0}"]
    pub type IEi = crate::RegValueT<IEi_SPEC>;

    impl IEi {
        #[doc = "Rx FIFO 0 New Message Interrupt Enable   RF0NE"]
        #[inline(always)]
        pub fn rf0ne(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, iei::Rf0Ne, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1,1,0,iei::Rf0Ne, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Rx FIFO 0 Watermark Reached Interrupt Enable   RF0WE"]
        #[inline(always)]
        pub fn rf0we(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, iei::Rf0We, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0x1,1,0,iei::Rf0We, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Rx FIFO 0 Full Interrupt Enable   RF0FE"]
        #[inline(always)]
        pub fn rf0fe(
            self,
        ) -> crate::common::RegisterField<2, 0x1, 1, 0, iei::Rf0Fe, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x1,1,0,iei::Rf0Fe, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Rx FIFO 0 Message Lost Interrupt Enable   RF0LE"]
        #[inline(always)]
        pub fn rf0le(
            self,
        ) -> crate::common::RegisterField<3, 0x1, 1, 0, iei::Rf0Le, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<3,0x1,1,0,iei::Rf0Le, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Rx FIFO 1 New Message Interrupt Enable   RF1NE"]
        #[inline(always)]
        pub fn rf1ne(
            self,
        ) -> crate::common::RegisterField<4, 0x1, 1, 0, iei::Rf1Ne, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x1,1,0,iei::Rf1Ne, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Rx FIFO 1 Watermark Reached Interrupt Enable   RF1WE"]
        #[inline(always)]
        pub fn rf1we(
            self,
        ) -> crate::common::RegisterField<5, 0x1, 1, 0, iei::Rf1We, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x1,1,0,iei::Rf1We, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Rx FIFO 1 Full Interrupt Enable   RF1FE"]
        #[inline(always)]
        pub fn rf1fe(
            self,
        ) -> crate::common::RegisterField<6, 0x1, 1, 0, iei::Rf1Fe, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<6,0x1,1,0,iei::Rf1Fe, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Rx FIFO 1 Message Lost Interrupt Enable   RF1LE"]
        #[inline(always)]
        pub fn rf1le(
            self,
        ) -> crate::common::RegisterField<7, 0x1, 1, 0, iei::Rf1Le, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<7,0x1,1,0,iei::Rf1Le, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "High Priority Message Interrupt Enable   HPME"]
        #[inline(always)]
        pub fn hpme(
            self,
        ) -> crate::common::RegisterField<8, 0x1, 1, 0, iei::Hpme, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x1,1,0,iei::Hpme, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Transmission Completed Interrupt Enable   TCE"]
        #[inline(always)]
        pub fn tce(
            self,
        ) -> crate::common::RegisterField<9, 0x1, 1, 0, iei::Tce, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<9,0x1,1,0,iei::Tce, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Transmission Cancellation Finished Interrupt Enable   TCFE"]
        #[inline(always)]
        pub fn tcfe(
            self,
        ) -> crate::common::RegisterField<10, 0x1, 1, 0, iei::Tcfe, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<10,0x1,1,0,iei::Tcfe, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Tx FIFO Empty Interrupt Enable   TFEE"]
        #[inline(always)]
        pub fn tfee(
            self,
        ) -> crate::common::RegisterField<11, 0x1, 1, 0, iei::Tfee, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<11,0x1,1,0,iei::Tfee, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Tx Event FIFO New Entry Interrupt Enable   TEFNE"]
        #[inline(always)]
        pub fn tefne(
            self,
        ) -> crate::common::RegisterField<12, 0x1, 1, 0, iei::Tefne, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0x1,1,0,iei::Tefne, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Tx Event FIFO Watermark Reached Interrupt Enable   TEFWE"]
        #[inline(always)]
        pub fn tefwe(
            self,
        ) -> crate::common::RegisterField<13, 0x1, 1, 0, iei::Tefwe, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<13,0x1,1,0,iei::Tefwe, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Tx Event FIFO Full Interrupt Enable   TEFFE"]
        #[inline(always)]
        pub fn teffe(
            self,
        ) -> crate::common::RegisterField<14, 0x1, 1, 0, iei::Teffe, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<14,0x1,1,0,iei::Teffe, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Tx Event FIFO Element Lost Interrupt Enable   TEFLE"]
        #[inline(always)]
        pub fn tefle(
            self,
        ) -> crate::common::RegisterField<15, 0x1, 1, 0, iei::Tefle, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<15,0x1,1,0,iei::Tefle, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Timestamp Wraparound Interrupt Enable   TSWE"]
        #[inline(always)]
        pub fn tswe(
            self,
        ) -> crate::common::RegisterField<16, 0x1, 1, 0, iei::Tswe, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1,1,0,iei::Tswe, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Message RAM Access Failure Interrupt Enable   MRAFE"]
        #[inline(always)]
        pub fn mrafe(
            self,
        ) -> crate::common::RegisterField<17, 0x1, 1, 0, iei::Mrafe, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<17,0x1,1,0,iei::Mrafe, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Timeout Occurred Interrupt Enable   TOOE"]
        #[inline(always)]
        pub fn tooe(
            self,
        ) -> crate::common::RegisterField<18, 0x1, 1, 0, iei::Tooe, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<18,0x1,1,0,iei::Tooe, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Message stored to Dedicated Rx Buffer Interrupt Enable   DRXE"]
        #[inline(always)]
        pub fn drxe(
            self,
        ) -> crate::common::RegisterField<19, 0x1, 1, 0, iei::Drxe, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<19,0x1,1,0,iei::Drxe, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Error Logging Overflow Interrupt Enable   ELOE"]
        #[inline(always)]
        pub fn eloe(
            self,
        ) -> crate::common::RegisterField<22, 0x1, 1, 0, iei::Eloe, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<22,0x1,1,0,iei::Eloe, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Error Passive Interrupt Enable   EPE"]
        #[inline(always)]
        pub fn epe(
            self,
        ) -> crate::common::RegisterField<23, 0x1, 1, 0, iei::Epe, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<23,0x1,1,0,iei::Epe, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Warning Status Interrupt Enable   EWE"]
        #[inline(always)]
        pub fn ewe(
            self,
        ) -> crate::common::RegisterField<24, 0x1, 1, 0, iei::Ewe, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0x1,1,0,iei::Ewe, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Bus Off Status Interrupt Enable   BOE"]
        #[inline(always)]
        pub fn boe(
            self,
        ) -> crate::common::RegisterField<25, 0x1, 1, 0, iei::Boe, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<25,0x1,1,0,iei::Boe, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Watchdog Interrupt Enable   WDIE"]
        #[inline(always)]
        pub fn wdie(
            self,
        ) -> crate::common::RegisterField<26, 0x1, 1, 0, iei::Wdie, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<26,0x1,1,0,iei::Wdie, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Protocol Error in Arbitration Phase Enable   PEAE"]
        #[inline(always)]
        pub fn peae(
            self,
        ) -> crate::common::RegisterField<27, 0x1, 1, 0, iei::Peae, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<27,0x1,1,0,iei::Peae, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Protocol Error in Data Phase Enable   PEDE"]
        #[inline(always)]
        pub fn pede(
            self,
        ) -> crate::common::RegisterField<28, 0x1, 1, 0, iei::Pede, IEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0x1,1,0,iei::Pede, IEi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for IEi {
        #[inline(always)]
        fn default() -> IEi {
            <crate::RegValueT<IEi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod iei {
        pub struct Rf0Ne_SPEC;
        pub type Rf0Ne = crate::EnumBitfieldStruct<u8, Rf0Ne_SPEC>;
        impl Rf0Ne {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rf0We_SPEC;
        pub type Rf0We = crate::EnumBitfieldStruct<u8, Rf0We_SPEC>;
        impl Rf0We {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rf0Fe_SPEC;
        pub type Rf0Fe = crate::EnumBitfieldStruct<u8, Rf0Fe_SPEC>;
        impl Rf0Fe {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rf0Le_SPEC;
        pub type Rf0Le = crate::EnumBitfieldStruct<u8, Rf0Le_SPEC>;
        impl Rf0Le {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rf1Ne_SPEC;
        pub type Rf1Ne = crate::EnumBitfieldStruct<u8, Rf1Ne_SPEC>;
        impl Rf1Ne {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rf1We_SPEC;
        pub type Rf1We = crate::EnumBitfieldStruct<u8, Rf1We_SPEC>;
        impl Rf1We {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rf1Fe_SPEC;
        pub type Rf1Fe = crate::EnumBitfieldStruct<u8, Rf1Fe_SPEC>;
        impl Rf1Fe {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rf1Le_SPEC;
        pub type Rf1Le = crate::EnumBitfieldStruct<u8, Rf1Le_SPEC>;
        impl Rf1Le {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Hpme_SPEC;
        pub type Hpme = crate::EnumBitfieldStruct<u8, Hpme_SPEC>;
        impl Hpme {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tce_SPEC;
        pub type Tce = crate::EnumBitfieldStruct<u8, Tce_SPEC>;
        impl Tce {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tcfe_SPEC;
        pub type Tcfe = crate::EnumBitfieldStruct<u8, Tcfe_SPEC>;
        impl Tcfe {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tfee_SPEC;
        pub type Tfee = crate::EnumBitfieldStruct<u8, Tfee_SPEC>;
        impl Tfee {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tefne_SPEC;
        pub type Tefne = crate::EnumBitfieldStruct<u8, Tefne_SPEC>;
        impl Tefne {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tefwe_SPEC;
        pub type Tefwe = crate::EnumBitfieldStruct<u8, Tefwe_SPEC>;
        impl Tefwe {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Teffe_SPEC;
        pub type Teffe = crate::EnumBitfieldStruct<u8, Teffe_SPEC>;
        impl Teffe {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tefle_SPEC;
        pub type Tefle = crate::EnumBitfieldStruct<u8, Tefle_SPEC>;
        impl Tefle {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tswe_SPEC;
        pub type Tswe = crate::EnumBitfieldStruct<u8, Tswe_SPEC>;
        impl Tswe {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Mrafe_SPEC;
        pub type Mrafe = crate::EnumBitfieldStruct<u8, Mrafe_SPEC>;
        impl Mrafe {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tooe_SPEC;
        pub type Tooe = crate::EnumBitfieldStruct<u8, Tooe_SPEC>;
        impl Tooe {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Drxe_SPEC;
        pub type Drxe = crate::EnumBitfieldStruct<u8, Drxe_SPEC>;
        impl Drxe {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Eloe_SPEC;
        pub type Eloe = crate::EnumBitfieldStruct<u8, Eloe_SPEC>;
        impl Eloe {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Epe_SPEC;
        pub type Epe = crate::EnumBitfieldStruct<u8, Epe_SPEC>;
        impl Epe {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Ewe_SPEC;
        pub type Ewe = crate::EnumBitfieldStruct<u8, Ewe_SPEC>;
        impl Ewe {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Boe_SPEC;
        pub type Boe = crate::EnumBitfieldStruct<u8, Boe_SPEC>;
        impl Boe {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Wdie_SPEC;
        pub type Wdie = crate::EnumBitfieldStruct<u8, Wdie_SPEC>;
        impl Wdie {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Peae_SPEC;
        pub type Peae = crate::EnumBitfieldStruct<u8, Peae_SPEC>;
        impl Peae {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Pede_SPEC;
        pub type Pede = crate::EnumBitfieldStruct<u8, Pede_SPEC>;
        impl Pede {
            #[doc = "0 Interrupt disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IlEi_SPEC;
    impl crate::sealed::RegSpec for IlEi_SPEC {
        type DataType = u32;
    }
    #[doc = "Interrupt Line Enable 0\n resetvalue={Application Reset:0x0}"]
    pub type IlEi = crate::RegValueT<IlEi_SPEC>;

    impl IlEi {
        #[doc = "Enable Interrupt Line 0   EINT0"]
        #[inline(always)]
        pub fn eint0(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, ilei::Eint0, IlEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1,1,0,ilei::Eint0, IlEi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable Interrupt Line 1   EINT1"]
        #[inline(always)]
        pub fn eint1(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, ilei::Eint1, IlEi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0x1,1,0,ilei::Eint1, IlEi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for IlEi {
        #[inline(always)]
        fn default() -> IlEi {
            <crate::RegValueT<IlEi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod ilei {
        pub struct Eint0_SPEC;
        pub type Eint0 = crate::EnumBitfieldStruct<u8, Eint0_SPEC>;
        impl Eint0 {
            #[doc = "0 Interrupt line 0 disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt line 0 enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Eint1_SPEC;
        pub type Eint1 = crate::EnumBitfieldStruct<u8, Eint1_SPEC>;
        impl Eint1 {
            #[doc = "0 Interrupt line 1 disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt line 1 enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IlSi_SPEC;
    impl crate::sealed::RegSpec for IlSi_SPEC {
        type DataType = u32;
    }
    #[doc = "Interrupt Line Select 0\n resetvalue={Application Reset:0x0}"]
    pub type IlSi = crate::RegValueT<IlSi_SPEC>;

    impl IlSi {
        #[doc = "Rx FIFO 0 New Message Interrupt Line   RF0NL"]
        #[inline(always)]
        pub fn rf0nl(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, ilsi::Rf0Nl, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1,1,0,ilsi::Rf0Nl, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Rx FIFO 0 Watermark Reached Interrupt Line   RF0WL"]
        #[inline(always)]
        pub fn rf0wl(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, ilsi::Rf0Wl, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0x1,1,0,ilsi::Rf0Wl, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Rx FIFO 0 Full Interrupt Line   RF0FL"]
        #[inline(always)]
        pub fn rf0fl(
            self,
        ) -> crate::common::RegisterField<2, 0x1, 1, 0, ilsi::Rf0Fl, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x1,1,0,ilsi::Rf0Fl, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Rx FIFO 0 Message Lost Interrupt Line   RF0LL"]
        #[inline(always)]
        pub fn rf0ll(
            self,
        ) -> crate::common::RegisterField<3, 0x1, 1, 0, ilsi::Rf0Ll, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<3,0x1,1,0,ilsi::Rf0Ll, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Rx FIFO 1 New Message Interrupt Line   RF1NL"]
        #[inline(always)]
        pub fn rf1nl(
            self,
        ) -> crate::common::RegisterField<4, 0x1, 1, 0, ilsi::Rf1Nl, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x1,1,0,ilsi::Rf1Nl, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Rx FIFO 1 Watermark Reached Interrupt Line   RF1WL"]
        #[inline(always)]
        pub fn rf1wl(
            self,
        ) -> crate::common::RegisterField<5, 0x1, 1, 0, ilsi::Rf1Wl, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x1,1,0,ilsi::Rf1Wl, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Rx FIFO 1 Full Interrupt Line   RF1FL"]
        #[inline(always)]
        pub fn rf1fl(
            self,
        ) -> crate::common::RegisterField<6, 0x1, 1, 0, ilsi::Rf1Fl, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<6,0x1,1,0,ilsi::Rf1Fl, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Rx FIFO 1 Message Lost Interrupt Line   RF1LL"]
        #[inline(always)]
        pub fn rf1ll(
            self,
        ) -> crate::common::RegisterField<7, 0x1, 1, 0, ilsi::Rf1Ll, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<7,0x1,1,0,ilsi::Rf1Ll, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "High Priority Message Interrupt Line   HPML"]
        #[inline(always)]
        pub fn hpml(
            self,
        ) -> crate::common::RegisterField<8, 0x1, 1, 0, ilsi::Hpml, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x1,1,0,ilsi::Hpml, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Transmission Completed Interrupt Line   TCL"]
        #[inline(always)]
        pub fn tcl(
            self,
        ) -> crate::common::RegisterField<9, 0x1, 1, 0, ilsi::Tcl, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<9,0x1,1,0,ilsi::Tcl, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Transmission Cancellation Finished Interrupt Line   TCFL"]
        #[inline(always)]
        pub fn tcfl(
            self,
        ) -> crate::common::RegisterField<10, 0x1, 1, 0, ilsi::Tcfl, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<10,0x1,1,0,ilsi::Tcfl, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Tx FIFO Empty Interrupt Line   TFEL"]
        #[inline(always)]
        pub fn tfel(
            self,
        ) -> crate::common::RegisterField<11, 0x1, 1, 0, ilsi::Tfel, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<11,0x1,1,0,ilsi::Tfel, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Tx Event FIFO New Entry Interrupt Line   TEFNL"]
        #[inline(always)]
        pub fn tefnl(
            self,
        ) -> crate::common::RegisterField<12, 0x1, 1, 0, ilsi::Tefnl, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0x1,1,0,ilsi::Tefnl, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Tx Event FIFO Watermark Reached Interrupt Line   TEFWL"]
        #[inline(always)]
        pub fn tefwl(
            self,
        ) -> crate::common::RegisterField<13, 0x1, 1, 0, ilsi::Tefwl, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<13,0x1,1,0,ilsi::Tefwl, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Tx Event FIFO Full Interrupt Line   TEFFL"]
        #[inline(always)]
        pub fn teffl(
            self,
        ) -> crate::common::RegisterField<14, 0x1, 1, 0, ilsi::Teffl, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<14,0x1,1,0,ilsi::Teffl, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Tx Event FIFO Element Lost Interrupt Line   TEFLL"]
        #[inline(always)]
        pub fn tefll(
            self,
        ) -> crate::common::RegisterField<15, 0x1, 1, 0, ilsi::Tefll, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<15,0x1,1,0,ilsi::Tefll, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Timestamp Wraparound Interrupt Line   TSWL"]
        #[inline(always)]
        pub fn tswl(
            self,
        ) -> crate::common::RegisterField<16, 0x1, 1, 0, ilsi::Tswl, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1,1,0,ilsi::Tswl, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Message RAM Access Failure Interrupt Line   MRAFL"]
        #[inline(always)]
        pub fn mrafl(
            self,
        ) -> crate::common::RegisterField<17, 0x1, 1, 0, ilsi::Mrafl, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<17,0x1,1,0,ilsi::Mrafl, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Timeout Occurred Interrupt Line   TOOL"]
        #[inline(always)]
        pub fn tool(
            self,
        ) -> crate::common::RegisterField<18, 0x1, 1, 0, ilsi::Tool, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<18,0x1,1,0,ilsi::Tool, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Message stored to Dedicated Rx Buffer Interrupt Line   DRXL"]
        #[inline(always)]
        pub fn drxl(
            self,
        ) -> crate::common::RegisterField<19, 0x1, 1, 0, ilsi::Drxl, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<19,0x1,1,0,ilsi::Drxl, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Bit Error Corrected Interrupt Line   BECL"]
        #[inline(always)]
        pub fn becl(
            self,
        ) -> crate::common::RegisterField<20, 0x1, 1, 0, ilsi::Becl, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0x1,1,0,ilsi::Becl, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Bit Error Uncorrected   BEUL. This feature shall not be connected."]
        #[inline(always)]
        pub fn beul(
            self,
        ) -> crate::common::RegisterField<21, 0x1, 1, 0, ilsi::Beul, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<21,0x1,1,0,ilsi::Beul, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Error Logging Overflow Interrupt Line   ELOL. This feature shall not be connected."]
        #[inline(always)]
        pub fn elol(
            self,
        ) -> crate::common::RegisterField<22, 0x1, 1, 0, ilsi::Elol, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<22,0x1,1,0,ilsi::Elol, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Error Passive Interrupt Line   EPL"]
        #[inline(always)]
        pub fn epl(
            self,
        ) -> crate::common::RegisterField<23, 0x1, 1, 0, ilsi::Epl, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<23,0x1,1,0,ilsi::Epl, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Warning Status Interrupt Line   EWL"]
        #[inline(always)]
        pub fn ewl(
            self,
        ) -> crate::common::RegisterField<24, 0x1, 1, 0, ilsi::Ewl, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0x1,1,0,ilsi::Ewl, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Bus Off Status Interrupt Line   BOL"]
        #[inline(always)]
        pub fn bol(
            self,
        ) -> crate::common::RegisterField<25, 0x1, 1, 0, ilsi::Bol, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<25,0x1,1,0,ilsi::Bol, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Watchdog Interrupt Line   WDIL"]
        #[inline(always)]
        pub fn wdil(
            self,
        ) -> crate::common::RegisterField<26, 0x1, 1, 0, ilsi::Wdil, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<26,0x1,1,0,ilsi::Wdil, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Protocol Error in Arbitration Phase Line   PEAL"]
        #[inline(always)]
        pub fn peal(
            self,
        ) -> crate::common::RegisterField<27, 0x1, 1, 0, ilsi::Peal, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<27,0x1,1,0,ilsi::Peal, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Protocol Error in Data Phase Line   PEDL"]
        #[inline(always)]
        pub fn pedl(
            self,
        ) -> crate::common::RegisterField<28, 0x1, 1, 0, ilsi::Pedl, IlSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0x1,1,0,ilsi::Pedl, IlSi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for IlSi {
        #[inline(always)]
        fn default() -> IlSi {
            <crate::RegValueT<IlSi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod ilsi {
        pub struct Rf0Nl_SPEC;
        pub type Rf0Nl = crate::EnumBitfieldStruct<u8, Rf0Nl_SPEC>;
        impl Rf0Nl {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rf0Wl_SPEC;
        pub type Rf0Wl = crate::EnumBitfieldStruct<u8, Rf0Wl_SPEC>;
        impl Rf0Wl {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rf0Fl_SPEC;
        pub type Rf0Fl = crate::EnumBitfieldStruct<u8, Rf0Fl_SPEC>;
        impl Rf0Fl {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rf0Ll_SPEC;
        pub type Rf0Ll = crate::EnumBitfieldStruct<u8, Rf0Ll_SPEC>;
        impl Rf0Ll {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rf1Nl_SPEC;
        pub type Rf1Nl = crate::EnumBitfieldStruct<u8, Rf1Nl_SPEC>;
        impl Rf1Nl {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rf1Wl_SPEC;
        pub type Rf1Wl = crate::EnumBitfieldStruct<u8, Rf1Wl_SPEC>;
        impl Rf1Wl {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rf1Fl_SPEC;
        pub type Rf1Fl = crate::EnumBitfieldStruct<u8, Rf1Fl_SPEC>;
        impl Rf1Fl {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rf1Ll_SPEC;
        pub type Rf1Ll = crate::EnumBitfieldStruct<u8, Rf1Ll_SPEC>;
        impl Rf1Ll {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Hpml_SPEC;
        pub type Hpml = crate::EnumBitfieldStruct<u8, Hpml_SPEC>;
        impl Hpml {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tcl_SPEC;
        pub type Tcl = crate::EnumBitfieldStruct<u8, Tcl_SPEC>;
        impl Tcl {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tcfl_SPEC;
        pub type Tcfl = crate::EnumBitfieldStruct<u8, Tcfl_SPEC>;
        impl Tcfl {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tfel_SPEC;
        pub type Tfel = crate::EnumBitfieldStruct<u8, Tfel_SPEC>;
        impl Tfel {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tefnl_SPEC;
        pub type Tefnl = crate::EnumBitfieldStruct<u8, Tefnl_SPEC>;
        impl Tefnl {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tefwl_SPEC;
        pub type Tefwl = crate::EnumBitfieldStruct<u8, Tefwl_SPEC>;
        impl Tefwl {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Teffl_SPEC;
        pub type Teffl = crate::EnumBitfieldStruct<u8, Teffl_SPEC>;
        impl Teffl {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tefll_SPEC;
        pub type Tefll = crate::EnumBitfieldStruct<u8, Tefll_SPEC>;
        impl Tefll {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tswl_SPEC;
        pub type Tswl = crate::EnumBitfieldStruct<u8, Tswl_SPEC>;
        impl Tswl {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Mrafl_SPEC;
        pub type Mrafl = crate::EnumBitfieldStruct<u8, Mrafl_SPEC>;
        impl Mrafl {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tool_SPEC;
        pub type Tool = crate::EnumBitfieldStruct<u8, Tool_SPEC>;
        impl Tool {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Drxl_SPEC;
        pub type Drxl = crate::EnumBitfieldStruct<u8, Drxl_SPEC>;
        impl Drxl {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Becl_SPEC;
        pub type Becl = crate::EnumBitfieldStruct<u8, Becl_SPEC>;
        impl Becl {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Beul_SPEC;
        pub type Beul = crate::EnumBitfieldStruct<u8, Beul_SPEC>;
        impl Beul {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Elol_SPEC;
        pub type Elol = crate::EnumBitfieldStruct<u8, Elol_SPEC>;
        impl Elol {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Epl_SPEC;
        pub type Epl = crate::EnumBitfieldStruct<u8, Epl_SPEC>;
        impl Epl {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Ewl_SPEC;
        pub type Ewl = crate::EnumBitfieldStruct<u8, Ewl_SPEC>;
        impl Ewl {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Bol_SPEC;
        pub type Bol = crate::EnumBitfieldStruct<u8, Bol_SPEC>;
        impl Bol {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Wdil_SPEC;
        pub type Wdil = crate::EnumBitfieldStruct<u8, Wdil_SPEC>;
        impl Wdil {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Peal_SPEC;
        pub type Peal = crate::EnumBitfieldStruct<u8, Peal_SPEC>;
        impl Peal {
            #[doc = "0 Interrupt        assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt        assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Pedl_SPEC;
        pub type Pedl = crate::EnumBitfieldStruct<u8, Pedl_SPEC>;
        impl Pedl {
            #[doc = "0 Interrupt assigned to interrupt line 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Interrupt assigned to interrupt line 1"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IRi_SPEC;
    impl crate::sealed::RegSpec for IRi_SPEC {
        type DataType = u32;
    }
    #[doc = "Interrupt Register 0\n resetvalue={Application Reset:0x0}"]
    pub type IRi = crate::RegValueT<IRi_SPEC>;

    impl IRi {
        #[doc = "Rx FIFO 0 New Message   RF0N"]
        #[inline(always)]
        pub fn rf0n(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, iri::Rf0N, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1,1,0,iri::Rf0N, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Rx FIFO 0 Watermark Reached   RF0W"]
        #[inline(always)]
        pub fn rf0w(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, iri::Rf0W, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0x1,1,0,iri::Rf0W, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Rx FIFO 0 Full   RF0F"]
        #[inline(always)]
        pub fn rf0f(
            self,
        ) -> crate::common::RegisterField<2, 0x1, 1, 0, iri::Rf0F, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x1,1,0,iri::Rf0F, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Rx FIFO 0 Message Lost   RF0L"]
        #[inline(always)]
        pub fn rf0l(
            self,
        ) -> crate::common::RegisterField<3, 0x1, 1, 0, iri::Rf0L, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<3,0x1,1,0,iri::Rf0L, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Rx FIFO 1 New Message   RF1N"]
        #[inline(always)]
        pub fn rf1n(
            self,
        ) -> crate::common::RegisterField<4, 0x1, 1, 0, iri::Rf1N, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x1,1,0,iri::Rf1N, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Rx FIFO 1 Watermark Reached   RF1W"]
        #[inline(always)]
        pub fn rf1w(
            self,
        ) -> crate::common::RegisterField<5, 0x1, 1, 0, iri::Rf1W, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x1,1,0,iri::Rf1W, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Rx FIFO 1 Full   RF1F"]
        #[inline(always)]
        pub fn rf1f(
            self,
        ) -> crate::common::RegisterField<6, 0x1, 1, 0, iri::Rf1F, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<6,0x1,1,0,iri::Rf1F, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Rx FIFO 1 Message Lost   RF1L"]
        #[inline(always)]
        pub fn rf1l(
            self,
        ) -> crate::common::RegisterField<7, 0x1, 1, 0, iri::Rf1L, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<7,0x1,1,0,iri::Rf1L, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "High Priority Message   HPM"]
        #[inline(always)]
        pub fn hpm(
            self,
        ) -> crate::common::RegisterField<8, 0x1, 1, 0, iri::Hpm, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x1,1,0,iri::Hpm, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Transmission Completed   TC"]
        #[inline(always)]
        pub fn tc(
            self,
        ) -> crate::common::RegisterField<9, 0x1, 1, 0, iri::Tc, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<9,0x1,1,0,iri::Tc, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Transmission Cancellation Finished   TCF"]
        #[inline(always)]
        pub fn tcf(
            self,
        ) -> crate::common::RegisterField<10, 0x1, 1, 0, iri::Tcf, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<10,0x1,1,0,iri::Tcf, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Tx FIFO Empty   TFE"]
        #[inline(always)]
        pub fn tfe(
            self,
        ) -> crate::common::RegisterField<11, 0x1, 1, 0, iri::Tfe, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<11,0x1,1,0,iri::Tfe, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Tx Event FIFO New Entry   TEFN"]
        #[inline(always)]
        pub fn tefn(
            self,
        ) -> crate::common::RegisterField<12, 0x1, 1, 0, iri::Tefn, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0x1,1,0,iri::Tefn, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Tx Event FIFO Watermark Reached   TEFW"]
        #[inline(always)]
        pub fn tefw(
            self,
        ) -> crate::common::RegisterField<13, 0x1, 1, 0, iri::Tefw, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<13,0x1,1,0,iri::Tefw, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Tx Event FIFO Full   TEFF"]
        #[inline(always)]
        pub fn teff(
            self,
        ) -> crate::common::RegisterField<14, 0x1, 1, 0, iri::Teff, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<14,0x1,1,0,iri::Teff, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Tx Event FIFO Element Lost   TEFL"]
        #[inline(always)]
        pub fn tefl(
            self,
        ) -> crate::common::RegisterField<15, 0x1, 1, 0, iri::Tefl, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<15,0x1,1,0,iri::Tefl, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Timestamp Wraparound   TSW"]
        #[inline(always)]
        pub fn tsw(
            self,
        ) -> crate::common::RegisterField<16, 0x1, 1, 0, iri::Tsw, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1,1,0,iri::Tsw, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Message RAM Access Failure   MRAF. The flag is set  when the Rx Handler has not completed acceptance filtering or storage of an accepted          message until the arbitration field of the following message has been          received. In this case acceptance filtering or message storage is          aborted and the Rx Handler starts processing of the following message. was not able to write a message to the Message RAM. In this case          message storage is aborted. In both cases the FIFO put index is not updated resp. the New Data flag        for a dedicated Rx Buffer is not set  a partly stored message is        overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message        from the Message RAM in time. In this case message transmission is        aborted. In case of a Tx Handler access failure the M CAN is switched into Restricted Operation Mode. To leave Restricted        Operation Mode  the Host CPU has to reset CCCR.ASM."]
        #[inline(always)]
        pub fn mraf(
            self,
        ) -> crate::common::RegisterField<17, 0x1, 1, 0, iri::Mraf, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<17,0x1,1,0,iri::Mraf, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Timeout Occurred   TOO"]
        #[inline(always)]
        pub fn too(
            self,
        ) -> crate::common::RegisterField<18, 0x1, 1, 0, iri::Too, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<18,0x1,1,0,iri::Too, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Message stored to Dedicated Rx Buffer   DRX. The flag is set whenever a received message has been stored into a        dedicated Rx Buffer."]
        #[inline(always)]
        pub fn drx(
            self,
        ) -> crate::common::RegisterField<19, 0x1, 1, 0, iri::Drx, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<19,0x1,1,0,iri::Drx, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Error Logging Overflow   ELO"]
        #[inline(always)]
        pub fn elo(
            self,
        ) -> crate::common::RegisterField<22, 0x1, 1, 0, iri::Elo, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<22,0x1,1,0,iri::Elo, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Error Passive   EP"]
        #[inline(always)]
        pub fn ep(
            self,
        ) -> crate::common::RegisterField<23, 0x1, 1, 0, iri::Ep, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<23,0x1,1,0,iri::Ep, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Warning Status   EW"]
        #[inline(always)]
        pub fn ew(
            self,
        ) -> crate::common::RegisterField<24, 0x1, 1, 0, iri::Ew, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0x1,1,0,iri::Ew, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Bus Off Status   BO"]
        #[inline(always)]
        pub fn bo(
            self,
        ) -> crate::common::RegisterField<25, 0x1, 1, 0, iri::Bo, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<25,0x1,1,0,iri::Bo, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Watchdog Interrupt   WDI"]
        #[inline(always)]
        pub fn wdi(
            self,
        ) -> crate::common::RegisterField<26, 0x1, 1, 0, iri::Wdi, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<26,0x1,1,0,iri::Wdi, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Protocol Error in Arbitration Phase   PEA.  Nominal Bit Time is used"]
        #[inline(always)]
        pub fn pea(
            self,
        ) -> crate::common::RegisterField<27, 0x1, 1, 0, iri::Pea, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<27,0x1,1,0,iri::Pea, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Protocol Error in Data Phase   PED.  Data Bit Time is used"]
        #[inline(always)]
        pub fn ped(
            self,
        ) -> crate::common::RegisterField<28, 0x1, 1, 0, iri::Ped, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0x1,1,0,iri::Ped, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for IRi {
        #[inline(always)]
        fn default() -> IRi {
            <crate::RegValueT<IRi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod iri {
        pub struct Rf0N_SPEC;
        pub type Rf0N = crate::EnumBitfieldStruct<u8, Rf0N_SPEC>;
        impl Rf0N {
            #[doc = "0 No new message written to Rx FIFO 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 New message written to Rx FIFO 0"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rf0W_SPEC;
        pub type Rf0W = crate::EnumBitfieldStruct<u8, Rf0W_SPEC>;
        impl Rf0W {
            #[doc = "0 Rx FIFO 0 fill level below watermark"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx FIFO 0 fill level reached watermark"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rf0F_SPEC;
        pub type Rf0F = crate::EnumBitfieldStruct<u8, Rf0F_SPEC>;
        impl Rf0F {
            #[doc = "0 Rx FIFO 0 not full"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx FIFO 0 full"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rf0L_SPEC;
        pub type Rf0L = crate::EnumBitfieldStruct<u8, Rf0L_SPEC>;
        impl Rf0L {
            #[doc = "0 No Rx FIFO 0 message lost"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx FIFO 0 message lost  also set after write attempt to Rx FIFO 0 of size zero"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rf1N_SPEC;
        pub type Rf1N = crate::EnumBitfieldStruct<u8, Rf1N_SPEC>;
        impl Rf1N {
            #[doc = "0 No new message written to Rx FIFO 1"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 New message written to Rx FIFO 1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rf1W_SPEC;
        pub type Rf1W = crate::EnumBitfieldStruct<u8, Rf1W_SPEC>;
        impl Rf1W {
            #[doc = "0 Rx FIFO 1 fill level below watermark"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx FIFO 1 fill level reached watermark"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rf1F_SPEC;
        pub type Rf1F = crate::EnumBitfieldStruct<u8, Rf1F_SPEC>;
        impl Rf1F {
            #[doc = "0 Rx FIFO 1 not full"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx FIFO 1 full"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rf1L_SPEC;
        pub type Rf1L = crate::EnumBitfieldStruct<u8, Rf1L_SPEC>;
        impl Rf1L {
            #[doc = "0 No Rx FIFO 1 message lost"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx FIFO 1 message lost  also set after write attempt to Rx FIFO 1 of size zero"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Hpm_SPEC;
        pub type Hpm = crate::EnumBitfieldStruct<u8, Hpm_SPEC>;
        impl Hpm {
            #[doc = "0 No high priority message received"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 High priority message received"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tc_SPEC;
        pub type Tc = crate::EnumBitfieldStruct<u8, Tc_SPEC>;
        impl Tc {
            #[doc = "0 No transmission completed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Transmission completed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tcf_SPEC;
        pub type Tcf = crate::EnumBitfieldStruct<u8, Tcf_SPEC>;
        impl Tcf {
            #[doc = "0 No transmission cancellation finished"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Transmission cancellation finished"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tfe_SPEC;
        pub type Tfe = crate::EnumBitfieldStruct<u8, Tfe_SPEC>;
        impl Tfe {
            #[doc = "0 Tx FIFO non empty"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Tx FIFO empty"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tefn_SPEC;
        pub type Tefn = crate::EnumBitfieldStruct<u8, Tefn_SPEC>;
        impl Tefn {
            #[doc = "0 Tx Event FIFO unchanged"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Tx Handler wrote Tx Event FIFO element"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tefw_SPEC;
        pub type Tefw = crate::EnumBitfieldStruct<u8, Tefw_SPEC>;
        impl Tefw {
            #[doc = "0 Tx Event FIFO fill level below watermark"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Tx Event FIFO fill level reached watermark"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Teff_SPEC;
        pub type Teff = crate::EnumBitfieldStruct<u8, Teff_SPEC>;
        impl Teff {
            #[doc = "0 Tx Event FIFO not full"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Tx Event FIFO full"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tefl_SPEC;
        pub type Tefl = crate::EnumBitfieldStruct<u8, Tefl_SPEC>;
        impl Tefl {
            #[doc = "0 No Tx Event FIFO element lost"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Tx Event FIFO element lost  also set after write attempt to Tx Event FIFO of size zero"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tsw_SPEC;
        pub type Tsw = crate::EnumBitfieldStruct<u8, Tsw_SPEC>;
        impl Tsw {
            #[doc = "0 No timestamp counter wrap around"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Timestamp counter wrapped around"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Mraf_SPEC;
        pub type Mraf = crate::EnumBitfieldStruct<u8, Mraf_SPEC>;
        impl Mraf {
            #[doc = "0 No Message RAM access failure occurred"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Message RAM access failure occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Too_SPEC;
        pub type Too = crate::EnumBitfieldStruct<u8, Too_SPEC>;
        impl Too {
            #[doc = "0 No timeout"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Timeout reached"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Drx_SPEC;
        pub type Drx = crate::EnumBitfieldStruct<u8, Drx_SPEC>;
        impl Drx {
            #[doc = "0 No Rx Buffer updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 At least one received message stored into an Rx Buffer"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Elo_SPEC;
        pub type Elo = crate::EnumBitfieldStruct<u8, Elo_SPEC>;
        impl Elo {
            #[doc = "0 CAN Error Logging Counter did not overflow"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Overflow of CAN Error Logging Counter occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Ep_SPEC;
        pub type Ep = crate::EnumBitfieldStruct<u8, Ep_SPEC>;
        impl Ep {
            #[doc = "0 Error Passive status unchanged"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Error Passive status changed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Ew_SPEC;
        pub type Ew = crate::EnumBitfieldStruct<u8, Ew_SPEC>;
        impl Ew {
            #[doc = "0 Error Warning status unchanged"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Error Warning status changed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Bo_SPEC;
        pub type Bo = crate::EnumBitfieldStruct<u8, Bo_SPEC>;
        impl Bo {
            #[doc = "0 Bus Off status unchanged"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Bus Off status changed"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Wdi_SPEC;
        pub type Wdi = crate::EnumBitfieldStruct<u8, Wdi_SPEC>;
        impl Wdi {
            #[doc = "0 No Message RAM Watchdog event occurred"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Message RAM Watchdog event due to missing READY"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Pea_SPEC;
        pub type Pea = crate::EnumBitfieldStruct<u8, Pea_SPEC>;
        impl Pea {
            #[doc = "0 No protocol error in arbitration phase"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Protocol error in arbitration phase detected  PSR.LEC   0 7"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Ped_SPEC;
        pub type Ped = crate::EnumBitfieldStruct<u8, Ped_SPEC>;
        impl Ped {
            #[doc = "0 No protocol error in data phase detected"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Protocol error in data phase detected  PSR.DLEC   0 7"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IsreGi_SPEC;
    impl crate::sealed::RegSpec for IsreGi_SPEC {
        type DataType = u32;
    }
    #[doc = "Interrupt Signalling Register 0\n resetvalue={Application Reset:0x0}"]
    pub type IsreGi = crate::RegValueT<IsreGi_SPEC>;

    impl IsreGi {
        #[doc = "A message stored in a receive buffer interrupt   REINT"]
        #[inline(always)]
        pub fn reint(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Receive FIFO1 is full interrupt   RxF1F"]
        #[inline(always)]
        pub fn rxf1f(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<1,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Receive FIFO0 is full interrupt   RxF0F"]
        #[inline(always)]
        pub fn rxf0f(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<2,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Receive FIFO1 got a new message interrupt   RxF1N"]
        #[inline(always)]
        pub fn rxf1n(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<3,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Receive FIFO0 got a new message interrupt   RxF0N"]
        #[inline(always)]
        pub fn rxf0n(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<4,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "A receive timeout event interrupt   RETI"]
        #[inline(always)]
        pub fn reti(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<5,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "A transmission queue event interrupt   TRAQ"]
        #[inline(always)]
        pub fn traq(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<6,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "A transmission control event interrupt   TRACO"]
        #[inline(always)]
        pub fn traco(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<7,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "A Transmit Event FIFO Incident interrupt   TEFIFO"]
        #[inline(always)]
        pub fn tefifo(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<8,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "A high priority event interrupt   HPE"]
        #[inline(always)]
        pub fn hpe(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<9,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "A watermark interrupt has been reached   WATI"]
        #[inline(always)]
        pub fn wati(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<10,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "An alert interrupt   ALRT"]
        #[inline(always)]
        pub fn alrt(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<11,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Module error interrupt   MOER"]
        #[inline(always)]
        pub fn moer(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<12,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "The safety counter interrupt ELO   SAFE"]
        #[inline(always)]
        pub fn safe(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<13,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Bus Off Interrupt   BOFF"]
        #[inline(always)]
        pub fn boff(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<14,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Last Error Interrupt   LOI"]
        #[inline(always)]
        pub fn loi(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, IsreGi_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<15,1,0,IsreGi_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for IsreGi {
        #[inline(always)]
        fn default() -> IsreGi {
            <crate::RegValueT<IsreGi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NbtPi_SPEC;
    impl crate::sealed::RegSpec for NbtPi_SPEC {
        type DataType = u32;
    }
    #[doc = "Nominal Bit Timing   Prescaler Register 0\n resetvalue={Application Reset:0x6000A03}"]
    pub type NbtPi = crate::RegValueT<NbtPi_SPEC>;

    impl NbtPi {
        #[doc = "Nominal Time segment after sample point   NTSEG2. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Valid values are 1 to 127. The actual interpretation by the hardware of        this value is such that one more than the programmed value is used."]
        #[inline(always)]
        pub fn ntseg2(
            self,
        ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, NbtPi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7f,1,0,u8, NbtPi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Nominal Time segment before sample point   NTSEG1. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Valid values are 1 to 255. The actual interpretation by the hardware of        this value is such that one more than the programmed value is used."]
        #[inline(always)]
        pub fn ntseg1(
            self,
        ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, NbtPi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xff,1,0,u8, NbtPi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Baud Rate Prescaler   NBRP. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. The value by which the oscillator frequency is divided for generating        the bit time quanta. The bit time is built up from a multiple of this        quanta. Valid values for the Baud Rate Prescaler are 0 to 511. The        actual interpretation by the hardware of this value is such that one        more than the value programmed here is used."]
        #[inline(always)]
        pub fn nbrp(
            self,
        ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, NbtPi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1ff,1,0,u16, NbtPi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Re  Synchronization Jump Width   NSJW. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Valid values are 0 to 127. The actual interpretation by the hardware of        this value is such that one more than the value programmed here is used."]
        #[inline(always)]
        pub fn nsjw(
            self,
        ) -> crate::common::RegisterField<25, 0x7f, 1, 0, u8, NbtPi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<25,0x7f,1,0,u8, NbtPi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for NbtPi {
        #[inline(always)]
        fn default() -> NbtPi {
            <crate::RegValueT<NbtPi_SPEC> as RegisterValue<_>>::new(100665859)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ndat1I_SPEC;
    impl crate::sealed::RegSpec for Ndat1I_SPEC {
        type DataType = u32;
    }
    #[doc = "New Data 1 0\n resetvalue={Application Reset:0x0}"]
    pub type Ndat1I = crate::RegValueT<Ndat1I_SPEC>;

    impl Ndat1I {
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd0(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, ndat1i::Nd0, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1,1,0,ndat1i::Nd0, Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd1(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, ndat1i::Nd1, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0x1,1,0,ndat1i::Nd1, Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd2(
            self,
        ) -> crate::common::RegisterField<2, 0x1, 1, 0, ndat1i::Nd2, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x1,1,0,ndat1i::Nd2, Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd3(
            self,
        ) -> crate::common::RegisterField<3, 0x1, 1, 0, ndat1i::Nd3, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<3,0x1,1,0,ndat1i::Nd3, Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd4(
            self,
        ) -> crate::common::RegisterField<4, 0x1, 1, 0, ndat1i::Nd4, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x1,1,0,ndat1i::Nd4, Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd5(
            self,
        ) -> crate::common::RegisterField<5, 0x1, 1, 0, ndat1i::Nd5, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x1,1,0,ndat1i::Nd5, Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd6(
            self,
        ) -> crate::common::RegisterField<6, 0x1, 1, 0, ndat1i::Nd6, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<6,0x1,1,0,ndat1i::Nd6, Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd7(
            self,
        ) -> crate::common::RegisterField<7, 0x1, 1, 0, ndat1i::Nd7, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<7,0x1,1,0,ndat1i::Nd7, Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd8(
            self,
        ) -> crate::common::RegisterField<8, 0x1, 1, 0, ndat1i::Nd8, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x1,1,0,ndat1i::Nd8, Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd9(
            self,
        ) -> crate::common::RegisterField<9, 0x1, 1, 0, ndat1i::Nd9, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<9,0x1,1,0,ndat1i::Nd9, Ndat1I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd10(
            self,
        ) -> crate::common::RegisterField<10, 0x1, 1, 0, ndat1i::Nd10, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                ndat1i::Nd10,
                Ndat1I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd11(
            self,
        ) -> crate::common::RegisterField<11, 0x1, 1, 0, ndat1i::Nd11, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                ndat1i::Nd11,
                Ndat1I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd12(
            self,
        ) -> crate::common::RegisterField<12, 0x1, 1, 0, ndat1i::Nd12, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                12,
                0x1,
                1,
                0,
                ndat1i::Nd12,
                Ndat1I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd13(
            self,
        ) -> crate::common::RegisterField<13, 0x1, 1, 0, ndat1i::Nd13, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                13,
                0x1,
                1,
                0,
                ndat1i::Nd13,
                Ndat1I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd14(
            self,
        ) -> crate::common::RegisterField<14, 0x1, 1, 0, ndat1i::Nd14, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                ndat1i::Nd14,
                Ndat1I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd15(
            self,
        ) -> crate::common::RegisterField<15, 0x1, 1, 0, ndat1i::Nd15, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                ndat1i::Nd15,
                Ndat1I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd16(
            self,
        ) -> crate::common::RegisterField<16, 0x1, 1, 0, ndat1i::Nd16, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                16,
                0x1,
                1,
                0,
                ndat1i::Nd16,
                Ndat1I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd17(
            self,
        ) -> crate::common::RegisterField<17, 0x1, 1, 0, ndat1i::Nd17, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                17,
                0x1,
                1,
                0,
                ndat1i::Nd17,
                Ndat1I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd18(
            self,
        ) -> crate::common::RegisterField<18, 0x1, 1, 0, ndat1i::Nd18, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                18,
                0x1,
                1,
                0,
                ndat1i::Nd18,
                Ndat1I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd19(
            self,
        ) -> crate::common::RegisterField<19, 0x1, 1, 0, ndat1i::Nd19, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                19,
                0x1,
                1,
                0,
                ndat1i::Nd19,
                Ndat1I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd20(
            self,
        ) -> crate::common::RegisterField<20, 0x1, 1, 0, ndat1i::Nd20, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                20,
                0x1,
                1,
                0,
                ndat1i::Nd20,
                Ndat1I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd21(
            self,
        ) -> crate::common::RegisterField<21, 0x1, 1, 0, ndat1i::Nd21, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                21,
                0x1,
                1,
                0,
                ndat1i::Nd21,
                Ndat1I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd22(
            self,
        ) -> crate::common::RegisterField<22, 0x1, 1, 0, ndat1i::Nd22, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                22,
                0x1,
                1,
                0,
                ndat1i::Nd22,
                Ndat1I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd23(
            self,
        ) -> crate::common::RegisterField<23, 0x1, 1, 0, ndat1i::Nd23, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                23,
                0x1,
                1,
                0,
                ndat1i::Nd23,
                Ndat1I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd24(
            self,
        ) -> crate::common::RegisterField<24, 0x1, 1, 0, ndat1i::Nd24, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                24,
                0x1,
                1,
                0,
                ndat1i::Nd24,
                Ndat1I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd25(
            self,
        ) -> crate::common::RegisterField<25, 0x1, 1, 0, ndat1i::Nd25, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                25,
                0x1,
                1,
                0,
                ndat1i::Nd25,
                Ndat1I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd26(
            self,
        ) -> crate::common::RegisterField<26, 0x1, 1, 0, ndat1i::Nd26, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                26,
                0x1,
                1,
                0,
                ndat1i::Nd26,
                Ndat1I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd27(
            self,
        ) -> crate::common::RegisterField<27, 0x1, 1, 0, ndat1i::Nd27, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                27,
                0x1,
                1,
                0,
                ndat1i::Nd27,
                Ndat1I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd28(
            self,
        ) -> crate::common::RegisterField<28, 0x1, 1, 0, ndat1i::Nd28, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                28,
                0x1,
                1,
                0,
                ndat1i::Nd28,
                Ndat1I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd29(
            self,
        ) -> crate::common::RegisterField<29, 0x1, 1, 0, ndat1i::Nd29, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                29,
                0x1,
                1,
                0,
                ndat1i::Nd29,
                Ndat1I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd30(
            self,
        ) -> crate::common::RegisterField<30, 0x1, 1, 0, ndat1i::Nd30, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                30,
                0x1,
                1,
                0,
                ndat1i::Nd30,
                Ndat1I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 31   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd31(
            self,
        ) -> crate::common::RegisterField<31, 0x1, 1, 0, ndat1i::Nd31, Ndat1I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                ndat1i::Nd31,
                Ndat1I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for Ndat1I {
        #[inline(always)]
        fn default() -> Ndat1I {
            <crate::RegValueT<Ndat1I_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod ndat1i {
        pub struct Nd0_SPEC;
        pub type Nd0 = crate::EnumBitfieldStruct<u8, Nd0_SPEC>;
        impl Nd0 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd1_SPEC;
        pub type Nd1 = crate::EnumBitfieldStruct<u8, Nd1_SPEC>;
        impl Nd1 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd2_SPEC;
        pub type Nd2 = crate::EnumBitfieldStruct<u8, Nd2_SPEC>;
        impl Nd2 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd3_SPEC;
        pub type Nd3 = crate::EnumBitfieldStruct<u8, Nd3_SPEC>;
        impl Nd3 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd4_SPEC;
        pub type Nd4 = crate::EnumBitfieldStruct<u8, Nd4_SPEC>;
        impl Nd4 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd5_SPEC;
        pub type Nd5 = crate::EnumBitfieldStruct<u8, Nd5_SPEC>;
        impl Nd5 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd6_SPEC;
        pub type Nd6 = crate::EnumBitfieldStruct<u8, Nd6_SPEC>;
        impl Nd6 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd7_SPEC;
        pub type Nd7 = crate::EnumBitfieldStruct<u8, Nd7_SPEC>;
        impl Nd7 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd8_SPEC;
        pub type Nd8 = crate::EnumBitfieldStruct<u8, Nd8_SPEC>;
        impl Nd8 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd9_SPEC;
        pub type Nd9 = crate::EnumBitfieldStruct<u8, Nd9_SPEC>;
        impl Nd9 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd10_SPEC;
        pub type Nd10 = crate::EnumBitfieldStruct<u8, Nd10_SPEC>;
        impl Nd10 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd11_SPEC;
        pub type Nd11 = crate::EnumBitfieldStruct<u8, Nd11_SPEC>;
        impl Nd11 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd12_SPEC;
        pub type Nd12 = crate::EnumBitfieldStruct<u8, Nd12_SPEC>;
        impl Nd12 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd13_SPEC;
        pub type Nd13 = crate::EnumBitfieldStruct<u8, Nd13_SPEC>;
        impl Nd13 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd14_SPEC;
        pub type Nd14 = crate::EnumBitfieldStruct<u8, Nd14_SPEC>;
        impl Nd14 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd15_SPEC;
        pub type Nd15 = crate::EnumBitfieldStruct<u8, Nd15_SPEC>;
        impl Nd15 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd16_SPEC;
        pub type Nd16 = crate::EnumBitfieldStruct<u8, Nd16_SPEC>;
        impl Nd16 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd17_SPEC;
        pub type Nd17 = crate::EnumBitfieldStruct<u8, Nd17_SPEC>;
        impl Nd17 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd18_SPEC;
        pub type Nd18 = crate::EnumBitfieldStruct<u8, Nd18_SPEC>;
        impl Nd18 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd19_SPEC;
        pub type Nd19 = crate::EnumBitfieldStruct<u8, Nd19_SPEC>;
        impl Nd19 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd20_SPEC;
        pub type Nd20 = crate::EnumBitfieldStruct<u8, Nd20_SPEC>;
        impl Nd20 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd21_SPEC;
        pub type Nd21 = crate::EnumBitfieldStruct<u8, Nd21_SPEC>;
        impl Nd21 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd22_SPEC;
        pub type Nd22 = crate::EnumBitfieldStruct<u8, Nd22_SPEC>;
        impl Nd22 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd23_SPEC;
        pub type Nd23 = crate::EnumBitfieldStruct<u8, Nd23_SPEC>;
        impl Nd23 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd24_SPEC;
        pub type Nd24 = crate::EnumBitfieldStruct<u8, Nd24_SPEC>;
        impl Nd24 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd25_SPEC;
        pub type Nd25 = crate::EnumBitfieldStruct<u8, Nd25_SPEC>;
        impl Nd25 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd26_SPEC;
        pub type Nd26 = crate::EnumBitfieldStruct<u8, Nd26_SPEC>;
        impl Nd26 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd27_SPEC;
        pub type Nd27 = crate::EnumBitfieldStruct<u8, Nd27_SPEC>;
        impl Nd27 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd28_SPEC;
        pub type Nd28 = crate::EnumBitfieldStruct<u8, Nd28_SPEC>;
        impl Nd28 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd29_SPEC;
        pub type Nd29 = crate::EnumBitfieldStruct<u8, Nd29_SPEC>;
        impl Nd29 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd30_SPEC;
        pub type Nd30 = crate::EnumBitfieldStruct<u8, Nd30_SPEC>;
        impl Nd30 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd31_SPEC;
        pub type Nd31 = crate::EnumBitfieldStruct<u8, Nd31_SPEC>;
        impl Nd31 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ndat2I_SPEC;
    impl crate::sealed::RegSpec for Ndat2I_SPEC {
        type DataType = u32;
    }
    #[doc = "New Data 2 0\n resetvalue={Application Reset:0x0}"]
    pub type Ndat2I = crate::RegValueT<Ndat2I_SPEC>;

    impl Ndat2I {
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd32(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, ndat2i::Nd32, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1,1,0,ndat2i::Nd32, Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd33(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, ndat2i::Nd33, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0x1,1,0,ndat2i::Nd33, Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd34(
            self,
        ) -> crate::common::RegisterField<2, 0x1, 1, 0, ndat2i::Nd34, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x1,1,0,ndat2i::Nd34, Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd35(
            self,
        ) -> crate::common::RegisterField<3, 0x1, 1, 0, ndat2i::Nd35, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<3,0x1,1,0,ndat2i::Nd35, Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd36(
            self,
        ) -> crate::common::RegisterField<4, 0x1, 1, 0, ndat2i::Nd36, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x1,1,0,ndat2i::Nd36, Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd37(
            self,
        ) -> crate::common::RegisterField<5, 0x1, 1, 0, ndat2i::Nd37, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x1,1,0,ndat2i::Nd37, Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd38(
            self,
        ) -> crate::common::RegisterField<6, 0x1, 1, 0, ndat2i::Nd38, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<6,0x1,1,0,ndat2i::Nd38, Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd39(
            self,
        ) -> crate::common::RegisterField<7, 0x1, 1, 0, ndat2i::Nd39, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<7,0x1,1,0,ndat2i::Nd39, Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd40(
            self,
        ) -> crate::common::RegisterField<8, 0x1, 1, 0, ndat2i::Nd40, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x1,1,0,ndat2i::Nd40, Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd41(
            self,
        ) -> crate::common::RegisterField<9, 0x1, 1, 0, ndat2i::Nd41, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<9,0x1,1,0,ndat2i::Nd41, Ndat2I_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd42(
            self,
        ) -> crate::common::RegisterField<10, 0x1, 1, 0, ndat2i::Nd42, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                ndat2i::Nd42,
                Ndat2I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd43(
            self,
        ) -> crate::common::RegisterField<11, 0x1, 1, 0, ndat2i::Nd43, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                ndat2i::Nd43,
                Ndat2I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd44(
            self,
        ) -> crate::common::RegisterField<12, 0x1, 1, 0, ndat2i::Nd44, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                12,
                0x1,
                1,
                0,
                ndat2i::Nd44,
                Ndat2I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd45(
            self,
        ) -> crate::common::RegisterField<13, 0x1, 1, 0, ndat2i::Nd45, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                13,
                0x1,
                1,
                0,
                ndat2i::Nd45,
                Ndat2I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd46(
            self,
        ) -> crate::common::RegisterField<14, 0x1, 1, 0, ndat2i::Nd46, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                ndat2i::Nd46,
                Ndat2I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd47(
            self,
        ) -> crate::common::RegisterField<15, 0x1, 1, 0, ndat2i::Nd47, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                ndat2i::Nd47,
                Ndat2I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd48(
            self,
        ) -> crate::common::RegisterField<16, 0x1, 1, 0, ndat2i::Nd48, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                16,
                0x1,
                1,
                0,
                ndat2i::Nd48,
                Ndat2I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd49(
            self,
        ) -> crate::common::RegisterField<17, 0x1, 1, 0, ndat2i::Nd49, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                17,
                0x1,
                1,
                0,
                ndat2i::Nd49,
                Ndat2I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd50(
            self,
        ) -> crate::common::RegisterField<18, 0x1, 1, 0, ndat2i::Nd50, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                18,
                0x1,
                1,
                0,
                ndat2i::Nd50,
                Ndat2I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd51(
            self,
        ) -> crate::common::RegisterField<19, 0x1, 1, 0, ndat2i::Nd51, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                19,
                0x1,
                1,
                0,
                ndat2i::Nd51,
                Ndat2I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd52(
            self,
        ) -> crate::common::RegisterField<20, 0x1, 1, 0, ndat2i::Nd52, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                20,
                0x1,
                1,
                0,
                ndat2i::Nd52,
                Ndat2I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd53(
            self,
        ) -> crate::common::RegisterField<21, 0x1, 1, 0, ndat2i::Nd53, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                21,
                0x1,
                1,
                0,
                ndat2i::Nd53,
                Ndat2I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd54(
            self,
        ) -> crate::common::RegisterField<22, 0x1, 1, 0, ndat2i::Nd54, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                22,
                0x1,
                1,
                0,
                ndat2i::Nd54,
                Ndat2I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd55(
            self,
        ) -> crate::common::RegisterField<23, 0x1, 1, 0, ndat2i::Nd55, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                23,
                0x1,
                1,
                0,
                ndat2i::Nd55,
                Ndat2I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd56(
            self,
        ) -> crate::common::RegisterField<24, 0x1, 1, 0, ndat2i::Nd56, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                24,
                0x1,
                1,
                0,
                ndat2i::Nd56,
                Ndat2I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd57(
            self,
        ) -> crate::common::RegisterField<25, 0x1, 1, 0, ndat2i::Nd57, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                25,
                0x1,
                1,
                0,
                ndat2i::Nd57,
                Ndat2I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd58(
            self,
        ) -> crate::common::RegisterField<26, 0x1, 1, 0, ndat2i::Nd58, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                26,
                0x1,
                1,
                0,
                ndat2i::Nd58,
                Ndat2I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd59(
            self,
        ) -> crate::common::RegisterField<27, 0x1, 1, 0, ndat2i::Nd59, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                27,
                0x1,
                1,
                0,
                ndat2i::Nd59,
                Ndat2I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd60(
            self,
        ) -> crate::common::RegisterField<28, 0x1, 1, 0, ndat2i::Nd60, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                28,
                0x1,
                1,
                0,
                ndat2i::Nd60,
                Ndat2I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd61(
            self,
        ) -> crate::common::RegisterField<29, 0x1, 1, 0, ndat2i::Nd61, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                29,
                0x1,
                1,
                0,
                ndat2i::Nd61,
                Ndat2I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd62(
            self,
        ) -> crate::common::RegisterField<30, 0x1, 1, 0, ndat2i::Nd62, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                30,
                0x1,
                1,
                0,
                ndat2i::Nd62,
                Ndat2I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "New Data in Rx Buffer 63   ND. The flag is set when the respective Rx Buffer has been updated from a        received frame. The flags remain set until the Host clears them. A flag        is cleared by writing a   8220 1  8221  to the corresponding bit position. Writing a          8220 0  8221  has no effect."]
        #[inline(always)]
        pub fn nd63(
            self,
        ) -> crate::common::RegisterField<31, 0x1, 1, 0, ndat2i::Nd63, Ndat2I_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                ndat2i::Nd63,
                Ndat2I_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for Ndat2I {
        #[inline(always)]
        fn default() -> Ndat2I {
            <crate::RegValueT<Ndat2I_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod ndat2i {
        pub struct Nd32_SPEC;
        pub type Nd32 = crate::EnumBitfieldStruct<u8, Nd32_SPEC>;
        impl Nd32 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd33_SPEC;
        pub type Nd33 = crate::EnumBitfieldStruct<u8, Nd33_SPEC>;
        impl Nd33 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd34_SPEC;
        pub type Nd34 = crate::EnumBitfieldStruct<u8, Nd34_SPEC>;
        impl Nd34 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd35_SPEC;
        pub type Nd35 = crate::EnumBitfieldStruct<u8, Nd35_SPEC>;
        impl Nd35 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd36_SPEC;
        pub type Nd36 = crate::EnumBitfieldStruct<u8, Nd36_SPEC>;
        impl Nd36 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd37_SPEC;
        pub type Nd37 = crate::EnumBitfieldStruct<u8, Nd37_SPEC>;
        impl Nd37 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd38_SPEC;
        pub type Nd38 = crate::EnumBitfieldStruct<u8, Nd38_SPEC>;
        impl Nd38 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd39_SPEC;
        pub type Nd39 = crate::EnumBitfieldStruct<u8, Nd39_SPEC>;
        impl Nd39 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd40_SPEC;
        pub type Nd40 = crate::EnumBitfieldStruct<u8, Nd40_SPEC>;
        impl Nd40 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd41_SPEC;
        pub type Nd41 = crate::EnumBitfieldStruct<u8, Nd41_SPEC>;
        impl Nd41 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd42_SPEC;
        pub type Nd42 = crate::EnumBitfieldStruct<u8, Nd42_SPEC>;
        impl Nd42 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd43_SPEC;
        pub type Nd43 = crate::EnumBitfieldStruct<u8, Nd43_SPEC>;
        impl Nd43 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd44_SPEC;
        pub type Nd44 = crate::EnumBitfieldStruct<u8, Nd44_SPEC>;
        impl Nd44 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd45_SPEC;
        pub type Nd45 = crate::EnumBitfieldStruct<u8, Nd45_SPEC>;
        impl Nd45 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd46_SPEC;
        pub type Nd46 = crate::EnumBitfieldStruct<u8, Nd46_SPEC>;
        impl Nd46 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd47_SPEC;
        pub type Nd47 = crate::EnumBitfieldStruct<u8, Nd47_SPEC>;
        impl Nd47 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd48_SPEC;
        pub type Nd48 = crate::EnumBitfieldStruct<u8, Nd48_SPEC>;
        impl Nd48 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd49_SPEC;
        pub type Nd49 = crate::EnumBitfieldStruct<u8, Nd49_SPEC>;
        impl Nd49 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd50_SPEC;
        pub type Nd50 = crate::EnumBitfieldStruct<u8, Nd50_SPEC>;
        impl Nd50 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd51_SPEC;
        pub type Nd51 = crate::EnumBitfieldStruct<u8, Nd51_SPEC>;
        impl Nd51 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd52_SPEC;
        pub type Nd52 = crate::EnumBitfieldStruct<u8, Nd52_SPEC>;
        impl Nd52 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd53_SPEC;
        pub type Nd53 = crate::EnumBitfieldStruct<u8, Nd53_SPEC>;
        impl Nd53 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd54_SPEC;
        pub type Nd54 = crate::EnumBitfieldStruct<u8, Nd54_SPEC>;
        impl Nd54 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd55_SPEC;
        pub type Nd55 = crate::EnumBitfieldStruct<u8, Nd55_SPEC>;
        impl Nd55 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd56_SPEC;
        pub type Nd56 = crate::EnumBitfieldStruct<u8, Nd56_SPEC>;
        impl Nd56 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd57_SPEC;
        pub type Nd57 = crate::EnumBitfieldStruct<u8, Nd57_SPEC>;
        impl Nd57 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd58_SPEC;
        pub type Nd58 = crate::EnumBitfieldStruct<u8, Nd58_SPEC>;
        impl Nd58 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd59_SPEC;
        pub type Nd59 = crate::EnumBitfieldStruct<u8, Nd59_SPEC>;
        impl Nd59 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd60_SPEC;
        pub type Nd60 = crate::EnumBitfieldStruct<u8, Nd60_SPEC>;
        impl Nd60 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd61_SPEC;
        pub type Nd61 = crate::EnumBitfieldStruct<u8, Nd61_SPEC>;
        impl Nd61 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd62_SPEC;
        pub type Nd62 = crate::EnumBitfieldStruct<u8, Nd62_SPEC>;
        impl Nd62 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Nd63_SPEC;
        pub type Nd63 = crate::EnumBitfieldStruct<u8, Nd63_SPEC>;
        impl Nd63 {
            #[doc = "0 Rx Buffer not        updated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Rx Buffer        updated from new message"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NpcRi_SPEC;
    impl crate::sealed::RegSpec for NpcRi_SPEC {
        type DataType = u32;
    }
    #[doc = "Node 0 Port Control Register\n resetvalue={Application Reset:0x0}"]
    pub type NpcRi = crate::RegValueT<NpcRi_SPEC>;

    impl NpcRi {
        #[doc = "Receive Select   RXSEL. RXSEL selects one out of 8 possible receive inputs. The CAN receive        signal is performed by the selected input.  see the device related        chapter for RXSEL"]
        #[inline(always)]
        pub fn rxsel(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, NpcRi_SPEC, crate::common::RW> {
            crate::common::RegisterField::<0,0x7,1,0,u8, NpcRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Loop Back Mode   LBM"]
        #[inline(always)]
        pub fn lbm(
            self,
        ) -> crate::common::RegisterField<8, 0x1, 1, 0, npcri::Lbm, NpcRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x1,1,0,npcri::Lbm, NpcRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Loop Back Mode Out   LOUT. This bit is not implemented in A step silicon. The loop back bus is switched to the external CAN bus of the node."]
        #[inline(always)]
        pub fn lout(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, NpcRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9,1,0,NpcRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable destructive read on ECR0.CEL   DELE. If this bit is set  the destructive read on ECRi.CEL and on the PSR        register takes place. Meaning  that with read access on ECRi  the CEL is        reset. The same is true for the PSR register  for the bits PXE  RFDF         RBRS  RESI  LEC and DLEC. After the destructive read it is advised to        reset the bit again."]
        #[inline(always)]
        pub fn dele(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, NpcRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,NpcRi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for NpcRi {
        #[inline(always)]
        fn default() -> NpcRi {
            <crate::RegValueT<NpcRi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod npcri {
        pub struct Lbm_SPEC;
        pub type Lbm = crate::EnumBitfieldStruct<u8, Lbm_SPEC>;
        impl Lbm {
            #[doc = "0 Loop Back Mode is disabled."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Loop Back Mode is enabled. This node is connected to an internal  virtual  loop back CAN bus. All CAN nodes which are in Loop Back Mode are connected to this virtual CAN bus so that they can communicate with each other internally. The external transmit line is forced recessive in Loop Back Mode."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PsRi_SPEC;
    impl crate::sealed::RegSpec for PsRi_SPEC {
        type DataType = u32;
    }
    #[doc = "Protocol Status Register 0\n resetvalue={Application Reset:0x707}"]
    pub type PsRi = crate::RegValueT<PsRi_SPEC>;

    impl PsRi {
        #[doc = "Last Error Code   LEC. The LEC indicates the type of the last error to occur on the CAN bus.        This field will be cleared to   8216 0  8217  when a message has been transferred         reception or transmission  without error. This bit field is set to 0x7        on read  if NPCRi.DELE is set. The Bus Off recovery sequence  see ISO11898 1  cannot be shortened by          setting or resetting CCCR.INIT. If the device goes Bus Off  it will          set CCCR.INIT of its own accord  stopping all bus activities. Once          CCCR.INIT has been cleared by the CPU  the device will then wait for          129 occurrences of Bus Idle  129   11 consecutive recessive bits           before resuming normal operation. At the end of the Bus Off recovery          sequence  the Error Management Counters will be reset. During the          waiting time after the resetting of CCCR.INIT  each time a sequence of          11 recessive bits has been monitored  a Bit0Error code is written to          PSR.LEC  enabling the CPU to readily check up whether the CAN bus is          stuck at dominant or continuously disturbed and to monitor the Bus Off          recovery sequence. ECR.REC is used to count these sequences."]
        #[inline(always)]
        pub fn lec(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, psri::Lec, PsRi_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0x7,1,0,psri::Lec, PsRi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Activity   ACT. Monitors the module s CAN communication state. ACT is set to  00  by a Protocol Exception Event."]
        #[inline(always)]
        pub fn act(
            self,
        ) -> crate::common::RegisterField<3, 0x3, 1, 0, psri::Act, PsRi_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<3,0x3,1,0,psri::Act, PsRi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Error Passive   EP"]
        #[inline(always)]
        pub fn ep(
            self,
        ) -> crate::common::RegisterField<5, 0x1, 1, 0, psri::Ep, PsRi_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<5,0x1,1,0,psri::Ep, PsRi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Warning Status   EW"]
        #[inline(always)]
        pub fn ew(
            self,
        ) -> crate::common::RegisterField<6, 0x1, 1, 0, psri::Ew, PsRi_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<6,0x1,1,0,psri::Ew, PsRi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Bus Off Status   BO"]
        #[inline(always)]
        pub fn bo(
            self,
        ) -> crate::common::RegisterField<7, 0x1, 1, 0, psri::Bo, PsRi_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<7,0x1,1,0,psri::Bo, PsRi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Data Phase Last Error Code   DLEC. Type of last error that occurred in the data phase of a CAN FD format        frame with its BRS flag set. Coding is the same as for LEC. This field        will be cleared to zero when a CAN FD format frame with its BRS flag set        has been transferred  reception or transmission  without error. This bit        field is set to 0x7 on read  if NPCRi.DELE is set. When a frame in CAN FD format has reached the data phase with BRS flag          set  the next CAN event  error or valid frame  will be shown in DLEC          instead of LEC. An error in a fixed stuff bit of a CAN FD CRC sequence          will be shown as a Form Error  not Stuff Error."]
        #[inline(always)]
        pub fn dlec(
            self,
        ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, PsRi_SPEC, crate::common::R> {
            crate::common::RegisterField::<8,0x7,1,0,u8, PsRi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "ESI flag of last received CAN FD Message   RESI. This bit is set together with REDF  independent of acceptance filtering. This bit is reset after read access  if NPCRi.DELE is set."]
        #[inline(always)]
        pub fn resi(
            self,
        ) -> crate::common::RegisterField<11, 0x1, 1, 0, psri::Resi, PsRi_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<11,0x1,1,0,psri::Resi, PsRi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BRS flag of last received CAN FD Message   RBRS. This bit is set together with REDF  independent of acceptance filtering. This bit is reset after read access  if NPCRi.DELE is set."]
        #[inline(always)]
        pub fn rbrs(
            self,
        ) -> crate::common::RegisterField<12, 0x1, 1, 0, psri::Rbrs, PsRi_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<12,0x1,1,0,psri::Rbrs, PsRi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Received a CAN FD Message   RFDF. This bit is set independent of acceptance filtering. This bit is reset after read access  if NPCRi.DELE is set."]
        #[inline(always)]
        pub fn rfdf(
            self,
        ) -> crate::common::RegisterField<13, 0x1, 1, 0, psri::Rfdf, PsRi_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<13,0x1,1,0,psri::Rfdf, PsRi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Protocol Exception Event   PXE. This bit is reset after read access  if NPCRi.DELE is set."]
        #[inline(always)]
        pub fn pxe(
            self,
        ) -> crate::common::RegisterField<14, 0x1, 1, 0, psri::Pxe, PsRi_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<14,0x1,1,0,psri::Pxe, PsRi_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Transmitter Delay Compensation Value   TDCV. Position of the secondary sample point  defined by the sum of the        measured delay from TX to RX and TDCR.TDCO. The SSP position is  in the        data phase  the number of mtq between the start of the transmitted bit        and the secondary sample point. Valid values are 0 to 127 mtq."]
        #[inline(always)]
        pub fn tdcv(
            self,
        ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, PsRi_SPEC, crate::common::R> {
            crate::common::RegisterField::<16,0x7f,1,0,u8, PsRi_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for PsRi {
        #[inline(always)]
        fn default() -> PsRi {
            <crate::RegValueT<PsRi_SPEC> as RegisterValue<_>>::new(1799)
        }
    }
    pub mod psri {
        pub struct Lec_SPEC;
        pub type Lec = crate::EnumBitfieldStruct<u8, Lec_SPEC>;
        impl Lec {
            #[doc = "0 No Error  No error occurred since LEC has been reset by successful reception or transmission."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Stuff Error  More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "2 Form Error  A fixed format part of a received frame has the wrong format."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "3 Ack Error  The message transmitted by the M CAN was not acknowledged by another node."]
            pub const CONST_33: Self = Self::new(3);
            #[doc = "4 Bit1 Error  During the transmission of a message  with the exception of the arbitration field   the device wanted to send a recessive level  bit of logical value  1    but the monitored bus value was dominant."]
            pub const CONST_44: Self = Self::new(4);
            #[doc = "5 Bit0 Error  During the transmission of a message  or acknowledge bit  or active error flag  or overload flag   the device wanted to send a dominant level  data or identifier bit logical value  0    but the monitored bus value was recessive. During Bus Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus Off recovery sequence  indicating the bus is not stuck at dominant or continuously disturbed ."]
            pub const CONST_55: Self = Self::new(5);
            #[doc = "6 CRC Error  The CRC check sum of a received message was incorrect. The CRC of an incoming message does not match with the CRC calculated from the received data."]
            pub const CONST_66: Self = Self::new(6);
            #[doc = "7 No Change  Any read access to the Protocol Status Register re initializes the LEC to  7 . When the LEC shows the value  7   no CAN bus event was detected since the last CPU read access to the Protocol Status Register."]
            pub const CONST_77: Self = Self::new(7);
        }
        pub struct Act_SPEC;
        pub type Act = crate::EnumBitfieldStruct<u8, Act_SPEC>;
        impl Act {
            #[doc = "00 Synchronizing   node is synchronizing on CAN communication"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Idle   node is neither receiver nor transmitter"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Receiver   node is operating as receiver"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Transmitter   node is operating as transmitter"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Ep_SPEC;
        pub type Ep = crate::EnumBitfieldStruct<u8, Ep_SPEC>;
        impl Ep {
            #[doc = "0 The M CAN is in the Error Active state. It normally takes part in bus communication and sends an active error flag when an error has been detected"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 The M CAN is in the Error Passive state"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Ew_SPEC;
        pub type Ew = crate::EnumBitfieldStruct<u8, Ew_SPEC>;
        impl Ew {
            #[doc = "0 Both error counters are below the Error Warning limit of 96"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 At least one of error counter has reached the Error Warning limit of 96"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Bo_SPEC;
        pub type Bo = crate::EnumBitfieldStruct<u8, Bo_SPEC>;
        impl Bo {
            #[doc = "0 The M CAN is not in Bus Off The Bus Off recovery sequence  see ISO11898 1  cannot be shortened by setting or resetting CCCR.INIT. If the device goes Bus Off  it will set CCCR.INIT of its own accord  stopping all bus activities. Once CCCR.INIT has been cleared by the CPU  the device will then wait for 129 occurrences of Bus Idle  129   11 consecutive recessive bits  before resuming normal operation. At the end of the Bus Off recovery sequence  the Error Management Counters will be reset. During the waiting time after the resetting of CCCR.INIT  each time a sequence of 11 recessive bits has been monitored  a Bit0Error code is written to PSR.LEC  enabling the CPU to readily check up whether the CAN bus is stuck at dominant or continuously disturbed and to monitor the Bus Off recovery sequence. ECR.REC is used to count these sequences."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 The M CAN is in Bus Off state"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Resi_SPEC;
        pub type Resi = crate::EnumBitfieldStruct<u8, Resi_SPEC>;
        impl Resi {
            #[doc = "0 Last received CAN FD message did not have its ESI flag set"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Last received CAN FD message had its ESI flag set"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rbrs_SPEC;
        pub type Rbrs = crate::EnumBitfieldStruct<u8, Rbrs_SPEC>;
        impl Rbrs {
            #[doc = "0 Last received CAN FD message did not have its BRS flag set"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Last received CAN FD message had its BRS flag set"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rfdf_SPEC;
        pub type Rfdf = crate::EnumBitfieldStruct<u8, Rfdf_SPEC>;
        impl Rfdf {
            #[doc = "0 Since this bit was reset by the CPU  no CAN FD message has been received"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Message in CAN FD format with FDF flag set  has been received"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Pxe_SPEC;
        pub type Pxe = crate::EnumBitfieldStruct<u8, Pxe_SPEC>;
        impl Pxe {
            #[doc = "0 No protocol exception event occurred since last read access"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Protocol exception event occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RwDi_SPEC;
    impl crate::sealed::RegSpec for RwDi_SPEC {
        type DataType = u32;
    }
    #[doc = "RAM Watchdog 0\n resetvalue={Application Reset:0x0}"]
    pub type RwDi = crate::RegValueT<RwDi_SPEC>;

    impl RwDi {
        #[doc = "Watchdog Configuration   WDC. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set. Start value of the Message RAM Watchdog Counter. With the reset value of  00  the counter is disabled."]
        #[inline(always)]
        pub fn wdc(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, rwdi::Wdc, RwDi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xff,1,0,rwdi::Wdc, RwDi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Watchdog Value   WDV. Actual Message RAM Watchdog Counter Value."]
        #[inline(always)]
        pub fn wdv(
            self,
        ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, RwDi_SPEC, crate::common::R> {
            crate::common::RegisterField::<8,0xff,1,0,u8, RwDi_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for RwDi {
        #[inline(always)]
        fn default() -> RwDi {
            <crate::RegValueT<RwDi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod rwdi {
        pub struct Wdc_SPEC;
        pub type Wdc = crate::EnumBitfieldStruct<u8, Wdc_SPEC>;
        impl Wdc {
            #[doc = "0 Watchdog disabled"]
            pub const CONST_00: Self = Self::new(0);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SidfCi_SPEC;
    impl crate::sealed::RegSpec for SidfCi_SPEC {
        type DataType = u32;
    }
    #[doc = "Standard ID Filter Configuration 0\n resetvalue={Application Reset:0x0}"]
    pub type SidfCi = crate::RegValueT<SidfCi_SPEC>;

    impl SidfCi {
        #[doc = "Filter List Standard Start Address   FLSSA. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Start address of standard Message ID filter list  32 bit word address ."]
        #[inline(always)]
        pub fn flssa(
            self,
        ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, SidfCi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x3fff,1,0,u16, SidfCi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "List Size Standard   LSS. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
        #[inline(always)]
        pub fn lss(
            self,
        ) -> crate::common::RegisterField<16, 0xff, 1, 0, sidfci::Lss, SidfCi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                16,
                0xff,
                1,
                0,
                sidfci::Lss,
                SidfCi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for SidfCi {
        #[inline(always)]
        fn default() -> SidfCi {
            <crate::RegValueT<SidfCi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod sidfci {
        pub struct Lss_SPEC;
        pub type Lss = crate::EnumBitfieldStruct<u8, Lss_SPEC>;
        impl Lss {
            #[doc = "0 No standard Message ID filter"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "128 Message ID filter elements"]
            pub const REST_255: Self = Self::new(255);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct StartadRi_SPEC;
    impl crate::sealed::RegSpec for StartadRi_SPEC {
        type DataType = u32;
    }
    #[doc = "Start Address Node 0\n resetvalue={Application Reset:0x0}"]
    pub type StartadRi = crate::RegValueT<StartadRi_SPEC>;

    impl StartadRi {
        #[doc = "Message RAM start   START. The address within the RAM area of the MCMCAN          of node i  where the message RAM to be protected starts"]
        #[inline(always)]
        pub fn start(
            self,
        ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, StartadRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x3fff,1,0,u16, StartadRi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for StartadRi {
        #[inline(always)]
        fn default() -> StartadRi {
            <crate::RegValueT<StartadRi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TdcRi_SPEC;
    impl crate::sealed::RegSpec for TdcRi_SPEC {
        type DataType = u32;
    }
    #[doc = "Transmitter Delay Compensation Register 0\n resetvalue={Application Reset:0x0}"]
    pub type TdcRi = crate::RegValueT<TdcRi_SPEC>;

    impl TdcRi {
        #[doc = "Transmitter Delay Compensation Filter Window Length   TDCF. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Defines the minimum value for the Secondary Sample        Point position  dominant edges on RX that would result in an earlier        Secondary Sample Point position are ignored for transmitter delay        measurement. This feature is enabled when TDCF is configured to a value        greater than TDCO. Valid values are from 0 to 127 mtq."]
        #[inline(always)]
        pub fn tdcf(
            self,
        ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, TdcRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7f,1,0,u8, TdcRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Transmitter Delay Compensation Offset   TDCO. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Offset value defining the distance between the        measured delay from TX to RX and the secondary sample point. Valid        values are 0 to 127 mtq. The duration of one mtq is equal to the fASYNi        clock period."]
        #[inline(always)]
        pub fn tdco(
            self,
        ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, TdcRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x7f,1,0,u8, TdcRi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for TdcRi {
        #[inline(always)]
        fn default() -> TdcRi {
            <crate::RegValueT<TdcRi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TesTi_SPEC;
    impl crate::sealed::RegSpec for TesTi_SPEC {
        type DataType = u32;
    }
    #[doc = "Test Register 0\n resetvalue={Application Reset:0x0}"]
    pub type TesTi = crate::RegValueT<TesTi_SPEC>;

    impl TesTi {
        #[doc = "Loop Back Mode   LBCK. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. This is the external loop back mode  visible on the outside."]
        #[inline(always)]
        pub fn lbck(
            self,
        ) -> crate::common::RegisterField<4, 0x1, 1, 0, testi::Lbck, TesTi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x1,1,0,testi::Lbck, TesTi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Control of Transmit Pin   TX. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
        #[inline(always)]
        pub fn tx(
            self,
        ) -> crate::common::RegisterField<5, 0x3, 1, 0, testi::Tx, TesTi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x3,1,0,testi::Tx, TesTi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Receive Pin   RX. Monitors the actual value of RX pin."]
        #[inline(always)]
        pub fn rx(
            self,
        ) -> crate::common::RegisterField<7, 0x1, 1, 0, testi::Rx, TesTi_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<7,0x1,1,0,testi::Rx, TesTi_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for TesTi {
        #[inline(always)]
        fn default() -> TesTi {
            <crate::RegValueT<TesTi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod testi {
        pub struct Lbck_SPEC;
        pub type Lbck = crate::EnumBitfieldStruct<u8, Lbck_SPEC>;
        impl Lbck {
            #[doc = "0 Reset value  Loop Back Mode is disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Loop Back Mode        is enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tx_SPEC;
        pub type Tx = crate::EnumBitfieldStruct<u8, Tx_SPEC>;
        impl Tx {
            #[doc = "00 Reset value  TX pin controlled by the CAN Core  updated at the end of the CAN bit time"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Sample Point can be monitored at the TX pin"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Dominant   0   level at TX pin."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Recessive   1   at RX pin."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Rx_SPEC;
        pub type Rx = crate::EnumBitfieldStruct<u8, Rx_SPEC>;
        impl Rx {
            #[doc = "0 The CAN bus is dominant  RXD    0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 The CAN bus is recessive  RXD    1"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TocCi_SPEC;
    impl crate::sealed::RegSpec for TocCi_SPEC {
        type DataType = u32;
    }
    #[doc = "Timeout Counter Configuration 0\n resetvalue={Application Reset:0x0FFFF0000}"]
    pub type TocCi = crate::RegValueT<TocCi_SPEC>;

    impl TocCi {
        #[doc = "Enable Timeout Counter   ETOC. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. For use of timeout function with CAN FD see chapter Timeout Counter."]
        #[inline(always)]
        pub fn etoc(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, tocci::Etoc, TocCi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1,1,0,tocci::Etoc, TocCi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Timeout Select   TOS. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set. When operating in Continuous mode  a write to TOCV presets the counter to the value configured by TOCC.TOP and continues down counting. When the Timeout Counter is controlled by one of the FIFOs  an empty FIFO presets the counter to the value configured by TOCC.TOP. Down counting is started when the first FIFO element is stored."]
        #[inline(always)]
        pub fn tos(
            self,
        ) -> crate::common::RegisterField<1, 0x3, 1, 0, tocci::Tos, TocCi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0x3,1,0,tocci::Tos, TocCi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Timeout Period   TOP. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set. Start value of the Timeout Counter  down counter . Configures the Timeout Period."]
        #[inline(always)]
        pub fn top(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, TocCi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, TocCi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for TocCi {
        #[inline(always)]
        fn default() -> TocCi {
            <crate::RegValueT<TocCi_SPEC> as RegisterValue<_>>::new(4294901760)
        }
    }
    pub mod tocci {
        pub struct Etoc_SPEC;
        pub type Etoc = crate::EnumBitfieldStruct<u8, Etoc_SPEC>;
        impl Etoc {
            #[doc = "0 Timeout Counter disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Timeout Counter enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tos_SPEC;
        pub type Tos = crate::EnumBitfieldStruct<u8, Tos_SPEC>;
        impl Tos {
            #[doc = "00 Continuous operation"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Timeout controlled by Tx Event FIFO"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Timeout controlled by Rx FIFO 0"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Timeout controlled by Rx FIFO 1"]
            pub const CONST_33: Self = Self::new(3);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TocVi_SPEC;
    impl crate::sealed::RegSpec for TocVi_SPEC {
        type DataType = u32;
    }
    #[doc = "Timeout Counter Value 0\n resetvalue={Application Reset:0x0FFFF}"]
    pub type TocVi = crate::RegValueT<TocVi_SPEC>;

    impl TocVi {
        #[doc = "Timeout Counter   TOC. The Timeout Counter is decremented in multiples of CAN bit times    8201 1  8230 16  8201           depending on the configuration of TSCC.TCP. When decremented to zero         interrupt flag IR.TOO is set and the Timeout Counter is stopped. Start        and reset restart conditions are configured via TOCC.TOS. Any write access will lead to clearing of the counter."]
        #[inline(always)]
        pub fn toc(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, TocVi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, TocVi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for TocVi {
        #[inline(always)]
        fn default() -> TocVi {
            <crate::RegValueT<TocVi_SPEC> as RegisterValue<_>>::new(65535)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TscCi_SPEC;
    impl crate::sealed::RegSpec for TscCi_SPEC {
        type DataType = u32;
    }
    #[doc = "Timestamp Counter Configuration 0\n resetvalue={Application Reset:0x0}"]
    pub type TscCi = crate::RegValueT<TscCi_SPEC>;

    impl TscCi {
        #[doc = "Time segment before sample point   TSS. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
        #[inline(always)]
        pub fn tss(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, tscci::Tss, TscCi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3,1,0,tscci::Tss, TscCi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Timestamp Counter Prescaler   TCP. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Configures the timestamp and timeout counters time unit in multiples of        CAN bit times    8201 1  8230 16  8201  . The actual interpretation by the hardware of        this value is such that one more than the value programmed here is used. With CAN FD an external counter is required for timestamp generation           TSS     8220 10  8221"]
        #[inline(always)]
        pub fn tcp(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, TscCi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, TscCi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for TscCi {
        #[inline(always)]
        fn default() -> TscCi {
            <crate::RegValueT<TscCi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod tscci {
        pub struct Tss_SPEC;
        pub type Tss = crate::EnumBitfieldStruct<u8, Tss_SPEC>;
        impl Tss {
            #[doc = "Timestamp counter value always 0x0000"]
            pub const CONST_33: Self = Self::new(3);
            #[doc = "Timestamp counter value incremented according to TCP"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "This timer is not implemented on A step silicon. External        timestamp counter value used  timer to be started in NTCCRy  the clock        source as well as the chosen prescaler has to be configured before using        this feature."]
            pub const CONST_22: Self = Self::new(2);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TscVi_SPEC;
    impl crate::sealed::RegSpec for TscVi_SPEC {
        type DataType = u32;
    }
    #[doc = "Timestamp Counter Value 0\n resetvalue={Application Reset:0x0}"]
    pub type TscVi = crate::RegValueT<TscVi_SPEC>;

    impl TscVi {
        #[doc = "Timestamp Counter   TSC. The internal external Timestamp Counter value is captured on start of        frame  both Rx and Tx . When TSCC.TSS     8220 01  8221   the Timestamp Counter is        incremented in multiples of CAN bit times    8201 1  8230 16  8201   depending on the        configuration of TSCC.TCP. A wrap around sets interrupt flag IR.TSW. Write access resets the counter to zero. When TSCC.TSS  160    160   8220 10  8221   TSC reflects the external Timestamp Counter value.        A write access has no impact. A   8220 wrap around  8221  is a change of the Timestamp Counter value from          non zero to zero not caused by write access to TSCV."]
        #[inline(always)]
        pub fn tsc(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, TscVi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, TscVi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for TscVi {
        #[inline(always)]
        fn default() -> TscVi {
            <crate::RegValueT<TscVi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TtcRi_SPEC;
    impl crate::sealed::RegSpec for TtcRi_SPEC {
        type DataType = u32;
    }
    #[doc = "Time Trigger Control Register\n resetvalue={Application Reset:0x0}"]
    pub type TtcRi = crate::RegValueT<TtcRi_SPEC>;

    impl TtcRi {
        #[doc = "External Trigger Event Selection   ETESEL. This bit field defines the external trigger event that can be used to        trigger the transmission of the reference message. The event causes the        Event Trigger to be triggered. Control settings for this will not be        influenced."]
        #[inline(always)]
        pub fn etesel(
            self,
        ) -> crate::common::RegisterField<2, 0x3, 1, 0, ttcri::Etesel, TtcRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x3,1,0,ttcri::Etesel, TtcRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "External Trigger Source Selection   ETSSEL. This bit fields selects the input source for the external reference        message trigger."]
        #[inline(always)]
        pub fn etssel(
            self,
        ) -> crate::common::RegisterField<4, 0x7, 1, 0, ttcri::Etssel, TtcRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x7,1,0,ttcri::Etssel, TtcRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "TTCapture Time Trigger Source Select   TTCTSS. This bit selects the input source for the TT Capture Time  TTCPT         trigger. This register influences the stop watch event trigger"]
        #[inline(always)]
        pub fn ttctss(
            self,
        ) -> crate::common::RegisterField<9, 0x7, 1, 0, ttcri::Ttctss, TtcRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<9,0x7,1,0,ttcri::Ttctss, TtcRi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for TtcRi {
        #[inline(always)]
        fn default() -> TtcRi {
            <crate::RegValueT<TtcRi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod ttcri {
        pub struct Etesel_SPEC;
        pub type Etesel = crate::EnumBitfieldStruct<u8, Etesel_SPEC>;
        impl Etesel {
            #[doc = "00 The external        event ECTTx does not trigger the transmission of the reference message."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 The reference message will be transmitted when a negative edge is detected at the selected input line ECTTx."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 The reference message will be transmitted when a positive edge is detected at the input line ECTTx."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 The reference message will be transmitted when a negative edge or a positive edge is detected at the input line ECTTx."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Etssel_SPEC;
        pub type Etssel = crate::EnumBitfieldStruct<u8, Etssel_SPEC>;
        impl Etssel {
            #[doc = "000 External trigger input line ECTT1 selected"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "001 External trigger input line ECTT2 selected"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "010 External trigger input line ECTT3 selected"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "011 External trigger input line ECTT4 selected"]
            pub const CONST_33: Self = Self::new(3);
            #[doc = "100 External trigger input line ECTT5 selected"]
            pub const CONST_44: Self = Self::new(4);
            #[doc = "101 External trigger input line ECTT6 selected"]
            pub const CONST_55: Self = Self::new(5);
            #[doc = "110 External trigger input line ECTT7 selected"]
            pub const CONST_66: Self = Self::new(6);
            #[doc = "111 External trigger input line ECTT8 selected"]
            pub const CONST_77: Self = Self::new(7);
        }
        pub struct Ttctss_SPEC;
        pub type Ttctss = crate::EnumBitfieldStruct<u8, Ttctss_SPEC>;
        impl Ttctss {
            #[doc = "No TTCPT trigger input allowed"]
            pub const CONST_00: Self = Self::new(0);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct XidaMi_SPEC;
    impl crate::sealed::RegSpec for XidaMi_SPEC {
        type DataType = u32;
    }
    #[doc = "Extended ID AND Mask 0\n resetvalue={Application Reset:0x1FFFFFFF}"]
    pub type XidaMi = crate::RegValueT<XidaMi_SPEC>;

    impl XidaMi {
        #[doc = "Extended ID Mask   EIDM. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set. For acceptance filtering of extended frames the Extended ID AND Mask is ANDed with the Message ID of a received frame. Intended for masking of 29 bit IDs in SAE J1939. With the reset value of all bits set to one the mask is not active."]
        #[inline(always)]
        pub fn eidm(
            self,
        ) -> crate::common::RegisterField<0, 0x1fffffff, 1, 0, u32, XidaMi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1fffffff,1,0,u32, XidaMi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for XidaMi {
        #[inline(always)]
        fn default() -> XidaMi {
            <crate::RegValueT<XidaMi_SPEC> as RegisterValue<_>>::new(536870911)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct XidfCi_SPEC;
    impl crate::sealed::RegSpec for XidfCi_SPEC {
        type DataType = u32;
    }
    #[doc = "Extended ID Filter Configuration 0\n resetvalue={Application Reset:0x0}"]
    pub type XidfCi = crate::RegValueT<XidfCi_SPEC>;

    impl XidfCi {
        #[doc = "Filter List Extended Start Address   FLESA. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Start address of extended Message ID filter list  32 bit word addess ."]
        #[inline(always)]
        pub fn flesa(
            self,
        ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, XidfCi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x3fff,1,0,u16, XidfCi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "List Size Extended   LSE. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
        #[inline(always)]
        pub fn lse(
            self,
        ) -> crate::common::RegisterField<16, 0x7f, 1, 0, xidfci::Lse, XidfCi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                16,
                0x7f,
                1,
                0,
                xidfci::Lse,
                XidfCi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for XidfCi {
        #[inline(always)]
        fn default() -> XidfCi {
            <crate::RegValueT<XidfCi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod xidfci {
        pub struct Lse_SPEC;
        pub type Lse = crate::EnumBitfieldStruct<u8, Lse_SPEC>;
        impl Lse {
            #[doc = "No standard Message ID filter"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "64 extended Message ID filter elements"]
            pub const REST_127: Self = Self::new(127);
        }
    }
    #[doc = "NT"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nt(pub(super) *mut u8);
    unsafe impl core::marker::Send for Nt {}
    unsafe impl core::marker::Sync for Nt {}
    impl Nt {
        #[doc = "Node 0 Timer A Transmit Trigger Register\n resetvalue={Application Reset:0x10000}"]
        #[inline(always)]
        pub const fn ntattri(&self) -> crate::common::Reg<nt::NtattRi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "Node 0 Timer B Transmit Trigger Register\n resetvalue={Application Reset:0x20000}"]
        #[inline(always)]
        pub const fn ntbttri(&self) -> crate::common::Reg<nt::NtbttRi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "Node 0 Timer Clock Control Register\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ntccri(&self) -> crate::common::Reg<nt::NtccRi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "Node 0 Timer C Transmit Trigger Register\n resetvalue={Application Reset:0x30000}"]
        #[inline(always)]
        pub const fn ntcttri(&self) -> crate::common::Reg<nt::NtcttRi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "Node 0 Timer Receive Timeout Register\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ntrtri(&self) -> crate::common::Reg<nt::NtrtRi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
        }
    }
    pub mod nt {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct NtattRi_SPEC;
        impl crate::sealed::RegSpec for NtattRi_SPEC {
            type DataType = u32;
        }
        #[doc = "Node 0 Timer A Transmit Trigger Register\n resetvalue={Application Reset:0x10000}"]
        pub type NtattRi = crate::RegValueT<NtattRi_SPEC>;

        impl NtattRi {
            #[doc = "Reload Value   RELOAD. This bit field contains the reload value for the timer. The timer will        restart when RELOAD is written."]
            #[inline(always)]
            pub fn reload(
                self,
            ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, NtattRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xffff,1,0,u16, NtattRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmit Message Object   TXMO. This transmit trigger is fixed to transmit buffer 1"]
            #[inline(always)]
            pub fn txmo(
                self,
            ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, NtattRi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<16,0xff,1,0,u8, NtattRi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Timer Start   STRT. This bit field controls the operation of the timer."]
            #[inline(always)]
            pub fn strt(
                self,
            ) -> crate::common::RegisterField<
                24,
                0x1,
                1,
                0,
                ntattri::Strt,
                NtattRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    24,
                    0x1,
                    1,
                    0,
                    ntattri::Strt,
                    NtattRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for NtattRi {
            #[inline(always)]
            fn default() -> NtattRi {
                <crate::RegValueT<NtattRi_SPEC> as RegisterValue<_>>::new(65536)
            }
        }
        pub mod ntattri {
            pub struct Strt_SPEC;
            pub type Strt = crate::EnumBitfieldStruct<u8, Strt_SPEC>;
            impl Strt {
                #[doc = "0 Timer is        stopped."]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Timer is        started."]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct NtbttRi_SPEC;
        impl crate::sealed::RegSpec for NtbttRi_SPEC {
            type DataType = u32;
        }
        #[doc = "Node 0 Timer B Transmit Trigger Register\n resetvalue={Application Reset:0x20000}"]
        pub type NtbttRi = crate::RegValueT<NtbttRi_SPEC>;

        impl NtbttRi {
            #[doc = "Reload Value   RELOAD. This bit field contains the reload value for the timer. The timer will restart when RELOAD is written."]
            #[inline(always)]
            pub fn reload(
                self,
            ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, NtbttRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xffff,1,0,u16, NtbttRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmit Message Object   TXMO. This transmit object is fixed to transmit buffer 2"]
            #[inline(always)]
            pub fn txmo(
                self,
            ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, NtbttRi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<16,0xff,1,0,u8, NtbttRi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Timer Start   STRT. This bit field controls the operation of the timer."]
            #[inline(always)]
            pub fn strt(
                self,
            ) -> crate::common::RegisterField<
                24,
                0x1,
                1,
                0,
                ntbttri::Strt,
                NtbttRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    24,
                    0x1,
                    1,
                    0,
                    ntbttri::Strt,
                    NtbttRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for NtbttRi {
            #[inline(always)]
            fn default() -> NtbttRi {
                <crate::RegValueT<NtbttRi_SPEC> as RegisterValue<_>>::new(131072)
            }
        }
        pub mod ntbttri {
            pub struct Strt_SPEC;
            pub type Strt = crate::EnumBitfieldStruct<u8, Strt_SPEC>;
            impl Strt {
                #[doc = "0 Timer is stopped."]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Timer is started."]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct NtccRi_SPEC;
        impl crate::sealed::RegSpec for NtccRi_SPEC {
            type DataType = u32;
        }
        #[doc = "Node 0 Timer Clock Control Register\n resetvalue={Application Reset:0x0}"]
        pub type NtccRi = crate::RegValueT<NtccRi_SPEC>;

        impl NtccRi {
            #[doc = "Timer Prescaler   TPSC. The duration of one timer clock is given by  TPSC  160    160 1  CAN bit times for        all NTCCRi.TRIGSRC settings."]
            #[inline(always)]
            pub fn tpsc(
                self,
            ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, NtccRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<8,0xf,1,0,u8, NtccRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Stamping Reset   STRESET. This bit is not implemented in A step silicon. This bit gives the possibility to reset the time stamp for CAN FD        messages."]
            #[inline(always)]
            pub fn streset(
                self,
            ) -> crate::common::RegisterFieldBool<14, 1, 0, NtccRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<14,1,0,NtccRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Stamping Start   STSTART. This bit is not implemented in A step silicon. This bit starts the external timer used for CAN FD messages. The source        and the prescaler are identical to the timers A B C."]
            #[inline(always)]
            pub fn ststart(
                self,
            ) -> crate::common::RegisterFieldBool<15, 1, 0, NtccRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<15,1,0,NtccRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Trigger Source   TRIGSRC. This bit selects the trigger source for the different modes in the node        timer."]
            #[inline(always)]
            pub fn trigsrc(
                self,
            ) -> crate::common::RegisterField<
                18,
                0x7,
                1,
                0,
                ntccri::Trigsrc,
                NtccRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    18,
                    0x7,
                    1,
                    0,
                    ntccri::Trigsrc,
                    NtccRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for NtccRi {
            #[inline(always)]
            fn default() -> NtccRi {
                <crate::RegValueT<NtccRi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod ntccri {
            pub struct Trigsrc_SPEC;
            pub type Trigsrc = crate::EnumBitfieldStruct<u8, Trigsrc_SPEC>;
            impl Trigsrc {
                #[doc = "Node i Timer is decremented per f SYNi prescaled by  TPSC   1  timing to 0."]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "System Timer  STM  trigger event enabled. Node i Timer is decremented per STM trigger event prescaled by  TPSC          1 ."]
                pub const CONST_11: Self = Self::new(1);
                #[doc = "General Timer  GTM  trigger event enabled. Node i Timer is decremented per GTM trigger event prescaled by  TPSC          1 ."]
                pub const CONST_22: Self = Self::new(2);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct NtcttRi_SPEC;
        impl crate::sealed::RegSpec for NtcttRi_SPEC {
            type DataType = u32;
        }
        #[doc = "Node 0 Timer C Transmit Trigger Register\n resetvalue={Application Reset:0x30000}"]
        pub type NtcttRi = crate::RegValueT<NtcttRi_SPEC>;

        impl NtcttRi {
            #[doc = "Reload Value   RELOAD. This bit field contains the reload value for the timer. The timer will        restart when RELOAD is written."]
            #[inline(always)]
            pub fn reload(
                self,
            ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, NtcttRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xffff,1,0,u16, NtcttRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Transmit Message Object   TXMO. This transmit trigger is fixed to transmit buffer 3"]
            #[inline(always)]
            pub fn txmo(
                self,
            ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, NtcttRi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<16,0xff,1,0,u8, NtcttRi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Timer Start   STRT. This bit field controls the operation of the timer."]
            #[inline(always)]
            pub fn strt(
                self,
            ) -> crate::common::RegisterField<
                24,
                0x1,
                1,
                0,
                ntcttri::Strt,
                NtcttRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    24,
                    0x1,
                    1,
                    0,
                    ntcttri::Strt,
                    NtcttRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for NtcttRi {
            #[inline(always)]
            fn default() -> NtcttRi {
                <crate::RegValueT<NtcttRi_SPEC> as RegisterValue<_>>::new(196608)
            }
        }
        pub mod ntcttri {
            pub struct Strt_SPEC;
            pub type Strt = crate::EnumBitfieldStruct<u8, Strt_SPEC>;
            impl Strt {
                #[doc = "0 Timer is stopped."]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Timer is started."]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct NtrtRi_SPEC;
        impl crate::sealed::RegSpec for NtrtRi_SPEC {
            type DataType = u32;
        }
        #[doc = "Node 0 Timer Receive Timeout Register\n resetvalue={Application Reset:0x0}"]
        pub type NtrtRi = crate::RegValueT<NtrtRi_SPEC>;

        impl NtrtRi {
            #[doc = "Reload Value   RELOAD. This bit field contains the reload value for the timer. The timer will        start when RELOAD   8800  0 is written. After half the time of the RELOAD        value  the interrupt flags of the receive buffers will be cleared        automatically  to ensure  that no message receive will be missed."]
            #[inline(always)]
            pub fn reload(
                self,
            ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, NtrtRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xffff,1,0,u16, NtrtRi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Timer Event Interrupt Enable   TEIE. This bit enables the node timer event interrupt of CAN node i. Bit field GRINT2.RETI selects the interrupt output line which becomes        activated at this type of interrupt."]
            #[inline(always)]
            pub fn teie(
                self,
            ) -> crate::common::RegisterField<
                22,
                0x1,
                1,
                0,
                ntrtri::Teie,
                NtrtRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    22,
                    0x1,
                    1,
                    0,
                    ntrtri::Teie,
                    NtrtRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Timer Event   TE. This flag is set on a node timer transition from 1 to 0 in Receive        Timeout Mode. This bit must be reset  i.e Write to   8216 0  8217   by software         writing a   8216 1  8217  has no effect. An interrupt request is generated if TEIE   1."]
            #[inline(always)]
            pub fn te(
                self,
            ) -> crate::common::RegisterField<
                23,
                0x1,
                1,
                0,
                ntrtri::Te,
                NtrtRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    23,
                    0x1,
                    1,
                    0,
                    ntrtri::Te,
                    NtrtRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for NtrtRi {
            #[inline(always)]
            fn default() -> NtrtRi {
                <crate::RegValueT<NtrtRi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod ntrtri {
            pub struct Teie_SPEC;
            pub type Teie = crate::EnumBitfieldStruct<u8, Teie_SPEC>;
            impl Teie {
                #[doc = "0 Timer event interrupt is disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Timer event interrupt is enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Te_SPEC;
            pub type Te = crate::EnumBitfieldStruct<u8, Te_SPEC>;
            impl Te {
                #[doc = "0 No timer event has occurred since last flag reset"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Timer event has occurred since last flag reset"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
    }
    #[doc = "RX"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rx(pub(super) *mut u8);
    unsafe impl core::marker::Send for Rx {}
    unsafe impl core::marker::Sync for Rx {}
    impl Rx {
        #[doc = "Rx Buffer Configuration 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn rxbci(&self) -> crate::common::Reg<rx::RxbCi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "Rx Buffer FIFO Element Size Configuration 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn rxesci(&self) -> crate::common::Reg<rx::RxesCi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
        }
        #[doc = "Rx FIFO 0 Acknowledge 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn rxf0ai(&self) -> crate::common::Reg<rx::Rxf0Ai_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "Rx FIFO 0 Configuration 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn rxf0ci(&self) -> crate::common::Reg<rx::Rxf0Ci_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "Rx FIFO 0 Status 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn rxf0si(&self) -> crate::common::Reg<rx::Rxf0Si_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "Rx FIFO 1 Acknowledge 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn rxf1ai(&self) -> crate::common::Reg<rx::Rxf1Ai_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
        }
        #[doc = "Rx FIFO 1 Configuration 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn rxf1ci(&self) -> crate::common::Reg<rx::Rxf1Ci_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "Rx FIFO 1 Status 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn rxf1si(&self) -> crate::common::Reg<rx::Rxf1Si_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
        }
    }
    pub mod rx {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxbCi_SPEC;
        impl crate::sealed::RegSpec for RxbCi_SPEC {
            type DataType = u32;
        }
        #[doc = "Rx Buffer Configuration 0\n resetvalue={Application Reset:0x0}"]
        pub type RxbCi = crate::RegValueT<RxbCi_SPEC>;

        impl RxbCi {
            #[doc = "Rx Buffer Start Address   RBSA. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Configures the start address of the Rx Buffers section in the Message        RAM  32 bit word address . Also        used to reference debug messages A  B  C."]
            #[inline(always)]
            pub fn rbsa(
                self,
            ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, RxbCi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<2,0x3fff,1,0,u16, RxbCi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for RxbCi {
            #[inline(always)]
            fn default() -> RxbCi {
                <crate::RegValueT<RxbCi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct RxesCi_SPEC;
        impl crate::sealed::RegSpec for RxesCi_SPEC {
            type DataType = u32;
        }
        #[doc = "Rx Buffer FIFO Element Size Configuration 0\n resetvalue={Application Reset:0x0}"]
        pub type RxesCi = crate::RegValueT<RxesCi_SPEC>;

        impl RxesCi {
            #[doc = "Rx FIFO 0 Data Field Size   F0DS. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set. In case the data field size of an accepted CAN frame exceeds the data field size configured for the matching Rx Buffer or Rx FIFO  only the number of bytes as configured by RXESC are stored to the Rx Buffer resp. Rx FIFO element. The rest of the frame s data field is ignored."]
            #[inline(always)]
            pub fn f0ds(
                self,
            ) -> crate::common::RegisterField<
                0,
                0x7,
                1,
                0,
                rxesci::F0Ds,
                RxesCi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    0,
                    0x7,
                    1,
                    0,
                    rxesci::F0Ds,
                    RxesCi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Rx FIFO 1 Data Field Size   F1DS. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set."]
            #[inline(always)]
            pub fn f1ds(
                self,
            ) -> crate::common::RegisterField<
                4,
                0x7,
                1,
                0,
                rxesci::F1Ds,
                RxesCi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    4,
                    0x7,
                    1,
                    0,
                    rxesci::F1Ds,
                    RxesCi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Rx Buffer Data Field Size   RBDS. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set."]
            #[inline(always)]
            pub fn rbds(
                self,
            ) -> crate::common::RegisterField<
                8,
                0x7,
                1,
                0,
                rxesci::Rbds,
                RxesCi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    8,
                    0x7,
                    1,
                    0,
                    rxesci::Rbds,
                    RxesCi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for RxesCi {
            #[inline(always)]
            fn default() -> RxesCi {
                <crate::RegValueT<RxesCi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod rxesci {
            pub struct F0Ds_SPEC;
            pub type F0Ds = crate::EnumBitfieldStruct<u8, F0Ds_SPEC>;
            impl F0Ds {
                #[doc = "000 8 byte data field"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "001 12 byte data field"]
                pub const CONST_11: Self = Self::new(1);
                #[doc = "010 16 byte data field"]
                pub const CONST_22: Self = Self::new(2);
                #[doc = "011 20 byte data field"]
                pub const CONST_33: Self = Self::new(3);
                #[doc = "100 24 byte data field"]
                pub const CONST_44: Self = Self::new(4);
                #[doc = "101 32 byte data field"]
                pub const CONST_55: Self = Self::new(5);
                #[doc = "110 48 byte data field"]
                pub const CONST_66: Self = Self::new(6);
                #[doc = "111 64 byte data field"]
                pub const CONST_77: Self = Self::new(7);
            }
            pub struct F1Ds_SPEC;
            pub type F1Ds = crate::EnumBitfieldStruct<u8, F1Ds_SPEC>;
            impl F1Ds {
                #[doc = "000 8 byte data field"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "001 12 byte data field"]
                pub const CONST_11: Self = Self::new(1);
                #[doc = "010 16 byte data field"]
                pub const CONST_22: Self = Self::new(2);
                #[doc = "011 20 byte data field"]
                pub const CONST_33: Self = Self::new(3);
                #[doc = "100 24 byte data field"]
                pub const CONST_44: Self = Self::new(4);
                #[doc = "101 32 byte data field"]
                pub const CONST_55: Self = Self::new(5);
                #[doc = "110 48 byte data field"]
                pub const CONST_66: Self = Self::new(6);
                #[doc = "111 64 byte data field"]
                pub const CONST_77: Self = Self::new(7);
            }
            pub struct Rbds_SPEC;
            pub type Rbds = crate::EnumBitfieldStruct<u8, Rbds_SPEC>;
            impl Rbds {
                #[doc = "000 8 byte data field"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "001 12 byte data field"]
                pub const CONST_11: Self = Self::new(1);
                #[doc = "010 16 byte data field"]
                pub const CONST_22: Self = Self::new(2);
                #[doc = "011 20 byte data field"]
                pub const CONST_33: Self = Self::new(3);
                #[doc = "100 24 byte data field"]
                pub const CONST_44: Self = Self::new(4);
                #[doc = "101 32 byte data field"]
                pub const CONST_55: Self = Self::new(5);
                #[doc = "110 48 byte data field"]
                pub const CONST_66: Self = Self::new(6);
                #[doc = "111 64 byte data field"]
                pub const CONST_77: Self = Self::new(7);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxf0Ai_SPEC;
        impl crate::sealed::RegSpec for Rxf0Ai_SPEC {
            type DataType = u32;
        }
        #[doc = "Rx FIFO 0 Acknowledge 0\n resetvalue={Application Reset:0x0}"]
        pub type Rxf0Ai = crate::RegValueT<Rxf0Ai_SPEC>;

        impl Rxf0Ai {
            #[doc = "Rx FIFO 0 Acknowledge Index   F0AI. After the Host has read a message or a sequence of messages from Rx FIFO 0 it has to write the buffer index of the last element read from Rx FIFO 0 to F0AI. This will set the Rx FIFO 0 Get Index RXF0S.F0GI to F0AI   1 and update the FIFO 0 Fill Level RXF0S.F0FL."]
            #[inline(always)]
            pub fn f0ai(
                self,
            ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Rxf0Ai_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0x3f,1,0,u8, Rxf0Ai_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for Rxf0Ai {
            #[inline(always)]
            fn default() -> Rxf0Ai {
                <crate::RegValueT<Rxf0Ai_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxf0Ci_SPEC;
        impl crate::sealed::RegSpec for Rxf0Ci_SPEC {
            type DataType = u32;
        }
        #[doc = "Rx FIFO 0 Configuration 0\n resetvalue={Application Reset:0x0}"]
        pub type Rxf0Ci = crate::RegValueT<Rxf0Ci_SPEC>;

        impl Rxf0Ci {
            #[doc = "Rx FIFO 0 Start Address   F0SA. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Start address of Rx FIFO 0 in Message RAM  32 bit word address  see CROSSREFERENCE  ."]
            #[inline(always)]
            pub fn f0sa(
                self,
            ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, Rxf0Ci_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<2,0x3fff,1,0,u16, Rxf0Ci_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Rx FIFO 0 Size   F0S. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn f0s(
                self,
            ) -> crate::common::RegisterField<
                16,
                0x7f,
                1,
                0,
                rxf0ci::F0S,
                Rxf0Ci_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    16,
                    0x7f,
                    1,
                    0,
                    rxf0ci::F0S,
                    Rxf0Ci_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Rx FIFO 0 Watermark   F0WM. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn f0wm(
                self,
            ) -> crate::common::RegisterField<
                24,
                0x7f,
                1,
                0,
                rxf0ci::F0Wm,
                Rxf0Ci_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    24,
                    0x7f,
                    1,
                    0,
                    rxf0ci::F0Wm,
                    Rxf0Ci_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "FIFO 0 Operation Mode   F0OM. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. FIFO 0 can be operated in blocking or in overwrite mode  see CROSSREFERENCE  ."]
            #[inline(always)]
            pub fn f0om(
                self,
            ) -> crate::common::RegisterField<
                31,
                0x1,
                1,
                0,
                rxf0ci::F0Om,
                Rxf0Ci_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    31,
                    0x1,
                    1,
                    0,
                    rxf0ci::F0Om,
                    Rxf0Ci_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for Rxf0Ci {
            #[inline(always)]
            fn default() -> Rxf0Ci {
                <crate::RegValueT<Rxf0Ci_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod rxf0ci {
            pub struct F0S_SPEC;
            pub type F0S = crate::EnumBitfieldStruct<u8, F0S_SPEC>;
            impl F0S {
                #[doc = "No Rx FIFO 0"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "64 Rx FIFO 0 elements"]
                pub const REST_127: Self = Self::new(127);
            }
            pub struct F0Wm_SPEC;
            pub type F0Wm = crate::EnumBitfieldStruct<u8, F0Wm_SPEC>;
            impl F0Wm {
                #[doc = "Watermark interrupt disabled"]
                pub const REST_127: Self = Self::new(127);
            }
            pub struct F0Om_SPEC;
            pub type F0Om = crate::EnumBitfieldStruct<u8, F0Om_SPEC>;
            impl F0Om {
                #[doc = "0 FIFO 0 blocking        mode"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 FIFO 0        overwrite mode"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxf0Si_SPEC;
        impl crate::sealed::RegSpec for Rxf0Si_SPEC {
            type DataType = u32;
        }
        #[doc = "Rx FIFO 0 Status 0\n resetvalue={Application Reset:0x0}"]
        pub type Rxf0Si = crate::RegValueT<Rxf0Si_SPEC>;

        impl Rxf0Si {
            #[doc = "Rx FIFO 0 Fill Level   F0FL. Number of elements stored in Rx FIFO 0  range 0 to 64."]
            #[inline(always)]
            pub fn f0fl(
                self,
            ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Rxf0Si_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0x7f,1,0,u8, Rxf0Si_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Rx FIFO 0 Get Index   F0GI. Rx FIFO 0 read index pointer  range 0 to 63."]
            #[inline(always)]
            pub fn f0gi(
                self,
            ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, Rxf0Si_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<8,0x3f,1,0,u8, Rxf0Si_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Rx FIFO 0 Put Index   F0PI. Rx FIFO 0 write index pointer  range 0 to 63."]
            #[inline(always)]
            pub fn f0pi(
                self,
            ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Rxf0Si_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<16,0x3f,1,0,u8, Rxf0Si_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Rx FIFO 0 Full   F0F"]
            #[inline(always)]
            pub fn f0f(
                self,
            ) -> crate::common::RegisterField<
                24,
                0x1,
                1,
                0,
                rxf0si::F0F,
                Rxf0Si_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    24,
                    0x1,
                    1,
                    0,
                    rxf0si::F0F,
                    Rxf0Si_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Rx FIFO 0 Message Lost   RF0L. This bit is a copy of interrupt flag IR.RF0L. When IR.RF0L is reset  this bit is also reset. Overwriting the oldest message when RXF0C.F0OM    1  will not set this flag."]
            #[inline(always)]
            pub fn rf0l(
                self,
            ) -> crate::common::RegisterField<
                25,
                0x1,
                1,
                0,
                rxf0si::Rf0L,
                Rxf0Si_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    25,
                    0x1,
                    1,
                    0,
                    rxf0si::Rf0L,
                    Rxf0Si_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for Rxf0Si {
            #[inline(always)]
            fn default() -> Rxf0Si {
                <crate::RegValueT<Rxf0Si_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod rxf0si {
            pub struct F0F_SPEC;
            pub type F0F = crate::EnumBitfieldStruct<u8, F0F_SPEC>;
            impl F0F {
                #[doc = "0 Rx FIFO 0 not full"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Rx FIFO 0 full"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Rf0L_SPEC;
            pub type Rf0L = crate::EnumBitfieldStruct<u8, Rf0L_SPEC>;
            impl Rf0L {
                #[doc = "0 No Rx FIFO 0 message lost"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Rx FIFO 0 message lost  also set after write attempt to Rx FIFO 0 of size zero"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxf1Ai_SPEC;
        impl crate::sealed::RegSpec for Rxf1Ai_SPEC {
            type DataType = u32;
        }
        #[doc = "Rx FIFO 1 Acknowledge 0\n resetvalue={Application Reset:0x0}"]
        pub type Rxf1Ai = crate::RegValueT<Rxf1Ai_SPEC>;

        impl Rxf1Ai {
            #[doc = "Rx FIFO 1 Acknowledge Index   F1AI. After the Host has read a message or a sequence of messages from Rx FIFO 1 it has to write the buffer index of the last element read from Rx FIFO 1 to F1AI. This will set the Rx FIFO 1 Get Index RXF1S.F1GI to F1AI   1 and update the FIFO 1 Fill Level RXF1S.F1FL"]
            #[inline(always)]
            pub fn f1ai(
                self,
            ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Rxf1Ai_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0x3f,1,0,u8, Rxf1Ai_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for Rxf1Ai {
            #[inline(always)]
            fn default() -> Rxf1Ai {
                <crate::RegValueT<Rxf1Ai_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxf1Ci_SPEC;
        impl crate::sealed::RegSpec for Rxf1Ci_SPEC {
            type DataType = u32;
        }
        #[doc = "Rx FIFO 1 Configuration 0\n resetvalue={Application Reset:0x0}"]
        pub type Rxf1Ci = crate::RegValueT<Rxf1Ci_SPEC>;

        impl Rxf1Ci {
            #[doc = "Rx FIFO 1 Start Address   F1SA. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Start address of Rx FIFO 1 in Message RAM  32 bit word address ."]
            #[inline(always)]
            pub fn f1sa(
                self,
            ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, Rxf1Ci_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<2,0x3fff,1,0,u16, Rxf1Ci_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Rx FIFO 1 Size   F1S. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn f1s(
                self,
            ) -> crate::common::RegisterField<
                16,
                0x7f,
                1,
                0,
                rxf1ci::F1S,
                Rxf1Ci_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    16,
                    0x7f,
                    1,
                    0,
                    rxf1ci::F1S,
                    Rxf1Ci_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Rx FIFO 1 Watermark   F1WM. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn f1wm(
                self,
            ) -> crate::common::RegisterField<
                24,
                0x7f,
                1,
                0,
                rxf1ci::F1Wm,
                Rxf1Ci_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    24,
                    0x7f,
                    1,
                    0,
                    rxf1ci::F1Wm,
                    Rxf1Ci_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "FIFO 1 Operation Mode   F1OM. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. FIFO 1 can be operated in blocking or in overwrite mode."]
            #[inline(always)]
            pub fn f1om(
                self,
            ) -> crate::common::RegisterField<
                31,
                0x1,
                1,
                0,
                rxf1ci::F1Om,
                Rxf1Ci_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    31,
                    0x1,
                    1,
                    0,
                    rxf1ci::F1Om,
                    Rxf1Ci_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for Rxf1Ci {
            #[inline(always)]
            fn default() -> Rxf1Ci {
                <crate::RegValueT<Rxf1Ci_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod rxf1ci {
            pub struct F1S_SPEC;
            pub type F1S = crate::EnumBitfieldStruct<u8, F1S_SPEC>;
            impl F1S {
                #[doc = "No Rx FIFO 1"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "64 Rx FIFO 1 elements"]
                pub const REST_127: Self = Self::new(127);
            }
            pub struct F1Wm_SPEC;
            pub type F1Wm = crate::EnumBitfieldStruct<u8, F1Wm_SPEC>;
            impl F1Wm {
                #[doc = "Watermark interrupt disabled"]
                pub const REST_127: Self = Self::new(127);
            }
            pub struct F1Om_SPEC;
            pub type F1Om = crate::EnumBitfieldStruct<u8, F1Om_SPEC>;
            impl F1Om {
                #[doc = "0 FIFO 1 blocking mode"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 FIFO 1 overwrite mode"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct Rxf1Si_SPEC;
        impl crate::sealed::RegSpec for Rxf1Si_SPEC {
            type DataType = u32;
        }
        #[doc = "Rx FIFO 1 Status 0\n resetvalue={Application Reset:0x0}"]
        pub type Rxf1Si = crate::RegValueT<Rxf1Si_SPEC>;

        impl Rxf1Si {
            #[doc = "Rx FIFO 1 Fill Level   F1FL. Number of elements stored in Rx FIFO 1  range 0 to 64."]
            #[inline(always)]
            pub fn f1fl(
                self,
            ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Rxf1Si_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0x7f,1,0,u8, Rxf1Si_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Rx FIFO 1 Get Index   F1GI. Rx FIFO 1 read index pointer  range 0 to 63."]
            #[inline(always)]
            pub fn f1gi(
                self,
            ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, Rxf1Si_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<8,0x3f,1,0,u8, Rxf1Si_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Rx FIFO 1 Put Index   F1PI. Rx FIFO 1 write index pointer  range 0 to 63."]
            #[inline(always)]
            pub fn f1pi(
                self,
            ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Rxf1Si_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<16,0x3f,1,0,u8, Rxf1Si_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Rx FIFO 1 Full   F1F"]
            #[inline(always)]
            pub fn f1f(
                self,
            ) -> crate::common::RegisterField<
                24,
                0x1,
                1,
                0,
                rxf1si::F1F,
                Rxf1Si_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    24,
                    0x1,
                    1,
                    0,
                    rxf1si::F1F,
                    Rxf1Si_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Rx FIFO 1 Message Lost   RF1L. This bit is a copy of interrupt flag IR.RF1L. When IR.RF1L is reset  this bit is also reset. Overwriting the oldest message when RXF1C.F1OM    1  will not set this flag."]
            #[inline(always)]
            pub fn rf1l(
                self,
            ) -> crate::common::RegisterField<
                25,
                0x1,
                1,
                0,
                rxf1si::Rf1L,
                Rxf1Si_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    25,
                    0x1,
                    1,
                    0,
                    rxf1si::Rf1L,
                    Rxf1Si_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for Rxf1Si {
            #[inline(always)]
            fn default() -> Rxf1Si {
                <crate::RegValueT<Rxf1Si_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod rxf1si {
            pub struct F1F_SPEC;
            pub type F1F = crate::EnumBitfieldStruct<u8, F1F_SPEC>;
            impl F1F {
                #[doc = "0 Rx FIFO 1 not full"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Rx FIFO 1 full"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Rf1L_SPEC;
            pub type Rf1L = crate::EnumBitfieldStruct<u8, Rf1L_SPEC>;
            impl Rf1L {
                #[doc = "0 No Rx FIFO 1 message lost"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Rx FIFO 1 message lost  also set after write attempt to Rx FIFO 1 of size zero"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
    }
    #[doc = "TT"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tt(pub(super) *mut u8);
    unsafe impl core::marker::Send for Tt {}
    unsafe impl core::marker::Sync for Tt {}
    impl Tt {
        #[doc = "TT Capture Time 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ttcpti(&self) -> crate::common::Reg<tt::TtcpTi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
        }
        #[doc = "TT Cycle Sync Mark 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ttcsmi(&self) -> crate::common::Reg<tt::TtcsMi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
        }
        #[doc = "TT Cycle Time   Count 0\n resetvalue={Application Reset:0x3F0000}"]
        #[inline(always)]
        pub const fn ttctci(&self) -> crate::common::Reg<tt::TtctCi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
        }
        #[doc = "TT Global Time Preset 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ttgtpi(&self) -> crate::common::Reg<tt::TtgtPi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
        }
        #[doc = "TT Interrupt Enable 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ttiei(&self) -> crate::common::Reg<tt::TtiEi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
        }
        #[doc = "TT Interrupt Line Select 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ttilsi(&self) -> crate::common::Reg<tt::TtilSi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
        }
        #[doc = "TT Interrupt Register 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ttiri(&self) -> crate::common::Reg<tt::TtiRi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
        }
        #[doc = "TT Local   Global Time 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ttlgti(&self) -> crate::common::Reg<tt::TtlgTi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
        }
        #[doc = "TT Matrix Limits 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ttmlmi(&self) -> crate::common::Reg<tt::TtmlMi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "TT Operation Configuration 0\n resetvalue={Application Reset:0x10000}"]
        #[inline(always)]
        pub const fn ttocfi(&self) -> crate::common::Reg<tt::TtocFi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "TT Operation Control 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ttocni(&self) -> crate::common::Reg<tt::TtocNi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "TT Operation Status 0\n resetvalue={Application Reset:0x20000080}"]
        #[inline(always)]
        pub const fn ttosti(&self) -> crate::common::Reg<tt::TtosTi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
        }
        #[doc = "TT Reference Message Configuration 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn ttrmci(&self) -> crate::common::Reg<tt::TtrmCi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "TT Trigger Memory Configuration 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn tttmci(&self) -> crate::common::Reg<tt::TttmCi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "TT Time Mark 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn tttmki(&self) -> crate::common::Reg<tt::TttmKi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
        }
        #[doc = "TUR Configuration 0\n resetvalue={Application Reset:0x10000000}"]
        #[inline(always)]
        pub const fn turcfi(&self) -> crate::common::Reg<tt::TurcFi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "TUR Numerator Actual 0\n resetvalue={Application Reset:0x10000}"]
        #[inline(always)]
        pub const fn turnai(&self) -> crate::common::Reg<tt::TurnAi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
        }
    }
    pub mod tt {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtcpTi_SPEC;
        impl crate::sealed::RegSpec for TtcpTi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Capture Time 0\n resetvalue={Application Reset:0x0}"]
        pub type TtcpTi = crate::RegValueT<TtcpTi_SPEC>;

        impl TtcpTi {
            #[doc = "Cycle Count Value   CCV. Cycle count value captured together with SWV. Captured cycle count value"]
            #[inline(always)]
            pub fn ccv(
                self,
            ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, TtcpTi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0x3f,1,0,u8, TtcpTi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Stop Watch Value   SWV. On a rising falling edge  as configured via TTOCN.SWP  at the Stop Watch        Trigger  when TTOCN.SWS is   8800    8220 00  8221  and TTIR.SWE is   8216 0  8217   the actual time        value as selected by TTOCN.SWS  cycle  local  global  is copied to SWV        and TTIR.SWE will be set to   8216 1  8217 . Capturing of the next stop watch value        is enabled by resetting TTIR.SWE. Captured Stop Watch value"]
            #[inline(always)]
            pub fn swv(
                self,
            ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, TtcpTi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<16,0xffff,1,0,u16, TtcpTi_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for TtcpTi {
            #[inline(always)]
            fn default() -> TtcpTi {
                <crate::RegValueT<TtcpTi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtcsMi_SPEC;
        impl crate::sealed::RegSpec for TtcsMi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Cycle Sync Mark 0\n resetvalue={Application Reset:0x0}"]
        pub type TtcsMi = crate::RegValueT<TtcsMi_SPEC>;

        impl TtcsMi {
            #[doc = "Cycle Sync Mark   CSM. The Cycle Sync Mark is measured in cycle time. It is updated when the        reference message becomes valid and retains its value until the next        reference message becomes valid. Captured cycle time"]
            #[inline(always)]
            pub fn csm(
                self,
            ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, TtcsMi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0xffff,1,0,u16, TtcsMi_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for TtcsMi {
            #[inline(always)]
            fn default() -> TtcsMi {
                <crate::RegValueT<TtcsMi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtctCi_SPEC;
        impl crate::sealed::RegSpec for TtctCi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Cycle Time   Count 0\n resetvalue={Application Reset:0x3F0000}"]
        pub type TtctCi = crate::RegValueT<TtctCi_SPEC>;

        impl TtctCi {
            #[doc = "Cycle Time   CT. Non fractional part of the difference of the node  8217 s local time and        Ref Mark. Cycle time value of TTCAN Basic Cycle"]
            #[inline(always)]
            pub fn ct(
                self,
            ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, TtctCi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0xffff,1,0,u16, TtctCi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Cycle Count   CC. Number of actual Basic Cycle in the System Matrix"]
            #[inline(always)]
            pub fn cc(
                self,
            ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, TtctCi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<16,0x3f,1,0,u8, TtctCi_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for TtctCi {
            #[inline(always)]
            fn default() -> TtctCi {
                <crate::RegValueT<TtctCi_SPEC> as RegisterValue<_>>::new(4128768)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtgtPi_SPEC;
        impl crate::sealed::RegSpec for TtgtPi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Global Time Preset 0\n resetvalue={Application Reset:0x0}"]
        pub type TtgtPi = crate::RegValueT<TtgtPi_SPEC>;

        impl TtgtPi {
            #[doc = "Time Preset   TP. CTP is write protected while TTOCN.ESCN or TTOST.SPL are set."]
            #[inline(always)]
            pub fn tp(
                self,
            ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, TtgtPi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xffff,1,0,u16, TtgtPi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Cycle Time Target Phase   CTP. CTP is write protected while TTOCN.ESCN or TTOST.SPL are set."]
            #[inline(always)]
            pub fn ctp(
                self,
            ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, TtgtPi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0xffff,1,0,u16, TtgtPi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for TtgtPi {
            #[inline(always)]
            fn default() -> TtgtPi {
                <crate::RegValueT<TtgtPi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtiEi_SPEC;
        impl crate::sealed::RegSpec for TtiEi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Interrupt Enable 0\n resetvalue={Application Reset:0x0}"]
        pub type TtiEi = crate::RegValueT<TtiEi_SPEC>;

        impl TtiEi {
            #[doc = "Start of Basic Cycle Interrupt Enable   SBCE"]
            #[inline(always)]
            pub fn sbce(
                self,
            ) -> crate::common::RegisterField<
                0,
                0x1,
                1,
                0,
                ttiei::Sbce,
                TtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    0,
                    0x1,
                    1,
                    0,
                    ttiei::Sbce,
                    TtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Start of Matrix Cycle Interrupt Enable   SMCE"]
            #[inline(always)]
            pub fn smce(
                self,
            ) -> crate::common::RegisterField<
                1,
                0x1,
                1,
                0,
                ttiei::Smce,
                TtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    1,
                    0x1,
                    1,
                    0,
                    ttiei::Smce,
                    TtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Change of Synchronization Mode Interrupt Enable   CSME"]
            #[inline(always)]
            pub fn csme(
                self,
            ) -> crate::common::RegisterField<
                2,
                0x1,
                1,
                0,
                ttiei::Csme,
                TtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    2,
                    0x1,
                    1,
                    0,
                    ttiei::Csme,
                    TtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Start of Gap Interrupt Enable   SOGE"]
            #[inline(always)]
            pub fn soge(
                self,
            ) -> crate::common::RegisterField<
                3,
                0x1,
                1,
                0,
                ttiei::Soge,
                TtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    3,
                    0x1,
                    1,
                    0,
                    ttiei::Soge,
                    TtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Register Time Mark Interrupt Enable   RTMIE"]
            #[inline(always)]
            pub fn rtmie(
                self,
            ) -> crate::common::RegisterField<
                4,
                0x1,
                1,
                0,
                ttiei::Rtmie,
                TtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    4,
                    0x1,
                    1,
                    0,
                    ttiei::Rtmie,
                    TtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Trigger Time Mark Event Internal Interrupt Enable   TTMIE"]
            #[inline(always)]
            pub fn ttmie(
                self,
            ) -> crate::common::RegisterField<
                5,
                0x1,
                1,
                0,
                ttiei::Ttmie,
                TtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    5,
                    0x1,
                    1,
                    0,
                    ttiei::Ttmie,
                    TtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Stop Watch Polarity Interrupt Enable   SWEE"]
            #[inline(always)]
            pub fn swee(
                self,
            ) -> crate::common::RegisterField<
                6,
                0x1,
                1,
                0,
                ttiei::Swee,
                TtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    6,
                    0x1,
                    1,
                    0,
                    ttiei::Swee,
                    TtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Global Time Wrap Interrupt Enable   GTWE"]
            #[inline(always)]
            pub fn gtwe(
                self,
            ) -> crate::common::RegisterField<
                7,
                0x1,
                1,
                0,
                ttiei::Gtwe,
                TtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    7,
                    0x1,
                    1,
                    0,
                    ttiei::Gtwe,
                    TtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Global Time Discontinuity Interrupt Enable   GTDE"]
            #[inline(always)]
            pub fn gtde(
                self,
            ) -> crate::common::RegisterField<
                8,
                0x1,
                1,
                0,
                ttiei::Gtde,
                TtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    8,
                    0x1,
                    1,
                    0,
                    ttiei::Gtde,
                    TtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Global Time Error Interrupt Enable   GTEE"]
            #[inline(always)]
            pub fn gtee(
                self,
            ) -> crate::common::RegisterField<
                9,
                0x1,
                1,
                0,
                ttiei::Gtee,
                TtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    9,
                    0x1,
                    1,
                    0,
                    ttiei::Gtee,
                    TtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Tx Count Underflow Interrupt Enable   TXUE"]
            #[inline(always)]
            pub fn txue(
                self,
            ) -> crate::common::RegisterField<
                10,
                0x1,
                1,
                0,
                ttiei::Txue,
                TtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    10,
                    0x1,
                    1,
                    0,
                    ttiei::Txue,
                    TtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Tx Count Overflow Interrupt Enable   TXOE"]
            #[inline(always)]
            pub fn txoe(
                self,
            ) -> crate::common::RegisterField<
                11,
                0x1,
                1,
                0,
                ttiei::Txoe,
                TtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    11,
                    0x1,
                    1,
                    0,
                    ttiei::Txoe,
                    TtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Scheduling Error 1 Interrupt Enable   SE1E"]
            #[inline(always)]
            pub fn se1e(
                self,
            ) -> crate::common::RegisterField<
                12,
                0x1,
                1,
                0,
                ttiei::Se1E,
                TtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    12,
                    0x1,
                    1,
                    0,
                    ttiei::Se1E,
                    TtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Scheduling Error 2 Interrupt Enable   SE2E"]
            #[inline(always)]
            pub fn se2e(
                self,
            ) -> crate::common::RegisterField<
                13,
                0x1,
                1,
                0,
                ttiei::Se2E,
                TtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    13,
                    0x1,
                    1,
                    0,
                    ttiei::Se2E,
                    TtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Change Error Level Interrupt Enable   ELCE"]
            #[inline(always)]
            pub fn elce(
                self,
            ) -> crate::common::RegisterField<
                14,
                0x1,
                1,
                0,
                ttiei::Elce,
                TtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    14,
                    0x1,
                    1,
                    0,
                    ttiei::Elce,
                    TtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Initialization Watch Trigger Interrupt Enable   IWTE"]
            #[inline(always)]
            pub fn iwte(
                self,
            ) -> crate::common::RegisterField<
                15,
                0x1,
                1,
                0,
                ttiei::Iwte,
                TtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    15,
                    0x1,
                    1,
                    0,
                    ttiei::Iwte,
                    TtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Watch Trigger Interrupt Enable   WTE"]
            #[inline(always)]
            pub fn wte(
                self,
            ) -> crate::common::RegisterField<
                16,
                0x1,
                1,
                0,
                ttiei::Wte,
                TtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    16,
                    0x1,
                    1,
                    0,
                    ttiei::Wte,
                    TtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Application Watchdog Interrupt Enable   AWE"]
            #[inline(always)]
            pub fn awe(
                self,
            ) -> crate::common::RegisterField<
                17,
                0x1,
                1,
                0,
                ttiei::Awe,
                TtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    17,
                    0x1,
                    1,
                    0,
                    ttiei::Awe,
                    TtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Configuration Error Interrupt Enable   CERE"]
            #[inline(always)]
            pub fn cere(
                self,
            ) -> crate::common::RegisterField<
                18,
                0x1,
                1,
                0,
                ttiei::Cere,
                TtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    18,
                    0x1,
                    1,
                    0,
                    ttiei::Cere,
                    TtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TtiEi {
            #[inline(always)]
            fn default() -> TtiEi {
                <crate::RegValueT<TtiEi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod ttiei {
            pub struct Sbce_SPEC;
            pub type Sbce = crate::EnumBitfieldStruct<u8, Sbce_SPEC>;
            impl Sbce {
                #[doc = "0 TT interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Smce_SPEC;
            pub type Smce = crate::EnumBitfieldStruct<u8, Smce_SPEC>;
            impl Smce {
                #[doc = "0 TT interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Csme_SPEC;
            pub type Csme = crate::EnumBitfieldStruct<u8, Csme_SPEC>;
            impl Csme {
                #[doc = "0 TT interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Soge_SPEC;
            pub type Soge = crate::EnumBitfieldStruct<u8, Soge_SPEC>;
            impl Soge {
                #[doc = "0 TT interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Rtmie_SPEC;
            pub type Rtmie = crate::EnumBitfieldStruct<u8, Rtmie_SPEC>;
            impl Rtmie {
                #[doc = "0 TT interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ttmie_SPEC;
            pub type Ttmie = crate::EnumBitfieldStruct<u8, Ttmie_SPEC>;
            impl Ttmie {
                #[doc = "0 TT interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Swee_SPEC;
            pub type Swee = crate::EnumBitfieldStruct<u8, Swee_SPEC>;
            impl Swee {
                #[doc = "0 TT interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Gtwe_SPEC;
            pub type Gtwe = crate::EnumBitfieldStruct<u8, Gtwe_SPEC>;
            impl Gtwe {
                #[doc = "0 TT interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Gtde_SPEC;
            pub type Gtde = crate::EnumBitfieldStruct<u8, Gtde_SPEC>;
            impl Gtde {
                #[doc = "0 TT interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Gtee_SPEC;
            pub type Gtee = crate::EnumBitfieldStruct<u8, Gtee_SPEC>;
            impl Gtee {
                #[doc = "0 TT interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Txue_SPEC;
            pub type Txue = crate::EnumBitfieldStruct<u8, Txue_SPEC>;
            impl Txue {
                #[doc = "0 TT interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Txoe_SPEC;
            pub type Txoe = crate::EnumBitfieldStruct<u8, Txoe_SPEC>;
            impl Txoe {
                #[doc = "0 TT interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Se1E_SPEC;
            pub type Se1E = crate::EnumBitfieldStruct<u8, Se1E_SPEC>;
            impl Se1E {
                #[doc = "0 TT interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Se2E_SPEC;
            pub type Se2E = crate::EnumBitfieldStruct<u8, Se2E_SPEC>;
            impl Se2E {
                #[doc = "0 TT interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Elce_SPEC;
            pub type Elce = crate::EnumBitfieldStruct<u8, Elce_SPEC>;
            impl Elce {
                #[doc = "0 TT interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Iwte_SPEC;
            pub type Iwte = crate::EnumBitfieldStruct<u8, Iwte_SPEC>;
            impl Iwte {
                #[doc = "0 TT interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Wte_SPEC;
            pub type Wte = crate::EnumBitfieldStruct<u8, Wte_SPEC>;
            impl Wte {
                #[doc = "0 TT interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Awe_SPEC;
            pub type Awe = crate::EnumBitfieldStruct<u8, Awe_SPEC>;
            impl Awe {
                #[doc = "0 TT interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cere_SPEC;
            pub type Cere = crate::EnumBitfieldStruct<u8, Cere_SPEC>;
            impl Cere {
                #[doc = "0 TT interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtilSi_SPEC;
        impl crate::sealed::RegSpec for TtilSi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Interrupt Line Select 0\n resetvalue={Application Reset:0x0}"]
        pub type TtilSi = crate::RegValueT<TtilSi_SPEC>;

        impl TtilSi {
            #[doc = "Start of Basic Cycle Interrupt Line   SBCL"]
            #[inline(always)]
            pub fn sbcl(
                self,
            ) -> crate::common::RegisterField<
                0,
                0x1,
                1,
                0,
                ttilsi::Sbcl,
                TtilSi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    0,
                    0x1,
                    1,
                    0,
                    ttilsi::Sbcl,
                    TtilSi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Start of Matrix Cycle Interrupt Line   SMCL"]
            #[inline(always)]
            pub fn smcl(
                self,
            ) -> crate::common::RegisterField<
                1,
                0x1,
                1,
                0,
                ttilsi::Smcl,
                TtilSi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    1,
                    0x1,
                    1,
                    0,
                    ttilsi::Smcl,
                    TtilSi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Change of Synchronization Mode Interrupt Line   CSML"]
            #[inline(always)]
            pub fn csml(
                self,
            ) -> crate::common::RegisterField<
                2,
                0x1,
                1,
                0,
                ttilsi::Csml,
                TtilSi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    2,
                    0x1,
                    1,
                    0,
                    ttilsi::Csml,
                    TtilSi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Start of Gap Interrupt Line   SOGL"]
            #[inline(always)]
            pub fn sogl(
                self,
            ) -> crate::common::RegisterField<
                3,
                0x1,
                1,
                0,
                ttilsi::Sogl,
                TtilSi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    3,
                    0x1,
                    1,
                    0,
                    ttilsi::Sogl,
                    TtilSi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Register Time Mark Interrupt Line   RTMIL"]
            #[inline(always)]
            pub fn rtmil(
                self,
            ) -> crate::common::RegisterField<
                4,
                0x1,
                1,
                0,
                ttilsi::Rtmil,
                TtilSi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    4,
                    0x1,
                    1,
                    0,
                    ttilsi::Rtmil,
                    TtilSi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Trigger Time Mark Event Interrupt Line   TTMIL"]
            #[inline(always)]
            pub fn ttmil(
                self,
            ) -> crate::common::RegisterField<
                5,
                0x1,
                1,
                0,
                ttilsi::Ttmil,
                TtilSi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    5,
                    0x1,
                    1,
                    0,
                    ttilsi::Ttmil,
                    TtilSi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Stop Watch Event Interrupt Line   SWEL"]
            #[inline(always)]
            pub fn swel(
                self,
            ) -> crate::common::RegisterField<
                6,
                0x1,
                1,
                0,
                ttilsi::Swel,
                TtilSi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    6,
                    0x1,
                    1,
                    0,
                    ttilsi::Swel,
                    TtilSi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Global Time Wrap Interrupt Line   GTWL"]
            #[inline(always)]
            pub fn gtwl(
                self,
            ) -> crate::common::RegisterField<
                7,
                0x1,
                1,
                0,
                ttilsi::Gtwl,
                TtilSi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    7,
                    0x1,
                    1,
                    0,
                    ttilsi::Gtwl,
                    TtilSi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Global Time Discontinuity Interrupt Line   GTDL"]
            #[inline(always)]
            pub fn gtdl(
                self,
            ) -> crate::common::RegisterField<
                8,
                0x1,
                1,
                0,
                ttilsi::Gtdl,
                TtilSi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    8,
                    0x1,
                    1,
                    0,
                    ttilsi::Gtdl,
                    TtilSi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Global Time Error Interrupt Line   GTEL"]
            #[inline(always)]
            pub fn gtel(
                self,
            ) -> crate::common::RegisterField<
                9,
                0x1,
                1,
                0,
                ttilsi::Gtel,
                TtilSi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    9,
                    0x1,
                    1,
                    0,
                    ttilsi::Gtel,
                    TtilSi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Tx Count Underflow Interrupt Line   TXUL"]
            #[inline(always)]
            pub fn txul(
                self,
            ) -> crate::common::RegisterField<
                10,
                0x1,
                1,
                0,
                ttilsi::Txul,
                TtilSi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    10,
                    0x1,
                    1,
                    0,
                    ttilsi::Txul,
                    TtilSi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Tx Count Overflow Interrupt Line   TXOL"]
            #[inline(always)]
            pub fn txol(
                self,
            ) -> crate::common::RegisterField<
                11,
                0x1,
                1,
                0,
                ttilsi::Txol,
                TtilSi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    11,
                    0x1,
                    1,
                    0,
                    ttilsi::Txol,
                    TtilSi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Scheduling Error 1 Interrupt Line   SE1L"]
            #[inline(always)]
            pub fn se1l(
                self,
            ) -> crate::common::RegisterField<
                12,
                0x1,
                1,
                0,
                ttilsi::Se1L,
                TtilSi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    12,
                    0x1,
                    1,
                    0,
                    ttilsi::Se1L,
                    TtilSi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Scheduling Error 2 Interrupt Line   SE2L"]
            #[inline(always)]
            pub fn se2l(
                self,
            ) -> crate::common::RegisterField<
                13,
                0x1,
                1,
                0,
                ttilsi::Se2L,
                TtilSi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    13,
                    0x1,
                    1,
                    0,
                    ttilsi::Se2L,
                    TtilSi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Change Error Level Interrupt Line   ELCL"]
            #[inline(always)]
            pub fn elcl(
                self,
            ) -> crate::common::RegisterField<
                14,
                0x1,
                1,
                0,
                ttilsi::Elcl,
                TtilSi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    14,
                    0x1,
                    1,
                    0,
                    ttilsi::Elcl,
                    TtilSi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Initialization Watch Trigger Interrupt Line   IWTL"]
            #[inline(always)]
            pub fn iwtl(
                self,
            ) -> crate::common::RegisterField<
                15,
                0x1,
                1,
                0,
                ttilsi::Iwtl,
                TtilSi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    15,
                    0x1,
                    1,
                    0,
                    ttilsi::Iwtl,
                    TtilSi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Watch Trigger Interrupt Line   WTL"]
            #[inline(always)]
            pub fn wtl(
                self,
            ) -> crate::common::RegisterField<
                16,
                0x1,
                1,
                0,
                ttilsi::Wtl,
                TtilSi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    16,
                    0x1,
                    1,
                    0,
                    ttilsi::Wtl,
                    TtilSi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Application Watchdog Interrupt Line   AWL"]
            #[inline(always)]
            pub fn awl(
                self,
            ) -> crate::common::RegisterField<
                17,
                0x1,
                1,
                0,
                ttilsi::Awl,
                TtilSi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    17,
                    0x1,
                    1,
                    0,
                    ttilsi::Awl,
                    TtilSi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Configuration Error Interrupt Line   CERL"]
            #[inline(always)]
            pub fn cerl(
                self,
            ) -> crate::common::RegisterField<
                18,
                0x1,
                1,
                0,
                ttilsi::Cerl,
                TtilSi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    18,
                    0x1,
                    1,
                    0,
                    ttilsi::Cerl,
                    TtilSi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TtilSi {
            #[inline(always)]
            fn default() -> TtilSi {
                <crate::RegValueT<TtilSi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod ttilsi {
            pub struct Sbcl_SPEC;
            pub type Sbcl = crate::EnumBitfieldStruct<u8, Sbcl_SPEC>;
            impl Sbcl {
                #[doc = "0 TT interrupt assigned to interrupt line 0"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt assigned to interrupt line 1"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Smcl_SPEC;
            pub type Smcl = crate::EnumBitfieldStruct<u8, Smcl_SPEC>;
            impl Smcl {
                #[doc = "0 TT interrupt assigned to interrupt line 0"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt assigned to interrupt line 1"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Csml_SPEC;
            pub type Csml = crate::EnumBitfieldStruct<u8, Csml_SPEC>;
            impl Csml {
                #[doc = "0 TT interrupt assigned to interrupt line 0"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt assigned to interrupt line 1"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Sogl_SPEC;
            pub type Sogl = crate::EnumBitfieldStruct<u8, Sogl_SPEC>;
            impl Sogl {
                #[doc = "0 TT interrupt assigned to interrupt line 0"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt assigned to interrupt line 1"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Rtmil_SPEC;
            pub type Rtmil = crate::EnumBitfieldStruct<u8, Rtmil_SPEC>;
            impl Rtmil {
                #[doc = "0 TT interrupt assigned to interrupt line 0"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt assigned to interrupt line 1"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ttmil_SPEC;
            pub type Ttmil = crate::EnumBitfieldStruct<u8, Ttmil_SPEC>;
            impl Ttmil {
                #[doc = "0 TT interrupt assigned to interrupt line 0"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt assigned to interrupt line 1"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Swel_SPEC;
            pub type Swel = crate::EnumBitfieldStruct<u8, Swel_SPEC>;
            impl Swel {
                #[doc = "0 TT interrupt assigned to interrupt line 0"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt assigned to interrupt line 1"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Gtwl_SPEC;
            pub type Gtwl = crate::EnumBitfieldStruct<u8, Gtwl_SPEC>;
            impl Gtwl {
                #[doc = "0 TT interrupt assigned to interrupt line 0"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt assigned to interrupt line 1"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Gtdl_SPEC;
            pub type Gtdl = crate::EnumBitfieldStruct<u8, Gtdl_SPEC>;
            impl Gtdl {
                #[doc = "0 TT interrupt assigned to interrupt line 0"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt assigned to interrupt line 1"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Gtel_SPEC;
            pub type Gtel = crate::EnumBitfieldStruct<u8, Gtel_SPEC>;
            impl Gtel {
                #[doc = "0 TT interrupt assigned to interrupt line 0"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt assigned to interrupt line 1"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Txul_SPEC;
            pub type Txul = crate::EnumBitfieldStruct<u8, Txul_SPEC>;
            impl Txul {
                #[doc = "0 TT interrupt assigned to interrupt line 0"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt assigned to interrupt line 1"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Txol_SPEC;
            pub type Txol = crate::EnumBitfieldStruct<u8, Txol_SPEC>;
            impl Txol {
                #[doc = "0 TT interrupt assigned to interrupt line 0"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt assigned to interrupt line 1"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Se1L_SPEC;
            pub type Se1L = crate::EnumBitfieldStruct<u8, Se1L_SPEC>;
            impl Se1L {
                #[doc = "0 TT interrupt assigned to interrupt line 0"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt assigned to interrupt line 1"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Se2L_SPEC;
            pub type Se2L = crate::EnumBitfieldStruct<u8, Se2L_SPEC>;
            impl Se2L {
                #[doc = "0 TT interrupt assigned to interrupt line 0"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt assigned to interrupt line 1"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Elcl_SPEC;
            pub type Elcl = crate::EnumBitfieldStruct<u8, Elcl_SPEC>;
            impl Elcl {
                #[doc = "0 TT interrupt assigned to interrupt line 0"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt assigned to interrupt line 1"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Iwtl_SPEC;
            pub type Iwtl = crate::EnumBitfieldStruct<u8, Iwtl_SPEC>;
            impl Iwtl {
                #[doc = "0 TT interrupt assigned to interrupt line 0"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt assigned to interrupt line 1"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Wtl_SPEC;
            pub type Wtl = crate::EnumBitfieldStruct<u8, Wtl_SPEC>;
            impl Wtl {
                #[doc = "0 TT interrupt assigned to interrupt line 0"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt assigned to interrupt line 1"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Awl_SPEC;
            pub type Awl = crate::EnumBitfieldStruct<u8, Awl_SPEC>;
            impl Awl {
                #[doc = "0 TT interrupt assigned to interrupt line 0"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt assigned to interrupt line 1"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cerl_SPEC;
            pub type Cerl = crate::EnumBitfieldStruct<u8, Cerl_SPEC>;
            impl Cerl {
                #[doc = "0 TT interrupt assigned to interrupt line 0"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 TT interrupt assigned to interrupt line 1"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtiRi_SPEC;
        impl crate::sealed::RegSpec for TtiRi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Interrupt Register 0\n resetvalue={Application Reset:0x0}"]
        pub type TtiRi = crate::RegValueT<TtiRi_SPEC>;

        impl TtiRi {
            #[doc = "Start of Basic Cycle   SBC"]
            #[inline(always)]
            pub fn sbc(
                self,
            ) -> crate::common::RegisterField<0, 0x1, 1, 0, ttiri::Sbc, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<
                    0,
                    0x1,
                    1,
                    0,
                    ttiri::Sbc,
                    TtiRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Start of Matrix Cycle   SMC"]
            #[inline(always)]
            pub fn smc(
                self,
            ) -> crate::common::RegisterField<1, 0x1, 1, 0, ttiri::Smc, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<
                    1,
                    0x1,
                    1,
                    0,
                    ttiri::Smc,
                    TtiRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Change of Synchronization Mode   CSM"]
            #[inline(always)]
            pub fn csm(
                self,
            ) -> crate::common::RegisterField<2, 0x1, 1, 0, ttiri::Csm, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<
                    2,
                    0x1,
                    1,
                    0,
                    ttiri::Csm,
                    TtiRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Start of Gap   SOG"]
            #[inline(always)]
            pub fn sog(
                self,
            ) -> crate::common::RegisterField<3, 0x1, 1, 0, ttiri::Sog, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<
                    3,
                    0x1,
                    1,
                    0,
                    ttiri::Sog,
                    TtiRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Register Time Mark Interrupt   RTMI. Set when time referenced by TTOCN.TMC         cycle  local  or global  equals TTTMK.TM         independent of the synchronization state."]
            #[inline(always)]
            pub fn rtmi(
                self,
            ) -> crate::common::RegisterField<
                4,
                0x1,
                1,
                0,
                ttiri::Rtmi,
                TtiRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    4,
                    0x1,
                    1,
                    0,
                    ttiri::Rtmi,
                    TtiRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Trigger Time Mark Event Internal   TTMI. Internal time mark events are configured by trigger memory element TMIN.        Set when the trigger memory element becomes active  and the M CAN is in synchronization state In Gap or In Schedule."]
            #[inline(always)]
            pub fn ttmi(
                self,
            ) -> crate::common::RegisterField<
                5,
                0x1,
                1,
                0,
                ttiri::Ttmi,
                TtiRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    5,
                    0x1,
                    1,
                    0,
                    ttiri::Ttmi,
                    TtiRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Stop Watch Polarity   SWE"]
            #[inline(always)]
            pub fn swe(
                self,
            ) -> crate::common::RegisterField<6, 0x1, 1, 0, ttiri::Swe, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<
                    6,
                    0x1,
                    1,
                    0,
                    ttiri::Swe,
                    TtiRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Global Time Wrap   GTW"]
            #[inline(always)]
            pub fn gtw(
                self,
            ) -> crate::common::RegisterField<7, 0x1, 1, 0, ttiri::Gtw, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<
                    7,
                    0x1,
                    1,
                    0,
                    ttiri::Gtw,
                    TtiRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Global Time Discontinuity   GTD"]
            #[inline(always)]
            pub fn gtd(
                self,
            ) -> crate::common::RegisterField<8, 0x1, 1, 0, ttiri::Gtd, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<
                    8,
                    0x1,
                    1,
                    0,
                    ttiri::Gtd,
                    TtiRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Global Time Error   GTE. Synchronization deviation SD exceeds limit specified by TTOCF.LDSDL         TTCAN Level  160 0 2 only."]
            #[inline(always)]
            pub fn gte(
                self,
            ) -> crate::common::RegisterField<9, 0x1, 1, 0, ttiri::Gte, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<
                    9,
                    0x1,
                    1,
                    0,
                    ttiri::Gte,
                    TtiRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Tx Count Underflow   TXU"]
            #[inline(always)]
            pub fn txu(
                self,
            ) -> crate::common::RegisterField<
                10,
                0x1,
                1,
                0,
                ttiri::Txu,
                TtiRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    10,
                    0x1,
                    1,
                    0,
                    ttiri::Txu,
                    TtiRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Tx Count Overflow   TXO"]
            #[inline(always)]
            pub fn txo(
                self,
            ) -> crate::common::RegisterField<
                11,
                0x1,
                1,
                0,
                ttiri::Txo,
                TtiRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    11,
                    0x1,
                    1,
                    0,
                    ttiri::Txo,
                    TtiRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Scheduling Error 1   SE1"]
            #[inline(always)]
            pub fn se1(
                self,
            ) -> crate::common::RegisterField<
                12,
                0x1,
                1,
                0,
                ttiri::Se1,
                TtiRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    12,
                    0x1,
                    1,
                    0,
                    ttiri::Se1,
                    TtiRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Scheduling Error 2   SE2"]
            #[inline(always)]
            pub fn se2(
                self,
            ) -> crate::common::RegisterField<
                13,
                0x1,
                1,
                0,
                ttiri::Se2,
                TtiRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    13,
                    0x1,
                    1,
                    0,
                    ttiri::Se2,
                    TtiRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Error Level Changed   ELC. Not set when error level changed during initialization."]
            #[inline(always)]
            pub fn elc(
                self,
            ) -> crate::common::RegisterField<
                14,
                0x1,
                1,
                0,
                ttiri::Elc,
                TtiRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    14,
                    0x1,
                    1,
                    0,
                    ttiri::Elc,
                    TtiRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Initialization Watch Trigger   IWT. The initialization is restarted by resetting IWT."]
            #[inline(always)]
            pub fn iwt(
                self,
            ) -> crate::common::RegisterField<
                15,
                0x1,
                1,
                0,
                ttiri::Iwt,
                TtiRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    15,
                    0x1,
                    1,
                    0,
                    ttiri::Iwt,
                    TtiRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Watch Trigger   WT"]
            #[inline(always)]
            pub fn wt(
                self,
            ) -> crate::common::RegisterField<16, 0x1, 1, 0, ttiri::Wt, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<
                    16,
                    0x1,
                    1,
                    0,
                    ttiri::Wt,
                    TtiRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Application Watchdog   AW"]
            #[inline(always)]
            pub fn aw(
                self,
            ) -> crate::common::RegisterField<17, 0x1, 1, 0, ttiri::Aw, TtiRi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<
                    17,
                    0x1,
                    1,
                    0,
                    ttiri::Aw,
                    TtiRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Configuration Error   CER. Trigger out of order."]
            #[inline(always)]
            pub fn cer(
                self,
            ) -> crate::common::RegisterField<
                18,
                0x1,
                1,
                0,
                ttiri::Cer,
                TtiRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    18,
                    0x1,
                    1,
                    0,
                    ttiri::Cer,
                    TtiRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TtiRi {
            #[inline(always)]
            fn default() -> TtiRi {
                <crate::RegValueT<TtiRi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod ttiri {
            pub struct Sbc_SPEC;
            pub type Sbc = crate::EnumBitfieldStruct<u8, Sbc_SPEC>;
            impl Sbc {
                #[doc = "0 No Basic Cycle started since bit has been reset"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Basic Cycle started"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Smc_SPEC;
            pub type Smc = crate::EnumBitfieldStruct<u8, Smc_SPEC>;
            impl Smc {
                #[doc = "0 No Matrix Cycle started since bit has been reset"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Matrix Cycle started"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Csm_SPEC;
            pub type Csm = crate::EnumBitfieldStruct<u8, Csm_SPEC>;
            impl Csm {
                #[doc = "0 No change in master to slave relation or schedule synchronization"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Master to slave relation or schedule synchronization changed  also set when TTOST.SPL is reset"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Sog_SPEC;
            pub type Sog = crate::EnumBitfieldStruct<u8, Sog_SPEC>;
            impl Sog {
                #[doc = "0 No reference message seen with Next is Gap bit set"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Reference message with Next is Gap bit set becomes valid"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Rtmi_SPEC;
            pub type Rtmi = crate::EnumBitfieldStruct<u8, Rtmi_SPEC>;
            impl Rtmi {
                #[doc = "0 Time mark not reached"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Time mark reached"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ttmi_SPEC;
            pub type Ttmi = crate::EnumBitfieldStruct<u8, Ttmi_SPEC>;
            impl Ttmi {
                #[doc = "0 Time mark not reached"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Time mark reached  Level 0  cycle time TTOCF.IRTO   0x200"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Swe_SPEC;
            pub type Swe = crate::EnumBitfieldStruct<u8, Swe_SPEC>;
            impl Swe {
                #[doc = "0 No rising falling edge at stop watch trigger detected"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Rising falling edge at stop watch trigger detected"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Gtw_SPEC;
            pub type Gtw = crate::EnumBitfieldStruct<u8, Gtw_SPEC>;
            impl Gtw {
                #[doc = "0 No global time wrap occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Global time wrap from 0xFFFF to 0x0000 occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Gtd_SPEC;
            pub type Gtd = crate::EnumBitfieldStruct<u8, Gtd_SPEC>;
            impl Gtd {
                #[doc = "0 No discontinuity of global time"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Discontinuity of global time"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Gte_SPEC;
            pub type Gte = crate::EnumBitfieldStruct<u8, Gte_SPEC>;
            impl Gte {
                #[doc = "0 Synchronization deviation within limit"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Synchronization deviation exceeded limit"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Txu_SPEC;
            pub type Txu = crate::EnumBitfieldStruct<u8, Txu_SPEC>;
            impl Txu {
                #[doc = "0 Number of Tx Trigger as expected"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Less Tx trigger than expected in one matrix cycle"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Txo_SPEC;
            pub type Txo = crate::EnumBitfieldStruct<u8, Txo_SPEC>;
            impl Txo {
                #[doc = "0 Number of Tx Trigger as expected"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 More Tx trigger than expected in one matrix cycle"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Se1_SPEC;
            pub type Se1 = crate::EnumBitfieldStruct<u8, Se1_SPEC>;
            impl Se1 {
                #[doc = "0 No scheduling error 1"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Scheduling error 1 occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Se2_SPEC;
            pub type Se2 = crate::EnumBitfieldStruct<u8, Se2_SPEC>;
            impl Se2 {
                #[doc = "0 No scheduling error 2"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Scheduling error 2 occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Elc_SPEC;
            pub type Elc = crate::EnumBitfieldStruct<u8, Elc_SPEC>;
            impl Elc {
                #[doc = "0 No change in error level"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Error level changed"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Iwt_SPEC;
            pub type Iwt = crate::EnumBitfieldStruct<u8, Iwt_SPEC>;
            impl Iwt {
                #[doc = "0 No missing reference message during system startup"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 No system startup due to missing reference message"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Wt_SPEC;
            pub type Wt = crate::EnumBitfieldStruct<u8, Wt_SPEC>;
            impl Wt {
                #[doc = "0 No missing reference message"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Missing reference message  Level 0  cycle time 0xFF00"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Aw_SPEC;
            pub type Aw = crate::EnumBitfieldStruct<u8, Aw_SPEC>;
            impl Aw {
                #[doc = "0 Application watchdog served in time"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Application watchdog not served in time"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cer_SPEC;
            pub type Cer = crate::EnumBitfieldStruct<u8, Cer_SPEC>;
            impl Cer {
                #[doc = "0 No error found in trigger list"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Error found in trigger list"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtlgTi_SPEC;
        impl crate::sealed::RegSpec for TtlgTi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Local   Global Time 0\n resetvalue={Application Reset:0x0}"]
        pub type TtlgTi = crate::RegValueT<TtlgTi_SPEC>;

        impl TtlgTi {
            #[doc = "Local Time   LT. Non fractional part of local time  incremented once each local NTU. Local time value of TTCAN node"]
            #[inline(always)]
            pub fn lt(
                self,
            ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, TtlgTi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0xffff,1,0,u16, TtlgTi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Global Time   GT. Non fractional part of the sum of the node  8217 s local time and its local        offset. Global time value of TTCAN network"]
            #[inline(always)]
            pub fn gt(
                self,
            ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, TtlgTi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<16,0xffff,1,0,u16, TtlgTi_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for TtlgTi {
            #[inline(always)]
            fn default() -> TtlgTi {
                <crate::RegValueT<TtlgTi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtmlMi_SPEC;
        impl crate::sealed::RegSpec for TtmlMi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Matrix Limits 0\n resetvalue={Application Reset:0x0}"]
        pub type TtmlMi = crate::RegValueT<TtmlMi_SPEC>;

        impl TtmlMi {
            #[doc = "Cycle Count Max   CCM. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. ISO 11898 4  5.2.1 requires  that only the listed cycle count values          are configured. Other values are possible but may lead to inconsistent          matrix cycles."]
            #[inline(always)]
            pub fn ccm(
                self,
            ) -> crate::common::RegisterField<
                0,
                0x3f,
                1,
                0,
                ttmlmi::Ccm,
                TtmlMi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    0,
                    0x3f,
                    1,
                    0,
                    ttmlmi::Ccm,
                    TtmlMi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cycle Start Synchronization   CSS. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Enables sync pulse output at start of cycle."]
            #[inline(always)]
            pub fn css(
                self,
            ) -> crate::common::RegisterField<
                6,
                0x3,
                1,
                0,
                ttmlmi::Css,
                TtmlMi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    6,
                    0x3,
                    1,
                    0,
                    ttmlmi::Css,
                    TtmlMi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Tx Enable Window   TXEW. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Length of Tx enable window  1 16 NTU cycles"]
            #[inline(always)]
            pub fn txew(
                self,
            ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, TtmlMi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<8,0xf,1,0,u8, TtmlMi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Expected Number of Tx Triggers   ENTT. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Expected number of Tx Triggers in one Matrix Cycle"]
            #[inline(always)]
            pub fn entt(
                self,
            ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, TtmlMi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0xfff,1,0,u16, TtmlMi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for TtmlMi {
            #[inline(always)]
            fn default() -> TtmlMi {
                <crate::RegValueT<TtmlMi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod ttmlmi {
            pub struct Ccm_SPEC;
            pub type Ccm = crate::EnumBitfieldStruct<u8, Ccm_SPEC>;
            impl Ccm {
                #[doc = "1 Basic Cycle per Matrix Cycle"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "2 Basic Cycles per Matrix Cycle"]
                pub const CONST_11: Self = Self::new(1);
                #[doc = "4 Basic Cycles per Matrix Cycle"]
                pub const CONST_33: Self = Self::new(3);
                #[doc = "8 Basic Cycles per Matrix Cycle"]
                pub const CONST_77: Self = Self::new(7);
                #[doc = "32 Basic Cycles per Matrix Cycle"]
                pub const CONST_3131: Self = Self::new(31);
                #[doc = "64 Basic Cycles per Matrix Cycle"]
                pub const CONST_6363: Self = Self::new(63);
                #[doc = "16 Basic Cycles per Matrix Cycle"]
                pub const CONST_1515: Self = Self::new(15);
            }
            pub struct Css_SPEC;
            pub type Css = crate::EnumBitfieldStruct<u8, Css_SPEC>;
            impl Css {
                #[doc = "00 No sync pulse"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "01 Sync pulse at        start of basic cycle"]
                pub const CONST_11: Self = Self::new(1);
                #[doc = "10 Sync pulse at        start of matrix cycle"]
                pub const CONST_22: Self = Self::new(2);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtocFi_SPEC;
        impl crate::sealed::RegSpec for TtocFi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Operation Configuration 0\n resetvalue={Application Reset:0x10000}"]
        pub type TtocFi = crate::RegValueT<TtocFi_SPEC>;

        impl TtocFi {
            #[doc = "Operation Mode   OM. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn om(
                self,
            ) -> crate::common::RegisterField<
                0,
                0x3,
                1,
                0,
                ttocfi::Om,
                TtocFi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    0,
                    0x3,
                    1,
                    0,
                    ttocfi::Om,
                    TtocFi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Gap Enable   GEN. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn gen(
                self,
            ) -> crate::common::RegisterField<
                3,
                0x1,
                1,
                0,
                ttocfi::Gen,
                TtocFi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    3,
                    0x1,
                    1,
                    0,
                    ttocfi::Gen,
                    TtocFi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Time Master   TM. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn tm(
                self,
            ) -> crate::common::RegisterField<
                4,
                0x1,
                1,
                0,
                ttocfi::Tm,
                TtocFi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    4,
                    0x1,
                    1,
                    0,
                    ttocfi::Tm,
                    TtocFi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "LD of Synchronization Deviation Limit   LDSDL. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. The Synchronization Deviation Limit SDL is configured by its dual        logarithm LDSDL with SDL   2  LDSDL  160    160 5  .        It should not exceed the clock tolerance given by the CAN bit timing        configuration. LD of Synchronization Deviation Limit  SDL   8804  32  8230 4096"]
            #[inline(always)]
            pub fn ldsdl(
                self,
            ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, TtocFi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<5,0x7,1,0,u8, TtocFi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Initial Reference Trigger Offset   IRTO. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Positive offset  range from 0 to 127"]
            #[inline(always)]
            pub fn irto(
                self,
            ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, TtocFi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<8,0x7f,1,0,u8, TtocFi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Enable External Clock Synchronization   EECS. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. If enabled  TUR configuration  TURCF.NCL only  may be updated during        TTCAN operation."]
            #[inline(always)]
            pub fn eecs(
                self,
            ) -> crate::common::RegisterField<
                15,
                0x1,
                1,
                0,
                ttocfi::Eecs,
                TtocFi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    15,
                    0x1,
                    1,
                    0,
                    ttocfi::Eecs,
                    TtocFi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Application Watchdog Limit   AWL. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. The application watchdog can be disabled by programming AWL to 0x00. Maximum time after which the application has to serve the application        watchdog. The application watchdog is incremented once each 256 NTUs."]
            #[inline(always)]
            pub fn awl(
                self,
            ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, TtocFi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0xff,1,0,u8, TtocFi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Enable Global Time Filtering   EGTF. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn egtf(
                self,
            ) -> crate::common::RegisterField<
                24,
                0x1,
                1,
                0,
                ttocfi::Egtf,
                TtocFi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    24,
                    0x1,
                    1,
                    0,
                    ttocfi::Egtf,
                    TtocFi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Enable Clock Calibration   ECC. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn ecc(
                self,
            ) -> crate::common::RegisterField<
                25,
                0x1,
                1,
                0,
                ttocfi::Ecc,
                TtocFi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    25,
                    0x1,
                    1,
                    0,
                    ttocfi::Ecc,
                    TtocFi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Event Trigger Polarity   EVTP. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn evtp(
                self,
            ) -> crate::common::RegisterField<
                26,
                0x1,
                1,
                0,
                ttocfi::Evtp,
                TtocFi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    26,
                    0x1,
                    1,
                    0,
                    ttocfi::Evtp,
                    TtocFi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TtocFi {
            #[inline(always)]
            fn default() -> TtocFi {
                <crate::RegValueT<TtocFi_SPEC> as RegisterValue<_>>::new(65536)
            }
        }
        pub mod ttocfi {
            pub struct Om_SPEC;
            pub type Om = crate::EnumBitfieldStruct<u8, Om_SPEC>;
            impl Om {
                #[doc = "00 Event driven CAN communication  default"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "01 TTCAN level 1"]
                pub const CONST_11: Self = Self::new(1);
                #[doc = "10 TTCAN level 2"]
                pub const CONST_22: Self = Self::new(2);
                #[doc = "11 TTCAN level 0"]
                pub const CONST_33: Self = Self::new(3);
            }
            pub struct Gen_SPEC;
            pub type Gen = crate::EnumBitfieldStruct<u8, Gen_SPEC>;
            impl Gen {
                #[doc = "0 Strictly time triggered operation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 External event synchronized time triggered operation"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tm_SPEC;
            pub type Tm = crate::EnumBitfieldStruct<u8, Tm_SPEC>;
            impl Tm {
                #[doc = "0 Time Master function disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Potential Time Master"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Eecs_SPEC;
            pub type Eecs = crate::EnumBitfieldStruct<u8, Eecs_SPEC>;
            impl Eecs {
                #[doc = "0 External clock synchronization in TTCAN Level 0 2 disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 External clock synchronization in TTCAN Level 0 2 enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Egtf_SPEC;
            pub type Egtf = crate::EnumBitfieldStruct<u8, Egtf_SPEC>;
            impl Egtf {
                #[doc = "0 Global time filtering in TTCAN Level 0 2 is disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Global time filtering in TTCAN Level 0 2 is enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ecc_SPEC;
            pub type Ecc = crate::EnumBitfieldStruct<u8, Ecc_SPEC>;
            impl Ecc {
                #[doc = "0 Automatic clock calibration in TTCAN Level 0 2 is disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Automatic clock calibration in TTCAN Level 0 2 is enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Evtp_SPEC;
            pub type Evtp = crate::EnumBitfieldStruct<u8, Evtp_SPEC>;
            impl Evtp {
                #[doc = "0 Rising edge trigger"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Falling edge trigger"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtocNi_SPEC;
        impl crate::sealed::RegSpec for TtocNi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Operation Control 0\n resetvalue={Application Reset:0x0}"]
        pub type TtocNi = crate::RegValueT<TtocNi_SPEC>;

        impl TtocNi {
            #[doc = "Set Global time   SGT. Writing a   8216 1  8217  to SGT sets TTOST.WGDT if the node is the actual Time        Master. SGT is reset after one Host clock period. The global time preset        takes effect when the node transmits the next reference message with the        Master Ref Mark modified by the preset value written to TTGTP."]
            #[inline(always)]
            pub fn sgt(
                self,
            ) -> crate::common::RegisterFieldBool<0, 1, 0, TtocNi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<0,1,0,TtocNi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "External Clock Synchronization   ECS. Writing a   8216 1  8217  to ECS sets TTOST.WECS if the node is the actual Time        Master. ECS is reset after one Host clock period. The external clock        synchronization takes effect at the start of the next basic cycle."]
            #[inline(always)]
            pub fn ecs(
                self,
            ) -> crate::common::RegisterFieldBool<1, 1, 0, TtocNi_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<1,1,0,TtocNi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Stop Watch Polarity   SWP"]
            #[inline(always)]
            pub fn swp(
                self,
            ) -> crate::common::RegisterField<
                2,
                0x1,
                1,
                0,
                ttocni::Swp,
                TtocNi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    2,
                    0x1,
                    1,
                    0,
                    ttocni::Swp,
                    TtocNi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Stop Watch Source   SWS"]
            #[inline(always)]
            pub fn sws(
                self,
            ) -> crate::common::RegisterField<
                3,
                0x3,
                1,
                0,
                ttocni::Sws,
                TtocNi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    3,
                    0x3,
                    1,
                    0,
                    ttocni::Sws,
                    TtocNi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Register Time Mark Interrupt Pulse Enable   RTIE. Register time mark interrupts are configured by register TTTMK. A        register time mark interrupt pulse with the length of one TTCAN clock        period is generated when the time referenced by TTOCN.TMC  cycle  local         or global  equals TTTMK.TM  independent of the synchronization state."]
            #[inline(always)]
            pub fn rtie(
                self,
            ) -> crate::common::RegisterField<
                5,
                0x1,
                1,
                0,
                ttocni::Rtie,
                TtocNi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    5,
                    0x1,
                    1,
                    0,
                    ttocni::Rtie,
                    TtocNi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Register Time Mark Compare   TMC. When changing the time mark reference  cycle  local  global time   it          is recommended to first write TMC     8220 00  8221   then reconfigure TTTMK  and          finally set TMC to the intended time reference."]
            #[inline(always)]
            pub fn tmc(
                self,
            ) -> crate::common::RegisterField<
                6,
                0x3,
                1,
                0,
                ttocni::Tmc,
                TtocNi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    6,
                    0x3,
                    1,
                    0,
                    ttocni::Tmc,
                    TtocNi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Trigger Time Mark Interrupt Pulse Enable   TTIE. External time mark events are configured by trigger memory element TMEX.        A trigger time mark interrupt pulse is generated when the trigger memory        element becomes active  and the M CAN is in synchronization state In Schedule or In Gap."]
            #[inline(always)]
            pub fn ttie(
                self,
            ) -> crate::common::RegisterField<
                8,
                0x1,
                1,
                0,
                ttocni::Ttie,
                TtocNi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    8,
                    0x1,
                    1,
                    0,
                    ttocni::Ttie,
                    TtocNi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Gap Control Select   GCS"]
            #[inline(always)]
            pub fn gcs(
                self,
            ) -> crate::common::RegisterField<
                9,
                0x1,
                1,
                0,
                ttocni::Gcs,
                TtocNi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    9,
                    0x1,
                    1,
                    0,
                    ttocni::Gcs,
                    TtocNi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Finish Gap   FGP. Set by the CPU  reset by each reference message"]
            #[inline(always)]
            pub fn fgp(
                self,
            ) -> crate::common::RegisterField<
                10,
                0x1,
                1,
                0,
                ttocni::Fgp,
                TtocNi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    10,
                    0x1,
                    1,
                    0,
                    ttocni::Fgp,
                    TtocNi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Time Mark Gap   TMG"]
            #[inline(always)]
            pub fn tmg(
                self,
            ) -> crate::common::RegisterField<
                11,
                0x1,
                1,
                0,
                ttocni::Tmg,
                TtocNi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    11,
                    0x1,
                    1,
                    0,
                    ttocni::Tmg,
                    TtocNi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Next is Gap   NIG. This bit can only be set when the M CAN is the actual Time Master and when it is configured for external        event synchronized time triggered operation  TTOCF.GEN     8216 1  8217"]
            #[inline(always)]
            pub fn nig(
                self,
            ) -> crate::common::RegisterField<
                12,
                0x1,
                1,
                0,
                ttocni::Nig,
                TtocNi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    12,
                    0x1,
                    1,
                    0,
                    ttocni::Nig,
                    TtocNi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "External Synchronization Control   ESCN. If enabled the M CAN synchronizes its cycle time phase to an external event signalled by a        rising edge at the event trigger."]
            #[inline(always)]
            pub fn escn(
                self,
            ) -> crate::common::RegisterField<
                13,
                0x1,
                1,
                0,
                ttocni::Escn,
                TtocNi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    13,
                    0x1,
                    1,
                    0,
                    ttocni::Escn,
                    TtocNi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "TT Operation Control Register Locked   LCKC. Set by a write access to register TTOCN. Reset when the updated        configuration has been synchronized into the CAN clock domain."]
            #[inline(always)]
            pub fn lckc(
                self,
            ) -> crate::common::RegisterField<
                15,
                0x1,
                1,
                0,
                ttocni::Lckc,
                TtocNi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    15,
                    0x1,
                    1,
                    0,
                    ttocni::Lckc,
                    TtocNi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TtocNi {
            #[inline(always)]
            fn default() -> TtocNi {
                <crate::RegValueT<TtocNi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod ttocni {
            pub struct Swp_SPEC;
            pub type Swp = crate::EnumBitfieldStruct<u8, Swp_SPEC>;
            impl Swp {
                #[doc = "0 Rising edge trigger"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Falling edge trigger"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Sws_SPEC;
            pub type Sws = crate::EnumBitfieldStruct<u8, Sws_SPEC>;
            impl Sws {
                #[doc = "00 Stop Watch disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "01 Actual value of cycle time is copied to TTCPT.SWV"]
                pub const CONST_11: Self = Self::new(1);
                #[doc = "10 Actual value of local time is copied to TTCPT.SWV"]
                pub const CONST_22: Self = Self::new(2);
                #[doc = "11 Actual value of global time is copied to TTCPT.SWV"]
                pub const CONST_33: Self = Self::new(3);
            }
            pub struct Rtie_SPEC;
            pub type Rtie = crate::EnumBitfieldStruct<u8, Rtie_SPEC>;
            impl Rtie {
                #[doc = "0 Register Time Mark Interrupt output disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Register Time Mark Interrupt output enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tmc_SPEC;
            pub type Tmc = crate::EnumBitfieldStruct<u8, Tmc_SPEC>;
            impl Tmc {
                #[doc = "00 No Register Time Mark Interrupt generated"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "01 Register Time Mark Interrupt if Time Mark   cycle time"]
                pub const CONST_11: Self = Self::new(1);
                #[doc = "10 Register Time Mark Interrupt if Time Mark   local time"]
                pub const CONST_22: Self = Self::new(2);
                #[doc = "11 Register Time Mark Interrupt if Time Mark   global time"]
                pub const CONST_33: Self = Self::new(3);
            }
            pub struct Ttie_SPEC;
            pub type Ttie = crate::EnumBitfieldStruct<u8, Ttie_SPEC>;
            impl Ttie {
                #[doc = "0 Trigger Time        Mark Interrupt output disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Trigger Time        Mark Interrupt output enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Gcs_SPEC;
            pub type Gcs = crate::EnumBitfieldStruct<u8, Gcs_SPEC>;
            impl Gcs {
                #[doc = "0 Gap control independent from event trigger"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Gap control by the event trigger"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Fgp_SPEC;
            pub type Fgp = crate::EnumBitfieldStruct<u8, Fgp_SPEC>;
            impl Fgp {
                #[doc = "0 No reference message requested"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Application requested start of reference message"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tmg_SPEC;
            pub type Tmg = crate::EnumBitfieldStruct<u8, Tmg_SPEC>;
            impl Tmg {
                #[doc = "0 Reset by each reference message"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Next reference message started when Register Time Mark interrupt TTIR.RTMI is activated"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Nig_SPEC;
            pub type Nig = crate::EnumBitfieldStruct<u8, Nig_SPEC>;
            impl Nig {
                #[doc = "0 No action  reset by reception of any reference message"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit next reference message with Next is Gap    1"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Escn_SPEC;
            pub type Escn = crate::EnumBitfieldStruct<u8, Escn_SPEC>;
            impl Escn {
                #[doc = "0 External synchronization disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 External synchronization enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Lckc_SPEC;
            pub type Lckc = crate::EnumBitfieldStruct<u8, Lckc_SPEC>;
            impl Lckc {
                #[doc = "0 Write access to TTOCN enabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Write access to TTOCN locked"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtosTi_SPEC;
        impl crate::sealed::RegSpec for TtosTi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Operation Status 0\n resetvalue={Application Reset:0x20000080}"]
        pub type TtosTi = crate::RegValueT<TtosTi_SPEC>;

        impl TtosTi {
            #[doc = "Error Level   EL"]
            #[inline(always)]
            pub fn el(
                self,
            ) -> crate::common::RegisterField<0, 0x3, 1, 0, ttosti::El, TtosTi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<
                    0,
                    0x3,
                    1,
                    0,
                    ttosti::El,
                    TtosTi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Master State   MS"]
            #[inline(always)]
            pub fn ms(
                self,
            ) -> crate::common::RegisterField<2, 0x3, 1, 0, ttosti::Ms, TtosTi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<
                    2,
                    0x3,
                    1,
                    0,
                    ttosti::Ms,
                    TtosTi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Synchronization State   SYS"]
            #[inline(always)]
            pub fn sys(
                self,
            ) -> crate::common::RegisterField<
                4,
                0x3,
                1,
                0,
                ttosti::Sys,
                TtosTi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    4,
                    0x3,
                    1,
                    0,
                    ttosti::Sys,
                    TtosTi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Quality of Global Time Phase   QGTP. Only relevant in TTCAN Level  160 0 and Level  160 2  otherwise fixed to   8216 0  8217 ."]
            #[inline(always)]
            pub fn qgtp(
                self,
            ) -> crate::common::RegisterField<
                6,
                0x1,
                1,
                0,
                ttosti::Qgtp,
                TtosTi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    6,
                    0x1,
                    1,
                    0,
                    ttosti::Qgtp,
                    TtosTi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Quality of Clock Speed   QCS. Only relevant in TTCAN Level  160 0 and Level  160 2  otherwise fixed to   8216 1  8217 ."]
            #[inline(always)]
            pub fn qcs(
                self,
            ) -> crate::common::RegisterField<
                7,
                0x1,
                1,
                0,
                ttosti::Qcs,
                TtosTi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    7,
                    0x1,
                    1,
                    0,
                    ttosti::Qcs,
                    TtosTi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Reference Trigger Offset   RTO. The Reference Trigger Offset value is a signed integer with a range from         127  0x81  to 127  0x7F . There is no notification when the lower limit        of  127 is reached. In case the M CAN becomes Time Master  MS 1 0      8220 11  8221    the reset of RTO is delayed due to        synchronization between Host and CAN clock domain. For time slaves the        value configured by TTOCF.IRTO is read. Actual Reference Trigger offset value"]
            #[inline(always)]
            pub fn rto(
                self,
            ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, TtosTi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<8,0xff,1,0,u8, TtosTi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Wait for Global Time Discontinuity   WGTD"]
            #[inline(always)]
            pub fn wgtd(
                self,
            ) -> crate::common::RegisterField<
                22,
                0x1,
                1,
                0,
                ttosti::Wgtd,
                TtosTi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    22,
                    0x1,
                    1,
                    0,
                    ttosti::Wgtd,
                    TtosTi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Gap Finished Indicator   GFI. Set when the CPU writes TTOCN.FGP  or by a time mark interrupt if        TMG  160    160   8216 1  8217   or via event trigger input if TTOCN.GCS  160    160   8216 1  8217 . Not set by        Ref Trigger Gap or when Gap is finished by another node sending a        reference message."]
            #[inline(always)]
            pub fn gfi(
                self,
            ) -> crate::common::RegisterField<
                23,
                0x1,
                1,
                0,
                ttosti::Gfi,
                TtosTi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    23,
                    0x1,
                    1,
                    0,
                    ttosti::Gfi,
                    TtosTi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Time Master Priority   TMP"]
            #[inline(always)]
            pub fn tmp(
                self,
            ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, TtosTi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<24,0x7,1,0,u8, TtosTi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Gap Started Indicator   GSI"]
            #[inline(always)]
            pub fn gsi(
                self,
            ) -> crate::common::RegisterField<
                27,
                0x1,
                1,
                0,
                ttosti::Gsi,
                TtosTi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    27,
                    0x1,
                    1,
                    0,
                    ttosti::Gsi,
                    TtosTi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Wait for Event   WFE"]
            #[inline(always)]
            pub fn wfe(
                self,
            ) -> crate::common::RegisterField<
                28,
                0x1,
                1,
                0,
                ttosti::Wfe,
                TtosTi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    28,
                    0x1,
                    1,
                    0,
                    ttosti::Wfe,
                    TtosTi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Application Watchdog Event   AWE. The application watchdog is served by reading TTOST. When the watchdog        is not served in time  bit AWE is set  all TTCAN communication is        stopped  and the M CAN is set into Bus Monitoring Mode."]
            #[inline(always)]
            pub fn awe(
                self,
            ) -> crate::common::RegisterField<
                29,
                0x1,
                1,
                0,
                ttosti::Awe,
                TtosTi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    29,
                    0x1,
                    1,
                    0,
                    ttosti::Awe,
                    TtosTi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Wait for External Clock Synchronization   WECS"]
            #[inline(always)]
            pub fn wecs(
                self,
            ) -> crate::common::RegisterField<
                30,
                0x1,
                1,
                0,
                ttosti::Wecs,
                TtosTi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    30,
                    0x1,
                    1,
                    0,
                    ttosti::Wecs,
                    TtosTi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Schedule Phase Lock   SPL. The bit is valid only when external synchronization is enabled         TTOCN.ESCN     8216 1  8217  . In this case it signals that the difference between        cycle time configured by TTGTP.CTP and the cycle time at the rising edge        at the event trigger is less or equal 9 NTU."]
            #[inline(always)]
            pub fn spl(
                self,
            ) -> crate::common::RegisterField<
                31,
                0x1,
                1,
                0,
                ttosti::Spl,
                TtosTi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    31,
                    0x1,
                    1,
                    0,
                    ttosti::Spl,
                    TtosTi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TtosTi {
            #[inline(always)]
            fn default() -> TtosTi {
                <crate::RegValueT<TtosTi_SPEC> as RegisterValue<_>>::new(536871040)
            }
        }
        pub mod ttosti {
            pub struct El_SPEC;
            pub type El = crate::EnumBitfieldStruct<u8, El_SPEC>;
            impl El {
                #[doc = "00 Severity 0   No Error"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "01 Severity 1   Warning"]
                pub const CONST_11: Self = Self::new(1);
                #[doc = "10 Severity 2   Error"]
                pub const CONST_22: Self = Self::new(2);
                #[doc = "11 Severity 3   Severe Error"]
                pub const CONST_33: Self = Self::new(3);
            }
            pub struct Ms_SPEC;
            pub type Ms = crate::EnumBitfieldStruct<u8, Ms_SPEC>;
            impl Ms {
                #[doc = "00 Master Off  no        master properties relevant"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "01 Operating as        Time Slave"]
                pub const CONST_11: Self = Self::new(1);
                #[doc = "10 Operating as        Backup Time Master"]
                pub const CONST_22: Self = Self::new(2);
                #[doc = "11 Operating as        current Time Master"]
                pub const CONST_33: Self = Self::new(3);
            }
            pub struct Sys_SPEC;
            pub type Sys = crate::EnumBitfieldStruct<u8, Sys_SPEC>;
            impl Sys {
                #[doc = "00 Out of Synchronization"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "01 Synchronizing to TTCAN communication"]
                pub const CONST_11: Self = Self::new(1);
                #[doc = "10 Schedule suspended by Gap  In Gap"]
                pub const CONST_22: Self = Self::new(2);
                #[doc = "11 Synchronized to schedule  In Schedule"]
                pub const CONST_33: Self = Self::new(3);
            }
            pub struct Qgtp_SPEC;
            pub type Qgtp = crate::EnumBitfieldStruct<u8, Qgtp_SPEC>;
            impl Qgtp {
                #[doc = "0 Global time not valid"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Global time in phase with Time Master"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Qcs_SPEC;
            pub type Qcs = crate::EnumBitfieldStruct<u8, Qcs_SPEC>;
            impl Qcs {
                #[doc = "0 Local clock speed not synchronized to Time Master clock speed"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Synchronization Deviation   SDL"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Wgtd_SPEC;
            pub type Wgtd = crate::EnumBitfieldStruct<u8, Wgtd_SPEC>;
            impl Wgtd {
                #[doc = "0 No global time preset pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Node waits for the global time preset to take effect. The bit is reset when the node has transmitted a reference message with Disc Bit    1  or after it received a reference message."]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Gfi_SPEC;
            pub type Gfi = crate::EnumBitfieldStruct<u8, Gfi_SPEC>;
            impl Gfi {
                #[doc = "0 Reset at the end of each reference message"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Gap finished by M CAN"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Gsi_SPEC;
            pub type Gsi = crate::EnumBitfieldStruct<u8, Gsi_SPEC>;
            impl Gsi {
                #[doc = "0 No Gap in schedule  reset by each reference message and for all time slaves"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Gap time after Basic Cycle has started"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Wfe_SPEC;
            pub type Wfe = crate::EnumBitfieldStruct<u8, Wfe_SPEC>;
            impl Wfe {
                #[doc = "0 No Gap announced  reset by a reference message with Next is Gap    0"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Reference message with Next is Gap    1  received"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Awe_SPEC;
            pub type Awe = crate::EnumBitfieldStruct<u8, Awe_SPEC>;
            impl Awe {
                #[doc = "0 Application Watchdog served in time"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Failed to serve Application Watchdog in time"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Wecs_SPEC;
            pub type Wecs = crate::EnumBitfieldStruct<u8, Wecs_SPEC>;
            impl Wecs {
                #[doc = "0 No external clock synchronization pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Node waits for external clock synchronization to take effect. The bit is reset at the start of the next basic cycle."]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Spl_SPEC;
            pub type Spl = crate::EnumBitfieldStruct<u8, Spl_SPEC>;
            impl Spl {
                #[doc = "0 Phase outside        range"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Phase inside        range"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TtrmCi_SPEC;
        impl crate::sealed::RegSpec for TtrmCi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Reference Message Configuration 0\n resetvalue={Application Reset:0x0}"]
        pub type TtrmCi = crate::RegValueT<TtrmCi_SPEC>;

        impl TtrmCi {
            #[doc = "Reference Identifier   RID. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Identifier transmitted with reference message and used for reference        message filtering. Standard or extended reference identifier depending        on bit XTD. A standard identifier has to be written to ID 28 18 ."]
            #[inline(always)]
            pub fn rid(
                self,
            ) -> crate::common::RegisterField<
                0,
                0x1fffffff,
                1,
                0,
                u32,
                TtrmCi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    0,
                    0x1fffffff,
                    1,
                    0,
                    u32,
                    TtrmCi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Extended Identifier   XTD. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn xtd(
                self,
            ) -> crate::common::RegisterField<
                30,
                0x1,
                1,
                0,
                ttrmci::Xtd,
                TtrmCi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    30,
                    0x1,
                    1,
                    0,
                    ttrmci::Xtd,
                    TtrmCi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Reference Message Payload Select   RMPS. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Ignored in case of time slaves."]
            #[inline(always)]
            pub fn rmps(
                self,
            ) -> crate::common::RegisterField<
                31,
                0x1,
                1,
                0,
                ttrmci::Rmps,
                TtrmCi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    31,
                    0x1,
                    1,
                    0,
                    ttrmci::Rmps,
                    TtrmCi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TtrmCi {
            #[inline(always)]
            fn default() -> TtrmCi {
                <crate::RegValueT<TtrmCi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod ttrmci {
            pub struct Xtd_SPEC;
            pub type Xtd = crate::EnumBitfieldStruct<u8, Xtd_SPEC>;
            impl Xtd {
                #[doc = "0 11 bit standard identifier"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 29 bit extended identifier"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Rmps_SPEC;
            pub type Rmps = crate::EnumBitfieldStruct<u8, Rmps_SPEC>;
            impl Rmps {
                #[doc = "0 Reference message has no additional payload"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 The following elements are taken from Tx Buffer 0   Message Marker MM  Event FIFO Control EFC  Data Length Code DLC  Data Bytes DB  Level 1  bytes 2 8  Level 0 2  bytes 5 8"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TttmCi_SPEC;
        impl crate::sealed::RegSpec for TttmCi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Trigger Memory Configuration 0\n resetvalue={Application Reset:0x0}"]
        pub type TttmCi = crate::RegValueT<TttmCi_SPEC>;

        impl TttmCi {
            #[doc = "Trigger Memory Start Address   TMSA. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Start address of Trigger Memory in Message RAM."]
            #[inline(always)]
            pub fn tmsa(
                self,
            ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, TttmCi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<2,0x3fff,1,0,u16, TttmCi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Trigger Memory Elements   TME. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn tme(
                self,
            ) -> crate::common::RegisterField<
                16,
                0x7f,
                1,
                0,
                tttmci::Tme,
                TttmCi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    16,
                    0x7f,
                    1,
                    0,
                    tttmci::Tme,
                    TttmCi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TttmCi {
            #[inline(always)]
            fn default() -> TttmCi {
                <crate::RegValueT<TttmCi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod tttmci {
            pub struct Tme_SPEC;
            pub type Tme = crate::EnumBitfieldStruct<u8, Tme_SPEC>;
            impl Tme {
                #[doc = "No Trigger Memory"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "64 Trigger Memory elements"]
                pub const REST_127: Self = Self::new(127);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TttmKi_SPEC;
        impl crate::sealed::RegSpec for TttmKi_SPEC {
            type DataType = u32;
        }
        #[doc = "TT Time Mark 0\n resetvalue={Application Reset:0x0}"]
        pub type TttmKi = crate::RegValueT<TttmKi_SPEC>;

        impl TttmKi {
            #[doc = "Time Mark   TM. When using byte access to register TTTMK it is recommended to first          disable the time mark compare function  TTOCN.TMC     8220 00  8221   to avoid          compares on inconsistent register values."]
            #[inline(always)]
            pub fn tm(
                self,
            ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, TttmKi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xffff,1,0,u16, TttmKi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Time Mark Cycle Code   TICC. Cycle count for which the time mark is valid."]
            #[inline(always)]
            pub fn ticc(
                self,
            ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, TttmKi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x7f,1,0,u8, TttmKi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "TT Time Mark Register Locked   LCKM. Always set by a write access to registers TTOCN. Set by write access to        register TTTMK when TTOCN.TMC   8800   quot 00  8221 . Reset        when the registers have been synchronized into the CAN clock domain."]
            #[inline(always)]
            pub fn lckm(
                self,
            ) -> crate::common::RegisterField<
                31,
                0x1,
                1,
                0,
                tttmki::Lckm,
                TttmKi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    31,
                    0x1,
                    1,
                    0,
                    tttmki::Lckm,
                    TttmKi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TttmKi {
            #[inline(always)]
            fn default() -> TttmKi {
                <crate::RegValueT<TttmKi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod tttmki {
            pub struct Lckm_SPEC;
            pub type Lckm = crate::EnumBitfieldStruct<u8, Lckm_SPEC>;
            impl Lckm {
                #[doc = "0 Write access to TTTMK enabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Write access to TTTMK locked"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TurcFi_SPEC;
        impl crate::sealed::RegSpec for TurcFi_SPEC {
            type DataType = u32;
        }
        #[doc = "TUR Configuration 0\n resetvalue={Application Reset:0x10000000}"]
        pub type TurcFi = crate::RegValueT<TurcFi_SPEC>;

        impl TurcFi {
            #[doc = "Numerator Configuration Low   NCL. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Write access to the TUR Numerator Configuration Low is only possible        during configuration with TURCF.ELT     8216 0  8217  or if TTOCF.EECS  external        clock synchronization enabled  is set. When a new value for NCL is        written outside TT Configuration Mode  the new value takes effect when        TTOST.WECS is cleared to   8216 0  8217 . NCL is locked TTOST.WECS is   8216 1  8217 . If NC  160  lt   160 7  160 x  160 DC in TTCAN Level 1  then it is required that subsequent          time marks in the Trigger Memory must differ by at least 2 NTU."]
            #[inline(always)]
            pub fn ncl(
                self,
            ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, TurcFi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xffff,1,0,u16, TurcFi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Denominator Configuration   DC. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn dc(
                self,
            ) -> crate::common::RegisterField<
                16,
                0x3fff,
                1,
                0,
                turcfi::Dc,
                TurcFi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    16,
                    0x3fff,
                    1,
                    0,
                    turcfi::Dc,
                    TurcFi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Enable Local Time   ELT. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Local time is started by setting ELT. It remains active until ELT is          reset or until the next hardware reset. TURCF.DC is locked when          TURCF.ELT     8216 1  8217 . If ELT is written to   8216 0  8217   the readable value will          stay at   8216 1  8217  until the new value has been synchronized into the CAN          clock domain. During this time write access to the other bits of the          register remains locked."]
            #[inline(always)]
            pub fn elt(
                self,
            ) -> crate::common::RegisterField<
                31,
                0x1,
                1,
                0,
                turcfi::Elt,
                TurcFi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    31,
                    0x1,
                    1,
                    0,
                    turcfi::Elt,
                    TurcFi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TurcFi {
            #[inline(always)]
            fn default() -> TurcFi {
                <crate::RegValueT<TurcFi_SPEC> as RegisterValue<_>>::new(268435456)
            }
        }
        pub mod turcfi {
            pub struct Dc_SPEC;
            pub type Dc = crate::EnumBitfieldStruct<u8, Dc_SPEC>;
            impl Dc {
                #[doc = "Illegal value"]
                pub const CONST_00: Self = Self::new(0);
            }
            pub struct Elt_SPEC;
            pub type Elt = crate::EnumBitfieldStruct<u8, Elt_SPEC>;
            impl Elt {
                #[doc = "0 Local time is stopped  default"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Local time is enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TurnAi_SPEC;
        impl crate::sealed::RegSpec for TurnAi_SPEC {
            type DataType = u32;
        }
        #[doc = "TUR Numerator Actual 0\n resetvalue={Application Reset:0x10000}"]
        pub type TurnAi = crate::RegValueT<TurnAi_SPEC>;

        impl TurnAi {
            #[doc = "Numerator Actual Value   NAV"]
            #[inline(always)]
            pub fn nav(
                self,
            ) -> crate::common::RegisterField<
                0,
                0x3ffff,
                1,
                0,
                turnai::Nav,
                TurnAi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    0,
                    0x3ffff,
                    1,
                    0,
                    turnai::Nav,
                    TurnAi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TurnAi {
            #[inline(always)]
            fn default() -> TurnAi {
                <crate::RegValueT<TurnAi_SPEC> as RegisterValue<_>>::new(65536)
            }
        }
        pub mod turnai {
            pub struct Nav_SPEC;
            pub type Nav = crate::EnumBitfieldStruct<u32, Nav_SPEC>;
            impl Nav {
                #[doc = "Illegal value"]
                pub const REST_262143: Self = Self::new(262143);
            }
        }
    }
    #[doc = "TX"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tx(pub(super) *mut u8);
    unsafe impl core::marker::Send for Tx {}
    unsafe impl core::marker::Sync for Tx {}
    impl Tx {
        #[doc = "Tx Buffer Add Request 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txbari(&self) -> crate::common::Reg<tx::TxbaRi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "Tx Buffer Cancellation Finished 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txbcfi(&self) -> crate::common::Reg<tx::TxbcFi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
        }
        #[doc = "Tx Buffer Cancellation Finished Interrupt Enable 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txbciei(&self) -> crate::common::Reg<tx::TxbciEi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
        }
        #[doc = "Tx Buffer Cancellation Request 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txbcri(&self) -> crate::common::Reg<tx::TxbcRi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "Tx Buffer Configuration 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txbci(&self) -> crate::common::Reg<tx::TxbCi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "Tx Buffer Request Pending 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txbrpi(&self) -> crate::common::Reg<tx::TxbrPi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "Tx Buffer Transmission Interrupt Enable 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txbtiei(&self) -> crate::common::Reg<tx::TxbtiEi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
        }
        #[doc = "Tx Buffer Transmission Occurred 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txbtoi(&self) -> crate::common::Reg<tx::TxbtOi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
        }
        #[doc = "Tx Event FIFO Acknowledge 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txefai(&self) -> crate::common::Reg<tx::TxefAi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
        }
        #[doc = "Tx Event FIFO Configuration 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txefci(&self) -> crate::common::Reg<tx::TxefCi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
        }
        #[doc = "Tx Event FIFO Status 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txefsi(&self) -> crate::common::Reg<tx::TxefSi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
        }
        #[doc = "Tx Buffer Element Size Configuration 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txesci(&self) -> crate::common::Reg<tx::TxesCi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "Tx FIFO Queue Status 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn txfqsi(&self) -> crate::common::Reg<tx::TxfqSi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
        }
    }
    pub mod tx {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxbaRi_SPEC;
        impl crate::sealed::RegSpec for TxbaRi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Buffer Add Request 0\n resetvalue={Application Reset:0x0}"]
        pub type TxbaRi = crate::RegValueT<TxbaRi_SPEC>;

        impl TxbaRi {
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar0(
                self,
            ) -> crate::common::RegisterField<
                0,
                0x1,
                1,
                0,
                txbari::Ar0,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    0,
                    0x1,
                    1,
                    0,
                    txbari::Ar0,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar1(
                self,
            ) -> crate::common::RegisterField<
                1,
                0x1,
                1,
                0,
                txbari::Ar1,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    1,
                    0x1,
                    1,
                    0,
                    txbari::Ar1,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar2(
                self,
            ) -> crate::common::RegisterField<
                2,
                0x1,
                1,
                0,
                txbari::Ar2,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    2,
                    0x1,
                    1,
                    0,
                    txbari::Ar2,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar3(
                self,
            ) -> crate::common::RegisterField<
                3,
                0x1,
                1,
                0,
                txbari::Ar3,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    3,
                    0x1,
                    1,
                    0,
                    txbari::Ar3,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar4(
                self,
            ) -> crate::common::RegisterField<
                4,
                0x1,
                1,
                0,
                txbari::Ar4,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    4,
                    0x1,
                    1,
                    0,
                    txbari::Ar4,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar5(
                self,
            ) -> crate::common::RegisterField<
                5,
                0x1,
                1,
                0,
                txbari::Ar5,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    5,
                    0x1,
                    1,
                    0,
                    txbari::Ar5,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar6(
                self,
            ) -> crate::common::RegisterField<
                6,
                0x1,
                1,
                0,
                txbari::Ar6,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    6,
                    0x1,
                    1,
                    0,
                    txbari::Ar6,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar7(
                self,
            ) -> crate::common::RegisterField<
                7,
                0x1,
                1,
                0,
                txbari::Ar7,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    7,
                    0x1,
                    1,
                    0,
                    txbari::Ar7,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar8(
                self,
            ) -> crate::common::RegisterField<
                8,
                0x1,
                1,
                0,
                txbari::Ar8,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    8,
                    0x1,
                    1,
                    0,
                    txbari::Ar8,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar9(
                self,
            ) -> crate::common::RegisterField<
                9,
                0x1,
                1,
                0,
                txbari::Ar9,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    9,
                    0x1,
                    1,
                    0,
                    txbari::Ar9,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar10(
                self,
            ) -> crate::common::RegisterField<
                10,
                0x1,
                1,
                0,
                txbari::Ar10,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    10,
                    0x1,
                    1,
                    0,
                    txbari::Ar10,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar11(
                self,
            ) -> crate::common::RegisterField<
                11,
                0x1,
                1,
                0,
                txbari::Ar11,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    11,
                    0x1,
                    1,
                    0,
                    txbari::Ar11,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar12(
                self,
            ) -> crate::common::RegisterField<
                12,
                0x1,
                1,
                0,
                txbari::Ar12,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    12,
                    0x1,
                    1,
                    0,
                    txbari::Ar12,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar13(
                self,
            ) -> crate::common::RegisterField<
                13,
                0x1,
                1,
                0,
                txbari::Ar13,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    13,
                    0x1,
                    1,
                    0,
                    txbari::Ar13,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar14(
                self,
            ) -> crate::common::RegisterField<
                14,
                0x1,
                1,
                0,
                txbari::Ar14,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    14,
                    0x1,
                    1,
                    0,
                    txbari::Ar14,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar15(
                self,
            ) -> crate::common::RegisterField<
                15,
                0x1,
                1,
                0,
                txbari::Ar15,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    15,
                    0x1,
                    1,
                    0,
                    txbari::Ar15,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar16(
                self,
            ) -> crate::common::RegisterField<
                16,
                0x1,
                1,
                0,
                txbari::Ar16,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    16,
                    0x1,
                    1,
                    0,
                    txbari::Ar16,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar17(
                self,
            ) -> crate::common::RegisterField<
                17,
                0x1,
                1,
                0,
                txbari::Ar17,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    17,
                    0x1,
                    1,
                    0,
                    txbari::Ar17,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar18(
                self,
            ) -> crate::common::RegisterField<
                18,
                0x1,
                1,
                0,
                txbari::Ar18,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    18,
                    0x1,
                    1,
                    0,
                    txbari::Ar18,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar19(
                self,
            ) -> crate::common::RegisterField<
                19,
                0x1,
                1,
                0,
                txbari::Ar19,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    19,
                    0x1,
                    1,
                    0,
                    txbari::Ar19,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar20(
                self,
            ) -> crate::common::RegisterField<
                20,
                0x1,
                1,
                0,
                txbari::Ar20,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    20,
                    0x1,
                    1,
                    0,
                    txbari::Ar20,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar21(
                self,
            ) -> crate::common::RegisterField<
                21,
                0x1,
                1,
                0,
                txbari::Ar21,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    21,
                    0x1,
                    1,
                    0,
                    txbari::Ar21,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar22(
                self,
            ) -> crate::common::RegisterField<
                22,
                0x1,
                1,
                0,
                txbari::Ar22,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    22,
                    0x1,
                    1,
                    0,
                    txbari::Ar22,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar23(
                self,
            ) -> crate::common::RegisterField<
                23,
                0x1,
                1,
                0,
                txbari::Ar23,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    23,
                    0x1,
                    1,
                    0,
                    txbari::Ar23,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar24(
                self,
            ) -> crate::common::RegisterField<
                24,
                0x1,
                1,
                0,
                txbari::Ar24,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    24,
                    0x1,
                    1,
                    0,
                    txbari::Ar24,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar25(
                self,
            ) -> crate::common::RegisterField<
                25,
                0x1,
                1,
                0,
                txbari::Ar25,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    25,
                    0x1,
                    1,
                    0,
                    txbari::Ar25,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar26(
                self,
            ) -> crate::common::RegisterField<
                26,
                0x1,
                1,
                0,
                txbari::Ar26,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    26,
                    0x1,
                    1,
                    0,
                    txbari::Ar26,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar27(
                self,
            ) -> crate::common::RegisterField<
                27,
                0x1,
                1,
                0,
                txbari::Ar27,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    27,
                    0x1,
                    1,
                    0,
                    txbari::Ar27,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar28(
                self,
            ) -> crate::common::RegisterField<
                28,
                0x1,
                1,
                0,
                txbari::Ar28,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    28,
                    0x1,
                    1,
                    0,
                    txbari::Ar28,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar29(
                self,
            ) -> crate::common::RegisterField<
                29,
                0x1,
                1,
                0,
                txbari::Ar29,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    29,
                    0x1,
                    1,
                    0,
                    txbari::Ar29,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar30(
                self,
            ) -> crate::common::RegisterField<
                30,
                0x1,
                1,
                0,
                txbari::Ar30,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    30,
                    0x1,
                    1,
                    0,
                    txbari::Ar30,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Add Request Tx Buffer 31   AR"]
            #[inline(always)]
            pub fn ar31(
                self,
            ) -> crate::common::RegisterField<
                31,
                0x1,
                1,
                0,
                txbari::Ar31,
                TxbaRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    31,
                    0x1,
                    1,
                    0,
                    txbari::Ar31,
                    TxbaRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TxbaRi {
            #[inline(always)]
            fn default() -> TxbaRi {
                <crate::RegValueT<TxbaRi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod txbari {
            pub struct Ar0_SPEC;
            pub type Ar0 = crate::EnumBitfieldStruct<u8, Ar0_SPEC>;
            impl Ar0 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar1_SPEC;
            pub type Ar1 = crate::EnumBitfieldStruct<u8, Ar1_SPEC>;
            impl Ar1 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar2_SPEC;
            pub type Ar2 = crate::EnumBitfieldStruct<u8, Ar2_SPEC>;
            impl Ar2 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar3_SPEC;
            pub type Ar3 = crate::EnumBitfieldStruct<u8, Ar3_SPEC>;
            impl Ar3 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar4_SPEC;
            pub type Ar4 = crate::EnumBitfieldStruct<u8, Ar4_SPEC>;
            impl Ar4 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar5_SPEC;
            pub type Ar5 = crate::EnumBitfieldStruct<u8, Ar5_SPEC>;
            impl Ar5 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar6_SPEC;
            pub type Ar6 = crate::EnumBitfieldStruct<u8, Ar6_SPEC>;
            impl Ar6 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar7_SPEC;
            pub type Ar7 = crate::EnumBitfieldStruct<u8, Ar7_SPEC>;
            impl Ar7 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar8_SPEC;
            pub type Ar8 = crate::EnumBitfieldStruct<u8, Ar8_SPEC>;
            impl Ar8 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar9_SPEC;
            pub type Ar9 = crate::EnumBitfieldStruct<u8, Ar9_SPEC>;
            impl Ar9 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar10_SPEC;
            pub type Ar10 = crate::EnumBitfieldStruct<u8, Ar10_SPEC>;
            impl Ar10 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar11_SPEC;
            pub type Ar11 = crate::EnumBitfieldStruct<u8, Ar11_SPEC>;
            impl Ar11 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar12_SPEC;
            pub type Ar12 = crate::EnumBitfieldStruct<u8, Ar12_SPEC>;
            impl Ar12 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar13_SPEC;
            pub type Ar13 = crate::EnumBitfieldStruct<u8, Ar13_SPEC>;
            impl Ar13 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar14_SPEC;
            pub type Ar14 = crate::EnumBitfieldStruct<u8, Ar14_SPEC>;
            impl Ar14 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar15_SPEC;
            pub type Ar15 = crate::EnumBitfieldStruct<u8, Ar15_SPEC>;
            impl Ar15 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar16_SPEC;
            pub type Ar16 = crate::EnumBitfieldStruct<u8, Ar16_SPEC>;
            impl Ar16 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar17_SPEC;
            pub type Ar17 = crate::EnumBitfieldStruct<u8, Ar17_SPEC>;
            impl Ar17 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar18_SPEC;
            pub type Ar18 = crate::EnumBitfieldStruct<u8, Ar18_SPEC>;
            impl Ar18 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar19_SPEC;
            pub type Ar19 = crate::EnumBitfieldStruct<u8, Ar19_SPEC>;
            impl Ar19 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar20_SPEC;
            pub type Ar20 = crate::EnumBitfieldStruct<u8, Ar20_SPEC>;
            impl Ar20 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar21_SPEC;
            pub type Ar21 = crate::EnumBitfieldStruct<u8, Ar21_SPEC>;
            impl Ar21 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar22_SPEC;
            pub type Ar22 = crate::EnumBitfieldStruct<u8, Ar22_SPEC>;
            impl Ar22 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar23_SPEC;
            pub type Ar23 = crate::EnumBitfieldStruct<u8, Ar23_SPEC>;
            impl Ar23 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar24_SPEC;
            pub type Ar24 = crate::EnumBitfieldStruct<u8, Ar24_SPEC>;
            impl Ar24 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar25_SPEC;
            pub type Ar25 = crate::EnumBitfieldStruct<u8, Ar25_SPEC>;
            impl Ar25 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar26_SPEC;
            pub type Ar26 = crate::EnumBitfieldStruct<u8, Ar26_SPEC>;
            impl Ar26 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar27_SPEC;
            pub type Ar27 = crate::EnumBitfieldStruct<u8, Ar27_SPEC>;
            impl Ar27 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar28_SPEC;
            pub type Ar28 = crate::EnumBitfieldStruct<u8, Ar28_SPEC>;
            impl Ar28 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar29_SPEC;
            pub type Ar29 = crate::EnumBitfieldStruct<u8, Ar29_SPEC>;
            impl Ar29 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar30_SPEC;
            pub type Ar30 = crate::EnumBitfieldStruct<u8, Ar30_SPEC>;
            impl Ar30 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ar31_SPEC;
            pub type Ar31 = crate::EnumBitfieldStruct<u8, Ar31_SPEC>;
            impl Ar31 {
                #[doc = "0 No transmission        request added"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        requested added"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxbcFi_SPEC;
        impl crate::sealed::RegSpec for TxbcFi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Buffer Cancellation Finished 0\n resetvalue={Application Reset:0x0}"]
        pub type TxbcFi = crate::RegValueT<TxbcFi_SPEC>;

        impl TxbcFi {
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf0(
                self,
            ) -> crate::common::RegisterField<
                0,
                0x1,
                1,
                0,
                txbcfi::Cf0,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    0,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf0,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf1(
                self,
            ) -> crate::common::RegisterField<
                1,
                0x1,
                1,
                0,
                txbcfi::Cf1,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    1,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf1,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf2(
                self,
            ) -> crate::common::RegisterField<
                2,
                0x1,
                1,
                0,
                txbcfi::Cf2,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    2,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf2,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf3(
                self,
            ) -> crate::common::RegisterField<
                3,
                0x1,
                1,
                0,
                txbcfi::Cf3,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    3,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf3,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf4(
                self,
            ) -> crate::common::RegisterField<
                4,
                0x1,
                1,
                0,
                txbcfi::Cf4,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    4,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf4,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf5(
                self,
            ) -> crate::common::RegisterField<
                5,
                0x1,
                1,
                0,
                txbcfi::Cf5,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    5,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf5,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf6(
                self,
            ) -> crate::common::RegisterField<
                6,
                0x1,
                1,
                0,
                txbcfi::Cf6,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    6,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf6,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf7(
                self,
            ) -> crate::common::RegisterField<
                7,
                0x1,
                1,
                0,
                txbcfi::Cf7,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    7,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf7,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf8(
                self,
            ) -> crate::common::RegisterField<
                8,
                0x1,
                1,
                0,
                txbcfi::Cf8,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    8,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf8,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf9(
                self,
            ) -> crate::common::RegisterField<
                9,
                0x1,
                1,
                0,
                txbcfi::Cf9,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    9,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf9,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf10(
                self,
            ) -> crate::common::RegisterField<
                10,
                0x1,
                1,
                0,
                txbcfi::Cf10,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    10,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf10,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf11(
                self,
            ) -> crate::common::RegisterField<
                11,
                0x1,
                1,
                0,
                txbcfi::Cf11,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    11,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf11,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf12(
                self,
            ) -> crate::common::RegisterField<
                12,
                0x1,
                1,
                0,
                txbcfi::Cf12,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    12,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf12,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf13(
                self,
            ) -> crate::common::RegisterField<
                13,
                0x1,
                1,
                0,
                txbcfi::Cf13,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    13,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf13,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf14(
                self,
            ) -> crate::common::RegisterField<
                14,
                0x1,
                1,
                0,
                txbcfi::Cf14,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    14,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf14,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf15(
                self,
            ) -> crate::common::RegisterField<
                15,
                0x1,
                1,
                0,
                txbcfi::Cf15,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    15,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf15,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf16(
                self,
            ) -> crate::common::RegisterField<
                16,
                0x1,
                1,
                0,
                txbcfi::Cf16,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    16,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf16,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf17(
                self,
            ) -> crate::common::RegisterField<
                17,
                0x1,
                1,
                0,
                txbcfi::Cf17,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    17,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf17,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf18(
                self,
            ) -> crate::common::RegisterField<
                18,
                0x1,
                1,
                0,
                txbcfi::Cf18,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    18,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf18,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf19(
                self,
            ) -> crate::common::RegisterField<
                19,
                0x1,
                1,
                0,
                txbcfi::Cf19,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    19,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf19,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf20(
                self,
            ) -> crate::common::RegisterField<
                20,
                0x1,
                1,
                0,
                txbcfi::Cf20,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    20,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf20,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf21(
                self,
            ) -> crate::common::RegisterField<
                21,
                0x1,
                1,
                0,
                txbcfi::Cf21,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    21,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf21,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf22(
                self,
            ) -> crate::common::RegisterField<
                22,
                0x1,
                1,
                0,
                txbcfi::Cf22,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    22,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf22,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf23(
                self,
            ) -> crate::common::RegisterField<
                23,
                0x1,
                1,
                0,
                txbcfi::Cf23,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    23,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf23,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf24(
                self,
            ) -> crate::common::RegisterField<
                24,
                0x1,
                1,
                0,
                txbcfi::Cf24,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    24,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf24,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf25(
                self,
            ) -> crate::common::RegisterField<
                25,
                0x1,
                1,
                0,
                txbcfi::Cf25,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    25,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf25,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf26(
                self,
            ) -> crate::common::RegisterField<
                26,
                0x1,
                1,
                0,
                txbcfi::Cf26,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    26,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf26,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf27(
                self,
            ) -> crate::common::RegisterField<
                27,
                0x1,
                1,
                0,
                txbcfi::Cf27,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    27,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf27,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf28(
                self,
            ) -> crate::common::RegisterField<
                28,
                0x1,
                1,
                0,
                txbcfi::Cf28,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    28,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf28,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf29(
                self,
            ) -> crate::common::RegisterField<
                29,
                0x1,
                1,
                0,
                txbcfi::Cf29,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    29,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf29,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf30(
                self,
            ) -> crate::common::RegisterField<
                30,
                0x1,
                1,
                0,
                txbcfi::Cf30,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    30,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf30,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Tx Buffer 31   CF"]
            #[inline(always)]
            pub fn cf31(
                self,
            ) -> crate::common::RegisterField<
                31,
                0x1,
                1,
                0,
                txbcfi::Cf31,
                TxbcFi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    31,
                    0x1,
                    1,
                    0,
                    txbcfi::Cf31,
                    TxbcFi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TxbcFi {
            #[inline(always)]
            fn default() -> TxbcFi {
                <crate::RegValueT<TxbcFi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod txbcfi {
            pub struct Cf0_SPEC;
            pub type Cf0 = crate::EnumBitfieldStruct<u8, Cf0_SPEC>;
            impl Cf0 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf1_SPEC;
            pub type Cf1 = crate::EnumBitfieldStruct<u8, Cf1_SPEC>;
            impl Cf1 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf2_SPEC;
            pub type Cf2 = crate::EnumBitfieldStruct<u8, Cf2_SPEC>;
            impl Cf2 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf3_SPEC;
            pub type Cf3 = crate::EnumBitfieldStruct<u8, Cf3_SPEC>;
            impl Cf3 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf4_SPEC;
            pub type Cf4 = crate::EnumBitfieldStruct<u8, Cf4_SPEC>;
            impl Cf4 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf5_SPEC;
            pub type Cf5 = crate::EnumBitfieldStruct<u8, Cf5_SPEC>;
            impl Cf5 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf6_SPEC;
            pub type Cf6 = crate::EnumBitfieldStruct<u8, Cf6_SPEC>;
            impl Cf6 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf7_SPEC;
            pub type Cf7 = crate::EnumBitfieldStruct<u8, Cf7_SPEC>;
            impl Cf7 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf8_SPEC;
            pub type Cf8 = crate::EnumBitfieldStruct<u8, Cf8_SPEC>;
            impl Cf8 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf9_SPEC;
            pub type Cf9 = crate::EnumBitfieldStruct<u8, Cf9_SPEC>;
            impl Cf9 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf10_SPEC;
            pub type Cf10 = crate::EnumBitfieldStruct<u8, Cf10_SPEC>;
            impl Cf10 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf11_SPEC;
            pub type Cf11 = crate::EnumBitfieldStruct<u8, Cf11_SPEC>;
            impl Cf11 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf12_SPEC;
            pub type Cf12 = crate::EnumBitfieldStruct<u8, Cf12_SPEC>;
            impl Cf12 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf13_SPEC;
            pub type Cf13 = crate::EnumBitfieldStruct<u8, Cf13_SPEC>;
            impl Cf13 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf14_SPEC;
            pub type Cf14 = crate::EnumBitfieldStruct<u8, Cf14_SPEC>;
            impl Cf14 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf15_SPEC;
            pub type Cf15 = crate::EnumBitfieldStruct<u8, Cf15_SPEC>;
            impl Cf15 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf16_SPEC;
            pub type Cf16 = crate::EnumBitfieldStruct<u8, Cf16_SPEC>;
            impl Cf16 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf17_SPEC;
            pub type Cf17 = crate::EnumBitfieldStruct<u8, Cf17_SPEC>;
            impl Cf17 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf18_SPEC;
            pub type Cf18 = crate::EnumBitfieldStruct<u8, Cf18_SPEC>;
            impl Cf18 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf19_SPEC;
            pub type Cf19 = crate::EnumBitfieldStruct<u8, Cf19_SPEC>;
            impl Cf19 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf20_SPEC;
            pub type Cf20 = crate::EnumBitfieldStruct<u8, Cf20_SPEC>;
            impl Cf20 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf21_SPEC;
            pub type Cf21 = crate::EnumBitfieldStruct<u8, Cf21_SPEC>;
            impl Cf21 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf22_SPEC;
            pub type Cf22 = crate::EnumBitfieldStruct<u8, Cf22_SPEC>;
            impl Cf22 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf23_SPEC;
            pub type Cf23 = crate::EnumBitfieldStruct<u8, Cf23_SPEC>;
            impl Cf23 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf24_SPEC;
            pub type Cf24 = crate::EnumBitfieldStruct<u8, Cf24_SPEC>;
            impl Cf24 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf25_SPEC;
            pub type Cf25 = crate::EnumBitfieldStruct<u8, Cf25_SPEC>;
            impl Cf25 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf26_SPEC;
            pub type Cf26 = crate::EnumBitfieldStruct<u8, Cf26_SPEC>;
            impl Cf26 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf27_SPEC;
            pub type Cf27 = crate::EnumBitfieldStruct<u8, Cf27_SPEC>;
            impl Cf27 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf28_SPEC;
            pub type Cf28 = crate::EnumBitfieldStruct<u8, Cf28_SPEC>;
            impl Cf28 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf29_SPEC;
            pub type Cf29 = crate::EnumBitfieldStruct<u8, Cf29_SPEC>;
            impl Cf29 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf30_SPEC;
            pub type Cf30 = crate::EnumBitfieldStruct<u8, Cf30_SPEC>;
            impl Cf30 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cf31_SPEC;
            pub type Cf31 = crate::EnumBitfieldStruct<u8, Cf31_SPEC>;
            impl Cf31 {
                #[doc = "0 No transmit        buffer cancellation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmit buffer        cancellation finished"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxbciEi_SPEC;
        impl crate::sealed::RegSpec for TxbciEi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Buffer Cancellation Finished Interrupt Enable 0\n resetvalue={Application Reset:0x0}"]
        pub type TxbciEi = crate::RegValueT<TxbciEi_SPEC>;

        impl TxbciEi {
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie0(
                self,
            ) -> crate::common::RegisterField<
                0,
                0x1,
                1,
                0,
                txbciei::Cfie0,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    0,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie0,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie1(
                self,
            ) -> crate::common::RegisterField<
                1,
                0x1,
                1,
                0,
                txbciei::Cfie1,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    1,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie1,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie2(
                self,
            ) -> crate::common::RegisterField<
                2,
                0x1,
                1,
                0,
                txbciei::Cfie2,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    2,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie2,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie3(
                self,
            ) -> crate::common::RegisterField<
                3,
                0x1,
                1,
                0,
                txbciei::Cfie3,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    3,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie3,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie4(
                self,
            ) -> crate::common::RegisterField<
                4,
                0x1,
                1,
                0,
                txbciei::Cfie4,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    4,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie4,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie5(
                self,
            ) -> crate::common::RegisterField<
                5,
                0x1,
                1,
                0,
                txbciei::Cfie5,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    5,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie5,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie6(
                self,
            ) -> crate::common::RegisterField<
                6,
                0x1,
                1,
                0,
                txbciei::Cfie6,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    6,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie6,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie7(
                self,
            ) -> crate::common::RegisterField<
                7,
                0x1,
                1,
                0,
                txbciei::Cfie7,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    7,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie7,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie8(
                self,
            ) -> crate::common::RegisterField<
                8,
                0x1,
                1,
                0,
                txbciei::Cfie8,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    8,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie8,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie9(
                self,
            ) -> crate::common::RegisterField<
                9,
                0x1,
                1,
                0,
                txbciei::Cfie9,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    9,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie9,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie10(
                self,
            ) -> crate::common::RegisterField<
                10,
                0x1,
                1,
                0,
                txbciei::Cfie10,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    10,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie10,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie11(
                self,
            ) -> crate::common::RegisterField<
                11,
                0x1,
                1,
                0,
                txbciei::Cfie11,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    11,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie11,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie12(
                self,
            ) -> crate::common::RegisterField<
                12,
                0x1,
                1,
                0,
                txbciei::Cfie12,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    12,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie12,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie13(
                self,
            ) -> crate::common::RegisterField<
                13,
                0x1,
                1,
                0,
                txbciei::Cfie13,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    13,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie13,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie14(
                self,
            ) -> crate::common::RegisterField<
                14,
                0x1,
                1,
                0,
                txbciei::Cfie14,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    14,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie14,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie15(
                self,
            ) -> crate::common::RegisterField<
                15,
                0x1,
                1,
                0,
                txbciei::Cfie15,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    15,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie15,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie16(
                self,
            ) -> crate::common::RegisterField<
                16,
                0x1,
                1,
                0,
                txbciei::Cfie16,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    16,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie16,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie17(
                self,
            ) -> crate::common::RegisterField<
                17,
                0x1,
                1,
                0,
                txbciei::Cfie17,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    17,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie17,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie18(
                self,
            ) -> crate::common::RegisterField<
                18,
                0x1,
                1,
                0,
                txbciei::Cfie18,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    18,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie18,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie19(
                self,
            ) -> crate::common::RegisterField<
                19,
                0x1,
                1,
                0,
                txbciei::Cfie19,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    19,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie19,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie20(
                self,
            ) -> crate::common::RegisterField<
                20,
                0x1,
                1,
                0,
                txbciei::Cfie20,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    20,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie20,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie21(
                self,
            ) -> crate::common::RegisterField<
                21,
                0x1,
                1,
                0,
                txbciei::Cfie21,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    21,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie21,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie22(
                self,
            ) -> crate::common::RegisterField<
                22,
                0x1,
                1,
                0,
                txbciei::Cfie22,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    22,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie22,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie23(
                self,
            ) -> crate::common::RegisterField<
                23,
                0x1,
                1,
                0,
                txbciei::Cfie23,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    23,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie23,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie24(
                self,
            ) -> crate::common::RegisterField<
                24,
                0x1,
                1,
                0,
                txbciei::Cfie24,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    24,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie24,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie25(
                self,
            ) -> crate::common::RegisterField<
                25,
                0x1,
                1,
                0,
                txbciei::Cfie25,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    25,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie25,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie26(
                self,
            ) -> crate::common::RegisterField<
                26,
                0x1,
                1,
                0,
                txbciei::Cfie26,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    26,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie26,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie27(
                self,
            ) -> crate::common::RegisterField<
                27,
                0x1,
                1,
                0,
                txbciei::Cfie27,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    27,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie27,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie28(
                self,
            ) -> crate::common::RegisterField<
                28,
                0x1,
                1,
                0,
                txbciei::Cfie28,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    28,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie28,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie29(
                self,
            ) -> crate::common::RegisterField<
                29,
                0x1,
                1,
                0,
                txbciei::Cfie29,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    29,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie29,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie30(
                self,
            ) -> crate::common::RegisterField<
                30,
                0x1,
                1,
                0,
                txbciei::Cfie30,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    30,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie30,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Finished Interrupt Enable Tx Buffer 31   CFIE"]
            #[inline(always)]
            pub fn cfie31(
                self,
            ) -> crate::common::RegisterField<
                31,
                0x1,
                1,
                0,
                txbciei::Cfie31,
                TxbciEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    31,
                    0x1,
                    1,
                    0,
                    txbciei::Cfie31,
                    TxbciEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TxbciEi {
            #[inline(always)]
            fn default() -> TxbciEi {
                <crate::RegValueT<TxbciEi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod txbciei {
            pub struct Cfie0_SPEC;
            pub type Cfie0 = crate::EnumBitfieldStruct<u8, Cfie0_SPEC>;
            impl Cfie0 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie1_SPEC;
            pub type Cfie1 = crate::EnumBitfieldStruct<u8, Cfie1_SPEC>;
            impl Cfie1 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie2_SPEC;
            pub type Cfie2 = crate::EnumBitfieldStruct<u8, Cfie2_SPEC>;
            impl Cfie2 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie3_SPEC;
            pub type Cfie3 = crate::EnumBitfieldStruct<u8, Cfie3_SPEC>;
            impl Cfie3 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie4_SPEC;
            pub type Cfie4 = crate::EnumBitfieldStruct<u8, Cfie4_SPEC>;
            impl Cfie4 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie5_SPEC;
            pub type Cfie5 = crate::EnumBitfieldStruct<u8, Cfie5_SPEC>;
            impl Cfie5 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie6_SPEC;
            pub type Cfie6 = crate::EnumBitfieldStruct<u8, Cfie6_SPEC>;
            impl Cfie6 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie7_SPEC;
            pub type Cfie7 = crate::EnumBitfieldStruct<u8, Cfie7_SPEC>;
            impl Cfie7 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie8_SPEC;
            pub type Cfie8 = crate::EnumBitfieldStruct<u8, Cfie8_SPEC>;
            impl Cfie8 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie9_SPEC;
            pub type Cfie9 = crate::EnumBitfieldStruct<u8, Cfie9_SPEC>;
            impl Cfie9 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie10_SPEC;
            pub type Cfie10 = crate::EnumBitfieldStruct<u8, Cfie10_SPEC>;
            impl Cfie10 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie11_SPEC;
            pub type Cfie11 = crate::EnumBitfieldStruct<u8, Cfie11_SPEC>;
            impl Cfie11 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie12_SPEC;
            pub type Cfie12 = crate::EnumBitfieldStruct<u8, Cfie12_SPEC>;
            impl Cfie12 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie13_SPEC;
            pub type Cfie13 = crate::EnumBitfieldStruct<u8, Cfie13_SPEC>;
            impl Cfie13 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie14_SPEC;
            pub type Cfie14 = crate::EnumBitfieldStruct<u8, Cfie14_SPEC>;
            impl Cfie14 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie15_SPEC;
            pub type Cfie15 = crate::EnumBitfieldStruct<u8, Cfie15_SPEC>;
            impl Cfie15 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie16_SPEC;
            pub type Cfie16 = crate::EnumBitfieldStruct<u8, Cfie16_SPEC>;
            impl Cfie16 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie17_SPEC;
            pub type Cfie17 = crate::EnumBitfieldStruct<u8, Cfie17_SPEC>;
            impl Cfie17 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie18_SPEC;
            pub type Cfie18 = crate::EnumBitfieldStruct<u8, Cfie18_SPEC>;
            impl Cfie18 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie19_SPEC;
            pub type Cfie19 = crate::EnumBitfieldStruct<u8, Cfie19_SPEC>;
            impl Cfie19 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie20_SPEC;
            pub type Cfie20 = crate::EnumBitfieldStruct<u8, Cfie20_SPEC>;
            impl Cfie20 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie21_SPEC;
            pub type Cfie21 = crate::EnumBitfieldStruct<u8, Cfie21_SPEC>;
            impl Cfie21 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie22_SPEC;
            pub type Cfie22 = crate::EnumBitfieldStruct<u8, Cfie22_SPEC>;
            impl Cfie22 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie23_SPEC;
            pub type Cfie23 = crate::EnumBitfieldStruct<u8, Cfie23_SPEC>;
            impl Cfie23 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie24_SPEC;
            pub type Cfie24 = crate::EnumBitfieldStruct<u8, Cfie24_SPEC>;
            impl Cfie24 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie25_SPEC;
            pub type Cfie25 = crate::EnumBitfieldStruct<u8, Cfie25_SPEC>;
            impl Cfie25 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie26_SPEC;
            pub type Cfie26 = crate::EnumBitfieldStruct<u8, Cfie26_SPEC>;
            impl Cfie26 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie27_SPEC;
            pub type Cfie27 = crate::EnumBitfieldStruct<u8, Cfie27_SPEC>;
            impl Cfie27 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie28_SPEC;
            pub type Cfie28 = crate::EnumBitfieldStruct<u8, Cfie28_SPEC>;
            impl Cfie28 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie29_SPEC;
            pub type Cfie29 = crate::EnumBitfieldStruct<u8, Cfie29_SPEC>;
            impl Cfie29 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie30_SPEC;
            pub type Cfie30 = crate::EnumBitfieldStruct<u8, Cfie30_SPEC>;
            impl Cfie30 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cfie31_SPEC;
            pub type Cfie31 = crate::EnumBitfieldStruct<u8, Cfie31_SPEC>;
            impl Cfie31 {
                #[doc = "0 Cancellation        finished interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        finished interrupt enabled"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxbcRi_SPEC;
        impl crate::sealed::RegSpec for TxbcRi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Buffer Cancellation Request 0\n resetvalue={Application Reset:0x0}"]
        pub type TxbcRi = crate::RegValueT<TxbcRi_SPEC>;

        impl TxbcRi {
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr0(
                self,
            ) -> crate::common::RegisterField<
                0,
                0x1,
                1,
                0,
                txbcri::Cr0,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    0,
                    0x1,
                    1,
                    0,
                    txbcri::Cr0,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr1(
                self,
            ) -> crate::common::RegisterField<
                1,
                0x1,
                1,
                0,
                txbcri::Cr1,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    1,
                    0x1,
                    1,
                    0,
                    txbcri::Cr1,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr2(
                self,
            ) -> crate::common::RegisterField<
                2,
                0x1,
                1,
                0,
                txbcri::Cr2,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    2,
                    0x1,
                    1,
                    0,
                    txbcri::Cr2,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr3(
                self,
            ) -> crate::common::RegisterField<
                3,
                0x1,
                1,
                0,
                txbcri::Cr3,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    3,
                    0x1,
                    1,
                    0,
                    txbcri::Cr3,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr4(
                self,
            ) -> crate::common::RegisterField<
                4,
                0x1,
                1,
                0,
                txbcri::Cr4,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    4,
                    0x1,
                    1,
                    0,
                    txbcri::Cr4,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr5(
                self,
            ) -> crate::common::RegisterField<
                5,
                0x1,
                1,
                0,
                txbcri::Cr5,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    5,
                    0x1,
                    1,
                    0,
                    txbcri::Cr5,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr6(
                self,
            ) -> crate::common::RegisterField<
                6,
                0x1,
                1,
                0,
                txbcri::Cr6,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    6,
                    0x1,
                    1,
                    0,
                    txbcri::Cr6,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr7(
                self,
            ) -> crate::common::RegisterField<
                7,
                0x1,
                1,
                0,
                txbcri::Cr7,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    7,
                    0x1,
                    1,
                    0,
                    txbcri::Cr7,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr8(
                self,
            ) -> crate::common::RegisterField<
                8,
                0x1,
                1,
                0,
                txbcri::Cr8,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    8,
                    0x1,
                    1,
                    0,
                    txbcri::Cr8,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr9(
                self,
            ) -> crate::common::RegisterField<
                9,
                0x1,
                1,
                0,
                txbcri::Cr9,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    9,
                    0x1,
                    1,
                    0,
                    txbcri::Cr9,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr10(
                self,
            ) -> crate::common::RegisterField<
                10,
                0x1,
                1,
                0,
                txbcri::Cr10,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    10,
                    0x1,
                    1,
                    0,
                    txbcri::Cr10,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr11(
                self,
            ) -> crate::common::RegisterField<
                11,
                0x1,
                1,
                0,
                txbcri::Cr11,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    11,
                    0x1,
                    1,
                    0,
                    txbcri::Cr11,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr12(
                self,
            ) -> crate::common::RegisterField<
                12,
                0x1,
                1,
                0,
                txbcri::Cr12,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    12,
                    0x1,
                    1,
                    0,
                    txbcri::Cr12,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr13(
                self,
            ) -> crate::common::RegisterField<
                13,
                0x1,
                1,
                0,
                txbcri::Cr13,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    13,
                    0x1,
                    1,
                    0,
                    txbcri::Cr13,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr14(
                self,
            ) -> crate::common::RegisterField<
                14,
                0x1,
                1,
                0,
                txbcri::Cr14,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    14,
                    0x1,
                    1,
                    0,
                    txbcri::Cr14,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr15(
                self,
            ) -> crate::common::RegisterField<
                15,
                0x1,
                1,
                0,
                txbcri::Cr15,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    15,
                    0x1,
                    1,
                    0,
                    txbcri::Cr15,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr16(
                self,
            ) -> crate::common::RegisterField<
                16,
                0x1,
                1,
                0,
                txbcri::Cr16,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    16,
                    0x1,
                    1,
                    0,
                    txbcri::Cr16,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr17(
                self,
            ) -> crate::common::RegisterField<
                17,
                0x1,
                1,
                0,
                txbcri::Cr17,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    17,
                    0x1,
                    1,
                    0,
                    txbcri::Cr17,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr18(
                self,
            ) -> crate::common::RegisterField<
                18,
                0x1,
                1,
                0,
                txbcri::Cr18,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    18,
                    0x1,
                    1,
                    0,
                    txbcri::Cr18,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr19(
                self,
            ) -> crate::common::RegisterField<
                19,
                0x1,
                1,
                0,
                txbcri::Cr19,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    19,
                    0x1,
                    1,
                    0,
                    txbcri::Cr19,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr20(
                self,
            ) -> crate::common::RegisterField<
                20,
                0x1,
                1,
                0,
                txbcri::Cr20,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    20,
                    0x1,
                    1,
                    0,
                    txbcri::Cr20,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr21(
                self,
            ) -> crate::common::RegisterField<
                21,
                0x1,
                1,
                0,
                txbcri::Cr21,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    21,
                    0x1,
                    1,
                    0,
                    txbcri::Cr21,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr22(
                self,
            ) -> crate::common::RegisterField<
                22,
                0x1,
                1,
                0,
                txbcri::Cr22,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    22,
                    0x1,
                    1,
                    0,
                    txbcri::Cr22,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr23(
                self,
            ) -> crate::common::RegisterField<
                23,
                0x1,
                1,
                0,
                txbcri::Cr23,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    23,
                    0x1,
                    1,
                    0,
                    txbcri::Cr23,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr24(
                self,
            ) -> crate::common::RegisterField<
                24,
                0x1,
                1,
                0,
                txbcri::Cr24,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    24,
                    0x1,
                    1,
                    0,
                    txbcri::Cr24,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr25(
                self,
            ) -> crate::common::RegisterField<
                25,
                0x1,
                1,
                0,
                txbcri::Cr25,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    25,
                    0x1,
                    1,
                    0,
                    txbcri::Cr25,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr26(
                self,
            ) -> crate::common::RegisterField<
                26,
                0x1,
                1,
                0,
                txbcri::Cr26,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    26,
                    0x1,
                    1,
                    0,
                    txbcri::Cr26,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr27(
                self,
            ) -> crate::common::RegisterField<
                27,
                0x1,
                1,
                0,
                txbcri::Cr27,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    27,
                    0x1,
                    1,
                    0,
                    txbcri::Cr27,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr28(
                self,
            ) -> crate::common::RegisterField<
                28,
                0x1,
                1,
                0,
                txbcri::Cr28,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    28,
                    0x1,
                    1,
                    0,
                    txbcri::Cr28,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr29(
                self,
            ) -> crate::common::RegisterField<
                29,
                0x1,
                1,
                0,
                txbcri::Cr29,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    29,
                    0x1,
                    1,
                    0,
                    txbcri::Cr29,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr30(
                self,
            ) -> crate::common::RegisterField<
                30,
                0x1,
                1,
                0,
                txbcri::Cr30,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    30,
                    0x1,
                    1,
                    0,
                    txbcri::Cr30,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Cancellation Request Tx Buffer 31   CR"]
            #[inline(always)]
            pub fn cr31(
                self,
            ) -> crate::common::RegisterField<
                31,
                0x1,
                1,
                0,
                txbcri::Cr31,
                TxbcRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    31,
                    0x1,
                    1,
                    0,
                    txbcri::Cr31,
                    TxbcRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TxbcRi {
            #[inline(always)]
            fn default() -> TxbcRi {
                <crate::RegValueT<TxbcRi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod txbcri {
            pub struct Cr0_SPEC;
            pub type Cr0 = crate::EnumBitfieldStruct<u8, Cr0_SPEC>;
            impl Cr0 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr1_SPEC;
            pub type Cr1 = crate::EnumBitfieldStruct<u8, Cr1_SPEC>;
            impl Cr1 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr2_SPEC;
            pub type Cr2 = crate::EnumBitfieldStruct<u8, Cr2_SPEC>;
            impl Cr2 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr3_SPEC;
            pub type Cr3 = crate::EnumBitfieldStruct<u8, Cr3_SPEC>;
            impl Cr3 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr4_SPEC;
            pub type Cr4 = crate::EnumBitfieldStruct<u8, Cr4_SPEC>;
            impl Cr4 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr5_SPEC;
            pub type Cr5 = crate::EnumBitfieldStruct<u8, Cr5_SPEC>;
            impl Cr5 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr6_SPEC;
            pub type Cr6 = crate::EnumBitfieldStruct<u8, Cr6_SPEC>;
            impl Cr6 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr7_SPEC;
            pub type Cr7 = crate::EnumBitfieldStruct<u8, Cr7_SPEC>;
            impl Cr7 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr8_SPEC;
            pub type Cr8 = crate::EnumBitfieldStruct<u8, Cr8_SPEC>;
            impl Cr8 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr9_SPEC;
            pub type Cr9 = crate::EnumBitfieldStruct<u8, Cr9_SPEC>;
            impl Cr9 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr10_SPEC;
            pub type Cr10 = crate::EnumBitfieldStruct<u8, Cr10_SPEC>;
            impl Cr10 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr11_SPEC;
            pub type Cr11 = crate::EnumBitfieldStruct<u8, Cr11_SPEC>;
            impl Cr11 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr12_SPEC;
            pub type Cr12 = crate::EnumBitfieldStruct<u8, Cr12_SPEC>;
            impl Cr12 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr13_SPEC;
            pub type Cr13 = crate::EnumBitfieldStruct<u8, Cr13_SPEC>;
            impl Cr13 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr14_SPEC;
            pub type Cr14 = crate::EnumBitfieldStruct<u8, Cr14_SPEC>;
            impl Cr14 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr15_SPEC;
            pub type Cr15 = crate::EnumBitfieldStruct<u8, Cr15_SPEC>;
            impl Cr15 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr16_SPEC;
            pub type Cr16 = crate::EnumBitfieldStruct<u8, Cr16_SPEC>;
            impl Cr16 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr17_SPEC;
            pub type Cr17 = crate::EnumBitfieldStruct<u8, Cr17_SPEC>;
            impl Cr17 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr18_SPEC;
            pub type Cr18 = crate::EnumBitfieldStruct<u8, Cr18_SPEC>;
            impl Cr18 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr19_SPEC;
            pub type Cr19 = crate::EnumBitfieldStruct<u8, Cr19_SPEC>;
            impl Cr19 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr20_SPEC;
            pub type Cr20 = crate::EnumBitfieldStruct<u8, Cr20_SPEC>;
            impl Cr20 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr21_SPEC;
            pub type Cr21 = crate::EnumBitfieldStruct<u8, Cr21_SPEC>;
            impl Cr21 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr22_SPEC;
            pub type Cr22 = crate::EnumBitfieldStruct<u8, Cr22_SPEC>;
            impl Cr22 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr23_SPEC;
            pub type Cr23 = crate::EnumBitfieldStruct<u8, Cr23_SPEC>;
            impl Cr23 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr24_SPEC;
            pub type Cr24 = crate::EnumBitfieldStruct<u8, Cr24_SPEC>;
            impl Cr24 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr25_SPEC;
            pub type Cr25 = crate::EnumBitfieldStruct<u8, Cr25_SPEC>;
            impl Cr25 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr26_SPEC;
            pub type Cr26 = crate::EnumBitfieldStruct<u8, Cr26_SPEC>;
            impl Cr26 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr27_SPEC;
            pub type Cr27 = crate::EnumBitfieldStruct<u8, Cr27_SPEC>;
            impl Cr27 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr28_SPEC;
            pub type Cr28 = crate::EnumBitfieldStruct<u8, Cr28_SPEC>;
            impl Cr28 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr29_SPEC;
            pub type Cr29 = crate::EnumBitfieldStruct<u8, Cr29_SPEC>;
            impl Cr29 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr30_SPEC;
            pub type Cr30 = crate::EnumBitfieldStruct<u8, Cr30_SPEC>;
            impl Cr30 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cr31_SPEC;
            pub type Cr31 = crate::EnumBitfieldStruct<u8, Cr31_SPEC>;
            impl Cr31 {
                #[doc = "0 No cancellation        pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Cancellation        pending"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxbCi_SPEC;
        impl crate::sealed::RegSpec for TxbCi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Buffer Configuration 0\n resetvalue={Application Reset:0x0}"]
        pub type TxbCi = crate::RegValueT<TxbCi_SPEC>;

        impl TxbCi {
            #[doc = "Tx Buffers Start Address   TBSA. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Start address of Tx Buffers section in Message RAM  32 bit word address ."]
            #[inline(always)]
            pub fn tbsa(
                self,
            ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, TxbCi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<2,0x3fff,1,0,u16, TxbCi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Number of Dedicated Transmit Buffers   NDTB. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Be aware that the sum of TFQS and NDTB may be not greater than 32.          There is no check for erroneous configurations. The Tx Buffers section          in the Message RAM starts with the dedicated Tx Buffers."]
            #[inline(always)]
            pub fn ndtb(
                self,
            ) -> crate::common::RegisterField<
                16,
                0x3f,
                1,
                0,
                txbci::Ndtb,
                TxbCi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    16,
                    0x3f,
                    1,
                    0,
                    txbci::Ndtb,
                    TxbCi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmit FIFO Queue Size   TFQS. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn tfqs(
                self,
            ) -> crate::common::RegisterField<
                24,
                0x3f,
                1,
                0,
                txbci::Tfqs,
                TxbCi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    24,
                    0x3f,
                    1,
                    0,
                    txbci::Tfqs,
                    TxbCi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Tx FIFO Queue Mode   TFQM. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set."]
            #[inline(always)]
            pub fn tfqm(
                self,
            ) -> crate::common::RegisterField<
                30,
                0x1,
                1,
                0,
                txbci::Tfqm,
                TxbCi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    30,
                    0x1,
                    1,
                    0,
                    txbci::Tfqm,
                    TxbCi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TxbCi {
            #[inline(always)]
            fn default() -> TxbCi {
                <crate::RegValueT<TxbCi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod txbci {
            pub struct Ndtb_SPEC;
            pub type Ndtb = crate::EnumBitfieldStruct<u8, Ndtb_SPEC>;
            impl Ndtb {
                #[doc = "No Dedicated Tx Buffers"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "32 Dedicated Tx Buffers"]
                pub const REST_63: Self = Self::new(63);
            }
            pub struct Tfqs_SPEC;
            pub type Tfqs = crate::EnumBitfieldStruct<u8, Tfqs_SPEC>;
            impl Tfqs {
                #[doc = "No Tx FIFO Queue"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "32 Tx Buffers used for Tx FIFO Queue"]
                pub const REST_63: Self = Self::new(63);
            }
            pub struct Tfqm_SPEC;
            pub type Tfqm = crate::EnumBitfieldStruct<u8, Tfqm_SPEC>;
            impl Tfqm {
                #[doc = "0 Tx FIFO operation"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Tx Queue operation"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxbrPi_SPEC;
        impl crate::sealed::RegSpec for TxbrPi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Buffer Request Pending 0\n resetvalue={Application Reset:0x0}"]
        pub type TxbrPi = crate::RegValueT<TxbrPi_SPEC>;

        impl TxbrPi {
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp0(
                self,
            ) -> crate::common::RegisterField<
                0,
                0x1,
                1,
                0,
                txbrpi::Trp0,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    0,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp0,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp1(
                self,
            ) -> crate::common::RegisterField<
                1,
                0x1,
                1,
                0,
                txbrpi::Trp1,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    1,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp1,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp2(
                self,
            ) -> crate::common::RegisterField<
                2,
                0x1,
                1,
                0,
                txbrpi::Trp2,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    2,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp2,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp3(
                self,
            ) -> crate::common::RegisterField<
                3,
                0x1,
                1,
                0,
                txbrpi::Trp3,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    3,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp3,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp4(
                self,
            ) -> crate::common::RegisterField<
                4,
                0x1,
                1,
                0,
                txbrpi::Trp4,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    4,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp4,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp5(
                self,
            ) -> crate::common::RegisterField<
                5,
                0x1,
                1,
                0,
                txbrpi::Trp5,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    5,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp5,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp6(
                self,
            ) -> crate::common::RegisterField<
                6,
                0x1,
                1,
                0,
                txbrpi::Trp6,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    6,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp6,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp7(
                self,
            ) -> crate::common::RegisterField<
                7,
                0x1,
                1,
                0,
                txbrpi::Trp7,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    7,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp7,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp8(
                self,
            ) -> crate::common::RegisterField<
                8,
                0x1,
                1,
                0,
                txbrpi::Trp8,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    8,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp8,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp9(
                self,
            ) -> crate::common::RegisterField<
                9,
                0x1,
                1,
                0,
                txbrpi::Trp9,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    9,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp9,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp10(
                self,
            ) -> crate::common::RegisterField<
                10,
                0x1,
                1,
                0,
                txbrpi::Trp10,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    10,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp10,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp11(
                self,
            ) -> crate::common::RegisterField<
                11,
                0x1,
                1,
                0,
                txbrpi::Trp11,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    11,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp11,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp12(
                self,
            ) -> crate::common::RegisterField<
                12,
                0x1,
                1,
                0,
                txbrpi::Trp12,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    12,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp12,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp13(
                self,
            ) -> crate::common::RegisterField<
                13,
                0x1,
                1,
                0,
                txbrpi::Trp13,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    13,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp13,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp14(
                self,
            ) -> crate::common::RegisterField<
                14,
                0x1,
                1,
                0,
                txbrpi::Trp14,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    14,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp14,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp15(
                self,
            ) -> crate::common::RegisterField<
                15,
                0x1,
                1,
                0,
                txbrpi::Trp15,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    15,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp15,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp16(
                self,
            ) -> crate::common::RegisterField<
                16,
                0x1,
                1,
                0,
                txbrpi::Trp16,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    16,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp16,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp17(
                self,
            ) -> crate::common::RegisterField<
                17,
                0x1,
                1,
                0,
                txbrpi::Trp17,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    17,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp17,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp18(
                self,
            ) -> crate::common::RegisterField<
                18,
                0x1,
                1,
                0,
                txbrpi::Trp18,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    18,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp18,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp19(
                self,
            ) -> crate::common::RegisterField<
                19,
                0x1,
                1,
                0,
                txbrpi::Trp19,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    19,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp19,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp20(
                self,
            ) -> crate::common::RegisterField<
                20,
                0x1,
                1,
                0,
                txbrpi::Trp20,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    20,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp20,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp21(
                self,
            ) -> crate::common::RegisterField<
                21,
                0x1,
                1,
                0,
                txbrpi::Trp21,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    21,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp21,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp22(
                self,
            ) -> crate::common::RegisterField<
                22,
                0x1,
                1,
                0,
                txbrpi::Trp22,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    22,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp22,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp23(
                self,
            ) -> crate::common::RegisterField<
                23,
                0x1,
                1,
                0,
                txbrpi::Trp23,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    23,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp23,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp24(
                self,
            ) -> crate::common::RegisterField<
                24,
                0x1,
                1,
                0,
                txbrpi::Trp24,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    24,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp24,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp25(
                self,
            ) -> crate::common::RegisterField<
                25,
                0x1,
                1,
                0,
                txbrpi::Trp25,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    25,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp25,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp26(
                self,
            ) -> crate::common::RegisterField<
                26,
                0x1,
                1,
                0,
                txbrpi::Trp26,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    26,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp26,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp27(
                self,
            ) -> crate::common::RegisterField<
                27,
                0x1,
                1,
                0,
                txbrpi::Trp27,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    27,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp27,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp28(
                self,
            ) -> crate::common::RegisterField<
                28,
                0x1,
                1,
                0,
                txbrpi::Trp28,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    28,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp28,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp29(
                self,
            ) -> crate::common::RegisterField<
                29,
                0x1,
                1,
                0,
                txbrpi::Trp29,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    29,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp29,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp30(
                self,
            ) -> crate::common::RegisterField<
                30,
                0x1,
                1,
                0,
                txbrpi::Trp30,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    30,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp30,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Request Pending Tx  Buffer 31   TRP"]
            #[inline(always)]
            pub fn trp31(
                self,
            ) -> crate::common::RegisterField<
                31,
                0x1,
                1,
                0,
                txbrpi::Trp31,
                TxbrPi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    31,
                    0x1,
                    1,
                    0,
                    txbrpi::Trp31,
                    TxbrPi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TxbrPi {
            #[inline(always)]
            fn default() -> TxbrPi {
                <crate::RegValueT<TxbrPi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod txbrpi {
            pub struct Trp0_SPEC;
            pub type Trp0 = crate::EnumBitfieldStruct<u8, Trp0_SPEC>;
            impl Trp0 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp1_SPEC;
            pub type Trp1 = crate::EnumBitfieldStruct<u8, Trp1_SPEC>;
            impl Trp1 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp2_SPEC;
            pub type Trp2 = crate::EnumBitfieldStruct<u8, Trp2_SPEC>;
            impl Trp2 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp3_SPEC;
            pub type Trp3 = crate::EnumBitfieldStruct<u8, Trp3_SPEC>;
            impl Trp3 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp4_SPEC;
            pub type Trp4 = crate::EnumBitfieldStruct<u8, Trp4_SPEC>;
            impl Trp4 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp5_SPEC;
            pub type Trp5 = crate::EnumBitfieldStruct<u8, Trp5_SPEC>;
            impl Trp5 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp6_SPEC;
            pub type Trp6 = crate::EnumBitfieldStruct<u8, Trp6_SPEC>;
            impl Trp6 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp7_SPEC;
            pub type Trp7 = crate::EnumBitfieldStruct<u8, Trp7_SPEC>;
            impl Trp7 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp8_SPEC;
            pub type Trp8 = crate::EnumBitfieldStruct<u8, Trp8_SPEC>;
            impl Trp8 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp9_SPEC;
            pub type Trp9 = crate::EnumBitfieldStruct<u8, Trp9_SPEC>;
            impl Trp9 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp10_SPEC;
            pub type Trp10 = crate::EnumBitfieldStruct<u8, Trp10_SPEC>;
            impl Trp10 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp11_SPEC;
            pub type Trp11 = crate::EnumBitfieldStruct<u8, Trp11_SPEC>;
            impl Trp11 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp12_SPEC;
            pub type Trp12 = crate::EnumBitfieldStruct<u8, Trp12_SPEC>;
            impl Trp12 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp13_SPEC;
            pub type Trp13 = crate::EnumBitfieldStruct<u8, Trp13_SPEC>;
            impl Trp13 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp14_SPEC;
            pub type Trp14 = crate::EnumBitfieldStruct<u8, Trp14_SPEC>;
            impl Trp14 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp15_SPEC;
            pub type Trp15 = crate::EnumBitfieldStruct<u8, Trp15_SPEC>;
            impl Trp15 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp16_SPEC;
            pub type Trp16 = crate::EnumBitfieldStruct<u8, Trp16_SPEC>;
            impl Trp16 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp17_SPEC;
            pub type Trp17 = crate::EnumBitfieldStruct<u8, Trp17_SPEC>;
            impl Trp17 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp18_SPEC;
            pub type Trp18 = crate::EnumBitfieldStruct<u8, Trp18_SPEC>;
            impl Trp18 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp19_SPEC;
            pub type Trp19 = crate::EnumBitfieldStruct<u8, Trp19_SPEC>;
            impl Trp19 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp20_SPEC;
            pub type Trp20 = crate::EnumBitfieldStruct<u8, Trp20_SPEC>;
            impl Trp20 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp21_SPEC;
            pub type Trp21 = crate::EnumBitfieldStruct<u8, Trp21_SPEC>;
            impl Trp21 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp22_SPEC;
            pub type Trp22 = crate::EnumBitfieldStruct<u8, Trp22_SPEC>;
            impl Trp22 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp23_SPEC;
            pub type Trp23 = crate::EnumBitfieldStruct<u8, Trp23_SPEC>;
            impl Trp23 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp24_SPEC;
            pub type Trp24 = crate::EnumBitfieldStruct<u8, Trp24_SPEC>;
            impl Trp24 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp25_SPEC;
            pub type Trp25 = crate::EnumBitfieldStruct<u8, Trp25_SPEC>;
            impl Trp25 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp26_SPEC;
            pub type Trp26 = crate::EnumBitfieldStruct<u8, Trp26_SPEC>;
            impl Trp26 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp27_SPEC;
            pub type Trp27 = crate::EnumBitfieldStruct<u8, Trp27_SPEC>;
            impl Trp27 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp28_SPEC;
            pub type Trp28 = crate::EnumBitfieldStruct<u8, Trp28_SPEC>;
            impl Trp28 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp29_SPEC;
            pub type Trp29 = crate::EnumBitfieldStruct<u8, Trp29_SPEC>;
            impl Trp29 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp30_SPEC;
            pub type Trp30 = crate::EnumBitfieldStruct<u8, Trp30_SPEC>;
            impl Trp30 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trp31_SPEC;
            pub type Trp31 = crate::EnumBitfieldStruct<u8, Trp31_SPEC>;
            impl Trp31 {
                #[doc = "0 No transmission        request pending"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        request pending"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxbtiEi_SPEC;
        impl crate::sealed::RegSpec for TxbtiEi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Buffer Transmission Interrupt Enable 0\n resetvalue={Application Reset:0x0}"]
        pub type TxbtiEi = crate::RegValueT<TxbtiEi_SPEC>;

        impl TxbtiEi {
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie0(
                self,
            ) -> crate::common::RegisterField<
                0,
                0x1,
                1,
                0,
                txbtiei::Tie0,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    0,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie0,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie1(
                self,
            ) -> crate::common::RegisterField<
                1,
                0x1,
                1,
                0,
                txbtiei::Tie1,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    1,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie1,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie2(
                self,
            ) -> crate::common::RegisterField<
                2,
                0x1,
                1,
                0,
                txbtiei::Tie2,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    2,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie2,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie3(
                self,
            ) -> crate::common::RegisterField<
                3,
                0x1,
                1,
                0,
                txbtiei::Tie3,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    3,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie3,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie4(
                self,
            ) -> crate::common::RegisterField<
                4,
                0x1,
                1,
                0,
                txbtiei::Tie4,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    4,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie4,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie5(
                self,
            ) -> crate::common::RegisterField<
                5,
                0x1,
                1,
                0,
                txbtiei::Tie5,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    5,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie5,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie6(
                self,
            ) -> crate::common::RegisterField<
                6,
                0x1,
                1,
                0,
                txbtiei::Tie6,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    6,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie6,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie7(
                self,
            ) -> crate::common::RegisterField<
                7,
                0x1,
                1,
                0,
                txbtiei::Tie7,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    7,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie7,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie8(
                self,
            ) -> crate::common::RegisterField<
                8,
                0x1,
                1,
                0,
                txbtiei::Tie8,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    8,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie8,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie9(
                self,
            ) -> crate::common::RegisterField<
                9,
                0x1,
                1,
                0,
                txbtiei::Tie9,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    9,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie9,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie10(
                self,
            ) -> crate::common::RegisterField<
                10,
                0x1,
                1,
                0,
                txbtiei::Tie10,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    10,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie10,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie11(
                self,
            ) -> crate::common::RegisterField<
                11,
                0x1,
                1,
                0,
                txbtiei::Tie11,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    11,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie11,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie12(
                self,
            ) -> crate::common::RegisterField<
                12,
                0x1,
                1,
                0,
                txbtiei::Tie12,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    12,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie12,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie13(
                self,
            ) -> crate::common::RegisterField<
                13,
                0x1,
                1,
                0,
                txbtiei::Tie13,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    13,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie13,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie14(
                self,
            ) -> crate::common::RegisterField<
                14,
                0x1,
                1,
                0,
                txbtiei::Tie14,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    14,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie14,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie15(
                self,
            ) -> crate::common::RegisterField<
                15,
                0x1,
                1,
                0,
                txbtiei::Tie15,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    15,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie15,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie16(
                self,
            ) -> crate::common::RegisterField<
                16,
                0x1,
                1,
                0,
                txbtiei::Tie16,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    16,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie16,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie17(
                self,
            ) -> crate::common::RegisterField<
                17,
                0x1,
                1,
                0,
                txbtiei::Tie17,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    17,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie17,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie18(
                self,
            ) -> crate::common::RegisterField<
                18,
                0x1,
                1,
                0,
                txbtiei::Tie18,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    18,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie18,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie19(
                self,
            ) -> crate::common::RegisterField<
                19,
                0x1,
                1,
                0,
                txbtiei::Tie19,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    19,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie19,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie20(
                self,
            ) -> crate::common::RegisterField<
                20,
                0x1,
                1,
                0,
                txbtiei::Tie20,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    20,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie20,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie21(
                self,
            ) -> crate::common::RegisterField<
                21,
                0x1,
                1,
                0,
                txbtiei::Tie21,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    21,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie21,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie22(
                self,
            ) -> crate::common::RegisterField<
                22,
                0x1,
                1,
                0,
                txbtiei::Tie22,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    22,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie22,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie23(
                self,
            ) -> crate::common::RegisterField<
                23,
                0x1,
                1,
                0,
                txbtiei::Tie23,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    23,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie23,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie24(
                self,
            ) -> crate::common::RegisterField<
                24,
                0x1,
                1,
                0,
                txbtiei::Tie24,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    24,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie24,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie25(
                self,
            ) -> crate::common::RegisterField<
                25,
                0x1,
                1,
                0,
                txbtiei::Tie25,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    25,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie25,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie26(
                self,
            ) -> crate::common::RegisterField<
                26,
                0x1,
                1,
                0,
                txbtiei::Tie26,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    26,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie26,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie27(
                self,
            ) -> crate::common::RegisterField<
                27,
                0x1,
                1,
                0,
                txbtiei::Tie27,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    27,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie27,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie28(
                self,
            ) -> crate::common::RegisterField<
                28,
                0x1,
                1,
                0,
                txbtiei::Tie28,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    28,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie28,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie29(
                self,
            ) -> crate::common::RegisterField<
                29,
                0x1,
                1,
                0,
                txbtiei::Tie29,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    29,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie29,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie30(
                self,
            ) -> crate::common::RegisterField<
                30,
                0x1,
                1,
                0,
                txbtiei::Tie30,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    30,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie30,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Interrupt Enable Tx Buffer 31   TIE"]
            #[inline(always)]
            pub fn tie31(
                self,
            ) -> crate::common::RegisterField<
                31,
                0x1,
                1,
                0,
                txbtiei::Tie31,
                TxbtiEi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    31,
                    0x1,
                    1,
                    0,
                    txbtiei::Tie31,
                    TxbtiEi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TxbtiEi {
            #[inline(always)]
            fn default() -> TxbtiEi {
                <crate::RegValueT<TxbtiEi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod txbtiei {
            pub struct Tie0_SPEC;
            pub type Tie0 = crate::EnumBitfieldStruct<u8, Tie0_SPEC>;
            impl Tie0 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie1_SPEC;
            pub type Tie1 = crate::EnumBitfieldStruct<u8, Tie1_SPEC>;
            impl Tie1 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie2_SPEC;
            pub type Tie2 = crate::EnumBitfieldStruct<u8, Tie2_SPEC>;
            impl Tie2 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie3_SPEC;
            pub type Tie3 = crate::EnumBitfieldStruct<u8, Tie3_SPEC>;
            impl Tie3 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie4_SPEC;
            pub type Tie4 = crate::EnumBitfieldStruct<u8, Tie4_SPEC>;
            impl Tie4 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie5_SPEC;
            pub type Tie5 = crate::EnumBitfieldStruct<u8, Tie5_SPEC>;
            impl Tie5 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie6_SPEC;
            pub type Tie6 = crate::EnumBitfieldStruct<u8, Tie6_SPEC>;
            impl Tie6 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie7_SPEC;
            pub type Tie7 = crate::EnumBitfieldStruct<u8, Tie7_SPEC>;
            impl Tie7 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie8_SPEC;
            pub type Tie8 = crate::EnumBitfieldStruct<u8, Tie8_SPEC>;
            impl Tie8 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie9_SPEC;
            pub type Tie9 = crate::EnumBitfieldStruct<u8, Tie9_SPEC>;
            impl Tie9 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie10_SPEC;
            pub type Tie10 = crate::EnumBitfieldStruct<u8, Tie10_SPEC>;
            impl Tie10 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie11_SPEC;
            pub type Tie11 = crate::EnumBitfieldStruct<u8, Tie11_SPEC>;
            impl Tie11 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie12_SPEC;
            pub type Tie12 = crate::EnumBitfieldStruct<u8, Tie12_SPEC>;
            impl Tie12 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie13_SPEC;
            pub type Tie13 = crate::EnumBitfieldStruct<u8, Tie13_SPEC>;
            impl Tie13 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie14_SPEC;
            pub type Tie14 = crate::EnumBitfieldStruct<u8, Tie14_SPEC>;
            impl Tie14 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie15_SPEC;
            pub type Tie15 = crate::EnumBitfieldStruct<u8, Tie15_SPEC>;
            impl Tie15 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie16_SPEC;
            pub type Tie16 = crate::EnumBitfieldStruct<u8, Tie16_SPEC>;
            impl Tie16 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie17_SPEC;
            pub type Tie17 = crate::EnumBitfieldStruct<u8, Tie17_SPEC>;
            impl Tie17 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie18_SPEC;
            pub type Tie18 = crate::EnumBitfieldStruct<u8, Tie18_SPEC>;
            impl Tie18 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie19_SPEC;
            pub type Tie19 = crate::EnumBitfieldStruct<u8, Tie19_SPEC>;
            impl Tie19 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie20_SPEC;
            pub type Tie20 = crate::EnumBitfieldStruct<u8, Tie20_SPEC>;
            impl Tie20 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie21_SPEC;
            pub type Tie21 = crate::EnumBitfieldStruct<u8, Tie21_SPEC>;
            impl Tie21 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie22_SPEC;
            pub type Tie22 = crate::EnumBitfieldStruct<u8, Tie22_SPEC>;
            impl Tie22 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie23_SPEC;
            pub type Tie23 = crate::EnumBitfieldStruct<u8, Tie23_SPEC>;
            impl Tie23 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie24_SPEC;
            pub type Tie24 = crate::EnumBitfieldStruct<u8, Tie24_SPEC>;
            impl Tie24 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie25_SPEC;
            pub type Tie25 = crate::EnumBitfieldStruct<u8, Tie25_SPEC>;
            impl Tie25 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie26_SPEC;
            pub type Tie26 = crate::EnumBitfieldStruct<u8, Tie26_SPEC>;
            impl Tie26 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie27_SPEC;
            pub type Tie27 = crate::EnumBitfieldStruct<u8, Tie27_SPEC>;
            impl Tie27 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie28_SPEC;
            pub type Tie28 = crate::EnumBitfieldStruct<u8, Tie28_SPEC>;
            impl Tie28 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie29_SPEC;
            pub type Tie29 = crate::EnumBitfieldStruct<u8, Tie29_SPEC>;
            impl Tie29 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie30_SPEC;
            pub type Tie30 = crate::EnumBitfieldStruct<u8, Tie30_SPEC>;
            impl Tie30 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tie31_SPEC;
            pub type Tie31 = crate::EnumBitfieldStruct<u8, Tie31_SPEC>;
            impl Tie31 {
                #[doc = "0 Transmission        interrupt disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        interrupt enable"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxbtOi_SPEC;
        impl crate::sealed::RegSpec for TxbtOi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Buffer Transmission Occurred 0\n resetvalue={Application Reset:0x0}"]
        pub type TxbtOi = crate::RegValueT<TxbtOi_SPEC>;

        impl TxbtOi {
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to0(
                self,
            ) -> crate::common::RegisterField<
                0,
                0x1,
                1,
                0,
                txbtoi::To0,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    0,
                    0x1,
                    1,
                    0,
                    txbtoi::To0,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to1(
                self,
            ) -> crate::common::RegisterField<
                1,
                0x1,
                1,
                0,
                txbtoi::To1,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    1,
                    0x1,
                    1,
                    0,
                    txbtoi::To1,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to2(
                self,
            ) -> crate::common::RegisterField<
                2,
                0x1,
                1,
                0,
                txbtoi::To2,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    2,
                    0x1,
                    1,
                    0,
                    txbtoi::To2,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to3(
                self,
            ) -> crate::common::RegisterField<
                3,
                0x1,
                1,
                0,
                txbtoi::To3,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    3,
                    0x1,
                    1,
                    0,
                    txbtoi::To3,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to4(
                self,
            ) -> crate::common::RegisterField<
                4,
                0x1,
                1,
                0,
                txbtoi::To4,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    4,
                    0x1,
                    1,
                    0,
                    txbtoi::To4,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to5(
                self,
            ) -> crate::common::RegisterField<
                5,
                0x1,
                1,
                0,
                txbtoi::To5,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    5,
                    0x1,
                    1,
                    0,
                    txbtoi::To5,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to6(
                self,
            ) -> crate::common::RegisterField<
                6,
                0x1,
                1,
                0,
                txbtoi::To6,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    6,
                    0x1,
                    1,
                    0,
                    txbtoi::To6,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to7(
                self,
            ) -> crate::common::RegisterField<
                7,
                0x1,
                1,
                0,
                txbtoi::To7,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    7,
                    0x1,
                    1,
                    0,
                    txbtoi::To7,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to8(
                self,
            ) -> crate::common::RegisterField<
                8,
                0x1,
                1,
                0,
                txbtoi::To8,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    8,
                    0x1,
                    1,
                    0,
                    txbtoi::To8,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to9(
                self,
            ) -> crate::common::RegisterField<
                9,
                0x1,
                1,
                0,
                txbtoi::To9,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    9,
                    0x1,
                    1,
                    0,
                    txbtoi::To9,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to10(
                self,
            ) -> crate::common::RegisterField<
                10,
                0x1,
                1,
                0,
                txbtoi::To10,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    10,
                    0x1,
                    1,
                    0,
                    txbtoi::To10,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to11(
                self,
            ) -> crate::common::RegisterField<
                11,
                0x1,
                1,
                0,
                txbtoi::To11,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    11,
                    0x1,
                    1,
                    0,
                    txbtoi::To11,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to12(
                self,
            ) -> crate::common::RegisterField<
                12,
                0x1,
                1,
                0,
                txbtoi::To12,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    12,
                    0x1,
                    1,
                    0,
                    txbtoi::To12,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to13(
                self,
            ) -> crate::common::RegisterField<
                13,
                0x1,
                1,
                0,
                txbtoi::To13,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    13,
                    0x1,
                    1,
                    0,
                    txbtoi::To13,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to14(
                self,
            ) -> crate::common::RegisterField<
                14,
                0x1,
                1,
                0,
                txbtoi::To14,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    14,
                    0x1,
                    1,
                    0,
                    txbtoi::To14,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to15(
                self,
            ) -> crate::common::RegisterField<
                15,
                0x1,
                1,
                0,
                txbtoi::To15,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    15,
                    0x1,
                    1,
                    0,
                    txbtoi::To15,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to16(
                self,
            ) -> crate::common::RegisterField<
                16,
                0x1,
                1,
                0,
                txbtoi::To16,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    16,
                    0x1,
                    1,
                    0,
                    txbtoi::To16,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to17(
                self,
            ) -> crate::common::RegisterField<
                17,
                0x1,
                1,
                0,
                txbtoi::To17,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    17,
                    0x1,
                    1,
                    0,
                    txbtoi::To17,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to18(
                self,
            ) -> crate::common::RegisterField<
                18,
                0x1,
                1,
                0,
                txbtoi::To18,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    18,
                    0x1,
                    1,
                    0,
                    txbtoi::To18,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to19(
                self,
            ) -> crate::common::RegisterField<
                19,
                0x1,
                1,
                0,
                txbtoi::To19,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    19,
                    0x1,
                    1,
                    0,
                    txbtoi::To19,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to20(
                self,
            ) -> crate::common::RegisterField<
                20,
                0x1,
                1,
                0,
                txbtoi::To20,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    20,
                    0x1,
                    1,
                    0,
                    txbtoi::To20,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to21(
                self,
            ) -> crate::common::RegisterField<
                21,
                0x1,
                1,
                0,
                txbtoi::To21,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    21,
                    0x1,
                    1,
                    0,
                    txbtoi::To21,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to22(
                self,
            ) -> crate::common::RegisterField<
                22,
                0x1,
                1,
                0,
                txbtoi::To22,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    22,
                    0x1,
                    1,
                    0,
                    txbtoi::To22,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to23(
                self,
            ) -> crate::common::RegisterField<
                23,
                0x1,
                1,
                0,
                txbtoi::To23,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    23,
                    0x1,
                    1,
                    0,
                    txbtoi::To23,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to24(
                self,
            ) -> crate::common::RegisterField<
                24,
                0x1,
                1,
                0,
                txbtoi::To24,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    24,
                    0x1,
                    1,
                    0,
                    txbtoi::To24,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to25(
                self,
            ) -> crate::common::RegisterField<
                25,
                0x1,
                1,
                0,
                txbtoi::To25,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    25,
                    0x1,
                    1,
                    0,
                    txbtoi::To25,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to26(
                self,
            ) -> crate::common::RegisterField<
                26,
                0x1,
                1,
                0,
                txbtoi::To26,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    26,
                    0x1,
                    1,
                    0,
                    txbtoi::To26,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to27(
                self,
            ) -> crate::common::RegisterField<
                27,
                0x1,
                1,
                0,
                txbtoi::To27,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    27,
                    0x1,
                    1,
                    0,
                    txbtoi::To27,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to28(
                self,
            ) -> crate::common::RegisterField<
                28,
                0x1,
                1,
                0,
                txbtoi::To28,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    28,
                    0x1,
                    1,
                    0,
                    txbtoi::To28,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to29(
                self,
            ) -> crate::common::RegisterField<
                29,
                0x1,
                1,
                0,
                txbtoi::To29,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    29,
                    0x1,
                    1,
                    0,
                    txbtoi::To29,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to30(
                self,
            ) -> crate::common::RegisterField<
                30,
                0x1,
                1,
                0,
                txbtoi::To30,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    30,
                    0x1,
                    1,
                    0,
                    txbtoi::To30,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Transmission Occurred Tx Buffer 31   TO"]
            #[inline(always)]
            pub fn to31(
                self,
            ) -> crate::common::RegisterField<
                31,
                0x1,
                1,
                0,
                txbtoi::To31,
                TxbtOi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    31,
                    0x1,
                    1,
                    0,
                    txbtoi::To31,
                    TxbtOi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TxbtOi {
            #[inline(always)]
            fn default() -> TxbtOi {
                <crate::RegValueT<TxbtOi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod txbtoi {
            pub struct To0_SPEC;
            pub type To0 = crate::EnumBitfieldStruct<u8, To0_SPEC>;
            impl To0 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To1_SPEC;
            pub type To1 = crate::EnumBitfieldStruct<u8, To1_SPEC>;
            impl To1 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To2_SPEC;
            pub type To2 = crate::EnumBitfieldStruct<u8, To2_SPEC>;
            impl To2 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To3_SPEC;
            pub type To3 = crate::EnumBitfieldStruct<u8, To3_SPEC>;
            impl To3 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To4_SPEC;
            pub type To4 = crate::EnumBitfieldStruct<u8, To4_SPEC>;
            impl To4 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To5_SPEC;
            pub type To5 = crate::EnumBitfieldStruct<u8, To5_SPEC>;
            impl To5 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To6_SPEC;
            pub type To6 = crate::EnumBitfieldStruct<u8, To6_SPEC>;
            impl To6 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To7_SPEC;
            pub type To7 = crate::EnumBitfieldStruct<u8, To7_SPEC>;
            impl To7 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To8_SPEC;
            pub type To8 = crate::EnumBitfieldStruct<u8, To8_SPEC>;
            impl To8 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To9_SPEC;
            pub type To9 = crate::EnumBitfieldStruct<u8, To9_SPEC>;
            impl To9 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To10_SPEC;
            pub type To10 = crate::EnumBitfieldStruct<u8, To10_SPEC>;
            impl To10 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To11_SPEC;
            pub type To11 = crate::EnumBitfieldStruct<u8, To11_SPEC>;
            impl To11 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To12_SPEC;
            pub type To12 = crate::EnumBitfieldStruct<u8, To12_SPEC>;
            impl To12 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To13_SPEC;
            pub type To13 = crate::EnumBitfieldStruct<u8, To13_SPEC>;
            impl To13 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To14_SPEC;
            pub type To14 = crate::EnumBitfieldStruct<u8, To14_SPEC>;
            impl To14 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To15_SPEC;
            pub type To15 = crate::EnumBitfieldStruct<u8, To15_SPEC>;
            impl To15 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To16_SPEC;
            pub type To16 = crate::EnumBitfieldStruct<u8, To16_SPEC>;
            impl To16 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To17_SPEC;
            pub type To17 = crate::EnumBitfieldStruct<u8, To17_SPEC>;
            impl To17 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To18_SPEC;
            pub type To18 = crate::EnumBitfieldStruct<u8, To18_SPEC>;
            impl To18 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To19_SPEC;
            pub type To19 = crate::EnumBitfieldStruct<u8, To19_SPEC>;
            impl To19 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To20_SPEC;
            pub type To20 = crate::EnumBitfieldStruct<u8, To20_SPEC>;
            impl To20 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To21_SPEC;
            pub type To21 = crate::EnumBitfieldStruct<u8, To21_SPEC>;
            impl To21 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To22_SPEC;
            pub type To22 = crate::EnumBitfieldStruct<u8, To22_SPEC>;
            impl To22 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To23_SPEC;
            pub type To23 = crate::EnumBitfieldStruct<u8, To23_SPEC>;
            impl To23 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To24_SPEC;
            pub type To24 = crate::EnumBitfieldStruct<u8, To24_SPEC>;
            impl To24 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To25_SPEC;
            pub type To25 = crate::EnumBitfieldStruct<u8, To25_SPEC>;
            impl To25 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To26_SPEC;
            pub type To26 = crate::EnumBitfieldStruct<u8, To26_SPEC>;
            impl To26 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To27_SPEC;
            pub type To27 = crate::EnumBitfieldStruct<u8, To27_SPEC>;
            impl To27 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To28_SPEC;
            pub type To28 = crate::EnumBitfieldStruct<u8, To28_SPEC>;
            impl To28 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To29_SPEC;
            pub type To29 = crate::EnumBitfieldStruct<u8, To29_SPEC>;
            impl To29 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To30_SPEC;
            pub type To30 = crate::EnumBitfieldStruct<u8, To30_SPEC>;
            impl To30 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct To31_SPEC;
            pub type To31 = crate::EnumBitfieldStruct<u8, To31_SPEC>;
            impl To31 {
                #[doc = "0 No transmission        occurred"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Transmission        occurred"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxefAi_SPEC;
        impl crate::sealed::RegSpec for TxefAi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Event FIFO Acknowledge 0\n resetvalue={Application Reset:0x0}"]
        pub type TxefAi = crate::RegValueT<TxefAi_SPEC>;

        impl TxefAi {
            #[doc = "Event FIFO Acknowledge Index   EFAI. After the Host has read an element or a sequence of elements from the Tx        Event FIFO it has to write the index of the last element read from Tx        Event FIFO to EFAI. This will set the Tx Event FIFO Get Index TXEFS.EFGI        to EFAI  160    160 1 and update the FIFO 0 Fill Level TXEFS.EFFL."]
            #[inline(always)]
            pub fn efai(
                self,
            ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, TxefAi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0x1f,1,0,u8, TxefAi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for TxefAi {
            #[inline(always)]
            fn default() -> TxefAi {
                <crate::RegValueT<TxefAi_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxefCi_SPEC;
        impl crate::sealed::RegSpec for TxefCi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Event FIFO Configuration 0\n resetvalue={Application Reset:0x0}"]
        pub type TxefCi = crate::RegValueT<TxefCi_SPEC>;

        impl TxefCi {
            #[doc = "Event FIFO Start Address   EFSA. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. Start address of Tx Event FIFO in Message RAM  32 bit word address ."]
            #[inline(always)]
            pub fn efsa(
                self,
            ) -> crate::common::RegisterField<2, 0x3fff, 1, 0, u16, TxefCi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<2,0x3fff,1,0,u16, TxefCi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Event FIFO Size   EFS. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set. The Tx Event FIFO elements are indexed from 0 to EFS   1"]
            #[inline(always)]
            pub fn efs(
                self,
            ) -> crate::common::RegisterField<
                16,
                0x3f,
                1,
                0,
                txefci::Efs,
                TxefCi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    16,
                    0x3f,
                    1,
                    0,
                    txefci::Efs,
                    TxefCi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Event FIFO Watermark   EFWM. This bitfield is CCE and INIT protected. Writes will only have effect         if both bits are set."]
            #[inline(always)]
            pub fn efwm(
                self,
            ) -> crate::common::RegisterField<
                24,
                0x3f,
                1,
                0,
                txefci::Efwm,
                TxefCi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    24,
                    0x3f,
                    1,
                    0,
                    txefci::Efwm,
                    TxefCi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TxefCi {
            #[inline(always)]
            fn default() -> TxefCi {
                <crate::RegValueT<TxefCi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod txefci {
            pub struct Efs_SPEC;
            pub type Efs = crate::EnumBitfieldStruct<u8, Efs_SPEC>;
            impl Efs {
                #[doc = "Tx Event FIFO disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "32 Tx Event FIFO elements"]
                pub const REST_63: Self = Self::new(63);
            }
            pub struct Efwm_SPEC;
            pub type Efwm = crate::EnumBitfieldStruct<u8, Efwm_SPEC>;
            impl Efwm {
                #[doc = "Watermark interrupt disabled"]
                pub const REST_63: Self = Self::new(63);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxefSi_SPEC;
        impl crate::sealed::RegSpec for TxefSi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Event FIFO Status 0\n resetvalue={Application Reset:0x0}"]
        pub type TxefSi = crate::RegValueT<TxefSi_SPEC>;

        impl TxefSi {
            #[doc = "Event FIFO Fill Level   EFFL. Number of elements stored in Tx Event FIFO  range 0 to 32."]
            #[inline(always)]
            pub fn effl(
                self,
            ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, TxefSi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0x3f,1,0,u8, TxefSi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Event FIFO Get Index   EFGI. Tx Event FIFO read index pointer  range 0 to 31."]
            #[inline(always)]
            pub fn efgi(
                self,
            ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, TxefSi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<8,0x1f,1,0,u8, TxefSi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Event FIFO Put Index   EFPI. Tx Event FIFO write index pointer  range 0 to 31."]
            #[inline(always)]
            pub fn efpi(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, TxefSi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, TxefSi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Event FIFO Full   EFF"]
            #[inline(always)]
            pub fn eff(
                self,
            ) -> crate::common::RegisterField<
                24,
                0x1,
                1,
                0,
                txefsi::Eff,
                TxefSi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    24,
                    0x1,
                    1,
                    0,
                    txefsi::Eff,
                    TxefSi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Tx Event FIFO Element Lost   TEFL. This bit is a copy of interrupt flag IR.TEFL. When IR.TEFL is reset  this bit is also reset."]
            #[inline(always)]
            pub fn tefl(
                self,
            ) -> crate::common::RegisterField<
                25,
                0x1,
                1,
                0,
                txefsi::Tefl,
                TxefSi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    25,
                    0x1,
                    1,
                    0,
                    txefsi::Tefl,
                    TxefSi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TxefSi {
            #[inline(always)]
            fn default() -> TxefSi {
                <crate::RegValueT<TxefSi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod txefsi {
            pub struct Eff_SPEC;
            pub type Eff = crate::EnumBitfieldStruct<u8, Eff_SPEC>;
            impl Eff {
                #[doc = "0 Tx Event FIFO not full"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Tx Event FIFO full"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tefl_SPEC;
            pub type Tefl = crate::EnumBitfieldStruct<u8, Tefl_SPEC>;
            impl Tefl {
                #[doc = "0 No Tx Event FIFO element lost"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Tx Event FIFO element lost  also set after write attempt to Tx Event FIFO of size zero."]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxesCi_SPEC;
        impl crate::sealed::RegSpec for TxesCi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx Buffer Element Size Configuration 0\n resetvalue={Application Reset:0x0}"]
        pub type TxesCi = crate::RegValueT<TxesCi_SPEC>;

        impl TxesCi {
            #[doc = "Tx Buffer Data Field Size   TBDS. This bitfield is CCE and INIT protected. Writes will only have effect  if both bits are set. In case the data length code DLC of a Tx Buffer element is configured to a value higher than the Tx Buffer data field size TXESC.TBDS  the bytes not defined by the Tx Buffer are transmitted as  0xCC   padding bytes ."]
            #[inline(always)]
            pub fn tbds(
                self,
            ) -> crate::common::RegisterField<
                0,
                0x7,
                1,
                0,
                txesci::Tbds,
                TxesCi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    0,
                    0x7,
                    1,
                    0,
                    txesci::Tbds,
                    TxesCi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TxesCi {
            #[inline(always)]
            fn default() -> TxesCi {
                <crate::RegValueT<TxesCi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod txesci {
            pub struct Tbds_SPEC;
            pub type Tbds = crate::EnumBitfieldStruct<u8, Tbds_SPEC>;
            impl Tbds {
                #[doc = "000 8 byte data field"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "001 12 byte data field"]
                pub const CONST_11: Self = Self::new(1);
                #[doc = "010 16 byte data field"]
                pub const CONST_22: Self = Self::new(2);
                #[doc = "011 20 byte data field"]
                pub const CONST_33: Self = Self::new(3);
                #[doc = "100 24 byte data field"]
                pub const CONST_44: Self = Self::new(4);
                #[doc = "101 32 byte data field"]
                pub const CONST_55: Self = Self::new(5);
                #[doc = "110 48 byte data field"]
                pub const CONST_66: Self = Self::new(6);
                #[doc = "111 64 byte data field"]
                pub const CONST_77: Self = Self::new(7);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TxfqSi_SPEC;
        impl crate::sealed::RegSpec for TxfqSi_SPEC {
            type DataType = u32;
        }
        #[doc = "Tx FIFO Queue Status 0\n resetvalue={Application Reset:0x0}"]
        pub type TxfqSi = crate::RegValueT<TxfqSi_SPEC>;

        impl TxfqSi {
            #[doc = "Tx FIFO Free Level   TFFL. Number of consecutive free Tx FIFO elements starting from TFGI  range 0        to 32. Read as zero when Tx Queue operation is configured  TXBC.TFQM            8216 1  8217   In case of mixed configurations where dedicated Tx Buffers are          combined with a Tx FIFO or a Tx Queue  the Put and Get Indices          indicate the number of the Tx Buffer starting with the first dedicated          Tx Buffers. Example  For a configuration of 12 dedicated Tx Buffers          and a Tx FIFO of 20 Buffers a Put Index of 15 points to the fourth          buffer of the Tx FIFO."]
            #[inline(always)]
            pub fn tffl(
                self,
            ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, TxfqSi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0x3f,1,0,u8, TxfqSi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Tx FIFO Get Index   TFGI. Tx FIFO read index pointer  range 0 to 31. Read as zero when Tx Queue   operation is configured  TXBC.TFQM    1  ."]
            #[inline(always)]
            pub fn tfgi(
                self,
            ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, TxfqSi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<8,0x1f,1,0,u8, TxfqSi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Tx FIFO Queue Put Index   TFQPI. Tx FIFO Queue write index pointer  range 0 to 31."]
            #[inline(always)]
            pub fn tfqpi(
                self,
            ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, TxfqSi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<16,0x1f,1,0,u8, TxfqSi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Tx FIFO Queue Full   TFQF"]
            #[inline(always)]
            pub fn tfqf(
                self,
            ) -> crate::common::RegisterField<
                21,
                0x1,
                1,
                0,
                txfqsi::Tfqf,
                TxfqSi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    21,
                    0x1,
                    1,
                    0,
                    txfqsi::Tfqf,
                    TxfqSi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TxfqSi {
            #[inline(always)]
            fn default() -> TxfqSi {
                <crate::RegValueT<TxfqSi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod txfqsi {
            pub struct Tfqf_SPEC;
            pub type Tfqf = crate::EnumBitfieldStruct<u8, Tfqf_SPEC>;
            impl Tfqf {
                #[doc = "0 Tx FIFO Queue not full"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Tx FIFO Queue full"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
    }
}
