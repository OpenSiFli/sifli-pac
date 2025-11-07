#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi {
    ptr: *mut u8,
}
unsafe impl Send for Spi {}
unsafe impl Sync for Spi {}
impl Spi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Top Control Register"]
    #[inline(always)]
    pub const fn top_ctrl(self) -> crate::common::Reg<regs::TopCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "FIFO Control Register"]
    #[inline(always)]
    pub const fn fifo_ctrl(self) -> crate::common::Reg<regs::FifoCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn inte(self) -> crate::common::Reg<regs::Inte, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "SPI Time Out Register"]
    #[inline(always)]
    pub const fn to(self) -> crate::common::Reg<regs::To, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "SPI DATA Register"]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<regs::Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "RWOT Control Register"]
    #[inline(always)]
    pub const fn rwot_ctrl(self) -> crate::common::Reg<regs::RwotCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "RWOT Counter Cycles Match Register"]
    #[inline(always)]
    pub const fn rwot_ccm(self) -> crate::common::Reg<regs::RwotCcm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "RWOT Counter Value Write for Red Request Register"]
    #[inline(always)]
    pub const fn rwot_cvwrn(self) -> crate::common::Reg<regs::RwotCvwrn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "CLK Control Register"]
    #[inline(always)]
    pub const fn clk_ctrl(self) -> crate::common::Reg<regs::ClkCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Three Wire Mode Control Register"]
    #[inline(always)]
    pub const fn triwire_ctrl(self) -> crate::common::Reg<regs::TriwireCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
}
pub mod regs;
