#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mailbox1 {
    ptr: *mut u8,
}
unsafe impl Send for Mailbox1 {}
unsafe impl Sync for Mailbox1 {}
impl Mailbox1 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn ier(self, n: usize) -> crate::common::Reg<regs::Ixr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 24usize) as _) }
    }
    #[inline(always)]
    pub const fn itr(self, n: usize) -> crate::common::Reg<regs::Ixr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize + n * 24usize) as _) }
    }
    #[inline(always)]
    pub const fn icr(self, n: usize) -> crate::common::Reg<regs::Ixr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize + n * 24usize) as _) }
    }
    #[inline(always)]
    pub const fn isr(self, n: usize) -> crate::common::Reg<regs::Ixr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize + n * 24usize) as _) }
    }
    #[inline(always)]
    pub const fn misr(self, n: usize) -> crate::common::Reg<regs::Ixr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 24usize) as _) }
    }
    #[inline(always)]
    pub const fn exr(self, n: usize) -> crate::common::Reg<regs::Exr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize + n * 24usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mailbox2 {
    ptr: *mut u8,
}
unsafe impl Send for Mailbox2 {}
unsafe impl Sync for Mailbox2 {}
impl Mailbox2 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn ier(self, n: usize) -> crate::common::Reg<regs::Ixr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 24usize) as _) }
    }
    #[inline(always)]
    pub const fn itr(self, n: usize) -> crate::common::Reg<regs::Ixr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize + n * 24usize) as _) }
    }
    #[inline(always)]
    pub const fn icr(self, n: usize) -> crate::common::Reg<regs::Ixr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize + n * 24usize) as _) }
    }
    #[inline(always)]
    pub const fn isr(self, n: usize) -> crate::common::Reg<regs::Ixr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize + n * 24usize) as _) }
    }
    #[inline(always)]
    pub const fn misr(self, n: usize) -> crate::common::Reg<regs::Ixr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 24usize) as _) }
    }
    #[inline(always)]
    pub const fn exr(self, n: usize) -> crate::common::Reg<regs::Exr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize + n * 24usize) as _) }
    }
}
pub mod regs;
