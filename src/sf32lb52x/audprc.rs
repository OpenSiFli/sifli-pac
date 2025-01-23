#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Audprc {
    ptr: *mut u8,
}
unsafe impl Send for Audprc {}
unsafe impl Sync for Audprc {}
impl Audprc {
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn stb(self) -> crate::common::Reg<regs::Stb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn irq(self) -> crate::common::Reg<regs::Irq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn tx_ch0_cfg(self) -> crate::common::Reg<regs::TxCh0Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_ch0_entry(self) -> crate::common::Reg<regs::TxCh0Entry, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_ch1_cfg(self) -> crate::common::Reg<regs::TxCh1Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_ch1_entry(self) -> crate::common::Reg<regs::TxCh1Entry, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn tx_ch2_cfg(self) -> crate::common::Reg<regs::TxCh2Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_ch2_entry(self) -> crate::common::Reg<regs::TxCh2Entry, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_ch3_cfg(self) -> crate::common::Reg<regs::TxCh3Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_ch3_entry(self) -> crate::common::Reg<regs::TxCh3Entry, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn rx_ch0_cfg(self) -> crate::common::Reg<regs::RxCh0Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn rx_ch0_entry(self) -> crate::common::Reg<regs::RxCh0Entry, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn rx_ch1_cfg(self) -> crate::common::Reg<regs::RxCh1Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn rx_ch1_entry(self) -> crate::common::Reg<regs::RxCh1Entry, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn tx_out_ch0_cfg(self) -> crate::common::Reg<regs::TxOutCh0Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_out_ch0_entry(
        self,
    ) -> crate::common::Reg<regs::TxOutCh0Entry, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_out_ch1_cfg(self) -> crate::common::Reg<regs::TxOutCh1Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_out_ch1_entry(
        self,
    ) -> crate::common::Reg<regs::TxOutCh1Entry, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[inline(always)]
    pub const fn dac_path_cfg0(self) -> crate::common::Reg<regs::DacPathCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_path_cfg1(self) -> crate::common::Reg<regs::DacPathCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_path_cfg2(self) -> crate::common::Reg<regs::DacPathCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_path_cfg3(self) -> crate::common::Reg<regs::DacPathCfg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn adc_path_cfg0(self) -> crate::common::Reg<regs::AdcPathCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg0(self) -> crate::common::Reg<regs::DacEqCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg1(self) -> crate::common::Reg<regs::DacEqCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg2(self) -> crate::common::Reg<regs::DacEqCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg3(self) -> crate::common::Reg<regs::DacEqCfg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg4(self) -> crate::common::Reg<regs::DacEqCfg4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg5(self) -> crate::common::Reg<regs::DacEqCfg5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg6(self) -> crate::common::Reg<regs::DacEqCfg6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg7(self) -> crate::common::Reg<regs::DacEqCfg7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg8(self) -> crate::common::Reg<regs::DacEqCfg8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg9(self) -> crate::common::Reg<regs::DacEqCfg9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg10(self) -> crate::common::Reg<regs::DacEqCfg10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg11(self) -> crate::common::Reg<regs::DacEqCfg11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg12(self) -> crate::common::Reg<regs::DacEqCfg12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg13(self) -> crate::common::Reg<regs::DacEqCfg13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg14(self) -> crate::common::Reg<regs::DacEqCfg14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg15(self) -> crate::common::Reg<regs::DacEqCfg15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg16(self) -> crate::common::Reg<regs::DacEqCfg16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg17(self) -> crate::common::Reg<regs::DacEqCfg17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg18(self) -> crate::common::Reg<regs::DacEqCfg18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg19(self) -> crate::common::Reg<regs::DacEqCfg19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg20(self) -> crate::common::Reg<regs::DacEqCfg20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg21(self) -> crate::common::Reg<regs::DacEqCfg21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg22(self) -> crate::common::Reg<regs::DacEqCfg22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg23(self) -> crate::common::Reg<regs::DacEqCfg23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg24(self) -> crate::common::Reg<regs::DacEqCfg24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg25(self) -> crate::common::Reg<regs::DacEqCfg25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg26(self) -> crate::common::Reg<regs::DacEqCfg26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg27(self) -> crate::common::Reg<regs::DacEqCfg27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg28(self) -> crate::common::Reg<regs::DacEqCfg28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg29(self) -> crate::common::Reg<regs::DacEqCfg29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg30(self) -> crate::common::Reg<regs::DacEqCfg30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg31(self) -> crate::common::Reg<regs::DacEqCfg31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg32(self) -> crate::common::Reg<regs::DacEqCfg32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg33(self) -> crate::common::Reg<regs::DacEqCfg33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg34(self) -> crate::common::Reg<regs::DacEqCfg34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg35(self) -> crate::common::Reg<regs::DacEqCfg35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg36(self) -> crate::common::Reg<regs::DacEqCfg36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg37(self) -> crate::common::Reg<regs::DacEqCfg37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg38(self) -> crate::common::Reg<regs::DacEqCfg38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg39(self) -> crate::common::Reg<regs::DacEqCfg39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg40(self) -> crate::common::Reg<regs::DacEqCfg40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg41(self) -> crate::common::Reg<regs::DacEqCfg41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg42(self) -> crate::common::Reg<regs::DacEqCfg42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg43(self) -> crate::common::Reg<regs::DacEqCfg43, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg44(self) -> crate::common::Reg<regs::DacEqCfg44, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg45(self) -> crate::common::Reg<regs::DacEqCfg45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg46(self) -> crate::common::Reg<regs::DacEqCfg46, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg47(self) -> crate::common::Reg<regs::DacEqCfg47, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg48(self) -> crate::common::Reg<regs::DacEqCfg48, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[inline(always)]
    pub const fn dac_eq_cfg49(self) -> crate::common::Reg<regs::DacEqCfg49, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[inline(always)]
    pub const fn reserved_in(self) -> crate::common::Reg<regs::ReservedIn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[inline(always)]
    pub const fn reserved_out(self) -> crate::common::Reg<regs::ReservedOut, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
}
pub mod regs;
