#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HpsysPinmux {
    ptr: *mut u8,
}
unsafe impl Send for HpsysPinmux {}
unsafe impl Sync for HpsysPinmux {}
impl HpsysPinmux {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn pad_sa00(self) -> crate::common::Reg<regs::PadSa00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn pad_sa01(self) -> crate::common::Reg<regs::PadSa01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn pad_sa02(self) -> crate::common::Reg<regs::PadSa02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn pad_sa03(self) -> crate::common::Reg<regs::PadSa03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn pad_sa04(self) -> crate::common::Reg<regs::PadSa04, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn pad_sa05(self) -> crate::common::Reg<regs::PadSa05, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn pad_sa06(self) -> crate::common::Reg<regs::PadSa06, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn pad_sa07(self) -> crate::common::Reg<regs::PadSa07, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn pad_sa08(self) -> crate::common::Reg<regs::PadSa08, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn pad_sa09(self) -> crate::common::Reg<regs::PadSa09, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn pad_sa10(self) -> crate::common::Reg<regs::PadSa10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn pad_sa11(self) -> crate::common::Reg<regs::PadSa11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn pad_sa12(self) -> crate::common::Reg<regs::PadSa12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn pad_pa0_38(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PadPa0_38, crate::common::RW> {
        assert!(n < 39usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn pad_pa39_42(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PadPa39_42, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn pad_pa43_44(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::PadPa43_44, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
