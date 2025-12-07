#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmuc {
    ptr: *mut u8,
}
unsafe impl Send for Pmuc {}
unsafe impl Sync for Pmuc {}
impl Pmuc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Wakeup Enable register"]
    #[inline(always)]
    pub const fn wer(self) -> crate::common::Reg<regs::Wer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Wakeup Status register"]
    #[inline(always)]
    pub const fn wsr(self) -> crate::common::Reg<regs::Wsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Wakeup Clear register"]
    #[inline(always)]
    pub const fn wcr(self) -> crate::common::Reg<regs::Wcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "VRTC Control Register"]
    #[inline(always)]
    pub const fn vrtc_cr(self) -> crate::common::Reg<regs::VrtcCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "VRET Control Register"]
    #[inline(always)]
    pub const fn vret_cr(self) -> crate::common::Reg<regs::VretCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "RC10K Control Register"]
    #[inline(always)]
    pub const fn lrc10_cr(self) -> crate::common::Reg<regs::Lrc10Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "RC32K Control Register"]
    #[inline(always)]
    pub const fn lrc32_cr(self) -> crate::common::Reg<regs::Lrc32Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "XTAL32K Control Register"]
    #[inline(always)]
    pub const fn lxt_cr(self) -> crate::common::Reg<regs::LxtCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "AON Bandgap Register"]
    #[inline(always)]
    pub const fn aon_bg(self) -> crate::common::Reg<regs::AonBg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "AON LDO Register"]
    #[inline(always)]
    pub const fn aon_ldo(self) -> crate::common::Reg<regs::AonLdo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "BUCK Control Register 1"]
    #[inline(always)]
    pub const fn buck_cr1(self) -> crate::common::Reg<regs::BuckCr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "BUCK Control Register 2"]
    #[inline(always)]
    pub const fn buck_cr2(self) -> crate::common::Reg<regs::BuckCr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Charger Control Register 1"]
    #[inline(always)]
    pub const fn chg_cr1(self) -> crate::common::Reg<regs::ChgCr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Charger Control Register 2"]
    #[inline(always)]
    pub const fn chg_cr2(self) -> crate::common::Reg<regs::ChgCr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Charger Control Register 3"]
    #[inline(always)]
    pub const fn chg_cr3(self) -> crate::common::Reg<regs::ChgCr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Charger Control Register 4"]
    #[inline(always)]
    pub const fn chg_cr4(self) -> crate::common::Reg<regs::ChgCr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Charger Control Register 5"]
    #[inline(always)]
    pub const fn chg_cr5(self) -> crate::common::Reg<regs::ChgCr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Charger Status Register"]
    #[inline(always)]
    pub const fn chg_sr(self) -> crate::common::Reg<regs::ChgSr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "HPSYS LDO Control Register"]
    #[inline(always)]
    pub const fn hpsys_ldo(self) -> crate::common::Reg<regs::HpsysLdo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "LPSYS LDO Control Register"]
    #[inline(always)]
    pub const fn lpsys_ldo(self) -> crate::common::Reg<regs::LpsysLdo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "HPSYS Switch Register"]
    #[inline(always)]
    pub const fn hpsys_swr(self) -> crate::common::Reg<regs::HpsysSwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "LPSYS Switch Register"]
    #[inline(always)]
    pub const fn lpsys_swr(self) -> crate::common::Reg<regs::LpsysSwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Peripherals LDO"]
    #[inline(always)]
    pub const fn peri_ldo(self) -> crate::common::Reg<regs::PeriLdo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "PMU Test Register"]
    #[inline(always)]
    pub const fn pmu_tr(self) -> crate::common::Reg<regs::PmuTr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "PMU Reserved Register"]
    #[inline(always)]
    pub const fn pmu_rsvd(self) -> crate::common::Reg<regs::PmuRsvd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "HXT48 Control Register 1"]
    #[inline(always)]
    pub const fn hxt_cr1(self) -> crate::common::Reg<regs::HxtCr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "HXT48 Control Register 2"]
    #[inline(always)]
    pub const fn hxt_cr2(self) -> crate::common::Reg<regs::HxtCr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "HXT48 Control Register 3"]
    #[inline(always)]
    pub const fn hxt_cr3(self) -> crate::common::Reg<regs::HxtCr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "HRC48 Control Register"]
    #[inline(always)]
    pub const fn hrc_cr(self) -> crate::common::Reg<regs::HrcCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "DBL96 Control Register"]
    #[inline(always)]
    pub const fn dbl96_cr(self) -> crate::common::Reg<regs::Dbl96Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "DBL96 Calibration Register"]
    #[inline(always)]
    pub const fn dbl96_calr(self) -> crate::common::Reg<regs::Dbl96Calr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "CAU Bandgap Register"]
    #[inline(always)]
    pub const fn cau_bgr(self) -> crate::common::Reg<regs::CauBgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "CAU Test Register"]
    #[inline(always)]
    pub const fn cau_tr(self) -> crate::common::Reg<regs::CauTr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "CAU Reserved Register"]
    #[inline(always)]
    pub const fn cau_rsvd(self) -> crate::common::Reg<regs::CauRsvd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Wakeup Count Register"]
    #[inline(always)]
    pub const fn wkup_cnt(self) -> crate::common::Reg<regs::WkupCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "PowerKey Count Register"]
    #[inline(always)]
    pub const fn pwrkey_cnt(self) -> crate::common::Reg<regs::PwrkeyCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn hpsys_vout(self) -> crate::common::Reg<regs::HpsysVout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[inline(always)]
    pub const fn lpsys_vout(self) -> crate::common::Reg<regs::LpsysVout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[inline(always)]
    pub const fn buck_vout(self) -> crate::common::Reg<regs::BuckVout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
}
pub mod regs;
