#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2s {
    ptr: *mut u8,
}
unsafe impl Send for I2s {}
unsafe impl Sync for I2s {}
impl I2s {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn tx_pcm_format(self) -> crate::common::Reg<regs::TxPcmFormat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_pcm_sample_clk(
        self,
    ) -> crate::common::Reg<regs::TxPcmSampleClk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_rs_smooth(self) -> crate::common::Reg<regs::TxRsSmooth, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_pcm_ch_sel(self) -> crate::common::Reg<regs::TxPcmChSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_vol_ctrl(self) -> crate::common::Reg<regs::TxVolCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_lr_bal_ctrl(self) -> crate::common::Reg<regs::TxLrBalCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn audio_tx_lrck_div(
        self,
    ) -> crate::common::Reg<regs::AudioTxLrckDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn audio_tx_bclk_div(
        self,
    ) -> crate::common::Reg<regs::AudioTxBclkDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn audio_tx_format(
        self,
    ) -> crate::common::Reg<regs::AudioTxFormat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[inline(always)]
    pub const fn audio_serial_timing(
        self,
    ) -> crate::common::Reg<regs::AudioSerialTiming, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[inline(always)]
    pub const fn audio_tx_func_en(
        self,
    ) -> crate::common::Reg<regs::AudioTxFuncEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[inline(always)]
    pub const fn audio_tx_pause(self) -> crate::common::Reg<regs::AudioTxPause, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[inline(always)]
    pub const fn audio_i2s_sl_merge(
        self,
    ) -> crate::common::Reg<regs::AudioI2sSlMerge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[inline(always)]
    pub const fn audio_rx_func_en(
        self,
    ) -> crate::common::Reg<regs::AudioRxFuncEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[inline(always)]
    pub const fn audio_rx_pause(self) -> crate::common::Reg<regs::AudioRxPause, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[inline(always)]
    pub const fn audio_rx_serial_timing(
        self,
    ) -> crate::common::Reg<regs::AudioRxSerialTiming, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[inline(always)]
    pub const fn audio_rx_pcm_dw(
        self,
    ) -> crate::common::Reg<regs::AudioRxPcmDw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[inline(always)]
    pub const fn audio_rx_lrck_div(
        self,
    ) -> crate::common::Reg<regs::AudioRxLrckDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[inline(always)]
    pub const fn audio_rx_bclk_div(
        self,
    ) -> crate::common::Reg<regs::AudioRxBclkDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[inline(always)]
    pub const fn record_data_sel(
        self,
    ) -> crate::common::Reg<regs::RecordDataSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[inline(always)]
    pub const fn rx_re_sample_clk_div(
        self,
    ) -> crate::common::Reg<regs::RxReSampleClkDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0170usize) as _) }
    }
    #[inline(always)]
    pub const fn rx_re_sample(self) -> crate::common::Reg<regs::RxReSample, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[inline(always)]
    pub const fn record_format(self) -> crate::common::Reg<regs::RecordFormat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
    #[inline(always)]
    pub const fn rx_ch_sel(self) -> crate::common::Reg<regs::RxChSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a0usize) as _) }
    }
    #[inline(always)]
    pub const fn bt_phone_ctrl(self) -> crate::common::Reg<regs::BtPhoneCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[inline(always)]
    pub const fn bb_pcm_format(self) -> crate::common::Reg<regs::BbPcmFormat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[inline(always)]
    pub const fn bt_pcm_dw(self) -> crate::common::Reg<regs::BtPcmDw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0220usize) as _) }
    }
    #[inline(always)]
    pub const fn bt_pcm_timing(self) -> crate::common::Reg<regs::BtPcmTiming, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0230usize) as _) }
    }
    #[inline(always)]
    pub const fn bt_pcm_clk_duty(
        self,
    ) -> crate::common::Reg<regs::BtPcmClkDuty, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0240usize) as _) }
    }
    #[inline(always)]
    pub const fn bt_pcm_sync_duty(
        self,
    ) -> crate::common::Reg<regs::BtPcmSyncDuty, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0250usize) as _) }
    }
    #[inline(always)]
    pub const fn bt_vol_ctrl(self) -> crate::common::Reg<regs::BtVolCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0260usize) as _) }
    }
    #[inline(always)]
    pub const fn int_mask(self) -> crate::common::Reg<regs::IntMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[inline(always)]
    pub const fn int_status(self) -> crate::common::Reg<regs::IntStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0310usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_dma_entry(self) -> crate::common::Reg<regs::TxDmaEntry, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[inline(always)]
    pub const fn rx_dma_entry(self) -> crate::common::Reg<regs::RxDmaEntry, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0440usize) as _) }
    }
    #[inline(always)]
    pub const fn dma_mask(self) -> crate::common::Reg<regs::DmaMask, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0480usize) as _) }
    }
    #[inline(always)]
    pub const fn debug_loop(self) -> crate::common::Reg<regs::DebugLoop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[inline(always)]
    pub const fn fifo_status(self) -> crate::common::Reg<regs::FifoStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_equalizer_en(
        self,
    ) -> crate::common::Reg<regs::TxEqualizerEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0700usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_equalizer_gain1(
        self,
    ) -> crate::common::Reg<regs::TxEqualizerGain1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0710usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_equalizer_gain2(
        self,
    ) -> crate::common::Reg<regs::TxEqualizerGain2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0720usize) as _) }
    }
}
pub mod regs;
