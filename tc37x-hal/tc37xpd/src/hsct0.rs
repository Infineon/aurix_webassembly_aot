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
#[doc = r"HSCT"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsct0(pub(super) *mut u8);
unsafe impl core::marker::Send for Hsct0 {}
unsafe impl core::marker::Sync for Hsct0 {}
impl Hsct0 {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(65532usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Configuration Physical Layer Register\n resetvalue={Application Reset:0x1F0000}"]
    #[inline(always)]
    pub const fn configphy(&self) -> crate::common::Reg<self::Configphy_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }

    #[doc = "Clear To Send Control Register\n resetvalue={Application Reset:0x1}"]
    #[inline(always)]
    pub const fn ctsctrl(&self) -> crate::common::Reg<self::Ctsctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "Transmission Disable Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn disable(&self) -> crate::common::Reg<self::Disable_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0B4C002}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Interface Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ifctrl(&self) -> crate::common::Reg<self::Ifctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Interface Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ifstat(&self) -> crate::common::Reg<self::Ifstat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }

    #[doc = "Initialization Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn init(&self) -> crate::common::Reg<self::Init_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "Interrupt register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn irq(&self) -> crate::common::Reg<self::Irq_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "Interrupt Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn irqclr(&self) -> crate::common::Reg<self::Irqclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }

    #[doc = "Interrupt Enable Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn irqen(&self) -> crate::common::Reg<self::Irqen_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }

    #[doc = "Reset Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst0(&self) -> crate::common::Reg<self::Krst0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(65524usize)) }
    }

    #[doc = "Reset Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst1(&self) -> crate::common::Reg<self::Krst1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(65520usize)) }
    }

    #[doc = "Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krstclr(&self) -> crate::common::Reg<self::Krstclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(65516usize)) }
    }

    #[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(65512usize)) }
    }

    #[doc = "Sleep Control Register\n resetvalue={Application Reset:0x200000}"]
    #[inline(always)]
    pub const fn sleepctrl(&self) -> crate::common::Reg<self::Sleepctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }

    #[doc = "Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn stat(&self) -> crate::common::Reg<self::Stat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "STATPHY\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn statphy(&self) -> crate::common::Reg<self::Statphy_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "Test Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn testctrl(&self) -> crate::common::Reg<self::Testctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }

    #[doc = "Test Monitor Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn testmon(&self) -> crate::common::Reg<self::Testmon_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(100usize)) }
    }

    #[doc = "Unsolicited Status Message Received\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn usmr(&self) -> crate::common::Reg<self::Usmr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }

    #[doc = "Unsolicited Status Message Send\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn usms(&self) -> crate::common::Reg<self::Usms_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(84usize)) }
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
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, accen0::En0, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,accen0::En0, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, accen0::En1, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,accen0::En1, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, accen0::En2, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,accen0::En2, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, accen0::En3, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,accen0::En3, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, accen0::En4, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,accen0::En4, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, accen0::En5, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,accen0::En5, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, accen0::En6, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,accen0::En6, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, accen0::En7, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,accen0::En7, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, accen0::En8, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,accen0::En8, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, accen0::En9, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,accen0::En9, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, accen0::En10, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,accen0::En10, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, accen0::En11, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,accen0::En11, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, accen0::En12, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,accen0::En12, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, accen0::En13, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,accen0::En13, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, accen0::En14, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,accen0::En14, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, accen0::En15, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,accen0::En15, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, accen0::En16, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,accen0::En16, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, accen0::En17, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,accen0::En17, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, accen0::En18, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,accen0::En18, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, accen0::En19, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,accen0::En19, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, accen0::En20, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,accen0::En20, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, accen0::En21, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,accen0::En21, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, accen0::En22, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,accen0::En22, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, accen0::En23, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,accen0::En23, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, accen0::En24, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,accen0::En24, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, accen0::En25, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,accen0::En25, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, accen0::En26, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,accen0::En26, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, accen0::En27, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,accen0::En27, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, accen0::En28, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,accen0::En28, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, accen0::En29, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,accen0::En29, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, accen0::En30, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,accen0::En30, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read write access to the protected resources for transactions with the Master TAG ID n"]
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
pub struct Clc_SPEC;
impl crate::sealed::RegSpec for Clc_SPEC {
    type DataType = u32;
}
#[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
pub type Clc = crate::RegValueT<Clc_SPEC>;

impl Clc {
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control  clock and local access  of the module. This bit switches the clock off immediately  therefore the software has to take care that the HSSL and HSCT communication has been terminated properly before setting this bit. The disable is performed after an acknowledge signal from the core logic is received. The acknowledge signal is directly looped back from the request signal."]
    #[inline(always)]
    pub fn disr(self) -> crate::common::RegisterFieldBool<0, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Disable Status Bit   DISS. Bit indicates the current status of the module and is set after the requested disable is active."]
    #[inline(always)]
    pub fn diss(self) -> crate::common::RegisterFieldBool<1, 1, 0, Clc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Clc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Chip System Sleep Mode Control   EDIS. The EDIS bit in the CLC register controls whether or not a module is stopped during Chip System initiated Sleep Mode. If EDIS is 0  a Sleep Mode request can be recognized by the module and  when received  its clock is immediately shut off. Therefore the software has to take care that the HSSL and HSCT communication has been terminated properly before activating the sleep mode request. If EDIS is set to 1  a Sleep Mode request is disregarded by the module and the module continues its operation. Note  This chip system sleep mode has nothing to do with the HSCT protocol sleep mode."]
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
pub struct Configphy_SPEC;
impl crate::sealed::RegSpec for Configphy_SPEC {
    type DataType = u32;
}
#[doc = "Configuration Physical Layer Register\n resetvalue={Application Reset:0x1F0000}"]
pub type Configphy = crate::RegValueT<Configphy_SPEC>;

impl Configphy {
    #[doc = "Physical Layer Power On.   PON"]
    #[inline(always)]
    pub fn pon(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Configphy_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,Configphy_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Correlator phase enable   allows to enable disable each of the 5 Phase outputs separately.   CORCEN"]
    #[inline(always)]
    pub fn corcen(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Configphy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Configphy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Configphy {
    #[inline(always)]
    fn default() -> Configphy {
        <crate::RegValueT<Configphy_SPEC> as RegisterValue<_>>::new(2031616)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctsctrl_SPEC;
impl crate::sealed::RegSpec for Ctsctrl_SPEC {
    type DataType = u32;
}
#[doc = "Clear To Send Control Register\n resetvalue={Application Reset:0x1}"]
pub type Ctsctrl = crate::RegValueT<Ctsctrl_SPEC>;

impl Ctsctrl {
    #[doc = "Transmit CTS Frame Generation   CTS FRAME. Dedicated CTS frames are generated after the receive data path is not        able to accept more data. The situation is indicated by the CTS header        bit in case there is currently no data to be transferred."]
    #[inline(always)]
    pub fn cts_frame(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        ctsctrl::CtsFrame,
        Ctsctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            ctsctrl::CtsFrame,
            Ctsctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Disable TX CTS signaling   CTS TXD. If this bit is set to 1  CTS signaling is not performed at the interface        and the status remains at the clear to send for every frame send."]
    #[inline(always)]
    pub fn cts_txd(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ctsctrl::CtsTxd, Ctsctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,ctsctrl::CtsTxd, Ctsctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disable RX CTS detection   CTS RXD. If this bit is set to 1  CTS detection is not performed at the receiver        and the status remains internally at clear to send for every frame        received."]
    #[inline(always)]
    pub fn cts_rxd(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ctsctrl::CtsRxd, Ctsctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,ctsctrl::CtsRxd, Ctsctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disable HSSL interface CTS Frame Blocking   HSSL CTS FBD. If this bit is set to 1  CTS signaling is not performed at the interface        and the status remains at the clear to send for every frame send."]
    #[inline(always)]
    pub fn hssl_cts_fbd(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        ctsctrl::HsslCtsFbd,
        Ctsctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            ctsctrl::HsslCtsFbd,
            Ctsctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Ctsctrl {
    #[inline(always)]
    fn default() -> Ctsctrl {
        <crate::RegValueT<Ctsctrl_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod ctsctrl {
    pub struct CtsFrame_SPEC;
    pub type CtsFrame = crate::EnumBitfieldStruct<u8, CtsFrame_SPEC>;
    impl CtsFrame {
        #[doc = "0 Generation of dedicated CTS frames disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Generation of dedicated CTS frames enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct CtsTxd_SPEC;
    pub type CtsTxd = crate::EnumBitfieldStruct<u8, CtsTxd_SPEC>;
    impl CtsTxd {
        #[doc = "0 Enable CTS signaling  default ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Disable CTS signaling."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct CtsRxd_SPEC;
    pub type CtsRxd = crate::EnumBitfieldStruct<u8, CtsRxd_SPEC>;
    impl CtsRxd {
        #[doc = "0 Enable CTS detection  default ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Disable CTS detection."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct HsslCtsFbd_SPEC;
    pub type HsslCtsFbd = crate::EnumBitfieldStruct<u8, HsslCtsFbd_SPEC>;
    impl HsslCtsFbd {
        #[doc = "0 Enable CTS frame blocking  default ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Disable CTS frame blocking."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Disable_SPEC;
impl crate::sealed::RegSpec for Disable_SPEC {
    type DataType = u32;
}
#[doc = "Transmission Disable Register\n resetvalue={Application Reset:0x3}"]
pub type Disable = crate::RegValueT<Disable_SPEC>;

impl Disable {
    #[doc = "Disable HSCT Transmit path in Master interface   TX DIS. Disable the transmit path of the HSCT interface. If this bit is set to 1          no transfer can be initiated and the LVDS driver is disabled."]
    #[inline(always)]
    pub fn tx_dis(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, disable::TxDis, Disable_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,disable::TxDis, Disable_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disable HSCT Receive path in Master interface   RX DIS. Disable the receive line path of the HSCT interface. If this bit is set        to 1   no transfer from the other side can be received and the Master RX        path is in a low power state. This feature is only available in the Master interface. Slave interface        RX path can not be disabled and a write to the register has no effect."]
    #[inline(always)]
    pub fn rx_dis(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, disable::RxDis, Disable_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,disable::RxDis, Disable_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disable RX Header Error Discard Payload data.   RX HEPD. Instead of discarding the Payload data at a header error the payload        data is passed to the higher layer  HSSL . Only channel data to HSSL is        affected. This function is available in Master and in Slave mode."]
    #[inline(always)]
    pub fn rx_hepd(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, disable::RxHepd, Disable_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,disable::RxHepd, Disable_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Disable {
    #[inline(always)]
    fn default() -> Disable {
        <crate::RegValueT<Disable_SPEC> as RegisterValue<_>>::new(3)
    }
}
pub mod disable {
    pub struct TxDis_SPEC;
    pub type TxDis = crate::EnumBitfieldStruct<u8, TxDis_SPEC>;
    impl TxDis {
        #[doc = "0 Enable"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Disable"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct RxDis_SPEC;
    pub type RxDis = crate::EnumBitfieldStruct<u8, RxDis_SPEC>;
    impl RxDis {
        #[doc = "0 Enable"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Disable"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct RxHepd_SPEC;
    pub type RxHepd = crate::EnumBitfieldStruct<u8, RxHepd_SPEC>;
    impl RxHepd {
        #[doc = "0 Header error received data is discarded"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Header error received data is passed to the higher hardware layer."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0B4C002}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MOD REV"]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, id::ModRev, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,id::ModRev, Id_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Module Number Type   MOD TYPE"]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, id::ModType, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,id::ModType, Id_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Module Number for module identification   MOD NUM"]
    #[inline(always)]
    pub fn mod_num(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, id::ModNum, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,id::ModNum, Id_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Id {
    #[inline(always)]
    fn default() -> Id {
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(11845634)
    }
}
pub mod id {
    pub struct ModRev_SPEC;
    pub type ModRev = crate::EnumBitfieldStruct<u8, ModRev_SPEC>;
    impl ModRev {
        #[doc = "02 The value of a module revision starts with 02 H  second revision ."]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct ModType_SPEC;
    pub type ModType = crate::EnumBitfieldStruct<u8, ModType_SPEC>;
    impl ModType {
        #[doc = "C0 32 bit peripheral"]
        pub const CONST_192192: Self = Self::new(192);
    }
    pub struct ModNum_SPEC;
    pub type ModNum = crate::EnumBitfieldStruct<u8, ModNum_SPEC>;
    impl ModNum {
        #[doc = "00B4 Is the module identification number for the HSCT interface."]
        pub const CONST_180180: Self = Self::new(180);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifctrl_SPEC;
impl crate::sealed::RegSpec for Ifctrl_SPEC {
    type DataType = u32;
}
#[doc = "Interface Control Register\n resetvalue={Application Reset:0x0}"]
pub type Ifctrl = crate::RegValueT<Ifctrl_SPEC>;

impl Ifctrl {
    #[doc = "Master Mode   Trigger for Interface Control Value to be send to Slave interface   IFCVS. See the table  quot Interface Control Payload Values quot . Master IF Mode  The value is taken as control frame value send as        payload to the Slave IF. Slave IF Mode  The value is a new configuration of the Slave IF  not        recommended flow    ."]
    #[inline(always)]
    pub fn ifcvs(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Ifctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Ifctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Master Mode   Slave IF control frame trigger   SIFCV. Changing the interface configuration  software must guarantee not having        transfers active on the interface. This register bit always reads back zero."]
    #[inline(always)]
    pub fn sifcv(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, ifctrl::Sifcv, Ifctrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<8,0x1,1,0,ifctrl::Sifcv, Ifctrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Master Mode RX speed   MRXSPEED. Register setting only valid in interface Master mode."]
    #[inline(always)]
    pub fn mrxspeed(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, ifctrl::Mrxspeed, Ifctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            ifctrl::Mrxspeed,
            Ifctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Master Mode TX speed   MTXSPEED. Register setting only valid in interface Master mode."]
    #[inline(always)]
    pub fn mtxspeed(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, ifctrl::Mtxspeed, Ifctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            18,
            0x3,
            1,
            0,
            ifctrl::Mtxspeed,
            Ifctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Interface TX Test Mode   IFTESTMD"]
    #[inline(always)]
    pub fn iftestmd(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, ifctrl::Iftestmd, Ifctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            ifctrl::Iftestmd,
            Ifctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Ifctrl {
    #[inline(always)]
    fn default() -> Ifctrl {
        <crate::RegValueT<Ifctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ifctrl {
    pub struct Sifcv_SPEC;
    pub type Sifcv = crate::EnumBitfieldStruct<u8, Sifcv_SPEC>;
    impl Sifcv {
        #[doc = "0 The IFCVS field configured value has no effect  default ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Writing a one to the register bit sends the control frame configured in register field IFCVS  if the interface is configured as Master. In Slave Mode the trigger has an effect and takes the IFCTRL.IFCVS value. In Slave mode this is not the recommended control flow. A frame based configuration shall be used instead. A control frame has higher priority then a register control. At a simultaneous occurrence of both configuration source the register configuration in Slave mode is lost."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mrxspeed_SPEC;
    pub type Mrxspeed = crate::EnumBitfieldStruct<u8, Mrxspeed_SPEC>;
    impl Mrxspeed {
        #[doc = "00 Low Speed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Medium Speed"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 High Speed"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 For future use"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Mtxspeed_SPEC;
    pub type Mtxspeed = crate::EnumBitfieldStruct<u8, Mtxspeed_SPEC>;
    impl Mtxspeed {
        #[doc = "00 Low Speed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 for future use"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 High Speed"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 For future use"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Iftestmd_SPEC;
    pub type Iftestmd = crate::EnumBitfieldStruct<u8, Iftestmd_SPEC>;
    impl Iftestmd {
        #[doc = "0 Test Mode disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Test Mode enabled   send out 101010101  test pattern continuously."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifstat_SPEC;
impl crate::sealed::RegSpec for Ifstat_SPEC {
    type DataType = u32;
}
#[doc = "Interface Status Register\n resetvalue={Application Reset:0x0}"]
pub type Ifstat = crate::RegValueT<Ifstat_SPEC>;

impl Ifstat {
    #[doc = "HSCT Slave interface Status for RX link   RX STAT. Slave interface transmitter only"]
    #[inline(always)]
    pub fn rx_stat(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, ifstat::RxStat, Ifstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x7,1,0,ifstat::RxStat, Ifstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "HSCT Slave interface Status for TX link   TX STAT. Slave interface receiver only"]
    #[inline(always)]
    pub fn tx_stat(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, ifstat::TxStat, Ifstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<3,0x3,1,0,ifstat::TxStat, Ifstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "HSCT LVDS Slave interface TX enable   TX EN. Slave interface only"]
    #[inline(always)]
    pub fn tx_en(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, ifstat::TxEn, Ifstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<5,0x1,1,0,ifstat::TxEn, Ifstat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Ifstat {
    #[inline(always)]
    fn default() -> Ifstat {
        <crate::RegValueT<Ifstat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ifstat {
    pub struct RxStat_SPEC;
    pub type RxStat = crate::EnumBitfieldStruct<u8, RxStat_SPEC>;
    impl RxStat {
        #[doc = "000 Interface is        disabled in RX link direction."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 Interface        runs at low speed on RX link."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 Interface        runs at medium speed on RX link."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 Interface        runs at high speed on RX link direction."]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "101 Clock test mode and low speed on RX ink"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 Clock test mode and medium speed on RX link"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "111 Clock test mode and high speed on RX link."]
        pub const CONST_77: Self = Self::new(7);
    }
    pub struct TxStat_SPEC;
    pub type TxStat = crate::EnumBitfieldStruct<u8, TxStat_SPEC>;
    impl TxStat {
        #[doc = "00 Interface runs at low speed on TX link."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Interface runs at high speed on TX link."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Loopback mode low speed."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Loopback mode high speed."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct TxEn_SPEC;
    pub type TxEn = crate::EnumBitfieldStruct<u8, TxEn_SPEC>;
    impl TxEn {
        #[doc = "0 LVDS TX disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 LVDS TX enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Init_SPEC;
impl crate::sealed::RegSpec for Init_SPEC {
    type DataType = u32;
}
#[doc = "Initialization Register\n resetvalue={Application Reset:0x0}"]
pub type Init = crate::RegValueT<Init_SPEC>;

impl Init {
    #[doc = "Enable HSCT SysClk in Master interface   SYS CLK EN. SysClk enable activates the SysClk. This feature is only available in the Master interface. In Slave interface mode the register setting does not have an effect."]
    #[inline(always)]
    pub fn sys_clk_en(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, init::SysClkEn, Init_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,init::SysClkEn, Init_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Interface Mode   IFM. Select the Interface Mode  Master IF or Slave IF ."]
    #[inline(always)]
    pub fn ifm(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, init::Ifm, Init_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,init::Ifm, Init_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Reference Clock Frequency Divider   SRCF. Defines physical layer reference frequency  PHY CLK  and respective        input frequency divider. The configuration is valid for Low speed and        Medium Speed mode."]
    #[inline(always)]
    pub fn srcf(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, init::Srcf, Init_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,init::Srcf, Init_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select SysClk Frequency Divider   SSCF. For master interface defines SysClk pad output frequency. The allowed        SysClk frequency is 10MHz or 20MHz."]
    #[inline(always)]
    pub fn sscf(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, init::Sscf, Init_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,init::Sscf, Init_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmit High Speed Divider.   TXHD. The Transmit High Speed data rate can be reduced by dividing factors.        The Transmit High Speed data rate is separated from the Receive High        Speed data rate."]
    #[inline(always)]
    pub fn txhd(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, init::Txhd, Init_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,init::Txhd, Init_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive High Speed Divider.   RXHD. The Receive High Speed data rate can be reduced by dividing factors. The Receive High Speed data rate is separated from the Transmit High Speed data rate. For future use configuration leads to no output clock  pll phase o lt 4 0 gt   delivered to the correlator."]
    #[inline(always)]
    pub fn rxhd(
        self,
    ) -> crate::common::RegisterField<19, 0x7, 1, 0, init::Rxhd, Init_SPEC, crate::common::RW> {
        crate::common::RegisterField::<19,0x7,1,0,init::Rxhd, Init_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Init {
    #[inline(always)]
    fn default() -> Init {
        <crate::RegValueT<Init_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod init {
    pub struct SysClkEn_SPEC;
    pub type SysClkEn = crate::EnumBitfieldStruct<u8, SysClkEn_SPEC>;
    impl SysClkEn {
        #[doc = "0 Disable  default"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ifm_SPEC;
    pub type Ifm = crate::EnumBitfieldStruct<u8, Ifm_SPEC>;
    impl Ifm {
        #[doc = "0 Master IF"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Slave IF"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Srcf_SPEC;
    pub type Srcf = crate::EnumBitfieldStruct<u8, Srcf_SPEC>;
    impl Srcf {
        #[doc = "00 REFCLK 10MHz  LS 5MBaud  MS n.a."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 REFCLK 20MHz  LS 5MBaud  MS 20MBaud"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 REFCLK 40MHz  LS 5MBaud  MS 20MBaud"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 For future use"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sscf_SPEC;
    pub type Sscf = crate::EnumBitfieldStruct<u8, Sscf_SPEC>;
    impl Sscf {
        #[doc = "00 SYSCLK DIV 1 1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 SYSCLK DIV 1 2"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 SYSCLK DIV 1 4"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 For future use"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Txhd_SPEC;
    pub type Txhd = crate::EnumBitfieldStruct<u8, Txhd_SPEC>;
    impl Txhd {
        #[doc = "000 Divider 1 1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 Divider 1 2"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 Divider 1 4"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 Divider 1 8"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 Divider 1 16"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 For future use."]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 For future use."]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "111 For future use."]
        pub const CONST_77: Self = Self::new(7);
    }
    pub struct Rxhd_SPEC;
    pub type Rxhd = crate::EnumBitfieldStruct<u8, Rxhd_SPEC>;
    impl Rxhd {
        #[doc = "000 Divider 1 1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 Divider 1 2"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 Divider 1 4"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 Divider 1 8"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 Divider 1 16"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 For future use."]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 For future use."]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "111 For future use."]
        pub const CONST_77: Self = Self::new(7);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irq_SPEC;
impl crate::sealed::RegSpec for Irq_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt register\n resetvalue={Application Reset:0x0}"]
pub type Irq = crate::RegValueT<Irq_SPEC>;

impl Irq {
    #[doc = "Header error detected   HER. Not supported size  Received command at Slave interface 8 bit size only   other command          sizes generate an error. Received command ping answer at Master interface 32 bit size only            other command sizes generate an error Unsolicited data 32 bit only Logical data channel size 8 bit Not supported logical channel type  0b1xxx 0010  Slave interface control and Slave interface read"]
    #[inline(always)]
    pub fn her(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, irq::Her, Irq_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,irq::Her, Irq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Payload error detected   PYER. Payload does not fit the header size"]
    #[inline(always)]
    pub fn pyer(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, irq::Pyer, Irq_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,irq::Pyer, Irq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "HSCT command error   CER. Control command not valid or control payload size bigger than 8 bit.        Single Slave specific commands received in Multi Slave Mode do not        trigger CER error  but only MSCE error."]
    #[inline(always)]
    pub fn cer(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, irq::Cer, Irq_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1,1,0,irq::Cer, Irq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "HSCT interface control frame send   IFCFS. The scheduled interface control command is send."]
    #[inline(always)]
    pub fn ifcfs(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, irq::Ifcfs, Irq_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x1,1,0,irq::Ifcfs, Irq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Speed Mode Switch Error  Master Mode only    SMER. Speed mode change did not work. Received PING payload not valid."]
    #[inline(always)]
    pub fn smer(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, irq::Smer, Irq_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x1,1,0,irq::Smer, Irq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Unsolicited message frame send finished   USMSF. Interrupt is indicated after the unsolicited message send is finished."]
    #[inline(always)]
    pub fn usmsf(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, irq::Usmsf, Irq_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x1,1,0,irq::Usmsf, Irq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PLL lost lock error   PLER. After the PLL locked  the PLL may loose lock  which is reflected by the        error"]
    #[inline(always)]
    pub fn pler(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, irq::Pler, Irq_SPEC, crate::common::R> {
        crate::common::RegisterField::<7,0x1,1,0,irq::Pler, Irq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Unsolicited Message Received   USM. Unsolicited message received indication. Unsolicited message indicates a        system event to the other interface side."]
    #[inline(always)]
    pub fn usm(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, irq::Usm, Irq_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x1,1,0,irq::Usm, Irq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "PING Answer Received   PAR. The received message was identified as PING."]
    #[inline(always)]
    pub fn par(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, irq::Par, Irq_SPEC, crate::common::R> {
        crate::common::RegisterField::<9,0x1,1,0,irq::Par, Irq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "TX transfer error occurred on a disabled TX channel.   TXTE. A disabled TX triggers an error interrupt  if  TX disabled on a pending or active data transfer. TX CTS configuration change on a active CTS frame."]
    #[inline(always)]
    pub fn txte(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, irq::Txte, Irq_SPEC, crate::common::R> {
        crate::common::RegisterField::<10,0x1,1,0,irq::Txte, Irq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Synchronization FIFO overflow  in RX direction    SFO. Physical layer to Controller data synchronization FIFO in RX transfer        direction hit an overflow situation. This interrupt is an indication about a to slow SRI clock compared to          Physical layer clock  which results in a overflow situation.  Minimum          SRI frequency 40 MHz."]
    #[inline(always)]
    pub fn sfo(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, irq::Sfo, Irq_SPEC, crate::common::R> {
        crate::common::RegisterField::<11,0x1,1,0,irq::Sfo, Irq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Synchronization FIFO underflow  in TX direction    SFU. Controller to Physical layer data synchronization FIFO in TX transfer        direction hit an underflow situation. This interrupt is an indication about a to slow SRI clock compared to          Physical layer clock  which results in a underflow situation.  Minimum          SRI frequency 40 MHz."]
    #[inline(always)]
    pub fn sfu(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, irq::Sfu, Irq_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x1,1,0,irq::Sfu, Irq_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Multi Slave scenario Command Error   MSCE. This interrupt indicates a control command which is not allowed in multi        Slave scenario. In Master and Slave mode a not allowed command results        to an error."]
    #[inline(always)]
    pub fn msce(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, irq::Msce, Irq_SPEC, crate::common::R> {
        crate::common::RegisterField::<13,0x1,1,0,irq::Msce, Irq_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Irq {
    #[inline(always)]
    fn default() -> Irq {
        <crate::RegValueT<Irq_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod irq {
    pub struct Her_SPEC;
    pub type Her = crate::EnumBitfieldStruct<u8, Her_SPEC>;
    impl Her {
        #[doc = "0 Header OK"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Header issue detected."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pyer_SPEC;
    pub type Pyer = crate::EnumBitfieldStruct<u8, Pyer_SPEC>;
    impl Pyer {
        #[doc = "0 Payload size OK"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Received payload size wrong"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cer_SPEC;
    pub type Cer = crate::EnumBitfieldStruct<u8, Cer_SPEC>;
    impl Cer {
        #[doc = "0 No command error"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 HSCT command error"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ifcfs_SPEC;
    pub type Ifcfs = crate::EnumBitfieldStruct<u8, Ifcfs_SPEC>;
    impl Ifcfs {
        #[doc = "0 No interface control command send"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interface control command send."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Smer_SPEC;
    pub type Smer = crate::EnumBitfieldStruct<u8, Smer_SPEC>;
    impl Smer {
        #[doc = "0 Interfaces runs at defined speed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Ping payload error"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Usmsf_SPEC;
    pub type Usmsf = crate::EnumBitfieldStruct<u8, Usmsf_SPEC>;
    impl Usmsf {
        #[doc = "0 No unsolicited message send."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Unsolicited message send finished."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pler_SPEC;
    pub type Pler = crate::EnumBitfieldStruct<u8, Pler_SPEC>;
    impl Pler {
        #[doc = "0 PLL lock"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PLL lock loss"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Usm_SPEC;
    pub type Usm = crate::EnumBitfieldStruct<u8, Usm_SPEC>;
    impl Usm {
        #[doc = "0 no unsolicited message available"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 unsolicited message available"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Par_SPEC;
    pub type Par = crate::EnumBitfieldStruct<u8, Par_SPEC>;
    impl Par {
        #[doc = "0 No PING message available"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PING message received"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Txte_SPEC;
    pub type Txte = crate::EnumBitfieldStruct<u8, Txte_SPEC>;
    impl Txte {
        #[doc = "0 No error situation occurred"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error situation occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sfo_SPEC;
    pub type Sfo = crate::EnumBitfieldStruct<u8, Sfo_SPEC>;
    impl Sfo {
        #[doc = "0 RX synchronization is running well."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 RX FIFO overflow"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sfu_SPEC;
    pub type Sfu = crate::EnumBitfieldStruct<u8, Sfu_SPEC>;
    impl Sfu {
        #[doc = "0 TX synchronization is running well."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 TX FIFO underflow"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msce_SPEC;
    pub type Msce = crate::EnumBitfieldStruct<u8, Msce_SPEC>;
    impl Msce {
        #[doc = "0 No multi Slave scenario command error."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Multi Slave scenario command error  not allowed command used"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqclr_SPEC;
impl crate::sealed::RegSpec for Irqclr_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Clear Register\n resetvalue={Application Reset:0x0}"]
pub type Irqclr = crate::RegValueT<Irqclr_SPEC>;

impl Irqclr {
    #[doc = "Header error detected interrupt clear   HERCLR"]
    #[inline(always)]
    pub fn herclr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Payload error detected interrupt clear   PYERCLR"]
    #[inline(always)]
    pub fn pyerclr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "HSCT command error interrupt clear   CERCLR"]
    #[inline(always)]
    pub fn cerclr(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "HSCT interface control command send interrupt clear   IFCFSCLR"]
    #[inline(always)]
    pub fn ifcfsclr(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Speed Mode Switch Error interrupt clear   SMERCLR"]
    #[inline(always)]
    pub fn smerclr(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Unsolicited message frame send finished interrupt clear   USMSFCLR"]
    #[inline(always)]
    pub fn usmsfclr(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "PLL lost lock error interrupt clear   PLERCLR"]
    #[inline(always)]
    pub fn plerclr(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Unsolicited Message received clear   USMCLR"]
    #[inline(always)]
    pub fn usmclr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "PING Answer received clear   PARCLR"]
    #[inline(always)]
    pub fn parclr(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "TX disable error interrupt clear   TXTECLR"]
    #[inline(always)]
    pub fn txteclr(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Synchronization FIFO overflow  in RX direction  interrupt clear   SFOCLR"]
    #[inline(always)]
    pub fn sfoclr(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Synchronization FIFO underflow  in TX direction  interrupt clear   SFUCLR"]
    #[inline(always)]
    pub fn sfuclr(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Multi Slave scenario Command Error interrupt clear   MSCELR"]
    #[inline(always)]
    pub fn mscelr(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Irqclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<13, 1, 0, Irqclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Irqclr {
    #[inline(always)]
    fn default() -> Irqclr {
        <crate::RegValueT<Irqclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irqen_SPEC;
impl crate::sealed::RegSpec for Irqen_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Enable Register\n resetvalue={Application Reset:0x0}"]
pub type Irqen = crate::RegValueT<Irqen_SPEC>;

impl Irqen {
    #[doc = "Header error detected interrupt enable   HEREN"]
    #[inline(always)]
    pub fn heren(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, irqen::Heren, Irqen_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,irqen::Heren, Irqen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Payload error detected interrupt enable   PYEREN"]
    #[inline(always)]
    pub fn pyeren(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, irqen::Pyeren, Irqen_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,irqen::Pyeren, Irqen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSCT command error interrupt enable   CEREN"]
    #[inline(always)]
    pub fn ceren(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, irqen::Ceren, Irqen_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,irqen::Ceren, Irqen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "HSCT interface control command send enable   IFCFSEN"]
    #[inline(always)]
    pub fn ifcfsen(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, irqen::Ifcfsen, Irqen_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,irqen::Ifcfsen, Irqen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Speed Mode Switch Error interrupt enable   SMEREN"]
    #[inline(always)]
    pub fn smeren(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, irqen::Smeren, Irqen_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,irqen::Smeren, Irqen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Unsolicited message frame send finished   USMSFEN"]
    #[inline(always)]
    pub fn usmsfen(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, irqen::Usmsfen, Irqen_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,irqen::Usmsfen, Irqen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PLL lost lock error interrupt enable   PLEREN"]
    #[inline(always)]
    pub fn pleren(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, irqen::Pleren, Irqen_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,irqen::Pleren, Irqen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Unsolicited Message received enable   USMEN"]
    #[inline(always)]
    pub fn usmen(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, irqen::Usmen, Irqen_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,irqen::Usmen, Irqen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PING Answer Received enable   PAREN"]
    #[inline(always)]
    pub fn paren(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, irqen::Paren, Irqen_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,irqen::Paren, Irqen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TX disable error interrupt enable   TXTEEN"]
    #[inline(always)]
    pub fn txteen(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, irqen::Txteen, Irqen_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,irqen::Txteen, Irqen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Synchronization FIFO overflow  in RX direction  interrupt enable   SFOEN"]
    #[inline(always)]
    pub fn sfoen(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, irqen::Sfoen, Irqen_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,irqen::Sfoen, Irqen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Synchronization FIFO underflow  in TX direction  interrupt enable   SFUEN"]
    #[inline(always)]
    pub fn sfuen(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, irqen::Sfuen, Irqen_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,irqen::Sfuen, Irqen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Multi Slave scenario Command Error interrupt enable   MSCEEN"]
    #[inline(always)]
    pub fn msceen(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, irqen::Msceen, Irqen_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,irqen::Msceen, Irqen_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Irqen {
    #[inline(always)]
    fn default() -> Irqen {
        <crate::RegValueT<Irqen_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod irqen {
    pub struct Heren_SPEC;
    pub type Heren = crate::EnumBitfieldStruct<u8, Heren_SPEC>;
    impl Heren {
        #[doc = "0 Interrupt disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pyeren_SPEC;
    pub type Pyeren = crate::EnumBitfieldStruct<u8, Pyeren_SPEC>;
    impl Pyeren {
        #[doc = "0 Interrupt disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ceren_SPEC;
    pub type Ceren = crate::EnumBitfieldStruct<u8, Ceren_SPEC>;
    impl Ceren {
        #[doc = "0 Interrupt disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ifcfsen_SPEC;
    pub type Ifcfsen = crate::EnumBitfieldStruct<u8, Ifcfsen_SPEC>;
    impl Ifcfsen {
        #[doc = "0 Interrupt disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Smeren_SPEC;
    pub type Smeren = crate::EnumBitfieldStruct<u8, Smeren_SPEC>;
    impl Smeren {
        #[doc = "0 Interrupt disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Usmsfen_SPEC;
    pub type Usmsfen = crate::EnumBitfieldStruct<u8, Usmsfen_SPEC>;
    impl Usmsfen {
        #[doc = "0 Interrupt disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pleren_SPEC;
    pub type Pleren = crate::EnumBitfieldStruct<u8, Pleren_SPEC>;
    impl Pleren {
        #[doc = "0 Interrupt disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Usmen_SPEC;
    pub type Usmen = crate::EnumBitfieldStruct<u8, Usmen_SPEC>;
    impl Usmen {
        #[doc = "0 Interrupt disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Paren_SPEC;
    pub type Paren = crate::EnumBitfieldStruct<u8, Paren_SPEC>;
    impl Paren {
        #[doc = "0 Interrupt disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Txteen_SPEC;
    pub type Txteen = crate::EnumBitfieldStruct<u8, Txteen_SPEC>;
    impl Txteen {
        #[doc = "0 Interrupt disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sfoen_SPEC;
    pub type Sfoen = crate::EnumBitfieldStruct<u8, Sfoen_SPEC>;
    impl Sfoen {
        #[doc = "0 Interrupt disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sfuen_SPEC;
    pub type Sfuen = crate::EnumBitfieldStruct<u8, Sfuen_SPEC>;
    impl Sfuen {
        #[doc = "0 Interrupt disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Msceen_SPEC;
    pub type Msceen = crate::EnumBitfieldStruct<u8, Msceen_SPEC>;
    impl Msceen {
        #[doc = "0 Interrupt disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krst0_SPEC;
impl crate::sealed::RegSpec for Krst0_SPEC {
    type DataType = u32;
}
#[doc = "Reset Register 0\n resetvalue={Application Reset:0x0}"]
pub type Krst0 = crate::RegValueT<Krst0_SPEC>;

impl Krst0 {
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel        reset will be executed if the reset bits of both kernel registers are        set. The RST bit will be cleared  re set to   180 0  180   by the BPI FPI after the        kernel reset was executed. It is strongly recommended to reset the HSCT and HSSL in sequence to avoid communication issues."]
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
#[doc = "Reset Register 1\n resetvalue={Application Reset:0x0}"]
pub type Krst1 = crate::RegValueT<Krst1_SPEC>;

impl Krst1 {
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel        reset will be executed if the reset bits of both kernel reset registers        is set. The RST bit will be cleared  re set to   180 0  180   by the BPI FPI after the        kernel reset was executed. It is strongly recommended to reset the HSCT and HSSL in sequence to avoid communication issues."]
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
#[doc = "Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
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
#[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0}"]
pub type Ocs = crate::RegValueT<Ocs_SPEC>;

impl Ocs {
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
    pub struct Tgb_SPEC;
    pub type Tgb = crate::EnumBitfieldStruct<u8, Tgb_SPEC>;
    impl Tgb {
        #[doc = "0 Trigger Set is output on OTGB0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Trigger Set is output on OTGB1"]
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
pub struct Sleepctrl_SPEC;
impl crate::sealed::RegSpec for Sleepctrl_SPEC {
    type DataType = u32;
}
#[doc = "Sleep Control Register\n resetvalue={Application Reset:0x200000}"]
pub type Sleepctrl = crate::RegValueT<Sleepctrl_SPEC>;

impl Sleepctrl {
    #[doc = "Sleep mode enabled   SLPEN. Sleep mode is enabled and performed after receiving a 1 at the end of a        received frame or in transmission direction  if no more data to be send."]
    #[inline(always)]
    pub fn slpen(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        sleepctrl::Slpen,
        Sleepctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            sleepctrl::Slpen,
            Sleepctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Clock Gating in Sleep Mode   SLPCLKG. In sleep mode the clock for HSCT  framer  deframer  can be gated in        order to minimize power consumption. Clock gating  Receiving path and transmitting path is separated."]
    #[inline(always)]
    pub fn slpclkg(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        sleepctrl::Slpclkg,
        Sleepctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            sleepctrl::Slpclkg,
            Sleepctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Counter Value for Determining the Wake up Time of the LVDS Line Driver   WKUPCNT. This counter value corresponds to wake up time the LVDS requires from        sleep to wake up. Counter is clocked by SRI clock."]
    #[inline(always)]
    pub fn wkupcnt(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Sleepctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Sleepctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Sleepctrl {
    #[inline(always)]
    fn default() -> Sleepctrl {
        <crate::RegValueT<Sleepctrl_SPEC> as RegisterValue<_>>::new(2097152)
    }
}
pub mod sleepctrl {
    pub struct Slpen_SPEC;
    pub type Slpen = crate::EnumBitfieldStruct<u8, Slpen_SPEC>;
    impl Slpen {
        #[doc = "0 Sleep mode disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Sleep mode enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Slpclkg_SPEC;
    pub type Slpclkg = crate::EnumBitfieldStruct<u8, Slpclkg_SPEC>;
    impl Slpclkg {
        #[doc = "0 Clock gating in sleep mode disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clock gating in sleep mode enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat_SPEC;
impl crate::sealed::RegSpec for Stat_SPEC {
    type DataType = u32;
}
#[doc = "Status Register\n resetvalue={Application Reset:0x0}"]
pub type Stat = crate::RegValueT<Stat_SPEC>;

impl Stat {
    #[doc = "RX  Receiving  Payload Size   RX PSIZE. Contains the payload size of the last received frame."]
    #[inline(always)]
    pub fn rx_psize(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Stat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0x7, 1, 0, u8, Stat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RX  Receiving  Logical Channel Type   RX CHANNEL. Contains the logical channel type of the last received frame. See Table         quot Logical Channel Type Coding quot ."]
    #[inline(always)]
    pub fn rx_channel(
        self,
    ) -> crate::common::RegisterField<3, 0xf, 1, 0, u8, Stat_SPEC, crate::common::R> {
        crate::common::RegisterField::<3, 0xf, 1, 0, u8, Stat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RX  Receiving  Sleep Mode Status   RX SLEEP"]
    #[inline(always)]
    pub fn rx_sleep(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, stat::RxSleep, Stat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<7,0x1,1,0,stat::RxSleep, Stat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "TX  Transmission  Sleep Mode Status   TX SLEEP"]
    #[inline(always)]
    pub fn tx_sleep(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, stat::TxSleep, Stat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0x1,1,0,stat::TxSleep, Stat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmission Payload Size   TX PSIZE. Coding of the logical channel type is according to the Table  Payload        Size Coding. This value was used in the logical channel type field in        the header for the actual transfer in transmit direction."]
    #[inline(always)]
    pub fn tx_psize(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, Stat_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x7,1,0,u8, Stat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmission Logical Channel Type   TX CHANNEL TYPE. Coding of the logical channel type is according to the Table  Logical        Channel Type Coding. This value was used in the logical channel type        field in the header for the actual transfer in transmit direction."]
    #[inline(always)]
    pub fn tx_channel_type(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Stat_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Stat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Last Interface Control Command Received   LIFCCMDR. The bit value reflects the last control command received. The bit is        active in Slave interface mode only. In Master mode it reflects logic 0        always.  See CROSSREFERENCE"]
    #[inline(always)]
    pub fn lifccmdr(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Stat_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Stat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        <crate::RegValueT<Stat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod stat {
    pub struct RxSleep_SPEC;
    pub type RxSleep = crate::EnumBitfieldStruct<u8, RxSleep_SPEC>;
    impl RxSleep {
        #[doc = "0 HSCT is in receive direction active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 HSCT is in receive direction in sleep mode"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct TxSleep_SPEC;
    pub type TxSleep = crate::EnumBitfieldStruct<u8, TxSleep_SPEC>;
    impl TxSleep {
        #[doc = "0 HSCT is in transmit direction active"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 HSCT is in transmit direction in sleep mode"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Statphy_SPEC;
impl crate::sealed::RegSpec for Statphy_SPEC {
    type DataType = u32;
}
#[doc = "STATPHY\n resetvalue={Application Reset:0x0}"]
pub type Statphy = crate::RegValueT<Statphy_SPEC>;

impl Statphy {
    #[doc = "PLL locked   PLOCK"]
    #[inline(always)]
    pub fn plock(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, statphy::Plock, Statphy_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,statphy::Plock, Statphy_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmitter speed   TXSA"]
    #[inline(always)]
    pub fn txsa(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, statphy::Txsa, Statphy_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x3,1,0,statphy::Txsa, Statphy_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receiver speed   RXSA"]
    #[inline(always)]
    pub fn rxsa(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, statphy::Rxsa, Statphy_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x3,1,0,statphy::Rxsa, Statphy_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Statphy {
    #[inline(always)]
    fn default() -> Statphy {
        <crate::RegValueT<Statphy_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod statphy {
    pub struct Plock_SPEC;
    pub type Plock = crate::EnumBitfieldStruct<u8, Plock_SPEC>;
    impl Plock {
        #[doc = "0 PLL out of lock  default"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PLL locked"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Txsa_SPEC;
    pub type Txsa = crate::EnumBitfieldStruct<u8, Txsa_SPEC>;
    impl Txsa {
        #[doc = "00 Transmitter in Low speed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Transmitter in Medium speed"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Transmitter in High speed"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Rxsa_SPEC;
    pub type Rxsa = crate::EnumBitfieldStruct<u8, Rxsa_SPEC>;
    impl Rxsa {
        #[doc = "00 Receiver in Low speed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Receiver in Medium speed"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Receiver in High speed"]
        pub const CONST_22: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Testctrl_SPEC;
impl crate::sealed::RegSpec for Testctrl_SPEC {
    type DataType = u32;
}
#[doc = "Test Control Register\n resetvalue={Application Reset:0x0}"]
pub type Testctrl = crate::RegValueT<Testctrl_SPEC>;

impl Testctrl {
    #[doc = "Enable Slave TX path  Slave interface mode only    TXENS. This function should be only used during interface testing the mode          and SW development. In functional mode the Slave interface should only          be controlled via transfer commands received from Master interface.           This trigger register reads back 0 always.  If TXENS and TXDISS are          written one  TXDISS has higher priority. The status is visible in          IFSTAT.TX EN."]
    #[inline(always)]
    pub fn txens(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, testctrl::Txens, Testctrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x1,1,0,testctrl::Txens, Testctrl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Disable Slave TX path  Slave Interface mode only    TXDISS. This function should be only used during interface testing the mode          and SW development. In functional mode the Slave interface should only          be controlled via transfer commands received from Master interface.           This trigger register reads back 0 always.  If TXENS and TXDISS are          written one  TXDISS has higher priority. The status is visible in          IFSTAT.TX EN."]
    #[inline(always)]
    pub fn txdiss(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, testctrl::Txdiss, Testctrl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            testctrl::Txdiss,
            Testctrl_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "LVDS loop back TX to RX enable   LLOPTXRX. Transmit data at LVDS is directly looped back to the LVDS RX. Data        transfer speed is defined by the TX speed configuration.  The data path        in the SoC is using the complete Transmit path through all functional        layers and is looped back at LVDS from TX to RX. Also at RX data path        all SoC data layers are active.  Requires same speed configuration for RX  and TX link before enabled."]
    #[inline(always)]
    pub fn lloptxrx(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        testctrl::Lloptxrx,
        Testctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            testctrl::Lloptxrx,
            Testctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "PRBS Pattern enable   PRBSEN. Enable of the PRBSEN bit allows a continuos PRBS stream with the        configured transfer speed Baud rate. This feature is available to        measure ISI during the time other SoC functions are running in an        applicative mode. This feature is for measurement purpose only."]
    #[inline(always)]
    pub fn prbsen(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        testctrl::Prbsen,
        Testctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            testctrl::Prbsen,
            Testctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Testctrl {
    #[inline(always)]
    fn default() -> Testctrl {
        <crate::RegValueT<Testctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod testctrl {
    pub struct Txens_SPEC;
    pub type Txens = crate::EnumBitfieldStruct<u8, Txens_SPEC>;
    impl Txens {
        #[doc = "0 Writing logic 0 has no effect"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Triggers the Slave interface TX enable."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Txdiss_SPEC;
    pub type Txdiss = crate::EnumBitfieldStruct<u8, Txdiss_SPEC>;
    impl Txdiss {
        #[doc = "0 Writing logic 0 has no effect"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Triggers the Slave interface TX disable."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lloptxrx_SPEC;
    pub type Lloptxrx = crate::EnumBitfieldStruct<u8, Lloptxrx_SPEC>;
    impl Lloptxrx {
        #[doc = "0 Disabled LVDS TX to RX data Loop back test mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled TX to RX data loop back test mode."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Prbsen_SPEC;
    pub type Prbsen = crate::EnumBitfieldStruct<u8, Prbsen_SPEC>;
    impl Prbsen {
        #[doc = "0 Disabled PRBS pattern generation on TX"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled PRBS pattern generation on TX."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Testmon_SPEC;
impl crate::sealed::RegSpec for Testmon_SPEC {
    type DataType = u32;
}
#[doc = "Test Monitor Register\n resetvalue={Application Reset:0x0}"]
pub type Testmon = crate::RegValueT<Testmon_SPEC>;

impl Testmon {
    #[doc = "TEST Receive path correlator 0 status   TRXCORS0"]
    #[inline(always)]
    pub fn trxcors0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, testmon::Trxcors0, Testmon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            testmon::Trxcors0,
            Testmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "TEST Receive path correlator 1status   TRXCORS1"]
    #[inline(always)]
    pub fn trxcors1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, testmon::Trxcors1, Testmon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            testmon::Trxcors1,
            Testmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "TEST Receive path correlator 2 status   TRXCORS2"]
    #[inline(always)]
    pub fn trxcors2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, testmon::Trxcors2, Testmon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            testmon::Trxcors2,
            Testmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "TEST Receive path correlator 3 status   TRXCORS3"]
    #[inline(always)]
    pub fn trxcors3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, testmon::Trxcors3, Testmon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            testmon::Trxcors3,
            Testmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "TEST Receive path correlator 4 status   TRXCORS4"]
    #[inline(always)]
    pub fn trxcors4(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, testmon::Trxcors4, Testmon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            testmon::Trxcors4,
            Testmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "TEST RX Synchronization Pattern detected   TRXSYNC"]
    #[inline(always)]
    pub fn trxsync(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, testmon::Trxsync, Testmon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            10,
            0x1,
            1,
            0,
            testmon::Trxsync,
            Testmon_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "TEST TX Frame Last   TTXFL"]
    #[inline(always)]
    pub fn ttxfl(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, testmon::Ttxfl, Testmon_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<11,0x1,1,0,testmon::Ttxfl, Testmon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "HSCT PHY Debug   shows the selected phase for sampling the RX LVDS bit stream.   LDBGSP"]
    #[inline(always)]
    pub fn ldbgsp(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, Testmon_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x7,1,0,u8, Testmon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "HSCT PHY Debug   Correlator toggle bit on a data change.   LDBGCOROT"]
    #[inline(always)]
    pub fn ldbgcorot(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Testmon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Testmon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "HSCT PHY Debug   this bit is toggled  whenever the phase is selected by the correlator.   LDBGSPT"]
    #[inline(always)]
    pub fn ldbgspt(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Testmon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Testmon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "LVDS PHY Debug   Indicates  if the LVDS receiver is in sleep  or in running state.   LDBGRXSLS"]
    #[inline(always)]
    pub fn ldbgrxsls(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Testmon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Testmon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "HSCT PHY Test monitor signal shows eye opening of the correlator and matching phases.   LDBGCORO"]
    #[inline(always)]
    pub fn ldbgcoro(
        self,
    ) -> crate::common::RegisterField<18, 0x1f, 1, 0, u8, Testmon_SPEC, crate::common::R> {
        crate::common::RegisterField::<18,0x1f,1,0,u8, Testmon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "HSCT monitor signals.  check signal description    HSCTMON"]
    #[inline(always)]
    pub fn hsctmon(
        self,
    ) -> crate::common::RegisterField<23, 0x1ff, 1, 0, u16, Testmon_SPEC, crate::common::R> {
        crate::common::RegisterField::<23,0x1ff,1,0,u16, Testmon_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Testmon {
    #[inline(always)]
    fn default() -> Testmon {
        <crate::RegValueT<Testmon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod testmon {
    pub struct Trxcors0_SPEC;
    pub type Trxcors0 = crate::EnumBitfieldStruct<u8, Trxcors0_SPEC>;
    impl Trxcors0 {
        #[doc = "00 Pattern 1  Checking for sync. pattern"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Pattern 2  Receiving second part of header"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Header  Receiving header"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Data  Receiving data pattern"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Trxcors1_SPEC;
    pub type Trxcors1 = crate::EnumBitfieldStruct<u8, Trxcors1_SPEC>;
    impl Trxcors1 {
        #[doc = "00 Pattern 1  Checking for sync. pattern"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Pattern 2  Receiving second part of header"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Header  Receiving header"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Data  Receiving data pattern"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Trxcors2_SPEC;
    pub type Trxcors2 = crate::EnumBitfieldStruct<u8, Trxcors2_SPEC>;
    impl Trxcors2 {
        #[doc = "00 Pattern 1  Checking for sync. pattern"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Pattern 2  Receiving second part of header"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Header  Receiving header"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Data  Receiving data pattern"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Trxcors3_SPEC;
    pub type Trxcors3 = crate::EnumBitfieldStruct<u8, Trxcors3_SPEC>;
    impl Trxcors3 {
        #[doc = "00 Pattern 1  Checking for sync. pattern"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Pattern 2  Receiving second part of header"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Header  Receiving header"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Data  Receiving data pattern"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Trxcors4_SPEC;
    pub type Trxcors4 = crate::EnumBitfieldStruct<u8, Trxcors4_SPEC>;
    impl Trxcors4 {
        #[doc = "00 Pattern 1  Checking for sync. pattern"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Pattern 2  Receiving second part of header"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Header  Receiving header"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Data  Receiving data pattern"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Trxsync_SPEC;
    pub type Trxsync = crate::EnumBitfieldStruct<u8, Trxsync_SPEC>;
    impl Trxsync {
        #[doc = "0 Toggle of the bit shows RX synchronization pattern found detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Toggle of the bit shows RX synchronization pattern found detected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ttxfl_SPEC;
    pub type Ttxfl = crate::EnumBitfieldStruct<u8, Ttxfl_SPEC>;
    impl Ttxfl {
        #[doc = "0 Toggle of the bit shows TX frame Last went out of PHY."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Toggle of the bit shows TX frame Last went out of PHY."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usmr_SPEC;
impl crate::sealed::RegSpec for Usmr_SPEC {
    type DataType = u32;
}
#[doc = "Unsolicited Status Message Received\n resetvalue={Application Reset:0x0}"]
pub type Usmr = crate::RegValueT<Usmr_SPEC>;

impl Usmr {
    #[doc = "Unsolicited status message received   USMR. The register contains the last received unsolicited status message."]
    #[inline(always)]
    pub fn usmr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Usmr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Usmr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Usmr {
    #[inline(always)]
    fn default() -> Usmr {
        <crate::RegValueT<Usmr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usms_SPEC;
impl crate::sealed::RegSpec for Usms_SPEC {
    type DataType = u32;
}
#[doc = "Unsolicited Status Message Send\n resetvalue={Application Reset:0x0}"]
pub type Usms = crate::RegValueT<Usms_SPEC>;

impl Usms {
    #[doc = "Unsolicited status message send   USMS. Writing to the register triggers an unsolicited status message to be        send to the other interface side."]
    #[inline(always)]
    pub fn usms(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Usms_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Usms_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Usms {
    #[inline(always)]
    fn default() -> Usms {
        <crate::RegValueT<Usms_SPEC> as RegisterValue<_>>::new(0)
    }
}
