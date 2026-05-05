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
#[doc = r"MSC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msc0(pub(super) *mut u8);
unsafe impl core::marker::Send for Msc0 {}
unsafe impl core::marker::Sync for Msc0 {}
impl Msc0 {
    #[doc = "Asynchronous Block Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn abc(&self) -> crate::common::Reg<self::Abc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }

    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(252usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Downstream Command Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dc(&self) -> crate::common::Reg<self::Dc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "Downstream Command Extension Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dce(&self) -> crate::common::Reg<self::Dce_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(124usize)) }
    }

    #[doc = "Downstream Command Mirror Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dcm(&self) -> crate::common::Reg<self::Dcm_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(120usize)) }
    }

    #[doc = "Downstream Data Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dd(&self) -> crate::common::Reg<self::Dd_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }

    #[doc = "Downstream Data Extension Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dde(&self) -> crate::common::Reg<self::Dde_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(116usize)) }
    }

    #[doc = "Downstream Data Mirror Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ddm(&self) -> crate::common::Reg<self::Ddm_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(112usize)) }
    }

    #[doc = "Downstream Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dsc(&self) -> crate::common::Reg<self::Dsc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "Downstream Control Enhanced Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dsce(&self) -> crate::common::Reg<self::Dsce_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(88usize)) }
    }

    #[doc = "Downstream Select Data Source High Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dsdsh(&self) -> crate::common::Reg<self::Dsdsh_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }

    #[doc = "Downstream Select Data Source High Extension Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dsdshe(&self) -> crate::common::Reg<self::Dsdshe_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(100usize)) }
    }

    #[doc = "Downstream Select Data Source Low Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dsdsl(&self) -> crate::common::Reg<self::Dsdsl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }

    #[doc = "Downstream Select Data Source Low Extension Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dsdsle(&self) -> crate::common::Reg<self::Dsdsle_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }

    #[doc = "Downstream Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dss(&self) -> crate::common::Reg<self::Dss_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }

    #[doc = "Downstream Timing Extension Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dste(&self) -> crate::common::Reg<self::Dste_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(108usize)) }
    }

    #[doc = "Emergency Stop Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn esr(&self) -> crate::common::Reg<self::Esr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }

    #[doc = "Emergency Stop Extension Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn esre(&self) -> crate::common::Reg<self::Esre_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(104usize)) }
    }

    #[doc = "Fractional Divider Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fdr(&self) -> crate::common::Reg<self::Fdr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }

    #[doc = "Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn icr(&self) -> crate::common::Reg<self::Icr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x28C010}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "Interrupt Set Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn isc(&self) -> crate::common::Reg<self::Isc_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }

    #[doc = "Interrupt Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn isr(&self) -> crate::common::Reg<self::Isr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
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

    #[doc = "Output Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ocr(&self) -> crate::common::Reg<self::Ocr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(76usize)) }
    }

    #[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(232usize)) }
    }

    #[doc = "Upstream Data Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn udx(&self) -> [crate::common::Reg<self::UDx_SPEC, crate::common::RW>; 4] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x30usize + 0xcusize)),
            ]
        }
    }

    #[doc = "Upstream Control Enhanced Register 1\n resetvalue={Application Reset:0x0FF}"]
    #[inline(always)]
    pub const fn usce(&self) -> crate::common::Reg<self::Usce_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(92usize)) }
    }

    #[doc = "Upstream Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn usr(&self) -> crate::common::Reg<self::Usr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Abc_SPEC;
impl crate::sealed::RegSpec for Abc_SPEC {
    type DataType = u32;
}
#[doc = "Asynchronous Block Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Abc = crate::RegValueT<Abc_SPEC>;

impl Abc {
    #[doc = "Duration of the Low Phase of the Shift Clock   LOW. Defines the duration of the low phase of the shift clock in periods of f A in the range of 1 to 16."]
    #[inline(always)]
    pub fn low(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Abc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0xf, 1, 0, u8, Abc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Duration of the High Phase of the Shift Clock   HIGH. Defines the duration of the high phase of the shift clock in periods of f A in the range of 1 to 16."]
    #[inline(always)]
    pub fn high(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Abc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4, 0xf, 1, 0, u8, Abc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Overflow Interrupt Node Pointer   OIP. OIP selects the service request output line SRn  n   0 3  for the overflow interrupt."]
    #[inline(always)]
    pub fn oip(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, abc::Oip, Abc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,abc::Oip, Abc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Overflow Alternate Service Request   OASR. Selects if the interrupt signal is routed to the alternate service request node. See CROSSREFERENCE ."]
    #[inline(always)]
    pub fn oasr(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, abc::Oasr, Abc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x1,1,0,abc::Oasr, Abc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Overflow Flag   OVF. Indicates if overflow error has occurred."]
    #[inline(always)]
    pub fn ovf(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, abc::Ovf, Abc_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x1,1,0,abc::Ovf, Abc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Overflow Flag Modify   OFM. Sets or clears the overflow flag OVF."]
    #[inline(always)]
    pub fn ofm(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, abc::Ofm, Abc_SPEC, crate::common::W> {
        crate::common::RegisterField::<13,0x3,1,0,abc::Ofm, Abc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Overflow Interrupt Enable   OIE. Enables or disables the path of the interrupt signal towards the        interrupt node. If enabled  an overflow event triggers an interrupt  and        if disabled then not. The OVF flag always indicates the occurrence of an overflow event         independently of OIE."]
    #[inline(always)]
    pub fn oie(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, abc::Oie, Abc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<15,0x1,1,0,abc::Oie, Abc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N Divider ABRA   NDA. Defines the division ratio in the range of 1 to 8."]
    #[inline(always)]
    pub fn nda(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Abc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Abc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Underflow Interrupt Node Pointer   UIP. UIP selects the service request output line SRn  n  160    160 0 3  for the        underflow interrupt."]
    #[inline(always)]
    pub fn uip(
        self,
    ) -> crate::common::RegisterField<19, 0x3, 1, 0, abc::Uip, Abc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<19,0x3,1,0,abc::Uip, Abc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Underflow Alternate Service Request   UASR. Selects if the interrupt signal is routed to the alternate service request node. See CROSSREFERENCE ."]
    #[inline(always)]
    pub fn uasr(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, abc::Uasr, Abc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x1,1,0,abc::Uasr, Abc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Underflow Flag   UNF. Indicates if underflow error has occurred."]
    #[inline(always)]
    pub fn unf(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, abc::Unf, Abc_SPEC, crate::common::R> {
        crate::common::RegisterField::<23,0x1,1,0,abc::Unf, Abc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Underflow Flag Modify   UFM. Sets or clears the underflow flag UNF."]
    #[inline(always)]
    pub fn ufm(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, abc::Ufm, Abc_SPEC, crate::common::W> {
        crate::common::RegisterField::<24,0x3,1,0,abc::Ufm, Abc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Underflow Interrupt Enable   UIE. Enables or disables the path of the interrupt signal towards the        interrupt node. If enabled  an underflow event triggers an interrupt         and if disabled then not. The UNF flag always indicates the occurrence of an underflow event         independently of UIE."]
    #[inline(always)]
    pub fn uie(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, abc::Uie, Abc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x1,1,0,abc::Uie, Abc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Select   CLKSEL. Selects the clock source for the ABRA block."]
    #[inline(always)]
    pub fn clksel(
        self,
    ) -> crate::common::RegisterField<27, 0x7, 1, 0, abc::Clksel, Abc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x7,1,0,abc::Clksel, Abc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Asynchronous Block Bypass   ABB. Defines if the asynchronous block and the n divider of the MSC downstream path  located parallel to the fractional divider  are used or not. If bypassed  then also disabled."]
    #[inline(always)]
    pub fn abb(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, abc::Abb, Abc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<31,0x1,1,0,abc::Abb, Abc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Abc {
    #[inline(always)]
    fn default() -> Abc {
        <crate::RegValueT<Abc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod abc {
    pub struct Oip_SPEC;
    pub type Oip = crate::EnumBitfieldStruct<u8, Oip_SPEC>;
    impl Oip {
        #[doc = "00 Service request output SR0 selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Service request output SR1 selected"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Service request output SR2 selected"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Service request output SR3 selected"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Oasr_SPEC;
    pub type Oasr = crate::EnumBitfieldStruct<u8, Oasr_SPEC>;
    impl Oasr {
        #[doc = "0 SR Multiplexer selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Alternate Service Request Node  SR4  selected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ovf_SPEC;
    pub type Ovf = crate::EnumBitfieldStruct<u8, Ovf_SPEC>;
    impl Ovf {
        #[doc = "0 No overflow error"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Overflow error"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ofm_SPEC;
    pub type Ofm = crate::EnumBitfieldStruct<u8, Ofm_SPEC>;
    impl Ofm {
        #[doc = "00 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Set  and triggers the overflow interrupt if enabled"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Clear  additionally flushes the ABRA FIFO and resets the bit counter"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 No action"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Oie_SPEC;
    pub type Oie = crate::EnumBitfieldStruct<u8, Oie_SPEC>;
    impl Oie {
        #[doc = "0 Overflow interrupt disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Overflow interrupt enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Uip_SPEC;
    pub type Uip = crate::EnumBitfieldStruct<u8, Uip_SPEC>;
    impl Uip {
        #[doc = "00 Service request output SR0 selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Service request output SR1 selected"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Service request output SR2 selected"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Service request output SR3 selected"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Uasr_SPEC;
    pub type Uasr = crate::EnumBitfieldStruct<u8, Uasr_SPEC>;
    impl Uasr {
        #[doc = "0 SR Multiplexer selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Alternate Service Request Node  SR4  selected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Unf_SPEC;
    pub type Unf = crate::EnumBitfieldStruct<u8, Unf_SPEC>;
    impl Unf {
        #[doc = "0 No underflow error"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Underflow error"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ufm_SPEC;
    pub type Ufm = crate::EnumBitfieldStruct<u8, Ufm_SPEC>;
    impl Ufm {
        #[doc = "00 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Set  also triggers the underflow interrupt if enabled"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Clear  additionally flushes the ABRA FIFO and resets the bit counter"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 No action"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Uie_SPEC;
    pub type Uie = crate::EnumBitfieldStruct<u8, Uie_SPEC>;
    impl Uie {
        #[doc = "0 Underflow        interrupt disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Underflow        interrupt enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clksel_SPEC;
    pub type Clksel = crate::EnumBitfieldStruct<u8, Clksel_SPEC>;
    impl Clksel {
        #[doc = "000 no clock"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 f PER"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Abb_SPEC;
    pub type Abb = crate::EnumBitfieldStruct<u8, Abb_SPEC>;
    impl Abb {
        #[doc = "0 Bypassed and disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Not bypassed and active"]
        pub const CONST_11: Self = Self::new(1);
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
    #[doc = "Sleep Mode Enable Control   EDIS. Used to control module  8217 s sleep mode."]
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
pub struct Dc_SPEC;
impl crate::sealed::RegSpec for Dc_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Command Register\n resetvalue={Application Reset:0x0}"]
pub type Dc = crate::RegValueT<Dc_SPEC>;

impl Dc {
    #[doc = "Downstream Command for SRL Shift Register   DCL. Contains the data bits to be transmitted during the SRL active phase of a command frame."]
    #[inline(always)]
    pub fn dcl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Dc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Dc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Downstream Command for SRH Shift Register   DCH. Contains the data bits to be transmitted during the SRH active phase of a command frame."]
    #[inline(always)]
    pub fn dch(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Dc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Dc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dc {
    #[inline(always)]
    fn default() -> Dc {
        <crate::RegValueT<Dc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dce_SPEC;
impl crate::sealed::RegSpec for Dce_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Command Extension Register\n resetvalue={Application Reset:0x0}"]
pub type Dce = crate::RegValueT<Dce_SPEC>;

impl Dce {
    #[doc = "Downstream Command Extension for SRH Shift Register   DCEH. Contains the data bits to be transmitted during the second half of the command frame in CX mode."]
    #[inline(always)]
    pub fn dceh(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Dce_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Dce_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dce {
    #[inline(always)]
    fn default() -> Dce {
        <crate::RegValueT<Dce_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcm_SPEC;
impl crate::sealed::RegSpec for Dcm_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Command Mirror Register\n resetvalue={Application Reset:0x0}"]
pub type Dcm = crate::RegValueT<Dcm_SPEC>;

impl Dcm {
    #[doc = "Downstream Command Mirror of the DC.DCL Bit Field   DCLM. Contains alternate write location for the command bits to be transmitted during the SRL active phase of a command frame."]
    #[inline(always)]
    pub fn dclm(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Dcm_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Dcm_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Downstream Command Mirror of the DC.DCH Bit Field   DCHM. Contains the alternate write location for the command bits to be transmitted during the SRH active phase of a command frame."]
    #[inline(always)]
    pub fn dchm(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Dcm_SPEC, crate::common::W> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Dcm_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Dcm {
    #[inline(always)]
    fn default() -> Dcm {
        <crate::RegValueT<Dcm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dd_SPEC;
impl crate::sealed::RegSpec for Dd_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Data Register\n resetvalue={Application Reset:0x0}"]
pub type Dd = crate::RegValueT<Dd_SPEC>;

impl Dd {
    #[doc = "Downstream Data for SRL Shift Register   DDL. Contains the data bits to be transmitted during the SRL active phase of a data frame."]
    #[inline(always)]
    pub fn ddl(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Dd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Dd_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Downstream Data for SRH Shift Register   DDH. Contains the data bits to be transmitted during the SRH active phase of a data frame."]
    #[inline(always)]
    pub fn ddh(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Dd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Dd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dd {
    #[inline(always)]
    fn default() -> Dd {
        <crate::RegValueT<Dd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dde_SPEC;
impl crate::sealed::RegSpec for Dde_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Data Extension Register\n resetvalue={Application Reset:0x0}"]
pub type Dde = crate::RegValueT<Dde_SPEC>;

impl Dde {
    #[doc = "Downstream Data Extension for SRL Shift Register   DDLE. Contains the data bits to be transmitted during the SRL active phase of        a data frame."]
    #[inline(always)]
    pub fn ddle(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Dde_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Dde_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Downstream Data Extension for SRH Shift Register   DDHE. Contains the data bits to be transmitted during the SRH active phase of        a data frame."]
    #[inline(always)]
    pub fn ddhe(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Dde_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Dde_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dde {
    #[inline(always)]
    fn default() -> Dde {
        <crate::RegValueT<Dde_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ddm_SPEC;
impl crate::sealed::RegSpec for Ddm_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Data Mirror Register\n resetvalue={Application Reset:0x0}"]
pub type Ddm = crate::RegValueT<Ddm_SPEC>;

impl Ddm {
    #[doc = "Downstream Data Mirror for SRL Shift Register   DDLM. Contains the data bits to be transmitted during the SRL active phase of        a data frame."]
    #[inline(always)]
    pub fn ddlm(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Ddm_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Ddm_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Downstream Data Mirror for SRH Shift Register   DDHM. Contains the data bits to be transmitted during the SRH active phase of        a data frame."]
    #[inline(always)]
    pub fn ddhm(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Ddm_SPEC, crate::common::W> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Ddm_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Ddm {
    #[inline(always)]
    fn default() -> Ddm {
        <crate::RegValueT<Ddm_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsc_SPEC;
impl crate::sealed::RegSpec for Dsc_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Control Register\n resetvalue={Application Reset:0x0}"]
pub type Dsc = crate::RegValueT<Dsc_SPEC>;

impl Dsc {
    #[doc = "Transmission Mode   TM. This bit selects the transmission mode of the downstream channel."]
    #[inline(always)]
    pub fn tm(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, dsc::Tm, Dsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,dsc::Tm, Dsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Command Pending   CP. This bit is set when the downstream command register DC is written. CP        is cleared when the first bit of the related command frame is sent out."]
    #[inline(always)]
    pub fn cp(self) -> crate::common::RegisterFieldBool<1, 1, 0, Dsc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Dsc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Data Pending   DP. In Triggered Mode  this bit is set when the set data pending bit ISC.SDP        is set by software. In Data Repetition Mode  this bit is set by hardware        at the last passive time frame. At the start of the data frame  DP is        cleared by hardware."]
    #[inline(always)]
    pub fn dp(self) -> crate::common::RegisterFieldBool<2, 1, 0, Dsc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Dsc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable SRL Active Phase Selection Bit   ENSELL. This bit determines whether a low level selection bit is inserted at the        beginning of a data frame  8217 s SRL active phase."]
    #[inline(always)]
    pub fn ensell(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, dsc::Ensell, Dsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x1,1,0,dsc::Ensell, Dsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable SRH Active Phase Selection Bit   ENSELH. This bit determines whether a low level selection bit is inserted at the        beginning of a data frame  8217 s SRH active phase."]
    #[inline(always)]
    pub fn enselh(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, dsc::Enselh, Dsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x1,1,0,dsc::Enselh, Dsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Downstream Disable   DSDIS. This bit indicates the state of the downstream channel operation."]
    #[inline(always)]
    pub fn dsdis(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, dsc::Dsdis, Dsc_SPEC, crate::common::R> {
        crate::common::RegisterField::<15,0x1,1,0,dsc::Dsdis, Dsc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Number of Bits Shifted at Command Frames   NBC. This bit field determines how many bits of the SRL SRH shift registers        are shifted out during transmission of a command frame."]
    #[inline(always)]
    pub fn nbc(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, dsc::Nbc, Dsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3f,1,0,dsc::Nbc, Dsc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Passive Phase Length at Data Frames   PPD. This bit field determines the length of the passive phase of a data        frame."]
    #[inline(always)]
    pub fn ppd(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, dsc::Ppd, Dsc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,dsc::Ppd, Dsc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dsc {
    #[inline(always)]
    fn default() -> Dsc {
        <crate::RegValueT<Dsc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dsc {
    pub struct Tm_SPEC;
    pub type Tm = crate::EnumBitfieldStruct<u8, Tm_SPEC>;
    impl Tm {
        #[doc = "0 Triggered Mode        selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Data Repetition        Mode selected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ensell_SPEC;
    pub type Ensell = crate::EnumBitfieldStruct<u8, Ensell_SPEC>;
    impl Ensell {
        #[doc = "0 No selection bit inserted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Low level selection bit inserted."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enselh_SPEC;
    pub type Enselh = crate::EnumBitfieldStruct<u8, Enselh_SPEC>;
    impl Enselh {
        #[doc = "0 No selection bit inserted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Low level selection bit inserted."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Dsdis_SPEC;
    pub type Dsdis = crate::EnumBitfieldStruct<u8, Dsdis_SPEC>;
    impl Dsdis {
        #[doc = "0 The downstream channel is enabled. A frame transmission can take place  Triggered Mode  or takes place  Data Repetition Mode ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Downstream Counter becomes disabled. No new frame transmission is started. A running frame transmission is always completed."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Nbc_SPEC;
    pub type Nbc = crate::EnumBitfieldStruct<u8, Nbc_SPEC>;
    impl Nbc {
        #[doc = "000000 No bit        shifted"]
        pub const CONST_00: Self = Self::new(0);
    }
    pub struct Ppd_SPEC;
    pub type Ppd = crate::EnumBitfieldStruct<u8, Ppd_SPEC>;
    impl Ppd {
        #[doc = "Passive phase length is 2 x t FCL  . 00000"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "Passive phase length is 2 x t FCL  . 00001"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsce_SPEC;
impl crate::sealed::RegSpec for Dsce_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Control Enhanced Register 1\n resetvalue={Application Reset:0x0}"]
pub type Dsce = crate::RegValueT<Dsce_SPEC>;

impl Dsce {
    #[doc = "Number of SRH Bits Shifted at Data Frames Extension   NDBHE. Additional MSB bit extension of the NDBH bit field. Adds 16 to the        resulting NDBH value if set."]
    #[inline(always)]
    pub fn ndbhe(self) -> crate::common::RegisterFieldBool<0, 1, 0, Dsce_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Dsce_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Number of SRL Bits Shifted at Data Frames Extension   NDBLE. Additional MSB bit extension of the NDBL bit field. Adds 16 to the        resulting NDBL value if set."]
    #[inline(always)]
    pub fn ndble(self) -> crate::common::RegisterFieldBool<1, 1, 0, Dsce_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Dsce_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Extension Enable   EXEN. Enables the extension bit fields."]
    #[inline(always)]
    pub fn exen(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, dsce::Exen, Dsce_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x1,1,0,dsce::Exen, Dsce_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Command Command Flag   CCF. This bit flags that a second command frame has been written without data        frame to be sent in between. It is active only if CDCM 1. Otherwise it        is always low."]
    #[inline(always)]
    pub fn ccf(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, dsce::Ccf, Dsce_SPEC, crate::common::R> {
        crate::common::RegisterField::<15,0x1,1,0,dsce::Ccf, Dsce_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Injection Enable of the Pin 0 Signal   INJENP0. This bit selects if an external signal is injected in a data frame."]
    #[inline(always)]
    pub fn injenp0(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, dsce::Injenp0, Dsce_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,dsce::Injenp0, Dsce_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Injection Position of the Pin 0 Signal   INJPOSP0. This bit selects the position of the injected external one bit signal         at the bit position in range of 0 to 63 in the data frame. If both PIN0POS and PIN1POS point to a same bit  PIN0POS has the higher        priority  that is PIN0 level will be visible in the data frame."]
    #[inline(always)]
    pub fn injposp0(
        self,
    ) -> crate::common::RegisterField<17, 0x3f, 1, 0, u8, Dsce_SPEC, crate::common::RW> {
        crate::common::RegisterField::<17,0x3f,1,0,u8, Dsce_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Injection Enable of the Pin 1 Signal   INJENP1. This bit selects if an external signal is injected in a data frame."]
    #[inline(always)]
    pub fn injenp1(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, dsce::Injenp1, Dsce_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,dsce::Injenp1, Dsce_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Injection Position of the Pin 1 Signal   INJPOSP1. This bit selects the position of the injected external one bit signal         at the bit position in range of 0 to 63 in the data frame."]
    #[inline(always)]
    pub fn injposp1(
        self,
    ) -> crate::common::RegisterField<25, 0x3f, 1, 0, u8, Dsce_SPEC, crate::common::RW> {
        crate::common::RegisterField::<25,0x3f,1,0,u8, Dsce_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Command Data Command in Data Repetition Mode   CDCM. This bit selects if a data frame is automatically inserted between two        consecutive command frame requests."]
    #[inline(always)]
    pub fn cdcm(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, dsce::Cdcm, Dsce_SPEC, crate::common::RW> {
        crate::common::RegisterField::<31,0x1,1,0,dsce::Cdcm, Dsce_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dsce {
    #[inline(always)]
    fn default() -> Dsce {
        <crate::RegValueT<Dsce_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dsce {
    pub struct Exen_SPEC;
    pub type Exen = crate::EnumBitfieldStruct<u8, Exen_SPEC>;
    impl Exen {
        #[doc = "0 disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ccf_SPEC;
    pub type Ccf = crate::EnumBitfieldStruct<u8, Ccf_SPEC>;
    impl Ccf {
        #[doc = "0 no second command"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 second command frame pending"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Injenp0_SPEC;
    pub type Injenp0 = crate::EnumBitfieldStruct<u8, Injenp0_SPEC>;
    impl Injenp0 {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Injenp1_SPEC;
    pub type Injenp1 = crate::EnumBitfieldStruct<u8, Injenp1_SPEC>;
    impl Injenp1 {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cdcm_SPEC;
    pub type Cdcm = crate::EnumBitfieldStruct<u8, Cdcm_SPEC>;
    impl Cdcm {
        #[doc = "0 Automatic data insertion disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Automatic data insertion enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsdsh_SPEC;
impl crate::sealed::RegSpec for Dsdsh_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Select Data Source High Register\n resetvalue={Application Reset:0x0}"]
pub type Dsdsh = crate::RegValueT<Dsdsh_SPEC>;

impl Dsdsh {
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, dsdsh::Sh0, Dsdsh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,dsdsh::Sh0, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, dsdsh::Sh1, Dsdsh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,dsdsh::Sh1, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, dsdsh::Sh2, Dsdsh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,dsdsh::Sh2, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, dsdsh::Sh3, Dsdsh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,dsdsh::Sh3, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh4(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, dsdsh::Sh4, Dsdsh_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,dsdsh::Sh4, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh5(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, dsdsh::Sh5, Dsdsh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,dsdsh::Sh5, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh6(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, dsdsh::Sh6, Dsdsh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,dsdsh::Sh6, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh7(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, dsdsh::Sh7, Dsdsh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,dsdsh::Sh7, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh8(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, dsdsh::Sh8, Dsdsh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3,1,0,dsdsh::Sh8, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh9(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, dsdsh::Sh9, Dsdsh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x3,1,0,dsdsh::Sh9, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh10(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, dsdsh::Sh10, Dsdsh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3,1,0,dsdsh::Sh10, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh11(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, dsdsh::Sh11, Dsdsh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x3,1,0,dsdsh::Sh11, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh12(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, dsdsh::Sh12, Dsdsh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x3,1,0,dsdsh::Sh12, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh13(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, dsdsh::Sh13, Dsdsh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x3,1,0,dsdsh::Sh13, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh14(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, dsdsh::Sh14, Dsdsh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x3,1,0,dsdsh::Sh14, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH15. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh15(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, dsdsh::Sh15, Dsdsh_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x3,1,0,dsdsh::Sh15, Dsdsh_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dsdsh {
    #[inline(always)]
    fn default() -> Dsdsh {
        <crate::RegValueT<Dsdsh_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dsdsh {
    pub struct Sh0_SPEC;
    pub type Sh0 = crate::EnumBitfieldStruct<u8, Sh0_SPEC>;
    impl Sh0 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINH input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINH input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh1_SPEC;
    pub type Sh1 = crate::EnumBitfieldStruct<u8, Sh1_SPEC>;
    impl Sh1 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINH input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINH input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh2_SPEC;
    pub type Sh2 = crate::EnumBitfieldStruct<u8, Sh2_SPEC>;
    impl Sh2 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINH input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINH input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh3_SPEC;
    pub type Sh3 = crate::EnumBitfieldStruct<u8, Sh3_SPEC>;
    impl Sh3 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINH input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINH input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh4_SPEC;
    pub type Sh4 = crate::EnumBitfieldStruct<u8, Sh4_SPEC>;
    impl Sh4 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINH input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINH input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh5_SPEC;
    pub type Sh5 = crate::EnumBitfieldStruct<u8, Sh5_SPEC>;
    impl Sh5 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINH input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINH input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh6_SPEC;
    pub type Sh6 = crate::EnumBitfieldStruct<u8, Sh6_SPEC>;
    impl Sh6 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINH input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINH input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh7_SPEC;
    pub type Sh7 = crate::EnumBitfieldStruct<u8, Sh7_SPEC>;
    impl Sh7 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINH input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINH input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh8_SPEC;
    pub type Sh8 = crate::EnumBitfieldStruct<u8, Sh8_SPEC>;
    impl Sh8 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINH input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINH input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh9_SPEC;
    pub type Sh9 = crate::EnumBitfieldStruct<u8, Sh9_SPEC>;
    impl Sh9 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINH input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINH input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh10_SPEC;
    pub type Sh10 = crate::EnumBitfieldStruct<u8, Sh10_SPEC>;
    impl Sh10 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINH input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINH input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh11_SPEC;
    pub type Sh11 = crate::EnumBitfieldStruct<u8, Sh11_SPEC>;
    impl Sh11 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINH input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINH input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh12_SPEC;
    pub type Sh12 = crate::EnumBitfieldStruct<u8, Sh12_SPEC>;
    impl Sh12 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINH input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINH input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh13_SPEC;
    pub type Sh13 = crate::EnumBitfieldStruct<u8, Sh13_SPEC>;
    impl Sh13 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINH input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINH input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh14_SPEC;
    pub type Sh14 = crate::EnumBitfieldStruct<u8, Sh14_SPEC>;
    impl Sh14 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINH input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINH input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh15_SPEC;
    pub type Sh15 = crate::EnumBitfieldStruct<u8, Sh15_SPEC>;
    impl Sh15 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINH input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINH input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsdshe_SPEC;
impl crate::sealed::RegSpec for Dsdshe_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Select Data Source High Extension Register\n resetvalue={Application Reset:0x0}"]
pub type Dsdshe = crate::RegValueT<Dsdshe_SPEC>;

impl Dsdshe {
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh16(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, dsdshe::Sh16, Dsdshe_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,dsdshe::Sh16, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh17(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, dsdshe::Sh17, Dsdshe_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,dsdshe::Sh17, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh18(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, dsdshe::Sh18, Dsdshe_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,dsdshe::Sh18, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh19(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, dsdshe::Sh19, Dsdshe_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,dsdshe::Sh19, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh20(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, dsdshe::Sh20, Dsdshe_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,dsdshe::Sh20, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh21(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, dsdshe::Sh21, Dsdshe_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,dsdshe::Sh21, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh22(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, dsdshe::Sh22, Dsdshe_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,dsdshe::Sh22, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh23(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, dsdshe::Sh23, Dsdshe_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,dsdshe::Sh23, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh24(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, dsdshe::Sh24, Dsdshe_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3,1,0,dsdshe::Sh24, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh25(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, dsdshe::Sh25, Dsdshe_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x3,1,0,dsdshe::Sh25, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh26(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, dsdshe::Sh26, Dsdshe_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3,1,0,dsdshe::Sh26, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh27(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, dsdshe::Sh27, Dsdshe_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x3,1,0,dsdshe::Sh27, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh28(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, dsdshe::Sh28, Dsdshe_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x3,1,0,dsdshe::Sh28, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh29(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, dsdshe::Sh29, Dsdshe_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x3,1,0,dsdshe::Sh29, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh30(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, dsdshe::Sh30, Dsdshe_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x3,1,0,dsdshe::Sh30, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRH   SH31. SHx determines which data source is used for the shift register bit SRH x  during data frame transmission."]
    #[inline(always)]
    pub fn sh31(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, dsdshe::Sh31, Dsdshe_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x3,1,0,dsdshe::Sh31, Dsdshe_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dsdshe {
    #[inline(always)]
    fn default() -> Dsdshe {
        <crate::RegValueT<Dsdshe_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dsdshe {
    pub struct Sh16_SPEC;
    pub type Sh16 = crate::EnumBitfieldStruct<u8, Sh16_SPEC>;
    impl Sh16 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINHE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINHE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh17_SPEC;
    pub type Sh17 = crate::EnumBitfieldStruct<u8, Sh17_SPEC>;
    impl Sh17 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINHE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINHE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh18_SPEC;
    pub type Sh18 = crate::EnumBitfieldStruct<u8, Sh18_SPEC>;
    impl Sh18 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINHE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINHE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh19_SPEC;
    pub type Sh19 = crate::EnumBitfieldStruct<u8, Sh19_SPEC>;
    impl Sh19 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINHE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINHE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh20_SPEC;
    pub type Sh20 = crate::EnumBitfieldStruct<u8, Sh20_SPEC>;
    impl Sh20 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINHE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINHE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh21_SPEC;
    pub type Sh21 = crate::EnumBitfieldStruct<u8, Sh21_SPEC>;
    impl Sh21 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINHE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINHE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh22_SPEC;
    pub type Sh22 = crate::EnumBitfieldStruct<u8, Sh22_SPEC>;
    impl Sh22 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINHE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINHE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh23_SPEC;
    pub type Sh23 = crate::EnumBitfieldStruct<u8, Sh23_SPEC>;
    impl Sh23 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINHE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINHE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh24_SPEC;
    pub type Sh24 = crate::EnumBitfieldStruct<u8, Sh24_SPEC>;
    impl Sh24 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINHE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINHE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh25_SPEC;
    pub type Sh25 = crate::EnumBitfieldStruct<u8, Sh25_SPEC>;
    impl Sh25 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINHE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINHE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh26_SPEC;
    pub type Sh26 = crate::EnumBitfieldStruct<u8, Sh26_SPEC>;
    impl Sh26 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINHE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINHE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh27_SPEC;
    pub type Sh27 = crate::EnumBitfieldStruct<u8, Sh27_SPEC>;
    impl Sh27 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINHE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINHE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh28_SPEC;
    pub type Sh28 = crate::EnumBitfieldStruct<u8, Sh28_SPEC>;
    impl Sh28 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINHE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINHE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh29_SPEC;
    pub type Sh29 = crate::EnumBitfieldStruct<u8, Sh29_SPEC>;
    impl Sh29 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINHE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINHE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh30_SPEC;
    pub type Sh30 = crate::EnumBitfieldStruct<u8, Sh30_SPEC>;
    impl Sh30 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINHE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINHE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sh31_SPEC;
    pub type Sh31 = crate::EnumBitfieldStruct<u8, Sh31_SPEC>;
    impl Sh31 {
        #[doc = "00 SRH x  is taken from data register DD.DDH x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRH x  is taken from the ALTINHE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRH x  is taken from the ALTINHE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsdsl_SPEC;
impl crate::sealed::RegSpec for Dsdsl_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Select Data Source Low Register\n resetvalue={Application Reset:0x0}"]
pub type Dsdsl = crate::RegValueT<Dsdsl_SPEC>;

impl Dsdsl {
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, dsdsl::Sl0, Dsdsl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,dsdsl::Sl0, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, dsdsl::Sl1, Dsdsl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,dsdsl::Sl1, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, dsdsl::Sl2, Dsdsl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,dsdsl::Sl2, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, dsdsl::Sl3, Dsdsl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,dsdsl::Sl3, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl4(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, dsdsl::Sl4, Dsdsl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,dsdsl::Sl4, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl5(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, dsdsl::Sl5, Dsdsl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,dsdsl::Sl5, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl6(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, dsdsl::Sl6, Dsdsl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,dsdsl::Sl6, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl7(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, dsdsl::Sl7, Dsdsl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,dsdsl::Sl7, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl8(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, dsdsl::Sl8, Dsdsl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3,1,0,dsdsl::Sl8, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl9(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, dsdsl::Sl9, Dsdsl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x3,1,0,dsdsl::Sl9, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl10(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, dsdsl::Sl10, Dsdsl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3,1,0,dsdsl::Sl10, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl11(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, dsdsl::Sl11, Dsdsl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x3,1,0,dsdsl::Sl11, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl12(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, dsdsl::Sl12, Dsdsl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x3,1,0,dsdsl::Sl12, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl13(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, dsdsl::Sl13, Dsdsl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x3,1,0,dsdsl::Sl13, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl14(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, dsdsl::Sl14, Dsdsl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x3,1,0,dsdsl::Sl14, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL15. SLx determines which data source is used for the shift register bit SRL x  for data frame transmission."]
    #[inline(always)]
    pub fn sl15(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, dsdsl::Sl15, Dsdsl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x3,1,0,dsdsl::Sl15, Dsdsl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dsdsl {
    #[inline(always)]
    fn default() -> Dsdsl {
        <crate::RegValueT<Dsdsl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dsdsl {
    pub struct Sl0_SPEC;
    pub type Sl0 = crate::EnumBitfieldStruct<u8, Sl0_SPEC>;
    impl Sl0 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINL input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINL input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl1_SPEC;
    pub type Sl1 = crate::EnumBitfieldStruct<u8, Sl1_SPEC>;
    impl Sl1 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINL input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINL input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl2_SPEC;
    pub type Sl2 = crate::EnumBitfieldStruct<u8, Sl2_SPEC>;
    impl Sl2 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINL input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINL input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl3_SPEC;
    pub type Sl3 = crate::EnumBitfieldStruct<u8, Sl3_SPEC>;
    impl Sl3 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINL input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINL input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl4_SPEC;
    pub type Sl4 = crate::EnumBitfieldStruct<u8, Sl4_SPEC>;
    impl Sl4 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINL input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINL input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl5_SPEC;
    pub type Sl5 = crate::EnumBitfieldStruct<u8, Sl5_SPEC>;
    impl Sl5 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINL input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINL input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl6_SPEC;
    pub type Sl6 = crate::EnumBitfieldStruct<u8, Sl6_SPEC>;
    impl Sl6 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINL input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINL input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl7_SPEC;
    pub type Sl7 = crate::EnumBitfieldStruct<u8, Sl7_SPEC>;
    impl Sl7 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINL input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINL input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl8_SPEC;
    pub type Sl8 = crate::EnumBitfieldStruct<u8, Sl8_SPEC>;
    impl Sl8 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINL input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINL input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl9_SPEC;
    pub type Sl9 = crate::EnumBitfieldStruct<u8, Sl9_SPEC>;
    impl Sl9 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINL input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINL input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl10_SPEC;
    pub type Sl10 = crate::EnumBitfieldStruct<u8, Sl10_SPEC>;
    impl Sl10 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINL input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINL input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl11_SPEC;
    pub type Sl11 = crate::EnumBitfieldStruct<u8, Sl11_SPEC>;
    impl Sl11 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINL input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINL input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl12_SPEC;
    pub type Sl12 = crate::EnumBitfieldStruct<u8, Sl12_SPEC>;
    impl Sl12 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINL input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINL input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl13_SPEC;
    pub type Sl13 = crate::EnumBitfieldStruct<u8, Sl13_SPEC>;
    impl Sl13 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINL input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINL input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl14_SPEC;
    pub type Sl14 = crate::EnumBitfieldStruct<u8, Sl14_SPEC>;
    impl Sl14 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINL input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINL input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl15_SPEC;
    pub type Sl15 = crate::EnumBitfieldStruct<u8, Sl15_SPEC>;
    impl Sl15 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINL input line x."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINL input line x in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsdsle_SPEC;
impl crate::sealed::RegSpec for Dsdsle_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Select Data Source Low Extension Register\n resetvalue={Application Reset:0x0}"]
pub type Dsdsle = crate::RegValueT<Dsdsle_SPEC>;

impl Dsdsle {
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl16(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, dsdsle::Sl16, Dsdsle_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,dsdsle::Sl16, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl17(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, dsdsle::Sl17, Dsdsle_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,dsdsle::Sl17, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl18(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, dsdsle::Sl18, Dsdsle_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,dsdsle::Sl18, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl19(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, dsdsle::Sl19, Dsdsle_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,dsdsle::Sl19, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl20(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, dsdsle::Sl20, Dsdsle_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,dsdsle::Sl20, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl21(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, dsdsle::Sl21, Dsdsle_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,dsdsle::Sl21, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl22(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, dsdsle::Sl22, Dsdsle_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x3,1,0,dsdsle::Sl22, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl23(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, dsdsle::Sl23, Dsdsle_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,dsdsle::Sl23, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl24(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, dsdsle::Sl24, Dsdsle_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3,1,0,dsdsle::Sl24, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl25(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, dsdsle::Sl25, Dsdsle_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x3,1,0,dsdsle::Sl25, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl26(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, dsdsle::Sl26, Dsdsle_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3,1,0,dsdsle::Sl26, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl27(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, dsdsle::Sl27, Dsdsle_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x3,1,0,dsdsle::Sl27, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl28(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, dsdsle::Sl28, Dsdsle_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x3,1,0,dsdsle::Sl28, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl29(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, dsdsle::Sl29, Dsdsle_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x3,1,0,dsdsle::Sl29, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl30(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, dsdsle::Sl30, Dsdsle_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x3,1,0,dsdsle::Sl30, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Source for SRL   SL31. SLx determines which data source is used for the shift register bit SRL x  during data frame transmission."]
    #[inline(always)]
    pub fn sl31(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, dsdsle::Sl31, Dsdsle_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x3,1,0,dsdsle::Sl31, Dsdsle_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dsdsle {
    #[inline(always)]
    fn default() -> Dsdsle {
        <crate::RegValueT<Dsdsle_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dsdsle {
    pub struct Sl16_SPEC;
    pub type Sl16 = crate::EnumBitfieldStruct<u8, Sl16_SPEC>;
    impl Sl16 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINLE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINLE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl17_SPEC;
    pub type Sl17 = crate::EnumBitfieldStruct<u8, Sl17_SPEC>;
    impl Sl17 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINLE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINLE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl18_SPEC;
    pub type Sl18 = crate::EnumBitfieldStruct<u8, Sl18_SPEC>;
    impl Sl18 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINLE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINLE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl19_SPEC;
    pub type Sl19 = crate::EnumBitfieldStruct<u8, Sl19_SPEC>;
    impl Sl19 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINLE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINLE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl20_SPEC;
    pub type Sl20 = crate::EnumBitfieldStruct<u8, Sl20_SPEC>;
    impl Sl20 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINLE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINLE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl21_SPEC;
    pub type Sl21 = crate::EnumBitfieldStruct<u8, Sl21_SPEC>;
    impl Sl21 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINLE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINLE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl22_SPEC;
    pub type Sl22 = crate::EnumBitfieldStruct<u8, Sl22_SPEC>;
    impl Sl22 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINLE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINLE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl23_SPEC;
    pub type Sl23 = crate::EnumBitfieldStruct<u8, Sl23_SPEC>;
    impl Sl23 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINLE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINLE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl24_SPEC;
    pub type Sl24 = crate::EnumBitfieldStruct<u8, Sl24_SPEC>;
    impl Sl24 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINLE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINLE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl25_SPEC;
    pub type Sl25 = crate::EnumBitfieldStruct<u8, Sl25_SPEC>;
    impl Sl25 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINLE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINLE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl26_SPEC;
    pub type Sl26 = crate::EnumBitfieldStruct<u8, Sl26_SPEC>;
    impl Sl26 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINLE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINLE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl27_SPEC;
    pub type Sl27 = crate::EnumBitfieldStruct<u8, Sl27_SPEC>;
    impl Sl27 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINLE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINLE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl28_SPEC;
    pub type Sl28 = crate::EnumBitfieldStruct<u8, Sl28_SPEC>;
    impl Sl28 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINLE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINLE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl29_SPEC;
    pub type Sl29 = crate::EnumBitfieldStruct<u8, Sl29_SPEC>;
    impl Sl29 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINLE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINLE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl30_SPEC;
    pub type Sl30 = crate::EnumBitfieldStruct<u8, Sl30_SPEC>;
    impl Sl30 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINLE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINLE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sl31_SPEC;
    pub type Sl31 = crate::EnumBitfieldStruct<u8, Sl31_SPEC>;
    impl Sl31 {
        #[doc = "00 SRL x  is taken from data register DD.DDL x ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "10 SRL x  is taken from the ALTINLE 15 0  input line x 16."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 SRL x  is taken from the ALTINLE 15 0  input line x 16 in inverted state."]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dss_SPEC;
impl crate::sealed::RegSpec for Dss_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Status Register\n resetvalue={Application Reset:0x0}"]
pub type Dss = crate::RegValueT<Dss_SPEC>;

impl Dss {
    #[doc = "Passive Time Frame Counter   PFC. In Data Repetition Mode  this bit field indicates the count of passive        time frames that are currently transmitted. In Triggered Mode PFC        remains at 0000 B ."]
    #[inline(always)]
    pub fn pfc(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, dss::Pfc, Dss_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,dss::Pfc, Dss_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Number Of Passive Time Frames   NPTF. This bit field indicates the number of passive time frames that are        inserted in Data Repetition Mode between two data frames."]
    #[inline(always)]
    pub fn nptf(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, dss::Nptf, Dss_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,dss::Nptf, Dss_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Downstream Counter   DC. This bit field indicates the number of downstream shift clock periods        that have been elapsed since the start of the current frame. DC is reset        at the end of a downstream frame."]
    #[inline(always)]
    pub fn dc(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, dss::Dc, Dss_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,dss::Dc, Dss_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Data Frame Active   DFA. This bit indicates if a data frame is currently sent out  active phase only."]
    #[inline(always)]
    pub fn dfa(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, dss::Dfa, Dss_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x1,1,0,dss::Dfa, Dss_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Command Frame Active   CFA. This bit indicates if a command frame is currently sent out  active phase only."]
    #[inline(always)]
    pub fn cfa(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, dss::Cfa, Dss_SPEC, crate::common::R> {
        crate::common::RegisterField::<25,0x1,1,0,dss::Cfa, Dss_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Dss {
    #[inline(always)]
    fn default() -> Dss {
        <crate::RegValueT<Dss_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dss {
    pub struct Pfc_SPEC;
    pub type Pfc = crate::EnumBitfieldStruct<u8, Pfc_SPEC>;
    impl Pfc {
        #[doc = "0000 Data frame        is transmitted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "0001 First        passive time frame is transmitted."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Nptf_SPEC;
    pub type Nptf = crate::EnumBitfieldStruct<u8, Nptf_SPEC>;
    impl Nptf {
        #[doc = "0000 No passive        time frame inserted."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 passive time frame inserted."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Dc_SPEC;
    pub type Dc = crate::EnumBitfieldStruct<u8, Dc_SPEC>;
    impl Dc {
        #[doc = "00 No shift clock        elapsed  after counter reset ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 1 shift clock        elapsed."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Dfa_SPEC;
    pub type Dfa = crate::EnumBitfieldStruct<u8, Dfa_SPEC>;
    impl Dfa {
        #[doc = "0 No data frame is currently sent out."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A data frame is currently sent out."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cfa_SPEC;
    pub type Cfa = crate::EnumBitfieldStruct<u8, Cfa_SPEC>;
    impl Cfa {
        #[doc = "0 No command frame is currently sent out."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A command frame is currently sent out."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dste_SPEC;
impl crate::sealed::RegSpec for Dste_SPEC {
    type DataType = u32;
}
#[doc = "Downstream Timing Extension Register\n resetvalue={Application Reset:0x0}"]
pub type Dste = crate::RegValueT<Dste_SPEC>;

impl Dste {
    #[doc = "Passive Phase Length at Data Frames Extension   PPDE. Additional MSB bits extension of the PPD bit field."]
    #[inline(always)]
    pub fn ppde(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Dste_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Dste_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Passive Phase Length at Control Frames Extension   PPCE. Additional MSB bits extension of the fixed length of 2 of the command        frames passive phase. The final length is 2  160    160 PPCE resulting in a range        of 2 to 65."]
    #[inline(always)]
    pub fn ppce(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, Dste_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8, Dste_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "N Divider Downstream   NDD. Defines the division ratio in the range of 1 to 16."]
    #[inline(always)]
    pub fn ndd(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Dste_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Dste_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PPCE Extension Bit on the MSB Side   PPCEM. This bit is the MSB extension bit for the PPCE bit field  extending        command frame passive phase to the value of 127. The values of 128 and        129 are not allowed."]
    #[inline(always)]
    pub fn ppcem(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, dste::Ppcem, Dste_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,dste::Ppcem, Dste_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Fast Mode   FM. Activates the fast mode and writing to the ABRA FIFO with 100  160 MBaud        input baud rate  in order to provide 80MBaud output baud rate. It is        also recommended for baud rate of 26 67  160 MBaud  as defined in the CROSSREFERENCE .        FM 0 is the compatibility setting with the previous generations of the        MSC."]
    #[inline(always)]
    pub fn fm(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, dste::Fm, Dste_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1,1,0,dste::Fm, Dste_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Command Extension Mode   CX. Activates 64 bit command frame feature."]
    #[inline(always)]
    pub fn cx(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, dste::Cx, Dste_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x1,1,0,dste::Cx, Dste_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Unlock CX and FM for one write access   UL1. Write access with UL1  160    160 1 unlocks the CX and FM bit fields for the        current write access. Write accesses to these bit fields with UL1  160    160 0 do        not have any effect. Returns zero on read."]
    #[inline(always)]
    pub fn ul1(self) -> crate::common::RegisterFieldBool<31, 1, 0, Dste_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Dste_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Dste {
    #[inline(always)]
    fn default() -> Dste {
        <crate::RegValueT<Dste_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dste {
    pub struct Ppcem_SPEC;
    pub type Ppcem = crate::EnumBitfieldStruct<u8, Ppcem_SPEC>;
    impl Ppcem {
        #[doc = "0 MSB value of 0"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MSB value of 1"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Fm_SPEC;
    pub type Fm = crate::EnumBitfieldStruct<u8, Fm_SPEC>;
    impl Fm {
        #[doc = "0 Fast Mode deactivated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Fast Mode activated"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cx_SPEC;
    pub type Cx = crate::EnumBitfieldStruct<u8, Cx_SPEC>;
    impl Cx {
        #[doc = "0 CX mode deactivated"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 CX mode activated"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Esr_SPEC;
impl crate::sealed::RegSpec for Esr_SPEC {
    type DataType = u32;
}
#[doc = "Emergency Stop Register\n resetvalue={Application Reset:0x0}"]
pub type Esr = crate::RegValueT<Esr_SPEC>;

impl Esr {
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, esr::Enl0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,esr::Enl0, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, esr::Enl1, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,esr::Enl1, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, esr::Enl2, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,esr::Enl2, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, esr::Enl3, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,esr::Enl3, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, esr::Enl4, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,esr::Enl4, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, esr::Enl5, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,esr::Enl5, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, esr::Enl6, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,esr::Enl6, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, esr::Enl7, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,esr::Enl7, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, esr::Enl8, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,esr::Enl8, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, esr::Enl9, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,esr::Enl9, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, esr::Enl10, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x1,1,0,esr::Enl10, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, esr::Enl11, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x1,1,0,esr::Enl11, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, esr::Enl12, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x1,1,0,esr::Enl12, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, esr::Enl13, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x1,1,0,esr::Enl13, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, esr::Enl14, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x1,1,0,esr::Enl14, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRL   ENL15. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, esr::Enl15, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<15,0x1,1,0,esr::Enl15, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh0(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, esr::Enh0, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1,1,0,esr::Enh0, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh1(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, esr::Enh1, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<17,0x1,1,0,esr::Enh1, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh2(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, esr::Enh2, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x1,1,0,esr::Enh2, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh3(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, esr::Enh3, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<19,0x1,1,0,esr::Enh3, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh4(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, esr::Enh4, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x1,1,0,esr::Enh4, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh5(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, esr::Enh5, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x1,1,0,esr::Enh5, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh6(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, esr::Enh6, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x1,1,0,esr::Enh6, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh7(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, esr::Enh7, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<23,0x1,1,0,esr::Enh7, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh8(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, esr::Enh8, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1,1,0,esr::Enh8, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh9(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, esr::Enh9, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<25,0x1,1,0,esr::Enh9, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh10(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, esr::Enh10, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x1,1,0,esr::Enh10, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh11(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, esr::Enh11, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<27,0x1,1,0,esr::Enh11, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh12(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, esr::Enh12, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x1,1,0,esr::Enh12, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh13(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, esr::Enh13, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x1,1,0,esr::Enh13, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh14(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, esr::Enh14, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x1,1,0,esr::Enh14, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 15 in SRH   ENH15. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh15(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, esr::Enh15, Esr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<31,0x1,1,0,esr::Enh15, Esr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Esr {
    #[inline(always)]
    fn default() -> Esr {
        <crate::RegValueT<Esr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod esr {
    pub struct Enl0_SPEC;
    pub type Enl0 = crate::EnumBitfieldStruct<u8, Enl0_SPEC>;
    impl Enl0 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl1_SPEC;
    pub type Enl1 = crate::EnumBitfieldStruct<u8, Enl1_SPEC>;
    impl Enl1 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl2_SPEC;
    pub type Enl2 = crate::EnumBitfieldStruct<u8, Enl2_SPEC>;
    impl Enl2 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl3_SPEC;
    pub type Enl3 = crate::EnumBitfieldStruct<u8, Enl3_SPEC>;
    impl Enl3 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl4_SPEC;
    pub type Enl4 = crate::EnumBitfieldStruct<u8, Enl4_SPEC>;
    impl Enl4 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl5_SPEC;
    pub type Enl5 = crate::EnumBitfieldStruct<u8, Enl5_SPEC>;
    impl Enl5 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl6_SPEC;
    pub type Enl6 = crate::EnumBitfieldStruct<u8, Enl6_SPEC>;
    impl Enl6 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl7_SPEC;
    pub type Enl7 = crate::EnumBitfieldStruct<u8, Enl7_SPEC>;
    impl Enl7 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl8_SPEC;
    pub type Enl8 = crate::EnumBitfieldStruct<u8, Enl8_SPEC>;
    impl Enl8 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl9_SPEC;
    pub type Enl9 = crate::EnumBitfieldStruct<u8, Enl9_SPEC>;
    impl Enl9 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl10_SPEC;
    pub type Enl10 = crate::EnumBitfieldStruct<u8, Enl10_SPEC>;
    impl Enl10 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl11_SPEC;
    pub type Enl11 = crate::EnumBitfieldStruct<u8, Enl11_SPEC>;
    impl Enl11 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl12_SPEC;
    pub type Enl12 = crate::EnumBitfieldStruct<u8, Enl12_SPEC>;
    impl Enl12 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl13_SPEC;
    pub type Enl13 = crate::EnumBitfieldStruct<u8, Enl13_SPEC>;
    impl Enl13 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl14_SPEC;
    pub type Enl14 = crate::EnumBitfieldStruct<u8, Enl14_SPEC>;
    impl Enl14 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl15_SPEC;
    pub type Enl15 = crate::EnumBitfieldStruct<u8, Enl15_SPEC>;
    impl Enl15 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh0_SPEC;
    pub type Enh0 = crate::EnumBitfieldStruct<u8, Enh0_SPEC>;
    impl Enh0 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh1_SPEC;
    pub type Enh1 = crate::EnumBitfieldStruct<u8, Enh1_SPEC>;
    impl Enh1 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh2_SPEC;
    pub type Enh2 = crate::EnumBitfieldStruct<u8, Enh2_SPEC>;
    impl Enh2 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh3_SPEC;
    pub type Enh3 = crate::EnumBitfieldStruct<u8, Enh3_SPEC>;
    impl Enh3 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh4_SPEC;
    pub type Enh4 = crate::EnumBitfieldStruct<u8, Enh4_SPEC>;
    impl Enh4 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh5_SPEC;
    pub type Enh5 = crate::EnumBitfieldStruct<u8, Enh5_SPEC>;
    impl Enh5 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh6_SPEC;
    pub type Enh6 = crate::EnumBitfieldStruct<u8, Enh6_SPEC>;
    impl Enh6 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh7_SPEC;
    pub type Enh7 = crate::EnumBitfieldStruct<u8, Enh7_SPEC>;
    impl Enh7 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh8_SPEC;
    pub type Enh8 = crate::EnumBitfieldStruct<u8, Enh8_SPEC>;
    impl Enh8 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh9_SPEC;
    pub type Enh9 = crate::EnumBitfieldStruct<u8, Enh9_SPEC>;
    impl Enh9 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh10_SPEC;
    pub type Enh10 = crate::EnumBitfieldStruct<u8, Enh10_SPEC>;
    impl Enh10 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh11_SPEC;
    pub type Enh11 = crate::EnumBitfieldStruct<u8, Enh11_SPEC>;
    impl Enh11 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh12_SPEC;
    pub type Enh12 = crate::EnumBitfieldStruct<u8, Enh12_SPEC>;
    impl Enh12 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh13_SPEC;
    pub type Enh13 = crate::EnumBitfieldStruct<u8, Enh13_SPEC>;
    impl Enh13 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh14_SPEC;
    pub type Enh14 = crate::EnumBitfieldStruct<u8, Enh14_SPEC>;
    impl Enh14 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh15_SPEC;
    pub type Enh15 = crate::EnumBitfieldStruct<u8, Enh15_SPEC>;
    impl Enh15 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Esre_SPEC;
impl crate::sealed::RegSpec for Esre_SPEC {
    type DataType = u32;
}
#[doc = "Emergency Stop Extension Register\n resetvalue={Application Reset:0x0}"]
pub type Esre = crate::RegValueT<Esre_SPEC>;

impl Esre {
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl16(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, esre::Enl16, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,esre::Enl16, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl17(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, esre::Enl17, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,esre::Enl17, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl18(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, esre::Enl18, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,esre::Enl18, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl19(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, esre::Enl19, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,esre::Enl19, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl20(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, esre::Enl20, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,esre::Enl20, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl21(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, esre::Enl21, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,esre::Enl21, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl22(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, esre::Enl22, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,esre::Enl22, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl23(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, esre::Enl23, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,esre::Enl23, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl24(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, esre::Enl24, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,esre::Enl24, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl25(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, esre::Enl25, Esre_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,esre::Enl25, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl26(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, esre::Enl26, Esre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,esre::Enl26, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl27(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, esre::Enl27, Esre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,esre::Enl27, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl28(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, esre::Enl28, Esre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,esre::Enl28, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl29(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, esre::Enl29, Esre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,esre::Enl29, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl30(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, esre::Enl30, Esre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,esre::Enl30, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRL   ENL31. This bit enables the emergency stop feature selectively for each SRL bit. If the emergency stop condition is met and enabled  ENLx   1   the SRL x  bit is of the data register DD.DDL x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enl31(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, esre::Enl31, Esre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,esre::Enl31, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh16(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, esre::Enh16, Esre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,esre::Enh16, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, esre::Enh17, Esre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,esre::Enh17, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh18(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, esre::Enh18, Esre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,esre::Enh18, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh19(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, esre::Enh19, Esre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,esre::Enh19, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh20(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, esre::Enh20, Esre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,esre::Enh20, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh21(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, esre::Enh21, Esre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,esre::Enh21, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, esre::Enh22, Esre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,esre::Enh22, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, esre::Enh23, Esre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,esre::Enh23, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, esre::Enh24, Esre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,esre::Enh24, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh25(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, esre::Enh25, Esre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,esre::Enh25, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh26(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, esre::Enh26, Esre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,esre::Enh26, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh27(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, esre::Enh27, Esre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,esre::Enh27, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh28(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, esre::Enh28, Esre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,esre::Enh28, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh29(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, esre::Enh29, Esre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,esre::Enh29, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, esre::Enh30, Esre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,esre::Enh30, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Emergency Stop Enable for Bit 31 in SRH   ENH31. This bit enables the emergency stop feature selectively for each SRH bit. If the emergency stop condition is met and enabled  ENHx   1   the SRH x  bit of the data register DD.DDH x  is used for the shift register load operation."]
    #[inline(always)]
    pub fn enh31(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, esre::Enh31, Esre_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,esre::Enh31, Esre_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Esre {
    #[inline(always)]
    fn default() -> Esre {
        <crate::RegValueT<Esre_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod esre {
    pub struct Enl16_SPEC;
    pub type Enl16 = crate::EnumBitfieldStruct<u8, Enl16_SPEC>;
    impl Enl16 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl17_SPEC;
    pub type Enl17 = crate::EnumBitfieldStruct<u8, Enl17_SPEC>;
    impl Enl17 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl18_SPEC;
    pub type Enl18 = crate::EnumBitfieldStruct<u8, Enl18_SPEC>;
    impl Enl18 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl19_SPEC;
    pub type Enl19 = crate::EnumBitfieldStruct<u8, Enl19_SPEC>;
    impl Enl19 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl20_SPEC;
    pub type Enl20 = crate::EnumBitfieldStruct<u8, Enl20_SPEC>;
    impl Enl20 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl21_SPEC;
    pub type Enl21 = crate::EnumBitfieldStruct<u8, Enl21_SPEC>;
    impl Enl21 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl22_SPEC;
    pub type Enl22 = crate::EnumBitfieldStruct<u8, Enl22_SPEC>;
    impl Enl22 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl23_SPEC;
    pub type Enl23 = crate::EnumBitfieldStruct<u8, Enl23_SPEC>;
    impl Enl23 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl24_SPEC;
    pub type Enl24 = crate::EnumBitfieldStruct<u8, Enl24_SPEC>;
    impl Enl24 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl25_SPEC;
    pub type Enl25 = crate::EnumBitfieldStruct<u8, Enl25_SPEC>;
    impl Enl25 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl26_SPEC;
    pub type Enl26 = crate::EnumBitfieldStruct<u8, Enl26_SPEC>;
    impl Enl26 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl27_SPEC;
    pub type Enl27 = crate::EnumBitfieldStruct<u8, Enl27_SPEC>;
    impl Enl27 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl28_SPEC;
    pub type Enl28 = crate::EnumBitfieldStruct<u8, Enl28_SPEC>;
    impl Enl28 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl29_SPEC;
    pub type Enl29 = crate::EnumBitfieldStruct<u8, Enl29_SPEC>;
    impl Enl29 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl30_SPEC;
    pub type Enl30 = crate::EnumBitfieldStruct<u8, Enl30_SPEC>;
    impl Enl30 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enl31_SPEC;
    pub type Enl31 = crate::EnumBitfieldStruct<u8, Enl31_SPEC>;
    impl Enl31 {
        #[doc = "0 Emergency stop feature for bit SRL x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRL x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh16_SPEC;
    pub type Enh16 = crate::EnumBitfieldStruct<u8, Enh16_SPEC>;
    impl Enh16 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh17_SPEC;
    pub type Enh17 = crate::EnumBitfieldStruct<u8, Enh17_SPEC>;
    impl Enh17 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh18_SPEC;
    pub type Enh18 = crate::EnumBitfieldStruct<u8, Enh18_SPEC>;
    impl Enh18 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh19_SPEC;
    pub type Enh19 = crate::EnumBitfieldStruct<u8, Enh19_SPEC>;
    impl Enh19 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh20_SPEC;
    pub type Enh20 = crate::EnumBitfieldStruct<u8, Enh20_SPEC>;
    impl Enh20 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh21_SPEC;
    pub type Enh21 = crate::EnumBitfieldStruct<u8, Enh21_SPEC>;
    impl Enh21 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh22_SPEC;
    pub type Enh22 = crate::EnumBitfieldStruct<u8, Enh22_SPEC>;
    impl Enh22 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh23_SPEC;
    pub type Enh23 = crate::EnumBitfieldStruct<u8, Enh23_SPEC>;
    impl Enh23 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh24_SPEC;
    pub type Enh24 = crate::EnumBitfieldStruct<u8, Enh24_SPEC>;
    impl Enh24 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh25_SPEC;
    pub type Enh25 = crate::EnumBitfieldStruct<u8, Enh25_SPEC>;
    impl Enh25 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh26_SPEC;
    pub type Enh26 = crate::EnumBitfieldStruct<u8, Enh26_SPEC>;
    impl Enh26 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh27_SPEC;
    pub type Enh27 = crate::EnumBitfieldStruct<u8, Enh27_SPEC>;
    impl Enh27 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh28_SPEC;
    pub type Enh28 = crate::EnumBitfieldStruct<u8, Enh28_SPEC>;
    impl Enh28 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh29_SPEC;
    pub type Enh29 = crate::EnumBitfieldStruct<u8, Enh29_SPEC>;
    impl Enh29 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh30_SPEC;
    pub type Enh30 = crate::EnumBitfieldStruct<u8, Enh30_SPEC>;
    impl Enh30 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Enh31_SPEC;
    pub type Enh31 = crate::EnumBitfieldStruct<u8, Enh31_SPEC>;
    impl Enh31 {
        #[doc = "0 Emergency stop feature for bit SRH x  is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The emergency stop feature for bit SRH x  is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdr_SPEC;
impl crate::sealed::RegSpec for Fdr_SPEC {
    type DataType = u32;
}
#[doc = "Fractional Divider Register\n resetvalue={Application Reset:0x0}"]
pub type Fdr = crate::RegValueT<Fdr_SPEC>;

impl Fdr {
    #[doc = "Step Value   STEP. Reload or addition value for RESULT."]
    #[inline(always)]
    pub fn step(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Fdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Fdr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Divider Mode   DM. DM selects normal or fractional divider mode."]
    #[inline(always)]
    pub fn dm(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Fdr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Fdr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Result Value   RESULT. Bit field for the addition result."]
    #[inline(always)]
    pub fn result(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Fdr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Fdr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Enable Hardware Clock Control   ENHW. Controls operation of ECEN input and DISCLK bit."]
    #[inline(always)]
    pub fn enhw(self) -> crate::common::RegisterFieldBool<30, 1, 0, Fdr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Fdr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable Clock   DISCLK. Hardware controlled disable for f MSC signal."]
    #[inline(always)]
    pub fn disclk(self) -> crate::common::RegisterFieldBool<31, 1, 0, Fdr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Fdr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Fdr {
    #[inline(always)]
    fn default() -> Fdr {
        <crate::RegValueT<Fdr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr_SPEC;
impl crate::sealed::RegSpec for Icr_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
pub type Icr = crate::RegValueT<Icr_SPEC>;

impl Icr {
    #[doc = "Data Frame Interrupt Node Pointer   EDIP. EDIP selects the service request output line SRn  n   0 3  for the data frame interrupt."]
    #[inline(always)]
    pub fn edip(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, icr::Edip, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,icr::Edip, Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data Frame Interrupt Enable   EDIE. This bit field determines the enable conditions for the data frame interrupt."]
    #[inline(always)]
    pub fn edie(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, icr::Edie, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,icr::Edie, Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Command Frame Interrupt Node Pointer   ECIP. ECIP selects the service request output line SRn  n   0 3  for the command frame interrupt."]
    #[inline(always)]
    pub fn ecip(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, icr::Ecip, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,icr::Ecip, Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Command Frame Interrupt Enable   ECIE. This bit enables the command frame interrupt."]
    #[inline(always)]
    pub fn ecie(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, icr::Ecie, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,icr::Ecie, Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Time Frame Interrupt Pointer   TFIP. TFIP selects the service request output line SRn  n   0 3  for the time frame interrupt."]
    #[inline(always)]
    pub fn tfip(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, icr::Tfip, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,icr::Tfip, Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Time Frame Interrupt Enable   TFIE. This bit enables the time frame interrupt."]
    #[inline(always)]
    pub fn tfie(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, icr::Tfie, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x1,1,0,icr::Tfie, Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive Data Interrupt Pointer   RDIP. RDIP selects the service request output line SRn  n   0 3  for the receive data interrupt."]
    #[inline(always)]
    pub fn rdip(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, icr::Rdip, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,icr::Rdip, Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Receive Data Interrupt Enable   RDIE. This bit field determines the enable conditions for the receive data interrupt."]
    #[inline(always)]
    pub fn rdie(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, icr::Rdie, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,icr::Rdie, Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        <crate::RegValueT<Icr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icr {
    pub struct Edip_SPEC;
    pub type Edip = crate::EnumBitfieldStruct<u8, Edip_SPEC>;
    impl Edip {
        #[doc = "00 Service request output SR0 selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Service request output SR1 selected"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Service request output SR2 selected"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Service request output SR3 selected"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Edie_SPEC;
    pub type Edie = crate::EnumBitfieldStruct<u8, Edie_SPEC>;
    impl Edie {
        #[doc = "00 Interrupt generation disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 An interrupt is generated when the last data bit has been shifted out."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 An interrupt is generated when the first data bit has been shifted out  but only if DSC.NDBL is not equal 00000 B . This means  at least one SRL bit must be shifted out for the first data bit shifted interrupt to become active."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Interrupt generation disabled"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Ecip_SPEC;
    pub type Ecip = crate::EnumBitfieldStruct<u8, Ecip_SPEC>;
    impl Ecip {
        #[doc = "00 Service request output SR0 selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Service request output SR1 selected"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Service request output SR2 selected"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Service request output SR3 selected"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Ecie_SPEC;
    pub type Ecie = crate::EnumBitfieldStruct<u8, Ecie_SPEC>;
    impl Ecie {
        #[doc = "0 Interrupt generation disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt generation enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tfip_SPEC;
    pub type Tfip = crate::EnumBitfieldStruct<u8, Tfip_SPEC>;
    impl Tfip {
        #[doc = "00 Service request output SR0 selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Service request output SR1 selected"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Service request output SR2 selected"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Service request output SR3 selected"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Tfie_SPEC;
    pub type Tfie = crate::EnumBitfieldStruct<u8, Tfie_SPEC>;
    impl Tfie {
        #[doc = "0 Interrupt generation disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt generation enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rdip_SPEC;
    pub type Rdip = crate::EnumBitfieldStruct<u8, Rdip_SPEC>;
    impl Rdip {
        #[doc = "00 Service request output SR0 selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Service request output SR1 selected"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Service request output SR2 selected"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Service request output SR3 selected"]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Rdie_SPEC;
    pub type Rdie = crate::EnumBitfieldStruct<u8, Rdie_SPEC>;
    impl Rdie {
        #[doc = "00 Interrupt generation disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 An interrupt is generated when data is received and written into the upstream data registers UDx  x   0 3 ."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 An interrupt is generated as with RDIE   01 B but only if the received data is not equal to 00 H ."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 An interrupt is generated when data is received and written into register UD3."]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x28C010}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MODREV. This bit field defines the module revision number. The value of a module        revision starts with 01 H  first        revision ."]
    #[inline(always)]
    pub fn modrev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MODTYPE. This bit field defines the module as a 32 bit module  C0 H"]
    #[inline(always)]
    pub fn modtype(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MODNUM. This bit field defines the module identification number for the MSC          0028 H"]
    #[inline(always)]
    pub fn modnum(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Id_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Id {
    #[inline(always)]
    fn default() -> Id {
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(2670608)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isc_SPEC;
impl crate::sealed::RegSpec for Isc_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Set Clear Register\n resetvalue={Application Reset:0x0}"]
pub type Isc = crate::RegValueT<Isc_SPEC>;

impl Isc {
    #[doc = "Clear DEDI Flag   CDEDI"]
    #[inline(always)]
    pub fn cdedi(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, isc::Cdedi, Isc_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0x1,1,0,isc::Cdedi, Isc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear DECI Flag   CDECI"]
    #[inline(always)]
    pub fn cdeci(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, isc::Cdeci, Isc_SPEC, crate::common::W> {
        crate::common::RegisterField::<1,0x1,1,0,isc::Cdeci, Isc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear DTFI Flag   CDTFI"]
    #[inline(always)]
    pub fn cdtfi(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, isc::Cdtfi, Isc_SPEC, crate::common::W> {
        crate::common::RegisterField::<2,0x1,1,0,isc::Cdtfi, Isc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear URDI Flag   CURDI"]
    #[inline(always)]
    pub fn curdi(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, isc::Curdi, Isc_SPEC, crate::common::W> {
        crate::common::RegisterField::<3,0x1,1,0,isc::Curdi, Isc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear DP Flag   CDP"]
    #[inline(always)]
    pub fn cdp(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, isc::Cdp, Isc_SPEC, crate::common::W> {
        crate::common::RegisterField::<4,0x1,1,0,isc::Cdp, Isc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear CP Flag   CCP"]
    #[inline(always)]
    pub fn ccp(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, isc::Ccp, Isc_SPEC, crate::common::W> {
        crate::common::RegisterField::<5,0x1,1,0,isc::Ccp, Isc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Clear DSDIS Flag   CDDIS"]
    #[inline(always)]
    pub fn cddis(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, isc::Cddis, Isc_SPEC, crate::common::W> {
        crate::common::RegisterField::<6,0x1,1,0,isc::Cddis, Isc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set DEDI Flag   SDEDI"]
    #[inline(always)]
    pub fn sdedi(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, isc::Sdedi, Isc_SPEC, crate::common::W> {
        crate::common::RegisterField::<16,0x1,1,0,isc::Sdedi, Isc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set DECI Flag   SDECI"]
    #[inline(always)]
    pub fn sdeci(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, isc::Sdeci, Isc_SPEC, crate::common::W> {
        crate::common::RegisterField::<17,0x1,1,0,isc::Sdeci, Isc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set DTFI Flag   SDTFI"]
    #[inline(always)]
    pub fn sdtfi(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, isc::Sdtfi, Isc_SPEC, crate::common::W> {
        crate::common::RegisterField::<18,0x1,1,0,isc::Sdtfi, Isc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set URDI Flag   SURDI"]
    #[inline(always)]
    pub fn surdi(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, isc::Surdi, Isc_SPEC, crate::common::W> {
        crate::common::RegisterField::<19,0x1,1,0,isc::Surdi, Isc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set DP Bit   SDP"]
    #[inline(always)]
    pub fn sdp(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, isc::Sdp, Isc_SPEC, crate::common::W> {
        crate::common::RegisterField::<20,0x1,1,0,isc::Sdp, Isc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set CP Flag   SCP"]
    #[inline(always)]
    pub fn scp(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, isc::Scp, Isc_SPEC, crate::common::W> {
        crate::common::RegisterField::<21,0x1,1,0,isc::Scp, Isc_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set DSDIS Flag   SDDIS"]
    #[inline(always)]
    pub fn sddis(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, isc::Sddis, Isc_SPEC, crate::common::W> {
        crate::common::RegisterField::<22,0x1,1,0,isc::Sddis, Isc_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Isc {
    #[inline(always)]
    fn default() -> Isc {
        <crate::RegValueT<Isc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod isc {
    pub struct Cdedi_SPEC;
    pub type Cdedi = crate::EnumBitfieldStruct<u8, Cdedi_SPEC>;
    impl Cdedi {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit ISR.DEDI is cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cdeci_SPEC;
    pub type Cdeci = crate::EnumBitfieldStruct<u8, Cdeci_SPEC>;
    impl Cdeci {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit ISR.DECI is cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cdtfi_SPEC;
    pub type Cdtfi = crate::EnumBitfieldStruct<u8, Cdtfi_SPEC>;
    impl Cdtfi {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit ISR.DTFI is cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Curdi_SPEC;
    pub type Curdi = crate::EnumBitfieldStruct<u8, Curdi_SPEC>;
    impl Curdi {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit ISR.URDI is cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cdp_SPEC;
    pub type Cdp = crate::EnumBitfieldStruct<u8, Cdp_SPEC>;
    impl Cdp {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit DSC.DP is cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ccp_SPEC;
    pub type Ccp = crate::EnumBitfieldStruct<u8, Ccp_SPEC>;
    impl Ccp {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit DSC.CP is cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cddis_SPEC;
    pub type Cddis = crate::EnumBitfieldStruct<u8, Cddis_SPEC>;
    impl Cddis {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit DSC.DSDIS is cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sdedi_SPEC;
    pub type Sdedi = crate::EnumBitfieldStruct<u8, Sdedi_SPEC>;
    impl Sdedi {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit ISR.DEDI is set."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sdeci_SPEC;
    pub type Sdeci = crate::EnumBitfieldStruct<u8, Sdeci_SPEC>;
    impl Sdeci {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit ISR.DECI is set."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sdtfi_SPEC;
    pub type Sdtfi = crate::EnumBitfieldStruct<u8, Sdtfi_SPEC>;
    impl Sdtfi {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit ISR.DTFI is set."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Surdi_SPEC;
    pub type Surdi = crate::EnumBitfieldStruct<u8, Surdi_SPEC>;
    impl Surdi {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit ISR.URDI is set."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sdp_SPEC;
    pub type Sdp = crate::EnumBitfieldStruct<u8, Sdp_SPEC>;
    impl Sdp {
        #[doc = "0 No effect"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit DSC.DP is set."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Scp_SPEC;
    pub type Scp = crate::EnumBitfieldStruct<u8, Scp_SPEC>;
    impl Scp {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit DSC.CP is set."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sddis_SPEC;
    pub type Sddis = crate::EnumBitfieldStruct<u8, Sddis_SPEC>;
    impl Sddis {
        #[doc = "0 No operation"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit DSC.DSDIS is set."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr_SPEC;
impl crate::sealed::RegSpec for Isr_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Status Register\n resetvalue={Application Reset:0x0}"]
pub type Isr = crate::RegValueT<Isr_SPEC>;

impl Isr {
    #[doc = "Data Frame Interrupt Flag   DEDI. This flag is always set by hardware when a downstream channel data frame        interrupt is generated. DEDI can be set or cleared by software when        writing to register ISC with the appropriate bits ISC.SDEDI or ISC.CDEDI        set."]
    #[inline(always)]
    pub fn dedi(self) -> crate::common::RegisterFieldBool<0, 1, 0, Isr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Isr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Command Frame Interrupt Flag   DECI. This flag is always set by hardware when a downstream channel command        frame interrupt is generated  whether or not it is enabled. DECI can be        set or cleared by software when writing to register ISC with the        appropriate bits SDECI or CDECI set."]
    #[inline(always)]
    pub fn deci(self) -> crate::common::RegisterFieldBool<1, 1, 0, Isr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Isr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Time Frame Interrupt Flag   DTFI. This flag is always set by hardware when a downstream channel time frame        interrupt is generated  whether or not it is enabled. DTFI can be set or        cleared by software when writing to register ISC with the appropriate        bits SDTFI or CDTFI set."]
    #[inline(always)]
    pub fn dtfi(self) -> crate::common::RegisterFieldBool<2, 1, 0, Isr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Isr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Receive Data Interrupt Flag   URDI. This flag is always set by hardware when an upstream channel receive        data interrupt is generated  whether or not it is enabled. URDI can be        set or cleared by software when writing to register ISC with the        appropriate bits SURDI or CURDI set."]
    #[inline(always)]
    pub fn urdi(self) -> crate::common::RegisterFieldBool<3, 1, 0, Isr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Isr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Isr {
    #[inline(always)]
    fn default() -> Isr {
        <crate::RegValueT<Isr_SPEC> as RegisterValue<_>>::new(0)
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
pub struct Ocr_SPEC;
impl crate::sealed::RegSpec for Ocr_SPEC {
    type DataType = u32;
}
#[doc = "Output Control Register\n resetvalue={Application Reset:0x0}"]
pub type Ocr = crate::RegValueT<Ocr_SPEC>;

impl Ocr {
    #[doc = "FCLP Line Polarity   CLP"]
    #[inline(always)]
    pub fn clp(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ocr::Clp, Ocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,ocr::Clp, Ocr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SOP Line Polarity   SLP"]
    #[inline(always)]
    pub fn slp(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ocr::Slp, Ocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,ocr::Slp, Ocr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Chip Selection Lines Polarity   CSLP. Bit CSLP is buffered during a frame transmission. This means that any        change of CSLP becomes valid first with the start of the next frame        transmission."]
    #[inline(always)]
    pub fn cslp(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ocr::Cslp, Ocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,ocr::Cslp, Ocr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SDI Line Polarity   ILP"]
    #[inline(always)]
    pub fn ilp(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, ocr::Ilp, Ocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,ocr::Ilp, Ocr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Control   CLKCTRL. This bit determines the activation of clock output FCL."]
    #[inline(always)]
    pub fn clkctrl(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, ocr::Clkctrl, Ocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,ocr::Clkctrl, Ocr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Chip Enable Selection for ENL   CSL. This bit field selects the chip enable output ENx that becomes active        during the SRL active phase  ENL  160    160 1  of a data frame. The active level        of ENx is defined by bit CSLP."]
    #[inline(always)]
    pub fn csl(
        self,
    ) -> crate::common::RegisterField<9, 0x3, 1, 0, ocr::Csl, Ocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x3,1,0,ocr::Csl, Ocr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Chip Enable Selection for ENH   CSH. This bit field selects the chip enable output ENx that becomes active        during the SRH active phase  ENH  160    160 1  of a data frame. The active level        of ENx is defined by bit CSLP."]
    #[inline(always)]
    pub fn csh(
        self,
    ) -> crate::common::RegisterField<11, 0x3, 1, 0, ocr::Csh, Ocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x3,1,0,ocr::Csh, Ocr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Chip Enable Selection for ENC   CSC. This bit field selects the chip enable output ENx that becomes active        during the active phase  ENC  160    160 1  of a command frame. The active level        of ENx is defined by bit CSLP."]
    #[inline(always)]
    pub fn csc(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, ocr::Csc, Ocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x3,1,0,ocr::Csc, Ocr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Serial Data Input Selection   SDISEL. This bit field selects the source for the serial data input SDI of the        upstream channel."]
    #[inline(always)]
    pub fn sdisel(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, ocr::Sdisel, Ocr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,ocr::Sdisel, Ocr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ocr {
    #[inline(always)]
    fn default() -> Ocr {
        <crate::RegValueT<Ocr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ocr {
    pub struct Clp_SPEC;
    pub type Clp = crate::EnumBitfieldStruct<u8, Clp_SPEC>;
    impl Clp {
        #[doc = "0 FCLP and FCL signal polarity is identical. FCLN signal has inverted FCL signal polarity."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FCLP signal has inverted FCL signal polarity. FCLN and FCL signal polarities are identical."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Slp_SPEC;
    pub type Slp = crate::EnumBitfieldStruct<u8, Slp_SPEC>;
    impl Slp {
        #[doc = "0 SOP and SO signal polarity is identical. SON signal has inverted SO signal polarity."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SOP signal has inverted SO signal polarity. SON and SO signal polarities are identical."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cslp_SPEC;
    pub type Cslp = crate::EnumBitfieldStruct<u8, Cslp_SPEC>;
    impl Cslp {
        #[doc = "0 EN 3 0  and ENL  ENH  ENC signal polarities are identical  high active ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 EN 3 0  signal polarities are inverted  low active  to the ENL  ENH  ENC signal polarities."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ilp_SPEC;
    pub type Ilp = crate::EnumBitfieldStruct<u8, Ilp_SPEC>;
    impl Ilp {
        #[doc = "0 SDI and SI signal polarities are identical."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SDI and SI signal polarities are inverted."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clkctrl_SPEC;
    pub type Clkctrl = crate::EnumBitfieldStruct<u8, Clkctrl_SPEC>;
    impl Clkctrl {
        #[doc = "0 FCL is activated only during the active phases of data or command frames  not during passive time frames ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 FCL is always active whether or not a downstream frame is currently transmitted."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Csl_SPEC;
    pub type Csl = crate::EnumBitfieldStruct<u8, Csl_SPEC>;
    impl Csl {
        #[doc = "00 EN0 line is selected for ENL."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 EN1 line is selected for ENL."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 EN2 line is selected for ENL."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 EN3 line is selected for ENL."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Csh_SPEC;
    pub type Csh = crate::EnumBitfieldStruct<u8, Csh_SPEC>;
    impl Csh {
        #[doc = "00 EN0 line is selected for ENH."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 EN1 line is selected for ENH."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 EN2 line is selected for ENH."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 EN3 line is selected for ENH."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Csc_SPEC;
    pub type Csc = crate::EnumBitfieldStruct<u8, Csc_SPEC>;
    impl Csc {
        #[doc = "00 EN0 line is selected for ENC."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 EN1 line is selected for ENC."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 EN2 line is selected for ENC."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 EN3 line is selected for ENC."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sdisel_SPEC;
    pub type Sdisel = crate::EnumBitfieldStruct<u8, Sdisel_SPEC>;
    impl Sdisel {
        #[doc = "000 SDI0 input is selected for SDI."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 SDI1 input is selected for SDI."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 SDI2 input is selected for SDI."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 SDI3 input is selected for SDI."]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 SDI4 input is selected for SDI."]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 SDI5 input is selected for SDI."]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 SDI6 input is selected for SDI."]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "111 SDI7 input is selected for SDI."]
        pub const CONST_77: Self = Self::new(7);
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
pub struct UDx_SPEC;
impl crate::sealed::RegSpec for UDx_SPEC {
    type DataType = u32;
}
#[doc = "Upstream Data Register 0\n resetvalue={Application Reset:0x0}"]
pub type UDx = crate::RegValueT<UDx_SPEC>;

impl UDx {
    #[doc = "Received Data   DATA. This bit field contains the 8 bit receive data."]
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, UDx_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, UDx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Valid Bit   V. This bit is set by hardware when the received data is written to UDx. Writing bit C   1 clears V. If hardware setting and software clearing of the valid bit occur simultaneously  bit V will be cleared."]
    #[inline(always)]
    pub fn v(self) -> crate::common::RegisterFieldBool<16, 1, 0, UDx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, UDx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Parity Bit   P. This flag contains the parity bit that has been received with the data frame."]
    #[inline(always)]
    pub fn p(self) -> crate::common::RegisterFieldBool<17, 1, 0, UDx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, UDx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Bit   C. C is always read as 0."]
    #[inline(always)]
    pub fn c(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, udx::C, UDx_SPEC, crate::common::W> {
        crate::common::RegisterField::<18,0x1,1,0,udx::C, UDx_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Lower Address Bit Field   LABF. This bit field contains the two address bits A 1 0  of the 4 bit address field  16 bit data frame . If 12 bit data frame is selected  LABF is always set to 00 B ."]
    #[inline(always)]
    pub fn labf(
        self,
    ) -> crate::common::RegisterField<19, 0x3, 1, 0, u8, UDx_SPEC, crate::common::R> {
        crate::common::RegisterField::<19, 0x3, 1, 0, u8, UDx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Internal Parity Flag   IPF. This bit contains the parity bit that has been calculated in the MSC during data frame reception."]
    #[inline(always)]
    pub fn ipf(self) -> crate::common::RegisterFieldBool<21, 1, 0, UDx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, UDx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Parity Error   PERR. This bit indicates if a start bit error  parity error  or stop bit error occurred during frame reception."]
    #[inline(always)]
    pub fn perr(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, udx::Perr, UDx_SPEC, crate::common::R> {
        crate::common::RegisterField::<22,0x1,1,0,udx::Perr, UDx_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for UDx {
    #[inline(always)]
    fn default() -> UDx {
        <crate::RegValueT<UDx_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod udx {
    pub struct C_SPEC;
    pub type C = crate::EnumBitfieldStruct<u8, C_SPEC>;
    impl C {
        #[doc = "0 No operation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Bit V is cleared."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Perr_SPEC;
    pub type Perr = crate::EnumBitfieldStruct<u8, Perr_SPEC>;
    impl Perr {
        #[doc = "0 No error detected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Error detected."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usce_SPEC;
impl crate::sealed::RegSpec for Usce_SPEC {
    type DataType = u32;
}
#[doc = "Upstream Control Enhanced Register 1\n resetvalue={Application Reset:0x0FF}"]
pub type Usce = crate::RegValueT<Usce_SPEC>;

impl Usce {
    #[doc = "Upstream Time out Prescaler   USTOPRE. Prescaler for the upstream time out limit. Expressed in upstream bit        times. 2 n divider in the range of 1        to 16384. Write to this bit field triggers new start of the watchdog timer."]
    #[inline(always)]
    pub fn ustopre(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Usce_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Usce_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Upstream Time out Value   USTOVAL. Upstream time out value for the N divider in the range of 1 to 16         expressed in number of upstream bit time lengths  the upstream bit time        is the reciprocal value of the upstream baud rate . Time out   BitTime          2  USTOPRE 1     USTOVAL 1 . Write to this bit field triggers new start of the watchdog timer."]
    #[inline(always)]
    pub fn ustoval(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Usce_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Usce_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Upstream Time out Interrupt Enable   USTOEN. Enable bit for the upstream time out interrupt. The Time out counter runs continuously independently of the USTOEN bit. The USTOEN enables only the interrupt generation."]
    #[inline(always)]
    pub fn ustoen(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, usce::Ustoen, Usce_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,usce::Ustoen, Usce_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Upstream Time out Flag   USTF. Signals an upstream timer event. If set by software  the interrupt is also triggered  if enabled. The watchdog timer runs continuously and sets the USTF flag at overflow  independently from the enable bit. Therefore this bit must be cleared with the same write access which enables the interrupt."]
    #[inline(always)]
    pub fn ustf(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, usce::Ustf, Usce_SPEC, crate::common::R> {
        crate::common::RegisterField::<9,0x1,1,0,usce::Ustf, Usce_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Upstream Time out Clear   USTC. Clears the USTF flag. Write only bit  reads as 0."]
    #[inline(always)]
    pub fn ustc(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, usce::Ustc, Usce_SPEC, crate::common::W> {
        crate::common::RegisterField::<10,0x1,1,0,usce::Ustc, Usce_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Upstream Time out Set   USTS. Sets the USTF flag. Write only bit  reads as 0."]
    #[inline(always)]
    pub fn usts(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, usce::Usts, Usce_SPEC, crate::common::W> {
        crate::common::RegisterField::<11,0x1,1,0,usce::Usts, Usce_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Upstream Time out Alternate Service Request   UTASR. Selects if the interrupt signal is routed to the alternate service request node. See CROSSREFERENCE ."]
    #[inline(always)]
    pub fn utasr(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, usce::Utasr, Usce_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,usce::Utasr, Usce_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Upstream Time out Interrupt Node Pointer   USTOIP. USTOIP selects the service request output line SRn  n   0 3  for the time out interrupt."]
    #[inline(always)]
    pub fn ustoip(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, usce::Ustoip, Usce_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x3,1,0,usce::Ustoip, Usce_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Usce {
    #[inline(always)]
    fn default() -> Usce {
        <crate::RegValueT<Usce_SPEC> as RegisterValue<_>>::new(255)
    }
}
pub mod usce {
    pub struct Ustoen_SPEC;
    pub type Ustoen = crate::EnumBitfieldStruct<u8, Ustoen_SPEC>;
    impl Ustoen {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ustf_SPEC;
    pub type Ustf = crate::EnumBitfieldStruct<u8, Ustf_SPEC>;
    impl Ustf {
        #[doc = "0 Disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ustc_SPEC;
    pub type Ustc = crate::EnumBitfieldStruct<u8, Ustc_SPEC>;
    impl Ustc {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Usts_SPEC;
    pub type Usts = crate::EnumBitfieldStruct<u8, Usts_SPEC>;
    impl Usts {
        #[doc = "0 No action"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Utasr_SPEC;
    pub type Utasr = crate::EnumBitfieldStruct<u8, Utasr_SPEC>;
    impl Utasr {
        #[doc = "0 SR Multiplexer selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Alternate Service Request Node  SR4  selected"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ustoip_SPEC;
    pub type Ustoip = crate::EnumBitfieldStruct<u8, Ustoip_SPEC>;
    impl Ustoip {
        #[doc = "00 Service request output SR0 selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Service request output SR1 selected"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Service request output SR2 selected"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Service request output SR3 selected"]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usr_SPEC;
impl crate::sealed::RegSpec for Usr_SPEC {
    type DataType = u32;
}
#[doc = "Upstream Status Register\n resetvalue={Application Reset:0x0}"]
pub type Usr = crate::RegValueT<Usr_SPEC>;

impl Usr {
    #[doc = "Upstream Channel Frame Type   UFT. This bit determines the frame type used by the upstream channel for data        reception."]
    #[inline(always)]
    pub fn uft(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, usr::Uft, Usr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,usr::Uft, Usr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Upstream Channel Receiving Rate   URR. This bit field determines the baud rate for the upstream channel."]
    #[inline(always)]
    pub fn urr(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, usr::Urr, Usr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7,1,0,usr::Urr, Usr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Parity Control   PCTR. This bit determines the parity mode used by the upstream channel for        data reception."]
    #[inline(always)]
    pub fn pctr(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, usr::Pctr, Usr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,usr::Pctr, Usr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Service Request Delay Control   SRDC. This bit determines the additional delay inserted by the upstream        channel when triggering the receive interrupt. Only the hardware        generated interrupt can be delayed. The software generated interrupt by        writing the ISC.SURDI bit is never delayed."]
    #[inline(always)]
    pub fn srdc(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, usr::Srdc, Usr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,usr::Srdc, Usr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Upstream Counter   UC. This bit field indicates the content of the upstream counter that counts the bits during upstream channel reception."]
    #[inline(always)]
    pub fn uc(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Usr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, Usr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Usr {
    #[inline(always)]
    fn default() -> Usr {
        <crate::RegValueT<Usr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod usr {
    pub struct Uft_SPEC;
    pub type Uft = crate::EnumBitfieldStruct<u8, Uft_SPEC>;
    impl Uft {
        #[doc = "0 12 bit upstream frame selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 16 bit upstream frame selected  with 4 bit address field"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Urr_SPEC;
    pub type Urr = crate::EnumBitfieldStruct<u8, Urr_SPEC>;
    impl Urr {
        #[doc = "000 Upstream        channel disabled  no reception is possible"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 Baud rate   f MSC  4"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 Baud rate   f MSC  8"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 Baud rate   f MSC  16"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 Baud rate   f MSC  32"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 Baud rate   f MSC  64"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 Baud rate   f MSC  128"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "111 Baud rate   f MSC  256"]
        pub const CONST_77: Self = Self::new(7);
    }
    pub struct Pctr_SPEC;
    pub type Pctr = crate::EnumBitfieldStruct<u8, Pctr_SPEC>;
    impl Pctr {
        #[doc = "0 Even parity mode is selected. A parity bit is set on an odd number of 1s in the serial address data stream."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Odd parity mode is selected. A parity bit is set on an even number of 1s in the serial address data stream."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Srdc_SPEC;
    pub type Srdc = crate::EnumBitfieldStruct<u8, Srdc_SPEC>;
    impl Srdc {
        #[doc = "0 No delay"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Delay of 1 bit time"]
        pub const CONST_11: Self = Self::new(1);
    }
}
