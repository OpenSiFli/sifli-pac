#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iwdt {
    ptr: *mut u8,
}
unsafe impl Send for Iwdt {}
unsafe impl Sync for Iwdt {}
impl Iwdt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "WatchDog Counter Value 0"]
    #[inline(always)]
    pub const fn wdt_cvr0(self) -> crate::common::Reg<regs::WdtCvr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "WatchDog Counter Value 1"]
    #[inline(always)]
    pub const fn wdt_cvr1(self) -> crate::common::Reg<regs::WdtCvr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "WatchDog Control Register"]
    #[inline(always)]
    pub const fn wdt_cr(self) -> crate::common::Reg<regs::WdtCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "WatchDog Counter Control Register"]
    #[inline(always)]
    pub const fn wdt_ccr(self) -> crate::common::Reg<regs::WdtCcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "WatchDog Interrupt Clear Register"]
    #[inline(always)]
    pub const fn wdt_icr(self) -> crate::common::Reg<regs::WdtIcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "WatchDog Status Register"]
    #[inline(always)]
    pub const fn wdt_sr(self) -> crate::common::Reg<regs::WdtSr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "WatchDog Write Protect Register"]
    #[inline(always)]
    pub const fn wdt_wp(self) -> crate::common::Reg<regs::WdtWp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "WatchDog Flag Register"]
    #[inline(always)]
    pub const fn wdt_fg(self) -> crate::common::Reg<regs::WdtFg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
pub mod regs;
