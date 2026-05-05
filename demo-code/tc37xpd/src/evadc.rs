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
#[doc = r"EVADC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evadc(pub(super) *mut u8);
unsafe impl core::marker::Send for Evadc {}
unsafe impl core::marker::Sync for Evadc {}
impl Evadc {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "Access Protection Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn accprot0(&self) -> crate::common::Reg<self::Accprot0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }

    #[doc = "Access Protection Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn accprot1(&self) -> crate::common::Reg<self::Accprot1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(140usize)) }
    }

    #[doc = "Access Protection Register 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn accprot2(&self) -> crate::common::Reg<self::Accprot2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "External Multiplexer Interface Select Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn emuxsel(&self) -> crate::common::Reg<self::Emuxsel_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1008usize)) }
    }

    #[doc = "Global Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn globcfg(&self) -> crate::common::Reg<self::Globcfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={PowerOn Reset:0x0C5C013}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst0(&self) -> crate::common::Reg<self::Krst0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst1(&self) -> crate::common::Reg<self::Krst1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }

    #[doc = "Kernel Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krstclr(&self) -> crate::common::Reg<self::Krstclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }

    #[doc = "OCDS Control and Status Register\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }
    #[doc = "FC"]
    #[inline(always)]
    pub fn fc(self) -> [self::Fc; 4] {
        unsafe {
            [
                self::Fc(self.0.add(0x3400usize + 0x0usize)),
                self::Fc(self.0.add(0x3400usize + 0x100usize)),
                self::Fc(self.0.add(0x3400usize + 0x200usize)),
                self::Fc(self.0.add(0x3400usize + 0x300usize)),
            ]
        }
    }
    #[doc = "GLOB"]
    #[inline(always)]
    pub fn glob(self) -> self::Glob {
        unsafe { self::Glob(self.0.add(160usize)) }
    }
    #[doc = "G"]
    #[inline(always)]
    pub fn g(self) -> [self::G; 12] {
        unsafe {
            [
                self::G(self.0.add(0x400usize + 0x0usize)),
                self::G(self.0.add(0x400usize + 0x400usize)),
                self::G(self.0.add(0x400usize + 0x800usize)),
                self::G(self.0.add(0x400usize + 0xc00usize)),
                self::G(self.0.add(0x400usize + 0x1000usize)),
                self::G(self.0.add(0x400usize + 0x1400usize)),
                self::G(self.0.add(0x400usize + 0x1800usize)),
                self::G(self.0.add(0x400usize + 0x1c00usize)),
                self::G(self.0.add(0x400usize + 0x2000usize)),
                self::G(self.0.add(0x400usize + 0x2400usize)),
                self::G(self.0.add(0x400usize + 0x2800usize)),
                self::G(self.0.add(0x400usize + 0x2c00usize)),
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
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, accen0::En0, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,accen0::En0, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, accen0::En1, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,accen0::En1, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, accen0::En2, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,accen0::En2, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, accen0::En3, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,accen0::En3, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, accen0::En4, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,accen0::En4, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, accen0::En5, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,accen0::En5, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, accen0::En6, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,accen0::En6, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, accen0::En7, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,accen0::En7, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, accen0::En8, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,accen0::En8, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, accen0::En9, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,accen0::En9, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, accen0::En10, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,accen0::En10, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, accen0::En11, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,accen0::En11, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, accen0::En12, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,accen0::En12, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, accen0::En13, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,accen0::En13, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, accen0::En14, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,accen0::En14, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, accen0::En15, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,accen0::En15, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, accen0::En16, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,accen0::En16, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, accen0::En17, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,accen0::En17, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, accen0::En18, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,accen0::En18, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, accen0::En19, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,accen0::En19, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, accen0::En20, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,accen0::En20, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, accen0::En21, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,accen0::En21, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, accen0::En22, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,accen0::En22, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, accen0::En23, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,accen0::En23, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, accen0::En24, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,accen0::En24, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, accen0::En25, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,accen0::En25, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, accen0::En26, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,accen0::En26, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, accen0::En27, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,accen0::En27, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, accen0::En28, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,accen0::En28, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, accen0::En29, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,accen0::En29, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, accen0::En30, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,accen0::En30, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID x"]
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
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En1_SPEC;
    pub type En1 = crate::EnumBitfieldStruct<u8, En1_SPEC>;
    impl En1 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En2_SPEC;
    pub type En2 = crate::EnumBitfieldStruct<u8, En2_SPEC>;
    impl En2 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En3_SPEC;
    pub type En3 = crate::EnumBitfieldStruct<u8, En3_SPEC>;
    impl En3 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En4_SPEC;
    pub type En4 = crate::EnumBitfieldStruct<u8, En4_SPEC>;
    impl En4 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En5_SPEC;
    pub type En5 = crate::EnumBitfieldStruct<u8, En5_SPEC>;
    impl En5 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En6_SPEC;
    pub type En6 = crate::EnumBitfieldStruct<u8, En6_SPEC>;
    impl En6 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En7_SPEC;
    pub type En7 = crate::EnumBitfieldStruct<u8, En7_SPEC>;
    impl En7 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En8_SPEC;
    pub type En8 = crate::EnumBitfieldStruct<u8, En8_SPEC>;
    impl En8 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En9_SPEC;
    pub type En9 = crate::EnumBitfieldStruct<u8, En9_SPEC>;
    impl En9 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En10_SPEC;
    pub type En10 = crate::EnumBitfieldStruct<u8, En10_SPEC>;
    impl En10 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En11_SPEC;
    pub type En11 = crate::EnumBitfieldStruct<u8, En11_SPEC>;
    impl En11 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En12_SPEC;
    pub type En12 = crate::EnumBitfieldStruct<u8, En12_SPEC>;
    impl En12 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En13_SPEC;
    pub type En13 = crate::EnumBitfieldStruct<u8, En13_SPEC>;
    impl En13 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En14_SPEC;
    pub type En14 = crate::EnumBitfieldStruct<u8, En14_SPEC>;
    impl En14 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En15_SPEC;
    pub type En15 = crate::EnumBitfieldStruct<u8, En15_SPEC>;
    impl En15 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En16_SPEC;
    pub type En16 = crate::EnumBitfieldStruct<u8, En16_SPEC>;
    impl En16 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En17_SPEC;
    pub type En17 = crate::EnumBitfieldStruct<u8, En17_SPEC>;
    impl En17 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En18_SPEC;
    pub type En18 = crate::EnumBitfieldStruct<u8, En18_SPEC>;
    impl En18 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En19_SPEC;
    pub type En19 = crate::EnumBitfieldStruct<u8, En19_SPEC>;
    impl En19 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En20_SPEC;
    pub type En20 = crate::EnumBitfieldStruct<u8, En20_SPEC>;
    impl En20 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En21_SPEC;
    pub type En21 = crate::EnumBitfieldStruct<u8, En21_SPEC>;
    impl En21 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En22_SPEC;
    pub type En22 = crate::EnumBitfieldStruct<u8, En22_SPEC>;
    impl En22 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En23_SPEC;
    pub type En23 = crate::EnumBitfieldStruct<u8, En23_SPEC>;
    impl En23 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En24_SPEC;
    pub type En24 = crate::EnumBitfieldStruct<u8, En24_SPEC>;
    impl En24 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En25_SPEC;
    pub type En25 = crate::EnumBitfieldStruct<u8, En25_SPEC>;
    impl En25 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En26_SPEC;
    pub type En26 = crate::EnumBitfieldStruct<u8, En26_SPEC>;
    impl En26 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En27_SPEC;
    pub type En27 = crate::EnumBitfieldStruct<u8, En27_SPEC>;
    impl En27 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En28_SPEC;
    pub type En28 = crate::EnumBitfieldStruct<u8, En28_SPEC>;
    impl En28 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En29_SPEC;
    pub type En29 = crate::EnumBitfieldStruct<u8, En29_SPEC>;
    impl En29 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En30_SPEC;
    pub type En30 = crate::EnumBitfieldStruct<u8, En30_SPEC>;
    impl En30 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En31_SPEC;
    pub type En31 = crate::EnumBitfieldStruct<u8, En31_SPEC>;
    impl En31 {
        #[doc = "0 No Write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accprot0_SPEC;
impl crate::sealed::RegSpec for Accprot0_SPEC {
    type DataType = u32;
}
#[doc = "Access Protection Register 0\n resetvalue={Application Reset:0x0}"]
pub type Accprot0 = crate::RegValueT<Accprot0_SPEC>;

impl Accprot0 {
    #[doc = "Access Protection Channel Control  Primary Groups. Each bit of this bitfield is associated with the corresponding group. 0  Full access to registers 1  Write access to channel control registers is blocked"]
    #[inline(always)]
    pub fn apcp(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Accprot0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Accprot0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Protection Channel Control  Secondary Groups. Each bit of this bitfield is associated with the corresponding group. 0  Full access to registers 1  Write access to channel control registers is blocked"]
    #[inline(always)]
    pub fn apcs(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Accprot0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Accprot0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Protection Initialization  Primary Groups. Each bit of this bitfield is associated with the corresponding group. 0  Full access to registers 1  Write access to initialization registers is blocked"]
    #[inline(always)]
    pub fn apip(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Accprot0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Accprot0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Protection Initialization  Secondary Groups. Each bit of this bitfield is associated with the corresponding group. 0  Full access to registers 1  Write access to initialization registers is blocked"]
    #[inline(always)]
    pub fn apis(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Accprot0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Accprot0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Accprot0 {
    #[inline(always)]
    fn default() -> Accprot0 {
        <crate::RegValueT<Accprot0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accprot1_SPEC;
impl crate::sealed::RegSpec for Accprot1_SPEC {
    type DataType = u32;
}
#[doc = "Access Protection Register 1\n resetvalue={Application Reset:0x0}"]
pub type Accprot1 = crate::RegValueT<Accprot1_SPEC>;

impl Accprot1 {
    #[doc = "Access Protection Service Request  Primary Groups. Each bit of this bitfield is associated with the corresponding group. 0  Full access to registers 1  Write access to service request registers is blocked"]
    #[inline(always)]
    pub fn apsp(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Accprot1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Accprot1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Protection Service Request  Secondary Groups. Each bit of this bitfield is associated with the corresponding group. 0  Full access to registers 1  Write access to service request registers is blocked"]
    #[inline(always)]
    pub fn apss(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Accprot1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Accprot1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Protection Result Registers  Primary Groups. Each bit of this bitfield is associated with the corresponding group. 0  Full access to registers 1  Write access to result registers is blocked"]
    #[inline(always)]
    pub fn aprp(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Accprot1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Accprot1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Protection Result Registers  Secondary Groups. Each bit of this bitfield is associated with the corresponding group. 0  Full access to registers 1  Write access to result registers is blocked"]
    #[inline(always)]
    pub fn aprs(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Accprot1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Accprot1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Accprot1 {
    #[inline(always)]
    fn default() -> Accprot1 {
        <crate::RegValueT<Accprot1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accprot2_SPEC;
impl crate::sealed::RegSpec for Accprot2_SPEC {
    type DataType = u32;
}
#[doc = "Access Protection Register 2\n resetvalue={Application Reset:0x0}"]
pub type Accprot2 = crate::RegValueT<Accprot2_SPEC>;

impl Accprot2 {
    #[doc = "Access Protection Fast Compare Channels. Each bit of this bitfield is associated with the corresponding channel. 0  Full access to registers 1  Write access to fast compare channel registers is blocked"]
    #[inline(always)]
    pub fn apf(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Accprot2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Accprot2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Protection Global Configuration   APGC"]
    #[inline(always)]
    pub fn apgc(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, accprot2::Apgc, Accprot2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            accprot2::Apgc,
            Accprot2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Protection External Multiplexer   APEM"]
    #[inline(always)]
    pub fn apem(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, accprot2::Apem, Accprot2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            accprot2::Apem,
            Accprot2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Access Protection Test Function   APTF"]
    #[inline(always)]
    pub fn aptf(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, accprot2::Aptf, Accprot2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            accprot2::Aptf,
            Accprot2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Accprot2 {
    #[inline(always)]
    fn default() -> Accprot2 {
        <crate::RegValueT<Accprot2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod accprot2 {
    pub struct Apgc_SPEC;
    pub type Apgc = crate::EnumBitfieldStruct<u8, Apgc_SPEC>;
    impl Apgc {
        #[doc = "Full access to register"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access to global configuration register is blocked"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Apem_SPEC;
    pub type Apem = crate::EnumBitfieldStruct<u8, Apem_SPEC>;
    impl Apem {
        #[doc = "Full access to registers"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access to external multiplexer registers is blocked"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Aptf_SPEC;
    pub type Aptf = crate::EnumBitfieldStruct<u8, Aptf_SPEC>;
    impl Aptf {
        #[doc = "Full access to register"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access to test function register is blocked"]
        pub const CONST_11: Self = Self::new(1);
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
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control of the module. Also the analog section        is disabled by clearing ANONS."]
    #[inline(always)]
    pub fn disr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, clc::Disr, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,clc::Disr, Clc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Module Disable Status Bit   DISS"]
    #[inline(always)]
    pub fn diss(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, clc::Diss, Clc_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,clc::Diss, Clc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Sleep Mode Enable Control   EDIS. Used to control the module  8217 s reaction to sleep mode."]
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
        #[doc = "On request  enable the module clock"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Off request  stop the module clock"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Diss_SPEC;
    pub type Diss = crate::EnumBitfieldStruct<u8, Diss_SPEC>;
    impl Diss {
        #[doc = "Module clock is enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Off  module is not clocked. 1 f SPB and f ADC are disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Edis_SPEC;
    pub type Edis = crate::EnumBitfieldStruct<u8, Edis_SPEC>;
    impl Edis {
        #[doc = "Sleep mode request is enabled and functional"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module disregards the sleep mode control signal"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emuxsel_SPEC;
impl crate::sealed::RegSpec for Emuxsel_SPEC {
    type DataType = u32;
}
#[doc = "External Multiplexer Interface Select Register\n resetvalue={Application Reset:0x0}"]
pub type Emuxsel = crate::RegValueT<Emuxsel_SPEC>;

impl Emuxsel {
    #[doc = "External Multiplexer Group for Interface 0. Defines the group whose external multiplexer control signals are routed        to EMUX interface 0  pins EMUX0x ."]
    #[inline(always)]
    pub fn emuxgrp0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Emuxsel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Emuxsel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "External Multiplexer Group for Interface 1. Defines the group whose external multiplexer control signals are routed        to EMUX interface 1  pins EMUX1x ."]
    #[inline(always)]
    pub fn emuxgrp1(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Emuxsel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Emuxsel_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Emuxsel {
    #[inline(always)]
    fn default() -> Emuxsel {
        <crate::RegValueT<Emuxsel_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Globcfg_SPEC;
impl crate::sealed::RegSpec for Globcfg_SPEC {
    type DataType = u32;
}
#[doc = "Global Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Globcfg = crate::RegValueT<Globcfg_SPEC>;

impl Globcfg {
    #[doc = "Unsynchronized Clock Generation   USC. Defines the way the analog clock is generated."]
    #[inline(always)]
    pub fn usc(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, globcfg::Usc, Globcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,globcfg::Usc, Globcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Supply Voltage Level   SUPLEV. Adjusts the analog circuitry to the supply voltage used in the        application system. Make sure to keep SUPLEV  160    160  00 or 01 in the case of a 5  160 V supply."]
    #[inline(always)]
    pub fn suplev(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, globcfg::Suplev, Globcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            13,
            0x3,
            1,
            0,
            globcfg::Suplev,
            Globcfg_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Write Control for Control Parameters   CPWC"]
    #[inline(always)]
    pub fn cpwc(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, globcfg::Cpwc, Globcfg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<15,0x1,1,0,globcfg::Cpwc, Globcfg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Start Up Calibration   SUCAL. Writing 1 to bit SUCAL initiates the start up calibration phase of all enabled analog converters  except for the fast compare channels . Before and during start up calibration  all converters must be inactive. After reset this is the case anyway."]
    #[inline(always)]
    pub fn sucal(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, globcfg::Sucal, Globcfg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<31,0x1,1,0,globcfg::Sucal, Globcfg_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Globcfg {
    #[inline(always)]
    fn default() -> Globcfg {
        <crate::RegValueT<Globcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod globcfg {
    pub struct Usc_SPEC;
    pub type Usc = crate::EnumBitfieldStruct<u8, Usc_SPEC>;
    impl Usc {
        #[doc = "Synchronized mode. Initial clock pulse is defined by the phase synchronizer"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Unsynchronized mode. 1 The analog        clock is generated independently."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Suplev_SPEC;
    pub type Suplev = crate::EnumBitfieldStruct<u8, Suplev_SPEC>;
    impl Suplev {
        #[doc = "Automatic control  voltage range is controlled by the power supply"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Upper voltage range  assume a 5 V power supply is connected"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "Lower voltage range  assume a 3.3 V power supply is connected"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Cpwc_SPEC;
    pub type Cpwc = crate::EnumBitfieldStruct<u8, Cpwc_SPEC>;
    impl Cpwc {
        #[doc = "No write access to control parameters"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Bitfields SUPLEV  USC can be written"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sucal_SPEC;
    pub type Sucal = crate::EnumBitfieldStruct<u8, Sucal_SPEC>;
    impl Sucal {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Initiate the start up calibration phase.  indication in bit GxARBCFG.CAL"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={PowerOn Reset:0x0C5C013}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision   MOD REV. Indicates the revision number of the implementation. This information        depends on the design step."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MOD TYPE. This internal marker is fixed to C0 ."]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number   MOD NUMBER. Indicates the module identification number   00C5   SARADC ."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(12959763)
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
    #[doc = "Kernel Reset   RST. Request a kernel reset. The reset is executed if the reset bits of both        kernel reset registers are set. RST is cleared after the kernel reset was executed."]
    #[inline(always)]
    pub fn rst(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, krst0::Rst, Krst0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,krst0::Rst, Krst0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Kernel Reset Status   RSTSTAT. Indicates an executed kernel reset. RSTSTAT is set after the execution        of a kernel reset in the same clock cycle in which the reset bits are        cleared. Clear RSTSTAT by setting bit CLR in register KRSTCLR."]
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
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "A kernel reset was requested"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rststat_SPEC;
    pub type Rststat = crate::EnumBitfieldStruct<u8, Rststat_SPEC>;
    impl Rststat {
        #[doc = "No kernel reset was executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Kernel reset was executed"]
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
    #[doc = "Kernel Reset   RST. Request a kernel reset. The reset is executed if the reset bits of both        kernel reset registers are set. RST is cleared after the kernel reset was executed."]
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
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "A kernel reset was requested"]
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
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Clear Kernel Reset Status KRST0.RSTSTAT"]
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
    #[doc = "TGS  TGB Write Protection   TG P. TGS and TGB are only written when TG P is 1  otherwise unchanged. Read        as 0."]
    #[inline(always)]
    pub fn tg_p(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ocs_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ocs_SPEC, crate::common::W>::from_register(
            self, 0,
        )
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
        #[doc = "No Trigger Set output"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Trigger Set 1. TS16 SSIGC  input sample signals of primary secondary groups"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "Trigger Set 2. TS16 SSIGF  input sample signals of fast compare channels"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Tgb_SPEC;
    pub type Tgb = crate::EnumBitfieldStruct<u8, Tgb_SPEC>;
    impl Tgb {
        #[doc = "Trigger Set is output on OTGB0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Trigger Set is output on OTGB1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sussta_SPEC;
    pub type Sussta = crate::EnumBitfieldStruct<u8, Sussta_SPEC>;
    impl Sussta {
        #[doc = "Module is not  yet  suspended"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is suspended"]
        pub const CONST_11: Self = Self::new(1);
    }
}

#[doc = "FC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fc(pub(super) *mut u8);
unsafe impl core::marker::Send for Fc {}
unsafe impl core::marker::Sync for Fc {}
impl Fc {
    #[doc = "Boundary Flag Register  FC Channel 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcxfcbfl(&self) -> crate::common::Reg<fc::FCxFcbfl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "Fast Compare Control Register  FC Channel 0\n resetvalue={Application Reset:0x0C20}"]
    #[inline(always)]
    pub const fn fcxfcctrl(&self) -> crate::common::Reg<fc::FCxFcctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Fast Comp. Hysteresis Register  FC Channel 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcxfchyst(&self) -> crate::common::Reg<fc::FCxFchyst_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Fast Compare Mode Register  FC Channel 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcxfcm(&self) -> crate::common::Reg<fc::FCxFcm_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Fast Compare Ramp Register 0  FC Channel 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcxfcramp0(&self) -> crate::common::Reg<fc::FCxFcramp0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Fast Compare Ramp Register 1  FC Channel 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcxfcramp1(&self) -> crate::common::Reg<fc::FCxFcramp1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Test Register  FC Channel 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcxtest(&self) -> crate::common::Reg<fc::FCxTest_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(248usize)) }
    }
}
pub mod fc {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCxFcbfl_SPEC;
    impl crate::sealed::RegSpec for FCxFcbfl_SPEC {
        type DataType = u32;
    }
    #[doc = "Boundary Flag Register  FC Channel 0\n resetvalue={Application Reset:0x0}"]
    pub type FCxFcbfl = crate::RegValueT<FCxFcbfl_SPEC>;

    impl FCxFcbfl {
        #[doc = "Boundary Flag   BFL"]
        #[inline(always)]
        pub fn bfl(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            fcxfcbfl::Bfl,
            FCxFcbfl_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                fcxfcbfl::Bfl,
                FCxFcbfl_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Boundary Flag Activation Select   BFA"]
        #[inline(always)]
        pub fn bfa(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            fcxfcbfl::Bfa,
            FCxFcbfl_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                fcxfcbfl::Bfa,
                FCxFcbfl_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Boundary Flag Inversion Control   BFI"]
        #[inline(always)]
        pub fn bfi(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1,
            1,
            0,
            fcxfcbfl::Bfi,
            FCxFcbfl_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                fcxfcbfl::Bfi,
                FCxFcbfl_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Boundary Flag Software Control   BFS"]
        #[inline(always)]
        pub fn bfs(
            self,
        ) -> crate::common::RegisterField<
            12,
            0x3,
            1,
            0,
            fcxfcbfl::Bfs,
            FCxFcbfl_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                12,
                0x3,
                1,
                0,
                fcxfcbfl::Bfs,
                FCxFcbfl_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Boundary Flag Mode Control   BFM"]
        #[inline(always)]
        pub fn bfm(
            self,
        ) -> crate::common::RegisterField<
            16,
            0x1,
            1,
            0,
            fcxfcbfl::Bfm,
            FCxFcbfl_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0x1,
                1,
                0,
                fcxfcbfl::Bfm,
                FCxFcbfl_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Boundary Flag Value   BFV. Defines the logic value that replaces the compare result while the gate        input is inactive  low  in lock mode  see also bitfield GTMODE in        register GxFCCTRL"]
        #[inline(always)]
        pub fn bfv(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, FCxFcbfl_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<17,1,0,FCxFcbfl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Fast Compare Result   FCR. Indicates the last generated compare result If the input signal equals the reference value the analog comparator          will return either  quot above quot  or  quot below quot ."]
        #[inline(always)]
        pub fn fcr(
            self,
        ) -> crate::common::RegisterField<
            28,
            0x1,
            1,
            0,
            fcxfcbfl::Fcr,
            FCxFcbfl_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                28,
                0x1,
                1,
                0,
                fcxfcbfl::Fcr,
                FCxFcbfl_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Valid Flag   VF. Indicates a new result in bit FCR. Bit VF is automatically cleared upon reading register FCxFCBFL."]
        #[inline(always)]
        pub fn vf(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            fcxfcbfl::Vf,
            FCxFcbfl_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                fcxfcbfl::Vf,
                FCxFcbfl_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for FCxFcbfl {
        #[inline(always)]
        fn default() -> FCxFcbfl {
            <crate::RegValueT<FCxFcbfl_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod fcxfcbfl {
        pub struct Bfl_SPEC;
        pub type Bfl = crate::EnumBitfieldStruct<u8, Bfl_SPEC>;
        impl Bfl {
            #[doc = "Passive state. result has not yet crossed the activation boundary  see bitfield BFA          or selected gate signal is inactive  or this boundary flag is disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Active state. result has crossed the activation boundary"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Bfa_SPEC;
        pub type Bfa = crate::EnumBitfieldStruct<u8, Bfa_SPEC>;
        impl Bfa {
            #[doc = "Set boundary flag BFL if new result FCR   1  input gets above the defined band or compare value   clear if FCR   0  input below"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Set boundary flag BFL if new result FCR   0  input gets below the defined band or compare value   clear if FCR   1  input above"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Bfi_SPEC;
        pub type Bfi = crate::EnumBitfieldStruct<u8, Bfi_SPEC>;
        impl Bfi {
            #[doc = "Use BFL directly"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Use inverted BFL"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Bfs_SPEC;
        pub type Bfs = crate::EnumBitfieldStruct<u8, Bfs_SPEC>;
        impl Bfs {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear BFL"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Set BFL"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Toggle BFL"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Bfm_SPEC;
        pub type Bfm = crate::EnumBitfieldStruct<u8, Bfm_SPEC>;
        impl Bfm {
            #[doc = "Disable boundary flag  BFL is not changed by FCR"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Enable boundary flag.  see also bitfield GTMODE in register CTRL"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Fcr_SPEC;
        pub type Fcr = crate::EnumBitfieldStruct<u8, Fcr_SPEC>;
        impl Fcr {
            #[doc = "Signal level below reference value"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Signal level above reference value"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Vf_SPEC;
        pub type Vf = crate::EnumBitfieldStruct<u8, Vf_SPEC>;
        impl Vf {
            #[doc = "No new result available"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Bit FCR has been updated with new value and has not yet been read"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCxFcctrl_SPEC;
    impl crate::sealed::RegSpec for FCxFcctrl_SPEC {
        type DataType = u32;
    }
    #[doc = "Fast Compare Control Register  FC Channel 0\n resetvalue={Application Reset:0x0C20}"]
    pub type FCxFcctrl = crate::RegValueT<FCxFcctrl_SPEC>;

    impl FCxFcctrl {
        #[doc = "Sample Time Control for Fast Comparisons   STCF. Number of additional clock cycles to be added to the minimum sample        phase of 2 analog clock cycles  Coding and resulting sample time see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn stcf(
            self,
        ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, FCxFcctrl_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1f,1,0,u8, FCxFcctrl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Reference Precharge Enable   RPE. Enabled after reset."]
        #[inline(always)]
        pub fn rpe(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            fcxfcctrl::Rpe,
            FCxFcctrl_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                fcxfcctrl::Rpe,
                FCxFcctrl_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Analog Input Precharge Enable for Fast Comparisons   AIPF. The buffer is enabled automatically while AIPF   8800  00 ."]
        #[inline(always)]
        pub fn aipf(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x3,
            1,
            0,
            fcxfcctrl::Aipf,
            FCxFcctrl_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                6,
                0x3,
                1,
                0,
                fcxfcctrl::Aipf,
                FCxFcctrl_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Channel Event Mode   CHEVMODE. Generate a channel event  For a service request summary  refer to CROSSREFERENCE ."]
        #[inline(always)]
        pub fn chevmode(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x3,
            1,
            0,
            fcxfcctrl::Chevmode,
            FCxFcctrl_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x3,
                1,
                0,
                fcxfcctrl::Chevmode,
                FCxFcctrl_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Divider Factor for the Analog Internal Clock   DIVA. Defines the frequency of the analog converter clock f ADCI  base clock for conversion steps   derived from the peripheral clock  f ADCI   f ADC   CP."]
        #[inline(always)]
        pub fn diva(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1f,
            1,
            0,
            fcxfcctrl::Diva,
            FCxFcctrl_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                10,
                0x1f,
                1,
                0,
                fcxfcctrl::Diva,
                FCxFcctrl_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Write Control for Control Parameters   CPWC"]
        #[inline(always)]
        pub fn cpwc(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            fcxfcctrl::Cpwc,
            FCxFcctrl_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                fcxfcctrl::Cpwc,
                FCxFcctrl_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "External Trigger Input Selection   XTSEL. The connected trigger input signals are listed in the product specific        appendix. The selected input signal can be used as a trigger source or as a gate          signal  depending on the operating mode."]
        #[inline(always)]
        pub fn xtsel(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, FCxFcctrl_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, FCxFcctrl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "External Trigger Level   XTLVL. Current level of the selected trigger input"]
        #[inline(always)]
        pub fn xtlvl(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, FCxFcctrl_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<20,1,0,FCxFcctrl_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Trigger Operating Mode   XTMODE"]
        #[inline(always)]
        pub fn xtmode(
            self,
        ) -> crate::common::RegisterField<
            21,
            0x3,
            1,
            0,
            fcxfcctrl::Xtmode,
            FCxFcctrl_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                21,
                0x3,
                1,
                0,
                fcxfcctrl::Xtmode,
                FCxFcctrl_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "External Trigger Polarity   XTPOL"]
        #[inline(always)]
        pub fn xtpol(
            self,
        ) -> crate::common::RegisterField<
            23,
            0x1,
            1,
            0,
            fcxfcctrl::Xtpol,
            FCxFcctrl_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                23,
                0x1,
                1,
                0,
                fcxfcctrl::Xtpol,
                FCxFcctrl_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Gate Operating Mode   GTMODE"]
        #[inline(always)]
        pub fn gtmode(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x3,
            1,
            0,
            fcxfcctrl::Gtmode,
            FCxFcctrl_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0x3,
                1,
                0,
                fcxfcctrl::Gtmode,
                FCxFcctrl_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Write Control for Trigger Gate Configuration   XTWC"]
        #[inline(always)]
        pub fn xtwc(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            fcxfcctrl::Xtwc,
            FCxFcctrl_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                fcxfcctrl::Xtwc,
                FCxFcctrl_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for FCxFcctrl {
        #[inline(always)]
        fn default() -> FCxFcctrl {
            <crate::RegValueT<FCxFcctrl_SPEC> as RegisterValue<_>>::new(3104)
        }
    }
    pub mod fcxfcctrl {
        pub struct Rpe_SPEC;
        pub type Rpe = crate::EnumBitfieldStruct<u8, Rpe_SPEC>;
        impl Rpe {
            #[doc = "No reference precharge. Only use V AREF   V AGND for the conversion."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Precharge enabled. 1 Use V DDM   V SSM for precharging and V AREF   V AGND for the final adjustment during a conversion. Precharge the reference input for 1 clock phase."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Aipf_SPEC;
        pub type Aipf = crate::EnumBitfieldStruct<u8, Aipf_SPEC>;
        impl Aipf {
            #[doc = "No precharge"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Precharge for 8 clock cycles"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Precharge for 16 clock cycles"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Precharge for 32 clock cycles"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Chevmode_SPEC;
        pub type Chevmode = crate::EnumBitfieldStruct<u8, Chevmode_SPEC>;
        impl Chevmode {
            #[doc = "Never"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "If result becomes high  above compare value"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "If result becomes low  below compare value"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "If result switches to either level"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Diva_SPEC;
        pub type Diva = crate::EnumBitfieldStruct<u8, Diva_SPEC>;
        impl Diva {
            #[doc = "CP   2  max. frequency"]
            pub const CONST_000: Self = Self::new(0);
            #[doc = "CP   2"]
            pub const CONST_011: Self = Self::new(1);
        }
        pub struct Cpwc_SPEC;
        pub type Cpwc = crate::EnumBitfieldStruct<u8, Cpwc_SPEC>;
        impl Cpwc {
            #[doc = "No write access to control parameters"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Bitfields DIVA  CHEVMODE  AIPF  RPC  STCF can be written"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Xtmode_SPEC;
        pub type Xtmode = crate::EnumBitfieldStruct<u8, Xtmode_SPEC>;
        impl Xtmode {
            #[doc = "No external trigger"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Trigger event upon a falling edge"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Trigger event upon a rising edge"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Trigger event upon any edge"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Xtpol_SPEC;
        pub type Xtpol = crate::EnumBitfieldStruct<u8, Xtpol_SPEC>;
        impl Xtpol {
            #[doc = "Use selected input signal directly"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Invert selected input signal"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Gtmode_SPEC;
        pub type Gtmode = crate::EnumBitfieldStruct<u8, Gtmode_SPEC>;
        impl Gtmode {
            #[doc = "No gate function"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Alternate value mode.  see CROSSREFERENCE"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Lock boundary flag.  see CROSSREFERENCE"]
            pub const CONST_22: Self = Self::new(2);
        }
        pub struct Xtwc_SPEC;
        pub type Xtwc = crate::EnumBitfieldStruct<u8, Xtwc_SPEC>;
        impl Xtwc {
            #[doc = "No write access to trigger configuration"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Bitfields FCCHNR  GTMODE  XTPOL  XTMODE  XTSEL can be written"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCxFchyst_SPEC;
    impl crate::sealed::RegSpec for FCxFchyst_SPEC {
        type DataType = u32;
    }
    #[doc = "Fast Comp. Hysteresis Register  FC Channel 0\n resetvalue={Application Reset:0x0}"]
    pub type FCxFchyst = crate::RegValueT<FCxFchyst_SPEC>;

    impl FCxFchyst {
        #[doc = "Lower Delta Value   DELTAMINUS. This value is subtracted from the reference value while the last result        is 1"]
        #[inline(always)]
        pub fn deltaminus(
            self,
        ) -> crate::common::RegisterField<2, 0x3ff, 1, 0, u16, FCxFchyst_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x3ff,1,0,u16, FCxFchyst_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Upper Delta Value   DELTAPLUS. This value is added to the reference value while the last result is 0"]
        #[inline(always)]
        pub fn deltaplus(
            self,
        ) -> crate::common::RegisterField<18, 0x3ff, 1, 0, u16, FCxFchyst_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<18,0x3ff,1,0,u16, FCxFchyst_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for FCxFchyst {
        #[inline(always)]
        fn default() -> FCxFchyst {
            <crate::RegValueT<FCxFchyst_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCxFcm_SPEC;
    impl crate::sealed::RegSpec for FCxFcm_SPEC {
        type DataType = u32;
    }
    #[doc = "Fast Compare Mode Register  FC Channel 0\n resetvalue={Application Reset:0x0}"]
    pub type FCxFcm = crate::RegValueT<FCxFcm_SPEC>;

    impl FCxFcm {
        #[doc = "Run Control for Compare Channel   RUNCOMP. Defines the basic run conditions of the fast compare channel."]
        #[inline(always)]
        pub fn runcomp(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x3,
            1,
            0,
            fcxfcm::Runcomp,
            FCxFcm_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x3,
                1,
                0,
                fcxfcm::Runcomp,
                FCxFcm_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Run Control for Ramp   RUNRAMP. Defines the run conditions for the ramp generation. Before changing the operating mode  stop the ramp timer  i.e. RUNRAMP            00 . REQTRx is the selected trigger          signal."]
        #[inline(always)]
        pub fn runramp(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x3,
            1,
            0,
            fcxfcm::Runramp,
            FCxFcm_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                2,
                0x3,
                1,
                0,
                fcxfcm::Runramp,
                FCxFcm_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Fast Compare Ramp Direction   FCRDIR"]
        #[inline(always)]
        pub fn fcrdir(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            fcxfcm::Fcrdir,
            FCxFcm_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                fcxfcm::Fcrdir,
                FCxFcm_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Analog Converter Control   ANON. The extended wakeup time is required before the analog part is fully          operable  see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn anon(
            self,
        ) -> crate::common::RegisterField<5, 0x1, 1, 0, fcxfcm::Anon, FCxFcm_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x1,1,0,fcxfcm::Anon, FCxFcm_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Analog Clock Synchronization Delay   ACSD. Defines the delay of the analog clock in clocks after the sync signal. Do not exceed the current DIVA setting for the clock delay. Valid only          if the phase synchronizer is selected  USC  160    160  0"]
        #[inline(always)]
        pub fn acsd(
            self,
        ) -> crate::common::RegisterField<6, 0x3, 1, 0, fcxfcm::Acsd, FCxFcm_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<6,0x3,1,0,fcxfcm::Acsd, FCxFcm_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Fast Compare Trigger Interval   FCTRIV. Defines the interval at which fast compare operations are triggered in        steps of 16   215  1  f ADC ."]
        #[inline(always)]
        pub fn fctriv(
            self,
        ) -> crate::common::RegisterField<
            8,
            0xff,
            1,
            0,
            fcxfcm::Fctriv,
            FCxFcm_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0xff,
                1,
                0,
                fcxfcm::Fctriv,
                FCxFcm_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Service Request Generation   SRG. For a service request summary  refer to CROSSREFERENCE ."]
        #[inline(always)]
        pub fn srg(
            self,
        ) -> crate::common::RegisterField<16, 0x3, 1, 0, fcxfcm::Srg, FCxFcm_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x3,1,0,fcxfcm::Srg, FCxFcm_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Automatic Update Enable   AUE. Defines the source of the value s  in bitfield FCREF."]
        #[inline(always)]
        pub fn aue(
            self,
        ) -> crate::common::RegisterField<18, 0x3, 1, 0, fcxfcm::Aue, FCxFcm_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<18,0x3,1,0,fcxfcm::Aue, FCxFcm_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Sample Synchronization Enable   SSE. See section CROSSREFERENCE ."]
        #[inline(always)]
        pub fn sse(
            self,
        ) -> crate::common::RegisterField<20, 0x1, 1, 0, fcxfcm::Sse, FCxFcm_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0x1,1,0,fcxfcm::Sse, FCxFcm_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Write Control for Fast Compare Mode Configuration   FCMWC"]
        #[inline(always)]
        pub fn fcmwc(
            self,
        ) -> crate::common::RegisterField<21, 0x1, 1, 0, fcxfcm::Fcmwc, FCxFcm_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<
                21,
                0x1,
                1,
                0,
                fcxfcm::Fcmwc,
                FCxFcm_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Fast Compare Reference Value   FCREF. The input level is compared to this value. The resulting reference level is V AREF   1  xa0 024   xd7   lt FCREF gt ."]
        #[inline(always)]
        pub fn fcref(
            self,
        ) -> crate::common::RegisterField<22, 0x3ff, 1, 0, u16, FCxFcm_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<22,0x3ff,1,0,u16, FCxFcm_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for FCxFcm {
        #[inline(always)]
        fn default() -> FCxFcm {
            <crate::RegValueT<FCxFcm_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod fcxfcm {
        pub struct Runcomp_SPEC;
        pub type Runcomp = crate::EnumBitfieldStruct<u8, Runcomp_SPEC>;
        impl Runcomp {
            #[doc = "Stop  no operation"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Always run"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Runramp_SPEC;
        pub type Runramp = crate::EnumBitfieldStruct<u8, Runramp_SPEC>;
        impl Runramp {
            #[doc = "Stop  no operation"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Start immediately when GxFCRAMP0 is written"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Start upon the selected trigger event of signal REQTRx"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Start immediately when GxFCRAMP0 is written and  stop upon the selected trigger event of signal REQTRx"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Fcrdir_SPEC;
        pub type Fcrdir = crate::EnumBitfieldStruct<u8, Fcrdir_SPEC>;
        impl Fcrdir {
            #[doc = "Ramp down. decrement ramp counter and stop when  lt counter gt   lt   FCRCOMPB"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Ramp up. increment ramp counter and stop when  lt counter gt   gt   FCRCOMPB"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Anon_SPEC;
        pub type Anon = crate::EnumBitfieldStruct<u8, Anon_SPEC>;
        impl Anon {
            #[doc = "Analog converter off"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Normal operation"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Acsd_SPEC;
        pub type Acsd = crate::EnumBitfieldStruct<u8, Acsd_SPEC>;
        impl Acsd {
            #[doc = "0  no delay"]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Fctriv_SPEC;
        pub type Fctriv = crate::EnumBitfieldStruct<u8, Fctriv_SPEC>;
        impl Fctriv {
            #[doc = "Interval is 16 clock cycles"]
            pub const CONST_000: Self = Self::new(0);
        }
        pub struct Srg_SPEC;
        pub type Srg = crate::EnumBitfieldStruct<u8, Srg_SPEC>;
        impl Srg {
            #[doc = "Off  no service requests are generated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Ramp end  issue service request when the ramp counter stops.  either by reaching FCRCOMPB or by RUNRAMP becoming 00"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "New value  issue service request when a value is written to FCREF"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "New result available"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Aue_SPEC;
        pub type Aue = crate::EnumBitfieldStruct<u8, Aue_SPEC>;
        impl Aue {
            #[doc = "No automatic update. value s  written by software."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Alternate value. While gate is active  high   value copied from bitfield FCRCOMPA While        gate is inactive  low   value copied from bitfield FCRCOMPB"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Ramp counter. value s  copied from ramp counter on ramp start or counter update."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Analog source. value s  written by the associated converter  see product specific        appendix ."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Sse_SPEC;
        pub type Sse = crate::EnumBitfieldStruct<u8, Sse_SPEC>;
        impl Sse {
            #[doc = "No synchronization"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Sample timing is synchronized. Recommended for operation of several ADCs."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Fcmwc_SPEC;
        pub type Fcmwc = crate::EnumBitfieldStruct<u8, Fcmwc_SPEC>;
        impl Fcmwc {
            #[doc = "No write access to FCM configuration"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Bitfields SSE  AUE  SRG  FCTRIV  ACSD  ANON  FCRDIR  RUNRAMP  RUNCOMP can be written"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCxFcramp0_SPEC;
    impl crate::sealed::RegSpec for FCxFcramp0_SPEC {
        type DataType = u32;
    }
    #[doc = "Fast Compare Ramp Register 0  FC Channel 0\n resetvalue={Application Reset:0x0}"]
    pub type FCxFcramp0 = crate::RegValueT<FCxFcramp0_SPEC>;

    impl FCxFcramp0 {
        #[doc = "Fast Compare Ramp Compare Value A   FCRCOMPA. The content of FCRCOMPA is copied to the ramp counter when a ramp is        started  i.e. upon the selected trigger event. The ramp counter defines        the reference value during ramp generation. It is  therefore  copied to        bitfield FCREF when the ramp is started and each time the counter        changes. FCRCOMPA is also used in alternate value mode while the gate is active         high ."]
        #[inline(always)]
        pub fn fcrcompa(
            self,
        ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, FCxFcramp0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3ff,1,0,u16, FCxFcramp0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Fast Compare Ramp Step Width   FCRSTEP. Configures the prescaler for FCRCOUNT in increments of 8   215  1  f ADC ."]
        #[inline(always)]
        pub fn fcrstep(
            self,
        ) -> crate::common::RegisterField<
            16,
            0xff,
            1,
            0,
            fcxfcramp0::Fcrstep,
            FCxFcramp0_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0xff,
                1,
                0,
                fcxfcramp0::Fcrstep,
                FCxFcramp0_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Write Control for Fast Compare Stepwidth   FSWC"]
        #[inline(always)]
        pub fn fswc(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            fcxfcramp0::Fswc,
            FCxFcramp0_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                fcxfcramp0::Fswc,
                FCxFcramp0_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for FCxFcramp0 {
        #[inline(always)]
        fn default() -> FCxFcramp0 {
            <crate::RegValueT<FCxFcramp0_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod fcxfcramp0 {
        pub struct Fcrstep_SPEC;
        pub type Fcrstep = crate::EnumBitfieldStruct<u8, Fcrstep_SPEC>;
        impl Fcrstep {
            #[doc = "Stepwidth is 8 clock cycles"]
            pub const CONST_000: Self = Self::new(0);
        }
        pub struct Fswc_SPEC;
        pub type Fswc = crate::EnumBitfieldStruct<u8, Fswc_SPEC>;
        impl Fswc {
            #[doc = "No write access to stepwidth"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Bitfield FCRSTEP can be written"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCxFcramp1_SPEC;
    impl crate::sealed::RegSpec for FCxFcramp1_SPEC {
        type DataType = u32;
    }
    #[doc = "Fast Compare Ramp Register 1  FC Channel 0\n resetvalue={Application Reset:0x0}"]
    pub type FCxFcramp1 = crate::RegValueT<FCxFcramp1_SPEC>;

    impl FCxFcramp1 {
        #[doc = "Fast Compare Ramp Compare Value B   FCRCOMPB. Defines the stop level of the generated ramp. FCRCOMPB is also used in alternate value mode while the gate is inactive         low ."]
        #[inline(always)]
        pub fn fcrcompb(
            self,
        ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, FCxFcramp1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3ff,1,0,u16, FCxFcramp1_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for FCxFcramp1 {
        #[inline(always)]
        fn default() -> FCxFcramp1 {
            <crate::RegValueT<FCxFcramp1_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FCxTest_SPEC;
    impl crate::sealed::RegSpec for FCxTest_SPEC {
        type DataType = u32;
    }
    #[doc = "Test Register  FC Channel 0\n resetvalue={Application Reset:0x0}"]
    pub type FCxTest = crate::RegValueT<FCxTest_SPEC>;

    impl FCxTest {
        #[doc = "User Test Vector   USRTEST. Described in the design specification."]
        #[inline(always)]
        pub fn usrtest(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, FCxTest_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, FCxTest_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Force FCONV   FFCONV"]
        #[inline(always)]
        pub fn ffconv(
            self,
        ) -> crate::common::RegisterField<
            16,
            0x1,
            1,
            0,
            fcxtest::Ffconv,
            FCxTest_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0x1,
                1,
                0,
                fcxtest::Ffconv,
                FCxTest_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Test Control   TC. Not listed combinations block write access to remaining bitfields."]
        #[inline(always)]
        pub fn tc(
            self,
        ) -> crate::common::RegisterField<28, 0xf, 1, 0, fcxtest::Tc, FCxTest_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<28,0xf,1,0,fcxtest::Tc, FCxTest_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for FCxTest {
        #[inline(always)]
        fn default() -> FCxTest {
            <crate::RegValueT<FCxTest_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod fcxtest {
        pub struct Ffconv_SPEC;
        pub type Ffconv = crate::EnumBitfieldStruct<u8, Ffconv_SPEC>;
        impl Ffconv {
            #[doc = "Signal FCONV is disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Signal FCONV is permanently asserted"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tc_SPEC;
        pub type Tc = crate::EnumBitfieldStruct<u8, Tc_SPEC>;
        impl Tc {
            #[doc = "The remaining bitfields of register TESTx can be written."]
            pub const CONST_1111: Self = Self::new(11);
        }
    }
}
#[doc = "GLOB"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Glob(pub(super) *mut u8);
unsafe impl core::marker::Send for Glob {}
unsafe impl core::marker::Sync for Glob {}
impl Glob {
    #[doc = "Global Boundary Select Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn globbound(&self) -> crate::common::Reg<glob::Globbound_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Global Event Flag Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn globeflag(&self) -> crate::common::Reg<glob::Globeflag_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }
    #[doc = "Input Class Register 0  Global\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn globiclassi(
        &self,
    ) -> [crate::common::Reg<glob::GlobiclasSi_SPEC, crate::common::RW>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
            ]
        }
    }
    #[doc = "Global Result Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn globrcr(&self) -> crate::common::Reg<glob::Globrcr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(480usize)) }
    }
    #[doc = "Global Result Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn globres(&self) -> crate::common::Reg<glob::Globres_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(608usize)) }
    }
    #[doc = "Global Result Register  Debug\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn globresd(&self) -> crate::common::Reg<glob::Globresd_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(736usize)) }
    }
    #[doc = "Global Test Enable Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn globte(&self) -> crate::common::Reg<glob::Globte_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(196usize)) }
    }
    #[doc = "Global Test Functions Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn globtf(&self) -> crate::common::Reg<glob::Globtf_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(192usize)) }
    }
}
pub mod glob {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Globbound_SPEC;
    impl crate::sealed::RegSpec for Globbound_SPEC {
        type DataType = u32;
    }
    #[doc = "Global Boundary Select Register\n resetvalue={Application Reset:0x0}"]
    pub type Globbound = crate::RegValueT<Globbound_SPEC>;

    impl Globbound {
        #[doc = "Boundary Value 0 for Limit Checking   BOUNDARY0. This value is compared against the conversion result."]
        #[inline(always)]
        pub fn boundary0(
            self,
        ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, Globbound_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xfff,1,0,u16, Globbound_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Boundary Value 1 for Limit Checking   BOUNDARY1. This value is compared against the conversion result."]
        #[inline(always)]
        pub fn boundary1(
            self,
        ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, Globbound_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xfff,1,0,u16, Globbound_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Globbound {
        #[inline(always)]
        fn default() -> Globbound {
            <crate::RegValueT<Globbound_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Globeflag_SPEC;
    impl crate::sealed::RegSpec for Globeflag_SPEC {
        type DataType = u32;
    }
    #[doc = "Global Event Flag Register\n resetvalue={Application Reset:0x0}"]
    pub type Globeflag = crate::RegValueT<Globeflag_SPEC>;

    impl Globeflag {
        #[doc = "Global Result Event   REVGLB"]
        #[inline(always)]
        pub fn revglb(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1,
            1,
            0,
            globeflag::Revglb,
            Globeflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                globeflag::Revglb,
                Globeflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Clear Global Result Event   REVGLBCLR"]
        #[inline(always)]
        pub fn revglbclr(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x1,
            1,
            0,
            globeflag::Revglbclr,
            Globeflag_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                24,
                0x1,
                1,
                0,
                globeflag::Revglbclr,
                Globeflag_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for Globeflag {
        #[inline(always)]
        fn default() -> Globeflag {
            <crate::RegValueT<Globeflag_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod globeflag {
        pub struct Revglb_SPEC;
        pub type Revglb = crate::EnumBitfieldStruct<u8, Revglb_SPEC>;
        impl Revglb {
            #[doc = "No result event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "New result was stored in register GLOBRES"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Revglbclr_SPEC;
        pub type Revglbclr = crate::EnumBitfieldStruct<u8, Revglbclr_SPEC>;
        impl Revglbclr {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the result event flag REVGLB"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GlobiclasSi_SPEC;
    impl crate::sealed::RegSpec for GlobiclasSi_SPEC {
        type DataType = u32;
    }
    #[doc = "Input Class Register 0  Global\n resetvalue={Application Reset:0x0}"]
    pub type GlobiclasSi = crate::RegValueT<GlobiclasSi_SPEC>;

    impl GlobiclasSi {
        #[doc = "Sample Time Control for Standard Conversions   STCS. Number of additional clock cycles to be added to the minimum sample        phase of 2 sample clock cycles  Coding and resulting sample time see CROSSREFERENCE . For conversions of external channels  the value from bitfield STCE can        be used."]
        #[inline(always)]
        pub fn stcs(
            self,
        ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, GlobiclasSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1f,1,0,u8, GlobiclasSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Analog Input Precharge Control for Standard Conversions   AIPS. Buffer must be enabled by BE  160    160 1  see GxANCFG  ."]
        #[inline(always)]
        pub fn aips(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x3,
            1,
            0,
            globiclassi::Aips,
            GlobiclasSi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                6,
                0x3,
                1,
                0,
                globiclassi::Aips,
                GlobiclasSi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Conversion Mode for Standard Conversions   CMS"]
        #[inline(always)]
        pub fn cms(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x3,
            1,
            0,
            globiclassi::Cms,
            GlobiclasSi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x3,
                1,
                0,
                globiclassi::Cms,
                GlobiclasSi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Spread Early Sample Point for Standard Conversions   SESPS"]
        #[inline(always)]
        pub fn sesps(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            globiclassi::Sesps,
            GlobiclasSi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                globiclassi::Sesps,
                GlobiclasSi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Sample Time Control for EMUX Conversions   STCE. Number of additional clock cycles to be added to the minimum sample        phase of 2 sample clock cycles  Coding and resulting sample time see CROSSREFERENCE . For conversions of standard channels  the value from bitfield STCS is        used."]
        #[inline(always)]
        pub fn stce(
            self,
        ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, GlobiclasSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1f,1,0,u8, GlobiclasSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Analog Input Precharge Control for EMUX Conversions   AIPE. Buffer must be enabled by BE  160    160 1  see GxANCFG  ."]
        #[inline(always)]
        pub fn aipe(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x3,
            1,
            0,
            globiclassi::Aipe,
            GlobiclasSi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                22,
                0x3,
                1,
                0,
                globiclassi::Aipe,
                GlobiclasSi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Conversion Mode for EMUX Conversions   CME"]
        #[inline(always)]
        pub fn cme(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x3,
            1,
            0,
            globiclassi::Cme,
            GlobiclasSi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0x3,
                1,
                0,
                globiclassi::Cme,
                GlobiclasSi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Spread Early Sample Point for EMUX Conversions   SESPE"]
        #[inline(always)]
        pub fn sespe(
            self,
        ) -> crate::common::RegisterField<
            26,
            0x1,
            1,
            0,
            globiclassi::Sespe,
            GlobiclasSi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                26,
                0x1,
                1,
                0,
                globiclassi::Sespe,
                GlobiclasSi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for GlobiclasSi {
        #[inline(always)]
        fn default() -> GlobiclasSi {
            <crate::RegValueT<GlobiclasSi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod globiclassi {
        pub struct Aips_SPEC;
        pub type Aips = crate::EnumBitfieldStruct<u8, Aips_SPEC>;
        impl Aips {
            #[doc = "No precharge"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Precharge for 8 clock cycles"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Precharge for 16 clock cycles"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Precharge for 32 clock cycles"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Cms_SPEC;
        pub type Cms = crate::EnumBitfieldStruct<u8, Cms_SPEC>;
        impl Cms {
            #[doc = "Standard conversion"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Noise reduction conversion level 1  1 additional conversion step"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Noise reduction conversion level 2  3 additional conversion steps"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Noise reduction conversion level 3  7 additional conversion steps"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Sesps_SPEC;
        pub type Sesps = crate::EnumBitfieldStruct<u8, Sesps_SPEC>;
        impl Sesps {
            #[doc = "Nominal sample timing"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Spread sample timing  end of sample phase is varied"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Aipe_SPEC;
        pub type Aipe = crate::EnumBitfieldStruct<u8, Aipe_SPEC>;
        impl Aipe {
            #[doc = "No precharge"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Precharge for 8 clock cycles"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Precharge for 16 clock cycles"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Precharge for 32 clock cycles"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Cme_SPEC;
        pub type Cme = crate::EnumBitfieldStruct<u8, Cme_SPEC>;
        impl Cme {
            #[doc = "Standard conversion"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Noise reduction conversion level 1  1 additional conversion step"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Noise reduction conversion level 2  3 additional conversion steps"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Noise reduction conversion level 3  7 additional conversion steps"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Sespe_SPEC;
        pub type Sespe = crate::EnumBitfieldStruct<u8, Sespe_SPEC>;
        impl Sespe {
            #[doc = "Nominal sample timing"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Spread sample timing  end of sample phase is varied"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Globrcr_SPEC;
    impl crate::sealed::RegSpec for Globrcr_SPEC {
        type DataType = u32;
    }
    #[doc = "Global Result Control Register\n resetvalue={Application Reset:0x0}"]
    pub type Globrcr = crate::RegValueT<Globrcr_SPEC>;

    impl Globrcr {
        #[doc = "Data Reduction Control   DRCTR. Defines how result values are stored accumulated in this register for        the final result. The data reduction counter DRC can be loaded from this        bitfield. Coding see CROSSREFERENCE Only        standard data reduction is available for the global result register         i.e. DMM is assumed as 00 ."]
        #[inline(always)]
        pub fn drctr(
            self,
        ) -> crate::common::RegisterField<
            16,
            0xf,
            1,
            0,
            globrcr::Drctr,
            Globrcr_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0xf,
                1,
                0,
                globrcr::Drctr,
                Globrcr_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Wait for Read Mode Enable   WFR. Refer also to  Wait for Read Mode ."]
        #[inline(always)]
        pub fn wfr(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x1,
            1,
            0,
            globrcr::Wfr,
            Globrcr_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0x1,
                1,
                0,
                globrcr::Wfr,
                Globrcr_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Service Request Generation Enable   SRGEN"]
        #[inline(always)]
        pub fn srgen(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            globrcr::Srgen,
            Globrcr_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                globrcr::Srgen,
                Globrcr_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for Globrcr {
        #[inline(always)]
        fn default() -> Globrcr {
            <crate::RegValueT<Globrcr_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod globrcr {
        pub struct Drctr_SPEC;
        pub type Drctr = crate::EnumBitfieldStruct<u8, Drctr_SPEC>;
        impl Drctr {
            #[doc = "Data reduction disabled"]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Wfr_SPEC;
        pub type Wfr = crate::EnumBitfieldStruct<u8, Wfr_SPEC>;
        impl Wfr {
            #[doc = "Overwrite mode"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Wait for read mode enabled for this register"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Srgen_SPEC;
        pub type Srgen = crate::EnumBitfieldStruct<u8, Srgen_SPEC>;
        impl Srgen {
            #[doc = "No service request"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Service request after a result event"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Globres_SPEC;
    impl crate::sealed::RegSpec for Globres_SPEC {
        type DataType = u32;
    }
    #[doc = "Global Result Register\n resetvalue={Application Reset:0x0}"]
    pub type Globres = crate::RegValueT<Globres_SPEC>;

    impl Globres {
        #[doc = "Result of most recent conversion   RESULT. The position of the result bits within this bitfield depends on the        configured operating mode. Refer to CROSSREFERENCE ."]
        #[inline(always)]
        pub fn result(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Globres_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, Globres_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Group Number   GNR. Indicates the group to which the channel number in bitfield CHNR refers."]
        #[inline(always)]
        pub fn gnr(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Globres_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, Globres_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Channel Number   CHNR. Indicates the channel number corresponding to the value in bitfield        RESULT."]
        #[inline(always)]
        pub fn chnr(
            self,
        ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, Globres_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<20,0x1f,1,0,u8, Globres_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "External Multiplexer Setting   EMUX. Indicates the setting of the external multiplexer  corresponding to the        value in bitfield RESULT."]
        #[inline(always)]
        pub fn emux(
            self,
        ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, Globres_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<25,0x7,1,0,u8, Globres_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Converted Request Source   CRS. Indicates the request source that as requested the conversion to which        the result value in bitfield RESULT belongs."]
        #[inline(always)]
        pub fn crs(
            self,
        ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, Globres_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<28,0x3,1,0,u8, Globres_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Valid Flag   VF. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf(
            self,
        ) -> crate::common::RegisterField<31, 0x1, 1, 0, globres::Vf, Globres_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                globres::Vf,
                Globres_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for Globres {
        #[inline(always)]
        fn default() -> Globres {
            <crate::RegValueT<Globres_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod globres {
        pub struct Vf_SPEC;
        pub type Vf = crate::EnumBitfieldStruct<u8, Vf_SPEC>;
        impl Vf {
            #[doc = "No new valid data available. Write access  No effect"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Bitfield RESULT contains valid data and has not yet been read. Write access  Clear this valid flag and the data reduction counter         overrides a hardware set action"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Globresd_SPEC;
    impl crate::sealed::RegSpec for Globresd_SPEC {
        type DataType = u32;
    }
    #[doc = "Global Result Register  Debug\n resetvalue={Application Reset:0x0}"]
    pub type Globresd = crate::RegValueT<Globresd_SPEC>;

    impl Globresd {
        #[doc = "Result of most recent conversion   RESULT. The position of the result bits within this bitfield depends on the        configured operating mode. Refer to CROSSREFERENCE ."]
        #[inline(always)]
        pub fn result(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Globresd_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, Globresd_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Group Number   GNR. Indicates the group to which the channel number in bitfield CHNR refers."]
        #[inline(always)]
        pub fn gnr(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Globresd_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, Globresd_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Channel Number   CHNR. Indicates the channel number corresponding to the value in bitfield        RESULT."]
        #[inline(always)]
        pub fn chnr(
            self,
        ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, Globresd_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<20,0x1f,1,0,u8, Globresd_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "External Multiplexer Setting   EMUX. Indicates the setting of the external multiplexer  corresponding to the        value in bitfield RESULT."]
        #[inline(always)]
        pub fn emux(
            self,
        ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, Globresd_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<25,0x7,1,0,u8, Globresd_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Converted Request Source   CRS. Indicates the request source that as requested the conversion to which        the result value in bitfield RESULT belongs."]
        #[inline(always)]
        pub fn crs(
            self,
        ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, Globresd_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<28,0x3,1,0,u8, Globresd_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Valid Flag   VF. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            globresd::Vf,
            Globresd_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                globresd::Vf,
                Globresd_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for Globresd {
        #[inline(always)]
        fn default() -> Globresd {
            <crate::RegValueT<Globresd_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod globresd {
        pub struct Vf_SPEC;
        pub type Vf = crate::EnumBitfieldStruct<u8, Vf_SPEC>;
        impl Vf {
            #[doc = "No new valid data available"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Bitfield RESULT contains valid data and has not yet been read"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Globte_SPEC;
    impl crate::sealed::RegSpec for Globte_SPEC {
        type DataType = u32;
    }
    #[doc = "Global Test Enable Register\n resetvalue={Application Reset:0x0}"]
    pub type Globte = crate::RegValueT<Globte_SPEC>;

    impl Globte {
        #[doc = "Test Function Enable  Primary Groups. Each bit of this bitfield is associated with the corresponding group. 0  No test functions for group x sequences 1  Test functions can be activated by group x sequences"]
        #[inline(always)]
        pub fn tfep(
            self,
        ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Globte_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xf,1,0,u8, Globte_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Test Function Enable  Secondary Groups. Each bit of this bitfield is associated with the corresponding group. 0  No test functions for group x sequences 1  Test functions can be activated by group x sequences"]
        #[inline(always)]
        pub fn tfes(
            self,
        ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Globte_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xf,1,0,u8, Globte_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Globte {
        #[inline(always)]
        fn default() -> Globte {
            <crate::RegValueT<Globte_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Globtf_SPEC;
    impl crate::sealed::RegSpec for Globtf_SPEC {
        type DataType = u32;
    }
    #[doc = "Global Test Functions Register\n resetvalue={Application Reset:0x0}"]
    pub type Globtf = crate::RegValueT<Globtf_SPEC>;

    impl Globtf {
        #[doc = "Conversion Diagnostics Channel   CDCH. Defines the channel number to be used for diagnostic conversions. Applies to MDPD  MDPU."]
        #[inline(always)]
        pub fn cdch(
            self,
        ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Globtf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xf,1,0,u8, Globtf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Conversion Diagnostics Group   CDGR. Defines the group number to be used for diagnostic conversions. Applies to all test functions of primary and secondary groups."]
        #[inline(always)]
        pub fn cdgr(
            self,
        ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Globtf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0xf,1,0,u8, Globtf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Converter Diagnostics Enable   CDEN"]
        #[inline(always)]
        pub fn cden(
            self,
        ) -> crate::common::RegisterField<8, 0x1, 1, 0, globtf::Cden, Globtf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x1,1,0,globtf::Cden, Globtf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Converter Diagnostics Pull Devices Select   CDSEL"]
        #[inline(always)]
        pub fn cdsel(
            self,
        ) -> crate::common::RegisterField<9, 0x3, 1, 0, globtf::Cdsel, Globtf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                9,
                0x3,
                1,
                0,
                globtf::Cdsel,
                Globtf_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Write Control for Conversion Diagnostics   CDWC"]
        #[inline(always)]
        pub fn cdwc(
            self,
        ) -> crate::common::RegisterField<15, 0x1, 1, 0, globtf::Cdwc, Globtf_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<15,0x1,1,0,globtf::Cdwc, Globtf_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Pull Down Diagnostics Enable   PDD. Channels with pull down diagnostics device are marked in the          product specific appendix."]
        #[inline(always)]
        pub fn pdd(
            self,
        ) -> crate::common::RegisterField<16, 0x1, 1, 0, globtf::Pdd, Globtf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1,1,0,globtf::Pdd, Globtf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Multiplexer Diagnostics Pull Down Devices Enable. Connecting combinations of pull up and or pull down devices generate        various loads for testing. Channels with pull down diagnostics device are marked in the          product specific appendix."]
        #[inline(always)]
        pub fn mdpd(
            self,
        ) -> crate::common::RegisterField<17, 0x1, 1, 0, globtf::Mdpd, Globtf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                17,
                0x1,
                1,
                0,
                globtf::Mdpd,
                Globtf_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Multiplexer Diagnostics Pull Up Devices Enable. Connecting combinations of pull up and or pull down devices generate various loads for testing. Channels with pull up diagnostics device are marked in the product specific appendix."]
        #[inline(always)]
        pub fn mdpu(
            self,
        ) -> crate::common::RegisterField<18, 0x1, 1, 0, globtf::Mdpu, Globtf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                18,
                0x1,
                1,
                0,
                globtf::Mdpu,
                Globtf_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Write Control for Multiplexer Diagnostics   MDWC"]
        #[inline(always)]
        pub fn mdwc(
            self,
        ) -> crate::common::RegisterField<23, 0x1, 1, 0, globtf::Mdwc, Globtf_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<23,0x1,1,0,globtf::Mdwc, Globtf_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for Globtf {
        #[inline(always)]
        fn default() -> Globtf {
            <crate::RegValueT<Globtf_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod globtf {
        pub struct Cden_SPEC;
        pub type Cden = crate::EnumBitfieldStruct<u8, Cden_SPEC>;
        impl Cden {
            #[doc = "All diagnostic pull devices are disconnected"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Diagnostic pull devices connected as selected by bitfield CDSEL"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cdsel_SPEC;
        pub type Cdsel = crate::EnumBitfieldStruct<u8, Cdsel_SPEC>;
        impl Cdsel {
            #[doc = "Connected to VDDM"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Connected to VSSM"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Connected to 1 2 VDDM"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Connected to 2 3rd VDDM"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Cdwc_SPEC;
        pub type Cdwc = crate::EnumBitfieldStruct<u8, Cdwc_SPEC>;
        impl Cdwc {
            #[doc = "No write access to parameters"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Bitfields CDSEL  CDEN  CDGR  CDCH can be written"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Pdd_SPEC;
        pub type Pdd = crate::EnumBitfieldStruct<u8, Pdd_SPEC>;
        impl Pdd {
            #[doc = "Disconnected"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "The pull down diagnostics device is active"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Mdpd_SPEC;
        pub type Mdpd = crate::EnumBitfieldStruct<u8, Mdpd_SPEC>;
        impl Mdpd {
            #[doc = "Disconnected"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "The respective pull down device is active"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Mdpu_SPEC;
        pub type Mdpu = crate::EnumBitfieldStruct<u8, Mdpu_SPEC>;
        impl Mdpu {
            #[doc = "Disconnected"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "The respective pull up device is active"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Mdwc_SPEC;
        pub type Mdwc = crate::EnumBitfieldStruct<u8, Mdwc_SPEC>;
        impl Mdwc {
            #[doc = "No write access to parameters"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Bitfields MDPD  MDPU  PDD can be written"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
}
#[doc = "G"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct G(pub(super) *mut u8);
unsafe impl core::marker::Send for G {}
unsafe impl core::marker::Sync for G {}
impl G {
    #[doc = "Alias Register  Group 0\n resetvalue={Application Reset:0x100}"]
    #[inline(always)]
    pub const fn gxalias(&self) -> crate::common::Reg<g::GxAlias_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(176usize)) }
    }
    #[doc = "Analog Fct. Config. Register  Group 0\n resetvalue={Application Reset:0x300004}"]
    #[inline(always)]
    pub const fn gxancfg(&self) -> crate::common::Reg<g::GxAncfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }
    #[doc = "Arbitration Config. Register  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxarbcfg(&self) -> crate::common::Reg<g::GxArbcfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }
    #[doc = "Arbitration Priority Register  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxarbpr(&self) -> crate::common::Reg<g::GxArbpr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }
    #[doc = "Boundary Select Register  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxbound(&self) -> crate::common::Reg<g::GxBound_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(184usize)) }
    }
    #[doc = "Channel Event Flag Clear Register  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxcefclr(&self) -> crate::common::Reg<g::GxCefclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(400usize)) }
    }
    #[doc = "Channel Event Flag Register  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxceflag(&self) -> crate::common::Reg<g::GxCeflag_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(384usize)) }
    }
    #[doc = "Group 0  Channel 0 Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxchctry(&self) -> [crate::common::Reg<g::GxChctRy_SPEC, crate::common::RW>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x200usize + 0x3cusize)),
            ]
        }
    }
    #[doc = "Ext. Multiplexer Channel Select Reg.  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxemuxcs(&self) -> crate::common::Reg<g::GxEmuxcs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(500usize)) }
    }
    #[doc = "External Multiplexer Control Reg.  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxemuxctr(&self) -> crate::common::Reg<g::GxEmuxctr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(496usize)) }
    }
    #[doc = "Input Class Register 0  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxiclassi(&self) -> [crate::common::Reg<g::GxIclasSi_SPEC, crate::common::RW>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0xa0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0xa0usize + 0x4usize)),
            ]
        }
    }
    #[doc = "Group 0 Result Control Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxrcry(&self) -> [crate::common::Reg<g::GxRcRy_SPEC, crate::common::RW>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x280usize + 0x3cusize)),
            ]
        }
    }
    #[doc = "Result Event Flag Clear Register  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxrefclr(&self) -> crate::common::Reg<g::GxRefclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(404usize)) }
    }
    #[doc = "Result Event Flag Register  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxreflag(&self) -> crate::common::Reg<g::GxReflag_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(388usize)) }
    }
    #[doc = "Group 0 Result Reg. 0  Debug\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxresdy(&self) -> [crate::common::Reg<g::GxResDy_SPEC, crate::common::R>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x380usize + 0x3cusize)),
            ]
        }
    }
    #[doc = "Group 0 Result Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxresy(&self) -> [crate::common::Reg<g::GxReSy_SPEC, crate::common::RW>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x300usize + 0x3cusize)),
            ]
        }
    }
    #[doc = "Source Event Flag Clear Reg.  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxsefclr(&self) -> crate::common::Reg<g::GxSefclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(408usize)) }
    }
    #[doc = "Source Event Flag Register  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxseflag(&self) -> crate::common::Reg<g::GxSeflag_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(392usize)) }
    }
    #[doc = "Service Request Software Activation Trigger  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxsract(&self) -> crate::common::Reg<g::GxSract_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(456usize)) }
    }
    #[doc = "Synchronization Control Register  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxsynctr(&self) -> crate::common::Reg<g::GxSynctr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(192usize)) }
    }
    #[doc = "Test Register  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxtest(&self) -> crate::common::Reg<g::GxTest_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1016usize)) }
    }
    #[doc = "Trigger Control Register  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxtrctr(&self) -> crate::common::Reg<g::GxTrctr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Valid Flag Register  Group 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn gxvfr(&self) -> crate::common::Reg<g::GxVfr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(504usize)) }
    }
    #[doc = "Q"]
    #[inline(always)]
    pub fn q(self) -> [g::Q; 3] {
        unsafe {
            [
                g::Q(self.0.add(0x100usize + 0x0usize)),
                g::Q(self.0.add(0x100usize + 0x20usize)),
                g::Q(self.0.add(0x100usize + 0x40usize)),
            ]
        }
    }
}
pub mod g {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxAlias_SPEC;
    impl crate::sealed::RegSpec for GxAlias_SPEC {
        type DataType = u32;
    }
    #[doc = "Alias Register  Group 0\n resetvalue={Application Reset:0x100}"]
    pub type GxAlias = crate::RegValueT<GxAlias_SPEC>;

    impl GxAlias {
        #[doc = "Alias Value for CH0 Conversion Requests   ALIAS0. Indicates the channel that is converted instead of channel CH0. The        conversion is done with the settings defined for channel CH0."]
        #[inline(always)]
        pub fn alias0(
            self,
        ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, GxAlias_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1f,1,0,u8, GxAlias_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Alias Value for CH1 Conversion Requests   ALIAS1. Indicates the channel that is converted instead of channel CH1. The        conversion is done with the settings defined for channel CH1."]
        #[inline(always)]
        pub fn alias1(
            self,
        ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, GxAlias_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x1f,1,0,u8, GxAlias_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GxAlias {
        #[inline(always)]
        fn default() -> GxAlias {
            <crate::RegValueT<GxAlias_SPEC> as RegisterValue<_>>::new(256)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxAncfg_SPEC;
    impl crate::sealed::RegSpec for GxAncfg_SPEC {
        type DataType = u32;
    }
    #[doc = "Analog Fct. Config. Register  Group 0\n resetvalue={Application Reset:0x300004}"]
    pub type GxAncfg = crate::RegValueT<GxAncfg_SPEC>;

    impl GxAncfg {
        #[doc = "Idle Precharge Enable   IPE"]
        #[inline(always)]
        pub fn ipe(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, gxancfg::Ipe, GxAncfg_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                gxancfg::Ipe,
                GxAncfg_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Input Buffer Enable   BE"]
        #[inline(always)]
        pub fn be(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, gxancfg::Be, GxAncfg_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0x1,1,0,gxancfg::Be, GxAncfg_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Reference Precharge Enable   RPE. Enabled after reset."]
        #[inline(always)]
        pub fn rpe(
            self,
        ) -> crate::common::RegisterField<2, 0x1, 1, 0, gxancfg::Rpe, GxAncfg_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                gxancfg::Rpe,
                GxAncfg_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Reference Precharge Control   RPC"]
        #[inline(always)]
        pub fn rpc(
            self,
        ) -> crate::common::RegisterField<3, 0x1, 1, 0, gxancfg::Rpc, GxAncfg_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                gxancfg::Rpc,
                GxAncfg_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Calibration Sample Time Control   CALSTC"]
        #[inline(always)]
        pub fn calstc(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x3,
            1,
            0,
            gxancfg::Calstc,
            GxAncfg_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x3,
                1,
                0,
                gxancfg::Calstc,
                GxAncfg_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Disable Post Calibration   DPCAL"]
        #[inline(always)]
        pub fn dpcal(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x1,
            1,
            0,
            gxancfg::Dpcal,
            GxAncfg_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                6,
                0x1,
                1,
                0,
                gxancfg::Dpcal,
                GxAncfg_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Analog Clock Synchronization Delay   ACSD. Defines the delay of the analog clock in clocks after the sync signal. Do not exceed the current DIVA setting for the clock delay. Valid only          if the phase synchronizer is selected  USC  160    160  0"]
        #[inline(always)]
        pub fn acsd(
            self,
        ) -> crate::common::RegisterField<
            16,
            0x7,
            1,
            0,
            gxancfg::Acsd,
            GxAncfg_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0x7,
                1,
                0,
                gxancfg::Acsd,
                GxAncfg_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Sample Synchronization Enable   SSE. See section CROSSREFERENCE ."]
        #[inline(always)]
        pub fn sse(
            self,
        ) -> crate::common::RegisterField<
            19,
            0x1,
            1,
            0,
            gxancfg::Sse,
            GxAncfg_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                19,
                0x1,
                1,
                0,
                gxancfg::Sse,
                GxAncfg_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Divider Factor for the Analog Internal Clock   DIVA. Defines the frequency of the analog converter clock f ADCI  base clock for conversion steps   derived from the peripheral clock  f ADCI   f ADC   CP."]
        #[inline(always)]
        pub fn diva(
            self,
        ) -> crate::common::RegisterField<
            20,
            0x1f,
            1,
            0,
            gxancfg::Diva,
            GxAncfg_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0x1f,
                1,
                0,
                gxancfg::Diva,
                GxAncfg_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Double Clock for the MSB Conversion   DCMSB. Selects an additional clock cycle for the conversion step of the MSB."]
        #[inline(always)]
        pub fn dcmsb(
            self,
        ) -> crate::common::RegisterField<
            25,
            0x1,
            1,
            0,
            gxancfg::Dcmsb,
            GxAncfg_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                25,
                0x1,
                1,
                0,
                gxancfg::Dcmsb,
                GxAncfg_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for GxAncfg {
        #[inline(always)]
        fn default() -> GxAncfg {
            <crate::RegValueT<GxAncfg_SPEC> as RegisterValue<_>>::new(3145732)
        }
    }
    pub mod gxancfg {
        pub struct Ipe_SPEC;
        pub type Ipe = crate::EnumBitfieldStruct<u8, Ipe_SPEC>;
        impl Ipe {
            #[doc = "No precharge  the sampling capacitor keeps the current charge"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "The sampling capacitor is precharged to approx. half the reference when idle"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Be_SPEC;
        pub type Be = crate::EnumBitfieldStruct<u8, Be_SPEC>;
        impl Be {
            #[doc = "Input buffer off  input buffering is not possible. Make sure AIPS AIPE   00 B ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Input buffer enabled  select buffering time via bitfields AIPS AIPE in register GxICLASS0 etc."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rpe_SPEC;
        pub type Rpe = crate::EnumBitfieldStruct<u8, Rpe_SPEC>;
        impl Rpe {
            #[doc = "No reference precharge. Only use V AREF   V AGND for the conversion."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Precharge enabled. 1 Use V DDM   V SSM for precharging and V AREF   V AGND for the final adjustment during a conversion."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rpc_SPEC;
        pub type Rpc = crate::EnumBitfieldStruct<u8, Rpc_SPEC>;
        impl Rpc {
            #[doc = "Precharge the reference input for 1 clock phase"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Precharge the reference input for 1 clock period  2 phases"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Calstc_SPEC;
        pub type Calstc = crate::EnumBitfieldStruct<u8, Calstc_SPEC>;
        impl Calstc {
            #[doc = "00 2   215  t ADCI"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 4   215  t ADCI"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 6   215  t ADCI"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 8   215  t ADCI"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Dpcal_SPEC;
        pub type Dpcal = crate::EnumBitfieldStruct<u8, Dpcal_SPEC>;
        impl Dpcal {
            #[doc = "Automatic post calibration after each conversion of group x"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "No post calibration"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Acsd_SPEC;
        pub type Acsd = crate::EnumBitfieldStruct<u8, Acsd_SPEC>;
        impl Acsd {
            #[doc = "0  no delay"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 clock cycle delay"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Sse_SPEC;
        pub type Sse = crate::EnumBitfieldStruct<u8, Sse_SPEC>;
        impl Sse {
            #[doc = "No synchronization"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Sample timing is synchronized. Recommended for operation of several ADC groups."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Diva_SPEC;
        pub type Diva = crate::EnumBitfieldStruct<u8, Diva_SPEC>;
        impl Diva {
            #[doc = "CP   2  max. frequency"]
            pub const CONST_000: Self = Self::new(0);
            #[doc = "CP   2"]
            pub const CONST_011: Self = Self::new(1);
        }
        pub struct Dcmsb_SPEC;
        pub type Dcmsb = crate::EnumBitfieldStruct<u8, Dcmsb_SPEC>;
        impl Dcmsb {
            #[doc = "1 clock cycle for the MSB  standard"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "2 clock cycles for the MSB  test only . Not used for standard applications. Keep DCMSB   0."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxArbcfg_SPEC;
    impl crate::sealed::RegSpec for GxArbcfg_SPEC {
        type DataType = u32;
    }
    #[doc = "Arbitration Config. Register  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxArbcfg = crate::RegValueT<GxArbcfg_SPEC>;

    impl GxArbcfg {
        #[doc = "Analog Converter Control   ANONC. Defines the value of bitfield ANONS in a stand alone converter or a        converter in master mode. Coding see ANONS or CROSSREFERENCE ."]
        #[inline(always)]
        pub fn anonc(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, GxArbcfg_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3,1,0,u8, GxArbcfg_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Analog Converter Control Status   ANONS. Defined by bitfield ANONC in a stand alone kernel or a kernel in master        mode. In slave mode  this bitfield is defined by bitfield ANONC of the        respective master kernel. See also CROSSREFERENCE ."]
        #[inline(always)]
        pub fn anons(
            self,
        ) -> crate::common::RegisterField<
            16,
            0x3,
            1,
            0,
            gxarbcfg::Anons,
            GxArbcfg_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                16,
                0x3,
                1,
                0,
                gxarbcfg::Anons,
                GxArbcfg_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Currently Converted Request Source   CSRC. Indicates the arbitration slot number of the current  BUSY  160    160 1  or of        the last  BUSY  160    160 0  conversion. This bitfield is updated when a        conversion is started."]
        #[inline(always)]
        pub fn csrc(
            self,
        ) -> crate::common::RegisterField<
            18,
            0x3,
            1,
            0,
            gxarbcfg::Csrc,
            GxArbcfg_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                18,
                0x3,
                1,
                0,
                gxarbcfg::Csrc,
                GxArbcfg_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Channel Number   CHNR. Indicates the current or last converted analog input channel. This        bitfield is updated when a conversion is started."]
        #[inline(always)]
        pub fn chnr(
            self,
        ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, GxArbcfg_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<20,0x1f,1,0,u8, GxArbcfg_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Synchronous Conversion Running   SYNRUN. Indicates that a synchronized    parallel  conversion is currently        running."]
        #[inline(always)]
        pub fn synrun(
            self,
        ) -> crate::common::RegisterField<
            25,
            0x1,
            1,
            0,
            gxarbcfg::Synrun,
            GxArbcfg_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                25,
                0x1,
                1,
                0,
                gxarbcfg::Synrun,
                GxArbcfg_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Start Up Calibration Active Indication   CAL. Indicates the start up calibration phase of the corresponding analog        converter. Start conversions only after the start up calibration phase is          complete  because a start up calibration will abort a running          conversion."]
        #[inline(always)]
        pub fn cal(
            self,
        ) -> crate::common::RegisterField<
            28,
            0x1,
            1,
            0,
            gxarbcfg::Cal,
            GxArbcfg_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                28,
                0x1,
                1,
                0,
                gxarbcfg::Cal,
                GxArbcfg_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Converter Busy Flag   BUSY"]
        #[inline(always)]
        pub fn busy(
            self,
        ) -> crate::common::RegisterField<
            30,
            0x1,
            1,
            0,
            gxarbcfg::Busy,
            GxArbcfg_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                30,
                0x1,
                1,
                0,
                gxarbcfg::Busy,
                GxArbcfg_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Sample Phase Flag   SAMPLE"]
        #[inline(always)]
        pub fn sample(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            gxarbcfg::Sample,
            GxArbcfg_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                gxarbcfg::Sample,
                GxArbcfg_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for GxArbcfg {
        #[inline(always)]
        fn default() -> GxArbcfg {
            <crate::RegValueT<GxArbcfg_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod gxarbcfg {
        pub struct Anons_SPEC;
        pub type Anons = crate::EnumBitfieldStruct<u8, Anons_SPEC>;
        impl Anons {
            #[doc = "Analog converter off"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Slow standby mode"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Fast standby mode"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Normal operation  permanently on"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Csrc_SPEC;
        pub type Csrc = crate::EnumBitfieldStruct<u8, Csrc_SPEC>;
        impl Csrc {
            #[doc = "Current last conversion for request source 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Current last conversion for request source 1"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Current last conversion for request source 2"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Current last conversion for synchronization request  slave converter"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Synrun_SPEC;
        pub type Synrun = crate::EnumBitfieldStruct<u8, Synrun_SPEC>;
        impl Synrun {
            #[doc = "Normal conversion or no conversion running"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "A synchronized conversion is running.  cannot be cancelled by higher priority requests"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cal_SPEC;
        pub type Cal = crate::EnumBitfieldStruct<u8, Cal_SPEC>;
        impl Cal {
            #[doc = "Completed or not yet started"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Start up calibration phase is active.  set one clock cycle after setting bit SUCAL"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Busy_SPEC;
        pub type Busy = crate::EnumBitfieldStruct<u8, Busy_SPEC>;
        impl Busy {
            #[doc = "Not busy"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Converter is busy with a conversion.  set one clock cycle after conversion start"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Sample_SPEC;
        pub type Sample = crate::EnumBitfieldStruct<u8, Sample_SPEC>;
        impl Sample {
            #[doc = "Converting or idle"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Input signal is currently sampled"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxArbpr_SPEC;
    impl crate::sealed::RegSpec for GxArbpr_SPEC {
        type DataType = u32;
    }
    #[doc = "Arbitration Priority Register  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxArbpr = crate::RegValueT<GxArbpr_SPEC>;

    impl GxArbpr {
        #[doc = "Priority of Request Source i. Arbitration priority of request source i  at input i"]
        #[inline(always)]
        pub fn prio0(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, GxArbpr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3,1,0,u8, GxArbpr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Conversion Start Mode of Request Source 2"]
        #[inline(always)]
        pub fn csm0(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            gxarbpr::Csm0,
            GxArbpr_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                gxarbpr::Csm0,
                GxArbpr_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Priority of Request Source i. Arbitration priority of request source i  at input i"]
        #[inline(always)]
        pub fn prio1(
            self,
        ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, GxArbpr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x3,1,0,u8, GxArbpr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Conversion Start Mode of Request Source 2"]
        #[inline(always)]
        pub fn csm1(
            self,
        ) -> crate::common::RegisterField<
            7,
            0x1,
            1,
            0,
            gxarbpr::Csm1,
            GxArbpr_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                gxarbpr::Csm1,
                GxArbpr_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Priority of Request Source i. Arbitration priority of request source i  at input i"]
        #[inline(always)]
        pub fn prio2(
            self,
        ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, GxArbpr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x3,1,0,u8, GxArbpr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Conversion Start Mode of Request Source 2"]
        #[inline(always)]
        pub fn csm2(
            self,
        ) -> crate::common::RegisterField<
            11,
            0x1,
            1,
            0,
            gxarbpr::Csm2,
            GxArbpr_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                gxarbpr::Csm2,
                GxArbpr_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Arbitration Source Input 2 Enable. Enables the associated arbitration source input of the arbiter. The        request source bits are not modified by write actions to ASENi."]
        #[inline(always)]
        pub fn asen0(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x1,
            1,
            0,
            gxarbpr::Asen0,
            GxArbpr_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0x1,
                1,
                0,
                gxarbpr::Asen0,
                GxArbpr_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Arbitration Source Input 2 Enable. Enables the associated arbitration source input of the arbiter. The        request source bits are not modified by write actions to ASENi."]
        #[inline(always)]
        pub fn asen1(
            self,
        ) -> crate::common::RegisterField<
            25,
            0x1,
            1,
            0,
            gxarbpr::Asen1,
            GxArbpr_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                25,
                0x1,
                1,
                0,
                gxarbpr::Asen1,
                GxArbpr_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Arbitration Source Input 2 Enable. Enables the associated arbitration source input of the arbiter. The        request source bits are not modified by write actions to ASENi."]
        #[inline(always)]
        pub fn asen2(
            self,
        ) -> crate::common::RegisterField<
            26,
            0x1,
            1,
            0,
            gxarbpr::Asen2,
            GxArbpr_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                26,
                0x1,
                1,
                0,
                gxarbpr::Asen2,
                GxArbpr_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for GxArbpr {
        #[inline(always)]
        fn default() -> GxArbpr {
            <crate::RegValueT<GxArbpr_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod gxarbpr {
        pub struct Csm0_SPEC;
        pub type Csm0 = crate::EnumBitfieldStruct<u8, Csm0_SPEC>;
        impl Csm0 {
            #[doc = "Wait for start mode"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Cancel inject repeat mode. i.e. this source can cancel conversions of other sources."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Csm1_SPEC;
        pub type Csm1 = crate::EnumBitfieldStruct<u8, Csm1_SPEC>;
        impl Csm1 {
            #[doc = "Wait for start mode"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Cancel inject repeat mode. i.e. this source can cancel conversions of other sources."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Csm2_SPEC;
        pub type Csm2 = crate::EnumBitfieldStruct<u8, Csm2_SPEC>;
        impl Csm2 {
            #[doc = "Wait for start mode"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Cancel inject repeat mode. i.e. this source can cancel conversions of other sources."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Asen0_SPEC;
        pub type Asen0 = crate::EnumBitfieldStruct<u8, Asen0_SPEC>;
        impl Asen0 {
            #[doc = "The arbitration source input is disabled.. Pending conversion requests from the associated request source are        disregarded."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "The arbitration source input is enabled.. Pending conversion requests from the associated request source are        arbitrated."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Asen1_SPEC;
        pub type Asen1 = crate::EnumBitfieldStruct<u8, Asen1_SPEC>;
        impl Asen1 {
            #[doc = "The arbitration source input is disabled.. Pending conversion requests from the associated request source are        disregarded."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "The arbitration source input is enabled.. Pending conversion requests from the associated request source are        arbitrated."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Asen2_SPEC;
        pub type Asen2 = crate::EnumBitfieldStruct<u8, Asen2_SPEC>;
        impl Asen2 {
            #[doc = "The arbitration source input is disabled.. Pending conversion requests from the associated request source are        disregarded."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "The arbitration source input is enabled.. Pending conversion requests from the associated request source are        arbitrated."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxBound_SPEC;
    impl crate::sealed::RegSpec for GxBound_SPEC {
        type DataType = u32;
    }
    #[doc = "Boundary Select Register  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxBound = crate::RegValueT<GxBound_SPEC>;

    impl GxBound {
        #[doc = "Boundary Value 0 for Limit Checking   BOUNDARY0. This value is compared against the conversion result."]
        #[inline(always)]
        pub fn boundary0(
            self,
        ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, GxBound_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xfff,1,0,u16, GxBound_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Boundary Value 1 for Limit Checking   BOUNDARY1. This value is compared against the conversion result."]
        #[inline(always)]
        pub fn boundary1(
            self,
        ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, GxBound_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xfff,1,0,u16, GxBound_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GxBound {
        #[inline(always)]
        fn default() -> GxBound {
            <crate::RegValueT<GxBound_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxCefclr_SPEC;
    impl crate::sealed::RegSpec for GxCefclr_SPEC {
        type DataType = u32;
    }
    #[doc = "Channel Event Flag Clear Register  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxCefclr = crate::RegValueT<GxCefclr_SPEC>;

    impl GxCefclr {
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev0(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            gxcefclr::Cev0,
            GxCefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                gxcefclr::Cev0,
                GxCefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev1(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            gxcefclr::Cev1,
            GxCefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                gxcefclr::Cev1,
                GxCefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev2(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            gxcefclr::Cev2,
            GxCefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                gxcefclr::Cev2,
                GxCefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev3(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            gxcefclr::Cev3,
            GxCefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                gxcefclr::Cev3,
                GxCefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev4(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            gxcefclr::Cev4,
            GxCefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                gxcefclr::Cev4,
                GxCefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev5(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            gxcefclr::Cev5,
            GxCefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                gxcefclr::Cev5,
                GxCefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev6(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x1,
            1,
            0,
            gxcefclr::Cev6,
            GxCefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                6,
                0x1,
                1,
                0,
                gxcefclr::Cev6,
                GxCefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev7(
            self,
        ) -> crate::common::RegisterField<
            7,
            0x1,
            1,
            0,
            gxcefclr::Cev7,
            GxCefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                gxcefclr::Cev7,
                GxCefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev8(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1,
            1,
            0,
            gxcefclr::Cev8,
            GxCefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                gxcefclr::Cev8,
                GxCefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev9(
            self,
        ) -> crate::common::RegisterField<
            9,
            0x1,
            1,
            0,
            gxcefclr::Cev9,
            GxCefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                9,
                0x1,
                1,
                0,
                gxcefclr::Cev9,
                GxCefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev10(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            gxcefclr::Cev10,
            GxCefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                gxcefclr::Cev10,
                GxCefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev11(
            self,
        ) -> crate::common::RegisterField<
            11,
            0x1,
            1,
            0,
            gxcefclr::Cev11,
            GxCefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                gxcefclr::Cev11,
                GxCefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev12(
            self,
        ) -> crate::common::RegisterField<
            12,
            0x1,
            1,
            0,
            gxcefclr::Cev12,
            GxCefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                12,
                0x1,
                1,
                0,
                gxcefclr::Cev12,
                GxCefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev13(
            self,
        ) -> crate::common::RegisterField<
            13,
            0x1,
            1,
            0,
            gxcefclr::Cev13,
            GxCefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                13,
                0x1,
                1,
                0,
                gxcefclr::Cev13,
                GxCefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev14(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x1,
            1,
            0,
            gxcefclr::Cev14,
            GxCefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                gxcefclr::Cev14,
                GxCefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Channel Event for Channel 15   CEV15"]
        #[inline(always)]
        pub fn cev15(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            gxcefclr::Cev15,
            GxCefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                gxcefclr::Cev15,
                GxCefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for GxCefclr {
        #[inline(always)]
        fn default() -> GxCefclr {
            <crate::RegValueT<GxCefclr_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod gxcefclr {
        pub struct Cev0_SPEC;
        pub type Cev0 = crate::EnumBitfieldStruct<u8, Cev0_SPEC>;
        impl Cev0 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the channel event flag in GxCEFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev1_SPEC;
        pub type Cev1 = crate::EnumBitfieldStruct<u8, Cev1_SPEC>;
        impl Cev1 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the channel event flag in GxCEFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev2_SPEC;
        pub type Cev2 = crate::EnumBitfieldStruct<u8, Cev2_SPEC>;
        impl Cev2 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the channel event flag in GxCEFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev3_SPEC;
        pub type Cev3 = crate::EnumBitfieldStruct<u8, Cev3_SPEC>;
        impl Cev3 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the channel event flag in GxCEFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev4_SPEC;
        pub type Cev4 = crate::EnumBitfieldStruct<u8, Cev4_SPEC>;
        impl Cev4 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the channel event flag in GxCEFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev5_SPEC;
        pub type Cev5 = crate::EnumBitfieldStruct<u8, Cev5_SPEC>;
        impl Cev5 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the channel event flag in GxCEFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev6_SPEC;
        pub type Cev6 = crate::EnumBitfieldStruct<u8, Cev6_SPEC>;
        impl Cev6 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the channel event flag in GxCEFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev7_SPEC;
        pub type Cev7 = crate::EnumBitfieldStruct<u8, Cev7_SPEC>;
        impl Cev7 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the channel event flag in GxCEFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev8_SPEC;
        pub type Cev8 = crate::EnumBitfieldStruct<u8, Cev8_SPEC>;
        impl Cev8 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the channel event flag in GxCEFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev9_SPEC;
        pub type Cev9 = crate::EnumBitfieldStruct<u8, Cev9_SPEC>;
        impl Cev9 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the channel event flag in GxCEFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev10_SPEC;
        pub type Cev10 = crate::EnumBitfieldStruct<u8, Cev10_SPEC>;
        impl Cev10 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the channel event flag in GxCEFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev11_SPEC;
        pub type Cev11 = crate::EnumBitfieldStruct<u8, Cev11_SPEC>;
        impl Cev11 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the channel event flag in GxCEFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev12_SPEC;
        pub type Cev12 = crate::EnumBitfieldStruct<u8, Cev12_SPEC>;
        impl Cev12 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the channel event flag in GxCEFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev13_SPEC;
        pub type Cev13 = crate::EnumBitfieldStruct<u8, Cev13_SPEC>;
        impl Cev13 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the channel event flag in GxCEFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev14_SPEC;
        pub type Cev14 = crate::EnumBitfieldStruct<u8, Cev14_SPEC>;
        impl Cev14 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the channel event flag in GxCEFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev15_SPEC;
        pub type Cev15 = crate::EnumBitfieldStruct<u8, Cev15_SPEC>;
        impl Cev15 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the channel event flag in GxCEFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxCeflag_SPEC;
    impl crate::sealed::RegSpec for GxCeflag_SPEC {
        type DataType = u32;
    }
    #[doc = "Channel Event Flag Register  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxCeflag = crate::RegValueT<GxCeflag_SPEC>;

    impl GxCeflag {
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev0(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            gxceflag::Cev0,
            GxCeflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                gxceflag::Cev0,
                GxCeflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev1(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            gxceflag::Cev1,
            GxCeflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                gxceflag::Cev1,
                GxCeflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev2(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            gxceflag::Cev2,
            GxCeflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                gxceflag::Cev2,
                GxCeflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev3(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            gxceflag::Cev3,
            GxCeflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                gxceflag::Cev3,
                GxCeflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev4(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            gxceflag::Cev4,
            GxCeflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                gxceflag::Cev4,
                GxCeflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev5(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            gxceflag::Cev5,
            GxCeflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                gxceflag::Cev5,
                GxCeflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev6(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x1,
            1,
            0,
            gxceflag::Cev6,
            GxCeflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                6,
                0x1,
                1,
                0,
                gxceflag::Cev6,
                GxCeflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev7(
            self,
        ) -> crate::common::RegisterField<
            7,
            0x1,
            1,
            0,
            gxceflag::Cev7,
            GxCeflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                gxceflag::Cev7,
                GxCeflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev8(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1,
            1,
            0,
            gxceflag::Cev8,
            GxCeflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                gxceflag::Cev8,
                GxCeflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev9(
            self,
        ) -> crate::common::RegisterField<
            9,
            0x1,
            1,
            0,
            gxceflag::Cev9,
            GxCeflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                9,
                0x1,
                1,
                0,
                gxceflag::Cev9,
                GxCeflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev10(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            gxceflag::Cev10,
            GxCeflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                gxceflag::Cev10,
                GxCeflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev11(
            self,
        ) -> crate::common::RegisterField<
            11,
            0x1,
            1,
            0,
            gxceflag::Cev11,
            GxCeflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                gxceflag::Cev11,
                GxCeflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev12(
            self,
        ) -> crate::common::RegisterField<
            12,
            0x1,
            1,
            0,
            gxceflag::Cev12,
            GxCeflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                12,
                0x1,
                1,
                0,
                gxceflag::Cev12,
                GxCeflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev13(
            self,
        ) -> crate::common::RegisterField<
            13,
            0x1,
            1,
            0,
            gxceflag::Cev13,
            GxCeflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                13,
                0x1,
                1,
                0,
                gxceflag::Cev13,
                GxCeflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev14(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x1,
            1,
            0,
            gxceflag::Cev14,
            GxCeflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                gxceflag::Cev14,
                GxCeflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Channel Event for Channel 15"]
        #[inline(always)]
        pub fn cev15(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            gxceflag::Cev15,
            GxCeflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                gxceflag::Cev15,
                GxCeflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for GxCeflag {
        #[inline(always)]
        fn default() -> GxCeflag {
            <crate::RegValueT<GxCeflag_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod gxceflag {
        pub struct Cev0_SPEC;
        pub type Cev0 = crate::EnumBitfieldStruct<u8, Cev0_SPEC>;
        impl Cev0 {
            #[doc = "0 No channel        event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 A channel event        has occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev1_SPEC;
        pub type Cev1 = crate::EnumBitfieldStruct<u8, Cev1_SPEC>;
        impl Cev1 {
            #[doc = "0 No channel        event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 A channel event        has occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev2_SPEC;
        pub type Cev2 = crate::EnumBitfieldStruct<u8, Cev2_SPEC>;
        impl Cev2 {
            #[doc = "0 No channel        event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 A channel event        has occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev3_SPEC;
        pub type Cev3 = crate::EnumBitfieldStruct<u8, Cev3_SPEC>;
        impl Cev3 {
            #[doc = "0 No channel        event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 A channel event        has occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev4_SPEC;
        pub type Cev4 = crate::EnumBitfieldStruct<u8, Cev4_SPEC>;
        impl Cev4 {
            #[doc = "0 No channel        event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 A channel event        has occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev5_SPEC;
        pub type Cev5 = crate::EnumBitfieldStruct<u8, Cev5_SPEC>;
        impl Cev5 {
            #[doc = "0 No channel        event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 A channel event        has occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev6_SPEC;
        pub type Cev6 = crate::EnumBitfieldStruct<u8, Cev6_SPEC>;
        impl Cev6 {
            #[doc = "0 No channel        event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 A channel event        has occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev7_SPEC;
        pub type Cev7 = crate::EnumBitfieldStruct<u8, Cev7_SPEC>;
        impl Cev7 {
            #[doc = "0 No channel        event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 A channel event        has occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev8_SPEC;
        pub type Cev8 = crate::EnumBitfieldStruct<u8, Cev8_SPEC>;
        impl Cev8 {
            #[doc = "0 No channel        event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 A channel event        has occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev9_SPEC;
        pub type Cev9 = crate::EnumBitfieldStruct<u8, Cev9_SPEC>;
        impl Cev9 {
            #[doc = "0 No channel        event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 A channel event        has occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev10_SPEC;
        pub type Cev10 = crate::EnumBitfieldStruct<u8, Cev10_SPEC>;
        impl Cev10 {
            #[doc = "0 No channel        event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 A channel event        has occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev11_SPEC;
        pub type Cev11 = crate::EnumBitfieldStruct<u8, Cev11_SPEC>;
        impl Cev11 {
            #[doc = "0 No channel        event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 A channel event        has occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev12_SPEC;
        pub type Cev12 = crate::EnumBitfieldStruct<u8, Cev12_SPEC>;
        impl Cev12 {
            #[doc = "0 No channel        event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 A channel event        has occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev13_SPEC;
        pub type Cev13 = crate::EnumBitfieldStruct<u8, Cev13_SPEC>;
        impl Cev13 {
            #[doc = "0 No channel        event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 A channel event        has occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev14_SPEC;
        pub type Cev14 = crate::EnumBitfieldStruct<u8, Cev14_SPEC>;
        impl Cev14 {
            #[doc = "0 No channel        event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 A channel event        has occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cev15_SPEC;
        pub type Cev15 = crate::EnumBitfieldStruct<u8, Cev15_SPEC>;
        impl Cev15 {
            #[doc = "0 No channel        event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 A channel event        has occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxChctRy_SPEC;
    impl crate::sealed::RegSpec for GxChctRy_SPEC {
        type DataType = u32;
    }
    #[doc = "Group 0  Channel 0 Control Register\n resetvalue={Application Reset:0x0}"]
    pub type GxChctRy = crate::RegValueT<GxChctRy_SPEC>;

    impl GxChctRy {
        #[doc = "Input Class Select   ICLSEL"]
        #[inline(always)]
        pub fn iclsel(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x3,
            1,
            0,
            gxchctry::Iclsel,
            GxChctRy_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x3,
                1,
                0,
                gxchctry::Iclsel,
                GxChctRy_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Lower Boundary Select   BNDSELL"]
        #[inline(always)]
        pub fn bndsell(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x3,
            1,
            0,
            gxchctry::Bndsell,
            GxChctRy_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x3,
                1,
                0,
                gxchctry::Bndsell,
                GxChctRy_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Upper Boundary Select   BNDSELU"]
        #[inline(always)]
        pub fn bndselu(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x3,
            1,
            0,
            gxchctry::Bndselu,
            GxChctRy_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                6,
                0x3,
                1,
                0,
                gxchctry::Bndselu,
                GxChctRy_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Channel Event Mode   CHEVMODE. Generate a channel event  with limit checking The        boundary band is defined as the area where the result is less than or        equal to the selected upper boundary and greater than or equal to the        selected lower boundary  see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn chevmode(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x3,
            1,
            0,
            gxchctry::Chevmode,
            GxChctRy_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x3,
                1,
                0,
                gxchctry::Chevmode,
                GxChctRy_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Synchronization Request   SYNC"]
        #[inline(always)]
        pub fn sync(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            gxchctry::Sync,
            GxChctRy_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                gxchctry::Sync,
                GxChctRy_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Reference Input Selection   REFSEL. Defines the reference voltage input to be used for conversions on this        channel. Some channels cannot select an        alternate reference."]
        #[inline(always)]
        pub fn refsel(
            self,
        ) -> crate::common::RegisterField<
            11,
            0x1,
            1,
            0,
            gxchctry::Refsel,
            GxChctRy_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                gxchctry::Refsel,
                GxChctRy_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "BoundaryExtension   BNDSELX. While BNDSELX   8800  0000   bitfields        BNDSELU and BNDSELL are concatenated and select the corresponding result        register as lower boundary."]
        #[inline(always)]
        pub fn bndselx(
            self,
        ) -> crate::common::RegisterField<
            12,
            0xf,
            1,
            0,
            gxchctry::Bndselx,
            GxChctRy_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                12,
                0xf,
                1,
                0,
                gxchctry::Bndselx,
                GxChctRy_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Result Register   RESREG"]
        #[inline(always)]
        pub fn resreg(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, GxChctRy_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, GxChctRy_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Result Target   RESTGT"]
        #[inline(always)]
        pub fn restgt(
            self,
        ) -> crate::common::RegisterField<
            20,
            0x1,
            1,
            0,
            gxchctry::Restgt,
            GxChctRy_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0x1,
                1,
                0,
                gxchctry::Restgt,
                GxChctRy_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Result Position   RESPOS"]
        #[inline(always)]
        pub fn respos(
            self,
        ) -> crate::common::RegisterField<
            21,
            0x1,
            1,
            0,
            gxchctry::Respos,
            GxChctRy_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                21,
                0x1,
                1,
                0,
                gxchctry::Respos,
                GxChctRy_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Broken Wire Detection Enable   BWDEN"]
        #[inline(always)]
        pub fn bwden(
            self,
        ) -> crate::common::RegisterField<
            30,
            0x1,
            1,
            0,
            gxchctry::Bwden,
            GxChctRy_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                30,
                0x1,
                1,
                0,
                gxchctry::Bwden,
                GxChctRy_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for GxChctRy {
        #[inline(always)]
        fn default() -> GxChctRy {
            <crate::RegValueT<GxChctRy_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod gxchctry {
        pub struct Iclsel_SPEC;
        pub type Iclsel = crate::EnumBitfieldStruct<u8, Iclsel_SPEC>;
        impl Iclsel {
            #[doc = "Use group specific class 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Use group specific class 1"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Use global class 0"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Use global class 1"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Bndsell_SPEC;
        pub type Bndsell = crate::EnumBitfieldStruct<u8, Bndsell_SPEC>;
        impl Bndsell {
            #[doc = "Use group specific boundary 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Use group specific boundary 1"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Use global boundary 0"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Use global boundary 1"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Bndselu_SPEC;
        pub type Bndselu = crate::EnumBitfieldStruct<u8, Bndselu_SPEC>;
        impl Bndselu {
            #[doc = "Use group specific boundary 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Use group specific boundary 1"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Use global boundary 0"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Use global boundary 1"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Chevmode_SPEC;
        pub type Chevmode = crate::EnumBitfieldStruct<u8, Chevmode_SPEC>;
        impl Chevmode {
            #[doc = "Never"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "If result is inside the boundary band"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "If result is outside the boundary band"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Always  ignore band"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Sync_SPEC;
        pub type Sync = crate::EnumBitfieldStruct<u8, Sync_SPEC>;
        impl Sync {
            #[doc = "No synchronization request  standalone operation"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Request a synchronized conversion of this channel. 1  only taken        into account for a master"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Refsel_SPEC;
        pub type Refsel = crate::EnumBitfieldStruct<u8, Refsel_SPEC>;
        impl Refsel {
            #[doc = "Standard reference input VAREF"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Alternate reference input from CH0"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Bndselx_SPEC;
        pub type Bndselx = crate::EnumBitfieldStruct<u8, Bndselx_SPEC>;
        impl Bndselx {
            #[doc = "Standard mode. select boundaries via BNDSELU BNDSELL"]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Restgt_SPEC;
        pub type Restgt = crate::EnumBitfieldStruct<u8, Restgt_SPEC>;
        impl Restgt {
            #[doc = "Store results in the selected group result register"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Store results in the global result register  not possible for synchronization slaves"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Respos_SPEC;
        pub type Respos = crate::EnumBitfieldStruct<u8, Respos_SPEC>;
        impl Respos {
            #[doc = "Read results right aligned  all modes"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Read results left aligned  std. conversions only"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Bwden_SPEC;
        pub type Bwden = crate::EnumBitfieldStruct<u8, Bwden_SPEC>;
        impl Bwden {
            #[doc = "Normal operation"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Additional preparation phase is enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxEmuxcs_SPEC;
    impl crate::sealed::RegSpec for GxEmuxcs_SPEC {
        type DataType = u32;
    }
    #[doc = "Ext. Multiplexer Channel Select Reg.  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxEmuxcs = crate::RegValueT<GxEmuxcs_SPEC>;

    impl GxEmuxcs {
        #[doc = "External Multiplexer Channel Select   EMUXCH. Defines the channel s  to which the external multiplexer control is        applied. EMXCSS  160    160 0  Channel number the lower 5 bits select an arbitrary channel  valid numbers are limited        by the number of available channels  unused bits shall be 0  EMXCSS  160    160 1  Channel enable each bit enables the associated channel  multiple channels can be        selected enabled"]
        #[inline(always)]
        pub fn emuxch(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, GxEmuxcs_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, GxEmuxcs_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GxEmuxcs {
        #[inline(always)]
        fn default() -> GxEmuxcs {
            <crate::RegValueT<GxEmuxcs_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxEmuxctr_SPEC;
    impl crate::sealed::RegSpec for GxEmuxctr_SPEC {
        type DataType = u32;
    }
    #[doc = "External Multiplexer Control Reg.  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxEmuxctr = crate::RegValueT<GxEmuxctr_SPEC>;

    impl GxEmuxctr {
        #[doc = "External Multiplexer Start Selection   EMUXSET. Defines the initial selection for the external multiplexer."]
        #[inline(always)]
        pub fn emuxset(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, GxEmuxctr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7,1,0,u8, GxEmuxctr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "External Multiplexer Coding Scheme   EMXCOD"]
        #[inline(always)]
        pub fn emxcod(
            self,
        ) -> crate::common::RegisterField<
            12,
            0x1,
            1,
            0,
            gxemuxctr::Emxcod,
            GxEmuxctr_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                12,
                0x1,
                1,
                0,
                gxemuxctr::Emxcod,
                GxEmuxctr_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "External Multiplexer Sample Time Control   EMXST"]
        #[inline(always)]
        pub fn emxst(
            self,
        ) -> crate::common::RegisterField<
            13,
            0x1,
            1,
            0,
            gxemuxctr::Emxst,
            GxEmuxctr_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                13,
                0x1,
                1,
                0,
                gxemuxctr::Emxst,
                GxEmuxctr_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "External Multiplexer Channel Selection Style   EMXCSS"]
        #[inline(always)]
        pub fn emxcss(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x1,
            1,
            0,
            gxemuxctr::Emxcss,
            GxEmuxctr_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                gxemuxctr::Emxcss,
                GxEmuxctr_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Write Control for EMUX Configuration   EMXWC"]
        #[inline(always)]
        pub fn emxwc(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            gxemuxctr::Emxwc,
            GxEmuxctr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                gxemuxctr::Emxwc,
                GxEmuxctr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "External Multiplexer Actual Selection   EMUXACT. Defines the current value for the external multiplexer selection. This        bitfield is loaded from bitfield EMUXSET and modified according to the        operating mode selected by bitfield EMUXMODE."]
        #[inline(always)]
        pub fn emuxact(
            self,
        ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, GxEmuxctr_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0x7,1,0,u8, GxEmuxctr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "External Multiplexer Channel Selection for Block Mode   EMUXCCB. Defines the channel that switches EMUXACT when converted. In block mode         all EMUX channels use the same control value."]
        #[inline(always)]
        pub fn emuxccb(
            self,
        ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, GxEmuxctr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0x1f,1,0,u8, GxEmuxctr_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GxEmuxctr {
        #[inline(always)]
        fn default() -> GxEmuxctr {
            <crate::RegValueT<GxEmuxctr_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod gxemuxctr {
        pub struct Emxcod_SPEC;
        pub type Emxcod = crate::EnumBitfieldStruct<u8, Emxcod_SPEC>;
        impl Emxcod {
            #[doc = "Output the channel number in binary code"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Output the channel number in Gray code"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Emxst_SPEC;
        pub type Emxst = crate::EnumBitfieldStruct<u8, Emxst_SPEC>;
        impl Emxst {
            #[doc = "Use STCE whenever the external channel selection changes"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Use STCE for each conversion of an external channel"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Emxcss_SPEC;
        pub type Emxcss = crate::EnumBitfieldStruct<u8, Emxcss_SPEC>;
        impl Emxcss {
            #[doc = "Channel number. Bitfield EMUXCH selects an arbitrary channel"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Channel enable. Each bit of bitfield EMUXCH selects the associated channel for EMUX        control"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Emxwc_SPEC;
        pub type Emxwc = crate::EnumBitfieldStruct<u8, Emxwc_SPEC>;
        impl Emxwc {
            #[doc = "No write access to EMUX cfg."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Bitfields EMXMODE  EMXCOD  EMXST  EMXCSS can be written"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxIclasSi_SPEC;
    impl crate::sealed::RegSpec for GxIclasSi_SPEC {
        type DataType = u32;
    }
    #[doc = "Input Class Register 0  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxIclasSi = crate::RegValueT<GxIclasSi_SPEC>;

    impl GxIclasSi {
        #[doc = "Sample Time Control for Standard Conversions   STCS. Number of additional clock cycles to be added to the minimum sample        phase of 2 sample clock cycles  Coding and resulting sample time see CROSSREFERENCE . For conversions of external channels  the value from bitfield STCE can        be used."]
        #[inline(always)]
        pub fn stcs(
            self,
        ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, GxIclasSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1f,1,0,u8, GxIclasSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Analog Input Precharge Control for Standard Conversions   AIPS. Buffer must be enabled by BE  160    160 1  see GxANCFG  ."]
        #[inline(always)]
        pub fn aips(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x3,
            1,
            0,
            gxiclassi::Aips,
            GxIclasSi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                6,
                0x3,
                1,
                0,
                gxiclassi::Aips,
                GxIclasSi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Conversion Mode for Standard Conversions   CMS"]
        #[inline(always)]
        pub fn cms(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x3,
            1,
            0,
            gxiclassi::Cms,
            GxIclasSi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x3,
                1,
                0,
                gxiclassi::Cms,
                GxIclasSi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Spread Early Sample Point for Standard Conversions   SESPS"]
        #[inline(always)]
        pub fn sesps(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            gxiclassi::Sesps,
            GxIclasSi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                gxiclassi::Sesps,
                GxIclasSi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Sample Time Control for EMUX Conversions   STCE. Number of additional clock cycles to be added to the minimum sample        phase of 2 sample clock cycles  Coding and resulting sample time see CROSSREFERENCE . For conversions of standard channels  the value from bitfield STCS is        used."]
        #[inline(always)]
        pub fn stce(
            self,
        ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, GxIclasSi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1f,1,0,u8, GxIclasSi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Analog Input Precharge Control for EMUX Conversions   AIPE. Buffer must be enabled by BE  160    160 1  see GxANCFG  ."]
        #[inline(always)]
        pub fn aipe(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x3,
            1,
            0,
            gxiclassi::Aipe,
            GxIclasSi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                22,
                0x3,
                1,
                0,
                gxiclassi::Aipe,
                GxIclasSi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Conversion Mode for EMUX Conversions   CME"]
        #[inline(always)]
        pub fn cme(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x3,
            1,
            0,
            gxiclassi::Cme,
            GxIclasSi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0x3,
                1,
                0,
                gxiclassi::Cme,
                GxIclasSi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Spread Early Sample Point for EMUX Conversions   SESPE"]
        #[inline(always)]
        pub fn sespe(
            self,
        ) -> crate::common::RegisterField<
            26,
            0x1,
            1,
            0,
            gxiclassi::Sespe,
            GxIclasSi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                26,
                0x1,
                1,
                0,
                gxiclassi::Sespe,
                GxIclasSi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for GxIclasSi {
        #[inline(always)]
        fn default() -> GxIclasSi {
            <crate::RegValueT<GxIclasSi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod gxiclassi {
        pub struct Aips_SPEC;
        pub type Aips = crate::EnumBitfieldStruct<u8, Aips_SPEC>;
        impl Aips {
            #[doc = "No precharge"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Precharge for 8 clock cycles"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Precharge for 16 clock cycles"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Precharge for 32 clock cycles"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Cms_SPEC;
        pub type Cms = crate::EnumBitfieldStruct<u8, Cms_SPEC>;
        impl Cms {
            #[doc = "Standard conversion"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Noise reduction conversion level 1  1 additional conversion step"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Noise reduction conversion level 2  3 additional conversion steps"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Noise reduction conversion level 3  7 additional conversion steps"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Sesps_SPEC;
        pub type Sesps = crate::EnumBitfieldStruct<u8, Sesps_SPEC>;
        impl Sesps {
            #[doc = "Nominal sample timing"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Spread sample timing  end of sample phase is varied"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Aipe_SPEC;
        pub type Aipe = crate::EnumBitfieldStruct<u8, Aipe_SPEC>;
        impl Aipe {
            #[doc = "No precharge"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Precharge for 8 clock cycles"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Precharge for 16 clock cycles"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Precharge for 32 clock cycles"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Cme_SPEC;
        pub type Cme = crate::EnumBitfieldStruct<u8, Cme_SPEC>;
        impl Cme {
            #[doc = "Standard conversion"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Noise reduction conversion level 1  1 additional conversion step"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Noise reduction conversion level 2  3 additional conversion steps"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Noise reduction conversion level 3  7 additional conversion steps"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Sespe_SPEC;
        pub type Sespe = crate::EnumBitfieldStruct<u8, Sespe_SPEC>;
        impl Sespe {
            #[doc = "Nominal sample timing"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Spread sample timing  end of sample phase is varied"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxRcRy_SPEC;
    impl crate::sealed::RegSpec for GxRcRy_SPEC {
        type DataType = u32;
    }
    #[doc = "Group 0 Result Control Register 0\n resetvalue={Application Reset:0x0}"]
    pub type GxRcRy = crate::RegValueT<GxRcRy_SPEC>;

    impl GxRcRy {
        #[doc = "Data Reduction Control   DRCTR. Defines how result values are stored accumulated in this register for        the final result. The data reduction counter DRC can be loaded from this        bitfield. The function of bitfield DRCTR is determined by bitfield DMM."]
        #[inline(always)]
        pub fn drctr(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, GxRcRy_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, GxRcRy_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Data Modification Mode   DMM. See CROSSREFERENCE"]
        #[inline(always)]
        pub fn dmm(
            self,
        ) -> crate::common::RegisterField<20, 0x3, 1, 0, gxrcry::Dmm, GxRcRy_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0x3,1,0,gxrcry::Dmm, GxRcRy_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Wait for Read Mode Enable   WFR"]
        #[inline(always)]
        pub fn wfr(
            self,
        ) -> crate::common::RegisterField<24, 0x1, 1, 0, gxrcry::Wfr, GxRcRy_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0x1,1,0,gxrcry::Wfr, GxRcRy_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "FIFO Mode Enable   FEN"]
        #[inline(always)]
        pub fn fen(
            self,
        ) -> crate::common::RegisterField<25, 0x3, 1, 0, gxrcry::Fen, GxRcRy_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<25,0x3,1,0,gxrcry::Fen, GxRcRy_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Service Request Generation Enable   SRGEN"]
        #[inline(always)]
        pub fn srgen(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            gxrcry::Srgen,
            GxRcRy_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                gxrcry::Srgen,
                GxRcRy_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for GxRcRy {
        #[inline(always)]
        fn default() -> GxRcRy {
            <crate::RegValueT<GxRcRy_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod gxrcry {
        pub struct Dmm_SPEC;
        pub type Dmm = crate::EnumBitfieldStruct<u8, Dmm_SPEC>;
        impl Dmm {
            #[doc = "Standard data reduction  accumulation"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Result filtering mode. The filter registers are cleared while bitfield DMM  160   8800   160  01 ."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Difference mode"]
            pub const CONST_22: Self = Self::new(2);
        }
        pub struct Wfr_SPEC;
        pub type Wfr = crate::EnumBitfieldStruct<u8, Wfr_SPEC>;
        impl Wfr {
            #[doc = "Overwrite mode"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Wait for read mode enabled for this register"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Fen_SPEC;
        pub type Fen = crate::EnumBitfieldStruct<u8, Fen_SPEC>;
        impl Fen {
            #[doc = "Separate result register"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Part of a FIFO structure  copy each new valid result"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Maximum mode  copy new result if bigger"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Minimum mode  copy new result if smaller"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Srgen_SPEC;
        pub type Srgen = crate::EnumBitfieldStruct<u8, Srgen_SPEC>;
        impl Srgen {
            #[doc = "No service request"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Service request after a result event"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxRefclr_SPEC;
    impl crate::sealed::RegSpec for GxRefclr_SPEC {
        type DataType = u32;
    }
    #[doc = "Result Event Flag Clear Register  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxRefclr = crate::RegValueT<GxRefclr_SPEC>;

    impl GxRefclr {
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev0(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            gxrefclr::Rev0,
            GxRefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                gxrefclr::Rev0,
                GxRefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev1(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            gxrefclr::Rev1,
            GxRefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                gxrefclr::Rev1,
                GxRefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev2(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            gxrefclr::Rev2,
            GxRefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                gxrefclr::Rev2,
                GxRefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev3(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            gxrefclr::Rev3,
            GxRefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                gxrefclr::Rev3,
                GxRefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev4(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            gxrefclr::Rev4,
            GxRefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                gxrefclr::Rev4,
                GxRefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev5(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            gxrefclr::Rev5,
            GxRefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                gxrefclr::Rev5,
                GxRefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev6(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x1,
            1,
            0,
            gxrefclr::Rev6,
            GxRefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                6,
                0x1,
                1,
                0,
                gxrefclr::Rev6,
                GxRefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev7(
            self,
        ) -> crate::common::RegisterField<
            7,
            0x1,
            1,
            0,
            gxrefclr::Rev7,
            GxRefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                gxrefclr::Rev7,
                GxRefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev8(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1,
            1,
            0,
            gxrefclr::Rev8,
            GxRefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                gxrefclr::Rev8,
                GxRefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev9(
            self,
        ) -> crate::common::RegisterField<
            9,
            0x1,
            1,
            0,
            gxrefclr::Rev9,
            GxRefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                9,
                0x1,
                1,
                0,
                gxrefclr::Rev9,
                GxRefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev10(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            gxrefclr::Rev10,
            GxRefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                gxrefclr::Rev10,
                GxRefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev11(
            self,
        ) -> crate::common::RegisterField<
            11,
            0x1,
            1,
            0,
            gxrefclr::Rev11,
            GxRefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                gxrefclr::Rev11,
                GxRefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev12(
            self,
        ) -> crate::common::RegisterField<
            12,
            0x1,
            1,
            0,
            gxrefclr::Rev12,
            GxRefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                12,
                0x1,
                1,
                0,
                gxrefclr::Rev12,
                GxRefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev13(
            self,
        ) -> crate::common::RegisterField<
            13,
            0x1,
            1,
            0,
            gxrefclr::Rev13,
            GxRefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                13,
                0x1,
                1,
                0,
                gxrefclr::Rev13,
                GxRefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev14(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x1,
            1,
            0,
            gxrefclr::Rev14,
            GxRefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                gxrefclr::Rev14,
                GxRefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev15(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            gxrefclr::Rev15,
            GxRefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                gxrefclr::Rev15,
                GxRefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for GxRefclr {
        #[inline(always)]
        fn default() -> GxRefclr {
            <crate::RegValueT<GxRefclr_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod gxrefclr {
        pub struct Rev0_SPEC;
        pub type Rev0 = crate::EnumBitfieldStruct<u8, Rev0_SPEC>;
        impl Rev0 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the result event flag in GxREFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev1_SPEC;
        pub type Rev1 = crate::EnumBitfieldStruct<u8, Rev1_SPEC>;
        impl Rev1 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the result event flag in GxREFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev2_SPEC;
        pub type Rev2 = crate::EnumBitfieldStruct<u8, Rev2_SPEC>;
        impl Rev2 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the result event flag in GxREFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev3_SPEC;
        pub type Rev3 = crate::EnumBitfieldStruct<u8, Rev3_SPEC>;
        impl Rev3 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the result event flag in GxREFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev4_SPEC;
        pub type Rev4 = crate::EnumBitfieldStruct<u8, Rev4_SPEC>;
        impl Rev4 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the result event flag in GxREFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev5_SPEC;
        pub type Rev5 = crate::EnumBitfieldStruct<u8, Rev5_SPEC>;
        impl Rev5 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the result event flag in GxREFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev6_SPEC;
        pub type Rev6 = crate::EnumBitfieldStruct<u8, Rev6_SPEC>;
        impl Rev6 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the result event flag in GxREFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev7_SPEC;
        pub type Rev7 = crate::EnumBitfieldStruct<u8, Rev7_SPEC>;
        impl Rev7 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the result event flag in GxREFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev8_SPEC;
        pub type Rev8 = crate::EnumBitfieldStruct<u8, Rev8_SPEC>;
        impl Rev8 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the result event flag in GxREFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev9_SPEC;
        pub type Rev9 = crate::EnumBitfieldStruct<u8, Rev9_SPEC>;
        impl Rev9 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the result event flag in GxREFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev10_SPEC;
        pub type Rev10 = crate::EnumBitfieldStruct<u8, Rev10_SPEC>;
        impl Rev10 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the result event flag in GxREFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev11_SPEC;
        pub type Rev11 = crate::EnumBitfieldStruct<u8, Rev11_SPEC>;
        impl Rev11 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the result event flag in GxREFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev12_SPEC;
        pub type Rev12 = crate::EnumBitfieldStruct<u8, Rev12_SPEC>;
        impl Rev12 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the result event flag in GxREFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev13_SPEC;
        pub type Rev13 = crate::EnumBitfieldStruct<u8, Rev13_SPEC>;
        impl Rev13 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the result event flag in GxREFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev14_SPEC;
        pub type Rev14 = crate::EnumBitfieldStruct<u8, Rev14_SPEC>;
        impl Rev14 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the result event flag in GxREFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev15_SPEC;
        pub type Rev15 = crate::EnumBitfieldStruct<u8, Rev15_SPEC>;
        impl Rev15 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the result event flag in GxREFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxReflag_SPEC;
    impl crate::sealed::RegSpec for GxReflag_SPEC {
        type DataType = u32;
    }
    #[doc = "Result Event Flag Register  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxReflag = crate::RegValueT<GxReflag_SPEC>;

    impl GxReflag {
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev0(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            gxreflag::Rev0,
            GxReflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                gxreflag::Rev0,
                GxReflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev1(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            gxreflag::Rev1,
            GxReflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                gxreflag::Rev1,
                GxReflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev2(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            gxreflag::Rev2,
            GxReflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                gxreflag::Rev2,
                GxReflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev3(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            gxreflag::Rev3,
            GxReflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                gxreflag::Rev3,
                GxReflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev4(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            gxreflag::Rev4,
            GxReflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                gxreflag::Rev4,
                GxReflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev5(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            gxreflag::Rev5,
            GxReflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                gxreflag::Rev5,
                GxReflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev6(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x1,
            1,
            0,
            gxreflag::Rev6,
            GxReflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                6,
                0x1,
                1,
                0,
                gxreflag::Rev6,
                GxReflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev7(
            self,
        ) -> crate::common::RegisterField<
            7,
            0x1,
            1,
            0,
            gxreflag::Rev7,
            GxReflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                gxreflag::Rev7,
                GxReflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev8(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1,
            1,
            0,
            gxreflag::Rev8,
            GxReflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                gxreflag::Rev8,
                GxReflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev9(
            self,
        ) -> crate::common::RegisterField<
            9,
            0x1,
            1,
            0,
            gxreflag::Rev9,
            GxReflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                9,
                0x1,
                1,
                0,
                gxreflag::Rev9,
                GxReflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev10(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            gxreflag::Rev10,
            GxReflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                gxreflag::Rev10,
                GxReflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev11(
            self,
        ) -> crate::common::RegisterField<
            11,
            0x1,
            1,
            0,
            gxreflag::Rev11,
            GxReflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                gxreflag::Rev11,
                GxReflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev12(
            self,
        ) -> crate::common::RegisterField<
            12,
            0x1,
            1,
            0,
            gxreflag::Rev12,
            GxReflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                12,
                0x1,
                1,
                0,
                gxreflag::Rev12,
                GxReflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev13(
            self,
        ) -> crate::common::RegisterField<
            13,
            0x1,
            1,
            0,
            gxreflag::Rev13,
            GxReflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                13,
                0x1,
                1,
                0,
                gxreflag::Rev13,
                GxReflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev14(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x1,
            1,
            0,
            gxreflag::Rev14,
            GxReflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                gxreflag::Rev14,
                GxReflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Result Event for Result Register 15"]
        #[inline(always)]
        pub fn rev15(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            gxreflag::Rev15,
            GxReflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                gxreflag::Rev15,
                GxReflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for GxReflag {
        #[inline(always)]
        fn default() -> GxReflag {
            <crate::RegValueT<GxReflag_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod gxreflag {
        pub struct Rev0_SPEC;
        pub type Rev0 = crate::EnumBitfieldStruct<u8, Rev0_SPEC>;
        impl Rev0 {
            #[doc = "No result event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "New result was stored in register GxRESy"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev1_SPEC;
        pub type Rev1 = crate::EnumBitfieldStruct<u8, Rev1_SPEC>;
        impl Rev1 {
            #[doc = "No result event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "New result was stored in register GxRESy"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev2_SPEC;
        pub type Rev2 = crate::EnumBitfieldStruct<u8, Rev2_SPEC>;
        impl Rev2 {
            #[doc = "No result event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "New result was stored in register GxRESy"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev3_SPEC;
        pub type Rev3 = crate::EnumBitfieldStruct<u8, Rev3_SPEC>;
        impl Rev3 {
            #[doc = "No result event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "New result was stored in register GxRESy"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev4_SPEC;
        pub type Rev4 = crate::EnumBitfieldStruct<u8, Rev4_SPEC>;
        impl Rev4 {
            #[doc = "No result event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "New result was stored in register GxRESy"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev5_SPEC;
        pub type Rev5 = crate::EnumBitfieldStruct<u8, Rev5_SPEC>;
        impl Rev5 {
            #[doc = "No result event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "New result was stored in register GxRESy"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev6_SPEC;
        pub type Rev6 = crate::EnumBitfieldStruct<u8, Rev6_SPEC>;
        impl Rev6 {
            #[doc = "No result event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "New result was stored in register GxRESy"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev7_SPEC;
        pub type Rev7 = crate::EnumBitfieldStruct<u8, Rev7_SPEC>;
        impl Rev7 {
            #[doc = "No result event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "New result was stored in register GxRESy"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev8_SPEC;
        pub type Rev8 = crate::EnumBitfieldStruct<u8, Rev8_SPEC>;
        impl Rev8 {
            #[doc = "No result event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "New result was stored in register GxRESy"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev9_SPEC;
        pub type Rev9 = crate::EnumBitfieldStruct<u8, Rev9_SPEC>;
        impl Rev9 {
            #[doc = "No result event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "New result was stored in register GxRESy"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev10_SPEC;
        pub type Rev10 = crate::EnumBitfieldStruct<u8, Rev10_SPEC>;
        impl Rev10 {
            #[doc = "No result event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "New result was stored in register GxRESy"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev11_SPEC;
        pub type Rev11 = crate::EnumBitfieldStruct<u8, Rev11_SPEC>;
        impl Rev11 {
            #[doc = "No result event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "New result was stored in register GxRESy"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev12_SPEC;
        pub type Rev12 = crate::EnumBitfieldStruct<u8, Rev12_SPEC>;
        impl Rev12 {
            #[doc = "No result event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "New result was stored in register GxRESy"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev13_SPEC;
        pub type Rev13 = crate::EnumBitfieldStruct<u8, Rev13_SPEC>;
        impl Rev13 {
            #[doc = "No result event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "New result was stored in register GxRESy"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev14_SPEC;
        pub type Rev14 = crate::EnumBitfieldStruct<u8, Rev14_SPEC>;
        impl Rev14 {
            #[doc = "No result event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "New result was stored in register GxRESy"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rev15_SPEC;
        pub type Rev15 = crate::EnumBitfieldStruct<u8, Rev15_SPEC>;
        impl Rev15 {
            #[doc = "No result event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "New result was stored in register GxRESy"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxResDy_SPEC;
    impl crate::sealed::RegSpec for GxResDy_SPEC {
        type DataType = u32;
    }
    #[doc = "Group 0 Result Reg. 0  Debug\n resetvalue={Application Reset:0x0}"]
    pub type GxResDy = crate::RegValueT<GxResDy_SPEC>;

    impl GxResDy {
        #[doc = "Result of Most Recent Conversion   RESULT. The position of the result bits within this bitfield depends on the        configured operating mode. Refer to CROSSREFERENCE ."]
        #[inline(always)]
        pub fn result(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, GxResDy_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, GxResDy_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Data Reduction Counter   DRC. Indicates the number of values still to be accumulated for the final        result. The final result is available and valid flag VF is set when        bitfield DRC becomes zero  by decrementing or by reload . See CROSSREFERENCE"]
        #[inline(always)]
        pub fn drc(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, GxResDy_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, GxResDy_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Channel Number   CHNR. Indicates the channel number corresponding to the value in bitfield        RESULT."]
        #[inline(always)]
        pub fn chnr(
            self,
        ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, GxResDy_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<20,0x1f,1,0,u8, GxResDy_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "External Multiplexer Setting   EMUX. Indicates the setting of the external multiplexer  corresponding to the        value in bitfield RESULT. Available in GxRESD0 only. Use GxRESD0 if EMUX information is required."]
        #[inline(always)]
        pub fn emux(
            self,
        ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, GxResDy_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<25,0x7,1,0,u8, GxResDy_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Converted Request Source   CRS. Indicates the request source that as requested the conversion to which        the result value in bitfield RESULT belongs."]
        #[inline(always)]
        pub fn crs(
            self,
        ) -> crate::common::RegisterField<28, 0x3, 1, 0, gxresdy::Crs, GxResDy_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<
                28,
                0x3,
                1,
                0,
                gxresdy::Crs,
                GxResDy_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Valid Flag   VF. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf(
            self,
        ) -> crate::common::RegisterField<31, 0x1, 1, 0, gxresdy::Vf, GxResDy_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<31,0x1,1,0,gxresdy::Vf, GxResDy_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for GxResDy {
        #[inline(always)]
        fn default() -> GxResDy {
            <crate::RegValueT<GxResDy_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod gxresdy {
        pub struct Crs_SPEC;
        pub type Crs = crate::EnumBitfieldStruct<u8, Crs_SPEC>;
        impl Crs {
            #[doc = "Request source 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Request source 1"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Request source 2"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Synchronized conversion"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Vf_SPEC;
        pub type Vf = crate::EnumBitfieldStruct<u8, Vf_SPEC>;
        impl Vf {
            #[doc = "No new result available"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Bitfield RESULT has been updated with new result value and has not yet been read  via GxRESy"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxReSy_SPEC;
    impl crate::sealed::RegSpec for GxReSy_SPEC {
        type DataType = u32;
    }
    #[doc = "Group 0 Result Register 0\n resetvalue={Application Reset:0x0}"]
    pub type GxReSy = crate::RegValueT<GxReSy_SPEC>;

    impl GxReSy {
        #[doc = "Result of Most Recent Conversion   RESULT. The position of the result bits within this bitfield depends on the        configured operating mode. Refer to CROSSREFERENCE . Bitfield RESULT is writeable by the application to set the initial value        for min max detection."]
        #[inline(always)]
        pub fn result(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, GxReSy_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, GxReSy_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Data Reduction Counter   DRC. Indicates the number of values still to be accumulated for the final        result. The final result is available and valid flag VF is set when        bitfield DRC becomes zero  by decrementing or by reload . See CROSSREFERENCE"]
        #[inline(always)]
        pub fn drc(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, GxReSy_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, GxReSy_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Channel Number   CHNR. Indicates the channel number corresponding to the value in bitfield        RESULT."]
        #[inline(always)]
        pub fn chnr(
            self,
        ) -> crate::common::RegisterField<20, 0x1f, 1, 0, u8, GxReSy_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<20,0x1f,1,0,u8, GxReSy_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "External Multiplexer Setting   EMUX. Indicates the setting of the external multiplexer  corresponding to the        value in bitfield RESULT. Available in GxRES0 only. Use GxRES0 if EMUX information is required."]
        #[inline(always)]
        pub fn emux(
            self,
        ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, GxReSy_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<25,0x7,1,0,u8, GxReSy_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Converted Request Source   CRS. Indicates the request source that as requested the conversion to which        the result value in bitfield RESULT belongs."]
        #[inline(always)]
        pub fn crs(
            self,
        ) -> crate::common::RegisterField<28, 0x3, 1, 0, gxresy::Crs, GxReSy_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<28,0x3,1,0,gxresy::Crs, GxReSy_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Valid Flag   VF. Indicates a new result in bitfield RESULT. Bit VF is automatically cleared upon reading register GxRESy."]
        #[inline(always)]
        pub fn vf(
            self,
        ) -> crate::common::RegisterField<31, 0x1, 1, 0, gxresy::Vf, GxReSy_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<31,0x1,1,0,gxresy::Vf, GxReSy_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for GxReSy {
        #[inline(always)]
        fn default() -> GxReSy {
            <crate::RegValueT<GxReSy_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod gxresy {
        pub struct Crs_SPEC;
        pub type Crs = crate::EnumBitfieldStruct<u8, Crs_SPEC>;
        impl Crs {
            #[doc = "Request source 0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Request source 1"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Request source 2"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Synchronized conversion"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Vf_SPEC;
        pub type Vf = crate::EnumBitfieldStruct<u8, Vf_SPEC>;
        impl Vf {
            #[doc = "No new result available"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Bitfield RESULT has been updated with new result value and has not yet been read"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxSefclr_SPEC;
    impl crate::sealed::RegSpec for GxSefclr_SPEC {
        type DataType = u32;
    }
    #[doc = "Source Event Flag Clear Reg.  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxSefclr = crate::RegValueT<GxSefclr_SPEC>;

    impl GxSefclr {
        #[doc = "Clear Source Event i"]
        #[inline(always)]
        pub fn sev0(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            gxsefclr::Sev0,
            GxSefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                gxsefclr::Sev0,
                GxSefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Source Event i"]
        #[inline(always)]
        pub fn sev1(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            gxsefclr::Sev1,
            GxSefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                gxsefclr::Sev1,
                GxSefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Clear Source Event i"]
        #[inline(always)]
        pub fn sev2(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            gxsefclr::Sev2,
            GxSefclr_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                gxsefclr::Sev2,
                GxSefclr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for GxSefclr {
        #[inline(always)]
        fn default() -> GxSefclr {
            <crate::RegValueT<GxSefclr_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod gxsefclr {
        pub struct Sev0_SPEC;
        pub type Sev0 = crate::EnumBitfieldStruct<u8, Sev0_SPEC>;
        impl Sev0 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the source event flag in GxSEFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Sev1_SPEC;
        pub type Sev1 = crate::EnumBitfieldStruct<u8, Sev1_SPEC>;
        impl Sev1 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the source event flag in GxSEFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Sev2_SPEC;
        pub type Sev2 = crate::EnumBitfieldStruct<u8, Sev2_SPEC>;
        impl Sev2 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear the source event flag in GxSEFLAG"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxSeflag_SPEC;
    impl crate::sealed::RegSpec for GxSeflag_SPEC {
        type DataType = u32;
    }
    #[doc = "Source Event Flag Register  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxSeflag = crate::RegValueT<GxSeflag_SPEC>;

    impl GxSeflag {
        #[doc = "Source Event i"]
        #[inline(always)]
        pub fn sev0(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            gxseflag::Sev0,
            GxSeflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                gxseflag::Sev0,
                GxSeflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Source Event i"]
        #[inline(always)]
        pub fn sev1(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            gxseflag::Sev1,
            GxSeflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                gxseflag::Sev1,
                GxSeflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Source Event i"]
        #[inline(always)]
        pub fn sev2(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            gxseflag::Sev2,
            GxSeflag_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                gxseflag::Sev2,
                GxSeflag_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for GxSeflag {
        #[inline(always)]
        fn default() -> GxSeflag {
            <crate::RegValueT<GxSeflag_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod gxseflag {
        pub struct Sev0_SPEC;
        pub type Sev0 = crate::EnumBitfieldStruct<u8, Sev0_SPEC>;
        impl Sev0 {
            #[doc = "No source event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "A source event has occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Sev1_SPEC;
        pub type Sev1 = crate::EnumBitfieldStruct<u8, Sev1_SPEC>;
        impl Sev1 {
            #[doc = "No source event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "A source event has occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Sev2_SPEC;
        pub type Sev2 = crate::EnumBitfieldStruct<u8, Sev2_SPEC>;
        impl Sev2 {
            #[doc = "No source event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "A source event has occurred"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxSract_SPEC;
    impl crate::sealed::RegSpec for GxSract_SPEC {
        type DataType = u32;
    }
    #[doc = "Service Request Software Activation Trigger  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxSract = crate::RegValueT<GxSract_SPEC>;

    impl GxSract {
        #[doc = "Activate Group Service Request Node 3   AGSR3"]
        #[inline(always)]
        pub fn agsr0(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            gxsract::Agsr0,
            GxSract_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                gxsract::Agsr0,
                GxSract_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Activate Group Service Request Node 3   AGSR3"]
        #[inline(always)]
        pub fn agsr1(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            gxsract::Agsr1,
            GxSract_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                gxsract::Agsr1,
                GxSract_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Activate Group Service Request Node 3   AGSR3"]
        #[inline(always)]
        pub fn agsr2(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            gxsract::Agsr2,
            GxSract_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                gxsract::Agsr2,
                GxSract_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Activate Group Service Request Node 3   AGSR3"]
        #[inline(always)]
        pub fn agsr3(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            gxsract::Agsr3,
            GxSract_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                gxsract::Agsr3,
                GxSract_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Activate Shared Service Request Node 3   ASSR3"]
        #[inline(always)]
        pub fn assr0(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1,
            1,
            0,
            gxsract::Assr0,
            GxSract_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                gxsract::Assr0,
                GxSract_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Activate Shared Service Request Node 3   ASSR3"]
        #[inline(always)]
        pub fn assr1(
            self,
        ) -> crate::common::RegisterField<
            9,
            0x1,
            1,
            0,
            gxsract::Assr1,
            GxSract_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                9,
                0x1,
                1,
                0,
                gxsract::Assr1,
                GxSract_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Activate Shared Service Request Node 3   ASSR3"]
        #[inline(always)]
        pub fn assr2(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            gxsract::Assr2,
            GxSract_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                gxsract::Assr2,
                GxSract_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Activate Shared Service Request Node 3   ASSR3"]
        #[inline(always)]
        pub fn assr3(
            self,
        ) -> crate::common::RegisterField<
            11,
            0x1,
            1,
            0,
            gxsract::Assr3,
            GxSract_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                gxsract::Assr3,
                GxSract_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for GxSract {
        #[inline(always)]
        fn default() -> GxSract {
            <crate::RegValueT<GxSract_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod gxsract {
        pub struct Agsr0_SPEC;
        pub type Agsr0 = crate::EnumBitfieldStruct<u8, Agsr0_SPEC>;
        impl Agsr0 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Activate the associated service request line"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Agsr1_SPEC;
        pub type Agsr1 = crate::EnumBitfieldStruct<u8, Agsr1_SPEC>;
        impl Agsr1 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Activate the associated service request line"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Agsr2_SPEC;
        pub type Agsr2 = crate::EnumBitfieldStruct<u8, Agsr2_SPEC>;
        impl Agsr2 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Activate the associated service request line"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Agsr3_SPEC;
        pub type Agsr3 = crate::EnumBitfieldStruct<u8, Agsr3_SPEC>;
        impl Agsr3 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Activate the associated service request line"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Assr0_SPEC;
        pub type Assr0 = crate::EnumBitfieldStruct<u8, Assr0_SPEC>;
        impl Assr0 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Activate the associated service request line"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Assr1_SPEC;
        pub type Assr1 = crate::EnumBitfieldStruct<u8, Assr1_SPEC>;
        impl Assr1 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Activate the associated service request line"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Assr2_SPEC;
        pub type Assr2 = crate::EnumBitfieldStruct<u8, Assr2_SPEC>;
        impl Assr2 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Activate the associated service request line"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Assr3_SPEC;
        pub type Assr3 = crate::EnumBitfieldStruct<u8, Assr3_SPEC>;
        impl Assr3 {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Activate the associated service request line"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxSynctr_SPEC;
    impl crate::sealed::RegSpec for GxSynctr_SPEC {
        type DataType = u32;
    }
    #[doc = "Synchronization Control Register  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxSynctr = crate::RegValueT<GxSynctr_SPEC>;

    impl GxSynctr {
        #[doc = "Start Selection   STSEL. Controls the synchronization mechanism of the ADC kernel. Control inputs CIx see CROSSREFERENCE            connected kernels see product specific appendix."]
        #[inline(always)]
        pub fn stsel(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x3,
            1,
            0,
            gxsynctr::Stsel,
            GxSynctr_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x3,
                1,
                0,
                gxsynctr::Stsel,
                GxSynctr_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Evaluate Ready Input Ri. Enables the ready input signal for a kernel of a synchronization group."]
        #[inline(always)]
        pub fn evalr1(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            gxsynctr::Evalr1,
            GxSynctr_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                gxsynctr::Evalr1,
                GxSynctr_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Evaluate Ready Input Ri. Enables the ready input signal for a kernel of a synchronization group."]
        #[inline(always)]
        pub fn evalr2(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            gxsynctr::Evalr2,
            GxSynctr_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                gxsynctr::Evalr2,
                GxSynctr_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Evaluate Ready Input Ri. Enables the ready input signal for a kernel of a synchronization group."]
        #[inline(always)]
        pub fn evalr3(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x1,
            1,
            0,
            gxsynctr::Evalr3,
            GxSynctr_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                6,
                0x1,
                1,
                0,
                gxsynctr::Evalr3,
                GxSynctr_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for GxSynctr {
        #[inline(always)]
        fn default() -> GxSynctr {
            <crate::RegValueT<GxSynctr_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod gxsynctr {
        pub struct Stsel_SPEC;
        pub type Stsel = crate::EnumBitfieldStruct<u8, Stsel_SPEC>;
        impl Stsel {
            #[doc = "Kernel is synchronization master. Use own bitfield GxARBCFG.ANONC."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Kernel is synchronization slave. Control information from input CI1."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Kernel is synchronization slave. Control information from input CI2."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Kernel is synchronization slave. Control information from input CI3."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Evalr1_SPEC;
        pub type Evalr1 = crate::EnumBitfieldStruct<u8, Evalr1_SPEC>;
        impl Evalr1 {
            #[doc = "No ready input control"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Ready input Ri is considered. for the start of a parallel conversion of this synchronization group"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Evalr2_SPEC;
        pub type Evalr2 = crate::EnumBitfieldStruct<u8, Evalr2_SPEC>;
        impl Evalr2 {
            #[doc = "No ready input control"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Ready input Ri is considered. for the start of a parallel conversion of this synchronization group"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Evalr3_SPEC;
        pub type Evalr3 = crate::EnumBitfieldStruct<u8, Evalr3_SPEC>;
        impl Evalr3 {
            #[doc = "No ready input control"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Ready input Ri is considered. for the start of a parallel conversion of this synchronization group"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxTest_SPEC;
    impl crate::sealed::RegSpec for GxTest_SPEC {
        type DataType = u32;
    }
    #[doc = "Test Register  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxTest = crate::RegValueT<GxTest_SPEC>;

    impl GxTest {
        #[doc = "User Test Vector   USRTEST. Described in the design specification."]
        #[inline(always)]
        pub fn usrtest(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, GxTest_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, GxTest_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Force FCONV   FFCONV"]
        #[inline(always)]
        pub fn ffconv(
            self,
        ) -> crate::common::RegisterField<
            16,
            0x1,
            1,
            0,
            gxtest::Ffconv,
            GxTest_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0x1,
                1,
                0,
                gxtest::Ffconv,
                GxTest_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Test Control   TC. Not listed combinations block write access to remaining bitfields."]
        #[inline(always)]
        pub fn tc(
            self,
        ) -> crate::common::RegisterField<28, 0xf, 1, 0, gxtest::Tc, GxTest_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<28,0xf,1,0,gxtest::Tc, GxTest_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for GxTest {
        #[inline(always)]
        fn default() -> GxTest {
            <crate::RegValueT<GxTest_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod gxtest {
        pub struct Ffconv_SPEC;
        pub type Ffconv = crate::EnumBitfieldStruct<u8, Ffconv_SPEC>;
        impl Ffconv {
            #[doc = "Signal FCONV is controlled by the EVADCDIG state machine"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Signal FCONV is permanently asserted"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tc_SPEC;
        pub type Tc = crate::EnumBitfieldStruct<u8, Tc_SPEC>;
        impl Tc {
            #[doc = "The remaining bitfields of register TESTx can be written."]
            pub const CONST_1111: Self = Self::new(11);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxTrctr_SPEC;
    impl crate::sealed::RegSpec for GxTrctr_SPEC {
        type DataType = u32;
    }
    #[doc = "Trigger Control Register  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxTrctr = crate::RegValueT<GxTrctr_SPEC>;

    impl GxTrctr {
        #[doc = "Trigger Sequence Counter   TSC. Controls the effect of an incoming internal trigger  If TSC  gt  00   decrement TSC by one."]
        #[inline(always)]
        pub fn tsc(
            self,
        ) -> crate::common::RegisterField<0, 0x3f, 1, 0, gxtrctr::Tsc, GxTrctr_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<
                0,
                0x3f,
                1,
                0,
                gxtrctr::Tsc,
                GxTrctr_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Queue Active   QACT. Indicates that request source Q2 is currently executing a sequence. Cleared by writing 1 to bit COV."]
        #[inline(always)]
        pub fn qact(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x1,
            1,
            0,
            gxtrctr::Qact,
            GxTrctr_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                gxtrctr::Qact,
                GxTrctr_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Overflow Detected   OV. Indicates that a trigger has been activated while the queue was still        active  QACT  160    160 1 . Cleared by writing 1 to bit COV."]
        #[inline(always)]
        pub fn ov(
            self,
        ) -> crate::common::RegisterField<15, 0x1, 1, 0, gxtrctr::Ov, GxTrctr_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<15,0x1,1,0,gxtrctr::Ov, GxTrctr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Trigger Sequence Counter Start Value   TSCSET. Defines the initial value of the trigger sequence counter TSC. TSC is reloaded with the value in TSCSET  when a trigger occurs while        TSC   00 . TSCSET is automatically copied to TSC when being written. CROSSREFERENCE"]
        #[inline(always)]
        pub fn tscset(
            self,
        ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, GxTrctr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x3f,1,0,u8, GxTrctr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Internal Trigger Input Selection   ITSEL. Internal triggers are generated by the respective source events. Enable a source event in the selected request source to generate an internal trigger signal. The selected trigger signal is internally connected to gate input GxREQGTP of this request source. It is selected when XTSEL   GTSEL   1111 ."]
        #[inline(always)]
        pub fn itsel(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x3,
            1,
            0,
            gxtrctr::Itsel,
            GxTrctr_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0x3,
                1,
                0,
                gxtrctr::Itsel,
                GxTrctr_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Service Request Disable   SRDIS. Controls if the source event of the selected trigger source also        activates a service request."]
        #[inline(always)]
        pub fn srdis(
            self,
        ) -> crate::common::RegisterField<
            28,
            0x1,
            1,
            0,
            gxtrctr::Srdis,
            GxTrctr_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                28,
                0x1,
                1,
                0,
                gxtrctr::Srdis,
                GxTrctr_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Clear Overflow Flag   COV"]
        #[inline(always)]
        pub fn cov(
            self,
        ) -> crate::common::RegisterField<31, 0x1, 1, 0, gxtrctr::Cov, GxTrctr_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                gxtrctr::Cov,
                GxTrctr_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for GxTrctr {
        #[inline(always)]
        fn default() -> GxTrctr {
            <crate::RegValueT<GxTrctr_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod gxtrctr {
        pub struct Tsc_SPEC;
        pub type Tsc = crate::EnumBitfieldStruct<u8, Tsc_SPEC>;
        impl Tsc {
            #[doc = "Issue conversion requests immediately. Write zero to bitfield TSCSET  to        disable the trigger sequence counter and start the sequence with each        incoming trigger."]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Qact_SPEC;
        pub type Qact = crate::EnumBitfieldStruct<u8, Qact_SPEC>;
        impl Qact {
            #[doc = "No activity"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Queue currently active"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Ov_SPEC;
        pub type Ov = crate::EnumBitfieldStruct<u8, Ov_SPEC>;
        impl Ov {
            #[doc = "No trigger event"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "An irregular trigger event has been detected"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Itsel_SPEC;
        pub type Itsel = crate::EnumBitfieldStruct<u8, Itsel_SPEC>;
        impl Itsel {
            #[doc = "Select queued request source Q0"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Select queued request source Q1"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Srdis_SPEC;
        pub type Srdis = crate::EnumBitfieldStruct<u8, Srdis_SPEC>;
        impl Srdis {
            #[doc = "Source event generates service request"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "No service request  only internal trigger generated"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cov_SPEC;
        pub type Cov = crate::EnumBitfieldStruct<u8, Cov_SPEC>;
        impl Cov {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear bits OV and QACT"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GxVfr_SPEC;
    impl crate::sealed::RegSpec for GxVfr_SPEC {
        type DataType = u32;
    }
    #[doc = "Valid Flag Register  Group 0\n resetvalue={Application Reset:0x0}"]
    pub type GxVfr = crate::RegValueT<GxVfr_SPEC>;

    impl GxVfr {
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf0(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, gxvfr::Vf0, GxVfr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1,1,0,gxvfr::Vf0, GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf1(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, gxvfr::Vf1, GxVfr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0x1,1,0,gxvfr::Vf1, GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf2(
            self,
        ) -> crate::common::RegisterField<2, 0x1, 1, 0, gxvfr::Vf2, GxVfr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x1,1,0,gxvfr::Vf2, GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf3(
            self,
        ) -> crate::common::RegisterField<3, 0x1, 1, 0, gxvfr::Vf3, GxVfr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<3,0x1,1,0,gxvfr::Vf3, GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf4(
            self,
        ) -> crate::common::RegisterField<4, 0x1, 1, 0, gxvfr::Vf4, GxVfr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x1,1,0,gxvfr::Vf4, GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf5(
            self,
        ) -> crate::common::RegisterField<5, 0x1, 1, 0, gxvfr::Vf5, GxVfr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x1,1,0,gxvfr::Vf5, GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf6(
            self,
        ) -> crate::common::RegisterField<6, 0x1, 1, 0, gxvfr::Vf6, GxVfr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<6,0x1,1,0,gxvfr::Vf6, GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf7(
            self,
        ) -> crate::common::RegisterField<7, 0x1, 1, 0, gxvfr::Vf7, GxVfr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<7,0x1,1,0,gxvfr::Vf7, GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf8(
            self,
        ) -> crate::common::RegisterField<8, 0x1, 1, 0, gxvfr::Vf8, GxVfr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x1,1,0,gxvfr::Vf8, GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf9(
            self,
        ) -> crate::common::RegisterField<9, 0x1, 1, 0, gxvfr::Vf9, GxVfr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<9,0x1,1,0,gxvfr::Vf9, GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf10(
            self,
        ) -> crate::common::RegisterField<10, 0x1, 1, 0, gxvfr::Vf10, GxVfr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<10,0x1,1,0,gxvfr::Vf10, GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf11(
            self,
        ) -> crate::common::RegisterField<11, 0x1, 1, 0, gxvfr::Vf11, GxVfr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<11,0x1,1,0,gxvfr::Vf11, GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf12(
            self,
        ) -> crate::common::RegisterField<12, 0x1, 1, 0, gxvfr::Vf12, GxVfr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0x1,1,0,gxvfr::Vf12, GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf13(
            self,
        ) -> crate::common::RegisterField<13, 0x1, 1, 0, gxvfr::Vf13, GxVfr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<13,0x1,1,0,gxvfr::Vf13, GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf14(
            self,
        ) -> crate::common::RegisterField<14, 0x1, 1, 0, gxvfr::Vf14, GxVfr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<14,0x1,1,0,gxvfr::Vf14, GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Valid Flag of Result Register x   VF15. Indicates a new result in bitfield RESULT."]
        #[inline(always)]
        pub fn vf15(
            self,
        ) -> crate::common::RegisterField<15, 0x1, 1, 0, gxvfr::Vf15, GxVfr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<15,0x1,1,0,gxvfr::Vf15, GxVfr_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GxVfr {
        #[inline(always)]
        fn default() -> GxVfr {
            <crate::RegValueT<GxVfr_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod gxvfr {
        pub struct Vf0_SPEC;
        pub type Vf0 = crate::EnumBitfieldStruct<u8, Vf0_SPEC>;
        impl Vf0 {
            #[doc = "No new valid data available. Write access  No effect"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Result register x contains valid data and has not yet been read. Write access  Clear this valid flag and bitfield DRC in register GxRESy         overrides a hardware set action ."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Vf1_SPEC;
        pub type Vf1 = crate::EnumBitfieldStruct<u8, Vf1_SPEC>;
        impl Vf1 {
            #[doc = "No new valid data available. Write access  No effect"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Result register x contains valid data and has not yet been read. Write access  Clear this valid flag and bitfield DRC in register GxRESy         overrides a hardware set action ."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Vf2_SPEC;
        pub type Vf2 = crate::EnumBitfieldStruct<u8, Vf2_SPEC>;
        impl Vf2 {
            #[doc = "No new valid data available. Write access  No effect"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Result register x contains valid data and has not yet been read. Write access  Clear this valid flag and bitfield DRC in register GxRESy         overrides a hardware set action ."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Vf3_SPEC;
        pub type Vf3 = crate::EnumBitfieldStruct<u8, Vf3_SPEC>;
        impl Vf3 {
            #[doc = "No new valid data available. Write access  No effect"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Result register x contains valid data and has not yet been read. Write access  Clear this valid flag and bitfield DRC in register GxRESy         overrides a hardware set action ."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Vf4_SPEC;
        pub type Vf4 = crate::EnumBitfieldStruct<u8, Vf4_SPEC>;
        impl Vf4 {
            #[doc = "No new valid data available. Write access  No effect"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Result register x contains valid data and has not yet been read. Write access  Clear this valid flag and bitfield DRC in register GxRESy         overrides a hardware set action ."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Vf5_SPEC;
        pub type Vf5 = crate::EnumBitfieldStruct<u8, Vf5_SPEC>;
        impl Vf5 {
            #[doc = "No new valid data available. Write access  No effect"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Result register x contains valid data and has not yet been read. Write access  Clear this valid flag and bitfield DRC in register GxRESy         overrides a hardware set action ."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Vf6_SPEC;
        pub type Vf6 = crate::EnumBitfieldStruct<u8, Vf6_SPEC>;
        impl Vf6 {
            #[doc = "No new valid data available. Write access  No effect"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Result register x contains valid data and has not yet been read. Write access  Clear this valid flag and bitfield DRC in register GxRESy         overrides a hardware set action ."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Vf7_SPEC;
        pub type Vf7 = crate::EnumBitfieldStruct<u8, Vf7_SPEC>;
        impl Vf7 {
            #[doc = "No new valid data available. Write access  No effect"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Result register x contains valid data and has not yet been read. Write access  Clear this valid flag and bitfield DRC in register GxRESy         overrides a hardware set action ."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Vf8_SPEC;
        pub type Vf8 = crate::EnumBitfieldStruct<u8, Vf8_SPEC>;
        impl Vf8 {
            #[doc = "No new valid data available. Write access  No effect"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Result register x contains valid data and has not yet been read. Write access  Clear this valid flag and bitfield DRC in register GxRESy         overrides a hardware set action ."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Vf9_SPEC;
        pub type Vf9 = crate::EnumBitfieldStruct<u8, Vf9_SPEC>;
        impl Vf9 {
            #[doc = "No new valid data available. Write access  No effect"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Result register x contains valid data and has not yet been read. Write access  Clear this valid flag and bitfield DRC in register GxRESy         overrides a hardware set action ."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Vf10_SPEC;
        pub type Vf10 = crate::EnumBitfieldStruct<u8, Vf10_SPEC>;
        impl Vf10 {
            #[doc = "No new valid data available. Write access  No effect"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Result register x contains valid data and has not yet been read. Write access  Clear this valid flag and bitfield DRC in register GxRESy         overrides a hardware set action ."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Vf11_SPEC;
        pub type Vf11 = crate::EnumBitfieldStruct<u8, Vf11_SPEC>;
        impl Vf11 {
            #[doc = "No new valid data available. Write access  No effect"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Result register x contains valid data and has not yet been read. Write access  Clear this valid flag and bitfield DRC in register GxRESy         overrides a hardware set action ."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Vf12_SPEC;
        pub type Vf12 = crate::EnumBitfieldStruct<u8, Vf12_SPEC>;
        impl Vf12 {
            #[doc = "No new valid data available. Write access  No effect"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Result register x contains valid data and has not yet been read. Write access  Clear this valid flag and bitfield DRC in register GxRESy         overrides a hardware set action ."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Vf13_SPEC;
        pub type Vf13 = crate::EnumBitfieldStruct<u8, Vf13_SPEC>;
        impl Vf13 {
            #[doc = "No new valid data available. Write access  No effect"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Result register x contains valid data and has not yet been read. Write access  Clear this valid flag and bitfield DRC in register GxRESy         overrides a hardware set action ."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Vf14_SPEC;
        pub type Vf14 = crate::EnumBitfieldStruct<u8, Vf14_SPEC>;
        impl Vf14 {
            #[doc = "No new valid data available. Write access  No effect"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Result register x contains valid data and has not yet been read. Write access  Clear this valid flag and bitfield DRC in register GxRESy         overrides a hardware set action ."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Vf15_SPEC;
        pub type Vf15 = crate::EnumBitfieldStruct<u8, Vf15_SPEC>;
        impl Vf15 {
            #[doc = "No new valid data available. Write access  No effect"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Result register x contains valid data and has not yet been read. Write access  Clear this valid flag and bitfield DRC in register GxRESy         overrides a hardware set action ."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc = "Q"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Q(pub(super) *mut u8);
    unsafe impl core::marker::Send for Q {}
    unsafe impl core::marker::Sync for Q {}
    impl Q {
        #[doc = "Queue 0 Register 0  Group 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn gxq0ri(&self) -> crate::common::Reg<q::GxQ0Ri_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
        }
        #[doc = "Queue 0 Backup Register  Group 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn gxqburi(&self) -> crate::common::Reg<q::GxQbuRi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
        }
        #[doc = "Queue 0 Source Contr. Register  Group 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn gxqctrli(&self) -> crate::common::Reg<q::GxQctrLi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "Queue 0 Input Register  Group 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn gxqinri(&self) -> crate::common::Reg<q::GxQinRi_SPEC, crate::common::W> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "Queue 0 Mode Register  Group 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn gxqmri(&self) -> crate::common::Reg<q::GxQmRi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
        }
        #[doc = "Queue 0 Status Register  Group 0\n resetvalue={Application Reset:0x20}"]
        #[inline(always)]
        pub const fn gxqsri(&self) -> crate::common::Reg<q::GxQsRi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "Queue 0 Requ. Timer Mode Reg.  Group 0\n resetvalue={Application Reset:0x0FFC00000}"]
        #[inline(always)]
        pub const fn gxreqtmi(&self) -> crate::common::Reg<q::GxReqtMi_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
        }
        #[doc = "Queue 0 Requ. Timer Status Reg.  Group 0\n resetvalue={Application Reset:0x0}"]
        #[inline(always)]
        pub const fn gxreqtsi(&self) -> crate::common::Reg<q::GxReqtSi_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
        }
    }
    pub mod q {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GxQ0Ri_SPEC;
        impl crate::sealed::RegSpec for GxQ0Ri_SPEC {
            type DataType = u32;
        }
        #[doc = "Queue 0 Register 0  Group 0\n resetvalue={Application Reset:0x0}"]
        pub type GxQ0Ri = crate::RegValueT<GxQ0Ri_SPEC>;

        impl GxQ0Ri {
            #[doc = "Request Channel Number   REQCHNR. Indicates the channel number to be converted."]
            #[inline(always)]
            pub fn reqchnr(
                self,
            ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, GxQ0Ri_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0x1f,1,0,u8, GxQ0Ri_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Refill   RF. Indicates the handling of handled requests."]
            #[inline(always)]
            pub fn rf(
                self,
            ) -> crate::common::RegisterField<5, 0x1, 1, 0, gxq0ri::Rf, GxQ0Ri_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<
                    5,
                    0x1,
                    1,
                    0,
                    gxq0ri::Rf,
                    GxQ0Ri_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Enable Source Interrupt   ENSI"]
            #[inline(always)]
            pub fn ensi(
                self,
            ) -> crate::common::RegisterField<
                6,
                0x1,
                1,
                0,
                gxq0ri::Ensi,
                GxQ0Ri_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    6,
                    0x1,
                    1,
                    0,
                    gxq0ri::Ensi,
                    GxQ0Ri_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "External Trigger   EXTR. Enables external trigger events."]
            #[inline(always)]
            pub fn extr(
                self,
            ) -> crate::common::RegisterField<
                7,
                0x1,
                1,
                0,
                gxq0ri::Extr,
                GxQ0Ri_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    7,
                    0x1,
                    1,
                    0,
                    gxq0ri::Extr,
                    GxQ0Ri_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Request Channel Number Valid   V. Indicates a valid queue entry in queue register 0."]
            #[inline(always)]
            pub fn v(
                self,
            ) -> crate::common::RegisterField<8, 0x1, 1, 0, gxq0ri::V, GxQ0Ri_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<8,0x1,1,0,gxq0ri::V, GxQ0Ri_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Pull Down Diagnostics Enable   PDD. Channels with pull down diagnostics device are marked in the          product specific appendix."]
            #[inline(always)]
            pub fn pdd(
                self,
            ) -> crate::common::RegisterField<
                9,
                0x1,
                1,
                0,
                gxq0ri::Pdd,
                GxQ0Ri_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    9,
                    0x1,
                    1,
                    0,
                    gxq0ri::Pdd,
                    GxQ0Ri_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Multiplexer Diagnostics Pull Devices Enable   MDPD MDPU. Connecting combinations of pull up and or pull down devices generate        various loads for testing. Channels with multiplexer diagnostics pull devices are marked in the          product specific appendix."]
            #[inline(always)]
            pub fn mdpd(
                self,
            ) -> crate::common::RegisterField<
                10,
                0x1,
                1,
                0,
                gxq0ri::Mdpd,
                GxQ0Ri_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    10,
                    0x1,
                    1,
                    0,
                    gxq0ri::Mdpd,
                    GxQ0Ri_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Multiplexer Diagnostics Pull Devices Enable   MDPD MDPU. Connecting combinations of pull up and or pull down devices generate        various loads for testing. Channels with multiplexer diagnostics pull devices are marked in the          product specific appendix."]
            #[inline(always)]
            pub fn mdpu(
                self,
            ) -> crate::common::RegisterField<
                11,
                0x1,
                1,
                0,
                gxq0ri::Mdpu,
                GxQ0Ri_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    11,
                    0x1,
                    1,
                    0,
                    gxq0ri::Mdpu,
                    GxQ0Ri_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Converter Diagnostics Enable   CDEN"]
            #[inline(always)]
            pub fn cden(
                self,
            ) -> crate::common::RegisterField<
                12,
                0x1,
                1,
                0,
                gxq0ri::Cden,
                GxQ0Ri_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    12,
                    0x1,
                    1,
                    0,
                    gxq0ri::Cden,
                    GxQ0Ri_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Converter Diagnostics Pull Devices Select   CDSEL"]
            #[inline(always)]
            pub fn cdsel(
                self,
            ) -> crate::common::RegisterField<
                13,
                0x3,
                1,
                0,
                gxq0ri::Cdsel,
                GxQ0Ri_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    13,
                    0x3,
                    1,
                    0,
                    gxq0ri::Cdsel,
                    GxQ0Ri_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for GxQ0Ri {
            #[inline(always)]
            fn default() -> GxQ0Ri {
                <crate::RegValueT<GxQ0Ri_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod gxq0ri {
            pub struct Rf_SPEC;
            pub type Rf = crate::EnumBitfieldStruct<u8, Rf_SPEC>;
            impl Rf {
                #[doc = "0 The request is discarded after the conversion start."]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 The request is automatically refilled into the queue after the conversion start."]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ensi_SPEC;
            pub type Ensi = crate::EnumBitfieldStruct<u8, Ensi_SPEC>;
            impl Ensi {
                #[doc = "0 No request source interrupt"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 A request source event interrupt is generated upon a request source event  related conversion is finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Extr_SPEC;
            pub type Extr = crate::EnumBitfieldStruct<u8, Extr_SPEC>;
            impl Extr {
                #[doc = "0 A valid queue entry immediately leads to a conversion request"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 The request handler waits for a trigger event"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct V_SPEC;
            pub type V = crate::EnumBitfieldStruct<u8, V_SPEC>;
            impl V {
                #[doc = "0 No valid queue entry"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 The queue entry is valid and leads to a conversion request"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Pdd_SPEC;
            pub type Pdd = crate::EnumBitfieldStruct<u8, Pdd_SPEC>;
            impl Pdd {
                #[doc = "0 Disconnected"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 The pull down diagnostics device is active"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Mdpd_SPEC;
            pub type Mdpd = crate::EnumBitfieldStruct<u8, Mdpd_SPEC>;
            impl Mdpd {
                #[doc = "0 Disconnected"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 The respective device is active"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Mdpu_SPEC;
            pub type Mdpu = crate::EnumBitfieldStruct<u8, Mdpu_SPEC>;
            impl Mdpu {
                #[doc = "0 Disconnected"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 The respective device is active"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cden_SPEC;
            pub type Cden = crate::EnumBitfieldStruct<u8, Cden_SPEC>;
            impl Cden {
                #[doc = "0 All diagnostic pull devices are disconnected"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Diagnostic pull devices connected as selected by bitfield CDSEL"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cdsel_SPEC;
            pub type Cdsel = crate::EnumBitfieldStruct<u8, Cdsel_SPEC>;
            impl Cdsel {
                #[doc = "00 Connected to VDDM"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "01 Connected to VSSM"]
                pub const CONST_11: Self = Self::new(1);
                #[doc = "10 Connected to 1 2 VDDM"]
                pub const CONST_22: Self = Self::new(2);
                #[doc = "11 Connected to 2 3rd VDDM"]
                pub const CONST_33: Self = Self::new(3);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GxQbuRi_SPEC;
        impl crate::sealed::RegSpec for GxQbuRi_SPEC {
            type DataType = u32;
        }
        #[doc = "Queue 0 Backup Register  Group 0\n resetvalue={Application Reset:0x0}"]
        pub type GxQbuRi = crate::RegValueT<GxQbuRi_SPEC>;

        impl GxQbuRi {
            #[doc = "Request Channel Number   REQCHNR. The channel number of the aborted conversion that has been requested by        this request source"]
            #[inline(always)]
            pub fn reqchnr(
                self,
            ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, GxQbuRi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0x1f,1,0,u8, GxQbuRi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Refill   RF. The refill control bit of the aborted conversion"]
            #[inline(always)]
            pub fn rf(
                self,
            ) -> crate::common::RegisterFieldBool<5, 1, 0, GxQbuRi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<5,1,0,GxQbuRi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Enable Source Interrupt   ENSI. The enable source interrupt control bit of the aborted conversion"]
            #[inline(always)]
            pub fn ensi(
                self,
            ) -> crate::common::RegisterFieldBool<6, 1, 0, GxQbuRi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<6,1,0,GxQbuRi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "External Trigger   EXTR. The external trigger control bit of the aborted conversion"]
            #[inline(always)]
            pub fn extr(
                self,
            ) -> crate::common::RegisterFieldBool<7, 1, 0, GxQbuRi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<7,1,0,GxQbuRi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Request Channel Number Valid   V. Indicates if the entry  REQCHNR  RF  TR  ENSI  in the queue backup        register is valid. Bit V is set when a running conversion  that has been        requested by this request source  is aborted  it is cleared when the        aborted conversion is restarted."]
            #[inline(always)]
            pub fn v(
                self,
            ) -> crate::common::RegisterField<
                8,
                0x1,
                1,
                0,
                gxqburi::V,
                GxQbuRi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    8,
                    0x1,
                    1,
                    0,
                    gxqburi::V,
                    GxQbuRi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Pull Down Diagnostics Enable   PDD. Channels with pull down diagnostics device are marked in the          product specific appendix."]
            #[inline(always)]
            pub fn pdd(
                self,
            ) -> crate::common::RegisterField<
                9,
                0x1,
                1,
                0,
                gxqburi::Pdd,
                GxQbuRi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    9,
                    0x1,
                    1,
                    0,
                    gxqburi::Pdd,
                    GxQbuRi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Multiplexer Diagnostics Pull Down Devices Enable. Connecting combinations of pull up and or pull down devices generate        various loads for testing. Channels with multiplexer diagnostics pull devices are marked in the          product specific appendix."]
            #[inline(always)]
            pub fn mdpd(
                self,
            ) -> crate::common::RegisterField<
                10,
                0x1,
                1,
                0,
                gxqburi::Mdpd,
                GxQbuRi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    10,
                    0x1,
                    1,
                    0,
                    gxqburi::Mdpd,
                    GxQbuRi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Multiplexer Diagnostics Pull Up Devices Enable. Connecting combinations of pull up and or pull down devices generate        various loads for testing. Channels with multiplexer diagnostics pull devices are marked in the          product specific appendix."]
            #[inline(always)]
            pub fn mdpu(
                self,
            ) -> crate::common::RegisterField<
                11,
                0x1,
                1,
                0,
                gxqburi::Mdpu,
                GxQbuRi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    11,
                    0x1,
                    1,
                    0,
                    gxqburi::Mdpu,
                    GxQbuRi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Converter Diagnostics Enable   CDEN"]
            #[inline(always)]
            pub fn cden(
                self,
            ) -> crate::common::RegisterField<
                12,
                0x1,
                1,
                0,
                gxqburi::Cden,
                GxQbuRi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    12,
                    0x1,
                    1,
                    0,
                    gxqburi::Cden,
                    GxQbuRi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Converter Diagnostics Pull Devices Select   CDSEL"]
            #[inline(always)]
            pub fn cdsel(
                self,
            ) -> crate::common::RegisterField<
                13,
                0x3,
                1,
                0,
                gxqburi::Cdsel,
                GxQbuRi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    13,
                    0x3,
                    1,
                    0,
                    gxqburi::Cdsel,
                    GxQbuRi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for GxQbuRi {
            #[inline(always)]
            fn default() -> GxQbuRi {
                <crate::RegValueT<GxQbuRi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod gxqburi {
            pub struct V_SPEC;
            pub type V = crate::EnumBitfieldStruct<u8, V_SPEC>;
            impl V {
                #[doc = "Backup register not valid"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "Backup register contains a valid entry.. This will be requested before a valid entry in queue register 0  stage        0  will be requested."]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Pdd_SPEC;
            pub type Pdd = crate::EnumBitfieldStruct<u8, Pdd_SPEC>;
            impl Pdd {
                #[doc = "Disconnected"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "The pull down diagnostics device is active"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Mdpd_SPEC;
            pub type Mdpd = crate::EnumBitfieldStruct<u8, Mdpd_SPEC>;
            impl Mdpd {
                #[doc = "Disconnected"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "The respective device is active"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Mdpu_SPEC;
            pub type Mdpu = crate::EnumBitfieldStruct<u8, Mdpu_SPEC>;
            impl Mdpu {
                #[doc = "Disconnected"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "The respective device is active"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cden_SPEC;
            pub type Cden = crate::EnumBitfieldStruct<u8, Cden_SPEC>;
            impl Cden {
                #[doc = "All diagnostic pull devices are disconnected"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "Diagnostic pull devices connected as selected by bitfield CDSEL"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cdsel_SPEC;
            pub type Cdsel = crate::EnumBitfieldStruct<u8, Cdsel_SPEC>;
            impl Cdsel {
                #[doc = "Connected to VDDM"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "Connected to VSSM"]
                pub const CONST_11: Self = Self::new(1);
                #[doc = "Connected to 1 2 VDDM"]
                pub const CONST_22: Self = Self::new(2);
                #[doc = "Connected to 2 3rd VDDM"]
                pub const CONST_33: Self = Self::new(3);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GxQctrLi_SPEC;
        impl crate::sealed::RegSpec for GxQctrLi_SPEC {
            type DataType = u32;
        }
        #[doc = "Queue 0 Source Contr. Register  Group 0\n resetvalue={Application Reset:0x0}"]
        pub type GxQctrLi = crate::RegValueT<GxQctrLi_SPEC>;

        impl GxQctrLi {
            #[doc = "Source specific Result Register   SRCRESREG"]
            #[inline(always)]
            pub fn srcresreg(
                self,
            ) -> crate::common::RegisterField<
                0,
                0xf,
                1,
                0,
                gxqctrli::Srcresreg,
                GxQctrLi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    0,
                    0xf,
                    1,
                    0,
                    gxqctrli::Srcresreg,
                    GxQctrLi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Trigger Source Selection   TRSEL. Daisy chaining via source events from the corresponding adjacent request source Qi. Use XTMODE   10 B ."]
            #[inline(always)]
            pub fn trsel(
                self,
            ) -> crate::common::RegisterField<
                6,
                0x3,
                1,
                0,
                gxqctrli::Trsel,
                GxQctrLi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    6,
                    0x3,
                    1,
                    0,
                    gxqctrli::Trsel,
                    GxQctrLi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "External Trigger Input Selection   XTSEL. The connected trigger input signals are listed in the product specific        appendix. XTSEL  160    160  1111 uses the          selected gate input as trigger source  ENGT must be 0X  ."]
            #[inline(always)]
            pub fn xtsel(
                self,
            ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, GxQctrLi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<8,0xf,1,0,u8, GxQctrLi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "External Trigger Level   XTLVL. Current level of the selected trigger input"]
            #[inline(always)]
            pub fn xtlvl(
                self,
            ) -> crate::common::RegisterFieldBool<12, 1, 0, GxQctrLi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<12,1,0,GxQctrLi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Trigger Operating Mode   XTMODE"]
            #[inline(always)]
            pub fn xtmode(
                self,
            ) -> crate::common::RegisterField<
                13,
                0x3,
                1,
                0,
                gxqctrli::Xtmode,
                GxQctrLi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    13,
                    0x3,
                    1,
                    0,
                    gxqctrli::Xtmode,
                    GxQctrLi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Write Control for Trigger Configuration   XTWC"]
            #[inline(always)]
            pub fn xtwc(
                self,
            ) -> crate::common::RegisterField<
                15,
                0x1,
                1,
                0,
                gxqctrli::Xtwc,
                GxQctrLi_SPEC,
                crate::common::W,
            > {
                crate::common::RegisterField::<
                    15,
                    0x1,
                    1,
                    0,
                    gxqctrli::Xtwc,
                    GxQctrLi_SPEC,
                    crate::common::W,
                >::from_register(self, 0)
            }
            #[doc = "Gate Input Selection   GTSEL. The connected gate input signals are listed in the product specific        appendix. GTSEL  160    160  1111 uses the          selected internal trigger source for queue 2."]
            #[inline(always)]
            pub fn gtsel(
                self,
            ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, GxQctrLi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0xf,1,0,u8, GxQctrLi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Gate Input Level   GTLVL. Current level of the selected gate input"]
            #[inline(always)]
            pub fn gtlvl(
                self,
            ) -> crate::common::RegisterFieldBool<20, 1, 0, GxQctrLi_SPEC, crate::common::R>
            {
                crate::common::RegisterFieldBool::<20,1,0,GxQctrLi_SPEC,crate::common::R>::from_register(self,0)
            }
            #[doc = "Write Control for Gate Configuration   GTWC"]
            #[inline(always)]
            pub fn gtwc(
                self,
            ) -> crate::common::RegisterField<
                23,
                0x1,
                1,
                0,
                gxqctrli::Gtwc,
                GxQctrLi_SPEC,
                crate::common::W,
            > {
                crate::common::RegisterField::<
                    23,
                    0x1,
                    1,
                    0,
                    gxqctrli::Gtwc,
                    GxQctrLi_SPEC,
                    crate::common::W,
                >::from_register(self, 0)
            }
            #[doc = "Timer Mode Enable   TMEN"]
            #[inline(always)]
            pub fn tmen(
                self,
            ) -> crate::common::RegisterField<
                28,
                0x1,
                1,
                0,
                gxqctrli::Tmen,
                GxQctrLi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    28,
                    0x1,
                    1,
                    0,
                    gxqctrli::Tmen,
                    GxQctrLi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Write Control for Timer Mode   TMWC"]
            #[inline(always)]
            pub fn tmwc(
                self,
            ) -> crate::common::RegisterField<
                31,
                0x1,
                1,
                0,
                gxqctrli::Tmwc,
                GxQctrLi_SPEC,
                crate::common::W,
            > {
                crate::common::RegisterField::<
                    31,
                    0x1,
                    1,
                    0,
                    gxqctrli::Tmwc,
                    GxQctrLi_SPEC,
                    crate::common::W,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for GxQctrLi {
            #[inline(always)]
            fn default() -> GxQctrLi {
                <crate::RegValueT<GxQctrLi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod gxqctrli {
            pub struct Srcresreg_SPEC;
            pub type Srcresreg = crate::EnumBitfieldStruct<u8, Srcresreg_SPEC>;
            impl Srcresreg {
                #[doc = "Use GxCHCTRy.RESREG RESTGT to select a result register"]
                pub const CONST_00: Self = Self::new(0);
            }
            pub struct Trsel_SPEC;
            pub type Trsel = crate::EnumBitfieldStruct<u8, Trsel_SPEC>;
            impl Trsel {
                #[doc = "External trigger  as selected by XTSEL"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "Source event of next lower group  for G0  highest available group"]
                pub const CONST_11: Self = Self::new(1);
                #[doc = "Source event of lowest group within cluster  primary or secondary   for lowest group  highest group within cluster"]
                pub const CONST_22: Self = Self::new(2);
                #[doc = "Source event of next higher group  for highest available group  G0"]
                pub const CONST_33: Self = Self::new(3);
            }
            pub struct Xtmode_SPEC;
            pub type Xtmode = crate::EnumBitfieldStruct<u8, Xtmode_SPEC>;
            impl Xtmode {
                #[doc = "No external trigger"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "Trigger event upon a falling edge"]
                pub const CONST_11: Self = Self::new(1);
                #[doc = "Trigger event upon a rising edge"]
                pub const CONST_22: Self = Self::new(2);
                #[doc = "Trigger event upon any edge"]
                pub const CONST_33: Self = Self::new(3);
            }
            pub struct Xtwc_SPEC;
            pub type Xtwc = crate::EnumBitfieldStruct<u8, Xtwc_SPEC>;
            impl Xtwc {
                #[doc = "No write access to trigger configuration"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "Bitfields XTMODE  XTSEL  TRSEL can be written"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Gtwc_SPEC;
            pub type Gtwc = crate::EnumBitfieldStruct<u8, Gtwc_SPEC>;
            impl Gtwc {
                #[doc = "No write access to gate configuration"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "Bitfield GTSEL can be written"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tmen_SPEC;
            pub type Tmen = crate::EnumBitfieldStruct<u8, Tmen_SPEC>;
            impl Tmen {
                #[doc = "No timer mode  standard gating mechanism can be used"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "Timer mode for equidistant sampling enabled  standard gating mechanism must be disabled"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Tmwc_SPEC;
            pub type Tmwc = crate::EnumBitfieldStruct<u8, Tmwc_SPEC>;
            impl Tmwc {
                #[doc = "No write access to timer mode"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "Bitfield TMEN can be written"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GxQinRi_SPEC;
        impl crate::sealed::RegSpec for GxQinRi_SPEC {
            type DataType = u32;
        }
        #[doc = "Queue 0 Input Register  Group 0\n resetvalue={Application Reset:0x0}"]
        pub type GxQinRi = crate::RegValueT<GxQinRi_SPEC>;

        impl GxQinRi {
            #[doc = "Request Channel Number   REQCHNR. Defines the channel number to be converted. Not available channel numbers are treated as channel 0."]
            #[inline(always)]
            pub fn reqchnr(
                self,
            ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, GxQinRi_SPEC, crate::common::W>
            {
                crate::common::RegisterField::<0,0x1f,1,0,u8, GxQinRi_SPEC,crate::common::W>::from_register(self,0)
            }
            #[doc = "Refill   RF"]
            #[inline(always)]
            pub fn rf(
                self,
            ) -> crate::common::RegisterField<
                5,
                0x1,
                1,
                0,
                gxqinri::Rf,
                GxQinRi_SPEC,
                crate::common::W,
            > {
                crate::common::RegisterField::<
                    5,
                    0x1,
                    1,
                    0,
                    gxqinri::Rf,
                    GxQinRi_SPEC,
                    crate::common::W,
                >::from_register(self, 0)
            }
            #[doc = "Enable Source Interrupt   ENSI"]
            #[inline(always)]
            pub fn ensi(
                self,
            ) -> crate::common::RegisterField<
                6,
                0x1,
                1,
                0,
                gxqinri::Ensi,
                GxQinRi_SPEC,
                crate::common::W,
            > {
                crate::common::RegisterField::<
                    6,
                    0x1,
                    1,
                    0,
                    gxqinri::Ensi,
                    GxQinRi_SPEC,
                    crate::common::W,
                >::from_register(self, 0)
            }
            #[doc = "External Trigger   EXTR. Enables the external trigger functionality. To use external triggers  enable them by setting bit GxQMRy.ENTR."]
            #[inline(always)]
            pub fn extr(
                self,
            ) -> crate::common::RegisterField<
                7,
                0x1,
                1,
                0,
                gxqinri::Extr,
                GxQinRi_SPEC,
                crate::common::W,
            > {
                crate::common::RegisterField::<
                    7,
                    0x1,
                    1,
                    0,
                    gxqinri::Extr,
                    GxQinRi_SPEC,
                    crate::common::W,
                >::from_register(self, 0)
            }
            #[doc = "Pull Down Diagnostics Enable   PDD. Channels with pull down diagnostics device are marked in the          product specific appendix."]
            #[inline(always)]
            pub fn pdd(
                self,
            ) -> crate::common::RegisterField<
                9,
                0x1,
                1,
                0,
                gxqinri::Pdd,
                GxQinRi_SPEC,
                crate::common::W,
            > {
                crate::common::RegisterField::<
                    9,
                    0x1,
                    1,
                    0,
                    gxqinri::Pdd,
                    GxQinRi_SPEC,
                    crate::common::W,
                >::from_register(self, 0)
            }
            #[doc = "Multiplexer Diagnostics Pull Devices Enable   MDPD MDPU. Connecting combinations of pull up and or pull down devices generate        various loads for testing. Channels with multiplexer diagnostics pull devices are marked in the          product specific appendix."]
            #[inline(always)]
            pub fn mdpd(
                self,
            ) -> crate::common::RegisterField<
                10,
                0x1,
                1,
                0,
                gxqinri::Mdpd,
                GxQinRi_SPEC,
                crate::common::W,
            > {
                crate::common::RegisterField::<
                    10,
                    0x1,
                    1,
                    0,
                    gxqinri::Mdpd,
                    GxQinRi_SPEC,
                    crate::common::W,
                >::from_register(self, 0)
            }
            #[doc = "Multiplexer Diagnostics Pull Devices Enable   MDPD MDPU. Connecting combinations of pull up and or pull down devices generate        various loads for testing. Channels with multiplexer diagnostics pull devices are marked in the          product specific appendix."]
            #[inline(always)]
            pub fn mdpu(
                self,
            ) -> crate::common::RegisterField<
                11,
                0x1,
                1,
                0,
                gxqinri::Mdpu,
                GxQinRi_SPEC,
                crate::common::W,
            > {
                crate::common::RegisterField::<
                    11,
                    0x1,
                    1,
                    0,
                    gxqinri::Mdpu,
                    GxQinRi_SPEC,
                    crate::common::W,
                >::from_register(self, 0)
            }
            #[doc = "Converter Diagnostics Enable   CDEN"]
            #[inline(always)]
            pub fn cden(
                self,
            ) -> crate::common::RegisterField<
                12,
                0x1,
                1,
                0,
                gxqinri::Cden,
                GxQinRi_SPEC,
                crate::common::W,
            > {
                crate::common::RegisterField::<
                    12,
                    0x1,
                    1,
                    0,
                    gxqinri::Cden,
                    GxQinRi_SPEC,
                    crate::common::W,
                >::from_register(self, 0)
            }
            #[doc = "Converter Diagnostics Pull Devices Select   CDSEL"]
            #[inline(always)]
            pub fn cdsel(
                self,
            ) -> crate::common::RegisterField<
                13,
                0x3,
                1,
                0,
                gxqinri::Cdsel,
                GxQinRi_SPEC,
                crate::common::W,
            > {
                crate::common::RegisterField::<
                    13,
                    0x3,
                    1,
                    0,
                    gxqinri::Cdsel,
                    GxQinRi_SPEC,
                    crate::common::W,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for GxQinRi {
            #[inline(always)]
            fn default() -> GxQinRi {
                <crate::RegValueT<GxQinRi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod gxqinri {
            pub struct Rf_SPEC;
            pub type Rf = crate::EnumBitfieldStruct<u8, Rf_SPEC>;
            impl Rf {
                #[doc = "No refill  this queue entry is converted once and then invalidated"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "Automatic refill  this queue entry is automatically reloaded into QINRx when the related conversion is started"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ensi_SPEC;
            pub type Ensi = crate::EnumBitfieldStruct<u8, Ensi_SPEC>;
            impl Ensi {
                #[doc = "No request source interrupt"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "A request source event interrupt is generated upon a request source event.  related conversion is finished"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Extr_SPEC;
            pub type Extr = crate::EnumBitfieldStruct<u8, Extr_SPEC>;
            impl Extr {
                #[doc = "A valid queue entry immediately leads to a conversion request."]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "A valid queue entry waits for a trigger event to occur before issuing a conversion request."]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Pdd_SPEC;
            pub type Pdd = crate::EnumBitfieldStruct<u8, Pdd_SPEC>;
            impl Pdd {
                #[doc = "Disconnected"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "The pull down diagnostics device is active"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Mdpd_SPEC;
            pub type Mdpd = crate::EnumBitfieldStruct<u8, Mdpd_SPEC>;
            impl Mdpd {
                #[doc = "Disconnected"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "The respective device is active"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Mdpu_SPEC;
            pub type Mdpu = crate::EnumBitfieldStruct<u8, Mdpu_SPEC>;
            impl Mdpu {
                #[doc = "Disconnected"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "The respective device is active"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cden_SPEC;
            pub type Cden = crate::EnumBitfieldStruct<u8, Cden_SPEC>;
            impl Cden {
                #[doc = "All diagnostic pull devices are disconnected"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "Diagnostic pull devices connected as selected by bitfield CDSEL"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cdsel_SPEC;
            pub type Cdsel = crate::EnumBitfieldStruct<u8, Cdsel_SPEC>;
            impl Cdsel {
                #[doc = "Connected to VDDM"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "Connected to VSSM"]
                pub const CONST_11: Self = Self::new(1);
                #[doc = "Connected to 1 2 VDDM"]
                pub const CONST_22: Self = Self::new(2);
                #[doc = "Connected to 2 3rd VDDM"]
                pub const CONST_33: Self = Self::new(3);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GxQmRi_SPEC;
        impl crate::sealed::RegSpec for GxQmRi_SPEC {
            type DataType = u32;
        }
        #[doc = "Queue 0 Mode Register  Group 0\n resetvalue={Application Reset:0x0}"]
        pub type GxQmRi = crate::RegValueT<GxQmRi_SPEC>;

        impl GxQmRi {
            #[doc = "Enable Gate   ENGT. Selects the gating functionality for the request source. REQGTx is the selected gating signal."]
            #[inline(always)]
            pub fn engt(
                self,
            ) -> crate::common::RegisterField<
                0,
                0x3,
                1,
                0,
                gxqmri::Engt,
                GxQmRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    0,
                    0x3,
                    1,
                    0,
                    gxqmri::Engt,
                    GxQmRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Enable External Trigger   ENTR"]
            #[inline(always)]
            pub fn entr(
                self,
            ) -> crate::common::RegisterField<
                2,
                0x1,
                1,
                0,
                gxqmri::Entr,
                GxQmRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    2,
                    0x1,
                    1,
                    0,
                    gxqmri::Entr,
                    GxQmRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Clear Valid Bit   CLRV"]
            #[inline(always)]
            pub fn clrv(
                self,
            ) -> crate::common::RegisterField<
                8,
                0x1,
                1,
                0,
                gxqmri::Clrv,
                GxQmRi_SPEC,
                crate::common::W,
            > {
                crate::common::RegisterField::<
                    8,
                    0x1,
                    1,
                    0,
                    gxqmri::Clrv,
                    GxQmRi_SPEC,
                    crate::common::W,
                >::from_register(self, 0)
            }
            #[doc = "Trigger Event   TREV. Generates a software trigger for the request source. To trigger the request timer instead  write 1 to bit GxREQTIMi.REQTS."]
            #[inline(always)]
            pub fn trev(
                self,
            ) -> crate::common::RegisterField<
                9,
                0x1,
                1,
                0,
                gxqmri::Trev,
                GxQmRi_SPEC,
                crate::common::W,
            > {
                crate::common::RegisterField::<
                    9,
                    0x1,
                    1,
                    0,
                    gxqmri::Trev,
                    GxQmRi_SPEC,
                    crate::common::W,
                >::from_register(self, 0)
            }
            #[doc = "Flush Queue   FLUSH"]
            #[inline(always)]
            pub fn flush(
                self,
            ) -> crate::common::RegisterField<
                10,
                0x1,
                1,
                0,
                gxqmri::Flush,
                GxQmRi_SPEC,
                crate::common::W,
            > {
                crate::common::RegisterField::<
                    10,
                    0x1,
                    1,
                    0,
                    gxqmri::Flush,
                    GxQmRi_SPEC,
                    crate::common::W,
                >::from_register(self, 0)
            }
            #[doc = "Clear Event Flag   CEV"]
            #[inline(always)]
            pub fn cev(
                self,
            ) -> crate::common::RegisterField<
                11,
                0x1,
                1,
                0,
                gxqmri::Cev,
                GxQmRi_SPEC,
                crate::common::W,
            > {
                crate::common::RegisterField::<
                    11,
                    0x1,
                    1,
                    0,
                    gxqmri::Cev,
                    GxQmRi_SPEC,
                    crate::common::W,
                >::from_register(self, 0)
            }
            #[doc = "Repeat Disable   RPTDIS"]
            #[inline(always)]
            pub fn rptdis(
                self,
            ) -> crate::common::RegisterField<
                16,
                0x1,
                1,
                0,
                gxqmri::Rptdis,
                GxQmRi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    16,
                    0x1,
                    1,
                    0,
                    gxqmri::Rptdis,
                    GxQmRi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for GxQmRi {
            #[inline(always)]
            fn default() -> GxQmRi {
                <crate::RegValueT<GxQmRi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
        pub mod gxqmri {
            pub struct Engt_SPEC;
            pub type Engt = crate::EnumBitfieldStruct<u8, Engt_SPEC>;
            impl Engt {
                #[doc = "No conversion requests are issued"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register"]
                pub const CONST_11: Self = Self::new(1);
                #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx   1"]
                pub const CONST_22: Self = Self::new(2);
                #[doc = "Conversion requests are issued if a valid conversion request is pending in the queue 0 register or in the backup register and REQGTx   0"]
                pub const CONST_33: Self = Self::new(3);
            }
            pub struct Entr_SPEC;
            pub type Entr = crate::EnumBitfieldStruct<u8, Entr_SPEC>;
            impl Entr {
                #[doc = "External trigger disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "The selected edge at the selected trigger input signal REQTR generates the trigger event"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Clrv_SPEC;
            pub type Clrv = crate::EnumBitfieldStruct<u8, Clrv_SPEC>;
            impl Clrv {
                #[doc = "No action"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "The next pending valid queue entry in the sequence and the event flag EV are cleared.. If there is a valid entry in the queue backup register  QBUR.V  160    160 1          this entry is cleared  otherwise the entry in queue register 0 is        cleared."]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Trev_SPEC;
            pub type Trev = crate::EnumBitfieldStruct<u8, Trev_SPEC>;
            impl Trev {
                #[doc = "No action"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "Generate a trigger event by software"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Flush_SPEC;
            pub type Flush = crate::EnumBitfieldStruct<u8, Flush_SPEC>;
            impl Flush {
                #[doc = "No action"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "Clear all queue entries  incl. backup stage  and the event flag EV.. 1 The queue        contains no more valid entry."]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Cev_SPEC;
            pub type Cev = crate::EnumBitfieldStruct<u8, Cev_SPEC>;
            impl Cev {
                #[doc = "No action"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "Clear bit EV"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Rptdis_SPEC;
            pub type Rptdis = crate::EnumBitfieldStruct<u8, Rptdis_SPEC>;
            impl Rptdis {
                #[doc = "A cancelled conversion is repeated"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "A cancelled conversion is discarded"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GxQsRi_SPEC;
        impl crate::sealed::RegSpec for GxQsRi_SPEC {
            type DataType = u32;
        }
        #[doc = "Queue 0 Status Register  Group 0\n resetvalue={Application Reset:0x20}"]
        pub type GxQsRi = crate::RegValueT<GxQsRi_SPEC>;

        impl GxQsRi {
            #[doc = "Filling Level for Queue   FILL. Indicates the number of valid queue entries. It is incremented each time        a new entry is written to QINRx or by an enabled refill mechanism. It is        decremented each time a requested conversion has been started. A new        entry is ignored if the filling level has reached its maximum value. Maximum fill level for primary groups  8 entries Maximum fill level for secondary groups  16 entries"]
            #[inline(always)]
            pub fn fill(
                self,
            ) -> crate::common::RegisterField<
                0,
                0xf,
                1,
                0,
                gxqsri::Fill,
                GxQsRi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    0,
                    0xf,
                    1,
                    0,
                    gxqsri::Fill,
                    GxQsRi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Queue Empty   EMPTY"]
            #[inline(always)]
            pub fn empty(
                self,
            ) -> crate::common::RegisterField<
                5,
                0x1,
                1,
                0,
                gxqsri::Empty,
                GxQsRi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    5,
                    0x1,
                    1,
                    0,
                    gxqsri::Empty,
                    GxQsRi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Request Gate Level   REQGT. Monitors the level at the selected REQGT input."]
            #[inline(always)]
            pub fn reqgt(
                self,
            ) -> crate::common::RegisterField<
                7,
                0x1,
                1,
                0,
                gxqsri::Reqgt,
                GxQsRi_SPEC,
                crate::common::R,
            > {
                crate::common::RegisterField::<
                    7,
                    0x1,
                    1,
                    0,
                    gxqsri::Reqgt,
                    GxQsRi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
            #[doc = "Event Detected   EV. Indicates that an event has been detected while at least one valid entry        has been in the queue  queue register 0 or backup stage . Once set  this        bit is cleared automatically when the requested conversion is started."]
            #[inline(always)]
            pub fn ev(
                self,
            ) -> crate::common::RegisterField<8, 0x1, 1, 0, gxqsri::Ev, GxQsRi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<
                    8,
                    0x1,
                    1,
                    0,
                    gxqsri::Ev,
                    GxQsRi_SPEC,
                    crate::common::R,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for GxQsRi {
            #[inline(always)]
            fn default() -> GxQsRi {
                <crate::RegValueT<GxQsRi_SPEC> as RegisterValue<_>>::new(32)
            }
        }
        pub mod gxqsri {
            pub struct Fill_SPEC;
            pub type Fill = crate::EnumBitfieldStruct<u8, Fill_SPEC>;
            impl Fill {
                #[doc = "There is 1   if EMPTY   0  or no  if EMPTY   1  valid entry in the queue"]
                pub const CONST_00: Self = Self::new(0);
            }
            pub struct Empty_SPEC;
            pub type Empty = crate::EnumBitfieldStruct<u8, Empty_SPEC>;
            impl Empty {
                #[doc = "There are valid entries in the queue  see FILL"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "No valid entries  queue is empty"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Reqgt_SPEC;
            pub type Reqgt = crate::EnumBitfieldStruct<u8, Reqgt_SPEC>;
            impl Reqgt {
                #[doc = "The gate input is low"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "The gate input is high"]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Ev_SPEC;
            pub type Ev = crate::EnumBitfieldStruct<u8, Ev_SPEC>;
            impl Ev {
                #[doc = "No trigger event"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "A trigger event has been detected"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GxReqtMi_SPEC;
        impl crate::sealed::RegSpec for GxReqtMi_SPEC {
            type DataType = u32;
        }
        #[doc = "Queue 0 Requ. Timer Mode Reg.  Group 0\n resetvalue={Application Reset:0x0FFC00000}"]
        pub type GxReqtMi = crate::RegValueT<GxReqtMi_SPEC>;

        impl GxReqtMi {
            #[doc = "Sequence Mode   SEQMOD. Selects how the request timer controls the conversion sequence. Before changing the operating mode  stop the sequence timer  i.e. SEQMOD          00 ."]
            #[inline(always)]
            pub fn seqmod(
                self,
            ) -> crate::common::RegisterField<
                0,
                0x3,
                1,
                0,
                gxreqtmi::Seqmod,
                GxReqtMi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    0,
                    0x3,
                    1,
                    0,
                    gxreqtmi::Seqmod,
                    GxReqtMi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Sequence Timer  Set Value   SEQTIMSET. Initial value for the sequence timer in steps of 16   215  t PER .        This value is loaded into SEQTIMER when a new request timer period is        started."]
            #[inline(always)]
            pub fn seqtimset(
                self,
            ) -> crate::common::RegisterField<6, 0x3ff, 1, 0, u16, GxReqtMi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<6,0x3ff,1,0,u16, GxReqtMi_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Request Timer Start Trigger   REQTS"]
            #[inline(always)]
            pub fn reqts(
                self,
            ) -> crate::common::RegisterField<
                16,
                0x1,
                1,
                0,
                gxreqtmi::Reqts,
                GxReqtMi_SPEC,
                crate::common::W,
            > {
                crate::common::RegisterField::<
                    16,
                    0x1,
                    1,
                    0,
                    gxreqtmi::Reqts,
                    GxReqtMi_SPEC,
                    crate::common::W,
                >::from_register(self, 0)
            }
            #[doc = "Enable External Trigger   ENTR"]
            #[inline(always)]
            pub fn entr(
                self,
            ) -> crate::common::RegisterField<
                17,
                0x1,
                1,
                0,
                gxreqtmi::Entr,
                GxReqtMi_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    17,
                    0x1,
                    1,
                    0,
                    gxreqtmi::Entr,
                    GxReqtMi_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Sequence Timer  Switch Off Value   SEQTIMOFF. The generated trigger signal is disabled when the timer value is equal to or below this threshold."]
            #[inline(always)]
            pub fn seqtimoff(
                self,
            ) -> crate::common::RegisterField<22, 0x3ff, 1, 0, u16, GxReqtMi_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<22,0x3ff,1,0,u16, GxReqtMi_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for GxReqtMi {
            #[inline(always)]
            fn default() -> GxReqtMi {
                <crate::RegValueT<GxReqtMi_SPEC> as RegisterValue<_>>::new(4290772992)
            }
        }
        pub mod gxreqtmi {
            pub struct Seqmod_SPEC;
            pub type Seqmod = crate::EnumBitfieldStruct<u8, Seqmod_SPEC>;
            impl Seqmod {
                #[doc = "Request timer off. Trigger events become effective immediately."]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "Pause after conversion. Conversions are requested after a trigger event and each time the        request timer has expired."]
                pub const CONST_11: Self = Self::new(1);
                #[doc = "Wait before first conversion. After a trigger event the request timer is started. The conversion        sequence is executed after the request timer has expired."]
                pub const CONST_22: Self = Self::new(2);
                #[doc = "Wait before each conversion. After a trigger event the request timer is started. Conversions are        requested each time the request timer has expired."]
                pub const CONST_33: Self = Self::new(3);
            }
            pub struct Reqts_SPEC;
            pub type Reqts = crate::EnumBitfieldStruct<u8, Reqts_SPEC>;
            impl Reqts {
                #[doc = "No action"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "Start the request timer immediately via software. Mode and start stop values must be configured        before setting the trigger bit REQTS."]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Entr_SPEC;
            pub type Entr = crate::EnumBitfieldStruct<u8, Entr_SPEC>;
            impl Entr {
                #[doc = "External trigger disabled"]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "The selected edge at the selected trigger input signal starts the request timer"]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct GxReqtSi_SPEC;
        impl crate::sealed::RegSpec for GxReqtSi_SPEC {
            type DataType = u32;
        }
        #[doc = "Queue 0 Requ. Timer Status Reg.  Group 0\n resetvalue={Application Reset:0x0}"]
        pub type GxReqtSi = crate::RegValueT<GxReqtSi_SPEC>;

        impl GxReqtSi {
            #[doc = "Sequence Timer   SEQTIMER. Counts the request timer periods in steps of 16   215  t ADC .        This timer is loaded from bitfield SEQTIMSET at the beginning of a        period."]
            #[inline(always)]
            pub fn seqtimer(
                self,
            ) -> crate::common::RegisterField<6, 0x3ff, 1, 0, u16, GxReqtSi_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<6,0x3ff,1,0,u16, GxReqtSi_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for GxReqtSi {
            #[inline(always)]
            fn default() -> GxReqtSi {
                <crate::RegValueT<GxReqtSi_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
