#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpadc {
    ptr: *mut u8,
}
unsafe impl Send for Gpadc {}
unsafe impl Sync for Gpadc {}
impl Gpadc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ADC Analog Config Register 1"]
    #[inline(always)]
    pub const fn cfg_reg1(self) -> crate::common::Reg<regs::CfgReg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "ADC Slot0 Config Register"]
    #[inline(always)]
    pub const fn slot(self, n: usize) -> crate::common::Reg<regs::Slot0Reg, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize + n * 4usize) as _) }
    }
    #[doc = "ADC Read Data0"]
    #[inline(always)]
    pub const fn rdata(self, n: usize) -> crate::common::Reg<regs::Rdata, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize + n * 4usize) as _) }
    }
    #[doc = "ADC Read Data For DMA"]
    #[inline(always)]
    pub const fn dma_rdata(self) -> crate::common::Reg<regs::DmaRdata, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "ADC Control Register"]
    #[inline(always)]
    pub const fn ctrl_reg(self) -> crate::common::Reg<regs::CtrlReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "ADC Control Register2"]
    #[inline(always)]
    pub const fn ctrl_reg2(self) -> crate::common::Reg<regs::CtrlReg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "GPADC Status Register"]
    #[inline(always)]
    pub const fn gpadc_status(self) -> crate::common::Reg<regs::GpadcStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "GPADC IRQ Register"]
    #[inline(always)]
    pub const fn gpadc_irq(self) -> crate::common::Reg<regs::GpadcIrq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
}
pub mod regs;
pub mod vals;
