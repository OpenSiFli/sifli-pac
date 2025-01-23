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
    pub const fn adc_cfg_reg1(self) -> crate::common::Reg<regs::AdcCfgReg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "ADC Slot0 Config Register"]
    #[inline(always)]
    pub const fn adc_slot0_reg(self) -> crate::common::Reg<regs::AdcSlot0Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "ADC Slot1 Config Register"]
    #[inline(always)]
    pub const fn adc_slot1_reg(self) -> crate::common::Reg<regs::AdcSlot1Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "ADC Slot2 Config Register"]
    #[inline(always)]
    pub const fn adc_slot2_reg(self) -> crate::common::Reg<regs::AdcSlot2Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "ADC Slot3 Config Register"]
    #[inline(always)]
    pub const fn adc_slot3_reg(self) -> crate::common::Reg<regs::AdcSlot3Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "ADC Slot4 Config Register"]
    #[inline(always)]
    pub const fn adc_slot4_reg(self) -> crate::common::Reg<regs::AdcSlot4Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "ADC Slot5 Config Register"]
    #[inline(always)]
    pub const fn adc_slot5_reg(self) -> crate::common::Reg<regs::AdcSlot5Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "ADC Slot6 Config Register"]
    #[inline(always)]
    pub const fn adc_slot6_reg(self) -> crate::common::Reg<regs::AdcSlot6Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "ADC Slot7 Config Register"]
    #[inline(always)]
    pub const fn adc_slot7_reg(self) -> crate::common::Reg<regs::AdcSlot7Reg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "ADC Read Data0"]
    #[inline(always)]
    pub const fn adc_rdata0(self) -> crate::common::Reg<regs::AdcRdata0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "ADC Read Data1"]
    #[inline(always)]
    pub const fn adc_rdata1(self) -> crate::common::Reg<regs::AdcRdata1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "ADC Read Data2"]
    #[inline(always)]
    pub const fn adc_rdata2(self) -> crate::common::Reg<regs::AdcRdata2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "ADC Read Data3"]
    #[inline(always)]
    pub const fn adc_rdata3(self) -> crate::common::Reg<regs::AdcRdata3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "ADC Read Data For DMA"]
    #[inline(always)]
    pub const fn adc_dma_rdata(self) -> crate::common::Reg<regs::AdcDmaRdata, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "ADC Control Register"]
    #[inline(always)]
    pub const fn adc_ctrl_reg(self) -> crate::common::Reg<regs::AdcCtrlReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "ADC Control Register2"]
    #[inline(always)]
    pub const fn adc_ctrl_reg2(self) -> crate::common::Reg<regs::AdcCtrlReg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "GPADC Status Register"]
    #[inline(always)]
    pub const fn gpadc_status(self) -> crate::common::Reg<regs::GpadcStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "GPADC IRQ Register"]
    #[inline(always)]
    pub const fn gpadc_irq(self) -> crate::common::Reg<regs::GpadcIrq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
}
pub mod regs;
