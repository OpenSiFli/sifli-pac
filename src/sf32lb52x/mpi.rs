#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mpi {
    ptr: *mut u8,
}
unsafe impl Send for Mpi {}
unsafe impl Sync for Mpi {}
impl Mpi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control Register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Data Register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Device Control Register"]
    #[inline(always)]
    pub const fn dcr(self) -> crate::common::Reg<regs::Dcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Prescaler Register"]
    #[inline(always)]
    pub const fn psclr(self) -> crate::common::Reg<regs::Psclr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Status Register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Status Clear Register"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Command Register"]
    #[inline(always)]
    pub const fn cmdr1(self) -> crate::common::Reg<regs::Cmdr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Address Register"]
    #[inline(always)]
    pub const fn ar1(self) -> crate::common::Reg<regs::Ar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Alternate Byte Register"]
    #[inline(always)]
    pub const fn abr1(self) -> crate::common::Reg<regs::Abr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Data Length Register"]
    #[inline(always)]
    pub const fn dlr1(self) -> crate::common::Reg<regs::Dlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Communication Configuration Register"]
    #[inline(always)]
    pub const fn ccr1(self) -> crate::common::Reg<regs::Ccr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Command Register"]
    #[inline(always)]
    pub const fn cmdr2(self) -> crate::common::Reg<regs::Cmdr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Address Register"]
    #[inline(always)]
    pub const fn ar2(self) -> crate::common::Reg<regs::Ar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Alternate Byte Register"]
    #[inline(always)]
    pub const fn abr2(self) -> crate::common::Reg<regs::Abr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Data Length Register"]
    #[inline(always)]
    pub const fn dlr2(self) -> crate::common::Reg<regs::Dlr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Communication Configuration Register"]
    #[inline(always)]
    pub const fn ccr2(self) -> crate::common::Reg<regs::Ccr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "AHB Command Register"]
    #[inline(always)]
    pub const fn hcmdr(self) -> crate::common::Reg<regs::Hcmdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "AHB Read Alternate Byte Register"]
    #[inline(always)]
    pub const fn hrabr(self) -> crate::common::Reg<regs::Hrabr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "AHB Read Communication Configuration Register"]
    #[inline(always)]
    pub const fn hrccr(self) -> crate::common::Reg<regs::Hrccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "AHB Write Alternate Byte Register"]
    #[inline(always)]
    pub const fn hwabr(self) -> crate::common::Reg<regs::Hwabr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "AHB Write Communication Configuration Register"]
    #[inline(always)]
    pub const fn hwccr(self) -> crate::common::Reg<regs::Hwccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "FIFO Control Register"]
    #[inline(always)]
    pub const fn fifocr(self) -> crate::common::Reg<regs::Fifocr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Miscelaneous Register"]
    #[inline(always)]
    pub const fn miscr(self) -> crate::common::Reg<regs::Miscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "CTR Starting Address Register"]
    #[inline(always)]
    pub const fn ctrsar(self) -> crate::common::Reg<regs::Ctrsar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "CTR Ending Address Register"]
    #[inline(always)]
    pub const fn ctrear(self) -> crate::common::Reg<regs::Ctrear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Nonce A Register"]
    #[inline(always)]
    pub const fn noncea(self) -> crate::common::Reg<regs::Noncea, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Nonce B Register"]
    #[inline(always)]
    pub const fn nonceb(self) -> crate::common::Reg<regs::Nonceb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "Address Aliasing Start Address Register"]
    #[inline(always)]
    pub const fn aasar(self) -> crate::common::Reg<regs::Aasar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "Address Aliasing Ending Address Register"]
    #[inline(always)]
    pub const fn aaear(self) -> crate::common::Reg<regs::Aaear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "Address Aliasing Offset Address Register"]
    #[inline(always)]
    pub const fn aaoar(self) -> crate::common::Reg<regs::Aaoar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "Command Interval Register"]
    #[inline(always)]
    pub const fn cir(self) -> crate::common::Reg<regs::Cir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "Status Match Register"]
    #[inline(always)]
    pub const fn smr(self) -> crate::common::Reg<regs::Smr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "Status Mask Register"]
    #[inline(always)]
    pub const fn smkr(self) -> crate::common::Reg<regs::Smkr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Timer Register"]
    #[inline(always)]
    pub const fn timr(self) -> crate::common::Reg<regs::Timr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "WDT Register"]
    #[inline(always)]
    pub const fn wdtr(self) -> crate::common::Reg<regs::Wdtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Prefetch Starting Address Register"]
    #[inline(always)]
    pub const fn prsar(self) -> crate::common::Reg<regs::Prsar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "Prefetch Ending Address Register"]
    #[inline(always)]
    pub const fn prear(self) -> crate::common::Reg<regs::Prear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "Calibration Clock Register"]
    #[inline(always)]
    pub const fn calcr(self) -> crate::common::Reg<regs::Calcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "APM32 Control Register"]
    #[inline(always)]
    pub const fn apm32cr(self) -> crate::common::Reg<regs::Apm32cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "Control Register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
}
pub mod regs;
