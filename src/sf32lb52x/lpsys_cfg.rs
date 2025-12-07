#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpsysCfg {
    ptr: *mut u8,
}
unsafe impl Send for LpsysCfg {}
unsafe impl Sync for LpsysCfg {}
impl LpsysCfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "System Configure Register"]
    #[inline(always)]
    pub const fn syscr(self) -> crate::common::Reg<regs::Syscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Mirrored RTC Time Register"]
    #[inline(always)]
    pub const fn rtc_tr(self) -> crate::common::Reg<regs::RtcTr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Mirrored RTC Date Register"]
    #[inline(always)]
    pub const fn rtc_dr(self) -> crate::common::Reg<regs::RtcDr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "ULP Memory Control register"]
    #[inline(always)]
    pub const fn ulpmcr(self) -> crate::common::Reg<regs::Ulpmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Debug Register"]
    #[inline(always)]
    pub const fn dbgr(self) -> crate::common::Reg<regs::Dbgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Memory Debug Register"]
    #[inline(always)]
    pub const fn mdbgr(self) -> crate::common::Reg<regs::Mdbgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "USART4 Pin Register"]
    #[inline(always)]
    pub const fn usart4_pinr(self) -> crate::common::Reg<regs::Usart4Pinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "USART5 Pin Register"]
    #[inline(always)]
    pub const fn usart5_pinr(self) -> crate::common::Reg<regs::Usart5Pinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "PTA Pin Register"]
    #[inline(always)]
    pub const fn pta_pinr(self) -> crate::common::Reg<regs::PtaPinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
}
pub mod regs;
