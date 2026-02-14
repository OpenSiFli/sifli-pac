#[doc = "AGC_CFG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AgcCfg1(pub u32);
impl AgcCfg1 {
    #[must_use]
    #[inline(always)]
    pub const fn adc_mag_thd0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_adc_mag_thd0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_mag_thd1(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_adc_mag_thd1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_mag_thd2(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_adc_mag_thd2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
    }
}
impl Default for AgcCfg1 {
    #[inline(always)]
    fn default() -> AgcCfg1 {
        AgcCfg1(0)
    }
}
impl core::fmt::Debug for AgcCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AgcCfg1")
            .field("adc_mag_thd0", &self.adc_mag_thd0())
            .field("adc_mag_thd1", &self.adc_mag_thd1())
            .field("adc_mag_thd2", &self.adc_mag_thd2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AgcCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AgcCfg1 {{ adc_mag_thd0: {=u16:?}, adc_mag_thd1: {=u16:?}, adc_mag_thd2: {=u16:?} }}",
            self.adc_mag_thd0(),
            self.adc_mag_thd1(),
            self.adc_mag_thd2()
        )
    }
}
#[doc = "AGC_CFG10"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AgcCfg10(pub u32);
impl AgcCfg10 {
    #[must_use]
    #[inline(always)]
    pub const fn agc_peakdet_timer_set1_1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_peakdet_timer_set1_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn agc_peakdet_cnt_thd1_1(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_peakdet_cnt_thd1_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn agc_peakdet_timer_set2_1(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_peakdet_timer_set2_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 14usize)) | (((val as u32) & 0x7f) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn agc_peakdet_cnt_thd2_1(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_peakdet_cnt_thd2_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 21usize)) | (((val as u32) & 0x7f) << 21usize);
    }
}
impl Default for AgcCfg10 {
    #[inline(always)]
    fn default() -> AgcCfg10 {
        AgcCfg10(0)
    }
}
impl core::fmt::Debug for AgcCfg10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AgcCfg10")
            .field("agc_peakdet_timer_set1_1", &self.agc_peakdet_timer_set1_1())
            .field("agc_peakdet_cnt_thd1_1", &self.agc_peakdet_cnt_thd1_1())
            .field("agc_peakdet_timer_set2_1", &self.agc_peakdet_timer_set2_1())
            .field("agc_peakdet_cnt_thd2_1", &self.agc_peakdet_cnt_thd2_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AgcCfg10 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "AgcCfg10 {{ agc_peakdet_timer_set1_1: {=u8:?}, agc_peakdet_cnt_thd1_1: {=u8:?}, agc_peakdet_timer_set2_1: {=u8:?}, agc_peakdet_cnt_thd2_1: {=u8:?} }}" , self . agc_peakdet_timer_set1_1 () , self . agc_peakdet_cnt_thd1_1 () , self . agc_peakdet_timer_set2_1 () , self . agc_peakdet_cnt_thd2_1 ())
    }
}
#[doc = "AGC_CFG11"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AgcCfg11(pub u32);
impl AgcCfg11 {
    #[must_use]
    #[inline(always)]
    pub const fn agc_peakdet_timer_set1_2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_peakdet_timer_set1_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn agc_peakdet_cnt_thd1_2(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_peakdet_cnt_thd1_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn agc_peakdet_timer_set2_2(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_peakdet_timer_set2_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 14usize)) | (((val as u32) & 0x7f) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn agc_peakdet_cnt_thd2_2(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_peakdet_cnt_thd2_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 21usize)) | (((val as u32) & 0x7f) << 21usize);
    }
}
impl Default for AgcCfg11 {
    #[inline(always)]
    fn default() -> AgcCfg11 {
        AgcCfg11(0)
    }
}
impl core::fmt::Debug for AgcCfg11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AgcCfg11")
            .field("agc_peakdet_timer_set1_2", &self.agc_peakdet_timer_set1_2())
            .field("agc_peakdet_cnt_thd1_2", &self.agc_peakdet_cnt_thd1_2())
            .field("agc_peakdet_timer_set2_2", &self.agc_peakdet_timer_set2_2())
            .field("agc_peakdet_cnt_thd2_2", &self.agc_peakdet_cnt_thd2_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AgcCfg11 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "AgcCfg11 {{ agc_peakdet_timer_set1_2: {=u8:?}, agc_peakdet_cnt_thd1_2: {=u8:?}, agc_peakdet_timer_set2_2: {=u8:?}, agc_peakdet_cnt_thd2_2: {=u8:?} }}" , self . agc_peakdet_timer_set1_2 () , self . agc_peakdet_cnt_thd1_2 () , self . agc_peakdet_timer_set2_2 () , self . agc_peakdet_cnt_thd2_2 ())
    }
}
#[doc = "AGC_CFG12"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AgcCfg12(pub u32);
impl AgcCfg12 {
    #[must_use]
    #[inline(always)]
    pub const fn agc_urun_window_1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_urun_window_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn agc_urun_window_2(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_urun_window_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_power_urun_thd(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_adc_power_urun_thd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 14usize)) | (((val as u32) & 0x7f) << 14usize);
    }
}
impl Default for AgcCfg12 {
    #[inline(always)]
    fn default() -> AgcCfg12 {
        AgcCfg12(0)
    }
}
impl core::fmt::Debug for AgcCfg12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AgcCfg12")
            .field("agc_urun_window_1", &self.agc_urun_window_1())
            .field("agc_urun_window_2", &self.agc_urun_window_2())
            .field("adc_power_urun_thd", &self.adc_power_urun_thd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AgcCfg12 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "AgcCfg12 {{ agc_urun_window_1: {=u8:?}, agc_urun_window_2: {=u8:?}, adc_power_urun_thd: {=u8:?} }}" , self . agc_urun_window_1 () , self . agc_urun_window_2 () , self . adc_power_urun_thd ())
    }
}
#[doc = "AGC_CFG2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AgcCfg2(pub u32);
impl AgcCfg2 {
    #[must_use]
    #[inline(always)]
    pub const fn adc_mag_cnt_thd0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_adc_mag_cnt_thd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_mag_cnt_thd1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_adc_mag_cnt_thd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_mag_cnt_thd2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_adc_mag_cnt_thd2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_mag_set(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_adc_mag_set(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 12usize)) | (((val as u32) & 0x03ff) << 12usize);
    }
}
impl Default for AgcCfg2 {
    #[inline(always)]
    fn default() -> AgcCfg2 {
        AgcCfg2(0)
    }
}
impl core::fmt::Debug for AgcCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AgcCfg2")
            .field("adc_mag_cnt_thd0", &self.adc_mag_cnt_thd0())
            .field("adc_mag_cnt_thd1", &self.adc_mag_cnt_thd1())
            .field("adc_mag_cnt_thd2", &self.adc_mag_cnt_thd2())
            .field("adc_mag_set", &self.adc_mag_set())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AgcCfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "AgcCfg2 {{ adc_mag_cnt_thd0: {=u8:?}, adc_mag_cnt_thd1: {=u8:?}, adc_mag_cnt_thd2: {=u8:?}, adc_mag_set: {=u16:?} }}" , self . adc_mag_cnt_thd0 () , self . adc_mag_cnt_thd1 () , self . adc_mag_cnt_thd2 () , self . adc_mag_set ())
    }
}
#[doc = "AGC_CFG3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AgcCfg3(pub u32);
impl AgcCfg3 {
    #[must_use]
    #[inline(always)]
    pub const fn adc_sat_thd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_adc_sat_thd(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_sat_num(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_adc_sat_num(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_sat_thd_bt(&self) -> u16 {
        let val = (self.0 >> 14usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_adc_sat_thd_bt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 14usize)) | (((val as u32) & 0x03ff) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_sat_num_bt(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_adc_sat_num_bt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for AgcCfg3 {
    #[inline(always)]
    fn default() -> AgcCfg3 {
        AgcCfg3(0)
    }
}
impl core::fmt::Debug for AgcCfg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AgcCfg3")
            .field("adc_sat_thd", &self.adc_sat_thd())
            .field("adc_sat_num", &self.adc_sat_num())
            .field("adc_sat_thd_bt", &self.adc_sat_thd_bt())
            .field("adc_sat_num_bt", &self.adc_sat_num_bt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AgcCfg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "AgcCfg3 {{ adc_sat_thd: {=u16:?}, adc_sat_num: {=u8:?}, adc_sat_thd_bt: {=u16:?}, adc_sat_num_bt: {=u8:?} }}" , self . adc_sat_thd () , self . adc_sat_num () , self . adc_sat_thd_bt () , self . adc_sat_num_bt ())
    }
}
#[doc = "AGC_CFG4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AgcCfg4(pub u32);
impl AgcCfg4 {
    #[must_use]
    #[inline(always)]
    pub const fn lna_mixer_gain_index_thd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_lna_mixer_gain_index_thd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cbpf_gain_index_thd(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_cbpf_gain_index_thd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vga_gain_index_thd(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_vga_gain_index_thd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lna_mixer_gain_index_init(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_lna_mixer_gain_index_init(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cbpf_gain_index_init(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_cbpf_gain_index_init(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vga_gain_index_init(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_vga_gain_index_init(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lna_mixer_gain_index_step(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_lna_mixer_gain_index_step(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cbpf_gain_index_step(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_cbpf_gain_index_step(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vga_gain_index_step(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_vga_gain_index_step(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 26usize)) | (((val as u32) & 0x0f) << 26usize);
    }
}
impl Default for AgcCfg4 {
    #[inline(always)]
    fn default() -> AgcCfg4 {
        AgcCfg4(0)
    }
}
impl core::fmt::Debug for AgcCfg4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AgcCfg4")
            .field("lna_mixer_gain_index_thd", &self.lna_mixer_gain_index_thd())
            .field("cbpf_gain_index_thd", &self.cbpf_gain_index_thd())
            .field("vga_gain_index_thd", &self.vga_gain_index_thd())
            .field(
                "lna_mixer_gain_index_init",
                &self.lna_mixer_gain_index_init(),
            )
            .field("cbpf_gain_index_init", &self.cbpf_gain_index_init())
            .field("vga_gain_index_init", &self.vga_gain_index_init())
            .field(
                "lna_mixer_gain_index_step",
                &self.lna_mixer_gain_index_step(),
            )
            .field("cbpf_gain_index_step", &self.cbpf_gain_index_step())
            .field("vga_gain_index_step", &self.vga_gain_index_step())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AgcCfg4 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "AgcCfg4 {{ lna_mixer_gain_index_thd: {=u8:?}, cbpf_gain_index_thd: {=u8:?}, vga_gain_index_thd: {=u8:?}, lna_mixer_gain_index_init: {=u8:?}, cbpf_gain_index_init: {=u8:?}, vga_gain_index_init: {=u8:?}, lna_mixer_gain_index_step: {=u8:?}, cbpf_gain_index_step: {=u8:?}, vga_gain_index_step: {=u8:?} }}" , self . lna_mixer_gain_index_thd () , self . cbpf_gain_index_thd () , self . vga_gain_index_thd () , self . lna_mixer_gain_index_init () , self . cbpf_gain_index_init () , self . vga_gain_index_init () , self . lna_mixer_gain_index_step () , self . cbpf_gain_index_step () , self . vga_gain_index_step ())
    }
}
#[doc = "AGC_CFG5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AgcCfg5(pub u32);
impl AgcCfg5 {
    #[must_use]
    #[inline(always)]
    pub const fn agc_cbpf_gain_index_setting0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_cbpf_gain_index_setting0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn agc_vga_gain_index_setting0(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_vga_gain_index_setting0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dig_gain_low(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_dig_gain_low(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dig_gain_high(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_dig_gain_high(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_power_target_bt(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_adc_power_target_bt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 18usize)) | (((val as u32) & 0x7f) << 18usize);
    }
}
impl Default for AgcCfg5 {
    #[inline(always)]
    fn default() -> AgcCfg5 {
        AgcCfg5(0)
    }
}
impl core::fmt::Debug for AgcCfg5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AgcCfg5")
            .field(
                "agc_cbpf_gain_index_setting0",
                &self.agc_cbpf_gain_index_setting0(),
            )
            .field(
                "agc_vga_gain_index_setting0",
                &self.agc_vga_gain_index_setting0(),
            )
            .field("dig_gain_low", &self.dig_gain_low())
            .field("dig_gain_high", &self.dig_gain_high())
            .field("adc_power_target_bt", &self.adc_power_target_bt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AgcCfg5 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "AgcCfg5 {{ agc_cbpf_gain_index_setting0: {=u8:?}, agc_vga_gain_index_setting0: {=u8:?}, dig_gain_low: {=u8:?}, dig_gain_high: {=u8:?}, adc_power_target_bt: {=u8:?} }}" , self . agc_cbpf_gain_index_setting0 () , self . agc_vga_gain_index_setting0 () , self . dig_gain_low () , self . dig_gain_high () , self . adc_power_target_bt ())
    }
}
#[doc = "AGC_CFG6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AgcCfg6(pub u32);
impl AgcCfg6 {
    #[must_use]
    #[inline(always)]
    pub const fn agc_delay_reset_1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_delay_reset_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn agc_delay_pkdet_1(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_delay_pkdet_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn agc_delay_lna_1(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_delay_lna_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 14usize)) | (((val as u32) & 0x7f) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn agc_delay_dig_1(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_delay_dig_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 21usize)) | (((val as u32) & 0x7f) << 21usize);
    }
}
impl Default for AgcCfg6 {
    #[inline(always)]
    fn default() -> AgcCfg6 {
        AgcCfg6(0)
    }
}
impl core::fmt::Debug for AgcCfg6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AgcCfg6")
            .field("agc_delay_reset_1", &self.agc_delay_reset_1())
            .field("agc_delay_pkdet_1", &self.agc_delay_pkdet_1())
            .field("agc_delay_lna_1", &self.agc_delay_lna_1())
            .field("agc_delay_dig_1", &self.agc_delay_dig_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AgcCfg6 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "AgcCfg6 {{ agc_delay_reset_1: {=u8:?}, agc_delay_pkdet_1: {=u8:?}, agc_delay_lna_1: {=u8:?}, agc_delay_dig_1: {=u8:?} }}" , self . agc_delay_reset_1 () , self . agc_delay_pkdet_1 () , self . agc_delay_lna_1 () , self . agc_delay_dig_1 ())
    }
}
#[doc = "AGC_CFG7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AgcCfg7(pub u32);
impl AgcCfg7 {
    #[must_use]
    #[inline(always)]
    pub const fn agc_delay_reset_2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_delay_reset_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn agc_delay_pkdet_2(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_delay_pkdet_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn agc_delay_lna_2(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_delay_lna_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 14usize)) | (((val as u32) & 0x7f) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn agc_delay_dig_2(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_delay_dig_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 21usize)) | (((val as u32) & 0x7f) << 21usize);
    }
}
impl Default for AgcCfg7 {
    #[inline(always)]
    fn default() -> AgcCfg7 {
        AgcCfg7(0)
    }
}
impl core::fmt::Debug for AgcCfg7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AgcCfg7")
            .field("agc_delay_reset_2", &self.agc_delay_reset_2())
            .field("agc_delay_pkdet_2", &self.agc_delay_pkdet_2())
            .field("agc_delay_lna_2", &self.agc_delay_lna_2())
            .field("agc_delay_dig_2", &self.agc_delay_dig_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AgcCfg7 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "AgcCfg7 {{ agc_delay_reset_2: {=u8:?}, agc_delay_pkdet_2: {=u8:?}, agc_delay_lna_2: {=u8:?}, agc_delay_dig_2: {=u8:?} }}" , self . agc_delay_reset_2 () , self . agc_delay_pkdet_2 () , self . agc_delay_lna_2 () , self . agc_delay_dig_2 ())
    }
}
#[doc = "AGC_CFG8"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AgcCfg8(pub u32);
impl AgcCfg8 {
    #[must_use]
    #[inline(always)]
    pub const fn agc_delay_cbpf_1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_delay_cbpf_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn agc_delay_adc_1(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_delay_adc_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dig_gain_window_1(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_dig_gain_window_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 14usize)) | (((val as u32) & 0x7f) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_power_target(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_adc_power_target(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 21usize)) | (((val as u32) & 0x7f) << 21usize);
    }
}
impl Default for AgcCfg8 {
    #[inline(always)]
    fn default() -> AgcCfg8 {
        AgcCfg8(0)
    }
}
impl core::fmt::Debug for AgcCfg8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AgcCfg8")
            .field("agc_delay_cbpf_1", &self.agc_delay_cbpf_1())
            .field("agc_delay_adc_1", &self.agc_delay_adc_1())
            .field("dig_gain_window_1", &self.dig_gain_window_1())
            .field("adc_power_target", &self.adc_power_target())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AgcCfg8 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "AgcCfg8 {{ agc_delay_cbpf_1: {=u8:?}, agc_delay_adc_1: {=u8:?}, dig_gain_window_1: {=u8:?}, adc_power_target: {=u8:?} }}" , self . agc_delay_cbpf_1 () , self . agc_delay_adc_1 () , self . dig_gain_window_1 () , self . adc_power_target ())
    }
}
#[doc = "AGC_CFG9"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AgcCfg9(pub u32);
impl AgcCfg9 {
    #[must_use]
    #[inline(always)]
    pub const fn agc_delay_cbpf_2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_delay_cbpf_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn agc_delay_adc_2(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_delay_adc_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dig_gain_window_2(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_dig_gain_window_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 14usize)) | (((val as u32) & 0x7f) << 14usize);
    }
}
impl Default for AgcCfg9 {
    #[inline(always)]
    fn default() -> AgcCfg9 {
        AgcCfg9(0)
    }
}
impl core::fmt::Debug for AgcCfg9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AgcCfg9")
            .field("agc_delay_cbpf_2", &self.agc_delay_cbpf_2())
            .field("agc_delay_adc_2", &self.agc_delay_adc_2())
            .field("dig_gain_window_2", &self.dig_gain_window_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AgcCfg9 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "AgcCfg9 {{ agc_delay_cbpf_2: {=u8:?}, agc_delay_adc_2: {=u8:?}, dig_gain_window_2: {=u8:?} }}" , self . agc_delay_cbpf_2 () , self . agc_delay_adc_2 () , self . dig_gain_window_2 ())
    }
}
#[doc = "AGC_CTRL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AgcCtrl(pub u32);
impl AgcCtrl {
    #[must_use]
    #[inline(always)]
    pub const fn agc_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_agc_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn agc_mode(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_agc_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dig_gain_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_dig_gain_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn agc_vgaadj_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_agc_vgaadj_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for AgcCtrl {
    #[inline(always)]
    fn default() -> AgcCtrl {
        AgcCtrl(0)
    }
}
impl core::fmt::Debug for AgcCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AgcCtrl")
            .field("agc_enable", &self.agc_enable())
            .field("agc_mode", &self.agc_mode())
            .field("dig_gain_en", &self.dig_gain_en())
            .field("agc_vgaadj_en", &self.agc_vgaadj_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AgcCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "AgcCtrl {{ agc_enable: {=bool:?}, agc_mode: {=bool:?}, dig_gain_en: {=bool:?}, agc_vgaadj_en: {=bool:?} }}" , self . agc_enable () , self . agc_mode () , self . dig_gain_en () , self . agc_vgaadj_en ())
    }
}
#[doc = "AGC_STATUS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AgcStatus(pub u32);
impl AgcStatus {
    #[must_use]
    #[inline(always)]
    pub const fn lna_mixer_gain_index(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_lna_mixer_gain_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cbpf_gain_index(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_cbpf_gain_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vga_gain_index(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_vga_gain_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_dig_gain(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_adc_dig_gain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rssi(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rssi(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pkdet_ana(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_pkdet_ana(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
}
impl Default for AgcStatus {
    #[inline(always)]
    fn default() -> AgcStatus {
        AgcStatus(0)
    }
}
impl core::fmt::Debug for AgcStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AgcStatus")
            .field("lna_mixer_gain_index", &self.lna_mixer_gain_index())
            .field("cbpf_gain_index", &self.cbpf_gain_index())
            .field("vga_gain_index", &self.vga_gain_index())
            .field("adc_dig_gain", &self.adc_dig_gain())
            .field("rssi", &self.rssi())
            .field("pkdet_ana", &self.pkdet_ana())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AgcStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "AgcStatus {{ lna_mixer_gain_index: {=u8:?}, cbpf_gain_index: {=u8:?}, vga_gain_index: {=u8:?}, adc_dig_gain: {=u8:?}, rssi: {=u8:?}, pkdet_ana: {=u8:?} }}" , self . lna_mixer_gain_index () , self . cbpf_gain_index () , self . vga_gain_index () , self . adc_dig_gain () , self . rssi () , self . pkdet_ana ())
    }
}
#[doc = "DEMOD_CFG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DemodCfg1(pub u32);
impl DemodCfg1 {
    #[must_use]
    #[inline(always)]
    pub const fn ble_mu_err(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ble_mu_err(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ble_mu_dc(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ble_mu_dc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ble_demod_g(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ble_demod_g(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 20usize)) | (((val as u32) & 0x07ff) << 20usize);
    }
}
impl Default for DemodCfg1 {
    #[inline(always)]
    fn default() -> DemodCfg1 {
        DemodCfg1(0)
    }
}
impl core::fmt::Debug for DemodCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DemodCfg1")
            .field("ble_mu_err", &self.ble_mu_err())
            .field("ble_mu_dc", &self.ble_mu_dc())
            .field("ble_demod_g", &self.ble_demod_g())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DemodCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DemodCfg1 {{ ble_mu_err: {=u16:?}, ble_mu_dc: {=u16:?}, ble_demod_g: {=u16:?} }}",
            self.ble_mu_err(),
            self.ble_mu_dc(),
            self.ble_demod_g()
        )
    }
}
#[doc = "DEMOD_CFG10"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DemodCfg10(pub u32);
impl DemodCfg10 {
    #[must_use]
    #[inline(always)]
    pub const fn br_demod_phase_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_br_demod_phase_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn br_demod_phase_3(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_br_demod_phase_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for DemodCfg10 {
    #[inline(always)]
    fn default() -> DemodCfg10 {
        DemodCfg10(0)
    }
}
impl core::fmt::Debug for DemodCfg10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DemodCfg10")
            .field("br_demod_phase_2", &self.br_demod_phase_2())
            .field("br_demod_phase_3", &self.br_demod_phase_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DemodCfg10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DemodCfg10 {{ br_demod_phase_2: {=u16:?}, br_demod_phase_3: {=u16:?} }}",
            self.br_demod_phase_2(),
            self.br_demod_phase_3()
        )
    }
}
#[doc = "DEMOD_CFG11"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DemodCfg11(pub u32);
impl DemodCfg11 {
    #[must_use]
    #[inline(always)]
    pub const fn br_demod_phase_4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_br_demod_phase_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn br_demod_phase_5(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_br_demod_phase_5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for DemodCfg11 {
    #[inline(always)]
    fn default() -> DemodCfg11 {
        DemodCfg11(0)
    }
}
impl core::fmt::Debug for DemodCfg11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DemodCfg11")
            .field("br_demod_phase_4", &self.br_demod_phase_4())
            .field("br_demod_phase_5", &self.br_demod_phase_5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DemodCfg11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DemodCfg11 {{ br_demod_phase_4: {=u16:?}, br_demod_phase_5: {=u16:?} }}",
            self.br_demod_phase_4(),
            self.br_demod_phase_5()
        )
    }
}
#[doc = "DEMOD_CFG12"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DemodCfg12(pub u32);
impl DemodCfg12 {
    #[must_use]
    #[inline(always)]
    pub const fn br_demod_phase_6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_br_demod_phase_6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn br_demod_phase_7(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_br_demod_phase_7(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for DemodCfg12 {
    #[inline(always)]
    fn default() -> DemodCfg12 {
        DemodCfg12(0)
    }
}
impl core::fmt::Debug for DemodCfg12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DemodCfg12")
            .field("br_demod_phase_6", &self.br_demod_phase_6())
            .field("br_demod_phase_7", &self.br_demod_phase_7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DemodCfg12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DemodCfg12 {{ br_demod_phase_6: {=u16:?}, br_demod_phase_7: {=u16:?} }}",
            self.br_demod_phase_6(),
            self.br_demod_phase_7()
        )
    }
}
#[doc = "DEMOD_CFG13"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DemodCfg13(pub u32);
impl DemodCfg13 {
    #[must_use]
    #[inline(always)]
    pub const fn br_demod_phase_ideal_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_br_demod_phase_ideal_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn br_demod_phase_ideal_1(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_br_demod_phase_ideal_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for DemodCfg13 {
    #[inline(always)]
    fn default() -> DemodCfg13 {
        DemodCfg13(0)
    }
}
impl core::fmt::Debug for DemodCfg13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DemodCfg13")
            .field("br_demod_phase_ideal_0", &self.br_demod_phase_ideal_0())
            .field("br_demod_phase_ideal_1", &self.br_demod_phase_ideal_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DemodCfg13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DemodCfg13 {{ br_demod_phase_ideal_0: {=u16:?}, br_demod_phase_ideal_1: {=u16:?} }}",
            self.br_demod_phase_ideal_0(),
            self.br_demod_phase_ideal_1()
        )
    }
}
#[doc = "DEMOD_CFG14"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DemodCfg14(pub u32);
impl DemodCfg14 {
    #[must_use]
    #[inline(always)]
    pub const fn br_demod_phase_ideal_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_br_demod_phase_ideal_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn br_demod_phase_ideal_3(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_br_demod_phase_ideal_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for DemodCfg14 {
    #[inline(always)]
    fn default() -> DemodCfg14 {
        DemodCfg14(0)
    }
}
impl core::fmt::Debug for DemodCfg14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DemodCfg14")
            .field("br_demod_phase_ideal_2", &self.br_demod_phase_ideal_2())
            .field("br_demod_phase_ideal_3", &self.br_demod_phase_ideal_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DemodCfg14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DemodCfg14 {{ br_demod_phase_ideal_2: {=u16:?}, br_demod_phase_ideal_3: {=u16:?} }}",
            self.br_demod_phase_ideal_2(),
            self.br_demod_phase_ideal_3()
        )
    }
}
#[doc = "DEMOD_CFG15"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DemodCfg15(pub u32);
impl DemodCfg15 {
    #[must_use]
    #[inline(always)]
    pub const fn ble_hadapt_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ble_hadapt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ble_mu_h(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ble_mu_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 1usize)) | (((val as u32) & 0x03ff) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ble_demod_h(&self) -> u16 {
        let val = (self.0 >> 11usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ble_demod_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 11usize)) | (((val as u32) & 0x07ff) << 11usize);
    }
}
impl Default for DemodCfg15 {
    #[inline(always)]
    fn default() -> DemodCfg15 {
        DemodCfg15(0)
    }
}
impl core::fmt::Debug for DemodCfg15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DemodCfg15")
            .field("ble_hadapt_en", &self.ble_hadapt_en())
            .field("ble_mu_h", &self.ble_mu_h())
            .field("ble_demod_h", &self.ble_demod_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DemodCfg15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DemodCfg15 {{ ble_hadapt_en: {=bool:?}, ble_mu_h: {=u16:?}, ble_demod_h: {=u16:?} }}",
            self.ble_hadapt_en(),
            self.ble_mu_h(),
            self.ble_demod_h()
        )
    }
}
#[doc = "DEMOD_CFG16"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DemodCfg16(pub u32);
impl DemodCfg16 {
    #[must_use]
    #[inline(always)]
    pub const fn br_hadapt_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_br_hadapt_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn br_mu_h(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_br_mu_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 1usize)) | (((val as u32) & 0x03ff) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn br_demod_h(&self) -> u16 {
        let val = (self.0 >> 11usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_br_demod_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 11usize)) | (((val as u32) & 0x07ff) << 11usize);
    }
}
impl Default for DemodCfg16 {
    #[inline(always)]
    fn default() -> DemodCfg16 {
        DemodCfg16(0)
    }
}
impl core::fmt::Debug for DemodCfg16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DemodCfg16")
            .field("br_hadapt_en", &self.br_hadapt_en())
            .field("br_mu_h", &self.br_mu_h())
            .field("br_demod_h", &self.br_demod_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DemodCfg16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DemodCfg16 {{ br_hadapt_en: {=bool:?}, br_mu_h: {=u16:?}, br_demod_h: {=u16:?} }}",
            self.br_hadapt_en(),
            self.br_mu_h(),
            self.br_demod_h()
        )
    }
}
#[doc = "DEMOD_CFG17"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DemodCfg17(pub u32);
impl DemodCfg17 {
    #[must_use]
    #[inline(always)]
    pub const fn hadapt_l(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_hadapt_l(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn hadapt_h(&self) -> u16 {
        let val = (self.0 >> 11usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_hadapt_h(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 11usize)) | (((val as u32) & 0x07ff) << 11usize);
    }
}
impl Default for DemodCfg17 {
    #[inline(always)]
    fn default() -> DemodCfg17 {
        DemodCfg17(0)
    }
}
impl core::fmt::Debug for DemodCfg17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DemodCfg17")
            .field("hadapt_l", &self.hadapt_l())
            .field("hadapt_h", &self.hadapt_h())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DemodCfg17 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DemodCfg17 {{ hadapt_l: {=u16:?}, hadapt_h: {=u16:?} }}",
            self.hadapt_l(),
            self.hadapt_h()
        )
    }
}
#[doc = "DEMOD_CFG2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DemodCfg2(pub u32);
impl DemodCfg2 {
    #[must_use]
    #[inline(always)]
    pub const fn ble_demod_phase_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ble_demod_phase_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ble_demod_phase_1(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ble_demod_phase_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for DemodCfg2 {
    #[inline(always)]
    fn default() -> DemodCfg2 {
        DemodCfg2(0)
    }
}
impl core::fmt::Debug for DemodCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DemodCfg2")
            .field("ble_demod_phase_0", &self.ble_demod_phase_0())
            .field("ble_demod_phase_1", &self.ble_demod_phase_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DemodCfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DemodCfg2 {{ ble_demod_phase_0: {=u16:?}, ble_demod_phase_1: {=u16:?} }}",
            self.ble_demod_phase_0(),
            self.ble_demod_phase_1()
        )
    }
}
#[doc = "DEMOD_CFG3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DemodCfg3(pub u32);
impl DemodCfg3 {
    #[must_use]
    #[inline(always)]
    pub const fn ble_demod_phase_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ble_demod_phase_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ble_demod_phase_3(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ble_demod_phase_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for DemodCfg3 {
    #[inline(always)]
    fn default() -> DemodCfg3 {
        DemodCfg3(0)
    }
}
impl core::fmt::Debug for DemodCfg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DemodCfg3")
            .field("ble_demod_phase_2", &self.ble_demod_phase_2())
            .field("ble_demod_phase_3", &self.ble_demod_phase_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DemodCfg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DemodCfg3 {{ ble_demod_phase_2: {=u16:?}, ble_demod_phase_3: {=u16:?} }}",
            self.ble_demod_phase_2(),
            self.ble_demod_phase_3()
        )
    }
}
#[doc = "DEMOD_CFG4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DemodCfg4(pub u32);
impl DemodCfg4 {
    #[must_use]
    #[inline(always)]
    pub const fn ble_demod_phase_4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ble_demod_phase_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ble_demod_phase_5(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ble_demod_phase_5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for DemodCfg4 {
    #[inline(always)]
    fn default() -> DemodCfg4 {
        DemodCfg4(0)
    }
}
impl core::fmt::Debug for DemodCfg4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DemodCfg4")
            .field("ble_demod_phase_4", &self.ble_demod_phase_4())
            .field("ble_demod_phase_5", &self.ble_demod_phase_5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DemodCfg4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DemodCfg4 {{ ble_demod_phase_4: {=u16:?}, ble_demod_phase_5: {=u16:?} }}",
            self.ble_demod_phase_4(),
            self.ble_demod_phase_5()
        )
    }
}
#[doc = "DEMOD_CFG5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DemodCfg5(pub u32);
impl DemodCfg5 {
    #[must_use]
    #[inline(always)]
    pub const fn ble_demod_phase_6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ble_demod_phase_6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ble_demod_phase_7(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ble_demod_phase_7(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for DemodCfg5 {
    #[inline(always)]
    fn default() -> DemodCfg5 {
        DemodCfg5(0)
    }
}
impl core::fmt::Debug for DemodCfg5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DemodCfg5")
            .field("ble_demod_phase_6", &self.ble_demod_phase_6())
            .field("ble_demod_phase_7", &self.ble_demod_phase_7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DemodCfg5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DemodCfg5 {{ ble_demod_phase_6: {=u16:?}, ble_demod_phase_7: {=u16:?} }}",
            self.ble_demod_phase_6(),
            self.ble_demod_phase_7()
        )
    }
}
#[doc = "DEMOD_CFG6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DemodCfg6(pub u32);
impl DemodCfg6 {
    #[must_use]
    #[inline(always)]
    pub const fn ble_demod_phase_ideal_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ble_demod_phase_ideal_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ble_demod_phase_ideal_1(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ble_demod_phase_ideal_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for DemodCfg6 {
    #[inline(always)]
    fn default() -> DemodCfg6 {
        DemodCfg6(0)
    }
}
impl core::fmt::Debug for DemodCfg6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DemodCfg6")
            .field("ble_demod_phase_ideal_0", &self.ble_demod_phase_ideal_0())
            .field("ble_demod_phase_ideal_1", &self.ble_demod_phase_ideal_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DemodCfg6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DemodCfg6 {{ ble_demod_phase_ideal_0: {=u16:?}, ble_demod_phase_ideal_1: {=u16:?} }}",
            self.ble_demod_phase_ideal_0(),
            self.ble_demod_phase_ideal_1()
        )
    }
}
#[doc = "DEMOD_CFG7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DemodCfg7(pub u32);
impl DemodCfg7 {
    #[must_use]
    #[inline(always)]
    pub const fn ble_demod_phase_ideal_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ble_demod_phase_ideal_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ble_demod_phase_ideal_3(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ble_demod_phase_ideal_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for DemodCfg7 {
    #[inline(always)]
    fn default() -> DemodCfg7 {
        DemodCfg7(0)
    }
}
impl core::fmt::Debug for DemodCfg7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DemodCfg7")
            .field("ble_demod_phase_ideal_2", &self.ble_demod_phase_ideal_2())
            .field("ble_demod_phase_ideal_3", &self.ble_demod_phase_ideal_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DemodCfg7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DemodCfg7 {{ ble_demod_phase_ideal_2: {=u16:?}, ble_demod_phase_ideal_3: {=u16:?} }}",
            self.ble_demod_phase_ideal_2(),
            self.ble_demod_phase_ideal_3()
        )
    }
}
#[doc = "DEMOD_CFG8"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DemodCfg8(pub u32);
impl DemodCfg8 {
    #[must_use]
    #[inline(always)]
    pub const fn br_mu_err(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_br_mu_err(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn br_mu_dc(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_br_mu_dc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn br_demod_g(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_br_demod_g(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 20usize)) | (((val as u32) & 0x07ff) << 20usize);
    }
}
impl Default for DemodCfg8 {
    #[inline(always)]
    fn default() -> DemodCfg8 {
        DemodCfg8(0)
    }
}
impl core::fmt::Debug for DemodCfg8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DemodCfg8")
            .field("br_mu_err", &self.br_mu_err())
            .field("br_mu_dc", &self.br_mu_dc())
            .field("br_demod_g", &self.br_demod_g())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DemodCfg8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DemodCfg8 {{ br_mu_err: {=u16:?}, br_mu_dc: {=u16:?}, br_demod_g: {=u16:?} }}",
            self.br_mu_err(),
            self.br_mu_dc(),
            self.br_demod_g()
        )
    }
}
#[doc = "DEMOD_CFG9"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DemodCfg9(pub u32);
impl DemodCfg9 {
    #[must_use]
    #[inline(always)]
    pub const fn br_demod_phase_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_br_demod_phase_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn br_demod_phase_1(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_br_demod_phase_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for DemodCfg9 {
    #[inline(always)]
    fn default() -> DemodCfg9 {
        DemodCfg9(0)
    }
}
impl core::fmt::Debug for DemodCfg9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DemodCfg9")
            .field("br_demod_phase_0", &self.br_demod_phase_0())
            .field("br_demod_phase_1", &self.br_demod_phase_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DemodCfg9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DemodCfg9 {{ br_demod_phase_0: {=u16:?}, br_demod_phase_1: {=u16:?} }}",
            self.br_demod_phase_0(),
            self.br_demod_phase_1()
        )
    }
}
#[doc = "EDRDEMOD_CFG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EdrdemodCfg1(pub u32);
impl EdrdemodCfg1 {
    #[must_use]
    #[inline(always)]
    pub const fn edr2_mu_err(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_edr2_mu_err(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn edr2_mu_dc(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_edr2_mu_dc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
    }
}
impl Default for EdrdemodCfg1 {
    #[inline(always)]
    fn default() -> EdrdemodCfg1 {
        EdrdemodCfg1(0)
    }
}
impl core::fmt::Debug for EdrdemodCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EdrdemodCfg1")
            .field("edr2_mu_err", &self.edr2_mu_err())
            .field("edr2_mu_dc", &self.edr2_mu_dc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EdrdemodCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EdrdemodCfg1 {{ edr2_mu_err: {=u16:?}, edr2_mu_dc: {=u16:?} }}",
            self.edr2_mu_err(),
            self.edr2_mu_dc()
        )
    }
}
#[doc = "EDRDEMOD_CFG2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EdrdemodCfg2(pub u32);
impl EdrdemodCfg2 {
    #[must_use]
    #[inline(always)]
    pub const fn edr3_mu_err(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_edr3_mu_err(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn edr3_mu_dc(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_edr3_mu_dc(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
    }
}
impl Default for EdrdemodCfg2 {
    #[inline(always)]
    fn default() -> EdrdemodCfg2 {
        EdrdemodCfg2(0)
    }
}
impl core::fmt::Debug for EdrdemodCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EdrdemodCfg2")
            .field("edr3_mu_err", &self.edr3_mu_err())
            .field("edr3_mu_dc", &self.edr3_mu_dc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EdrdemodCfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EdrdemodCfg2 {{ edr3_mu_err: {=u16:?}, edr3_mu_dc: {=u16:?} }}",
            self.edr3_mu_err(),
            self.edr3_mu_dc()
        )
    }
}
#[doc = "EDRSYNC_CFG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EdrsyncCfg1(pub u32);
impl EdrsyncCfg1 {
    #[must_use]
    #[inline(always)]
    pub const fn edrsync_cnt_thd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_edrsync_cnt_thd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn edrsync_phasecorr_thd(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x0003_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_edrsync_phasecorr_thd(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 8usize)) | (((val as u32) & 0x0003_ffff) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn edrsync_method(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_edrsync_method(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for EdrsyncCfg1 {
    #[inline(always)]
    fn default() -> EdrsyncCfg1 {
        EdrsyncCfg1(0)
    }
}
impl core::fmt::Debug for EdrsyncCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EdrsyncCfg1")
            .field("edrsync_cnt_thd", &self.edrsync_cnt_thd())
            .field("edrsync_phasecorr_thd", &self.edrsync_phasecorr_thd())
            .field("edrsync_method", &self.edrsync_method())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EdrsyncCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "EdrsyncCfg1 {{ edrsync_cnt_thd: {=u8:?}, edrsync_phasecorr_thd: {=u32:?}, edrsync_method: {=bool:?} }}" , self . edrsync_cnt_thd () , self . edrsync_phasecorr_thd () , self . edrsync_method ())
    }
}
#[doc = "EDRSYNC_CFG2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EdrsyncCfg2(pub u32);
impl EdrsyncCfg2 {
    #[must_use]
    #[inline(always)]
    pub const fn edrsync_phaseunwrap_thd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_edrsync_phaseunwrap_thd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for EdrsyncCfg2 {
    #[inline(always)]
    fn default() -> EdrsyncCfg2 {
        EdrsyncCfg2(0)
    }
}
impl core::fmt::Debug for EdrsyncCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EdrsyncCfg2")
            .field("edrsync_phaseunwrap_thd", &self.edrsync_phaseunwrap_thd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EdrsyncCfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "EdrsyncCfg2 {{ edrsync_phaseunwrap_thd: {=u8:?} }}",
            self.edrsync_phaseunwrap_thd()
        )
    }
}
#[doc = "EDRTED_CFG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EdrtedCfg1(pub u32);
impl EdrtedCfg1 {
    #[must_use]
    #[inline(always)]
    pub const fn ted_edr2_mu_p(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ted_edr2_mu_p(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ted_edr2_mu_f(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ted_edr2_mu_f(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ted_edr3_mu_p(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ted_edr3_mu_p(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ted_edr3_mu_f(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ted_edr3_mu_f(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for EdrtedCfg1 {
    #[inline(always)]
    fn default() -> EdrtedCfg1 {
        EdrtedCfg1(0)
    }
}
impl core::fmt::Debug for EdrtedCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EdrtedCfg1")
            .field("ted_edr2_mu_p", &self.ted_edr2_mu_p())
            .field("ted_edr2_mu_f", &self.ted_edr2_mu_f())
            .field("ted_edr3_mu_p", &self.ted_edr3_mu_p())
            .field("ted_edr3_mu_f", &self.ted_edr3_mu_f())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EdrtedCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "EdrtedCfg1 {{ ted_edr2_mu_p: {=u8:?}, ted_edr2_mu_f: {=u8:?}, ted_edr3_mu_p: {=u8:?}, ted_edr3_mu_f: {=u8:?} }}" , self . ted_edr2_mu_p () , self . ted_edr2_mu_f () , self . ted_edr3_mu_p () , self . ted_edr3_mu_f ())
    }
}
#[doc = "INTERP_CFG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InterpCfg1(pub u32);
impl InterpCfg1 {
    #[must_use]
    #[inline(always)]
    pub const fn timing_factor(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_timing_factor(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn interp_method_u(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_interp_method_u(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn interp_en_u(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_interp_en_u(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn interp_method_b(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_interp_method_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn interp_en_b(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_interp_en_b(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for InterpCfg1 {
    #[inline(always)]
    fn default() -> InterpCfg1 {
        InterpCfg1(0)
    }
}
impl core::fmt::Debug for InterpCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("InterpCfg1")
            .field("timing_factor", &self.timing_factor())
            .field("interp_method_u", &self.interp_method_u())
            .field("interp_en_u", &self.interp_en_u())
            .field("interp_method_b", &self.interp_method_b())
            .field("interp_en_b", &self.interp_en_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for InterpCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "InterpCfg1 {{ timing_factor: {=u8:?}, interp_method_u: {=bool:?}, interp_en_u: {=bool:?}, interp_method_b: {=bool:?}, interp_en_b: {=bool:?} }}" , self . timing_factor () , self . interp_method_u () , self . interp_en_u () , self . interp_method_b () , self . interp_en_b ())
    }
}
#[doc = "LFP_MMDIV_CFG0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LfpMmdivCfg0(pub u32);
impl LfpMmdivCfg0 {
    #[must_use]
    #[inline(always)]
    pub const fn rx_mmdiv_offset_1m(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0001_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_rx_mmdiv_offset_1m(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
    }
}
impl Default for LfpMmdivCfg0 {
    #[inline(always)]
    fn default() -> LfpMmdivCfg0 {
        LfpMmdivCfg0(0)
    }
}
impl core::fmt::Debug for LfpMmdivCfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LfpMmdivCfg0")
            .field("rx_mmdiv_offset_1m", &self.rx_mmdiv_offset_1m())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LfpMmdivCfg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LfpMmdivCfg0 {{ rx_mmdiv_offset_1m: {=u32:?} }}",
            self.rx_mmdiv_offset_1m()
        )
    }
}
#[doc = "LFP_MMDIV_CFG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LfpMmdivCfg1(pub u32);
impl LfpMmdivCfg1 {
    #[must_use]
    #[inline(always)]
    pub const fn rx_mmdiv_offset_2m(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0001_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_rx_mmdiv_offset_2m(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
    }
}
impl Default for LfpMmdivCfg1 {
    #[inline(always)]
    fn default() -> LfpMmdivCfg1 {
        LfpMmdivCfg1(0)
    }
}
impl core::fmt::Debug for LfpMmdivCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LfpMmdivCfg1")
            .field("rx_mmdiv_offset_2m", &self.rx_mmdiv_offset_2m())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LfpMmdivCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LfpMmdivCfg1 {{ rx_mmdiv_offset_2m: {=u32:?} }}",
            self.rx_mmdiv_offset_2m()
        )
    }
}
#[doc = "LFP_MMDIV_CFG2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LfpMmdivCfg2(pub u32);
impl LfpMmdivCfg2 {
    #[must_use]
    #[inline(always)]
    pub const fn rx_mmdiv_offset_bt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0001_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_rx_mmdiv_offset_bt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
    }
}
impl Default for LfpMmdivCfg2 {
    #[inline(always)]
    fn default() -> LfpMmdivCfg2 {
        LfpMmdivCfg2(0)
    }
}
impl core::fmt::Debug for LfpMmdivCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LfpMmdivCfg2")
            .field("rx_mmdiv_offset_bt", &self.rx_mmdiv_offset_bt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LfpMmdivCfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LfpMmdivCfg2 {{ rx_mmdiv_offset_bt: {=u32:?} }}",
            self.rx_mmdiv_offset_bt()
        )
    }
}
#[doc = "LFP_MMDIV_CFG3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LfpMmdivCfg3(pub u32);
impl LfpMmdivCfg3 {
    #[must_use]
    #[inline(always)]
    pub const fn tx_mmdiv_offset(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0001_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_tx_mmdiv_offset(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
    }
}
impl Default for LfpMmdivCfg3 {
    #[inline(always)]
    fn default() -> LfpMmdivCfg3 {
        LfpMmdivCfg3(0)
    }
}
impl core::fmt::Debug for LfpMmdivCfg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LfpMmdivCfg3")
            .field("tx_mmdiv_offset", &self.tx_mmdiv_offset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LfpMmdivCfg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LfpMmdivCfg3 {{ tx_mmdiv_offset: {=u32:?} }}",
            self.tx_mmdiv_offset()
        )
    }
}
#[doc = "LFP_MMDIV_CFG4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LfpMmdivCfg4(pub u32);
impl LfpMmdivCfg4 {
    #[must_use]
    #[inline(always)]
    pub const fn rf_mmdiv_test(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x01ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_rf_mmdiv_test(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rf_test_mode(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rf_test_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for LfpMmdivCfg4 {
    #[inline(always)]
    fn default() -> LfpMmdivCfg4 {
        LfpMmdivCfg4(0)
    }
}
impl core::fmt::Debug for LfpMmdivCfg4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LfpMmdivCfg4")
            .field("rf_mmdiv_test", &self.rf_mmdiv_test())
            .field("rf_test_mode", &self.rf_test_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LfpMmdivCfg4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LfpMmdivCfg4 {{ rf_mmdiv_test: {=u32:?}, rf_test_mode: {=bool:?} }}",
            self.rf_mmdiv_test(),
            self.rf_test_mode()
        )
    }
}
#[doc = "LNA_GAIN_TBL0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LnaGainTbl0(pub u32);
impl LnaGainTbl0 {
    #[must_use]
    #[inline(always)]
    pub const fn lna_gain_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_lna_gain_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lna_gain_1(&self) -> u16 {
        let val = (self.0 >> 11usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_lna_gain_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 11usize)) | (((val as u32) & 0x07ff) << 11usize);
    }
}
impl Default for LnaGainTbl0 {
    #[inline(always)]
    fn default() -> LnaGainTbl0 {
        LnaGainTbl0(0)
    }
}
impl core::fmt::Debug for LnaGainTbl0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LnaGainTbl0")
            .field("lna_gain_0", &self.lna_gain_0())
            .field("lna_gain_1", &self.lna_gain_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LnaGainTbl0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LnaGainTbl0 {{ lna_gain_0: {=u16:?}, lna_gain_1: {=u16:?} }}",
            self.lna_gain_0(),
            self.lna_gain_1()
        )
    }
}
#[doc = "LNA_GAIN_TBL1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LnaGainTbl1(pub u32);
impl LnaGainTbl1 {
    #[must_use]
    #[inline(always)]
    pub const fn lna_gain_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_lna_gain_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lna_gain_3(&self) -> u16 {
        let val = (self.0 >> 11usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_lna_gain_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 11usize)) | (((val as u32) & 0x07ff) << 11usize);
    }
}
impl Default for LnaGainTbl1 {
    #[inline(always)]
    fn default() -> LnaGainTbl1 {
        LnaGainTbl1(0)
    }
}
impl core::fmt::Debug for LnaGainTbl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LnaGainTbl1")
            .field("lna_gain_2", &self.lna_gain_2())
            .field("lna_gain_3", &self.lna_gain_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LnaGainTbl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LnaGainTbl1 {{ lna_gain_2: {=u16:?}, lna_gain_3: {=u16:?} }}",
            self.lna_gain_2(),
            self.lna_gain_3()
        )
    }
}
#[doc = "LNA_GAIN_TBL2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LnaGainTbl2(pub u32);
impl LnaGainTbl2 {
    #[must_use]
    #[inline(always)]
    pub const fn lna_gain_4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_lna_gain_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lna_gain_5(&self) -> u16 {
        let val = (self.0 >> 11usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_lna_gain_5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 11usize)) | (((val as u32) & 0x07ff) << 11usize);
    }
}
impl Default for LnaGainTbl2 {
    #[inline(always)]
    fn default() -> LnaGainTbl2 {
        LnaGainTbl2(0)
    }
}
impl core::fmt::Debug for LnaGainTbl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LnaGainTbl2")
            .field("lna_gain_4", &self.lna_gain_4())
            .field("lna_gain_5", &self.lna_gain_5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LnaGainTbl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LnaGainTbl2 {{ lna_gain_4: {=u16:?}, lna_gain_5: {=u16:?} }}",
            self.lna_gain_4(),
            self.lna_gain_5()
        )
    }
}
#[doc = "LNA_GAIN_TBL3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LnaGainTbl3(pub u32);
impl LnaGainTbl3 {
    #[must_use]
    #[inline(always)]
    pub const fn lna_gain_6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_lna_gain_6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lna_gain_7(&self) -> u16 {
        let val = (self.0 >> 11usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_lna_gain_7(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 11usize)) | (((val as u32) & 0x07ff) << 11usize);
    }
}
impl Default for LnaGainTbl3 {
    #[inline(always)]
    fn default() -> LnaGainTbl3 {
        LnaGainTbl3(0)
    }
}
impl core::fmt::Debug for LnaGainTbl3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LnaGainTbl3")
            .field("lna_gain_6", &self.lna_gain_6())
            .field("lna_gain_7", &self.lna_gain_7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LnaGainTbl3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LnaGainTbl3 {{ lna_gain_6: {=u16:?}, lna_gain_7: {=u16:?} }}",
            self.lna_gain_6(),
            self.lna_gain_7()
        )
    }
}
#[doc = "LNA_GAIN_TBL4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LnaGainTbl4(pub u32);
impl LnaGainTbl4 {
    #[must_use]
    #[inline(always)]
    pub const fn lna_gain_8(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_lna_gain_8(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lna_gain_9(&self) -> u16 {
        let val = (self.0 >> 11usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_lna_gain_9(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 11usize)) | (((val as u32) & 0x07ff) << 11usize);
    }
}
impl Default for LnaGainTbl4 {
    #[inline(always)]
    fn default() -> LnaGainTbl4 {
        LnaGainTbl4(0)
    }
}
impl core::fmt::Debug for LnaGainTbl4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LnaGainTbl4")
            .field("lna_gain_8", &self.lna_gain_8())
            .field("lna_gain_9", &self.lna_gain_9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LnaGainTbl4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LnaGainTbl4 {{ lna_gain_8: {=u16:?}, lna_gain_9: {=u16:?} }}",
            self.lna_gain_8(),
            self.lna_gain_9()
        )
    }
}
#[doc = "LNA_GAIN_TBL5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LnaGainTbl5(pub u32);
impl LnaGainTbl5 {
    #[must_use]
    #[inline(always)]
    pub const fn lna_gain_10(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_lna_gain_10(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lna_gain_11(&self) -> u16 {
        let val = (self.0 >> 11usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_lna_gain_11(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 11usize)) | (((val as u32) & 0x07ff) << 11usize);
    }
}
impl Default for LnaGainTbl5 {
    #[inline(always)]
    fn default() -> LnaGainTbl5 {
        LnaGainTbl5(0)
    }
}
impl core::fmt::Debug for LnaGainTbl5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LnaGainTbl5")
            .field("lna_gain_10", &self.lna_gain_10())
            .field("lna_gain_11", &self.lna_gain_11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LnaGainTbl5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LnaGainTbl5 {{ lna_gain_10: {=u16:?}, lna_gain_11: {=u16:?} }}",
            self.lna_gain_10(),
            self.lna_gain_11()
        )
    }
}
#[doc = "LPF_CFG0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpfCfg0(pub u32);
impl LpfCfg0 {
    #[must_use]
    #[inline(always)]
    pub const fn lpf_coef_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_lpf_coef_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lpf_coef_1(&self) -> u16 {
        let val = (self.0 >> 9usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_lpf_coef_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 9usize)) | (((val as u32) & 0x01ff) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lpf_coef_2(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_lpf_coef_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 18usize)) | (((val as u32) & 0x01ff) << 18usize);
    }
}
impl Default for LpfCfg0 {
    #[inline(always)]
    fn default() -> LpfCfg0 {
        LpfCfg0(0)
    }
}
impl core::fmt::Debug for LpfCfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpfCfg0")
            .field("lpf_coef_0", &self.lpf_coef_0())
            .field("lpf_coef_1", &self.lpf_coef_1())
            .field("lpf_coef_2", &self.lpf_coef_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpfCfg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LpfCfg0 {{ lpf_coef_0: {=u16:?}, lpf_coef_1: {=u16:?}, lpf_coef_2: {=u16:?} }}",
            self.lpf_coef_0(),
            self.lpf_coef_1(),
            self.lpf_coef_2()
        )
    }
}
#[doc = "LPF_CFG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpfCfg1(pub u32);
impl LpfCfg1 {
    #[must_use]
    #[inline(always)]
    pub const fn lpf_coef_3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_lpf_coef_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lpf_coef_4(&self) -> u16 {
        let val = (self.0 >> 9usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_lpf_coef_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 9usize)) | (((val as u32) & 0x01ff) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lpf_coef_5(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_lpf_coef_5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 18usize)) | (((val as u32) & 0x01ff) << 18usize);
    }
}
impl Default for LpfCfg1 {
    #[inline(always)]
    fn default() -> LpfCfg1 {
        LpfCfg1(0)
    }
}
impl core::fmt::Debug for LpfCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpfCfg1")
            .field("lpf_coef_3", &self.lpf_coef_3())
            .field("lpf_coef_4", &self.lpf_coef_4())
            .field("lpf_coef_5", &self.lpf_coef_5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpfCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LpfCfg1 {{ lpf_coef_3: {=u16:?}, lpf_coef_4: {=u16:?}, lpf_coef_5: {=u16:?} }}",
            self.lpf_coef_3(),
            self.lpf_coef_4(),
            self.lpf_coef_5()
        )
    }
}
#[doc = "LPF_CFG2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpfCfg2(pub u32);
impl LpfCfg2 {
    #[must_use]
    #[inline(always)]
    pub const fn lpf_coef_6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_lpf_coef_6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lpf_coef_7(&self) -> u16 {
        let val = (self.0 >> 9usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_lpf_coef_7(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 9usize)) | (((val as u32) & 0x01ff) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lpf_coef_8(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_lpf_coef_8(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 18usize)) | (((val as u32) & 0x01ff) << 18usize);
    }
}
impl Default for LpfCfg2 {
    #[inline(always)]
    fn default() -> LpfCfg2 {
        LpfCfg2(0)
    }
}
impl core::fmt::Debug for LpfCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpfCfg2")
            .field("lpf_coef_6", &self.lpf_coef_6())
            .field("lpf_coef_7", &self.lpf_coef_7())
            .field("lpf_coef_8", &self.lpf_coef_8())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpfCfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LpfCfg2 {{ lpf_coef_6: {=u16:?}, lpf_coef_7: {=u16:?}, lpf_coef_8: {=u16:?} }}",
            self.lpf_coef_6(),
            self.lpf_coef_7(),
            self.lpf_coef_8()
        )
    }
}
#[doc = "LPF_CFG3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpfCfg3(pub u32);
impl LpfCfg3 {
    #[must_use]
    #[inline(always)]
    pub const fn lpf_coef_9(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_lpf_coef_9(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for LpfCfg3 {
    #[inline(always)]
    fn default() -> LpfCfg3 {
        LpfCfg3(0)
    }
}
impl core::fmt::Debug for LpfCfg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpfCfg3")
            .field("lpf_coef_9", &self.lpf_coef_9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpfCfg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LpfCfg3 {{ lpf_coef_9: {=u16:?} }}", self.lpf_coef_9())
    }
}
#[doc = "MIXER_CFG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MixerCfg1(pub u32);
impl MixerCfg1 {
    #[must_use]
    #[inline(always)]
    pub const fn rx_mixer_phase_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rx_mixer_phase_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rx_mixer_phase_2(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rx_mixer_phase_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rx_mixer_phase_bt(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rx_mixer_phase_bt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
    }
}
impl Default for MixerCfg1 {
    #[inline(always)]
    fn default() -> MixerCfg1 {
        MixerCfg1(0)
    }
}
impl core::fmt::Debug for MixerCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MixerCfg1")
            .field("rx_mixer_phase_1", &self.rx_mixer_phase_1())
            .field("rx_mixer_phase_2", &self.rx_mixer_phase_2())
            .field("rx_mixer_phase_bt", &self.rx_mixer_phase_bt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MixerCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "MixerCfg1 {{ rx_mixer_phase_1: {=u16:?}, rx_mixer_phase_2: {=u16:?}, rx_mixer_phase_bt: {=u16:?} }}" , self . rx_mixer_phase_1 () , self . rx_mixer_phase_2 () , self . rx_mixer_phase_bt ())
    }
}
#[doc = "NOTCH_CFG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NotchCfg1(pub u32);
impl NotchCfg1 {
    #[must_use]
    #[inline(always)]
    pub const fn notch_b0_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_notch_b0_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn notch_b1_1(&self) -> u16 {
        let val = (self.0 >> 14usize) & 0x3fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_notch_b1_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 14usize)) | (((val as u32) & 0x3fff) << 14usize);
    }
}
impl Default for NotchCfg1 {
    #[inline(always)]
    fn default() -> NotchCfg1 {
        NotchCfg1(0)
    }
}
impl core::fmt::Debug for NotchCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NotchCfg1")
            .field("notch_b0_1", &self.notch_b0_1())
            .field("notch_b1_1", &self.notch_b1_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NotchCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NotchCfg1 {{ notch_b0_1: {=u16:?}, notch_b1_1: {=u16:?} }}",
            self.notch_b0_1(),
            self.notch_b1_1()
        )
    }
}
#[doc = "NOTCH_CFG10"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NotchCfg10(pub u32);
impl NotchCfg10 {
    #[must_use]
    #[inline(always)]
    pub const fn chnl_notch_en1_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_chnl_notch_en1_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for NotchCfg10 {
    #[inline(always)]
    fn default() -> NotchCfg10 {
        NotchCfg10(0)
    }
}
impl core::fmt::Debug for NotchCfg10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NotchCfg10")
            .field("chnl_notch_en1_2", &self.chnl_notch_en1_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NotchCfg10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NotchCfg10 {{ chnl_notch_en1_2: {=u32:?} }}",
            self.chnl_notch_en1_2()
        )
    }
}
#[doc = "NOTCH_CFG11"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NotchCfg11(pub u32);
impl NotchCfg11 {
    #[must_use]
    #[inline(always)]
    pub const fn chnl_notch_en2_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_chnl_notch_en2_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn notch_rssi_thd_2(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_notch_rssi_thd_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 15usize)) | (((val as u32) & 0xff) << 15usize);
    }
}
impl Default for NotchCfg11 {
    #[inline(always)]
    fn default() -> NotchCfg11 {
        NotchCfg11(0)
    }
}
impl core::fmt::Debug for NotchCfg11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NotchCfg11")
            .field("chnl_notch_en2_2", &self.chnl_notch_en2_2())
            .field("notch_rssi_thd_2", &self.notch_rssi_thd_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NotchCfg11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NotchCfg11 {{ chnl_notch_en2_2: {=u16:?}, notch_rssi_thd_2: {=u8:?} }}",
            self.chnl_notch_en2_2(),
            self.notch_rssi_thd_2()
        )
    }
}
#[doc = "NOTCH_CFG12"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NotchCfg12(pub u32);
impl NotchCfg12 {
    #[must_use]
    #[inline(always)]
    pub const fn chnl_notch_en0_b(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_chnl_notch_en0_b(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for NotchCfg12 {
    #[inline(always)]
    fn default() -> NotchCfg12 {
        NotchCfg12(0)
    }
}
impl core::fmt::Debug for NotchCfg12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NotchCfg12")
            .field("chnl_notch_en0_b", &self.chnl_notch_en0_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NotchCfg12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NotchCfg12 {{ chnl_notch_en0_b: {=u32:?} }}",
            self.chnl_notch_en0_b()
        )
    }
}
#[doc = "NOTCH_CFG13"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NotchCfg13(pub u32);
impl NotchCfg13 {
    #[must_use]
    #[inline(always)]
    pub const fn chnl_notch_en1_b(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_chnl_notch_en1_b(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for NotchCfg13 {
    #[inline(always)]
    fn default() -> NotchCfg13 {
        NotchCfg13(0)
    }
}
impl core::fmt::Debug for NotchCfg13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NotchCfg13")
            .field("chnl_notch_en1_b", &self.chnl_notch_en1_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NotchCfg13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NotchCfg13 {{ chnl_notch_en1_b: {=u32:?} }}",
            self.chnl_notch_en1_b()
        )
    }
}
#[doc = "NOTCH_CFG14"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NotchCfg14(pub u32);
impl NotchCfg14 {
    #[must_use]
    #[inline(always)]
    pub const fn chnl_notch_en2_b(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_chnl_notch_en2_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn notch_rssi_thd_b(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_notch_rssi_thd_b(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 15usize)) | (((val as u32) & 0xff) << 15usize);
    }
}
impl Default for NotchCfg14 {
    #[inline(always)]
    fn default() -> NotchCfg14 {
        NotchCfg14(0)
    }
}
impl core::fmt::Debug for NotchCfg14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NotchCfg14")
            .field("chnl_notch_en2_b", &self.chnl_notch_en2_b())
            .field("notch_rssi_thd_b", &self.notch_rssi_thd_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NotchCfg14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NotchCfg14 {{ chnl_notch_en2_b: {=u16:?}, notch_rssi_thd_b: {=u8:?} }}",
            self.chnl_notch_en2_b(),
            self.notch_rssi_thd_b()
        )
    }
}
#[doc = "NOTCH_CFG2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NotchCfg2(pub u32);
impl NotchCfg2 {
    #[must_use]
    #[inline(always)]
    pub const fn notch_a2_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_notch_a2_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn notch_a2_2(&self) -> u16 {
        let val = (self.0 >> 14usize) & 0x3fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_notch_a2_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 14usize)) | (((val as u32) & 0x3fff) << 14usize);
    }
}
impl Default for NotchCfg2 {
    #[inline(always)]
    fn default() -> NotchCfg2 {
        NotchCfg2(0)
    }
}
impl core::fmt::Debug for NotchCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NotchCfg2")
            .field("notch_a2_1", &self.notch_a2_1())
            .field("notch_a2_2", &self.notch_a2_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NotchCfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NotchCfg2 {{ notch_a2_1: {=u16:?}, notch_a2_2: {=u16:?} }}",
            self.notch_a2_1(),
            self.notch_a2_2()
        )
    }
}
#[doc = "NOTCH_CFG3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NotchCfg3(pub u32);
impl NotchCfg3 {
    #[must_use]
    #[inline(always)]
    pub const fn notch_b0_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_notch_b0_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn notch_b1_2(&self) -> u16 {
        let val = (self.0 >> 14usize) & 0x3fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_notch_b1_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 14usize)) | (((val as u32) & 0x3fff) << 14usize);
    }
}
impl Default for NotchCfg3 {
    #[inline(always)]
    fn default() -> NotchCfg3 {
        NotchCfg3(0)
    }
}
impl core::fmt::Debug for NotchCfg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NotchCfg3")
            .field("notch_b0_2", &self.notch_b0_2())
            .field("notch_b1_2", &self.notch_b1_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NotchCfg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NotchCfg3 {{ notch_b0_2: {=u16:?}, notch_b1_2: {=u16:?} }}",
            self.notch_b0_2(),
            self.notch_b1_2()
        )
    }
}
#[doc = "NOTCH_CFG4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NotchCfg4(pub u32);
impl NotchCfg4 {
    #[must_use]
    #[inline(always)]
    pub const fn notch_b0_b(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_notch_b0_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn notch_b1_b(&self) -> u16 {
        let val = (self.0 >> 14usize) & 0x3fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_notch_b1_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 14usize)) | (((val as u32) & 0x3fff) << 14usize);
    }
}
impl Default for NotchCfg4 {
    #[inline(always)]
    fn default() -> NotchCfg4 {
        NotchCfg4(0)
    }
}
impl core::fmt::Debug for NotchCfg4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NotchCfg4")
            .field("notch_b0_b", &self.notch_b0_b())
            .field("notch_b1_b", &self.notch_b1_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NotchCfg4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NotchCfg4 {{ notch_b0_b: {=u16:?}, notch_b1_b: {=u16:?} }}",
            self.notch_b0_b(),
            self.notch_b1_b()
        )
    }
}
#[doc = "NOTCH_CFG5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NotchCfg5(pub u32);
impl NotchCfg5 {
    #[must_use]
    #[inline(always)]
    pub const fn notch_a2_b(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_notch_a2_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
}
impl Default for NotchCfg5 {
    #[inline(always)]
    fn default() -> NotchCfg5 {
        NotchCfg5(0)
    }
}
impl core::fmt::Debug for NotchCfg5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NotchCfg5")
            .field("notch_a2_b", &self.notch_a2_b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NotchCfg5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "NotchCfg5 {{ notch_a2_b: {=u16:?} }}", self.notch_a2_b())
    }
}
#[doc = "NOTCH_CFG6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NotchCfg6(pub u32);
impl NotchCfg6 {
    #[must_use]
    #[inline(always)]
    pub const fn chnl_notch_en0_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_chnl_notch_en0_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for NotchCfg6 {
    #[inline(always)]
    fn default() -> NotchCfg6 {
        NotchCfg6(0)
    }
}
impl core::fmt::Debug for NotchCfg6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NotchCfg6")
            .field("chnl_notch_en0_1", &self.chnl_notch_en0_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NotchCfg6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NotchCfg6 {{ chnl_notch_en0_1: {=u32:?} }}",
            self.chnl_notch_en0_1()
        )
    }
}
#[doc = "NOTCH_CFG7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NotchCfg7(pub u32);
impl NotchCfg7 {
    #[must_use]
    #[inline(always)]
    pub const fn chnl_notch_en1_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_chnl_notch_en1_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for NotchCfg7 {
    #[inline(always)]
    fn default() -> NotchCfg7 {
        NotchCfg7(0)
    }
}
impl core::fmt::Debug for NotchCfg7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NotchCfg7")
            .field("chnl_notch_en1_1", &self.chnl_notch_en1_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NotchCfg7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NotchCfg7 {{ chnl_notch_en1_1: {=u32:?} }}",
            self.chnl_notch_en1_1()
        )
    }
}
#[doc = "NOTCH_CFG8"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NotchCfg8(pub u32);
impl NotchCfg8 {
    #[must_use]
    #[inline(always)]
    pub const fn chnl_notch_en2_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_chnl_notch_en2_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn notch_rssi_thd_1(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_notch_rssi_thd_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 15usize)) | (((val as u32) & 0xff) << 15usize);
    }
}
impl Default for NotchCfg8 {
    #[inline(always)]
    fn default() -> NotchCfg8 {
        NotchCfg8(0)
    }
}
impl core::fmt::Debug for NotchCfg8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NotchCfg8")
            .field("chnl_notch_en2_1", &self.chnl_notch_en2_1())
            .field("notch_rssi_thd_1", &self.notch_rssi_thd_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NotchCfg8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NotchCfg8 {{ chnl_notch_en2_1: {=u16:?}, notch_rssi_thd_1: {=u8:?} }}",
            self.chnl_notch_en2_1(),
            self.notch_rssi_thd_1()
        )
    }
}
#[doc = "NOTCH_CFG9"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NotchCfg9(pub u32);
impl NotchCfg9 {
    #[must_use]
    #[inline(always)]
    pub const fn chnl_notch_en0_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_chnl_notch_en0_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for NotchCfg9 {
    #[inline(always)]
    fn default() -> NotchCfg9 {
        NotchCfg9(0)
    }
}
impl core::fmt::Debug for NotchCfg9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NotchCfg9")
            .field("chnl_notch_en0_2", &self.chnl_notch_en0_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NotchCfg9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "NotchCfg9 {{ chnl_notch_en0_2: {=u32:?} }}",
            self.chnl_notch_en0_2()
        )
    }
}
#[doc = "PKTDET_CFG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PktdetCfg1(pub u32);
impl PktdetCfg1 {
    #[must_use]
    #[inline(always)]
    pub const fn ble_pktdet_thd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ble_pktdet_thd(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ble_pkt_cnt_thd(&self) -> u16 {
        let val = (self.0 >> 14usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_ble_pkt_cnt_thd(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 14usize)) | (((val as u32) & 0x01ff) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ble_hard_corr_thd(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ble_hard_corr_thd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 23usize)) | (((val as u32) & 0x1f) << 23usize);
    }
}
impl Default for PktdetCfg1 {
    #[inline(always)]
    fn default() -> PktdetCfg1 {
        PktdetCfg1(0)
    }
}
impl core::fmt::Debug for PktdetCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PktdetCfg1")
            .field("ble_pktdet_thd", &self.ble_pktdet_thd())
            .field("ble_pkt_cnt_thd", &self.ble_pkt_cnt_thd())
            .field("ble_hard_corr_thd", &self.ble_hard_corr_thd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PktdetCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "PktdetCfg1 {{ ble_pktdet_thd: {=u16:?}, ble_pkt_cnt_thd: {=u16:?}, ble_hard_corr_thd: {=u8:?} }}" , self . ble_pktdet_thd () , self . ble_pkt_cnt_thd () , self . ble_hard_corr_thd ())
    }
}
#[doc = "PKTDET_CFG2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PktdetCfg2(pub u32);
impl PktdetCfg2 {
    #[must_use]
    #[inline(always)]
    pub const fn br_pktdet_thd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_br_pktdet_thd(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn br_pkt_cnt_thd(&self) -> u16 {
        let val = (self.0 >> 14usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_br_pkt_cnt_thd(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 14usize)) | (((val as u32) & 0x01ff) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn br_hard_corr_thd(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_br_hard_corr_thd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 23usize)) | (((val as u32) & 0x1f) << 23usize);
    }
}
impl Default for PktdetCfg2 {
    #[inline(always)]
    fn default() -> PktdetCfg2 {
        PktdetCfg2(0)
    }
}
impl core::fmt::Debug for PktdetCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PktdetCfg2")
            .field("br_pktdet_thd", &self.br_pktdet_thd())
            .field("br_pkt_cnt_thd", &self.br_pkt_cnt_thd())
            .field("br_hard_corr_thd", &self.br_hard_corr_thd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PktdetCfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "PktdetCfg2 {{ br_pktdet_thd: {=u16:?}, br_pkt_cnt_thd: {=u16:?}, br_hard_corr_thd: {=u8:?} }}" , self . br_pktdet_thd () , self . br_pkt_cnt_thd () , self . br_hard_corr_thd ())
    }
}
#[doc = "RCOS_CFG0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RcosCfg0(pub u32);
impl RcosCfg0 {
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_1(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for RcosCfg0 {
    #[inline(always)]
    fn default() -> RcosCfg0 {
        RcosCfg0(0)
    }
}
impl core::fmt::Debug for RcosCfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RcosCfg0")
            .field("rcos_coef_0", &self.rcos_coef_0())
            .field("rcos_coef_1", &self.rcos_coef_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RcosCfg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RcosCfg0 {{ rcos_coef_0: {=u16:?}, rcos_coef_1: {=u16:?} }}",
            self.rcos_coef_0(),
            self.rcos_coef_1()
        )
    }
}
#[doc = "RCOS_CFG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RcosCfg1(pub u32);
impl RcosCfg1 {
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_3(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for RcosCfg1 {
    #[inline(always)]
    fn default() -> RcosCfg1 {
        RcosCfg1(0)
    }
}
impl core::fmt::Debug for RcosCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RcosCfg1")
            .field("rcos_coef_2", &self.rcos_coef_2())
            .field("rcos_coef_3", &self.rcos_coef_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RcosCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RcosCfg1 {{ rcos_coef_2: {=u16:?}, rcos_coef_3: {=u16:?} }}",
            self.rcos_coef_2(),
            self.rcos_coef_3()
        )
    }
}
#[doc = "RCOS_CFG10"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RcosCfg10(pub u32);
impl RcosCfg10 {
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_20(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_20(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_21(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_21(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for RcosCfg10 {
    #[inline(always)]
    fn default() -> RcosCfg10 {
        RcosCfg10(0)
    }
}
impl core::fmt::Debug for RcosCfg10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RcosCfg10")
            .field("rcos_coef_20", &self.rcos_coef_20())
            .field("rcos_coef_21", &self.rcos_coef_21())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RcosCfg10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RcosCfg10 {{ rcos_coef_20: {=u16:?}, rcos_coef_21: {=u16:?} }}",
            self.rcos_coef_20(),
            self.rcos_coef_21()
        )
    }
}
#[doc = "RCOS_CFG11"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RcosCfg11(pub u32);
impl RcosCfg11 {
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_22(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_22(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_23(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_23(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for RcosCfg11 {
    #[inline(always)]
    fn default() -> RcosCfg11 {
        RcosCfg11(0)
    }
}
impl core::fmt::Debug for RcosCfg11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RcosCfg11")
            .field("rcos_coef_22", &self.rcos_coef_22())
            .field("rcos_coef_23", &self.rcos_coef_23())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RcosCfg11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RcosCfg11 {{ rcos_coef_22: {=u16:?}, rcos_coef_23: {=u16:?} }}",
            self.rcos_coef_22(),
            self.rcos_coef_23()
        )
    }
}
#[doc = "RCOS_CFG12"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RcosCfg12(pub u32);
impl RcosCfg12 {
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_24(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_24(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_25(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_25(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for RcosCfg12 {
    #[inline(always)]
    fn default() -> RcosCfg12 {
        RcosCfg12(0)
    }
}
impl core::fmt::Debug for RcosCfg12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RcosCfg12")
            .field("rcos_coef_24", &self.rcos_coef_24())
            .field("rcos_coef_25", &self.rcos_coef_25())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RcosCfg12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RcosCfg12 {{ rcos_coef_24: {=u16:?}, rcos_coef_25: {=u16:?} }}",
            self.rcos_coef_24(),
            self.rcos_coef_25()
        )
    }
}
#[doc = "RCOS_CFG13"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RcosCfg13(pub u32);
impl RcosCfg13 {
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_26(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_26(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_27(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_27(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for RcosCfg13 {
    #[inline(always)]
    fn default() -> RcosCfg13 {
        RcosCfg13(0)
    }
}
impl core::fmt::Debug for RcosCfg13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RcosCfg13")
            .field("rcos_coef_26", &self.rcos_coef_26())
            .field("rcos_coef_27", &self.rcos_coef_27())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RcosCfg13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RcosCfg13 {{ rcos_coef_26: {=u16:?}, rcos_coef_27: {=u16:?} }}",
            self.rcos_coef_26(),
            self.rcos_coef_27()
        )
    }
}
#[doc = "RCOS_CFG14"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RcosCfg14(pub u32);
impl RcosCfg14 {
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_28(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_28(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_29(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_29(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for RcosCfg14 {
    #[inline(always)]
    fn default() -> RcosCfg14 {
        RcosCfg14(0)
    }
}
impl core::fmt::Debug for RcosCfg14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RcosCfg14")
            .field("rcos_coef_28", &self.rcos_coef_28())
            .field("rcos_coef_29", &self.rcos_coef_29())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RcosCfg14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RcosCfg14 {{ rcos_coef_28: {=u16:?}, rcos_coef_29: {=u16:?} }}",
            self.rcos_coef_28(),
            self.rcos_coef_29()
        )
    }
}
#[doc = "RCOS_CFG15"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RcosCfg15(pub u32);
impl RcosCfg15 {
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_30(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_30(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_31(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_31(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for RcosCfg15 {
    #[inline(always)]
    fn default() -> RcosCfg15 {
        RcosCfg15(0)
    }
}
impl core::fmt::Debug for RcosCfg15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RcosCfg15")
            .field("rcos_coef_30", &self.rcos_coef_30())
            .field("rcos_coef_31", &self.rcos_coef_31())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RcosCfg15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RcosCfg15 {{ rcos_coef_30: {=u16:?}, rcos_coef_31: {=u16:?} }}",
            self.rcos_coef_30(),
            self.rcos_coef_31()
        )
    }
}
#[doc = "RCOS_CFG16"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RcosCfg16(pub u32);
impl RcosCfg16 {
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_32(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_32(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_33(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_33(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for RcosCfg16 {
    #[inline(always)]
    fn default() -> RcosCfg16 {
        RcosCfg16(0)
    }
}
impl core::fmt::Debug for RcosCfg16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RcosCfg16")
            .field("rcos_coef_32", &self.rcos_coef_32())
            .field("rcos_coef_33", &self.rcos_coef_33())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RcosCfg16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RcosCfg16 {{ rcos_coef_32: {=u16:?}, rcos_coef_33: {=u16:?} }}",
            self.rcos_coef_32(),
            self.rcos_coef_33()
        )
    }
}
#[doc = "RCOS_CFG17"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RcosCfg17(pub u32);
impl RcosCfg17 {
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_34(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_34(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_35(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_35(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for RcosCfg17 {
    #[inline(always)]
    fn default() -> RcosCfg17 {
        RcosCfg17(0)
    }
}
impl core::fmt::Debug for RcosCfg17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RcosCfg17")
            .field("rcos_coef_34", &self.rcos_coef_34())
            .field("rcos_coef_35", &self.rcos_coef_35())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RcosCfg17 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RcosCfg17 {{ rcos_coef_34: {=u16:?}, rcos_coef_35: {=u16:?} }}",
            self.rcos_coef_34(),
            self.rcos_coef_35()
        )
    }
}
#[doc = "RCOS_CFG18"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RcosCfg18(pub u32);
impl RcosCfg18 {
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_36(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_36(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for RcosCfg18 {
    #[inline(always)]
    fn default() -> RcosCfg18 {
        RcosCfg18(0)
    }
}
impl core::fmt::Debug for RcosCfg18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RcosCfg18")
            .field("rcos_coef_36", &self.rcos_coef_36())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RcosCfg18 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RcosCfg18 {{ rcos_coef_36: {=u16:?} }}",
            self.rcos_coef_36()
        )
    }
}
#[doc = "RCOS_CFG2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RcosCfg2(pub u32);
impl RcosCfg2 {
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_5(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_5(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for RcosCfg2 {
    #[inline(always)]
    fn default() -> RcosCfg2 {
        RcosCfg2(0)
    }
}
impl core::fmt::Debug for RcosCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RcosCfg2")
            .field("rcos_coef_4", &self.rcos_coef_4())
            .field("rcos_coef_5", &self.rcos_coef_5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RcosCfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RcosCfg2 {{ rcos_coef_4: {=u16:?}, rcos_coef_5: {=u16:?} }}",
            self.rcos_coef_4(),
            self.rcos_coef_5()
        )
    }
}
#[doc = "RCOS_CFG3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RcosCfg3(pub u32);
impl RcosCfg3 {
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_6(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_7(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_7(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for RcosCfg3 {
    #[inline(always)]
    fn default() -> RcosCfg3 {
        RcosCfg3(0)
    }
}
impl core::fmt::Debug for RcosCfg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RcosCfg3")
            .field("rcos_coef_6", &self.rcos_coef_6())
            .field("rcos_coef_7", &self.rcos_coef_7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RcosCfg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RcosCfg3 {{ rcos_coef_6: {=u16:?}, rcos_coef_7: {=u16:?} }}",
            self.rcos_coef_6(),
            self.rcos_coef_7()
        )
    }
}
#[doc = "RCOS_CFG4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RcosCfg4(pub u32);
impl RcosCfg4 {
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_8(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_8(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_9(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_9(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for RcosCfg4 {
    #[inline(always)]
    fn default() -> RcosCfg4 {
        RcosCfg4(0)
    }
}
impl core::fmt::Debug for RcosCfg4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RcosCfg4")
            .field("rcos_coef_8", &self.rcos_coef_8())
            .field("rcos_coef_9", &self.rcos_coef_9())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RcosCfg4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RcosCfg4 {{ rcos_coef_8: {=u16:?}, rcos_coef_9: {=u16:?} }}",
            self.rcos_coef_8(),
            self.rcos_coef_9()
        )
    }
}
#[doc = "RCOS_CFG5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RcosCfg5(pub u32);
impl RcosCfg5 {
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_10(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_10(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_11(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_11(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for RcosCfg5 {
    #[inline(always)]
    fn default() -> RcosCfg5 {
        RcosCfg5(0)
    }
}
impl core::fmt::Debug for RcosCfg5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RcosCfg5")
            .field("rcos_coef_10", &self.rcos_coef_10())
            .field("rcos_coef_11", &self.rcos_coef_11())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RcosCfg5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RcosCfg5 {{ rcos_coef_10: {=u16:?}, rcos_coef_11: {=u16:?} }}",
            self.rcos_coef_10(),
            self.rcos_coef_11()
        )
    }
}
#[doc = "RCOS_CFG6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RcosCfg6(pub u32);
impl RcosCfg6 {
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_12(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_12(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_13(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_13(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for RcosCfg6 {
    #[inline(always)]
    fn default() -> RcosCfg6 {
        RcosCfg6(0)
    }
}
impl core::fmt::Debug for RcosCfg6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RcosCfg6")
            .field("rcos_coef_12", &self.rcos_coef_12())
            .field("rcos_coef_13", &self.rcos_coef_13())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RcosCfg6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RcosCfg6 {{ rcos_coef_12: {=u16:?}, rcos_coef_13: {=u16:?} }}",
            self.rcos_coef_12(),
            self.rcos_coef_13()
        )
    }
}
#[doc = "RCOS_CFG7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RcosCfg7(pub u32);
impl RcosCfg7 {
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_14(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_14(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_15(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_15(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for RcosCfg7 {
    #[inline(always)]
    fn default() -> RcosCfg7 {
        RcosCfg7(0)
    }
}
impl core::fmt::Debug for RcosCfg7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RcosCfg7")
            .field("rcos_coef_14", &self.rcos_coef_14())
            .field("rcos_coef_15", &self.rcos_coef_15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RcosCfg7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RcosCfg7 {{ rcos_coef_14: {=u16:?}, rcos_coef_15: {=u16:?} }}",
            self.rcos_coef_14(),
            self.rcos_coef_15()
        )
    }
}
#[doc = "RCOS_CFG8"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RcosCfg8(pub u32);
impl RcosCfg8 {
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_16(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_17(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_17(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for RcosCfg8 {
    #[inline(always)]
    fn default() -> RcosCfg8 {
        RcosCfg8(0)
    }
}
impl core::fmt::Debug for RcosCfg8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RcosCfg8")
            .field("rcos_coef_16", &self.rcos_coef_16())
            .field("rcos_coef_17", &self.rcos_coef_17())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RcosCfg8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RcosCfg8 {{ rcos_coef_16: {=u16:?}, rcos_coef_17: {=u16:?} }}",
            self.rcos_coef_16(),
            self.rcos_coef_17()
        )
    }
}
#[doc = "RCOS_CFG9"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RcosCfg9(pub u32);
impl RcosCfg9 {
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_18(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_18(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rcos_coef_19(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_rcos_coef_19(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for RcosCfg9 {
    #[inline(always)]
    fn default() -> RcosCfg9 {
        RcosCfg9(0)
    }
}
impl core::fmt::Debug for RcosCfg9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RcosCfg9")
            .field("rcos_coef_18", &self.rcos_coef_18())
            .field("rcos_coef_19", &self.rcos_coef_19())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RcosCfg9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RcosCfg9 {{ rcos_coef_18: {=u16:?}, rcos_coef_19: {=u16:?} }}",
            self.rcos_coef_18(),
            self.rcos_coef_19()
        )
    }
}
#[doc = "RSSI_CFG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RssiCfg1(pub u32);
impl RssiCfg1 {
    #[must_use]
    #[inline(always)]
    pub const fn dig_gain_low_db(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_dig_gain_low_db(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dig_gain_high_db(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_dig_gain_high_db(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rssi_mu(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rssi_mu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rssi_offset(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rssi_offset(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 17usize)) | (((val as u32) & 0x3f) << 17usize);
    }
}
impl Default for RssiCfg1 {
    #[inline(always)]
    fn default() -> RssiCfg1 {
        RssiCfg1(0)
    }
}
impl core::fmt::Debug for RssiCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RssiCfg1")
            .field("dig_gain_low_db", &self.dig_gain_low_db())
            .field("dig_gain_high_db", &self.dig_gain_high_db())
            .field("rssi_mu", &self.rssi_mu())
            .field("rssi_offset", &self.rssi_offset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RssiCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "RssiCfg1 {{ dig_gain_low_db: {=u8:?}, dig_gain_high_db: {=u8:?}, rssi_mu: {=u8:?}, rssi_offset: {=u8:?} }}" , self . dig_gain_low_db () , self . dig_gain_high_db () , self . rssi_mu () , self . rssi_offset ())
    }
}
#[doc = "RX_CTRL1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxCtrl1(pub u32);
impl RxCtrl1 {
    #[must_use]
    #[inline(always)]
    pub const fn adc_sign(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_adc_sign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mixer_iq_swap_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mixer_iq_swap_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_iq_swap_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_adc_iq_swap_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn force_rx_on(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_force_rx_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_q_en_1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_adc_q_en_1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lpf1_sample_phase_sel(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_lpf1_sample_phase_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rssi_sample_sel(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rssi_sample_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn phy_rx_dump_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_phy_rx_dump_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rx_dump_clk_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rx_dump_clk_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rx_dump_data_sel(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rx_dump_data_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rx_dbg_trig_sel(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rx_dbg_trig_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 15usize)) | (((val as u32) & 0x03) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rx_dbg_data_sel(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rx_dbg_data_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 17usize)) | (((val as u32) & 0x1f) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rx_loopback_mode(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rx_loopback_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rx_dump_q_sel(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rx_dump_q_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn frc_adc_24m(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_frc_adc_24m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn edr_notch_off_en(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_edr_notch_off_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn bt_op_mode(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_bt_op_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for RxCtrl1 {
    #[inline(always)]
    fn default() -> RxCtrl1 {
        RxCtrl1(0)
    }
}
impl core::fmt::Debug for RxCtrl1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxCtrl1")
            .field("adc_sign", &self.adc_sign())
            .field("mixer_iq_swap_en", &self.mixer_iq_swap_en())
            .field("adc_iq_swap_en", &self.adc_iq_swap_en())
            .field("force_rx_on", &self.force_rx_on())
            .field("adc_q_en_1", &self.adc_q_en_1())
            .field("lpf1_sample_phase_sel", &self.lpf1_sample_phase_sel())
            .field("rssi_sample_sel", &self.rssi_sample_sel())
            .field("phy_rx_dump_en", &self.phy_rx_dump_en())
            .field("rx_dump_clk_sel", &self.rx_dump_clk_sel())
            .field("rx_dump_data_sel", &self.rx_dump_data_sel())
            .field("rx_dbg_trig_sel", &self.rx_dbg_trig_sel())
            .field("rx_dbg_data_sel", &self.rx_dbg_data_sel())
            .field("rx_loopback_mode", &self.rx_loopback_mode())
            .field("rx_dump_q_sel", &self.rx_dump_q_sel())
            .field("frc_adc_24m", &self.frc_adc_24m())
            .field("edr_notch_off_en", &self.edr_notch_off_en())
            .field("bt_op_mode", &self.bt_op_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxCtrl1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "RxCtrl1 {{ adc_sign: {=bool:?}, mixer_iq_swap_en: {=bool:?}, adc_iq_swap_en: {=bool:?}, force_rx_on: {=bool:?}, adc_q_en_1: {=bool:?}, lpf1_sample_phase_sel: {=bool:?}, rssi_sample_sel: {=bool:?}, phy_rx_dump_en: {=bool:?}, rx_dump_clk_sel: {=u8:?}, rx_dump_data_sel: {=u8:?}, rx_dbg_trig_sel: {=u8:?}, rx_dbg_data_sel: {=u8:?}, rx_loopback_mode: {=bool:?}, rx_dump_q_sel: {=bool:?}, frc_adc_24m: {=bool:?}, edr_notch_off_en: {=bool:?}, bt_op_mode: {=bool:?} }}" , self . adc_sign () , self . mixer_iq_swap_en () , self . adc_iq_swap_en () , self . force_rx_on () , self . adc_q_en_1 () , self . lpf1_sample_phase_sel () , self . rssi_sample_sel () , self . phy_rx_dump_en () , self . rx_dump_clk_sel () , self . rx_dump_data_sel () , self . rx_dbg_trig_sel () , self . rx_dbg_data_sel () , self . rx_loopback_mode () , self . rx_dump_q_sel () , self . frc_adc_24m () , self . edr_notch_off_en () , self . bt_op_mode ())
    }
}
#[doc = "RX_CTRL2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxCtrl2(pub u32);
impl RxCtrl2 {
    #[must_use]
    #[inline(always)]
    pub const fn force_clk_on_agc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_force_clk_on_agc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn force_rx_reset(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_force_rx_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cbpf_edr_switch_t0(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_cbpf_edr_switch_t0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u32) & 0x1f) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cbpf_edr_switch_t1(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_cbpf_edr_switch_t1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 7usize)) | (((val as u32) & 0x1f) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_edr_switch_t0(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_adc_edr_switch_t0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_edr_switch_t1(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_adc_edr_switch_t1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 17usize)) | (((val as u32) & 0x1f) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_q_en_2(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_adc_q_en_2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_q_en_c(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_adc_q_en_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_q_en_br(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_adc_q_en_br(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_q_en_edr(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_adc_q_en_edr(&mut self, val: bool) {
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
}
impl Default for RxCtrl2 {
    #[inline(always)]
    fn default() -> RxCtrl2 {
        RxCtrl2(0)
    }
}
impl core::fmt::Debug for RxCtrl2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxCtrl2")
            .field("force_clk_on_agc", &self.force_clk_on_agc())
            .field("force_rx_reset", &self.force_rx_reset())
            .field("cbpf_edr_switch_t0", &self.cbpf_edr_switch_t0())
            .field("cbpf_edr_switch_t1", &self.cbpf_edr_switch_t1())
            .field("adc_edr_switch_t0", &self.adc_edr_switch_t0())
            .field("adc_edr_switch_t1", &self.adc_edr_switch_t1())
            .field("adc_q_en_2", &self.adc_q_en_2())
            .field("adc_q_en_c", &self.adc_q_en_c())
            .field("adc_q_en_br", &self.adc_q_en_br())
            .field("adc_q_en_edr", &self.adc_q_en_edr())
            .field("adc_q_en_frc_en", &self.adc_q_en_frc_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxCtrl2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "RxCtrl2 {{ force_clk_on_agc: {=bool:?}, force_rx_reset: {=bool:?}, cbpf_edr_switch_t0: {=u8:?}, cbpf_edr_switch_t1: {=u8:?}, adc_edr_switch_t0: {=u8:?}, adc_edr_switch_t1: {=u8:?}, adc_q_en_2: {=bool:?}, adc_q_en_c: {=bool:?}, adc_q_en_br: {=bool:?}, adc_q_en_edr: {=bool:?}, adc_q_en_frc_en: {=bool:?} }}" , self . force_clk_on_agc () , self . force_rx_reset () , self . cbpf_edr_switch_t0 () , self . cbpf_edr_switch_t1 () , self . adc_edr_switch_t0 () , self . adc_edr_switch_t1 () , self . adc_q_en_2 () , self . adc_q_en_c () , self . adc_q_en_br () , self . adc_q_en_edr () , self . adc_q_en_frc_en ())
    }
}
#[doc = "RX_HFP_CFG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxHfpCfg(pub u32);
impl RxHfpCfg {
    #[must_use]
    #[inline(always)]
    pub const fn rx_hfp_fcw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rx_hfp_fcw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for RxHfpCfg {
    #[inline(always)]
    fn default() -> RxHfpCfg {
        RxHfpCfg(0)
    }
}
impl core::fmt::Debug for RxHfpCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxHfpCfg")
            .field("rx_hfp_fcw", &self.rx_hfp_fcw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxHfpCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RxHfpCfg {{ rx_hfp_fcw: {=u8:?} }}", self.rx_hfp_fcw())
    }
}
#[doc = "RX_STATUS1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxStatus1(pub u32);
impl RxStatus1 {
    #[must_use]
    #[inline(always)]
    pub const fn cfo_phase(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_cfo_phase(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn edr_carrier_phase(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_edr_carrier_phase(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val as u32) & 0x0fff) << 12usize);
    }
}
impl Default for RxStatus1 {
    #[inline(always)]
    fn default() -> RxStatus1 {
        RxStatus1(0)
    }
}
impl core::fmt::Debug for RxStatus1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxStatus1")
            .field("cfo_phase", &self.cfo_phase())
            .field("edr_carrier_phase", &self.edr_carrier_phase())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxStatus1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RxStatus1 {{ cfo_phase: {=u16:?}, edr_carrier_phase: {=u16:?} }}",
            self.cfo_phase(),
            self.edr_carrier_phase()
        )
    }
}
#[doc = "TED_CFG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TedCfg1(pub u32);
impl TedCfg1 {
    #[must_use]
    #[inline(always)]
    pub const fn ted_mu_p_u(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ted_mu_p_u(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ted_mu_f_u(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ted_mu_f_u(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ted_mu_p_br(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ted_mu_p_br(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ted_mu_f_br(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ted_mu_f_br(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for TedCfg1 {
    #[inline(always)]
    fn default() -> TedCfg1 {
        TedCfg1(0)
    }
}
impl core::fmt::Debug for TedCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TedCfg1")
            .field("ted_mu_p_u", &self.ted_mu_p_u())
            .field("ted_mu_f_u", &self.ted_mu_f_u())
            .field("ted_mu_p_br", &self.ted_mu_p_br())
            .field("ted_mu_f_br", &self.ted_mu_f_br())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TedCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TedCfg1 {{ ted_mu_p_u: {=u8:?}, ted_mu_f_u: {=u8:?}, ted_mu_p_br: {=u8:?}, ted_mu_f_br: {=u8:?} }}" , self . ted_mu_p_u () , self . ted_mu_f_u () , self . ted_mu_p_br () , self . ted_mu_f_br ())
    }
}
#[doc = "TX_CTRL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxCtrl(pub u32);
impl TxCtrl {
    #[must_use]
    #[inline(always)]
    pub const fn force_tx_on(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_force_tx_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_loopback_mode(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_tx_loopback_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mod_method_ble(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mod_method_ble(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mod_method_br(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mod_method_br(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mod_method_edr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mod_method_edr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mmdiv_sel(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mmdiv_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dac_sign(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_dac_sign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn guard_time_sel(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_guard_time_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn gfsk_rd_en(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_gfsk_rd_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dpsk_rd_en(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_dpsk_rd_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mac_mod_ctrl_en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_mac_mod_ctrl_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_dc_cal_iq_swap(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_tx_dc_cal_iq_swap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rcos_win_en(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rcos_win_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for TxCtrl {
    #[inline(always)]
    fn default() -> TxCtrl {
        TxCtrl(0)
    }
}
impl core::fmt::Debug for TxCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxCtrl")
            .field("force_tx_on", &self.force_tx_on())
            .field("tx_loopback_mode", &self.tx_loopback_mode())
            .field("mod_method_ble", &self.mod_method_ble())
            .field("mod_method_br", &self.mod_method_br())
            .field("mod_method_edr", &self.mod_method_edr())
            .field("mmdiv_sel", &self.mmdiv_sel())
            .field("dac_sign", &self.dac_sign())
            .field("guard_time_sel", &self.guard_time_sel())
            .field("gfsk_rd_en", &self.gfsk_rd_en())
            .field("dpsk_rd_en", &self.dpsk_rd_en())
            .field("mac_mod_ctrl_en", &self.mac_mod_ctrl_en())
            .field("tx_dc_cal_iq_swap", &self.tx_dc_cal_iq_swap())
            .field("rcos_win_en", &self.rcos_win_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TxCtrl {{ force_tx_on: {=bool:?}, tx_loopback_mode: {=bool:?}, mod_method_ble: {=bool:?}, mod_method_br: {=bool:?}, mod_method_edr: {=bool:?}, mmdiv_sel: {=bool:?}, dac_sign: {=bool:?}, guard_time_sel: {=u8:?}, gfsk_rd_en: {=bool:?}, dpsk_rd_en: {=bool:?}, mac_mod_ctrl_en: {=bool:?}, tx_dc_cal_iq_swap: {=bool:?}, rcos_win_en: {=bool:?} }}" , self . force_tx_on () , self . tx_loopback_mode () , self . mod_method_ble () , self . mod_method_br () , self . mod_method_edr () , self . mmdiv_sel () , self . dac_sign () , self . guard_time_sel () , self . gfsk_rd_en () , self . dpsk_rd_en () , self . mac_mod_ctrl_en () , self . tx_dc_cal_iq_swap () , self . rcos_win_en ())
    }
}
#[doc = "TX_DC_CAL_CFG0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxDcCalCfg0(pub u32);
impl TxDcCalCfg0 {
    #[must_use]
    #[inline(always)]
    pub const fn tx_dc_cal_phase0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_tx_dc_cal_phase0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_dc_cal_phase1(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_tx_dc_cal_phase1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_dc_cal_en(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_tx_dc_cal_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for TxDcCalCfg0 {
    #[inline(always)]
    fn default() -> TxDcCalCfg0 {
        TxDcCalCfg0(0)
    }
}
impl core::fmt::Debug for TxDcCalCfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxDcCalCfg0")
            .field("tx_dc_cal_phase0", &self.tx_dc_cal_phase0())
            .field("tx_dc_cal_phase1", &self.tx_dc_cal_phase1())
            .field("tx_dc_cal_en", &self.tx_dc_cal_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxDcCalCfg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TxDcCalCfg0 {{ tx_dc_cal_phase0: {=u16:?}, tx_dc_cal_phase1: {=u16:?}, tx_dc_cal_en: {=bool:?} }}" , self . tx_dc_cal_phase0 () , self . tx_dc_cal_phase1 () , self . tx_dc_cal_en ())
    }
}
#[doc = "TX_DC_CAL_CFG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxDcCalCfg1(pub u32);
impl TxDcCalCfg1 {
    #[must_use]
    #[inline(always)]
    pub const fn tx_dc_cal_phase_init0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_tx_dc_cal_phase_init0(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_dc_cal_phase_init1(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_tx_dc_cal_phase_init1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
    }
}
impl Default for TxDcCalCfg1 {
    #[inline(always)]
    fn default() -> TxDcCalCfg1 {
        TxDcCalCfg1(0)
    }
}
impl core::fmt::Debug for TxDcCalCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxDcCalCfg1")
            .field("tx_dc_cal_phase_init0", &self.tx_dc_cal_phase_init0())
            .field("tx_dc_cal_phase_init1", &self.tx_dc_cal_phase_init1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxDcCalCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TxDcCalCfg1 {{ tx_dc_cal_phase_init0: {=u16:?}, tx_dc_cal_phase_init1: {=u16:?} }}",
            self.tx_dc_cal_phase_init0(),
            self.tx_dc_cal_phase_init1()
        )
    }
}
#[doc = "TX_DC_CAL_CFG2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxDcCalCfg2(pub u32);
impl TxDcCalCfg2 {
    #[must_use]
    #[inline(always)]
    pub const fn tx_dc_cal_gain0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_dc_cal_gain0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_dc_cal_gain1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_dc_cal_gain1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for TxDcCalCfg2 {
    #[inline(always)]
    fn default() -> TxDcCalCfg2 {
        TxDcCalCfg2(0)
    }
}
impl core::fmt::Debug for TxDcCalCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxDcCalCfg2")
            .field("tx_dc_cal_gain0", &self.tx_dc_cal_gain0())
            .field("tx_dc_cal_gain1", &self.tx_dc_cal_gain1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxDcCalCfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TxDcCalCfg2 {{ tx_dc_cal_gain0: {=u8:?}, tx_dc_cal_gain1: {=u8:?} }}",
            self.tx_dc_cal_gain0(),
            self.tx_dc_cal_gain1()
        )
    }
}
#[doc = "TX_DPSK_CFG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxDpskCfg1(pub u32);
impl TxDpskCfg1 {
    #[must_use]
    #[inline(always)]
    pub const fn tx_dpsk_gain_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_dpsk_gain_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_dpsk_gain_1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_dpsk_gain_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_dpsk_gain_2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_dpsk_gain_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_dpsk_gain_3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_dpsk_gain_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for TxDpskCfg1 {
    #[inline(always)]
    fn default() -> TxDpskCfg1 {
        TxDpskCfg1(0)
    }
}
impl core::fmt::Debug for TxDpskCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxDpskCfg1")
            .field("tx_dpsk_gain_0", &self.tx_dpsk_gain_0())
            .field("tx_dpsk_gain_1", &self.tx_dpsk_gain_1())
            .field("tx_dpsk_gain_2", &self.tx_dpsk_gain_2())
            .field("tx_dpsk_gain_3", &self.tx_dpsk_gain_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxDpskCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TxDpskCfg1 {{ tx_dpsk_gain_0: {=u8:?}, tx_dpsk_gain_1: {=u8:?}, tx_dpsk_gain_2: {=u8:?}, tx_dpsk_gain_3: {=u8:?} }}" , self . tx_dpsk_gain_0 () , self . tx_dpsk_gain_1 () , self . tx_dpsk_gain_2 () , self . tx_dpsk_gain_3 ())
    }
}
#[doc = "TX_DPSK_CFG2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxDpskCfg2(pub u32);
impl TxDpskCfg2 {
    #[must_use]
    #[inline(always)]
    pub const fn tx_dpsk_gain_4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_dpsk_gain_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_dpsk_gain_5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_dpsk_gain_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_dpsk_gain_6(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_dpsk_gain_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_dpsk_gain_7(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_dpsk_gain_7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for TxDpskCfg2 {
    #[inline(always)]
    fn default() -> TxDpskCfg2 {
        TxDpskCfg2(0)
    }
}
impl core::fmt::Debug for TxDpskCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxDpskCfg2")
            .field("tx_dpsk_gain_4", &self.tx_dpsk_gain_4())
            .field("tx_dpsk_gain_5", &self.tx_dpsk_gain_5())
            .field("tx_dpsk_gain_6", &self.tx_dpsk_gain_6())
            .field("tx_dpsk_gain_7", &self.tx_dpsk_gain_7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxDpskCfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TxDpskCfg2 {{ tx_dpsk_gain_4: {=u8:?}, tx_dpsk_gain_5: {=u8:?}, tx_dpsk_gain_6: {=u8:?}, tx_dpsk_gain_7: {=u8:?} }}" , self . tx_dpsk_gain_4 () , self . tx_dpsk_gain_5 () , self . tx_dpsk_gain_6 () , self . tx_dpsk_gain_7 ())
    }
}
#[doc = "TX_GAUSSFLT_CFG1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxGaussfltCfg1(pub u32);
impl TxGaussfltCfg1 {
    #[must_use]
    #[inline(always)]
    pub const fn polar_gauss_gain_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_polar_gauss_gain_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn polar_gauss_gain_2(&self) -> u16 {
        let val = (self.0 >> 9usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_polar_gauss_gain_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 9usize)) | (((val as u32) & 0x01ff) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn polar_gauss_gain_br(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_polar_gauss_gain_br(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 18usize)) | (((val as u32) & 0x01ff) << 18usize);
    }
}
impl Default for TxGaussfltCfg1 {
    #[inline(always)]
    fn default() -> TxGaussfltCfg1 {
        TxGaussfltCfg1(0)
    }
}
impl core::fmt::Debug for TxGaussfltCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxGaussfltCfg1")
            .field("polar_gauss_gain_1", &self.polar_gauss_gain_1())
            .field("polar_gauss_gain_2", &self.polar_gauss_gain_2())
            .field("polar_gauss_gain_br", &self.polar_gauss_gain_br())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxGaussfltCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TxGaussfltCfg1 {{ polar_gauss_gain_1: {=u16:?}, polar_gauss_gain_2: {=u16:?}, polar_gauss_gain_br: {=u16:?} }}" , self . polar_gauss_gain_1 () , self . polar_gauss_gain_2 () , self . polar_gauss_gain_br ())
    }
}
#[doc = "TX_GAUSSFLT_CFG2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxGaussfltCfg2(pub u32);
impl TxGaussfltCfg2 {
    #[must_use]
    #[inline(always)]
    pub const fn iq_gauss_gain_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_iq_gauss_gain_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn iq_gauss_gain_2(&self) -> u16 {
        let val = (self.0 >> 9usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_iq_gauss_gain_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 9usize)) | (((val as u32) & 0x01ff) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn iq_gauss_gain_br(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_iq_gauss_gain_br(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 18usize)) | (((val as u32) & 0x01ff) << 18usize);
    }
}
impl Default for TxGaussfltCfg2 {
    #[inline(always)]
    fn default() -> TxGaussfltCfg2 {
        TxGaussfltCfg2(0)
    }
}
impl core::fmt::Debug for TxGaussfltCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxGaussfltCfg2")
            .field("iq_gauss_gain_1", &self.iq_gauss_gain_1())
            .field("iq_gauss_gain_2", &self.iq_gauss_gain_2())
            .field("iq_gauss_gain_br", &self.iq_gauss_gain_br())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxGaussfltCfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TxGaussfltCfg2 {{ iq_gauss_gain_1: {=u16:?}, iq_gauss_gain_2: {=u16:?}, iq_gauss_gain_br: {=u16:?} }}" , self . iq_gauss_gain_1 () , self . iq_gauss_gain_2 () , self . iq_gauss_gain_br ())
    }
}
#[doc = "TX_HFP_CFG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxHfpCfg(pub u32);
impl TxHfpCfg {
    #[must_use]
    #[inline(always)]
    pub const fn tx_kcal_coef(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_tx_kcal_coef(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_kcal(&self) -> u16 {
        let val = (self.0 >> 9usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_tx_kcal(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 9usize)) | (((val as u32) & 0x0fff) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn hfp_fcw_sel(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_hfp_fcw_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn hfp_fcw(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_hfp_fcw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 22usize)) | (((val as u32) & 0x3f) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn hfp_delay_sel(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_hfp_delay_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
}
impl Default for TxHfpCfg {
    #[inline(always)]
    fn default() -> TxHfpCfg {
        TxHfpCfg(0)
    }
}
impl core::fmt::Debug for TxHfpCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxHfpCfg")
            .field("tx_kcal_coef", &self.tx_kcal_coef())
            .field("tx_kcal", &self.tx_kcal())
            .field("hfp_fcw_sel", &self.hfp_fcw_sel())
            .field("hfp_fcw", &self.hfp_fcw())
            .field("hfp_delay_sel", &self.hfp_delay_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxHfpCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TxHfpCfg {{ tx_kcal_coef: {=u16:?}, tx_kcal: {=u16:?}, hfp_fcw_sel: {=bool:?}, hfp_fcw: {=u8:?}, hfp_delay_sel: {=u8:?} }}" , self . tx_kcal_coef () , self . tx_kcal () , self . hfp_fcw_sel () , self . hfp_fcw () , self . hfp_delay_sel ())
    }
}
#[doc = "TX_IF_MOD_CFG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxIfModCfg(pub u32);
impl TxIfModCfg {
    #[must_use]
    #[inline(always)]
    pub const fn tx_if_phase_ble(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_tx_if_phase_ble(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_if_phase_br(&self) -> u16 {
        let val = (self.0 >> 10usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_tx_if_phase_br(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_ramp_bypass(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_tx_ramp_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for TxIfModCfg {
    #[inline(always)]
    fn default() -> TxIfModCfg {
        TxIfModCfg(0)
    }
}
impl core::fmt::Debug for TxIfModCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxIfModCfg")
            .field("tx_if_phase_ble", &self.tx_if_phase_ble())
            .field("tx_if_phase_br", &self.tx_if_phase_br())
            .field("tx_ramp_bypass", &self.tx_ramp_bypass())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxIfModCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TxIfModCfg {{ tx_if_phase_ble: {=u16:?}, tx_if_phase_br: {=u16:?}, tx_ramp_bypass: {=bool:?} }}" , self . tx_if_phase_ble () , self . tx_if_phase_br () , self . tx_ramp_bypass ())
    }
}
#[doc = "TX_IF_MOD_CFG2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxIfModCfg2(pub u32);
impl TxIfModCfg2 {
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_ble_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_ble_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_ble_1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_ble_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_ble_2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_ble_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_ble_3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_ble_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for TxIfModCfg2 {
    #[inline(always)]
    fn default() -> TxIfModCfg2 {
        TxIfModCfg2(0)
    }
}
impl core::fmt::Debug for TxIfModCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxIfModCfg2")
            .field("tx_mod_gain_ble_0", &self.tx_mod_gain_ble_0())
            .field("tx_mod_gain_ble_1", &self.tx_mod_gain_ble_1())
            .field("tx_mod_gain_ble_2", &self.tx_mod_gain_ble_2())
            .field("tx_mod_gain_ble_3", &self.tx_mod_gain_ble_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxIfModCfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TxIfModCfg2 {{ tx_mod_gain_ble_0: {=u8:?}, tx_mod_gain_ble_1: {=u8:?}, tx_mod_gain_ble_2: {=u8:?}, tx_mod_gain_ble_3: {=u8:?} }}" , self . tx_mod_gain_ble_0 () , self . tx_mod_gain_ble_1 () , self . tx_mod_gain_ble_2 () , self . tx_mod_gain_ble_3 ())
    }
}
#[doc = "TX_IF_MOD_CFG3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxIfModCfg3(pub u32);
impl TxIfModCfg3 {
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_ble_4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_ble_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_ble_5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_ble_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_ble_6(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_ble_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_ble_7(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_ble_7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for TxIfModCfg3 {
    #[inline(always)]
    fn default() -> TxIfModCfg3 {
        TxIfModCfg3(0)
    }
}
impl core::fmt::Debug for TxIfModCfg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxIfModCfg3")
            .field("tx_mod_gain_ble_4", &self.tx_mod_gain_ble_4())
            .field("tx_mod_gain_ble_5", &self.tx_mod_gain_ble_5())
            .field("tx_mod_gain_ble_6", &self.tx_mod_gain_ble_6())
            .field("tx_mod_gain_ble_7", &self.tx_mod_gain_ble_7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxIfModCfg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TxIfModCfg3 {{ tx_mod_gain_ble_4: {=u8:?}, tx_mod_gain_ble_5: {=u8:?}, tx_mod_gain_ble_6: {=u8:?}, tx_mod_gain_ble_7: {=u8:?} }}" , self . tx_mod_gain_ble_4 () , self . tx_mod_gain_ble_5 () , self . tx_mod_gain_ble_6 () , self . tx_mod_gain_ble_7 ())
    }
}
#[doc = "TX_IF_MOD_CFG4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxIfModCfg4(pub u32);
impl TxIfModCfg4 {
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_br_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_br_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_br_1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_br_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_br_2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_br_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_br_3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_br_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for TxIfModCfg4 {
    #[inline(always)]
    fn default() -> TxIfModCfg4 {
        TxIfModCfg4(0)
    }
}
impl core::fmt::Debug for TxIfModCfg4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxIfModCfg4")
            .field("tx_mod_gain_br_0", &self.tx_mod_gain_br_0())
            .field("tx_mod_gain_br_1", &self.tx_mod_gain_br_1())
            .field("tx_mod_gain_br_2", &self.tx_mod_gain_br_2())
            .field("tx_mod_gain_br_3", &self.tx_mod_gain_br_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxIfModCfg4 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TxIfModCfg4 {{ tx_mod_gain_br_0: {=u8:?}, tx_mod_gain_br_1: {=u8:?}, tx_mod_gain_br_2: {=u8:?}, tx_mod_gain_br_3: {=u8:?} }}" , self . tx_mod_gain_br_0 () , self . tx_mod_gain_br_1 () , self . tx_mod_gain_br_2 () , self . tx_mod_gain_br_3 ())
    }
}
#[doc = "TX_IF_MOD_CFG5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxIfModCfg5(pub u32);
impl TxIfModCfg5 {
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_br_4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_br_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_br_5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_br_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_br_6(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_br_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_br_7(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_br_7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for TxIfModCfg5 {
    #[inline(always)]
    fn default() -> TxIfModCfg5 {
        TxIfModCfg5(0)
    }
}
impl core::fmt::Debug for TxIfModCfg5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxIfModCfg5")
            .field("tx_mod_gain_br_4", &self.tx_mod_gain_br_4())
            .field("tx_mod_gain_br_5", &self.tx_mod_gain_br_5())
            .field("tx_mod_gain_br_6", &self.tx_mod_gain_br_6())
            .field("tx_mod_gain_br_7", &self.tx_mod_gain_br_7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxIfModCfg5 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TxIfModCfg5 {{ tx_mod_gain_br_4: {=u8:?}, tx_mod_gain_br_5: {=u8:?}, tx_mod_gain_br_6: {=u8:?}, tx_mod_gain_br_7: {=u8:?} }}" , self . tx_mod_gain_br_4 () , self . tx_mod_gain_br_5 () , self . tx_mod_gain_br_6 () , self . tx_mod_gain_br_7 ())
    }
}
#[doc = "TX_IF_MOD_CFG6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxIfModCfg6(pub u32);
impl TxIfModCfg6 {
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_edr_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_edr_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_edr_1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_edr_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_edr_2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_edr_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_edr_3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_edr_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for TxIfModCfg6 {
    #[inline(always)]
    fn default() -> TxIfModCfg6 {
        TxIfModCfg6(0)
    }
}
impl core::fmt::Debug for TxIfModCfg6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxIfModCfg6")
            .field("tx_mod_gain_edr_0", &self.tx_mod_gain_edr_0())
            .field("tx_mod_gain_edr_1", &self.tx_mod_gain_edr_1())
            .field("tx_mod_gain_edr_2", &self.tx_mod_gain_edr_2())
            .field("tx_mod_gain_edr_3", &self.tx_mod_gain_edr_3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxIfModCfg6 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TxIfModCfg6 {{ tx_mod_gain_edr_0: {=u8:?}, tx_mod_gain_edr_1: {=u8:?}, tx_mod_gain_edr_2: {=u8:?}, tx_mod_gain_edr_3: {=u8:?} }}" , self . tx_mod_gain_edr_0 () , self . tx_mod_gain_edr_1 () , self . tx_mod_gain_edr_2 () , self . tx_mod_gain_edr_3 ())
    }
}
#[doc = "TX_IF_MOD_CFG7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxIfModCfg7(pub u32);
impl TxIfModCfg7 {
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_edr_4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_edr_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_edr_5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_edr_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_edr_6(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_edr_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_mod_gain_edr_7(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_mod_gain_edr_7(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for TxIfModCfg7 {
    #[inline(always)]
    fn default() -> TxIfModCfg7 {
        TxIfModCfg7(0)
    }
}
impl core::fmt::Debug for TxIfModCfg7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxIfModCfg7")
            .field("tx_mod_gain_edr_4", &self.tx_mod_gain_edr_4())
            .field("tx_mod_gain_edr_5", &self.tx_mod_gain_edr_5())
            .field("tx_mod_gain_edr_6", &self.tx_mod_gain_edr_6())
            .field("tx_mod_gain_edr_7", &self.tx_mod_gain_edr_7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxIfModCfg7 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TxIfModCfg7 {{ tx_mod_gain_edr_4: {=u8:?}, tx_mod_gain_edr_5: {=u8:?}, tx_mod_gain_edr_6: {=u8:?}, tx_mod_gain_edr_7: {=u8:?} }}" , self . tx_mod_gain_edr_4 () , self . tx_mod_gain_edr_5 () , self . tx_mod_gain_edr_6 () , self . tx_mod_gain_edr_7 ())
    }
}
#[doc = "TX_LFP_CFG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxLfpCfg(pub u32);
impl TxLfpCfg {
    #[must_use]
    #[inline(always)]
    pub const fn lfp_fcw_sel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_lfp_fcw_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lfp_fcw(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_lfp_fcw(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 1usize)) | (((val as u32) & 0x03ff) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_sdm_dither_en(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_tx_sdm_dither_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tx_sdm_sel(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_tx_sdm_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for TxLfpCfg {
    #[inline(always)]
    fn default() -> TxLfpCfg {
        TxLfpCfg(0)
    }
}
impl core::fmt::Debug for TxLfpCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxLfpCfg")
            .field("lfp_fcw_sel", &self.lfp_fcw_sel())
            .field("lfp_fcw", &self.lfp_fcw())
            .field("tx_sdm_dither_en", &self.tx_sdm_dither_en())
            .field("tx_sdm_sel", &self.tx_sdm_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxLfpCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TxLfpCfg {{ lfp_fcw_sel: {=bool:?}, lfp_fcw: {=u16:?}, tx_sdm_dither_en: {=u8:?}, tx_sdm_sel: {=bool:?} }}" , self . lfp_fcw_sel () , self . lfp_fcw () , self . tx_sdm_dither_en () , self . tx_sdm_sel ())
    }
}
#[doc = "TX_PA_CFG"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxPaCfg(pub u32);
impl TxPaCfg {
    #[must_use]
    #[inline(always)]
    pub const fn pa_ramp_force(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_pa_ramp_force(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pa_ramp_factor_idx(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_pa_ramp_factor_idx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
}
impl Default for TxPaCfg {
    #[inline(always)]
    fn default() -> TxPaCfg {
        TxPaCfg(0)
    }
}
impl core::fmt::Debug for TxPaCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxPaCfg")
            .field("pa_ramp_force", &self.pa_ramp_force())
            .field("pa_ramp_factor_idx", &self.pa_ramp_factor_idx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxPaCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TxPaCfg {{ pa_ramp_force: {=u8:?}, pa_ramp_factor_idx: {=u8:?} }}",
            self.pa_ramp_force(),
            self.pa_ramp_factor_idx()
        )
    }
}
#[doc = "TX_RCC_CTRL"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxRccCtrl(pub u32);
impl TxRccCtrl {
    #[must_use]
    #[inline(always)]
    pub const fn force_pa_ctrl_on(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_force_pa_ctrl_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn force_lfp_on(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_force_lfp_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn force_hfp_on(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_force_hfp_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn force_if_mod_on(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_force_if_mod_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn force_rc_on(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_force_rc_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn force_gaussflt_on(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_force_gaussflt_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn force_tx_reset(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_force_tx_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for TxRccCtrl {
    #[inline(always)]
    fn default() -> TxRccCtrl {
        TxRccCtrl(0)
    }
}
impl core::fmt::Debug for TxRccCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxRccCtrl")
            .field("force_pa_ctrl_on", &self.force_pa_ctrl_on())
            .field("force_lfp_on", &self.force_lfp_on())
            .field("force_hfp_on", &self.force_hfp_on())
            .field("force_if_mod_on", &self.force_if_mod_on())
            .field("force_rc_on", &self.force_rc_on())
            .field("force_gaussflt_on", &self.force_gaussflt_on())
            .field("force_tx_reset", &self.force_tx_reset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxRccCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TxRccCtrl {{ force_pa_ctrl_on: {=bool:?}, force_lfp_on: {=bool:?}, force_hfp_on: {=bool:?}, force_if_mod_on: {=bool:?}, force_rc_on: {=bool:?}, force_gaussflt_on: {=bool:?}, force_tx_reset: {=bool:?} }}" , self . force_pa_ctrl_on () , self . force_lfp_on () , self . force_hfp_on () , self . force_if_mod_on () , self . force_rc_on () , self . force_gaussflt_on () , self . force_tx_reset ())
    }
}
