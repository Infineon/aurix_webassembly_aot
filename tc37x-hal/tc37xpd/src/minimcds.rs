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
#[doc = r"MINIMCDS"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Minimcds(pub(super) *mut u8);
unsafe impl core::marker::Send for Minimcds {}
unsafe impl core::marker::Sync for Minimcds {}
impl Minimcds {
    #[doc = "Clock Control Register\n resetvalue={PowerOn Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "MCDS Control Register\n resetvalue={PowerOn Reset:0x0A0,MCDS Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ct(&self) -> crate::common::Reg<self::Ct_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }

    #[doc = "Trace Buffer Bottom Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn fifobot(&self) -> crate::common::Reg<self::Fifobot_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(516usize)) }
    }

    #[doc = "Trace Buffer Control Register\n resetvalue={PowerOn Reset:0x2002}"]
    #[inline(always)]
    pub const fn fifoctl(&self) -> crate::common::Reg<self::Fifoctl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(528usize)) }
    }

    #[doc = "Trace Buffer Write Pointer\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn fifonow(&self) -> crate::common::Reg<self::Fifonow_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(512usize)) }
    }

    #[doc = "FIFO Overflow Counter Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn fifoovrcnt(&self) -> crate::common::Reg<self::Fifoovrcnt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(540usize)) }
    }

    #[doc = "Trace Buffer PRE POST Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn fifopre(&self) -> crate::common::Reg<self::Fifopre_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(520usize)) }
    }

    #[doc = "Trace Buffer Top Register\n resetvalue={MCDS Reset:0x1FFF}"]
    #[inline(always)]
    pub const fn fifotop(&self) -> crate::common::Reg<self::Fifotop_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(524usize)) }
    }

    #[doc = "Trace Buffer Comparator Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn fifowarnx(
        &self,
    ) -> [crate::common::Reg<self::FifowarNx_SPEC, crate::common::RW>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x214usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x214usize + 0x4usize)),
            ]
        }
    }

    #[doc = "Module Identification Register\n resetvalue={PowerOn Reset:0x0D6C007,MCDS Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }

    #[doc = "MCDS Signal Source Control\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn mux(&self) -> crate::common::Reg<self::Mux_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }

    #[doc = "MCDS TC Signal Source Control\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn mux_tc_rc(&self) -> crate::common::Reg<self::MuxTcRc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = "OCDS Control and Status\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }

    #[doc = "Clock Counter Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn tsuemucnt(&self) -> crate::common::Reg<self::Tsuemucnt_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1032usize)) }
    }

    #[doc = "Clock Prescaler Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn tsuprscl(&self) -> crate::common::Reg<self::Tsuprscl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1028usize)) }
    }

    #[doc = "Clock Counter Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn tsurefcnt(&self) -> crate::common::Reg<self::Tsurefcnt_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(1024usize)) }
    }
    #[doc = "MCX"]
    #[inline(always)]
    pub fn mcx(self) -> self::Mcx {
        unsafe { self::Mcx(self.0.add(2048usize)) }
    }
    #[doc = "TCX"]
    #[inline(always)]
    pub fn tcx(self) -> self::Tcx {
        unsafe { self::Tcx(self.0.add(8192usize)) }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clc_SPEC;
impl crate::sealed::RegSpec for Clc_SPEC {
    type DataType = u32;
}
#[doc = "Clock Control Register\n resetvalue={PowerOn Reset:0x3}"]
pub type Clc = crate::RegValueT<Clc_SPEC>;

impl Clc {
    #[doc = "Disable Request Bit   DISR. This bit is used to request a change of the clocking state. Turn off requests are ignored while the Flush flag is not asserted."]
    #[inline(always)]
    pub fn disr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, clc::Disr, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,clc::Disr, Clc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disable Status Bit   DISS. This bit field indicates the current clocking state."]
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
        <crate::RegValueT<Clc_SPEC> as RegisterValue<_>>::new(3)
    }
}
pub mod clc {
    pub struct Disr_SPEC;
    pub type Disr = crate::EnumBitfieldStruct<u8, Disr_SPEC>;
    impl Disr {
        #[doc = "0 Module incl.        TRAM enable  clock on  is requested."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module incl.        TRAM disable  clock off  is requested."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Diss_SPEC;
    pub type Diss = crate::EnumBitfieldStruct<u8, Diss_SPEC>;
    impl Diss {
        #[doc = "0 Module incl.        TRAM enabled  clock on ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module incl.        TRAM disabled  clock off ."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ct_SPEC;
impl crate::sealed::RegSpec for Ct_SPEC {
    type DataType = u32;
}
#[doc = "MCDS Control Register\n resetvalue={PowerOn Reset:0x0A0,MCDS Reset:0x0,Debug Reset:0x0}"]
pub type Ct = crate::RegValueT<Ct_SPEC>;

impl Ct {
    #[doc = "Key OK Flag   KOK. This bit is always set for MINIMCDS ."]
    #[inline(always)]
    pub fn kok(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ct_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ct_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Key Available Flag   KAV. This bit is always set for MINIMCDS ."]
    #[inline(always)]
    pub fn kav(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ct_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ct_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MCDS Enable Flag   EN. This bit field indicates whether the MCDS is write enabled."]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, ct::En, Ct_SPEC, crate::common::R> {
        crate::common::RegisterField::<13,0x1,1,0,ct::En, Ct_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Clear MCDS Enable Flag   CLRE. This bit allows to reset the MCDS enable Flag by software and thus to write protect the MCDS address range."]
    #[inline(always)]
    pub fn clre(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, ct::Clre, Ct_SPEC, crate::common::W> {
        crate::common::RegisterField::<14,0x1,1,0,ct::Clre, Ct_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set MCDS Enable Flag   SETE. This control bit is the only way to set the MCDS enable flag."]
    #[inline(always)]
    pub fn sete(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, ct::Sete, Ct_SPEC, crate::common::W> {
        crate::common::RegisterField::<15,0x1,1,0,ct::Sete, Ct_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Bus Error Disable Flag   BED. This bit field indicates whether writing to an SFR not implemented in        the MCDS address space is considered an error. In TC3xx the usage of this bit is obsolete for miniMCDS."]
    #[inline(always)]
    pub fn bed(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, ct::Bed, Ct_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1,1,0,ct::Bed, Ct_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bus Error Disable Protection   BED P. This bit determines whether BED is changed by a write to the CT register."]
    #[inline(always)]
    pub fn bed_p(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, ct::BedP, Ct_SPEC, crate::common::W> {
        crate::common::RegisterField::<19,0x1,1,0,ct::BedP, Ct_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Invalid Read Access Flag   IRA. This bit field indicates whether any read access to the MCDS address range failed. This is not a bus error  the MCDS module just returns all zeros in this case."]
    #[inline(always)]
    pub fn ira(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, ct::Ira, Ct_SPEC, crate::common::R> {
        crate::common::RegisterField::<21,0x1,1,0,ct::Ira, Ct_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Clear Invalid Access Bits   CLRI. This control bit is the only way the clear the error bits."]
    #[inline(always)]
    pub fn clri(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, ct::Clri, Ct_SPEC, crate::common::W> {
        crate::common::RegisterField::<22,0x1,1,0,ct::Clri, Ct_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Invalid Write Access Flag   IWA. This bit field indicates whether any write access to the MCDS address range failed. The MCDS module ignores the data of erroneous writes."]
    #[inline(always)]
    pub fn iwa(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, ct::Iwa, Ct_SPEC, crate::common::R> {
        crate::common::RegisterField::<23,0x1,1,0,ct::Iwa, Ct_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "MCDS ResetFlag   RES. This bit reflects the current value of the MCDS Reset request. Granting of the request will automatically clear this bit."]
    #[inline(always)]
    pub fn res(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, ct::Res, Ct_SPEC, crate::common::R> {
        crate::common::RegisterField::<29,0x1,1,0,ct::Res, Ct_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "MCDS ResetRequest Bit   SETR. This bit can be used to cause a MCDS Reset   affecting debug resources like the MCDS registers as specified in the Reset behavior tables."]
    #[inline(always)]
    pub fn setr(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, ct::Setr, Ct_SPEC, crate::common::W> {
        crate::common::RegisterField::<31,0x1,1,0,ct::Setr, Ct_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Ct {
    #[inline(always)]
    fn default() -> Ct {
        <crate::RegValueT<Ct_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ct {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 MCDS disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MCDS enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clre_SPEC;
    pub type Clre = crate::EnumBitfieldStruct<u8, Clre_SPEC>;
    impl Clre {
        #[doc = "0 No effect."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Clear the EN Flag."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sete_SPEC;
    pub type Sete = crate::EnumBitfieldStruct<u8, Sete_SPEC>;
    impl Sete {
        #[doc = "0 No effect."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Set the EN Flag."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Bed_SPEC;
    pub type Bed = crate::EnumBitfieldStruct<u8, Bed_SPEC>;
    impl Bed {
        #[doc = "0 Writing to        unused address causes Bus Error."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 No Bus Error        when writing to MCDS."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct BedP_SPEC;
    pub type BedP = crate::EnumBitfieldStruct<u8, BedP_SPEC>;
    impl BedP {
        #[doc = "0 BED is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 BED is written with the new value."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ira_SPEC;
    pub type Ira = crate::EnumBitfieldStruct<u8, Ira_SPEC>;
    impl Ira {
        #[doc = "0 All reads from the MCDS address range could be executed without problem."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 At least one read access was attempted from an address in the MCDS range where no register is located."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clri_SPEC;
    pub type Clri = crate::EnumBitfieldStruct<u8, Clri_SPEC>;
    impl Clri {
        #[doc = "0 No effect."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 IWA and IRA are reset."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Iwa_SPEC;
    pub type Iwa = crate::EnumBitfieldStruct<u8, Iwa_SPEC>;
    impl Iwa {
        #[doc = "0 All writes to the MCDS address range could be executed without problem."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 At least one write access was attempted to an address in the MCDS range where no register is located."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Res_SPEC;
    pub type Res = crate::EnumBitfieldStruct<u8, Res_SPEC>;
    impl Res {
        #[doc = "0 No MCDS Reset ongoing."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 MCDS Reset currently in progress."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Setr_SPEC;
    pub type Setr = crate::EnumBitfieldStruct<u8, Setr_SPEC>;
    impl Setr {
        #[doc = "0 No effect."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Request MCDS Reset ."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifobot_SPEC;
impl crate::sealed::RegSpec for Fifobot_SPEC {
    type DataType = u32;
}
#[doc = "Trace Buffer Bottom Register\n resetvalue={MCDS Reset:0x0}"]
pub type Fifobot = crate::RegValueT<Fifobot_SPEC>;

impl Fifobot {
    #[doc = "Trace Buffer lower Bound   BOTTOM. See CROSSREFERENCE for a functional description. The relevant width of the field depends on the size of the buffer memory see CROSSREFERENCE"]
    #[inline(always)]
    pub fn bottom(
        self,
    ) -> crate::common::RegisterField<10, 0x7, 1, 0, u8, Fifobot_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x7,1,0,u8, Fifobot_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Fifobot {
    #[inline(always)]
    fn default() -> Fifobot {
        <crate::RegValueT<Fifobot_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifoctl_SPEC;
impl crate::sealed::RegSpec for Fifoctl_SPEC {
    type DataType = u32;
}
#[doc = "Trace Buffer Control Register\n resetvalue={PowerOn Reset:0x2002}"]
pub type Fifoctl = crate::RegValueT<Fifoctl_SPEC>;

impl Fifoctl {
    #[doc = "Trigger Received Flag   TRG. This bit indicated whether the trace recording is in the PRE or POST Trigger area. The switch is controlled by the trace done action of TQU MCX. The TRG flag is cleared by the next CLR bit."]
    #[inline(always)]
    pub fn trg(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, fifoctl::Trg, Fifoctl_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x1,1,0,fifoctl::Trg, Fifoctl_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "FIFO Feeder Empty   FFE. This bit indicates that all primary  secondary and DMC internal FIFOs have been emptied. After Power On Reset and MCDS Reset this bit will be set automatically as the FIFOs are flushed synchronously."]
    #[inline(always)]
    pub fn ffe(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, fifoctl::Ffe, Fifoctl_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,fifoctl::Ffe, Fifoctl_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Trigger DisableFlag   TRDIS. This bit shows the current state of the Trigger Disable control signal. The Trigger Disable signal can be set by the TROFF bit. The Trigger Disable signal can be cleared by the TRON bit. This bit is automatically reset by MCDS Reset ."]
    #[inline(always)]
    pub fn trdis(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, fifoctl::Trdis, Fifoctl_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<9,0x1,1,0,fifoctl::Trdis, Fifoctl_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ClearTrigger DisableFlag   TRON. This write only bit is used by the tool to globally enable all trigger pools of all TQUs."]
    #[inline(always)]
    pub fn tron(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, fifoctl::Tron, Fifoctl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<10,0x1,1,0,fifoctl::Tron, Fifoctl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "SetTrigger DisableFlag   TROFF. This write only bit is the only way to set the Trigger Disable flag. It is used to mask  disable  all trigger pools of all TQUs."]
    #[inline(always)]
    pub fn troff(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, fifoctl::Troff, Fifoctl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<11,0x1,1,0,fifoctl::Troff, Fifoctl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "FlushFlag   FLSH. This bit shows the current state of the Flush control signal. The Flush signal can be set by the SET bit and by the hardware  at end of trace . The Flush signal can only be cleared by the CLR bit. This bit is automatically set by MCDS Reset ."]
    #[inline(always)]
    pub fn flsh(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, fifoctl::Flsh, Fifoctl_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<13,0x1,1,0,fifoctl::Flsh, Fifoctl_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ClearFlushFlag   CLR. This write only bit is the only way to start trace recording."]
    #[inline(always)]
    pub fn clr(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, fifoctl::Clr, Fifoctl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<14,0x1,1,0,fifoctl::Clr, Fifoctl_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "SetFlushFlag   SET. This write only bit is the only way to set the Flush flag by software."]
    #[inline(always)]
    pub fn set(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, fifoctl::Set, Fifoctl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<15,0x1,1,0,fifoctl::Set, Fifoctl_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for Fifoctl {
    #[inline(always)]
    fn default() -> Fifoctl {
        <crate::RegValueT<Fifoctl_SPEC> as RegisterValue<_>>::new(8194)
    }
}
pub mod fifoctl {
    pub struct Trg_SPEC;
    pub type Trg = crate::EnumBitfieldStruct<u8, Trg_SPEC>;
    impl Trg {
        #[doc = "0 No trace done has been seen since the last CLR bit setting  still writing in the Pre Trigger area ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The trace done signal was asserted at least once since CLR was set last time  writing now in the Post Trigger area ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ffe_SPEC;
    pub type Ffe = crate::EnumBitfieldStruct<u8, Ffe_SPEC>;
    impl Ffe {
        #[doc = "0 At least one message has not been transferred to the buffer memory."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 All messages have been written to the buffer memory."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Trdis_SPEC;
    pub type Trdis = crate::EnumBitfieldStruct<u8, Trdis_SPEC>;
    impl Trdis {
        #[doc = "0 Trigger processing active."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 All Triggers masked."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tron_SPEC;
    pub type Tron = crate::EnumBitfieldStruct<u8, Tron_SPEC>;
    impl Tron {
        #[doc = "0 No effect."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The TRDIS flag is reset  effectively starting the trigger processing."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Troff_SPEC;
    pub type Troff = crate::EnumBitfieldStruct<u8, Troff_SPEC>;
    impl Troff {
        #[doc = "0 No effect."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The TRDIS flag is set  effectively forcing all trigger inputs of all TQUs  including the performance data to the Event Counters  to  0 ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Flsh_SPEC;
    pub type Flsh = crate::EnumBitfieldStruct<u8, Flsh_SPEC>;
    impl Flsh {
        #[doc = "0 Tracing active."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Tracing stopped."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Clr_SPEC;
    pub type Clr = crate::EnumBitfieldStruct<u8, Clr_SPEC>;
    impl Clr {
        #[doc = "0 No effect."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The FLSH flag is reset  effectively starting the trace recording from the address given in BOT register."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Set_SPEC;
    pub type Set = crate::EnumBitfieldStruct<u8, Set_SPEC>;
    impl Set {
        #[doc = "0 No effect."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The FLSH flag is set  effectively forcing the trace recording to stop."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifonow_SPEC;
impl crate::sealed::RegSpec for Fifonow_SPEC {
    type DataType = u32;
}
#[doc = "Trace Buffer Write Pointer\n resetvalue={PowerOn Reset:0x0}"]
pub type Fifonow = crate::RegValueT<Fifonow_SPEC>;

impl Fifonow {
    #[doc = "Trace Buffer Current Write Pointer   NOW. This number is the current value of the DMC s internal write pointer. When tracing stops this register points to the buffer memory word containing the  lt end of trace gt  message. The relevant width of the field depends on the size of the buffer memory see CROSSREFERENCE"]
    #[inline(always)]
    pub fn now(
        self,
    ) -> crate::common::RegisterField<5, 0xff, 1, 0, u8, Fifonow_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0xff,1,0,u8, Fifonow_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Fifonow {
    #[inline(always)]
    fn default() -> Fifonow {
        <crate::RegValueT<Fifonow_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifoovrcnt_SPEC;
impl crate::sealed::RegSpec for Fifoovrcnt_SPEC {
    type DataType = u32;
}
#[doc = "FIFO Overflow Counter Register\n resetvalue={PowerOn Reset:0x0}"]
pub type Fifoovrcnt = crate::RegValueT<Fifoovrcnt_SPEC>;

impl Fifoovrcnt {
    #[doc = "FIFO Overflow Counter   COUNT. The counter increments in case an ERR message is written to one of the        Trace Unit FIFOs. In case multiple error messages are written the counter still        increments just by one."]
    #[inline(always)]
    pub fn count(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Fifoovrcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Fifoovrcnt_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Clear counter   CLR"]
    #[inline(always)]
    pub fn clr(
        self,
    ) -> crate::common::RegisterField<
        15,
        0x1,
        1,
        0,
        fifoovrcnt::Clr,
        Fifoovrcnt_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            fifoovrcnt::Clr,
            Fifoovrcnt_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Fifoovrcnt {
    #[inline(always)]
    fn default() -> Fifoovrcnt {
        <crate::RegValueT<Fifoovrcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fifoovrcnt {
    pub struct Clr_SPEC;
    pub type Clr = crate::EnumBitfieldStruct<u8, Clr_SPEC>;
    impl Clr {
        #[doc = "0 Counter is not changed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Counter is cleared"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifopre_SPEC;
impl crate::sealed::RegSpec for Fifopre_SPEC {
    type DataType = u32;
}
#[doc = "Trace Buffer PRE POST Register\n resetvalue={MCDS Reset:0x0}"]
pub type Fifopre = crate::RegValueT<Fifopre_SPEC>;

impl Fifopre {
    #[doc = "Trace Buffer Pre Trigger Area Size   PRE. See CROSSREFERENCE for a functional description. The relevant width of the field depends on the size of the buffer memory see CROSSREFERENCE"]
    #[inline(always)]
    pub fn pre(
        self,
    ) -> crate::common::RegisterField<5, 0xff, 1, 0, u8, Fifopre_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0xff,1,0,u8, Fifopre_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Fifopre {
    #[inline(always)]
    fn default() -> Fifopre {
        <crate::RegValueT<Fifopre_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifotop_SPEC;
impl crate::sealed::RegSpec for Fifotop_SPEC {
    type DataType = u32;
}
#[doc = "Trace Buffer Top Register\n resetvalue={MCDS Reset:0x1FFF}"]
pub type Fifotop = crate::RegValueT<Fifotop_SPEC>;

impl Fifotop {
    #[doc = "Trace Buffer upper Bound   TOP. See CROSSREFERENCE for a functional description. The relevant width of the field depends on the size of the buffer memory see CROSSREFERENCE"]
    #[inline(always)]
    pub fn top(
        self,
    ) -> crate::common::RegisterField<5, 0xff, 1, 0, u8, Fifotop_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0xff,1,0,u8, Fifotop_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Fifotop {
    #[inline(always)]
    fn default() -> Fifotop {
        <crate::RegValueT<Fifotop_SPEC> as RegisterValue<_>>::new(8191)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FifowarNx_SPEC;
impl crate::sealed::RegSpec for FifowarNx_SPEC {
    type DataType = u32;
}
#[doc = "Trace Buffer Comparator Register\n resetvalue={MCDS Reset:0x0}"]
pub type FifowarNx = crate::RegValueT<FifowarNx_SPEC>;

impl FifowarNx {
    #[doc = "Trace Buffer Warn Level   WARN. See CROSSREFERENCE for a functional description. The relevant width of the field depends on the size of the buffer memory see CROSSREFERENCE"]
    #[inline(always)]
    pub fn warn(
        self,
    ) -> crate::common::RegisterField<5, 0xff, 1, 0, u8, FifowarNx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0xff,1,0,u8, FifowarNx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Trigger Generation   EN"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, fifowarnx::En, FifowarNx_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            31,
            0x1,
            1,
            0,
            fifowarnx::En,
            FifowarNx_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for FifowarNx {
    #[inline(always)]
    fn default() -> FifowarNx {
        <crate::RegValueT<FifowarNx_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod fifowarnx {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Trigger generation is disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Trigger generation is enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={PowerOn Reset:0x0D6C007,MCDS Reset:0x0,Debug Reset:0x0}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision   MOD REV. This bit field indicates the revision number of the module implementation."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MOD TYPE"]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number   MOD NUMBER"]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mux_SPEC;
impl crate::sealed::RegSpec for Mux_SPEC {
    type DataType = u32;
}
#[doc = "MCDS Signal Source Control\n resetvalue={PowerOn Reset:0x0}"]
pub type Mux = crate::RegValueT<Mux_SPEC>;

impl Mux {
    #[doc = "Trace Source Select 0   TMUX0. This bit field determines which trace source is seen by POBx. This bit field can only be changed  if TM0 P is written          8216 1  8217   160 simultaneously. Not all sources are available in all products of the family. Please          refer to Trace Source Multiplexer TMUX  Setting Options table in ED."]
    #[inline(always)]
    pub fn tmux0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, mux::Tmux0, Mux_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,mux::Tmux0, Mux_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trace Source Select 3 Protection   TM3 P. This bit determines whether TMUXz is changed by a write to the MUX register. TM1 P  TM2 P  TM3 P are not relevant for miniMCDS."]
    #[inline(always)]
    pub fn tm0_p(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, mux::Tm0P, Mux_SPEC, crate::common::W> {
        crate::common::RegisterField::<7,0x1,1,0,mux::Tm0P, Mux_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Trace Source Select 3 Protection   TM3 P. This bit determines whether TMUXz is changed by a write to the MUX register. TM1 P  TM2 P  TM3 P are not relevant for miniMCDS."]
    #[inline(always)]
    pub fn tm1_p(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, mux::Tm1P, Mux_SPEC, crate::common::W> {
        crate::common::RegisterField::<15,0x1,1,0,mux::Tm1P, Mux_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Trace Source Select 3 Protection   TM3 P. This bit determines whether TMUXz is changed by a write to the MUX register. TM1 P  TM2 P  TM3 P are not relevant for miniMCDS."]
    #[inline(always)]
    pub fn tm2_p(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, mux::Tm2P, Mux_SPEC, crate::common::W> {
        crate::common::RegisterField::<23,0x1,1,0,mux::Tm2P, Mux_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Trace Source Select 3 Protection   TM3 P. This bit determines whether TMUXz is changed by a write to the MUX register. TM1 P  TM2 P  TM3 P are not relevant for miniMCDS."]
    #[inline(always)]
    pub fn tm3_p(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, mux::Tm3P, Mux_SPEC, crate::common::W> {
        crate::common::RegisterField::<31,0x1,1,0,mux::Tm3P, Mux_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Trace Source Select 1   TMUX1. Not relevant for miniMCDS."]
    #[inline(always)]
    pub fn tmux1(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Mux_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8, 0xf, 1, 0, u8, Mux_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Trace Source Select 2   TMUX2. Not relevant for miniMCDS."]
    #[inline(always)]
    pub fn tmux2(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Mux_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Mux_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trace Source Select 3   TMUX3. Not relevant for miniMCDS."]
    #[inline(always)]
    pub fn tmux3(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Mux_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Mux_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Mux {
    #[inline(always)]
    fn default() -> Mux {
        <crate::RegValueT<Mux_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mux {
    pub struct Tmux0_SPEC;
    pub type Tmux0 = crate::EnumBitfieldStruct<u8, Tmux0_SPEC>;
    impl Tmux0 {
        #[doc = "0000 Nothing selected"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "0001 CPU0 or CPU0 MEMSlave"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "0010 CPU1 or CPU1 MEMSlave"]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "0011 CPU2 or CPU2 MEMSlave"]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "0100 CPU3 or CPU3 MEMSlave"]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "0101 CPU4 or CPU4 MEMSlave"]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "0110 CPU5 or CPU5 MEMSlave"]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "1001 LMU0"]
        pub const CONST_99: Self = Self::new(9);
        #[doc = "1010 OLDA"]
        pub const CONST_1010: Self = Self::new(10);
        #[doc = "1101 OTGB"]
        pub const CONST_1313: Self = Self::new(13);
        #[doc = "1110 OTGBM  GTM"]
        pub const CONST_1414: Self = Self::new(14);
    }
    pub struct Tm0P_SPEC;
    pub type Tm0P = crate::EnumBitfieldStruct<u8, Tm0P_SPEC>;
    impl Tm0P {
        #[doc = "0 TMUXz is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 TMUXz is written with the new value."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tm1P_SPEC;
    pub type Tm1P = crate::EnumBitfieldStruct<u8, Tm1P_SPEC>;
    impl Tm1P {
        #[doc = "0 TMUXz is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 TMUXz is written with the new value."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tm2P_SPEC;
    pub type Tm2P = crate::EnumBitfieldStruct<u8, Tm2P_SPEC>;
    impl Tm2P {
        #[doc = "0 TMUXz is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 TMUXz is written with the new value."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tm3P_SPEC;
    pub type Tm3P = crate::EnumBitfieldStruct<u8, Tm3P_SPEC>;
    impl Tm3P {
        #[doc = "0 TMUXz is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 TMUXz is written with the new value."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MuxTcRc_SPEC;
impl crate::sealed::RegSpec for MuxTcRc_SPEC {
    type DataType = u32;
}
#[doc = "MCDS TC Signal Source Control\n resetvalue={PowerOn Reset:0x0}"]
pub type MuxTcRc = crate::RegValueT<MuxTcRc_SPEC>;

impl MuxTcRc {
    #[doc = "TC MUX Trace Source Select   TC MUX5. This bit field determines which trace source is selected. This bit field can only be changed  if TC TM P is written          8216 1  8217   160 simultaneously. Not all CPUs are available in all products of the family."]
    #[inline(always)]
    pub fn tc_mux0(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        mux_tc_rc::TcMux0,
        MuxTcRc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            mux_tc_rc::TcMux0,
            MuxTcRc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "TC MUX Trace Source Select   TC MUX5. This bit field determines which trace source is selected. This bit field can only be changed  if TC TM P is written          8216 1  8217   160 simultaneously. Not all CPUs are available in all products of the family."]
    #[inline(always)]
    pub fn tc_mux1(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        mux_tc_rc::TcMux1,
        MuxTcRc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            mux_tc_rc::TcMux1,
            MuxTcRc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "TC MUX Trace Source Select   TC MUX5. This bit field determines which trace source is selected. This bit field can only be changed  if TC TM P is written          8216 1  8217   160 simultaneously. Not all CPUs are available in all products of the family."]
    #[inline(always)]
    pub fn tc_mux2(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x3,
        1,
        0,
        mux_tc_rc::TcMux2,
        MuxTcRc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x3,
            1,
            0,
            mux_tc_rc::TcMux2,
            MuxTcRc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "TC MUX Trace Source Select   TC MUX5. This bit field determines which trace source is selected. This bit field can only be changed  if TC TM P is written          8216 1  8217   160 simultaneously. Not all CPUs are available in all products of the family."]
    #[inline(always)]
    pub fn tc_mux3(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x3,
        1,
        0,
        mux_tc_rc::TcMux3,
        MuxTcRc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x3,
            1,
            0,
            mux_tc_rc::TcMux3,
            MuxTcRc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "TC MUX Trace Source Select   TC MUX5. This bit field determines which trace source is selected. This bit field can only be changed  if TC TM P is written          8216 1  8217   160 simultaneously. Not all CPUs are available in all products of the family."]
    #[inline(always)]
    pub fn tc_mux4(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        mux_tc_rc::TcMux4,
        MuxTcRc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            mux_tc_rc::TcMux4,
            MuxTcRc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "TC MUX Trace Source Select   TC MUX5. This bit field determines which trace source is selected. This bit field can only be changed  if TC TM P is written          8216 1  8217   160 simultaneously. Not all CPUs are available in all products of the family."]
    #[inline(always)]
    pub fn tc_mux5(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        mux_tc_rc::TcMux5,
        MuxTcRc_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            mux_tc_rc::TcMux5,
            MuxTcRc_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "TC MUX Trace Source Select Protection   TC TM P. This bit determines whether TC MUXz is changed by a write to the MUX TC RC register."]
    #[inline(always)]
    pub fn tc_tm_p(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, mux_tc_rc::TcTmP, MuxTcRc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<
            15,
            0x1,
            1,
            0,
            mux_tc_rc::TcTmP,
            MuxTcRc_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Reference Clock Select   RC. This control bit determines which source is used as reference clock by        the TSU. This bit can only be changed  if RC P is written   8216 1  8217   160 simultaneously. The reference clock source is sampled with the emulation clock.          Therefore no reference clock faster than half the emulation clock must          be used."]
    #[inline(always)]
    pub fn rc(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, mux_tc_rc::Rc, MuxTcRc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,mux_tc_rc::Rc, MuxTcRc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reference Clock Select Protection   RC P. This bit determines whether RC is changed by a write to the MUX TC RC register."]
    #[inline(always)]
    pub fn rc_p(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, mux_tc_rc::RcP, MuxTcRc_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<27,0x1,1,0,mux_tc_rc::RcP, MuxTcRc_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl core::default::Default for MuxTcRc {
    #[inline(always)]
    fn default() -> MuxTcRc {
        <crate::RegValueT<MuxTcRc_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod mux_tc_rc {
    pub struct TcMux0_SPEC;
    pub type TcMux0 = crate::EnumBitfieldStruct<u8, TcMux0_SPEC>;
    impl TcMux0 {
        #[doc = "00 CPUz"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 CPUz"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 CPUz MEMSlave  PSPR  DSPR ..."]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct TcMux1_SPEC;
    pub type TcMux1 = crate::EnumBitfieldStruct<u8, TcMux1_SPEC>;
    impl TcMux1 {
        #[doc = "00 CPUz"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 CPUz"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 CPUz MEMSlave  PSPR  DSPR ..."]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct TcMux2_SPEC;
    pub type TcMux2 = crate::EnumBitfieldStruct<u8, TcMux2_SPEC>;
    impl TcMux2 {
        #[doc = "00 CPUz"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 CPUz"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 CPUz MEMSlave  PSPR  DSPR ..."]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct TcMux3_SPEC;
    pub type TcMux3 = crate::EnumBitfieldStruct<u8, TcMux3_SPEC>;
    impl TcMux3 {
        #[doc = "00 CPUz"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 CPUz"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 CPUz MEMSlave  PSPR  DSPR ..."]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct TcMux4_SPEC;
    pub type TcMux4 = crate::EnumBitfieldStruct<u8, TcMux4_SPEC>;
    impl TcMux4 {
        #[doc = "00 CPUz"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 CPUz"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 CPUz MEMSlave  PSPR  DSPR ..."]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct TcMux5_SPEC;
    pub type TcMux5 = crate::EnumBitfieldStruct<u8, TcMux5_SPEC>;
    impl TcMux5 {
        #[doc = "00 CPUz"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 CPUz"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 CPUz MEMSlave  PSPR  DSPR ..."]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct TcTmP_SPEC;
    pub type TcTmP = crate::EnumBitfieldStruct<u8, TcTmP_SPEC>;
    impl TcTmP {
        #[doc = "0 TC MUXz is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 TC MUXz is written with the new value."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rc_SPEC;
    pub type Rc = crate::EnumBitfieldStruct<u8, Rc_SPEC>;
    impl Rc {
        #[doc = "0 Reference is        fSOURCE0 24  REFCLK1 ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Reference is        fSOURCE1 24  REFCLK2 ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct RcP_SPEC;
    pub type RcP = crate::EnumBitfieldStruct<u8, RcP_SPEC>;
    impl RcP {
        #[doc = "0 RC is not changed."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 RC is written with the new value."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ocs_SPEC;
impl crate::sealed::RegSpec for Ocs_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Control and Status\n resetvalue={MCDS Reset:0x0}"]
pub type Ocs = crate::RegValueT<Ocs_SPEC>;

impl Ocs {
    #[doc = "OCDS Suspend Control   SUS. Controls the sensitivity to the suspend signal coming from the SCU before Power On Reset"]
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
    #[doc = "Suspend State inverted busy o    SUSSTA"]
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
        #[doc = "1 Hard suspend  Not all outstanding messages in             FIFOs are written to TRAM. After a few clock cycles messages are dropped ."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "2 Soft suspend  All outstanding messages in             FIFOs are written to TRAM  until FIFOCTL.FFE is set . No messages are dropped ."]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Sussta_SPEC;
    pub type Sussta = crate::EnumBitfieldStruct<u8, Sussta_SPEC>;
    impl Sussta {
        #[doc = "0 Module is not  yet  suspended  or disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Module is suspended"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsuemucnt_SPEC;
impl crate::sealed::RegSpec for Tsuemucnt_SPEC {
    type DataType = u32;
}
#[doc = "Clock Counter Register\n resetvalue={PowerOn Reset:0x0}"]
pub type Tsuemucnt = crate::RegValueT<Tsuemucnt_SPEC>;

impl Tsuemucnt {
    #[doc = "Current Count Value   COUNT. For a functional description of the counter see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn count(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Tsuemucnt_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Tsuemucnt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Tsuemucnt {
    #[inline(always)]
    fn default() -> Tsuemucnt {
        <crate::RegValueT<Tsuemucnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsuprscl_SPEC;
impl crate::sealed::RegSpec for Tsuprscl_SPEC {
    type DataType = u32;
}
#[doc = "Clock Prescaler Register\n resetvalue={PowerOn Reset:0x0}"]
pub type Tsuprscl = crate::RegValueT<Tsuprscl_SPEC>;

impl Tsuprscl {
    #[doc = "Prescaler Reload Value   RELOAD. For a functional description of the prescaler see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn reload(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Tsuprscl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Tsuprscl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Tsuprscl {
    #[inline(always)]
    fn default() -> Tsuprscl {
        <crate::RegValueT<Tsuprscl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsurefcnt_SPEC;
impl crate::sealed::RegSpec for Tsurefcnt_SPEC {
    type DataType = u32;
}
#[doc = "Clock Counter Register\n resetvalue={PowerOn Reset:0x0}"]
pub type Tsurefcnt = crate::RegValueT<Tsurefcnt_SPEC>;

impl Tsurefcnt {
    #[doc = "Current Count Value   COUNT. For a functional description of the counter see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn count(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Tsurefcnt_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Tsurefcnt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Tsurefcnt {
    #[inline(always)]
    fn default() -> Tsurefcnt {
        <crate::RegValueT<Tsurefcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc = "MCX"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcx(pub(super) *mut u8);
unsafe impl core::marker::Send for Mcx {}
unsafe impl core::marker::Sync for Mcx {}
impl Mcx {
    #[doc = "Action Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn mcxactx(&self) -> [crate::common::Reg<mcx::McxacTx_SPEC, crate::common::RW>; 42] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x5cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x60usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x64usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x68usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x6cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x70usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x74usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x78usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x7cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x80usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x84usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x88usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x8cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x90usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x94usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x98usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0x9cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0xa0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x80usize + 0xa4usize)),
            ]
        }
    }
    #[doc = "Event Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn mcxevtx(&self) -> [crate::common::Reg<mcx::McxevTx_SPEC, crate::common::RW>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x3cusize)),
            ]
        }
    }
    #[doc = "CNT"]
    #[inline(always)]
    pub fn cnt(self) -> [mcx::Cnt; 8] {
        unsafe {
            [
                mcx::Cnt(self.0.add(0x200usize + 0x0usize)),
                mcx::Cnt(self.0.add(0x200usize + 0x10usize)),
                mcx::Cnt(self.0.add(0x200usize + 0x20usize)),
                mcx::Cnt(self.0.add(0x200usize + 0x30usize)),
                mcx::Cnt(self.0.add(0x200usize + 0x40usize)),
                mcx::Cnt(self.0.add(0x200usize + 0x50usize)),
                mcx::Cnt(self.0.add(0x200usize + 0x60usize)),
                mcx::Cnt(self.0.add(0x200usize + 0x70usize)),
            ]
        }
    }
}
pub mod mcx {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct McxacTx_SPEC;
    impl crate::sealed::RegSpec for McxacTx_SPEC {
        type DataType = u32;
    }
    #[doc = "Action Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type McxacTx = crate::RegValueT<McxacTx_SPEC>;

    impl McxacTx {
        #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn ais0(
            self,
        ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, McxacTx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1f,1,0,u8, McxacTx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn ais1(
            self,
        ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, McxacTx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x1f,1,0,u8, McxacTx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn ais2(
            self,
        ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, McxacTx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1f,1,0,u8, McxacTx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn ais3(
            self,
        ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, McxacTx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0x1f,1,0,u8, McxacTx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
        #[inline(always)]
        pub fn aiq0(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x3,
            1,
            0,
            mcxactx::Aiq0,
            McxacTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0x3,
                1,
                0,
                mcxactx::Aiq0,
                McxacTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
        #[inline(always)]
        pub fn aiq1(
            self,
        ) -> crate::common::RegisterField<
            13,
            0x3,
            1,
            0,
            mcxactx::Aiq1,
            McxacTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                13,
                0x3,
                1,
                0,
                mcxactx::Aiq1,
                McxacTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
        #[inline(always)]
        pub fn aiq2(
            self,
        ) -> crate::common::RegisterField<
            21,
            0x3,
            1,
            0,
            mcxactx::Aiq2,
            McxacTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                21,
                0x3,
                1,
                0,
                mcxactx::Aiq2,
                McxacTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
        #[inline(always)]
        pub fn aiq3(
            self,
        ) -> crate::common::RegisterField<
            29,
            0x3,
            1,
            0,
            mcxactx::Aiq3,
            McxacTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                29,
                0x3,
                1,
                0,
                mcxactx::Aiq3,
                McxacTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISq and processed according to AIQq is evaluated. The action is active for one emulation clock cycle if LVq is programmed 0."]
        #[inline(always)]
        pub fn lv0(
            self,
        ) -> crate::common::RegisterField<7, 0x1, 1, 0, mcxactx::Lv0, McxacTx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                mcxactx::Lv0,
                McxacTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISq and processed according to AIQq is evaluated. The action is active for one emulation clock cycle if LVq is programmed 0."]
        #[inline(always)]
        pub fn lv1(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            mcxactx::Lv1,
            McxacTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                mcxactx::Lv1,
                McxacTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISq and processed according to AIQq is evaluated. The action is active for one emulation clock cycle if LVq is programmed 0."]
        #[inline(always)]
        pub fn lv2(
            self,
        ) -> crate::common::RegisterField<
            23,
            0x1,
            1,
            0,
            mcxactx::Lv2,
            McxacTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                23,
                0x1,
                1,
                0,
                mcxactx::Lv2,
                McxacTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISq and processed according to AIQq is evaluated. The action is active for one emulation clock cycle if LVq is programmed 0."]
        #[inline(always)]
        pub fn lv3(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            mcxactx::Lv3,
            McxacTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                mcxactx::Lv3,
                McxacTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for McxacTx {
        #[inline(always)]
        fn default() -> McxacTx {
            <crate::RegValueT<McxacTx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod mcxactx {
        pub struct Aiq0_SPEC;
        pub type Aiq0 = crate::EnumBitfieldStruct<u8, Aiq0_SPEC>;
        impl Aiq0 {
            #[doc = "00 Input is ignored  replaced by 0 ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Input data bit x is inverted."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Input data bit x is connected directly."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Input is replaced by 1."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Aiq1_SPEC;
        pub type Aiq1 = crate::EnumBitfieldStruct<u8, Aiq1_SPEC>;
        impl Aiq1 {
            #[doc = "00 Input is ignored  replaced by 0 ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Input data bit x is inverted."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Input data bit x is connected directly."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Input is replaced by 1."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Aiq2_SPEC;
        pub type Aiq2 = crate::EnumBitfieldStruct<u8, Aiq2_SPEC>;
        impl Aiq2 {
            #[doc = "00 Input is ignored  replaced by 0 ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Input data bit x is inverted."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Input data bit x is connected directly."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Input is replaced by 1."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Aiq3_SPEC;
        pub type Aiq3 = crate::EnumBitfieldStruct<u8, Aiq3_SPEC>;
        impl Aiq3 {
            #[doc = "00 Input is ignored  replaced by 0 ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Input data bit x is inverted."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Input data bit x is connected directly."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Input is replaced by 1."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Lv0_SPEC;
        pub type Lv0 = crate::EnumBitfieldStruct<u8, Lv0_SPEC>;
        impl Lv0 {
            #[doc = "0 Look at inactive to active transitions."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Look for active state."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Lv1_SPEC;
        pub type Lv1 = crate::EnumBitfieldStruct<u8, Lv1_SPEC>;
        impl Lv1 {
            #[doc = "0 Look at inactive to active transitions."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Look for active state."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Lv2_SPEC;
        pub type Lv2 = crate::EnumBitfieldStruct<u8, Lv2_SPEC>;
        impl Lv2 {
            #[doc = "0 Look at inactive to active transitions."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Look for active state."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Lv3_SPEC;
        pub type Lv3 = crate::EnumBitfieldStruct<u8, Lv3_SPEC>;
        impl Lv3 {
            #[doc = "0 Look at inactive to active transitions."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Look for active state."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct McxevTx_SPEC;
    impl crate::sealed::RegSpec for McxevTx_SPEC {
        type DataType = u32;
    }
    #[doc = "Event Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type McxevTx = crate::RegValueT<McxevTx_SPEC>;

    impl McxevTx {
        #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
        #[inline(always)]
        pub fn eiq0(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x3,
            1,
            0,
            mcxevtx::Eiq0,
            McxevTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x3,
                1,
                0,
                mcxevtx::Eiq0,
                McxevTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
        #[inline(always)]
        pub fn eiq1(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x3,
            1,
            0,
            mcxevtx::Eiq1,
            McxevTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                2,
                0x3,
                1,
                0,
                mcxevtx::Eiq1,
                McxevTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
        #[inline(always)]
        pub fn eiq2(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x3,
            1,
            0,
            mcxevtx::Eiq2,
            McxevTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x3,
                1,
                0,
                mcxevtx::Eiq2,
                McxevTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
        #[inline(always)]
        pub fn eiq3(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x3,
            1,
            0,
            mcxevtx::Eiq3,
            McxevTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                6,
                0x3,
                1,
                0,
                mcxevtx::Eiq3,
                McxevTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
        #[inline(always)]
        pub fn eiq4(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x3,
            1,
            0,
            mcxevtx::Eiq4,
            McxevTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x3,
                1,
                0,
                mcxevtx::Eiq4,
                McxevTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
        #[inline(always)]
        pub fn eiq5(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x3,
            1,
            0,
            mcxevtx::Eiq5,
            McxevTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                10,
                0x3,
                1,
                0,
                mcxevtx::Eiq5,
                McxevTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
        #[inline(always)]
        pub fn eiq6(
            self,
        ) -> crate::common::RegisterField<
            12,
            0x3,
            1,
            0,
            mcxevtx::Eiq6,
            McxevTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                12,
                0x3,
                1,
                0,
                mcxevtx::Eiq6,
                McxevTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
        #[inline(always)]
        pub fn eiq7(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x3,
            1,
            0,
            mcxevtx::Eiq7,
            McxevTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                14,
                0x3,
                1,
                0,
                mcxevtx::Eiq7,
                McxevTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
        #[inline(always)]
        pub fn eiq8(
            self,
        ) -> crate::common::RegisterField<
            16,
            0x3,
            1,
            0,
            mcxevtx::Eiq8,
            McxevTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0x3,
                1,
                0,
                mcxevtx::Eiq8,
                McxevTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
        #[inline(always)]
        pub fn eiq9(
            self,
        ) -> crate::common::RegisterField<
            18,
            0x3,
            1,
            0,
            mcxevtx::Eiq9,
            McxevTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                18,
                0x3,
                1,
                0,
                mcxevtx::Eiq9,
                McxevTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
        #[inline(always)]
        pub fn eiq10(
            self,
        ) -> crate::common::RegisterField<
            20,
            0x3,
            1,
            0,
            mcxevtx::Eiq10,
            McxevTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0x3,
                1,
                0,
                mcxevtx::Eiq10,
                McxevTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
        #[inline(always)]
        pub fn eiq11(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x3,
            1,
            0,
            mcxevtx::Eiq11,
            McxevTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                22,
                0x3,
                1,
                0,
                mcxevtx::Eiq11,
                McxevTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
        #[inline(always)]
        pub fn eiq12(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x3,
            1,
            0,
            mcxevtx::Eiq12,
            McxevTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0x3,
                1,
                0,
                mcxevtx::Eiq12,
                McxevTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
        #[inline(always)]
        pub fn eiq13(
            self,
        ) -> crate::common::RegisterField<
            26,
            0x3,
            1,
            0,
            mcxevtx::Eiq13,
            McxevTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                26,
                0x3,
                1,
                0,
                mcxevtx::Eiq13,
                McxevTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
        #[inline(always)]
        pub fn eiq14(
            self,
        ) -> crate::common::RegisterField<
            28,
            0x3,
            1,
            0,
            mcxevtx::Eiq14,
            McxevTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                28,
                0x3,
                1,
                0,
                mcxevtx::Eiq14,
                McxevTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
        #[inline(always)]
        pub fn eiq15(
            self,
        ) -> crate::common::RegisterField<
            30,
            0x3,
            1,
            0,
            mcxevtx::Eiq15,
            McxevTx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                30,
                0x3,
                1,
                0,
                mcxevtx::Eiq15,
                McxevTx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for McxevTx {
        #[inline(always)]
        fn default() -> McxevTx {
            <crate::RegValueT<McxevTx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod mcxevtx {
        pub struct Eiq0_SPEC;
        pub type Eiq0 = crate::EnumBitfieldStruct<u8, Eiq0_SPEC>;
        impl Eiq0 {
            #[doc = "00 Event is disabled  input replaced by 0 ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Trigger x is inverted."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Trigger x is connected directly."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Trigger x is ignored  replaced by 1 ."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Eiq1_SPEC;
        pub type Eiq1 = crate::EnumBitfieldStruct<u8, Eiq1_SPEC>;
        impl Eiq1 {
            #[doc = "00 Event is disabled  input replaced by 0 ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Trigger x is inverted."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Trigger x is connected directly."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Trigger x is ignored  replaced by 1 ."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Eiq2_SPEC;
        pub type Eiq2 = crate::EnumBitfieldStruct<u8, Eiq2_SPEC>;
        impl Eiq2 {
            #[doc = "00 Event is disabled  input replaced by 0 ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Trigger x is inverted."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Trigger x is connected directly."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Trigger x is ignored  replaced by 1 ."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Eiq3_SPEC;
        pub type Eiq3 = crate::EnumBitfieldStruct<u8, Eiq3_SPEC>;
        impl Eiq3 {
            #[doc = "00 Event is disabled  input replaced by 0 ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Trigger x is inverted."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Trigger x is connected directly."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Trigger x is ignored  replaced by 1 ."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Eiq4_SPEC;
        pub type Eiq4 = crate::EnumBitfieldStruct<u8, Eiq4_SPEC>;
        impl Eiq4 {
            #[doc = "00 Event is disabled  input replaced by 0 ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Trigger x is inverted."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Trigger x is connected directly."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Trigger x is ignored  replaced by 1 ."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Eiq5_SPEC;
        pub type Eiq5 = crate::EnumBitfieldStruct<u8, Eiq5_SPEC>;
        impl Eiq5 {
            #[doc = "00 Event is disabled  input replaced by 0 ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Trigger x is inverted."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Trigger x is connected directly."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Trigger x is ignored  replaced by 1 ."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Eiq6_SPEC;
        pub type Eiq6 = crate::EnumBitfieldStruct<u8, Eiq6_SPEC>;
        impl Eiq6 {
            #[doc = "00 Event is disabled  input replaced by 0 ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Trigger x is inverted."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Trigger x is connected directly."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Trigger x is ignored  replaced by 1 ."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Eiq7_SPEC;
        pub type Eiq7 = crate::EnumBitfieldStruct<u8, Eiq7_SPEC>;
        impl Eiq7 {
            #[doc = "00 Event is disabled  input replaced by 0 ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Trigger x is inverted."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Trigger x is connected directly."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Trigger x is ignored  replaced by 1 ."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Eiq8_SPEC;
        pub type Eiq8 = crate::EnumBitfieldStruct<u8, Eiq8_SPEC>;
        impl Eiq8 {
            #[doc = "00 Event is disabled  input replaced by 0 ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Trigger x is inverted."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Trigger x is connected directly."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Trigger x is ignored  replaced by 1 ."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Eiq9_SPEC;
        pub type Eiq9 = crate::EnumBitfieldStruct<u8, Eiq9_SPEC>;
        impl Eiq9 {
            #[doc = "00 Event is disabled  input replaced by 0 ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Trigger x is inverted."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Trigger x is connected directly."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Trigger x is ignored  replaced by 1 ."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Eiq10_SPEC;
        pub type Eiq10 = crate::EnumBitfieldStruct<u8, Eiq10_SPEC>;
        impl Eiq10 {
            #[doc = "00 Event is disabled  input replaced by 0 ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Trigger x is inverted."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Trigger x is connected directly."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Trigger x is ignored  replaced by 1 ."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Eiq11_SPEC;
        pub type Eiq11 = crate::EnumBitfieldStruct<u8, Eiq11_SPEC>;
        impl Eiq11 {
            #[doc = "00 Event is disabled  input replaced by 0 ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Trigger x is inverted."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Trigger x is connected directly."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Trigger x is ignored  replaced by 1 ."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Eiq12_SPEC;
        pub type Eiq12 = crate::EnumBitfieldStruct<u8, Eiq12_SPEC>;
        impl Eiq12 {
            #[doc = "00 Event is disabled  input replaced by 0 ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Trigger x is inverted."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Trigger x is connected directly."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Trigger x is ignored  replaced by 1 ."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Eiq13_SPEC;
        pub type Eiq13 = crate::EnumBitfieldStruct<u8, Eiq13_SPEC>;
        impl Eiq13 {
            #[doc = "00 Event is disabled  input replaced by 0 ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Trigger x is inverted."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Trigger x is connected directly."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Trigger x is ignored  replaced by 1 ."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Eiq14_SPEC;
        pub type Eiq14 = crate::EnumBitfieldStruct<u8, Eiq14_SPEC>;
        impl Eiq14 {
            #[doc = "00 Event is disabled  input replaced by 0 ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Trigger x is inverted."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Trigger x is connected directly."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Trigger x is ignored  replaced by 1 ."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Eiq15_SPEC;
        pub type Eiq15 = crate::EnumBitfieldStruct<u8, Eiq15_SPEC>;
        impl Eiq15 {
            #[doc = "00 Event is disabled  input replaced by 0 ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Trigger x is inverted."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Trigger x is connected directly."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Trigger x is ignored  replaced by 1 ."]
            pub const CONST_33: Self = Self::new(3);
        }
    }
    #[doc = "CNT"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt(pub(super) *mut u8);
    unsafe impl core::marker::Send for Cnt {}
    unsafe impl core::marker::Sync for Cnt {}
    impl Cnt {
        #[doc = "Counter Control Register\n resetvalue={MCDS Reset:0x0}"]
        #[inline(always)]
        pub const fn mcxcclj(&self) -> crate::common::Reg<cnt::McxccLj_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "Current Count Register\n resetvalue={PowerOn Reset:0x0}"]
        #[inline(always)]
        pub const fn mcxcntj(&self) -> crate::common::Reg<cnt::McxcnTj_SPEC, crate::common::R> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "Counter Limit Register\n resetvalue={MCDS Reset:0x0}"]
        #[inline(always)]
        pub const fn mcxlmtj(&self) -> crate::common::Reg<cnt::McxlmTj_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
        }
    }
    pub mod cnt {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct McxccLj_SPEC;
        impl crate::sealed::RegSpec for McxccLj_SPEC {
            type DataType = u32;
        }
        #[doc = "Counter Control Register\n resetvalue={MCDS Reset:0x0}"]
        pub type McxccLj = crate::RegValueT<McxccLj_SPEC>;

        impl McxccLj {
            #[doc = "Count Input Selector   INC1. Index of the event to be processed. Encoding see CROSSREFERENCE . Note that the value 0 in this field does select nothing."]
            #[inline(always)]
            pub fn inc0(
                self,
            ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, McxccLj_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0x7f,1,0,u8, McxccLj_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Count Input Selector   INC1. Index of the event to be processed. Encoding see CROSSREFERENCE . Note that the value 0 in this field does select nothing."]
            #[inline(always)]
            pub fn inc1(
                self,
            ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, McxccLj_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<16,0x7f,1,0,u8, McxccLj_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Count Input Level Mode   ILV1. For a functional description of the counter see CROSSREFERENCE ."]
            #[inline(always)]
            pub fn ilv0(
                self,
            ) -> crate::common::RegisterFieldBool<7, 1, 0, McxccLj_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<7,1,0,McxccLj_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Count Input Level Mode   ILV1. For a functional description of the counter see CROSSREFERENCE ."]
            #[inline(always)]
            pub fn ilv1(
                self,
            ) -> crate::common::RegisterFieldBool<23, 1, 0, McxccLj_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<23,1,0,McxccLj_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Clear Input Selector   CLR1. Index of the event to be processed. Encoding see CROSSREFERENCE . Note that the value 0 in this field does select nothing."]
            #[inline(always)]
            pub fn clr0(
                self,
            ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, McxccLj_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<8,0x3f,1,0,u8, McxccLj_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Clear Input Selector   CLR1. Index of the event to be processed. Encoding see CROSSREFERENCE . Note that the value 0 in this field does select nothing."]
            #[inline(always)]
            pub fn clr1(
                self,
            ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, McxccLj_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<24,0x3f,1,0,u8, McxccLj_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Clear Input Level Mode   CLV1. For a functional description of the counter see CROSSREFERENCE ."]
            #[inline(always)]
            pub fn clv0(
                self,
            ) -> crate::common::RegisterFieldBool<15, 1, 0, McxccLj_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<15,1,0,McxccLj_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Clear Input Level Mode   CLV1. For a functional description of the counter see CROSSREFERENCE ."]
            #[inline(always)]
            pub fn clv1(
                self,
            ) -> crate::common::RegisterFieldBool<31, 1, 0, McxccLj_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<31,1,0,McxccLj_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for McxccLj {
            #[inline(always)]
            fn default() -> McxccLj {
                <crate::RegValueT<McxccLj_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct McxcnTj_SPEC;
        impl crate::sealed::RegSpec for McxcnTj_SPEC {
            type DataType = u32;
        }
        #[doc = "Current Count Register\n resetvalue={PowerOn Reset:0x0}"]
        pub type McxcnTj = crate::RegValueT<McxcnTj_SPEC>;

        impl McxcnTj {
            #[doc = "Current Counter   COUNT. For a functional description of the counter see CROSSREFERENCE ."]
            #[inline(always)]
            pub fn count(
                self,
            ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, McxcnTj_SPEC, crate::common::R>
            {
                crate::common::RegisterField::<0,0xffff,1,0,u16, McxcnTj_SPEC,crate::common::R>::from_register(self,0)
            }
        }
        impl core::default::Default for McxcnTj {
            #[inline(always)]
            fn default() -> McxcnTj {
                <crate::RegValueT<McxcnTj_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct McxlmTj_SPEC;
        impl crate::sealed::RegSpec for McxlmTj_SPEC {
            type DataType = u32;
        }
        #[doc = "Counter Limit Register\n resetvalue={MCDS Reset:0x0}"]
        pub type McxlmTj = crate::RegValueT<McxlmTj_SPEC>;

        impl McxlmTj {
            #[doc = "Counter Limit   LIMIT. For a functional description of the counter see CROSSREFERENCE ."]
            #[inline(always)]
            pub fn limit(
                self,
            ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, McxlmTj_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0xffff,1,0,u16, McxlmTj_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Modulo Count Control   MOD1. For a functional description of the counter see CROSSREFERENCE ."]
            #[inline(always)]
            pub fn mod0(
                self,
            ) -> crate::common::RegisterFieldBool<30, 1, 0, McxlmTj_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<30,1,0,McxlmTj_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Modulo Count Control   MOD1. For a functional description of the counter see CROSSREFERENCE ."]
            #[inline(always)]
            pub fn mod1(
                self,
            ) -> crate::common::RegisterFieldBool<31, 1, 0, McxlmTj_SPEC, crate::common::RW>
            {
                crate::common::RegisterFieldBool::<31,1,0,McxlmTj_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for McxlmTj {
            #[inline(always)]
            fn default() -> McxlmTj {
                <crate::RegValueT<McxlmTj_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
#[doc = "TCX"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcx(pub(super) *mut u8);
unsafe impl core::marker::Send for Tcx {}
unsafe impl core::marker::Sync for Tcx {}
impl Tcx {
    #[doc = "Compact Function Trace Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxcft(&self) -> crate::common::Reg<tcx::Tcxcft_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Current Instruction Pointer\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxcip(&self) -> crate::common::Reg<tcx::Tcxcip_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Debug Status Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxdcsts(&self) -> crate::common::Reg<tcx::Tcxdcsts_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "AC"]
    #[inline(always)]
    pub fn ac(self) -> [tcx::Ac; 2] {
        unsafe {
            [
                tcx::Ac(self.0.add(0x500usize + 0x0usize)),
                tcx::Ac(self.0.add(0x500usize + 0x18usize)),
            ]
        }
    }
    #[doc = "EA"]
    #[inline(always)]
    pub fn ea(self) -> [tcx::Ea; 2] {
        unsafe {
            [
                tcx::Ea(self.0.add(0x400usize + 0x0usize)),
                tcx::Ea(self.0.add(0x400usize + 0x10usize)),
            ]
        }
    }
    #[doc = "IP"]
    #[inline(always)]
    pub fn ip(self) -> [tcx::Ip; 2] {
        unsafe {
            [
                tcx::Ip(self.0.add(0x1000usize + 0x0usize)),
                tcx::Ip(self.0.add(0x1000usize + 0x10usize)),
            ]
        }
    }
    #[doc = "WD"]
    #[inline(always)]
    pub fn wd(self) -> [tcx::Wd; 2] {
        unsafe {
            [
                tcx::Wd(self.0.add(0x480usize + 0x0usize)),
                tcx::Wd(self.0.add(0x480usize + 0x20usize)),
            ]
        }
    }
}
pub mod tcx {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tcxcft_SPEC;
    impl crate::sealed::RegSpec for Tcxcft_SPEC {
        type DataType = u32;
    }
    #[doc = "Compact Function Trace Register\n resetvalue={MCDS Reset:0x0}"]
    pub type Tcxcft = crate::RegValueT<Tcxcft_SPEC>;

    impl Tcxcft {
        #[doc = "Length of very short leaf function   VSHRT FCT. A TriCore 16 bit instruction corresponds to a value of 1  a 32 bit instruction to a value of 2."]
        #[inline(always)]
        pub fn vshrt_fct(
            self,
        ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Tcxcft_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3ff,1,0,u16, Tcxcft_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Length of short leaf function   SHRT FCT. This value has to be greater than VSHRT FCT  if VSHRT FCT  lt  gt  0."]
        #[inline(always)]
        pub fn shrt_fct(
            self,
        ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Tcxcft_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x3ff,1,0,u16, Tcxcft_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Tcxcft {
        #[inline(always)]
        fn default() -> Tcxcft {
            <crate::RegValueT<Tcxcft_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tcxcip_SPEC;
    impl crate::sealed::RegSpec for Tcxcip_SPEC {
        type DataType = u32;
    }
    #[doc = "Current Instruction Pointer\n resetvalue={PowerOn Reset:0x0}"]
    pub type Tcxcip = crate::RegValueT<Tcxcip_SPEC>;

    impl Tcxcip {
        #[doc = "Current Instruction Pointer   CURRENT. To help debugging  the last valid value is kept when the TriCore stops executing. The value 0000 0000 indicates that no valid IP was ever seen."]
        #[inline(always)]
        pub fn current(
            self,
        ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, Tcxcip_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<1,0x7fffffff,1,0,u32, Tcxcip_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for Tcxcip {
        #[inline(always)]
        fn default() -> Tcxcip {
            <crate::RegValueT<Tcxcip_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tcxdcsts_SPEC;
    impl crate::sealed::RegSpec for Tcxdcsts_SPEC {
        type DataType = u32;
    }
    #[doc = "Debug Status Register\n resetvalue={PowerOn Reset:0x0}"]
    pub type Tcxdcsts = crate::RegValueT<Tcxdcsts_SPEC>;

    impl Tcxdcsts {
        #[doc = "Suspended Flag   SUS"]
        #[inline(always)]
        pub fn sus(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            tcxdcsts::Sus,
            Tcxdcsts_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                tcxdcsts::Sus,
                Tcxdcsts_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Run Flag   IDLE"]
        #[inline(always)]
        pub fn idle(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            tcxdcsts::Idle,
            Tcxdcsts_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                tcxdcsts::Idle,
                Tcxdcsts_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Halted Flag   HALT. This is basically the DBGSR.HALT bit."]
        #[inline(always)]
        pub fn halt(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            tcxdcsts::Halt,
            Tcxdcsts_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                tcxdcsts::Halt,
                Tcxdcsts_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Interrupt Service Flag   ISR. This bit indicated whether regular code or a trap interrupt handler is executed."]
        #[inline(always)]
        pub fn isr(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            tcxdcsts::Isr,
            Tcxdcsts_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                tcxdcsts::Isr,
                Tcxdcsts_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Hardware Break Flag   HBRK. This bit reflects the reaction to a break request from a OTGS trigger line  filtered by the EXEVT register."]
        #[inline(always)]
        pub fn hbrk(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            tcxdcsts::Hbrk,
            Tcxdcsts_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                tcxdcsts::Hbrk,
                Tcxdcsts_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Software Break Flag   SBRK. This bit reflects the reaction to a DEBUG instruction  filtered by the SWEVT register."]
        #[inline(always)]
        pub fn sbrk(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            tcxdcsts::Sbrk,
            Tcxdcsts_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                tcxdcsts::Sbrk,
                Tcxdcsts_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Interrupt Enable Flag   IEN"]
        #[inline(always)]
        pub fn ien(
            self,
        ) -> crate::common::RegisterField<
            7,
            0x1,
            1,
            0,
            tcxdcsts::Ien,
            Tcxdcsts_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                tcxdcsts::Ien,
                Tcxdcsts_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Debug Enabled Flag   DBGEN"]
        #[inline(always)]
        pub fn dbgen(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1,
            1,
            0,
            tcxdcsts::Dbgen,
            Tcxdcsts_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                tcxdcsts::Dbgen,
                Tcxdcsts_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Current Clock Divider   CLKDIV. The SCU provides the clock for the TriCore as well as for MCDS. The current ration is reported here."]
        #[inline(always)]
        pub fn clkdiv(
            self,
        ) -> crate::common::RegisterField<
            9,
            0x3,
            1,
            0,
            tcxdcsts::Clkdiv,
            Tcxdcsts_SPEC,
            crate::common::R,
        > {
            crate::common::RegisterField::<
                9,
                0x3,
                1,
                0,
                tcxdcsts::Clkdiv,
                Tcxdcsts_SPEC,
                crate::common::R,
            >::from_register(self, 0)
        }
        #[doc = "Nested Interrupt Level   NESTED ISR. The nested interrupt level is exactly indicated for nesting levels 0          15. For nesting levels greater 15 the value shown is 0xF."]
        #[inline(always)]
        pub fn nested_isr(
            self,
        ) -> crate::common::RegisterField<11, 0xf, 1, 0, u8, Tcxdcsts_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<11,0xf,1,0,u8, Tcxdcsts_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for Tcxdcsts {
        #[inline(always)]
        fn default() -> Tcxdcsts {
            <crate::RegValueT<Tcxdcsts_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod tcxdcsts {
        pub struct Sus_SPEC;
        pub type Sus = crate::EnumBitfieldStruct<u8, Sus_SPEC>;
        impl Sus {
            #[doc = "0 TriCore is not suspended."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 TriCore is suspended."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Idle_SPEC;
        pub type Idle = crate::EnumBitfieldStruct<u8, Idle_SPEC>;
        impl Idle {
            #[doc = "0 TriCore is running."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 TriCore is not running."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Halt_SPEC;
        pub type Halt = crate::EnumBitfieldStruct<u8, Halt_SPEC>;
        impl Halt {
            #[doc = "0 TriCore is not halted."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 TriCore is halted."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Isr_SPEC;
        pub type Isr = crate::EnumBitfieldStruct<u8, Isr_SPEC>;
        impl Isr {
            #[doc = "0 TriCore executes regular code."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 TriCore executes trap interrupt handler code."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Hbrk_SPEC;
        pub type Hbrk = crate::EnumBitfieldStruct<u8, Hbrk_SPEC>;
        impl Hbrk {
            #[doc = "0 TriCore has not received a break request."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 TriCore has received a break request."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Sbrk_SPEC;
        pub type Sbrk = crate::EnumBitfieldStruct<u8, Sbrk_SPEC>;
        impl Sbrk {
            #[doc = "0 TriCore has not hit a DEBUG instruction."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 TriCore has hit a DEBUG instruction."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Ien_SPEC;
        pub type Ien = crate::EnumBitfieldStruct<u8, Ien_SPEC>;
        impl Ien {
            #[doc = "0 TriCore will not react to any interrupt request."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 TriCore will react to interrupt requests."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Dbgen_SPEC;
        pub type Dbgen = crate::EnumBitfieldStruct<u8, Dbgen_SPEC>;
        impl Dbgen {
            #[doc = "0 OCDS logic of TriCore is disabled."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 OCDS logic of TriCore is enabled."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Clkdiv_SPEC;
        pub type Clkdiv = crate::EnumBitfieldStruct<u8, Clkdiv_SPEC>;
        impl Clkdiv {
            #[doc = "00 TriCore and MCDS run at the same clock rate."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 TriCore runs faster than MCDS."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 MCDS runs faster than TriCore."]
            pub const CONST_22: Self = Self::new(2);
        }
    }
    #[doc = "AC"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ac(pub(super) *mut u8);
    unsafe impl core::marker::Send for Ac {}
    unsafe impl core::marker::Sync for Ac {}
    impl Ac {
        #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
        #[inline(always)]
        pub const fn tcxacbndj(&self) -> crate::common::Reg<ac::TcxacbnDj_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x7FFF}"]
        #[inline(always)]
        pub const fn tcxacmskj(&self) -> crate::common::Reg<ac::TcxacmsKj_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
        #[inline(always)]
        pub const fn tcxacrngj(&self) -> crate::common::Reg<ac::TcxacrnGj_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
        }
    }
    pub mod ac {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TcxacbnDj_SPEC;
        impl crate::sealed::RegSpec for TcxacbnDj_SPEC {
            type DataType = u32;
        }
        #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
        pub type TcxacbnDj = crate::RegValueT<TcxacbnDj_SPEC>;

        impl TcxacbnDj {
            #[doc = "Mode Comparator range lower bound   BOUND. The bit string  consisting of SVM  bus master ID  sub channel and direction  masked by TCXACMSKx  is compared to this lower bound. For a functional description of the comparator see CROSSREFERENCE ."]
            #[inline(always)]
            pub fn bound(
                self,
            ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, TcxacbnDj_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<
                    0,
                    0x3fff,
                    1,
                    0,
                    u16,
                    TcxacbnDj_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TcxacbnDj {
            #[inline(always)]
            fn default() -> TcxacbnDj {
                <crate::RegValueT<TcxacbnDj_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TcxacmsKj_SPEC;
        impl crate::sealed::RegSpec for TcxacmsKj_SPEC {
            type DataType = u32;
        }
        #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x7FFF}"]
        pub type TcxacmsKj = crate::RegValueT<TcxacmsKj_SPEC>;

        impl TcxacmsKj {
            #[doc = "Comparator bit mask for Supervisor Mode   SVM. For a functional description of the comparator see CROSSREFERENCE ."]
            #[inline(always)]
            pub fn svm(
                self,
            ) -> crate::common::RegisterField<
                0,
                0x1,
                1,
                0,
                tcxacmskj::Svm,
                TcxacmsKj_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    0,
                    0x1,
                    1,
                    0,
                    tcxacmskj::Svm,
                    TcxacmsKj_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Comparator bit mask for Master ID   MASTER. The master ID from the bus  encoding see CROSSREFERENCE   is ANDed with this bit pattern and the result used as bit  4 1  for the comparison. For a functional description of the comparator see CROSSREFERENCE ."]
            #[inline(always)]
            pub fn master(
                self,
            ) -> crate::common::RegisterField<1, 0xf, 1, 0, u8, TcxacmsKj_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<1,0xf,1,0,u8, TcxacmsKj_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Comparator bit mask for Sub channel ID   SUBCHANNEL. The sub channel ID from the bus  encoding see CROSSREFERENCE          is ANDed with this bit pattern and the result used as bit  11 5  for the        comparison. For a functional description of the comparator see CROSSREFERENCE ."]
            #[inline(always)]
            pub fn subchannel(
                self,
            ) -> crate::common::RegisterField<5, 0x7f, 1, 0, u8, TcxacmsKj_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<5,0x7f,1,0,u8, TcxacmsKj_SPEC,crate::common::RW>::from_register(self,0)
            }
            #[doc = "Comparator bit mask for Direction Write   WR. For a functional description of the comparator see CROSSREFERENCE ."]
            #[inline(always)]
            pub fn wr(
                self,
            ) -> crate::common::RegisterField<
                12,
                0x1,
                1,
                0,
                tcxacmskj::Wr,
                TcxacmsKj_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    12,
                    0x1,
                    1,
                    0,
                    tcxacmskj::Wr,
                    TcxacmsKj_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Comparator bit mask for Direction Read   RD. For a functional description of the comparator see CROSSREFERENCE ."]
            #[inline(always)]
            pub fn rd(
                self,
            ) -> crate::common::RegisterField<
                13,
                0x1,
                1,
                0,
                tcxacmskj::Rd,
                TcxacmsKj_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    13,
                    0x1,
                    1,
                    0,
                    tcxacmskj::Rd,
                    TcxacmsKj_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
            #[doc = "Bit mask for Transaction Type   MSK. For a functional description of the comparator see CROSSREFERENCE . The MSK bits of a register group are ORed. Setting the bit in one of the registers  x 0 3  enables the bit mask functionality."]
            #[inline(always)]
            pub fn msk(
                self,
            ) -> crate::common::RegisterField<
                14,
                0x1,
                1,
                0,
                tcxacmskj::Msk,
                TcxacmsKj_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    14,
                    0x1,
                    1,
                    0,
                    tcxacmskj::Msk,
                    TcxacmsKj_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TcxacmsKj {
            #[inline(always)]
            fn default() -> TcxacmsKj {
                <crate::RegValueT<TcxacmsKj_SPEC> as RegisterValue<_>>::new(32767)
            }
        }
        pub mod tcxacmskj {
            pub struct Svm_SPEC;
            pub type Svm = crate::EnumBitfieldStruct<u8, Svm_SPEC>;
            impl Svm {
                #[doc = "0 Overwrite SVM with 0 for the comparison."]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Use SVM as bit 0 for the comparison."]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Wr_SPEC;
            pub type Wr = crate::EnumBitfieldStruct<u8, Wr_SPEC>;
            impl Wr {
                #[doc = "0 Ignore the write indication of the transaction  bit 12 of the compared pattern is set to 0."]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Bit 12 of the compared pattern is set to 1 if the current transaction is a write. Otherwise bit 12 is set to 0."]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Rd_SPEC;
            pub type Rd = crate::EnumBitfieldStruct<u8, Rd_SPEC>;
            impl Rd {
                #[doc = "0 Ignore the read indication of the transaction  bit 13 of the compared pattern is set to 0."]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 Bit 13 of the compared pattern is set to 1 if the current transaction is a read. Otherwise bit 13 is set to 0."]
                pub const CONST_11: Self = Self::new(1);
            }
            pub struct Msk_SPEC;
            pub type Msk = crate::EnumBitfieldStruct<u8, Msk_SPEC>;
            impl Msk {
                #[doc = "0 subchannel  master and SVM are included in DTU messages."]
                pub const CONST_00: Self = Self::new(0);
                #[doc = "1 subchannel  master and SVM are set to zero and as such not included in DTU messages."]
                pub const CONST_11: Self = Self::new(1);
            }
        }
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TcxacrnGj_SPEC;
        impl crate::sealed::RegSpec for TcxacrnGj_SPEC {
            type DataType = u32;
        }
        #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
        pub type TcxacrnGj = crate::RegValueT<TcxacrnGj_SPEC>;

        impl TcxacrnGj {
            #[doc = "Mode Comparator range size   RANGE. For a functional description of the comparator see CROSSREFERENCE ."]
            #[inline(always)]
            pub fn range(
                self,
            ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, TcxacrnGj_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<
                    0,
                    0x3fff,
                    1,
                    0,
                    u16,
                    TcxacrnGj_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TcxacrnGj {
            #[inline(always)]
            fn default() -> TcxacrnGj {
                <crate::RegValueT<TcxacrnGj_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
    #[doc = "EA"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ea(pub(super) *mut u8);
    unsafe impl core::marker::Send for Ea {}
    unsafe impl core::marker::Sync for Ea {}
    impl Ea {
        #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
        #[inline(always)]
        pub const fn tcxeabndj(&self) -> crate::common::Reg<ea::TcxeabnDj_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
        #[inline(always)]
        pub const fn tcxearngj(&self) -> crate::common::Reg<ea::TcxearnGj_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
        }
    }
    pub mod ea {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TcxeabnDj_SPEC;
        impl crate::sealed::RegSpec for TcxeabnDj_SPEC {
            type DataType = u32;
        }
        #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
        pub type TcxeabnDj = crate::RegValueT<TcxeabnDj_SPEC>;

        impl TcxeabnDj {
            #[doc = "Address Comparator range lower bound   BOUND. For a functional description of the comparator see CROSSREFERENCE . When defining the comparators for the effective address keep in mind that many memory locations can be accessed under two different addresses  namely as cached or non cached memory range."]
            #[inline(always)]
            pub fn bound(
                self,
            ) -> crate::common::RegisterField<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcxeabnDj_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    0,
                    0xffffffff,
                    1,
                    0,
                    u32,
                    TcxeabnDj_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TcxeabnDj {
            #[inline(always)]
            fn default() -> TcxeabnDj {
                <crate::RegValueT<TcxeabnDj_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TcxearnGj_SPEC;
        impl crate::sealed::RegSpec for TcxearnGj_SPEC {
            type DataType = u32;
        }
        #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
        pub type TcxearnGj = crate::RegValueT<TcxearnGj_SPEC>;

        impl TcxearnGj {
            #[doc = "Address Comparator range size   RANGE. For a functional description of the comparator see CROSSREFERENCE ."]
            #[inline(always)]
            pub fn range(
                self,
            ) -> crate::common::RegisterField<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcxearnGj_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    0,
                    0xffffffff,
                    1,
                    0,
                    u32,
                    TcxearnGj_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TcxearnGj {
            #[inline(always)]
            fn default() -> TcxearnGj {
                <crate::RegValueT<TcxearnGj_SPEC> as RegisterValue<_>>::new(4294967295)
            }
        }
    }
    #[doc = "IP"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ip(pub(super) *mut u8);
    unsafe impl core::marker::Send for Ip {}
    unsafe impl core::marker::Sync for Ip {}
    impl Ip {
        #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
        #[inline(always)]
        pub const fn tcxipbndj(&self) -> crate::common::Reg<ip::TcxipbnDj_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFE}"]
        #[inline(always)]
        pub const fn tcxiprngj(&self) -> crate::common::Reg<ip::TcxiprnGj_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
        }
    }
    pub mod ip {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TcxipbnDj_SPEC;
        impl crate::sealed::RegSpec for TcxipbnDj_SPEC {
            type DataType = u32;
        }
        #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
        pub type TcxipbnDj = crate::RegValueT<TcxipbnDj_SPEC>;

        impl TcxipbnDj {
            #[doc = "IP Comparator range lower bound   BOUND. As TriCore instructions are sized in multiples of 16 bit the LSB is fixed to 0. For a functional description of the comparator see CROSSREFERENCE . When defining the comparators for the TriCore instruction pointer keep in mind that many memory locations can be accessed under two different addresses  namely as cached or non cached memory range."]
            #[inline(always)]
            pub fn bound(
                self,
            ) -> crate::common::RegisterField<
                1,
                0x7fffffff,
                1,
                0,
                u32,
                TcxipbnDj_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    1,
                    0x7fffffff,
                    1,
                    0,
                    u32,
                    TcxipbnDj_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TcxipbnDj {
            #[inline(always)]
            fn default() -> TcxipbnDj {
                <crate::RegValueT<TcxipbnDj_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TcxiprnGj_SPEC;
        impl crate::sealed::RegSpec for TcxiprnGj_SPEC {
            type DataType = u32;
        }
        #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFE}"]
        pub type TcxiprnGj = crate::RegValueT<TcxiprnGj_SPEC>;

        impl TcxiprnGj {
            #[doc = "IP Comparator range size   RANGE. As TriCore instructions are sized in multiples of 16 bit the LSB is fixed to 0. For a functional description of the comparator see CROSSREFERENCE ."]
            #[inline(always)]
            pub fn range(
                self,
            ) -> crate::common::RegisterField<
                1,
                0x7fffffff,
                1,
                0,
                u32,
                TcxiprnGj_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    1,
                    0x7fffffff,
                    1,
                    0,
                    u32,
                    TcxiprnGj_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TcxiprnGj {
            #[inline(always)]
            fn default() -> TcxiprnGj {
                <crate::RegValueT<TcxiprnGj_SPEC> as RegisterValue<_>>::new(4294967294)
            }
        }
    }
    #[doc = "WD"]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wd(pub(super) *mut u8);
    unsafe impl core::marker::Send for Wd {}
    unsafe impl core::marker::Sync for Wd {}
    impl Wd {
        #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
        #[inline(always)]
        pub const fn tcxwdbndj(&self) -> crate::common::Reg<wd::TcxwdbnDj_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
        }
        #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
        #[inline(always)]
        pub const fn tcxwdmskj(&self) -> crate::common::Reg<wd::TcxwdmsKj_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
        }
        #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
        #[inline(always)]
        pub const fn tcxwdrngj(&self) -> crate::common::Reg<wd::TcxwdrnGj_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
        }
        #[doc = "Comparator Sign Register 0\n resetvalue={MCDS Reset:0x0}"]
        #[inline(always)]
        pub const fn tcxwdsgnj(&self) -> crate::common::Reg<wd::TcxwdsgNj_SPEC, crate::common::RW> {
            unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
        }
    }
    pub mod wd {
        #[allow(unused_imports)]
        use crate::common::*;
        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TcxwdbnDj_SPEC;
        impl crate::sealed::RegSpec for TcxwdbnDj_SPEC {
            type DataType = u32;
        }
        #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
        pub type TcxwdbnDj = crate::RegValueT<TcxwdbnDj_SPEC>;

        impl TcxwdbnDj {
            #[doc = "Data Comparator range lower bound   BOUND. For a functional description of the comparator see CROSSREFERENCE ."]
            #[inline(always)]
            pub fn bound(
                self,
            ) -> crate::common::RegisterField<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcxwdbnDj_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    0,
                    0xffffffff,
                    1,
                    0,
                    u32,
                    TcxwdbnDj_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TcxwdbnDj {
            #[inline(always)]
            fn default() -> TcxwdbnDj {
                <crate::RegValueT<TcxwdbnDj_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TcxwdmsKj_SPEC;
        impl crate::sealed::RegSpec for TcxwdmsKj_SPEC {
            type DataType = u32;
        }
        #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
        pub type TcxwdmsKj = crate::RegValueT<TcxwdmsKj_SPEC>;

        impl TcxwdmsKj {
            #[doc = "Data Comparator bit mask   MASK. For a functional description of the comparator see CROSSREFERENCE ."]
            #[inline(always)]
            pub fn mask(
                self,
            ) -> crate::common::RegisterField<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcxwdmsKj_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    0,
                    0xffffffff,
                    1,
                    0,
                    u32,
                    TcxwdmsKj_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TcxwdmsKj {
            #[inline(always)]
            fn default() -> TcxwdmsKj {
                <crate::RegValueT<TcxwdmsKj_SPEC> as RegisterValue<_>>::new(4294967295)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TcxwdrnGj_SPEC;
        impl crate::sealed::RegSpec for TcxwdrnGj_SPEC {
            type DataType = u32;
        }
        #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
        pub type TcxwdrnGj = crate::RegValueT<TcxwdrnGj_SPEC>;

        impl TcxwdrnGj {
            #[doc = "Data Comparator range size   RANGE. For a functional description of the comparator see CROSSREFERENCE ."]
            #[inline(always)]
            pub fn range(
                self,
            ) -> crate::common::RegisterField<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcxwdrnGj_SPEC,
                crate::common::RW,
            > {
                crate::common::RegisterField::<
                    0,
                    0xffffffff,
                    1,
                    0,
                    u32,
                    TcxwdrnGj_SPEC,
                    crate::common::RW,
                >::from_register(self, 0)
            }
        }
        impl core::default::Default for TcxwdrnGj {
            #[inline(always)]
            fn default() -> TcxwdrnGj {
                <crate::RegValueT<TcxwdrnGj_SPEC> as RegisterValue<_>>::new(0)
            }
        }

        #[doc(hidden)]
        #[derive(Copy, Clone, Eq, PartialEq)]
        pub struct TcxwdsgNj_SPEC;
        impl crate::sealed::RegSpec for TcxwdsgNj_SPEC {
            type DataType = u32;
        }
        #[doc = "Comparator Sign Register 0\n resetvalue={MCDS Reset:0x0}"]
        pub type TcxwdsgNj = crate::RegValueT<TcxwdsgNj_SPEC>;

        impl TcxwdsgNj {
            #[doc = "Bit number  1 31  of sign bit   SIGN. For a functional description of the comparator see CROSSREFERENCE ."]
            #[inline(always)]
            pub fn sign(
                self,
            ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, TcxwdsgNj_SPEC, crate::common::RW>
            {
                crate::common::RegisterField::<0,0x1f,1,0,u8, TcxwdsgNj_SPEC,crate::common::RW>::from_register(self,0)
            }
        }
        impl core::default::Default for TcxwdsgNj {
            #[inline(always)]
            fn default() -> TcxwdsgNj {
                <crate::RegValueT<TcxwdsgNj_SPEC> as RegisterValue<_>>::new(0)
            }
        }
    }
}
