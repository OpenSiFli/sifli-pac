#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Audcodec {
    ptr: *mut u8,
}
unsafe impl Send for Audcodec {}
unsafe impl Sync for Audcodec {}
impl Audcodec {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn id(self) -> crate::common::Reg<regs::Id, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn irq(self) -> crate::common::Reg<regs::Irq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn irq_msk(self) -> crate::common::Reg<regs::IrqMsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn dac_cfg(self) -> crate::common::Reg<regs::DacCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn adc_cfg(self) -> crate::common::Reg<regs::AdcCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn apb_stat(self) -> crate::common::Reg<regs::ApbStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn adc_ch0_cfg(self) -> crate::common::Reg<regs::AdcCh0Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn adc_ch1_cfg(self) -> crate::common::Reg<regs::AdcCh1Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_ch0_cfg(self) -> crate::common::Reg<regs::DacCh0Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_ch0_cfg_ext(
        self,
    ) -> crate::common::Reg<regs::DacCh0CfgExt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_ch1_cfg(self) -> crate::common::Reg<regs::DacCh1Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_ch1_cfg_ext(
        self,
    ) -> crate::common::Reg<regs::DacCh1CfgExt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn adc_ch0_entry(self) -> crate::common::Reg<regs::AdcCh0Entry, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn adc_ch1_entry(self) -> crate::common::Reg<regs::AdcCh1Entry, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_ch0_entry(self) -> crate::common::Reg<regs::DacCh0Entry, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_ch1_entry(self) -> crate::common::Reg<regs::DacCh1Entry, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_ch0_debug(self) -> crate::common::Reg<regs::DacCh0Debug, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_ch1_debug(self) -> crate::common::Reg<regs::DacCh1Debug, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn dac_ch0_dc(self) -> crate::common::Reg<regs::DacCh0Dc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_ch1_dc(self) -> crate::common::Reg<regs::DacCh1Dc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn common_cfg(self) -> crate::common::Reg<regs::CommonCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn bg_cfg0(self) -> crate::common::Reg<regs::BgCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[inline(always)]
    pub const fn bg_cfg1(self) -> crate::common::Reg<regs::BgCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[inline(always)]
    pub const fn bg_cfg2(self) -> crate::common::Reg<regs::BgCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[inline(always)]
    pub const fn refgen_cfg(self) -> crate::common::Reg<regs::RefgenCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn pll_cfg0(self) -> crate::common::Reg<regs::PllCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[inline(always)]
    pub const fn pll_cfg1(self) -> crate::common::Reg<regs::PllCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[inline(always)]
    pub const fn pll_cfg2(self) -> crate::common::Reg<regs::PllCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[inline(always)]
    pub const fn pll_cfg3(self) -> crate::common::Reg<regs::PllCfg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn pll_cfg4(self) -> crate::common::Reg<regs::PllCfg4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[inline(always)]
    pub const fn pll_cfg5(self) -> crate::common::Reg<regs::PllCfg5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[inline(always)]
    pub const fn pll_cfg6(self) -> crate::common::Reg<regs::PllCfg6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[inline(always)]
    pub const fn pll_stat(self) -> crate::common::Reg<regs::PllStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[inline(always)]
    pub const fn pll_cal_cfg(self) -> crate::common::Reg<regs::PllCalCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[inline(always)]
    pub const fn pll_cal_result(self) -> crate::common::Reg<regs::PllCalResult, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[inline(always)]
    pub const fn adc_ana_cfg(self) -> crate::common::Reg<regs::AdcAnaCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[inline(always)]
    pub const fn adc1_cfg1(self) -> crate::common::Reg<regs::Adc1Cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[inline(always)]
    pub const fn adc1_cfg2(self) -> crate::common::Reg<regs::Adc1Cfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[inline(always)]
    pub const fn adc2_cfg1(self) -> crate::common::Reg<regs::Adc2Cfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[inline(always)]
    pub const fn adc2_cfg2(self) -> crate::common::Reg<regs::Adc2Cfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[inline(always)]
    pub const fn dac1_cfg(self) -> crate::common::Reg<regs::Dac1Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn dac2_cfg(self) -> crate::common::Reg<regs::Dac2Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[inline(always)]
    pub const fn reserved_in0(self) -> crate::common::Reg<regs::ReservedIn0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[inline(always)]
    pub const fn reserved_in1(self) -> crate::common::Reg<regs::ReservedIn1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[inline(always)]
    pub const fn reserved_out(self) -> crate::common::Reg<regs::ReservedOut, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
}
pub mod regs;
