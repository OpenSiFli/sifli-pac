#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpsysRcc {
    ptr: *mut u8,
}
unsafe impl Send for LpsysRcc {}
unsafe impl Sync for LpsysRcc {}
impl LpsysRcc {
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Enable Register 1"]
    #[inline(always)]
    pub const fn enr1(self) -> crate::common::Reg<regs::Enr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Enable Set Register 1"]
    #[inline(always)]
    pub const fn esr1(self) -> crate::common::Reg<regs::Esr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Enable Clear Register 1"]
    #[inline(always)]
    pub const fn ecr1(self) -> crate::common::Reg<regs::Ecr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Clock Select Register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Clock Configuration Register"]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Debug Register"]
    #[inline(always)]
    pub const fn dbgr(self) -> crate::common::Reg<regs::Dbgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
pub mod regs;
