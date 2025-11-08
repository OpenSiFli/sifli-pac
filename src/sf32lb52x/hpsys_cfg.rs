#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HpsysCfg {
    ptr: *mut u8,
}
unsafe impl Send for HpsysCfg {}
unsafe impl Sync for HpsysCfg {}
impl HpsysCfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Boot Mode Register"]
    #[inline(always)]
    pub const fn bmr(self) -> crate::common::Reg<regs::Bmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "ID Register"]
    #[inline(always)]
    pub const fn idr(self) -> crate::common::Reg<regs::Idr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "SW Control Register"]
    #[inline(always)]
    pub const fn swcr(self) -> crate::common::Reg<regs::Swcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Security Control Register"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "System Configure Register"]
    #[inline(always)]
    pub const fn syscr(self) -> crate::common::Reg<regs::Syscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Mirrored RTC Time Register"]
    #[inline(always)]
    pub const fn rtc_tr(self) -> crate::common::Reg<regs::RtcTr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Mirrored RTC Date Register"]
    #[inline(always)]
    pub const fn rtc_dr(self) -> crate::common::Reg<regs::RtcDr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "ULP Memory Control register"]
    #[inline(always)]
    pub const fn ulpmcr(self) -> crate::common::Reg<regs::Ulpmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Debug Select Register"]
    #[inline(always)]
    pub const fn dbgr(self) -> crate::common::Reg<regs::Dbgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Memory Debug Register"]
    #[inline(always)]
    pub const fn mdbgr(self) -> crate::common::Reg<regs::Mdbgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Interrupt Selection for LCPU"]
    #[inline(always)]
    pub const fn lpirq(self) -> crate::common::Reg<regs::Lpirq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "USB Control register"]
    #[inline(always)]
    pub const fn usbcr(self) -> crate::common::Reg<regs::Usbcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "HPSYS RSVD Register"]
    #[inline(always)]
    pub const fn sys_rsvd(self) -> crate::common::Reg<regs::SysRsvd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "I2C1 Pin Register"]
    #[inline(always)]
    pub const fn i2c1_pinr(self) -> crate::common::Reg<regs::I2c1Pinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "I2C2 Pin Register"]
    #[inline(always)]
    pub const fn i2c2_pinr(self) -> crate::common::Reg<regs::I2c2Pinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "I2C3 Pin Register"]
    #[inline(always)]
    pub const fn i2c3_pinr(self) -> crate::common::Reg<regs::I2c3Pinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "I2C4 Pin Register"]
    #[inline(always)]
    pub const fn i2c4_pinr(self) -> crate::common::Reg<regs::I2c4Pinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "USART1 Pin Register"]
    #[inline(always)]
    pub const fn usart1_pinr(self) -> crate::common::Reg<regs::Usart1Pinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "USART2 Pin Register"]
    #[inline(always)]
    pub const fn usart2_pinr(self) -> crate::common::Reg<regs::Usart2Pinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "USART3 Pin Register"]
    #[inline(always)]
    pub const fn usart3_pinr(self) -> crate::common::Reg<regs::Usart3Pinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "GPTIM1 Pin Register"]
    #[inline(always)]
    pub const fn gptim1_pinr(self) -> crate::common::Reg<regs::Gptim1Pinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "GPTIM2 Pin Register"]
    #[inline(always)]
    pub const fn gptim2_pinr(self) -> crate::common::Reg<regs::Gptim2Pinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "GPTIM ETR Pin Register"]
    #[inline(always)]
    pub const fn etr_pinr(self) -> crate::common::Reg<regs::EtrPinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "LPTIM1 Pin Register"]
    #[inline(always)]
    pub const fn lptim1_pinr(self) -> crate::common::Reg<regs::Lptim1Pinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "LPTIM2 Pin Register"]
    #[inline(always)]
    pub const fn lptim2_pinr(self) -> crate::common::Reg<regs::Lptim2Pinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "ATIM1 Pin Register 1"]
    #[inline(always)]
    pub const fn atim1_pinr1(self) -> crate::common::Reg<regs::Atim1Pinr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "ATIM1 Pin Register 2"]
    #[inline(always)]
    pub const fn atim1_pinr2(self) -> crate::common::Reg<regs::Atim1Pinr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "ATIM1 Pin Register 3"]
    #[inline(always)]
    pub const fn atim1_pinr3(self) -> crate::common::Reg<regs::Atim1Pinr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "PTA Pin Register"]
    #[inline(always)]
    pub const fn pta_pinr(self) -> crate::common::Reg<regs::PtaPinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "ANAU Control Register"]
    #[inline(always)]
    pub const fn anau_cr(self) -> crate::common::Reg<regs::AnauCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "ANAU Reserve Register"]
    #[inline(always)]
    pub const fn anau_rsvd(self) -> crate::common::Reg<regs::AnauRsvd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "Analog Test Register"]
    #[inline(always)]
    pub const fn anatr(self) -> crate::common::Reg<regs::Anatr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "CAU2 Control Register"]
    #[inline(always)]
    pub const fn cau2_cr(self) -> crate::common::Reg<regs::Cau2Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "CAU2 RSVD Register1"]
    #[inline(always)]
    pub const fn cau2_rsvd(self) -> crate::common::Reg<regs::Cau2Rsvd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
}
pub mod regs;
