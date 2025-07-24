#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac {
    ptr: *mut u8,
}
unsafe impl Send for Dmac {}
unsafe impl Sync for Dmac {}
impl Dmac {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn ifcr(self) -> crate::common::Reg<regs::Ifcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 20usize) as _) }
    }
    #[inline(always)]
    pub const fn cndtr(self, n: usize) -> crate::common::Reg<regs::Cndtr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize + n * 20usize) as _) }
    }
    #[inline(always)]
    pub const fn cpar(self, n: usize) -> crate::common::Reg<regs::Cpar, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize + n * 20usize) as _) }
    }
    #[inline(always)]
    pub const fn cm0ar(self, n: usize) -> crate::common::Reg<regs::Cm0ar, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize + n * 20usize) as _) }
    }
    #[inline(always)]
    pub const fn cbsr(self, n: usize) -> crate::common::Reg<regs::Cbsr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 20usize) as _) }
    }
    #[inline(always)]
    pub const fn cselr(self, n: usize) -> crate::common::Reg<regs::Cselr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
