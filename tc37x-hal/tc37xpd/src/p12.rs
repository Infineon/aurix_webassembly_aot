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
#[doc = r"General Purpose I O Ports and Peripheral I O Lines"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P12(pub(super) *mut u8);
unsafe impl core::marker::Send for P12 {}
unsafe impl core::marker::Sync for P12 {}
impl P12 {
    #[doc = "Port 12 Access Enable Register 0\n resetvalue={:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(252usize)) }
    }

    #[doc = "Port 12 Emergency Stop Register\n resetvalue={:0x0}"]
    #[inline(always)]
    pub const fn esr(&self) -> crate::common::Reg<self::Esr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }

    #[doc = "Port 12 Identification Register\n resetvalue={:0x0C8C000}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Port 12 Input Register\n resetvalue={:0x0}"]
    #[inline(always)]
    pub const fn r#in(&self) -> crate::common::Reg<self::In_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "Port 12 Input Output Control Register 0\n resetvalue={:0x0,:0x0,:0x10101010,:0x10101010}"]
    #[inline(always)]
    pub const fn iocr0(&self) -> crate::common::Reg<self::Iocr0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "Port 12 Output Modification Clear Register\n resetvalue={:0x0}"]
    #[inline(always)]
    pub const fn omcr(&self) -> crate::common::Reg<self::Omcr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(148usize)) }
    }

    #[doc = "Port 12 Output Modification Clear Register 0\n resetvalue={:0x0}"]
    #[inline(always)]
    pub const fn omcr0(&self) -> crate::common::Reg<self::Omcr0_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "Port 12 Output Modification Register\n resetvalue={:0x0}"]
    #[inline(always)]
    pub const fn omr(&self) -> crate::common::Reg<self::Omr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }

    #[doc = "Port 12 Output Modification Set Register\n resetvalue={:0x0}"]
    #[inline(always)]
    pub const fn omsr(&self) -> crate::common::Reg<self::Omsr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }

    #[doc = "Port 12 Output Modification Set Register 0\n resetvalue={:0x0}"]
    #[inline(always)]
    pub const fn omsr0(&self) -> crate::common::Reg<self::Omsr0_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(112usize)) }
    }

    #[doc = "Port 12 Output Register\n resetvalue={:0x0}"]
    #[inline(always)]
    pub const fn out(&self) -> crate::common::Reg<self::Out_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Port 12 Pin Function Decision Control Register\n resetvalue={:0x0,:0x0,After SSW execution:0x0,After SSW execution:0x0}"]
    #[inline(always)]
    pub const fn pdisc(&self) -> crate::common::Reg<self::Pdisc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }

    #[doc = "Port 12 Pad Driver Mode Register 0\n resetvalue={:0x0,:0x0,After SSW execution:0x2222,After SSW execution:0x2222}"]
    #[inline(always)]
    pub const fn pdr0(&self) -> crate::common::Reg<self::Pdr0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accen0_SPEC;
impl crate::sealed::RegSpec for Accen0_SPEC {
    type DataType = u32;
}
#[doc = "Port 12 Access Enable Register 0\n resetvalue={:0x0FFFFFFFF}"]
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
pub struct Esr_SPEC;
impl crate::sealed::RegSpec for Esr_SPEC {
    type DataType = u32;
}
#[doc = "Port 12 Emergency Stop Register\n resetvalue={:0x0}"]
pub type Esr = crate::RegValueT<Esr_SPEC>;

impl Esr {
    #[doc = "Emergency Stop Enable for Pin 15. This bit enables the emergency stop function for all GPIO lines. If the        emergency stop condition is met and enabled  the output selection is        automatically switched from alternate output function to GPIO input        function."]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, esr::En0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,esr::En0, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Pin 15. This bit enables the emergency stop function for all GPIO lines. If the        emergency stop condition is met and enabled  the output selection is        automatically switched from alternate output function to GPIO input        function."]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, esr::En1, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,esr::En1, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Pin 15. This bit enables the emergency stop function for all GPIO lines. If the        emergency stop condition is met and enabled  the output selection is        automatically switched from alternate output function to GPIO input        function."]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, esr::En2, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,esr::En2, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Pin 15. This bit enables the emergency stop function for all GPIO lines. If the        emergency stop condition is met and enabled  the output selection is        automatically switched from alternate output function to GPIO input        function."]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, esr::En3, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,esr::En3, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Esr {
    #[inline(always)]
    fn default() -> Esr {
        <crate::RegValueT<Esr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod esr {
    pub struct En0_SPEC;
    pub type En0 = crate::EnumBitfieldStruct<u8, En0_SPEC>;
    impl En0 {
        #[doc = "0 Emergency stop        function for Pn.x is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Emergency stop function for Pn.x is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En1_SPEC;
    pub type En1 = crate::EnumBitfieldStruct<u8, En1_SPEC>;
    impl En1 {
        #[doc = "0 Emergency stop        function for Pn.x is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Emergency stop function for Pn.x is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En2_SPEC;
    pub type En2 = crate::EnumBitfieldStruct<u8, En2_SPEC>;
    impl En2 {
        #[doc = "0 Emergency stop        function for Pn.x is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Emergency stop function for Pn.x is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En3_SPEC;
    pub type En3 = crate::EnumBitfieldStruct<u8, En3_SPEC>;
    impl En3 {
        #[doc = "0 Emergency stop        function for Pn.x is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Emergency stop function for Pn.x is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Port 12 Identification Register\n resetvalue={:0x0C8C000}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number. This bit field indicates the revision number of the TC39x module  01 H   160    160 first revision ."]
    #[inline(always)]
    pub fn modrev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type. This bit field is C0 H . It defines a        32 bit module"]
    #[inline(always)]
    pub fn modtype(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number. This bit field defines the module identification number. The value for        the Ports module is 00C8"]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(13156352)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct In_SPEC;
impl crate::sealed::RegSpec for In_SPEC {
    type DataType = u32;
}
#[doc = "Port 12 Input Register\n resetvalue={:0x0}"]
pub type In = crate::RegValueT<In_SPEC>;

impl In {
    #[doc = "Input Bit 15. This bit indicates the level at the input pin Pn.x."]
    #[inline(always)]
    pub fn p0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, r#in::P0, In_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,r#in::P0, In_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Input Bit 15. This bit indicates the level at the input pin Pn.x."]
    #[inline(always)]
    pub fn p1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, r#in::P1, In_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,r#in::P1, In_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Input Bit 15. This bit indicates the level at the input pin Pn.x."]
    #[inline(always)]
    pub fn p2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, r#in::P2, In_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,r#in::P2, In_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Input Bit 15. This bit indicates the level at the input pin Pn.x."]
    #[inline(always)]
    pub fn p3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, r#in::P3, In_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1,1,0,r#in::P3, In_SPEC,crate::common::R>::from_register(self,0)
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
        #[doc = "0 The input level of Pn.x is 0."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The input level of Pn.x is 1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct P1_SPEC;
    pub type P1 = crate::EnumBitfieldStruct<u8, P1_SPEC>;
    impl P1 {
        #[doc = "0 The input level of Pn.x is 0."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The input level of Pn.x is 1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct P2_SPEC;
    pub type P2 = crate::EnumBitfieldStruct<u8, P2_SPEC>;
    impl P2 {
        #[doc = "0 The input level of Pn.x is 0."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The input level of Pn.x is 1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct P3_SPEC;
    pub type P3 = crate::EnumBitfieldStruct<u8, P3_SPEC>;
    impl P3 {
        #[doc = "0 The input level of Pn.x is 0."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The input level of Pn.x is 1."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iocr0_SPEC;
impl crate::sealed::RegSpec for Iocr0_SPEC {
    type DataType = u32;
}
#[doc = "Port 12 Input Output Control Register 0\n resetvalue={:0x0,:0x0,:0x10101010,:0x10101010}"]
pub type Iocr0 = crate::RegValueT<Iocr0_SPEC>;

impl Iocr0 {
    #[doc = "Port Control for Pin 3. This bit field defines the Port n line x functionality according to Table          only input selection apply."]
    #[inline(always)]
    pub fn pc0(
        self,
    ) -> crate::common::RegisterField<3, 0x1f, 1, 0, u8, Iocr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1f,1,0,u8, Iocr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Control for Pin 3. This bit field defines the Port n line x functionality according to Table          only input selection apply."]
    #[inline(always)]
    pub fn pc1(
        self,
    ) -> crate::common::RegisterField<11, 0x1f, 1, 0, u8, Iocr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x1f,1,0,u8, Iocr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Control for Pin 3. This bit field defines the Port n line x functionality according to Table          only input selection apply."]
    #[inline(always)]
    pub fn pc2(
        self,
    ) -> crate::common::RegisterField<19, 0x1f, 1, 0, u8, Iocr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<19,0x1f,1,0,u8, Iocr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Port Control for Pin 3. This bit field defines the Port n line x functionality according to Table          only input selection apply."]
    #[inline(always)]
    pub fn pc3(
        self,
    ) -> crate::common::RegisterField<27, 0x1f, 1, 0, u8, Iocr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1f,1,0,u8, Iocr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Iocr0 {
    #[inline(always)]
    fn default() -> Iocr0 {
        <crate::RegValueT<Iocr0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Omcr_SPEC;
impl crate::sealed::RegSpec for Omcr_SPEC {
    type DataType = u32;
}
#[doc = "Port 12 Output Modification Clear Register\n resetvalue={:0x0}"]
pub type Omcr = crate::RegValueT<Omcr_SPEC>;

impl Omcr {
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl0(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, omcr::Pcl0, Omcr_SPEC, crate::common::W> {
        crate::common::RegisterField::<16,0x1,1,0,omcr::Pcl0, Omcr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl1(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, omcr::Pcl1, Omcr_SPEC, crate::common::W> {
        crate::common::RegisterField::<17,0x1,1,0,omcr::Pcl1, Omcr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl2(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, omcr::Pcl2, Omcr_SPEC, crate::common::W> {
        crate::common::RegisterField::<18,0x1,1,0,omcr::Pcl2, Omcr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear Bit 15. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl3(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, omcr::Pcl3, Omcr_SPEC, crate::common::W> {
        crate::common::RegisterField::<19,0x1,1,0,omcr::Pcl3, Omcr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Omcr {
    #[inline(always)]
    fn default() -> Omcr {
        <crate::RegValueT<Omcr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod omcr {
    pub struct Pcl0_SPEC;
    pub type Pcl0 = crate::EnumBitfieldStruct<u8, Pcl0_SPEC>;
    impl Pcl0 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears Pn OUT.Px."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pcl1_SPEC;
    pub type Pcl1 = crate::EnumBitfieldStruct<u8, Pcl1_SPEC>;
    impl Pcl1 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears Pn OUT.Px."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pcl2_SPEC;
    pub type Pcl2 = crate::EnumBitfieldStruct<u8, Pcl2_SPEC>;
    impl Pcl2 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears Pn OUT.Px."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pcl3_SPEC;
    pub type Pcl3 = crate::EnumBitfieldStruct<u8, Pcl3_SPEC>;
    impl Pcl3 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears Pn OUT.Px."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Omcr0_SPEC;
impl crate::sealed::RegSpec for Omcr0_SPEC {
    type DataType = u32;
}
#[doc = "Port 12 Output Modification Clear Register 0\n resetvalue={:0x0}"]
pub type Omcr0 = crate::RegValueT<Omcr0_SPEC>;

impl Omcr0 {
    #[doc = "Clear Bit 3. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl0(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, omcr0::Pcl0, Omcr0_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0x1,1,0,omcr0::Pcl0, Omcr0_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear Bit 3. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl1(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, omcr0::Pcl1, Omcr0_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<17,0x1,1,0,omcr0::Pcl1, Omcr0_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear Bit 3. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl2(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, omcr0::Pcl2, Omcr0_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<18,0x1,1,0,omcr0::Pcl2, Omcr0_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear Bit 3. Setting this bit will clear the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn pcl3(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, omcr0::Pcl3, Omcr0_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<19,0x1,1,0,omcr0::Pcl3, Omcr0_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Omcr0 {
    #[inline(always)]
    fn default() -> Omcr0 {
        <crate::RegValueT<Omcr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod omcr0 {
    pub struct Pcl0_SPEC;
    pub type Pcl0 = crate::EnumBitfieldStruct<u8, Pcl0_SPEC>;
    impl Pcl0 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears Pn OUT.Px"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pcl1_SPEC;
    pub type Pcl1 = crate::EnumBitfieldStruct<u8, Pcl1_SPEC>;
    impl Pcl1 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears Pn OUT.Px"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pcl2_SPEC;
    pub type Pcl2 = crate::EnumBitfieldStruct<u8, Pcl2_SPEC>;
    impl Pcl2 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears Pn OUT.Px"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pcl3_SPEC;
    pub type Pcl3 = crate::EnumBitfieldStruct<u8, Pcl3_SPEC>;
    impl Pcl3 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears Pn OUT.Px"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Omr_SPEC;
impl crate::sealed::RegSpec for Omr_SPEC {
    type DataType = u32;
}
#[doc = "Port 12 Output Modification Register\n resetvalue={:0x0}"]
pub type Omr = crate::RegValueT<Omr_SPEC>;

impl Omr {
    #[doc = "Set Bit 15. Setting this bit will set or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn ps0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, omr::Ps0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0x1,1,0,omr::Ps0, Omr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Bit 15. Setting this bit will set or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn ps1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, omr::Ps1, Omr_SPEC, crate::common::W> {
        crate::common::RegisterField::<1,0x1,1,0,omr::Ps1, Omr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Bit 15. Setting this bit will set or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn ps2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, omr::Ps2, Omr_SPEC, crate::common::W> {
        crate::common::RegisterField::<2,0x1,1,0,omr::Ps2, Omr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Bit 15. Setting this bit will set or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn ps3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, omr::Ps3, Omr_SPEC, crate::common::W> {
        crate::common::RegisterField::<3,0x1,1,0,omr::Ps3, Omr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear Bit 15. Setting this bit will clear or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn pcl0(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, omr::Pcl0, Omr_SPEC, crate::common::W> {
        crate::common::RegisterField::<16,0x1,1,0,omr::Pcl0, Omr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear Bit 15. Setting this bit will clear or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn pcl1(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, omr::Pcl1, Omr_SPEC, crate::common::W> {
        crate::common::RegisterField::<17,0x1,1,0,omr::Pcl1, Omr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear Bit 15. Setting this bit will clear or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn pcl2(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, omr::Pcl2, Omr_SPEC, crate::common::W> {
        crate::common::RegisterField::<18,0x1,1,0,omr::Pcl2, Omr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear Bit 15. Setting this bit will clear or toggle the corresponding bit in the port        output register Pn OUT. Read as 0. The function of this bit is shown in Table ."]
    #[inline(always)]
    pub fn pcl3(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, omr::Pcl3, Omr_SPEC, crate::common::W> {
        crate::common::RegisterField::<19,0x1,1,0,omr::Pcl3, Omr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Omr {
    #[inline(always)]
    fn default() -> Omr {
        <crate::RegValueT<Omr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod omr {
    pub struct Ps0_SPEC;
    pub type Ps0 = crate::EnumBitfieldStruct<u8, Ps0_SPEC>;
    impl Ps0 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Sets or toggles Pn OUT.Px."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ps1_SPEC;
    pub type Ps1 = crate::EnumBitfieldStruct<u8, Ps1_SPEC>;
    impl Ps1 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Sets or toggles Pn OUT.Px."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ps2_SPEC;
    pub type Ps2 = crate::EnumBitfieldStruct<u8, Ps2_SPEC>;
    impl Ps2 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Sets or toggles Pn OUT.Px."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ps3_SPEC;
    pub type Ps3 = crate::EnumBitfieldStruct<u8, Ps3_SPEC>;
    impl Ps3 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Sets or toggles Pn OUT.Px."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pcl0_SPEC;
    pub type Pcl0 = crate::EnumBitfieldStruct<u8, Pcl0_SPEC>;
    impl Pcl0 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears or toggles Pn OUT.Px."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pcl1_SPEC;
    pub type Pcl1 = crate::EnumBitfieldStruct<u8, Pcl1_SPEC>;
    impl Pcl1 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears or toggles Pn OUT.Px."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pcl2_SPEC;
    pub type Pcl2 = crate::EnumBitfieldStruct<u8, Pcl2_SPEC>;
    impl Pcl2 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears or toggles Pn OUT.Px."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pcl3_SPEC;
    pub type Pcl3 = crate::EnumBitfieldStruct<u8, Pcl3_SPEC>;
    impl Pcl3 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears or toggles Pn OUT.Px."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Omsr_SPEC;
impl crate::sealed::RegSpec for Omsr_SPEC {
    type DataType = u32;
}
#[doc = "Port 12 Output Modification Set Register\n resetvalue={:0x0}"]
pub type Omsr = crate::RegValueT<Omsr_SPEC>;

impl Omsr {
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, omsr::Ps0, Omsr_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0x1,1,0,omsr::Ps0, Omsr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, omsr::Ps1, Omsr_SPEC, crate::common::W> {
        crate::common::RegisterField::<1,0x1,1,0,omsr::Ps1, Omsr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, omsr::Ps2, Omsr_SPEC, crate::common::W> {
        crate::common::RegisterField::<2,0x1,1,0,omsr::Ps2, Omsr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Bit 15. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, omsr::Ps3, Omsr_SPEC, crate::common::W> {
        crate::common::RegisterField::<3,0x1,1,0,omsr::Ps3, Omsr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Omsr {
    #[inline(always)]
    fn default() -> Omsr {
        <crate::RegValueT<Omsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod omsr {
    pub struct Ps0_SPEC;
    pub type Ps0 = crate::EnumBitfieldStruct<u8, Ps0_SPEC>;
    impl Ps0 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Sets Pn OUT.Px"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ps1_SPEC;
    pub type Ps1 = crate::EnumBitfieldStruct<u8, Ps1_SPEC>;
    impl Ps1 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Sets Pn OUT.Px"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ps2_SPEC;
    pub type Ps2 = crate::EnumBitfieldStruct<u8, Ps2_SPEC>;
    impl Ps2 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Sets Pn OUT.Px"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ps3_SPEC;
    pub type Ps3 = crate::EnumBitfieldStruct<u8, Ps3_SPEC>;
    impl Ps3 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Sets Pn OUT.Px"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Omsr0_SPEC;
impl crate::sealed::RegSpec for Omsr0_SPEC {
    type DataType = u32;
}
#[doc = "Port 12 Output Modification Set Register 0\n resetvalue={:0x0}"]
pub type Omsr0 = crate::RegValueT<Omsr0_SPEC>;

impl Omsr0 {
    #[doc = "Set Bit 3. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, omsr0::Ps0, Omsr0_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0x1,1,0,omsr0::Ps0, Omsr0_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Bit 3. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, omsr0::Ps1, Omsr0_SPEC, crate::common::W> {
        crate::common::RegisterField::<1,0x1,1,0,omsr0::Ps1, Omsr0_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Bit 3. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, omsr0::Ps2, Omsr0_SPEC, crate::common::W> {
        crate::common::RegisterField::<2,0x1,1,0,omsr0::Ps2, Omsr0_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Bit 3. Setting this bit will set the corresponding bit in the port output        register Pn OUT. Read as 0."]
    #[inline(always)]
    pub fn ps3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, omsr0::Ps3, Omsr0_SPEC, crate::common::W> {
        crate::common::RegisterField::<3,0x1,1,0,omsr0::Ps3, Omsr0_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Omsr0 {
    #[inline(always)]
    fn default() -> Omsr0 {
        <crate::RegValueT<Omsr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod omsr0 {
    pub struct Ps0_SPEC;
    pub type Ps0 = crate::EnumBitfieldStruct<u8, Ps0_SPEC>;
    impl Ps0 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Sets Pn OUT.Px"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ps1_SPEC;
    pub type Ps1 = crate::EnumBitfieldStruct<u8, Ps1_SPEC>;
    impl Ps1 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Sets Pn OUT.Px"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ps2_SPEC;
    pub type Ps2 = crate::EnumBitfieldStruct<u8, Ps2_SPEC>;
    impl Ps2 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Sets Pn OUT.Px"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ps3_SPEC;
    pub type Ps3 = crate::EnumBitfieldStruct<u8, Ps3_SPEC>;
    impl Ps3 {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Sets Pn OUT.Px"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Out_SPEC;
impl crate::sealed::RegSpec for Out_SPEC {
    type DataType = u32;
}
#[doc = "Port 12 Output Register\n resetvalue={:0x0}"]
pub type Out = crate::RegValueT<Out_SPEC>;

impl Out {
    #[doc = "Output Bit 15. This bit determines the level at the output pin Pn.x if the output is        selected as GPIO output. Pn.x can also be set or cleared by control bits of the Pn OMSR  Pn OMCR        or Pn OMR registers."]
    #[inline(always)]
    pub fn p0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, out::P0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,out::P0, Out_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Output Bit 15. This bit determines the level at the output pin Pn.x if the output is        selected as GPIO output. Pn.x can also be set or cleared by control bits of the Pn OMSR  Pn OMCR        or Pn OMR registers."]
    #[inline(always)]
    pub fn p1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, out::P1, Out_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,out::P1, Out_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Output Bit 15. This bit determines the level at the output pin Pn.x if the output is        selected as GPIO output. Pn.x can also be set or cleared by control bits of the Pn OMSR  Pn OMCR        or Pn OMR registers."]
    #[inline(always)]
    pub fn p2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, out::P2, Out_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,out::P2, Out_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Output Bit 15. This bit determines the level at the output pin Pn.x if the output is        selected as GPIO output. Pn.x can also be set or cleared by control bits of the Pn OMSR  Pn OMCR        or Pn OMR registers."]
    #[inline(always)]
    pub fn p3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, out::P3, Out_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,out::P3, Out_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "0 The output level of Pn.x is 0."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The output level of Pn.x is 1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct P1_SPEC;
    pub type P1 = crate::EnumBitfieldStruct<u8, P1_SPEC>;
    impl P1 {
        #[doc = "0 The output level of Pn.x is 0."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The output level of Pn.x is 1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct P2_SPEC;
    pub type P2 = crate::EnumBitfieldStruct<u8, P2_SPEC>;
    impl P2 {
        #[doc = "0 The output level of Pn.x is 0."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The output level of Pn.x is 1."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct P3_SPEC;
    pub type P3 = crate::EnumBitfieldStruct<u8, P3_SPEC>;
    impl P3 {
        #[doc = "0 The output level of Pn.x is 0."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The output level of Pn.x is 1."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdisc_SPEC;
impl crate::sealed::RegSpec for Pdisc_SPEC {
    type DataType = u32;
}
#[doc = "Port 12 Pin Function Decision Control Register\n resetvalue={:0x0,:0x0,After SSW execution:0x0,After SSW execution:0x0}"]
pub type Pdisc = crate::RegValueT<Pdisc_SPEC>;

impl Pdisc {
    #[doc = "Pin Function Decision Control for Pin 15. This bit selects the function of the port pad."]
    #[inline(always)]
    pub fn pdis0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, pdisc::Pdis0, Pdisc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,pdisc::Pdis0, Pdisc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pin Function Decision Control for Pin 15. This bit selects the function of the port pad."]
    #[inline(always)]
    pub fn pdis1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, pdisc::Pdis1, Pdisc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,pdisc::Pdis1, Pdisc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pin Function Decision Control for Pin 15. This bit selects the function of the port pad."]
    #[inline(always)]
    pub fn pdis2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, pdisc::Pdis2, Pdisc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,pdisc::Pdis2, Pdisc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pin Function Decision Control for Pin 15. This bit selects the function of the port pad."]
    #[inline(always)]
    pub fn pdis3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, pdisc::Pdis3, Pdisc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,pdisc::Pdis3, Pdisc_SPEC,crate::common::RW>::from_register(self,0)
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
        #[doc = "0 Digital        functionality of pad Pn.x is enabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Digital        functionality  including pull resistors  of pad Pn.x is disabled. Analog        input function  where this is available  can be used."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pdis1_SPEC;
    pub type Pdis1 = crate::EnumBitfieldStruct<u8, Pdis1_SPEC>;
    impl Pdis1 {
        #[doc = "0 Digital        functionality of pad Pn.x is enabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Digital        functionality  including pull resistors  of pad Pn.x is disabled. Analog        input function  where this is available  can be used."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pdis2_SPEC;
    pub type Pdis2 = crate::EnumBitfieldStruct<u8, Pdis2_SPEC>;
    impl Pdis2 {
        #[doc = "0 Digital        functionality of pad Pn.x is enabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Digital        functionality  including pull resistors  of pad Pn.x is disabled. Analog        input function  where this is available  can be used."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pdis3_SPEC;
    pub type Pdis3 = crate::EnumBitfieldStruct<u8, Pdis3_SPEC>;
    impl Pdis3 {
        #[doc = "0 Digital        functionality of pad Pn.x is enabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Digital        functionality  including pull resistors  of pad Pn.x is disabled. Analog        input function  where this is available  can be used."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdr0_SPEC;
impl crate::sealed::RegSpec for Pdr0_SPEC {
    type DataType = u32;
}
#[doc = "Port 12 Pad Driver Mode Register 0\n resetvalue={:0x0,:0x0,After SSW execution:0x2222,After SSW execution:0x2222}"]
pub type Pdr0 = crate::RegValueT<Pdr0_SPEC>;

impl Pdr0 {
    #[doc = "Pad Driver Mode for Pin 7"]
    #[inline(always)]
    pub fn pd0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Level Selection for Pin 7"]
    #[inline(always)]
    pub fn pl0(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Driver Mode for Pin 7"]
    #[inline(always)]
    pub fn pd1(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Level Selection for Pin 7"]
    #[inline(always)]
    pub fn pl1(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Driver Mode for Pin 7"]
    #[inline(always)]
    pub fn pd2(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Level Selection for Pin 7"]
    #[inline(always)]
    pub fn pl2(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Driver Mode for Pin 7"]
    #[inline(always)]
    pub fn pd3(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pad Level Selection for Pin 7"]
    #[inline(always)]
    pub fn pl3(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Pdr0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Pdr0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pdr0 {
    #[inline(always)]
    fn default() -> Pdr0 {
        <crate::RegValueT<Pdr0_SPEC> as RegisterValue<_>>::new(0)
    }
}
