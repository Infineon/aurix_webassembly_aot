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
#[doc = r"LMU DAM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dam0(pub(super) *mut u8);
unsafe impl core::marker::Send for Dam0 {}
unsafe impl core::marker::Sync for Dam0 {}
impl Dam0 {
    #[doc = "DAM Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "DAM Access Enable Register 1\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen1(&self) -> crate::common::Reg<self::Accen1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "DAM Clock Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "DAM Memory Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn memcon(&self) -> crate::common::Reg<self::Memcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "DAM Module ID Register\n resetvalue={Application Reset:0x088C003}"]
    #[inline(always)]
    pub const fn modid(&self) -> crate::common::Reg<self::Modid_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "RGNACCEN"]
    #[inline(always)]
    pub fn rgnaccen(self) -> [self::Rgnaccen; 8] {
        unsafe {
            [
                self::Rgnaccen(self.0.add(0xd8usize + 0x0usize)),
                self::Rgnaccen(self.0.add(0xd8usize + 0x10usize)),
                self::Rgnaccen(self.0.add(0xd8usize + 0x20usize)),
                self::Rgnaccen(self.0.add(0xd8usize + 0x30usize)),
                self::Rgnaccen(self.0.add(0xd8usize + 0x40usize)),
                self::Rgnaccen(self.0.add(0xd8usize + 0x50usize)),
                self::Rgnaccen(self.0.add(0xd8usize + 0x60usize)),
                self::Rgnaccen(self.0.add(0xd8usize + 0x70usize)),
            ]
        }
    }
    #[doc = "RGN"]
    #[inline(always)]
    pub fn rgn(self) -> [self::Rgn; 8] {
        unsafe {
            [
                self::Rgn(self.0.add(0x50usize + 0x0usize)),
                self::Rgn(self.0.add(0x50usize + 0x10usize)),
                self::Rgn(self.0.add(0x50usize + 0x20usize)),
                self::Rgn(self.0.add(0x50usize + 0x30usize)),
                self::Rgn(self.0.add(0x50usize + 0x40usize)),
                self::Rgn(self.0.add(0x50usize + 0x50usize)),
                self::Rgn(self.0.add(0x50usize + 0x60usize)),
                self::Rgn(self.0.add(0x50usize + 0x70usize)),
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
#[doc = "DAM Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Accen0 = crate::RegValueT<Accen0_SPEC>;

impl Accen0 {
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, accen0::En0, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,accen0::En0, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, accen0::En1, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,accen0::En1, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, accen0::En2, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,accen0::En2, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, accen0::En3, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,accen0::En3, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, accen0::En4, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,accen0::En4, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, accen0::En5, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,accen0::En5, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, accen0::En6, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,accen0::En6, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, accen0::En7, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,accen0::En7, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, accen0::En8, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,accen0::En8, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, accen0::En9, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,accen0::En9, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, accen0::En10, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,accen0::En10, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, accen0::En11, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,accen0::En11, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, accen0::En12, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,accen0::En12, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, accen0::En13, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,accen0::En13, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, accen0::En14, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,accen0::En14, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, accen0::En15, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,accen0::En15, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, accen0::En16, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,accen0::En16, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, accen0::En17, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,accen0::En17, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, accen0::En18, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,accen0::En18, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, accen0::En19, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,accen0::En19, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, accen0::En20, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,accen0::En20, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, accen0::En21, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,accen0::En21, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, accen0::En22, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,accen0::En22, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, accen0::En23, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,accen0::En23, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, accen0::En24, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,accen0::En24, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, accen0::En25, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,accen0::En25, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, accen0::En26, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,accen0::En26, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, accen0::En27, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,accen0::En27, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, accen0::En28, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,accen0::En28, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, accen0::En29, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,accen0::En29, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, accen0::En30, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,accen0::En30, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
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
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En1_SPEC;
    pub type En1 = crate::EnumBitfieldStruct<u8, En1_SPEC>;
    impl En1 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En2_SPEC;
    pub type En2 = crate::EnumBitfieldStruct<u8, En2_SPEC>;
    impl En2 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En3_SPEC;
    pub type En3 = crate::EnumBitfieldStruct<u8, En3_SPEC>;
    impl En3 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En4_SPEC;
    pub type En4 = crate::EnumBitfieldStruct<u8, En4_SPEC>;
    impl En4 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En5_SPEC;
    pub type En5 = crate::EnumBitfieldStruct<u8, En5_SPEC>;
    impl En5 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En6_SPEC;
    pub type En6 = crate::EnumBitfieldStruct<u8, En6_SPEC>;
    impl En6 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En7_SPEC;
    pub type En7 = crate::EnumBitfieldStruct<u8, En7_SPEC>;
    impl En7 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En8_SPEC;
    pub type En8 = crate::EnumBitfieldStruct<u8, En8_SPEC>;
    impl En8 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En9_SPEC;
    pub type En9 = crate::EnumBitfieldStruct<u8, En9_SPEC>;
    impl En9 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En10_SPEC;
    pub type En10 = crate::EnumBitfieldStruct<u8, En10_SPEC>;
    impl En10 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En11_SPEC;
    pub type En11 = crate::EnumBitfieldStruct<u8, En11_SPEC>;
    impl En11 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En12_SPEC;
    pub type En12 = crate::EnumBitfieldStruct<u8, En12_SPEC>;
    impl En12 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En13_SPEC;
    pub type En13 = crate::EnumBitfieldStruct<u8, En13_SPEC>;
    impl En13 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En14_SPEC;
    pub type En14 = crate::EnumBitfieldStruct<u8, En14_SPEC>;
    impl En14 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En15_SPEC;
    pub type En15 = crate::EnumBitfieldStruct<u8, En15_SPEC>;
    impl En15 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En16_SPEC;
    pub type En16 = crate::EnumBitfieldStruct<u8, En16_SPEC>;
    impl En16 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En17_SPEC;
    pub type En17 = crate::EnumBitfieldStruct<u8, En17_SPEC>;
    impl En17 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En18_SPEC;
    pub type En18 = crate::EnumBitfieldStruct<u8, En18_SPEC>;
    impl En18 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En19_SPEC;
    pub type En19 = crate::EnumBitfieldStruct<u8, En19_SPEC>;
    impl En19 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En20_SPEC;
    pub type En20 = crate::EnumBitfieldStruct<u8, En20_SPEC>;
    impl En20 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En21_SPEC;
    pub type En21 = crate::EnumBitfieldStruct<u8, En21_SPEC>;
    impl En21 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En22_SPEC;
    pub type En22 = crate::EnumBitfieldStruct<u8, En22_SPEC>;
    impl En22 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En23_SPEC;
    pub type En23 = crate::EnumBitfieldStruct<u8, En23_SPEC>;
    impl En23 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En24_SPEC;
    pub type En24 = crate::EnumBitfieldStruct<u8, En24_SPEC>;
    impl En24 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En25_SPEC;
    pub type En25 = crate::EnumBitfieldStruct<u8, En25_SPEC>;
    impl En25 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En26_SPEC;
    pub type En26 = crate::EnumBitfieldStruct<u8, En26_SPEC>;
    impl En26 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En27_SPEC;
    pub type En27 = crate::EnumBitfieldStruct<u8, En27_SPEC>;
    impl En27 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En28_SPEC;
    pub type En28 = crate::EnumBitfieldStruct<u8, En28_SPEC>;
    impl En28 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En29_SPEC;
    pub type En29 = crate::EnumBitfieldStruct<u8, En29_SPEC>;
    impl En29 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En30_SPEC;
    pub type En30 = crate::EnumBitfieldStruct<u8, En30_SPEC>;
    impl En30 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En31_SPEC;
    pub type En31 = crate::EnumBitfieldStruct<u8, En31_SPEC>;
    impl En31 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accen1_SPEC;
impl crate::sealed::RegSpec for Accen1_SPEC {
    type DataType = u32;
}
#[doc = "DAM Access Enable Register 1\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Accen1 = crate::RegValueT<Accen1_SPEC>;

impl Accen1 {
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en32(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, accen1::En32, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,accen1::En32, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en33(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, accen1::En33, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,accen1::En33, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en34(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, accen1::En34, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,accen1::En34, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en35(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, accen1::En35, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,accen1::En35, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en36(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, accen1::En36, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,accen1::En36, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en37(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, accen1::En37, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,accen1::En37, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en38(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, accen1::En38, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,accen1::En38, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en39(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, accen1::En39, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,accen1::En39, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en40(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, accen1::En40, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,accen1::En40, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en41(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, accen1::En41, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,accen1::En41, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en42(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, accen1::En42, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,accen1::En42, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en43(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, accen1::En43, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,accen1::En43, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en44(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, accen1::En44, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,accen1::En44, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en45(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, accen1::En45, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,accen1::En45, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en46(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, accen1::En46, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,accen1::En46, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en47(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, accen1::En47, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,accen1::En47, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en48(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, accen1::En48, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,accen1::En48, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en49(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, accen1::En49, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,accen1::En49, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en50(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, accen1::En50, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,accen1::En50, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en51(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, accen1::En51, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,accen1::En51, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en52(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, accen1::En52, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,accen1::En52, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en53(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, accen1::En53, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,accen1::En53, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en54(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, accen1::En54, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,accen1::En54, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en55(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, accen1::En55, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,accen1::En55, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en56(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, accen1::En56, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,accen1::En56, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en57(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, accen1::En57, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,accen1::En57, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en58(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, accen1::En58, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,accen1::En58, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en59(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, accen1::En59, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,accen1::En59, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en60(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, accen1::En60, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,accen1::En60, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en61(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, accen1::En61, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,accen1::En61, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en62(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, accen1::En62, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,accen1::En62, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the DAM register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en63(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, accen1::En63, Accen1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,accen1::En63, Accen1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Accen1 {
    #[inline(always)]
    fn default() -> Accen1 {
        <crate::RegValueT<Accen1_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod accen1 {
    pub struct En32_SPEC;
    pub type En32 = crate::EnumBitfieldStruct<u8, En32_SPEC>;
    impl En32 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En33_SPEC;
    pub type En33 = crate::EnumBitfieldStruct<u8, En33_SPEC>;
    impl En33 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En34_SPEC;
    pub type En34 = crate::EnumBitfieldStruct<u8, En34_SPEC>;
    impl En34 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En35_SPEC;
    pub type En35 = crate::EnumBitfieldStruct<u8, En35_SPEC>;
    impl En35 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En36_SPEC;
    pub type En36 = crate::EnumBitfieldStruct<u8, En36_SPEC>;
    impl En36 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En37_SPEC;
    pub type En37 = crate::EnumBitfieldStruct<u8, En37_SPEC>;
    impl En37 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En38_SPEC;
    pub type En38 = crate::EnumBitfieldStruct<u8, En38_SPEC>;
    impl En38 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En39_SPEC;
    pub type En39 = crate::EnumBitfieldStruct<u8, En39_SPEC>;
    impl En39 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En40_SPEC;
    pub type En40 = crate::EnumBitfieldStruct<u8, En40_SPEC>;
    impl En40 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En41_SPEC;
    pub type En41 = crate::EnumBitfieldStruct<u8, En41_SPEC>;
    impl En41 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En42_SPEC;
    pub type En42 = crate::EnumBitfieldStruct<u8, En42_SPEC>;
    impl En42 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En43_SPEC;
    pub type En43 = crate::EnumBitfieldStruct<u8, En43_SPEC>;
    impl En43 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En44_SPEC;
    pub type En44 = crate::EnumBitfieldStruct<u8, En44_SPEC>;
    impl En44 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En45_SPEC;
    pub type En45 = crate::EnumBitfieldStruct<u8, En45_SPEC>;
    impl En45 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En46_SPEC;
    pub type En46 = crate::EnumBitfieldStruct<u8, En46_SPEC>;
    impl En46 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En47_SPEC;
    pub type En47 = crate::EnumBitfieldStruct<u8, En47_SPEC>;
    impl En47 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En48_SPEC;
    pub type En48 = crate::EnumBitfieldStruct<u8, En48_SPEC>;
    impl En48 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En49_SPEC;
    pub type En49 = crate::EnumBitfieldStruct<u8, En49_SPEC>;
    impl En49 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En50_SPEC;
    pub type En50 = crate::EnumBitfieldStruct<u8, En50_SPEC>;
    impl En50 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En51_SPEC;
    pub type En51 = crate::EnumBitfieldStruct<u8, En51_SPEC>;
    impl En51 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En52_SPEC;
    pub type En52 = crate::EnumBitfieldStruct<u8, En52_SPEC>;
    impl En52 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En53_SPEC;
    pub type En53 = crate::EnumBitfieldStruct<u8, En53_SPEC>;
    impl En53 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En54_SPEC;
    pub type En54 = crate::EnumBitfieldStruct<u8, En54_SPEC>;
    impl En54 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En55_SPEC;
    pub type En55 = crate::EnumBitfieldStruct<u8, En55_SPEC>;
    impl En55 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En56_SPEC;
    pub type En56 = crate::EnumBitfieldStruct<u8, En56_SPEC>;
    impl En56 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En57_SPEC;
    pub type En57 = crate::EnumBitfieldStruct<u8, En57_SPEC>;
    impl En57 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En58_SPEC;
    pub type En58 = crate::EnumBitfieldStruct<u8, En58_SPEC>;
    impl En58 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En59_SPEC;
    pub type En59 = crate::EnumBitfieldStruct<u8, En59_SPEC>;
    impl En59 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En60_SPEC;
    pub type En60 = crate::EnumBitfieldStruct<u8, En60_SPEC>;
    impl En60 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En61_SPEC;
    pub type En61 = crate::EnumBitfieldStruct<u8, En61_SPEC>;
    impl En61 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En62_SPEC;
    pub type En62 = crate::EnumBitfieldStruct<u8, En62_SPEC>;
    impl En62 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct En63_SPEC;
    pub type En63 = crate::EnumBitfieldStruct<u8, En63_SPEC>;
    impl En63 {
        #[doc = "Write access will terminate with error and will not be executed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Write and read accesses will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clc_SPEC;
impl crate::sealed::RegSpec for Clc_SPEC {
    type DataType = u32;
}
#[doc = "DAM Clock Control Register\n resetvalue={Application Reset:0x0}"]
pub type Clc = crate::RegValueT<Clc_SPEC>;

impl Clc {
    #[doc = "LMU DAM Disable Request Bit   DISR. This bit is used for enable disable control of the DAM ."]
    #[inline(always)]
    pub fn disr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, clc::Disr, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,clc::Disr, Clc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LMU DAM Disable Status Bit   DISS. Current state of DAM ."]
    #[inline(always)]
    pub fn diss(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, clc::Diss, Clc_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,clc::Diss, Clc_SPEC,crate::common::R>::from_register(self,0)
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
        #[doc = "0 DAM disable is not requested"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 DAM disable is requested"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Diss_SPEC;
    pub type Diss = crate::EnumBitfieldStruct<u8, Diss_SPEC>;
    impl Diss {
        #[doc = "0 DAM is enabled  default after reset"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 DAM is disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memcon_SPEC;
impl crate::sealed::RegSpec for Memcon_SPEC {
    type DataType = u32;
}
#[doc = "DAM Memory Control Register\n resetvalue={Application Reset:0x0}"]
pub type Memcon = crate::RegValueT<Memcon_SPEC>;

impl Memcon {
    #[doc = "Read Only Memory   ROM. Configure RAM to be Read Only Memory"]
    #[inline(always)]
    pub fn rom(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, memcon::Rom, Memcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,memcon::Rom, Memcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Internal ECC Error   INTERR. Flag set by hardware when the DAM logic detects an ECC error while accessing the RAM. This bit is cleared        by writing 0 but cannot be set by        software."]
    #[inline(always)]
    pub fn interr(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, memcon::Interr, Memcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,memcon::Interr, Memcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Internal Read Modify Write Error   RMWERR. Flag set by hardware when the DAM logic detects an ECC error on the read phase of an internal RMW        operation. This bit is cleared by writing 0 but cannot be set by software."]
    #[inline(always)]
    pub fn rmwerr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, memcon::Rmwerr, Memcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,memcon::Rmwerr, Memcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SRI Data Phase ECC Error   DATAERR. Flag set by hardware when the SRI interface detects an ECC error in the        data phase of an incoming write transaction. This bit is cleared by        writing 0 but cannot be set by software."]
    #[inline(always)]
    pub fn dataerr(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, memcon::Dataerr, Memcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,memcon::Dataerr, Memcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SRI Address Phase ECC Error   ADDERR. Flag set by hardware when the SRI interface detects an ECC error in the        address phase of an incoming transaction. This bit is cleared by writing 0 but cannot be set by software."]
    #[inline(always)]
    pub fn adderr(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, memcon::Adderr, Memcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,memcon::Adderr, Memcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Protection Bit for Memory Integrity Control Bit   PMIC. Will always return 0 when read"]
    #[inline(always)]
    pub fn pmic(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, memcon::Pmic, Memcon_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0x1,1,0,memcon::Pmic, Memcon_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "ECC Error Disable   ERRDIS. When set SRI bus errors caused by ECC errors in data read from the SRAM        will be disabled"]
    #[inline(always)]
    pub fn errdis(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, memcon::Errdis, Memcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,memcon::Errdis, Memcon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Memcon {
    #[inline(always)]
    fn default() -> Memcon {
        <crate::RegValueT<Memcon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod memcon {
    pub struct Rom_SPEC;
    pub type Rom = crate::EnumBitfieldStruct<u8, Rom_SPEC>;
    impl Rom {
        #[doc = "0 RAM can be written to from the SRI"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 RAM cannot br written to from the SRI"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Interr_SPEC;
    pub type Interr = crate::EnumBitfieldStruct<u8, Interr_SPEC>;
    impl Interr {
        #[doc = "0 No error has occurred"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An error has been observed during a RAM access."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rmwerr_SPEC;
    pub type Rmwerr = crate::EnumBitfieldStruct<u8, Rmwerr_SPEC>;
    impl Rmwerr {
        #[doc = "0 No error has occurred"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An error has been observed during an iRMW operation."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Dataerr_SPEC;
    pub type Dataerr = crate::EnumBitfieldStruct<u8, Dataerr_SPEC>;
    impl Dataerr {
        #[doc = "0 No error has occurred"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An ECC error has been observed on an SRI transaction addressed to the DAM"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Adderr_SPEC;
    pub type Adderr = crate::EnumBitfieldStruct<u8, Adderr_SPEC>;
    impl Adderr {
        #[doc = "0 No error has occurred"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An ECC error has been observed on an SRI transaction addressed to the DAM"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pmic_SPEC;
    pub type Pmic = crate::EnumBitfieldStruct<u8, Pmic_SPEC>;
    impl Pmic {
        #[doc = "0 Bit Protection  Bit 9 remains unchanged after MEMCON write."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit 9 will be updated by the current write to MEMCON"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Errdis_SPEC;
    pub type Errdis = crate::EnumBitfieldStruct<u8, Errdis_SPEC>;
    impl Errdis {
        #[doc = "0 Normal behavior. SRI error will occur on SRAM ECC errors. Default after reset"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Test Mode. SRI errors will not be generated on an SRAM ECC error. This does not affect the generation of interrupts."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modid_SPEC;
impl crate::sealed::RegSpec for Modid_SPEC {
    type DataType = u32;
}
#[doc = "DAM Module ID Register\n resetvalue={Application Reset:0x088C003}"]
pub type Modid = crate::RegValueT<Modid_SPEC>;

impl Modid {
    #[doc = "Module Identification Value   ID VALUE"]
    #[inline(always)]
    pub fn id_value(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Modid_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Modid_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Modid {
    #[inline(always)]
    fn default() -> Modid {
        <crate::RegValueT<Modid_SPEC> as RegisterValue<_>>::new(8962051)
    }
}

#[doc = "RGNACCEN"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rgnaccen(pub(super) *mut u8);
unsafe impl core::marker::Send for Rgnaccen {}
unsafe impl core::marker::Sync for Rgnaccen {}
impl Rgnaccen {
    #[doc = "DAM Region Read Enable Register A\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn rgnaccenrai(
        &self,
    ) -> crate::common::Reg<rgnaccen::RgnaccenrAi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "DAM Region Read Enable Register B\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn rgnaccenrbi(
        &self,
    ) -> crate::common::Reg<rgnaccen::RgnaccenrBi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
}
pub mod rgnaccen {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnaccenrAi_SPEC;
    impl crate::sealed::RegSpec for RgnaccenrAi_SPEC {
        type DataType = u32;
    }
    #[doc = "DAM Region Read Enable Register A\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type RgnaccenrAi = crate::RegValueT<RgnaccenrAi_SPEC>;

    impl RgnaccenrAi {
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en0(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            rgnaccenrai::En0,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                rgnaccenrai::En0,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en1(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            rgnaccenrai::En1,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                rgnaccenrai::En1,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en2(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            rgnaccenrai::En2,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                rgnaccenrai::En2,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en3(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            rgnaccenrai::En3,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                rgnaccenrai::En3,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en4(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            rgnaccenrai::En4,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                rgnaccenrai::En4,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en5(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            rgnaccenrai::En5,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                rgnaccenrai::En5,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en6(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x1,
            1,
            0,
            rgnaccenrai::En6,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                6,
                0x1,
                1,
                0,
                rgnaccenrai::En6,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en7(
            self,
        ) -> crate::common::RegisterField<
            7,
            0x1,
            1,
            0,
            rgnaccenrai::En7,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                rgnaccenrai::En7,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en8(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1,
            1,
            0,
            rgnaccenrai::En8,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                rgnaccenrai::En8,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en9(
            self,
        ) -> crate::common::RegisterField<
            9,
            0x1,
            1,
            0,
            rgnaccenrai::En9,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                9,
                0x1,
                1,
                0,
                rgnaccenrai::En9,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en10(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            rgnaccenrai::En10,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                rgnaccenrai::En10,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en11(
            self,
        ) -> crate::common::RegisterField<
            11,
            0x1,
            1,
            0,
            rgnaccenrai::En11,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                rgnaccenrai::En11,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en12(
            self,
        ) -> crate::common::RegisterField<
            12,
            0x1,
            1,
            0,
            rgnaccenrai::En12,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                12,
                0x1,
                1,
                0,
                rgnaccenrai::En12,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en13(
            self,
        ) -> crate::common::RegisterField<
            13,
            0x1,
            1,
            0,
            rgnaccenrai::En13,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                13,
                0x1,
                1,
                0,
                rgnaccenrai::En13,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en14(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x1,
            1,
            0,
            rgnaccenrai::En14,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                rgnaccenrai::En14,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en15(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            rgnaccenrai::En15,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                rgnaccenrai::En15,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en16(
            self,
        ) -> crate::common::RegisterField<
            16,
            0x1,
            1,
            0,
            rgnaccenrai::En16,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0x1,
                1,
                0,
                rgnaccenrai::En16,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en17(
            self,
        ) -> crate::common::RegisterField<
            17,
            0x1,
            1,
            0,
            rgnaccenrai::En17,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                17,
                0x1,
                1,
                0,
                rgnaccenrai::En17,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en18(
            self,
        ) -> crate::common::RegisterField<
            18,
            0x1,
            1,
            0,
            rgnaccenrai::En18,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                18,
                0x1,
                1,
                0,
                rgnaccenrai::En18,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en19(
            self,
        ) -> crate::common::RegisterField<
            19,
            0x1,
            1,
            0,
            rgnaccenrai::En19,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                19,
                0x1,
                1,
                0,
                rgnaccenrai::En19,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en20(
            self,
        ) -> crate::common::RegisterField<
            20,
            0x1,
            1,
            0,
            rgnaccenrai::En20,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0x1,
                1,
                0,
                rgnaccenrai::En20,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en21(
            self,
        ) -> crate::common::RegisterField<
            21,
            0x1,
            1,
            0,
            rgnaccenrai::En21,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                21,
                0x1,
                1,
                0,
                rgnaccenrai::En21,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en22(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x1,
            1,
            0,
            rgnaccenrai::En22,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                22,
                0x1,
                1,
                0,
                rgnaccenrai::En22,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en23(
            self,
        ) -> crate::common::RegisterField<
            23,
            0x1,
            1,
            0,
            rgnaccenrai::En23,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                23,
                0x1,
                1,
                0,
                rgnaccenrai::En23,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en24(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x1,
            1,
            0,
            rgnaccenrai::En24,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0x1,
                1,
                0,
                rgnaccenrai::En24,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en25(
            self,
        ) -> crate::common::RegisterField<
            25,
            0x1,
            1,
            0,
            rgnaccenrai::En25,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                25,
                0x1,
                1,
                0,
                rgnaccenrai::En25,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en26(
            self,
        ) -> crate::common::RegisterField<
            26,
            0x1,
            1,
            0,
            rgnaccenrai::En26,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                26,
                0x1,
                1,
                0,
                rgnaccenrai::En26,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en27(
            self,
        ) -> crate::common::RegisterField<
            27,
            0x1,
            1,
            0,
            rgnaccenrai::En27,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                27,
                0x1,
                1,
                0,
                rgnaccenrai::En27,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en28(
            self,
        ) -> crate::common::RegisterField<
            28,
            0x1,
            1,
            0,
            rgnaccenrai::En28,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                28,
                0x1,
                1,
                0,
                rgnaccenrai::En28,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en29(
            self,
        ) -> crate::common::RegisterField<
            29,
            0x1,
            1,
            0,
            rgnaccenrai::En29,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                29,
                0x1,
                1,
                0,
                rgnaccenrai::En29,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en30(
            self,
        ) -> crate::common::RegisterField<
            30,
            0x1,
            1,
            0,
            rgnaccenrai::En30,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                30,
                0x1,
                1,
                0,
                rgnaccenrai::En30,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en31(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            rgnaccenrai::En31,
            RgnaccenrAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                rgnaccenrai::En31,
                RgnaccenrAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for RgnaccenrAi {
        #[inline(always)]
        fn default() -> RgnaccenrAi {
            <crate::RegValueT<RgnaccenrAi_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
    pub mod rgnaccenrai {
        pub struct En0_SPEC;
        pub type En0 = crate::EnumBitfieldStruct<u8, En0_SPEC>;
        impl En0 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En1_SPEC;
        pub type En1 = crate::EnumBitfieldStruct<u8, En1_SPEC>;
        impl En1 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En2_SPEC;
        pub type En2 = crate::EnumBitfieldStruct<u8, En2_SPEC>;
        impl En2 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En3_SPEC;
        pub type En3 = crate::EnumBitfieldStruct<u8, En3_SPEC>;
        impl En3 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En4_SPEC;
        pub type En4 = crate::EnumBitfieldStruct<u8, En4_SPEC>;
        impl En4 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En5_SPEC;
        pub type En5 = crate::EnumBitfieldStruct<u8, En5_SPEC>;
        impl En5 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En6_SPEC;
        pub type En6 = crate::EnumBitfieldStruct<u8, En6_SPEC>;
        impl En6 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En7_SPEC;
        pub type En7 = crate::EnumBitfieldStruct<u8, En7_SPEC>;
        impl En7 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En8_SPEC;
        pub type En8 = crate::EnumBitfieldStruct<u8, En8_SPEC>;
        impl En8 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En9_SPEC;
        pub type En9 = crate::EnumBitfieldStruct<u8, En9_SPEC>;
        impl En9 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En10_SPEC;
        pub type En10 = crate::EnumBitfieldStruct<u8, En10_SPEC>;
        impl En10 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En11_SPEC;
        pub type En11 = crate::EnumBitfieldStruct<u8, En11_SPEC>;
        impl En11 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En12_SPEC;
        pub type En12 = crate::EnumBitfieldStruct<u8, En12_SPEC>;
        impl En12 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En13_SPEC;
        pub type En13 = crate::EnumBitfieldStruct<u8, En13_SPEC>;
        impl En13 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En14_SPEC;
        pub type En14 = crate::EnumBitfieldStruct<u8, En14_SPEC>;
        impl En14 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En15_SPEC;
        pub type En15 = crate::EnumBitfieldStruct<u8, En15_SPEC>;
        impl En15 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En16_SPEC;
        pub type En16 = crate::EnumBitfieldStruct<u8, En16_SPEC>;
        impl En16 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En17_SPEC;
        pub type En17 = crate::EnumBitfieldStruct<u8, En17_SPEC>;
        impl En17 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En18_SPEC;
        pub type En18 = crate::EnumBitfieldStruct<u8, En18_SPEC>;
        impl En18 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En19_SPEC;
        pub type En19 = crate::EnumBitfieldStruct<u8, En19_SPEC>;
        impl En19 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En20_SPEC;
        pub type En20 = crate::EnumBitfieldStruct<u8, En20_SPEC>;
        impl En20 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En21_SPEC;
        pub type En21 = crate::EnumBitfieldStruct<u8, En21_SPEC>;
        impl En21 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En22_SPEC;
        pub type En22 = crate::EnumBitfieldStruct<u8, En22_SPEC>;
        impl En22 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En23_SPEC;
        pub type En23 = crate::EnumBitfieldStruct<u8, En23_SPEC>;
        impl En23 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En24_SPEC;
        pub type En24 = crate::EnumBitfieldStruct<u8, En24_SPEC>;
        impl En24 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En25_SPEC;
        pub type En25 = crate::EnumBitfieldStruct<u8, En25_SPEC>;
        impl En25 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En26_SPEC;
        pub type En26 = crate::EnumBitfieldStruct<u8, En26_SPEC>;
        impl En26 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En27_SPEC;
        pub type En27 = crate::EnumBitfieldStruct<u8, En27_SPEC>;
        impl En27 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En28_SPEC;
        pub type En28 = crate::EnumBitfieldStruct<u8, En28_SPEC>;
        impl En28 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En29_SPEC;
        pub type En29 = crate::EnumBitfieldStruct<u8, En29_SPEC>;
        impl En29 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En30_SPEC;
        pub type En30 = crate::EnumBitfieldStruct<u8, En30_SPEC>;
        impl En30 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En31_SPEC;
        pub type En31 = crate::EnumBitfieldStruct<u8, En31_SPEC>;
        impl En31 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnaccenrBi_SPEC;
    impl crate::sealed::RegSpec for RgnaccenrBi_SPEC {
        type DataType = u32;
    }
    #[doc = "DAM Region Read Enable Register B\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type RgnaccenrBi = crate::RegValueT<RgnaccenrBi_SPEC>;

    impl RgnaccenrBi {
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en32(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            rgnaccenrbi::En32,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                rgnaccenrbi::En32,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en33(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            rgnaccenrbi::En33,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                rgnaccenrbi::En33,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en34(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            rgnaccenrbi::En34,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                rgnaccenrbi::En34,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en35(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            rgnaccenrbi::En35,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                rgnaccenrbi::En35,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en36(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            rgnaccenrbi::En36,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                rgnaccenrbi::En36,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en37(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            rgnaccenrbi::En37,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                rgnaccenrbi::En37,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en38(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x1,
            1,
            0,
            rgnaccenrbi::En38,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                6,
                0x1,
                1,
                0,
                rgnaccenrbi::En38,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en39(
            self,
        ) -> crate::common::RegisterField<
            7,
            0x1,
            1,
            0,
            rgnaccenrbi::En39,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                rgnaccenrbi::En39,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en40(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1,
            1,
            0,
            rgnaccenrbi::En40,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                rgnaccenrbi::En40,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en41(
            self,
        ) -> crate::common::RegisterField<
            9,
            0x1,
            1,
            0,
            rgnaccenrbi::En41,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                9,
                0x1,
                1,
                0,
                rgnaccenrbi::En41,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en42(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            rgnaccenrbi::En42,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                rgnaccenrbi::En42,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en43(
            self,
        ) -> crate::common::RegisterField<
            11,
            0x1,
            1,
            0,
            rgnaccenrbi::En43,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                rgnaccenrbi::En43,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en44(
            self,
        ) -> crate::common::RegisterField<
            12,
            0x1,
            1,
            0,
            rgnaccenrbi::En44,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                12,
                0x1,
                1,
                0,
                rgnaccenrbi::En44,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en45(
            self,
        ) -> crate::common::RegisterField<
            13,
            0x1,
            1,
            0,
            rgnaccenrbi::En45,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                13,
                0x1,
                1,
                0,
                rgnaccenrbi::En45,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en46(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x1,
            1,
            0,
            rgnaccenrbi::En46,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                rgnaccenrbi::En46,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en47(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            rgnaccenrbi::En47,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                rgnaccenrbi::En47,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en48(
            self,
        ) -> crate::common::RegisterField<
            16,
            0x1,
            1,
            0,
            rgnaccenrbi::En48,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0x1,
                1,
                0,
                rgnaccenrbi::En48,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en49(
            self,
        ) -> crate::common::RegisterField<
            17,
            0x1,
            1,
            0,
            rgnaccenrbi::En49,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                17,
                0x1,
                1,
                0,
                rgnaccenrbi::En49,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en50(
            self,
        ) -> crate::common::RegisterField<
            18,
            0x1,
            1,
            0,
            rgnaccenrbi::En50,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                18,
                0x1,
                1,
                0,
                rgnaccenrbi::En50,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en51(
            self,
        ) -> crate::common::RegisterField<
            19,
            0x1,
            1,
            0,
            rgnaccenrbi::En51,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                19,
                0x1,
                1,
                0,
                rgnaccenrbi::En51,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en52(
            self,
        ) -> crate::common::RegisterField<
            20,
            0x1,
            1,
            0,
            rgnaccenrbi::En52,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0x1,
                1,
                0,
                rgnaccenrbi::En52,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en53(
            self,
        ) -> crate::common::RegisterField<
            21,
            0x1,
            1,
            0,
            rgnaccenrbi::En53,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                21,
                0x1,
                1,
                0,
                rgnaccenrbi::En53,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en54(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x1,
            1,
            0,
            rgnaccenrbi::En54,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                22,
                0x1,
                1,
                0,
                rgnaccenrbi::En54,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en55(
            self,
        ) -> crate::common::RegisterField<
            23,
            0x1,
            1,
            0,
            rgnaccenrbi::En55,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                23,
                0x1,
                1,
                0,
                rgnaccenrbi::En55,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en56(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x1,
            1,
            0,
            rgnaccenrbi::En56,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0x1,
                1,
                0,
                rgnaccenrbi::En56,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en57(
            self,
        ) -> crate::common::RegisterField<
            25,
            0x1,
            1,
            0,
            rgnaccenrbi::En57,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                25,
                0x1,
                1,
                0,
                rgnaccenrbi::En57,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en58(
            self,
        ) -> crate::common::RegisterField<
            26,
            0x1,
            1,
            0,
            rgnaccenrbi::En58,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                26,
                0x1,
                1,
                0,
                rgnaccenrbi::En58,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en59(
            self,
        ) -> crate::common::RegisterField<
            27,
            0x1,
            1,
            0,
            rgnaccenrbi::En59,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                27,
                0x1,
                1,
                0,
                rgnaccenrbi::En59,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en60(
            self,
        ) -> crate::common::RegisterField<
            28,
            0x1,
            1,
            0,
            rgnaccenrbi::En60,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                28,
                0x1,
                1,
                0,
                rgnaccenrbi::En60,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en61(
            self,
        ) -> crate::common::RegisterField<
            29,
            0x1,
            1,
            0,
            rgnaccenrbi::En61,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                29,
                0x1,
                1,
                0,
                rgnaccenrbi::En61,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en62(
            self,
        ) -> crate::common::RegisterField<
            30,
            0x1,
            1,
            0,
            rgnaccenrbi::En62,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                30,
                0x1,
                1,
                0,
                rgnaccenrbi::En62,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en63(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            rgnaccenrbi::En63,
            RgnaccenrBi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                rgnaccenrbi::En63,
                RgnaccenrBi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for RgnaccenrBi {
        #[inline(always)]
        fn default() -> RgnaccenrBi {
            <crate::RegValueT<RgnaccenrBi_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
    pub mod rgnaccenrbi {
        pub struct En32_SPEC;
        pub type En32 = crate::EnumBitfieldStruct<u8, En32_SPEC>;
        impl En32 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En33_SPEC;
        pub type En33 = crate::EnumBitfieldStruct<u8, En33_SPEC>;
        impl En33 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En34_SPEC;
        pub type En34 = crate::EnumBitfieldStruct<u8, En34_SPEC>;
        impl En34 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En35_SPEC;
        pub type En35 = crate::EnumBitfieldStruct<u8, En35_SPEC>;
        impl En35 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En36_SPEC;
        pub type En36 = crate::EnumBitfieldStruct<u8, En36_SPEC>;
        impl En36 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En37_SPEC;
        pub type En37 = crate::EnumBitfieldStruct<u8, En37_SPEC>;
        impl En37 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En38_SPEC;
        pub type En38 = crate::EnumBitfieldStruct<u8, En38_SPEC>;
        impl En38 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En39_SPEC;
        pub type En39 = crate::EnumBitfieldStruct<u8, En39_SPEC>;
        impl En39 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En40_SPEC;
        pub type En40 = crate::EnumBitfieldStruct<u8, En40_SPEC>;
        impl En40 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En41_SPEC;
        pub type En41 = crate::EnumBitfieldStruct<u8, En41_SPEC>;
        impl En41 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En42_SPEC;
        pub type En42 = crate::EnumBitfieldStruct<u8, En42_SPEC>;
        impl En42 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En43_SPEC;
        pub type En43 = crate::EnumBitfieldStruct<u8, En43_SPEC>;
        impl En43 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En44_SPEC;
        pub type En44 = crate::EnumBitfieldStruct<u8, En44_SPEC>;
        impl En44 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En45_SPEC;
        pub type En45 = crate::EnumBitfieldStruct<u8, En45_SPEC>;
        impl En45 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En46_SPEC;
        pub type En46 = crate::EnumBitfieldStruct<u8, En46_SPEC>;
        impl En46 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En47_SPEC;
        pub type En47 = crate::EnumBitfieldStruct<u8, En47_SPEC>;
        impl En47 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En48_SPEC;
        pub type En48 = crate::EnumBitfieldStruct<u8, En48_SPEC>;
        impl En48 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En49_SPEC;
        pub type En49 = crate::EnumBitfieldStruct<u8, En49_SPEC>;
        impl En49 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En50_SPEC;
        pub type En50 = crate::EnumBitfieldStruct<u8, En50_SPEC>;
        impl En50 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En51_SPEC;
        pub type En51 = crate::EnumBitfieldStruct<u8, En51_SPEC>;
        impl En51 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En52_SPEC;
        pub type En52 = crate::EnumBitfieldStruct<u8, En52_SPEC>;
        impl En52 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En53_SPEC;
        pub type En53 = crate::EnumBitfieldStruct<u8, En53_SPEC>;
        impl En53 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En54_SPEC;
        pub type En54 = crate::EnumBitfieldStruct<u8, En54_SPEC>;
        impl En54 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En55_SPEC;
        pub type En55 = crate::EnumBitfieldStruct<u8, En55_SPEC>;
        impl En55 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En56_SPEC;
        pub type En56 = crate::EnumBitfieldStruct<u8, En56_SPEC>;
        impl En56 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En57_SPEC;
        pub type En57 = crate::EnumBitfieldStruct<u8, En57_SPEC>;
        impl En57 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En58_SPEC;
        pub type En58 = crate::EnumBitfieldStruct<u8, En58_SPEC>;
        impl En58 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En59_SPEC;
        pub type En59 = crate::EnumBitfieldStruct<u8, En59_SPEC>;
        impl En59 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En60_SPEC;
        pub type En60 = crate::EnumBitfieldStruct<u8, En60_SPEC>;
        impl En60 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En61_SPEC;
        pub type En61 = crate::EnumBitfieldStruct<u8, En61_SPEC>;
        impl En61 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En62_SPEC;
        pub type En62 = crate::EnumBitfieldStruct<u8, En62_SPEC>;
        impl En62 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En63_SPEC;
        pub type En63 = crate::EnumBitfieldStruct<u8, En63_SPEC>;
        impl En63 {
            #[doc = "0 No read        accesses are permitted for this region. Read accesses will terminate        with an error condition"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Read permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
}
#[doc = "RGN"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rgn(pub(super) *mut u8);
unsafe impl core::marker::Send for Rgn {}
unsafe impl core::marker::Sync for Rgn {}
impl Rgn {
    #[doc = "DAM Region Write Enable Register A\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn rgnaccenwax(
        &self,
    ) -> crate::common::Reg<rgn::RgnaccenwAx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "DAM Region Write Enable Register B\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn rgnaccenwbx(
        &self,
    ) -> crate::common::Reg<rgn::RgnaccenwBx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "DAM Region Lower Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rgnlax(&self) -> crate::common::Reg<rgn::RgnlAx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "DAM Region Upper Address Register\n resetvalue={Application Reset:0x0FFFFFFE0}"]
    #[inline(always)]
    pub const fn rgnuax(&self) -> crate::common::Reg<rgn::RgnuAx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
}
pub mod rgn {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnaccenwAx_SPEC;
    impl crate::sealed::RegSpec for RgnaccenwAx_SPEC {
        type DataType = u32;
    }
    #[doc = "DAM Region Write Enable Register A\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type RgnaccenwAx = crate::RegValueT<RgnaccenwAx_SPEC>;

    impl RgnaccenwAx {
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en0(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            rgnaccenwax::En0,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                rgnaccenwax::En0,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en1(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            rgnaccenwax::En1,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                rgnaccenwax::En1,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en2(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            rgnaccenwax::En2,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                rgnaccenwax::En2,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en3(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            rgnaccenwax::En3,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                rgnaccenwax::En3,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en4(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            rgnaccenwax::En4,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                rgnaccenwax::En4,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en5(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            rgnaccenwax::En5,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                rgnaccenwax::En5,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en6(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x1,
            1,
            0,
            rgnaccenwax::En6,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                6,
                0x1,
                1,
                0,
                rgnaccenwax::En6,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en7(
            self,
        ) -> crate::common::RegisterField<
            7,
            0x1,
            1,
            0,
            rgnaccenwax::En7,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                rgnaccenwax::En7,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en8(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1,
            1,
            0,
            rgnaccenwax::En8,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                rgnaccenwax::En8,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en9(
            self,
        ) -> crate::common::RegisterField<
            9,
            0x1,
            1,
            0,
            rgnaccenwax::En9,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                9,
                0x1,
                1,
                0,
                rgnaccenwax::En9,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en10(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            rgnaccenwax::En10,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                rgnaccenwax::En10,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en11(
            self,
        ) -> crate::common::RegisterField<
            11,
            0x1,
            1,
            0,
            rgnaccenwax::En11,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                rgnaccenwax::En11,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en12(
            self,
        ) -> crate::common::RegisterField<
            12,
            0x1,
            1,
            0,
            rgnaccenwax::En12,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                12,
                0x1,
                1,
                0,
                rgnaccenwax::En12,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en13(
            self,
        ) -> crate::common::RegisterField<
            13,
            0x1,
            1,
            0,
            rgnaccenwax::En13,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                13,
                0x1,
                1,
                0,
                rgnaccenwax::En13,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en14(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x1,
            1,
            0,
            rgnaccenwax::En14,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                rgnaccenwax::En14,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en15(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            rgnaccenwax::En15,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                rgnaccenwax::En15,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en16(
            self,
        ) -> crate::common::RegisterField<
            16,
            0x1,
            1,
            0,
            rgnaccenwax::En16,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0x1,
                1,
                0,
                rgnaccenwax::En16,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en17(
            self,
        ) -> crate::common::RegisterField<
            17,
            0x1,
            1,
            0,
            rgnaccenwax::En17,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                17,
                0x1,
                1,
                0,
                rgnaccenwax::En17,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en18(
            self,
        ) -> crate::common::RegisterField<
            18,
            0x1,
            1,
            0,
            rgnaccenwax::En18,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                18,
                0x1,
                1,
                0,
                rgnaccenwax::En18,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en19(
            self,
        ) -> crate::common::RegisterField<
            19,
            0x1,
            1,
            0,
            rgnaccenwax::En19,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                19,
                0x1,
                1,
                0,
                rgnaccenwax::En19,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en20(
            self,
        ) -> crate::common::RegisterField<
            20,
            0x1,
            1,
            0,
            rgnaccenwax::En20,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0x1,
                1,
                0,
                rgnaccenwax::En20,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en21(
            self,
        ) -> crate::common::RegisterField<
            21,
            0x1,
            1,
            0,
            rgnaccenwax::En21,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                21,
                0x1,
                1,
                0,
                rgnaccenwax::En21,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en22(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x1,
            1,
            0,
            rgnaccenwax::En22,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                22,
                0x1,
                1,
                0,
                rgnaccenwax::En22,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en23(
            self,
        ) -> crate::common::RegisterField<
            23,
            0x1,
            1,
            0,
            rgnaccenwax::En23,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                23,
                0x1,
                1,
                0,
                rgnaccenwax::En23,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en24(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x1,
            1,
            0,
            rgnaccenwax::En24,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0x1,
                1,
                0,
                rgnaccenwax::En24,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en25(
            self,
        ) -> crate::common::RegisterField<
            25,
            0x1,
            1,
            0,
            rgnaccenwax::En25,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                25,
                0x1,
                1,
                0,
                rgnaccenwax::En25,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en26(
            self,
        ) -> crate::common::RegisterField<
            26,
            0x1,
            1,
            0,
            rgnaccenwax::En26,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                26,
                0x1,
                1,
                0,
                rgnaccenwax::En26,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en27(
            self,
        ) -> crate::common::RegisterField<
            27,
            0x1,
            1,
            0,
            rgnaccenwax::En27,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                27,
                0x1,
                1,
                0,
                rgnaccenwax::En27,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en28(
            self,
        ) -> crate::common::RegisterField<
            28,
            0x1,
            1,
            0,
            rgnaccenwax::En28,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                28,
                0x1,
                1,
                0,
                rgnaccenwax::En28,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en29(
            self,
        ) -> crate::common::RegisterField<
            29,
            0x1,
            1,
            0,
            rgnaccenwax::En29,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                29,
                0x1,
                1,
                0,
                rgnaccenwax::En29,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en30(
            self,
        ) -> crate::common::RegisterField<
            30,
            0x1,
            1,
            0,
            rgnaccenwax::En30,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                30,
                0x1,
                1,
                0,
                rgnaccenwax::En30,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en31(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            rgnaccenwax::En31,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                rgnaccenwax::En31,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for RgnaccenwAx {
        #[inline(always)]
        fn default() -> RgnaccenwAx {
            <crate::RegValueT<RgnaccenwAx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
    pub mod rgnaccenwax {
        pub struct En0_SPEC;
        pub type En0 = crate::EnumBitfieldStruct<u8, En0_SPEC>;
        impl En0 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En1_SPEC;
        pub type En1 = crate::EnumBitfieldStruct<u8, En1_SPEC>;
        impl En1 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En2_SPEC;
        pub type En2 = crate::EnumBitfieldStruct<u8, En2_SPEC>;
        impl En2 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En3_SPEC;
        pub type En3 = crate::EnumBitfieldStruct<u8, En3_SPEC>;
        impl En3 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En4_SPEC;
        pub type En4 = crate::EnumBitfieldStruct<u8, En4_SPEC>;
        impl En4 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En5_SPEC;
        pub type En5 = crate::EnumBitfieldStruct<u8, En5_SPEC>;
        impl En5 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En6_SPEC;
        pub type En6 = crate::EnumBitfieldStruct<u8, En6_SPEC>;
        impl En6 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En7_SPEC;
        pub type En7 = crate::EnumBitfieldStruct<u8, En7_SPEC>;
        impl En7 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En8_SPEC;
        pub type En8 = crate::EnumBitfieldStruct<u8, En8_SPEC>;
        impl En8 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En9_SPEC;
        pub type En9 = crate::EnumBitfieldStruct<u8, En9_SPEC>;
        impl En9 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En10_SPEC;
        pub type En10 = crate::EnumBitfieldStruct<u8, En10_SPEC>;
        impl En10 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En11_SPEC;
        pub type En11 = crate::EnumBitfieldStruct<u8, En11_SPEC>;
        impl En11 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En12_SPEC;
        pub type En12 = crate::EnumBitfieldStruct<u8, En12_SPEC>;
        impl En12 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En13_SPEC;
        pub type En13 = crate::EnumBitfieldStruct<u8, En13_SPEC>;
        impl En13 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En14_SPEC;
        pub type En14 = crate::EnumBitfieldStruct<u8, En14_SPEC>;
        impl En14 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En15_SPEC;
        pub type En15 = crate::EnumBitfieldStruct<u8, En15_SPEC>;
        impl En15 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En16_SPEC;
        pub type En16 = crate::EnumBitfieldStruct<u8, En16_SPEC>;
        impl En16 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En17_SPEC;
        pub type En17 = crate::EnumBitfieldStruct<u8, En17_SPEC>;
        impl En17 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En18_SPEC;
        pub type En18 = crate::EnumBitfieldStruct<u8, En18_SPEC>;
        impl En18 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En19_SPEC;
        pub type En19 = crate::EnumBitfieldStruct<u8, En19_SPEC>;
        impl En19 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En20_SPEC;
        pub type En20 = crate::EnumBitfieldStruct<u8, En20_SPEC>;
        impl En20 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En21_SPEC;
        pub type En21 = crate::EnumBitfieldStruct<u8, En21_SPEC>;
        impl En21 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En22_SPEC;
        pub type En22 = crate::EnumBitfieldStruct<u8, En22_SPEC>;
        impl En22 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En23_SPEC;
        pub type En23 = crate::EnumBitfieldStruct<u8, En23_SPEC>;
        impl En23 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En24_SPEC;
        pub type En24 = crate::EnumBitfieldStruct<u8, En24_SPEC>;
        impl En24 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En25_SPEC;
        pub type En25 = crate::EnumBitfieldStruct<u8, En25_SPEC>;
        impl En25 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En26_SPEC;
        pub type En26 = crate::EnumBitfieldStruct<u8, En26_SPEC>;
        impl En26 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En27_SPEC;
        pub type En27 = crate::EnumBitfieldStruct<u8, En27_SPEC>;
        impl En27 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En28_SPEC;
        pub type En28 = crate::EnumBitfieldStruct<u8, En28_SPEC>;
        impl En28 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En29_SPEC;
        pub type En29 = crate::EnumBitfieldStruct<u8, En29_SPEC>;
        impl En29 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En30_SPEC;
        pub type En30 = crate::EnumBitfieldStruct<u8, En30_SPEC>;
        impl En30 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En31_SPEC;
        pub type En31 = crate::EnumBitfieldStruct<u8, En31_SPEC>;
        impl En31 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnaccenwBx_SPEC;
    impl crate::sealed::RegSpec for RgnaccenwBx_SPEC {
        type DataType = u32;
    }
    #[doc = "DAM Region Write Enable Register B\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type RgnaccenwBx = crate::RegValueT<RgnaccenwBx_SPEC>;

    impl RgnaccenwBx {
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en32(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            rgnaccenwbx::En32,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                rgnaccenwbx::En32,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en33(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            rgnaccenwbx::En33,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                rgnaccenwbx::En33,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en34(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            rgnaccenwbx::En34,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                rgnaccenwbx::En34,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en35(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            rgnaccenwbx::En35,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                rgnaccenwbx::En35,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en36(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            rgnaccenwbx::En36,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                rgnaccenwbx::En36,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en37(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            rgnaccenwbx::En37,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                rgnaccenwbx::En37,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en38(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x1,
            1,
            0,
            rgnaccenwbx::En38,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                6,
                0x1,
                1,
                0,
                rgnaccenwbx::En38,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en39(
            self,
        ) -> crate::common::RegisterField<
            7,
            0x1,
            1,
            0,
            rgnaccenwbx::En39,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                rgnaccenwbx::En39,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en40(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1,
            1,
            0,
            rgnaccenwbx::En40,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                rgnaccenwbx::En40,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en41(
            self,
        ) -> crate::common::RegisterField<
            9,
            0x1,
            1,
            0,
            rgnaccenwbx::En41,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                9,
                0x1,
                1,
                0,
                rgnaccenwbx::En41,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en42(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            rgnaccenwbx::En42,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                rgnaccenwbx::En42,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en43(
            self,
        ) -> crate::common::RegisterField<
            11,
            0x1,
            1,
            0,
            rgnaccenwbx::En43,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                rgnaccenwbx::En43,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en44(
            self,
        ) -> crate::common::RegisterField<
            12,
            0x1,
            1,
            0,
            rgnaccenwbx::En44,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                12,
                0x1,
                1,
                0,
                rgnaccenwbx::En44,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en45(
            self,
        ) -> crate::common::RegisterField<
            13,
            0x1,
            1,
            0,
            rgnaccenwbx::En45,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                13,
                0x1,
                1,
                0,
                rgnaccenwbx::En45,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en46(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x1,
            1,
            0,
            rgnaccenwbx::En46,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                rgnaccenwbx::En46,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en47(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            rgnaccenwbx::En47,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                rgnaccenwbx::En47,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en48(
            self,
        ) -> crate::common::RegisterField<
            16,
            0x1,
            1,
            0,
            rgnaccenwbx::En48,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0x1,
                1,
                0,
                rgnaccenwbx::En48,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en49(
            self,
        ) -> crate::common::RegisterField<
            17,
            0x1,
            1,
            0,
            rgnaccenwbx::En49,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                17,
                0x1,
                1,
                0,
                rgnaccenwbx::En49,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en50(
            self,
        ) -> crate::common::RegisterField<
            18,
            0x1,
            1,
            0,
            rgnaccenwbx::En50,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                18,
                0x1,
                1,
                0,
                rgnaccenwbx::En50,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en51(
            self,
        ) -> crate::common::RegisterField<
            19,
            0x1,
            1,
            0,
            rgnaccenwbx::En51,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                19,
                0x1,
                1,
                0,
                rgnaccenwbx::En51,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en52(
            self,
        ) -> crate::common::RegisterField<
            20,
            0x1,
            1,
            0,
            rgnaccenwbx::En52,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0x1,
                1,
                0,
                rgnaccenwbx::En52,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en53(
            self,
        ) -> crate::common::RegisterField<
            21,
            0x1,
            1,
            0,
            rgnaccenwbx::En53,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                21,
                0x1,
                1,
                0,
                rgnaccenwbx::En53,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en54(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x1,
            1,
            0,
            rgnaccenwbx::En54,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                22,
                0x1,
                1,
                0,
                rgnaccenwbx::En54,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en55(
            self,
        ) -> crate::common::RegisterField<
            23,
            0x1,
            1,
            0,
            rgnaccenwbx::En55,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                23,
                0x1,
                1,
                0,
                rgnaccenwbx::En55,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en56(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x1,
            1,
            0,
            rgnaccenwbx::En56,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0x1,
                1,
                0,
                rgnaccenwbx::En56,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en57(
            self,
        ) -> crate::common::RegisterField<
            25,
            0x1,
            1,
            0,
            rgnaccenwbx::En57,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                25,
                0x1,
                1,
                0,
                rgnaccenwbx::En57,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en58(
            self,
        ) -> crate::common::RegisterField<
            26,
            0x1,
            1,
            0,
            rgnaccenwbx::En58,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                26,
                0x1,
                1,
                0,
                rgnaccenwbx::En58,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en59(
            self,
        ) -> crate::common::RegisterField<
            27,
            0x1,
            1,
            0,
            rgnaccenwbx::En59,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                27,
                0x1,
                1,
                0,
                rgnaccenwbx::En59,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en60(
            self,
        ) -> crate::common::RegisterField<
            28,
            0x1,
            1,
            0,
            rgnaccenwbx::En60,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                28,
                0x1,
                1,
                0,
                rgnaccenwbx::En60,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en61(
            self,
        ) -> crate::common::RegisterField<
            29,
            0x1,
            1,
            0,
            rgnaccenwbx::En61,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                29,
                0x1,
                1,
                0,
                rgnaccenwbx::En61,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en62(
            self,
        ) -> crate::common::RegisterField<
            30,
            0x1,
            1,
            0,
            rgnaccenwbx::En62,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                30,
                0x1,
                1,
                0,
                rgnaccenwbx::En62,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en63(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            rgnaccenwbx::En63,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                rgnaccenwbx::En63,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for RgnaccenwBx {
        #[inline(always)]
        fn default() -> RgnaccenwBx {
            <crate::RegValueT<RgnaccenwBx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
    pub mod rgnaccenwbx {
        pub struct En32_SPEC;
        pub type En32 = crate::EnumBitfieldStruct<u8, En32_SPEC>;
        impl En32 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En33_SPEC;
        pub type En33 = crate::EnumBitfieldStruct<u8, En33_SPEC>;
        impl En33 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En34_SPEC;
        pub type En34 = crate::EnumBitfieldStruct<u8, En34_SPEC>;
        impl En34 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En35_SPEC;
        pub type En35 = crate::EnumBitfieldStruct<u8, En35_SPEC>;
        impl En35 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En36_SPEC;
        pub type En36 = crate::EnumBitfieldStruct<u8, En36_SPEC>;
        impl En36 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En37_SPEC;
        pub type En37 = crate::EnumBitfieldStruct<u8, En37_SPEC>;
        impl En37 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En38_SPEC;
        pub type En38 = crate::EnumBitfieldStruct<u8, En38_SPEC>;
        impl En38 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En39_SPEC;
        pub type En39 = crate::EnumBitfieldStruct<u8, En39_SPEC>;
        impl En39 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En40_SPEC;
        pub type En40 = crate::EnumBitfieldStruct<u8, En40_SPEC>;
        impl En40 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En41_SPEC;
        pub type En41 = crate::EnumBitfieldStruct<u8, En41_SPEC>;
        impl En41 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En42_SPEC;
        pub type En42 = crate::EnumBitfieldStruct<u8, En42_SPEC>;
        impl En42 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En43_SPEC;
        pub type En43 = crate::EnumBitfieldStruct<u8, En43_SPEC>;
        impl En43 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En44_SPEC;
        pub type En44 = crate::EnumBitfieldStruct<u8, En44_SPEC>;
        impl En44 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En45_SPEC;
        pub type En45 = crate::EnumBitfieldStruct<u8, En45_SPEC>;
        impl En45 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En46_SPEC;
        pub type En46 = crate::EnumBitfieldStruct<u8, En46_SPEC>;
        impl En46 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En47_SPEC;
        pub type En47 = crate::EnumBitfieldStruct<u8, En47_SPEC>;
        impl En47 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En48_SPEC;
        pub type En48 = crate::EnumBitfieldStruct<u8, En48_SPEC>;
        impl En48 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En49_SPEC;
        pub type En49 = crate::EnumBitfieldStruct<u8, En49_SPEC>;
        impl En49 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En50_SPEC;
        pub type En50 = crate::EnumBitfieldStruct<u8, En50_SPEC>;
        impl En50 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En51_SPEC;
        pub type En51 = crate::EnumBitfieldStruct<u8, En51_SPEC>;
        impl En51 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En52_SPEC;
        pub type En52 = crate::EnumBitfieldStruct<u8, En52_SPEC>;
        impl En52 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En53_SPEC;
        pub type En53 = crate::EnumBitfieldStruct<u8, En53_SPEC>;
        impl En53 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En54_SPEC;
        pub type En54 = crate::EnumBitfieldStruct<u8, En54_SPEC>;
        impl En54 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En55_SPEC;
        pub type En55 = crate::EnumBitfieldStruct<u8, En55_SPEC>;
        impl En55 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En56_SPEC;
        pub type En56 = crate::EnumBitfieldStruct<u8, En56_SPEC>;
        impl En56 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En57_SPEC;
        pub type En57 = crate::EnumBitfieldStruct<u8, En57_SPEC>;
        impl En57 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En58_SPEC;
        pub type En58 = crate::EnumBitfieldStruct<u8, En58_SPEC>;
        impl En58 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En59_SPEC;
        pub type En59 = crate::EnumBitfieldStruct<u8, En59_SPEC>;
        impl En59 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En60_SPEC;
        pub type En60 = crate::EnumBitfieldStruct<u8, En60_SPEC>;
        impl En60 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En61_SPEC;
        pub type En61 = crate::EnumBitfieldStruct<u8, En61_SPEC>;
        impl En61 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En62_SPEC;
        pub type En62 = crate::EnumBitfieldStruct<u8, En62_SPEC>;
        impl En62 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct En63_SPEC;
        pub type En63 = crate::EnumBitfieldStruct<u8, En63_SPEC>;
        impl En63 {
            #[doc = "0 No write        accesses identified with TAG n are permitted for this region. Writes        accesses will terminate with an error condition."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write permitted        for this region"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnlAx_SPEC;
    impl crate::sealed::RegSpec for RgnlAx_SPEC {
        type DataType = u32;
    }
    #[doc = "DAM Region Lower Address Register\n resetvalue={Application Reset:0x0}"]
    pub type RgnlAx = crate::RegValueT<RgnlAx_SPEC>;

    impl RgnlAx {
        #[doc = "Region Lower Address   ADDR. Bits 31 to 5 of the SRI address which is the lower bound of the defined        memory region"]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, RgnlAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x7ffffff,1,0,u32, RgnlAx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for RgnlAx {
        #[inline(always)]
        fn default() -> RgnlAx {
            <crate::RegValueT<RgnlAx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnuAx_SPEC;
    impl crate::sealed::RegSpec for RgnuAx_SPEC {
        type DataType = u32;
    }
    #[doc = "DAM Region Upper Address Register\n resetvalue={Application Reset:0x0FFFFFFE0}"]
    pub type RgnuAx = crate::RegValueT<RgnuAx_SPEC>;

    impl RgnuAx {
        #[doc = "Region Lower Address   ADDR. Bits 31 to 5 of the SRI address which is the upper bound of the defined        memory region. i.e. the first address outside the protected region."]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, RgnuAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x7ffffff,1,0,u32, RgnuAx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for RgnuAx {
        #[inline(always)]
        fn default() -> RgnuAx {
            <crate::RegValueT<RgnuAx_SPEC> as RegisterValue<_>>::new(4294967264)
        }
    }
}
