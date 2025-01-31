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
#[doc = r"EDSADC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edsadc(pub(super) *mut u8);
unsafe impl core::marker::Send for Edsadc {}
unsafe impl core::marker::Sync for Edsadc {}
impl Edsadc {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "Access Protection Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn accprot(&self) -> crate::common::Reg<self::Accprot_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }

    #[doc = "Carrier Generator Configuration Register\n resetvalue={Application Reset:0x7100000}"]
    #[inline(always)]
    pub const fn cgcfg(&self) -> crate::common::Reg<self::Cgcfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Event Flag Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn evflag(&self) -> crate::common::Reg<self::Evflag_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(224usize)) }
    }

    #[doc = "Event Flag Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn evflagclr(&self) -> crate::common::Reg<self::Evflagclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(228usize)) }
    }

    #[doc = "Global Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn globcfg(&self) -> crate::common::Reg<self::Globcfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "Global Run Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn globrc(&self) -> crate::common::Reg<self::Globrc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={PowerOn Reset:0x0C6C007}"]
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
    #[doc = "CH"]
    #[inline(always)]
    pub fn ch(self) -> [self::Ch; 6] {
        unsafe {
            [
                self::Ch(self.0.add(0x100usize + 0x0usize)),
                self::Ch(self.0.add(0x100usize + 0x100usize)),
                self::Ch(self.0.add(0x100usize + 0x200usize)),
                self::Ch(self.0.add(0x100usize + 0x300usize)),
                self::Ch(self.0.add(0x100usize + 0x400usize)),
                self::Ch(self.0.add(0x100usize + 0x500usize)),
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
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, accen0::En0, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,accen0::En0, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, accen0::En1, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,accen0::En1, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, accen0::En2, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,accen0::En2, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, accen0::En3, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,accen0::En3, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, accen0::En4, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,accen0::En4, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, accen0::En5, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,accen0::En5, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, accen0::En6, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,accen0::En6, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, accen0::En7, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,accen0::En7, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, accen0::En8, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,accen0::En8, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, accen0::En9, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,accen0::En9, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, accen0::En10, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,accen0::En10, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, accen0::En11, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,accen0::En11, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, accen0::En12, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,accen0::En12, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, accen0::En13, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,accen0::En13, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, accen0::En14, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,accen0::En14, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, accen0::En15, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,accen0::En15, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, accen0::En16, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,accen0::En16, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, accen0::En17, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,accen0::En17, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, accen0::En18, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,accen0::En18, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, accen0::En19, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,accen0::En19, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, accen0::En20, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,accen0::En20, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, accen0::En21, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,accen0::En21, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, accen0::En22, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,accen0::En22, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, accen0::En23, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,accen0::En23, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, accen0::En24, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,accen0::En24, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, accen0::En25, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,accen0::En25, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, accen0::En26, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,accen0::En26, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, accen0::En27, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,accen0::En27, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, accen0::En28, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,accen0::En28, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, accen0::En29, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,accen0::En29, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, accen0::En30, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,accen0::En30, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
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
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En1_SPEC;
    pub type En1 = crate::EnumBitfieldStruct<u8, En1_SPEC>;
    impl En1 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En2_SPEC;
    pub type En2 = crate::EnumBitfieldStruct<u8, En2_SPEC>;
    impl En2 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En3_SPEC;
    pub type En3 = crate::EnumBitfieldStruct<u8, En3_SPEC>;
    impl En3 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En4_SPEC;
    pub type En4 = crate::EnumBitfieldStruct<u8, En4_SPEC>;
    impl En4 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En5_SPEC;
    pub type En5 = crate::EnumBitfieldStruct<u8, En5_SPEC>;
    impl En5 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En6_SPEC;
    pub type En6 = crate::EnumBitfieldStruct<u8, En6_SPEC>;
    impl En6 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En7_SPEC;
    pub type En7 = crate::EnumBitfieldStruct<u8, En7_SPEC>;
    impl En7 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En8_SPEC;
    pub type En8 = crate::EnumBitfieldStruct<u8, En8_SPEC>;
    impl En8 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En9_SPEC;
    pub type En9 = crate::EnumBitfieldStruct<u8, En9_SPEC>;
    impl En9 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En10_SPEC;
    pub type En10 = crate::EnumBitfieldStruct<u8, En10_SPEC>;
    impl En10 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En11_SPEC;
    pub type En11 = crate::EnumBitfieldStruct<u8, En11_SPEC>;
    impl En11 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En12_SPEC;
    pub type En12 = crate::EnumBitfieldStruct<u8, En12_SPEC>;
    impl En12 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En13_SPEC;
    pub type En13 = crate::EnumBitfieldStruct<u8, En13_SPEC>;
    impl En13 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En14_SPEC;
    pub type En14 = crate::EnumBitfieldStruct<u8, En14_SPEC>;
    impl En14 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En15_SPEC;
    pub type En15 = crate::EnumBitfieldStruct<u8, En15_SPEC>;
    impl En15 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En16_SPEC;
    pub type En16 = crate::EnumBitfieldStruct<u8, En16_SPEC>;
    impl En16 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En17_SPEC;
    pub type En17 = crate::EnumBitfieldStruct<u8, En17_SPEC>;
    impl En17 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En18_SPEC;
    pub type En18 = crate::EnumBitfieldStruct<u8, En18_SPEC>;
    impl En18 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En19_SPEC;
    pub type En19 = crate::EnumBitfieldStruct<u8, En19_SPEC>;
    impl En19 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En20_SPEC;
    pub type En20 = crate::EnumBitfieldStruct<u8, En20_SPEC>;
    impl En20 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En21_SPEC;
    pub type En21 = crate::EnumBitfieldStruct<u8, En21_SPEC>;
    impl En21 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En22_SPEC;
    pub type En22 = crate::EnumBitfieldStruct<u8, En22_SPEC>;
    impl En22 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En23_SPEC;
    pub type En23 = crate::EnumBitfieldStruct<u8, En23_SPEC>;
    impl En23 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En24_SPEC;
    pub type En24 = crate::EnumBitfieldStruct<u8, En24_SPEC>;
    impl En24 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En25_SPEC;
    pub type En25 = crate::EnumBitfieldStruct<u8, En25_SPEC>;
    impl En25 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En26_SPEC;
    pub type En26 = crate::EnumBitfieldStruct<u8, En26_SPEC>;
    impl En26 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En27_SPEC;
    pub type En27 = crate::EnumBitfieldStruct<u8, En27_SPEC>;
    impl En27 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En28_SPEC;
    pub type En28 = crate::EnumBitfieldStruct<u8, En28_SPEC>;
    impl En28 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En29_SPEC;
    pub type En29 = crate::EnumBitfieldStruct<u8, En29_SPEC>;
    impl En29 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En30_SPEC;
    pub type En30 = crate::EnumBitfieldStruct<u8, En30_SPEC>;
    impl En30 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En31_SPEC;
    pub type En31 = crate::EnumBitfieldStruct<u8, En31_SPEC>;
    impl En31 {
        #[doc = "No write access"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accprot_SPEC;
impl crate::sealed::RegSpec for Accprot_SPEC {
    type DataType = u32;
}
#[doc = "Access Protection Register\n resetvalue={Application Reset:0x0}"]
pub type Accprot = crate::RegValueT<Accprot_SPEC>;

impl Accprot {
    #[doc = "Register Group 7   RG07"]
    #[inline(always)]
    pub fn rg00(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, accprot::Rg00, Accprot_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,accprot::Rg00, Accprot_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Register Group 7   RG07"]
    #[inline(always)]
    pub fn rg01(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, accprot::Rg01, Accprot_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,accprot::Rg01, Accprot_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Register Group 7   RG07"]
    #[inline(always)]
    pub fn rg02(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, accprot::Rg02, Accprot_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,accprot::Rg02, Accprot_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Register Group 7   RG07"]
    #[inline(always)]
    pub fn rg03(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, accprot::Rg03, Accprot_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,accprot::Rg03, Accprot_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Register Group 7   RG07"]
    #[inline(always)]
    pub fn rg04(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, accprot::Rg04, Accprot_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,accprot::Rg04, Accprot_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Register Group 7   RG07"]
    #[inline(always)]
    pub fn rg05(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, accprot::Rg05, Accprot_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,accprot::Rg05, Accprot_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Register Group 7   RG07"]
    #[inline(always)]
    pub fn rg06(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, accprot::Rg06, Accprot_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,accprot::Rg06, Accprot_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Register Group 7   RG07"]
    #[inline(always)]
    pub fn rg07(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, accprot::Rg07, Accprot_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,accprot::Rg07, Accprot_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Register Group 10   RG10"]
    #[inline(always)]
    pub fn rg10(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, accprot::Rg10, Accprot_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,accprot::Rg10, Accprot_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Register Group Global   RGG"]
    #[inline(always)]
    pub fn rgg(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, accprot::Rgg, Accprot_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,accprot::Rgg, Accprot_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Accprot {
    #[inline(always)]
    fn default() -> Accprot {
        <crate::RegValueT<Accprot_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod accprot {
    pub struct Rg00_SPEC;
    pub type Rg00 = crate::EnumBitfieldStruct<u8, Rg00_SPEC>;
    impl Rg00 {
        #[doc = "Full access to register group x"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access to registers of group x is blocked"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rg01_SPEC;
    pub type Rg01 = crate::EnumBitfieldStruct<u8, Rg01_SPEC>;
    impl Rg01 {
        #[doc = "Full access to register group x"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access to registers of group x is blocked"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rg02_SPEC;
    pub type Rg02 = crate::EnumBitfieldStruct<u8, Rg02_SPEC>;
    impl Rg02 {
        #[doc = "Full access to register group x"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access to registers of group x is blocked"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rg03_SPEC;
    pub type Rg03 = crate::EnumBitfieldStruct<u8, Rg03_SPEC>;
    impl Rg03 {
        #[doc = "Full access to register group x"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access to registers of group x is blocked"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rg04_SPEC;
    pub type Rg04 = crate::EnumBitfieldStruct<u8, Rg04_SPEC>;
    impl Rg04 {
        #[doc = "Full access to register group x"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access to registers of group x is blocked"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rg05_SPEC;
    pub type Rg05 = crate::EnumBitfieldStruct<u8, Rg05_SPEC>;
    impl Rg05 {
        #[doc = "Full access to register group x"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access to registers of group x is blocked"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rg06_SPEC;
    pub type Rg06 = crate::EnumBitfieldStruct<u8, Rg06_SPEC>;
    impl Rg06 {
        #[doc = "Full access to register group x"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access to registers of group x is blocked"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rg07_SPEC;
    pub type Rg07 = crate::EnumBitfieldStruct<u8, Rg07_SPEC>;
    impl Rg07 {
        #[doc = "Full access to register group x"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access to registers of group x is blocked"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rg10_SPEC;
    pub type Rg10 = crate::EnumBitfieldStruct<u8, Rg10_SPEC>;
    impl Rg10 {
        #[doc = "Full access to register group 10"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access to registers of group 10 is blocked"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rgg_SPEC;
    pub type Rgg = crate::EnumBitfieldStruct<u8, Rgg_SPEC>;
    impl Rgg {
        #[doc = "Full access to global register group"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write access to global registers is blocked"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cgcfg_SPEC;
impl crate::sealed::RegSpec for Cgcfg_SPEC {
    type DataType = u32;
}
#[doc = "Carrier Generator Configuration Register\n resetvalue={Application Reset:0x7100000}"]
pub type Cgcfg = crate::RegValueT<Cgcfg_SPEC>;

impl Cgcfg {
    #[doc = "Carrier Generator Operating Mode   CGMOD. Stopping the carrier generator  CGMOD  160    160  00          terminates the PWM output after completion of the current period         indicated by bit RUN  160    160 0 ."]
    #[inline(always)]
    pub fn cgmod(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, cgcfg::Cgmod, Cgcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,cgcfg::Cgmod, Cgcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bit Reverse PWM Generation   BREV"]
    #[inline(always)]
    pub fn brev(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, cgcfg::Brev, Cgcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,cgcfg::Brev, Cgcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Signal Polarity   SIGPOL"]
    #[inline(always)]
    pub fn sigpol(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, cgcfg::Sigpol, Cgcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,cgcfg::Sigpol, Cgcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Divider Factor for the PWM Pattern Signal Generator   DIVCG. Defines the input frequency of the carrier signal generator  derived        from the selected internal clock source  f CG   f PER   CGP. The frequency of the carrier signal itself is f CG   1024."]
    #[inline(always)]
    pub fn divcg(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Cgcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Cgcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Run Indicator   RUN"]
    #[inline(always)]
    pub fn run(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, cgcfg::Run, Cgcfg_SPEC, crate::common::R> {
        crate::common::RegisterField::<15,0x1,1,0,cgcfg::Run, Cgcfg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Bit Counter   BITCOUNT. Counts the 32 cycles generated for each step"]
    #[inline(always)]
    pub fn bitcount(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Cgcfg_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Cgcfg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Step Counter   STEPCOUNT. Counts the 8 steps generated for each quadrant of the carrier signal        period"]
    #[inline(always)]
    pub fn stepcount(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Cgcfg_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Cgcfg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Step Counter Sign   STEPS. Indicates the sign of the step counter value"]
    #[inline(always)]
    pub fn steps(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, cgcfg::Steps, Cgcfg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<28,0x1,1,0,cgcfg::Steps, Cgcfg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Step Counter Direction   STEPD"]
    #[inline(always)]
    pub fn stepd(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, cgcfg::Stepd, Cgcfg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<29,0x1,1,0,cgcfg::Stepd, Cgcfg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Sign Signal from Carrier Generator   SGNCG"]
    #[inline(always)]
    pub fn sgncg(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, cgcfg::Sgncg, Cgcfg_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<30,0x1,1,0,cgcfg::Sgncg, Cgcfg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Cgcfg {
    #[inline(always)]
    fn default() -> Cgcfg {
        <crate::RegValueT<Cgcfg_SPEC> as RegisterValue<_>>::new(118489088)
    }
}
pub mod cgcfg {
    pub struct Cgmod_SPEC;
    pub type Cgmod = crate::EnumBitfieldStruct<u8, Cgmod_SPEC>;
    impl Cgmod {
        #[doc = "Stopped"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Square wave"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "Triangle"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "Sine wave"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Brev_SPEC;
    pub type Brev = crate::EnumBitfieldStruct<u8, Brev_SPEC>;
    impl Brev {
        #[doc = "Normal mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Bit reverse mode"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sigpol_SPEC;
    pub type Sigpol = crate::EnumBitfieldStruct<u8, Sigpol_SPEC>;
    impl Sigpol {
        #[doc = "Normal  . carrier signal begins with  1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Inverted  . carrier signal begins with  1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Run_SPEC;
    pub type Run = crate::EnumBitfieldStruct<u8, Run_SPEC>;
    impl Run {
        #[doc = "Stopped  cleared at the end of a period"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Running"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Steps_SPEC;
    pub type Steps = crate::EnumBitfieldStruct<u8, Steps_SPEC>;
    impl Steps {
        #[doc = "Step counter value is positive"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Step counter value is negative"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Stepd_SPEC;
    pub type Stepd = crate::EnumBitfieldStruct<u8, Stepd_SPEC>;
    impl Stepd {
        #[doc = "Step counter is counting down"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Step counter is counting up"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sgncg_SPEC;
    pub type Sgncg = crate::EnumBitfieldStruct<u8, Sgncg_SPEC>;
    impl Sgncg {
        #[doc = "Positive values"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Negative values"]
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
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control of the module."]
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
        #[doc = "Off  module is not clocked. 1 f SPB and f PER are disabled"]
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
pub struct Evflag_SPEC;
impl crate::sealed::RegSpec for Evflag_SPEC {
    type DataType = u32;
}
#[doc = "Event Flag Register\n resetvalue={Application Reset:0x0}"]
pub type Evflag = crate::RegValueT<Evflag_SPEC>;

impl Evflag {
    #[doc = "Result Event   RESEV5. Bit RESEVx is cleared when result register RESMx is read  or when bit          RESECx is written with 1."]
    #[inline(always)]
    pub fn resev0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, evflag::Resev0, Evflag_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,evflag::Resev0, Evflag_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Result Event   RESEV5. Bit RESEVx is cleared when result register RESMx is read  or when bit          RESECx is written with 1."]
    #[inline(always)]
    pub fn resev1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, evflag::Resev1, Evflag_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,evflag::Resev1, Evflag_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Result Event   RESEV5. Bit RESEVx is cleared when result register RESMx is read  or when bit          RESECx is written with 1."]
    #[inline(always)]
    pub fn resev2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, evflag::Resev2, Evflag_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,evflag::Resev2, Evflag_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Result Event   RESEV5. Bit RESEVx is cleared when result register RESMx is read  or when bit          RESECx is written with 1."]
    #[inline(always)]
    pub fn resev3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, evflag::Resev3, Evflag_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,evflag::Resev3, Evflag_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Result Event   RESEV5. Bit RESEVx is cleared when result register RESMx is read  or when bit          RESECx is written with 1."]
    #[inline(always)]
    pub fn resev4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, evflag::Resev4, Evflag_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,evflag::Resev4, Evflag_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Result Event   RESEV5. Bit RESEVx is cleared when result register RESMx is read  or when bit          RESECx is written with 1."]
    #[inline(always)]
    pub fn resev5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, evflag::Resev5, Evflag_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,evflag::Resev5, Evflag_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alarm Event   ALEV5"]
    #[inline(always)]
    pub fn alev0(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, evflag::Alev0, Evflag_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,evflag::Alev0, Evflag_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alarm Event   ALEV5"]
    #[inline(always)]
    pub fn alev1(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, evflag::Alev1, Evflag_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,evflag::Alev1, Evflag_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alarm Event   ALEV5"]
    #[inline(always)]
    pub fn alev2(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, evflag::Alev2, Evflag_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,evflag::Alev2, Evflag_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alarm Event   ALEV5"]
    #[inline(always)]
    pub fn alev3(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, evflag::Alev3, Evflag_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,evflag::Alev3, Evflag_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alarm Event   ALEV5"]
    #[inline(always)]
    pub fn alev4(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, evflag::Alev4, Evflag_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,evflag::Alev4, Evflag_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Alarm Event   ALEV5"]
    #[inline(always)]
    pub fn alev5(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, evflag::Alev5, Evflag_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,evflag::Alev5, Evflag_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Evflag {
    #[inline(always)]
    fn default() -> Evflag {
        <crate::RegValueT<Evflag_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod evflag {
    pub struct Resev0_SPEC;
    pub type Resev0 = crate::EnumBitfieldStruct<u8, Resev0_SPEC>;
    impl Resev0 {
        #[doc = "No result event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "New result value is generated by the filter chain"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Resev1_SPEC;
    pub type Resev1 = crate::EnumBitfieldStruct<u8, Resev1_SPEC>;
    impl Resev1 {
        #[doc = "No result event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "New result value is generated by the filter chain"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Resev2_SPEC;
    pub type Resev2 = crate::EnumBitfieldStruct<u8, Resev2_SPEC>;
    impl Resev2 {
        #[doc = "No result event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "New result value is generated by the filter chain"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Resev3_SPEC;
    pub type Resev3 = crate::EnumBitfieldStruct<u8, Resev3_SPEC>;
    impl Resev3 {
        #[doc = "No result event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "New result value is generated by the filter chain"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Resev4_SPEC;
    pub type Resev4 = crate::EnumBitfieldStruct<u8, Resev4_SPEC>;
    impl Resev4 {
        #[doc = "No result event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "New result value is generated by the filter chain"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Resev5_SPEC;
    pub type Resev5 = crate::EnumBitfieldStruct<u8, Resev5_SPEC>;
    impl Resev5 {
        #[doc = "No result event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "New result value is generated by the filter chain"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Alev0_SPEC;
    pub type Alev0 = crate::EnumBitfieldStruct<u8, Alev0_SPEC>;
    impl Alev0 {
        #[doc = "No alarm event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "An alarm event has occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Alev1_SPEC;
    pub type Alev1 = crate::EnumBitfieldStruct<u8, Alev1_SPEC>;
    impl Alev1 {
        #[doc = "No alarm event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "An alarm event has occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Alev2_SPEC;
    pub type Alev2 = crate::EnumBitfieldStruct<u8, Alev2_SPEC>;
    impl Alev2 {
        #[doc = "No alarm event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "An alarm event has occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Alev3_SPEC;
    pub type Alev3 = crate::EnumBitfieldStruct<u8, Alev3_SPEC>;
    impl Alev3 {
        #[doc = "No alarm event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "An alarm event has occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Alev4_SPEC;
    pub type Alev4 = crate::EnumBitfieldStruct<u8, Alev4_SPEC>;
    impl Alev4 {
        #[doc = "No alarm event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "An alarm event has occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Alev5_SPEC;
    pub type Alev5 = crate::EnumBitfieldStruct<u8, Alev5_SPEC>;
    impl Alev5 {
        #[doc = "No alarm event"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "An alarm event has occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evflagclr_SPEC;
impl crate::sealed::RegSpec for Evflagclr_SPEC {
    type DataType = u32;
}
#[doc = "Event Flag Clear Register\n resetvalue={Application Reset:0x0}"]
pub type Evflagclr = crate::RegValueT<Evflagclr_SPEC>;

impl Evflagclr {
    #[doc = "Result Event Clear   RESEC5"]
    #[inline(always)]
    pub fn resec0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        evflagclr::Resec0,
        Evflagclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            evflagclr::Resec0,
            Evflagclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Result Event Clear   RESEC5"]
    #[inline(always)]
    pub fn resec1(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        evflagclr::Resec1,
        Evflagclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            evflagclr::Resec1,
            Evflagclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Result Event Clear   RESEC5"]
    #[inline(always)]
    pub fn resec2(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        evflagclr::Resec2,
        Evflagclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            evflagclr::Resec2,
            Evflagclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Result Event Clear   RESEC5"]
    #[inline(always)]
    pub fn resec3(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        evflagclr::Resec3,
        Evflagclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            evflagclr::Resec3,
            Evflagclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Result Event Clear   RESEC5"]
    #[inline(always)]
    pub fn resec4(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        evflagclr::Resec4,
        Evflagclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            evflagclr::Resec4,
            Evflagclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Result Event Clear   RESEC5"]
    #[inline(always)]
    pub fn resec5(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        evflagclr::Resec5,
        Evflagclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            evflagclr::Resec5,
            Evflagclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Alarm Event Clear   ALEC5"]
    #[inline(always)]
    pub fn alec0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        evflagclr::Alec0,
        Evflagclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            evflagclr::Alec0,
            Evflagclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Alarm Event Clear   ALEC5"]
    #[inline(always)]
    pub fn alec1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        evflagclr::Alec1,
        Evflagclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            evflagclr::Alec1,
            Evflagclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Alarm Event Clear   ALEC5"]
    #[inline(always)]
    pub fn alec2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        evflagclr::Alec2,
        Evflagclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            evflagclr::Alec2,
            Evflagclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Alarm Event Clear   ALEC5"]
    #[inline(always)]
    pub fn alec3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        evflagclr::Alec3,
        Evflagclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            evflagclr::Alec3,
            Evflagclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Alarm Event Clear   ALEC5"]
    #[inline(always)]
    pub fn alec4(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        evflagclr::Alec4,
        Evflagclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            evflagclr::Alec4,
            Evflagclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Alarm Event Clear   ALEC5"]
    #[inline(always)]
    pub fn alec5(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        evflagclr::Alec5,
        Evflagclr_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            evflagclr::Alec5,
            Evflagclr_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Evflagclr {
    #[inline(always)]
    fn default() -> Evflagclr {
        <crate::RegValueT<Evflagclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod evflagclr {
    pub struct Resec0_SPEC;
    pub type Resec0 = crate::EnumBitfieldStruct<u8, Resec0_SPEC>;
    impl Resec0 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Clear bit RESEVx"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Resec1_SPEC;
    pub type Resec1 = crate::EnumBitfieldStruct<u8, Resec1_SPEC>;
    impl Resec1 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Clear bit RESEVx"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Resec2_SPEC;
    pub type Resec2 = crate::EnumBitfieldStruct<u8, Resec2_SPEC>;
    impl Resec2 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Clear bit RESEVx"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Resec3_SPEC;
    pub type Resec3 = crate::EnumBitfieldStruct<u8, Resec3_SPEC>;
    impl Resec3 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Clear bit RESEVx"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Resec4_SPEC;
    pub type Resec4 = crate::EnumBitfieldStruct<u8, Resec4_SPEC>;
    impl Resec4 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Clear bit RESEVx"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Resec5_SPEC;
    pub type Resec5 = crate::EnumBitfieldStruct<u8, Resec5_SPEC>;
    impl Resec5 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Clear bit RESEVx"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Alec0_SPEC;
    pub type Alec0 = crate::EnumBitfieldStruct<u8, Alec0_SPEC>;
    impl Alec0 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Clear bit ALEVx"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Alec1_SPEC;
    pub type Alec1 = crate::EnumBitfieldStruct<u8, Alec1_SPEC>;
    impl Alec1 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Clear bit ALEVx"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Alec2_SPEC;
    pub type Alec2 = crate::EnumBitfieldStruct<u8, Alec2_SPEC>;
    impl Alec2 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Clear bit ALEVx"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Alec3_SPEC;
    pub type Alec3 = crate::EnumBitfieldStruct<u8, Alec3_SPEC>;
    impl Alec3 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Clear bit ALEVx"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Alec4_SPEC;
    pub type Alec4 = crate::EnumBitfieldStruct<u8, Alec4_SPEC>;
    impl Alec4 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Clear bit ALEVx"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Alec5_SPEC;
    pub type Alec5 = crate::EnumBitfieldStruct<u8, Alec5_SPEC>;
    impl Alec5 {
        #[doc = "No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Clear bit ALEVx"]
        pub const CONST_11: Self = Self::new(1);
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
    #[doc = "Unsynchronized Clock Generation   USC. Defines the way the modulator clock is generated."]
    #[inline(always)]
    pub fn usc(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, globcfg::Usc, Globcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,globcfg::Usc, Globcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Supply Voltage Level. Adjusts the analog circuitry to the supply voltage used in the        application system. Make sure to keep SUPLEV   00 B or 01 B in the case of a 5 V supply."]
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
    #[doc = "Write Control for Clock Parameters   CPWC"]
    #[inline(always)]
    pub fn cpwc(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, globcfg::Cpwc, Globcfg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<15,0x1,1,0,globcfg::Cpwc, Globcfg_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Supervision Signal Select. Defines the supervision signal of the channel selected by SVCH to be        output."]
    #[inline(always)]
    pub fn svsig(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, globcfg::Svsig, Globcfg_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x3,1,0,globcfg::Svsig, Globcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Write Control for Supervision Parameters"]
    #[inline(always)]
    pub fn svwc(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, globcfg::Svwc, Globcfg_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<31,0x1,1,0,globcfg::Svwc, Globcfg_SPEC,crate::common::W>::from_register(self,0)
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
        #[doc = "Synchronized mode. Rising clock edge is defined by the phase synchronizer."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Unsynchronized mode. The modulator clock is generated independently."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Suplev_SPEC;
    pub type Suplev = crate::EnumBitfieldStruct<u8, Suplev_SPEC>;
    impl Suplev {
        #[doc = "Automatic control  voltage range is controlled by the power supply"]
        pub const CONST_000: Self = Self::new(0);
        #[doc = "Upper voltage range  assume a 5 V power supply is connected"]
        pub const CONST_011: Self = Self::new(1);
        #[doc = "Lower voltage range  assume a 3.3 V power supply is connected"]
        pub const CONST_102: Self = Self::new(2);
    }
    pub struct Cpwc_SPEC;
    pub type Cpwc = crate::EnumBitfieldStruct<u8, Cpwc_SPEC>;
    impl Cpwc {
        #[doc = "No write access to clock parameters"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Bitfields SUPLEV  USC  DITRIM can be written"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Svsig_SPEC;
    pub type Svsig = crate::EnumBitfieldStruct<u8, Svsig_SPEC>;
    impl Svsig {
        #[doc = "Off  no supervision signal"]
        pub const CONST_000: Self = Self::new(0);
        #[doc = "1.2 V supply voltage"]
        pub const CONST_011: Self = Self::new(1);
        #[doc = "3.3 V supply voltage"]
        pub const CONST_102: Self = Self::new(2);
    }
    pub struct Svwc_SPEC;
    pub type Svwc = crate::EnumBitfieldStruct<u8, Svwc_SPEC>;
    impl Svwc {
        #[doc = "No write access to supervision parameters"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Bitfields SVSIG  SVCH can be written"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Globrc_SPEC;
impl crate::sealed::RegSpec for Globrc_SPEC {
    type DataType = u32;
}
#[doc = "Global Run Control Register\n resetvalue={Application Reset:0x0}"]
pub type Globrc = crate::RegValueT<Globrc_SPEC>;

impl Globrc {
    #[doc = "Channel 5 Run Control   CH5RUN. Each bit  when set  enables the corresponding demodulator channel. When CHxRUN is set  all filter blocks are cleared and the FIFO is        flushed."]
    #[inline(always)]
    pub fn ch0run(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, globrc::Ch0Run, Globrc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,globrc::Ch0Run, Globrc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 5 Run Control   CH5RUN. Each bit  when set  enables the corresponding demodulator channel. When CHxRUN is set  all filter blocks are cleared and the FIFO is        flushed."]
    #[inline(always)]
    pub fn ch1run(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, globrc::Ch1Run, Globrc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,globrc::Ch1Run, Globrc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 5 Run Control   CH5RUN. Each bit  when set  enables the corresponding demodulator channel. When CHxRUN is set  all filter blocks are cleared and the FIFO is        flushed."]
    #[inline(always)]
    pub fn ch2run(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, globrc::Ch2Run, Globrc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,globrc::Ch2Run, Globrc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 5 Run Control   CH5RUN. Each bit  when set  enables the corresponding demodulator channel. When CHxRUN is set  all filter blocks are cleared and the FIFO is        flushed."]
    #[inline(always)]
    pub fn ch3run(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, globrc::Ch3Run, Globrc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,globrc::Ch3Run, Globrc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 5 Run Control   CH5RUN. Each bit  when set  enables the corresponding demodulator channel. When CHxRUN is set  all filter blocks are cleared and the FIFO is        flushed."]
    #[inline(always)]
    pub fn ch4run(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, globrc::Ch4Run, Globrc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,globrc::Ch4Run, Globrc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Channel 5 Run Control   CH5RUN. Each bit  when set  enables the corresponding demodulator channel. When CHxRUN is set  all filter blocks are cleared and the FIFO is        flushed."]
    #[inline(always)]
    pub fn ch5run(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, globrc::Ch5Run, Globrc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,globrc::Ch5Run, Globrc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Modulator 5 Run Control   M5RUN. Each bit  when set  enables the corresponding modulator."]
    #[inline(always)]
    pub fn m0run(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, globrc::M0Run, Globrc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,globrc::M0Run, Globrc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Modulator 5 Run Control   M5RUN. Each bit  when set  enables the corresponding modulator."]
    #[inline(always)]
    pub fn m1run(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, globrc::M1Run, Globrc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,globrc::M1Run, Globrc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Modulator 5 Run Control   M5RUN. Each bit  when set  enables the corresponding modulator."]
    #[inline(always)]
    pub fn m2run(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, globrc::M2Run, Globrc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,globrc::M2Run, Globrc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Modulator 5 Run Control   M5RUN. Each bit  when set  enables the corresponding modulator."]
    #[inline(always)]
    pub fn m3run(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, globrc::M3Run, Globrc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,globrc::M3Run, Globrc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Modulator 5 Run Control   M5RUN. Each bit  when set  enables the corresponding modulator."]
    #[inline(always)]
    pub fn m4run(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, globrc::M4Run, Globrc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,globrc::M4Run, Globrc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Modulator 5 Run Control   M5RUN. Each bit  when set  enables the corresponding modulator."]
    #[inline(always)]
    pub fn m5run(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, globrc::M5Run, Globrc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,globrc::M5Run, Globrc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Globrc {
    #[inline(always)]
    fn default() -> Globrc {
        <crate::RegValueT<Globrc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod globrc {
    pub struct Ch0Run_SPEC;
    pub type Ch0Run = crate::EnumBitfieldStruct<u8, Ch0Run_SPEC>;
    impl Ch0Run {
        #[doc = "Stop channel x"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Demodulator channel x is enabled and runs"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ch1Run_SPEC;
    pub type Ch1Run = crate::EnumBitfieldStruct<u8, Ch1Run_SPEC>;
    impl Ch1Run {
        #[doc = "Stop channel x"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Demodulator channel x is enabled and runs"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ch2Run_SPEC;
    pub type Ch2Run = crate::EnumBitfieldStruct<u8, Ch2Run_SPEC>;
    impl Ch2Run {
        #[doc = "Stop channel x"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Demodulator channel x is enabled and runs"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ch3Run_SPEC;
    pub type Ch3Run = crate::EnumBitfieldStruct<u8, Ch3Run_SPEC>;
    impl Ch3Run {
        #[doc = "Stop channel x"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Demodulator channel x is enabled and runs"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ch4Run_SPEC;
    pub type Ch4Run = crate::EnumBitfieldStruct<u8, Ch4Run_SPEC>;
    impl Ch4Run {
        #[doc = "Stop channel x"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Demodulator channel x is enabled and runs"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ch5Run_SPEC;
    pub type Ch5Run = crate::EnumBitfieldStruct<u8, Ch5Run_SPEC>;
    impl Ch5Run {
        #[doc = "Stop channel x"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Demodulator channel x is enabled and runs"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct M0Run_SPEC;
    pub type M0Run = crate::EnumBitfieldStruct<u8, M0Run_SPEC>;
    impl M0Run {
        #[doc = "Stop clock for on chip and external modulator x"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Modulator x is enabled and can run. Additional run control via the selected gate      signal is possible by the automatic power control feature  bitfield APC in      register MODCFG  ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct M1Run_SPEC;
    pub type M1Run = crate::EnumBitfieldStruct<u8, M1Run_SPEC>;
    impl M1Run {
        #[doc = "Stop clock for on chip and external modulator x"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Modulator x is enabled and can run. Additional run control via the selected gate      signal is possible by the automatic power control feature  bitfield APC in      register MODCFG  ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct M2Run_SPEC;
    pub type M2Run = crate::EnumBitfieldStruct<u8, M2Run_SPEC>;
    impl M2Run {
        #[doc = "Stop clock for on chip and external modulator x"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Modulator x is enabled and can run. Additional run control via the selected gate      signal is possible by the automatic power control feature  bitfield APC in      register MODCFG  ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct M3Run_SPEC;
    pub type M3Run = crate::EnumBitfieldStruct<u8, M3Run_SPEC>;
    impl M3Run {
        #[doc = "Stop clock for on chip and external modulator x"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Modulator x is enabled and can run. Additional run control via the selected gate      signal is possible by the automatic power control feature  bitfield APC in      register MODCFG  ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct M4Run_SPEC;
    pub type M4Run = crate::EnumBitfieldStruct<u8, M4Run_SPEC>;
    impl M4Run {
        #[doc = "Stop clock for on chip and external modulator x"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Modulator x is enabled and can run. Additional run control via the selected gate      signal is possible by the automatic power control feature  bitfield APC in      register MODCFG  ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct M5Run_SPEC;
    pub type M5Run = crate::EnumBitfieldStruct<u8, M5Run_SPEC>;
    impl M5Run {
        #[doc = "Stop clock for on chip and external modulator x"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Modulator x is enabled and can run. Additional run control via the selected gate      signal is possible by the automatic power control feature  bitfield APC in      register MODCFG  ."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={PowerOn Reset:0x0C6C007}"]
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
    #[doc = "Module Number   MOD NUMBER. Indicates the module identification number   00C6   EDSADC"]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(13025287)
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
    pub struct Sussta_SPEC;
    pub type Sussta = crate::EnumBitfieldStruct<u8, Sussta_SPEC>;
    impl Sussta {
        #[doc = "Module is not  yet  suspended"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Module is suspended"]
        pub const CONST_11: Self = Self::new(1);
    }
}

#[doc = "CH"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch(pub(super) *mut u8);
unsafe impl core::marker::Send for Ch {}
unsafe impl core::marker::Sync for Ch {}
impl Ch {
    #[doc = "Boundary Select Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn boundselx(&self) -> crate::common::Reg<ch::BoundseLx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(120usize)) }
    }
    #[doc = "Carrier Generator Synchronization Reg. 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cgsyncx(&self) -> crate::common::Reg<ch::CgsynCx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }
    #[doc = "Demodulator Input Config. Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dicfgx(&self) -> crate::common::Reg<ch::DicfGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Auxiliary Filter Configuration Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcfgax(&self) -> crate::common::Reg<ch::FcfgAx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(112usize)) }
    }
    #[doc = "Filter Configuration Register 0  CIC Filter\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcfgcx(&self) -> crate::common::Reg<ch::FcfgCx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Filter Configuration Register 0  Main\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcfgmx(&self) -> crate::common::Reg<ch::FcfgMx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Filter Counter Register 0  CIC Filter\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcntcx(&self) -> crate::common::Reg<ch::FcntCx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Gain Calibration Register 0\n resetvalue={Application Reset:0x61A81000}"]
    #[inline(always)]
    pub const fn gaincalx(&self) -> crate::common::Reg<ch::GaincaLx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }
    #[doc = "Gain Correction Register 0\n resetvalue={Application Reset:0x1000}"]
    #[inline(always)]
    pub const fn gaincorrx(&self) -> crate::common::Reg<ch::GaincorRx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }
    #[doc = "Gain Control Register 0\n resetvalue={Application Reset:0x1000}"]
    #[inline(always)]
    pub const fn gainctrx(&self) -> crate::common::Reg<ch::GainctRx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }
    #[doc = "Intermediate Integration Value\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn iivalx(&self) -> crate::common::Reg<ch::IivaLx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Integrator Status Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn istatx(&self) -> crate::common::Reg<ch::IstaTx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }
    #[doc = "Integration Window Control Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn iwctrx(&self) -> crate::common::Reg<ch::IwctRx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "Modulator Configuration Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn modcfgx(&self) -> crate::common::Reg<ch::ModcfGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Offset Compensation Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn offcompx(&self) -> crate::common::Reg<ch::OffcomPx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }
    #[doc = "Overshoot Compensation Cfg. Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ovscfgx(&self) -> crate::common::Reg<ch::OvscfGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Rectification Configuration Register 0\n resetvalue={Application Reset:0x080000000}"]
    #[inline(always)]
    pub const fn rectcfgx(&self) -> crate::common::Reg<ch::RectcfGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(168usize)) }
    }
    #[doc = "Result Register 0 Auxiliary\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn resax(&self) -> crate::common::Reg<ch::ResAx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }
    #[doc = "Result Register 0 Main\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn resmx(&self) -> crate::common::Reg<ch::ResMx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }
    #[doc = "Result FIFO Control Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rfcx(&self) -> crate::common::Reg<ch::RfCx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }
    #[doc = "Test Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn testx(&self) -> crate::common::Reg<ch::TesTx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(236usize)) }
    }
    #[doc = "Time Stamp Counter 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tscntx(&self) -> crate::common::Reg<ch::TscnTx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(84usize)) }
    }
    #[doc = "Time Stamp Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tstmpx(&self) -> crate::common::Reg<ch::TstmPx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }
    #[doc = "Common Mode Voltage Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn vcmx(&self) -> crate::common::Reg<ch::VcMx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(176usize)) }
    }
}
pub mod ch {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BoundseLx_SPEC;
    impl crate::sealed::RegSpec for BoundseLx_SPEC {
        type DataType = u32;
    }
    #[doc = "Boundary Select Register 0\n resetvalue={Application Reset:0x0}"]
    pub type BoundseLx = crate::RegValueT<BoundseLx_SPEC>;

    impl BoundseLx {
        #[doc = "Lower Boundary Value for Limit Checking   BOUNDARYL. This  two  8217 s complement  value is compared to the upper bits of the CIC        filter results."]
        #[inline(always)]
        pub fn boundaryl(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, BoundseLx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, BoundseLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Upper Boundary Value for Limit Checking   BOUNDARYU. This  two  8217 s complement  value is compared to the upper bits of the CIC        filter results."]
        #[inline(always)]
        pub fn boundaryu(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, BoundseLx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, BoundseLx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for BoundseLx {
        #[inline(always)]
        fn default() -> BoundseLx {
            <crate::RegValueT<BoundseLx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CgsynCx_SPEC;
    impl crate::sealed::RegSpec for CgsynCx_SPEC {
        type DataType = u32;
    }
    #[doc = "Carrier Generator Synchronization Reg. 0\n resetvalue={Application Reset:0x0}"]
    pub type CgsynCx = crate::RegValueT<CgsynCx_SPEC>;

    impl CgsynCx {
        #[doc = "Sign Delay Counter   SDCOUNT. Counts the result values from the filter chain to delay the carrier sign        signal"]
        #[inline(always)]
        pub fn sdcount(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, CgsynCx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xff,1,0,u8, CgsynCx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Sign Delay Capture Value   SDCAP. Indicates the result values counted between the begin of the positive        halfwave of the carrier signal and the first received positive value."]
        #[inline(always)]
        pub fn sdcap(
            self,
        ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, CgsynCx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<8,0xff,1,0,u8, CgsynCx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Sign Delay Value for Positive Halfwave   SDPOS. Defines the content of SDCOUNT to generate a negative delayed sign        signal  SGND ."]
        #[inline(always)]
        pub fn sdpos(
            self,
        ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, CgsynCx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xff,1,0,u8, CgsynCx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Sign Delay Value for Negative Halfwave   SDNEG. Defines the content of SDCOUNT to generate a positive delayed sign        signal  SGND ."]
        #[inline(always)]
        pub fn sdneg(
            self,
        ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, CgsynCx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0xff,1,0,u8, CgsynCx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for CgsynCx {
        #[inline(always)]
        fn default() -> CgsynCx {
            <crate::RegValueT<CgsynCx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DicfGx_SPEC;
    impl crate::sealed::RegSpec for DicfGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Demodulator Input Config. Register 0\n resetvalue={Application Reset:0x0}"]
    pub type DicfGx = crate::RegValueT<DicfGx_SPEC>;

    impl DicfGx {
        #[doc = "Data Stream Select   DSS"]
        #[inline(always)]
        pub fn dss(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, dicfgx::Dss, DicfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7,1,0,dicfgx::Dss, DicfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Data Source for External Modulator   DSRCEX"]
        #[inline(always)]
        pub fn dsrcex(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x7,
            1,
            0,
            dicfgx::Dsrcex,
            DicfGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x7,
                1,
                0,
                dicfgx::Dsrcex,
                DicfGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Clock Source for External Modulator   CSRCEX"]
        #[inline(always)]
        pub fn csrcex(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x7,
            1,
            0,
            dicfgx::Csrcex,
            DicfGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x7,
                1,
                0,
                dicfgx::Csrcex,
                DicfGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Write Control for Data Stream Selection   DSWC"]
        #[inline(always)]
        pub fn dswc(
            self,
        ) -> crate::common::RegisterField<15, 0x1, 1, 0, dicfgx::Dswc, DicfGx_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<15,0x1,1,0,dicfgx::Dswc, DicfGx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Trigger Select   TRSEL. Selects an input for the trigger signal used for the following features         see also CROSSREFERENCE    To avoid unintended triggers  select the        trigger source first before enabling the corresponding function. integrator control  timestamp  multiplexer control  modulator control         APC   service request gating. The product specific appendix details the connected trigger input        signals."]
        #[inline(always)]
        pub fn trsel(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, DicfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, DicfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Integrator Trigger Mode   ITRMODE. To ensure proper operation  ensure that bitfield ITRMODE is 00 before selecting any other trigger mode. The        integration trigger mode controls bit INTEN in register IWCTRx and hence the operation of the integrator  Bit INTEN is set when ITRMODE  160    160  11 or when the selected trigger signal transition occurs. Bit INTEN is cleared when ITRMODE  160    160  00          after REPVAL 1 integration cycles  IWS  160    160 0  or when the inverse trigger        signal transition occurs  IWS  160    160 1 ."]
        #[inline(always)]
        pub fn itrmode(
            self,
        ) -> crate::common::RegisterField<
            20,
            0x3,
            1,
            0,
            dicfgx::Itrmode,
            DicfGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0x3,
                1,
                0,
                dicfgx::Itrmode,
                DicfGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Timestamp Trigger Mode   TSTRMODE. The timestamp trigger mode controls capturing the timestamp information        to register TSTMPx ."]
        #[inline(always)]
        pub fn tstrmode(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x3,
            1,
            0,
            dicfgx::Tstrmode,
            DicfGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                22,
                0x3,
                1,
                0,
                dicfgx::Tstrmode,
                DicfGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Data Read Mode   DRM. Selects the data that is returned when register RESMx is read  see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn drm(
            self,
        ) -> crate::common::RegisterField<26, 0x3, 1, 0, dicfgx::Drm, DicfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<26,0x3,1,0,dicfgx::Drm, DicfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Time Stamp Mode   TSM. See CROSSREFERENCE ."]
        #[inline(always)]
        pub fn tsm(
            self,
        ) -> crate::common::RegisterField<28, 0x1, 1, 0, dicfgx::Tsm, DicfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0x1,1,0,dicfgx::Tsm, DicfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Result Display Mode   RDM"]
        #[inline(always)]
        pub fn rdm(
            self,
        ) -> crate::common::RegisterField<29, 0x1, 1, 0, dicfgx::Rdm, DicfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<29,0x1,1,0,dicfgx::Rdm, DicfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Write Control for Mode Settings   MSWC"]
        #[inline(always)]
        pub fn mswc(
            self,
        ) -> crate::common::RegisterField<31, 0x1, 1, 0, dicfgx::Mswc, DicfGx_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<31,0x1,1,0,dicfgx::Mswc, DicfGx_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for DicfGx {
        #[inline(always)]
        fn default() -> DicfGx {
            <crate::RegValueT<DicfGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod dicfgx {
        pub struct Dss_SPEC;
        pub type Dss = crate::EnumBitfieldStruct<u8, Dss_SPEC>;
        impl Dss {
            #[doc = "On chip modulator"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "External modulator  use rising and falling clock edge  double data"]
            pub const CONST_33: Self = Self::new(3);
            #[doc = "External modulator  use each falling clock edge  direct clock"]
            pub const CONST_44: Self = Self::new(4);
            #[doc = "External modulator  use each rising clock edge  direct clock"]
            pub const CONST_55: Self = Self::new(5);
            #[doc = "External modulator  use every 2nd falling clock edge  dbl. clock"]
            pub const CONST_66: Self = Self::new(6);
            #[doc = "External modulator  use every 2nd rising clock edge  dbl. clock"]
            pub const CONST_77: Self = Self::new(7);
        }
        pub struct Dsrcex_SPEC;
        pub type Dsrcex = crate::EnumBitfieldStruct<u8, Dsrcex_SPEC>;
        impl Dsrcex {
            #[doc = "External  from input A  direct"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "External  from input A  inverted"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "External  from input B  direct"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "External  from input B  inverted"]
            pub const CONST_33: Self = Self::new(3);
            #[doc = "External  from input C  direct"]
            pub const CONST_44: Self = Self::new(4);
            #[doc = "External  from input C  inverted"]
            pub const CONST_55: Self = Self::new(5);
            #[doc = "External  from input D  direct"]
            pub const CONST_66: Self = Self::new(6);
            #[doc = "External  from input D  inverted"]
            pub const CONST_77: Self = Self::new(7);
        }
        pub struct Csrcex_SPEC;
        pub type Csrcex = crate::EnumBitfieldStruct<u8, Csrcex_SPEC>;
        impl Csrcex {
            #[doc = "000 Internal        clock"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "011 External  from input A"]
            pub const CONST_33: Self = Self::new(3);
            #[doc = "100 External  from input B"]
            pub const CONST_44: Self = Self::new(4);
            #[doc = "101 External  from input C"]
            pub const CONST_55: Self = Self::new(5);
        }
        pub struct Dswc_SPEC;
        pub type Dswc = crate::EnumBitfieldStruct<u8, Dswc_SPEC>;
        impl Dswc {
            #[doc = "0 No write access to data parameters"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Bitfields        CSRCEX  DSRCEX  DSS can be written"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Itrmode_SPEC;
        pub type Itrmode = crate::EnumBitfieldStruct<u8, Itrmode_SPEC>;
        impl Itrmode {
            #[doc = "No integration trigger  integrator bypassed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Trigger event upon a falling edge"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Trigger event upon a rising edge"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "No trigger  integrator active all the time"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Tstrmode_SPEC;
        pub type Tstrmode = crate::EnumBitfieldStruct<u8, Tstrmode_SPEC>;
        impl Tstrmode {
            #[doc = "No timestamp trigger"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Trigger event upon a falling edge"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Trigger event upon a rising edge"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Trigger event upon each edge"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Drm_SPEC;
        pub type Drm = crate::EnumBitfieldStruct<u8, Drm_SPEC>;
        impl Drm {
            #[doc = "Single  Issue one 16 bit value per read access  sign on high bits"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Single  Issue one 16 bit value per read access  timestamp or zero on high bits"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Double  Issue two 16 bit values per read access"]
            pub const CONST_22: Self = Self::new(2);
        }
        pub struct Tsm_SPEC;
        pub type Tsm = crate::EnumBitfieldStruct<u8, Tsm_SPEC>;
        impl Tsm {
            #[doc = "No timestamp  only issue result values"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Insert timestamp upon the trigger  when the gate opens"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rdm_SPEC;
        pub type Rdm = crate::EnumBitfieldStruct<u8, Rdm_SPEC>;
        impl Rdm {
            #[doc = "Signed mode. result values range from  2 15 to  2 15"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Unsigned mode. result values range from 0 to  2 16  shifted by 2 15"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Mswc_SPEC;
        pub type Mswc = crate::EnumBitfieldStruct<u8, Mswc_SPEC>;
        impl Mswc {
            #[doc = "No write access to mode settings"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Bitfields RDM  TSM  DRM  TSTRMODE  ITRMODE  TRSEL can be written"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FcfgAx_SPEC;
    impl crate::sealed::RegSpec for FcfgAx_SPEC {
        type DataType = u32;
    }
    #[doc = "Auxiliary Filter Configuration Register 0\n resetvalue={Application Reset:0x0}"]
    pub type FcfgAx = crate::RegValueT<FcfgAx_SPEC>;

    impl FcfgAx {
        #[doc = "CIC Filter  Auxiliary  Enable"]
        #[inline(always)]
        pub fn cfaen(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, fcfgax::Cfaen, FcfgAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                fcfgax::Cfaen,
                FcfgAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "CIC Filter  Auxiliary  Decimation Factor"]
        #[inline(always)]
        pub fn cfadf(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, fcfgax::Cfadf, FcfgAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                fcfgax::Cfadf,
                FcfgAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "CIC Filter  Auxiliary  Decimation Counter. The decimation counter counts the filter cycles until an output is        generated  i.e. the oversampling rate."]
        #[inline(always)]
        pub fn cfacnt(
            self,
        ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, FcfgAx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0x1f,1,0,u8, FcfgAx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for FcfgAx {
        #[inline(always)]
        fn default() -> FcfgAx {
            <crate::RegValueT<FcfgAx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod fcfgax {
        pub struct Cfaen_SPEC;
        pub type Cfaen = crate::EnumBitfieldStruct<u8, Cfaen_SPEC>;
        impl Cfaen {
            #[doc = "Off  Auxiliary filter is not active"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Auxiliary filter is active and generates results and alarm events"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cfadf_SPEC;
        pub type Cfadf = crate::EnumBitfieldStruct<u8, Cfadf_SPEC>;
        impl Cfadf {
            #[doc = "OSR   16"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "OSR   32"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FcfgCx_SPEC;
    impl crate::sealed::RegSpec for FcfgCx_SPEC {
        type DataType = u32;
    }
    #[doc = "Filter Configuration Register 0  CIC Filter\n resetvalue={Application Reset:0x0}"]
    pub type FcfgCx = crate::RegValueT<FcfgCx_SPEC>;

    impl FcfgCx {
        #[doc = "CIC Filter Decimation Factor   CFMDF. Defines the oversampling rate of the CIC filter  OSR   CFMDF  160    160 1. Valid values are 0 03 to 1FF  OSR   4 to 512 ."]
        #[inline(always)]
        pub fn cfmdf(
            self,
        ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, FcfgCx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1ff,1,0,u16, FcfgCx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "CIC Filter Start Value   CFMSV. The decimation counter begins counting at value CFMSV  when started or        restarted. Valid values are 003 to CFMDF  4 to        selected OSR . Start values above the selected        oversampling rate may lead to overflows in the CIC filter"]
        #[inline(always)]
        pub fn cfmsv(
            self,
        ) -> crate::common::RegisterField<16, 0x1ff, 1, 0, u16, FcfgCx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1ff,1,0,u16, FcfgCx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for FcfgCx {
        #[inline(always)]
        fn default() -> FcfgCx {
            <crate::RegValueT<FcfgCx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FcfgMx_SPEC;
    impl crate::sealed::RegSpec for FcfgMx_SPEC {
        type DataType = u32;
    }
    #[doc = "Filter Configuration Register 0  Main\n resetvalue={Application Reset:0x0}"]
    pub type FcfgMx = crate::RegValueT<FcfgMx_SPEC>;

    impl FcfgMx {
        #[doc = "FIR0 Filter Enable   FIR0EN"]
        #[inline(always)]
        pub fn fir0en(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            fcfgmx::Fir0En,
            FcfgMx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                fcfgmx::Fir0En,
                FcfgMx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "FIR1 Filter Enable   FIR1EN"]
        #[inline(always)]
        pub fn fir1en(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            fcfgmx::Fir1En,
            FcfgMx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                fcfgmx::Fir1En,
                FcfgMx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Overshoot Compensation Enable"]
        #[inline(always)]
        pub fn ovcen(
            self,
        ) -> crate::common::RegisterField<2, 0x1, 1, 0, fcfgmx::Ovcen, FcfgMx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                fcfgmx::Ovcen,
                FcfgMx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "FIR1 Filter Decimation Rate   FIR1DEC"]
        #[inline(always)]
        pub fn fir1dec(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            fcfgmx::Fir1Dec,
            FcfgMx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                fcfgmx::Fir1Dec,
                FcfgMx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "CIC Filter Mode   CICMOD"]
        #[inline(always)]
        pub fn cicmod(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            fcfgmx::Cicmod,
            FcfgMx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                fcfgmx::Cicmod,
                FcfgMx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Prefilter Enable   PFEN"]
        #[inline(always)]
        pub fn pfen(
            self,
        ) -> crate::common::RegisterField<5, 0x1, 1, 0, fcfgmx::Pfen, FcfgMx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x1,1,0,fcfgmx::Pfen, FcfgMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Offset Compensation Filter Enable   OCEN"]
        #[inline(always)]
        pub fn ocen(
            self,
        ) -> crate::common::RegisterField<8, 0x7, 1, 0, fcfgmx::Ocen, FcfgMx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x7,1,0,fcfgmx::Ocen, FcfgMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Offset Protection   OFFPROT. Controls the influence of the calibration sequence on register OFFCOMP."]
        #[inline(always)]
        pub fn offprot(
            self,
        ) -> crate::common::RegisterField<
            11,
            0x1,
            1,
            0,
            fcfgmx::Offprot,
            FcfgMx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                fcfgmx::Offprot,
                FcfgMx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Write Control for Filter Modes   FMWC"]
        #[inline(always)]
        pub fn fmwc(
            self,
        ) -> crate::common::RegisterField<15, 0x1, 1, 0, fcfgmx::Fmwc, FcfgMx_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<15,0x1,1,0,fcfgmx::Fmwc, FcfgMx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Service Request Generation for Main Service Request   SRGM"]
        #[inline(always)]
        pub fn srgm(
            self,
        ) -> crate::common::RegisterField<16, 0x3, 1, 0, fcfgmx::Srgm, FcfgMx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                16,
                0x3,
                1,
                0,
                fcfgmx::Srgm,
                FcfgMx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Service Request Generation for Alternate Service Request   SRGA. SRGA  160    160  00 can be used for testing  If TEST.ICTEN  160    160  1            Reload request for test shift register"]
        #[inline(always)]
        pub fn srga(
            self,
        ) -> crate::common::RegisterField<20, 0x3, 1, 0, fcfgmx::Srga, FcfgMx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                20,
                0x3,
                1,
                0,
                fcfgmx::Srga,
                FcfgMx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Event Select   ESEL. Defines when a comparator event is generated."]
        #[inline(always)]
        pub fn esel(
            self,
        ) -> crate::common::RegisterField<22, 0x3, 1, 0, fcfgmx::Esel, FcfgMx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                22,
                0x3,
                1,
                0,
                fcfgmx::Esel,
                FcfgMx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Event Gating. Defines if alarm events are coupled to the integration window."]
        #[inline(always)]
        pub fn egt(
            self,
        ) -> crate::common::RegisterField<24, 0x1, 1, 0, fcfgmx::Egt, FcfgMx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0x1,1,0,fcfgmx::Egt, FcfgMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Calibration Trigger   CALIB"]
        #[inline(always)]
        pub fn calib(
            self,
        ) -> crate::common::RegisterField<28, 0x1, 1, 0, fcfgmx::Calib, FcfgMx_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<
                28,
                0x1,
                1,
                0,
                fcfgmx::Calib,
                FcfgMx_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Automatic Calibration Control   AUTOCAL"]
        #[inline(always)]
        pub fn autocal(
            self,
        ) -> crate::common::RegisterField<
            29,
            0x1,
            1,
            0,
            fcfgmx::Autocal,
            FcfgMx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                29,
                0x1,
                1,
                0,
                fcfgmx::Autocal,
                FcfgMx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Write Control for Calibration and Service Request Modes   CSRWC"]
        #[inline(always)]
        pub fn csrwc(
            self,
        ) -> crate::common::RegisterField<31, 0x1, 1, 0, fcfgmx::Csrwc, FcfgMx_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                fcfgmx::Csrwc,
                FcfgMx_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for FcfgMx {
        #[inline(always)]
        fn default() -> FcfgMx {
            <crate::RegValueT<FcfgMx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod fcfgmx {
        pub struct Fir0En_SPEC;
        pub type Fir0En = crate::EnumBitfieldStruct<u8, Fir0En_SPEC>;
        impl Fir0En {
            #[doc = "FIR0 disabled and bypassed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "FIR0 filter enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Fir1En_SPEC;
        pub type Fir1En = crate::EnumBitfieldStruct<u8, Fir1En_SPEC>;
        impl Fir1En {
            #[doc = "FIR1 disabled and bypassed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "FIR1 filter enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Ovcen_SPEC;
        pub type Ovcen = crate::EnumBitfieldStruct<u8, Ovcen_SPEC>;
        impl Ovcen {
            #[doc = "Disabled  feed FIR filter directly"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Attenuate response to fast edges"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Fir1Dec_SPEC;
        pub type Fir1Dec = crate::EnumBitfieldStruct<u8, Fir1Dec_SPEC>;
        impl Fir1Dec {
            #[doc = "FIR1 filter does not decimate"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Decimation 2 1 for FIR1"]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Cicmod_SPEC;
        pub type Cicmod = crate::EnumBitfieldStruct<u8, Cicmod_SPEC>;
        impl Cicmod {
            #[doc = "CIC3"]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Pfen_SPEC;
        pub type Pfen = crate::EnumBitfieldStruct<u8, Pfen_SPEC>;
        impl Pfen {
            #[doc = "Off"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Prefilter enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Ocen_SPEC;
        pub type Ocen = crate::EnumBitfieldStruct<u8, Ocen_SPEC>;
        impl Ocen {
            #[doc = "Offset compensation filter disabled  register OFFCOMP not changed"]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct Offprot_SPEC;
        pub type Offprot = crate::EnumBitfieldStruct<u8, Offprot_SPEC>;
        impl Offprot {
            #[doc = "Unprotected  calibration sequence updates bitfield OFFSET"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Protected  bitfield OFFSET is locked and not modified by calibration"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Fmwc_SPEC;
        pub type Fmwc = crate::EnumBitfieldStruct<u8, Fmwc_SPEC>;
        impl Fmwc {
            #[doc = "No write access to filter modes"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Bitfields OFFPROT  OCEN  PFEN  CICMOD  FIR1DEC  OVCEN  FIR1EN  FIR0EN can be written"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Srgm_SPEC;
        pub type Srgm = crate::EnumBitfieldStruct<u8, Srgm_SPEC>;
        impl Srgm {
            #[doc = "Never  service request disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "While gate  selected trigger signal  is high"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "While gate  selected trigger signal  is low"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Always  as selected by bitfield SRLVL. 11"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Srga_SPEC;
        pub type Srga = crate::EnumBitfieldStruct<u8, Srga_SPEC>;
        impl Srga {
            #[doc = "Never  service request disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Comparator event  as selected by bitfield ESEL EGT"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Timestamp event"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Alternate source  . Capturing of a sign delay value to register CGSYNC"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Esel_SPEC;
        pub type Esel = crate::EnumBitfieldStruct<u8, Esel_SPEC>;
        impl Esel {
            #[doc = "Always  for each new result value"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "If result is inside the boundary band"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "If result is outside the boundary band"]
            pub const CONST_22: Self = Self::new(2);
        }
        pub struct Egt_SPEC;
        pub type Egt = crate::EnumBitfieldStruct<u8, Egt_SPEC>;
        impl Egt {
            #[doc = "Separate  generate events according to ESEL"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Coupled  generate events only when the integrator is enabled and after the discard phase defined by bitfield NVALDIS. While the integrator is bypassed  event gating suppresses alarm service      requests  result values are still generated and stored."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Calib_SPEC;
        pub type Calib = crate::EnumBitfieldStruct<u8, Calib_SPEC>;
        impl Calib {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Start the calibration algorithm now"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Autocal_SPEC;
        pub type Autocal = crate::EnumBitfieldStruct<u8, Autocal_SPEC>;
        impl Autocal {
            #[doc = "Calibration algorithm started by software  set bit CALIB"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Automatically start the calibration algorithm. 1 when the        selected service request gate closes  trailing edge  see bitfield SRGM"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Csrwc_SPEC;
        pub type Csrwc = crate::EnumBitfieldStruct<u8, Csrwc_SPEC>;
        impl Csrwc {
            #[doc = "No write access to calibration and service request modes"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Bitfields AUTOCAL  CALIB  EGT  ESEL  SRGA  SRGM can be written"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FcntCx_SPEC;
    impl crate::sealed::RegSpec for FcntCx_SPEC {
        type DataType = u32;
    }
    #[doc = "Filter Counter Register 0  CIC Filter\n resetvalue={Application Reset:0x0}"]
    pub type FcntCx = crate::RegValueT<FcntCx_SPEC>;

    impl FcntCx {
        #[doc = "CIC Filter Decimation Counter   CFMDCNT. The decimation counter counts the filter cycles until an output is        generated  i.e. the oversampling rate. CFMDCNT counts down from the respective start value."]
        #[inline(always)]
        pub fn cfmdcnt(
            self,
        ) -> crate::common::RegisterField<0, 0x1ff, 1, 0, u16, FcntCx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0x1ff,1,0,u16, FcntCx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Calibration Status Flag   CAL. Bitfield CAL is set to 01 in the next          clock cycle after setting bit CALIB or after detecting the selected          trigger  if auto calibration is activated ."]
        #[inline(always)]
        pub fn cal(
            self,
        ) -> crate::common::RegisterField<30, 0x3, 1, 0, fcntcx::Cal, FcntCx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<30,0x3,1,0,fcntcx::Cal, FcntCx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for FcntCx {
        #[inline(always)]
        fn default() -> FcntCx {
            <crate::RegValueT<FcntCx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod fcntcx {
        pub struct Cal_SPEC;
        pub type Cal = crate::EnumBitfieldStruct<u8, Cal_SPEC>;
        impl Cal {
            #[doc = "Uncalibrated  initial state after reset"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "The calibration algorithm is currently running"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Calibrated  normal operation is possible"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Calibration terminated incorrectly"]
            pub const CONST_33: Self = Self::new(3);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GaincaLx_SPEC;
    impl crate::sealed::RegSpec for GaincaLx_SPEC {
        type DataType = u32;
    }
    #[doc = "Gain Calibration Register 0\n resetvalue={Application Reset:0x61A81000}"]
    pub type GaincaLx = crate::RegValueT<GaincaLx_SPEC>;

    impl GaincaLx {
        #[doc = "Multiplication Factor for Gain Calibration   CALFACTOR. The resulting factor is   lt CALFACTOR gt    4  160 096  The initial value of 4  160 096   1000            corresponds to a factor of 1.000."]
        #[inline(always)]
        pub fn calfactor(
            self,
        ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, GaincaLx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1fff,1,0,u16, GaincaLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Target Value for Calibrated Fullscale   CALTARGET. Defines the target value for the calibration algorithm. The initial value of 25  160 000   61A8            corresponds to 0.2  160 mV per LSB."]
        #[inline(always)]
        pub fn caltarget(
            self,
        ) -> crate::common::RegisterField<16, 0x7fff, 1, 0, u16, GaincaLx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x7fff,1,0,u16, GaincaLx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GaincaLx {
        #[inline(always)]
        fn default() -> GaincaLx {
            <crate::RegValueT<GaincaLx_SPEC> as RegisterValue<_>>::new(1638404096)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GaincorRx_SPEC;
    impl crate::sealed::RegSpec for GaincorRx_SPEC {
        type DataType = u32;
    }
    #[doc = "Gain Correction Register 0\n resetvalue={Application Reset:0x1000}"]
    pub type GaincorRx = crate::RegValueT<GaincorRx_SPEC>;

    impl GaincorRx {
        #[doc = "Multiplication Factor for Gain Correction   GAINFACTOR. The resulting factor is   lt GAINFACTOR gt    4  160 096"]
        #[inline(always)]
        pub fn gainfactor(
            self,
        ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, GaincorRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1fff,1,0,u16, GaincorRx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GaincorRx {
        #[inline(always)]
        fn default() -> GaincorRx {
            <crate::RegValueT<GaincorRx_SPEC> as RegisterValue<_>>::new(4096)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GainctRx_SPEC;
    impl crate::sealed::RegSpec for GainctRx_SPEC {
        type DataType = u32;
    }
    #[doc = "Gain Control Register 0\n resetvalue={Application Reset:0x1000}"]
    pub type GainctRx = crate::RegValueT<GainctRx_SPEC>;

    impl GainctRx {
        #[doc = "Multiplication Factor for Gain Correction During Calibration   GAINFACTOR. The resulting factor is   lt GAINFACTOR gt    4  160 096"]
        #[inline(always)]
        pub fn gainfactor(
            self,
        ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, GainctRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1fff,1,0,u16, GainctRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Decimation Rate of the CIC Filter During Calibration   CICDEC"]
        #[inline(always)]
        pub fn cicdec(
            self,
        ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, GainctRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0x7,1,0,u8, GainctRx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for GainctRx {
        #[inline(always)]
        fn default() -> GainctRx {
            <crate::RegValueT<GainctRx_SPEC> as RegisterValue<_>>::new(4096)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IivaLx_SPEC;
    impl crate::sealed::RegSpec for IivaLx_SPEC {
        type DataType = u32;
    }
    #[doc = "Intermediate Integration Value\n resetvalue={Application Reset:0x0}"]
    pub type IivaLx = crate::RegValueT<IivaLx_SPEC>;

    impl IivaLx {
        #[doc = "Result of most recent accumulation   IVAL"]
        #[inline(always)]
        pub fn ival(
            self,
        ) -> crate::common::RegisterField<0, 0x3ffffff, 1, 0, u32, IivaLx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0x3ffffff,1,0,u32, IivaLx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for IivaLx {
        #[inline(always)]
        fn default() -> IivaLx {
            <crate::RegValueT<IivaLx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IstaTx_SPEC;
    impl crate::sealed::RegSpec for IstaTx_SPEC {
        type DataType = u32;
    }
    #[doc = "Integrator Status Register 0\n resetvalue={Application Reset:0x0}"]
    pub type IstaTx = crate::RegValueT<IstaTx_SPEC>;

    impl IstaTx {
        #[doc = "Number of Values Counted   NVALCNT. Counts the number of integrated values until integration is started         NVALDIS  or completed  NVALINT"]
        #[inline(always)]
        pub fn nvalcnt(
            self,
        ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, IstaTx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0x3f,1,0,u8, IstaTx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Integration Cycle Counter   REPCNT. Counts the number of integration cycles if activated  IWS  160    160 0 . This number is selected via bitfield REPVAL."]
        #[inline(always)]
        pub fn repcnt(
            self,
        ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, IstaTx_SPEC, crate::common::R> {
            crate::common::RegisterField::<8,0xf,1,0,u8, IstaTx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Integration Enable   INTEN. Indicates the activity of the integrator. For        the control of bit INTEN  see also bitfield ITRMODE in register DICFGx ."]
        #[inline(always)]
        pub fn inten(
            self,
        ) -> crate::common::RegisterField<15, 0x1, 1, 0, istatx::Inten, IstaTx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                istatx::Inten,
                IstaTx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for IstaTx {
        #[inline(always)]
        fn default() -> IstaTx {
            <crate::RegValueT<IstaTx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod istatx {
        pub struct Inten_SPEC;
        pub type Inten = crate::EnumBitfieldStruct<u8, Inten_SPEC>;
        impl Inten {
            #[doc = "Integration stopped.. INTEN is cleared at the end of the integration window  i.e. after REPVAL      cycles or upon the inverse trigger event transition of the external      trigger signal."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Integration enabled.. INTEN is set when the channel is started while permanent integration is        selected  ITRMODE   11   or upon the        defined trigger event."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IwctRx_SPEC;
    impl crate::sealed::RegSpec for IwctRx_SPEC {
        type DataType = u32;
    }
    #[doc = "Integration Window Control Register 0\n resetvalue={Application Reset:0x0}"]
    pub type IwctRx = crate::RegValueT<IwctRx_SPEC>;

    impl IwctRx {
        #[doc = "Integration Window Size   IWS"]
        #[inline(always)]
        pub fn iws(
            self,
        ) -> crate::common::RegisterField<4, 0x1, 1, 0, iwctrx::Iws, IwctRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x1,1,0,iwctrx::Iws, IwctRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Filter Chain Restart Control"]
        #[inline(always)]
        pub fn frc(
            self,
        ) -> crate::common::RegisterField<5, 0x1, 1, 0, iwctrx::Frc, IwctRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x1,1,0,iwctrx::Frc, IwctRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Number of Integration Cycles   REPVAL. Defines the number of integration cycles to be counted by REPCNT if        activated  IWS  160    160 0 . The number of cycles is REPVAL 1."]
        #[inline(always)]
        pub fn repval(
            self,
        ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, IwctRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xf,1,0,u8, IwctRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Number of Values Discarded   NVALDIS. Start the integration cycle after NVALDIS values"]
        #[inline(always)]
        pub fn nvaldis(
            self,
        ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, IwctRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x3f,1,0,u8, IwctRx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Number of Values to be Accumulated   NVALINT. Stop the integration cycle after NVALINT 1 values Use intervals of 2 minimum  so no data is lost due to the data shifter."]
        #[inline(always)]
        pub fn nvalint(
            self,
        ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, IwctRx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0x3f,1,0,u8, IwctRx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for IwctRx {
        #[inline(always)]
        fn default() -> IwctRx {
            <crate::RegValueT<IwctRx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod iwctrx {
        pub struct Iws_SPEC;
        pub type Iws = crate::EnumBitfieldStruct<u8, Iws_SPEC>;
        impl Iws {
            #[doc = "Internal control  stop integrator after REPVAL 1 integration cycles"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "External control  stop integrator upon the inverse trigger event"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Frc_SPEC;
        pub type Frc = crate::EnumBitfieldStruct<u8, Frc_SPEC>;
        impl Frc {
            #[doc = "Restart the filter chain when an integration window starts"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "No influence on filter chain when an integration window starts  except for the integrator itself"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ModcfGx_SPEC;
    impl crate::sealed::RegSpec for ModcfGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Modulator Configuration Register 0\n resetvalue={Application Reset:0x0}"]
    pub type ModcfGx = crate::RegValueT<ModcfGx_SPEC>;

    impl ModcfGx {
        #[doc = "Configuration of Positive Input Line   INCFGP. Defines the internal connection of the positive input."]
        #[inline(always)]
        pub fn incfgp(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x3,
            1,
            0,
            modcfgx::Incfgp,
            ModcfGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x3,
                1,
                0,
                modcfgx::Incfgp,
                ModcfGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Configuration of Negative Input Line   INCFGN. Defines the internal connection of the negative input."]
        #[inline(always)]
        pub fn incfgn(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x3,
            1,
            0,
            modcfgx::Incfgn,
            ModcfGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                2,
                0x3,
                1,
                0,
                modcfgx::Incfgn,
                ModcfGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Input Pin Selection   INSEL. Defines the initial or permanent setting for the input multiplexer         bitfield INMUX  depending on the selected operating mode  bitfield        INMODE ."]
        #[inline(always)]
        pub fn insel(
            self,
        ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, ModcfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x3,1,0,u8, ModcfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Input Multiplexer Setting   INMUX. Indicates the current setting of the input multiplexer connecting the        input pins to the buffer inputs. The product specific appendix details the available channels and their        inputs."]
        #[inline(always)]
        pub fn inmux(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x3,
            1,
            0,
            modcfgx::Inmux,
            ModcfGx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                10,
                0x3,
                1,
                0,
                modcfgx::Inmux,
                ModcfGx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Input Multiplexer Control Mode   INMODE. Defines the condition for a trigger event to control the input        multiplexer. Bitfield INMAC selects the action upon a trigger event."]
        #[inline(always)]
        pub fn inmode(
            self,
        ) -> crate::common::RegisterField<
            12,
            0x3,
            1,
            0,
            modcfgx::Inmode,
            ModcfGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                12,
                0x3,
                1,
                0,
                modcfgx::Inmode,
                ModcfGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Input Multiplexer Action Control   INMAC. Defines the mechanism by which the input multiplexer is controlled."]
        #[inline(always)]
        pub fn inmac(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x1,
            1,
            0,
            modcfgx::Inmac,
            ModcfGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                modcfgx::Inmac,
                ModcfGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Write Control for Input Parameters   INCWC"]
        #[inline(always)]
        pub fn incwc(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            modcfgx::Incwc,
            ModcfGx_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                modcfgx::Incwc,
                ModcfGx_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
        #[doc = "Modulator Clock Period   DIVM. Defines the period of the modulator clock  on chip external   derived        from the peripheral clock  t MOD   t PER   215  CP   f MOD   f PER   CP ."]
        #[inline(always)]
        pub fn divm(
            self,
        ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, ModcfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x7,1,0,u8, ModcfGx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Analog Clock Synchronization Delay   ACSD. Defines the delay in clocks after the sync signal. Valid only if the phase synchronizer is selected  USC  160    160  0  ."]
        #[inline(always)]
        pub fn acsd(
            self,
        ) -> crate::common::RegisterField<
            20,
            0x7,
            1,
            0,
            modcfgx::Acsd,
            ModcfGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0x7,
                1,
                0,
                modcfgx::Acsd,
                ModcfGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Dithering Function Enable   DITHEN. Controls the dithering function for each modulator separately."]
        #[inline(always)]
        pub fn dithen(
            self,
        ) -> crate::common::RegisterField<
            26,
            0x1,
            1,
            0,
            modcfgx::Dithen,
            ModcfGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                26,
                0x1,
                1,
                0,
                modcfgx::Dithen,
                ModcfGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Integrator Reset Enable   IREN. Controls the modulator overdrive handling"]
        #[inline(always)]
        pub fn iren(
            self,
        ) -> crate::common::RegisterField<
            27,
            0x1,
            1,
            0,
            modcfgx::Iren,
            ModcfGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                27,
                0x1,
                1,
                0,
                modcfgx::Iren,
                ModcfGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Automatic Power Control   APC"]
        #[inline(always)]
        pub fn apc(
            self,
        ) -> crate::common::RegisterField<
            28,
            0x3,
            1,
            0,
            modcfgx::Apc,
            ModcfGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                28,
                0x3,
                1,
                0,
                modcfgx::Apc,
                ModcfGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Write Control for Modulator Mode Settings   MMWC"]
        #[inline(always)]
        pub fn mmwc(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            modcfgx::Mmwc,
            ModcfGx_SPEC,
            crate::common::W,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                modcfgx::Mmwc,
                ModcfGx_SPEC,
                crate::common::W,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for ModcfGx {
        #[inline(always)]
        fn default() -> ModcfGx {
            <crate::RegValueT<ModcfGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod modcfgx {
        pub struct Incfgp_SPEC;
        pub type Incfgp = crate::EnumBitfieldStruct<u8, Incfgp_SPEC>;
        impl Incfgp {
            #[doc = "00 Input pin"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Reference        voltage V AREF"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "V REFX  see VREFXSEL setting in        register VCM"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Reference        ground V AGND"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Incfgn_SPEC;
        pub type Incfgn = crate::EnumBitfieldStruct<u8, Incfgn_SPEC>;
        impl Incfgn {
            #[doc = "00 Input pin"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Reference        voltage V AREF"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "V REFX  see VREFXSEL setting in        register VCM"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Reference        ground V AGND"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Inmux_SPEC;
        pub type Inmux = crate::EnumBitfieldStruct<u8, Inmux_SPEC>;
        impl Inmux {
            #[doc = "Input pin position A"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Input pin position B"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Input pin position C"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Input pin position D"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Inmode_SPEC;
        pub type Inmode = crate::EnumBitfieldStruct<u8, Inmode_SPEC>;
        impl Inmode {
            #[doc = "Software control  INMUX follows INSEL"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Trigger event upon a falling edge"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Trigger event upon a rising edge"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Trigger event upon any edge"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Inmac_SPEC;
        pub type Inmac = crate::EnumBitfieldStruct<u8, Inmac_SPEC>;
        impl Inmac {
            #[doc = "Preset mode  load INMUX upon a trigger"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Single step mode.  decrement INMUX upon a trigger  wrap around to  lt INSEL gt"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Incwc_SPEC;
        pub type Incwc = crate::EnumBitfieldStruct<u8, Incwc_SPEC>;
        impl Incwc {
            #[doc = "No write access to input parameters"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Bitfields INCFGP  INCFGN  GAINSEL  INSEL  INMODE  INMAC can be written"]
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
        pub struct Dithen_SPEC;
        pub type Dithen = crate::EnumBitfieldStruct<u8, Dithen_SPEC>;
        impl Dithen {
            #[doc = "Disable dithering"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Dithering is enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Iren_SPEC;
        pub type Iren = crate::EnumBitfieldStruct<u8, Iren_SPEC>;
        impl Iren {
            #[doc = "No integrator reset"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Integrators are reset in case of an overdrive"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Apc_SPEC;
        pub type Apc = crate::EnumBitfieldStruct<u8, Apc_SPEC>;
        impl Apc {
            #[doc = "Off  Modulator active while its associated bit MxRUN is set"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Slow standby mode. on chip modulator and voltage regulator are deactivated  external        modulator clock is disabled  while the gate signal  selected trigger         is inactive"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Fast standby mode . on chip modulator is deactivated  external modulator clock is        disabled  while the gate signal  selected trigger  is inactive"]
            pub const CONST_22: Self = Self::new(2);
        }
        pub struct Mmwc_SPEC;
        pub type Mmwc = crate::EnumBitfieldStruct<u8, Mmwc_SPEC>;
        impl Mmwc {
            #[doc = "No write access to mode settings"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Bitfields APC  IREN  DITHEN  ACSD  DIVM can be written"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OffcomPx_SPEC;
    impl crate::sealed::RegSpec for OffcomPx_SPEC {
        type DataType = u32;
    }
    #[doc = "Offset Compensation Register 0\n resetvalue={Application Reset:0x0}"]
    pub type OffcomPx = crate::RegValueT<OffcomPx_SPEC>;

    impl OffcomPx {
        #[doc = "Offset Value   OFFSET. Half of this signed value is subtracted from each result produced by the        filter chain. Bit 0 represents 1 2 LSB. This increases the precision in case of          accumulated result values  e.g. in the integrator."]
        #[inline(always)]
        pub fn offset(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, OffcomPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, OffcomPx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for OffcomPx {
        #[inline(always)]
        fn default() -> OffcomPx {
            <crate::RegValueT<OffcomPx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OvscfGx_SPEC;
    impl crate::sealed::RegSpec for OvscfGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Overshoot Compensation Cfg. Register 0\n resetvalue={Application Reset:0x0}"]
    pub type OvscfGx = crate::RegValueT<OvscfGx_SPEC>;

    impl OvscfGx {
        #[doc = "Slew Rate Filter Strength. Defines the time constant for the slew rate filter."]
        #[inline(always)]
        pub fn srfs(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x3,
            1,
            0,
            ovscfgx::Srfs,
            OvscfGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x3,
                1,
                0,
                ovscfgx::Srfs,
                OvscfGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Slew Rate Filter Run Time. Defines the time constant for the slew rate filter."]
        #[inline(always)]
        pub fn srfrt(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x3,
            1,
            0,
            ovscfgx::Srfrt,
            OvscfGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                2,
                0x3,
                1,
                0,
                ovscfgx::Srfrt,
                OvscfGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Step Detection Mode. Defines when the slew rate filter is activated."]
        #[inline(always)]
        pub fn sdm(
            self,
        ) -> crate::common::RegisterField<4, 0x1, 1, 0, ovscfgx::Sdm, OvscfGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                ovscfgx::Sdm,
                OvscfGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Step Detection Threshold. Defines the threshold value  magnitude  used for step detection. The        threshold value is  lt SDTH gt    215  32"]
        #[inline(always)]
        pub fn sdth(
            self,
        ) -> crate::common::RegisterField<
            16,
            0x7ff,
            1,
            0,
            ovscfgx::Sdth,
            OvscfGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0x7ff,
                1,
                0,
                ovscfgx::Sdth,
                OvscfGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for OvscfGx {
        #[inline(always)]
        fn default() -> OvscfGx {
            <crate::RegValueT<OvscfGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod ovscfgx {
        pub struct Srfs_SPEC;
        pub type Srfs = crate::EnumBitfieldStruct<u8, Srfs_SPEC>;
        impl Srfs {
            #[doc = "Minimum filter effect  early attenuation  linear operation"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Weak filter effect"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Medium filter effect"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Maximum filter effect  steep beginning  smooth end"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Srfrt_SPEC;
        pub type Srfrt = crate::EnumBitfieldStruct<u8, Srfrt_SPEC>;
        impl Srfrt {
            #[doc = "2 input cycles"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "4 input cycles"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "8 input cycles"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "16 input cycles"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Sdm_SPEC;
        pub type Sdm = crate::EnumBitfieldStruct<u8, Sdm_SPEC>;
        impl Sdm {
            #[doc = "Compare threshold to difference of current and last input"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Compare threshold to difference of current and second last input"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Sdth_SPEC;
        pub type Sdth = crate::EnumBitfieldStruct<u8, Sdth_SPEC>;
        impl Sdth {
            #[doc = "0  slew rate filter active all the time"]
            pub const CONST_00: Self = Self::new(0);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RectcfGx_SPEC;
    impl crate::sealed::RegSpec for RectcfGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Rectification Configuration Register 0\n resetvalue={Application Reset:0x080000000}"]
    pub type RectcfGx = crate::RegValueT<RectcfGx_SPEC>;

    impl RectcfGx {
        #[doc = "Rectification Enable   RFEN. General control of the rectifier circuit. Rectification is only active while the integrator is active."]
        #[inline(always)]
        pub fn rfen(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            rectcfgx::Rfen,
            RectcfGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                rectcfgx::Rfen,
                RectcfGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Sign Source. Selects the sign signal that is to be delayed."]
        #[inline(always)]
        pub fn ssrc(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x3,
            1,
            0,
            rectcfgx::Ssrc,
            RectcfGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x3,
                1,
                0,
                rectcfgx::Ssrc,
                RectcfGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Valid Flag   SDCVAL. Indicates a new value in bitfield SDCAP."]
        #[inline(always)]
        pub fn sdcval(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            rectcfgx::Sdcval,
            RectcfGx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                rectcfgx::Sdcval,
                RectcfGx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Selected Carrier Sign Signal   SGNCS"]
        #[inline(always)]
        pub fn sgncs(
            self,
        ) -> crate::common::RegisterField<
            30,
            0x1,
            1,
            0,
            rectcfgx::Sgncs,
            RectcfGx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                30,
                0x1,
                1,
                0,
                rectcfgx::Sgncs,
                RectcfGx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Sign Signal Delayed   SGND"]
        #[inline(always)]
        pub fn sgnd(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            rectcfgx::Sgnd,
            RectcfGx_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                rectcfgx::Sgnd,
                RectcfGx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for RectcfGx {
        #[inline(always)]
        fn default() -> RectcfGx {
            <crate::RegValueT<RectcfGx_SPEC> as RegisterValue<_>>::new(2147483648)
        }
    }
    pub mod rectcfgx {
        pub struct Rfen_SPEC;
        pub type Rfen = crate::EnumBitfieldStruct<u8, Rfen_SPEC>;
        impl Rfen {
            #[doc = "No rectification  data not altered"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Data are rectified according to SGND"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Ssrc_SPEC;
        pub type Ssrc = crate::EnumBitfieldStruct<u8, Ssrc_SPEC>;
        impl Ssrc {
            #[doc = "On chip carrier generator"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Sign of result of channel selected by bitfield SSCH"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "External sign signal A"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "External sign signal B"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Sdcval_SPEC;
        pub type Sdcval = crate::EnumBitfieldStruct<u8, Sdcval_SPEC>;
        impl Sdcval {
            #[doc = "No new result available"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Bitfield SDCAP has been updated with a new captured value and has not yet been read"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Sgncs_SPEC;
        pub type Sgncs = crate::EnumBitfieldStruct<u8, Sgncs_SPEC>;
        impl Sgncs {
            #[doc = "Positive values"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Negative values"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Sgnd_SPEC;
        pub type Sgnd = crate::EnumBitfieldStruct<u8, Sgnd_SPEC>;
        impl Sgnd {
            #[doc = "Positive values"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Negative values"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ResAx_SPEC;
    impl crate::sealed::RegSpec for ResAx_SPEC {
        type DataType = u32;
    }
    #[doc = "Result Register 0 Auxiliary\n resetvalue={Application Reset:0x0}"]
    pub type ResAx = crate::RegValueT<ResAx_SPEC>;

    impl ResAx {
        #[doc = "Most Recent Result of Auxiliary Filter"]
        #[inline(always)]
        pub fn result(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, ResAx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, ResAx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for ResAx {
        #[inline(always)]
        fn default() -> ResAx {
            <crate::RegValueT<ResAx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ResMx_SPEC;
    impl crate::sealed::RegSpec for ResMx_SPEC {
        type DataType = u32;
    }
    #[doc = "Result Register 0 Main\n resetvalue={Application Reset:0x0}"]
    pub type ResMx = crate::RegValueT<ResMx_SPEC>;

    impl ResMx {
        #[doc = "Result Value Lower Part   RESULTLO. Returns the next value from the result FIFO  Result or timestamp  see CROSSREFERENCE"]
        #[inline(always)]
        pub fn resultlo(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, ResMx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, ResMx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Result Value Higher Part   RESULTHI. Returns an additional value  Sign extension  result from FIFO  timestamp  or zero  see CROSSREFERENCE"]
        #[inline(always)]
        pub fn resulthi(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, ResMx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, ResMx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for ResMx {
        #[inline(always)]
        fn default() -> ResMx {
            <crate::RegValueT<ResMx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RfCx_SPEC;
    impl crate::sealed::RegSpec for RfCx_SPEC {
        type DataType = u32;
    }
    #[doc = "Result FIFO Control Register 0\n resetvalue={Application Reset:0x0}"]
    pub type RfCx = crate::RegValueT<RfCx_SPEC>;

    impl RfCx {
        #[doc = "Service Request FIFO Level"]
        #[inline(always)]
        pub fn srlvl(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, rfcx::Srlvl, RfCx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3,1,0,rfcx::Srlvl, RfCx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Read Error Flag Clear"]
        #[inline(always)]
        pub fn rdec(
            self,
        ) -> crate::common::RegisterField<4, 0x1, 1, 0, rfcx::Rdec, RfCx_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<4,0x1,1,0,rfcx::Rdec, RfCx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Write Error Flag Clear"]
        #[inline(always)]
        pub fn wrec(
            self,
        ) -> crate::common::RegisterField<5, 0x1, 1, 0, rfcx::Wrec, RfCx_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<5,0x1,1,0,rfcx::Wrec, RfCx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "FIFO Flush"]
        #[inline(always)]
        pub fn fifl(
            self,
        ) -> crate::common::RegisterField<6, 0x1, 1, 0, rfcx::Fifl, RfCx_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<6,0x1,1,0,rfcx::Fifl, RfCx_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Read Error Flag"]
        #[inline(always)]
        pub fn rderr(
            self,
        ) -> crate::common::RegisterField<20, 0x1, 1, 0, rfcx::Rderr, RfCx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<20,0x1,1,0,rfcx::Rderr, RfCx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Write Error Flag"]
        #[inline(always)]
        pub fn wrerr(
            self,
        ) -> crate::common::RegisterField<21, 0x1, 1, 0, rfcx::Wrerr, RfCx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<21,0x1,1,0,rfcx::Wrerr, RfCx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for RfCx {
        #[inline(always)]
        fn default() -> RfCx {
            <crate::RegValueT<RfCx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod rfcx {
        pub struct Srlvl_SPEC;
        pub type Srlvl = crate::EnumBitfieldStruct<u8, Srlvl_SPEC>;
        impl Srlvl {
            #[doc = "Generate service request when FIFO fill level reaches 1 value"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Generate service request when FIFO fill level reaches 2 values"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Generate service request when FIFO fill level reaches 3 values"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Generate service request when FIFO fill level reaches 4 values"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Rdec_SPEC;
        pub type Rdec = crate::EnumBitfieldStruct<u8, Rdec_SPEC>;
        impl Rdec {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear flag RDERR"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Wrec_SPEC;
        pub type Wrec = crate::EnumBitfieldStruct<u8, Wrec_SPEC>;
        impl Wrec {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Clear flag WRERR"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Fifl_SPEC;
        pub type Fifl = crate::EnumBitfieldStruct<u8, Fifl_SPEC>;
        impl Fifl {
            #[doc = "No action"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Remove all entries from result FIFO"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rderr_SPEC;
        pub type Rderr = crate::EnumBitfieldStruct<u8, Rderr_SPEC>;
        impl Rderr {
            #[doc = "No problem encountered"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "A read access occurred while the FIFO was empty. A read error is also indicated when a read access occurs during the        FIFO  8217 s synchronization stall phase  4 clock cycles after a read access .        Clear this sticky flag by writing 1 to bit RDEC."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Wrerr_SPEC;
        pub type Wrerr = crate::EnumBitfieldStruct<u8, Wrerr_SPEC>;
        impl Wrerr {
            #[doc = "No problem encountered"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "A write access occurred while the FIFO was full. Clear this sticky flag by writing 1 to bit WREC."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TesTx_SPEC;
    impl crate::sealed::RegSpec for TesTx_SPEC {
        type DataType = u32;
    }
    #[doc = "Test Register 0\n resetvalue={Application Reset:0x0}"]
    pub type TesTx = crate::RegValueT<TesTx_SPEC>;

    impl TesTx {
        #[doc = "Internal Channel Test Mode   ICTM. Enable the fractional reference voltage VAREF X by setting bit          VCM.VXON. 20   181 s settling time are required."]
        #[inline(always)]
        pub fn ictm(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, testx::Ictm, TesTx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3,1,0,testx::Ictm, TesTx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Internal Channel Test Enable   ICTEN"]
        #[inline(always)]
        pub fn icten(
            self,
        ) -> crate::common::RegisterField<2, 0x1, 1, 0, testx::Icten, TesTx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x1,1,0,testx::Icten, TesTx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Test Mode   TESTMODE. Selects which voltage is connected to the analog test bus. All other        combinations must not be used  MAY LEAD TO SHORTCUTS  ."]
        #[inline(always)]
        pub fn testmode(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1f,
            1,
            0,
            testx::Testmode,
            TesTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x1f,
                1,
                0,
                testx::Testmode,
                TesTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Enable burn in mode for IVR"]
        #[inline(always)]
        pub fn ivrenbi(
            self,
        ) -> crate::common::RegisterField<
            13,
            0x1,
            1,
            0,
            testx::Ivrenbi,
            TesTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                13,
                0x1,
                1,
                0,
                testx::Ivrenbi,
                TesTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Comparator Delay in Quantizer   COMPDELAY"]
        #[inline(always)]
        pub fn compdelay(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x3,
            1,
            0,
            testx::Compdelay,
            TesTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                14,
                0x3,
                1,
                0,
                testx::Compdelay,
                TesTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Bias Current Control for Comparator"]
        #[inline(always)]
        pub fn biascomp(
            self,
        ) -> crate::common::RegisterField<
            16,
            0x3,
            1,
            0,
            testx::Biascomp,
            TesTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0x3,
                1,
                0,
                testx::Biascomp,
                TesTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Bias Current Control for OTA2"]
        #[inline(always)]
        pub fn biasota2(
            self,
        ) -> crate::common::RegisterField<
            18,
            0x3,
            1,
            0,
            testx::Biasota2,
            TesTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                18,
                0x3,
                1,
                0,
                testx::Biasota2,
                TesTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Bias Current Control for OTA1"]
        #[inline(always)]
        pub fn biasota1(
            self,
        ) -> crate::common::RegisterField<
            20,
            0x3,
            1,
            0,
            testx::Biasota1,
            TesTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0x3,
                1,
                0,
                testx::Biasota1,
                TesTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Bias Current Control for Reference Buffer"]
        #[inline(always)]
        pub fn biasrbuf(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x3,
            1,
            0,
            testx::Biasrbuf,
            TesTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                22,
                0x3,
                1,
                0,
                testx::Biasrbuf,
                TesTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Bias Current Control for Single Ended Buffer"]
        #[inline(always)]
        pub fn biassebuf(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x3,
            1,
            0,
            testx::Biassebuf,
            TesTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0x3,
                1,
                0,
                testx::Biassebuf,
                TesTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Test Control   TC. Not listed combinations block write access to remaining bitfields."]
        #[inline(always)]
        pub fn tc(
            self,
        ) -> crate::common::RegisterField<28, 0xf, 1, 0, testx::Tc, TesTx_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<28,0xf,1,0,testx::Tc, TesTx_SPEC,crate::common::W>::from_register(self,0)
        }
    }
    impl core::default::Default for TesTx {
        #[inline(always)]
        fn default() -> TesTx {
            <crate::RegValueT<TesTx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod testx {
        pub struct Ictm_SPEC;
        pub type Ictm = crate::EnumBitfieldStruct<u8, Ictm_SPEC>;
        impl Ictm {
            #[doc = "00 Channel test        mode  analog path  using V AREF"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Channel test        mode  analog path  using V AREF   X"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Channel test        mode  digital path  using V AREF"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Channel test        mode  digital path  using V AREF   X"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Icten_SPEC;
        pub type Icten = crate::EnumBitfieldStruct<u8, Icten_SPEC>;
        impl Icten {
            #[doc = "Normal operation"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Channel test mode according to bitfield ICTM"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Testmode_SPEC;
        pub type Testmode = crate::EnumBitfieldStruct<u8, Testmode_SPEC>;
        impl Testmode {
            #[doc = "None"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1.2 V supply"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "3.3 V supply"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Kilis voltage  module power consumption"]
            pub const CONST_44: Self = Self::new(4);
        }
        pub struct Ivrenbi_SPEC;
        pub type Ivrenbi = crate::EnumBitfieldStruct<u8, Ivrenbi_SPEC>;
        impl Ivrenbi {
            #[doc = "Normal operation"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "IVR is switched to burn in mode"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Compdelay_SPEC;
        pub type Compdelay = crate::EnumBitfieldStruct<u8, Compdelay_SPEC>;
        impl Compdelay {
            #[doc = "Delay 1"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Delay 2"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "Delay 3"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "Delay 4"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Biascomp_SPEC;
        pub type Biascomp = crate::EnumBitfieldStruct<u8, Biascomp_SPEC>;
        impl Biascomp {
            #[doc = "00 Nominal        current"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Minimum        current"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Maximum        current"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Nominal        current"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Biasota2_SPEC;
        pub type Biasota2 = crate::EnumBitfieldStruct<u8, Biasota2_SPEC>;
        impl Biasota2 {
            #[doc = "00 Nominal        current"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Minimum        current"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Maximum        current"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Nominal        current"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Biasota1_SPEC;
        pub type Biasota1 = crate::EnumBitfieldStruct<u8, Biasota1_SPEC>;
        impl Biasota1 {
            #[doc = "00 Nominal current"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Minimum current"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Maximum current"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Nominal current"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Biasrbuf_SPEC;
        pub type Biasrbuf = crate::EnumBitfieldStruct<u8, Biasrbuf_SPEC>;
        impl Biasrbuf {
            #[doc = "00 Nominal current"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Minimum current"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Maximum current"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Nominal current"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Biassebuf_SPEC;
        pub type Biassebuf = crate::EnumBitfieldStruct<u8, Biassebuf_SPEC>;
        impl Biassebuf {
            #[doc = "00 Nominal current"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Minimum current"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Maximum current"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Nominal current"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Tc_SPEC;
        pub type Tc = crate::EnumBitfieldStruct<u8, Tc_SPEC>;
        impl Tc {
            #[doc = "Write access enabled. The remaining bitfields of register TESTx can be written."]
            pub const CONST_1111: Self = Self::new(11);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TscnTx_SPEC;
    impl crate::sealed::RegSpec for TscnTx_SPEC {
        type DataType = u32;
    }
    #[doc = "Time Stamp Counter 0\n resetvalue={Application Reset:0x0}"]
    pub type TscnTx = crate::RegValueT<TscnTx_SPEC>;

    impl TscnTx {
        #[doc = "Timestamp Counter Value   TSCOUNT. TSCOUNT is clocked with the modulator clock and is cleared when a new result value has been generated."]
        #[inline(always)]
        pub fn tscount(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, TscnTx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, TscnTx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Timestamp Counter Clock Selection   TSCLK"]
        #[inline(always)]
        pub fn tsclk(
            self,
        ) -> crate::common::RegisterField<
            16,
            0x3,
            1,
            0,
            tscntx::Tsclk,
            TscnTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0x3,
                1,
                0,
                tscntx::Tsclk,
                TscnTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Timestamp Counter Run Control   TSCRUN"]
        #[inline(always)]
        pub fn tscrun(
            self,
        ) -> crate::common::RegisterField<
            19,
            0x1,
            1,
            0,
            tscntx::Tscrun,
            TscnTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                19,
                0x1,
                1,
                0,
                tscntx::Tscrun,
                TscnTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Analog MUX Setting Copy Enable   AMXCOPY. Allows copying of bitfield AMX into bitfield TIMESTAMP  in register TSTMPx  ."]
        #[inline(always)]
        pub fn amxcopy(
            self,
        ) -> crate::common::RegisterField<
            20,
            0x1,
            1,
            0,
            tscntx::Amxcopy,
            TscnTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0x1,
                1,
                0,
                tscntx::Amxcopy,
                TscnTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for TscnTx {
        #[inline(always)]
        fn default() -> TscnTx {
            <crate::RegValueT<TscnTx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod tscntx {
        pub struct Tsclk_SPEC;
        pub type Tsclk = crate::EnumBitfieldStruct<u8, Tsclk_SPEC>;
        impl Tsclk {
            #[doc = "00 f TSTMP   f MOD"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 f TSTMP   f MOD   2"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 f TSTMP   f MOD   4"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 f TSTMP   f MOD   8"]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Tscrun_SPEC;
        pub type Tscrun = crate::EnumBitfieldStruct<u8, Tscrun_SPEC>;
        impl Tscrun {
            #[doc = "Timestamp counter is off"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Timestamp counter is counting at the rate selected by bitfield TSCLK"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Amxcopy_SPEC;
        pub type Amxcopy = crate::EnumBitfieldStruct<u8, Amxcopy_SPEC>;
        impl Amxcopy {
            #[doc = "Do not copy  timestamp uses all 16 bits"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Copy AMX to bits TIMESTAMP 15 14   timestamp uses lower 14 bits. 1"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TstmPx_SPEC;
    impl crate::sealed::RegSpec for TstmPx_SPEC {
        type DataType = u32;
    }
    #[doc = "Time Stamp Register 0\n resetvalue={Application Reset:0x0}"]
    pub type TstmPx = crate::RegValueT<TstmPx_SPEC>;

    impl TstmPx {
        #[doc = "The Most Recent Captured Timestamp Value   TIMESTAMP. This value is copied from the timestamp counter TSCOUNT If bit AMXCOPY in register TSCNTx is 1  bits TIMESTAMP 15 14  are replaced with a copy of bitfield AMX."]
        #[inline(always)]
        pub fn timestamp(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, TstmPx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, TstmPx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Analog Multiplexer Setting   AMX. This value is copied from bitfield INMUX in register MODCFGx"]
        #[inline(always)]
        pub fn amx(
            self,
        ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, TstmPx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0x3,1,0,u8, TstmPx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Timestamp Valid   TSVAL. Indicates valid timestamp information."]
        #[inline(always)]
        pub fn tsval(
            self,
        ) -> crate::common::RegisterField<31, 0x1, 1, 0, tstmpx::Tsval, TstmPx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                tstmpx::Tsval,
                TstmPx_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for TstmPx {
        #[inline(always)]
        fn default() -> TstmPx {
            <crate::RegValueT<TstmPx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod tstmpx {
        pub struct Tsval_SPEC;
        pub type Tsval = crate::EnumBitfieldStruct<u8, Tsval_SPEC>;
        impl Tsval {
            #[doc = "No timestamp trigger occurred since last read access"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "Timestamp information has been stored after a timestamp trigger"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct VcMx_SPEC;
    impl crate::sealed::RegSpec for VcMx_SPEC {
        type DataType = u32;
    }
    #[doc = "Common Mode Voltage Register 0\n resetvalue={Application Reset:0x0}"]
    pub type VcMx = crate::RegValueT<VcMx_SPEC>;

    impl VcMx {
        #[doc = "Fractional Reference Voltage Selection   VREFXSEL"]
        #[inline(always)]
        pub fn vrefxsel(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, vcmx::Vrefxsel, VcMx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3,1,0,vcmx::Vrefxsel, VcMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Fractional Reference Voltage Enable   VXON"]
        #[inline(always)]
        pub fn vxon(
            self,
        ) -> crate::common::RegisterField<2, 0x1, 1, 0, vcmx::Vxon, VcMx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x1,1,0,vcmx::Vxon, VcMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Voltage Control of Positive Inputs 3 of CH0. Defines the connection of the respective positive input y to the common        mode voltage. y indicates the input of the analog multiplexers  if available ."]
        #[inline(always)]
        pub fn inpvc0(
            self,
        ) -> crate::common::RegisterField<16, 0x1, 1, 0, vcmx::Inpvc0, VcMx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1,1,0,vcmx::Inpvc0, VcMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Voltage Control of Positive Inputs 3 of CH0. Defines the connection of the respective positive input y to the common        mode voltage. y indicates the input of the analog multiplexers  if available ."]
        #[inline(always)]
        pub fn inpvc1(
            self,
        ) -> crate::common::RegisterField<17, 0x1, 1, 0, vcmx::Inpvc1, VcMx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<17,0x1,1,0,vcmx::Inpvc1, VcMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Voltage Control of Positive Inputs 3 of CH0. Defines the connection of the respective positive input y to the common        mode voltage. y indicates the input of the analog multiplexers  if available ."]
        #[inline(always)]
        pub fn inpvc2(
            self,
        ) -> crate::common::RegisterField<18, 0x1, 1, 0, vcmx::Inpvc2, VcMx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<18,0x1,1,0,vcmx::Inpvc2, VcMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Voltage Control of Positive Inputs 3 of CH0. Defines the connection of the respective positive input y to the common        mode voltage. y indicates the input of the analog multiplexers  if available ."]
        #[inline(always)]
        pub fn inpvc3(
            self,
        ) -> crate::common::RegisterField<19, 0x1, 1, 0, vcmx::Inpvc3, VcMx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<19,0x1,1,0,vcmx::Inpvc3, VcMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Voltage Control of Negative Inputs 3 of CH0. Defines the connection of the respective negative input y to the common        mode voltage. y indicates the input of the analog multiplexers  if available ."]
        #[inline(always)]
        pub fn innvc0(
            self,
        ) -> crate::common::RegisterField<20, 0x1, 1, 0, vcmx::Innvc0, VcMx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0x1,1,0,vcmx::Innvc0, VcMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Voltage Control of Negative Inputs 3 of CH0. Defines the connection of the respective negative input y to the common        mode voltage. y indicates the input of the analog multiplexers  if available ."]
        #[inline(always)]
        pub fn innvc1(
            self,
        ) -> crate::common::RegisterField<21, 0x1, 1, 0, vcmx::Innvc1, VcMx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<21,0x1,1,0,vcmx::Innvc1, VcMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Voltage Control of Negative Inputs 3 of CH0. Defines the connection of the respective negative input y to the common        mode voltage. y indicates the input of the analog multiplexers  if available ."]
        #[inline(always)]
        pub fn innvc2(
            self,
        ) -> crate::common::RegisterField<22, 0x1, 1, 0, vcmx::Innvc2, VcMx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<22,0x1,1,0,vcmx::Innvc2, VcMx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Voltage Control of Negative Inputs 3 of CH0. Defines the connection of the respective negative input y to the common        mode voltage. y indicates the input of the analog multiplexers  if available ."]
        #[inline(always)]
        pub fn innvc3(
            self,
        ) -> crate::common::RegisterField<23, 0x1, 1, 0, vcmx::Innvc3, VcMx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<23,0x1,1,0,vcmx::Innvc3, VcMx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for VcMx {
        #[inline(always)]
        fn default() -> VcMx {
            <crate::RegValueT<VcMx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod vcmx {
        pub struct Vrefxsel_SPEC;
        pub type Vrefxsel = crate::EnumBitfieldStruct<u8, Vrefxsel_SPEC>;
        impl Vrefxsel {
            #[doc = "00 V REFX   V AREF   2"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 V REFX   V AREF   4"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 V REFX   V AREF   8"]
            pub const CONST_22: Self = Self::new(2);
        }
        pub struct Vxon_SPEC;
        pub type Vxon = crate::EnumBitfieldStruct<u8, Vxon_SPEC>;
        impl Vxon {
            #[doc = "0 V REFX is not connected"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 V REFX is connected  value according to VREFXSEL"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Inpvc0_SPEC;
        pub type Inpvc0 = crate::EnumBitfieldStruct<u8, Inpvc0_SPEC>;
        impl Inpvc0 {
            #[doc = "No connection to common mode voltage"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "This pin is connected to the common mode voltage"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Inpvc1_SPEC;
        pub type Inpvc1 = crate::EnumBitfieldStruct<u8, Inpvc1_SPEC>;
        impl Inpvc1 {
            #[doc = "No connection to common mode voltage"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "This pin is connected to the common mode voltage"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Inpvc2_SPEC;
        pub type Inpvc2 = crate::EnumBitfieldStruct<u8, Inpvc2_SPEC>;
        impl Inpvc2 {
            #[doc = "No connection to common mode voltage"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "This pin is connected to the common mode voltage"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Inpvc3_SPEC;
        pub type Inpvc3 = crate::EnumBitfieldStruct<u8, Inpvc3_SPEC>;
        impl Inpvc3 {
            #[doc = "No connection to common mode voltage"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "This pin is connected to the common mode voltage"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Innvc0_SPEC;
        pub type Innvc0 = crate::EnumBitfieldStruct<u8, Innvc0_SPEC>;
        impl Innvc0 {
            #[doc = "No connection to common mode voltage"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "This pin is connected to the common mode voltage"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Innvc1_SPEC;
        pub type Innvc1 = crate::EnumBitfieldStruct<u8, Innvc1_SPEC>;
        impl Innvc1 {
            #[doc = "No connection to common mode voltage"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "This pin is connected to the common mode voltage"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Innvc2_SPEC;
        pub type Innvc2 = crate::EnumBitfieldStruct<u8, Innvc2_SPEC>;
        impl Innvc2 {
            #[doc = "No connection to common mode voltage"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "This pin is connected to the common mode voltage"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Innvc3_SPEC;
        pub type Innvc3 = crate::EnumBitfieldStruct<u8, Innvc3_SPEC>;
        impl Innvc3 {
            #[doc = "No connection to common mode voltage"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "This pin is connected to the common mode voltage"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
}
