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
#[doc = r"ASCLIN8"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asclin8(pub(super) *mut u8);
unsafe impl core::marker::Send for Asclin8 {}
unsafe impl core::marker::Sync for Asclin8 {}
impl Asclin8 {
    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(252usize)) }
    }

    #[doc = "Bit Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn bitcon(&self) -> crate::common::Reg<self::Bitcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Baud Rate Detection Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn brd(&self) -> crate::common::Reg<self::Brd_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "Baud Rate Generation Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn brg(&self) -> crate::common::Reg<self::Brg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Clock Selection Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn csr(&self) -> crate::common::Reg<self::Csr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(76usize)) }
    }

    #[doc = "Data Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn datcon(&self) -> crate::common::Reg<self::Datcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "Flags Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn flags(&self) -> crate::common::Reg<self::Flags_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }

    #[doc = "Flags Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn flagsclear(&self) -> crate::common::Reg<self::Flagsclear_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }

    #[doc = "Flags Enable Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn flagsenable(
        &self,
    ) -> crate::common::Reg<self::Flagsenable_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "Flags Set Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn flagsset(&self) -> crate::common::Reg<self::Flagsset_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }

    #[doc = "Frame Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn framecon(&self) -> crate::common::Reg<self::Framecon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0C1C000}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Input and Output Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn iocr(&self) -> crate::common::Reg<self::Iocr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
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

    #[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0,PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(232usize)) }
    }

    #[doc = "Receive Data Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rxdata(&self) -> crate::common::Reg<self::Rxdata_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }

    #[doc = "Receive Data Debug Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rxdatad(&self) -> crate::common::Reg<self::Rxdatad_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }

    #[doc = "RX FIFO Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rxfifocon(&self) -> crate::common::Reg<self::Rxfifocon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "Transmit Data Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn txdata(&self) -> crate::common::Reg<self::Txdata_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }

    #[doc = "TX FIFO Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn txfifocon(&self) -> crate::common::Reg<self::Txfifocon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "LIN"]
    #[inline(always)]
    pub fn lin(self) -> self::Lin {
        unsafe { self::Lin(self.0.add(40usize)) }
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
pub struct Bitcon_SPEC;
impl crate::sealed::RegSpec for Bitcon_SPEC {
    type DataType = u32;
}
#[doc = "Bit Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Bitcon = crate::RegValueT<Bitcon_SPEC>;

impl Bitcon {
    #[doc = "Prescaling of the Fractional Divider   PRESCALER. Prescaler bit field with values in the range of 0 to 4095  defining        division ratios from 1 to 4096. Used also as a microtick generator for        the input digital filter."]
    #[inline(always)]
    pub fn prescaler(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, Bitcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16, Bitcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Oversampling Factor   OVERSAMPLING. Defines the bit length in ticks in the range of 1 to 16. The lengths of        1 to 3 are not allowed. The position of the sampling points is shown in CROSSREFERENCE ."]
    #[inline(always)]
    pub fn oversampling(
        self,
    ) -> crate::common::RegisterField<
        16,
        0xf,
        1,
        0,
        bitcon::Oversampling,
        Bitcon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0xf,
            1,
            0,
            bitcon::Oversampling,
            Bitcon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Sample Point Position   SAMPLEPOINT. Programmable in the range of 0 to 15 according to the CROSSREFERENCE . For example  if three sample points at position 7  8  9 are required         this bit field would contain 9. In SPI mode  this bit field   1 defines the length of the first SCLK        half period in ticks. Values equal or higher then the OVERSAMPLING value are forbidden."]
    #[inline(always)]
    pub fn samplepoint(
        self,
    ) -> crate::common::RegisterField<
        24,
        0xf,
        1,
        0,
        bitcon::Samplepoint,
        Bitcon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0xf,
            1,
            0,
            bitcon::Samplepoint,
            Bitcon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Sample Mode   SM. Number of samples per bit."]
    #[inline(always)]
    pub fn sm(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, bitcon::Sm, Bitcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,bitcon::Sm, Bitcon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Bitcon {
    #[inline(always)]
    fn default() -> Bitcon {
        <crate::RegValueT<Bitcon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bitcon {
    pub struct Oversampling_SPEC;
    pub type Oversampling = crate::EnumBitfieldStruct<u8, Oversampling_SPEC>;
    impl Oversampling {
        #[doc = "0000 1  not        allowed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "0001 2  not        allowed"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "0010 3  not        allowed"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Samplepoint_SPEC;
    pub type Samplepoint = crate::EnumBitfieldStruct<u8, Samplepoint_SPEC>;
    impl Samplepoint {
        #[doc = "0000 0  not        allowed"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Sm_SPEC;
    pub type Sm = crate::EnumBitfieldStruct<u8, Sm_SPEC>;
    impl Sm {
        #[doc = "0 1"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 3"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Brd_SPEC;
impl crate::sealed::RegSpec for Brd_SPEC {
    type DataType = u32;
}
#[doc = "Baud Rate Detection Register\n resetvalue={Application Reset:0x0}"]
pub type Brd = crate::RegValueT<Brd_SPEC>;

impl Brd {
    #[doc = "Lower Limit   LOWERLIMIT. This field defines the 8 most significant bits of the 12 bit compare value. The lower four bits are 1000 . See CROSSREFERENCE ."]
    #[inline(always)]
    pub fn lowerlimit(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Brd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Brd_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Upper Limit   UPPERLIMIT. This field defines the 8 most significant bits of the 12 bit compare value. The lower four bits are 1000 . See CROSSREFERENCE ."]
    #[inline(always)]
    pub fn upperlimit(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Brd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Brd_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Measured Value of 8 bits from Sync Field   MEASURED. This bit field contains the measured value of the duration of 8 bits        form the sync field of the LIN header in microticks. It is automatically        loaded in the denominator of the fractional divider  in case of LIN        slave operation with autobaud detection."]
    #[inline(always)]
    pub fn measured(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, Brd_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xfff,1,0,u16, Brd_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Brd {
    #[inline(always)]
    fn default() -> Brd {
        <crate::RegValueT<Brd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Brg_SPEC;
impl crate::sealed::RegSpec for Brg_SPEC {
    type DataType = u32;
}
#[doc = "Baud Rate Generation Register\n resetvalue={Application Reset:0x0}"]
pub type Brg = crate::RegValueT<Brg_SPEC>;

impl Brg {
    #[doc = "Denominator   DENOMINATOR. Programmed by software  in a range of 0 to 4095.  The setting of 0 is not allowed If the module is used as ASC  SPI  LIN master and LIN slave without autobaud detection  this value determines the baud rate. In slave mode with autobaud detection  it contains the nominal value. For the value measured by the autobaud detection hardware  see the BRD register."]
    #[inline(always)]
    pub fn denominator(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, Brg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16, Brg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Numerator   NUMERATOR. Defines the numerator of the fractional divider in a range of 0 to 4095. Programmed by software. The setting of 0 is not allowed."]
    #[inline(always)]
    pub fn numerator(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, Brg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xfff,1,0,u16, Brg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Brg {
    #[inline(always)]
    fn default() -> Brg {
        <crate::RegValueT<Brg_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Sleep Mode Enable Control   EDIS. Used to control module s sleep mode  that is if the sensitivity of the module to the sleep signal is enabled to react to it or disabled to ignore it."]
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
    pub struct Edis_SPEC;
    pub type Edis = crate::EnumBitfieldStruct<u8, Edis_SPEC>;
    impl Edis {
        #[doc = "0 Enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr_SPEC;
impl crate::sealed::RegSpec for Csr_SPEC {
    type DataType = u32;
}
#[doc = "Clock Selection Register\n resetvalue={Application Reset:0x0}"]
pub type Csr = crate::RegValueT<Csr_SPEC>;

impl Csr {
    #[doc = "Baud Rate Logic Clock Select   CLKSEL"]
    #[inline(always)]
    pub fn clksel(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, csr::Clksel, Csr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,csr::Clksel, Csr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock On Flag   CON. Shows if the clock in the bit time domain is switched on or off. Many        configuration registers can be written only if this bit shows 0  see        header of registers IOCR  BITCON  FRAMECON  BRG  BRD  LINCON  LINBTIMER         LINHTIMER ."]
    #[inline(always)]
    pub fn con(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, csr::Con, Csr_SPEC, crate::common::R> {
        crate::common::RegisterField::<31,0x1,1,0,csr::Con, Csr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Csr {
    #[inline(always)]
    fn default() -> Csr {
        <crate::RegValueT<Csr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod csr {
    pub struct Clksel_SPEC;
    pub type Clksel = crate::EnumBitfieldStruct<u8, Clksel_SPEC>;
    impl Clksel {
        #[doc = "00000 No clock        supplied"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "00010 f ASCLINF"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "00100 f ASCLINS  and f OSC0          see CCU"]
        pub const CONST_44: Self = Self::new(4);
    }
    pub struct Con_SPEC;
    pub type Con = crate::EnumBitfieldStruct<u8, Con_SPEC>;
    impl Con {
        #[doc = "0 Clock is off"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clock is on"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Datcon_SPEC;
impl crate::sealed::RegSpec for Datcon_SPEC {
    type DataType = u32;
}
#[doc = "Data Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Datcon = crate::RegValueT<Datcon_SPEC>;

impl Datcon {
    #[doc = "Data Length   DATLEN. Defines the number of bits in a character. In the ASC mode  standard        length is 7  8  or 9 bits. In the SPI mode  there is no standard length. In ASC and SPI modes  any length from 2 to 16 bits is possible  although        not standard for some protocols. In LIN mode  standard length is 8 bits per character. Therefore  this        field defines the number of data bytes of the response."]
    #[inline(always)]
    pub fn datlen(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Datcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Datcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Header Only   HO. Defines if the LIN frame shall consist of a header and response or of a header only."]
    #[inline(always)]
    pub fn ho(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, datcon::Ho, Datcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,datcon::Ho, Datcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Response Mode   RM. Defines if the RESPONSE bit field defines a LIN Response or LIN Frame timeout threshold. See CROSSREFERENCE ."]
    #[inline(always)]
    pub fn rm(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, datcon::Rm, Datcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,datcon::Rm, Datcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Checksum Mode   CSM. Defines if the classic or the enhanced checksum will be calculated by the checksum block."]
    #[inline(always)]
    pub fn csm(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, datcon::Csm, Datcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,datcon::Csm, Datcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Response Timeout Threshold Value   RESPONSE. Defines the timer limit in the range of 1 to 256 bit times."]
    #[inline(always)]
    pub fn response(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Datcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Datcon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Datcon {
    #[inline(always)]
    fn default() -> Datcon {
        <crate::RegValueT<Datcon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod datcon {
    pub struct Ho_SPEC;
    pub type Ho = crate::EnumBitfieldStruct<u8, Ho_SPEC>;
    impl Ho {
        #[doc = "0 Header and response expected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Header only expected  response ignored"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rm_SPEC;
    pub type Rm = crate::EnumBitfieldStruct<u8, Rm_SPEC>;
    impl Rm {
        #[doc = "0 Frame"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Response"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Csm_SPEC;
    pub type Csm = crate::EnumBitfieldStruct<u8, Csm_SPEC>;
    impl Csm {
        #[doc = "0 Classic"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enhanced"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flags_SPEC;
impl crate::sealed::RegSpec for Flags_SPEC {
    type DataType = u32;
}
#[doc = "Flags Register\n resetvalue={Application Reset:0x0}"]
pub type Flags = crate::RegValueT<Flags_SPEC>;

impl Flags {
    #[doc = "Transmit Header End Flag   TH. Signals the HEADER TX END event.  Set by hardware  clear by software. If enabled  a transmit interrupt is triggered."]
    #[inline(always)]
    pub fn th(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, flags::Th, Flags_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,flags::Th, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit Response End Flag   TR. Signals that RESPONSE TX END event.  Set by hardware  clear by software. If enabled  a transmit interrupt is triggered."]
    #[inline(always)]
    pub fn tr(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, flags::Tr, Flags_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,flags::Tr, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive Header End Flag   RH. Signals that HEADER RX END event.  Set by hardware  clear by software. If enabled  a receive interrupt is triggered."]
    #[inline(always)]
    pub fn rh(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, flags::Rh, Flags_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x1,1,0,flags::Rh, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive Response End Flag   RR. Signals that RESPONSE RX END event.  Set by hardware  clear by software. If enabled  a receive interrupt is triggered."]
    #[inline(always)]
    pub fn rr(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, flags::Rr, Flags_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1,1,0,flags::Rr, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Falling Edge from Level 1 to Level 0 Detected   FED. This bit is set by hardware when a falling edge is detected on the RX line."]
    #[inline(always)]
    pub fn fed(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, flags::Fed, Flags_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x1,1,0,flags::Fed, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Rising Edge from Level 0 to Level 1 Detected   RED. This bit is set by hardware when a rising edge is detected on the RX line."]
    #[inline(always)]
    pub fn red(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, flags::Red, Flags_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x1,1,0,flags::Red, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit Wake Request Flag   TWRQ. Signals that transmission of wake has been requested. No interrupt triggered. As soon as the wake pulse transmission starts  the bit is cleared by the hardware."]
    #[inline(always)]
    pub fn twrq(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, flags::Twrq, Flags_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<13,0x1,1,0,flags::Twrq, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit Header Request Flag   THRQ. Signals that transmission of header has been requested. No interrupt triggered. As soon as the header transmission starts  the bit is cleared by the hardware."]
    #[inline(always)]
    pub fn thrq(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, flags::Thrq, Flags_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<14,0x1,1,0,flags::Thrq, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit Response Request Flag   TRRQ. Signals that transmission of response has been requested. No interrupt triggered. As soon as the response transmission starts  the bit is cleared by the hardware."]
    #[inline(always)]
    pub fn trrq(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, flags::Trrq, Flags_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<15,0x1,1,0,flags::Trrq, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Parity Error Flag   PE. Signals parity error. If enabled  an error interrupt is triggered. Parity error occurs if the internally calculated parity bit is not equal to the received parity bit."]
    #[inline(always)]
    pub fn pe(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, flags::Pe, Flags_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x1,1,0,flags::Pe, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmission Completed Flag   TC. Signals an end of an ASC or SPI frame. This bit is set after the last        stop bit transmission in ASC mode  or after the trailing delay in case        of SPI mode. In LIN mode  if the node is transmitting a header this flag        is set after each transmission break  incl lead  field  Sync field or        PID field. If the node is transmitting a response  this flag is set        after each byte is transmitted. If enabled  an EX interrupt is        triggered. Should be cleared by software."]
    #[inline(always)]
    pub fn tc(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, flags::Tc, Flags_SPEC, crate::common::R> {
        crate::common::RegisterField::<17,0x1,1,0,flags::Tc, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Framing Error Flag   FE. Signals framing error. If enabled  an error interrupt is triggered. Framing error occurs if  0  is received at a stop bit position. If autobaud detection is deactivated  then the sync field is checked for framing error."]
    #[inline(always)]
    pub fn fe(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, flags::Fe, Flags_SPEC, crate::common::R> {
        crate::common::RegisterField::<18,0x1,1,0,flags::Fe, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Header Timeout Flag   HT. Signals violation of the header duration limit. If enabled  an error interrupt is triggered."]
    #[inline(always)]
    pub fn ht(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, flags::Ht, Flags_SPEC, crate::common::R> {
        crate::common::RegisterField::<19,0x1,1,0,flags::Ht, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Response Timeout Flag   RT. Signals violation of the response or frame duration limit as defined in DATCON .RM bit. If enabled  an error interrupt is triggered."]
    #[inline(always)]
    pub fn rt(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, flags::Rt, Flags_SPEC, crate::common::R> {
        crate::common::RegisterField::<20,0x1,1,0,flags::Rt, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Break Detected Flag   BD. Signals a detection of a break pulse. If enabled  an error interrupt is triggered. Slave mode only."]
    #[inline(always)]
    pub fn bd(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, flags::Bd, Flags_SPEC, crate::common::R> {
        crate::common::RegisterField::<21,0x1,1,0,flags::Bd, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "LIN Parity Error Flag   LP. Signals parity error in the LIN identifier. If enabled  an error interrupt is triggered. Applies to LIN mode only. LIN parity error occurs if the internally calculated parity bits are not equal to the received parity bits."]
    #[inline(always)]
    pub fn lp(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, flags::Lp, Flags_SPEC, crate::common::R> {
        crate::common::RegisterField::<22,0x1,1,0,flags::Lp, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "LIN Autobaud Detection Error Flag   LA. Signals baudrate outside the range defined by BRD .LOWERLIMIT and BRD .UPPERLIMIT."]
    #[inline(always)]
    pub fn la(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, flags::La, Flags_SPEC, crate::common::R> {
        crate::common::RegisterField::<23,0x1,1,0,flags::La, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "LIN Checksum Error Flag   LC. Signals checksum error when receiving response  if the internally        calculated checksum is different than the received checksum. If enabled         an error interrupt is triggered."]
    #[inline(always)]
    pub fn lc(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, flags::Lc, Flags_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x1,1,0,flags::Lc, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Collision Detection Error Flag   CE. When transmitting  signals if the transmitted data differs from the received data. If enabled  an error interrupt is triggered in case of a mismatch. Collision detection is mandatory only when supporting LIN version 2.1."]
    #[inline(always)]
    pub fn ce(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, flags::Ce, Flags_SPEC, crate::common::R> {
        crate::common::RegisterField::<25,0x1,1,0,flags::Ce, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive FIFO Overflow Flag   RFO. Signals an overflow error. If enabled  an error interrupt is triggered."]
    #[inline(always)]
    pub fn rfo(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, flags::Rfo, Flags_SPEC, crate::common::R> {
        crate::common::RegisterField::<26,0x1,1,0,flags::Rfo, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive FIFO Underflow Flag   RFU. Signals an underflow error. If enabled  an error interrupt is triggered.        See also CROSSREFERENCE ."]
    #[inline(always)]
    pub fn rfu(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, flags::Rfu, Flags_SPEC, crate::common::R> {
        crate::common::RegisterField::<27,0x1,1,0,flags::Rfu, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive FIFO Level Flag   RFL. This flag signals whenever a RXFIFO fill interrupt is generated based on        RXFIFOCON.FM mode."]
    #[inline(always)]
    pub fn rfl(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, flags::Rfl, Flags_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0x1,1,0,flags::Rfl, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit FIFO Overflow Flag   TFO. Signals an overflow error. If enabled  an error interrupt is triggered."]
    #[inline(always)]
    pub fn tfo(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, flags::Tfo, Flags_SPEC, crate::common::R> {
        crate::common::RegisterField::<30,0x1,1,0,flags::Tfo, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit FIFO Level Flag   TFL. This flag signals whenever a TXFIFO refill interrupt  if enabled  is        generated based on TXFIFOCON.FM mode."]
    #[inline(always)]
    pub fn tfl(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, flags::Tfl, Flags_SPEC, crate::common::R> {
        crate::common::RegisterField::<31,0x1,1,0,flags::Tfl, Flags_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Flags {
    #[inline(always)]
    fn default() -> Flags {
        <crate::RegValueT<Flags_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod flags {
    pub struct Th_SPEC;
    pub type Th = crate::EnumBitfieldStruct<u8, Th_SPEC>;
    impl Th {
        #[doc = "0 No HEADER TX END event since the last clear by software"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 New HEADER TX END event since the last clear by software"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tr_SPEC;
    pub type Tr = crate::EnumBitfieldStruct<u8, Tr_SPEC>;
    impl Tr {
        #[doc = "0 No RESPONSE TX END event since the last clear by software"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 New RESPONSE TX END event since the last clear by software"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rh_SPEC;
    pub type Rh = crate::EnumBitfieldStruct<u8, Rh_SPEC>;
    impl Rh {
        #[doc = "0 No HEADER RX END event since the last clear by software"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 New HEADER RX END event since the last clear by software"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rr_SPEC;
    pub type Rr = crate::EnumBitfieldStruct<u8, Rr_SPEC>;
    impl Rr {
        #[doc = "0 No RESPONSE RX END event since the last clear by software"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 New RX RESPONSE END event since the last clear by software"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fed_SPEC;
    pub type Fed = crate::EnumBitfieldStruct<u8, Fed_SPEC>;
    impl Fed {
        #[doc = "0 No falling edge detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Falling edge detected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Red_SPEC;
    pub type Red = crate::EnumBitfieldStruct<u8, Red_SPEC>;
    impl Red {
        #[doc = "0 No rising edge detected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Rising edge detected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Twrq_SPEC;
    pub type Twrq = crate::EnumBitfieldStruct<u8, Twrq_SPEC>;
    impl Twrq {
        #[doc = "0 No Transmit Wake Request pending"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transmit Wake Request pending."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Thrq_SPEC;
    pub type Thrq = crate::EnumBitfieldStruct<u8, Thrq_SPEC>;
    impl Thrq {
        #[doc = "0 No Transmit Header Request pending"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transmit Header Request pending."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trrq_SPEC;
    pub type Trrq = crate::EnumBitfieldStruct<u8, Trrq_SPEC>;
    impl Trrq {
        #[doc = "0 No Transmit Response Request pending"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transmit Response Request pending."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pe_SPEC;
    pub type Pe = crate::EnumBitfieldStruct<u8, Pe_SPEC>;
    impl Pe {
        #[doc = "0 Last message received error free"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Last message received with parity error"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tc_SPEC;
    pub type Tc = crate::EnumBitfieldStruct<u8, Tc_SPEC>;
    impl Tc {
        #[doc = "0 No end of frame event occurred"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 End of frame event occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fe_SPEC;
    pub type Fe = crate::EnumBitfieldStruct<u8, Fe_SPEC>;
    impl Fe {
        #[doc = "0 Last message received error free"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Last message received with error"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ht_SPEC;
    pub type Ht = crate::EnumBitfieldStruct<u8, Ht_SPEC>;
    impl Ht {
        #[doc = "0 No HEADER OVERFLOW event since the last clear by software"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 New HEADER OVERFLOW event since the last clear by software"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rt_SPEC;
    pub type Rt = crate::EnumBitfieldStruct<u8, Rt_SPEC>;
    impl Rt {
        #[doc = "0 No timeout event since the last clear by software"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 New timeout event since the last clear by software"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Bd_SPEC;
    pub type Bd = crate::EnumBitfieldStruct<u8, Bd_SPEC>;
    impl Bd {
        #[doc = "0 No Break Wake Stuck event since the last clear by software"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 New Break Wake Stuck event since the last clear by software"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lp_SPEC;
    pub type Lp = crate::EnumBitfieldStruct<u8, Lp_SPEC>;
    impl Lp {
        #[doc = "0 Last ID received error free"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Last ID received with parity error"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct La_SPEC;
    pub type La = crate::EnumBitfieldStruct<u8, La_SPEC>;
    impl La {
        #[doc = "0 No autobaud detection error"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Autobaud detection error"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lc_SPEC;
    pub type Lc = crate::EnumBitfieldStruct<u8, Lc_SPEC>;
    impl Lc {
        #[doc = "0 Last checksum error free"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Last checksum shows an error"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ce_SPEC;
    pub type Ce = crate::EnumBitfieldStruct<u8, Ce_SPEC>;
    impl Ce {
        #[doc = "0 No mismatch"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Mismatch detected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfo_SPEC;
    pub type Rfo = crate::EnumBitfieldStruct<u8, Rfo_SPEC>;
    impl Rfo {
        #[doc = "0 No overflow error occurred"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Overflow error occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfu_SPEC;
    pub type Rfu = crate::EnumBitfieldStruct<u8, Rfu_SPEC>;
    impl Rfu {
        #[doc = "0 No underflow error occurred"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Underflow error occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfl_SPEC;
    pub type Rfl = crate::EnumBitfieldStruct<u8, Rfl_SPEC>;
    impl Rfl {
        #[doc = "0 No receive interrupt occurred"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Receive interrupt occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tfo_SPEC;
    pub type Tfo = crate::EnumBitfieldStruct<u8, Tfo_SPEC>;
    impl Tfo {
        #[doc = "0 No overflow error occurred"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Overflow error occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tfl_SPEC;
    pub type Tfl = crate::EnumBitfieldStruct<u8, Tfl_SPEC>;
    impl Tfl {
        #[doc = "0 No transmit interrupt occurred"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Transmit interrupt occurred"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flagsclear_SPEC;
impl crate::sealed::RegSpec for Flagsclear_SPEC {
    type DataType = u32;
}
#[doc = "Flags Clear Register\n resetvalue={Application Reset:0x0}"]
pub type Flagsclear = crate::RegValueT<Flagsclear_SPEC>;

impl Flagsclear {
    #[doc = "Flag Clear Bit   THC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn thc(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        flagsclear::Thc,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            flagsclear::Thc,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Clear Bit   TRC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn trc(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        flagsclear::Trc,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            flagsclear::Trc,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Clear Bit   RHC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn rhc(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        flagsclear::Rhc,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            flagsclear::Rhc,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Clear Bit   RRC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn rrc(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        flagsclear::Rrc,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            flagsclear::Rrc,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Clear Bit   FEDC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn fedc(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        flagsclear::Fedc,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            flagsclear::Fedc,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Clear Bit   REDC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn redc(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        flagsclear::Redc,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            flagsclear::Redc,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Clear Bit   TWRQC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn twrqc(
        self,
    ) -> crate::common::RegisterField<
        13,
        0x1,
        1,
        0,
        flagsclear::Twrqc,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            flagsclear::Twrqc,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Clear Bit   THRQC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn thrqc(
        self,
    ) -> crate::common::RegisterField<
        14,
        0x1,
        1,
        0,
        flagsclear::Thrqc,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            flagsclear::Thrqc,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Clear Bit   TRRQC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn trrqc(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        flagsclear::Trrqc,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            flagsclear::Trrqc,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Clear Bit   PEC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn pec(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        flagsclear::Pec,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            flagsclear::Pec,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Clear Bit   TCC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn tcc(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        flagsclear::Tcc,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            flagsclear::Tcc,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Clear Bit   FEC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn fec(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        flagsclear::Fec,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            flagsclear::Fec,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Clear Bit   HTC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn htc(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        flagsclear::Htc,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            flagsclear::Htc,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Clear Bit   RTC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn rtc(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        flagsclear::Rtc,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            flagsclear::Rtc,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Clear Bit   BDC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn bdc(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        flagsclear::Bdc,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            flagsclear::Bdc,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Clear Bit   LPC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn lpc(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        flagsclear::Lpc,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            flagsclear::Lpc,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Clear Bit   LAC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn lac(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        flagsclear::Lac,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            flagsclear::Lac,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Clear Bit   LCC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn lcc(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        flagsclear::Lcc,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            flagsclear::Lcc,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Clear Bit   CEC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn cec(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        flagsclear::Cec,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            flagsclear::Cec,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Clear Bit   RFOC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn rfoc(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        flagsclear::Rfoc,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            flagsclear::Rfoc,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Clear Bit   RFUC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn rfuc(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        flagsclear::Rfuc,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            flagsclear::Rfuc,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Clear Bit   RFLC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn rflc(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        flagsclear::Rflc,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            flagsclear::Rflc,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Clear Bit   TFOC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn tfoc(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        flagsclear::Tfoc,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            flagsclear::Tfoc,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Clear Bit   TFLC. Write of   8220 1  8221  in this bit clears the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn tflc(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        flagsclear::Tflc,
        Flagsclear_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            flagsclear::Tflc,
            Flagsclear_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Flagsclear {
    #[inline(always)]
    fn default() -> Flagsclear {
        <crate::RegValueT<Flagsclear_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod flagsclear {
    pub struct Thc_SPEC;
    pub type Thc = crate::EnumBitfieldStruct<u8, Thc_SPEC>;
    impl Thc {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trc_SPEC;
    pub type Trc = crate::EnumBitfieldStruct<u8, Trc_SPEC>;
    impl Trc {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rhc_SPEC;
    pub type Rhc = crate::EnumBitfieldStruct<u8, Rhc_SPEC>;
    impl Rhc {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rrc_SPEC;
    pub type Rrc = crate::EnumBitfieldStruct<u8, Rrc_SPEC>;
    impl Rrc {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fedc_SPEC;
    pub type Fedc = crate::EnumBitfieldStruct<u8, Fedc_SPEC>;
    impl Fedc {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Redc_SPEC;
    pub type Redc = crate::EnumBitfieldStruct<u8, Redc_SPEC>;
    impl Redc {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Twrqc_SPEC;
    pub type Twrqc = crate::EnumBitfieldStruct<u8, Twrqc_SPEC>;
    impl Twrqc {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear the        corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Thrqc_SPEC;
    pub type Thrqc = crate::EnumBitfieldStruct<u8, Thrqc_SPEC>;
    impl Thrqc {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trrqc_SPEC;
    pub type Trrqc = crate::EnumBitfieldStruct<u8, Trrqc_SPEC>;
    impl Trrqc {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pec_SPEC;
    pub type Pec = crate::EnumBitfieldStruct<u8, Pec_SPEC>;
    impl Pec {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tcc_SPEC;
    pub type Tcc = crate::EnumBitfieldStruct<u8, Tcc_SPEC>;
    impl Tcc {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fec_SPEC;
    pub type Fec = crate::EnumBitfieldStruct<u8, Fec_SPEC>;
    impl Fec {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Htc_SPEC;
    pub type Htc = crate::EnumBitfieldStruct<u8, Htc_SPEC>;
    impl Htc {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rtc_SPEC;
    pub type Rtc = crate::EnumBitfieldStruct<u8, Rtc_SPEC>;
    impl Rtc {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Bdc_SPEC;
    pub type Bdc = crate::EnumBitfieldStruct<u8, Bdc_SPEC>;
    impl Bdc {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lpc_SPEC;
    pub type Lpc = crate::EnumBitfieldStruct<u8, Lpc_SPEC>;
    impl Lpc {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lac_SPEC;
    pub type Lac = crate::EnumBitfieldStruct<u8, Lac_SPEC>;
    impl Lac {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lcc_SPEC;
    pub type Lcc = crate::EnumBitfieldStruct<u8, Lcc_SPEC>;
    impl Lcc {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cec_SPEC;
    pub type Cec = crate::EnumBitfieldStruct<u8, Cec_SPEC>;
    impl Cec {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfoc_SPEC;
    pub type Rfoc = crate::EnumBitfieldStruct<u8, Rfoc_SPEC>;
    impl Rfoc {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfuc_SPEC;
    pub type Rfuc = crate::EnumBitfieldStruct<u8, Rfuc_SPEC>;
    impl Rfuc {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rflc_SPEC;
    pub type Rflc = crate::EnumBitfieldStruct<u8, Rflc_SPEC>;
    impl Rflc {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tfoc_SPEC;
    pub type Tfoc = crate::EnumBitfieldStruct<u8, Tfoc_SPEC>;
    impl Tfoc {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tflc_SPEC;
    pub type Tflc = crate::EnumBitfieldStruct<u8, Tflc_SPEC>;
    impl Tflc {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clears the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flagsenable_SPEC;
impl crate::sealed::RegSpec for Flagsenable_SPEC {
    type DataType = u32;
}
#[doc = "Flags Enable Register\n resetvalue={Application Reset:0x0}"]
pub type Flagsenable = crate::RegValueT<Flagsenable_SPEC>;

impl Flagsenable {
    #[doc = "Flag Enable Bit   THE. This bit enables the interrupt for the flag at the corresponding        position in the FLAGS register."]
    #[inline(always)]
    pub fn the(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        flagsenable::The,
        Flagsenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            flagsenable::The,
            Flagsenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Flag Enable Bit   TRE. This bit enables the interrupt for the flag at the corresponding position in the FLAGS register."]
    #[inline(always)]
    pub fn tre(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        flagsenable::Tre,
        Flagsenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            flagsenable::Tre,
            Flagsenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Flag Enable Bit   RHE. This bit enables the interrupt for the flag at the corresponding position in the FLAGS register."]
    #[inline(always)]
    pub fn rhe(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        flagsenable::Rhe,
        Flagsenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            flagsenable::Rhe,
            Flagsenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Flag Enable Bit   RRE. This bit enables the interrupt for the flag at the corresponding position in the FLAGS register."]
    #[inline(always)]
    pub fn rre(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        flagsenable::Rre,
        Flagsenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            flagsenable::Rre,
            Flagsenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Flag Enable Bit   FEDE. This bit enables the interrupt for the flag at the corresponding position in the FLAGS register."]
    #[inline(always)]
    pub fn fede(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        flagsenable::Fede,
        Flagsenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            flagsenable::Fede,
            Flagsenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Flag Enable Bit   REDE. This bit enables the interrupt for the flag at the corresponding position in the FLAGS register."]
    #[inline(always)]
    pub fn rede(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        flagsenable::Rede,
        Flagsenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            flagsenable::Rede,
            Flagsenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Flag Enable Bit   PEE. This bit enables the interrupt for the flag at the corresponding position in the FLAGS register."]
    #[inline(always)]
    pub fn pee(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x1,
        1,
        0,
        flagsenable::Pee,
        Flagsenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x1,
            1,
            0,
            flagsenable::Pee,
            Flagsenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Flag Enable Bit   TCE. This bit enables the interrupt for the flag at the corresponding position in the FLAGS register."]
    #[inline(always)]
    pub fn tce(
        self,
    ) -> crate::common::RegisterField<
        17,
        0x1,
        1,
        0,
        flagsenable::Tce,
        Flagsenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            17,
            0x1,
            1,
            0,
            flagsenable::Tce,
            Flagsenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Flag Enable Bit   FEE. This bit enables the interrupt for the flag at the corresponding position in the FLAGS register."]
    #[inline(always)]
    pub fn fee(
        self,
    ) -> crate::common::RegisterField<
        18,
        0x1,
        1,
        0,
        flagsenable::Fee,
        Flagsenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            18,
            0x1,
            1,
            0,
            flagsenable::Fee,
            Flagsenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Flag Enable Bit   HTE. This bit enables the interrupt for the flag at the corresponding        position in the FLAGS register."]
    #[inline(always)]
    pub fn hte(
        self,
    ) -> crate::common::RegisterField<
        19,
        0x1,
        1,
        0,
        flagsenable::Hte,
        Flagsenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            19,
            0x1,
            1,
            0,
            flagsenable::Hte,
            Flagsenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Flag Enable Bit   RTE. This bit enables the interrupt for the flag at the corresponding position in the FLAGS register."]
    #[inline(always)]
    pub fn rte(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x1,
        1,
        0,
        flagsenable::Rte,
        Flagsenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x1,
            1,
            0,
            flagsenable::Rte,
            Flagsenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Flag Enable Bit   BDE. This bit enables the interrupt for the flag at the corresponding position in the FLAGS register."]
    #[inline(always)]
    pub fn bde(
        self,
    ) -> crate::common::RegisterField<
        21,
        0x1,
        1,
        0,
        flagsenable::Bde,
        Flagsenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            21,
            0x1,
            1,
            0,
            flagsenable::Bde,
            Flagsenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Flag Enable Bit   LPE. This bit enables the interrupt for the flag at the corresponding position in the FLAGS register."]
    #[inline(always)]
    pub fn lpe(
        self,
    ) -> crate::common::RegisterField<
        22,
        0x1,
        1,
        0,
        flagsenable::Lpe,
        Flagsenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            22,
            0x1,
            1,
            0,
            flagsenable::Lpe,
            Flagsenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Flag Enable Bit   LAE. This bit enables the interrupt for the flag at the corresponding        position in the FLAGS register."]
    #[inline(always)]
    pub fn lae(
        self,
    ) -> crate::common::RegisterField<
        23,
        0x1,
        1,
        0,
        flagsenable::Lae,
        Flagsenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            23,
            0x1,
            1,
            0,
            flagsenable::Lae,
            Flagsenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Flag Enable Bit   LCE. This bit enables the interrupt for the flag at the corresponding        position in the FLAGS register."]
    #[inline(always)]
    pub fn lce(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x1,
        1,
        0,
        flagsenable::Lce,
        Flagsenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x1,
            1,
            0,
            flagsenable::Lce,
            Flagsenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Flag Enable Bit   CEE. This bit enables the interrupt for the flag at the corresponding position in the FLAGS register."]
    #[inline(always)]
    pub fn cee(
        self,
    ) -> crate::common::RegisterField<
        25,
        0x1,
        1,
        0,
        flagsenable::Cee,
        Flagsenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            25,
            0x1,
            1,
            0,
            flagsenable::Cee,
            Flagsenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Flag Enable Bit   RFOE. This bit enables the interrupt for the flag at the corresponding position in the FLAGS register."]
    #[inline(always)]
    pub fn rfoe(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x1,
        1,
        0,
        flagsenable::Rfoe,
        Flagsenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            26,
            0x1,
            1,
            0,
            flagsenable::Rfoe,
            Flagsenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Flag Enable Bit   RFUE. This bit enables the interrupt for the flag at the corresponding position in the FLAGS register."]
    #[inline(always)]
    pub fn rfue(
        self,
    ) -> crate::common::RegisterField<
        27,
        0x1,
        1,
        0,
        flagsenable::Rfue,
        Flagsenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            27,
            0x1,
            1,
            0,
            flagsenable::Rfue,
            Flagsenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Flag Enable Bit   RFLE. This bit enables the interrupt for the flag at the corresponding position in the FLAGS register."]
    #[inline(always)]
    pub fn rfle(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x1,
        1,
        0,
        flagsenable::Rfle,
        Flagsenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            28,
            0x1,
            1,
            0,
            flagsenable::Rfle,
            Flagsenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Flag Enable Bit   TFOE. This bit enables the interrupt for the flag at the corresponding position in the FLAGS register."]
    #[inline(always)]
    pub fn tfoe(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x1,
        1,
        0,
        flagsenable::Tfoe,
        Flagsenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            30,
            0x1,
            1,
            0,
            flagsenable::Tfoe,
            Flagsenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Flag Enable Bit   TFLE. This bit enables the interrupt for the flag at the corresponding position in the FLAGS register."]
    #[inline(always)]
    pub fn tfle(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        flagsenable::Tfle,
        Flagsenable_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            flagsenable::Tfle,
            Flagsenable_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Flagsenable {
    #[inline(always)]
    fn default() -> Flagsenable {
        <crate::RegValueT<Flagsenable_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod flagsenable {
    pub struct The_SPEC;
    pub type The = crate::EnumBitfieldStruct<u8, The_SPEC>;
    impl The {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tre_SPEC;
    pub type Tre = crate::EnumBitfieldStruct<u8, Tre_SPEC>;
    impl Tre {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rhe_SPEC;
    pub type Rhe = crate::EnumBitfieldStruct<u8, Rhe_SPEC>;
    impl Rhe {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rre_SPEC;
    pub type Rre = crate::EnumBitfieldStruct<u8, Rre_SPEC>;
    impl Rre {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fede_SPEC;
    pub type Fede = crate::EnumBitfieldStruct<u8, Fede_SPEC>;
    impl Fede {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rede_SPEC;
    pub type Rede = crate::EnumBitfieldStruct<u8, Rede_SPEC>;
    impl Rede {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pee_SPEC;
    pub type Pee = crate::EnumBitfieldStruct<u8, Pee_SPEC>;
    impl Pee {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tce_SPEC;
    pub type Tce = crate::EnumBitfieldStruct<u8, Tce_SPEC>;
    impl Tce {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fee_SPEC;
    pub type Fee = crate::EnumBitfieldStruct<u8, Fee_SPEC>;
    impl Fee {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hte_SPEC;
    pub type Hte = crate::EnumBitfieldStruct<u8, Hte_SPEC>;
    impl Hte {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rte_SPEC;
    pub type Rte = crate::EnumBitfieldStruct<u8, Rte_SPEC>;
    impl Rte {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Bde_SPEC;
    pub type Bde = crate::EnumBitfieldStruct<u8, Bde_SPEC>;
    impl Bde {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lpe_SPEC;
    pub type Lpe = crate::EnumBitfieldStruct<u8, Lpe_SPEC>;
    impl Lpe {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lae_SPEC;
    pub type Lae = crate::EnumBitfieldStruct<u8, Lae_SPEC>;
    impl Lae {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lce_SPEC;
    pub type Lce = crate::EnumBitfieldStruct<u8, Lce_SPEC>;
    impl Lce {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cee_SPEC;
    pub type Cee = crate::EnumBitfieldStruct<u8, Cee_SPEC>;
    impl Cee {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfoe_SPEC;
    pub type Rfoe = crate::EnumBitfieldStruct<u8, Rfoe_SPEC>;
    impl Rfoe {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfue_SPEC;
    pub type Rfue = crate::EnumBitfieldStruct<u8, Rfue_SPEC>;
    impl Rfue {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfle_SPEC;
    pub type Rfle = crate::EnumBitfieldStruct<u8, Rfle_SPEC>;
    impl Rfle {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tfoe_SPEC;
    pub type Tfoe = crate::EnumBitfieldStruct<u8, Tfoe_SPEC>;
    impl Tfoe {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tfle_SPEC;
    pub type Tfle = crate::EnumBitfieldStruct<u8, Tfle_SPEC>;
    impl Tfle {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flagsset_SPEC;
impl crate::sealed::RegSpec for Flagsset_SPEC {
    type DataType = u32;
}
#[doc = "Flags Set Register\n resetvalue={Application Reset:0x0}"]
pub type Flagsset = crate::RegValueT<Flagsset_SPEC>;

impl Flagsset {
    #[doc = "Flag Set Bit   THS. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn ths(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, flagsset::Ths, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x1,1,0,flagsset::Ths, Flagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Flag Set Bit   TRS. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn trs(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, flagsset::Trs, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<1,0x1,1,0,flagsset::Trs, Flagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Flag Set Bit   RHS. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn rhs(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, flagsset::Rhs, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<2,0x1,1,0,flagsset::Rhs, Flagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Flag Set Bit   RRS. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn rrs(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, flagsset::Rrs, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<3,0x1,1,0,flagsset::Rrs, Flagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Flag Set Bit   FEDS. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn feds(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, flagsset::Feds, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<5,0x1,1,0,flagsset::Feds, Flagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Flag Set Bit   REDS. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn reds(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, flagsset::Reds, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<6,0x1,1,0,flagsset::Reds, Flagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Flag Set Bit   TWRQS. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn twrqs(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, flagsset::Twrqs, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            13,
            0x1,
            1,
            0,
            flagsset::Twrqs,
            Flagsset_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Set Bit   THRQS. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn thrqs(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, flagsset::Thrqs, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            14,
            0x1,
            1,
            0,
            flagsset::Thrqs,
            Flagsset_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Set Bit   TRRQS. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn trrqs(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, flagsset::Trrqs, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            flagsset::Trrqs,
            Flagsset_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Flag Set Bit   PES. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn pes(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, flagsset::Pes, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<16,0x1,1,0,flagsset::Pes, Flagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Flag Set Bit   TCS. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn tcs(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, flagsset::Tcs, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<17,0x1,1,0,flagsset::Tcs, Flagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Flag Set Bit   FES. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn fes(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, flagsset::Fes, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<18,0x1,1,0,flagsset::Fes, Flagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Flag Set Bit   HTS. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn hts(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, flagsset::Hts, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<19,0x1,1,0,flagsset::Hts, Flagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Flag Set Bit   RTS. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn rts(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, flagsset::Rts, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<20,0x1,1,0,flagsset::Rts, Flagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Flag Set Bit   BDS. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn bds(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, flagsset::Bds, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<21,0x1,1,0,flagsset::Bds, Flagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Flag Set Bit   LPS. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn lps(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, flagsset::Lps, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<22,0x1,1,0,flagsset::Lps, Flagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Flag Set Bit   LAS. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn las(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, flagsset::Las, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<23,0x1,1,0,flagsset::Las, Flagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Flag Set Bit   LCS. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn lcs(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, flagsset::Lcs, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<24,0x1,1,0,flagsset::Lcs, Flagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Flag Set Bit   CES. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn ces(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, flagsset::Ces, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<25,0x1,1,0,flagsset::Ces, Flagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Flag Set Bit   RFOS. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn rfos(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, flagsset::Rfos, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<26,0x1,1,0,flagsset::Rfos, Flagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Flag Set Bit   RFUS. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn rfus(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, flagsset::Rfus, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<27,0x1,1,0,flagsset::Rfus, Flagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Flag Set Bit   RFLS. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn rfls(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, flagsset::Rfls, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<28,0x1,1,0,flagsset::Rfls, Flagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Flag Set Bit   TFOS. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn tfos(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, flagsset::Tfos, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<30,0x1,1,0,flagsset::Tfos, Flagsset_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Flag Set Bit   TFLS. Write of   8220 1  8221  in this bit sets the bit at the corresponding position in        the FLAGS register."]
    #[inline(always)]
    pub fn tfls(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, flagsset::Tfls, Flagsset_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<31,0x1,1,0,flagsset::Tfls, Flagsset_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Flagsset {
    #[inline(always)]
    fn default() -> Flagsset {
        <crate::RegValueT<Flagsset_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod flagsset {
    pub struct Ths_SPEC;
    pub type Ths = crate::EnumBitfieldStruct<u8, Ths_SPEC>;
    impl Ths {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trs_SPEC;
    pub type Trs = crate::EnumBitfieldStruct<u8, Trs_SPEC>;
    impl Trs {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rhs_SPEC;
    pub type Rhs = crate::EnumBitfieldStruct<u8, Rhs_SPEC>;
    impl Rhs {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rrs_SPEC;
    pub type Rrs = crate::EnumBitfieldStruct<u8, Rrs_SPEC>;
    impl Rrs {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Feds_SPEC;
    pub type Feds = crate::EnumBitfieldStruct<u8, Feds_SPEC>;
    impl Feds {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Reds_SPEC;
    pub type Reds = crate::EnumBitfieldStruct<u8, Reds_SPEC>;
    impl Reds {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Twrqs_SPEC;
    pub type Twrqs = crate::EnumBitfieldStruct<u8, Twrqs_SPEC>;
    impl Twrqs {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Thrqs_SPEC;
    pub type Thrqs = crate::EnumBitfieldStruct<u8, Thrqs_SPEC>;
    impl Thrqs {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trrqs_SPEC;
    pub type Trrqs = crate::EnumBitfieldStruct<u8, Trrqs_SPEC>;
    impl Trrqs {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pes_SPEC;
    pub type Pes = crate::EnumBitfieldStruct<u8, Pes_SPEC>;
    impl Pes {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tcs_SPEC;
    pub type Tcs = crate::EnumBitfieldStruct<u8, Tcs_SPEC>;
    impl Tcs {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fes_SPEC;
    pub type Fes = crate::EnumBitfieldStruct<u8, Fes_SPEC>;
    impl Fes {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Hts_SPEC;
    pub type Hts = crate::EnumBitfieldStruct<u8, Hts_SPEC>;
    impl Hts {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rts_SPEC;
    pub type Rts = crate::EnumBitfieldStruct<u8, Rts_SPEC>;
    impl Rts {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Bds_SPEC;
    pub type Bds = crate::EnumBitfieldStruct<u8, Bds_SPEC>;
    impl Bds {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lps_SPEC;
    pub type Lps = crate::EnumBitfieldStruct<u8, Lps_SPEC>;
    impl Lps {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Las_SPEC;
    pub type Las = crate::EnumBitfieldStruct<u8, Las_SPEC>;
    impl Las {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lcs_SPEC;
    pub type Lcs = crate::EnumBitfieldStruct<u8, Lcs_SPEC>;
    impl Lcs {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ces_SPEC;
    pub type Ces = crate::EnumBitfieldStruct<u8, Ces_SPEC>;
    impl Ces {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfos_SPEC;
    pub type Rfos = crate::EnumBitfieldStruct<u8, Rfos_SPEC>;
    impl Rfos {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfus_SPEC;
    pub type Rfus = crate::EnumBitfieldStruct<u8, Rfus_SPEC>;
    impl Rfus {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rfls_SPEC;
    pub type Rfls = crate::EnumBitfieldStruct<u8, Rfls_SPEC>;
    impl Rfls {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tfos_SPEC;
    pub type Tfos = crate::EnumBitfieldStruct<u8, Tfos_SPEC>;
    impl Tfos {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tfls_SPEC;
    pub type Tfls = crate::EnumBitfieldStruct<u8, Tfls_SPEC>;
    impl Tfls {
        #[doc = "0 No action          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the corresponding flag"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Framecon_SPEC;
impl crate::sealed::RegSpec for Framecon_SPEC {
    type DataType = u32;
}
#[doc = "Frame Control Register\n resetvalue={Application Reset:0x0}"]
pub type Framecon = crate::RegValueT<Framecon_SPEC>;

impl Framecon {
    #[doc = "Duration of the IDLE delay   IDLE. Defines the duration of the IDLE delay in bit times. If more characters        are available in the TXFIFO  this is the pause inserted between the        characters. In the SPI mode  this is the idle time between the frames.        In the ASC and LIN mode  this is the pause inserted between transmission        of bytes. Idle also applies to the pause between the header and the        response  response space . The collision detection runs in parallel to the idle delay and in LIN          master mode it may extend the time between two bytes for one bit          length. This effect may occur if the round trip delay including the          digital filter delay is longer than the idle delay. For LIN slave mode          use IDLE 0."]
    #[inline(always)]
    pub fn idle(
        self,
    ) -> crate::common::RegisterField<6, 0x7, 1, 0, u8, Framecon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x7,1,0,u8, Framecon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Number of Stop Bits   STOP. Defines the number of stop bits in ASC and LIN mode  or the trailing        delay in SPI mode. In ASC mode  standard values are 1 and 2. In LIN        mode  standard value is 1. In SPI mode there is no standard value. Nevertheless  all settings are possible in all modes."]
    #[inline(always)]
    pub fn stop(
        self,
    ) -> crate::common::RegisterField<9, 0x7, 1, 0, framecon::Stop, Framecon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x7,1,0,framecon::Stop, Framecon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Duration of the Leading Delay   LEAD. Defines the leading delay in bit times in SPI mode. Has no meaning in the ASC mode. In LIN mode  this is a delay inserted between the end of the break and        the start of the sync character."]
    #[inline(always)]
    pub fn lead(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, framecon::Lead, Framecon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            12,
            0x7,
            1,
            0,
            framecon::Lead,
            Framecon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Mode Selection   MODE. This bit field defines the basic operating mode of the module. In INIT mode  all outputs are at inactive level  and the module does not        respond to the input signals. Changing the mode of the module must be        done by switching first to INIT mode  and then to the other mode. The SCLK signal generated by the module is active only in the SPI mode. The CTS output generated by the module is active only in the ASC mode."]
    #[inline(always)]
    pub fn mode(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, framecon::Mode, Framecon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            framecon::Mode,
            Framecon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Shift Direction   MSB. Defines the shift direction of the shift register. Relevant for the SPI mode. In ASC and LIN modes  should be set to zero. Parity bit is shifted out last independently of the shift direction."]
    #[inline(always)]
    pub fn msb(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, framecon::Msb, Framecon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,framecon::Msb, Framecon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Collision Detection Enable   CEN. Enables the collision detection mechanism."]
    #[inline(always)]
    pub fn cen(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, framecon::Cen, Framecon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,framecon::Cen, Framecon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Parity Enable   PEN. Enables the parity bit attached to the data bits. Parity bit can be used for ASC and SPI protocols. The standard LIN bytes do not use this parity bit."]
    #[inline(always)]
    pub fn pen(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, framecon::Pen, Framecon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,framecon::Pen, Framecon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Parity Type   ODD. Defines the type of parity bit attached to the data bits. This setting is valid for all modes of operation  ASC  LIN  SPI ."]
    #[inline(always)]
    pub fn odd(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, framecon::Odd, Framecon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,framecon::Odd, Framecon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Framecon {
    #[inline(always)]
    fn default() -> Framecon {
        <crate::RegValueT<Framecon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod framecon {
    pub struct Stop_SPEC;
    pub type Stop = crate::EnumBitfieldStruct<u8, Stop_SPEC>;
    impl Stop {
        #[doc = "000 0  not        allowed in ASC and LIN modes"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Lead_SPEC;
    pub type Lead = crate::EnumBitfieldStruct<u8, Lead_SPEC>;
    impl Lead {
        #[doc = "000 0  not        allowed in LIN mode and 4 Wire SPI mode"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Mode_SPEC;
    pub type Mode = crate::EnumBitfieldStruct<u8, Mode_SPEC>;
    impl Mode {
        #[doc = "00 INIT mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 ASC mode"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 SPI mode"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 LIN mode"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Msb_SPEC;
    pub type Msb = crate::EnumBitfieldStruct<u8, Msb_SPEC>;
    impl Msb {
        #[doc = "0 LSB first"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MSB first"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cen_SPEC;
    pub type Cen = crate::EnumBitfieldStruct<u8, Cen_SPEC>;
    impl Cen {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pen_SPEC;
    pub type Pen = crate::EnumBitfieldStruct<u8, Pen_SPEC>;
    impl Pen {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Odd_SPEC;
    pub type Odd = crate::EnumBitfieldStruct<u8, Odd_SPEC>;
    impl Odd {
        #[doc = "0 Even"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Odd"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0C1C000}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MODREV. MODREV defines the module revision number. The value of a module revision starts with 01 H  first revision ."]
    #[inline(always)]
    pub fn modrev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MODTYPE. This bit field is C0 H . It defines a 32 bit module."]
    #[inline(always)]
    pub fn modtype(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MODNUMBER. This bit field together with MODTYPE uniquely identifies a module."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(12697600)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iocr_SPEC;
impl crate::sealed::RegSpec for Iocr_SPEC {
    type DataType = u32;
}
#[doc = "Input and Output Control Register\n resetvalue={Application Reset:0x0}"]
pub type Iocr = crate::RegValueT<Iocr_SPEC>;

impl Iocr {
    #[doc = "Alternate Input Select   ALTI. Selects the alternate input for the RX signal"]
    #[inline(always)]
    pub fn alti(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, iocr::Alti, Iocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,iocr::Alti, Iocr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Digital Glitch Filter Depth   DEPTH. DEPTH determines the number of port input samples clocked with        microticks that are taken into account for the calculation of the        floating average. The higher the DEPTH is chosen to be  the longer the        glitches that are suppressed and the longer the delay of the input        signal introduced by this filter."]
    #[inline(always)]
    pub fn depth(
        self,
    ) -> crate::common::RegisterField<4, 0x3f, 1, 0, iocr::Depth, Iocr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3f,1,0,iocr::Depth, Iocr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CTS Select   CTS. Selects the CTS input pin out of maximum four possible."]
    #[inline(always)]
    pub fn cts(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, iocr::Cts, Iocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,iocr::Cts, Iocr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RTS CTS Polarity   RCPOL. RCPOL defines the active level or the RTS and CTS signals. Active means        ready clear to send."]
    #[inline(always)]
    pub fn rcpol(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, iocr::Rcpol, Iocr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,iocr::Rcpol, Iocr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Polarity in Synchronous Mode   CPOL. CPOL defines the idle level of the clock signal if the module is set in        the SPI mode. The idle level is the level outside the data transmission        time intervals. Default is low level."]
    #[inline(always)]
    pub fn cpol(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, iocr::Cpol, Iocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x1,1,0,iocr::Cpol, Iocr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Slave Polarity in Synchronous Mode   SPOL. Defines the idle level of the SLSO signal  which is the level outside        the data transmission  leading and trailing time intervals."]
    #[inline(always)]
    pub fn spol(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, iocr::Spol, Iocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1,1,0,iocr::Spol, Iocr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Loop Back Mode   LB. Enables the in module connection of the transmit signal to receive        signal. If Loop back is enabled  the module can be run and tested        without an external connection  in ASC and SPI modes. In LIN mode  loopback should not be used  because the module can be        either master or slave."]
    #[inline(always)]
    pub fn lb(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, iocr::Lb, Iocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x1,1,0,iocr::Lb, Iocr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Input Signal CTS Enable   CTSEN. Enables the sensitivity of the module to the external CTS signal. If        disabled  the CTS signal is considered being permanently active."]
    #[inline(always)]
    pub fn ctsen(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, iocr::Ctsen, Iocr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,iocr::Ctsen, Iocr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive Monitor   RXM. Shows the status of the receive signal."]
    #[inline(always)]
    pub fn rxm(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, iocr::Rxm, Iocr_SPEC, crate::common::R> {
        crate::common::RegisterField::<30,0x1,1,0,iocr::Rxm, Iocr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Transmit Monitor   TXM. Shows the status of the transmit signal."]
    #[inline(always)]
    pub fn txm(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, iocr::Txm, Iocr_SPEC, crate::common::R> {
        crate::common::RegisterField::<31,0x1,1,0,iocr::Txm, Iocr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Iocr {
    #[inline(always)]
    fn default() -> Iocr {
        <crate::RegValueT<Iocr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod iocr {
    pub struct Alti_SPEC;
    pub type Alti = crate::EnumBitfieldStruct<u8, Alti_SPEC>;
    impl Alti {
        #[doc = "Alternate Input A selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Alternate Input B selected"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "Alternate Input C selected"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "Alternate Input D selected"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "Alternate Input E selected"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "Alternate Input F selected"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "Alternate Input G selected"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "Alternate Input H selected"]
        pub const CONST_77: Self = Self::new(7);
    }
    pub struct Depth_SPEC;
    pub type Depth = crate::EnumBitfieldStruct<u8, Depth_SPEC>;
    impl Depth {
        #[doc = "000000 off         default"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Cts_SPEC;
    pub type Cts = crate::EnumBitfieldStruct<u8, Cts_SPEC>;
    impl Cts {
        #[doc = "00 ACTSA"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 ACTSB"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 ACTSC"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 ACTSD"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Rcpol_SPEC;
    pub type Rcpol = crate::EnumBitfieldStruct<u8, Rcpol_SPEC>;
    impl Rcpol {
        #[doc = "0 Active high"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Active low"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cpol_SPEC;
    pub type Cpol = crate::EnumBitfieldStruct<u8, Cpol_SPEC>;
    impl Cpol {
        #[doc = "0 Idle low"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Idle high"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Spol_SPEC;
    pub type Spol = crate::EnumBitfieldStruct<u8, Spol_SPEC>;
    impl Spol {
        #[doc = "0 Idle low"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Idle high"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Lb_SPEC;
    pub type Lb = crate::EnumBitfieldStruct<u8, Lb_SPEC>;
    impl Lb {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ctsen_SPEC;
    pub type Ctsen = crate::EnumBitfieldStruct<u8, Ctsen_SPEC>;
    impl Ctsen {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rxm_SPEC;
    pub type Rxm = crate::EnumBitfieldStruct<u8, Rxm_SPEC>;
    impl Rxm {
        #[doc = "0 Current signal is low."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Current signal is high."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Txm_SPEC;
    pub type Txm = crate::EnumBitfieldStruct<u8, Txm_SPEC>;
    impl Txm {
        #[doc = "0 Current signal is low."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Current signal is high."]
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel        reset will be executed if the reset bits of both kernel reset registers        is set. The RST bit will be cleared  re set to   180 0  180   by the BPI FPI after the        kernel reset was executed."]
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
#[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0,PowerOn Reset:0x0}"]
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
        #[doc = "2 Soft suspend"]
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
pub struct Rxdata_SPEC;
impl crate::sealed::RegSpec for Rxdata_SPEC {
    type DataType = u32;
}
#[doc = "Receive Data Register\n resetvalue={Application Reset:0x0}"]
pub type Rxdata = crate::RegValueT<Rxdata_SPEC>;

impl Rxdata {
    #[doc = "Data   DATA. Reading from this bit field takes content from the RXFIFO  depending on        the read width   8  16 or 32 bit  configured in RXFIFOCON.OUTW . A        write access to this register has no effect."]
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Rxdata_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Rxdata_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Rxdata {
    #[inline(always)]
    fn default() -> Rxdata {
        <crate::RegValueT<Rxdata_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxdatad_SPEC;
impl crate::sealed::RegSpec for Rxdatad_SPEC {
    type DataType = u32;
}
#[doc = "Receive Data Debug Register\n resetvalue={Application Reset:0x0}"]
pub type Rxdatad = crate::RegValueT<Rxdatad_SPEC>;

impl Rxdatad {
    #[doc = "Data   DATA. Reading from this bit field takes content from the RXFIFO  depending on        the read width   8  16 or 32 bit  see RXFIFOCON.OUTW   but does not        influence the read pointer of the RXFIFO. A write access to this        register has no effect."]
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Rxdatad_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Rxdatad_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Rxdatad {
    #[inline(always)]
    fn default() -> Rxdatad {
        <crate::RegValueT<Rxdatad_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxfifocon_SPEC;
impl crate::sealed::RegSpec for Rxfifocon_SPEC {
    type DataType = u32;
}
#[doc = "RX FIFO Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Rxfifocon = crate::RegValueT<Rxfifocon_SPEC>;

impl Rxfifocon {
    #[doc = "Flush the receive FIFO   FLUSH. Write of 1 brings the Rx FIFO in empty state. Write of 0 has no effect."]
    #[inline(always)]
    pub fn flush(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        rxfifocon::Flush,
        Rxfifocon_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            rxfifocon::Flush,
            Rxfifocon_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Receive FIFO Inlet Enable   ENI. Enables the receiver and the filling of the Rx FIFO through the shift        register. In LIN slave mode  this bit is set by hardware after the        correct reception of the sync byte. The software can clear this bit        after reception of an foreign ID in order to suppress the reception of        the following response."]
    #[inline(always)]
    pub fn eni(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, rxfifocon::Eni, Rxfifocon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            rxfifocon::Eni,
            Rxfifocon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "RXFIFO Mode   FM. Selects between the RXFIFO Modes."]
    #[inline(always)]
    pub fn fm(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, rxfifocon::Fm, Rxfifocon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,rxfifocon::Fm, Rxfifocon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive FIFO Outlet Width   OUTW. Defines the number of bytes read to the Rx FIFO with one FPI bus read."]
    #[inline(always)]
    pub fn outw(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        rxfifocon::Outw,
        Rxfifocon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            rxfifocon::Outw,
            Rxfifocon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "FIFO Interrupt Level   INTLEVEL. Defines the filling level that triggers a drain interrupt or DMA access.        An interrupt is generated when the filling level rises to INTLEVEL or        beyond  each time when a data byte is delivered to the FIFO. This        behavior corresponds to the Combined Compatibility Mode of interrupt        generation. See also Single Move Mode and Batch Move Mode for two        additional modes."]
    #[inline(always)]
    pub fn intlevel(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Rxfifocon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Rxfifocon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "FIFO Filling Level   FILL. Read only bit field containing the current filling level of the FIFO."]
    #[inline(always)]
    pub fn fill(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Rxfifocon_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Rxfifocon_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Receive Buffer Mode   BUF. If this bit is zero  then the RXFIFO behaves normally as described in this document. If this bit is set  the RXFIFO behaves as simple 32 bit one stage RX buffer  which is overwritten with each new received data. The received bits appear in the RXDATA register on the lowest bit locations. The upper locations are padded with zeros."]
    #[inline(always)]
    pub fn buf(
        self,
    ) -> crate::common::RegisterField<
        31,
        0x1,
        1,
        0,
        rxfifocon::Buf,
        Rxfifocon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            rxfifocon::Buf,
            Rxfifocon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Rxfifocon {
    #[inline(always)]
    fn default() -> Rxfifocon {
        <crate::RegValueT<Rxfifocon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod rxfifocon {
    pub struct Flush_SPEC;
    pub type Flush = crate::EnumBitfieldStruct<u8, Flush_SPEC>;
    impl Flush {
        #[doc = "0 No effect          This value will be always read back from the module."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Empty the Rx FIFO"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eni_SPEC;
    pub type Eni = crate::EnumBitfieldStruct<u8, Eni_SPEC>;
    impl Eni {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fm_SPEC;
    pub type Fm = crate::EnumBitfieldStruct<u8, Fm_SPEC>;
    impl Fm {
        #[doc = "00 Combined Move Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Single Move Mode"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Batch Move Mode"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Outw_SPEC;
    pub type Outw = crate::EnumBitfieldStruct<u8, Outw_SPEC>;
    impl Outw {
        #[doc = "00 0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 1"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 4"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Buf_SPEC;
    pub type Buf = crate::EnumBitfieldStruct<u8, Buf_SPEC>;
    impl Buf {
        #[doc = "0 RXFIFO"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Single Stage RX Buffer"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txdata_SPEC;
impl crate::sealed::RegSpec for Txdata_SPEC {
    type DataType = u32;
}
#[doc = "Transmit Data Register\n resetvalue={Application Reset:0x0}"]
pub type Txdata = crate::RegValueT<Txdata_SPEC>;

impl Txdata {
    #[doc = "Data   DATA. Writing to this bit field writes the content to the TXFIFO  depending on        the write width   8  16 or 32 bit  configured in TXFIFOCON.INW . A        read access to this register returns 0x0."]
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Txdata_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Txdata_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Txdata {
    #[inline(always)]
    fn default() -> Txdata {
        <crate::RegValueT<Txdata_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txfifocon_SPEC;
impl crate::sealed::RegSpec for Txfifocon_SPEC {
    type DataType = u32;
}
#[doc = "TX FIFO Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Txfifocon = crate::RegValueT<Txfifocon_SPEC>;

impl Txfifocon {
    #[doc = "Flush the transmit FIFO   FLUSH. Write of 1 brings the Tx FIFO in empty state. Write of 0 has no effect."]
    #[inline(always)]
    pub fn flush(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        txfifocon::Flush,
        Txfifocon_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            txfifocon::Flush,
            Txfifocon_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Transmit FIFO Outlet Enable   ENO. Enables the TxFIFO outlet. In SPI and ASC modes  data transmission starts immediately when the data is available  whereas in LIN case the transmission start is controlled by the protocol engine."]
    #[inline(always)]
    pub fn eno(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, txfifocon::Eno, Txfifocon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            txfifocon::Eno,
            Txfifocon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "TXFIFO Mode   FM. Selects between the TXFIFO Modes."]
    #[inline(always)]
    pub fn fm(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, txfifocon::Fm, Txfifocon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,txfifocon::Fm, Txfifocon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transmit FIFO Inlet Width   INW. Defines the number of bytes written to the Tx FIFO with one FPI bus write."]
    #[inline(always)]
    pub fn inw(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, txfifocon::Inw, Txfifocon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            txfifocon::Inw,
            Txfifocon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "FIFO Interrupt Level   INTLEVEL. Defines the filling level that triggers a re fill interrupt or DMA        access. An interrupt is generated when the filling level falls to INTLEVEL or        below  each time when a data byte is taken out of the FIFO. This        behavior corresponds to the Combined Compatibility Mode of interrupt        generation. See also Single Move Mode and Batch Move Mode for two        additional modes."]
    #[inline(always)]
    pub fn intlevel(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Txfifocon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Txfifocon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "FIFO Filling Level   FILL. Read only bit field containing the current filling level of the FIFO."]
    #[inline(always)]
    pub fn fill(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Txfifocon_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Txfifocon_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Txfifocon {
    #[inline(always)]
    fn default() -> Txfifocon {
        <crate::RegValueT<Txfifocon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod txfifocon {
    pub struct Flush_SPEC;
    pub type Flush = crate::EnumBitfieldStruct<u8, Flush_SPEC>;
    impl Flush {
        #[doc = "0 No effect          This value will be always read back from the Bit."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Empty the Tx        FIFO"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eno_SPEC;
    pub type Eno = crate::EnumBitfieldStruct<u8, Eno_SPEC>;
    impl Eno {
        #[doc = "0 Disabled. In LIN case  if the protocol engine tries to fetch data."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled. In LIN case  no data is moved to the shift register until it is fetched by the protocol engine."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fm_SPEC;
    pub type Fm = crate::EnumBitfieldStruct<u8, Fm_SPEC>;
    impl Fm {
        #[doc = "00 Combined Move Mode"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Single Move Mode"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Batch Move Mode"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Inw_SPEC;
    pub type Inw = crate::EnumBitfieldStruct<u8, Inw_SPEC>;
    impl Inw {
        #[doc = "00 0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 1"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 2"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 4"]
        pub const CONST_33: Self = Self::new(3);
    }
}

#[doc = "LIN"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lin(pub(super) *mut u8);
unsafe impl core::marker::Send for Lin {}
unsafe impl core::marker::Sync for Lin {}
impl Lin {
    #[doc = "LIN Break Timer Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn linbtimer(&self) -> crate::common::Reg<lin::Linbtimer_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "LIN Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn lincon(&self) -> crate::common::Reg<lin::Lincon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "LIN Header Timer Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn linhtimer(&self) -> crate::common::Reg<lin::Linhtimer_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
pub mod lin {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Linbtimer_SPEC;
    impl crate::sealed::RegSpec for Linbtimer_SPEC {
        type DataType = u32;
    }
    #[doc = "LIN Break Timer Register\n resetvalue={Application Reset:0x0}"]
    pub type Linbtimer = crate::RegValueT<Linbtimer_SPEC>;

    impl Linbtimer {
        #[doc = "Break Pulse Generation and Detection   BREAK. In LIN slave mode  this bit field defines the duration of the detection threshold for the break pulse. In LIN master mode  this bit field defines the duration of the transmitted break pulse. The time unit is bit time."]
        #[inline(always)]
        pub fn r#break(
            self,
        ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Linbtimer_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3f,1,0,u8, Linbtimer_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Linbtimer {
        #[inline(always)]
        fn default() -> Linbtimer {
            <crate::RegValueT<Linbtimer_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lincon_SPEC;
    impl crate::sealed::RegSpec for Lincon_SPEC {
        type DataType = u32;
    }
    #[doc = "LIN Control Register\n resetvalue={Application Reset:0x0}"]
    pub type Lincon = crate::RegValueT<Lincon_SPEC>;

    impl Lincon {
        #[doc = "Checksum Injection   CSI. Defines if the received checksum byte is written into the RXFIFO or not.  See CROSSREFERENCE ."]
        #[inline(always)]
        pub fn csi(
            self,
        ) -> crate::common::RegisterField<23, 0x1, 1, 0, lincon::Csi, Lincon_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<23,0x1,1,0,lincon::Csi, Lincon_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Hardware Checksum Enable   CSEN. Enables the hardware checksum generation and checking."]
        #[inline(always)]
        pub fn csen(
            self,
        ) -> crate::common::RegisterField<25, 0x1, 1, 0, lincon::Csen, Lincon_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                25,
                0x1,
                1,
                0,
                lincon::Csen,
                Lincon_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Master Slave Mode   MS. Configures if the module in LIN mode operates as master or as slave."]
        #[inline(always)]
        pub fn ms(
            self,
        ) -> crate::common::RegisterField<26, 0x1, 1, 0, lincon::Ms, Lincon_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<26,0x1,1,0,lincon::Ms, Lincon_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Autobaud Detection   ABD. Enables the autobaud detection feature in LIN slave mode. In all other        operating modes of the module  LIN master  ASC  SPI  not effective. If the autobaud detection is disabled  the oscillator precision of the        slave is sufficient   the sync field  byte field and stop bit  is        checked if correct. If not correct  a framing error is triggered."]
        #[inline(always)]
        pub fn abd(
            self,
        ) -> crate::common::RegisterField<27, 0x1, 1, 0, lincon::Abd, Lincon_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<27,0x1,1,0,lincon::Abd, Lincon_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Lincon {
        #[inline(always)]
        fn default() -> Lincon {
            <crate::RegValueT<Lincon_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod lincon {
        pub struct Csi_SPEC;
        pub type Csi = crate::EnumBitfieldStruct<u8, Csi_SPEC>;
        impl Csi {
            #[doc = "0 Not written"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Written"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Csen_SPEC;
        pub type Csen = crate::EnumBitfieldStruct<u8, Csen_SPEC>;
        impl Csen {
            #[doc = "0 Disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Ms_SPEC;
        pub type Ms = crate::EnumBitfieldStruct<u8, Ms_SPEC>;
        impl Ms {
            #[doc = "0 Slave"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Master"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Abd_SPEC;
        pub type Abd = crate::EnumBitfieldStruct<u8, Abd_SPEC>;
        impl Abd {
            #[doc = "0 Disabled"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Enabled"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Linhtimer_SPEC;
    impl crate::sealed::RegSpec for Linhtimer_SPEC {
        type DataType = u32;
    }
    #[doc = "LIN Header Timer Register\n resetvalue={Application Reset:0x0}"]
    pub type Linhtimer = crate::RegValueT<Linhtimer_SPEC>;

    impl Linhtimer {
        #[doc = "Header Timeout Threshold Value   HEADER. Defines the timer limit in the range of 0 to 255 bit times."]
        #[inline(always)]
        pub fn header(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Linhtimer_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xff,1,0,u8, Linhtimer_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Linhtimer {
        #[inline(always)]
        fn default() -> Linhtimer {
            <crate::RegValueT<Linhtimer_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
