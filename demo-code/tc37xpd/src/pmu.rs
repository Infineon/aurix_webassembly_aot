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
#[doc = r"PMU"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmu(pub(super) *mut u8);
unsafe impl core::marker::Send for Pmu {}
unsafe impl core::marker::Sync for Pmu {}
impl Pmu {
    #[doc = "Boot ROM Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn bromcon(&self) -> crate::common::Reg<self::Bromcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1320usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0E6C000}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1288usize)) }
    }

    #[doc = "Tuning Protection control\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tpcon(&self) -> crate::common::Reg<self::Tpcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1296usize)) }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bromcon_SPEC;
impl crate::sealed::RegSpec for Bromcon_SPEC {
    type DataType = u32;
}
#[doc = "Boot ROM Control Register\n resetvalue={Application Reset:0x0}"]
pub type Bromcon = crate::RegValueT<Bromcon_SPEC>;

impl Bromcon {
    #[doc = "Double Bit Error   DBER. This bit is set when any code fetch or data read has a double bit error. It can also be set by writing  1  to it. This bit is cleared by writing  0  to it."]
    #[inline(always)]
    pub fn dber(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Bromcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Bromcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable ECC Decoding   ECDECDIS"]
    #[inline(always)]
    pub fn ecdecdis(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        bromcon::Ecdecdis,
        Bromcon_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            bromcon::Ecdecdis,
            Bromcon_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Map ECC Plane   ECCMAP"]
    #[inline(always)]
    pub fn eccmap(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, bromcon::Eccmap, Bromcon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,bromcon::Eccmap, Bromcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Saturating Single Bit Error Count   SBERCOUNT. This field is incremented with each code fetch or data read that detected a single bit error. The counter saturates at F . It can be cleared by writing 0 to it."]
    #[inline(always)]
    pub fn sbercount(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Bromcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Bromcon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Bromcon {
    #[inline(always)]
    fn default() -> Bromcon {
        <crate::RegValueT<Bromcon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod bromcon {
    pub struct Ecdecdis_SPEC;
    pub type Ecdecdis = crate::EnumBitfieldStruct<u8, Ecdecdis_SPEC>;
    impl Ecdecdis {
        #[doc = "0 Single bit error correction is enabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Single bit error correction is disabled. Depending on ECCMAP the raw data or the raw ECC codes can be read."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Eccmap_SPEC;
    pub type Eccmap = crate::EnumBitfieldStruct<u8, Eccmap_SPEC>;
    impl Eccmap {
        #[doc = "0 Read accesses deliver data. With ECDECDIS 1 the raw data is read  else corrected data."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read accesses deliver the raw ECC codes."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0E6C000}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MOD REV. MOD REV defines the module revision number. See DMU Design Specification for MOD REV value."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MOD TYPE. This bit field is C0 H . It defines the        module as a 32 bit module."]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MOD NUMBER. This bit field defines the module identification number."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(15122432)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tpcon_SPEC;
impl crate::sealed::RegSpec for Tpcon_SPEC {
    type DataType = u32;
}
#[doc = "Tuning Protection control\n resetvalue={Application Reset:0x0}"]
pub type Tpcon = crate::RegValueT<Tpcon_SPEC>;

impl Tpcon {
    #[doc = "See Tuning Protection specification"]
    #[inline(always)]
    pub fn sfrf(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Tpcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8, Tpcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "See Tuning Protection specification"]
    #[inline(always)]
    pub fn frf(self) -> crate::common::RegisterFieldBool<7, 1, 0, Tpcon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Tpcon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "See Tuning Protection specification"]
    #[inline(always)]
    pub fn stptrf(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, Tpcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7f,1,0,u8, Tpcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "See Tuning Protection specification"]
    #[inline(always)]
    pub fn sorc(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Tpcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Tpcon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Tpcon {
    #[inline(always)]
    fn default() -> Tpcon {
        <crate::RegValueT<Tpcon_SPEC> as RegisterValue<_>>::new(0)
    }
}
