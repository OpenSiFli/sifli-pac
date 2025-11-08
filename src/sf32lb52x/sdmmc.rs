#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdmmc {
    ptr: *mut u8,
}
unsafe impl Send for Sdmmc {}
unsafe impl Sync for Sdmmc {}
impl Sdmmc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "command and data status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "command control register"]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "command argument register"]
    #[inline(always)]
    pub const fn car(self) -> crate::common::Reg<regs::Car, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "response command index register"]
    #[inline(always)]
    pub const fn rir(self) -> crate::common::Reg<regs::Rir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "response command argument1 register"]
    #[inline(always)]
    pub const fn rar1(self) -> crate::common::Reg<regs::Rar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "response command argument2 register"]
    #[inline(always)]
    pub const fn rar2(self) -> crate::common::Reg<regs::Rar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "response command argument3 register"]
    #[inline(always)]
    pub const fn rar3(self) -> crate::common::Reg<regs::Rar3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "response command argument4 register"]
    #[inline(always)]
    pub const fn rar4(self) -> crate::common::Reg<regs::Rar4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "timeout count register"]
    #[inline(always)]
    pub const fn tor(self) -> crate::common::Reg<regs::Tor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "data control register"]
    #[inline(always)]
    pub const fn dcr(self) -> crate::common::Reg<regs::Dcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "data length register"]
    #[inline(always)]
    pub const fn dlr(self) -> crate::common::Reg<regs::Dlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "command and data interrupt mask register"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "clock control register"]
    #[inline(always)]
    pub const fn clkcr(self) -> crate::common::Reg<regs::Clkcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "card interface control and card detect register"]
    #[inline(always)]
    pub const fn cdr(self) -> crate::common::Reg<regs::Cdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "card debug port1 register"]
    #[inline(always)]
    pub const fn dbgr1(self) -> crate::common::Reg<regs::Dbgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "card debug port2 register"]
    #[inline(always)]
    pub const fn dbgr2(self) -> crate::common::Reg<regs::Dbgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "CE-ATA/SDIO mode register"]
    #[inline(always)]
    pub const fn ceata(self) -> crate::common::Reg<regs::Ceata, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "data status register"]
    #[inline(always)]
    pub const fn dsr(self) -> crate::common::Reg<regs::Dsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "clock duty cycle register"]
    #[inline(always)]
    pub const fn cdcr(self) -> crate::common::Reg<regs::Cdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "cache status register"]
    #[inline(always)]
    pub const fn casr(self) -> crate::common::Reg<regs::Casr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "cache control register"]
    #[inline(always)]
    pub const fn cacr(self) -> crate::common::Reg<regs::Cacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "cache counter register"]
    #[inline(always)]
    pub const fn cacnt(self) -> crate::common::Reg<regs::Cacnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "cache offset register"]
    #[inline(always)]
    pub const fn caoff(self) -> crate::common::Reg<regs::Caoff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "FIFO entry"]
    #[inline(always)]
    pub const fn fifo(self) -> crate::common::Reg<regs::Fifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
}
pub mod regs;
