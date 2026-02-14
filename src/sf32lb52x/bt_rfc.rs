#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BtRfc {
    ptr: *mut u8,
}
unsafe impl Send for BtRfc {}
unsafe impl Sync for BtRfc {}
impl BtRfc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn vco_reg1(self) -> crate::common::Reg<regs::VcoReg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn vco_reg2(self) -> crate::common::Reg<regs::VcoReg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn vco_reg3(self) -> crate::common::Reg<regs::VcoReg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn misc_ctrl_reg(self) -> crate::common::Reg<regs::MiscCtrlReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn rf_lodist_reg(self) -> crate::common::Reg<regs::RfLodistReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn fbdv_reg1(self) -> crate::common::Reg<regs::FbdvReg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn fbdv_reg2(self) -> crate::common::Reg<regs::FbdvReg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn pfdcp_reg(self) -> crate::common::Reg<regs::PfdcpReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn lpf_reg(self) -> crate::common::Reg<regs::LpfReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn edr_cal_reg1(self) -> crate::common::Reg<regs::EdrCalReg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn oslo_reg(self) -> crate::common::Reg<regs::OsloReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn atest_reg(self) -> crate::common::Reg<regs::AtestReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn dtest_reg(self) -> crate::common::Reg<regs::DtestReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn trf_reg1(self) -> crate::common::Reg<regs::TrfReg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn trf_reg2(self) -> crate::common::Reg<regs::TrfReg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn trf_edr_reg1(self) -> crate::common::Reg<regs::TrfEdrReg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn trf_edr_reg2(self) -> crate::common::Reg<regs::TrfEdrReg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn rrf_reg(self) -> crate::common::Reg<regs::RrfReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn rbb_reg1(self) -> crate::common::Reg<regs::RbbReg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn rbb_reg2(self) -> crate::common::Reg<regs::RbbReg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[inline(always)]
    pub const fn rbb_reg3(self) -> crate::common::Reg<regs::RbbReg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn rbb_reg4(self) -> crate::common::Reg<regs::RbbReg4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn rbb_reg5(self) -> crate::common::Reg<regs::RbbReg5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn rbb_reg6(self) -> crate::common::Reg<regs::RbbReg6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn adc_reg(self) -> crate::common::Reg<regs::AdcReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn tbb_reg(self) -> crate::common::Reg<regs::TbbReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn atstbuf_reg(self) -> crate::common::Reg<regs::AtstbufReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[inline(always)]
    pub const fn inccal_reg1(self) -> crate::common::Reg<regs::InccalReg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[inline(always)]
    pub const fn inccal_reg2(self) -> crate::common::Reg<regs::InccalReg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[inline(always)]
    pub const fn roscal_reg1(self) -> crate::common::Reg<regs::RoscalReg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[inline(always)]
    pub const fn roscal_reg2(self) -> crate::common::Reg<regs::RoscalReg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn rcroscal_reg(self) -> crate::common::Reg<regs::RcroscalReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[inline(always)]
    pub const fn pacal_reg(self) -> crate::common::Reg<regs::PacalReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[inline(always)]
    pub const fn cu_addr_reg1(self) -> crate::common::Reg<regs::CuAddrReg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[inline(always)]
    pub const fn cu_addr_reg2(self) -> crate::common::Reg<regs::CuAddrReg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn cu_addr_reg3(self) -> crate::common::Reg<regs::CuAddrReg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[inline(always)]
    pub const fn cal_addr_reg1(self) -> crate::common::Reg<regs::CalAddrReg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[inline(always)]
    pub const fn cal_addr_reg2(self) -> crate::common::Reg<regs::CalAddrReg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[inline(always)]
    pub const fn cal_addr_reg3(self) -> crate::common::Reg<regs::CalAddrReg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[inline(always)]
    pub const fn agc_reg(self) -> crate::common::Reg<regs::AgcReg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[inline(always)]
    pub const fn iq_pwr_reg1(self) -> crate::common::Reg<regs::IqPwrReg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[inline(always)]
    pub const fn iq_pwr_reg2(self) -> crate::common::Reg<regs::IqPwrReg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
}
pub mod regs;
