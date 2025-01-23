#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsen {
    ptr: *mut u8,
}
unsafe impl Send for Tsen {}
unsafe impl Sync for Tsen {}
impl Tsen {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "TSEN Analog Control Register"]
    #[inline(always)]
    pub const fn tsen_ctrl_reg(self) -> crate::common::Reg<regs::TsenCtrlReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Tsen Read Data"]
    #[inline(always)]
    pub const fn tsen_rdata(self) -> crate::common::Reg<regs::TsenRdata, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Tsen IRQ Register"]
    #[inline(always)]
    pub const fn tsen_irq(self) -> crate::common::Reg<regs::TsenIrq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
pub mod regs;
