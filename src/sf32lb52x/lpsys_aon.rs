#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpsysAon {
    ptr: *mut u8,
}
unsafe impl Send for LpsysAon {}
unsafe impl Sync for LpsysAon {}
impl LpsysAon {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Power Mode Register"]
    #[inline(always)]
    pub const fn pmr(self) -> crate::common::Reg<regs::Pmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Control Register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Control Register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Control Register 3"]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Active Mode Control register"]
    #[inline(always)]
    pub const fn acr(self) -> crate::common::Reg<regs::Acr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Light Sleep Ctrl Register"]
    #[inline(always)]
    pub const fn lscr(self) -> crate::common::Reg<regs::Lscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Deep Sleep Ctrl Register"]
    #[inline(always)]
    pub const fn dscr(self) -> crate::common::Reg<regs::Dscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Standby Mode Ctrl Register"]
    #[inline(always)]
    pub const fn sbcr(self) -> crate::common::Reg<regs::Sbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Wakeup Enable register"]
    #[inline(always)]
    pub const fn wer(self) -> crate::common::Reg<regs::Wer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Wakeup Status register"]
    #[inline(always)]
    pub const fn wsr(self) -> crate::common::Reg<regs::Wsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Wakeup Clear register"]
    #[inline(always)]
    pub const fn wcr(self) -> crate::common::Reg<regs::Wcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Inter System Status Register"]
    #[inline(always)]
    pub const fn issr(self) -> crate::common::Reg<regs::Issr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "BT sleep time target"]
    #[inline(always)]
    pub const fn target(self) -> crate::common::Reg<regs::Target, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "BT actual sleep time"]
    #[inline(always)]
    pub const fn actual(self) -> crate::common::Reg<regs::Actual, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "time before bt awake"]
    #[inline(always)]
    pub const fn pre_wkup(self) -> crate::common::Reg<regs::PreWkup, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "BT sleep configuration"]
    #[inline(always)]
    pub const fn slp_cfg(self) -> crate::common::Reg<regs::SlpCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "BT sleep control"]
    #[inline(always)]
    pub const fn slp_ctrl(self) -> crate::common::Reg<regs::SlpCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Analog Control Register"]
    #[inline(always)]
    pub const fn anacr(self) -> crate::common::Reg<regs::Anacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Global Timer Register"]
    #[inline(always)]
    pub const fn gtimr(self) -> crate::common::Reg<regs::Gtimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Reserved Register 0"]
    #[inline(always)]
    pub const fn reserve0(self) -> crate::common::Reg<regs::Reserve0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Reserved Register 1"]
    #[inline(always)]
    pub const fn reserve1(self) -> crate::common::Reg<regs::Reserve1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Stack Pointer Register"]
    #[inline(always)]
    pub const fn spr(self) -> crate::common::Reg<regs::Spr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Pointer Counter Register"]
    #[inline(always)]
    pub const fn pcr(self) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
}
pub mod regs;
