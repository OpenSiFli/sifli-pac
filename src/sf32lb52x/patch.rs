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
    #[doc = "Patch channel register"]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> crate::common::Reg<regs::Ch, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "Patch channel enable register"]
    #[inline(always)]
    pub const fn cer(self) -> crate::common::Reg<regs::Cer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Patch channel status register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "Patch channel data register"]
    #[inline(always)]
    pub const fn cdr(self) -> crate::common::Reg<regs::Cdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "Patch interrupt enable register"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "Patch interrupt status register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "Patch interrupt clear register"]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "Patch version register"]
    #[inline(always)]
    pub const fn ver(self) -> crate::common::Reg<regs::Ver, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
}
pub mod regs;
