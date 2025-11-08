#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Patch {
    ptr: *mut u8,
}
unsafe impl Send for Patch {}
unsafe impl Sync for Patch {}
impl Patch {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Channel register；支持索引 0..31"]
    #[inline(always)]
    pub const fn ch(self, idx: usize) -> crate::common::Reg<regs::Ch, crate::common::RW> {
        assert!(idx < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(idx * 4usize) as _) }
    }
    #[doc = "Channel Enable Register"]
    #[inline(always)]
    pub const fn cer(self) -> crate::common::Reg<regs::Cer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Channel Status Register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Channel Data Register"]
    #[inline(always)]
    pub const fn cdr(self) -> crate::common::Reg<regs::Cdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "Version Register"]
    #[inline(always)]
    pub const fn ver(self) -> crate::common::Reg<regs::Ver, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
}
pub mod regs;
