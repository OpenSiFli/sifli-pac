#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpsysPinmux {
    ptr: *mut u8,
}
unsafe impl Send for LpsysPinmux {}
unsafe impl Sync for LpsysPinmux {}
impl LpsysPinmux {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn pad_pb00(self) -> crate::common::Reg<regs::PadPb00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn pad_pb01(self) -> crate::common::Reg<regs::PadPb01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn pad_pb02(self) -> crate::common::Reg<regs::PadPb02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn pad_pb03(self) -> crate::common::Reg<regs::PadPb03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
pub mod regs;
