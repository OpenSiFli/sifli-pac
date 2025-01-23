#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HpsysRcc {
    ptr: *mut u8,
}
unsafe impl Send for HpsysRcc {}
unsafe impl Sync for HpsysRcc {}
impl HpsysRcc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Reset Register 1"]
    #[inline(always)]
    pub const fn rstr1(self) -> crate::common::Reg<regs::Rstr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Reset Register 2"]
    #[inline(always)]
    pub const fn rstr2(self) -> crate::common::Reg<regs::Rstr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Enable Register 1"]
    #[inline(always)]
    pub const fn enr1(self) -> crate::common::Reg<regs::Enr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Enable Register 2"]
    #[inline(always)]
    pub const fn enr2(self) -> crate::common::Reg<regs::Enr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Enable Set Register 1"]
    #[inline(always)]
    pub const fn esr1(self) -> crate::common::Reg<regs::Esr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Enable Set Register 2"]
    #[inline(always)]
    pub const fn esr2(self) -> crate::common::Reg<regs::Esr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Enable Clear Register 1"]
    #[inline(always)]
    pub const fn ecr1(self) -> crate::common::Reg<regs::Ecr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Enable Clear Register 2"]
    #[inline(always)]
    pub const fn ecr2(self) -> crate::common::Reg<regs::Ecr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Clock Select Register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Clock Configuration Register"]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "USBC Control Register"]
    #[inline(always)]
    pub const fn usbcr(self) -> crate::common::Reg<regs::Usbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "DLL1 Control Register"]
    #[inline(always)]
    pub const fn dll1cr(self) -> crate::common::Reg<regs::Dll1cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "DLL2 Control Register"]
    #[inline(always)]
    pub const fn dll2cr(self) -> crate::common::Reg<regs::Dll2cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "HRC Calibration Register 1"]
    #[inline(always)]
    pub const fn hrccal1(self) -> crate::common::Reg<regs::Hrccal1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "HRC Calibration Register 2"]
    #[inline(always)]
    pub const fn hrccal2(self) -> crate::common::Reg<regs::Hrccal2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Debug Clock Register"]
    #[inline(always)]
    pub const fn dbgclkr(self) -> crate::common::Reg<regs::Dbgclkr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Debug Register"]
    #[inline(always)]
    pub const fn dbgr(self) -> crate::common::Reg<regs::Dbgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Deep WFI mode Clock Configuration Register"]
    #[inline(always)]
    pub const fn dwcfgr(self) -> crate::common::Reg<regs::Dwcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
}
pub mod regs;
