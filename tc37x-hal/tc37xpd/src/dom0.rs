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
#[doc = r"DOM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dom0(pub(super) *mut u8);
unsafe impl core::marker::Send for Dom0 {}
unsafe impl core::marker::Sync for Dom0 {}
impl Dom0 {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1264usize)) }
    }

    #[doc = "Domain 0 Bridge Control Register\n resetvalue={Application Reset:0x0,Application Reset:0x200,Application Reset:0x0}"]
    #[inline(always)]
    pub const fn brcon(&self) -> crate::common::Reg<self::Brcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1072usize)) }
    }

    #[doc = "Identification Register\n resetvalue={Application Reset:0x4D000}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1032usize)) }
    }

    #[doc = "Protocol Error Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pestat(&self) -> crate::common::Reg<self::Pestat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1040usize)) }
    }

    #[doc = "Transaction ID Enable Register\n resetvalue={Application Reset:0x0FFF3FFF}"]
    #[inline(always)]
    pub const fn tiden(&self) -> crate::common::Reg<self::Tiden_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1056usize)) }
    }

    #[doc = "Transaction ID Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tidstat(&self) -> crate::common::Reg<self::Tidstat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1048usize)) }
    }
    #[doc = "SCICTRL"]
    #[inline(always)]
    pub fn scictrl(self) -> [self::Scictrl; 16] {
        unsafe {
            [
                self::Scictrl(self.0.add(0x0usize + 0x0usize)),
                self::Scictrl(self.0.add(0x0usize + 0x20usize)),
                self::Scictrl(self.0.add(0x0usize + 0x40usize)),
                self::Scictrl(self.0.add(0x0usize + 0x60usize)),
                self::Scictrl(self.0.add(0x0usize + 0x80usize)),
                self::Scictrl(self.0.add(0x0usize + 0xa0usize)),
                self::Scictrl(self.0.add(0x0usize + 0xc0usize)),
                self::Scictrl(self.0.add(0x0usize + 0xe0usize)),
                self::Scictrl(self.0.add(0x0usize + 0x100usize)),
                self::Scictrl(self.0.add(0x0usize + 0x120usize)),
                self::Scictrl(self.0.add(0x0usize + 0x140usize)),
                self::Scictrl(self.0.add(0x0usize + 0x160usize)),
                self::Scictrl(self.0.add(0x0usize + 0x180usize)),
                self::Scictrl(self.0.add(0x0usize + 0x1a0usize)),
                self::Scictrl(self.0.add(0x0usize + 0x1c0usize)),
                self::Scictrl(self.0.add(0x0usize + 0x1e0usize)),
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
pub struct Brcon_SPEC;
impl crate::sealed::RegSpec for Brcon_SPEC {
    type DataType = u32;
}
#[doc = "Domain 0 Bridge Control Register\n resetvalue={Application Reset:0x0,Application Reset:0x200,Application Reset:0x0}"]
pub type Brcon = crate::RegValueT<Brcon_SPEC>;

impl Brcon {
    #[doc = "Online Data Acquisition Enable   OLDAEN. This bit is used to control trap generated for write accesses to the        OLDA address range associated with this domain."]
    #[inline(always)]
    pub fn oldaen(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, brcon::Oldaen, Brcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,brcon::Oldaen, Brcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disable Fast Request Feature for SPB to SRI Transactions   FREQDISF"]
    #[inline(always)]
    pub fn freqdisf(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, brcon::Freqdisf, Brcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,brcon::Freqdisf, Brcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "No Deferred Transactions   NODELTR"]
    #[inline(always)]
    pub fn nodeltr(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, brcon::Nodeltr, Brcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,brcon::Nodeltr, Brcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "No deferred RMW transactions   NORMW"]
    #[inline(always)]
    pub fn normw(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, brcon::Normw, Brcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,brcon::Normw, Brcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SPB Max Wait States   MAX WS. If deferring is enabled  defer a SPB read or RMW transaction after        waiting for MAX WS clocks.The value should be greater than 32         otherwise  all transactions will be retried"]
    #[inline(always)]
    pub fn max_ws(
        self,
    ) -> crate::common::RegisterField<13, 0x7f, 1, 0, u8, Brcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x7f,1,0,u8, Brcon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Brcon {
    #[inline(always)]
    fn default() -> Brcon {
        <crate::RegValueT<Brcon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod brcon {
    pub struct Oldaen_SPEC;
    pub type Oldaen = crate::EnumBitfieldStruct<u8, Oldaen_SPEC>;
    impl Oldaen {
        #[doc = "0 Trap generated on a write access to the OLDA memory range."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No trap generated on a write access to the OLDA memory range."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Freqdisf_SPEC;
    pub type Freqdisf = crate::EnumBitfieldStruct<u8, Freqdisf_SPEC>;
    impl Freqdisf {
        #[doc = "0 Fast request feature is enabled  default"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Fast request feature is disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Nodeltr_SPEC;
    pub type Nodeltr = crate::EnumBitfieldStruct<u8, Nodeltr_SPEC>;
    impl Nodeltr {
        #[doc = "0 SPB Transactions are deferred An SPB transaction can be deferred by responding to it with a RETRY rather than a completion code. if MAX WS are exceeded"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SPB Transactions are never deferred  default"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Normw_SPEC;
    pub type Normw = crate::EnumBitfieldStruct<u8, Normw_SPEC>;
    impl Normw {
        #[doc = "0 SPB RMW transactions may be deferred  default"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SPB RMW transactions are not deferred"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Identification Register\n resetvalue={Application Reset:0x4D000}"]
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
    #[doc = "Module Type   MOD TYPE. The bit field is set to D0H which defines the module as a 32 bit module."]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MOD NUMBER. This bit field defines a module identification number. The value for the  lt Default   Font gt SRI Fabric is 0004H."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(315392)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pestat_SPEC;
impl crate::sealed::RegSpec for Pestat_SPEC {
    type DataType = u32;
}
#[doc = "Protocol Error Status Register\n resetvalue={Application Reset:0x0}"]
pub type Pestat = crate::RegValueT<Pestat_SPEC>;

impl Pestat {
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci0(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, pestat::Pesci0, Pestat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,pestat::Pesci0, Pestat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci1(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, pestat::Pesci1, Pestat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,pestat::Pesci1, Pestat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci2(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, pestat::Pesci2, Pestat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,pestat::Pesci2, Pestat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci3(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, pestat::Pesci3, Pestat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,pestat::Pesci3, Pestat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci4(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, pestat::Pesci4, Pestat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,pestat::Pesci4, Pestat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci5(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, pestat::Pesci5, Pestat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,pestat::Pesci5, Pestat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci6(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, pestat::Pesci6, Pestat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,pestat::Pesci6, Pestat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci7(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, pestat::Pesci7, Pestat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,pestat::Pesci7, Pestat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci8(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, pestat::Pesci8, Pestat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,pestat::Pesci8, Pestat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci9(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, pestat::Pesci9, Pestat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,pestat::Pesci9, Pestat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci10(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, pestat::Pesci10, Pestat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,pestat::Pesci10, Pestat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci11(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, pestat::Pesci11, Pestat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,pestat::Pesci11, Pestat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci12(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, pestat::Pesci12, Pestat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,pestat::Pesci12, Pestat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci13(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, pestat::Pesci13, Pestat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,pestat::Pesci13, Pestat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci14(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, pestat::Pesci14, Pestat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,pestat::Pesci14, Pestat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Protocol Error status of SCI15   PESCI15. Writing 0 to the bit leaves the content unchanged  while writing 1 to        the bit clears it. In case the bit is simultaneously cleared via        software and set via hardware  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn pesci15(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, pestat::Pesci15, Pestat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,pestat::Pesci15, Pestat_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pestat {
    #[inline(always)]
    fn default() -> Pestat {
        <crate::RegValueT<Pestat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pestat {
    pub struct Pesci0_SPEC;
    pub type Pesci0 = crate::EnumBitfieldStruct<u8, Pesci0_SPEC>;
    impl Pesci0 {
        #[doc = "0 No protocol error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A protocol error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pesci1_SPEC;
    pub type Pesci1 = crate::EnumBitfieldStruct<u8, Pesci1_SPEC>;
    impl Pesci1 {
        #[doc = "0 No protocol error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A protocol error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pesci2_SPEC;
    pub type Pesci2 = crate::EnumBitfieldStruct<u8, Pesci2_SPEC>;
    impl Pesci2 {
        #[doc = "0 No protocol error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A protocol error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pesci3_SPEC;
    pub type Pesci3 = crate::EnumBitfieldStruct<u8, Pesci3_SPEC>;
    impl Pesci3 {
        #[doc = "0 No protocol error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A protocol error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pesci4_SPEC;
    pub type Pesci4 = crate::EnumBitfieldStruct<u8, Pesci4_SPEC>;
    impl Pesci4 {
        #[doc = "0 No protocol error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A protocol error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pesci5_SPEC;
    pub type Pesci5 = crate::EnumBitfieldStruct<u8, Pesci5_SPEC>;
    impl Pesci5 {
        #[doc = "0 No protocol error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A protocol error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pesci6_SPEC;
    pub type Pesci6 = crate::EnumBitfieldStruct<u8, Pesci6_SPEC>;
    impl Pesci6 {
        #[doc = "0 No protocol error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A protocol error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pesci7_SPEC;
    pub type Pesci7 = crate::EnumBitfieldStruct<u8, Pesci7_SPEC>;
    impl Pesci7 {
        #[doc = "0 No protocol error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A protocol error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pesci8_SPEC;
    pub type Pesci8 = crate::EnumBitfieldStruct<u8, Pesci8_SPEC>;
    impl Pesci8 {
        #[doc = "0 No protocol error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A protocol error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pesci9_SPEC;
    pub type Pesci9 = crate::EnumBitfieldStruct<u8, Pesci9_SPEC>;
    impl Pesci9 {
        #[doc = "0 No protocol error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A protocol error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pesci10_SPEC;
    pub type Pesci10 = crate::EnumBitfieldStruct<u8, Pesci10_SPEC>;
    impl Pesci10 {
        #[doc = "0 No protocol error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A protocol error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pesci11_SPEC;
    pub type Pesci11 = crate::EnumBitfieldStruct<u8, Pesci11_SPEC>;
    impl Pesci11 {
        #[doc = "0 No protocol error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A protocol error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pesci12_SPEC;
    pub type Pesci12 = crate::EnumBitfieldStruct<u8, Pesci12_SPEC>;
    impl Pesci12 {
        #[doc = "0 No protocol error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A protocol error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pesci13_SPEC;
    pub type Pesci13 = crate::EnumBitfieldStruct<u8, Pesci13_SPEC>;
    impl Pesci13 {
        #[doc = "0 No protocol error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A protocol error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pesci14_SPEC;
    pub type Pesci14 = crate::EnumBitfieldStruct<u8, Pesci14_SPEC>;
    impl Pesci14 {
        #[doc = "0 No protocol error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A protocol error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pesci15_SPEC;
    pub type Pesci15 = crate::EnumBitfieldStruct<u8, Pesci15_SPEC>;
    impl Pesci15 {
        #[doc = "0 No protocol error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A protocol error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tiden_SPEC;
impl crate::sealed::RegSpec for Tiden_SPEC {
    type DataType = u32;
}
#[doc = "Transaction ID Enable Register\n resetvalue={Application Reset:0x0FFF3FFF}"]
pub type Tiden = crate::RegValueT<Tiden_SPEC>;

impl Tiden {
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, tiden::Ensci0, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,tiden::Ensci0, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, tiden::Ensci1, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,tiden::Ensci1, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, tiden::Ensci2, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,tiden::Ensci2, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, tiden::Ensci3, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,tiden::Ensci3, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, tiden::Ensci4, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,tiden::Ensci4, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, tiden::Ensci5, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,tiden::Ensci5, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, tiden::Ensci6, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,tiden::Ensci6, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, tiden::Ensci7, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,tiden::Ensci7, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, tiden::Ensci8, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,tiden::Ensci8, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, tiden::Ensci9, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,tiden::Ensci9, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, tiden::Ensci10, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,tiden::Ensci10, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, tiden::Ensci11, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,tiden::Ensci11, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, tiden::Ensci12, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,tiden::Ensci12, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, tiden::Ensci13, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,tiden::Ensci13, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, tiden::Ensci14, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,tiden::Ensci14, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E15able Transaction ID Error from SCIn   ENSCI15"]
    #[inline(always)]
    pub fn ensci15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, tiden::Ensci15, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,tiden::Ensci15, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci0(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, tiden::Enmci0, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,tiden::Enmci0, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci1(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, tiden::Enmci1, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,tiden::Enmci1, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci2(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, tiden::Enmci2, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,tiden::Enmci2, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci3(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, tiden::Enmci3, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,tiden::Enmci3, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci4(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, tiden::Enmci4, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,tiden::Enmci4, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci5(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, tiden::Enmci5, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,tiden::Enmci5, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci6(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, tiden::Enmci6, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,tiden::Enmci6, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci7(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, tiden::Enmci7, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,tiden::Enmci7, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci8(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, tiden::Enmci8, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,tiden::Enmci8, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci9(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, tiden::Enmci9, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,tiden::Enmci9, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci10(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, tiden::Enmci10, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,tiden::Enmci10, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "E11able Transaction ID Error from MCIn   ENMCI11"]
    #[inline(always)]
    pub fn enmci11(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, tiden::Enmci11, Tiden_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,tiden::Enmci11, Tiden_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Tiden {
    #[inline(always)]
    fn default() -> Tiden {
        <crate::RegValueT<Tiden_SPEC> as RegisterValue<_>>::new(268386303)
    }
}
pub mod tiden {
    pub struct Ensci0_SPEC;
    pub type Ensci0 = crate::EnumBitfieldStruct<u8, Ensci0_SPEC>;
    impl Ensci0 {
        #[doc = "0 Transaction ID errors from SCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID errors from SCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ensci1_SPEC;
    pub type Ensci1 = crate::EnumBitfieldStruct<u8, Ensci1_SPEC>;
    impl Ensci1 {
        #[doc = "0 Transaction ID errors from SCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID errors from SCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ensci2_SPEC;
    pub type Ensci2 = crate::EnumBitfieldStruct<u8, Ensci2_SPEC>;
    impl Ensci2 {
        #[doc = "0 Transaction ID errors from SCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID errors from SCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ensci3_SPEC;
    pub type Ensci3 = crate::EnumBitfieldStruct<u8, Ensci3_SPEC>;
    impl Ensci3 {
        #[doc = "0 Transaction ID errors from SCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID errors from SCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ensci4_SPEC;
    pub type Ensci4 = crate::EnumBitfieldStruct<u8, Ensci4_SPEC>;
    impl Ensci4 {
        #[doc = "0 Transaction ID errors from SCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID errors from SCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ensci5_SPEC;
    pub type Ensci5 = crate::EnumBitfieldStruct<u8, Ensci5_SPEC>;
    impl Ensci5 {
        #[doc = "0 Transaction ID errors from SCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID errors from SCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ensci6_SPEC;
    pub type Ensci6 = crate::EnumBitfieldStruct<u8, Ensci6_SPEC>;
    impl Ensci6 {
        #[doc = "0 Transaction ID errors from SCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID errors from SCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ensci7_SPEC;
    pub type Ensci7 = crate::EnumBitfieldStruct<u8, Ensci7_SPEC>;
    impl Ensci7 {
        #[doc = "0 Transaction ID errors from SCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID errors from SCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ensci8_SPEC;
    pub type Ensci8 = crate::EnumBitfieldStruct<u8, Ensci8_SPEC>;
    impl Ensci8 {
        #[doc = "0 Transaction ID errors from SCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID errors from SCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ensci9_SPEC;
    pub type Ensci9 = crate::EnumBitfieldStruct<u8, Ensci9_SPEC>;
    impl Ensci9 {
        #[doc = "0 Transaction ID errors from SCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID errors from SCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ensci10_SPEC;
    pub type Ensci10 = crate::EnumBitfieldStruct<u8, Ensci10_SPEC>;
    impl Ensci10 {
        #[doc = "0 Transaction ID errors from SCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID errors from SCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ensci11_SPEC;
    pub type Ensci11 = crate::EnumBitfieldStruct<u8, Ensci11_SPEC>;
    impl Ensci11 {
        #[doc = "0 Transaction ID errors from SCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID errors from SCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ensci12_SPEC;
    pub type Ensci12 = crate::EnumBitfieldStruct<u8, Ensci12_SPEC>;
    impl Ensci12 {
        #[doc = "0 Transaction ID errors from SCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID errors from SCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ensci13_SPEC;
    pub type Ensci13 = crate::EnumBitfieldStruct<u8, Ensci13_SPEC>;
    impl Ensci13 {
        #[doc = "0 Transaction ID errors from SCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID errors from SCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ensci14_SPEC;
    pub type Ensci14 = crate::EnumBitfieldStruct<u8, Ensci14_SPEC>;
    impl Ensci14 {
        #[doc = "0 Transaction ID errors from SCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID errors from SCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ensci15_SPEC;
    pub type Ensci15 = crate::EnumBitfieldStruct<u8, Ensci15_SPEC>;
    impl Ensci15 {
        #[doc = "0 Transaction ID errors from SCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID errors from SCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enmci0_SPEC;
    pub type Enmci0 = crate::EnumBitfieldStruct<u8, Enmci0_SPEC>;
    impl Enmci0 {
        #[doc = "0 Transaction ID        errors from MCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID        errors from MCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enmci1_SPEC;
    pub type Enmci1 = crate::EnumBitfieldStruct<u8, Enmci1_SPEC>;
    impl Enmci1 {
        #[doc = "0 Transaction ID        errors from MCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID        errors from MCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enmci2_SPEC;
    pub type Enmci2 = crate::EnumBitfieldStruct<u8, Enmci2_SPEC>;
    impl Enmci2 {
        #[doc = "0 Transaction ID        errors from MCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID        errors from MCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enmci3_SPEC;
    pub type Enmci3 = crate::EnumBitfieldStruct<u8, Enmci3_SPEC>;
    impl Enmci3 {
        #[doc = "0 Transaction ID        errors from MCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID        errors from MCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enmci4_SPEC;
    pub type Enmci4 = crate::EnumBitfieldStruct<u8, Enmci4_SPEC>;
    impl Enmci4 {
        #[doc = "0 Transaction ID        errors from MCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID        errors from MCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enmci5_SPEC;
    pub type Enmci5 = crate::EnumBitfieldStruct<u8, Enmci5_SPEC>;
    impl Enmci5 {
        #[doc = "0 Transaction ID        errors from MCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID        errors from MCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enmci6_SPEC;
    pub type Enmci6 = crate::EnumBitfieldStruct<u8, Enmci6_SPEC>;
    impl Enmci6 {
        #[doc = "0 Transaction ID        errors from MCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID        errors from MCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enmci7_SPEC;
    pub type Enmci7 = crate::EnumBitfieldStruct<u8, Enmci7_SPEC>;
    impl Enmci7 {
        #[doc = "0 Transaction ID        errors from MCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID        errors from MCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enmci8_SPEC;
    pub type Enmci8 = crate::EnumBitfieldStruct<u8, Enmci8_SPEC>;
    impl Enmci8 {
        #[doc = "0 Transaction ID        errors from MCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID        errors from MCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enmci9_SPEC;
    pub type Enmci9 = crate::EnumBitfieldStruct<u8, Enmci9_SPEC>;
    impl Enmci9 {
        #[doc = "0 Transaction ID        errors from MCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID        errors from MCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enmci10_SPEC;
    pub type Enmci10 = crate::EnumBitfieldStruct<u8, Enmci10_SPEC>;
    impl Enmci10 {
        #[doc = "0 Transaction ID        errors from MCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID        errors from MCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enmci11_SPEC;
    pub type Enmci11 = crate::EnumBitfieldStruct<u8, Enmci11_SPEC>;
    impl Enmci11 {
        #[doc = "0 Transaction ID        errors from MCIn are not indicated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transaction ID        errors from MCIn are indicated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tidstat_SPEC;
impl crate::sealed::RegSpec for Tidstat_SPEC {
    type DataType = u32;
}
#[doc = "Transaction ID Status Register\n resetvalue={Application Reset:0x0}"]
pub type Tidstat = crate::RegValueT<Tidstat_SPEC>;

impl Tidstat {
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, tidstat::Tidsci0, Tidstat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            tidstat::Tidsci0,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, tidstat::Tidsci1, Tidstat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            tidstat::Tidsci1,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, tidstat::Tidsci2, Tidstat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            tidstat::Tidsci2,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, tidstat::Tidsci3, Tidstat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            tidstat::Tidsci3,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, tidstat::Tidsci4, Tidstat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            tidstat::Tidsci4,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, tidstat::Tidsci5, Tidstat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            tidstat::Tidsci5,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, tidstat::Tidsci6, Tidstat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            tidstat::Tidsci6,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, tidstat::Tidsci7, Tidstat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            tidstat::Tidsci7,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, tidstat::Tidsci8, Tidstat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            tidstat::Tidsci8,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, tidstat::Tidsci9, Tidstat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            tidstat::Tidsci9,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci10(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x1,
        1,
        0,
        tidstat::Tidsci10,
        Tidstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            tidstat::Tidsci10,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci11(
        self,
    ) -> crate::common::RegisterField<
        11,
        0x1,
        1,
        0,
        tidstat::Tidsci11,
        Tidstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            11,
            0x1,
            1,
            0,
            tidstat::Tidsci11,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci12(
        self,
    ) -> crate::common::RegisterField<
        12,
        0x1,
        1,
        0,
        tidstat::Tidsci12,
        Tidstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            12,
            0x1,
            1,
            0,
            tidstat::Tidsci12,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci13(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        tidstat::Tidsci13,
        Tidstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            tidstat::Tidsci13,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci14(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        tidstat::Tidsci14,
        Tidstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            tidstat::Tidsci14,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra15saction ID Error from SCIn Status   TIDSCI15. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidsci15(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        tidstat::Tidsci15,
        Tidstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            tidstat::Tidsci15,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci0(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        tidstat::Tidmci0,
        Tidstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            tidstat::Tidmci0,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci1(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        tidstat::Tidmci1,
        Tidstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            tidstat::Tidmci1,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci2(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        tidstat::Tidmci2,
        Tidstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            tidstat::Tidmci2,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci3(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        tidstat::Tidmci3,
        Tidstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            tidstat::Tidmci3,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci4(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        tidstat::Tidmci4,
        Tidstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            tidstat::Tidmci4,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci5(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        tidstat::Tidmci5,
        Tidstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            tidstat::Tidmci5,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci6(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        tidstat::Tidmci6,
        Tidstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            tidstat::Tidmci6,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci7(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        tidstat::Tidmci7,
        Tidstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            tidstat::Tidmci7,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci8(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        tidstat::Tidmci8,
        Tidstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            tidstat::Tidmci8,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci9(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        tidstat::Tidmci9,
        Tidstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            tidstat::Tidmci9,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci10(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        tidstat::Tidmci10,
        Tidstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            tidstat::Tidmci10,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Tra11saction ID Error from MCIn Status   TIDMCI11. Writing 0 to the bit leaves the content unchanged. Writing 1 to the bit clears it. If the bit is simultaneously cleared via software and set due to a          hardware error  the bit remains set and is not cleared."]
    #[inline(always)]
    pub fn tidmci11(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        tidstat::Tidmci11,
        Tidstat_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            tidstat::Tidmci11,
            Tidstat_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Tidstat {
    #[inline(always)]
    fn default() -> Tidstat {
        <crate::RegValueT<Tidstat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod tidstat {
    pub struct Tidsci0_SPEC;
    pub type Tidsci0 = crate::EnumBitfieldStruct<u8, Tidsci0_SPEC>;
    impl Tidsci0 {
        #[doc = "0 No transaction ID error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction ID error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidsci1_SPEC;
    pub type Tidsci1 = crate::EnumBitfieldStruct<u8, Tidsci1_SPEC>;
    impl Tidsci1 {
        #[doc = "0 No transaction ID error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction ID error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidsci2_SPEC;
    pub type Tidsci2 = crate::EnumBitfieldStruct<u8, Tidsci2_SPEC>;
    impl Tidsci2 {
        #[doc = "0 No transaction ID error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction ID error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidsci3_SPEC;
    pub type Tidsci3 = crate::EnumBitfieldStruct<u8, Tidsci3_SPEC>;
    impl Tidsci3 {
        #[doc = "0 No transaction ID error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction ID error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidsci4_SPEC;
    pub type Tidsci4 = crate::EnumBitfieldStruct<u8, Tidsci4_SPEC>;
    impl Tidsci4 {
        #[doc = "0 No transaction ID error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction ID error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidsci5_SPEC;
    pub type Tidsci5 = crate::EnumBitfieldStruct<u8, Tidsci5_SPEC>;
    impl Tidsci5 {
        #[doc = "0 No transaction ID error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction ID error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidsci6_SPEC;
    pub type Tidsci6 = crate::EnumBitfieldStruct<u8, Tidsci6_SPEC>;
    impl Tidsci6 {
        #[doc = "0 No transaction ID error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction ID error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidsci7_SPEC;
    pub type Tidsci7 = crate::EnumBitfieldStruct<u8, Tidsci7_SPEC>;
    impl Tidsci7 {
        #[doc = "0 No transaction ID error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction ID error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidsci8_SPEC;
    pub type Tidsci8 = crate::EnumBitfieldStruct<u8, Tidsci8_SPEC>;
    impl Tidsci8 {
        #[doc = "0 No transaction ID error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction ID error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidsci9_SPEC;
    pub type Tidsci9 = crate::EnumBitfieldStruct<u8, Tidsci9_SPEC>;
    impl Tidsci9 {
        #[doc = "0 No transaction ID error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction ID error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidsci10_SPEC;
    pub type Tidsci10 = crate::EnumBitfieldStruct<u8, Tidsci10_SPEC>;
    impl Tidsci10 {
        #[doc = "0 No transaction ID error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction ID error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidsci11_SPEC;
    pub type Tidsci11 = crate::EnumBitfieldStruct<u8, Tidsci11_SPEC>;
    impl Tidsci11 {
        #[doc = "0 No transaction ID error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction ID error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidsci12_SPEC;
    pub type Tidsci12 = crate::EnumBitfieldStruct<u8, Tidsci12_SPEC>;
    impl Tidsci12 {
        #[doc = "0 No transaction ID error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction ID error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidsci13_SPEC;
    pub type Tidsci13 = crate::EnumBitfieldStruct<u8, Tidsci13_SPEC>;
    impl Tidsci13 {
        #[doc = "0 No transaction ID error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction ID error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidsci14_SPEC;
    pub type Tidsci14 = crate::EnumBitfieldStruct<u8, Tidsci14_SPEC>;
    impl Tidsci14 {
        #[doc = "0 No transaction ID error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction ID error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidsci15_SPEC;
    pub type Tidsci15 = crate::EnumBitfieldStruct<u8, Tidsci15_SPEC>;
    impl Tidsci15 {
        #[doc = "0 No transaction ID error has been indicated by SCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction ID error has been indicated by SCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidmci0_SPEC;
    pub type Tidmci0 = crate::EnumBitfieldStruct<u8, Tidmci0_SPEC>;
    impl Tidmci0 {
        #[doc = "0 No transaction        ID error has been indicated by MCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction        ID error has been indicated by MCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidmci1_SPEC;
    pub type Tidmci1 = crate::EnumBitfieldStruct<u8, Tidmci1_SPEC>;
    impl Tidmci1 {
        #[doc = "0 No transaction        ID error has been indicated by MCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction        ID error has been indicated by MCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidmci2_SPEC;
    pub type Tidmci2 = crate::EnumBitfieldStruct<u8, Tidmci2_SPEC>;
    impl Tidmci2 {
        #[doc = "0 No transaction        ID error has been indicated by MCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction        ID error has been indicated by MCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidmci3_SPEC;
    pub type Tidmci3 = crate::EnumBitfieldStruct<u8, Tidmci3_SPEC>;
    impl Tidmci3 {
        #[doc = "0 No transaction        ID error has been indicated by MCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction        ID error has been indicated by MCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidmci4_SPEC;
    pub type Tidmci4 = crate::EnumBitfieldStruct<u8, Tidmci4_SPEC>;
    impl Tidmci4 {
        #[doc = "0 No transaction        ID error has been indicated by MCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction        ID error has been indicated by MCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidmci5_SPEC;
    pub type Tidmci5 = crate::EnumBitfieldStruct<u8, Tidmci5_SPEC>;
    impl Tidmci5 {
        #[doc = "0 No transaction        ID error has been indicated by MCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction        ID error has been indicated by MCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidmci6_SPEC;
    pub type Tidmci6 = crate::EnumBitfieldStruct<u8, Tidmci6_SPEC>;
    impl Tidmci6 {
        #[doc = "0 No transaction        ID error has been indicated by MCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction        ID error has been indicated by MCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidmci7_SPEC;
    pub type Tidmci7 = crate::EnumBitfieldStruct<u8, Tidmci7_SPEC>;
    impl Tidmci7 {
        #[doc = "0 No transaction        ID error has been indicated by MCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction        ID error has been indicated by MCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidmci8_SPEC;
    pub type Tidmci8 = crate::EnumBitfieldStruct<u8, Tidmci8_SPEC>;
    impl Tidmci8 {
        #[doc = "0 No transaction        ID error has been indicated by MCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction        ID error has been indicated by MCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidmci9_SPEC;
    pub type Tidmci9 = crate::EnumBitfieldStruct<u8, Tidmci9_SPEC>;
    impl Tidmci9 {
        #[doc = "0 No transaction        ID error has been indicated by MCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction        ID error has been indicated by MCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidmci10_SPEC;
    pub type Tidmci10 = crate::EnumBitfieldStruct<u8, Tidmci10_SPEC>;
    impl Tidmci10 {
        #[doc = "0 No transaction        ID error has been indicated by MCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction        ID error has been indicated by MCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tidmci11_SPEC;
    pub type Tidmci11 = crate::EnumBitfieldStruct<u8, Tidmci11_SPEC>;
    impl Tidmci11 {
        #[doc = "0 No transaction        ID error has been indicated by MCIn"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A transaction        ID error has been indicated by MCIn"]
        pub const CONST_11: Self = Self::new(1);
    }
}

#[doc = "SCICTRL"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scictrl(pub(super) *mut u8);
unsafe impl core::marker::Send for Scictrl {}
unsafe impl core::marker::Sync for Scictrl {}
impl Scictrl {
    #[doc = "SCI 0 Error Address Capture Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn erraddrx(&self) -> crate::common::Reg<scictrl::ErraddRx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "SCI 0 Error Capture Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn errx(&self) -> crate::common::Reg<scictrl::ErRx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Protocol Error Control Register 0\n resetvalue={Application Reset:0x1}"]
    #[inline(always)]
    pub const fn peconx(&self) -> crate::common::Reg<scictrl::PecoNx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "SCI0 Arbiter Priority Register\n resetvalue={Application Reset:0x70000}"]
    #[inline(always)]
    pub const fn priorityx(
        &self,
    ) -> crate::common::Reg<scictrl::PrioritYx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
pub mod scictrl {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ErraddRx_SPEC;
    impl crate::sealed::RegSpec for ErraddRx_SPEC {
        type DataType = u32;
    }
    #[doc = "SCI 0 Error Address Capture Register\n resetvalue={Application Reset:0x0}"]
    pub type ErraddRx = crate::RegValueT<ErraddRx_SPEC>;

    impl ErraddRx {
        #[doc = "Transaction Address   ADDR. This bitfield contains the address of the erroneous transaction from the address phase"]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, ErraddRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, ErraddRx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for ErraddRx {
        #[inline(always)]
        fn default() -> ErraddRx {
            <crate::RegValueT<ErraddRx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ErRx_SPEC;
    impl crate::sealed::RegSpec for ErRx_SPEC {
        type DataType = u32;
    }
    #[doc = "SCI 0 Error Capture Register\n resetvalue={Application Reset:0x0}"]
    pub type ErRx = crate::RegValueT<ErRx_SPEC>;

    impl ErRx {
        #[doc = "Read Status   RD"]
        #[inline(always)]
        pub fn rd_n(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, errx::RdN, ErRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0x1,1,0,errx::RdN, ErRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Write Status   WR"]
        #[inline(always)]
        pub fn wr_n(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, errx::WrN, ErRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<1,0x1,1,0,errx::WrN, ErRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Supervisor Mode Status   SVM"]
        #[inline(always)]
        pub fn svm(
            self,
        ) -> crate::common::RegisterField<2, 0x1, 1, 0, errx::Svm, ErRx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<2,0x1,1,0,errx::Svm, ErRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Operation Code   OPC. This field contains the opcode of the erroneous transaction  see CROSSREFERENCE for details."]
        #[inline(always)]
        pub fn opc(
            self,
        ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, ErRx_SPEC, crate::common::R> {
            crate::common::RegisterField::<4,0xf,1,0,u8, ErRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Transaction ID   TR ID. This field contains the Master  8217 s transaction ID of the erroneous        transaction. The Transaction ID is built out of the Master  8217 s 6 bit        unique TAG ID  TR ID 5 0    and a 2 bit running number TR ID 7 6 ."]
        #[inline(always)]
        pub fn tr_id(
            self,
        ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, ErRx_SPEC, crate::common::R> {
            crate::common::RegisterField::<8,0xff,1,0,u8, ErRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Address Phase Error Detection Information   ADDR EDC. This field contains the Address Phase Error Detection Information of the        erroneous transaction."]
        #[inline(always)]
        pub fn addr_edc(
            self,
        ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, ErRx_SPEC, crate::common::R> {
            crate::common::RegisterField::<16,0xff,1,0,u8, ErRx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "MCI Sideband Signals  7 0    MCI SBS. This bit field contains the MCI Sideband Signals  7 0  that are related        to the transaction information captured. In the AURIX  8482  TC3xx family  the sideband signals are used by the DMA to provide information        about the DMA requestor of a DMA transaction  for the encoding see CROSSREFERENCE  ."]
        #[inline(always)]
        pub fn mci_sbs(
            self,
        ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, ErRx_SPEC, crate::common::R> {
            crate::common::RegisterField::<24,0xff,1,0,u8, ErRx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for ErRx {
        #[inline(always)]
        fn default() -> ErRx {
            <crate::RegValueT<ErRx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod errx {
        pub struct RdN_SPEC;
        pub type RdN = crate::EnumBitfieldStruct<u8, RdN_SPEC>;
        impl RdN {
            #[doc = "0 The read signal line was asserted  read or start of RMW transaction"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 The read line was deasserted  no read transaction"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct WrN_SPEC;
        pub type WrN = crate::EnumBitfieldStruct<u8, WrN_SPEC>;
        impl WrN {
            #[doc = "0 The write signal line was asserted  write or start of RMW transaction"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 The write signal line was deasserted  no write transaction"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Svm_SPEC;
        pub type Svm = crate::EnumBitfieldStruct<u8, Svm_SPEC>;
        impl Svm {
            #[doc = "0 The supervisor mode signal line was deasserted"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 The supervisor mode signal line was asserted"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PecoNx_SPEC;
    impl crate::sealed::RegSpec for PecoNx_SPEC {
        type DataType = u32;
    }
    #[doc = "Protocol Error Control Register 0\n resetvalue={Application Reset:0x1}"]
    pub type PecoNx = crate::RegValueT<PecoNx_SPEC>;

    impl PecoNx {
        #[doc = "Protocol Error Enable   PEEN"]
        #[inline(always)]
        pub fn peen(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, peconx::Peen, PecoNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1,1,0,peconx::Peen, PecoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Set Protocol Error   SETPE. This allows SW to mimic a protocol error and present an indication        similar to a hardware detected error. After setting this bit  it is        automatically cleared by the hardware in the cycle after the write."]
        #[inline(always)]
        pub fn setpe(
            self,
        ) -> crate::common::RegisterField<2, 0x1, 1, 0, peconx::Setpe, PecoNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                peconx::Setpe,
                PecoNx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Protocol Error Acknowledge   PEACK. Writing a one to this bit while it  x2019 s set has the following results  The error lock of the corresponding ERRADDRx and ERRx are released  and the registers will be updated for the next protocol error detected  see CROSSREFERENCE  . After setting this bit  it is automatically cleared by the hardware in the cycle after the write."]
        #[inline(always)]
        pub fn peack(
            self,
        ) -> crate::common::RegisterField<4, 0x1, 1, 0, peconx::Peack, PecoNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                peconx::Peack,
                PecoNx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for PecoNx {
        #[inline(always)]
        fn default() -> PecoNx {
            <crate::RegValueT<PecoNx_SPEC> as RegisterValue<_>>::new(1)
        }
    }
    pub mod peconx {
        pub struct Peen_SPEC;
        pub type Peen = crate::EnumBitfieldStruct<u8, Peen_SPEC>;
        impl Peen {
            #[doc = "0 Protocol errors are not indicated and no information is captured."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Protocol errors are indicated and information is captured."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Setpe_SPEC;
        pub type Setpe = crate::EnumBitfieldStruct<u8, Setpe_SPEC>;
        impl Setpe {
            #[doc = "0 No protocol error indication is generated"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 A protocol error indication is generated"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Peack_SPEC;
        pub type Peack = crate::EnumBitfieldStruct<u8, Peack_SPEC>;
        impl Peack {
            #[doc = "0 Default value"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 A Protocol Error for this arbiter has been indicated. The corresponding ERRADDR and ERR registers are not updated for new errors."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PrioritYx_SPEC;
    impl crate::sealed::RegSpec for PrioritYx_SPEC {
        type DataType = u32;
    }
    #[doc = "SCI0 Arbiter Priority Register\n resetvalue={Application Reset:0x70000}"]
    pub type PrioritYx = crate::RegValueT<PrioritYx_SPEC>;

    impl PrioritYx {
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci0_p(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            priorityx::Mci0P,
            PrioritYx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                priorityx::Mci0P,
                PrioritYx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci1_p(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            priorityx::Mci1P,
            PrioritYx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                priorityx::Mci1P,
                PrioritYx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci2_p(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            priorityx::Mci2P,
            PrioritYx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                priorityx::Mci2P,
                PrioritYx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci3_p(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            priorityx::Mci3P,
            PrioritYx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                priorityx::Mci3P,
                PrioritYx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci4_p(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            priorityx::Mci4P,
            PrioritYx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                priorityx::Mci4P,
                PrioritYx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci5_p(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            priorityx::Mci5P,
            PrioritYx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                priorityx::Mci5P,
                PrioritYx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci6_p(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x1,
            1,
            0,
            priorityx::Mci6P,
            PrioritYx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                6,
                0x1,
                1,
                0,
                priorityx::Mci6P,
                PrioritYx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci7_p(
            self,
        ) -> crate::common::RegisterField<
            7,
            0x1,
            1,
            0,
            priorityx::Mci7P,
            PrioritYx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                priorityx::Mci7P,
                PrioritYx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci8_p(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1,
            1,
            0,
            priorityx::Mci8P,
            PrioritYx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                priorityx::Mci8P,
                PrioritYx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci9_p(
            self,
        ) -> crate::common::RegisterField<
            9,
            0x1,
            1,
            0,
            priorityx::Mci9P,
            PrioritYx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                9,
                0x1,
                1,
                0,
                priorityx::Mci9P,
                PrioritYx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci10_p(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            priorityx::Mci10P,
            PrioritYx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                priorityx::Mci10P,
                PrioritYx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "MCI11 Priority   MCI11 P"]
        #[inline(always)]
        pub fn mci11_p(
            self,
        ) -> crate::common::RegisterField<
            11,
            0x1,
            1,
            0,
            priorityx::Mci11P,
            PrioritYx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                priorityx::Mci11P,
                PrioritYx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "High Priority Round Share   HPRS. Number of transactions to give to the high priority round robin before a        transaction from low priority round  when request saturated . This        number may not be less than the number of high priority MCI programmed        via. MCIn P."]
        #[inline(always)]
        pub fn hprs(
            self,
        ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, PrioritYx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x7,1,0,u8, PrioritYx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for PrioritYx {
        #[inline(always)]
        fn default() -> PrioritYx {
            <crate::RegValueT<PrioritYx_SPEC> as RegisterValue<_>>::new(458752)
        }
    }
    pub mod priorityx {
        pub struct Mci0P_SPEC;
        pub type Mci0P = crate::EnumBitfieldStruct<u8, Mci0P_SPEC>;
        impl Mci0P {
            #[doc = "0 MCIn requests        are arbitrated in the low priority round robin."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 MCIn requests        are arbitrated in the high priority round robin."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Mci1P_SPEC;
        pub type Mci1P = crate::EnumBitfieldStruct<u8, Mci1P_SPEC>;
        impl Mci1P {
            #[doc = "0 MCIn requests        are arbitrated in the low priority round robin."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 MCIn requests        are arbitrated in the high priority round robin."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Mci2P_SPEC;
        pub type Mci2P = crate::EnumBitfieldStruct<u8, Mci2P_SPEC>;
        impl Mci2P {
            #[doc = "0 MCIn requests        are arbitrated in the low priority round robin."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 MCIn requests        are arbitrated in the high priority round robin."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Mci3P_SPEC;
        pub type Mci3P = crate::EnumBitfieldStruct<u8, Mci3P_SPEC>;
        impl Mci3P {
            #[doc = "0 MCIn requests        are arbitrated in the low priority round robin."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 MCIn requests        are arbitrated in the high priority round robin."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Mci4P_SPEC;
        pub type Mci4P = crate::EnumBitfieldStruct<u8, Mci4P_SPEC>;
        impl Mci4P {
            #[doc = "0 MCIn requests        are arbitrated in the low priority round robin."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 MCIn requests        are arbitrated in the high priority round robin."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Mci5P_SPEC;
        pub type Mci5P = crate::EnumBitfieldStruct<u8, Mci5P_SPEC>;
        impl Mci5P {
            #[doc = "0 MCIn requests        are arbitrated in the low priority round robin."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 MCIn requests        are arbitrated in the high priority round robin."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Mci6P_SPEC;
        pub type Mci6P = crate::EnumBitfieldStruct<u8, Mci6P_SPEC>;
        impl Mci6P {
            #[doc = "0 MCIn requests        are arbitrated in the low priority round robin."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 MCIn requests        are arbitrated in the high priority round robin."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Mci7P_SPEC;
        pub type Mci7P = crate::EnumBitfieldStruct<u8, Mci7P_SPEC>;
        impl Mci7P {
            #[doc = "0 MCIn requests        are arbitrated in the low priority round robin."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 MCIn requests        are arbitrated in the high priority round robin."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Mci8P_SPEC;
        pub type Mci8P = crate::EnumBitfieldStruct<u8, Mci8P_SPEC>;
        impl Mci8P {
            #[doc = "0 MCIn requests        are arbitrated in the low priority round robin."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 MCIn requests        are arbitrated in the high priority round robin."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Mci9P_SPEC;
        pub type Mci9P = crate::EnumBitfieldStruct<u8, Mci9P_SPEC>;
        impl Mci9P {
            #[doc = "0 MCIn requests        are arbitrated in the low priority round robin."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 MCIn requests        are arbitrated in the high priority round robin."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Mci10P_SPEC;
        pub type Mci10P = crate::EnumBitfieldStruct<u8, Mci10P_SPEC>;
        impl Mci10P {
            #[doc = "0 MCIn requests        are arbitrated in the low priority round robin."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 MCIn requests        are arbitrated in the high priority round robin."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Mci11P_SPEC;
        pub type Mci11P = crate::EnumBitfieldStruct<u8, Mci11P_SPEC>;
        impl Mci11P {
            #[doc = "0 MCIn requests        are arbitrated in the low priority round robin."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 MCIn requests        are arbitrated in the high priority round robin."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
}
