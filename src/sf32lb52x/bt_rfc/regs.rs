#[doc = "ADC_REG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcReg(pub u32);
impl AdcReg {
    #[must_use]
    #[inline(always)]
    pub const fn brf_sel_ldovref_adcref_lv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_sel_ldovref_adcref_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_en_ldo_adcref_lv(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_en_ldo_adcref_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_sel_ldovref_adc_lv(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_sel_ldovref_adc_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_en_ldo_adc_lv(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_en_ldo_adc_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_rstb_adc_lv(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_rstb_adc_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_adc_vsp_lv(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_adc_vsp_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_adc_cmpcl_lv(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_adc_cmpcl_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_adc_cmm_lv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_adc_cmm_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_en_adc_q_lv(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_en_adc_q_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_en_adc_i_lv(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_en_adc_i_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for AdcReg {
    #[inline(always)]
    fn default() -> AdcReg {
        AdcReg(0)
    }
}
impl core::fmt::Debug for AdcReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcReg")
            .field(
                "brf_sel_ldovref_adcref_lv",
                &self.brf_sel_ldovref_adcref_lv(),
            )
            .field("brf_en_ldo_adcref_lv", &self.brf_en_ldo_adcref_lv())
            .field("brf_sel_ldovref_adc_lv", &self.brf_sel_ldovref_adc_lv())
            .field("brf_en_ldo_adc_lv", &self.brf_en_ldo_adc_lv())
            .field("brf_rstb_adc_lv", &self.brf_rstb_adc_lv())
            .field("brf_adc_vsp_lv", &self.brf_adc_vsp_lv())
            .field("brf_adc_cmpcl_lv", &self.brf_adc_cmpcl_lv())
            .field("brf_adc_cmm_lv", &self.brf_adc_cmm_lv())
            .field("brf_en_adc_q_lv", &self.brf_en_adc_q_lv())
            .field("brf_en_adc_i_lv", &self.brf_en_adc_i_lv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcReg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "AdcReg {{ brf_sel_ldovref_adcref_lv: {=u8:?}, brf_en_ldo_adcref_lv: {=bool:?}, brf_sel_ldovref_adc_lv: {=u8:?}, brf_en_ldo_adc_lv: {=bool:?}, brf_rstb_adc_lv: {=bool:?}, brf_adc_vsp_lv: {=u8:?}, brf_adc_cmpcl_lv: {=u8:?}, brf_adc_cmm_lv: {=u8:?}, brf_en_adc_q_lv: {=bool:?}, brf_en_adc_i_lv: {=bool:?} }}" , self . brf_sel_ldovref_adcref_lv () , self . brf_en_ldo_adcref_lv () , self . brf_sel_ldovref_adc_lv () , self . brf_en_ldo_adc_lv () , self . brf_rstb_adc_lv () , self . brf_adc_vsp_lv () , self . brf_adc_cmpcl_lv () , self . brf_adc_cmm_lv () , self . brf_en_adc_q_lv () , self . brf_en_adc_i_lv ())
    }
}
#[doc = "AGC_REG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AgcReg(pub u32);
impl AgcReg {
    #[must_use]
    #[inline(always)]
    pub const fn lna_gain_frc_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_lna_gain_frc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cbpf_gain_frc_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cbpf_gain_frc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vga_gain_frc_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_vga_gain_frc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lna_gc(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_lna_gc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cbpf_gc(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_cbpf_gc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vga_gc(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_vga_gc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u32) & 0x1f) << 9usize);
    }
}
impl Default for AgcReg {
    #[inline(always)]
    fn default() -> AgcReg {
        AgcReg(0)
    }
}
impl core::fmt::Debug for AgcReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AgcReg")
            .field("lna_gain_frc_en", &self.lna_gain_frc_en())
            .field("cbpf_gain_frc_en", &self.cbpf_gain_frc_en())
            .field("vga_gain_frc_en", &self.vga_gain_frc_en())
            .field("lna_gc", &self.lna_gc())
            .field("cbpf_gc", &self.cbpf_gc())
            .field("vga_gc", &self.vga_gc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AgcReg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "AgcReg {{ lna_gain_frc_en: {=bool:?}, cbpf_gain_frc_en: {=bool:?}, vga_gain_frc_en: {=bool:?}, lna_gc: {=u8:?}, cbpf_gc: {=u8:?}, vga_gc: {=u8:?} }}" , self . lna_gain_frc_en () , self . cbpf_gain_frc_en () , self . vga_gain_frc_en () , self . lna_gc () , self . cbpf_gc () , self . vga_gc ())
    }
}
#[doc = "ATEST_REG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AtestReg(pub u32);
impl AtestReg {
    #[must_use]
    #[inline(always)]
    pub const fn brf_dc_tr_lv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_dc_tr_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_dc_br_lv(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_dc_br_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_dc_mr_lv(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_dc_mr_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
}
impl Default for AtestReg {
    #[inline(always)]
    fn default() -> AtestReg {
        AtestReg(0)
    }
}
impl core::fmt::Debug for AtestReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AtestReg")
            .field("brf_dc_tr_lv", &self.brf_dc_tr_lv())
            .field("brf_dc_br_lv", &self.brf_dc_br_lv())
            .field("brf_dc_mr_lv", &self.brf_dc_mr_lv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AtestReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AtestReg {{ brf_dc_tr_lv: {=u8:?}, brf_dc_br_lv: {=u8:?}, brf_dc_mr_lv: {=u8:?} }}",
            self.brf_dc_tr_lv(),
            self.brf_dc_br_lv(),
            self.brf_dc_mr_lv()
        )
    }
}
#[doc = "ATSTBUF_REG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AtstbufReg(pub u32);
impl AtstbufReg {
    #[must_use]
    #[inline(always)]
    pub const fn brf_atstbuf_w2x_stg2_lv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_atstbuf_w2x_stg2_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_atstbuf_w2x_stg1_lv(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_atstbuf_w2x_stg1_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_atstbuf_vstart_lv(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_atstbuf_vstart_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_atstbuf_vcmref_lv(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_atstbuf_vcmref_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_atstbuf_bm_lv(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_atstbuf_bm_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_atstbuf_rz_lv(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_atstbuf_rz_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_atstbuf_cc_lv(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_atstbuf_cc_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_atstbuf_cfman_lv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_atstbuf_cfman_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_atstbuf_man_cfsel_lv(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_atstbuf_man_cfsel_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_atstbuf_gc_lv(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_atstbuf_gc_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_atstbuf_ch_sel_lv(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_atstbuf_ch_sel_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_en_atstbuf_lv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_en_atstbuf_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for AtstbufReg {
    #[inline(always)]
    fn default() -> AtstbufReg {
        AtstbufReg(0)
    }
}
impl core::fmt::Debug for AtstbufReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AtstbufReg")
            .field("brf_atstbuf_w2x_stg2_lv", &self.brf_atstbuf_w2x_stg2_lv())
            .field("brf_atstbuf_w2x_stg1_lv", &self.brf_atstbuf_w2x_stg1_lv())
            .field("brf_atstbuf_vstart_lv", &self.brf_atstbuf_vstart_lv())
            .field("brf_atstbuf_vcmref_lv", &self.brf_atstbuf_vcmref_lv())
            .field("brf_atstbuf_bm_lv", &self.brf_atstbuf_bm_lv())
            .field("brf_atstbuf_rz_lv", &self.brf_atstbuf_rz_lv())
            .field("brf_atstbuf_cc_lv", &self.brf_atstbuf_cc_lv())
            .field("brf_atstbuf_cfman_lv", &self.brf_atstbuf_cfman_lv())
            .field("brf_atstbuf_man_cfsel_lv", &self.brf_atstbuf_man_cfsel_lv())
            .field("brf_atstbuf_gc_lv", &self.brf_atstbuf_gc_lv())
            .field("brf_atstbuf_ch_sel_lv", &self.brf_atstbuf_ch_sel_lv())
            .field("brf_en_atstbuf_lv", &self.brf_en_atstbuf_lv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AtstbufReg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "AtstbufReg {{ brf_atstbuf_w2x_stg2_lv: {=bool:?}, brf_atstbuf_w2x_stg1_lv: {=bool:?}, brf_atstbuf_vstart_lv: {=u8:?}, brf_atstbuf_vcmref_lv: {=u8:?}, brf_atstbuf_bm_lv: {=u8:?}, brf_atstbuf_rz_lv: {=u8:?}, brf_atstbuf_cc_lv: {=u8:?}, brf_atstbuf_cfman_lv: {=u8:?}, brf_atstbuf_man_cfsel_lv: {=bool:?}, brf_atstbuf_gc_lv: {=u8:?}, brf_atstbuf_ch_sel_lv: {=bool:?}, brf_en_atstbuf_lv: {=bool:?} }}" , self . brf_atstbuf_w2x_stg2_lv () , self . brf_atstbuf_w2x_stg1_lv () , self . brf_atstbuf_vstart_lv () , self . brf_atstbuf_vcmref_lv () , self . brf_atstbuf_bm_lv () , self . brf_atstbuf_rz_lv () , self . brf_atstbuf_cc_lv () , self . brf_atstbuf_cfman_lv () , self . brf_atstbuf_man_cfsel_lv () , self . brf_atstbuf_gc_lv () , self . brf_atstbuf_ch_sel_lv () , self . brf_en_atstbuf_lv ())
    }
}
#[doc = "CAL_ADDR_REG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CalAddrReg1(pub u32);
impl CalAddrReg1 {
    #[must_use]
    #[inline(always)]
    pub const fn ble_rx_cal_addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ble_rx_cal_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn bt_rx_cal_addr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_bt_rx_cal_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for CalAddrReg1 {
    #[inline(always)]
    fn default() -> CalAddrReg1 {
        CalAddrReg1(0)
    }
}
impl core::fmt::Debug for CalAddrReg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CalAddrReg1")
            .field("ble_rx_cal_addr", &self.ble_rx_cal_addr())
            .field("bt_rx_cal_addr", &self.bt_rx_cal_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CalAddrReg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CalAddrReg1 {{ ble_rx_cal_addr: {=u16:?}, bt_rx_cal_addr: {=u16:?} }}",
            self.ble_rx_cal_addr(),
            self.bt_rx_cal_addr()
        )
    }
}
#[doc = "CAL_ADDR_REG2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CalAddrReg2(pub u32);
impl CalAddrReg2 {
    #[must_use]
    #[inline(always)]
    pub const fn ble_tx_cal_addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ble_tx_cal_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn bt_tx_cal_addr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_bt_tx_cal_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for CalAddrReg2 {
    #[inline(always)]
    fn default() -> CalAddrReg2 {
        CalAddrReg2(0)
    }
}
impl core::fmt::Debug for CalAddrReg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CalAddrReg2")
            .field("ble_tx_cal_addr", &self.ble_tx_cal_addr())
            .field("bt_tx_cal_addr", &self.bt_tx_cal_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CalAddrReg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CalAddrReg2 {{ ble_tx_cal_addr: {=u16:?}, bt_tx_cal_addr: {=u16:?} }}",
            self.ble_tx_cal_addr(),
            self.bt_tx_cal_addr()
        )
    }
}
#[doc = "CAL_ADDR_REG3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CalAddrReg3(pub u32);
impl CalAddrReg3 {
    #[must_use]
    #[inline(always)]
    pub const fn txdc_cal_addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_txdc_cal_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for CalAddrReg3 {
    #[inline(always)]
    fn default() -> CalAddrReg3 {
        CalAddrReg3(0)
    }
}
impl core::fmt::Debug for CalAddrReg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CalAddrReg3")
            .field("txdc_cal_addr", &self.txdc_cal_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CalAddrReg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CalAddrReg3 {{ txdc_cal_addr: {=u16:?} }}",
            self.txdc_cal_addr()
        )
    }
}
#[doc = "CU_ADDR_REG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CuAddrReg1(pub u32);
impl CuAddrReg1 {
    #[must_use]
    #[inline(always)]
    pub const fn rxon_cfg_addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rxon_cfg_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rxoff_cfg_addr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rxoff_cfg_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for CuAddrReg1 {
    #[inline(always)]
    fn default() -> CuAddrReg1 {
        CuAddrReg1(0)
    }
}
impl core::fmt::Debug for CuAddrReg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CuAddrReg1")
            .field("rxon_cfg_addr", &self.rxon_cfg_addr())
            .field("rxoff_cfg_addr", &self.rxoff_cfg_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CuAddrReg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CuAddrReg1 {{ rxon_cfg_addr: {=u16:?}, rxoff_cfg_addr: {=u16:?} }}",
            self.rxon_cfg_addr(),
            self.rxoff_cfg_addr()
        )
    }
}
#[doc = "CU_ADDR_REG2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CuAddrReg2(pub u32);
impl CuAddrReg2 {
    #[must_use]
    #[inline(always)]
    pub const fn txon_cfg_addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_txon_cfg_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn txoff_cfg_addr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_txoff_cfg_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for CuAddrReg2 {
    #[inline(always)]
    fn default() -> CuAddrReg2 {
        CuAddrReg2(0)
    }
}
impl core::fmt::Debug for CuAddrReg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CuAddrReg2")
            .field("txon_cfg_addr", &self.txon_cfg_addr())
            .field("txoff_cfg_addr", &self.txoff_cfg_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CuAddrReg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CuAddrReg2 {{ txon_cfg_addr: {=u16:?}, txoff_cfg_addr: {=u16:?} }}",
            self.txon_cfg_addr(),
            self.txoff_cfg_addr()
        )
    }
}
#[doc = "CU_ADDR_REG3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CuAddrReg3(pub u32);
impl CuAddrReg3 {
    #[must_use]
    #[inline(always)]
    pub const fn bt_txon_cfg_addr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_bt_txon_cfg_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn bt_txoff_cfg_addr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_bt_txoff_cfg_addr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for CuAddrReg3 {
    #[inline(always)]
    fn default() -> CuAddrReg3 {
        CuAddrReg3(0)
    }
}
impl core::fmt::Debug for CuAddrReg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CuAddrReg3")
            .field("bt_txon_cfg_addr", &self.bt_txon_cfg_addr())
            .field("bt_txoff_cfg_addr", &self.bt_txoff_cfg_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CuAddrReg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CuAddrReg3 {{ bt_txon_cfg_addr: {=u16:?}, bt_txoff_cfg_addr: {=u16:?} }}",
            self.bt_txon_cfg_addr(),
            self.bt_txoff_cfg_addr()
        )
    }
}
#[doc = "DTEST_REG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DtestReg(pub u32);
impl DtestReg {
    #[must_use]
    #[inline(always)]
    pub const fn brf_fbdv_dtest_tr_lv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_fbdv_dtest_tr_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_fbdv_dtest_en_lv(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_fbdv_dtest_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for DtestReg {
    #[inline(always)]
    fn default() -> DtestReg {
        DtestReg(0)
    }
}
impl core::fmt::Debug for DtestReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DtestReg")
            .field("brf_fbdv_dtest_tr_lv", &self.brf_fbdv_dtest_tr_lv())
            .field("brf_fbdv_dtest_en_lv", &self.brf_fbdv_dtest_en_lv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DtestReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DtestReg {{ brf_fbdv_dtest_tr_lv: {=u8:?}, brf_fbdv_dtest_en_lv: {=bool:?} }}",
            self.brf_fbdv_dtest_tr_lv(),
            self.brf_fbdv_dtest_en_lv()
        )
    }
}
#[doc = "EDR_CAL_REG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EdrCalReg1(pub u32);
impl EdrCalReg1 {
    #[must_use]
    #[inline(always)]
    pub const fn brf_edr_vco_pdx_lv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_edr_vco_pdx_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_edr_vco_idac_lv(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_edr_vco_idac_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_oslo_fc_lv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_oslo_fc_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_oslo_bm_lv(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_oslo_bm_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_edr_tmxcap_sel_lv(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_trf_edr_tmxcap_sel_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for EdrCalReg1 {
    #[inline(always)]
    fn default() -> EdrCalReg1 {
        EdrCalReg1(0)
    }
}
impl core::fmt::Debug for EdrCalReg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EdrCalReg1")
            .field("brf_edr_vco_pdx_lv", &self.brf_edr_vco_pdx_lv())
            .field("brf_edr_vco_idac_lv", &self.brf_edr_vco_idac_lv())
            .field("brf_oslo_fc_lv", &self.brf_oslo_fc_lv())
            .field("brf_oslo_bm_lv", &self.brf_oslo_bm_lv())
            .field(
                "brf_trf_edr_tmxcap_sel_lv",
                &self.brf_trf_edr_tmxcap_sel_lv(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EdrCalReg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "EdrCalReg1 {{ brf_edr_vco_pdx_lv: {=u8:?}, brf_edr_vco_idac_lv: {=u8:?}, brf_oslo_fc_lv: {=u8:?}, brf_oslo_bm_lv: {=u8:?}, brf_trf_edr_tmxcap_sel_lv: {=u8:?} }}" , self . brf_edr_vco_pdx_lv () , self . brf_edr_vco_idac_lv () , self . brf_oslo_fc_lv () , self . brf_oslo_bm_lv () , self . brf_trf_edr_tmxcap_sel_lv ())
    }
}
#[doc = "FBDV_REG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FbdvReg1(pub u32);
impl FbdvReg1 {
    #[must_use]
    #[inline(always)]
    pub const fn brf_fkcal_cnt_rdy_lv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_fkcal_cnt_rdy_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_fkcal_cnt_rstb_lv(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_fkcal_cnt_rstb_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_fkcal_cnt_en_lv(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_fkcal_cnt_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_sdm_clk_sel_lv(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_sdm_clk_sel_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_fbdv_mod_stg_lv(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_fbdv_mod_stg_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_fbdv_rstb_sync_en_lv(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_fbdv_rstb_sync_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_fbdv_rstb_lv(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_fbdv_rstb_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_fbdv_ldo_vref_lv(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_fbdv_ldo_vref_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_fbdv_en_lv(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_fbdv_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for FbdvReg1 {
    #[inline(always)]
    fn default() -> FbdvReg1 {
        FbdvReg1(0)
    }
}
impl core::fmt::Debug for FbdvReg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FbdvReg1")
            .field("brf_fkcal_cnt_rdy_lv", &self.brf_fkcal_cnt_rdy_lv())
            .field("brf_fkcal_cnt_rstb_lv", &self.brf_fkcal_cnt_rstb_lv())
            .field("brf_fkcal_cnt_en_lv", &self.brf_fkcal_cnt_en_lv())
            .field("brf_sdm_clk_sel_lv", &self.brf_sdm_clk_sel_lv())
            .field("brf_fbdv_mod_stg_lv", &self.brf_fbdv_mod_stg_lv())
            .field("brf_fbdv_rstb_sync_en_lv", &self.brf_fbdv_rstb_sync_en_lv())
            .field("brf_fbdv_rstb_lv", &self.brf_fbdv_rstb_lv())
            .field("brf_fbdv_ldo_vref_lv", &self.brf_fbdv_ldo_vref_lv())
            .field("brf_fbdv_en_lv", &self.brf_fbdv_en_lv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FbdvReg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "FbdvReg1 {{ brf_fkcal_cnt_rdy_lv: {=bool:?}, brf_fkcal_cnt_rstb_lv: {=bool:?}, brf_fkcal_cnt_en_lv: {=bool:?}, brf_sdm_clk_sel_lv: {=bool:?}, brf_fbdv_mod_stg_lv: {=u8:?}, brf_fbdv_rstb_sync_en_lv: {=bool:?}, brf_fbdv_rstb_lv: {=bool:?}, brf_fbdv_ldo_vref_lv: {=u8:?}, brf_fbdv_en_lv: {=bool:?} }}" , self . brf_fkcal_cnt_rdy_lv () , self . brf_fkcal_cnt_rstb_lv () , self . brf_fkcal_cnt_en_lv () , self . brf_sdm_clk_sel_lv () , self . brf_fbdv_mod_stg_lv () , self . brf_fbdv_rstb_sync_en_lv () , self . brf_fbdv_rstb_lv () , self . brf_fbdv_ldo_vref_lv () , self . brf_fbdv_en_lv ())
    }
}
#[doc = "FBDV_REG2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FbdvReg2(pub u32);
impl FbdvReg2 {
    #[must_use]
    #[inline(always)]
    pub const fn brf_fkcal_cnt_divn_lv(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_brf_fkcal_cnt_divn_lv(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_fkcal_cnt_op_lv(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_brf_fkcal_cnt_op_lv(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for FbdvReg2 {
    #[inline(always)]
    fn default() -> FbdvReg2 {
        FbdvReg2(0)
    }
}
impl core::fmt::Debug for FbdvReg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FbdvReg2")
            .field("brf_fkcal_cnt_divn_lv", &self.brf_fkcal_cnt_divn_lv())
            .field("brf_fkcal_cnt_op_lv", &self.brf_fkcal_cnt_op_lv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FbdvReg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FbdvReg2 {{ brf_fkcal_cnt_divn_lv: {=u16:?}, brf_fkcal_cnt_op_lv: {=u16:?} }}",
            self.brf_fkcal_cnt_divn_lv(),
            self.brf_fkcal_cnt_op_lv()
        )
    }
}
#[doc = "INCCAL_REG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InccalReg1(pub u32);
impl InccalReg1 {
    #[must_use]
    #[inline(always)]
    pub const fn vco3g_auto_incacal_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_vco3g_auto_incacal_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vco3g_auto_incfcal_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_vco3g_auto_incfcal_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vco3g_incacal_wait_time(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_vco3g_incacal_wait_time(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vco3g_incfcal_wait_time(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_vco3g_incfcal_wait_time(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vco3g_idac_offset(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_vco3g_idac_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 14usize)) | (((val as u32) & 0x7f) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vco3g_pdx_offset(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_vco3g_pdx_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 21usize)) | (((val as u32) & 0xff) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn inccal_start(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_inccal_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn frc_inccal_clk_on(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_frc_inccal_clk_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for InccalReg1 {
    #[inline(always)]
    fn default() -> InccalReg1 {
        InccalReg1(0)
    }
}
impl core::fmt::Debug for InccalReg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("InccalReg1")
            .field("vco3g_auto_incacal_en", &self.vco3g_auto_incacal_en())
            .field("vco3g_auto_incfcal_en", &self.vco3g_auto_incfcal_en())
            .field("vco3g_incacal_wait_time", &self.vco3g_incacal_wait_time())
            .field("vco3g_incfcal_wait_time", &self.vco3g_incfcal_wait_time())
            .field("vco3g_idac_offset", &self.vco3g_idac_offset())
            .field("vco3g_pdx_offset", &self.vco3g_pdx_offset())
            .field("inccal_start", &self.inccal_start())
            .field("frc_inccal_clk_on", &self.frc_inccal_clk_on())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for InccalReg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "InccalReg1 {{ vco3g_auto_incacal_en: {=bool:?}, vco3g_auto_incfcal_en: {=bool:?}, vco3g_incacal_wait_time: {=u8:?}, vco3g_incfcal_wait_time: {=u8:?}, vco3g_idac_offset: {=u8:?}, vco3g_pdx_offset: {=u8:?}, inccal_start: {=bool:?}, frc_inccal_clk_on: {=bool:?} }}" , self . vco3g_auto_incacal_en () , self . vco3g_auto_incfcal_en () , self . vco3g_incacal_wait_time () , self . vco3g_incfcal_wait_time () , self . vco3g_idac_offset () , self . vco3g_pdx_offset () , self . inccal_start () , self . frc_inccal_clk_on ())
    }
}
#[doc = "INCCAL_REG2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InccalReg2(pub u32);
impl InccalReg2 {
    #[must_use]
    #[inline(always)]
    pub const fn vco5g_auto_incacal_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_vco5g_auto_incacal_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vco5g_auto_incfcal_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_vco5g_auto_incfcal_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vco5g_incacal_wait_time(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_vco5g_incacal_wait_time(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vco5g_incfcal_wait_time(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_vco5g_incfcal_wait_time(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vco5g_idac_offset(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_vco5g_idac_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 14usize)) | (((val as u32) & 0x7f) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vco5g_pdx_offset(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_vco5g_pdx_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 21usize)) | (((val as u32) & 0xff) << 21usize);
    }
}
impl Default for InccalReg2 {
    #[inline(always)]
    fn default() -> InccalReg2 {
        InccalReg2(0)
    }
}
impl core::fmt::Debug for InccalReg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("InccalReg2")
            .field("vco5g_auto_incacal_en", &self.vco5g_auto_incacal_en())
            .field("vco5g_auto_incfcal_en", &self.vco5g_auto_incfcal_en())
            .field("vco5g_incacal_wait_time", &self.vco5g_incacal_wait_time())
            .field("vco5g_incfcal_wait_time", &self.vco5g_incfcal_wait_time())
            .field("vco5g_idac_offset", &self.vco5g_idac_offset())
            .field("vco5g_pdx_offset", &self.vco5g_pdx_offset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for InccalReg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "InccalReg2 {{ vco5g_auto_incacal_en: {=bool:?}, vco5g_auto_incfcal_en: {=bool:?}, vco5g_incacal_wait_time: {=u8:?}, vco5g_incfcal_wait_time: {=u8:?}, vco5g_idac_offset: {=u8:?}, vco5g_pdx_offset: {=u8:?} }}" , self . vco5g_auto_incacal_en () , self . vco5g_auto_incfcal_en () , self . vco5g_incacal_wait_time () , self . vco5g_incfcal_wait_time () , self . vco5g_idac_offset () , self . vco5g_pdx_offset ())
    }
}
#[doc = "IQ_PWR_REG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IqPwrReg1(pub u32);
impl IqPwrReg1 {
    #[must_use]
    #[inline(always)]
    pub const fn tx_dc_cal_coef0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_tx_dc_cal_coef0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_dc_cal_coef1(&self) -> u16 {
        let val = (self.0 >> 14usize) & 0x3fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_tx_dc_cal_coef1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 14usize)) | (((val as u32) & 0x3fff) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn edr_tmxbuf_gc_gfsk(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_edr_tmxbuf_gc_gfsk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for IqPwrReg1 {
    #[inline(always)]
    fn default() -> IqPwrReg1 {
        IqPwrReg1(0)
    }
}
impl core::fmt::Debug for IqPwrReg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IqPwrReg1")
            .field("tx_dc_cal_coef0", &self.tx_dc_cal_coef0())
            .field("tx_dc_cal_coef1", &self.tx_dc_cal_coef1())
            .field("edr_tmxbuf_gc_gfsk", &self.edr_tmxbuf_gc_gfsk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IqPwrReg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "IqPwrReg1 {{ tx_dc_cal_coef0: {=u16:?}, tx_dc_cal_coef1: {=u16:?}, edr_tmxbuf_gc_gfsk: {=u8:?} }}" , self . tx_dc_cal_coef0 () , self . tx_dc_cal_coef1 () , self . edr_tmxbuf_gc_gfsk ())
    }
}
#[doc = "IQ_PWR_REG2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IqPwrReg2(pub u32);
impl IqPwrReg2 {
    #[must_use]
    #[inline(always)]
    pub const fn tx_dc_cal_offset_q(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_tx_dc_cal_offset_q(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_edr_pa_bm_lv(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_trf_edr_pa_bm_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_dc_cal_offset_i(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_tx_dc_cal_offset_i(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn edr_lpf_bypass(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_edr_lpf_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn edr_tmxbuf_gc_dpsk(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_edr_tmxbuf_gc_dpsk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for IqPwrReg2 {
    #[inline(always)]
    fn default() -> IqPwrReg2 {
        IqPwrReg2(0)
    }
}
impl core::fmt::Debug for IqPwrReg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IqPwrReg2")
            .field("tx_dc_cal_offset_q", &self.tx_dc_cal_offset_q())
            .field("brf_trf_edr_pa_bm_lv", &self.brf_trf_edr_pa_bm_lv())
            .field("tx_dc_cal_offset_i", &self.tx_dc_cal_offset_i())
            .field("edr_lpf_bypass", &self.edr_lpf_bypass())
            .field("edr_tmxbuf_gc_dpsk", &self.edr_tmxbuf_gc_dpsk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IqPwrReg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "IqPwrReg2 {{ tx_dc_cal_offset_q: {=u16:?}, brf_trf_edr_pa_bm_lv: {=u8:?}, tx_dc_cal_offset_i: {=u16:?}, edr_lpf_bypass: {=bool:?}, edr_tmxbuf_gc_dpsk: {=u8:?} }}" , self . tx_dc_cal_offset_q () , self . brf_trf_edr_pa_bm_lv () , self . tx_dc_cal_offset_i () , self . edr_lpf_bypass () , self . edr_tmxbuf_gc_dpsk ())
    }
}
#[doc = "LPF_REG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpfReg(pub u32);
impl LpfReg {
    #[must_use]
    #[inline(always)]
    pub const fn brf_lpf_rz_sel_lv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_lpf_rz_sel_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_lpf_rp4_sel_lv(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_lpf_rp4_sel_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_lpf_cz_sel_lv(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_lpf_cz_sel_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_lpf_cp4_sel_lv(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_lpf_cp4_sel_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_lpf_cp3_sel_lv(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_lpf_cp3_sel_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_lo_open_lv(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_lo_open_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for LpfReg {
    #[inline(always)]
    fn default() -> LpfReg {
        LpfReg(0)
    }
}
impl core::fmt::Debug for LpfReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpfReg")
            .field("brf_lpf_rz_sel_lv", &self.brf_lpf_rz_sel_lv())
            .field("brf_lpf_rp4_sel_lv", &self.brf_lpf_rp4_sel_lv())
            .field("brf_lpf_cz_sel_lv", &self.brf_lpf_cz_sel_lv())
            .field("brf_lpf_cp4_sel_lv", &self.brf_lpf_cp4_sel_lv())
            .field("brf_lpf_cp3_sel_lv", &self.brf_lpf_cp3_sel_lv())
            .field("brf_lo_open_lv", &self.brf_lo_open_lv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpfReg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "LpfReg {{ brf_lpf_rz_sel_lv: {=u8:?}, brf_lpf_rp4_sel_lv: {=u8:?}, brf_lpf_cz_sel_lv: {=u8:?}, brf_lpf_cp4_sel_lv: {=u8:?}, brf_lpf_cp3_sel_lv: {=u8:?}, brf_lo_open_lv: {=bool:?} }}" , self . brf_lpf_rz_sel_lv () , self . brf_lpf_rp4_sel_lv () , self . brf_lpf_cz_sel_lv () , self . brf_lpf_cp4_sel_lv () , self . brf_lpf_cp3_sel_lv () , self . brf_lo_open_lv ())
    }
}
#[doc = "MISC_CTRL_REG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MiscCtrlReg(pub u32);
impl MiscCtrlReg {
    #[must_use]
    #[inline(always)]
    pub const fn pdx_force_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pdx_force_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn idac_force_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_idac_force_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn xtal_ref_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_xtal_ref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_clk_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_adc_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_fifo_clk_phase_sel(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_adc_fifo_clk_phase_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn unlock_flag_clr(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_unlock_flag_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn xtal_ref_en_frc_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_xtal_ref_en_frc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_clk_en_frc_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_adc_clk_en_frc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_clk_sel(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_adc_clk_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_clk_sel_frc_en(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_adc_clk_sel_frc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cbpf_bw_frc_en(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cbpf_bw_frc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cbpf_wx2_stg1_frc_en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cbpf_wx2_stg1_frc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cbpf_wx2_stg2_frc_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cbpf_wx2_stg2_frc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rvga_wx2_stg1_frc_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rvga_wx2_stg1_frc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rvga_wx2_stg2_frc_en(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rvga_wx2_stg2_frc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pkdet_en_early_off_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pkdet_en_early_off_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn xtal_rfch_sel_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_xtal_rfch_sel_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn iq_swap_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_iq_swap_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn bt_xtal_ref_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_bt_xtal_ref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dac_clk_en(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_dac_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn edr_unlock_flag_clr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_edr_unlock_flag_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn edr_xtal_ref_en(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_edr_xtal_ref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn edr_xtal_ref_en_frc_en(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_edr_xtal_ref_en_frc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dac_clk_en_frc_en(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_dac_clk_en_frc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn bypass_dac_fifo(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_bypass_dac_fifo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dac_wclk_edge_sel(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_dac_wclk_edge_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_q_en_frc_en(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_adc_q_en_frc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn en_2m_mod_frc_en(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_en_2m_mod_frc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for MiscCtrlReg {
    #[inline(always)]
    fn default() -> MiscCtrlReg {
        MiscCtrlReg(0)
    }
}
impl core::fmt::Debug for MiscCtrlReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MiscCtrlReg")
            .field("pdx_force_en", &self.pdx_force_en())
            .field("idac_force_en", &self.idac_force_en())
            .field("xtal_ref_en", &self.xtal_ref_en())
            .field("adc_clk_en", &self.adc_clk_en())
            .field("adc_fifo_clk_phase_sel", &self.adc_fifo_clk_phase_sel())
            .field("unlock_flag_clr", &self.unlock_flag_clr())
            .field("xtal_ref_en_frc_en", &self.xtal_ref_en_frc_en())
            .field("adc_clk_en_frc_en", &self.adc_clk_en_frc_en())
            .field("adc_clk_sel", &self.adc_clk_sel())
            .field("adc_clk_sel_frc_en", &self.adc_clk_sel_frc_en())
            .field("cbpf_bw_frc_en", &self.cbpf_bw_frc_en())
            .field("cbpf_wx2_stg1_frc_en", &self.cbpf_wx2_stg1_frc_en())
            .field("cbpf_wx2_stg2_frc_en", &self.cbpf_wx2_stg2_frc_en())
            .field("rvga_wx2_stg1_frc_en", &self.rvga_wx2_stg1_frc_en())
            .field("rvga_wx2_stg2_frc_en", &self.rvga_wx2_stg2_frc_en())
            .field("pkdet_en_early_off_en", &self.pkdet_en_early_off_en())
            .field("xtal_rfch_sel_en", &self.xtal_rfch_sel_en())
            .field("iq_swap_en", &self.iq_swap_en())
            .field("bt_xtal_ref_en", &self.bt_xtal_ref_en())
            .field("dac_clk_en", &self.dac_clk_en())
            .field("edr_unlock_flag_clr", &self.edr_unlock_flag_clr())
            .field("edr_xtal_ref_en", &self.edr_xtal_ref_en())
            .field("edr_xtal_ref_en_frc_en", &self.edr_xtal_ref_en_frc_en())
            .field("dac_clk_en_frc_en", &self.dac_clk_en_frc_en())
            .field("bypass_dac_fifo", &self.bypass_dac_fifo())
            .field("dac_wclk_edge_sel", &self.dac_wclk_edge_sel())
            .field("adc_q_en_frc_en", &self.adc_q_en_frc_en())
            .field("en_2m_mod_frc_en", &self.en_2m_mod_frc_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MiscCtrlReg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "MiscCtrlReg {{ pdx_force_en: {=bool:?}, idac_force_en: {=bool:?}, xtal_ref_en: {=bool:?}, adc_clk_en: {=bool:?}, adc_fifo_clk_phase_sel: {=bool:?}, unlock_flag_clr: {=bool:?}, xtal_ref_en_frc_en: {=bool:?}, adc_clk_en_frc_en: {=bool:?}, adc_clk_sel: {=bool:?}, adc_clk_sel_frc_en: {=bool:?}, cbpf_bw_frc_en: {=bool:?}, cbpf_wx2_stg1_frc_en: {=bool:?}, cbpf_wx2_stg2_frc_en: {=bool:?}, rvga_wx2_stg1_frc_en: {=bool:?}, rvga_wx2_stg2_frc_en: {=bool:?}, pkdet_en_early_off_en: {=bool:?}, xtal_rfch_sel_en: {=bool:?}, iq_swap_en: {=bool:?}, bt_xtal_ref_en: {=bool:?}, dac_clk_en: {=bool:?}, edr_unlock_flag_clr: {=bool:?}, edr_xtal_ref_en: {=bool:?}, edr_xtal_ref_en_frc_en: {=bool:?}, dac_clk_en_frc_en: {=bool:?}, bypass_dac_fifo: {=bool:?}, dac_wclk_edge_sel: {=bool:?}, adc_q_en_frc_en: {=bool:?}, en_2m_mod_frc_en: {=bool:?} }}" , self . pdx_force_en () , self . idac_force_en () , self . xtal_ref_en () , self . adc_clk_en () , self . adc_fifo_clk_phase_sel () , self . unlock_flag_clr () , self . xtal_ref_en_frc_en () , self . adc_clk_en_frc_en () , self . adc_clk_sel () , self . adc_clk_sel_frc_en () , self . cbpf_bw_frc_en () , self . cbpf_wx2_stg1_frc_en () , self . cbpf_wx2_stg2_frc_en () , self . rvga_wx2_stg1_frc_en () , self . rvga_wx2_stg2_frc_en () , self . pkdet_en_early_off_en () , self . xtal_rfch_sel_en () , self . iq_swap_en () , self . bt_xtal_ref_en () , self . dac_clk_en () , self . edr_unlock_flag_clr () , self . edr_xtal_ref_en () , self . edr_xtal_ref_en_frc_en () , self . dac_clk_en_frc_en () , self . bypass_dac_fifo () , self . dac_wclk_edge_sel () , self . adc_q_en_frc_en () , self . en_2m_mod_frc_en ())
    }
}
#[doc = "OSLO_REG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OsloReg(pub u32);
impl OsloReg {
    #[must_use]
    #[inline(always)]
    pub const fn brf_oslo_acal_cmp_lv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_oslo_acal_cmp_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_oslo_pkdet_en_lv(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_oslo_pkdet_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_oslo_ngm_en_lv(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_oslo_ngm_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_oslo_fcal_en_lv(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_oslo_fcal_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_oslo_pkdet_vref_lv(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_oslo_pkdet_vref_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_oslo_ldo_vref_lv(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_oslo_ldo_vref_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 7usize)) | (((val as u32) & 0x0f) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_oslo_en_lv(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_oslo_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for OsloReg {
    #[inline(always)]
    fn default() -> OsloReg {
        OsloReg(0)
    }
}
impl core::fmt::Debug for OsloReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OsloReg")
            .field("brf_oslo_acal_cmp_lv", &self.brf_oslo_acal_cmp_lv())
            .field("brf_oslo_pkdet_en_lv", &self.brf_oslo_pkdet_en_lv())
            .field("brf_oslo_ngm_en_lv", &self.brf_oslo_ngm_en_lv())
            .field("brf_oslo_fcal_en_lv", &self.brf_oslo_fcal_en_lv())
            .field("brf_oslo_pkdet_vref_lv", &self.brf_oslo_pkdet_vref_lv())
            .field("brf_oslo_ldo_vref_lv", &self.brf_oslo_ldo_vref_lv())
            .field("brf_oslo_en_lv", &self.brf_oslo_en_lv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for OsloReg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "OsloReg {{ brf_oslo_acal_cmp_lv: {=bool:?}, brf_oslo_pkdet_en_lv: {=bool:?}, brf_oslo_ngm_en_lv: {=bool:?}, brf_oslo_fcal_en_lv: {=bool:?}, brf_oslo_pkdet_vref_lv: {=u8:?}, brf_oslo_ldo_vref_lv: {=u8:?}, brf_oslo_en_lv: {=bool:?} }}" , self . brf_oslo_acal_cmp_lv () , self . brf_oslo_pkdet_en_lv () , self . brf_oslo_ngm_en_lv () , self . brf_oslo_fcal_en_lv () , self . brf_oslo_pkdet_vref_lv () , self . brf_oslo_ldo_vref_lv () , self . brf_oslo_en_lv ())
    }
}
#[doc = "PACAL_REG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PacalReg(pub u32);
impl PacalReg {
    #[must_use]
    #[inline(always)]
    pub const fn pacal_start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pacal_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pacal_done(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pacal_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sgn_cal_rslt(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_sgn_cal_rslt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn bc_cal_rslt(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_bc_cal_rslt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pacal_rdy(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pacal_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pa_rstb_frc_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pa_rstb_frc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pacal_clk_en(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pacal_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for PacalReg {
    #[inline(always)]
    fn default() -> PacalReg {
        PacalReg(0)
    }
}
impl core::fmt::Debug for PacalReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PacalReg")
            .field("pacal_start", &self.pacal_start())
            .field("pacal_done", &self.pacal_done())
            .field("sgn_cal_rslt", &self.sgn_cal_rslt())
            .field("bc_cal_rslt", &self.bc_cal_rslt())
            .field("pacal_rdy", &self.pacal_rdy())
            .field("pa_rstb_frc_en", &self.pa_rstb_frc_en())
            .field("pacal_clk_en", &self.pacal_clk_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PacalReg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "PacalReg {{ pacal_start: {=bool:?}, pacal_done: {=bool:?}, sgn_cal_rslt: {=bool:?}, bc_cal_rslt: {=u8:?}, pacal_rdy: {=bool:?}, pa_rstb_frc_en: {=bool:?}, pacal_clk_en: {=bool:?} }}" , self . pacal_start () , self . pacal_done () , self . sgn_cal_rslt () , self . bc_cal_rslt () , self . pacal_rdy () , self . pa_rstb_frc_en () , self . pacal_clk_en ())
    }
}
#[doc = "PFDCP_REG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PfdcpReg(pub u32);
impl PfdcpReg {
    #[must_use]
    #[inline(always)]
    pub const fn brf_csd_dn_lv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_csd_dn_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_csd_up_lv(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_csd_up_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_lo_unlock_lv(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_lo_unlock_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pfdcp_csd_reset_lv(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_pfdcp_csd_reset_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pfdcp_csd_en_lv(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_pfdcp_csd_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pfdcp_icp_os_lv(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_pfdcp_icp_os_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 5usize)) | (((val as u32) & 0x3f) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pfdcp_icp_set_lv(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_pfdcp_icp_set_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pfdcp_ldo_vref_lv(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_pfdcp_ldo_vref_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 15usize)) | (((val as u32) & 0x0f) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pfdcp_en_lv(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_pfdcp_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for PfdcpReg {
    #[inline(always)]
    fn default() -> PfdcpReg {
        PfdcpReg(0)
    }
}
impl core::fmt::Debug for PfdcpReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PfdcpReg")
            .field("brf_csd_dn_lv", &self.brf_csd_dn_lv())
            .field("brf_csd_up_lv", &self.brf_csd_up_lv())
            .field("brf_lo_unlock_lv", &self.brf_lo_unlock_lv())
            .field("brf_pfdcp_csd_reset_lv", &self.brf_pfdcp_csd_reset_lv())
            .field("brf_pfdcp_csd_en_lv", &self.brf_pfdcp_csd_en_lv())
            .field("brf_pfdcp_icp_os_lv", &self.brf_pfdcp_icp_os_lv())
            .field("brf_pfdcp_icp_set_lv", &self.brf_pfdcp_icp_set_lv())
            .field("brf_pfdcp_ldo_vref_lv", &self.brf_pfdcp_ldo_vref_lv())
            .field("brf_pfdcp_en_lv", &self.brf_pfdcp_en_lv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PfdcpReg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "PfdcpReg {{ brf_csd_dn_lv: {=bool:?}, brf_csd_up_lv: {=bool:?}, brf_lo_unlock_lv: {=bool:?}, brf_pfdcp_csd_reset_lv: {=bool:?}, brf_pfdcp_csd_en_lv: {=bool:?}, brf_pfdcp_icp_os_lv: {=u8:?}, brf_pfdcp_icp_set_lv: {=u8:?}, brf_pfdcp_ldo_vref_lv: {=u8:?}, brf_pfdcp_en_lv: {=bool:?} }}" , self . brf_csd_dn_lv () , self . brf_csd_up_lv () , self . brf_lo_unlock_lv () , self . brf_pfdcp_csd_reset_lv () , self . brf_pfdcp_csd_en_lv () , self . brf_pfdcp_icp_os_lv () , self . brf_pfdcp_icp_set_lv () , self . brf_pfdcp_ldo_vref_lv () , self . brf_pfdcp_en_lv ())
    }
}
#[doc = "RBB_REG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RbbReg1(pub u32);
impl RbbReg1 {
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_fc_lv_2m(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_fc_lv_2m(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_bm_lv_2m(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_bm_lv_2m(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_cc_lv_2m(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_cc_lv_2m(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_sel_ldovref_rbb_lv(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_sel_ldovref_rbb_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_en_ldo_rbb_lv(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_en_ldo_rbb_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pkdet_vth2q_bt(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_pkdet_vth2q_bt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 14usize)) | (((val as u32) & 0x0f) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pkdet_vth2i_bt(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_pkdet_vth2i_bt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pkdet_vth1q_bt(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_pkdet_vth1q_bt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pkdet_vth1i_bt(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_pkdet_vth1i_bt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 26usize)) | (((val as u32) & 0x0f) << 26usize);
    }
}
impl Default for RbbReg1 {
    #[inline(always)]
    fn default() -> RbbReg1 {
        RbbReg1(0)
    }
}
impl core::fmt::Debug for RbbReg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RbbReg1")
            .field("brf_cbpf_fc_lv_2m", &self.brf_cbpf_fc_lv_2m())
            .field("brf_cbpf_bm_lv_2m", &self.brf_cbpf_bm_lv_2m())
            .field("brf_cbpf_cc_lv_2m", &self.brf_cbpf_cc_lv_2m())
            .field("brf_sel_ldovref_rbb_lv", &self.brf_sel_ldovref_rbb_lv())
            .field("brf_en_ldo_rbb_lv", &self.brf_en_ldo_rbb_lv())
            .field("brf_pkdet_vth2q_bt", &self.brf_pkdet_vth2q_bt())
            .field("brf_pkdet_vth2i_bt", &self.brf_pkdet_vth2i_bt())
            .field("brf_pkdet_vth1q_bt", &self.brf_pkdet_vth1q_bt())
            .field("brf_pkdet_vth1i_bt", &self.brf_pkdet_vth1i_bt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RbbReg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "RbbReg1 {{ brf_cbpf_fc_lv_2m: {=u8:?}, brf_cbpf_bm_lv_2m: {=u8:?}, brf_cbpf_cc_lv_2m: {=u8:?}, brf_sel_ldovref_rbb_lv: {=u8:?}, brf_en_ldo_rbb_lv: {=bool:?}, brf_pkdet_vth2q_bt: {=u8:?}, brf_pkdet_vth2i_bt: {=u8:?}, brf_pkdet_vth1q_bt: {=u8:?}, brf_pkdet_vth1i_bt: {=u8:?} }}" , self . brf_cbpf_fc_lv_2m () , self . brf_cbpf_bm_lv_2m () , self . brf_cbpf_cc_lv_2m () , self . brf_sel_ldovref_rbb_lv () , self . brf_en_ldo_rbb_lv () , self . brf_pkdet_vth2q_bt () , self . brf_pkdet_vth2i_bt () , self . brf_pkdet_vth1q_bt () , self . brf_pkdet_vth1i_bt ())
    }
}
#[doc = "RBB_REG2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RbbReg2(pub u32);
impl RbbReg2 {
    #[must_use]
    #[inline(always)]
    pub const fn brf_rvga_man_cfsel_lv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_rvga_man_cfsel_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_rvga_gc_lv(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_rvga_gc_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 1usize)) | (((val as u32) & 0x1f) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_en_rvga_q_lv(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_en_rvga_q_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_en_rvga_i_lv(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_en_rvga_i_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_w2x_stg2_lv(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_w2x_stg2_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_w2x_stg1_lv(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_w2x_stg1_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_gc_lv(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_gc_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_en_rc(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_en_rc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_fc_lv(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_fc_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_bw_lv(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_bw_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_vstart_lv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_vstart_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_vcmref_lv(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_vcmref_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_bm_lv(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_bm_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_cc_lv(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_cc_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val as u32) & 0x0f) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_en_cbpf_lv(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_en_cbpf_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for RbbReg2 {
    #[inline(always)]
    fn default() -> RbbReg2 {
        RbbReg2(0)
    }
}
impl core::fmt::Debug for RbbReg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RbbReg2")
            .field("brf_rvga_man_cfsel_lv", &self.brf_rvga_man_cfsel_lv())
            .field("brf_rvga_gc_lv", &self.brf_rvga_gc_lv())
            .field("brf_en_rvga_q_lv", &self.brf_en_rvga_q_lv())
            .field("brf_en_rvga_i_lv", &self.brf_en_rvga_i_lv())
            .field("brf_cbpf_w2x_stg2_lv", &self.brf_cbpf_w2x_stg2_lv())
            .field("brf_cbpf_w2x_stg1_lv", &self.brf_cbpf_w2x_stg1_lv())
            .field("brf_cbpf_gc_lv", &self.brf_cbpf_gc_lv())
            .field("brf_cbpf_en_rc", &self.brf_cbpf_en_rc())
            .field("brf_cbpf_fc_lv", &self.brf_cbpf_fc_lv())
            .field("brf_cbpf_bw_lv", &self.brf_cbpf_bw_lv())
            .field("brf_cbpf_vstart_lv", &self.brf_cbpf_vstart_lv())
            .field("brf_cbpf_vcmref_lv", &self.brf_cbpf_vcmref_lv())
            .field("brf_cbpf_bm_lv", &self.brf_cbpf_bm_lv())
            .field("brf_cbpf_cc_lv", &self.brf_cbpf_cc_lv())
            .field("brf_en_cbpf_lv", &self.brf_en_cbpf_lv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RbbReg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "RbbReg2 {{ brf_rvga_man_cfsel_lv: {=bool:?}, brf_rvga_gc_lv: {=u8:?}, brf_en_rvga_q_lv: {=bool:?}, brf_en_rvga_i_lv: {=bool:?}, brf_cbpf_w2x_stg2_lv: {=bool:?}, brf_cbpf_w2x_stg1_lv: {=bool:?}, brf_cbpf_gc_lv: {=u8:?}, brf_cbpf_en_rc: {=bool:?}, brf_cbpf_fc_lv: {=u8:?}, brf_cbpf_bw_lv: {=bool:?}, brf_cbpf_vstart_lv: {=u8:?}, brf_cbpf_vcmref_lv: {=u8:?}, brf_cbpf_bm_lv: {=u8:?}, brf_cbpf_cc_lv: {=u8:?}, brf_en_cbpf_lv: {=bool:?} }}" , self . brf_rvga_man_cfsel_lv () , self . brf_rvga_gc_lv () , self . brf_en_rvga_q_lv () , self . brf_en_rvga_i_lv () , self . brf_cbpf_w2x_stg2_lv () , self . brf_cbpf_w2x_stg1_lv () , self . brf_cbpf_gc_lv () , self . brf_cbpf_en_rc () , self . brf_cbpf_fc_lv () , self . brf_cbpf_bw_lv () , self . brf_cbpf_vstart_lv () , self . brf_cbpf_vcmref_lv () , self . brf_cbpf_bm_lv () , self . brf_cbpf_cc_lv () , self . brf_en_cbpf_lv ())
    }
}
#[doc = "RBB_REG3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RbbReg3(pub u32);
impl RbbReg3 {
    #[must_use]
    #[inline(always)]
    pub const fn brf_en_pkdet_lv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_en_pkdet_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_rvga_w2x_stg2_lv(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_rvga_w2x_stg2_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_rvga_w2x_stg1_lv(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_rvga_w2x_stg1_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_rvga_vstart_lv(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_rvga_vstart_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_rvga_vcmref_lv(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_rvga_vcmref_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_rvga_bm_lv(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_rvga_bm_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_rvga_rz_lv(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_rvga_rz_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_rvga_cc_lv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_rvga_cc_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_rvga_cfman_lv(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_rvga_cfman_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
}
impl Default for RbbReg3 {
    #[inline(always)]
    fn default() -> RbbReg3 {
        RbbReg3(0)
    }
}
impl core::fmt::Debug for RbbReg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RbbReg3")
            .field("brf_en_pkdet_lv", &self.brf_en_pkdet_lv())
            .field("brf_rvga_w2x_stg2_lv", &self.brf_rvga_w2x_stg2_lv())
            .field("brf_rvga_w2x_stg1_lv", &self.brf_rvga_w2x_stg1_lv())
            .field("brf_rvga_vstart_lv", &self.brf_rvga_vstart_lv())
            .field("brf_rvga_vcmref_lv", &self.brf_rvga_vcmref_lv())
            .field("brf_rvga_bm_lv", &self.brf_rvga_bm_lv())
            .field("brf_rvga_rz_lv", &self.brf_rvga_rz_lv())
            .field("brf_rvga_cc_lv", &self.brf_rvga_cc_lv())
            .field("brf_rvga_cfman_lv", &self.brf_rvga_cfman_lv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RbbReg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "RbbReg3 {{ brf_en_pkdet_lv: {=u8:?}, brf_rvga_w2x_stg2_lv: {=bool:?}, brf_rvga_w2x_stg1_lv: {=bool:?}, brf_rvga_vstart_lv: {=u8:?}, brf_rvga_vcmref_lv: {=u8:?}, brf_rvga_bm_lv: {=u8:?}, brf_rvga_rz_lv: {=u8:?}, brf_rvga_cc_lv: {=u8:?}, brf_rvga_cfman_lv: {=u8:?} }}" , self . brf_en_pkdet_lv () , self . brf_rvga_w2x_stg2_lv () , self . brf_rvga_w2x_stg1_lv () , self . brf_rvga_vstart_lv () , self . brf_rvga_vcmref_lv () , self . brf_rvga_bm_lv () , self . brf_rvga_rz_lv () , self . brf_rvga_cc_lv () , self . brf_rvga_cfman_lv ())
    }
}
#[doc = "RBB_REG4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RbbReg4(pub u32);
impl RbbReg4 {
    #[must_use]
    #[inline(always)]
    pub const fn brf_pkdet_vth2q_lv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_pkdet_vth2q_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pkdet_vth2i_lv(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_pkdet_vth2i_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pkdet_vth1q_lv(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_pkdet_vth1q_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pkdet_vth1i_lv(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_pkdet_vth1i_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_dos_q_lv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_dos_q_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_dos_i_lv(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_dos_i_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 23usize)) | (((val as u32) & 0x7f) << 23usize);
    }
}
impl Default for RbbReg4 {
    #[inline(always)]
    fn default() -> RbbReg4 {
        RbbReg4(0)
    }
}
impl core::fmt::Debug for RbbReg4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RbbReg4")
            .field("brf_pkdet_vth2q_lv", &self.brf_pkdet_vth2q_lv())
            .field("brf_pkdet_vth2i_lv", &self.brf_pkdet_vth2i_lv())
            .field("brf_pkdet_vth1q_lv", &self.brf_pkdet_vth1q_lv())
            .field("brf_pkdet_vth1i_lv", &self.brf_pkdet_vth1i_lv())
            .field("brf_dos_q_lv", &self.brf_dos_q_lv())
            .field("brf_dos_i_lv", &self.brf_dos_i_lv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RbbReg4 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "RbbReg4 {{ brf_pkdet_vth2q_lv: {=u8:?}, brf_pkdet_vth2i_lv: {=u8:?}, brf_pkdet_vth1q_lv: {=u8:?}, brf_pkdet_vth1i_lv: {=u8:?}, brf_dos_q_lv: {=u8:?}, brf_dos_i_lv: {=u8:?} }}" , self . brf_pkdet_vth2q_lv () , self . brf_pkdet_vth2i_lv () , self . brf_pkdet_vth1q_lv () , self . brf_pkdet_vth1i_lv () , self . brf_dos_q_lv () , self . brf_dos_i_lv ())
    }
}
#[doc = "RBB_REG5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RbbReg5(pub u32);
impl RbbReg5 {
    #[must_use]
    #[inline(always)]
    pub const fn brf_rvga_tx_lpbk_en_lv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_rvga_tx_lpbk_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_bt_en_lv(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_bt_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_iary_bm_lv(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_iary_bm_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_en_iarray_lv(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_en_iarray_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_en_osdacq_lv(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_en_osdacq_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_en_osdaci_lv(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_en_osdaci_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_rstb_rccal_lv(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_rstb_rccal_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_capman_lv(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_capman_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 9usize)) | (((val as u32) & 0x1f) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_rccal_mancap_lv(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_rccal_mancap_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_rccal_selxo_lv(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_rccal_selxo_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_en_rccal_lv(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_en_rccal_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pkdet_bm_lv(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_pkdet_bm_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_bt_en_frc_en(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_bt_en_frc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for RbbReg5 {
    #[inline(always)]
    fn default() -> RbbReg5 {
        RbbReg5(0)
    }
}
impl core::fmt::Debug for RbbReg5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RbbReg5")
            .field("brf_rvga_tx_lpbk_en_lv", &self.brf_rvga_tx_lpbk_en_lv())
            .field("brf_cbpf_bt_en_lv", &self.brf_cbpf_bt_en_lv())
            .field("brf_iary_bm_lv", &self.brf_iary_bm_lv())
            .field("brf_en_iarray_lv", &self.brf_en_iarray_lv())
            .field("brf_en_osdacq_lv", &self.brf_en_osdacq_lv())
            .field("brf_en_osdaci_lv", &self.brf_en_osdaci_lv())
            .field("brf_rstb_rccal_lv", &self.brf_rstb_rccal_lv())
            .field("brf_cbpf_capman_lv", &self.brf_cbpf_capman_lv())
            .field("brf_rccal_mancap_lv", &self.brf_rccal_mancap_lv())
            .field("brf_rccal_selxo_lv", &self.brf_rccal_selxo_lv())
            .field("brf_en_rccal_lv", &self.brf_en_rccal_lv())
            .field("brf_pkdet_bm_lv", &self.brf_pkdet_bm_lv())
            .field("brf_cbpf_bt_en_frc_en", &self.brf_cbpf_bt_en_frc_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RbbReg5 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "RbbReg5 {{ brf_rvga_tx_lpbk_en_lv: {=bool:?}, brf_cbpf_bt_en_lv: {=bool:?}, brf_iary_bm_lv: {=u8:?}, brf_en_iarray_lv: {=bool:?}, brf_en_osdacq_lv: {=bool:?}, brf_en_osdaci_lv: {=bool:?}, brf_rstb_rccal_lv: {=bool:?}, brf_cbpf_capman_lv: {=u8:?}, brf_rccal_mancap_lv: {=bool:?}, brf_rccal_selxo_lv: {=bool:?}, brf_en_rccal_lv: {=bool:?}, brf_pkdet_bm_lv: {=u8:?}, brf_cbpf_bt_en_frc_en: {=bool:?} }}" , self . brf_rvga_tx_lpbk_en_lv () , self . brf_cbpf_bt_en_lv () , self . brf_iary_bm_lv () , self . brf_en_iarray_lv () , self . brf_en_osdacq_lv () , self . brf_en_osdaci_lv () , self . brf_rstb_rccal_lv () , self . brf_cbpf_capman_lv () , self . brf_rccal_mancap_lv () , self . brf_rccal_selxo_lv () , self . brf_en_rccal_lv () , self . brf_pkdet_bm_lv () , self . brf_cbpf_bt_en_frc_en ())
    }
}
#[doc = "RBB_REG6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RbbReg6(pub u32);
impl RbbReg6 {
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_fc_lv_br(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_fc_lv_br(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_bm_lv_br(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_bm_lv_br(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_cc_lv_br(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_cc_lv_br(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_fc_lv_edr(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_fc_lv_edr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_bm_lv_edr(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_bm_lv_edr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_cc_lv_edr(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_cc_lv_edr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 14usize)) | (((val as u32) & 0x0f) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_rvga_w2x_stg2_lv_br(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_rvga_w2x_stg2_lv_br(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_rvga_w2x_stg1_lv_br(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_rvga_w2x_stg1_lv_br(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_rvga_w2x_stg2_lv_edr(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_rvga_w2x_stg2_lv_edr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_rvga_w2x_stg1_lv_edr(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_rvga_w2x_stg1_lv_edr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_w2x_stg2_lv_br(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_w2x_stg2_lv_br(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_w2x_stg1_lv_br(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_w2x_stg1_lv_br(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_w2x_stg2_lv_edr(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_w2x_stg2_lv_edr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_w2x_stg1_lv_edr(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_w2x_stg1_lv_edr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_bw_lv_br(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_bw_lv_br(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_cbpf_bw_lv_edr(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_cbpf_bw_lv_edr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
}
impl Default for RbbReg6 {
    #[inline(always)]
    fn default() -> RbbReg6 {
        RbbReg6(0)
    }
}
impl core::fmt::Debug for RbbReg6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RbbReg6")
            .field("brf_cbpf_fc_lv_br", &self.brf_cbpf_fc_lv_br())
            .field("brf_cbpf_bm_lv_br", &self.brf_cbpf_bm_lv_br())
            .field("brf_cbpf_cc_lv_br", &self.brf_cbpf_cc_lv_br())
            .field("brf_cbpf_fc_lv_edr", &self.brf_cbpf_fc_lv_edr())
            .field("brf_cbpf_bm_lv_edr", &self.brf_cbpf_bm_lv_edr())
            .field("brf_cbpf_cc_lv_edr", &self.brf_cbpf_cc_lv_edr())
            .field("brf_rvga_w2x_stg2_lv_br", &self.brf_rvga_w2x_stg2_lv_br())
            .field("brf_rvga_w2x_stg1_lv_br", &self.brf_rvga_w2x_stg1_lv_br())
            .field("brf_rvga_w2x_stg2_lv_edr", &self.brf_rvga_w2x_stg2_lv_edr())
            .field("brf_rvga_w2x_stg1_lv_edr", &self.brf_rvga_w2x_stg1_lv_edr())
            .field("brf_cbpf_w2x_stg2_lv_br", &self.brf_cbpf_w2x_stg2_lv_br())
            .field("brf_cbpf_w2x_stg1_lv_br", &self.brf_cbpf_w2x_stg1_lv_br())
            .field("brf_cbpf_w2x_stg2_lv_edr", &self.brf_cbpf_w2x_stg2_lv_edr())
            .field("brf_cbpf_w2x_stg1_lv_edr", &self.brf_cbpf_w2x_stg1_lv_edr())
            .field("brf_cbpf_bw_lv_br", &self.brf_cbpf_bw_lv_br())
            .field("brf_cbpf_bw_lv_edr", &self.brf_cbpf_bw_lv_edr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RbbReg6 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "RbbReg6 {{ brf_cbpf_fc_lv_br: {=u8:?}, brf_cbpf_bm_lv_br: {=u8:?}, brf_cbpf_cc_lv_br: {=u8:?}, brf_cbpf_fc_lv_edr: {=u8:?}, brf_cbpf_bm_lv_edr: {=u8:?}, brf_cbpf_cc_lv_edr: {=u8:?}, brf_rvga_w2x_stg2_lv_br: {=bool:?}, brf_rvga_w2x_stg1_lv_br: {=bool:?}, brf_rvga_w2x_stg2_lv_edr: {=bool:?}, brf_rvga_w2x_stg1_lv_edr: {=bool:?}, brf_cbpf_w2x_stg2_lv_br: {=bool:?}, brf_cbpf_w2x_stg1_lv_br: {=bool:?}, brf_cbpf_w2x_stg2_lv_edr: {=bool:?}, brf_cbpf_w2x_stg1_lv_edr: {=bool:?}, brf_cbpf_bw_lv_br: {=bool:?}, brf_cbpf_bw_lv_edr: {=bool:?} }}" , self . brf_cbpf_fc_lv_br () , self . brf_cbpf_bm_lv_br () , self . brf_cbpf_cc_lv_br () , self . brf_cbpf_fc_lv_edr () , self . brf_cbpf_bm_lv_edr () , self . brf_cbpf_cc_lv_edr () , self . brf_rvga_w2x_stg2_lv_br () , self . brf_rvga_w2x_stg1_lv_br () , self . brf_rvga_w2x_stg2_lv_edr () , self . brf_rvga_w2x_stg1_lv_edr () , self . brf_cbpf_w2x_stg2_lv_br () , self . brf_cbpf_w2x_stg1_lv_br () , self . brf_cbpf_w2x_stg2_lv_edr () , self . brf_cbpf_w2x_stg1_lv_edr () , self . brf_cbpf_bw_lv_br () , self . brf_cbpf_bw_lv_edr ())
    }
}
#[doc = "RCROSCAL_REG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RcroscalReg(pub u32);
impl RcroscalReg {
    #[must_use]
    #[inline(always)]
    pub const fn ros_adc_q(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ros_adc_q(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ros_adc_i(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ros_adc_i(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rccal_done(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rccal_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rccal_start(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rccal_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rc_capcode_offset(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rc_capcode_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rc_capcode(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rc_capcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
    }
}
impl Default for RcroscalReg {
    #[inline(always)]
    fn default() -> RcroscalReg {
        RcroscalReg(0)
    }
}
impl core::fmt::Debug for RcroscalReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RcroscalReg")
            .field("ros_adc_q", &self.ros_adc_q())
            .field("ros_adc_i", &self.ros_adc_i())
            .field("rccal_done", &self.rccal_done())
            .field("rccal_start", &self.rccal_start())
            .field("rc_capcode_offset", &self.rc_capcode_offset())
            .field("rc_capcode", &self.rc_capcode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RcroscalReg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "RcroscalReg {{ ros_adc_q: {=u16:?}, ros_adc_i: {=u16:?}, rccal_done: {=bool:?}, rccal_start: {=bool:?}, rc_capcode_offset: {=u8:?}, rc_capcode: {=u8:?} }}" , self . ros_adc_q () , self . ros_adc_i () , self . rccal_done () , self . rccal_start () , self . rc_capcode_offset () , self . rc_capcode ())
    }
}
#[doc = "RF_LODIST_REG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RfLodistReg(pub u32);
impl RfLodistReg {
    #[must_use]
    #[inline(always)]
    pub const fn brf_lodistedr_en_lv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_lodistedr_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_lodistedr_ldo_vref_lv(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_lodistedr_ldo_vref_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_lodistedr_tx_sel_lv(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_lodistedr_tx_sel_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_lodist5g_edrtx_en_lv(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_lodist5g_edrtx_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_lodist5g_bletx_en_lv(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_lodist5g_bletx_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_lodist5g_rx_en_lv(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_lodist5g_rx_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_lodist5g_fbdv_str_lv(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_lodist5g_fbdv_str_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_lodist5g_tx_str_lv(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_lodist5g_tx_str_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_lodist5g_rx_str_lv(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_lodist5g_rx_str_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_lo_iary_en_lv(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_lo_iary_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_en_rfbg_lv(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_en_rfbg_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_en_vddpsw_lv(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_en_vddpsw_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for RfLodistReg {
    #[inline(always)]
    fn default() -> RfLodistReg {
        RfLodistReg(0)
    }
}
impl core::fmt::Debug for RfLodistReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RfLodistReg")
            .field("brf_lodistedr_en_lv", &self.brf_lodistedr_en_lv())
            .field(
                "brf_lodistedr_ldo_vref_lv",
                &self.brf_lodistedr_ldo_vref_lv(),
            )
            .field("brf_lodistedr_tx_sel_lv", &self.brf_lodistedr_tx_sel_lv())
            .field("brf_lodist5g_edrtx_en_lv", &self.brf_lodist5g_edrtx_en_lv())
            .field("brf_lodist5g_bletx_en_lv", &self.brf_lodist5g_bletx_en_lv())
            .field("brf_lodist5g_rx_en_lv", &self.brf_lodist5g_rx_en_lv())
            .field("brf_lodist5g_fbdv_str_lv", &self.brf_lodist5g_fbdv_str_lv())
            .field("brf_lodist5g_tx_str_lv", &self.brf_lodist5g_tx_str_lv())
            .field("brf_lodist5g_rx_str_lv", &self.brf_lodist5g_rx_str_lv())
            .field("brf_lo_iary_en_lv", &self.brf_lo_iary_en_lv())
            .field("brf_en_rfbg_lv", &self.brf_en_rfbg_lv())
            .field("brf_en_vddpsw_lv", &self.brf_en_vddpsw_lv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RfLodistReg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "RfLodistReg {{ brf_lodistedr_en_lv: {=bool:?}, brf_lodistedr_ldo_vref_lv: {=u8:?}, brf_lodistedr_tx_sel_lv: {=u8:?}, brf_lodist5g_edrtx_en_lv: {=bool:?}, brf_lodist5g_bletx_en_lv: {=bool:?}, brf_lodist5g_rx_en_lv: {=bool:?}, brf_lodist5g_fbdv_str_lv: {=u8:?}, brf_lodist5g_tx_str_lv: {=u8:?}, brf_lodist5g_rx_str_lv: {=u8:?}, brf_lo_iary_en_lv: {=bool:?}, brf_en_rfbg_lv: {=bool:?}, brf_en_vddpsw_lv: {=bool:?} }}" , self . brf_lodistedr_en_lv () , self . brf_lodistedr_ldo_vref_lv () , self . brf_lodistedr_tx_sel_lv () , self . brf_lodist5g_edrtx_en_lv () , self . brf_lodist5g_bletx_en_lv () , self . brf_lodist5g_rx_en_lv () , self . brf_lodist5g_fbdv_str_lv () , self . brf_lodist5g_tx_str_lv () , self . brf_lodist5g_rx_str_lv () , self . brf_lo_iary_en_lv () , self . brf_en_rfbg_lv () , self . brf_en_vddpsw_lv ())
    }
}
#[doc = "ROSCAL_REG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RoscalReg1(pub u32);
impl RoscalReg1 {
    #[must_use]
    #[inline(always)]
    pub const fn roscal_start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_roscal_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn roscal_bypass(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_roscal_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn en_rosdac_i(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_en_rosdac_i(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn en_rosdac_q(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_en_rosdac_q(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn roscal_ta(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_roscal_ta(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 4usize)) | (((val as u32) & 0x01ff) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn roscal_tb(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_roscal_tb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn roscal_tc(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_roscal_tc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
    }
}
impl Default for RoscalReg1 {
    #[inline(always)]
    fn default() -> RoscalReg1 {
        RoscalReg1(0)
    }
}
impl core::fmt::Debug for RoscalReg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RoscalReg1")
            .field("roscal_start", &self.roscal_start())
            .field("roscal_bypass", &self.roscal_bypass())
            .field("en_rosdac_i", &self.en_rosdac_i())
            .field("en_rosdac_q", &self.en_rosdac_q())
            .field("roscal_ta", &self.roscal_ta())
            .field("roscal_tb", &self.roscal_tb())
            .field("roscal_tc", &self.roscal_tc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RoscalReg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "RoscalReg1 {{ roscal_start: {=bool:?}, roscal_bypass: {=bool:?}, en_rosdac_i: {=bool:?}, en_rosdac_q: {=bool:?}, roscal_ta: {=u16:?}, roscal_tb: {=u8:?}, roscal_tc: {=u8:?} }}" , self . roscal_start () , self . roscal_bypass () , self . en_rosdac_i () , self . en_rosdac_q () , self . roscal_ta () , self . roscal_tb () , self . roscal_tc ())
    }
}
#[doc = "ROSCAL_REG2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RoscalReg2(pub u32);
impl RoscalReg2 {
    #[must_use]
    #[inline(always)]
    pub const fn roscal_done(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_roscal_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dos_i_sw(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_dos_i_sw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dos_q_sw(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_dos_q_sw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
}
impl Default for RoscalReg2 {
    #[inline(always)]
    fn default() -> RoscalReg2 {
        RoscalReg2(0)
    }
}
impl core::fmt::Debug for RoscalReg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RoscalReg2")
            .field("roscal_done", &self.roscal_done())
            .field("dos_i_sw", &self.dos_i_sw())
            .field("dos_q_sw", &self.dos_q_sw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RoscalReg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RoscalReg2 {{ roscal_done: {=bool:?}, dos_i_sw: {=u8:?}, dos_q_sw: {=u8:?} }}",
            self.roscal_done(),
            self.dos_i_sw(),
            self.dos_q_sw()
        )
    }
}
#[doc = "RRF_REG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RrfReg(pub u32);
impl RrfReg {
    #[must_use]
    #[inline(always)]
    pub const fn brf_mx_bm_lv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_mx_bm_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_mx_pu_lv(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_mx_pu_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_lna_match_lv(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_lna_match_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_lna_shuntsw_lv(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_lna_shuntsw_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_lna_fbrtrim_lv(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_lna_fbrtrim_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_lna_gc_lv(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_lna_gc_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_lna_bm_lv(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_lna_bm_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_lna_pu_lv(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_lna_pu_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_rrf_ldo_vref_sel_lv(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_rrf_ldo_vref_sel_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_rrf_ldo11_en_lv(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_rrf_ldo11_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for RrfReg {
    #[inline(always)]
    fn default() -> RrfReg {
        RrfReg(0)
    }
}
impl core::fmt::Debug for RrfReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RrfReg")
            .field("brf_mx_bm_lv", &self.brf_mx_bm_lv())
            .field("brf_mx_pu_lv", &self.brf_mx_pu_lv())
            .field("brf_lna_match_lv", &self.brf_lna_match_lv())
            .field("brf_lna_shuntsw_lv", &self.brf_lna_shuntsw_lv())
            .field("brf_lna_fbrtrim_lv", &self.brf_lna_fbrtrim_lv())
            .field("brf_lna_gc_lv", &self.brf_lna_gc_lv())
            .field("brf_lna_bm_lv", &self.brf_lna_bm_lv())
            .field("brf_lna_pu_lv", &self.brf_lna_pu_lv())
            .field("brf_rrf_ldo_vref_sel_lv", &self.brf_rrf_ldo_vref_sel_lv())
            .field("brf_rrf_ldo11_en_lv", &self.brf_rrf_ldo11_en_lv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RrfReg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "RrfReg {{ brf_mx_bm_lv: {=u8:?}, brf_mx_pu_lv: {=bool:?}, brf_lna_match_lv: {=u8:?}, brf_lna_shuntsw_lv: {=bool:?}, brf_lna_fbrtrim_lv: {=u8:?}, brf_lna_gc_lv: {=u8:?}, brf_lna_bm_lv: {=u8:?}, brf_lna_pu_lv: {=bool:?}, brf_rrf_ldo_vref_sel_lv: {=u8:?}, brf_rrf_ldo11_en_lv: {=bool:?} }}" , self . brf_mx_bm_lv () , self . brf_mx_pu_lv () , self . brf_lna_match_lv () , self . brf_lna_shuntsw_lv () , self . brf_lna_fbrtrim_lv () , self . brf_lna_gc_lv () , self . brf_lna_bm_lv () , self . brf_lna_pu_lv () , self . brf_rrf_ldo_vref_sel_lv () , self . brf_rrf_ldo11_en_lv ())
    }
}
#[doc = "TBB_REG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TbbReg(pub u32);
impl TbbReg {
    #[must_use]
    #[inline(always)]
    pub const fn brf_sel_ldovref_dac_dvdd_lv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_sel_ldovref_dac_dvdd_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_sel_ldovref_dac_avdd_lv(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_sel_ldovref_dac_avdd_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_en_tbb_iarray_lv(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_en_tbb_iarray_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_en_ldo_dac_dvdd_lv(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_en_ldo_dac_dvdd_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_en_ldo_dac_avdd_lv(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_en_ldo_dac_avdd_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_en_dac_lv(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_en_dac_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_dac_start_lv(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_dac_start_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_dac_sel_clk_bar_lv(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_dac_sel_clk_bar_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_dac_lsb_cnt_lv(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_dac_lsb_cnt_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
}
impl Default for TbbReg {
    #[inline(always)]
    fn default() -> TbbReg {
        TbbReg(0)
    }
}
impl core::fmt::Debug for TbbReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TbbReg")
            .field(
                "brf_sel_ldovref_dac_dvdd_lv",
                &self.brf_sel_ldovref_dac_dvdd_lv(),
            )
            .field(
                "brf_sel_ldovref_dac_avdd_lv",
                &self.brf_sel_ldovref_dac_avdd_lv(),
            )
            .field("brf_en_tbb_iarray_lv", &self.brf_en_tbb_iarray_lv())
            .field("brf_en_ldo_dac_dvdd_lv", &self.brf_en_ldo_dac_dvdd_lv())
            .field("brf_en_ldo_dac_avdd_lv", &self.brf_en_ldo_dac_avdd_lv())
            .field("brf_en_dac_lv", &self.brf_en_dac_lv())
            .field("brf_dac_start_lv", &self.brf_dac_start_lv())
            .field("brf_dac_sel_clk_bar_lv", &self.brf_dac_sel_clk_bar_lv())
            .field("brf_dac_lsb_cnt_lv", &self.brf_dac_lsb_cnt_lv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TbbReg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TbbReg {{ brf_sel_ldovref_dac_dvdd_lv: {=u8:?}, brf_sel_ldovref_dac_avdd_lv: {=u8:?}, brf_en_tbb_iarray_lv: {=bool:?}, brf_en_ldo_dac_dvdd_lv: {=bool:?}, brf_en_ldo_dac_avdd_lv: {=bool:?}, brf_en_dac_lv: {=bool:?}, brf_dac_start_lv: {=bool:?}, brf_dac_sel_clk_bar_lv: {=bool:?}, brf_dac_lsb_cnt_lv: {=u8:?} }}" , self . brf_sel_ldovref_dac_dvdd_lv () , self . brf_sel_ldovref_dac_avdd_lv () , self . brf_en_tbb_iarray_lv () , self . brf_en_ldo_dac_dvdd_lv () , self . brf_en_ldo_dac_avdd_lv () , self . brf_en_dac_lv () , self . brf_dac_start_lv () , self . brf_dac_sel_clk_bar_lv () , self . brf_dac_lsb_cnt_lv ())
    }
}
#[doc = "TRF_EDR_REG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrfEdrReg1(pub u32);
impl TrfEdrReg1 {
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_edr_pacas_bm_lv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_trf_edr_pacas_bm_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_edr_pa_pu_lv(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_trf_edr_pa_pu_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_edr_tmxcap_bm_lv(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_trf_edr_tmxcap_bm_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_edr_tmxcap_sel_lv(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_trf_edr_tmxcap_sel_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_edr_tmxcas_bm_lv(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_trf_edr_tmxcas_bm_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_edr_tmxcas_sel_lv(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_trf_edr_tmxcas_sel_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_edr_tmx_pu_lv(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_trf_edr_tmx_pu_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_edr_lobias_bm_lv(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_trf_edr_lobias_bm_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_edr_tmxbuf_ibld_lv(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_trf_edr_tmxbuf_ibld_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 15usize)) | (((val as u32) & 0x0f) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_edr_tmxbuf_pu_lv(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_trf_edr_tmxbuf_pu_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_edr_iarray_en_lv(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_trf_edr_iarray_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for TrfEdrReg1 {
    #[inline(always)]
    fn default() -> TrfEdrReg1 {
        TrfEdrReg1(0)
    }
}
impl core::fmt::Debug for TrfEdrReg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrfEdrReg1")
            .field("brf_trf_edr_pacas_bm_lv", &self.brf_trf_edr_pacas_bm_lv())
            .field("brf_trf_edr_pa_pu_lv", &self.brf_trf_edr_pa_pu_lv())
            .field("brf_trf_edr_tmxcap_bm_lv", &self.brf_trf_edr_tmxcap_bm_lv())
            .field(
                "brf_trf_edr_tmxcap_sel_lv",
                &self.brf_trf_edr_tmxcap_sel_lv(),
            )
            .field("brf_trf_edr_tmxcas_bm_lv", &self.brf_trf_edr_tmxcas_bm_lv())
            .field(
                "brf_trf_edr_tmxcas_sel_lv",
                &self.brf_trf_edr_tmxcas_sel_lv(),
            )
            .field("brf_trf_edr_tmx_pu_lv", &self.brf_trf_edr_tmx_pu_lv())
            .field("brf_trf_edr_lobias_bm_lv", &self.brf_trf_edr_lobias_bm_lv())
            .field(
                "brf_trf_edr_tmxbuf_ibld_lv",
                &self.brf_trf_edr_tmxbuf_ibld_lv(),
            )
            .field("brf_trf_edr_tmxbuf_pu_lv", &self.brf_trf_edr_tmxbuf_pu_lv())
            .field("brf_trf_edr_iarray_en_lv", &self.brf_trf_edr_iarray_en_lv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrfEdrReg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TrfEdrReg1 {{ brf_trf_edr_pacas_bm_lv: {=u8:?}, brf_trf_edr_pa_pu_lv: {=bool:?}, brf_trf_edr_tmxcap_bm_lv: {=u8:?}, brf_trf_edr_tmxcap_sel_lv: {=u8:?}, brf_trf_edr_tmxcas_bm_lv: {=u8:?}, brf_trf_edr_tmxcas_sel_lv: {=bool:?}, brf_trf_edr_tmx_pu_lv: {=bool:?}, brf_trf_edr_lobias_bm_lv: {=u8:?}, brf_trf_edr_tmxbuf_ibld_lv: {=u8:?}, brf_trf_edr_tmxbuf_pu_lv: {=bool:?}, brf_trf_edr_iarray_en_lv: {=bool:?} }}" , self . brf_trf_edr_pacas_bm_lv () , self . brf_trf_edr_pa_pu_lv () , self . brf_trf_edr_tmxcap_bm_lv () , self . brf_trf_edr_tmxcap_sel_lv () , self . brf_trf_edr_tmxcas_bm_lv () , self . brf_trf_edr_tmxcas_sel_lv () , self . brf_trf_edr_tmx_pu_lv () , self . brf_trf_edr_lobias_bm_lv () , self . brf_trf_edr_tmxbuf_ibld_lv () , self . brf_trf_edr_tmxbuf_pu_lv () , self . brf_trf_edr_iarray_en_lv ())
    }
}
#[doc = "TRF_EDR_REG2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrfEdrReg2(pub u32);
impl TrfEdrReg2 {
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_edr_pwrmtr_os_pn_lv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_trf_edr_pwrmtr_os_pn_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_edr_pwrmtr_os_lv(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_trf_edr_pwrmtr_os_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_edr_pwrmtr_gc_lv(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_trf_edr_pwrmtr_gc_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_edr_pwrmtr_bm_lv(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_trf_edr_pwrmtr_bm_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_edr_pwrmtr_en_lv(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_trf_edr_pwrmtr_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_edr_pa_xfmr_sg_lv(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_trf_edr_pa_xfmr_sg_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_edr_papmos_bm_lv(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_trf_edr_papmos_bm_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_edr_pacap_bm_lv(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_trf_edr_pacap_bm_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 15usize)) | (((val as u32) & 0x03) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_edr_pacap_en_lv(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_trf_edr_pacap_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for TrfEdrReg2 {
    #[inline(always)]
    fn default() -> TrfEdrReg2 {
        TrfEdrReg2(0)
    }
}
impl core::fmt::Debug for TrfEdrReg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrfEdrReg2")
            .field(
                "brf_trf_edr_pwrmtr_os_pn_lv",
                &self.brf_trf_edr_pwrmtr_os_pn_lv(),
            )
            .field("brf_trf_edr_pwrmtr_os_lv", &self.brf_trf_edr_pwrmtr_os_lv())
            .field("brf_trf_edr_pwrmtr_gc_lv", &self.brf_trf_edr_pwrmtr_gc_lv())
            .field("brf_trf_edr_pwrmtr_bm_lv", &self.brf_trf_edr_pwrmtr_bm_lv())
            .field("brf_trf_edr_pwrmtr_en_lv", &self.brf_trf_edr_pwrmtr_en_lv())
            .field(
                "brf_trf_edr_pa_xfmr_sg_lv",
                &self.brf_trf_edr_pa_xfmr_sg_lv(),
            )
            .field("brf_trf_edr_papmos_bm_lv", &self.brf_trf_edr_papmos_bm_lv())
            .field("brf_trf_edr_pacap_bm_lv", &self.brf_trf_edr_pacap_bm_lv())
            .field("brf_trf_edr_pacap_en_lv", &self.brf_trf_edr_pacap_en_lv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrfEdrReg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TrfEdrReg2 {{ brf_trf_edr_pwrmtr_os_pn_lv: {=bool:?}, brf_trf_edr_pwrmtr_os_lv: {=u8:?}, brf_trf_edr_pwrmtr_gc_lv: {=u8:?}, brf_trf_edr_pwrmtr_bm_lv: {=u8:?}, brf_trf_edr_pwrmtr_en_lv: {=bool:?}, brf_trf_edr_pa_xfmr_sg_lv: {=bool:?}, brf_trf_edr_papmos_bm_lv: {=u8:?}, brf_trf_edr_pacap_bm_lv: {=u8:?}, brf_trf_edr_pacap_en_lv: {=bool:?} }}" , self . brf_trf_edr_pwrmtr_os_pn_lv () , self . brf_trf_edr_pwrmtr_os_lv () , self . brf_trf_edr_pwrmtr_gc_lv () , self . brf_trf_edr_pwrmtr_bm_lv () , self . brf_trf_edr_pwrmtr_en_lv () , self . brf_trf_edr_pa_xfmr_sg_lv () , self . brf_trf_edr_papmos_bm_lv () , self . brf_trf_edr_pacap_bm_lv () , self . brf_trf_edr_pacap_en_lv ())
    }
}
#[doc = "TRF_REG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrfReg1(pub u32);
impl TrfReg1 {
    #[must_use]
    #[inline(always)]
    pub const fn brf_pa_cas_bp_lv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_pa_cas_bp_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pa_pm_lv(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_pa_pm_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pa_vc_lv(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_pa_vc_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 3usize)) | (((val as u32) & 0x3f) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pa_rstn_lv(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_pa_rstn_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pa_setbc_lv(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_pa_setbc_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pa_setsgn_lv(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_pa_setsgn_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pa_bcsel_lv(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_pa_bcsel_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_sig_en_lv(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_trf_sig_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_trf_ldo_vref_sel_lv(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_trf_ldo_vref_sel_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 17usize)) | (((val as u32) & 0x0f) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pa_out_pu_lv(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_pa_out_pu_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pa_buf_pu_lv(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_pa_buf_pu_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
}
impl Default for TrfReg1 {
    #[inline(always)]
    fn default() -> TrfReg1 {
        TrfReg1(0)
    }
}
impl core::fmt::Debug for TrfReg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrfReg1")
            .field("brf_pa_cas_bp_lv", &self.brf_pa_cas_bp_lv())
            .field("brf_pa_pm_lv", &self.brf_pa_pm_lv())
            .field("brf_pa_vc_lv", &self.brf_pa_vc_lv())
            .field("brf_pa_rstn_lv", &self.brf_pa_rstn_lv())
            .field("brf_pa_setbc_lv", &self.brf_pa_setbc_lv())
            .field("brf_pa_setsgn_lv", &self.brf_pa_setsgn_lv())
            .field("brf_pa_bcsel_lv", &self.brf_pa_bcsel_lv())
            .field("brf_trf_sig_en_lv", &self.brf_trf_sig_en_lv())
            .field("brf_trf_ldo_vref_sel_lv", &self.brf_trf_ldo_vref_sel_lv())
            .field("brf_pa_out_pu_lv", &self.brf_pa_out_pu_lv())
            .field("brf_pa_buf_pu_lv", &self.brf_pa_buf_pu_lv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrfReg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TrfReg1 {{ brf_pa_cas_bp_lv: {=bool:?}, brf_pa_pm_lv: {=u8:?}, brf_pa_vc_lv: {=u8:?}, brf_pa_rstn_lv: {=bool:?}, brf_pa_setbc_lv: {=u8:?}, brf_pa_setsgn_lv: {=bool:?}, brf_pa_bcsel_lv: {=bool:?}, brf_trf_sig_en_lv: {=bool:?}, brf_trf_ldo_vref_sel_lv: {=u8:?}, brf_pa_out_pu_lv: {=bool:?}, brf_pa_buf_pu_lv: {=bool:?} }}" , self . brf_pa_cas_bp_lv () , self . brf_pa_pm_lv () , self . brf_pa_vc_lv () , self . brf_pa_rstn_lv () , self . brf_pa_setbc_lv () , self . brf_pa_setsgn_lv () , self . brf_pa_bcsel_lv () , self . brf_trf_sig_en_lv () , self . brf_trf_ldo_vref_sel_lv () , self . brf_pa_out_pu_lv () , self . brf_pa_buf_pu_lv ())
    }
}
#[doc = "TRF_REG2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrfReg2(pub u32);
impl TrfReg2 {
    #[must_use]
    #[inline(always)]
    pub const fn brf_pa_atten_gain_lv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_pa_atten_gain_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pa_atten_en_lv(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_pa_atten_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pa_match2_lv(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_pa_match2_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pa_match1_lv(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_pa_match1_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pa_tx_rx_lv(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_pa_tx_rx_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pa_mcap_lv(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_pa_mcap_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pa_unit_sel_lv(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_pa_unit_sel_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pa_bufload_sel_lv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_pa_bufload_sel_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_pa_bm_lv(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_pa_bm_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
}
impl Default for TrfReg2 {
    #[inline(always)]
    fn default() -> TrfReg2 {
        TrfReg2(0)
    }
}
impl core::fmt::Debug for TrfReg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrfReg2")
            .field("brf_pa_atten_gain_lv", &self.brf_pa_atten_gain_lv())
            .field("brf_pa_atten_en_lv", &self.brf_pa_atten_en_lv())
            .field("brf_pa_match2_lv", &self.brf_pa_match2_lv())
            .field("brf_pa_match1_lv", &self.brf_pa_match1_lv())
            .field("brf_pa_tx_rx_lv", &self.brf_pa_tx_rx_lv())
            .field("brf_pa_mcap_lv", &self.brf_pa_mcap_lv())
            .field("brf_pa_unit_sel_lv", &self.brf_pa_unit_sel_lv())
            .field("brf_pa_bufload_sel_lv", &self.brf_pa_bufload_sel_lv())
            .field("brf_pa_bm_lv", &self.brf_pa_bm_lv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrfReg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TrfReg2 {{ brf_pa_atten_gain_lv: {=u8:?}, brf_pa_atten_en_lv: {=bool:?}, brf_pa_match2_lv: {=u8:?}, brf_pa_match1_lv: {=u8:?}, brf_pa_tx_rx_lv: {=bool:?}, brf_pa_mcap_lv: {=bool:?}, brf_pa_unit_sel_lv: {=u8:?}, brf_pa_bufload_sel_lv: {=u8:?}, brf_pa_bm_lv: {=u8:?} }}" , self . brf_pa_atten_gain_lv () , self . brf_pa_atten_en_lv () , self . brf_pa_match2_lv () , self . brf_pa_match1_lv () , self . brf_pa_tx_rx_lv () , self . brf_pa_mcap_lv () , self . brf_pa_unit_sel_lv () , self . brf_pa_bufload_sel_lv () , self . brf_pa_bm_lv ())
    }
}
#[doc = "VCO_REG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VcoReg1(pub u32);
impl VcoReg1 {
    #[must_use]
    #[inline(always)]
    pub const fn brf_en_2m_mod_lv(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_en_2m_mod_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco_var_vvn_bm_lv(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_vco_var_vvn_bm_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco_cbank_vvn_bm_lv(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_vco_cbank_vvn_bm_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco_flt_en_lv(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_vco_flt_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco_ldo_vref_lv(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_vco_ldo_vref_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco5g_en_lv(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_vco5g_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco3g_en_lv(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_vco3g_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for VcoReg1 {
    #[inline(always)]
    fn default() -> VcoReg1 {
        VcoReg1(0)
    }
}
impl core::fmt::Debug for VcoReg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VcoReg1")
            .field("brf_en_2m_mod_lv", &self.brf_en_2m_mod_lv())
            .field("brf_vco_var_vvn_bm_lv", &self.brf_vco_var_vvn_bm_lv())
            .field("brf_vco_cbank_vvn_bm_lv", &self.brf_vco_cbank_vvn_bm_lv())
            .field("brf_vco_flt_en_lv", &self.brf_vco_flt_en_lv())
            .field("brf_vco_ldo_vref_lv", &self.brf_vco_ldo_vref_lv())
            .field("brf_vco5g_en_lv", &self.brf_vco5g_en_lv())
            .field("brf_vco3g_en_lv", &self.brf_vco3g_en_lv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VcoReg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "VcoReg1 {{ brf_en_2m_mod_lv: {=bool:?}, brf_vco_var_vvn_bm_lv: {=u8:?}, brf_vco_cbank_vvn_bm_lv: {=u8:?}, brf_vco_flt_en_lv: {=bool:?}, brf_vco_ldo_vref_lv: {=u8:?}, brf_vco5g_en_lv: {=bool:?}, brf_vco3g_en_lv: {=bool:?} }}" , self . brf_en_2m_mod_lv () , self . brf_vco_var_vvn_bm_lv () , self . brf_vco_cbank_vvn_bm_lv () , self . brf_vco_flt_en_lv () , self . brf_vco_ldo_vref_lv () , self . brf_vco5g_en_lv () , self . brf_vco3g_en_lv ())
    }
}
#[doc = "VCO_REG2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VcoReg2(pub u32);
impl VcoReg2 {
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco_acal_vl_sel_lv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_vco_acal_vl_sel_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco_acal_vh_sel_lv(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_vco_acal_vh_sel_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco_acal_en_lv(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_vco_acal_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco_incfcal_vl_sel_lv(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_vco_incfcal_vl_sel_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco_incfcal_vh_sel_lv(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_vco_incfcal_vh_sel_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco_incfcal_en_lv(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_vco_incfcal_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco_fkcal_vc_sel_lv(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_vco_fkcal_vc_sel_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco_fkcal_en_lv(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_vco_fkcal_en_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_en_mod_inphase_lv(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_en_mod_inphase_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco3g_acal_up_lv(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_vco3g_acal_up_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco3g_acal_incal_lv(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_vco3g_acal_incal_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco3g_incfcal_up_lv(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_vco3g_incfcal_up_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco3g_incfcal_incal_lv(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_vco3g_incfcal_incal_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco5g_acal_up_lv(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_vco5g_acal_up_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco5g_acal_incal_lv(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_vco5g_acal_incal_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco5g_incfcal_up_lv(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_vco5g_incfcal_up_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco5g_incfcal_incal_lv(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_brf_vco5g_incfcal_incal_lv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for VcoReg2 {
    #[inline(always)]
    fn default() -> VcoReg2 {
        VcoReg2(0)
    }
}
impl core::fmt::Debug for VcoReg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VcoReg2")
            .field("brf_vco_acal_vl_sel_lv", &self.brf_vco_acal_vl_sel_lv())
            .field("brf_vco_acal_vh_sel_lv", &self.brf_vco_acal_vh_sel_lv())
            .field("brf_vco_acal_en_lv", &self.brf_vco_acal_en_lv())
            .field(
                "brf_vco_incfcal_vl_sel_lv",
                &self.brf_vco_incfcal_vl_sel_lv(),
            )
            .field(
                "brf_vco_incfcal_vh_sel_lv",
                &self.brf_vco_incfcal_vh_sel_lv(),
            )
            .field("brf_vco_incfcal_en_lv", &self.brf_vco_incfcal_en_lv())
            .field("brf_vco_fkcal_vc_sel_lv", &self.brf_vco_fkcal_vc_sel_lv())
            .field("brf_vco_fkcal_en_lv", &self.brf_vco_fkcal_en_lv())
            .field("brf_en_mod_inphase_lv", &self.brf_en_mod_inphase_lv())
            .field("brf_vco3g_acal_up_lv", &self.brf_vco3g_acal_up_lv())
            .field("brf_vco3g_acal_incal_lv", &self.brf_vco3g_acal_incal_lv())
            .field("brf_vco3g_incfcal_up_lv", &self.brf_vco3g_incfcal_up_lv())
            .field(
                "brf_vco3g_incfcal_incal_lv",
                &self.brf_vco3g_incfcal_incal_lv(),
            )
            .field("brf_vco5g_acal_up_lv", &self.brf_vco5g_acal_up_lv())
            .field("brf_vco5g_acal_incal_lv", &self.brf_vco5g_acal_incal_lv())
            .field("brf_vco5g_incfcal_up_lv", &self.brf_vco5g_incfcal_up_lv())
            .field(
                "brf_vco5g_incfcal_incal_lv",
                &self.brf_vco5g_incfcal_incal_lv(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VcoReg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "VcoReg2 {{ brf_vco_acal_vl_sel_lv: {=u8:?}, brf_vco_acal_vh_sel_lv: {=u8:?}, brf_vco_acal_en_lv: {=bool:?}, brf_vco_incfcal_vl_sel_lv: {=u8:?}, brf_vco_incfcal_vh_sel_lv: {=u8:?}, brf_vco_incfcal_en_lv: {=bool:?}, brf_vco_fkcal_vc_sel_lv: {=u8:?}, brf_vco_fkcal_en_lv: {=bool:?}, brf_en_mod_inphase_lv: {=bool:?}, brf_vco3g_acal_up_lv: {=bool:?}, brf_vco3g_acal_incal_lv: {=bool:?}, brf_vco3g_incfcal_up_lv: {=bool:?}, brf_vco3g_incfcal_incal_lv: {=bool:?}, brf_vco5g_acal_up_lv: {=bool:?}, brf_vco5g_acal_incal_lv: {=bool:?}, brf_vco5g_incfcal_up_lv: {=bool:?}, brf_vco5g_incfcal_incal_lv: {=bool:?} }}" , self . brf_vco_acal_vl_sel_lv () , self . brf_vco_acal_vh_sel_lv () , self . brf_vco_acal_en_lv () , self . brf_vco_incfcal_vl_sel_lv () , self . brf_vco_incfcal_vh_sel_lv () , self . brf_vco_incfcal_en_lv () , self . brf_vco_fkcal_vc_sel_lv () , self . brf_vco_fkcal_en_lv () , self . brf_en_mod_inphase_lv () , self . brf_vco3g_acal_up_lv () , self . brf_vco3g_acal_incal_lv () , self . brf_vco3g_incfcal_up_lv () , self . brf_vco3g_incfcal_incal_lv () , self . brf_vco5g_acal_up_lv () , self . brf_vco5g_acal_incal_lv () , self . brf_vco5g_incfcal_up_lv () , self . brf_vco5g_incfcal_incal_lv ())
    }
}
#[doc = "VCO_REG3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VcoReg3(pub u32);
impl VcoReg3 {
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco_pdx_lv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_vco_pdx_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn brf_vco_idac_lv(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_brf_vco_idac_lv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_kcal(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_tx_kcal(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for VcoReg3 {
    #[inline(always)]
    fn default() -> VcoReg3 {
        VcoReg3(0)
    }
}
impl core::fmt::Debug for VcoReg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VcoReg3")
            .field("brf_vco_pdx_lv", &self.brf_vco_pdx_lv())
            .field("brf_vco_idac_lv", &self.brf_vco_idac_lv())
            .field("tx_kcal", &self.tx_kcal())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VcoReg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "VcoReg3 {{ brf_vco_pdx_lv: {=u8:?}, brf_vco_idac_lv: {=u8:?}, tx_kcal: {=u16:?} }}",
            self.brf_vco_pdx_lv(),
            self.brf_vco_idac_lv(),
            self.tx_kcal()
        )
    }
}
