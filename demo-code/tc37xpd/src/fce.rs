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
#[doc = r"FCE"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fce(pub(super) *mut u8);
unsafe impl core::marker::Send for Fce {}
unsafe impl core::marker::Sync for Fce {}
impl Fce {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(252usize)) }
    }

    #[doc = "Channels Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn chsts(&self) -> crate::common::Reg<self::Chsts_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0CAC003}"]
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
    #[doc = "IN"]
    #[inline(always)]
    pub fn r#in(self) -> [self::In; 8] {
        unsafe {
            [
                self::In(self.0.add(0x100usize + 0x0usize)),
                self::In(self.0.add(0x100usize + 0x20usize)),
                self::In(self.0.add(0x100usize + 0x40usize)),
                self::In(self.0.add(0x100usize + 0x60usize)),
                self::In(self.0.add(0x100usize + 0x80usize)),
                self::In(self.0.add(0x100usize + 0xa0usize)),
                self::In(self.0.add(0x100usize + 0xc0usize)),
                self::In(self.0.add(0x100usize + 0xe0usize)),
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
pub struct Chsts_SPEC;
impl crate::sealed::RegSpec for Chsts_SPEC {
    type DataType = u32;
}
#[doc = "Channels Status Register\n resetvalue={Application Reset:0x0}"]
pub type Chsts = crate::RegValueT<Chsts_SPEC>;

impl Chsts {
    #[doc = "Channel0 Status   CH0. This bit is the result of an OR operation of the various status bits of channel 0  see STS register ."]
    #[inline(always)]
    pub fn ch0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Chsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Chsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel1 Status   CH1. This bit is the result of an OR operation of the various status bits of channel 1  see STS register ."]
    #[inline(always)]
    pub fn ch1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Chsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Chsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel2 Status   CH2. This bit is the result of an OR operation of the various status bits of channel 2  see STS register ."]
    #[inline(always)]
    pub fn ch2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Chsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Chsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel3 Status   CH3. This bit is the result of an OR operation of the various status bits of channel 3  see STS register ."]
    #[inline(always)]
    pub fn ch3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Chsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Chsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel4 Status   CH4. This bit is the result of an OR operation of the various status bits of channel 4  see STS register ."]
    #[inline(always)]
    pub fn ch4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Chsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Chsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel5 Status   CH5. This bit is the result of an OR operation of the various status bits of channel 5  see STS register ."]
    #[inline(always)]
    pub fn ch5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Chsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Chsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel6 Status   CH6. This bit is the result of an OR operation of the various status bits of channel 6  see STS register ."]
    #[inline(always)]
    pub fn ch6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Chsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Chsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel7 Status   CH7. This bit is the result of an OR operation of the various status bits of channel 7  see STS register ."]
    #[inline(always)]
    pub fn ch7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Chsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Chsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Chsts {
    #[inline(always)]
    fn default() -> Chsts {
        <crate::RegValueT<Chsts_SPEC> as RegisterValue<_>>::new(0)
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
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0CAC003}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MOD REV. This bit field defines the module revision number. The value of a module revision starts with 01H  first revision . The current revision number is 03H."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MOD TYPE. The bit field is set to C0H which defines the module as a 32 bit module."]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MOD NUMBER. This bit field defines a module identification number. The value for the FCE module is 00CA H ."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(13287427)
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

#[doc = "IN"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct In(pub(super) *mut u8);
unsafe impl core::marker::Send for In {}
unsafe impl core::marker::Sync for In {}
impl In {
    #[doc = "CRC Configuration Register 0\n resetvalue={Application Reset:0x700}"]
    #[inline(always)]
    pub const fn cfgi(&self) -> crate::common::Reg<r#in::CfGi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "CRC Check Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn checki(&self) -> crate::common::Reg<r#in::ChecKi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "CRC Regsister 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn crci(&self) -> crate::common::Reg<r#in::CrCi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "CRC Test Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ctri(&self) -> crate::common::Reg<r#in::CtRi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Input Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn iri(&self) -> crate::common::Reg<r#in::IRi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "CRC Length Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn lengthi(&self) -> crate::common::Reg<r#in::LengtHi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "CRC Result Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn resi(&self) -> crate::common::Reg<r#in::ReSi_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "CRC Status Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn stsi(&self) -> crate::common::Reg<r#in::StSi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
}
pub mod r#in {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CfGi_SPEC;
    impl crate::sealed::RegSpec for CfGi_SPEC {
        type DataType = u32;
    }
    #[doc = "CRC Configuration Register 0\n resetvalue={Application Reset:0x700}"]
    pub type CfGi = crate::RegValueT<CfGi_SPEC>;

    impl CfGi {
        #[doc = "CRC Mismatch Interrupt   CMI"]
        #[inline(always)]
        pub fn cmi(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, cfgi::Cmi, CfGi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1,1,0,cfgi::Cmi, CfGi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Configuration Error Interrupt   CEI. When enabled  a Configuration Error Interrupt is generated whenever a mismatch is detected in the CFG and CHECK redundant registers."]
        #[inline(always)]
        pub fn cei(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, cfgi::Cei, CfGi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0x1,1,0,cfgi::Cei, CfGi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Length Error Interrupt   LEI. When enabled  a Length Error Interrupt is generated if software writes to IR register with LENGTH equal to 0 and CFG.CCE is set to 1."]
        #[inline(always)]
        pub fn lei(
            self,
        ) -> crate::common::RegisterField<2, 0x1, 1, 0, cfgi::Lei, CfGi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x1,1,0,cfgi::Lei, CfGi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Bus Error Interrupt   BEI. When enabled  an interrupt  BEF  is generated if a bus write transaction        with an access width smaller than the kernel width is issued to the        input register. In this case  the corresponding value written to the IR        is discarded and no CRC computation takes place."]
        #[inline(always)]
        pub fn bei(
            self,
        ) -> crate::common::RegisterField<3, 0x1, 1, 0, cfgi::Bei, CfGi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<3,0x1,1,0,cfgi::Bei, CfGi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "CRC Check Comparison   CCE"]
        #[inline(always)]
        pub fn cce(
            self,
        ) -> crate::common::RegisterField<4, 0x1, 1, 0, cfgi::Cce, CfGi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x1,1,0,cfgi::Cce, CfGi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Automatic Length Reload   ALR"]
        #[inline(always)]
        pub fn alr(
            self,
        ) -> crate::common::RegisterField<5, 0x1, 1, 0, cfgi::Alr, CfGi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x1,1,0,cfgi::Alr, CfGi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "IR Byte Wise Reflection   REFIN"]
        #[inline(always)]
        pub fn refin(
            self,
        ) -> crate::common::RegisterField<8, 0x1, 1, 0, cfgi::Refin, CfGi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x1,1,0,cfgi::Refin, CfGi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "CRC Bit Wise Reflection   REFOUT. The alignment of the reflection is the same as the kernel polynomial        width. Eg. 32 bit kernel  Bitwise reflection by 32 bits."]
        #[inline(always)]
        pub fn refout(
            self,
        ) -> crate::common::RegisterField<9, 0x1, 1, 0, cfgi::Refout, CfGi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<9,0x1,1,0,cfgi::Refout, CfGi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Selects the value to be xored with the final CRC   XSEL"]
        #[inline(always)]
        pub fn xsel(
            self,
        ) -> crate::common::RegisterField<10, 0x1, 1, 0, cfgi::Xsel, CfGi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<10,0x1,1,0,cfgi::Xsel, CfGi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Swaps the order of the bytes in the IR input register.   BYTESWAP"]
        #[inline(always)]
        pub fn byteswap(
            self,
        ) -> crate::common::RegisterField<11, 0x1, 1, 0, cfgi::Byteswap, CfGi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                cfgi::Byteswap,
                CfGi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for CfGi {
        #[inline(always)]
        fn default() -> CfGi {
            <crate::RegValueT<CfGi_SPEC> as RegisterValue<_>>::new(1792)
        }
    }
    pub mod cfgi {
        pub struct Cmi_SPEC;
        pub type Cmi = crate::EnumBitfieldStruct<u8, Cmi_SPEC>;
        impl Cmi {
            #[doc = "0 CRC Mismatch Interrupt is disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 CRC Mismatch Interrupt is enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cei_SPEC;
        pub type Cei = crate::EnumBitfieldStruct<u8, Cei_SPEC>;
        impl Cei {
            #[doc = "0 Configuration Error Interrupt is disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Configuration Error Interrupt is enablede"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Lei_SPEC;
        pub type Lei = crate::EnumBitfieldStruct<u8, Lei_SPEC>;
        impl Lei {
            #[doc = "0 Length Error Interrupt is disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Length Error Interrupt is enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Bei_SPEC;
        pub type Bei = crate::EnumBitfieldStruct<u8, Bei_SPEC>;
        impl Bei {
            #[doc = "0 Bus Error Interrupt is disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Bus Error Interrupt is enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cce_SPEC;
        pub type Cce = crate::EnumBitfieldStruct<u8, Cce_SPEC>;
        impl Cce {
            #[doc = "0 CRC check comparison at the end of a message is disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 CRC check comparison at the end of a message is enabled. In this case  if length error is set  STS.LEF   1  and IR is written to  then length is set to all ones  independent of the previous length value. The ALR bit is also ignored in such a condition."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Alr_SPEC;
        pub type Alr = crate::EnumBitfieldStruct<u8, Alr_SPEC>;
        impl Alr {
            #[doc = "0 Disables automatic reload of the LENGTH field."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Enables automatic reload of the LENGTH field at the end of a message."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Refin_SPEC;
        pub type Refin = crate::EnumBitfieldStruct<u8, Refin_SPEC>;
        impl Refin {
            #[doc = "0 IR Byte Wise Reflection is disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 IR Byte Wise Reflection is enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Refout_SPEC;
        pub type Refout = crate::EnumBitfieldStruct<u8, Refout_SPEC>;
        impl Refout {
            #[doc = "0 CRC bit wise is        disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 CRC bit wise is        enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Xsel_SPEC;
        pub type Xsel = crate::EnumBitfieldStruct<u8, Xsel_SPEC>;
        impl Xsel {
            #[doc = "0 0x00000000"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 0xFFFFFFFF"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Byteswap_SPEC;
        pub type Byteswap = crate::EnumBitfieldStruct<u8, Byteswap_SPEC>;
        impl Byteswap {
            #[doc = "0 The order of bytes in IR register are not swapped before CRC computation."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 The order of bytes in the IR register are swapped before CRC computation. Big endian input is converted to Little endian and vice versa.  When 8 bit CRC is chosen  this has no effect ."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChecKi_SPEC;
    impl crate::sealed::RegSpec for ChecKi_SPEC {
        type DataType = u32;
    }
    #[doc = "CRC Check Register 0\n resetvalue={Application Reset:0x0}"]
    pub type ChecKi = crate::RegValueT<ChecKi_SPEC>;

    impl ChecKi {
        #[doc = "CHECK Register   CHECK. Expected CRC value to be checked by the hardware upon detection of a 1 to 0 transition of the LENGTH register. The comparison is enabled by the CFG.CCE bit field"]
        #[inline(always)]
        pub fn check(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, ChecKi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, ChecKi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for ChecKi {
        #[inline(always)]
        fn default() -> ChecKi {
            <crate::RegValueT<ChecKi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CrCi_SPEC;
    impl crate::sealed::RegSpec for CrCi_SPEC {
        type DataType = u32;
    }
    #[doc = "CRC Regsister 0\n resetvalue={Application Reset:0x0}"]
    pub type CrCi = crate::RegValueT<CrCi_SPEC>;

    impl CrCi {
        #[doc = "CRC Register   CRC. This register enables to directly access the internal CRC register"]
        #[inline(always)]
        pub fn crc(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, CrCi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, CrCi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for CrCi {
        #[inline(always)]
        fn default() -> CrCi {
            <crate::RegValueT<CrCi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtRi_SPEC;
    impl crate::sealed::RegSpec for CtRi_SPEC {
        type DataType = u32;
    }
    #[doc = "CRC Test Register 0\n resetvalue={Application Reset:0x0}"]
    pub type CtRi = crate::RegValueT<CtRi_SPEC>;

    impl CtRi {
        #[doc = "Force CRC Mismatch   FCM. Forces the CRC compare logic to issue an error regardless of the CHECK and CRC values. The hardware detects a 0 to 1 transition of this bit field and triggers a CRC Mismatch interrupt"]
        #[inline(always)]
        pub fn fcm(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, CtRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0, 1, 0, CtRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Force CFG Register Mismatch   FRM CFG. This field is used to control the error injection mechanism used to check the compare logic of the redundant CFG registers. This is a one shot operation. When the hardware detects a 0 to 1 transition of this bit field it triggers a Configuration Mismatch interrupt  if enabled by the corresponding CFGm register ."]
        #[inline(always)]
        pub fn frm_cfg(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, CtRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1, 1, 0, CtRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Force Check Register Mismatch   FRM CHECK. This field is used to control the error injection mechanism used to check the compare logic of the redundant CHECK registers. This is a one shot operation. The hardware detects a 0 to 1 transition of this bit field and triggers a Check Register Mismatch interrupt  if enabled by the corresponding CFGm register ."]
        #[inline(always)]
        pub fn frm_check(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, CtRi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2, 1, 0, CtRi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
    }
    impl core::default::Default for CtRi {
        #[inline(always)]
        fn default() -> CtRi {
            <crate::RegValueT<CtRi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IRi_SPEC;
    impl crate::sealed::RegSpec for IRi_SPEC {
        type DataType = u32;
    }
    #[doc = "Input Register 0\n resetvalue={Application Reset:0x0}"]
    pub type IRi = crate::RegValueT<IRi_SPEC>;

    impl IRi {
        #[doc = "Input Register   IR. This bit field holds the input data to be computed. In case the channel        is configured to use 16 bit or 8 bit CRC  only the LSB 16 or 8 bits will        be used as input."]
        #[inline(always)]
        pub fn ir(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, IRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, IRi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for IRi {
        #[inline(always)]
        fn default() -> IRi {
            <crate::RegValueT<IRi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LengtHi_SPEC;
    impl crate::sealed::RegSpec for LengtHi_SPEC {
        type DataType = u32;
    }
    #[doc = "CRC Length Register 0\n resetvalue={Application Reset:0x0}"]
    pub type LengtHi = crate::RegValueT<LengtHi_SPEC>;

    impl LengtHi {
        #[doc = "Message Length Register   LENGTH. Number of words  bit width of each word in terms of KERNEL polynomial width  building the message over which the CRC checksum is calculated. This bit field is modified by the hardware  every write to the IR register decrements the value of the LENGTH bit field. If the CFG.ALR field is set to 1  the LENGTH field shall be reloaded with its configuration value at the end of the cycle where LENGTH reaches 0."]
        #[inline(always)]
        pub fn length(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, LengtHi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, LengtHi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for LengtHi {
        #[inline(always)]
        fn default() -> LengtHi {
            <crate::RegValueT<LengtHi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ReSi_SPEC;
    impl crate::sealed::RegSpec for ReSi_SPEC {
        type DataType = u32;
    }
    #[doc = "CRC Result Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type ReSi = crate::RegValueT<ReSi_SPEC>;

    impl ReSi {
        #[doc = "Result Register   RES. Returns the final CRC value including CRC reflection and final XOR according to the CFG register configuration. Writing to this register produces a bus error. If the channel is configured to use 16 bit or 8 bit CRC  the MSB 16 or 24 bits respectively shall be read as 0."]
        #[inline(always)]
        pub fn res(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, ReSi_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, ReSi_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for ReSi {
        #[inline(always)]
        fn default() -> ReSi {
            <crate::RegValueT<ReSi_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct StSi_SPEC;
    impl crate::sealed::RegSpec for StSi_SPEC {
        type DataType = u32;
    }
    #[doc = "CRC Status Register 0\n resetvalue={Application Reset:0x0}"]
    pub type StSi = crate::RegValueT<StSi_SPEC>;

    impl StSi {
        #[doc = "CRC Mismatch Flag   CMF. This bit is set per hardware only. To clear this bit  software must write a 0 to this bit field location. Writing 1 to this bit has no effect."]
        #[inline(always)]
        pub fn cmf(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, StSi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0, 1, 0, StSi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Configuration Error Flag   CEF. This bit is set per hardware only. To clear this bit  software must write a 0 to this bit field location. Writing a 1 has no effect."]
        #[inline(always)]
        pub fn cef(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, StSi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1, 1, 0, StSi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Length Error Flag   LEF. This bit is set per hardware only. To clear this bit  software must write a 0 to this bit field location. Writing 1 has no effect."]
        #[inline(always)]
        pub fn lef(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, StSi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2, 1, 0, StSi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Bus Error Flag   BEF. This bit is set per hardware only. To clear this bit  software must write a 0 to this bit field location. Writing 1 has no effect."]
        #[inline(always)]
        pub fn bef(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, StSi_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3, 1, 0, StSi_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
    }
    impl core::default::Default for StSi {
        #[inline(always)]
        fn default() -> StSi {
            <crate::RegValueT<StSi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
