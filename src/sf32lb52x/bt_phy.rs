#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BtPhy {
    ptr: *mut u8,
}
unsafe impl Send for BtPhy {}
unsafe impl Sync for BtPhy {}
impl BtPhy {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn rx_ctrl1(self) -> crate::common::Reg<regs::RxCtrl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn rx_ctrl2(self) -> crate::common::Reg<regs::RxCtrl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn notch_cfg1(self) -> crate::common::Reg<regs::NotchCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn notch_cfg2(self) -> crate::common::Reg<regs::NotchCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn notch_cfg3(self) -> crate::common::Reg<regs::NotchCfg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn notch_cfg4(self) -> crate::common::Reg<regs::NotchCfg4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn notch_cfg5(self) -> crate::common::Reg<regs::NotchCfg5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn notch_cfg6(self) -> crate::common::Reg<regs::NotchCfg6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn notch_cfg7(self) -> crate::common::Reg<regs::NotchCfg7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn notch_cfg8(self) -> crate::common::Reg<regs::NotchCfg8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn notch_cfg9(self) -> crate::common::Reg<regs::NotchCfg9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn notch_cfg10(self) -> crate::common::Reg<regs::NotchCfg10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn notch_cfg11(self) -> crate::common::Reg<regs::NotchCfg11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn notch_cfg12(self) -> crate::common::Reg<regs::NotchCfg12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn notch_cfg13(self) -> crate::common::Reg<regs::NotchCfg13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn notch_cfg14(self) -> crate::common::Reg<regs::NotchCfg14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[inline(always)]
    pub const fn interp_cfg1(self) -> crate::common::Reg<regs::InterpCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn ted_cfg1(self) -> crate::common::Reg<regs::TedCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[inline(always)]
    pub const fn mixer_cfg1(self) -> crate::common::Reg<regs::MixerCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[inline(always)]
    pub const fn pktdet_cfg1(self) -> crate::common::Reg<regs::PktdetCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[inline(always)]
    pub const fn pktdet_cfg2(self) -> crate::common::Reg<regs::PktdetCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn demod_cfg1(self) -> crate::common::Reg<regs::DemodCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[inline(always)]
    pub const fn demod_cfg2(self) -> crate::common::Reg<regs::DemodCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[inline(always)]
    pub const fn demod_cfg3(self) -> crate::common::Reg<regs::DemodCfg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn demod_cfg4(self) -> crate::common::Reg<regs::DemodCfg4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn demod_cfg5(self) -> crate::common::Reg<regs::DemodCfg5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn demod_cfg6(self) -> crate::common::Reg<regs::DemodCfg6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[inline(always)]
    pub const fn demod_cfg7(self) -> crate::common::Reg<regs::DemodCfg7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[inline(always)]
    pub const fn demod_cfg8(self) -> crate::common::Reg<regs::DemodCfg8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn demod_cfg9(self) -> crate::common::Reg<regs::DemodCfg9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[inline(always)]
    pub const fn demod_cfg10(self) -> crate::common::Reg<regs::DemodCfg10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[inline(always)]
    pub const fn demod_cfg11(self) -> crate::common::Reg<regs::DemodCfg11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[inline(always)]
    pub const fn demod_cfg12(self) -> crate::common::Reg<regs::DemodCfg12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn demod_cfg13(self) -> crate::common::Reg<regs::DemodCfg13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[inline(always)]
    pub const fn demod_cfg14(self) -> crate::common::Reg<regs::DemodCfg14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[inline(always)]
    pub const fn demod_cfg15(self) -> crate::common::Reg<regs::DemodCfg15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[inline(always)]
    pub const fn demod_cfg16(self) -> crate::common::Reg<regs::DemodCfg16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn demod_cfg17(self) -> crate::common::Reg<regs::DemodCfg17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[inline(always)]
    pub const fn rx_status1(self) -> crate::common::Reg<regs::RxStatus1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[inline(always)]
    pub const fn agc_ctrl(self) -> crate::common::Reg<regs::AgcCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[inline(always)]
    pub const fn agc_cfg1(self) -> crate::common::Reg<regs::AgcCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[inline(always)]
    pub const fn agc_cfg2(self) -> crate::common::Reg<regs::AgcCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[inline(always)]
    pub const fn agc_cfg3(self) -> crate::common::Reg<regs::AgcCfg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[inline(always)]
    pub const fn agc_cfg4(self) -> crate::common::Reg<regs::AgcCfg4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[inline(always)]
    pub const fn agc_cfg5(self) -> crate::common::Reg<regs::AgcCfg5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[inline(always)]
    pub const fn agc_cfg6(self) -> crate::common::Reg<regs::AgcCfg6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[inline(always)]
    pub const fn agc_cfg7(self) -> crate::common::Reg<regs::AgcCfg7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[inline(always)]
    pub const fn agc_cfg8(self) -> crate::common::Reg<regs::AgcCfg8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[inline(always)]
    pub const fn agc_cfg9(self) -> crate::common::Reg<regs::AgcCfg9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn agc_cfg10(self) -> crate::common::Reg<regs::AgcCfg10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[inline(always)]
    pub const fn agc_cfg11(self) -> crate::common::Reg<regs::AgcCfg11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[inline(always)]
    pub const fn agc_cfg12(self) -> crate::common::Reg<regs::AgcCfg12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[inline(always)]
    pub const fn rssi_cfg1(self) -> crate::common::Reg<regs::RssiCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[inline(always)]
    pub const fn agc_status(self) -> crate::common::Reg<regs::AgcStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[inline(always)]
    pub const fn edrsync_cfg1(self) -> crate::common::Reg<regs::EdrsyncCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[inline(always)]
    pub const fn edrsync_cfg2(self) -> crate::common::Reg<regs::EdrsyncCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[inline(always)]
    pub const fn edrdemod_cfg1(self) -> crate::common::Reg<regs::EdrdemodCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[inline(always)]
    pub const fn edrdemod_cfg2(self) -> crate::common::Reg<regs::EdrdemodCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[inline(always)]
    pub const fn edrted_cfg1(self) -> crate::common::Reg<regs::EdrtedCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_ctrl(self) -> crate::common::Reg<regs::TxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[inline(always)]
    pub const fn tx_rcc_ctrl(self) -> crate::common::Reg<regs::TxRccCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_gaussflt_cfg1(
        self,
    ) -> crate::common::Reg<regs::TxGaussfltCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_gaussflt_cfg2(
        self,
    ) -> crate::common::Reg<regs::TxGaussfltCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_if_mod_cfg(self) -> crate::common::Reg<regs::TxIfModCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[inline(always)]
    pub const fn tx_if_mod_cfg2(self) -> crate::common::Reg<regs::TxIfModCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_if_mod_cfg3(self) -> crate::common::Reg<regs::TxIfModCfg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_if_mod_cfg4(self) -> crate::common::Reg<regs::TxIfModCfg4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_if_mod_cfg5(self) -> crate::common::Reg<regs::TxIfModCfg5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[inline(always)]
    pub const fn tx_if_mod_cfg6(self) -> crate::common::Reg<regs::TxIfModCfg6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_if_mod_cfg7(self) -> crate::common::Reg<regs::TxIfModCfg7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_hfp_cfg(self) -> crate::common::Reg<regs::TxHfpCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_lfp_cfg(self) -> crate::common::Reg<regs::TxLfpCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[inline(always)]
    pub const fn tx_pa_cfg(self) -> crate::common::Reg<regs::TxPaCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_dpsk_cfg1(self) -> crate::common::Reg<regs::TxDpskCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_dpsk_cfg2(self) -> crate::common::Reg<regs::TxDpskCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_dc_cal_cfg0(self) -> crate::common::Reg<regs::TxDcCalCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[inline(always)]
    pub const fn tx_dc_cal_cfg1(self) -> crate::common::Reg<regs::TxDcCalCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_dc_cal_cfg2(self) -> crate::common::Reg<regs::TxDcCalCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[inline(always)]
    pub const fn lfp_mmdiv_cfg0(self) -> crate::common::Reg<regs::LfpMmdivCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[inline(always)]
    pub const fn lfp_mmdiv_cfg1(self) -> crate::common::Reg<regs::LfpMmdivCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[inline(always)]
    pub const fn lfp_mmdiv_cfg2(self) -> crate::common::Reg<regs::LfpMmdivCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[inline(always)]
    pub const fn lfp_mmdiv_cfg3(self) -> crate::common::Reg<regs::LfpMmdivCfg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[inline(always)]
    pub const fn lfp_mmdiv_cfg4(self) -> crate::common::Reg<regs::LfpMmdivCfg4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[inline(always)]
    pub const fn rx_hfp_cfg(self) -> crate::common::Reg<regs::RxHfpCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[inline(always)]
    pub const fn lna_gain_tbl0(self) -> crate::common::Reg<regs::LnaGainTbl0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[inline(always)]
    pub const fn lna_gain_tbl1(self) -> crate::common::Reg<regs::LnaGainTbl1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[inline(always)]
    pub const fn lna_gain_tbl2(self) -> crate::common::Reg<regs::LnaGainTbl2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[inline(always)]
    pub const fn lna_gain_tbl3(self) -> crate::common::Reg<regs::LnaGainTbl3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[inline(always)]
    pub const fn lna_gain_tbl4(self) -> crate::common::Reg<regs::LnaGainTbl4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[inline(always)]
    pub const fn lna_gain_tbl5(self) -> crate::common::Reg<regs::LnaGainTbl5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[inline(always)]
    pub const fn rcos_cfg0(self) -> crate::common::Reg<regs::RcosCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[inline(always)]
    pub const fn rcos_cfg1(self) -> crate::common::Reg<regs::RcosCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[inline(always)]
    pub const fn rcos_cfg2(self) -> crate::common::Reg<regs::RcosCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[inline(always)]
    pub const fn rcos_cfg3(self) -> crate::common::Reg<regs::RcosCfg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[inline(always)]
    pub const fn rcos_cfg4(self) -> crate::common::Reg<regs::RcosCfg4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[inline(always)]
    pub const fn rcos_cfg5(self) -> crate::common::Reg<regs::RcosCfg5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[inline(always)]
    pub const fn rcos_cfg6(self) -> crate::common::Reg<regs::RcosCfg6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[inline(always)]
    pub const fn rcos_cfg7(self) -> crate::common::Reg<regs::RcosCfg7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[inline(always)]
    pub const fn rcos_cfg8(self) -> crate::common::Reg<regs::RcosCfg8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[inline(always)]
    pub const fn rcos_cfg9(self) -> crate::common::Reg<regs::RcosCfg9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[inline(always)]
    pub const fn rcos_cfg10(self) -> crate::common::Reg<regs::RcosCfg10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[inline(always)]
    pub const fn rcos_cfg11(self) -> crate::common::Reg<regs::RcosCfg11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[inline(always)]
    pub const fn rcos_cfg12(self) -> crate::common::Reg<regs::RcosCfg12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[inline(always)]
    pub const fn rcos_cfg13(self) -> crate::common::Reg<regs::RcosCfg13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[inline(always)]
    pub const fn rcos_cfg14(self) -> crate::common::Reg<regs::RcosCfg14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[inline(always)]
    pub const fn rcos_cfg15(self) -> crate::common::Reg<regs::RcosCfg15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[inline(always)]
    pub const fn rcos_cfg16(self) -> crate::common::Reg<regs::RcosCfg16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[inline(always)]
    pub const fn rcos_cfg17(self) -> crate::common::Reg<regs::RcosCfg17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[inline(always)]
    pub const fn rcos_cfg18(self) -> crate::common::Reg<regs::RcosCfg18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[inline(always)]
    pub const fn lpf_cfg0(self) -> crate::common::Reg<regs::LpfCfg0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[inline(always)]
    pub const fn lpf_cfg1(self) -> crate::common::Reg<regs::LpfCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[inline(always)]
    pub const fn lpf_cfg2(self) -> crate::common::Reg<regs::LpfCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[inline(always)]
    pub const fn lpf_cfg3(self) -> crate::common::Reg<regs::LpfCfg3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
}
pub mod regs;
