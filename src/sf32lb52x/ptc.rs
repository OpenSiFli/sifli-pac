#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ptc {
    ptr: *mut u8,
}
unsafe impl Send for Ptc {}
unsafe impl Sync for Ptc {}
impl Ptc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "interrupt status register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "interrupt clear register"]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "interrupt enable register"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "task 1 control register"]
    #[inline(always)]
    pub const fn tcr1(self) -> crate::common::Reg<regs::Tcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "task 1 address register"]
    #[inline(always)]
    pub const fn tar1(self) -> crate::common::Reg<regs::Tar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "task 1 data register"]
    #[inline(always)]
    pub const fn tdr1(self) -> crate::common::Reg<regs::Tdr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "task 1 repetition and delay counter register"]
    #[inline(always)]
    pub const fn rcr1(self) -> crate::common::Reg<regs::Rcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn tcr2(self) -> crate::common::Reg<regs::Tcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn tar2(self) -> crate::common::Reg<regs::Tar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn tdr2(self) -> crate::common::Reg<regs::Tdr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "task 2 repetition and delay counter register"]
    #[inline(always)]
    pub const fn rcr2(self) -> crate::common::Reg<regs::Rcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn tcr3(self) -> crate::common::Reg<regs::Tcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn tar3(self) -> crate::common::Reg<regs::Tar3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn tdr3(self) -> crate::common::Reg<regs::Tdr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "task 3 repetition and delay counter register"]
    #[inline(always)]
    pub const fn rcr3(self) -> crate::common::Reg<regs::Rcr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn tcr4(self) -> crate::common::Reg<regs::Tcr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn tar4(self) -> crate::common::Reg<regs::Tar4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn tdr4(self) -> crate::common::Reg<regs::Tdr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "task 4 repetition and delay counter register"]
    #[inline(always)]
    pub const fn rcr4(self) -> crate::common::Reg<regs::Rcr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[inline(always)]
    pub const fn tcr5(self) -> crate::common::Reg<regs::Tcr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn tar5(self) -> crate::common::Reg<regs::Tar5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn tdr5(self) -> crate::common::Reg<regs::Tdr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "task 5 repetition counter register"]
    #[inline(always)]
    pub const fn rcr5(self) -> crate::common::Reg<regs::Rcr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn tcr6(self) -> crate::common::Reg<regs::Tcr6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn tar6(self) -> crate::common::Reg<regs::Tar6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn tdr6(self) -> crate::common::Reg<regs::Tdr6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "task 6 repetition counter register"]
    #[inline(always)]
    pub const fn rcr6(self) -> crate::common::Reg<regs::Rcr6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[inline(always)]
    pub const fn tcr7(self) -> crate::common::Reg<regs::Tcr7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn tar7(self) -> crate::common::Reg<regs::Tar7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[inline(always)]
    pub const fn tdr7(self) -> crate::common::Reg<regs::Tdr7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "task 7 repetition counter register"]
    #[inline(always)]
    pub const fn rcr7(self) -> crate::common::Reg<regs::Rcr7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[inline(always)]
    pub const fn tcr8(self) -> crate::common::Reg<regs::Tcr8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn tar8(self) -> crate::common::Reg<regs::Tar8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[inline(always)]
    pub const fn tdr8(self) -> crate::common::Reg<regs::Tdr8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "task 8 repetition counter register"]
    #[inline(always)]
    pub const fn rcr8(self) -> crate::common::Reg<regs::Rcr8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "temporary memory 1"]
    #[inline(always)]
    pub const fn mem1(self) -> crate::common::Reg<regs::Mem1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "temporary memory 2"]
    #[inline(always)]
    pub const fn mem2(self) -> crate::common::Reg<regs::Mem2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "temporary memory 3"]
    #[inline(always)]
    pub const fn mem3(self) -> crate::common::Reg<regs::Mem3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "temporary memory 4"]
    #[inline(always)]
    pub const fn mem4(self) -> crate::common::Reg<regs::Mem4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[inline(always)]
    pub const fn gpio31_0(self) -> crate::common::Reg<regs::Gpio310, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[inline(always)]
    pub const fn gpio63_32(self) -> crate::common::Reg<regs::Gpio6332, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[inline(always)]
    pub const fn gpio95_64(self) -> crate::common::Reg<regs::Gpio9564, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
}
pub mod regs;
