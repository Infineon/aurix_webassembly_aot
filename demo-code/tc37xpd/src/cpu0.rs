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
#[doc = r"CPU"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu0(pub(super) *mut u8);
unsafe impl core::marker::Send for Cpu0 {}
unsafe impl core::marker::Sync for Cpu0 {}
impl Cpu0 {
    #[doc = "CPUx Address General Purpose Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ay(&self) -> [crate::common::Reg<self::Ay_SPEC, crate::common::RW>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x1ff80usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff80usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff80usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff80usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff80usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff80usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff80usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff80usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff80usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff80usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff80usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff80usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff80usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff80usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff80usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff80usize + 0x3cusize)),
            ]
        }
    }

    #[doc = "CPUx Base Interrupt Vector Table Pointer\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn biv(&self) -> crate::common::Reg<self::Biv_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130592usize)) }
    }

    #[doc = "CPUx Base Trap Vector Table Pointer\n resetvalue={Application Reset:0x0A0000100}"]
    #[inline(always)]
    pub const fn btv(&self) -> crate::common::Reg<self::Btv_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130596usize)) }
    }

    #[doc = "CPUx CPU Clock Cycle Count\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ccnt(&self) -> crate::common::Reg<self::Ccnt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130052usize)) }
    }

    #[doc = "CPUx Counter Control\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn cctrl(&self) -> crate::common::Reg<self::Cctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130048usize)) }
    }

    #[doc = "CPUx Compatibility Control Register\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn compat(&self) -> crate::common::Reg<self::Compat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(103424usize)) }
    }

    #[doc = "CPUx Core Identification Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn core_id(&self) -> crate::common::Reg<self::CoreId_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130588usize)) }
    }

    #[doc = "CPUx Identification Register TC1.6.2P\n resetvalue={Application Reset:0x0C0C021}"]
    #[inline(always)]
    pub const fn cpu_id(&self) -> crate::common::Reg<self::CpuId_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130584usize)) }
    }

    #[doc = "CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cpxe_0(&self) -> crate::common::Reg<self::Cpxe0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(122880usize)) }
    }

    #[doc = "CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cpxe_1(&self) -> crate::common::Reg<self::Cpxe1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(122884usize)) }
    }

    #[doc = "CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cpxe_2(&self) -> crate::common::Reg<self::Cpxe2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(122888usize)) }
    }

    #[doc = "CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cpxe_3(&self) -> crate::common::Reg<self::Cpxe3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(122892usize)) }
    }

    #[doc = "CPUx Code Protection Execute Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cpxe_4(&self) -> crate::common::Reg<self::Cpxe4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(122944usize)) }
    }

    #[doc = "CPUx Code Protection Execute Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cpxe_5(&self) -> crate::common::Reg<self::Cpxe5_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(122948usize)) }
    }

    #[doc = "CPUx Core Register Access Event\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn crevt(&self) -> crate::common::Reg<self::Crevt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130316usize)) }
    }

    #[doc = "CPUx Customer ID register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cus_id(&self) -> crate::common::Reg<self::CusId_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130640usize)) }
    }

    #[doc = "CPUx Data Asynchronous Trap Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn datr(&self) -> crate::common::Reg<self::Datr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(102424usize)) }
    }

    #[doc = "CPUx Debug Status Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn dbgsr(&self) -> crate::common::Reg<self::Dbgsr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130304usize)) }
    }

    #[doc = "CPUx Debug Trap Control Register\n resetvalue={Application Reset:0x1}"]
    #[inline(always)]
    pub const fn dbgtcr(&self) -> crate::common::Reg<self::Dbgtcr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130376usize)) }
    }

    #[doc = "CPUx Data Memory Control Register\n resetvalue={Application Reset:0x2}"]
    #[inline(always)]
    pub const fn dcon0(&self) -> crate::common::Reg<self::Dcon0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(102464usize)) }
    }

    #[doc = "CPUx Data Control Register 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dcon2(&self) -> crate::common::Reg<self::Dcon2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(102400usize)) }
    }

    #[doc = "CPUx Debug Context Save Area Pointer\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dcx(&self) -> crate::common::Reg<self::Dcx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130372usize)) }
    }

    #[doc = "CPUx Data Error Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn deadd(&self) -> crate::common::Reg<self::Deadd_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(102428usize)) }
    }

    #[doc = "CPUx Data Integrity Error Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn diear(&self) -> crate::common::Reg<self::Diear_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(102432usize)) }
    }

    #[doc = "CPUx Data Integrity Error Trap Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dietr(&self) -> crate::common::Reg<self::Dietr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(102436usize)) }
    }

    #[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccena0_r(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccena0R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57992usize)) }
    }

    #[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccena0_w(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccena0W_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57864usize)) }
    }

    #[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccena1_r(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccena1R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(58008usize)) }
    }

    #[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccena1_w(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccena1W_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57880usize)) }
    }

    #[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccena2_r(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccena2R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(58024usize)) }
    }

    #[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccena2_w(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccena2W_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57896usize)) }
    }

    #[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccena3_r(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccena3R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(58040usize)) }
    }

    #[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccena3_w(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccena3W_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57912usize)) }
    }

    #[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccena4_r(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccena4R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(58056usize)) }
    }

    #[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccena4_w(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccena4W_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57928usize)) }
    }

    #[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccena5_r(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccena5R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(58072usize)) }
    }

    #[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccena5_w(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccena5W_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57944usize)) }
    }

    #[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccena6_r(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccena6R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(58088usize)) }
    }

    #[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccena6_w(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccena6W_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57960usize)) }
    }

    #[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccena7_r(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccena7R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(58104usize)) }
    }

    #[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccena7_w(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccena7W_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57976usize)) }
    }

    #[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccenb0_r(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccenb0R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57996usize)) }
    }

    #[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccenb0_w(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccenb0W_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57868usize)) }
    }

    #[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccenb1_r(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccenb1R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(58012usize)) }
    }

    #[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccenb1_w(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccenb1W_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57884usize)) }
    }

    #[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccenb2_r(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccenb2R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(58028usize)) }
    }

    #[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccenb2_w(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccenb2W_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57900usize)) }
    }

    #[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccenb3_r(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccenb3R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(58044usize)) }
    }

    #[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccenb3_w(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccenb3W_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57916usize)) }
    }

    #[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccenb4_r(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccenb4R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(58060usize)) }
    }

    #[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccenb4_w(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccenb4W_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57932usize)) }
    }

    #[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccenb5_r(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccenb5R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(58076usize)) }
    }

    #[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccenb5_w(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccenb5W_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57948usize)) }
    }

    #[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccenb6_r(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccenb6R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(58092usize)) }
    }

    #[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccenb6_w(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccenb6W_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57964usize)) }
    }

    #[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccenb7_r(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccenb7R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(58108usize)) }
    }

    #[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnaccenb7_w(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnaccenb7W_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57980usize)) }
    }

    #[doc = "CPUx  Safety Protection DLMU Region Lower Address Register 7\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnla0(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnla0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57856usize)) }
    }

    #[doc = "CPUx  Safety Protection DLMU Region Lower Address Register 7\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnla1(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnla1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57872usize)) }
    }

    #[doc = "CPUx  Safety Protection DLMU Region Lower Address Register 7\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnla2(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnla2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57888usize)) }
    }

    #[doc = "CPUx  Safety Protection DLMU Region Lower Address Register 7\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnla3(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnla3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57904usize)) }
    }

    #[doc = "CPUx  Safety Protection DLMU Region Lower Address Register 7\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnla4(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnla4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57920usize)) }
    }

    #[doc = "CPUx  Safety Protection DLMU Region Lower Address Register 7\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnla5(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnla5_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57936usize)) }
    }

    #[doc = "CPUx  Safety Protection DLMU Region Lower Address Register 7\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnla6(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnla6_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57952usize)) }
    }

    #[doc = "CPUx  Safety Protection DLMU Region Lower Address Register 7\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnla7(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnla7_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57968usize)) }
    }

    #[doc = "CPUx  Safety protection DLMU Region Upper Address Register 7\n resetvalue={Application Reset:0x0FFFFFFE0}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnua0(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnua0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57860usize)) }
    }

    #[doc = "CPUx  Safety protection DLMU Region Upper Address Register 7\n resetvalue={Application Reset:0x0FFFFFFE0}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnua1(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnua1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57876usize)) }
    }

    #[doc = "CPUx  Safety protection DLMU Region Upper Address Register 7\n resetvalue={Application Reset:0x0FFFFFFE0}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnua2(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnua2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57892usize)) }
    }

    #[doc = "CPUx  Safety protection DLMU Region Upper Address Register 7\n resetvalue={Application Reset:0x0FFFFFFE0}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnua3(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnua3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57908usize)) }
    }

    #[doc = "CPUx  Safety protection DLMU Region Upper Address Register 7\n resetvalue={Application Reset:0x0FFFFFFE0}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnua4(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnua4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57924usize)) }
    }

    #[doc = "CPUx  Safety protection DLMU Region Upper Address Register 7\n resetvalue={Application Reset:0x0FFFFFFE0}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnua5(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnua5_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57940usize)) }
    }

    #[doc = "CPUx  Safety protection DLMU Region Upper Address Register 7\n resetvalue={Application Reset:0x0FFFFFFE0}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnua6(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnua6_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57956usize)) }
    }

    #[doc = "CPUx  Safety protection DLMU Region Upper Address Register 7\n resetvalue={Application Reset:0x0FFFFFFE0}"]
    #[inline(always)]
    pub const fn dlmu_sprot_rgnua7(
        &self,
    ) -> crate::common::Reg<self::DlmuSprotRgnua7_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57972usize)) }
    }

    #[doc = "CPUx Debug Monitor Start Address\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dms(&self) -> crate::common::Reg<self::Dms_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130368usize)) }
    }

    #[doc = "CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpre_0(&self) -> crate::common::Reg<self::Dpre0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(122896usize)) }
    }

    #[doc = "CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpre_1(&self) -> crate::common::Reg<self::Dpre1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(122900usize)) }
    }

    #[doc = "CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpre_2(&self) -> crate::common::Reg<self::Dpre2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(122904usize)) }
    }

    #[doc = "CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpre_3(&self) -> crate::common::Reg<self::Dpre3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(122908usize)) }
    }

    #[doc = "CPUx Data Protection Read Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpre_4(&self) -> crate::common::Reg<self::Dpre4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(122960usize)) }
    }

    #[doc = "CPUx Data Protection Read Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpre_5(&self) -> crate::common::Reg<self::Dpre5_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(122964usize)) }
    }

    #[doc = "CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpwe_0(&self) -> crate::common::Reg<self::Dpwe0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(122912usize)) }
    }

    #[doc = "CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpwe_1(&self) -> crate::common::Reg<self::Dpwe1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(122916usize)) }
    }

    #[doc = "CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpwe_2(&self) -> crate::common::Reg<self::Dpwe2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(122920usize)) }
    }

    #[doc = "CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpwe_3(&self) -> crate::common::Reg<self::Dpwe3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(122924usize)) }
    }

    #[doc = "CPUx Data Protection Write Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpwe_4(&self) -> crate::common::Reg<self::Dpwe4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(122976usize)) }
    }

    #[doc = "CPUx Data Protection Write Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpwe_5(&self) -> crate::common::Reg<self::Dpwe5_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(122980usize)) }
    }

    #[doc = "CPUx Data Synchronous Trap Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dstr(&self) -> crate::common::Reg<self::Dstr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(102416usize)) }
    }

    #[doc = "CPUx Data General Purpose Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dy(&self) -> [crate::common::Reg<self::Dy_SPEC, crate::common::RW>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x1ff00usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff00usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff00usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff00usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff00usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff00usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff00usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff00usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff00usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff00usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff00usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff00usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff00usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff00usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff00usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.0.add(0x1ff00usize + 0x3cusize)),
            ]
        }
    }

    #[doc = "CPUx External Event Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn exevt(&self) -> crate::common::Reg<self::Exevt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130312usize)) }
    }

    #[doc = "CPUx Free CSA List Head Pointer\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcx(&self) -> crate::common::Reg<self::Fcx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130616usize)) }
    }

    #[doc = "CPUx Flash Configuration Register 0\n resetvalue={Application Reset:0x3F3F3F3F,CFS Value:0x22212120}"]
    #[inline(always)]
    pub const fn flashcon0(&self) -> crate::common::Reg<self::Flashcon0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4352usize)) }
    }

    #[doc = "CPUx Flash Configuration Register 1\n resetvalue={Application Reset:0x2020000}"]
    #[inline(always)]
    pub const fn flashcon1(&self) -> crate::common::Reg<self::Flashcon1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4356usize)) }
    }

    #[doc = "CPUx Flash Configuration Register 2\n resetvalue={Application Reset:0x0AA020A0A}"]
    #[inline(always)]
    pub const fn flashcon2(&self) -> crate::common::Reg<self::Flashcon2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4360usize)) }
    }

    #[doc = "CPUx Flash Configuration Register 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn flashcon3(&self) -> crate::common::Reg<self::Flashcon3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4364usize)) }
    }

    #[doc = "CPUx Flash Configuration Register 4\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn flashcon4(&self) -> crate::common::Reg<self::Flashcon4_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4368usize)) }
    }

    #[doc = "CPUx Instruction Count\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn icnt(&self) -> crate::common::Reg<self::Icnt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130056usize)) }
    }

    #[doc = "CPUx Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn icr(&self) -> crate::common::Reg<self::Icr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130604usize)) }
    }

    #[doc = "CPUx Interrupt Stack Pointer\n resetvalue={Application Reset:0x100}"]
    #[inline(always)]
    pub const fn isp(&self) -> crate::common::Reg<self::Isp_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130600usize)) }
    }

    #[doc = "CPUx  Reset Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst0(&self) -> crate::common::Reg<self::Krst0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(53248usize)) }
    }

    #[doc = "CPUx  Reset Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst1(&self) -> crate::common::Reg<self::Krst1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(53252usize)) }
    }

    #[doc = "CPUx Reset Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krstclr(&self) -> crate::common::Reg<self::Krstclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(53256usize)) }
    }

    #[doc = "CPUx Free CSA List Limit Pointer\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn lcx(&self) -> crate::common::Reg<self::Lcx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130620usize)) }
    }

    #[doc = "CPUx  Safety Protection Region LPB Read Access Enable Register A\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn lpb_sprot_accena_r(
        &self,
    ) -> crate::common::Reg<self::LpbSprotAccenaR_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57616usize)) }
    }

    #[doc = "CPUx Safety Protection Region LPB Read Access Enable Register B\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn lpb_sprot_accenb_r(
        &self,
    ) -> crate::common::Reg<self::LpbSprotAccenbR_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57620usize)) }
    }

    #[doc = "CPUx Multi Count Register 1\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn m1cnt(&self) -> crate::common::Reg<self::M1Cnt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130060usize)) }
    }

    #[doc = "CPUx Multi Count Register 2\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn m2cnt(&self) -> crate::common::Reg<self::M2Cnt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130064usize)) }
    }

    #[doc = "CPUx Multi Count Register 3\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn m3cnt(&self) -> crate::common::Reg<self::M3Cnt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130068usize)) }
    }

    #[doc = "CPUx  Overlay Range Select Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn osel(&self) -> crate::common::Reg<self::Osel_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64256usize)) }
    }

    #[doc = "CPUx Program Counter\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pc(&self) -> crate::common::Reg<self::Pc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130568usize)) }
    }

    #[doc = "CPUx Program Control 0\n resetvalue={Application Reset:0x2}"]
    #[inline(always)]
    pub const fn pcon0(&self) -> crate::common::Reg<self::Pcon0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(102924usize)) }
    }

    #[doc = "CPUx Program Control 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pcon1(&self) -> crate::common::Reg<self::Pcon1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(102916usize)) }
    }

    #[doc = "CPUx Program Control 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pcon2(&self) -> crate::common::Reg<self::Pcon2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(102920usize)) }
    }

    #[doc = "CPUx Previous Context Information Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pcxi(&self) -> crate::common::Reg<self::Pcxi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130560usize)) }
    }

    #[doc = "CPUx Program Integrity Error Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn piear(&self) -> crate::common::Reg<self::Piear_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(102928usize)) }
    }

    #[doc = "CPUx Program Integrity Error Trap Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pietr(&self) -> crate::common::Reg<self::Pietr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(102932usize)) }
    }

    #[doc = "CPUx Data Access CacheabilityRegister\n resetvalue={Application Reset:0x300}"]
    #[inline(always)]
    pub const fn pma0(&self) -> crate::common::Reg<self::Pma0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(98560usize)) }
    }

    #[doc = "CPUx Code Access CacheabilityRegister\n resetvalue={Application Reset:0x300}"]
    #[inline(always)]
    pub const fn pma1(&self) -> crate::common::Reg<self::Pma1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(98564usize)) }
    }

    #[doc = "CPUx  Peripheral Space Identifier register\n resetvalue={Application Reset:0x0C000}"]
    #[inline(always)]
    pub const fn pma2(&self) -> crate::common::Reg<self::Pma2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(98568usize)) }
    }

    #[doc = "CPUx Program Synchronous Trap Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pstr(&self) -> crate::common::Reg<self::Pstr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(102912usize)) }
    }

    #[doc = "CPUx Program Status Word\n resetvalue={Application Reset:0x0B80}"]
    #[inline(always)]
    pub const fn psw(&self) -> crate::common::Reg<self::Psw_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130564usize)) }
    }

    #[doc = "CPUx SRI Error Generation Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn segen(&self) -> crate::common::Reg<self::Segen_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(69680usize)) }
    }

    #[doc = "CPUx  Safety Protection Register Access Enable Register A\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn sfr_sprot_accena_w(
        &self,
    ) -> crate::common::Reg<self::SfrSprotAccenaW_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57600usize)) }
    }

    #[doc = "CPUx  Safety Protection Region Access Enable Register B\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn sfr_sprot_accenb_w(
        &self,
    ) -> crate::common::Reg<self::SfrSprotAccenbW_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57604usize)) }
    }

    #[doc = "CPUx SIST Mode Access Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn smacon(&self) -> crate::common::Reg<self::Smacon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(102412usize)) }
    }

    #[doc = "CPUx Safety Protection Region SPR Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn spr_sprot_rgnaccena0_r(
        &self,
    ) -> crate::common::Reg<self::SprSprotRgnaccena0R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57480usize)) }
    }

    #[doc = "CPUx Safety Protection Region SPR Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn spr_sprot_rgnaccena1_r(
        &self,
    ) -> crate::common::Reg<self::SprSprotRgnaccena1R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57496usize)) }
    }

    #[doc = "CPUx Safety Protection Region SPR Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn spr_sprot_rgnaccena2_r(
        &self,
    ) -> crate::common::Reg<self::SprSprotRgnaccena2R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57512usize)) }
    }

    #[doc = "CPUx Safety Protection Region SPR Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn spr_sprot_rgnaccena3_r(
        &self,
    ) -> crate::common::Reg<self::SprSprotRgnaccena3R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57528usize)) }
    }

    #[doc = "CPUx Safety Protection Region SPR Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn spr_sprot_rgnaccena4_r(
        &self,
    ) -> crate::common::Reg<self::SprSprotRgnaccena4R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57544usize)) }
    }

    #[doc = "CPUx Safety Protection Region SPR Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn spr_sprot_rgnaccena5_r(
        &self,
    ) -> crate::common::Reg<self::SprSprotRgnaccena5R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57560usize)) }
    }

    #[doc = "CPUx Safety Protection Region SPR Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn spr_sprot_rgnaccena6_r(
        &self,
    ) -> crate::common::Reg<self::SprSprotRgnaccena6R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57576usize)) }
    }

    #[doc = "CPUx Safety Protection Region SPR Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn spr_sprot_rgnaccena7_r(
        &self,
    ) -> crate::common::Reg<self::SprSprotRgnaccena7R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57592usize)) }
    }

    #[doc = "CPUx Safety Protection Region SPR Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn spr_sprot_rgnaccenb0_r(
        &self,
    ) -> crate::common::Reg<self::SprSprotRgnaccenb0R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57484usize)) }
    }

    #[doc = "CPUx Safety Protection Region SPR Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn spr_sprot_rgnaccenb1_r(
        &self,
    ) -> crate::common::Reg<self::SprSprotRgnaccenb1R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57500usize)) }
    }

    #[doc = "CPUx Safety Protection Region SPR Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn spr_sprot_rgnaccenb2_r(
        &self,
    ) -> crate::common::Reg<self::SprSprotRgnaccenb2R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57516usize)) }
    }

    #[doc = "CPUx Safety Protection Region SPR Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn spr_sprot_rgnaccenb3_r(
        &self,
    ) -> crate::common::Reg<self::SprSprotRgnaccenb3R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57532usize)) }
    }

    #[doc = "CPUx Safety Protection Region SPR Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn spr_sprot_rgnaccenb4_r(
        &self,
    ) -> crate::common::Reg<self::SprSprotRgnaccenb4R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57548usize)) }
    }

    #[doc = "CPUx Safety Protection Region SPR Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn spr_sprot_rgnaccenb5_r(
        &self,
    ) -> crate::common::Reg<self::SprSprotRgnaccenb5R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57564usize)) }
    }

    #[doc = "CPUx Safety Protection Region SPR Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn spr_sprot_rgnaccenb6_r(
        &self,
    ) -> crate::common::Reg<self::SprSprotRgnaccenb6R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57580usize)) }
    }

    #[doc = "CPUx Safety Protection Region SPR Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn spr_sprot_rgnaccenb7_r(
        &self,
    ) -> crate::common::Reg<self::SprSprotRgnaccenb7R_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(57596usize)) }
    }

    #[doc = "CPUx Software Debug Event\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn swevt(&self) -> crate::common::Reg<self::Swevt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130320usize)) }
    }

    #[doc = "CPUx System Configuration Register\n resetvalue={Application Reset:0x0,Application Reset:0x0}"]
    #[inline(always)]
    pub const fn syscon(&self) -> crate::common::Reg<self::Syscon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130580usize)) }
    }

    #[doc = "CPUx Task Address Space Identifier Register\n resetvalue={Application Reset:0x1F}"]
    #[inline(always)]
    pub const fn task_asi(&self) -> crate::common::Reg<self::TaskAsi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(98308usize)) }
    }

    #[doc = "CPUx TriggerAddressx\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn trig_acc(&self) -> crate::common::Reg<self::TrigAcc_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(130352usize)) }
    }
    #[doc = "BLK"]
    #[inline(always)]
    pub fn blk(self) -> [self::Blk; 32] {
        unsafe {
            [
                self::Blk(self.0.add(0xfb10usize + 0x0usize)),
                self::Blk(self.0.add(0xfb10usize + 0xcusize)),
                self::Blk(self.0.add(0xfb10usize + 0x18usize)),
                self::Blk(self.0.add(0xfb10usize + 0x24usize)),
                self::Blk(self.0.add(0xfb10usize + 0x30usize)),
                self::Blk(self.0.add(0xfb10usize + 0x3cusize)),
                self::Blk(self.0.add(0xfb10usize + 0x48usize)),
                self::Blk(self.0.add(0xfb10usize + 0x54usize)),
                self::Blk(self.0.add(0xfb10usize + 0x60usize)),
                self::Blk(self.0.add(0xfb10usize + 0x6cusize)),
                self::Blk(self.0.add(0xfb10usize + 0x78usize)),
                self::Blk(self.0.add(0xfb10usize + 0x84usize)),
                self::Blk(self.0.add(0xfb10usize + 0x90usize)),
                self::Blk(self.0.add(0xfb10usize + 0x9cusize)),
                self::Blk(self.0.add(0xfb10usize + 0xa8usize)),
                self::Blk(self.0.add(0xfb10usize + 0xb4usize)),
                self::Blk(self.0.add(0xfb10usize + 0xc0usize)),
                self::Blk(self.0.add(0xfb10usize + 0xccusize)),
                self::Blk(self.0.add(0xfb10usize + 0xd8usize)),
                self::Blk(self.0.add(0xfb10usize + 0xe4usize)),
                self::Blk(self.0.add(0xfb10usize + 0xf0usize)),
                self::Blk(self.0.add(0xfb10usize + 0xfcusize)),
                self::Blk(self.0.add(0xfb10usize + 0x108usize)),
                self::Blk(self.0.add(0xfb10usize + 0x114usize)),
                self::Blk(self.0.add(0xfb10usize + 0x120usize)),
                self::Blk(self.0.add(0xfb10usize + 0x12cusize)),
                self::Blk(self.0.add(0xfb10usize + 0x138usize)),
                self::Blk(self.0.add(0xfb10usize + 0x144usize)),
                self::Blk(self.0.add(0xfb10usize + 0x150usize)),
                self::Blk(self.0.add(0xfb10usize + 0x15cusize)),
                self::Blk(self.0.add(0xfb10usize + 0x168usize)),
                self::Blk(self.0.add(0xfb10usize + 0x174usize)),
            ]
        }
    }
    #[doc = "CPR"]
    #[inline(always)]
    pub fn cpr(self) -> [self::Cpr; 10] {
        unsafe {
            [
                self::Cpr(self.0.add(0x1d000usize + 0x0usize)),
                self::Cpr(self.0.add(0x1d000usize + 0x8usize)),
                self::Cpr(self.0.add(0x1d000usize + 0x10usize)),
                self::Cpr(self.0.add(0x1d000usize + 0x18usize)),
                self::Cpr(self.0.add(0x1d000usize + 0x20usize)),
                self::Cpr(self.0.add(0x1d000usize + 0x28usize)),
                self::Cpr(self.0.add(0x1d000usize + 0x30usize)),
                self::Cpr(self.0.add(0x1d000usize + 0x38usize)),
                self::Cpr(self.0.add(0x1d000usize + 0x40usize)),
                self::Cpr(self.0.add(0x1d000usize + 0x48usize)),
            ]
        }
    }
    #[doc = "DPR"]
    #[inline(always)]
    pub fn dpr(self) -> [self::Dpr; 18] {
        unsafe {
            [
                self::Dpr(self.0.add(0x1c000usize + 0x0usize)),
                self::Dpr(self.0.add(0x1c000usize + 0x8usize)),
                self::Dpr(self.0.add(0x1c000usize + 0x10usize)),
                self::Dpr(self.0.add(0x1c000usize + 0x18usize)),
                self::Dpr(self.0.add(0x1c000usize + 0x20usize)),
                self::Dpr(self.0.add(0x1c000usize + 0x28usize)),
                self::Dpr(self.0.add(0x1c000usize + 0x30usize)),
                self::Dpr(self.0.add(0x1c000usize + 0x38usize)),
                self::Dpr(self.0.add(0x1c000usize + 0x40usize)),
                self::Dpr(self.0.add(0x1c000usize + 0x48usize)),
                self::Dpr(self.0.add(0x1c000usize + 0x50usize)),
                self::Dpr(self.0.add(0x1c000usize + 0x58usize)),
                self::Dpr(self.0.add(0x1c000usize + 0x60usize)),
                self::Dpr(self.0.add(0x1c000usize + 0x68usize)),
                self::Dpr(self.0.add(0x1c000usize + 0x70usize)),
                self::Dpr(self.0.add(0x1c000usize + 0x78usize)),
                self::Dpr(self.0.add(0x1c000usize + 0x80usize)),
                self::Dpr(self.0.add(0x1c000usize + 0x88usize)),
            ]
        }
    }
    #[doc = "FPU TRAP"]
    #[inline(always)]
    pub fn fpu_trap(self) -> self::FpuTrap {
        unsafe { self::FpuTrap(self.0.add(106496usize)) }
    }
    #[doc = "RGN"]
    #[inline(always)]
    pub fn rgn(self) -> [self::Rgn; 8] {
        unsafe {
            [
                self::Rgn(self.0.add(0xe000usize + 0x0usize)),
                self::Rgn(self.0.add(0xe000usize + 0x10usize)),
                self::Rgn(self.0.add(0xe000usize + 0x20usize)),
                self::Rgn(self.0.add(0xe000usize + 0x30usize)),
                self::Rgn(self.0.add(0xe000usize + 0x40usize)),
                self::Rgn(self.0.add(0xe000usize + 0x50usize)),
                self::Rgn(self.0.add(0xe000usize + 0x60usize)),
                self::Rgn(self.0.add(0xe000usize + 0x70usize)),
            ]
        }
    }
    #[doc = "TPS"]
    #[inline(always)]
    pub fn tps(self) -> self::Tps {
        unsafe { self::Tps(self.0.add(123904usize)) }
    }
    #[doc = "TPS EXTIM"]
    #[inline(always)]
    pub fn tps_extim(self) -> self::TpsExtim {
        unsafe { self::TpsExtim(self.0.add(123968usize)) }
    }
    #[doc = "Trigger"]
    #[inline(always)]
    pub fn tr(self) -> [self::Tr; 8] {
        unsafe {
            [
                self::Tr(self.0.add(0x1f000usize + 0x0usize)),
                self::Tr(self.0.add(0x1f000usize + 0x8usize)),
                self::Tr(self.0.add(0x1f000usize + 0x10usize)),
                self::Tr(self.0.add(0x1f000usize + 0x18usize)),
                self::Tr(self.0.add(0x1f000usize + 0x20usize)),
                self::Tr(self.0.add(0x1f000usize + 0x28usize)),
                self::Tr(self.0.add(0x1f000usize + 0x30usize)),
                self::Tr(self.0.add(0x1f000usize + 0x38usize)),
            ]
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ay_SPEC;
impl crate::sealed::RegSpec for Ay_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Address General Purpose Register 0\n resetvalue={Application Reset:0x0}"]
pub type Ay = crate::RegValueT<Ay_SPEC>;

impl Ay {
    #[doc = "Address Register   ADDR. General purpose registers"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Ay_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Ay_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Ay {
    #[inline(always)]
    fn default() -> Ay {
        <crate::RegValueT<Ay_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Biv_SPEC;
impl crate::sealed::RegSpec for Biv_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Base Interrupt Vector Table Pointer\n resetvalue={Application Reset:0x0}"]
pub type Biv = crate::RegValueT<Biv_SPEC>;

impl Biv {
    #[doc = "Vector Spacing Select   VSS. 0  32 byte vector spacing. 1  8 Byte vector spacing."]
    #[inline(always)]
    pub fn vss(self) -> crate::common::RegisterFieldBool<0, 1, 0, Biv_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Biv_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Base Address of Interrupt Vector Table   BIV. The address in the BIV register must be aligned to an even byte address  halfword address . Because of the simple ORing of the left shifted priority number and the contents of the BIV register  the alignment of the base address of the vector table must be to a power of two boundary  dependent on the number of interrupt entries used. For the full range of 256 interrupt entries an alignment to an 8 KByte boundary is required. If fewer sources are used  the alignment requirements are correspondingly relaxed."]
    #[inline(always)]
    pub fn biv(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, Biv_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32, Biv_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Biv {
    #[inline(always)]
    fn default() -> Biv {
        <crate::RegValueT<Biv_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btv_SPEC;
impl crate::sealed::RegSpec for Btv_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Base Trap Vector Table Pointer\n resetvalue={Application Reset:0x0A0000100}"]
pub type Btv = crate::RegValueT<Btv_SPEC>;

impl Btv {
    #[doc = "Base Address of Trap Vector Table   BTV. The address in the BTV register must be aligned to an even byte address  halfword address . Also  due to the simple ORing of the left shifted trap identification number and the contents of the BTV register  the alignment of the base address of the vector table must be to a power of two boundary. There are eight different trap classes  resulting in Trap Classes from 0 to 7. The contents of BTV should therefore be set to at least a 256 byte boundary  8 Trap Classes   8 word spacing ."]
    #[inline(always)]
    pub fn btv(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, Btv_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32, Btv_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Btv {
    #[inline(always)]
    fn default() -> Btv {
        <crate::RegValueT<Btv_SPEC> as RegisterValue<_>>::new(2684354816)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccnt_SPEC;
impl crate::sealed::RegSpec for Ccnt_SPEC {
    type DataType = u32;
}
#[doc = "CPUx CPU Clock Cycle Count\n resetvalue={Debug Reset:0x0}"]
pub type Ccnt = crate::RegValueT<Ccnt_SPEC>;

impl Ccnt {
    #[doc = "Count Value   CountValue. Current Count of the CPU Clock Cycles."]
    #[inline(always)]
    pub fn countvalue(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffffff, 1, 0, u32, Ccnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7fffffff,1,0,u32, Ccnt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sticky Overflow Bit   SOvf. This bit is set by hardware when count value  30 0    31 h7FFF FFFF. It can only be cleared by software."]
    #[inline(always)]
    pub fn sovf(self) -> crate::common::RegisterFieldBool<31, 1, 0, Ccnt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ccnt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ccnt {
    #[inline(always)]
    fn default() -> Ccnt {
        <crate::RegValueT<Ccnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cctrl_SPEC;
impl crate::sealed::RegSpec for Cctrl_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Counter Control\n resetvalue={Debug Reset:0x0}"]
pub type Cctrl = crate::RegValueT<Cctrl_SPEC>;

impl Cctrl {
    #[doc = "Counter Mode   CM"]
    #[inline(always)]
    pub fn cm(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, cctrl::Cm, Cctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,cctrl::Cm, Cctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Count Enable   CE"]
    #[inline(always)]
    pub fn ce(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, cctrl::Ce, Cctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,cctrl::Ce, Cctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "M1CNT Configuration   M1"]
    #[inline(always)]
    pub fn m1(
        self,
    ) -> crate::common::RegisterField<2, 0x7, 1, 0, u8, Cctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x7,1,0,u8, Cctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "M2CNT Configuration   M2"]
    #[inline(always)]
    pub fn m2(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, Cctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8, Cctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "M3CNT Configuration   M3"]
    #[inline(always)]
    pub fn m3(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Cctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Cctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Cctrl {
    #[inline(always)]
    fn default() -> Cctrl {
        <crate::RegValueT<Cctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cctrl {
    pub struct Cm_SPEC;
    pub type Cm = crate::EnumBitfieldStruct<u8, Cm_SPEC>;
    impl Cm {
        #[doc = "0 Normal Mode."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Task Mode."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Ce_SPEC;
    pub type Ce = crate::EnumBitfieldStruct<u8, Ce_SPEC>;
    impl Ce {
        #[doc = "0 Disable the counters  CCNT  ICNT  M1CNT  M2CNT  M3CNT."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enable the counters  CCNT  ICNT  M1CNT  M2CNT  M3CNT."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Compat_SPEC;
impl crate::sealed::RegSpec for Compat_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Compatibility Control Register\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Compat = crate::RegValueT<Compat_SPEC>;

impl Compat {
    #[doc = "Rounding Mode Compatibility   RM"]
    #[inline(always)]
    pub fn rm(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, compat::Rm, Compat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,compat::Rm, Compat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SYSCON Safety Protection Mode Compatibility   SP"]
    #[inline(always)]
    pub fn sp(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, compat::Sp, Compat_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,compat::Sp, Compat_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Compat {
    #[inline(always)]
    fn default() -> Compat {
        <crate::RegValueT<Compat_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod compat {
    pub struct Rm_SPEC;
    pub type Rm = crate::EnumBitfieldStruct<u8, Rm_SPEC>;
    impl Rm {
        #[doc = "0 PSW.RM not restored by RET."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 PSW.RM restored by RET  TC1.3 behavior ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sp_SPEC;
    pub type Sp = crate::EnumBitfieldStruct<u8, Sp_SPEC>;
    impl Sp {
        #[doc = "0 SYSCON 31 1  safety endinit protected."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 SYSCON 31 1  not safety endinit protected  TC1.3 behavior ."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoreId_SPEC;
impl crate::sealed::RegSpec for CoreId_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Core Identification Register\n resetvalue={Application Reset:0x0}"]
pub type CoreId = crate::RegValueT<CoreId_SPEC>;

impl CoreId {
    #[doc = "Core Identification Number   CORE ID. The identification number of the core."]
    #[inline(always)]
    pub fn core_id(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, CoreId_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8, CoreId_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for CoreId {
    #[inline(always)]
    fn default() -> CoreId {
        <crate::RegValueT<CoreId_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CpuId_SPEC;
impl crate::sealed::RegSpec for CpuId_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Identification Register TC1.6.2P\n resetvalue={Application Reset:0x0C0C021}"]
pub type CpuId = crate::RegValueT<CpuId_SPEC>;

impl CpuId {
    #[doc = "Revision Number   MOD REV"]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, cpu_id::ModRev, CpuId_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,cpu_id::ModRev, CpuId_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "32 Bit Module Enable   MOD 32B"]
    #[inline(always)]
    pub fn mod_32b(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, cpu_id::Mod32B, CpuId_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,cpu_id::Mod32B, CpuId_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Module Identification Number   MOD"]
    #[inline(always)]
    pub fn r#mod(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, cpu_id::Mod, CpuId_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xffff,1,0,cpu_id::Mod, CpuId_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for CpuId {
    #[inline(always)]
    fn default() -> CpuId {
        <crate::RegValueT<CpuId_SPEC> as RegisterValue<_>>::new(12632097)
    }
}
pub mod cpu_id {
    pub struct ModRev_SPEC;
    pub type ModRev = crate::EnumBitfieldStruct<u8, ModRev_SPEC>;
    impl ModRev {
        #[doc = "20 Reset value"]
        pub const CONST_3232: Self = Self::new(32);
    }
    pub struct Mod32B_SPEC;
    pub type Mod32B = crate::EnumBitfieldStruct<u8, Mod32B_SPEC>;
    impl Mod32B {
        #[doc = "C0 A value of C0 H in this field indicates a 32 bit module with a 32 bit module ID register."]
        pub const CONST_192192: Self = Self::new(192);
    }
    pub struct Mod_SPEC;
    pub type Mod = crate::EnumBitfieldStruct<u8, Mod_SPEC>;
    impl Mod {
        #[doc = "00C0 For module identification."]
        pub const CONST_192192: Self = Self::new(192);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpxe0_SPEC;
impl crate::sealed::RegSpec for Cpxe0_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Cpxe0 = crate::RegValueT<Cpxe0_SPEC>;

impl Cpxe0 {
    #[doc = "Execute Enable Range select   XE n"]
    #[inline(always)]
    pub fn xe_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, cpxe_0::XeN, Cpxe0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,cpxe_0::XeN, Cpxe0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Cpxe0 {
    #[inline(always)]
    fn default() -> Cpxe0 {
        <crate::RegValueT<Cpxe0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cpxe_0 {
    pub struct XeN_SPEC;
    pub type XeN = crate::EnumBitfieldStruct<u8, XeN_SPEC>;
    impl XeN {
        #[doc = "0 Code Protection        Range n not enabled for execution"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Code Protection        Range n enabled for execution"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpxe1_SPEC;
impl crate::sealed::RegSpec for Cpxe1_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Cpxe1 = crate::RegValueT<Cpxe1_SPEC>;

impl Cpxe1 {
    #[doc = "Execute Enable Range select   XE n"]
    #[inline(always)]
    pub fn xe_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, cpxe_1::XeN, Cpxe1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,cpxe_1::XeN, Cpxe1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Cpxe1 {
    #[inline(always)]
    fn default() -> Cpxe1 {
        <crate::RegValueT<Cpxe1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cpxe_1 {
    pub struct XeN_SPEC;
    pub type XeN = crate::EnumBitfieldStruct<u8, XeN_SPEC>;
    impl XeN {
        #[doc = "0 Code Protection        Range n not enabled for execution"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Code Protection        Range n enabled for execution"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpxe2_SPEC;
impl crate::sealed::RegSpec for Cpxe2_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Cpxe2 = crate::RegValueT<Cpxe2_SPEC>;

impl Cpxe2 {
    #[doc = "Execute Enable Range select   XE n"]
    #[inline(always)]
    pub fn xe_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, cpxe_2::XeN, Cpxe2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,cpxe_2::XeN, Cpxe2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Cpxe2 {
    #[inline(always)]
    fn default() -> Cpxe2 {
        <crate::RegValueT<Cpxe2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cpxe_2 {
    pub struct XeN_SPEC;
    pub type XeN = crate::EnumBitfieldStruct<u8, XeN_SPEC>;
    impl XeN {
        #[doc = "0 Code Protection        Range n not enabled for execution"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Code Protection        Range n enabled for execution"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpxe3_SPEC;
impl crate::sealed::RegSpec for Cpxe3_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Cpxe3 = crate::RegValueT<Cpxe3_SPEC>;

impl Cpxe3 {
    #[doc = "Execute Enable Range select   XE n"]
    #[inline(always)]
    pub fn xe_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, cpxe_3::XeN, Cpxe3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,cpxe_3::XeN, Cpxe3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Cpxe3 {
    #[inline(always)]
    fn default() -> Cpxe3 {
        <crate::RegValueT<Cpxe3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cpxe_3 {
    pub struct XeN_SPEC;
    pub type XeN = crate::EnumBitfieldStruct<u8, XeN_SPEC>;
    impl XeN {
        #[doc = "0 Code Protection        Range n not enabled for execution"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Code Protection        Range n enabled for execution"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpxe4_SPEC;
impl crate::sealed::RegSpec for Cpxe4_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Code Protection Execute Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
pub type Cpxe4 = crate::RegValueT<Cpxe4_SPEC>;

impl Cpxe4 {
    #[doc = "Execute Enable Range select   XE n"]
    #[inline(always)]
    pub fn xe_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, cpxe_4::XeN, Cpxe4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,cpxe_4::XeN, Cpxe4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Cpxe4 {
    #[inline(always)]
    fn default() -> Cpxe4 {
        <crate::RegValueT<Cpxe4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cpxe_4 {
    pub struct XeN_SPEC;
    pub type XeN = crate::EnumBitfieldStruct<u8, XeN_SPEC>;
    impl XeN {
        #[doc = "0 Code Protection        Range n not enabled for execution"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Code Protection        Range n enabled for execution"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpxe5_SPEC;
impl crate::sealed::RegSpec for Cpxe5_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Code Protection Execute Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
pub type Cpxe5 = crate::RegValueT<Cpxe5_SPEC>;

impl Cpxe5 {
    #[doc = "Execute Enable Range select   XE n"]
    #[inline(always)]
    pub fn xe_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, cpxe_5::XeN, Cpxe5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ff,1,0,cpxe_5::XeN, Cpxe5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Cpxe5 {
    #[inline(always)]
    fn default() -> Cpxe5 {
        <crate::RegValueT<Cpxe5_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod cpxe_5 {
    pub struct XeN_SPEC;
    pub type XeN = crate::EnumBitfieldStruct<u8, XeN_SPEC>;
    impl XeN {
        #[doc = "0 Code Protection        Range n not enabled for execution"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Code Protection        Range n enabled for execution"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crevt_SPEC;
impl crate::sealed::RegSpec for Crevt_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Core Register Access Event\n resetvalue={Debug Reset:0x0}"]
pub type Crevt = crate::RegValueT<Crevt_SPEC>;

impl Crevt {
    #[doc = "Event Associated   EVTA. Debug Action associated with the Debug Event"]
    #[inline(always)]
    pub fn evta(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, crevt::Evta, Crevt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,crevt::Evta, Crevt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Break Before Make  BBM  or Break After Make  BAM  Selection   BBM"]
    #[inline(always)]
    pub fn bbm(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, crevt::Bbm, Crevt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,crevt::Bbm, Crevt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Breakout Disable   BOD"]
    #[inline(always)]
    pub fn bod(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, crevt::Bod, Crevt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,crevt::Bod, Crevt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CDC Suspend Out Signal State   SUSP. Value to be assigned to the CDC suspend out signal when the Debug Event is raised."]
    #[inline(always)]
    pub fn susp(self) -> crate::common::RegisterFieldBool<5, 1, 0, Crevt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Crevt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Counter   CNT. When this event occurs adjust the control of the performance counters in task mode as follows"]
    #[inline(always)]
    pub fn cnt(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, crevt::Cnt, Crevt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,crevt::Cnt, Crevt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Crevt {
    #[inline(always)]
    fn default() -> Crevt {
        <crate::RegValueT<Crevt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod crevt {
    pub struct Evta_SPEC;
    pub type Evta = crate::EnumBitfieldStruct<u8, Evta_SPEC>;
    impl Evta {
        #[doc = "000 BOD 0  Disabled. BOD 1  Disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 BOD 0  Pulse BRKOUT Signal. BOD 1  None."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 BOD 0  Halt and pulse BRKOUT Signal. BOD 1  Halt."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 BOD 0  Breakpoint trap and pulse BRKOUT Signal. BOD 1  Breakpoint trap."]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 BOD 0  Breakpoint interrupt 0 and pulse BRKOUT Signal. BOD 1  Breakpoint interrupt 0."]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 BOD 0  If implemented  breakpoint interrupt 1 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 1. If not implemented  None."]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 BOD 0  If implemented  breakpoint interrupt 2 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 2. If not implemented  None."]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "111 BOD 0  If implemented  breakpoint interrupt 3 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 3. If not implemented  None."]
        pub const CONST_77: Self = Self::new(7);
    }
    pub struct Bbm_SPEC;
    pub type Bbm = crate::EnumBitfieldStruct<u8, Bbm_SPEC>;
    impl Bbm {
        #[doc = "0 Break after make  BAM ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Break before make  BBM ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Bod_SPEC;
    pub type Bod = crate::EnumBitfieldStruct<u8, Bod_SPEC>;
    impl Bod {
        #[doc = "0 BRKOUT signal asserted according to the action specified in the EVTA field."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 BRKOUT signal not asserted. This takes priority over any assertion generated by the EVTA field."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cnt_SPEC;
    pub type Cnt = crate::EnumBitfieldStruct<u8, Cnt_SPEC>;
    impl Cnt {
        #[doc = "00 No change."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Start the performance counters."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Stop the performance counters."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Toggle the performance counter control  i.e. start it if it is currently stopped  stop it if it is currently running ."]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CusId_SPEC;
impl crate::sealed::RegSpec for CusId_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Customer ID register\n resetvalue={Application Reset:0x0}"]
pub type CusId = crate::RegValueT<CusId_SPEC>;

impl CusId {
    #[doc = "Customer ID   CID. See CROSSREFERENCE for the relation between CUS ID and CORE ID for each derivative"]
    #[inline(always)]
    pub fn cid(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, CusId_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8, CusId_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for CusId {
    #[inline(always)]
    fn default() -> CusId {
        <crate::RegValueT<CusId_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Datr_SPEC;
impl crate::sealed::RegSpec for Datr_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Asynchronous Trap Register\n resetvalue={Application Reset:0x0}"]
pub type Datr = crate::RegValueT<Datr_SPEC>;

impl Datr {
    #[doc = "Store Bus Error   SBE"]
    #[inline(always)]
    pub fn sbe(self) -> crate::common::RegisterFieldBool<3, 1, 0, Datr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Datr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Cache Writeback Error   CWE"]
    #[inline(always)]
    pub fn cwe(self) -> crate::common::RegisterFieldBool<9, 1, 0, Datr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Datr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Cache Flush Error   CFE"]
    #[inline(always)]
    pub fn cfe(self) -> crate::common::RegisterFieldBool<10, 1, 0, Datr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Datr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Store Overlay Error   SOE"]
    #[inline(always)]
    pub fn soe(self) -> crate::common::RegisterFieldBool<14, 1, 0, Datr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Datr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Datr {
    #[inline(always)]
    fn default() -> Datr {
        <crate::RegValueT<Datr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgsr_SPEC;
impl crate::sealed::RegSpec for Dbgsr_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Debug Status Register\n resetvalue={Debug Reset:0x0}"]
pub type Dbgsr = crate::RegValueT<Dbgsr_SPEC>;

impl Dbgsr {
    #[doc = "Debug Enable   DE. Determines whether the CDC is enabled or not."]
    #[inline(always)]
    pub fn de(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, dbgsr::De, Dbgsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,dbgsr::De, Dbgsr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "CPU Halt Request   Status Field   HALT. HALT can be set or cleared by software. HALT 0  is the actual Halt bit. HALT 1  is a mask bit to specify whether or not HALT 0  is to be updated on a software write. HALT 1  is always read as 0. HALT 1  must be set to 1 in order to update HALT 0  by software  R  read  W  write ."]
    #[inline(always)]
    pub fn halt(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, dbgsr::Halt, Dbgsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x3,1,0,dbgsr::Halt, Dbgsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Suspend in Halt   SIH. State of the Suspend In signal."]
    #[inline(always)]
    pub fn sih(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, dbgsr::Sih, Dbgsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<3,0x1,1,0,dbgsr::Sih, Dbgsr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Current State of the Core Suspend Out Signal   SUSP"]
    #[inline(always)]
    pub fn susp(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, dbgsr::Susp, Dbgsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,dbgsr::Susp, Dbgsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Previous State of Core Suspend Out Signal   PREVSUSP. Updated when a Debug Event causes a hardware update of DBGSR.SUSP. This field is not updated for writes to DBGSR.SUSP."]
    #[inline(always)]
    pub fn prevsusp(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, dbgsr::Prevsusp, Dbgsr_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x1,1,0,dbgsr::Prevsusp, Dbgsr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Posted Event   PEVT"]
    #[inline(always)]
    pub fn pevt(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, dbgsr::Pevt, Dbgsr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,dbgsr::Pevt, Dbgsr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dbgsr {
    #[inline(always)]
    fn default() -> Dbgsr {
        <crate::RegValueT<Dbgsr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dbgsr {
    pub struct De_SPEC;
    pub type De = crate::EnumBitfieldStruct<u8, De_SPEC>;
    impl De {
        #[doc = "0 The CDC is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The CDC is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Halt_SPEC;
    pub type Halt = crate::EnumBitfieldStruct<u8, Halt_SPEC>;
    impl Halt {
        #[doc = "00 R  CPU running.  W  HALT 0  unchanged."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 R  CPU halted.  W  HALT 0  unchanged."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 R  Not Applicable. W  reset HALT 0 ."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 R  Not Applicable. W  If DBGSR.DE    1  The CDC is enabled   set HALT 0 . If DBGSR.DE    0  The CDC is not enabled   HALT 0  is left unchanged."]
        pub const CONST_33: Self = Self::new(3);
    }
    pub struct Sih_SPEC;
    pub type Sih = crate::EnumBitfieldStruct<u8, Sih_SPEC>;
    impl Sih {
        #[doc = "0 The Suspend In signal is negated. The CPU is not in Halt Mode   except when the Halt mechanism is set following a Debug Event or a write to DBGSR.HALT ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 The Suspend In signal is asserted. The CPU is in Halt Mode."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Susp_SPEC;
    pub type Susp = crate::EnumBitfieldStruct<u8, Susp_SPEC>;
    impl Susp {
        #[doc = "0 Core suspend out inactive."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Core suspend out active."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Prevsusp_SPEC;
    pub type Prevsusp = crate::EnumBitfieldStruct<u8, Prevsusp_SPEC>;
    impl Prevsusp {
        #[doc = "0 Previous core suspend out inactive."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Previous core suspend out active."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pevt_SPEC;
    pub type Pevt = crate::EnumBitfieldStruct<u8, Pevt_SPEC>;
    impl Pevt {
        #[doc = "0 No posted event."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Posted event."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgtcr_SPEC;
impl crate::sealed::RegSpec for Dbgtcr_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Debug Trap Control Register\n resetvalue={Application Reset:0x1}"]
pub type Dbgtcr = crate::RegValueT<Dbgtcr_SPEC>;

impl Dbgtcr {
    #[doc = "Debug Trap Active Bit   DTA. A breakpoint trap may only be taken in the condition DTA    0. Taking a breakpoint trap sets the DTA bit to one. Further breakpoint traps are therefore disabled until such time as the breakpoint trap handler clears the DTA bit or until the breakpoint trap handler terminates with a RFM."]
    #[inline(always)]
    pub fn dta(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, dbgtcr::Dta, Dbgtcr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,dbgtcr::Dta, Dbgtcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dbgtcr {
    #[inline(always)]
    fn default() -> Dbgtcr {
        <crate::RegValueT<Dbgtcr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod dbgtcr {
    pub struct Dta_SPEC;
    pub type Dta = crate::EnumBitfieldStruct<u8, Dta_SPEC>;
    impl Dta {
        #[doc = "0 No breakpoint trap is active."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A breakpoint Trap is active"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcon0_SPEC;
impl crate::sealed::RegSpec for Dcon0_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Memory Control Register\n resetvalue={Application Reset:0x2}"]
pub type Dcon0 = crate::RegValueT<Dcon0_SPEC>;

impl Dcon0 {
    #[doc = "Data Cache Bypass   DCBYP"]
    #[inline(always)]
    pub fn dcbyp(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, dcon0::Dcbyp, Dcon0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,dcon0::Dcbyp, Dcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dcon0 {
    #[inline(always)]
    fn default() -> Dcon0 {
        <crate::RegValueT<Dcon0_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod dcon0 {
    pub struct Dcbyp_SPEC;
    pub type Dcbyp = crate::EnumBitfieldStruct<u8, Dcbyp_SPEC>;
    impl Dcbyp {
        #[doc = "0 DCache   DRB enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 DCache   DRB Bypass  disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcon2_SPEC;
impl crate::sealed::RegSpec for Dcon2_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Control Register 2\n resetvalue={Application Reset:0x0}"]
pub type Dcon2 = crate::RegValueT<Dcon2_SPEC>;

impl Dcon2 {
    #[doc = "Data Cache Size   DCACHE SZE. In KBytes"]
    #[inline(always)]
    pub fn dcache_sze(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Dcon2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Dcon2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Data Scratch Size   DSCRATCH SZE. In KBytes"]
    #[inline(always)]
    pub fn dscratch_sze(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Dcon2_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Dcon2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Dcon2 {
    #[inline(always)]
    fn default() -> Dcon2 {
        <crate::RegValueT<Dcon2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcx_SPEC;
impl crate::sealed::RegSpec for Dcx_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Debug Context Save Area Pointer\n resetvalue={Application Reset:0x0}"]
pub type Dcx = crate::RegValueT<Dcx_SPEC>;

impl Dcx {
    #[doc = "Debug Context Save Area Pointer   DCXValue. Address where the debug context is stored following a breakpoint trap."]
    #[inline(always)]
    pub fn dcxvalue(
        self,
    ) -> crate::common::RegisterField<6, 0x3ffffff, 1, 0, u32, Dcx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3ffffff,1,0,u32, Dcx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dcx {
    #[inline(always)]
    fn default() -> Dcx {
        <crate::RegValueT<Dcx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Deadd_SPEC;
impl crate::sealed::RegSpec for Deadd_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Error Address Register\n resetvalue={Application Reset:0x0}"]
pub type Deadd = crate::RegValueT<Deadd_SPEC>;

impl Deadd {
    #[doc = "Error Address   ERROR ADDRESS"]
    #[inline(always)]
    pub fn error_address(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Deadd_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Deadd_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Deadd {
    #[inline(always)]
    fn default() -> Deadd {
        <crate::RegValueT<Deadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diear_SPEC;
impl crate::sealed::RegSpec for Diear_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Integrity Error Address Register\n resetvalue={Application Reset:0x0}"]
pub type Diear = crate::RegValueT<Diear_SPEC>;

impl Diear {
    #[doc = "Transaction Address   TA. Physical address being accessed by operation that encountered data integrity error."]
    #[inline(always)]
    pub fn ta(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Diear_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Diear_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Diear {
    #[inline(always)]
    fn default() -> Diear {
        <crate::RegValueT<Diear_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dietr_SPEC;
impl crate::sealed::RegSpec for Dietr_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Integrity Error Trap Register\n resetvalue={Application Reset:0x0}"]
pub type Dietr = crate::RegValueT<Dietr_SPEC>;

impl Dietr {
    #[doc = "Integrity Error Detected   IED"]
    #[inline(always)]
    pub fn ied(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, dietr::Ied, Dietr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,dietr::Ied, Dietr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Integrity Error   Tag Memory   IE T"]
    #[inline(always)]
    pub fn ie_t(self) -> crate::common::RegisterFieldBool<1, 1, 0, Dietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Dietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Integrity Error   Cache Memory   IE C"]
    #[inline(always)]
    pub fn ie_c(self) -> crate::common::RegisterFieldBool<2, 1, 0, Dietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Dietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Integrity Error   Scratchpad Memory   IE S"]
    #[inline(always)]
    pub fn ie_s(self) -> crate::common::RegisterFieldBool<3, 1, 0, Dietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Dietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Integrity Error   Bus Interface   IE BI"]
    #[inline(always)]
    pub fn ie_bi(self) -> crate::common::RegisterFieldBool<4, 1, 0, Dietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Dietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Error Information   E INFO. If IE BS   1  Bus Master Tag ID of requesting masterIf IE C   1  Cache way."]
    #[inline(always)]
    pub fn e_info(
        self,
    ) -> crate::common::RegisterField<5, 0x3f, 1, 0, u8, Dietr_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x3f,1,0,u8, Dietr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Dual Bit Error Detected   IE UNC"]
    #[inline(always)]
    pub fn ie_unc(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Dietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Safety Protection Error Detected   IE SP"]
    #[inline(always)]
    pub fn ie_sp(self) -> crate::common::RegisterFieldBool<12, 1, 0, Dietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Dietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Bus Slave Access Indicator   IE BS"]
    #[inline(always)]
    pub fn ie_bs(self) -> crate::common::RegisterFieldBool<13, 1, 0, Dietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Dietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Integrity Error   DLMU   IE DLMU"]
    #[inline(always)]
    pub fn ie_dlmu(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Dietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Dietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Integrity Error   Local Pflash Bank   IE LPB"]
    #[inline(always)]
    pub fn ie_lpb(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Dietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Dietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Memory Test Mode Violation detected   IE MTMV"]
    #[inline(always)]
    pub fn ie_mtmv(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Dietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Dietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Dietr {
    #[inline(always)]
    fn default() -> Dietr {
        <crate::RegValueT<Dietr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dietr {
    pub struct Ied_SPEC;
    pub type Ied = crate::EnumBitfieldStruct<u8, Ied_SPEC>;
    impl Ied {
        #[doc = "0 Write  Clear IED bit  re enable DIETR and DIEAR update. Read   No data integrity error condition occurred"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write   No Effect. Read  Data integrity error condition detected. DIETR and DIEAR contents valid  further DIETR and DIEAR updates disabled.."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccena0R_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccena0R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccena0R = crate::RegValueT<DlmuSprotRgnaccena0R_SPEC>;

impl DlmuSprotRgnaccena0R {
    #[doc = "Read Access Enable for Master TAG ID n  n 0 31    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        dlmu_sprot_rgnaccena0_r::En,
        DlmuSprotRgnaccena0R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            dlmu_sprot_rgnaccena0_r::En,
            DlmuSprotRgnaccena0R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccena0R {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccena0R {
        <crate::RegValueT<DlmuSprotRgnaccena0R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod dlmu_sprot_rgnaccena0_r {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Read access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccena0W_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccena0W_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccena0W = crate::RegValueT<DlmuSprotRgnaccena0W_SPEC>;

impl DlmuSprotRgnaccena0W {
    #[doc = "Write Access Enable for Master TAG ID n  n 0 31    EN. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        dlmu_sprot_rgnaccena0_w::En,
        DlmuSprotRgnaccena0W_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            dlmu_sprot_rgnaccena0_w::En,
            DlmuSprotRgnaccena0W_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccena0W {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccena0W {
        <crate::RegValueT<DlmuSprotRgnaccena0W_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod dlmu_sprot_rgnaccena0_w {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccena1R_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccena1R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccena1R = crate::RegValueT<DlmuSprotRgnaccena1R_SPEC>;

impl DlmuSprotRgnaccena1R {
    #[doc = "Read Access Enable for Master TAG ID n  n 0 31    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        dlmu_sprot_rgnaccena1_r::En,
        DlmuSprotRgnaccena1R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            dlmu_sprot_rgnaccena1_r::En,
            DlmuSprotRgnaccena1R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccena1R {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccena1R {
        <crate::RegValueT<DlmuSprotRgnaccena1R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod dlmu_sprot_rgnaccena1_r {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Read access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccena1W_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccena1W_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccena1W = crate::RegValueT<DlmuSprotRgnaccena1W_SPEC>;

impl DlmuSprotRgnaccena1W {
    #[doc = "Write Access Enable for Master TAG ID n  n 0 31    EN. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        dlmu_sprot_rgnaccena1_w::En,
        DlmuSprotRgnaccena1W_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            dlmu_sprot_rgnaccena1_w::En,
            DlmuSprotRgnaccena1W_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccena1W {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccena1W {
        <crate::RegValueT<DlmuSprotRgnaccena1W_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod dlmu_sprot_rgnaccena1_w {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccena2R_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccena2R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccena2R = crate::RegValueT<DlmuSprotRgnaccena2R_SPEC>;

impl DlmuSprotRgnaccena2R {
    #[doc = "Read Access Enable for Master TAG ID n  n 0 31    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        dlmu_sprot_rgnaccena2_r::En,
        DlmuSprotRgnaccena2R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            dlmu_sprot_rgnaccena2_r::En,
            DlmuSprotRgnaccena2R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccena2R {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccena2R {
        <crate::RegValueT<DlmuSprotRgnaccena2R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod dlmu_sprot_rgnaccena2_r {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Read access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccena2W_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccena2W_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccena2W = crate::RegValueT<DlmuSprotRgnaccena2W_SPEC>;

impl DlmuSprotRgnaccena2W {
    #[doc = "Write Access Enable for Master TAG ID n  n 0 31    EN. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        dlmu_sprot_rgnaccena2_w::En,
        DlmuSprotRgnaccena2W_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            dlmu_sprot_rgnaccena2_w::En,
            DlmuSprotRgnaccena2W_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccena2W {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccena2W {
        <crate::RegValueT<DlmuSprotRgnaccena2W_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod dlmu_sprot_rgnaccena2_w {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccena3R_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccena3R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccena3R = crate::RegValueT<DlmuSprotRgnaccena3R_SPEC>;

impl DlmuSprotRgnaccena3R {
    #[doc = "Read Access Enable for Master TAG ID n  n 0 31    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        dlmu_sprot_rgnaccena3_r::En,
        DlmuSprotRgnaccena3R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            dlmu_sprot_rgnaccena3_r::En,
            DlmuSprotRgnaccena3R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccena3R {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccena3R {
        <crate::RegValueT<DlmuSprotRgnaccena3R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod dlmu_sprot_rgnaccena3_r {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Read access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccena3W_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccena3W_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccena3W = crate::RegValueT<DlmuSprotRgnaccena3W_SPEC>;

impl DlmuSprotRgnaccena3W {
    #[doc = "Write Access Enable for Master TAG ID n  n 0 31    EN. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        dlmu_sprot_rgnaccena3_w::En,
        DlmuSprotRgnaccena3W_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            dlmu_sprot_rgnaccena3_w::En,
            DlmuSprotRgnaccena3W_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccena3W {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccena3W {
        <crate::RegValueT<DlmuSprotRgnaccena3W_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod dlmu_sprot_rgnaccena3_w {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccena4R_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccena4R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccena4R = crate::RegValueT<DlmuSprotRgnaccena4R_SPEC>;

impl DlmuSprotRgnaccena4R {
    #[doc = "Read Access Enable for Master TAG ID n  n 0 31    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        dlmu_sprot_rgnaccena4_r::En,
        DlmuSprotRgnaccena4R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            dlmu_sprot_rgnaccena4_r::En,
            DlmuSprotRgnaccena4R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccena4R {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccena4R {
        <crate::RegValueT<DlmuSprotRgnaccena4R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod dlmu_sprot_rgnaccena4_r {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Read access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccena4W_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccena4W_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccena4W = crate::RegValueT<DlmuSprotRgnaccena4W_SPEC>;

impl DlmuSprotRgnaccena4W {
    #[doc = "Write Access Enable for Master TAG ID n  n 0 31    EN. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        dlmu_sprot_rgnaccena4_w::En,
        DlmuSprotRgnaccena4W_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            dlmu_sprot_rgnaccena4_w::En,
            DlmuSprotRgnaccena4W_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccena4W {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccena4W {
        <crate::RegValueT<DlmuSprotRgnaccena4W_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod dlmu_sprot_rgnaccena4_w {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccena5R_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccena5R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccena5R = crate::RegValueT<DlmuSprotRgnaccena5R_SPEC>;

impl DlmuSprotRgnaccena5R {
    #[doc = "Read Access Enable for Master TAG ID n  n 0 31    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        dlmu_sprot_rgnaccena5_r::En,
        DlmuSprotRgnaccena5R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            dlmu_sprot_rgnaccena5_r::En,
            DlmuSprotRgnaccena5R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccena5R {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccena5R {
        <crate::RegValueT<DlmuSprotRgnaccena5R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod dlmu_sprot_rgnaccena5_r {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Read access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccena5W_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccena5W_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccena5W = crate::RegValueT<DlmuSprotRgnaccena5W_SPEC>;

impl DlmuSprotRgnaccena5W {
    #[doc = "Write Access Enable for Master TAG ID n  n 0 31    EN. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        dlmu_sprot_rgnaccena5_w::En,
        DlmuSprotRgnaccena5W_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            dlmu_sprot_rgnaccena5_w::En,
            DlmuSprotRgnaccena5W_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccena5W {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccena5W {
        <crate::RegValueT<DlmuSprotRgnaccena5W_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod dlmu_sprot_rgnaccena5_w {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccena6R_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccena6R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccena6R = crate::RegValueT<DlmuSprotRgnaccena6R_SPEC>;

impl DlmuSprotRgnaccena6R {
    #[doc = "Read Access Enable for Master TAG ID n  n 0 31    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        dlmu_sprot_rgnaccena6_r::En,
        DlmuSprotRgnaccena6R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            dlmu_sprot_rgnaccena6_r::En,
            DlmuSprotRgnaccena6R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccena6R {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccena6R {
        <crate::RegValueT<DlmuSprotRgnaccena6R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod dlmu_sprot_rgnaccena6_r {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Read access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccena6W_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccena6W_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccena6W = crate::RegValueT<DlmuSprotRgnaccena6W_SPEC>;

impl DlmuSprotRgnaccena6W {
    #[doc = "Write Access Enable for Master TAG ID n  n 0 31    EN. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        dlmu_sprot_rgnaccena6_w::En,
        DlmuSprotRgnaccena6W_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            dlmu_sprot_rgnaccena6_w::En,
            DlmuSprotRgnaccena6W_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccena6W {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccena6W {
        <crate::RegValueT<DlmuSprotRgnaccena6W_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod dlmu_sprot_rgnaccena6_w {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccena7R_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccena7R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccena7R = crate::RegValueT<DlmuSprotRgnaccena7R_SPEC>;

impl DlmuSprotRgnaccena7R {
    #[doc = "Read Access Enable for Master TAG ID n  n 0 31    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        dlmu_sprot_rgnaccena7_r::En,
        DlmuSprotRgnaccena7R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            dlmu_sprot_rgnaccena7_r::En,
            DlmuSprotRgnaccena7R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccena7R {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccena7R {
        <crate::RegValueT<DlmuSprotRgnaccena7R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod dlmu_sprot_rgnaccena7_r {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Read access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccena7W_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccena7W_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccena7W = crate::RegValueT<DlmuSprotRgnaccena7W_SPEC>;

impl DlmuSprotRgnaccena7W {
    #[doc = "Write Access Enable for Master TAG ID n  n 0 31    EN. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        dlmu_sprot_rgnaccena7_w::En,
        DlmuSprotRgnaccena7W_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            dlmu_sprot_rgnaccena7_w::En,
            DlmuSprotRgnaccena7W_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccena7W {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccena7W {
        <crate::RegValueT<DlmuSprotRgnaccena7W_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod dlmu_sprot_rgnaccena7_w {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Write access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccenb0R_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccenb0R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccenb0R = crate::RegValueT<DlmuSprotRgnaccenb0R_SPEC>;

impl DlmuSprotRgnaccenb0R {
    #[doc = "Read Access Enable for Master TAG ID n  n 32 63    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        DlmuSprotRgnaccenb0R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            DlmuSprotRgnaccenb0R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccenb0R {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccenb0R {
        <crate::RegValueT<DlmuSprotRgnaccenb0R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccenb0W_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccenb0W_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccenb0W = crate::RegValueT<DlmuSprotRgnaccenb0W_SPEC>;

impl DlmuSprotRgnaccenb0W {
    #[doc = "Write Access Enable for Master TAG ID n  n 32 63    EN. This bit enables write access to the module kernel addresses for        transactions with the Master TAGID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        DlmuSprotRgnaccenb0W_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            DlmuSprotRgnaccenb0W_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccenb0W {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccenb0W {
        <crate::RegValueT<DlmuSprotRgnaccenb0W_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccenb1R_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccenb1R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccenb1R = crate::RegValueT<DlmuSprotRgnaccenb1R_SPEC>;

impl DlmuSprotRgnaccenb1R {
    #[doc = "Read Access Enable for Master TAG ID n  n 32 63    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        DlmuSprotRgnaccenb1R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            DlmuSprotRgnaccenb1R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccenb1R {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccenb1R {
        <crate::RegValueT<DlmuSprotRgnaccenb1R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccenb1W_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccenb1W_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccenb1W = crate::RegValueT<DlmuSprotRgnaccenb1W_SPEC>;

impl DlmuSprotRgnaccenb1W {
    #[doc = "Write Access Enable for Master TAG ID n  n 32 63    EN. This bit enables write access to the module kernel addresses for        transactions with the Master TAGID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        DlmuSprotRgnaccenb1W_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            DlmuSprotRgnaccenb1W_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccenb1W {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccenb1W {
        <crate::RegValueT<DlmuSprotRgnaccenb1W_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccenb2R_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccenb2R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccenb2R = crate::RegValueT<DlmuSprotRgnaccenb2R_SPEC>;

impl DlmuSprotRgnaccenb2R {
    #[doc = "Read Access Enable for Master TAG ID n  n 32 63    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        DlmuSprotRgnaccenb2R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            DlmuSprotRgnaccenb2R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccenb2R {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccenb2R {
        <crate::RegValueT<DlmuSprotRgnaccenb2R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccenb2W_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccenb2W_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccenb2W = crate::RegValueT<DlmuSprotRgnaccenb2W_SPEC>;

impl DlmuSprotRgnaccenb2W {
    #[doc = "Write Access Enable for Master TAG ID n  n 32 63    EN. This bit enables write access to the module kernel addresses for        transactions with the Master TAGID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        DlmuSprotRgnaccenb2W_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            DlmuSprotRgnaccenb2W_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccenb2W {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccenb2W {
        <crate::RegValueT<DlmuSprotRgnaccenb2W_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccenb3R_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccenb3R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccenb3R = crate::RegValueT<DlmuSprotRgnaccenb3R_SPEC>;

impl DlmuSprotRgnaccenb3R {
    #[doc = "Read Access Enable for Master TAG ID n  n 32 63    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        DlmuSprotRgnaccenb3R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            DlmuSprotRgnaccenb3R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccenb3R {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccenb3R {
        <crate::RegValueT<DlmuSprotRgnaccenb3R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccenb3W_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccenb3W_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccenb3W = crate::RegValueT<DlmuSprotRgnaccenb3W_SPEC>;

impl DlmuSprotRgnaccenb3W {
    #[doc = "Write Access Enable for Master TAG ID n  n 32 63    EN. This bit enables write access to the module kernel addresses for        transactions with the Master TAGID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        DlmuSprotRgnaccenb3W_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            DlmuSprotRgnaccenb3W_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccenb3W {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccenb3W {
        <crate::RegValueT<DlmuSprotRgnaccenb3W_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccenb4R_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccenb4R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccenb4R = crate::RegValueT<DlmuSprotRgnaccenb4R_SPEC>;

impl DlmuSprotRgnaccenb4R {
    #[doc = "Read Access Enable for Master TAG ID n  n 32 63    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        DlmuSprotRgnaccenb4R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            DlmuSprotRgnaccenb4R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccenb4R {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccenb4R {
        <crate::RegValueT<DlmuSprotRgnaccenb4R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccenb4W_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccenb4W_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccenb4W = crate::RegValueT<DlmuSprotRgnaccenb4W_SPEC>;

impl DlmuSprotRgnaccenb4W {
    #[doc = "Write Access Enable for Master TAG ID n  n 32 63    EN. This bit enables write access to the module kernel addresses for        transactions with the Master TAGID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        DlmuSprotRgnaccenb4W_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            DlmuSprotRgnaccenb4W_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccenb4W {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccenb4W {
        <crate::RegValueT<DlmuSprotRgnaccenb4W_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccenb5R_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccenb5R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccenb5R = crate::RegValueT<DlmuSprotRgnaccenb5R_SPEC>;

impl DlmuSprotRgnaccenb5R {
    #[doc = "Read Access Enable for Master TAG ID n  n 32 63    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        DlmuSprotRgnaccenb5R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            DlmuSprotRgnaccenb5R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccenb5R {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccenb5R {
        <crate::RegValueT<DlmuSprotRgnaccenb5R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccenb5W_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccenb5W_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccenb5W = crate::RegValueT<DlmuSprotRgnaccenb5W_SPEC>;

impl DlmuSprotRgnaccenb5W {
    #[doc = "Write Access Enable for Master TAG ID n  n 32 63    EN. This bit enables write access to the module kernel addresses for        transactions with the Master TAGID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        DlmuSprotRgnaccenb5W_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            DlmuSprotRgnaccenb5W_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccenb5W {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccenb5W {
        <crate::RegValueT<DlmuSprotRgnaccenb5W_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccenb6R_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccenb6R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccenb6R = crate::RegValueT<DlmuSprotRgnaccenb6R_SPEC>;

impl DlmuSprotRgnaccenb6R {
    #[doc = "Read Access Enable for Master TAG ID n  n 32 63    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        DlmuSprotRgnaccenb6R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            DlmuSprotRgnaccenb6R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccenb6R {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccenb6R {
        <crate::RegValueT<DlmuSprotRgnaccenb6R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccenb6W_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccenb6W_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccenb6W = crate::RegValueT<DlmuSprotRgnaccenb6W_SPEC>;

impl DlmuSprotRgnaccenb6W {
    #[doc = "Write Access Enable for Master TAG ID n  n 32 63    EN. This bit enables write access to the module kernel addresses for        transactions with the Master TAGID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        DlmuSprotRgnaccenb6W_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            DlmuSprotRgnaccenb6W_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccenb6W {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccenb6W {
        <crate::RegValueT<DlmuSprotRgnaccenb6W_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccenb7R_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccenb7R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region DLMU Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccenb7R = crate::RegValueT<DlmuSprotRgnaccenb7R_SPEC>;

impl DlmuSprotRgnaccenb7R {
    #[doc = "Read Access Enable for Master TAG ID n  n 32 63    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        DlmuSprotRgnaccenb7R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            DlmuSprotRgnaccenb7R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccenb7R {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccenb7R {
        <crate::RegValueT<DlmuSprotRgnaccenb7R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnaccenb7W_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnaccenb7W_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection Region DLMU Write Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type DlmuSprotRgnaccenb7W = crate::RegValueT<DlmuSprotRgnaccenb7W_SPEC>;

impl DlmuSprotRgnaccenb7W {
    #[doc = "Write Access Enable for Master TAG ID n  n 32 63    EN. This bit enables write access to the module kernel addresses for        transactions with the Master TAGID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        DlmuSprotRgnaccenb7W_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            DlmuSprotRgnaccenb7W_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnaccenb7W {
    #[inline(always)]
    fn default() -> DlmuSprotRgnaccenb7W {
        <crate::RegValueT<DlmuSprotRgnaccenb7W_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnla0_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnla0_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection DLMU Region Lower Address Register 7\n resetvalue={Application Reset:0x0}"]
pub type DlmuSprotRgnla0 = crate::RegValueT<DlmuSprotRgnla0_SPEC>;

impl DlmuSprotRgnla0 {
    #[doc = "Region Lower Address   ADDR. Bits 31 to 5 of the address which is the lower bound of the defined memory region"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7ffffff,
        1,
        0,
        u32,
        DlmuSprotRgnla0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            DlmuSprotRgnla0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnla0 {
    #[inline(always)]
    fn default() -> DlmuSprotRgnla0 {
        <crate::RegValueT<DlmuSprotRgnla0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnla1_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnla1_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection DLMU Region Lower Address Register 7\n resetvalue={Application Reset:0x0}"]
pub type DlmuSprotRgnla1 = crate::RegValueT<DlmuSprotRgnla1_SPEC>;

impl DlmuSprotRgnla1 {
    #[doc = "Region Lower Address   ADDR. Bits 31 to 5 of the address which is the lower bound of the defined memory region"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7ffffff,
        1,
        0,
        u32,
        DlmuSprotRgnla1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            DlmuSprotRgnla1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnla1 {
    #[inline(always)]
    fn default() -> DlmuSprotRgnla1 {
        <crate::RegValueT<DlmuSprotRgnla1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnla2_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnla2_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection DLMU Region Lower Address Register 7\n resetvalue={Application Reset:0x0}"]
pub type DlmuSprotRgnla2 = crate::RegValueT<DlmuSprotRgnla2_SPEC>;

impl DlmuSprotRgnla2 {
    #[doc = "Region Lower Address   ADDR. Bits 31 to 5 of the address which is the lower bound of the defined memory region"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7ffffff,
        1,
        0,
        u32,
        DlmuSprotRgnla2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            DlmuSprotRgnla2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnla2 {
    #[inline(always)]
    fn default() -> DlmuSprotRgnla2 {
        <crate::RegValueT<DlmuSprotRgnla2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnla3_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnla3_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection DLMU Region Lower Address Register 7\n resetvalue={Application Reset:0x0}"]
pub type DlmuSprotRgnla3 = crate::RegValueT<DlmuSprotRgnla3_SPEC>;

impl DlmuSprotRgnla3 {
    #[doc = "Region Lower Address   ADDR. Bits 31 to 5 of the address which is the lower bound of the defined memory region"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7ffffff,
        1,
        0,
        u32,
        DlmuSprotRgnla3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            DlmuSprotRgnla3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnla3 {
    #[inline(always)]
    fn default() -> DlmuSprotRgnla3 {
        <crate::RegValueT<DlmuSprotRgnla3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnla4_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnla4_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection DLMU Region Lower Address Register 7\n resetvalue={Application Reset:0x0}"]
pub type DlmuSprotRgnla4 = crate::RegValueT<DlmuSprotRgnla4_SPEC>;

impl DlmuSprotRgnla4 {
    #[doc = "Region Lower Address   ADDR. Bits 31 to 5 of the address which is the lower bound of the defined memory region"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7ffffff,
        1,
        0,
        u32,
        DlmuSprotRgnla4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            DlmuSprotRgnla4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnla4 {
    #[inline(always)]
    fn default() -> DlmuSprotRgnla4 {
        <crate::RegValueT<DlmuSprotRgnla4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnla5_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnla5_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection DLMU Region Lower Address Register 7\n resetvalue={Application Reset:0x0}"]
pub type DlmuSprotRgnla5 = crate::RegValueT<DlmuSprotRgnla5_SPEC>;

impl DlmuSprotRgnla5 {
    #[doc = "Region Lower Address   ADDR. Bits 31 to 5 of the address which is the lower bound of the defined memory region"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7ffffff,
        1,
        0,
        u32,
        DlmuSprotRgnla5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            DlmuSprotRgnla5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnla5 {
    #[inline(always)]
    fn default() -> DlmuSprotRgnla5 {
        <crate::RegValueT<DlmuSprotRgnla5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnla6_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnla6_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection DLMU Region Lower Address Register 7\n resetvalue={Application Reset:0x0}"]
pub type DlmuSprotRgnla6 = crate::RegValueT<DlmuSprotRgnla6_SPEC>;

impl DlmuSprotRgnla6 {
    #[doc = "Region Lower Address   ADDR. Bits 31 to 5 of the address which is the lower bound of the defined memory region"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7ffffff,
        1,
        0,
        u32,
        DlmuSprotRgnla6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            DlmuSprotRgnla6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnla6 {
    #[inline(always)]
    fn default() -> DlmuSprotRgnla6 {
        <crate::RegValueT<DlmuSprotRgnla6_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnla7_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnla7_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection DLMU Region Lower Address Register 7\n resetvalue={Application Reset:0x0}"]
pub type DlmuSprotRgnla7 = crate::RegValueT<DlmuSprotRgnla7_SPEC>;

impl DlmuSprotRgnla7 {
    #[doc = "Region Lower Address   ADDR. Bits 31 to 5 of the address which is the lower bound of the defined memory region"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7ffffff,
        1,
        0,
        u32,
        DlmuSprotRgnla7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            DlmuSprotRgnla7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnla7 {
    #[inline(always)]
    fn default() -> DlmuSprotRgnla7 {
        <crate::RegValueT<DlmuSprotRgnla7_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnua0_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnua0_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety protection DLMU Region Upper Address Register 7\n resetvalue={Application Reset:0x0FFFFFFE0}"]
pub type DlmuSprotRgnua0 = crate::RegValueT<DlmuSprotRgnua0_SPEC>;

impl DlmuSprotRgnua0 {
    #[doc = "Region Upper Address   ADDR. Bits 31 to 5 of the address which is the upper bound of the defined memory region"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7ffffff,
        1,
        0,
        u32,
        DlmuSprotRgnua0_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            DlmuSprotRgnua0_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnua0 {
    #[inline(always)]
    fn default() -> DlmuSprotRgnua0 {
        <crate::RegValueT<DlmuSprotRgnua0_SPEC> as RegisterValue<_>>::new(4294967264)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnua1_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnua1_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety protection DLMU Region Upper Address Register 7\n resetvalue={Application Reset:0x0FFFFFFE0}"]
pub type DlmuSprotRgnua1 = crate::RegValueT<DlmuSprotRgnua1_SPEC>;

impl DlmuSprotRgnua1 {
    #[doc = "Region Upper Address   ADDR. Bits 31 to 5 of the address which is the upper bound of the defined memory region"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7ffffff,
        1,
        0,
        u32,
        DlmuSprotRgnua1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            DlmuSprotRgnua1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnua1 {
    #[inline(always)]
    fn default() -> DlmuSprotRgnua1 {
        <crate::RegValueT<DlmuSprotRgnua1_SPEC> as RegisterValue<_>>::new(4294967264)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnua2_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnua2_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety protection DLMU Region Upper Address Register 7\n resetvalue={Application Reset:0x0FFFFFFE0}"]
pub type DlmuSprotRgnua2 = crate::RegValueT<DlmuSprotRgnua2_SPEC>;

impl DlmuSprotRgnua2 {
    #[doc = "Region Upper Address   ADDR. Bits 31 to 5 of the address which is the upper bound of the defined memory region"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7ffffff,
        1,
        0,
        u32,
        DlmuSprotRgnua2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            DlmuSprotRgnua2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnua2 {
    #[inline(always)]
    fn default() -> DlmuSprotRgnua2 {
        <crate::RegValueT<DlmuSprotRgnua2_SPEC> as RegisterValue<_>>::new(4294967264)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnua3_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnua3_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety protection DLMU Region Upper Address Register 7\n resetvalue={Application Reset:0x0FFFFFFE0}"]
pub type DlmuSprotRgnua3 = crate::RegValueT<DlmuSprotRgnua3_SPEC>;

impl DlmuSprotRgnua3 {
    #[doc = "Region Upper Address   ADDR. Bits 31 to 5 of the address which is the upper bound of the defined memory region"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7ffffff,
        1,
        0,
        u32,
        DlmuSprotRgnua3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            DlmuSprotRgnua3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnua3 {
    #[inline(always)]
    fn default() -> DlmuSprotRgnua3 {
        <crate::RegValueT<DlmuSprotRgnua3_SPEC> as RegisterValue<_>>::new(4294967264)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnua4_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnua4_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety protection DLMU Region Upper Address Register 7\n resetvalue={Application Reset:0x0FFFFFFE0}"]
pub type DlmuSprotRgnua4 = crate::RegValueT<DlmuSprotRgnua4_SPEC>;

impl DlmuSprotRgnua4 {
    #[doc = "Region Upper Address   ADDR. Bits 31 to 5 of the address which is the upper bound of the defined memory region"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7ffffff,
        1,
        0,
        u32,
        DlmuSprotRgnua4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            DlmuSprotRgnua4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnua4 {
    #[inline(always)]
    fn default() -> DlmuSprotRgnua4 {
        <crate::RegValueT<DlmuSprotRgnua4_SPEC> as RegisterValue<_>>::new(4294967264)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnua5_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnua5_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety protection DLMU Region Upper Address Register 7\n resetvalue={Application Reset:0x0FFFFFFE0}"]
pub type DlmuSprotRgnua5 = crate::RegValueT<DlmuSprotRgnua5_SPEC>;

impl DlmuSprotRgnua5 {
    #[doc = "Region Upper Address   ADDR. Bits 31 to 5 of the address which is the upper bound of the defined memory region"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7ffffff,
        1,
        0,
        u32,
        DlmuSprotRgnua5_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            DlmuSprotRgnua5_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnua5 {
    #[inline(always)]
    fn default() -> DlmuSprotRgnua5 {
        <crate::RegValueT<DlmuSprotRgnua5_SPEC> as RegisterValue<_>>::new(4294967264)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnua6_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnua6_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety protection DLMU Region Upper Address Register 7\n resetvalue={Application Reset:0x0FFFFFFE0}"]
pub type DlmuSprotRgnua6 = crate::RegValueT<DlmuSprotRgnua6_SPEC>;

impl DlmuSprotRgnua6 {
    #[doc = "Region Upper Address   ADDR. Bits 31 to 5 of the address which is the upper bound of the defined memory region"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7ffffff,
        1,
        0,
        u32,
        DlmuSprotRgnua6_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            DlmuSprotRgnua6_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnua6 {
    #[inline(always)]
    fn default() -> DlmuSprotRgnua6 {
        <crate::RegValueT<DlmuSprotRgnua6_SPEC> as RegisterValue<_>>::new(4294967264)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DlmuSprotRgnua7_SPEC;
impl crate::sealed::RegSpec for DlmuSprotRgnua7_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety protection DLMU Region Upper Address Register 7\n resetvalue={Application Reset:0x0FFFFFFE0}"]
pub type DlmuSprotRgnua7 = crate::RegValueT<DlmuSprotRgnua7_SPEC>;

impl DlmuSprotRgnua7 {
    #[doc = "Region Upper Address   ADDR. Bits 31 to 5 of the address which is the upper bound of the defined memory region"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x7ffffff,
        1,
        0,
        u32,
        DlmuSprotRgnua7_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            DlmuSprotRgnua7_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for DlmuSprotRgnua7 {
    #[inline(always)]
    fn default() -> DlmuSprotRgnua7 {
        <crate::RegValueT<DlmuSprotRgnua7_SPEC> as RegisterValue<_>>::new(4294967264)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dms_SPEC;
impl crate::sealed::RegSpec for Dms_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Debug Monitor Start Address\n resetvalue={Application Reset:0x0}"]
pub type Dms = crate::RegValueT<Dms_SPEC>;

impl Dms {
    #[doc = "Debug Monitor Start Address   DMSValue. The address at which monitor code execution begins when a breakpoint trap is taken."]
    #[inline(always)]
    pub fn dmsvalue(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, Dms_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32, Dms_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dms {
    #[inline(always)]
    fn default() -> Dms {
        <crate::RegValueT<Dms_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpre0_SPEC;
impl crate::sealed::RegSpec for Dpre0_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Dpre0 = crate::RegValueT<Dpre0_SPEC>;

impl Dpre0 {
    #[doc = "Read Enable Range Select   RE n"]
    #[inline(always)]
    pub fn re_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, dpre_0::ReN, Dpre0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpre_0::ReN, Dpre0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dpre0 {
    #[inline(always)]
    fn default() -> Dpre0 {
        <crate::RegValueT<Dpre0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpre_0 {
    pub struct ReN_SPEC;
    pub type ReN = crate::EnumBitfieldStruct<u8, ReN_SPEC>;
    impl ReN {
        #[doc = "0 Data Protection Range n not enabled for data read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Data Protection Range n enabled for data read"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpre1_SPEC;
impl crate::sealed::RegSpec for Dpre1_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Dpre1 = crate::RegValueT<Dpre1_SPEC>;

impl Dpre1 {
    #[doc = "Read Enable Range Select   RE n"]
    #[inline(always)]
    pub fn re_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, dpre_1::ReN, Dpre1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpre_1::ReN, Dpre1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dpre1 {
    #[inline(always)]
    fn default() -> Dpre1 {
        <crate::RegValueT<Dpre1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpre_1 {
    pub struct ReN_SPEC;
    pub type ReN = crate::EnumBitfieldStruct<u8, ReN_SPEC>;
    impl ReN {
        #[doc = "0 Data Protection Range n not enabled for data read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Data Protection Range n enabled for data read"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpre2_SPEC;
impl crate::sealed::RegSpec for Dpre2_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Dpre2 = crate::RegValueT<Dpre2_SPEC>;

impl Dpre2 {
    #[doc = "Read Enable Range Select   RE n"]
    #[inline(always)]
    pub fn re_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, dpre_2::ReN, Dpre2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpre_2::ReN, Dpre2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dpre2 {
    #[inline(always)]
    fn default() -> Dpre2 {
        <crate::RegValueT<Dpre2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpre_2 {
    pub struct ReN_SPEC;
    pub type ReN = crate::EnumBitfieldStruct<u8, ReN_SPEC>;
    impl ReN {
        #[doc = "0 Data Protection Range n not enabled for data read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Data Protection Range n enabled for data read"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpre3_SPEC;
impl crate::sealed::RegSpec for Dpre3_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Dpre3 = crate::RegValueT<Dpre3_SPEC>;

impl Dpre3 {
    #[doc = "Read Enable Range Select   RE n"]
    #[inline(always)]
    pub fn re_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, dpre_3::ReN, Dpre3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpre_3::ReN, Dpre3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dpre3 {
    #[inline(always)]
    fn default() -> Dpre3 {
        <crate::RegValueT<Dpre3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpre_3 {
    pub struct ReN_SPEC;
    pub type ReN = crate::EnumBitfieldStruct<u8, ReN_SPEC>;
    impl ReN {
        #[doc = "0 Data Protection Range n not enabled for data read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Data Protection Range n enabled for data read"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpre4_SPEC;
impl crate::sealed::RegSpec for Dpre4_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Read Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
pub type Dpre4 = crate::RegValueT<Dpre4_SPEC>;

impl Dpre4 {
    #[doc = "Read Enable Range Select   RE n"]
    #[inline(always)]
    pub fn re_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, dpre_4::ReN, Dpre4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpre_4::ReN, Dpre4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dpre4 {
    #[inline(always)]
    fn default() -> Dpre4 {
        <crate::RegValueT<Dpre4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpre_4 {
    pub struct ReN_SPEC;
    pub type ReN = crate::EnumBitfieldStruct<u8, ReN_SPEC>;
    impl ReN {
        #[doc = "0 Data Protection Range n not enabled for data read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Data Protection Range n enabled for data read"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpre5_SPEC;
impl crate::sealed::RegSpec for Dpre5_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Read Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
pub type Dpre5 = crate::RegValueT<Dpre5_SPEC>;

impl Dpre5 {
    #[doc = "Read Enable Range Select   RE n"]
    #[inline(always)]
    pub fn re_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, dpre_5::ReN, Dpre5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpre_5::ReN, Dpre5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dpre5 {
    #[inline(always)]
    fn default() -> Dpre5 {
        <crate::RegValueT<Dpre5_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpre_5 {
    pub struct ReN_SPEC;
    pub type ReN = crate::EnumBitfieldStruct<u8, ReN_SPEC>;
    impl ReN {
        #[doc = "0 Data Protection Range n not enabled for data read"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Data Protection Range n enabled for data read"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpwe0_SPEC;
impl crate::sealed::RegSpec for Dpwe0_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Dpwe0 = crate::RegValueT<Dpwe0_SPEC>;

impl Dpwe0 {
    #[doc = "Write Enable Range Select   WE n"]
    #[inline(always)]
    pub fn we_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, dpwe_0::WeN, Dpwe0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpwe_0::WeN, Dpwe0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dpwe0 {
    #[inline(always)]
    fn default() -> Dpwe0 {
        <crate::RegValueT<Dpwe0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpwe_0 {
    pub struct WeN_SPEC;
    pub type WeN = crate::EnumBitfieldStruct<u8, WeN_SPEC>;
    impl WeN {
        #[doc = "0 Data Protection Range n not enabled for data write"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Data Protection Range n enabled for data write"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpwe1_SPEC;
impl crate::sealed::RegSpec for Dpwe1_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Dpwe1 = crate::RegValueT<Dpwe1_SPEC>;

impl Dpwe1 {
    #[doc = "Write Enable Range Select   WE n"]
    #[inline(always)]
    pub fn we_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, dpwe_1::WeN, Dpwe1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpwe_1::WeN, Dpwe1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dpwe1 {
    #[inline(always)]
    fn default() -> Dpwe1 {
        <crate::RegValueT<Dpwe1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpwe_1 {
    pub struct WeN_SPEC;
    pub type WeN = crate::EnumBitfieldStruct<u8, WeN_SPEC>;
    impl WeN {
        #[doc = "0 Data Protection Range n not enabled for data write"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Data Protection Range n enabled for data write"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpwe2_SPEC;
impl crate::sealed::RegSpec for Dpwe2_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Dpwe2 = crate::RegValueT<Dpwe2_SPEC>;

impl Dpwe2 {
    #[doc = "Write Enable Range Select   WE n"]
    #[inline(always)]
    pub fn we_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, dpwe_2::WeN, Dpwe2_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpwe_2::WeN, Dpwe2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dpwe2 {
    #[inline(always)]
    fn default() -> Dpwe2 {
        <crate::RegValueT<Dpwe2_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpwe_2 {
    pub struct WeN_SPEC;
    pub type WeN = crate::EnumBitfieldStruct<u8, WeN_SPEC>;
    impl WeN {
        #[doc = "0 Data Protection Range n not enabled for data write"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Data Protection Range n enabled for data write"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpwe3_SPEC;
impl crate::sealed::RegSpec for Dpwe3_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Dpwe3 = crate::RegValueT<Dpwe3_SPEC>;

impl Dpwe3 {
    #[doc = "Write Enable Range Select   WE n"]
    #[inline(always)]
    pub fn we_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, dpwe_3::WeN, Dpwe3_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpwe_3::WeN, Dpwe3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dpwe3 {
    #[inline(always)]
    fn default() -> Dpwe3 {
        <crate::RegValueT<Dpwe3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpwe_3 {
    pub struct WeN_SPEC;
    pub type WeN = crate::EnumBitfieldStruct<u8, WeN_SPEC>;
    impl WeN {
        #[doc = "0 Data Protection Range n not enabled for data write"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Data Protection Range n enabled for data write"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpwe4_SPEC;
impl crate::sealed::RegSpec for Dpwe4_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Write Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
pub type Dpwe4 = crate::RegValueT<Dpwe4_SPEC>;

impl Dpwe4 {
    #[doc = "Write Enable Range Select   WE n"]
    #[inline(always)]
    pub fn we_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, dpwe_4::WeN, Dpwe4_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpwe_4::WeN, Dpwe4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dpwe4 {
    #[inline(always)]
    fn default() -> Dpwe4 {
        <crate::RegValueT<Dpwe4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpwe_4 {
    pub struct WeN_SPEC;
    pub type WeN = crate::EnumBitfieldStruct<u8, WeN_SPEC>;
    impl WeN {
        #[doc = "0 Data Protection Range n not enabled for data write"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Data Protection Range n enabled for data write"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpwe5_SPEC;
impl crate::sealed::RegSpec for Dpwe5_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Write Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
pub type Dpwe5 = crate::RegValueT<Dpwe5_SPEC>;

impl Dpwe5 {
    #[doc = "Write Enable Range Select   WE n"]
    #[inline(always)]
    pub fn we_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, dpwe_5::WeN, Dpwe5_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3ffff,1,0,dpwe_5::WeN, Dpwe5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dpwe5 {
    #[inline(always)]
    fn default() -> Dpwe5 {
        <crate::RegValueT<Dpwe5_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod dpwe_5 {
    pub struct WeN_SPEC;
    pub type WeN = crate::EnumBitfieldStruct<u8, WeN_SPEC>;
    impl WeN {
        #[doc = "0 Data Protection Range n not enabled for data write"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Data Protection Range n enabled for data write"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dstr_SPEC;
impl crate::sealed::RegSpec for Dstr_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Synchronous Trap Register\n resetvalue={Application Reset:0x0}"]
pub type Dstr = crate::RegValueT<Dstr_SPEC>;

impl Dstr {
    #[doc = "Scratch Range Error   SRE. A scratch Range Error occurs whenever an access to the data scratch is outside the range of the SRAM."]
    #[inline(always)]
    pub fn sre(self) -> crate::common::RegisterFieldBool<0, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Global Address Error   GAE. Load or store to local code scratch address outside of the lower 1MByte."]
    #[inline(always)]
    pub fn gae(self) -> crate::common::RegisterFieldBool<1, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Load Bus Error   LBE. A Load Bus Error will be set whenever the SRI flags an error due a load from external memory."]
    #[inline(always)]
    pub fn lbe(self) -> crate::common::RegisterFieldBool<2, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Local DLMU Range Error   DRE. A DLMU Range Error occurs whenever an access to the local DLMU region is outside the physically implemented memory."]
    #[inline(always)]
    pub fn dre(self) -> crate::common::RegisterFieldBool<3, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Cache Refill Error   CRE. A Cache Refill Error will be set whenever the SRI flags an error due a cache refill from external memory."]
    #[inline(always)]
    pub fn cre(self) -> crate::common::RegisterFieldBool<6, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "DTAG MSIST Error   DTME. Access to memory mapped DTAG range outside of physically implemented memory."]
    #[inline(always)]
    pub fn dtme(self) -> crate::common::RegisterFieldBool<14, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Load Overlay Error   LOE. Load to invalid overlay address."]
    #[inline(always)]
    pub fn loe(self) -> crate::common::RegisterFieldBool<15, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Segment Difference Error   SDE. Load or store access where base address is in different segment to access address."]
    #[inline(always)]
    pub fn sde(self) -> crate::common::RegisterFieldBool<16, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Segment Crossing Error   SCE. Load or store access across segment boundary."]
    #[inline(always)]
    pub fn sce(self) -> crate::common::RegisterFieldBool<17, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CSFR Access Error   CAC. Load or store to local CSFR space."]
    #[inline(always)]
    pub fn cac(self) -> crate::common::RegisterFieldBool<18, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Memory Protection Error   MPE. Data access violating memory protection."]
    #[inline(always)]
    pub fn mpe(self) -> crate::common::RegisterFieldBool<19, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Context Location Error   CLE. Context operation to invalid location."]
    #[inline(always)]
    pub fn cle(self) -> crate::common::RegisterFieldBool<20, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Alignment Error   ALN. Data access causing alignment error."]
    #[inline(always)]
    pub fn aln(self) -> crate::common::RegisterFieldBool<24, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Dstr {
    #[inline(always)]
    fn default() -> Dstr {
        <crate::RegValueT<Dstr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dy_SPEC;
impl crate::sealed::RegSpec for Dy_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data General Purpose Register 0\n resetvalue={Application Reset:0x0}"]
pub type Dy = crate::RegValueT<Dy_SPEC>;

impl Dy {
    #[doc = "Data Register   DATA. General purpose registers"]
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Dy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Dy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Dy {
    #[inline(always)]
    fn default() -> Dy {
        <crate::RegValueT<Dy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Exevt_SPEC;
impl crate::sealed::RegSpec for Exevt_SPEC {
    type DataType = u32;
}
#[doc = "CPUx External Event Register\n resetvalue={Debug Reset:0x0}"]
pub type Exevt = crate::RegValueT<Exevt_SPEC>;

impl Exevt {
    #[doc = "Event Associated   EVTA. Specifies the Debug Action associated with the Debug Event"]
    #[inline(always)]
    pub fn evta(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, exevt::Evta, Exevt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,exevt::Evta, Exevt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Break Before Make  BBM  or Break After Make  BAM  Selection   BBM"]
    #[inline(always)]
    pub fn bbm(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, exevt::Bbm, Exevt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,exevt::Bbm, Exevt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Breakout Disable   BOD"]
    #[inline(always)]
    pub fn bod(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, exevt::Bod, Exevt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,exevt::Bod, Exevt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CDC Suspend Out Signal State   SUSP. Value to be assigned to the CDC suspend out signal when the Debug Event is raised."]
    #[inline(always)]
    pub fn susp(self) -> crate::common::RegisterFieldBool<5, 1, 0, Exevt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Exevt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Counter   CNT. When this event occurs adjust the control of the performance counters in task mode as follows"]
    #[inline(always)]
    pub fn cnt(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, exevt::Cnt, Exevt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,exevt::Cnt, Exevt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Exevt {
    #[inline(always)]
    fn default() -> Exevt {
        <crate::RegValueT<Exevt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod exevt {
    pub struct Evta_SPEC;
    pub type Evta = crate::EnumBitfieldStruct<u8, Evta_SPEC>;
    impl Evta {
        #[doc = "000 BOD 0  Disabled. BOD 1  Disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 BOD 0  Pulse BRKOUT Signal. BOD 1  None."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 BOD 0  Halt and pulse BRKOUT Signal. BOD 1  Halt."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 BOD 0  Breakpoint trap and pulse. BRKOUT Signal. BOD 1  Breakpoint trap."]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 BOD 0  Breakpoint interrupt 0 and pulse BRKOUT Signal. BOD 1  Breakpoint interrupt 0."]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 BOD 0  If implemented  breakpoint interrupt 1 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 1. If not implemented  None."]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 BOD 0  If implemented  breakpoint interrupt 2 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 2. If not implemented  None."]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "111 BOD 0  If implemented  breakpoint interrupt 3 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 3. If not implemented  None."]
        pub const CONST_77: Self = Self::new(7);
    }
    pub struct Bbm_SPEC;
    pub type Bbm = crate::EnumBitfieldStruct<u8, Bbm_SPEC>;
    impl Bbm {
        #[doc = "0 Break after make  BAM ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Break before make  BBM ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Bod_SPEC;
    pub type Bod = crate::EnumBitfieldStruct<u8, Bod_SPEC>;
    impl Bod {
        #[doc = "0 BRKOUT signal asserted according to the Debug Action specified in the EVTA field."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 BRKOUT signal not asserted. This takes priority over any assertion generated by the EVTA field."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cnt_SPEC;
    pub type Cnt = crate::EnumBitfieldStruct<u8, Cnt_SPEC>;
    impl Cnt {
        #[doc = "00 No change."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Start the performance counters."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Stop the performance counters."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Toggle the performance counter control  i.e. start it if it is currently stopped  stop it if it is currently running ."]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcx_SPEC;
impl crate::sealed::RegSpec for Fcx_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Free CSA List Head Pointer\n resetvalue={Application Reset:0x0}"]
pub type Fcx = crate::RegValueT<Fcx_SPEC>;

impl Fcx {
    #[doc = "FCX Offset Address Field   FCXO. The FCXO and FCXS fields together form the FCX pointer  which points to the next available CSA."]
    #[inline(always)]
    pub fn fcxo(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Fcx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Fcx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "FCX Segment Address Field   FCXS. Used in conjunction with the FCXO field."]
    #[inline(always)]
    pub fn fcxs(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Fcx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Fcx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Fcx {
    #[inline(always)]
    fn default() -> Fcx {
        <crate::RegValueT<Fcx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flashcon0_SPEC;
impl crate::sealed::RegSpec for Flashcon0_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Flash Configuration Register 0\n resetvalue={Application Reset:0x3F3F3F3F,CFS Value:0x22212120}"]
pub type Flashcon0 = crate::RegValueT<Flashcon0_SPEC>;

impl Flashcon0 {
    #[doc = "Flash Prefetch Buffer 1 Configuration. FPB is assigned to on  xa0 chip  xa0 bus master with master tag id equal to TAG1."]
    #[inline(always)]
    pub fn tag1(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Flashcon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Flashcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Flash Prefetch Buffer 2 Configuration. FPB is assigned to on  xa0 chip  xa0 bus master with master tag id equal to TAG2."]
    #[inline(always)]
    pub fn tag2(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, Flashcon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3f,1,0,u8, Flashcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Flash Prefetch Buffer 3 Configuration. FPB is assigned to on chip bus master with master tag id equal to TAG3."]
    #[inline(always)]
    pub fn tag3(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Flashcon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Flashcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Flash Prefetch Buffer 4 Configuration. FPB is assigned to on chip bus master with master tag id equal to TAG4."]
    #[inline(always)]
    pub fn tag4(
        self,
    ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, Flashcon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3f,1,0,u8, Flashcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Flashcon0 {
    #[inline(always)]
    fn default() -> Flashcon0 {
        <crate::RegValueT<Flashcon0_SPEC> as RegisterValue<_>>::new(572596512)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flashcon1_SPEC;
impl crate::sealed::RegSpec for Flashcon1_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Flash Configuration Register 1\n resetvalue={Application Reset:0x2020000}"]
pub type Flashcon1 = crate::RegValueT<Flashcon1_SPEC>;

impl Flashcon1 {
    #[doc = "Stall Bus Request. This field must not be changed while any bank is busy. The results are        unpredictable. It is strongly recommended to configure this field once        and avoid changing it during operation. Reading a flash bank  erase counter  or status register  in Sleep Mode        must result in a bus error independent of this field  although it        reports   8220 busy  8221  ."]
    #[inline(always)]
    pub fn stall(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        flashcon1::Stall,
        Flashcon1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            flashcon1::Stall,
            Flashcon1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Mask PFLASH Uncorrectable ECC Bit Error. No value other than 01 B or 10 B shall be programmed in the register bitfields. The system will behave as specified for 10 B if 00 B or 11 B is programmed."]
    #[inline(always)]
    pub fn maskuecc(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        flashcon1::Maskuecc,
        Flashcon1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            flashcon1::Maskuecc,
            Flashcon1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Mask PFLASH ECC Error and send Uncorrected data. No value other than 01 B or 10 B shall be programmed in the register bitfields. The system will behave as specified for 10 B if 00 B or 11 B is programmed."]
    #[inline(always)]
    pub fn tdisudata(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        flashcon1::Tdisudata,
        Flashcon1_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            flashcon1::Tdisudata,
            Flashcon1_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Flashcon1 {
    #[inline(always)]
    fn default() -> Flashcon1 {
        <crate::RegValueT<Flashcon1_SPEC> as RegisterValue<_>>::new(33685504)
    }
}
pub mod flashcon1 {
    pub struct Stall_SPEC;
    pub type Stall = crate::EnumBitfieldStruct<u8, Stall_SPEC>;
    impl Stall {
        #[doc = "0 B Error Reading local PFlash bank  erase counter  or status registers  when DMU STATUS.PxBUSY is set  generates a bus error."]
        pub const ERROR_0: Self = Self::new(0);
        #[doc = "1 B Stall Reading local PFlash bank  erase counter  or status registers when DMU HF STATUS.PxBUSY is set delays the response by inserting additional wait cycles until PxBUSY is cleared."]
        pub const STALL_1: Self = Self::new(1);
    }
    pub struct Maskuecc_SPEC;
    pub type Maskuecc = crate::EnumBitfieldStruct<u8, Maskuecc_SPEC>;
    impl Maskuecc {
        #[doc = "01 B If a local PFLASH uncorrectable ECC error occurs  then the error is globally disabled for any requesting master reading the local PFLASH ."]
        pub const CONST_011: Self = Self::new(1);
        #[doc = "10 B If a PFLASH uncorrectable ECC error occurs then an uncorrectable ECC error is reported to the CPU."]
        pub const CONST_102: Self = Self::new(2);
    }
    pub struct Tdisudata_SPEC;
    pub type Tdisudata = crate::EnumBitfieldStruct<u8, Tdisudata_SPEC>;
    impl Tdisudata {
        #[doc = "01 B If a PFLASH ECC error occurs then no error is reported to the CPU and uncorrected data is sent to the CPU when in CFTM. If not in CFTM  an ECC error is reported to the CPU."]
        pub const CONST_011: Self = Self::new(1);
        #[doc = "10 B If a PFLASH ECC error occurs then an uncorrectable ECC error is reported to the CPU."]
        pub const CONST_102: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flashcon2_SPEC;
impl crate::sealed::RegSpec for Flashcon2_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Flash Configuration Register 2\n resetvalue={Application Reset:0x0AA020A0A}"]
pub type Flashcon2 = crate::RegValueT<Flashcon2_SPEC>;

impl Flashcon2 {
    #[doc = "Address Buffer Recording Disable. The system will behave as specified for 10 B if 00 B or 11 B is programmed  and an alarm will be generated. SBAB         DBAB  MBAB and ZBAB alarms will not be generated if RECDIS is 01 B ."]
    #[inline(always)]
    pub fn recdis(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        flashcon2::Recdis,
        Flashcon2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            flashcon2::Recdis,
            Flashcon2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "ECC Correction Disable   ECCCORDIS. The system will behave as specified for 10 B if 00 B or 11 B is programmed  and an alarm will be generated on the        next read."]
    #[inline(always)]
    pub fn ecccordis(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x3,
        1,
        0,
        flashcon2::Ecccordis,
        Flashcon2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            flashcon2::Ecccordis,
            Flashcon2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Hard Margin Selection. This register setting is effective only if FLASHCON2.MSEL   01 B . The system will behave as specified for 10 B if 00 B or        11 B is programmed  and an alarm will be generated."]
    #[inline(always)]
    pub fn hmargin(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x3,
        1,
        0,
        flashcon2::Hmargin,
        Flashcon2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x3,
            1,
            0,
            flashcon2::Hmargin,
            Flashcon2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Margin Read Selection. The system will behave as specified for 10 B if 00 B or 11 B is programmed  and an alarm will be generated."]
    #[inline(always)]
    pub fn msel(
        self,
    ) -> crate::common::RegisterField<
        10,
        0x3,
        1,
        0,
        flashcon2::Msel,
        Flashcon2_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            10,
            0x3,
            1,
            0,
            flashcon2::Msel,
            Flashcon2_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Clear ECC Status Register. The system will behave as specified for 10 B if 00 B or 11 B is programmed  and an alarm will be generated."]
    #[inline(always)]
    pub fn eccsclr(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        flashcon2::Eccsclr,
        Flashcon2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            flashcon2::Eccsclr,
            Flashcon2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Clear SBAB Record Registers. The system will behave as specified for 10 B if 00 B or 11 B is programmed  and an alarm will be generated."]
    #[inline(always)]
    pub fn sbabclr(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        flashcon2::Sbabclr,
        Flashcon2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            flashcon2::Sbabclr,
            Flashcon2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Clear DBAB Record Registers. The system will behave as specified for 10 B if 00 B or 11 B is programmed  and an alarm will be generated."]
    #[inline(always)]
    pub fn dbabclr(
        self,
    ) -> crate::common::RegisterField<
        26,
        0x3,
        1,
        0,
        flashcon2::Dbabclr,
        Flashcon2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            26,
            0x3,
            1,
            0,
            flashcon2::Dbabclr,
            Flashcon2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Clear MBAB Record Registers. The system will behave as specified for 10 B if 00 B or 11 B is programmed  and an alarm will be generated."]
    #[inline(always)]
    pub fn mbabclr(
        self,
    ) -> crate::common::RegisterField<
        28,
        0x3,
        1,
        0,
        flashcon2::Mbabclr,
        Flashcon2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            28,
            0x3,
            1,
            0,
            flashcon2::Mbabclr,
            Flashcon2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
    #[doc = "Clear ZBAB Record Registers   ZBABCLR. The system will behave as specified for 10 B if 00 B or 11 B is programmed  and an alarm will be generated."]
    #[inline(always)]
    pub fn zbabclr(
        self,
    ) -> crate::common::RegisterField<
        30,
        0x3,
        1,
        0,
        flashcon2::Zbabclr,
        Flashcon2_SPEC,
        crate::common::W,
    > {
        crate::common::RegisterField::<
            30,
            0x3,
            1,
            0,
            flashcon2::Zbabclr,
            Flashcon2_SPEC,
            crate::common::W,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Flashcon2 {
    #[inline(always)]
    fn default() -> Flashcon2 {
        <crate::RegValueT<Flashcon2_SPEC> as RegisterValue<_>>::new(2852260362)
    }
}
pub mod flashcon2 {
    pub struct Recdis_SPEC;
    pub type Recdis = crate::EnumBitfieldStruct<u8, Recdis_SPEC>;
    impl Recdis {
        #[doc = "01 B Disable local PFlash bank ECC error recording in SBAB  DBAB  MBAB and ZBAB."]
        pub const CONST_011: Self = Self::new(1);
        #[doc = "10 B Enable local PFlash bank ECC error recording in SBAB  DBAB  MBAB and ZBAB."]
        pub const CONST_102: Self = Self::new(2);
    }
    pub struct Ecccordis_SPEC;
    pub type Ecccordis = crate::EnumBitfieldStruct<u8, Ecccordis_SPEC>;
    impl Ecccordis {
        #[doc = "01 B ECC correction for the local PFlash bank read path is disabled."]
        pub const CONST_011: Self = Self::new(1);
        #[doc = "10 B ECC correction for the local PFlash bank read path is enabled."]
        pub const CONST_102: Self = Self::new(2);
    }
    pub struct Hmargin_SPEC;
    pub type Hmargin = crate::EnumBitfieldStruct<u8, Hmargin_SPEC>;
    impl Hmargin {
        #[doc = "01 B Tight1 Tight margin for 1  high  level  sub optimal 1 read as 0  for the local PFLASH."]
        pub const CONST_011: Self = Self::new(1);
        #[doc = "10 B Tight0 Tight margin for 0  low  level  sub optimal 0 read as 1  for the local PFLASH."]
        pub const CONST_102: Self = Self::new(2);
    }
    pub struct Msel_SPEC;
    pub type Msel = crate::EnumBitfieldStruct<u8, Msel_SPEC>;
    impl Msel {
        #[doc = "10 B Read with the standard margin for the local PFLASH."]
        pub const CONST_102: Self = Self::new(2);
        #[doc = "01 B Read with the margin selected by FLASHCON2.HMARGIN for the local PFLASH."]
        pub const CONST_011: Self = Self::new(1);
    }
    pub struct Eccsclr_SPEC;
    pub type Eccsclr = crate::EnumBitfieldStruct<u8, Eccsclr_SPEC>;
    impl Eccsclr {
        #[doc = "10 B No action."]
        pub const CONST_102: Self = Self::new(2);
        #[doc = "01 B Clear local PFlash bank ECC status register and the SBER and DBER alarms."]
        pub const CONST_011: Self = Self::new(1);
    }
    pub struct Sbabclr_SPEC;
    pub type Sbabclr = crate::EnumBitfieldStruct<u8, Sbabclr_SPEC>;
    impl Sbabclr {
        #[doc = "10 B No action."]
        pub const CONST_102: Self = Self::new(2);
        #[doc = "01 B Clear local PFlash bank SBAB record registers and the SBAB alarm."]
        pub const CONST_011: Self = Self::new(1);
    }
    pub struct Dbabclr_SPEC;
    pub type Dbabclr = crate::EnumBitfieldStruct<u8, Dbabclr_SPEC>;
    impl Dbabclr {
        #[doc = "10 B No action."]
        pub const CONST_102: Self = Self::new(2);
        #[doc = "01 B Clear local PFlash bank DBAB record registers and DBAB alarm."]
        pub const CONST_011: Self = Self::new(1);
    }
    pub struct Mbabclr_SPEC;
    pub type Mbabclr = crate::EnumBitfieldStruct<u8, Mbabclr_SPEC>;
    impl Mbabclr {
        #[doc = "10 B No action."]
        pub const CONST_102: Self = Self::new(2);
        #[doc = "01 B Clear local PFlash bank MBAB record registers and the MBAB alarm."]
        pub const CONST_011: Self = Self::new(1);
    }
    pub struct Zbabclr_SPEC;
    pub type Zbabclr = crate::EnumBitfieldStruct<u8, Zbabclr_SPEC>;
    impl Zbabclr {
        #[doc = "10 B No action."]
        pub const CONST_102: Self = Self::new(2);
        #[doc = "01 B Clear local PFlash bank ZBAB record registers and the ZBAB alarm."]
        pub const CONST_011: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flashcon3_SPEC;
impl crate::sealed::RegSpec for Flashcon3_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Flash Configuration Register 3\n resetvalue={Application Reset:0x0}"]
pub type Flashcon3 = crate::RegValueT<Flashcon3_SPEC>;

impl Flashcon3 {
    #[doc = "ECC Error Injection . Setting this bit enforces an error in the ECC error correction        supervision circuit. This can be used to check the correct function of        this circuit."]
    #[inline(always)]
    pub fn eccerrinj(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        flashcon3::Eccerrinj,
        Flashcon3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            flashcon3::Eccerrinj,
            Flashcon3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "EDC Error Injection. Setting this bit enforces an error in the EDC checker logic. This can be        used to check the correct function of this circuit."]
    #[inline(always)]
    pub fn edcerrinj(
        self,
    ) -> crate::common::RegisterField<
        1,
        0x1,
        1,
        0,
        flashcon3::Edcerrinj,
        Flashcon3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            1,
            0x1,
            1,
            0,
            flashcon3::Edcerrinj,
            Flashcon3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Corrected Single Bits Address Buffer  SBAB  Error Injection. Error injection logic to check the correct function of the SMU alarm."]
    #[inline(always)]
    pub fn sbaberrinj(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        flashcon3::Sbaberrinj,
        Flashcon3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            flashcon3::Sbaberrinj,
            Flashcon3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Corrected Double Bits Address Buffer  DBAB  Error Injection. Error injection logic to check the correct function of the SMU alarm."]
    #[inline(always)]
    pub fn dbaberrinj(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        flashcon3::Dbaberrinj,
        Flashcon3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            flashcon3::Dbaberrinj,
            Flashcon3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Uncorrected Multi Bit Address Buffer  MBAB  Error Injection. Error injection logic to check the correct function of the SMU alarm."]
    #[inline(always)]
    pub fn mbaberrinj(
        self,
    ) -> crate::common::RegisterField<
        4,
        0x1,
        1,
        0,
        flashcon3::Mbaberrinj,
        Flashcon3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            4,
            0x1,
            1,
            0,
            flashcon3::Mbaberrinj,
            Flashcon3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Uncorrected All Zeros Bits Address Buffer  ZBAB  Error Injection. Error injection logic to check the correct function of the SMU alarm."]
    #[inline(always)]
    pub fn zbaberrinj(
        self,
    ) -> crate::common::RegisterField<
        5,
        0x1,
        1,
        0,
        flashcon3::Zbaberrinj,
        Flashcon3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            5,
            0x1,
            1,
            0,
            flashcon3::Zbaberrinj,
            Flashcon3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Single Bit Error  SBER  Injection . Setting this bit generates SBER SMU alarm  and can be used to check the        correct function of this alarm."]
    #[inline(always)]
    pub fn sbererrinj(
        self,
    ) -> crate::common::RegisterField<
        6,
        0x1,
        1,
        0,
        flashcon3::Sbererrinj,
        Flashcon3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            6,
            0x1,
            1,
            0,
            flashcon3::Sbererrinj,
            Flashcon3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Double Bit Error  DBER  Injection. Setting this bit generates DBER SMU alarm and can be used to check the        correct function of the DBER SMU alarm."]
    #[inline(always)]
    pub fn dbererrinj(
        self,
    ) -> crate::common::RegisterField<
        7,
        0x1,
        1,
        0,
        flashcon3::Dbererrinj,
        Flashcon3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            7,
            0x1,
            1,
            0,
            flashcon3::Dbererrinj,
            Flashcon3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "NVM Configuration  NVMCER  Injection. Setting this bit enforces an error from the error detection logic        covering NVM configuration registers  and can be used to check the        correct function of the NVMCER SMU alarm."]
    #[inline(always)]
    pub fn nvmcerrinj(
        self,
    ) -> crate::common::RegisterField<
        8,
        0x1,
        1,
        0,
        flashcon3::Nvmcerrinj,
        Flashcon3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            8,
            0x1,
            1,
            0,
            flashcon3::Nvmcerrinj,
            Flashcon3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Flashcon Error  FLCONER  Injection. Setting this bit enforces an error from the error detection logic        covering FLASHCON complement pair register bits in the PFRWB  RECDIS         ECCCORDIS  HMARGIN  MSEL and all CLR bits   and can be used to check the        correct function of the FLCONER SMU alarm."]
    #[inline(always)]
    pub fn flconerrinj(
        self,
    ) -> crate::common::RegisterField<
        9,
        0x1,
        1,
        0,
        flashcon3::Flconerrinj,
        Flashcon3_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            9,
            0x1,
            1,
            0,
            flashcon3::Flconerrinj,
            Flashcon3_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Flashcon3 {
    #[inline(always)]
    fn default() -> Flashcon3 {
        <crate::RegValueT<Flashcon3_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod flashcon3 {
    pub struct Eccerrinj_SPEC;
    pub type Eccerrinj = crate::EnumBitfieldStruct<u8, Eccerrinj_SPEC>;
    impl Eccerrinj {
        #[doc = "0 B ECC logic operates normally."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 B An error is injected into the ECC logic."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Edcerrinj_SPEC;
    pub type Edcerrinj = crate::EnumBitfieldStruct<u8, Edcerrinj_SPEC>;
    impl Edcerrinj {
        #[doc = "0 B EDC logic operates normally."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 B An error is injected into the EDC checker logic."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sbaberrinj_SPEC;
    pub type Sbaberrinj = crate::EnumBitfieldStruct<u8, Sbaberrinj_SPEC>;
    impl Sbaberrinj {
        #[doc = "0 B SBAB logic operates normally."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 B Inject an SBAB alarm to the SMU."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Dbaberrinj_SPEC;
    pub type Dbaberrinj = crate::EnumBitfieldStruct<u8, Dbaberrinj_SPEC>;
    impl Dbaberrinj {
        #[doc = "0 B DBAB logic operates normally."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 B Inject an DBAB alarm to the SMU."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Mbaberrinj_SPEC;
    pub type Mbaberrinj = crate::EnumBitfieldStruct<u8, Mbaberrinj_SPEC>;
    impl Mbaberrinj {
        #[doc = "0 B MBAB logic operates normally."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 B Inject an MBAB alarm to the SMU."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Zbaberrinj_SPEC;
    pub type Zbaberrinj = crate::EnumBitfieldStruct<u8, Zbaberrinj_SPEC>;
    impl Zbaberrinj {
        #[doc = "0 B ZBAB logic operates normally."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 B Inject an ZBAB alarm to the SMU."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Sbererrinj_SPEC;
    pub type Sbererrinj = crate::EnumBitfieldStruct<u8, Sbererrinj_SPEC>;
    impl Sbererrinj {
        #[doc = "0 B SBER logic operates normally."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 B Inject a SBER alarm to the SMU."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Dbererrinj_SPEC;
    pub type Dbererrinj = crate::EnumBitfieldStruct<u8, Dbererrinj_SPEC>;
    impl Dbererrinj {
        #[doc = "0 B DBER logic operates normally."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 B Inject a DBER alarm to the SMU."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Nvmcerrinj_SPEC;
    pub type Nvmcerrinj = crate::EnumBitfieldStruct<u8, Nvmcerrinj_SPEC>;
    impl Nvmcerrinj {
        #[doc = "0 B NVM configuration logic operates normally."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 B Inject a NVMCER alarm to the SMU."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Flconerrinj_SPEC;
    pub type Flconerrinj = crate::EnumBitfieldStruct<u8, Flconerrinj_SPEC>;
    impl Flconerrinj {
        #[doc = "0 B FLASHCON register configuration operates normally."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 B Inject a FLCONER alarm to the SMU."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flashcon4_SPEC;
impl crate::sealed::RegSpec for Flashcon4_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Flash Configuration Register 4\n resetvalue={Application Reset:0x0}"]
pub type Flashcon4 = crate::RegValueT<Flashcon4_SPEC>;

impl Flashcon4 {
    #[doc = "Disable direct LPB access. Disable direct access by the CPU to the Local Pflash Bank  LPB ."]
    #[inline(always)]
    pub fn ddis(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x1,
        1,
        0,
        flashcon4::Ddis,
        Flashcon4_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x1,
            1,
            0,
            flashcon4::Ddis,
            Flashcon4_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Flashcon4 {
    #[inline(always)]
    fn default() -> Flashcon4 {
        <crate::RegValueT<Flashcon4_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod flashcon4 {
    pub struct Ddis_SPEC;
    pub type Ddis = crate::EnumBitfieldStruct<u8, Ddis_SPEC>;
    impl Ddis {
        #[doc = "0 B Direct access to the Local Pflash bank is enabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 B Direct access to the Local Pflash bank is disabled."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icnt_SPEC;
impl crate::sealed::RegSpec for Icnt_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Instruction Count\n resetvalue={Debug Reset:0x0}"]
pub type Icnt = crate::RegValueT<Icnt_SPEC>;

impl Icnt {
    #[doc = "Count Value   CountValue. Count of the Instructions Executed."]
    #[inline(always)]
    pub fn countvalue(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffffff, 1, 0, u32, Icnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7fffffff,1,0,u32, Icnt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sticky Overflow Bit   SOvf. This bit is set by hardware when count value  30 0    31 h7FFF FFFF. It can only be cleared by software."]
    #[inline(always)]
    pub fn sovf(self) -> crate::common::RegisterFieldBool<31, 1, 0, Icnt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Icnt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Icnt {
    #[inline(always)]
    fn default() -> Icnt {
        <crate::RegValueT<Icnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr_SPEC;
impl crate::sealed::RegSpec for Icr_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
pub type Icr = crate::RegValueT<Icr_SPEC>;

impl Icr {
    #[doc = "Current CPU Priority Number   CCPN. The Current CPU Priority Number  CCPN  bit field indicates the current priority level of the CPU. It is automatically updated by hardware on entry or exit of Interrupt Service Routines  ISRs  and through the execution of a BISR instruction. CCPN can also be updated through an MTCR instruction."]
    #[inline(always)]
    pub fn ccpn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Global Interrupt Enable Bit   IE. The interrupt enable bit globally enables the CPU service request system. Whether a service request is delivered to the CPU depends on the individual Service Request Enable Bits  SRE  in the SRNs  and the current state of the CPU. ICR.IE is automatically updated by hardware on entry and exit of an Interrupt Service Routine  ISR . ICR.IE is cleared to 0 when an interrupt is taken  and is restored to the previous value when the ISR executes an RFE instruction to terminate itself. ICR.IE can also be updated through the execution of the ENABLE  DISABLE  MTCR  and BISR instructions."]
    #[inline(always)]
    pub fn ie(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, icr::Ie, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<15,0x1,1,0,icr::Ie, Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Pending Interrupt Priority Number   PIPN. A read only bit field that is updated by the ICU at the end of each interrupt arbitration process. It indicates the priority number of the pending service request. ICR.PIPN is set to 0 when no request is pending  and at the beginning of each new arbitration process. ..."]
    #[inline(always)]
    pub fn pipn(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, icr::Pipn, Icr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,icr::Pipn, Icr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        <crate::RegValueT<Icr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod icr {
    pub struct Ie_SPEC;
    pub type Ie = crate::EnumBitfieldStruct<u8, Ie_SPEC>;
    impl Ie {
        #[doc = "0 Interrupt system is globally disabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Interrupt system is globally enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pipn_SPEC;
    pub type Pipn = crate::EnumBitfieldStruct<u8, Pipn_SPEC>;
    impl Pipn {
        #[doc = "00 No valid pending request."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Request pending  lowest priority."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "FF Request pending  highest priority."]
        pub const CONST_255255: Self = Self::new(255);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isp_SPEC;
impl crate::sealed::RegSpec for Isp_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Interrupt Stack Pointer\n resetvalue={Application Reset:0x100}"]
pub type Isp = crate::RegValueT<Isp_SPEC>;

impl Isp {
    #[doc = "Interrupt Stack Pointer   ISP"]
    #[inline(always)]
    pub fn isp(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Isp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Isp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Isp {
    #[inline(always)]
    fn default() -> Isp {
        <crate::RegValueT<Isp_SPEC> as RegisterValue<_>>::new(256)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krst0_SPEC;
impl crate::sealed::RegSpec for Krst0_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Reset Register 0\n resetvalue={Application Reset:0x0}"]
pub type Krst0 = crate::RegValueT<Krst0_SPEC>;

impl Krst0 {
    #[doc = "Kernel Reset   RST. This reset bit can be used to request a kernel reset. The kernel reset        will be executed if the reset bits of both kernel registers are set. The RST bit will be cleared  re set to   180 0  180   by the BPI FPI after the        kernel reset was executed."]
    #[inline(always)]
    pub fn rst(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, krst0::Rst, Krst0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,krst0::Rst, Krst0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Kernel Reset Status   RSTSTAT. This bit indicates wether a kernel reset was executed or not. This bit        is set immediately after the execution of a kernel reset. These bits can be cleared by writing   180 1  180  to the CLR bit in the related        KRSTCLR register."]
    #[inline(always)]
    pub fn rststat(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, krst0::Rststat, Krst0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x3,1,0,krst0::Rststat, Krst0_SPEC,crate::common::R>::from_register(self,0)
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
        #[doc = "0 No kernel reset        was requested"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A kernel reset        was requested"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Rststat_SPEC;
    pub type Rststat = crate::EnumBitfieldStruct<u8, Rststat_SPEC>;
    impl Rststat {
        #[doc = "00 No kernel        reset was executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Kernel reset        was requested by hardware since last clear  SMU"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Kernel reset        was requested by software since last clear  KRST"]
        pub const CONST_22: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krst1_SPEC;
impl crate::sealed::RegSpec for Krst1_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Reset Register 1\n resetvalue={Application Reset:0x0}"]
pub type Krst1 = crate::RegValueT<Krst1_SPEC>;

impl Krst1 {
    #[doc = "Kernel Reset   RST. This reset bit can be used to request a kernel reset. The kernel reset        will be executed if the reset bits of both kernel reset registers is set. The RST bit will be cleared  re set to   180 0  180   after the kernel reset was        executed."]
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
        #[doc = "0 No kernel reset        was requested"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 A kernel reset        was requested"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krstclr_SPEC;
impl crate::sealed::RegSpec for Krstclr_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Reset Clear Register\n resetvalue={Application Reset:0x0}"]
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
        #[doc = "1 Clear Kernel Reset Status KRST0.RSTSTAT 1 0"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcx_SPEC;
impl crate::sealed::RegSpec for Lcx_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Free CSA List Limit Pointer\n resetvalue={Application Reset:0x0}"]
pub type Lcx = crate::RegValueT<Lcx_SPEC>;

impl Lcx {
    #[doc = "LCX Offset Field   LCXO. The LCXO and LCXS fields form the pointer LCX  which points to the last available CSA."]
    #[inline(always)]
    pub fn lcxo(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Lcx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Lcx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LCX Segment Address   LCXS. This field is used in conjunction with the LCXO field."]
    #[inline(always)]
    pub fn lcxs(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Lcx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Lcx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Lcx {
    #[inline(always)]
    fn default() -> Lcx {
        <crate::RegValueT<Lcx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpbSprotAccenaR_SPEC;
impl crate::sealed::RegSpec for LpbSprotAccenaR_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection Region LPB Read Access Enable Register A\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type LpbSprotAccenaR = crate::RegValueT<LpbSprotAccenaR_SPEC>;

impl LpbSprotAccenaR {
    #[doc = "Access Enable for Master TAG ID n  n  0 31    EN. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        lpb_sprot_accena_r::En,
        LpbSprotAccenaR_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            lpb_sprot_accena_r::En,
            LpbSprotAccenaR_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for LpbSprotAccenaR {
    #[inline(always)]
    fn default() -> LpbSprotAccenaR {
        <crate::RegValueT<LpbSprotAccenaR_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod lpb_sprot_accena_r {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Read access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpbSprotAccenbR_SPEC;
impl crate::sealed::RegSpec for LpbSprotAccenbR_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region LPB Read Access Enable Register B\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type LpbSprotAccenbR = crate::RegValueT<LpbSprotAccenbR_SPEC>;

impl LpbSprotAccenbR {
    #[doc = "Access Enable for Master TAG ID n  n  32 63    EN. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        LpbSprotAccenbR_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            LpbSprotAccenbR_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for LpbSprotAccenbR {
    #[inline(always)]
    fn default() -> LpbSprotAccenbR {
        <crate::RegValueT<LpbSprotAccenbR_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct M1Cnt_SPEC;
impl crate::sealed::RegSpec for M1Cnt_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Multi Count Register 1\n resetvalue={Debug Reset:0x0}"]
pub type M1Cnt = crate::RegValueT<M1Cnt_SPEC>;

impl M1Cnt {
    #[doc = "Count Value   CountValue. Count of the Selected Event."]
    #[inline(always)]
    pub fn countvalue(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffffff, 1, 0, u32, M1Cnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7fffffff,1,0,u32, M1Cnt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sticky Overflow Bit   SOvf. This bit is set by hardware when count value  30 0    31 h7FFF FFFF. It can only be cleared by software."]
    #[inline(always)]
    pub fn sovf(self) -> crate::common::RegisterFieldBool<31, 1, 0, M1Cnt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, M1Cnt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for M1Cnt {
    #[inline(always)]
    fn default() -> M1Cnt {
        <crate::RegValueT<M1Cnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct M2Cnt_SPEC;
impl crate::sealed::RegSpec for M2Cnt_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Multi Count Register 2\n resetvalue={Debug Reset:0x0}"]
pub type M2Cnt = crate::RegValueT<M2Cnt_SPEC>;

impl M2Cnt {
    #[doc = "Count Value   CountValue. Count of the Selected Event."]
    #[inline(always)]
    pub fn countvalue(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffffff, 1, 0, u32, M2Cnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7fffffff,1,0,u32, M2Cnt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sticky Overflow Bit   SOvf. This bit is set by hardware when count value  30 0    31 h7FFF FFFF. It can only be cleared by software."]
    #[inline(always)]
    pub fn sovf(self) -> crate::common::RegisterFieldBool<31, 1, 0, M2Cnt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, M2Cnt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for M2Cnt {
    #[inline(always)]
    fn default() -> M2Cnt {
        <crate::RegValueT<M2Cnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct M3Cnt_SPEC;
impl crate::sealed::RegSpec for M3Cnt_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Multi Count Register 3\n resetvalue={Debug Reset:0x0}"]
pub type M3Cnt = crate::RegValueT<M3Cnt_SPEC>;

impl M3Cnt {
    #[doc = "Count Value   CountValue. Count of the Selected Event."]
    #[inline(always)]
    pub fn countvalue(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffffff, 1, 0, u32, M3Cnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7fffffff,1,0,u32, M3Cnt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sticky Overflow Bit   SOvf. This bit is set by hardware when count value  30 0    31 h7FFF FFFF. It can only be cleared by software."]
    #[inline(always)]
    pub fn sovf(self) -> crate::common::RegisterFieldBool<31, 1, 0, M3Cnt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, M3Cnt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for M3Cnt {
    #[inline(always)]
    fn default() -> M3Cnt {
        <crate::RegValueT<M3Cnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osel_SPEC;
impl crate::sealed::RegSpec for Osel_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Overlay Range Select Register\n resetvalue={Application Reset:0x0}"]
pub type Osel = crate::RegValueT<Osel_SPEC>;

impl Osel {
    #[doc = "Shadow Overlay Enable   SHOVEN x . One enable bit is provided for each of the 32 overlay blocks."]
    #[inline(always)]
    pub fn shoven_x(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        osel::ShovenX,
        Osel_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            osel::ShovenX,
            Osel_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for Osel {
    #[inline(always)]
    fn default() -> Osel {
        <crate::RegValueT<Osel_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod osel {
    pub struct ShovenX_SPEC;
    pub type ShovenX = crate::EnumBitfieldStruct<u8, ShovenX_SPEC>;
    impl ShovenX {
        #[doc = "0 Overlay block x is disabled when OVCCON.OVSTRT is set."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Overlay block x is enabled when OVCCON.OVSTRT is set."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pc_SPEC;
impl crate::sealed::RegSpec for Pc_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Program Counter\n resetvalue={Application Reset:0x0}"]
pub type Pc = crate::RegValueT<Pc_SPEC>;

impl Pc {
    #[doc = "Program Counter   PC"]
    #[inline(always)]
    pub fn pc(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, Pc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32, Pc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pc {
    #[inline(always)]
    fn default() -> Pc {
        <crate::RegValueT<Pc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcon0_SPEC;
impl crate::sealed::RegSpec for Pcon0_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Program Control 0\n resetvalue={Application Reset:0x2}"]
pub type Pcon0 = crate::RegValueT<Pcon0_SPEC>;

impl Pcon0 {
    #[doc = "Program Cache Bypass   PCBYP"]
    #[inline(always)]
    pub fn pcbyp(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, pcon0::Pcbyp, Pcon0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,pcon0::Pcbyp, Pcon0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pcon0 {
    #[inline(always)]
    fn default() -> Pcon0 {
        <crate::RegValueT<Pcon0_SPEC> as RegisterValue<_>>::new(2)
    }
}
pub mod pcon0 {
    pub struct Pcbyp_SPEC;
    pub type Pcbyp = crate::EnumBitfieldStruct<u8, Pcbyp_SPEC>;
    impl Pcbyp {
        #[doc = "0 Cache enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Cache bypass  disabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcon1_SPEC;
impl crate::sealed::RegSpec for Pcon1_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Program Control 1\n resetvalue={Application Reset:0x0}"]
pub type Pcon1 = crate::RegValueT<Pcon1_SPEC>;

impl Pcon1 {
    #[doc = "Program Cache Invalidate   PCINV"]
    #[inline(always)]
    pub fn pcinv(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, pcon1::Pcinv, Pcon1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,pcon1::Pcinv, Pcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Program Buffer Invalidate   PBINV. Write Operation  This field returns 0 when read."]
    #[inline(always)]
    pub fn pbinv(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, pcon1::Pbinv, Pcon1_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,pcon1::Pbinv, Pcon1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pcon1 {
    #[inline(always)]
    fn default() -> Pcon1 {
        <crate::RegValueT<Pcon1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pcon1 {
    pub struct Pcinv_SPEC;
    pub type Pcinv = crate::EnumBitfieldStruct<u8, Pcinv_SPEC>;
    impl Pcinv {
        #[doc = "0 Write  No effect  normal instruction cache operation. Read   Normal operation  instruction cache available"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write   Initiate invalidation of entire instruction cache. Read  Instruction cache invalidation in progress. Instruction cache unavailable."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Pbinv_SPEC;
    pub type Pbinv = crate::EnumBitfieldStruct<u8, Pbinv_SPEC>;
    impl Pbinv {
        #[doc = "0 Write  No effect. Normal program line buffer operation."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write  Invalidate the program line buffer."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcon2_SPEC;
impl crate::sealed::RegSpec for Pcon2_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Program Control 2\n resetvalue={Application Reset:0x0}"]
pub type Pcon2 = crate::RegValueT<Pcon2_SPEC>;

impl Pcon2 {
    #[doc = "Program Cache Size  ICACHE  in KBytes   PCACHE SZE. In KBytes"]
    #[inline(always)]
    pub fn pcache_sze(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Pcon2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Pcon2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Program Scratch Size in KBytes   PSCRATCH SZE. In KBytes"]
    #[inline(always)]
    pub fn pscratch_sze(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Pcon2_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Pcon2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pcon2 {
    #[inline(always)]
    fn default() -> Pcon2 {
        <crate::RegValueT<Pcon2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcxi_SPEC;
impl crate::sealed::RegSpec for Pcxi_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Previous Context Information Register\n resetvalue={Application Reset:0x0}"]
pub type Pcxi = crate::RegValueT<Pcxi_SPEC>;

impl Pcxi {
    #[doc = "Previous Context Pointer Offset Field   PCXO. The PCXO and PCXS fields form the pointer PCX  which points to the CSA of the previous context."]
    #[inline(always)]
    pub fn pcxo(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Pcxi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Pcxi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Previous Context Pointer Segment Address   PCXS. Contains the segment address portion of the PCX. This field is used in conjunction with the PCXO field."]
    #[inline(always)]
    pub fn pcxs(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Pcxi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Pcxi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Upper or Lower Context Tag   UL. Identifies the type of context saved. If the type does not match the type expected when a context restore operation is performed  a trap is generated."]
    #[inline(always)]
    pub fn ul(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, pcxi::Ul, Pcxi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x1,1,0,pcxi::Ul, Pcxi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Previous Interrupt Enable   PIE. Indicates the state of the interrupt enable bit  ICR.IE  for the interrupted task."]
    #[inline(always)]
    pub fn pie(self) -> crate::common::RegisterFieldBool<21, 1, 0, Pcxi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Pcxi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Previous CPU Priority Number   PCPN. Contains the priority level number of the interrupted task."]
    #[inline(always)]
    pub fn pcpn(
        self,
    ) -> crate::common::RegisterField<22, 0xff, 1, 0, u8, Pcxi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0xff,1,0,u8, Pcxi_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pcxi {
    #[inline(always)]
    fn default() -> Pcxi {
        <crate::RegValueT<Pcxi_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pcxi {
    pub struct Ul_SPEC;
    pub type Ul = crate::EnumBitfieldStruct<u8, Ul_SPEC>;
    impl Ul {
        #[doc = "0 Lower Context"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Upper Context"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Piear_SPEC;
impl crate::sealed::RegSpec for Piear_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Program Integrity Error Address Register\n resetvalue={Application Reset:0x0}"]
pub type Piear = crate::RegValueT<Piear_SPEC>;

impl Piear {
    #[doc = "Transaction Address   TA. Physical address being accessed by operation that encountered program integrity error."]
    #[inline(always)]
    pub fn ta(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Piear_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Piear_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Piear {
    #[inline(always)]
    fn default() -> Piear {
        <crate::RegValueT<Piear_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pietr_SPEC;
impl crate::sealed::RegSpec for Pietr_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Program Integrity Error Trap Register\n resetvalue={Application Reset:0x0}"]
pub type Pietr = crate::RegValueT<Pietr_SPEC>;

impl Pietr {
    #[doc = "Integrity Error Detected   IED"]
    #[inline(always)]
    pub fn ied(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, pietr::Ied, Pietr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,pietr::Ied, Pietr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Integrity Error   TAG Memory   IE T"]
    #[inline(always)]
    pub fn ie_t(self) -> crate::common::RegisterFieldBool<1, 1, 0, Pietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Integrity Error   Cache Memory   IE C"]
    #[inline(always)]
    pub fn ie_c(self) -> crate::common::RegisterFieldBool<2, 1, 0, Pietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Pietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Integrity Error   Scratchpad Memory   IE S"]
    #[inline(always)]
    pub fn ie_s(self) -> crate::common::RegisterFieldBool<3, 1, 0, Pietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Pietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Integrity Error   Bus Interface   IE BI"]
    #[inline(always)]
    pub fn ie_bi(self) -> crate::common::RegisterFieldBool<4, 1, 0, Pietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Pietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Error Information   E INFO. If IE BS  1  Bus Master Tag ID of requesting masterIf IE C   1  Cache way."]
    #[inline(always)]
    pub fn e_info(
        self,
    ) -> crate::common::RegisterField<5, 0x3f, 1, 0, u8, Pietr_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x3f,1,0,u8, Pietr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Integrity Error   Uncorrectable Error Detected   IE UNC"]
    #[inline(always)]
    pub fn ie_unc(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Pietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Pietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Safety Protection Error Detected   IE SP"]
    #[inline(always)]
    pub fn ie_sp(self) -> crate::common::RegisterFieldBool<12, 1, 0, Pietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Pietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Bus Slave Access Indicator   IE BS"]
    #[inline(always)]
    pub fn ie_bs(self) -> crate::common::RegisterFieldBool<13, 1, 0, Pietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Pietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Phase error detected at SRI slave interface   IE ADDR"]
    #[inline(always)]
    pub fn ie_addr(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Pietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Pietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Integrity Error   Local Pflash bank   IE LPB"]
    #[inline(always)]
    pub fn ie_lpb(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Pietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Pietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Memory Test Mode Violation detected   IE MTMV"]
    #[inline(always)]
    pub fn ie_mtmv(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Pietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Pietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Pietr {
    #[inline(always)]
    fn default() -> Pietr {
        <crate::RegValueT<Pietr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod pietr {
    pub struct Ied_SPEC;
    pub type Ied = crate::EnumBitfieldStruct<u8, Ied_SPEC>;
    impl Ied {
        #[doc = "0 Write  Clear IED bit  re enable PIETR and PIEAR update. Read   No data integrity error condition occurred"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write   No Effect. Read  Data integrity error condition detected. PIETR and PIEAR contents valid  further PIETR and PIEAR updates disabled.."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pma0_SPEC;
impl crate::sealed::RegSpec for Pma0_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Access CacheabilityRegister\n resetvalue={Application Reset:0x300}"]
pub type Pma0 = crate::RegValueT<Pma0_SPEC>;

impl Pma0 {
    #[doc = "Data Access Cacheability Segments FHto 0H   DAC.  Note   segments F H  E H  D H and A H are constrained to be        non cacheable"]
    #[inline(always)]
    pub fn dac(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Pma0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Pma0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pma0 {
    #[inline(always)]
    fn default() -> Pma0 {
        <crate::RegValueT<Pma0_SPEC> as RegisterValue<_>>::new(768)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pma1_SPEC;
impl crate::sealed::RegSpec for Pma1_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Code Access CacheabilityRegister\n resetvalue={Application Reset:0x300}"]
pub type Pma1 = crate::RegValueT<Pma1_SPEC>;

impl Pma1 {
    #[doc = "Code Access Cacheability Segments FH 0H   CAC.  Note  Segments F H  E H  C H  A H are constrained to be non cacheable"]
    #[inline(always)]
    pub fn cac(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Pma1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Pma1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Pma1 {
    #[inline(always)]
    fn default() -> Pma1 {
        <crate::RegValueT<Pma1_SPEC> as RegisterValue<_>>::new(768)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pma2_SPEC;
impl crate::sealed::RegSpec for Pma2_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Peripheral Space Identifier register\n resetvalue={Application Reset:0x0C000}"]
pub type Pma2 = crate::RegValueT<Pma2_SPEC>;

impl Pma2 {
    #[doc = "Peripheral Space Identifier Segments FH 0H   PSI"]
    #[inline(always)]
    pub fn psi(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Pma2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Pma2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Pma2 {
    #[inline(always)]
    fn default() -> Pma2 {
        <crate::RegValueT<Pma2_SPEC> as RegisterValue<_>>::new(49152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pstr_SPEC;
impl crate::sealed::RegSpec for Pstr_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Program Synchronous Trap Register\n resetvalue={Application Reset:0x0}"]
pub type Pstr = crate::RegValueT<Pstr_SPEC>;

impl Pstr {
    #[doc = "Fetch Range Error   FRE. A Fetch Range Error occurs whenever an access to the Program Scratch is outside the range of the SRAM."]
    #[inline(always)]
    pub fn fre(self) -> crate::common::RegisterFieldBool<0, 1, 0, Pstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fetch Bus Error   FBE. A Fetch bus error will be set whenever the SRI flags an error due a fetch from external memory. This will be set for both direct fetches from the bus and for cache refills."]
    #[inline(always)]
    pub fn fbe(self) -> crate::common::RegisterFieldBool<2, 1, 0, Pstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Pstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fetch Peripheral Error   FPE. A Fetch peripheral error will be flagged whenever a fetch is attempted to peripheral space."]
    #[inline(always)]
    pub fn fpe(self) -> crate::common::RegisterFieldBool<12, 1, 0, Pstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Pstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fetch MSIST Error   FME. During SIST mode  a fetch from the PTAG will cause a PSE trap to occur."]
    #[inline(always)]
    pub fn fme(self) -> crate::common::RegisterFieldBool<14, 1, 0, Pstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Pstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Pstr {
    #[inline(always)]
    fn default() -> Pstr {
        <crate::RegValueT<Pstr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psw_SPEC;
impl crate::sealed::RegSpec for Psw_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Program Status Word\n resetvalue={Application Reset:0x0B80}"]
pub type Psw = crate::RegValueT<Psw_SPEC>;

impl Psw {
    #[doc = "Call Depth Counter   CDC. Consists of two variable width subfields. The first subfield consists of a string of zero or more initial 1 bits  terminated by the first 0 bit. The remaining bits form the second subfield  CDC.COUNT  which constitutes the Call Depth Count value. The count value is incremented on each Call and is decremented on a Return. 0cccccc B   6 bit counter  trap on overflow. 10ccccc B   5 bit counter  trap on overflow. 110cccc B   4 bit counter  trap on overflow. 1110ccc B   3 bit counter  trap on overflow. 11110cc B   2 bit counter  trap on overflow. 111110c B   1 bit counter  trap on overflow. 1111110 B   Trap every call  Call Trace mode . 1111111 B   Disable Call Depth Counting. When the call depth count  CDC.COUNT  overflows a trap  CDO  is generated. Setting the CDC to 1111110 B allows no bits for the counter and causes every call to be trapped. This is used for Call Depth Tracing. Setting the CDC to 1111111 B disables Call Depth Counting."]
    #[inline(always)]
    pub fn cdc(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Psw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8, Psw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Call Depth Count Enable   CDE. Enables call depth counting  provided that the PSW.CDC mask field is not all set to 1. If PSW.CDC   1111111 B   call depth counting is disabled regardless of the setting on the PSW.CDE bit."]
    #[inline(always)]
    pub fn cde(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, psw::Cde, Psw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,psw::Cde, Psw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Stack Control   IS. Determines if the current execution thread is using the shared global  interrupt  stack or a user stack."]
    #[inline(always)]
    pub fn is(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, psw::Is, Psw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,psw::Is, Psw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Privilege Level Control  I O Privilege    IO. Determines the access level to special function registers and peripheral devices."]
    #[inline(always)]
    pub fn io(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, psw::Io, Psw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,psw::Io, Psw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Safe Task Identifier   S"]
    #[inline(always)]
    pub fn s(self) -> crate::common::RegisterFieldBool<14, 1, 0, Psw_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Psw_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "User Status Bits   USB. The eight most significant bits of the PSW are designated as User Status Bits. These bits may be set or cleared as side effects of instruction execution. Refer to the TriCore Architecture manual for details."]
    #[inline(always)]
    pub fn usb(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Psw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Psw_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Psw {
    #[inline(always)]
    fn default() -> Psw {
        <crate::RegValueT<Psw_SPEC> as RegisterValue<_>>::new(2944)
    }
}
pub mod psw {
    pub struct Cde_SPEC;
    pub type Cde = crate::EnumBitfieldStruct<u8, Cde_SPEC>;
    impl Cde {
        #[doc = "0 Call depth counting is temporarily disabled. It is automatically re enabled after execution of the next Call instruction."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Call depth counting is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Is_SPEC;
    pub type Is = crate::EnumBitfieldStruct<u8, Is_SPEC>;
    impl Is {
        #[doc = "0 User Stack.  If an interrupt is taken when the IS bit is 0  then the stack pointer register is loaded from the ISP register before execution starts at the first instruction of the Interrupt Service Routine  ISR ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Shared Global Stack.  If an interrupt is taken when the PSW.IS bit is 1  then the current value of the stack pointer is used by the Interrupt Service Routine  ISR ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Io_SPEC;
    pub type Io = crate::EnumBitfieldStruct<u8, Io_SPEC>;
    impl Io {
        #[doc = "00 User 0 Mode No peripheral access. Access to memory regions with the peripheral space attribute are prohibited and results in a PSE or MPP trap. This access level is given to tasks that need not directly access peripheral devices. Tasks at this level do not have permission to enable or disable interrupts."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 User 1 Mode Regular peripheral access. Enables access to common peripheral devices that are not specially protected  including read write access to serial I O ports  read access to timers  and access to most I O status registers. Tasks at this level may disable interrupts."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Supervisor Mode Enables access to all peripheral devices. It enables read write access to core registers and protected peripheral devices. Tasks at this level may disable interrupts."]
        pub const CONST_22: Self = Self::new(2);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Segen_SPEC;
impl crate::sealed::RegSpec for Segen_SPEC {
    type DataType = u32;
}
#[doc = "CPUx SRI Error Generation Register\n resetvalue={Application Reset:0x0}"]
pub type Segen = crate::RegValueT<Segen_SPEC>;

impl Segen {
    #[doc = "Address ECC Bit Flip   ADFLIP. SRI address ECC Bits to be flipped on the next read or write transaction from the DMI when enabled by AE."]
    #[inline(always)]
    pub fn adflip(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, segen::Adflip, Segen_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xff,1,0,segen::Adflip, Segen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Type of error   ADTYPE"]
    #[inline(always)]
    pub fn adtype(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, segen::Adtype, Segen_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,segen::Adtype, Segen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Activate Error Enable   AE. Enabled the selective inverting of SRI ECC packet bits defined by ADFLIP. This bit will be cleared by hardware after the next SRI read or write transaction from the DMI."]
    #[inline(always)]
    pub fn ae(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, segen::Ae, Segen_SPEC, crate::common::RW> {
        crate::common::RegisterField::<31,0x1,1,0,segen::Ae, Segen_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Segen {
    #[inline(always)]
    fn default() -> Segen {
        <crate::RegValueT<Segen_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod segen {
    pub struct Adflip_SPEC;
    pub type Adflip = crate::EnumBitfieldStruct<u8, Adflip_SPEC>;
    impl Adflip {
        #[doc = "0 No Flip"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Flip"]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Adtype_SPEC;
    pub type Adtype = crate::EnumBitfieldStruct<u8, Adtype_SPEC>;
    impl Adtype {
        #[doc = "00 Data Master Address Phase"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Data Master Write Data"]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Data Slave Read Data"]
        pub const CONST_22: Self = Self::new(2);
    }
    pub struct Ae_SPEC;
    pub type Ae = crate::EnumBitfieldStruct<u8, Ae_SPEC>;
    impl Ae {
        #[doc = "0 Not Enabled"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Enabled"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SfrSprotAccenaW_SPEC;
impl crate::sealed::RegSpec for SfrSprotAccenaW_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection Register Access Enable Register A\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type SfrSprotAccenaW = crate::RegValueT<SfrSprotAccenaW_SPEC>;

impl SfrSprotAccenaW {
    #[doc = "Access Enable for Master TAG ID n  n  0 31    EN. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        sfr_sprot_accena_w::En,
        SfrSprotAccenaW_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            sfr_sprot_accena_w::En,
            SfrSprotAccenaW_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for SfrSprotAccenaW {
    #[inline(always)]
    fn default() -> SfrSprotAccenaW {
        <crate::RegValueT<SfrSprotAccenaW_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod sfr_sprot_accena_w {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Write access        will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Write access        will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SfrSprotAccenbW_SPEC;
impl crate::sealed::RegSpec for SfrSprotAccenbW_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Safety Protection Region Access Enable Register B\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type SfrSprotAccenbW = crate::RegValueT<SfrSprotAccenbW_SPEC>;

impl SfrSprotAccenbW {
    #[doc = "Access Enable for Master TAG ID n  n  32 63    EN. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        SfrSprotAccenbW_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            SfrSprotAccenbW_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for SfrSprotAccenbW {
    #[inline(always)]
    fn default() -> SfrSprotAccenbW {
        <crate::RegValueT<SfrSprotAccenbW_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smacon_SPEC;
impl crate::sealed::RegSpec for Smacon_SPEC {
    type DataType = u32;
}
#[doc = "CPUx SIST Mode Access Control Register\n resetvalue={Application Reset:0x0}"]
pub type Smacon = crate::RegValueT<Smacon_SPEC>;

impl Smacon {
    #[doc = "In Order Data Transactions   IODT"]
    #[inline(always)]
    pub fn iodt(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, smacon::Iodt, Smacon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,smacon::Iodt, Smacon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Smacon {
    #[inline(always)]
    fn default() -> Smacon {
        <crate::RegValueT<Smacon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smacon {
    pub struct Iodt_SPEC;
    pub type Iodt = crate::EnumBitfieldStruct<u8, Iodt_SPEC>;
    impl Iodt {
        #[doc = "0 Normal operation  Non dependent loads bypass stores."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 In order operation  Loads always flush preceding stores  processor store buffer disabled."]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SprSprotRgnaccena0R_SPEC;
impl crate::sealed::RegSpec for SprSprotRgnaccena0R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region SPR Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type SprSprotRgnaccena0R = crate::RegValueT<SprSprotRgnaccena0R_SPEC>;

impl SprSprotRgnaccena0R {
    #[doc = "Read Access Enable for Master TAG ID n  n 0 31    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        spr_sprot_rgnaccena0_r::En,
        SprSprotRgnaccena0R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            spr_sprot_rgnaccena0_r::En,
            SprSprotRgnaccena0R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for SprSprotRgnaccena0R {
    #[inline(always)]
    fn default() -> SprSprotRgnaccena0R {
        <crate::RegValueT<SprSprotRgnaccena0R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod spr_sprot_rgnaccena0_r {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Read access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SprSprotRgnaccena1R_SPEC;
impl crate::sealed::RegSpec for SprSprotRgnaccena1R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region SPR Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type SprSprotRgnaccena1R = crate::RegValueT<SprSprotRgnaccena1R_SPEC>;

impl SprSprotRgnaccena1R {
    #[doc = "Read Access Enable for Master TAG ID n  n 0 31    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        spr_sprot_rgnaccena1_r::En,
        SprSprotRgnaccena1R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            spr_sprot_rgnaccena1_r::En,
            SprSprotRgnaccena1R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for SprSprotRgnaccena1R {
    #[inline(always)]
    fn default() -> SprSprotRgnaccena1R {
        <crate::RegValueT<SprSprotRgnaccena1R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod spr_sprot_rgnaccena1_r {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Read access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SprSprotRgnaccena2R_SPEC;
impl crate::sealed::RegSpec for SprSprotRgnaccena2R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region SPR Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type SprSprotRgnaccena2R = crate::RegValueT<SprSprotRgnaccena2R_SPEC>;

impl SprSprotRgnaccena2R {
    #[doc = "Read Access Enable for Master TAG ID n  n 0 31    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        spr_sprot_rgnaccena2_r::En,
        SprSprotRgnaccena2R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            spr_sprot_rgnaccena2_r::En,
            SprSprotRgnaccena2R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for SprSprotRgnaccena2R {
    #[inline(always)]
    fn default() -> SprSprotRgnaccena2R {
        <crate::RegValueT<SprSprotRgnaccena2R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod spr_sprot_rgnaccena2_r {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Read access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SprSprotRgnaccena3R_SPEC;
impl crate::sealed::RegSpec for SprSprotRgnaccena3R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region SPR Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type SprSprotRgnaccena3R = crate::RegValueT<SprSprotRgnaccena3R_SPEC>;

impl SprSprotRgnaccena3R {
    #[doc = "Read Access Enable for Master TAG ID n  n 0 31    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        spr_sprot_rgnaccena3_r::En,
        SprSprotRgnaccena3R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            spr_sprot_rgnaccena3_r::En,
            SprSprotRgnaccena3R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for SprSprotRgnaccena3R {
    #[inline(always)]
    fn default() -> SprSprotRgnaccena3R {
        <crate::RegValueT<SprSprotRgnaccena3R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod spr_sprot_rgnaccena3_r {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Read access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SprSprotRgnaccena4R_SPEC;
impl crate::sealed::RegSpec for SprSprotRgnaccena4R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region SPR Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type SprSprotRgnaccena4R = crate::RegValueT<SprSprotRgnaccena4R_SPEC>;

impl SprSprotRgnaccena4R {
    #[doc = "Read Access Enable for Master TAG ID n  n 0 31    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        spr_sprot_rgnaccena4_r::En,
        SprSprotRgnaccena4R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            spr_sprot_rgnaccena4_r::En,
            SprSprotRgnaccena4R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for SprSprotRgnaccena4R {
    #[inline(always)]
    fn default() -> SprSprotRgnaccena4R {
        <crate::RegValueT<SprSprotRgnaccena4R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod spr_sprot_rgnaccena4_r {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Read access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SprSprotRgnaccena5R_SPEC;
impl crate::sealed::RegSpec for SprSprotRgnaccena5R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region SPR Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type SprSprotRgnaccena5R = crate::RegValueT<SprSprotRgnaccena5R_SPEC>;

impl SprSprotRgnaccena5R {
    #[doc = "Read Access Enable for Master TAG ID n  n 0 31    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        spr_sprot_rgnaccena5_r::En,
        SprSprotRgnaccena5R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            spr_sprot_rgnaccena5_r::En,
            SprSprotRgnaccena5R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for SprSprotRgnaccena5R {
    #[inline(always)]
    fn default() -> SprSprotRgnaccena5R {
        <crate::RegValueT<SprSprotRgnaccena5R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod spr_sprot_rgnaccena5_r {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Read access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SprSprotRgnaccena6R_SPEC;
impl crate::sealed::RegSpec for SprSprotRgnaccena6R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region SPR Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type SprSprotRgnaccena6R = crate::RegValueT<SprSprotRgnaccena6R_SPEC>;

impl SprSprotRgnaccena6R {
    #[doc = "Read Access Enable for Master TAG ID n  n 0 31    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        spr_sprot_rgnaccena6_r::En,
        SprSprotRgnaccena6R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            spr_sprot_rgnaccena6_r::En,
            SprSprotRgnaccena6R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for SprSprotRgnaccena6R {
    #[inline(always)]
    fn default() -> SprSprotRgnaccena6R {
        <crate::RegValueT<SprSprotRgnaccena6R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod spr_sprot_rgnaccena6_r {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Read access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SprSprotRgnaccena7R_SPEC;
impl crate::sealed::RegSpec for SprSprotRgnaccena7R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region SPR Read Access Enable Register A7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type SprSprotRgnaccena7R = crate::RegValueT<SprSprotRgnaccena7R_SPEC>;

impl SprSprotRgnaccena7R {
    #[doc = "Read Access Enable for Master TAG ID n  n 0 31    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        spr_sprot_rgnaccena7_r::En,
        SprSprotRgnaccena7R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            spr_sprot_rgnaccena7_r::En,
            SprSprotRgnaccena7R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for SprSprotRgnaccena7R {
    #[inline(always)]
    fn default() -> SprSprotRgnaccena7R {
        <crate::RegValueT<SprSprotRgnaccena7R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod spr_sprot_rgnaccena7_r {
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "0 Read access will not be executed"]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Read access will be executed"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SprSprotRgnaccenb0R_SPEC;
impl crate::sealed::RegSpec for SprSprotRgnaccenb0R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region SPR Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type SprSprotRgnaccenb0R = crate::RegValueT<SprSprotRgnaccenb0R_SPEC>;

impl SprSprotRgnaccenb0R {
    #[doc = "Read Access Enable for Master TAG ID n  n 32 63    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        SprSprotRgnaccenb0R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            SprSprotRgnaccenb0R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for SprSprotRgnaccenb0R {
    #[inline(always)]
    fn default() -> SprSprotRgnaccenb0R {
        <crate::RegValueT<SprSprotRgnaccenb0R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SprSprotRgnaccenb1R_SPEC;
impl crate::sealed::RegSpec for SprSprotRgnaccenb1R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region SPR Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type SprSprotRgnaccenb1R = crate::RegValueT<SprSprotRgnaccenb1R_SPEC>;

impl SprSprotRgnaccenb1R {
    #[doc = "Read Access Enable for Master TAG ID n  n 32 63    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        SprSprotRgnaccenb1R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            SprSprotRgnaccenb1R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for SprSprotRgnaccenb1R {
    #[inline(always)]
    fn default() -> SprSprotRgnaccenb1R {
        <crate::RegValueT<SprSprotRgnaccenb1R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SprSprotRgnaccenb2R_SPEC;
impl crate::sealed::RegSpec for SprSprotRgnaccenb2R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region SPR Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type SprSprotRgnaccenb2R = crate::RegValueT<SprSprotRgnaccenb2R_SPEC>;

impl SprSprotRgnaccenb2R {
    #[doc = "Read Access Enable for Master TAG ID n  n 32 63    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        SprSprotRgnaccenb2R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            SprSprotRgnaccenb2R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for SprSprotRgnaccenb2R {
    #[inline(always)]
    fn default() -> SprSprotRgnaccenb2R {
        <crate::RegValueT<SprSprotRgnaccenb2R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SprSprotRgnaccenb3R_SPEC;
impl crate::sealed::RegSpec for SprSprotRgnaccenb3R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region SPR Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type SprSprotRgnaccenb3R = crate::RegValueT<SprSprotRgnaccenb3R_SPEC>;

impl SprSprotRgnaccenb3R {
    #[doc = "Read Access Enable for Master TAG ID n  n 32 63    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        SprSprotRgnaccenb3R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            SprSprotRgnaccenb3R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for SprSprotRgnaccenb3R {
    #[inline(always)]
    fn default() -> SprSprotRgnaccenb3R {
        <crate::RegValueT<SprSprotRgnaccenb3R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SprSprotRgnaccenb4R_SPEC;
impl crate::sealed::RegSpec for SprSprotRgnaccenb4R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region SPR Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type SprSprotRgnaccenb4R = crate::RegValueT<SprSprotRgnaccenb4R_SPEC>;

impl SprSprotRgnaccenb4R {
    #[doc = "Read Access Enable for Master TAG ID n  n 32 63    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        SprSprotRgnaccenb4R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            SprSprotRgnaccenb4R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for SprSprotRgnaccenb4R {
    #[inline(always)]
    fn default() -> SprSprotRgnaccenb4R {
        <crate::RegValueT<SprSprotRgnaccenb4R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SprSprotRgnaccenb5R_SPEC;
impl crate::sealed::RegSpec for SprSprotRgnaccenb5R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region SPR Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type SprSprotRgnaccenb5R = crate::RegValueT<SprSprotRgnaccenb5R_SPEC>;

impl SprSprotRgnaccenb5R {
    #[doc = "Read Access Enable for Master TAG ID n  n 32 63    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        SprSprotRgnaccenb5R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            SprSprotRgnaccenb5R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for SprSprotRgnaccenb5R {
    #[inline(always)]
    fn default() -> SprSprotRgnaccenb5R {
        <crate::RegValueT<SprSprotRgnaccenb5R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SprSprotRgnaccenb6R_SPEC;
impl crate::sealed::RegSpec for SprSprotRgnaccenb6R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region SPR Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type SprSprotRgnaccenb6R = crate::RegValueT<SprSprotRgnaccenb6R_SPEC>;

impl SprSprotRgnaccenb6R {
    #[doc = "Read Access Enable for Master TAG ID n  n 32 63    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        SprSprotRgnaccenb6R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            SprSprotRgnaccenb6R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for SprSprotRgnaccenb6R {
    #[inline(always)]
    fn default() -> SprSprotRgnaccenb6R {
        <crate::RegValueT<SprSprotRgnaccenb6R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SprSprotRgnaccenb7R_SPEC;
impl crate::sealed::RegSpec for SprSprotRgnaccenb7R_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Safety Protection Region SPR Read Access Enable Register B7\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type SprSprotRgnaccenb7R = crate::RegValueT<SprSprotRgnaccenb7R_SPEC>;

impl SprSprotRgnaccenb7R {
    #[doc = "Read Access Enable for Master TAG ID n  n 32 63    EN. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xffffffff,
        1,
        0,
        u32,
        SprSprotRgnaccenb7R_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0xffffffff,
            1,
            0,
            u32,
            SprSprotRgnaccenb7R_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl core::default::Default for SprSprotRgnaccenb7R {
    #[inline(always)]
    fn default() -> SprSprotRgnaccenb7R {
        <crate::RegValueT<SprSprotRgnaccenb7R_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swevt_SPEC;
impl crate::sealed::RegSpec for Swevt_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Software Debug Event\n resetvalue={Debug Reset:0x0}"]
pub type Swevt = crate::RegValueT<Swevt_SPEC>;

impl Swevt {
    #[doc = "Event Associated   EVTA. Debug Action associated with the Debug Event"]
    #[inline(always)]
    pub fn evta(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, swevt::Evta, Swevt_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x7,1,0,swevt::Evta, Swevt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Break Before Make  BBM  or Break After Make  BAM  Selection   BBM"]
    #[inline(always)]
    pub fn bbm(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, swevt::Bbm, Swevt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,swevt::Bbm, Swevt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Breakout Disable   BOD"]
    #[inline(always)]
    pub fn bod(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, swevt::Bod, Swevt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,swevt::Bod, Swevt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CDC Suspend Out Signal State   SUSP. Value to be assigned to the CDC suspend out signal when the event is raised."]
    #[inline(always)]
    pub fn susp(self) -> crate::common::RegisterFieldBool<5, 1, 0, Swevt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Swevt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Counter   CNT. When this event occurs adjust the control of the performance counters in task mode as follows"]
    #[inline(always)]
    pub fn cnt(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, swevt::Cnt, Swevt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,swevt::Cnt, Swevt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Swevt {
    #[inline(always)]
    fn default() -> Swevt {
        <crate::RegValueT<Swevt_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod swevt {
    pub struct Evta_SPEC;
    pub type Evta = crate::EnumBitfieldStruct<u8, Evta_SPEC>;
    impl Evta {
        #[doc = "000 BOD 0  Disabled. BOD 1  Disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "001 BOD 0  Pulse BRKOUT Signal. BOD 1  None."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "010 BOD 0  Halt and pulse BRKOUT Signal. BOD 1  Halt."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "011 BOD 0  Breakpoint trap and pulse BRKOUT Signal. BOD 1  Breakpoint trap."]
        pub const CONST_33: Self = Self::new(3);
        #[doc = "100 BOD 0  Breakpoint interrupt 0 and pulse BRKOUT Signal. BOD 1  Breakpoint interrupt 0."]
        pub const CONST_44: Self = Self::new(4);
        #[doc = "101 BOD 0  If implemented  breakpoint interrupt 1 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 1. If not implemented  None."]
        pub const CONST_55: Self = Self::new(5);
        #[doc = "110 BOD 0  If implemented  breakpoint interrupt 2 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 2. If not implemented  None."]
        pub const CONST_66: Self = Self::new(6);
        #[doc = "111 BOD 0  If implemented  breakpoint interrupt 3 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 3. If not implemented  None."]
        pub const CONST_77: Self = Self::new(7);
    }
    pub struct Bbm_SPEC;
    pub type Bbm = crate::EnumBitfieldStruct<u8, Bbm_SPEC>;
    impl Bbm {
        #[doc = "0 Break after make  BAM ."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Break before make  BBM ."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Bod_SPEC;
    pub type Bod = crate::EnumBitfieldStruct<u8, Bod_SPEC>;
    impl Bod {
        #[doc = "0 BRKOUT signal asserted according to the action specified in the EVTA field."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 BRKOUT signal not asserted. This takes priority over any assertion generated by the EVTA field."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Cnt_SPEC;
    pub type Cnt = crate::EnumBitfieldStruct<u8, Cnt_SPEC>;
    impl Cnt {
        #[doc = "00 No change."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "01 Start the performance counters."]
        pub const CONST_11: Self = Self::new(1);
        #[doc = "10 Stop the performance counters."]
        pub const CONST_22: Self = Self::new(2);
        #[doc = "11 Toggle the performance counter control  i.e. start it if it is currently stopped  stop it if it is currently running ."]
        pub const CONST_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscon_SPEC;
impl crate::sealed::RegSpec for Syscon_SPEC {
    type DataType = u32;
}
#[doc = "CPUx System Configuration Register\n resetvalue={Application Reset:0x0,Application Reset:0x0}"]
pub type Syscon = crate::RegValueT<Syscon_SPEC>;

impl Syscon {
    #[doc = "Free Context List Depleted Sticky Flag   FCDSF. This sticky bit indicates that a FCD  Free Context List Depleted  trap occurred since the bit was last cleared by software."]
    #[inline(always)]
    pub fn fcdsf(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, syscon::Fcdsf, Syscon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,syscon::Fcdsf, Syscon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Memory Protection Enable   PROTEN. Enables the memory protection system. Memory protection is controlled through the memory protection register        sets. Note  Initialize the protection register sets prior to setting        PROTEN to one."]
    #[inline(always)]
    pub fn proten(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, syscon::Proten, Syscon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,syscon::Proten, Syscon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Temporal Protection Enable   TPROTEN. Enable the Temporal Protection system."]
    #[inline(always)]
    pub fn tproten(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, syscon::Tproten, Syscon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,syscon::Tproten, Syscon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Initial State Interrupt   IS. of PSW.S bit in interrupt handle"]
    #[inline(always)]
    pub fn is(self) -> crate::common::RegisterFieldBool<3, 1, 0, Syscon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Syscon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Initial State Trap   TS. of PSW.S bit in trap handle"]
    #[inline(always)]
    pub fn ts(self) -> crate::common::RegisterFieldBool<4, 1, 0, Syscon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Syscon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emulator Space Disable. Disable the Emulator Space system"]
    #[inline(always)]
    pub fn esdis(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Syscon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Syscon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "User 1 Instruction execution disable   U1 IED. Disable the execution of User 1 mode instructions in User 1 IO mode. Disables User 1 ability to enable and  disable interrupts."]
    #[inline(always)]
    pub fn u1_ied(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Syscon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Syscon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "User 1 Peripheral access as supervisor   U1 IOS. Allow User 1 mode tasks to access peripherals as if in Supervisor mode. Enables User 1 access to all  peripheral registers."]
    #[inline(always)]
    pub fn u1_ios(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Syscon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Syscon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Boot Halt   BHALT"]
    #[inline(always)]
    pub fn bhalt(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, syscon::Bhalt, Syscon_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,syscon::Bhalt, Syscon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Syscon {
    #[inline(always)]
    fn default() -> Syscon {
        <crate::RegValueT<Syscon_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod syscon {
    pub struct Fcdsf_SPEC;
    pub type Fcdsf = crate::EnumBitfieldStruct<u8, Fcdsf_SPEC>;
    impl Fcdsf {
        #[doc = "0 No FCD trap occurred since the last clear."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 An FCD trap occurred since the last clear."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Proten_SPEC;
    pub type Proten = crate::EnumBitfieldStruct<u8, Proten_SPEC>;
    impl Proten {
        #[doc = "0 Memory Protection is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Memory Protection is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Tproten_SPEC;
    pub type Tproten = crate::EnumBitfieldStruct<u8, Tproten_SPEC>;
    impl Tproten {
        #[doc = "0 Temporal Protection is disabled."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Temporal Protection is enabled."]
        pub const CONST_11: Self = Self::new(1);
    }
    pub struct Bhalt_SPEC;
    pub type Bhalt = crate::EnumBitfieldStruct<u8, Bhalt_SPEC>;
    impl Bhalt {
        #[doc = "0 Core is not in boot halt."]
        pub const CONST_00: Self = Self::new(0);
        #[doc = "1 Core is in boot halt  write to 0 will exit"]
        pub const CONST_11: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TaskAsi_SPEC;
impl crate::sealed::RegSpec for TaskAsi_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Task Address Space Identifier Register\n resetvalue={Application Reset:0x1F}"]
pub type TaskAsi = crate::RegValueT<TaskAsi_SPEC>;

impl TaskAsi {
    #[doc = "Address Space Identifier   ASI. The ASI register contains the Address Space Identifier of the current process."]
    #[inline(always)]
    pub fn asi(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, TaskAsi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, TaskAsi_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for TaskAsi {
    #[inline(always)]
    fn default() -> TaskAsi {
        <crate::RegValueT<TaskAsi_SPEC> as RegisterValue<_>>::new(31)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrigAcc_SPEC;
impl crate::sealed::RegSpec for TrigAcc_SPEC {
    type DataType = u32;
}
#[doc = "CPUx TriggerAddressx\n resetvalue={Debug Reset:0x0}"]
pub type TrigAcc = crate::RegValueT<TrigAcc_SPEC>;

impl TrigAcc {
    #[doc = "Trigger 0   T0. active since last cleared"]
    #[inline(always)]
    pub fn t0(self) -> crate::common::RegisterFieldBool<0, 1, 0, TrigAcc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, TrigAcc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Trigger 1   T1. active since last cleared"]
    #[inline(always)]
    pub fn t1(self) -> crate::common::RegisterFieldBool<1, 1, 0, TrigAcc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, TrigAcc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Trigger 2   T2. active since last cleared"]
    #[inline(always)]
    pub fn t2(self) -> crate::common::RegisterFieldBool<2, 1, 0, TrigAcc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, TrigAcc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Trigger 3   T3. active since last cleared"]
    #[inline(always)]
    pub fn t3(self) -> crate::common::RegisterFieldBool<3, 1, 0, TrigAcc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, TrigAcc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Trigger 4   T4. active since last cleared"]
    #[inline(always)]
    pub fn t4(self) -> crate::common::RegisterFieldBool<4, 1, 0, TrigAcc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, TrigAcc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Trigger 5   T5. active since last cleared"]
    #[inline(always)]
    pub fn t5(self) -> crate::common::RegisterFieldBool<5, 1, 0, TrigAcc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, TrigAcc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Trigger 6   T6. active since last cleared"]
    #[inline(always)]
    pub fn t6(self) -> crate::common::RegisterFieldBool<6, 1, 0, TrigAcc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, TrigAcc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Trigger 7   T7. active since last cleared"]
    #[inline(always)]
    pub fn t7(self) -> crate::common::RegisterFieldBool<7, 1, 0, TrigAcc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, TrigAcc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for TrigAcc {
    #[inline(always)]
    fn default() -> TrigAcc {
        <crate::RegValueT<TrigAcc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc = "BLK"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Blk(pub(super) *mut u8);
unsafe impl core::marker::Send for Blk {}
unsafe impl core::marker::Sync for Blk {}
impl Blk {
    #[doc = "CPUx Overlay Mask Register 0\n resetvalue={Application Reset:0x0FFFFFE0}"]
    #[inline(always)]
    pub const fn omaski(&self) -> crate::common::Reg<blk::OmasKi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "CPUx Overlay Target Address Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn otari(&self) -> crate::common::Reg<blk::OtaRi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "CPUx Redirected Address Base Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rabri(&self) -> crate::common::Reg<blk::RabRi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod blk {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OmasKi_SPEC;
    impl crate::sealed::RegSpec for OmasKi_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Overlay Mask Register 0\n resetvalue={Application Reset:0x0FFFFFE0}"]
    pub type OmasKi = crate::RegValueT<OmasKi_SPEC>;

    impl OmasKi {
        #[doc = "Overlay Address Mask   OMASK. This bitfield determines the overlay block size and the bits used for address comparison and translation.  ...   Zero  bits determine the corresponding address bits which are not used in the address comparison and thus determine the block size  corresponding final address bits are derived from the original data address.  One  bits determine the corresponding address bits which are used for the address comparison  corresponding final address bits are derived from RABRx register in case of address match."]
        #[inline(always)]
        pub fn omask(
            self,
        ) -> crate::common::RegisterField<
            5,
            0xfff,
            1,
            0,
            omaski::Omask,
            OmasKi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0xfff,
                1,
                0,
                omaski::Omask,
                OmasKi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Fixed  1  Values   ONE. Corresponding address bits are participating in the address comparison. Corresponding final address bits are taken from RABRx."]
        #[inline(always)]
        pub fn one(
            self,
        ) -> crate::common::RegisterField<17, 0x7ff, 1, 0, u16, OmasKi_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<17,0x7ff,1,0,u16, OmasKi_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for OmasKi {
        #[inline(always)]
        fn default() -> OmasKi {
            <crate::RegValueT<OmasKi_SPEC> as RegisterValue<_>>::new(268435424)
        }
    }
    pub mod omaski {
        pub struct Omask_SPEC;
        pub type Omask = crate::EnumBitfieldStruct<u16, Omask_SPEC>;
        impl Omask {
            #[doc = "000000000000 B   128 Kbyte block size"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "100000000000 B   64 Kbyte block size"]
            pub const CONST_20482048: Self = Self::new(2048);
            #[doc = "110000000000 B   32 Kbyte block size"]
            pub const CONST_30723072: Self = Self::new(3072);
            #[doc = "111111111110 B   64 byte block size"]
            pub const CONST_40944094: Self = Self::new(4094);
            #[doc = "111111111111 B    xa0 32 byte block size"]
            pub const CONST_40954095: Self = Self::new(4095);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OtaRi_SPEC;
    impl crate::sealed::RegSpec for OtaRi_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Overlay Target Address Register 0\n resetvalue={Application Reset:0x0}"]
    pub type OtaRi = crate::RegValueT<OtaRi_SPEC>;

    impl OtaRi {
        #[doc = "Target Base   TBASE. This field holds the base address of the overlay memory block in the target memory. If the corresponding bit in OMASK register is set to one TBASE bit value is used in the address match. If the corresponding bit in OMASK register is set to zero TBASE bit value is ignored."]
        #[inline(always)]
        pub fn tbase(
            self,
        ) -> crate::common::RegisterField<5, 0x7fffff, 1, 0, u32, OtaRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x7fffff,1,0,u32, OtaRi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for OtaRi {
        #[inline(always)]
        fn default() -> OtaRi {
            <crate::RegValueT<OtaRi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RabRi_SPEC;
    impl crate::sealed::RegSpec for RabRi_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Redirected Address Base Register 0\n resetvalue={Application Reset:0x0}"]
    pub type RabRi = crate::RegValueT<RabRi_SPEC>;

    impl RabRi {
        #[doc = "Overlay Block Base Address   OBASE. Bits 21..5 of the base address the overlay memory block in the overlay memory. If the corresponding bit in OMASK register is set to one  OBASE bit value is used in the redirection address. If the corresponding bit in OMASK register is set to zero  OBASE bit value is ignored."]
        #[inline(always)]
        pub fn obase(
            self,
        ) -> crate::common::RegisterField<5, 0x1ffff, 1, 0, u32, RabRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x1ffff,1,0,u32, RabRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Overlay Memory Select   OMEM. Selects overlay memory used for redirection."]
        #[inline(always)]
        pub fn omem(
            self,
        ) -> crate::common::RegisterField<24, 0xf, 1, 0, rabri::Omem, RabRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0xf,1,0,rabri::Omem, RabRi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Overlay Enabled   OVEN. This bit controls whether the overlay function of overlay block x is enabled. This bit can also be changed when OVCCON.OVSTP or OVCCON.OVSTRT is set. See OVCCON register description."]
        #[inline(always)]
        pub fn oven(
            self,
        ) -> crate::common::RegisterField<31, 0x1, 1, 0, rabri::Oven, RabRi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<31,0x1,1,0,rabri::Oven, RabRi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for RabRi {
        #[inline(always)]
        fn default() -> RabRi {
            <crate::RegValueT<RabRi_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod rabri {
        pub struct Omem_SPEC;
        pub type Omem = crate::EnumBitfieldStruct<u8, Omem_SPEC>;
        impl Omem {
            #[doc = "0 Redirection to Core 0 DSPR PSPR memory"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Redirection to Core 1 DSPR PSPR memory"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "2 Redirection to Core 2 DSPR PSPR memory"]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "3 Redirection to Core 3 DSPR PSPR memory"]
            pub const CONST_33: Self = Self::new(3);
            #[doc = "4 Redirection to Core 4 DSPR PSPR memory"]
            pub const CONST_44: Self = Self::new(4);
            #[doc = "5 Redirection to Core 5 DSPR PSPR memory"]
            pub const CONST_55: Self = Self::new(5);
            #[doc = "8 Redirection to LMU"]
            pub const CONST_88: Self = Self::new(8);
            #[doc = "9 Redirection to EMEM"]
            pub const CONST_99: Self = Self::new(9);
            #[doc = "A Redirection to EBU"]
            pub const CONST_1010: Self = Self::new(10);
        }
        pub struct Oven_SPEC;
        pub type Oven = crate::EnumBitfieldStruct<u8, Oven_SPEC>;
        impl Oven {
            #[doc = "0 Overlay function of block x is disabled."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Overlay function of block x is enabled."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
}
#[doc = "CPR"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpr(pub(super) *mut u8);
unsafe impl core::marker::Send for Cpr {}
unsafe impl core::marker::Sync for Cpr {}
impl Cpr {
    #[doc = "CPUx Code Protection Range 0 Lower Bound Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cpry_l(&self) -> crate::common::Reg<cpr::CpRyL_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "CPUx Code Protection Range 0 Upper Bound Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cpry_u(&self) -> crate::common::Reg<cpr::CpRyU_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
}
pub mod cpr {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpRyL_SPEC;
    impl crate::sealed::RegSpec for CpRyL_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Code Protection Range 0 Lower Bound Register\n resetvalue={Application Reset:0x0}"]
    pub type CpRyL = crate::RegValueT<CpRyL_SPEC>;

    impl CpRyL {
        #[doc = "CPRy Lower Boundary Address   LOWBND"]
        #[inline(always)]
        pub fn lowbnd(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, CpRyL_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x7ffffff,1,0,u32, CpRyL_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for CpRyL {
        #[inline(always)]
        fn default() -> CpRyL {
            <crate::RegValueT<CpRyL_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpRyU_SPEC;
    impl crate::sealed::RegSpec for CpRyU_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Code Protection Range 0 Upper Bound Register\n resetvalue={Application Reset:0x0}"]
    pub type CpRyU = crate::RegValueT<CpRyU_SPEC>;

    impl CpRyU {
        #[doc = "CPR0 m Upper Boundary Address   UPPBND"]
        #[inline(always)]
        pub fn uppbnd(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, CpRyU_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x7ffffff,1,0,u32, CpRyU_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for CpRyU {
        #[inline(always)]
        fn default() -> CpRyU {
            <crate::RegValueT<CpRyU_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "DPR"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpr(pub(super) *mut u8);
unsafe impl core::marker::Send for Dpr {}
unsafe impl core::marker::Sync for Dpr {}
impl Dpr {
    #[doc = "CPUx Data Protection Range 0  Lower Bound Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpry_l(&self) -> crate::common::Reg<dpr::DpRyL_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "CPUx Data Protection Range 0  Upper Bound Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpry_u(&self) -> crate::common::Reg<dpr::DpRyU_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
}
pub mod dpr {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DpRyL_SPEC;
    impl crate::sealed::RegSpec for DpRyL_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Data Protection Range 0  Lower Bound Register\n resetvalue={Application Reset:0x0}"]
    pub type DpRyL = crate::RegValueT<DpRyL_SPEC>;

    impl DpRyL {
        #[doc = "DPRy Lower Boundary Address   LOWBND"]
        #[inline(always)]
        pub fn lowbnd(
            self,
        ) -> crate::common::RegisterField<3, 0x1fffffff, 1, 0, u32, DpRyL_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<3,0x1fffffff,1,0,u32, DpRyL_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for DpRyL {
        #[inline(always)]
        fn default() -> DpRyL {
            <crate::RegValueT<DpRyL_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DpRyU_SPEC;
    impl crate::sealed::RegSpec for DpRyU_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Data Protection Range 0  Upper Bound Register\n resetvalue={Application Reset:0x0}"]
    pub type DpRyU = crate::RegValueT<DpRyU_SPEC>;

    impl DpRyU {
        #[doc = "DPRy Upper Boundary Address   UPPBND"]
        #[inline(always)]
        pub fn uppbnd(
            self,
        ) -> crate::common::RegisterField<3, 0x1fffffff, 1, 0, u32, DpRyU_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<3,0x1fffffff,1,0,u32, DpRyU_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for DpRyU {
        #[inline(always)]
        fn default() -> DpRyU {
            <crate::RegValueT<DpRyU_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "FPU TRAP"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FpuTrap(pub(super) *mut u8);
unsafe impl core::marker::Send for FpuTrap {}
unsafe impl core::marker::Sync for FpuTrap {}
impl FpuTrap {
    #[doc = "CPUx Trap Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn con(&self) -> crate::common::Reg<fpu_trap::Con_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "CPUx Trapping Instruction Opcode Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn opc(&self) -> crate::common::Reg<fpu_trap::Opc_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "CPUx Trapping Instruction Program Counter Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pc(&self) -> crate::common::Reg<fpu_trap::Pc_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "CPUx Trapping Instruction Operand Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn src1(&self) -> crate::common::Reg<fpu_trap::Src1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "CPUx Trapping Instruction Operand Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn src2(&self) -> crate::common::Reg<fpu_trap::Src2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "CPUx Trapping Instruction Operand Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn src3(&self) -> crate::common::Reg<fpu_trap::Src3_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
}
pub mod fpu_trap {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Con_SPEC;
    impl crate::sealed::RegSpec for Con_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Trap Control Register\n resetvalue={Application Reset:0x0}"]
    pub type Con = crate::RegValueT<Con_SPEC>;

    impl Con {
        #[doc = "Trap Status   TST"]
        #[inline(always)]
        pub fn tst(
            self,
        ) -> crate::common::RegisterField<0, 0x1, 1, 0, con::Tst, Con_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0x1,1,0,con::Tst, Con_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Trap Clear   TCL. Read  always reads as 0."]
        #[inline(always)]
        pub fn tcl(
            self,
        ) -> crate::common::RegisterField<1, 0x1, 1, 0, con::Tcl, Con_SPEC, crate::common::W>
        {
            crate::common::RegisterField::<1,0x1,1,0,con::Tcl, Con_SPEC,crate::common::W>::from_register(self,0)
        }
        #[doc = "Captured Rounding Mode   RM. The rounding mode of the captured instruction. Only valid when TST is asserted. Note that this is the rounding mode supplied to the FPU for the exceptional instruction. UPDFL instructions may cause a trap and change the rounding mode. In this case the RM bits capture the input rounding mode"]
        #[inline(always)]
        pub fn rm(
            self,
        ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Con_SPEC, crate::common::R> {
            crate::common::RegisterField::<8,0x3,1,0,u8, Con_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "FX Trap Enable   FXE. When set  an instruction generating an FX exception will trigger a trap."]
        #[inline(always)]
        pub fn fxe(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, Con_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<18, 1, 0, Con_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "FU Trap Enable   FUE. When set  an instruction generating an FU exception will trigger a trap."]
        #[inline(always)]
        pub fn fue(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, Con_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<19, 1, 0, Con_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "FZ Trap Enable   FZE. When set  an instruction generating an FZ exception will trigger a trap."]
        #[inline(always)]
        pub fn fze(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, Con_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<20, 1, 0, Con_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "FV Trap Enable   FVE. When set  an instruction generating an FV exception will trigger a trap."]
        #[inline(always)]
        pub fn fve(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, Con_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<21, 1, 0, Con_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "FI Trap Enable   FIE. When set  an instruction generating an FI exception will trigger a trap."]
        #[inline(always)]
        pub fn fie(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, Con_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<22, 1, 0, Con_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Captured FX   FX. Asserted if the captured instruction asserted FX. Only valid when TST is asserted."]
        #[inline(always)]
        pub fn fx(self) -> crate::common::RegisterFieldBool<26, 1, 0, Con_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<26, 1, 0, Con_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Captured FU   FU. Asserted if the captured instruction asserted FU. Only valid when TST is asserted."]
        #[inline(always)]
        pub fn fu(self) -> crate::common::RegisterFieldBool<27, 1, 0, Con_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<27, 1, 0, Con_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Captured FZ   FZ. Asserted if the captured instruction asserted FZ. Only valid when TST is asserted"]
        #[inline(always)]
        pub fn fz(self) -> crate::common::RegisterFieldBool<28, 1, 0, Con_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<28, 1, 0, Con_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Captured FV   FV. Asserted if the captured instruction asserted FV. Only valid when TST is asserted"]
        #[inline(always)]
        pub fn fv(self) -> crate::common::RegisterFieldBool<29, 1, 0, Con_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<29, 1, 0, Con_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Captured FI   FI. Asserted if the captured instruction asserted FI. Only valid when TST is asserted"]
        #[inline(always)]
        pub fn fi(self) -> crate::common::RegisterFieldBool<30, 1, 0, Con_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<30, 1, 0, Con_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
    }
    impl core::default::Default for Con {
        #[inline(always)]
        fn default() -> Con {
            <crate::RegValueT<Con_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod con {
        pub struct Tst_SPEC;
        pub type Tst = crate::EnumBitfieldStruct<u8, Tst_SPEC>;
        impl Tst {
            #[doc = "0 No instruction captured.  The next enabled exception will cause the exceptional instruction to be captured."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Instruction captured. No further enabled exceptions will be captured until TST is cleared."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Tcl_SPEC;
        pub type Tcl = crate::EnumBitfieldStruct<u8, Tcl_SPEC>;
        impl Tcl {
            #[doc = "0 No effect."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Clears the trapped instruction  TST will be negated ."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Opc_SPEC;
    impl crate::sealed::RegSpec for Opc_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Trapping Instruction Opcode Register\n resetvalue={Application Reset:0x0}"]
    pub type Opc = crate::RegValueT<Opc_SPEC>;

    impl Opc {
        #[doc = "Captured Opcode   OPC. The secondary opcode of the captured instruction. When FPU TRAP OPC.FMT 0 only bits  3 0  are defined. OPC is valid only when FPU TRAP CON.TST is asserted."]
        #[inline(always)]
        pub fn opc(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Opc_SPEC, crate::common::R> {
            crate::common::RegisterField::<0,0xff,1,0,u8, Opc_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Captured Instruction Format   FMT. The format of the captured instruction s opcode. Only valid when FPU TRAP CON.TST is asserted."]
        #[inline(always)]
        pub fn fmt(
            self,
        ) -> crate::common::RegisterField<8, 0x1, 1, 0, opc::Fmt, Opc_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<8,0x1,1,0,opc::Fmt, Opc_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Captured Destination Register   DREG. The destination register of the captured instruction. ... Only valid when FPU TRAP CON.TST is asserted."]
        #[inline(always)]
        pub fn dreg(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, opc::Dreg, Opc_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0xf,1,0,opc::Dreg, Opc_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for Opc {
        #[inline(always)]
        fn default() -> Opc {
            <crate::RegValueT<Opc_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod opc {
        pub struct Fmt_SPEC;
        pub type Fmt = crate::EnumBitfieldStruct<u8, Fmt_SPEC>;
        impl Fmt {
            #[doc = "0 RRR"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 RR"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Dreg_SPEC;
        pub type Dreg = crate::EnumBitfieldStruct<u8, Dreg_SPEC>;
        impl Dreg {
            #[doc = "0 Data general purpose register 0."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "F Data general purpose register 15."]
            pub const CONST_1515: Self = Self::new(15);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pc_SPEC;
    impl crate::sealed::RegSpec for Pc_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Trapping Instruction Program Counter Register\n resetvalue={Application Reset:0x0}"]
    pub type Pc = crate::RegValueT<Pc_SPEC>;

    impl Pc {
        #[doc = "Captured Program Counter   PC. The program counter  virtual address  of the captured instruction. Only valid when FPU TRAP CON.TST is asserted."]
        #[inline(always)]
        pub fn pc(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Pc_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, Pc_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for Pc {
        #[inline(always)]
        fn default() -> Pc {
            <crate::RegValueT<Pc_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Src1_SPEC;
    impl crate::sealed::RegSpec for Src1_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Trapping Instruction Operand Register\n resetvalue={Application Reset:0x0}"]
    pub type Src1 = crate::RegValueT<Src1_SPEC>;

    impl Src1 {
        #[doc = "Captured SRC1 Operand   SRC1. The SRC1 operand of the captured instruction. Only valid when FPU TRAP CON.TST is asserted."]
        #[inline(always)]
        pub fn src1(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Src1_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, Src1_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for Src1 {
        #[inline(always)]
        fn default() -> Src1 {
            <crate::RegValueT<Src1_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Src2_SPEC;
    impl crate::sealed::RegSpec for Src2_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Trapping Instruction Operand Register\n resetvalue={Application Reset:0x0}"]
    pub type Src2 = crate::RegValueT<Src2_SPEC>;

    impl Src2 {
        #[doc = "Captured SRC2 Operand   SRC2. The SRC2 operand of the captured instruction. Only valid when FPU TRAP CON.TST is asserted."]
        #[inline(always)]
        pub fn src2(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Src2_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, Src2_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for Src2 {
        #[inline(always)]
        fn default() -> Src2 {
            <crate::RegValueT<Src2_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Src3_SPEC;
    impl crate::sealed::RegSpec for Src3_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Trapping Instruction Operand Register\n resetvalue={Application Reset:0x0}"]
    pub type Src3 = crate::RegValueT<Src3_SPEC>;

    impl Src3 {
        #[doc = "Captured SRC3 Operand   SRC3. The SRC3 operand of the captured instruction. Only valid when FPU TRAP CON.TST is asserted."]
        #[inline(always)]
        pub fn src3(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Src3_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, Src3_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for Src3 {
        #[inline(always)]
        fn default() -> Src3 {
            <crate::RegValueT<Src3_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "RGN"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rgn(pub(super) *mut u8);
unsafe impl core::marker::Send for Rgn {}
unsafe impl core::marker::Sync for Rgn {}
impl Rgn {
    #[doc = "CPUx  Safety Protection Region SPR Write Access Enable Register A0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn spr_sprot_rgnaccenai_w(
        &self,
    ) -> crate::common::Reg<rgn::SprSprotRgnaccenAiW_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "CPUx  Safety Protection Region SPR Write Access Enable Register B0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn spr_sprot_rgnaccenbi_w(
        &self,
    ) -> crate::common::Reg<rgn::SprSprotRgnaccenBiW_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "CPUx  Safety Protection SPR Region Lower Address Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn spr_sprot_rgnlai(
        &self,
    ) -> crate::common::Reg<rgn::SprSprotRgnlAi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "CPUx  Safety protection SPR Region Upper Address Register 0\n resetvalue={Application Reset:0x0FFFFFFE0}"]
    #[inline(always)]
    pub const fn spr_sprot_rgnuai(
        &self,
    ) -> crate::common::Reg<rgn::SprSprotRgnuAi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
}
pub mod rgn {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SprSprotRgnaccenAiW_SPEC;
    impl crate::sealed::RegSpec for SprSprotRgnaccenAiW_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx  Safety Protection Region SPR Write Access Enable Register A0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type SprSprotRgnaccenAiW = crate::RegValueT<SprSprotRgnaccenAiW_SPEC>;

    impl SprSprotRgnaccenAiW {
        #[doc = "Write Access Enable for Master TAG ID n  n 0 31    EN. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            spr_sprot_rgnaccenai_w::En,
            SprSprotRgnaccenAiW_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                spr_sprot_rgnaccenai_w::En,
                SprSprotRgnaccenAiW_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for SprSprotRgnaccenAiW {
        #[inline(always)]
        fn default() -> SprSprotRgnaccenAiW {
            <crate::RegValueT<SprSprotRgnaccenAiW_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
    pub mod spr_sprot_rgnaccenai_w {
        pub struct En_SPEC;
        pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
        impl En {
            #[doc = "0 Write access will not be executed"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Write access will be executed"]
            pub const CONST_11: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SprSprotRgnaccenBiW_SPEC;
    impl crate::sealed::RegSpec for SprSprotRgnaccenBiW_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx  Safety Protection Region SPR Write Access Enable Register B0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type SprSprotRgnaccenBiW = crate::RegValueT<SprSprotRgnaccenBiW_SPEC>;

    impl SprSprotRgnaccenBiW {
        #[doc = "Write Access Enable for Master TAG ID n  n 32 63    EN. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            SprSprotRgnaccenBiW_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                SprSprotRgnaccenBiW_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for SprSprotRgnaccenBiW {
        #[inline(always)]
        fn default() -> SprSprotRgnaccenBiW {
            <crate::RegValueT<SprSprotRgnaccenBiW_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SprSprotRgnlAi_SPEC;
    impl crate::sealed::RegSpec for SprSprotRgnlAi_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx  Safety Protection SPR Region Lower Address Register 0\n resetvalue={Application Reset:0x0}"]
    pub type SprSprotRgnlAi = crate::RegValueT<SprSprotRgnlAi_SPEC>;

    impl SprSprotRgnlAi {
        #[doc = "Region Lower Address   ADDR. Bits 31 to 5 of the address which is the lower bound of the defined memory region"]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            SprSprotRgnlAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0x7ffffff,
                1,
                0,
                u32,
                SprSprotRgnlAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for SprSprotRgnlAi {
        #[inline(always)]
        fn default() -> SprSprotRgnlAi {
            <crate::RegValueT<SprSprotRgnlAi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SprSprotRgnuAi_SPEC;
    impl crate::sealed::RegSpec for SprSprotRgnuAi_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx  Safety protection SPR Region Upper Address Register 0\n resetvalue={Application Reset:0x0FFFFFFE0}"]
    pub type SprSprotRgnuAi = crate::RegValueT<SprSprotRgnuAi_SPEC>;

    impl SprSprotRgnuAi {
        #[doc = "Region Upper Address   ADDR. Bits 31 to 5 of the address which is the upper bound of the defined memory region"]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x7ffffff,
            1,
            0,
            u32,
            SprSprotRgnuAi_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0x7ffffff,
                1,
                0,
                u32,
                SprSprotRgnuAi_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl core::default::Default for SprSprotRgnuAi {
        #[inline(always)]
        fn default() -> SprSprotRgnuAi {
            <crate::RegValueT<SprSprotRgnuAi_SPEC> as RegisterValue<_>>::new(4294967264)
        }
    }
}
#[doc = "TPS"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tps(pub(super) *mut u8);
unsafe impl core::marker::Send for Tps {}
unsafe impl core::marker::Sync for Tps {}
impl Tps {
    #[doc = "CPUx Temporal Protection System Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn con(&self) -> crate::common::Reg<tps::Con_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "CPUx Temporal Protection System Timer Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn timer(&self) -> [crate::common::Reg<tps::Timer_SPEC, crate::common::RW>; 3] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.0.add(0x4usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.0.add(0x4usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.0.add(0x4usize + 0x8usize)),
            ]
        }
    }
}
pub mod tps {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Con_SPEC;
    impl crate::sealed::RegSpec for Con_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Temporal Protection System Control Register\n resetvalue={Application Reset:0x0}"]
    pub type Con = crate::RegValueT<Con_SPEC>;

    impl Con {
        #[doc = "Timer0 Expired Flag   TEXP0. Set when the corresponding timer expires. Cleared on any write to the  TIMER0 register."]
        #[inline(always)]
        pub fn texp0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, Con_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0, 1, 0, Con_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Timer1 Expired Flag   TEXP1. Set when the corresponding timer expires. Cleared on any write to the  TIMER1 register."]
        #[inline(always)]
        pub fn texp1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, Con_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<1, 1, 0, Con_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Timer1 Expired Flag   TEXP2. Set when the corresponding timer expires. Cleared on any write to the  TIMER1 register."]
        #[inline(always)]
        pub fn texp2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, Con_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<2, 1, 0, Con_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Temporal Protection Trap   TTRAP. If set  indicates that a TAE trap has been requested. Any subsequent TAE traps are disabled. A write clears the flag and re enables TAE traps."]
        #[inline(always)]
        pub fn ttrap(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, Con_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<16, 1, 0, Con_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
    }
    impl core::default::Default for Con {
        #[inline(always)]
        fn default() -> Con {
            <crate::RegValueT<Con_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer_SPEC;
    impl crate::sealed::RegSpec for Timer_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Temporal Protection System Timer Register 0\n resetvalue={Application Reset:0x0}"]
    pub type Timer = crate::RegValueT<Timer_SPEC>;

    impl Timer {
        #[doc = "Temporal Protection Timer   Timer. Writing zero de activates the Timer. Writing a non zero value starts the Timer. Any write clears the corresponding TPS CON.TEXP flag. Read returns the current Timer value."]
        #[inline(always)]
        pub fn timer(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Timer_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, Timer_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Timer {
        #[inline(always)]
        fn default() -> Timer {
            <crate::RegValueT<Timer_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "TPS EXTIM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TpsExtim(pub(super) *mut u8);
unsafe impl core::marker::Send for TpsExtim {}
unsafe impl core::marker::Sync for TpsExtim {}
impl TpsExtim {
    #[doc = "CPUx Exception Timer Class Enable Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn class_en(&self) -> crate::common::Reg<tps_extim::ClassEn_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "CPUx Exception Entry Timer Current Value\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn entry_cval(
        &self,
    ) -> crate::common::Reg<tps_extim::EntryCval_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "CPUx Exception Entry Timer Load Value\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn entry_lval(
        &self,
    ) -> crate::common::Reg<tps_extim::EntryLval_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "CPUx Exception Exit Timer Current Value\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn exit_cval(
        &self,
    ) -> crate::common::Reg<tps_extim::ExitCval_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "CPUx Exception Exit  Timer Load Value\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn exit_lval(
        &self,
    ) -> crate::common::Reg<tps_extim::ExitLval_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "CPUx Exception Timer FCX Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcx(&self) -> crate::common::Reg<tps_extim::Fcx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "CPUx Exception Timer Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn stat(&self) -> crate::common::Reg<tps_extim::Stat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
}
pub mod tps_extim {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClassEn_SPEC;
    impl crate::sealed::RegSpec for ClassEn_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Exception Timer Class Enable Register\n resetvalue={Application Reset:0x0}"]
    pub type ClassEn = crate::RegValueT<ClassEn_SPEC>;

    impl ClassEn {
        #[doc = "Exception Timer Class Enables   EXTIM CLASS EN. Trap Class enables for exception timer."]
        #[inline(always)]
        pub fn extim_class_en(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, ClassEn_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xff,1,0,u8, ClassEn_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for ClassEn {
        #[inline(always)]
        fn default() -> ClassEn {
            <crate::RegValueT<ClassEn_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EntryCval_SPEC;
    impl crate::sealed::RegSpec for EntryCval_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Exception Entry Timer Current Value\n resetvalue={Application Reset:0x0}"]
    pub type EntryCval = crate::RegValueT<EntryCval_SPEC>;

    impl EntryCval {
        #[doc = "Exception Entry Timer Current Value   ENTRY CVAL. Current value of the exception entry timer."]
        #[inline(always)]
        pub fn entry_cval(
            self,
        ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, EntryCval_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xfff,1,0,u16, EntryCval_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for EntryCval {
        #[inline(always)]
        fn default() -> EntryCval {
            <crate::RegValueT<EntryCval_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EntryLval_SPEC;
    impl crate::sealed::RegSpec for EntryLval_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Exception Entry Timer Load Value\n resetvalue={Application Reset:0x0}"]
    pub type EntryLval = crate::RegValueT<EntryLval_SPEC>;

    impl EntryLval {
        #[doc = "Exception Entry Timer Load value   ENTRY LVAL. Value loaded into the exception entry timer on detection of an enabled exception. Bits  3 0  are constrained to be 0"]
        #[inline(always)]
        pub fn entry_lval(
            self,
        ) -> crate::common::RegisterField<4, 0xff, 1, 0, u8, EntryLval_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0xff,1,0,u8, EntryLval_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for EntryLval {
        #[inline(always)]
        fn default() -> EntryLval {
            <crate::RegValueT<EntryLval_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExitCval_SPEC;
    impl crate::sealed::RegSpec for ExitCval_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Exception Exit Timer Current Value\n resetvalue={Application Reset:0x0}"]
    pub type ExitCval = crate::RegValueT<ExitCval_SPEC>;

    impl ExitCval {
        #[doc = "Exception Exit Timer Current Value   EXIT CVAL. Current value of the exception exit timer."]
        #[inline(always)]
        pub fn exit_cval(
            self,
        ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, ExitCval_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffff,1,0,u32, ExitCval_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for ExitCval {
        #[inline(always)]
        fn default() -> ExitCval {
            <crate::RegValueT<ExitCval_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExitLval_SPEC;
    impl crate::sealed::RegSpec for ExitLval_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Exception Exit  Timer Load Value\n resetvalue={Application Reset:0x0}"]
    pub type ExitLval = crate::RegValueT<ExitLval_SPEC>;

    impl ExitLval {
        #[doc = "Exception Exit Timer Load value   EXIT LVAL. Value loaded into the exception exit timer on detection of an enabled exception. Bits  3 0  are constrained to be 0"]
        #[inline(always)]
        pub fn exit_lval(
            self,
        ) -> crate::common::RegisterField<4, 0xfffff, 1, 0, u32, ExitLval_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0xfffff,1,0,u32, ExitLval_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for ExitLval {
        #[inline(always)]
        fn default() -> ExitLval {
            <crate::RegValueT<ExitLval_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcx_SPEC;
    impl crate::sealed::RegSpec for Fcx_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Exception Timer FCX Register\n resetvalue={Application Reset:0x0}"]
    pub type Fcx = crate::RegValueT<Fcx_SPEC>;

    impl Fcx {
        #[doc = "Exception Exit Timer FCX   EXIT FCX. Exception Exit Timer FCX of triggering trap."]
        #[inline(always)]
        pub fn exit_fcx(
            self,
        ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, Fcx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xfffff,1,0,u32, Fcx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl core::default::Default for Fcx {
        #[inline(always)]
        fn default() -> Fcx {
            <crate::RegValueT<Fcx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Stat_SPEC;
    impl crate::sealed::RegSpec for Stat_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Exception Timer Status Register\n resetvalue={Application Reset:0x0}"]
    pub type Stat = crate::RegValueT<Stat_SPEC>;

    impl Stat {
        #[doc = "Exception Exit Timer TIN   EXIT TIN. Exception Exit Timer TIN of triggering trap."]
        #[inline(always)]
        pub fn exit_tin(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Stat_SPEC, crate::common::RW> {
            crate::common::RegisterField::<0,0xff,1,0,u8, Stat_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Exception Exit Timer Class   EXIT CLASS. Exception exit Timer Class of triggering trap."]
        #[inline(always)]
        pub fn exit_class(
            self,
        ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Stat_SPEC, crate::common::RW> {
            crate::common::RegisterField::<8,0x7,1,0,u8, Stat_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Exception Exit Timer Alarm Triggered   EXIT AT. Exception Exit Timer Alarm triggered sticky bit. Alarm triggered since last cleared."]
        #[inline(always)]
        pub fn exit_at(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, Stat_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<15, 1, 0, Stat_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Exception Entry Timer TIN   ENTRY TIN. Exception Entry Timer TIN of triggering trap."]
        #[inline(always)]
        pub fn entry_tin(
            self,
        ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Stat_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xff,1,0,u8, Stat_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Exception Entry Timer Class   ENTRY CLASS. Exception Entry Timer Class of triggering trap."]
        #[inline(always)]
        pub fn entry_class(
            self,
        ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Stat_SPEC, crate::common::RW> {
            crate::common::RegisterField::<24,0x7,1,0,u8, Stat_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Exception Entry Timer Alarm Triggered   ENTRY AT. Exception Entry Timer Alarm triggered sticky bit. Alarm triggered since last cleared."]
        #[inline(always)]
        pub fn entry_at(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, Stat_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<31, 1, 0, Stat_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
    }
    impl core::default::Default for Stat {
        #[inline(always)]
        fn default() -> Stat {
            <crate::RegValueT<Stat_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "Trigger"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tr(pub(super) *mut u8);
unsafe impl core::marker::Send for Tr {}
unsafe impl core::marker::Sync for Tr {}
impl Tr {
    #[doc = "CPUx Trigger Address 0\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn triadr(&self) -> crate::common::Reg<tr::TRiAdr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "CPUx Trigger Event 0\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn trievt(&self) -> crate::common::Reg<tr::TRiEvt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod tr {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TRiAdr_SPEC;
    impl crate::sealed::RegSpec for TRiAdr_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Trigger Address 0\n resetvalue={Debug Reset:0x0}"]
    pub type TRiAdr = crate::RegValueT<TRiAdr_SPEC>;

    impl TRiAdr {
        #[doc = "Comparison Address   ADDR. For PC comparison  bit 0  is always zero."]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, TRiAdr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, TRiAdr_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for TRiAdr {
        #[inline(always)]
        fn default() -> TRiAdr {
            <crate::RegValueT<TRiAdr_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TRiEvt_SPEC;
    impl crate::sealed::RegSpec for TRiEvt_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Trigger Event 0\n resetvalue={Debug Reset:0x0}"]
    pub type TRiEvt = crate::RegValueT<TRiEvt_SPEC>;

    impl TRiEvt {
        #[doc = "Event Associated   EVTA. Specifies the Debug Action associated with the Debug Event"]
        #[inline(always)]
        pub fn evta(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, trievt::Evta, TRiEvt_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7,1,0,trievt::Evta, TRiEvt_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Break Before Make  BBM  or Break After Make  BAM  Selection   BBM. Code triggers BBM or BAM selection. Data access and data code combination access triggers can only create BAM Debug Events. When these triggers occur  TRnEVT.BBM is ignored."]
        #[inline(always)]
        pub fn bbm(
            self,
        ) -> crate::common::RegisterField<3, 0x1, 1, 0, trievt::Bbm, TRiEvt_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<3,0x1,1,0,trievt::Bbm, TRiEvt_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Breakout Disable   BOD"]
        #[inline(always)]
        pub fn bod(
            self,
        ) -> crate::common::RegisterField<4, 0x1, 1, 0, trievt::Bod, TRiEvt_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x1,1,0,trievt::Bod, TRiEvt_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "CDC Suspend Out Signal State   SUSP. Value to be assigned to the CDC suspend out signal when the Debug Event is raised."]
        #[inline(always)]
        pub fn susp(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, TRiEvt_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,TRiEvt_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Counter   CNT. When this event occurs adjust the control of the performance counters in task mode as follows"]
        #[inline(always)]
        pub fn cnt(
            self,
        ) -> crate::common::RegisterField<6, 0x3, 1, 0, trievt::Cnt, TRiEvt_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<6,0x3,1,0,trievt::Cnt, TRiEvt_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Input Selection   TYP"]
        #[inline(always)]
        pub fn typ(
            self,
        ) -> crate::common::RegisterField<12, 0x1, 1, 0, trievt::Typ, TRiEvt_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0x1,1,0,trievt::Typ, TRiEvt_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Compare Type   RNG. Once an even numbered comparator has been set to range  the EVTR settings of its associated upper neighbour will be ignored."]
        #[inline(always)]
        pub fn rng(
            self,
        ) -> crate::common::RegisterField<13, 0x1, 1, 0, trievt::Rng, TRiEvt_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<13,0x1,1,0,trievt::Rng, TRiEvt_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable ASI Comparison   ASI EN"]
        #[inline(always)]
        pub fn asi_en(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            trievt::AsiEn,
            TRiEvt_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                trievt::AsiEn,
                TRiEvt_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Address Space Identifier   ASI. The ASI of the Debug Trigger process."]
        #[inline(always)]
        pub fn asi(
            self,
        ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, TRiEvt_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1f,1,0,u8, TRiEvt_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Address Store   AST. Used in conjunction with TYP 0"]
        #[inline(always)]
        pub fn ast(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, TRiEvt_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<27,1,0,TRiEvt_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Address Load   ALD. Used in conjunction with TYP 0"]
        #[inline(always)]
        pub fn ald(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, TRiEvt_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<28,1,0,TRiEvt_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for TRiEvt {
        #[inline(always)]
        fn default() -> TRiEvt {
            <crate::RegValueT<TRiEvt_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod trievt {
        pub struct Evta_SPEC;
        pub type Evta = crate::EnumBitfieldStruct<u8, Evta_SPEC>;
        impl Evta {
            #[doc = "000 BOD 0  Disabled. BOD 1  Disabled."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "001 BOD 0  Pulse BRKOUT Signal. BOD 1  None."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "010 BOD 0  Halt and pulse BRKOUT Signal. BOD 1  Halt."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "011 BOD 0  Breakpoint trap and pulse BRKOUT Signal. BOD 1  Breakpoint trap."]
            pub const CONST_33: Self = Self::new(3);
            #[doc = "100 BOD 0  Breakpoint interrupt 0 and pulse BRKOUT Signal. BOD 1  Breakpoint interrupt 0."]
            pub const CONST_44: Self = Self::new(4);
            #[doc = "101 BOD 0  If implemented  breakpoint interrupt 1 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 1. If not implemented  None."]
            pub const CONST_55: Self = Self::new(5);
            #[doc = "110 BOD 0  If implemented  breakpoint interrupt 2 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 2. If not implemented  None."]
            pub const CONST_66: Self = Self::new(6);
            #[doc = "111 BOD 0  If implemented  breakpoint interrupt 3 and pulse BRKOUT Signal. BOD 1  If implemented  breakpoint interrupt 3. If not implemented  None."]
            pub const CONST_77: Self = Self::new(7);
        }
        pub struct Bbm_SPEC;
        pub type Bbm = crate::EnumBitfieldStruct<u8, Bbm_SPEC>;
        impl Bbm {
            #[doc = "0 Code only triggers Break After Make  BAM ."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Code only triggers Break Before Make  BBM ."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Bod_SPEC;
        pub type Bod = crate::EnumBitfieldStruct<u8, Bod_SPEC>;
        impl Bod {
            #[doc = "0 BRKOUT signal asserted according to the action specified in the EVTA field."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 BRKOUT signal not asserted. This takes priority over any assertion generated by the EVTA field."]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Cnt_SPEC;
        pub type Cnt = crate::EnumBitfieldStruct<u8, Cnt_SPEC>;
        impl Cnt {
            #[doc = "00 No change."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "01 Start the performance counters."]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "10 Stop the performance counters."]
            pub const CONST_22: Self = Self::new(2);
            #[doc = "11 Toggle the performance counter control  i.e. start it if it is currently stopped  stop it if it is currently running ."]
            pub const CONST_33: Self = Self::new(3);
        }
        pub struct Typ_SPEC;
        pub type Typ = crate::EnumBitfieldStruct<u8, Typ_SPEC>;
        impl Typ {
            #[doc = "0 Address"]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 PC"]
            pub const CONST_11: Self = Self::new(1);
        }
        pub struct Rng_SPEC;
        pub type Rng = crate::EnumBitfieldStruct<u8, Rng_SPEC>;
        impl Rng {
            #[doc = "1 Range"]
            pub const CONST_11: Self = Self::new(1);
            #[doc = "0 Equality"]
            pub const CONST_00: Self = Self::new(0);
        }
        pub struct AsiEn_SPEC;
        pub type AsiEn = crate::EnumBitfieldStruct<u8, AsiEn_SPEC>;
        impl AsiEn {
            #[doc = "0 No ASI comparison performed. Debug Trigger is valid for all processes."]
            pub const CONST_00: Self = Self::new(0);
            #[doc = "1 Enable ASI comparison. Debug Events are only triggered when the current process ASI matches TRnEVT.ASI."]
            pub const CONST_11: Self = Self::new(1);
        }
    }
}
