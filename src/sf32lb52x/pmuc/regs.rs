#[doc = "AON Bandgap Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AonBg(pub u32);
impl AonBg {
    #[must_use]
    #[inline(always)]
    pub const fn buf_vos_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_buf_vos_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn buf_vos_step(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_buf_vos_step(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn buf_vos_polar(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_buf_vos_polar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for AonBg {
    #[inline(always)]
    fn default() -> AonBg {
        AonBg(0)
    }
}
impl core::fmt::Debug for AonBg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AonBg")
            .field("buf_vos_trim", &self.buf_vos_trim())
            .field("buf_vos_step", &self.buf_vos_step())
            .field("buf_vos_polar", &self.buf_vos_polar())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AonBg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AonBg {{ buf_vos_trim: {=u8:?}, buf_vos_step: {=u8:?}, buf_vos_polar: {=bool:?} }}",
            self.buf_vos_trim(),
            self.buf_vos_step(),
            self.buf_vos_polar()
        )
    }
}
#[doc = "AON LDO Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AonLdo(pub u32);
impl AonLdo {
    #[must_use]
    #[inline(always)]
    pub const fn vbat_ldo_set_vout(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_vbat_ldo_set_vout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vbat_por_th(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_vbat_por_th(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
}
impl Default for AonLdo {
    #[inline(always)]
    fn default() -> AonLdo {
        AonLdo(0)
    }
}
impl core::fmt::Debug for AonLdo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AonLdo")
            .field("vbat_ldo_set_vout", &self.vbat_ldo_set_vout())
            .field("vbat_por_th", &self.vbat_por_th())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AonLdo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AonLdo {{ vbat_ldo_set_vout: {=u8:?}, vbat_por_th: {=u8:?} }}",
            self.vbat_ldo_set_vout(),
            self.vbat_por_th()
        )
    }
}
#[doc = "BUCK Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BuckCr1(pub u32);
impl BuckCr1 {
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ctrl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn mot_ctune(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_mot_ctune(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cot_ctune(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_cot_ctune(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn comp_bm_ahi(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_comp_bm_ahi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn comp_iq_tune(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_comp_iq_tune(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn comp_idyn_tune(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_comp_idyn_tune(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 15usize)) | (((val as u32) & 0x03) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn iocp_tune(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_iocp_tune(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sel_iocp_hi(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_sel_iocp_hi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sel_lx22(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_sel_lx22(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ocp_aon(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ocp_aon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn zcd_aon(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_zcd_aon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn uvlo_x_bias(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_uvlo_x_bias(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn bg_buf_vos_trim(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_bg_buf_vos_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn bg_buf_vos_step(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_bg_buf_vos_step(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn bg_buf_vos_polar(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_bg_buf_vos_polar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ss_done(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ss_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for BuckCr1 {
    #[inline(always)]
    fn default() -> BuckCr1 {
        BuckCr1(0)
    }
}
impl core::fmt::Debug for BuckCr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BuckCr1")
            .field("en", &self.en())
            .field("ctrl", &self.ctrl())
            .field("mot_ctune", &self.mot_ctune())
            .field("cot_ctune", &self.cot_ctune())
            .field("comp_bm_ahi", &self.comp_bm_ahi())
            .field("comp_iq_tune", &self.comp_iq_tune())
            .field("comp_idyn_tune", &self.comp_idyn_tune())
            .field("iocp_tune", &self.iocp_tune())
            .field("sel_iocp_hi", &self.sel_iocp_hi())
            .field("sel_lx22", &self.sel_lx22())
            .field("ocp_aon", &self.ocp_aon())
            .field("zcd_aon", &self.zcd_aon())
            .field("uvlo_x_bias", &self.uvlo_x_bias())
            .field("bg_buf_vos_trim", &self.bg_buf_vos_trim())
            .field("bg_buf_vos_step", &self.bg_buf_vos_step())
            .field("bg_buf_vos_polar", &self.bg_buf_vos_polar())
            .field("ss_done", &self.ss_done())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BuckCr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "BuckCr1 {{ en: {=bool:?}, ctrl: {=bool:?}, mot_ctune: {=u8:?}, cot_ctune: {=u8:?}, comp_bm_ahi: {=bool:?}, comp_iq_tune: {=u8:?}, comp_idyn_tune: {=u8:?}, iocp_tune: {=u8:?}, sel_iocp_hi: {=bool:?}, sel_lx22: {=bool:?}, ocp_aon: {=bool:?}, zcd_aon: {=bool:?}, uvlo_x_bias: {=bool:?}, bg_buf_vos_trim: {=u8:?}, bg_buf_vos_step: {=u8:?}, bg_buf_vos_polar: {=bool:?}, ss_done: {=bool:?} }}" , self . en () , self . ctrl () , self . mot_ctune () , self . cot_ctune () , self . comp_bm_ahi () , self . comp_iq_tune () , self . comp_idyn_tune () , self . iocp_tune () , self . sel_iocp_hi () , self . sel_lx22 () , self . ocp_aon () , self . zcd_aon () , self . uvlo_x_bias () , self . bg_buf_vos_trim () , self . bg_buf_vos_step () , self . bg_buf_vos_polar () , self . ss_done ())
    }
}
#[doc = "BUCK Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BuckCr2(pub u32);
impl BuckCr2 {
    #[must_use]
    #[inline(always)]
    pub const fn h2m_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_h2m_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn h2l_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_h2l_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn m2l_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_m2l_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn l2m_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_l2m_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn m2h_cnt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_m2h_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn l2h_cnt(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_l2h_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn l2m_cnt(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_l2m_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn bypass_pg(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_bypass_pg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn bypass_ocp(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_bypass_ocp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn bypass_uvlo(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_bypass_uvlo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn force_rdy(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_force_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "1.1V"]
    #[must_use]
    #[inline(always)]
    pub const fn set_vout_m(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "1.1V"]
    #[inline(always)]
    pub const fn set_set_vout_m(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "0.75V"]
    #[must_use]
    #[inline(always)]
    pub const fn set_vout_l(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "0.75V"]
    #[inline(always)]
    pub const fn set_set_vout_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Discharge for TDIS*4 LP clock cycles during reboot"]
    #[must_use]
    #[inline(always)]
    pub const fn tdis(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Discharge for TDIS*4 LP clock cycles during reboot"]
    #[inline(always)]
    pub const fn set_tdis(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for BuckCr2 {
    #[inline(always)]
    fn default() -> BuckCr2 {
        BuckCr2(0)
    }
}
impl core::fmt::Debug for BuckCr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BuckCr2")
            .field("h2m_en", &self.h2m_en())
            .field("h2l_en", &self.h2l_en())
            .field("m2l_en", &self.m2l_en())
            .field("l2m_en", &self.l2m_en())
            .field("m2h_cnt", &self.m2h_cnt())
            .field("l2h_cnt", &self.l2h_cnt())
            .field("l2m_cnt", &self.l2m_cnt())
            .field("bypass_pg", &self.bypass_pg())
            .field("bypass_ocp", &self.bypass_ocp())
            .field("bypass_uvlo", &self.bypass_uvlo())
            .field("force_rdy", &self.force_rdy())
            .field("set_vout_m", &self.set_vout_m())
            .field("set_vout_l", &self.set_vout_l())
            .field("tdis", &self.tdis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BuckCr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "BuckCr2 {{ h2m_en: {=bool:?}, h2l_en: {=bool:?}, m2l_en: {=bool:?}, l2m_en: {=bool:?}, m2h_cnt: {=u8:?}, l2h_cnt: {=u8:?}, l2m_cnt: {=u8:?}, bypass_pg: {=bool:?}, bypass_ocp: {=bool:?}, bypass_uvlo: {=bool:?}, force_rdy: {=bool:?}, set_vout_m: {=u8:?}, set_vout_l: {=u8:?}, tdis: {=u8:?} }}" , self . h2m_en () , self . h2l_en () , self . m2l_en () , self . l2m_en () , self . m2h_cnt () , self . l2h_cnt () , self . l2m_cnt () , self . bypass_pg () , self . bypass_ocp () , self . bypass_uvlo () , self . force_rdy () , self . set_vout_m () , self . set_vout_l () , self . tdis ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BuckVout(pub u32);
impl BuckVout {
    #[doc = "0xF - 1.35V, 0xD - 1.25V, 0x9 - 1.05V, 0x6 - 0.9V, 0x2 - 0.7V"]
    #[must_use]
    #[inline(always)]
    pub const fn vout(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "0xF - 1.35V, 0xD - 1.25V, 0x9 - 1.05V, 0x6 - 0.9V, 0x2 - 0.7V"]
    #[inline(always)]
    pub const fn set_vout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for BuckVout {
    #[inline(always)]
    fn default() -> BuckVout {
        BuckVout(0)
    }
}
impl core::fmt::Debug for BuckVout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BuckVout")
            .field("vout", &self.vout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BuckVout {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BuckVout {{ vout: {=u8:?} }}", self.vout())
    }
}
#[doc = "CAU Bandgap Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CauBgr(pub u32);
impl CauBgr {
    #[must_use]
    #[inline(always)]
    pub const fn hpbg_vddpsw_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_hpbg_vddpsw_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn hpbg_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_hpbg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lpbg_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_lpbg_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lpbg_vref06(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_lpbg_vref06(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn lpbg_vref12(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_lpbg_vref12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 7usize)) | (((val as u32) & 0x0f) << 7usize);
    }
}
impl Default for CauBgr {
    #[inline(always)]
    fn default() -> CauBgr {
        CauBgr(0)
    }
}
impl core::fmt::Debug for CauBgr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CauBgr")
            .field("hpbg_vddpsw_en", &self.hpbg_vddpsw_en())
            .field("hpbg_en", &self.hpbg_en())
            .field("lpbg_en", &self.lpbg_en())
            .field("lpbg_vref06", &self.lpbg_vref06())
            .field("lpbg_vref12", &self.lpbg_vref12())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CauBgr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "CauBgr {{ hpbg_vddpsw_en: {=bool:?}, hpbg_en: {=bool:?}, lpbg_en: {=bool:?}, lpbg_vref06: {=u8:?}, lpbg_vref12: {=u8:?} }}" , self . hpbg_vddpsw_en () , self . hpbg_en () , self . lpbg_en () , self . lpbg_vref06 () , self . lpbg_vref12 ())
    }
}
#[doc = "CAU Reserved Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CauRsvd(pub u32);
impl CauRsvd {
    #[must_use]
    #[inline(always)]
    pub const fn reserve0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_reserve0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn reserve1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_reserve1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn reserve2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_reserve2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for CauRsvd {
    #[inline(always)]
    fn default() -> CauRsvd {
        CauRsvd(0)
    }
}
impl core::fmt::Debug for CauRsvd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CauRsvd")
            .field("reserve0", &self.reserve0())
            .field("reserve1", &self.reserve1())
            .field("reserve2", &self.reserve2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CauRsvd {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CauRsvd {{ reserve0: {=u8:?}, reserve1: {=u8:?}, reserve2: {=u8:?} }}",
            self.reserve0(),
            self.reserve1(),
            self.reserve2()
        )
    }
}
#[doc = "CAU Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CauTr(pub u32);
impl CauTr {
    #[must_use]
    #[inline(always)]
    pub const fn cau_dc_tr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_cau_dc_tr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cau_dc_br(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_cau_dc_br(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cau_dc_mr(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_cau_dc_mr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
}
impl Default for CauTr {
    #[inline(always)]
    fn default() -> CauTr {
        CauTr(0)
    }
}
impl core::fmt::Debug for CauTr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CauTr")
            .field("cau_dc_tr", &self.cau_dc_tr())
            .field("cau_dc_br", &self.cau_dc_br())
            .field("cau_dc_mr", &self.cau_dc_mr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CauTr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CauTr {{ cau_dc_tr: {=u8:?}, cau_dc_br: {=u8:?}, cau_dc_mr: {=u8:?} }}",
            self.cau_dc_tr(),
            self.cau_dc_br(),
            self.cau_dc_mr()
        )
    }
}
#[doc = "Charger Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgCr1(pub u32);
impl ChgCr1 {
    #[doc = "only available when CR3 FORCE_CTRL bit is set"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "only available when CR3 FORCE_CTRL bit is set"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "only available when CR3 FORCE_CTRL bit is set"]
    #[must_use]
    #[inline(always)]
    pub const fn loop_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "only available when CR3 FORCE_CTRL bit is set"]
    #[inline(always)]
    pub const fn set_loop_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cc_ictrl(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_cc_ictrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 2usize)) | (((val as u32) & 0x3f) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cc_vctrl(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_cc_vctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cc_mp(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_cc_mp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 14usize)) | (((val as u32) & 0x1f) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cc_mn(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_cc_mn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cc_range(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_cc_range(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cv_vctrl(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_cv_vctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
    }
}
impl Default for ChgCr1 {
    #[inline(always)]
    fn default() -> ChgCr1 {
        ChgCr1(0)
    }
}
impl core::fmt::Debug for ChgCr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChgCr1")
            .field("en", &self.en())
            .field("loop_en", &self.loop_en())
            .field("cc_ictrl", &self.cc_ictrl())
            .field("cc_vctrl", &self.cc_vctrl())
            .field("cc_mp", &self.cc_mp())
            .field("cc_mn", &self.cc_mn())
            .field("cc_range", &self.cc_range())
            .field("cv_vctrl", &self.cv_vctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChgCr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "ChgCr1 {{ en: {=bool:?}, loop_en: {=bool:?}, cc_ictrl: {=u8:?}, cc_vctrl: {=u8:?}, cc_mp: {=u8:?}, cc_mn: {=u8:?}, cc_range: {=u8:?}, cv_vctrl: {=u8:?} }}" , self . en () , self . loop_en () , self . cc_ictrl () , self . cc_vctrl () , self . cc_mp () , self . cc_mn () , self . cc_range () , self . cv_vctrl ())
    }
}
#[doc = "Charger Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgCr2(pub u32);
impl ChgCr2 {
    #[must_use]
    #[inline(always)]
    pub const fn bg_prog_v1p2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_bg_prog_v1p2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn precc_range(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_precc_range(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn precc_ictrl(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_precc_ictrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rep_vctrl(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rep_vctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn high_vctrl(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_high_vctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn bm_eoc(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_bm_eoc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn range_eoc(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_range_eoc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vbat_range(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_vbat_range(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for ChgCr2 {
    #[inline(always)]
    fn default() -> ChgCr2 {
        ChgCr2(0)
    }
}
impl core::fmt::Debug for ChgCr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChgCr2")
            .field("bg_prog_v1p2", &self.bg_prog_v1p2())
            .field("precc_range", &self.precc_range())
            .field("precc_ictrl", &self.precc_ictrl())
            .field("rep_vctrl", &self.rep_vctrl())
            .field("high_vctrl", &self.high_vctrl())
            .field("bm_eoc", &self.bm_eoc())
            .field("range_eoc", &self.range_eoc())
            .field("vbat_range", &self.vbat_range())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChgCr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "ChgCr2 {{ bg_prog_v1p2: {=u8:?}, precc_range: {=u8:?}, precc_ictrl: {=u8:?}, rep_vctrl: {=u8:?}, high_vctrl: {=u8:?}, bm_eoc: {=u8:?}, range_eoc: {=bool:?}, vbat_range: {=u8:?} }}" , self . bg_prog_v1p2 () , self . precc_range () , self . precc_ictrl () , self . rep_vctrl () , self . high_vctrl () , self . bm_eoc () , self . range_eoc () , self . vbat_range ())
    }
}
#[doc = "Charger Control Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgCr3(pub u32);
impl ChgCr3 {
    #[must_use]
    #[inline(always)]
    pub const fn dly1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_dly1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dly2(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_dly2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
    }
    #[doc = "When charger plugged out, this bit will auto reset"]
    #[must_use]
    #[inline(always)]
    pub const fn force_rst(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "When charger plugged out, this bit will auto reset"]
    #[inline(always)]
    pub const fn set_force_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "When charger plugged out, this bit will auto reset"]
    #[must_use]
    #[inline(always)]
    pub const fn force_ctrl(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "When charger plugged out, this bit will auto reset"]
    #[inline(always)]
    pub const fn set_force_ctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for ChgCr3 {
    #[inline(always)]
    fn default() -> ChgCr3 {
        ChgCr3(0)
    }
}
impl core::fmt::Debug for ChgCr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChgCr3")
            .field("dly1", &self.dly1())
            .field("dly2", &self.dly2())
            .field("force_rst", &self.force_rst())
            .field("force_ctrl", &self.force_ctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChgCr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "ChgCr3 {{ dly1: {=u8:?}, dly2: {=u8:?}, force_rst: {=bool:?}, force_ctrl: {=bool:?} }}" , self . dly1 () , self . dly2 () , self . force_rst () , self . force_ctrl ())
    }
}
#[doc = "Charger Control Register 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgCr4(pub u32);
impl ChgCr4 {
    #[must_use]
    #[inline(always)]
    pub const fn ie_vbus_rdy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ie_vbus_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ie_vbat_high(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ie_vbat_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ie_above_rep(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ie_above_rep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ie_above_cc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ie_above_cc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ie_cc_mode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ie_cc_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ie_cv_mode(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ie_cv_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ie_eoc_mode(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ie_eoc_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ie_eoc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ie_eoc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, others - both edge"]
    #[must_use]
    #[inline(always)]
    pub const fn im_vbus_rdy(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[doc = "0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, others - both edge"]
    #[inline(always)]
    pub const fn set_im_vbus_rdy(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn im_vbat_high(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_im_vbat_high(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn im_above_rep(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_im_above_rep(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn im_above_cc(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_im_above_cc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn im_cc_mode(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_im_cc_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 23usize)) | (((val as u32) & 0x07) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn im_cv_mode(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_im_cv_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val as u32) & 0x07) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn im_eoc_mode(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_im_eoc_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for ChgCr4 {
    #[inline(always)]
    fn default() -> ChgCr4 {
        ChgCr4(0)
    }
}
impl core::fmt::Debug for ChgCr4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChgCr4")
            .field("ie_vbus_rdy", &self.ie_vbus_rdy())
            .field("ie_vbat_high", &self.ie_vbat_high())
            .field("ie_above_rep", &self.ie_above_rep())
            .field("ie_above_cc", &self.ie_above_cc())
            .field("ie_cc_mode", &self.ie_cc_mode())
            .field("ie_cv_mode", &self.ie_cv_mode())
            .field("ie_eoc_mode", &self.ie_eoc_mode())
            .field("ie_eoc", &self.ie_eoc())
            .field("im_vbus_rdy", &self.im_vbus_rdy())
            .field("im_vbat_high", &self.im_vbat_high())
            .field("im_above_rep", &self.im_above_rep())
            .field("im_above_cc", &self.im_above_cc())
            .field("im_cc_mode", &self.im_cc_mode())
            .field("im_cv_mode", &self.im_cv_mode())
            .field("im_eoc_mode", &self.im_eoc_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChgCr4 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "ChgCr4 {{ ie_vbus_rdy: {=bool:?}, ie_vbat_high: {=bool:?}, ie_above_rep: {=bool:?}, ie_above_cc: {=bool:?}, ie_cc_mode: {=bool:?}, ie_cv_mode: {=bool:?}, ie_eoc_mode: {=bool:?}, ie_eoc: {=bool:?}, im_vbus_rdy: {=u8:?}, im_vbat_high: {=u8:?}, im_above_rep: {=u8:?}, im_above_cc: {=u8:?}, im_cc_mode: {=u8:?}, im_cv_mode: {=u8:?}, im_eoc_mode: {=u8:?} }}" , self . ie_vbus_rdy () , self . ie_vbat_high () , self . ie_above_rep () , self . ie_above_cc () , self . ie_cc_mode () , self . ie_cv_mode () , self . ie_eoc_mode () , self . ie_eoc () , self . im_vbus_rdy () , self . im_vbat_high () , self . im_above_rep () , self . im_above_cc () , self . im_cc_mode () , self . im_cv_mode () , self . im_eoc_mode ())
    }
}
#[doc = "Charger Control Register 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgCr5(pub u32);
impl ChgCr5 {
    #[must_use]
    #[inline(always)]
    pub const fn ic_vbus_rdy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ic_vbus_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ic_vbat_high(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ic_vbat_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ic_above_rep(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ic_above_rep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ic_above_cc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ic_above_cc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ic_cc_mode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ic_cc_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ic_cv_mode(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ic_cv_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ic_eoc_mode(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ic_eoc_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ic_eoc(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ic_eoc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn is_vbus_rdy(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_is_vbus_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn is_vbat_high(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_is_vbat_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn is_above_rep(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_is_above_rep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn is_above_cc(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_is_above_cc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn is_cc_mode(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_is_cc_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn is_cv_mode(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_is_cv_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn is_eoc_mode(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_is_eoc_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn is_eoc(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_is_eoc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for ChgCr5 {
    #[inline(always)]
    fn default() -> ChgCr5 {
        ChgCr5(0)
    }
}
impl core::fmt::Debug for ChgCr5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChgCr5")
            .field("ic_vbus_rdy", &self.ic_vbus_rdy())
            .field("ic_vbat_high", &self.ic_vbat_high())
            .field("ic_above_rep", &self.ic_above_rep())
            .field("ic_above_cc", &self.ic_above_cc())
            .field("ic_cc_mode", &self.ic_cc_mode())
            .field("ic_cv_mode", &self.ic_cv_mode())
            .field("ic_eoc_mode", &self.ic_eoc_mode())
            .field("ic_eoc", &self.ic_eoc())
            .field("is_vbus_rdy", &self.is_vbus_rdy())
            .field("is_vbat_high", &self.is_vbat_high())
            .field("is_above_rep", &self.is_above_rep())
            .field("is_above_cc", &self.is_above_cc())
            .field("is_cc_mode", &self.is_cc_mode())
            .field("is_cv_mode", &self.is_cv_mode())
            .field("is_eoc_mode", &self.is_eoc_mode())
            .field("is_eoc", &self.is_eoc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChgCr5 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "ChgCr5 {{ ic_vbus_rdy: {=bool:?}, ic_vbat_high: {=bool:?}, ic_above_rep: {=bool:?}, ic_above_cc: {=bool:?}, ic_cc_mode: {=bool:?}, ic_cv_mode: {=bool:?}, ic_eoc_mode: {=bool:?}, ic_eoc: {=bool:?}, is_vbus_rdy: {=bool:?}, is_vbat_high: {=bool:?}, is_above_rep: {=bool:?}, is_above_cc: {=bool:?}, is_cc_mode: {=bool:?}, is_cv_mode: {=bool:?}, is_eoc_mode: {=bool:?}, is_eoc: {=bool:?} }}" , self . ic_vbus_rdy () , self . ic_vbat_high () , self . ic_above_rep () , self . ic_above_cc () , self . ic_cc_mode () , self . ic_cv_mode () , self . ic_eoc_mode () , self . ic_eoc () , self . is_vbus_rdy () , self . is_vbat_high () , self . is_above_rep () , self . is_above_cc () , self . is_cc_mode () , self . is_cv_mode () , self . is_eoc_mode () , self . is_eoc ())
    }
}
#[doc = "Charger Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChgSr(pub u32);
impl ChgSr {
    #[must_use]
    #[inline(always)]
    pub const fn vbus_rdy_out(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_vbus_rdy_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vbat_high_out(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_vbat_high_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vbat_above_rep_out(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_vbat_above_rep_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vbat_above_cc_out(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_vbat_above_cc_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cc_mode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cc_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cv_mode(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cv_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn eoc_mode(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_eoc_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Charger finite state machine"]
    #[must_use]
    #[inline(always)]
    pub const fn chg_state(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x7f;
        val as u8
    }
    #[doc = "Charger finite state machine"]
    #[inline(always)]
    pub const fn set_chg_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
    }
}
impl Default for ChgSr {
    #[inline(always)]
    fn default() -> ChgSr {
        ChgSr(0)
    }
}
impl core::fmt::Debug for ChgSr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChgSr")
            .field("vbus_rdy_out", &self.vbus_rdy_out())
            .field("vbat_high_out", &self.vbat_high_out())
            .field("vbat_above_rep_out", &self.vbat_above_rep_out())
            .field("vbat_above_cc_out", &self.vbat_above_cc_out())
            .field("cc_mode", &self.cc_mode())
            .field("cv_mode", &self.cv_mode())
            .field("eoc_mode", &self.eoc_mode())
            .field("chg_state", &self.chg_state())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChgSr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "ChgSr {{ vbus_rdy_out: {=bool:?}, vbat_high_out: {=bool:?}, vbat_above_rep_out: {=bool:?}, vbat_above_cc_out: {=bool:?}, cc_mode: {=bool:?}, cv_mode: {=bool:?}, eoc_mode: {=bool:?}, chg_state: {=u8:?} }}" , self . vbus_rdy_out () , self . vbat_high_out () , self . vbat_above_rep_out () , self . vbat_above_cc_out () , self . cc_mode () , self . cv_mode () , self . eoc_mode () , self . chg_state ())
    }
}
#[doc = "Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr(pub u32);
impl Cr {
    #[doc = "LP clock for watchdog and FSM. 0 - LRC10, 1 - LRC32"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_lpclk(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "LP clock for watchdog and FSM. 0 - LRC10, 1 - LRC32"]
    #[inline(always)]
    pub const fn set_sel_lpclk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write 1 to enter hibernate mode; write 0 to clear when exit from hibernate"]
    #[must_use]
    #[inline(always)]
    pub const fn hiber_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to enter hibernate mode; write 0 to clear when exit from hibernate"]
    #[inline(always)]
    pub const fn set_hiber_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write 1 to reboot; write 0 to clear after boot up"]
    #[must_use]
    #[inline(always)]
    pub const fn reboot(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to reboot; write 0 to clear after boot up"]
    #[inline(always)]
    pub const fn set_reboot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If set to 1, IO retained during hibernate mode; otherwise, high-Z"]
    #[must_use]
    #[inline(always)]
    pub const fn pin_ret(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, IO retained during hibernate mode; otherwise, high-Z"]
    #[inline(always)]
    pub const fn set_pin_ret(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "4/6 - both edge (high-active detection), 5/7 - both edge (low-active detection)"]
    #[must_use]
    #[inline(always)]
    pub const fn pin0_mode(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "4/6 - both edge (high-active detection), 5/7 - both edge (low-active detection)"]
    #[inline(always)]
    pub const fn set_pin0_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge"]
    #[must_use]
    #[inline(always)]
    pub const fn pin1_mode(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[doc = "0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge"]
    #[inline(always)]
    pub const fn set_pin1_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
    }
    #[doc = "select one out of PA\\[44:24\\]. 0 - PA24, 1 - PA25, 20 - PA44, etc."]
    #[must_use]
    #[inline(always)]
    pub const fn pin0_sel(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "select one out of PA\\[44:24\\]. 0 - PA24, 1 - PA25, 20 - PA44, etc."]
    #[inline(always)]
    pub const fn set_pin0_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pin1_sel(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_pin1_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
    }
}
impl Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0)
    }
}
impl core::fmt::Debug for Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr")
            .field("sel_lpclk", &self.sel_lpclk())
            .field("hiber_en", &self.hiber_en())
            .field("reboot", &self.reboot())
            .field("pin_ret", &self.pin_ret())
            .field("pin0_mode", &self.pin0_mode())
            .field("pin1_mode", &self.pin1_mode())
            .field("pin0_sel", &self.pin0_sel())
            .field("pin1_sel", &self.pin1_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cr {{ sel_lpclk: {=bool:?}, hiber_en: {=bool:?}, reboot: {=bool:?}, pin_ret: {=bool:?}, pin0_mode: {=u8:?}, pin1_mode: {=u8:?}, pin0_sel: {=u8:?}, pin1_sel: {=u8:?} }}" , self . sel_lpclk () , self . hiber_en () , self . reboot () , self . pin_ret () , self . pin0_mode () , self . pin1_mode () , self . pin0_sel () , self . pin1_sel ())
    }
}
#[doc = "DBL96 Calibration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbl96Calr(pub u32);
impl Dbl96Calr {
    #[must_use]
    #[inline(always)]
    pub const fn cal_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cal_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cal_close_ext_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cal_close_ext_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cal_op(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_cal_op(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 2usize)) | (((val as u32) & 0x07ff) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cal_lock(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cal_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for Dbl96Calr {
    #[inline(always)]
    fn default() -> Dbl96Calr {
        Dbl96Calr(0)
    }
}
impl core::fmt::Debug for Dbl96Calr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbl96Calr")
            .field("cal_en", &self.cal_en())
            .field("cal_close_ext_en", &self.cal_close_ext_en())
            .field("cal_op", &self.cal_op())
            .field("cal_lock", &self.cal_lock())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbl96Calr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dbl96Calr {{ cal_en: {=bool:?}, cal_close_ext_en: {=bool:?}, cal_op: {=u16:?}, cal_lock: {=bool:?} }}" , self . cal_en () , self . cal_close_ext_en () , self . cal_op () , self . cal_lock ())
    }
}
#[doc = "DBL96 Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbl96Cr(pub u32);
impl Dbl96Cr {
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn out_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_out_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn todig_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_todig_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn todig_str(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_todig_str(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn torf_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_torf_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn tooslo_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_tooslo_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn loop_rstb(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_loop_rstb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ph_en(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ph_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dly_en(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_dly_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dly_ext_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_dly_ext_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dly_sel_ext_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_dly_sel_ext_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dly_sel_ext(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_dly_sel_ext(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 18usize)) | (((val as u32) & 0x07ff) << 18usize);
    }
}
impl Default for Dbl96Cr {
    #[inline(always)]
    fn default() -> Dbl96Cr {
        Dbl96Cr(0)
    }
}
impl core::fmt::Debug for Dbl96Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbl96Cr")
            .field("en", &self.en())
            .field("out_en", &self.out_en())
            .field("todig_en", &self.todig_en())
            .field("todig_str", &self.todig_str())
            .field("torf_en", &self.torf_en())
            .field("tooslo_en", &self.tooslo_en())
            .field("loop_rstb", &self.loop_rstb())
            .field("ph_en", &self.ph_en())
            .field("dly_en", &self.dly_en())
            .field("dly_ext_en", &self.dly_ext_en())
            .field("dly_sel_ext_en", &self.dly_sel_ext_en())
            .field("dly_sel_ext", &self.dly_sel_ext())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbl96Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dbl96Cr {{ en: {=bool:?}, out_en: {=bool:?}, todig_en: {=bool:?}, todig_str: {=u8:?}, torf_en: {=bool:?}, tooslo_en: {=bool:?}, loop_rstb: {=bool:?}, ph_en: {=u8:?}, dly_en: {=u8:?}, dly_ext_en: {=bool:?}, dly_sel_ext_en: {=bool:?}, dly_sel_ext: {=u16:?} }}" , self . en () , self . out_en () , self . todig_en () , self . todig_str () , self . torf_en () , self . tooslo_en () , self . loop_rstb () , self . ph_en () , self . dly_en () , self . dly_ext_en () , self . dly_sel_ext_en () , self . dly_sel_ext ())
    }
}
#[doc = "HPSYS LDO Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HpsysLdo(pub u32);
impl HpsysLdo {
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn bp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_bp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "optional voltage (0.9V)"]
    #[must_use]
    #[inline(always)]
    pub const fn vref(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "optional voltage (0.9V)"]
    #[inline(always)]
    pub const fn set_vref(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[doc = "Lower voltage for deep sleep mode (0.6V)"]
    #[must_use]
    #[inline(always)]
    pub const fn vref2(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x0f;
        val as u8
    }
    #[doc = "Lower voltage for deep sleep mode (0.6V)"]
    #[inline(always)]
    pub const fn set_vref2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
    }
    #[doc = "HPSYS_LDO power up delay in CLK_LP cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn dly(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "HPSYS_LDO power up delay in CLK_LP cycles"]
    #[inline(always)]
    pub const fn set_dly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rdy(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for HpsysLdo {
    #[inline(always)]
    fn default() -> HpsysLdo {
        HpsysLdo(0)
    }
}
impl core::fmt::Debug for HpsysLdo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HpsysLdo")
            .field("en", &self.en())
            .field("bp", &self.bp())
            .field("vref", &self.vref())
            .field("vref2", &self.vref2())
            .field("dly", &self.dly())
            .field("rdy", &self.rdy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HpsysLdo {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "HpsysLdo {{ en: {=bool:?}, bp: {=bool:?}, vref: {=u8:?}, vref2: {=u8:?}, dly: {=u8:?}, rdy: {=bool:?} }}" , self . en () , self . bp () , self . vref () , self . vref2 () , self . dly () , self . rdy ())
    }
}
#[doc = "HPSYS Switch Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HpsysSwr(pub u32);
impl HpsysSwr {
    #[doc = "0\\] - RET_LDO; \\[1\\] - HPSYS_LDO"]
    #[must_use]
    #[inline(always)]
    pub const fn psw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "0\\] - RET_LDO; \\[1\\] - HPSYS_LDO"]
    #[inline(always)]
    pub const fn set_psw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "PSW value during DS/SB"]
    #[must_use]
    #[inline(always)]
    pub const fn psw_ret(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "PSW value during DS/SB"]
    #[inline(always)]
    pub const fn set_psw_ret(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "wait for N cycles before asserting RDY"]
    #[must_use]
    #[inline(always)]
    pub const fn dly(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "wait for N cycles before asserting RDY"]
    #[inline(always)]
    pub const fn set_dly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Cut off VHPMEM entirely during standby. No retention"]
    #[must_use]
    #[inline(always)]
    pub const fn noret(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Cut off VHPMEM entirely during standby. No retention"]
    #[inline(always)]
    pub const fn set_noret(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rdy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for HpsysSwr {
    #[inline(always)]
    fn default() -> HpsysSwr {
        HpsysSwr(0)
    }
}
impl core::fmt::Debug for HpsysSwr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HpsysSwr")
            .field("psw", &self.psw())
            .field("psw_ret", &self.psw_ret())
            .field("dly", &self.dly())
            .field("noret", &self.noret())
            .field("rdy", &self.rdy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HpsysSwr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "HpsysSwr {{ psw: {=u8:?}, psw_ret: {=u8:?}, dly: {=u8:?}, noret: {=bool:?}, rdy: {=bool:?} }}" , self . psw () , self . psw_ret () , self . dly () , self . noret () , self . rdy ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HpsysVout(pub u32);
impl HpsysVout {
    #[doc = "0xD - 1.2V, 0xA - 1.1V, 0x8 - 1.0V, 0x5 - 0.9V"]
    #[must_use]
    #[inline(always)]
    pub const fn vout(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "0xD - 1.2V, 0xA - 1.1V, 0x8 - 1.0V, 0x5 - 0.9V"]
    #[inline(always)]
    pub const fn set_vout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for HpsysVout {
    #[inline(always)]
    fn default() -> HpsysVout {
        HpsysVout(0)
    }
}
impl core::fmt::Debug for HpsysVout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HpsysVout")
            .field("vout", &self.vout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HpsysVout {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "HpsysVout {{ vout: {=u8:?} }}", self.vout())
    }
}
#[doc = "HRC48 Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HrcCr(pub u32);
impl HrcCr {
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ldo_vref(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ldo_vref(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn freq_trim(&self) -> u16 {
        let val = (self.0 >> 5usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_freq_trim(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 5usize)) | (((val as u32) & 0x03ff) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn temp_trim(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_temp_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn clk96m_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_clk96m_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn clkhp_en(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_clkhp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn clkhp_sel(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_clkhp_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val as u32) & 0x03) << 21usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn clkhp_str(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_clkhp_str(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 23usize)) | (((val as u32) & 0x03) << 23usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn clklp_en(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_clklp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn clklp_sel(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_clklp_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn clklp_str(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_clklp_str(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "number of cycles for BG ready. 0 - one cycle of CLK_LP; 1 - two cycles of CLK_LP"]
    #[must_use]
    #[inline(always)]
    pub const fn dly(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "number of cycles for BG ready. 0 - one cycle of CLK_LP; 1 - two cycles of CLK_LP"]
    #[inline(always)]
    pub const fn set_dly(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for HrcCr {
    #[inline(always)]
    fn default() -> HrcCr {
        HrcCr(0)
    }
}
impl core::fmt::Debug for HrcCr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HrcCr")
            .field("en", &self.en())
            .field("ldo_vref", &self.ldo_vref())
            .field("freq_trim", &self.freq_trim())
            .field("temp_trim", &self.temp_trim())
            .field("clk96m_en", &self.clk96m_en())
            .field("clkhp_en", &self.clkhp_en())
            .field("clkhp_sel", &self.clkhp_sel())
            .field("clkhp_str", &self.clkhp_str())
            .field("clklp_en", &self.clklp_en())
            .field("clklp_sel", &self.clklp_sel())
            .field("clklp_str", &self.clklp_str())
            .field("dly", &self.dly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HrcCr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "HrcCr {{ en: {=bool:?}, ldo_vref: {=u8:?}, freq_trim: {=u16:?}, temp_trim: {=u8:?}, clk96m_en: {=bool:?}, clkhp_en: {=bool:?}, clkhp_sel: {=u8:?}, clkhp_str: {=u8:?}, clklp_en: {=bool:?}, clklp_sel: {=u8:?}, clklp_str: {=u8:?}, dly: {=bool:?} }}" , self . en () , self . ldo_vref () , self . freq_trim () , self . temp_trim () , self . clk96m_en () , self . clkhp_en () , self . clkhp_sel () , self . clkhp_str () , self . clklp_en () , self . clklp_sel () , self . clklp_str () , self . dly ())
    }
}
#[doc = "HXT48 Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HxtCr1(pub u32);
impl HxtCr1 {
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn buf_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_buf_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn buf_dig_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_buf_dig_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn buf_dig_str(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_buf_dig_str(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn buf_dll_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_buf_dll_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn buf_dll_str(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_buf_dll_str(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn buf_aud_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_buf_aud_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn buf_aud_str(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_buf_aud_str(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn buf_rf_str(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_buf_rf_str(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ldo_vref(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ldo_vref(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ldo_flt_rsel(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ldo_flt_rsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn gm_en(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_gm_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cbank_sel(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_cbank_sel(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 20usize)) | (((val as u32) & 0x03ff) << 20usize);
    }
}
impl Default for HxtCr1 {
    #[inline(always)]
    fn default() -> HxtCr1 {
        HxtCr1(0)
    }
}
impl core::fmt::Debug for HxtCr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HxtCr1")
            .field("en", &self.en())
            .field("buf_en", &self.buf_en())
            .field("buf_dig_en", &self.buf_dig_en())
            .field("buf_dig_str", &self.buf_dig_str())
            .field("buf_dll_en", &self.buf_dll_en())
            .field("buf_dll_str", &self.buf_dll_str())
            .field("buf_aud_en", &self.buf_aud_en())
            .field("buf_aud_str", &self.buf_aud_str())
            .field("buf_rf_str", &self.buf_rf_str())
            .field("ldo_vref", &self.ldo_vref())
            .field("ldo_flt_rsel", &self.ldo_flt_rsel())
            .field("gm_en", &self.gm_en())
            .field("cbank_sel", &self.cbank_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HxtCr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "HxtCr1 {{ en: {=bool:?}, buf_en: {=bool:?}, buf_dig_en: {=bool:?}, buf_dig_str: {=u8:?}, buf_dll_en: {=bool:?}, buf_dll_str: {=u8:?}, buf_aud_en: {=bool:?}, buf_aud_str: {=u8:?}, buf_rf_str: {=u8:?}, ldo_vref: {=u8:?}, ldo_flt_rsel: {=u8:?}, gm_en: {=bool:?}, cbank_sel: {=u16:?} }}" , self . en () , self . buf_en () , self . buf_dig_en () , self . buf_dig_str () , self . buf_dll_en () , self . buf_dll_str () , self . buf_aud_en () , self . buf_aud_str () , self . buf_rf_str () , self . ldo_vref () , self . ldo_flt_rsel () , self . gm_en () , self . cbank_sel ())
    }
}
#[doc = "HXT48 Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HxtCr2(pub u32);
impl HxtCr2 {
    #[must_use]
    #[inline(always)]
    pub const fn agc_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_agc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn agc_istart_sel(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_agc_istart_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn agc_vth(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_vth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn agc_vindc(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_agc_vindc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn acbuf_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_acbuf_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn acbuf_rsel(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_acbuf_rsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn buf_sel2(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_buf_sel2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn buf_sel3(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_buf_sel3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn idac_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_idac_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn idac(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_idac(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sdadc_clkin_en(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_sdadc_clkin_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sdadc_clkdiv1_sel(&self) -> u8 {
        let val = (self.0 >> 27usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_sdadc_clkdiv1_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sdadc_clkdiv2_sel(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_sdadc_clkdiv2_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn sleep_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_sleep_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for HxtCr2 {
    #[inline(always)]
    fn default() -> HxtCr2 {
        HxtCr2(0)
    }
}
impl core::fmt::Debug for HxtCr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HxtCr2")
            .field("agc_en", &self.agc_en())
            .field("agc_istart_sel", &self.agc_istart_sel())
            .field("agc_vth", &self.agc_vth())
            .field("agc_vindc", &self.agc_vindc())
            .field("acbuf_sel", &self.acbuf_sel())
            .field("acbuf_rsel", &self.acbuf_rsel())
            .field("buf_sel2", &self.buf_sel2())
            .field("buf_sel3", &self.buf_sel3())
            .field("idac_en", &self.idac_en())
            .field("idac", &self.idac())
            .field("sdadc_clkin_en", &self.sdadc_clkin_en())
            .field("sdadc_clkdiv1_sel", &self.sdadc_clkdiv1_sel())
            .field("sdadc_clkdiv2_sel", &self.sdadc_clkdiv2_sel())
            .field("sleep_en", &self.sleep_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HxtCr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "HxtCr2 {{ agc_en: {=bool:?}, agc_istart_sel: {=bool:?}, agc_vth: {=u8:?}, agc_vindc: {=u8:?}, acbuf_sel: {=u8:?}, acbuf_rsel: {=bool:?}, buf_sel2: {=u8:?}, buf_sel3: {=u8:?}, idac_en: {=bool:?}, idac: {=u16:?}, sdadc_clkin_en: {=bool:?}, sdadc_clkdiv1_sel: {=u8:?}, sdadc_clkdiv2_sel: {=u8:?}, sleep_en: {=bool:?} }}" , self . agc_en () , self . agc_istart_sel () , self . agc_vth () , self . agc_vindc () , self . acbuf_sel () , self . acbuf_rsel () , self . buf_sel2 () , self . buf_sel3 () , self . idac_en () , self . idac () , self . sdadc_clkin_en () , self . sdadc_clkdiv1_sel () , self . sdadc_clkdiv2_sel () , self . sleep_en ())
    }
}
#[doc = "HXT48 Control Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HxtCr3(pub u32);
impl HxtCr3 {
    #[must_use]
    #[inline(always)]
    pub const fn buf_dac_str(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_buf_dac_str(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn buf_oslo_str(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_buf_oslo_str(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dly(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_dly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 4usize)) | (((val as u32) & 0x3f) << 4usize);
    }
}
impl Default for HxtCr3 {
    #[inline(always)]
    fn default() -> HxtCr3 {
        HxtCr3(0)
    }
}
impl core::fmt::Debug for HxtCr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HxtCr3")
            .field("buf_dac_str", &self.buf_dac_str())
            .field("buf_oslo_str", &self.buf_oslo_str())
            .field("dly", &self.dly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HxtCr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HxtCr3 {{ buf_dac_str: {=u8:?}, buf_oslo_str: {=u8:?}, dly: {=u8:?} }}",
            self.buf_dac_str(),
            self.buf_oslo_str(),
            self.dly()
        )
    }
}
#[doc = "LPSYS LDO Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpsysLdo(pub u32);
impl LpsysLdo {
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn bp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_bp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "optional voltage (1.0V)"]
    #[must_use]
    #[inline(always)]
    pub const fn vref(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "optional voltage (1.0V)"]
    #[inline(always)]
    pub const fn set_vref(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[doc = "Lower voltage for deep sleep mode (0.6V)"]
    #[must_use]
    #[inline(always)]
    pub const fn vref2(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x0f;
        val as u8
    }
    #[doc = "Lower voltage for deep sleep mode (0.6V)"]
    #[inline(always)]
    pub const fn set_vref2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
    }
    #[doc = "LPSYS_LDO power up delay in CLK_LP cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn dly(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "LPSYS_LDO power up delay in CLK_LP cycles"]
    #[inline(always)]
    pub const fn set_dly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rdy(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for LpsysLdo {
    #[inline(always)]
    fn default() -> LpsysLdo {
        LpsysLdo(0)
    }
}
impl core::fmt::Debug for LpsysLdo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpsysLdo")
            .field("en", &self.en())
            .field("bp", &self.bp())
            .field("vref", &self.vref())
            .field("vref2", &self.vref2())
            .field("dly", &self.dly())
            .field("rdy", &self.rdy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpsysLdo {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "LpsysLdo {{ en: {=bool:?}, bp: {=bool:?}, vref: {=u8:?}, vref2: {=u8:?}, dly: {=u8:?}, rdy: {=bool:?} }}" , self . en () , self . bp () , self . vref () , self . vref2 () , self . dly () , self . rdy ())
    }
}
#[doc = "LPSYS Switch Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpsysSwr(pub u32);
impl LpsysSwr {
    #[doc = "0\\] - RET_LDO; \\[1\\] - LPSYS_LDO"]
    #[must_use]
    #[inline(always)]
    pub const fn psw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "0\\] - RET_LDO; \\[1\\] - LPSYS_LDO"]
    #[inline(always)]
    pub const fn set_psw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "PSW value during DS/SB"]
    #[must_use]
    #[inline(always)]
    pub const fn psw_ret(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "PSW value during DS/SB"]
    #[inline(always)]
    pub const fn set_psw_ret(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "wait for N cycles before asserting RDY"]
    #[must_use]
    #[inline(always)]
    pub const fn dly(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "wait for N cycles before asserting RDY"]
    #[inline(always)]
    pub const fn set_dly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Cut off VLPMEM entirely during standby. No retention"]
    #[must_use]
    #[inline(always)]
    pub const fn noret(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Cut off VLPMEM entirely during standby. No retention"]
    #[inline(always)]
    pub const fn set_noret(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rdy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for LpsysSwr {
    #[inline(always)]
    fn default() -> LpsysSwr {
        LpsysSwr(0)
    }
}
impl core::fmt::Debug for LpsysSwr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpsysSwr")
            .field("psw", &self.psw())
            .field("psw_ret", &self.psw_ret())
            .field("dly", &self.dly())
            .field("noret", &self.noret())
            .field("rdy", &self.rdy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpsysSwr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "LpsysSwr {{ psw: {=u8:?}, psw_ret: {=u8:?}, dly: {=u8:?}, noret: {=bool:?}, rdy: {=bool:?} }}" , self . psw () , self . psw_ret () , self . dly () , self . noret () , self . rdy ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpsysVout(pub u32);
impl LpsysVout {
    #[doc = "0x8 - 1.0V, 0x5 - 0.9V"]
    #[must_use]
    #[inline(always)]
    pub const fn vout(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "0x8 - 1.0V, 0x5 - 0.9V"]
    #[inline(always)]
    pub const fn set_vout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for LpsysVout {
    #[inline(always)]
    fn default() -> LpsysVout {
        LpsysVout(0)
    }
}
impl core::fmt::Debug for LpsysVout {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpsysVout")
            .field("vout", &self.vout())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpsysVout {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LpsysVout {{ vout: {=u8:?} }}", self.vout())
    }
}
#[doc = "RC10K Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lrc10Cr(pub u32);
impl Lrc10Cr {
    #[doc = "Enabled by default"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enabled by default"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cmpbm1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_cmpbm1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cmpbm2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cmpbm2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn chgcrt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_chgcrt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn chgcap(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_chgcap(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn refres(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_refres(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rdy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Lrc10Cr {
    #[inline(always)]
    fn default() -> Lrc10Cr {
        Lrc10Cr(0)
    }
}
impl core::fmt::Debug for Lrc10Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lrc10Cr")
            .field("en", &self.en())
            .field("cmpbm1", &self.cmpbm1())
            .field("cmpbm2", &self.cmpbm2())
            .field("chgcrt", &self.chgcrt())
            .field("chgcap", &self.chgcap())
            .field("refres", &self.refres())
            .field("rdy", &self.rdy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lrc10Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Lrc10Cr {{ en: {=bool:?}, cmpbm1: {=u8:?}, cmpbm2: {=bool:?}, chgcrt: {=u8:?}, chgcap: {=u8:?}, refres: {=bool:?}, rdy: {=bool:?} }}" , self . en () , self . cmpbm1 () , self . cmpbm2 () , self . chgcrt () , self . chgcap () , self . refres () , self . rdy ())
    }
}
#[doc = "RC32K Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lrc32Cr(pub u32);
impl Lrc32Cr {
    #[doc = "Disabled by default"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Disabled by default"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cmpbm1(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_cmpbm1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cmpbm2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cmpbm2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn chgcrt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_chgcrt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rsel(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_rsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rdy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Lrc32Cr {
    #[inline(always)]
    fn default() -> Lrc32Cr {
        Lrc32Cr(0)
    }
}
impl core::fmt::Debug for Lrc32Cr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lrc32Cr")
            .field("en", &self.en())
            .field("cmpbm1", &self.cmpbm1())
            .field("cmpbm2", &self.cmpbm2())
            .field("chgcrt", &self.chgcrt())
            .field("rsel", &self.rsel())
            .field("rdy", &self.rdy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lrc32Cr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Lrc32Cr {{ en: {=bool:?}, cmpbm1: {=u8:?}, cmpbm2: {=bool:?}, chgcrt: {=u8:?}, rsel: {=u8:?}, rdy: {=bool:?} }}" , self . en () , self . cmpbm1 () , self . cmpbm2 () , self . chgcrt () , self . rsel () , self . rdy ())
    }
}
#[doc = "XTAL32K Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LxtCr(pub u32);
impl LxtCr {
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rsn(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rsn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn bm(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_bm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn amp_bm(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub const fn set_amp_bm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ampctrl_enb(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ampctrl_enb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn bmsel(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_bmsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn bmstart(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_bmstart(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cap_sel(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_cap_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "use external 32K from Pin"]
    #[must_use]
    #[inline(always)]
    pub const fn ext_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "use external 32K from Pin"]
    #[inline(always)]
    pub const fn set_ext_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rdy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for LxtCr {
    #[inline(always)]
    fn default() -> LxtCr {
        LxtCr(0)
    }
}
impl core::fmt::Debug for LxtCr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LxtCr")
            .field("en", &self.en())
            .field("rsn", &self.rsn())
            .field("bm", &self.bm())
            .field("amp_bm", &self.amp_bm())
            .field("ampctrl_enb", &self.ampctrl_enb())
            .field("bmsel", &self.bmsel())
            .field("bmstart", &self.bmstart())
            .field("cap_sel", &self.cap_sel())
            .field("ext_en", &self.ext_en())
            .field("rdy", &self.rdy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LxtCr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "LxtCr {{ en: {=bool:?}, rsn: {=bool:?}, bm: {=u8:?}, amp_bm: {=u8:?}, ampctrl_enb: {=bool:?}, bmsel: {=bool:?}, bmstart: {=u8:?}, cap_sel: {=bool:?}, ext_en: {=bool:?}, rdy: {=bool:?} }}" , self . en () , self . rsn () , self . bm () , self . amp_bm () , self . ampctrl_enb () , self . bmsel () , self . bmstart () , self . cap_sel () , self . ext_en () , self . rdy ())
    }
}
#[doc = "Peripherals LDO"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PeriLdo(pub u32);
impl PeriLdo {
    #[must_use]
    #[inline(always)]
    pub const fn en_ldo18(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_en_ldo18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ldo18_vref_sel(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_ldo18_vref_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn ldo18_pd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_ldo18_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn en_vdd33_ldo2(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_en_vdd33_ldo2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vdd33_ldo2_set_vout(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_vdd33_ldo2_set_vout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vdd33_ldo2_pd(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_vdd33_ldo2_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn en_vdd33_ldo3(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_en_vdd33_ldo3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vdd33_ldo3_set_vout(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_vdd33_ldo3_set_vout(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 17usize)) | (((val as u32) & 0x0f) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vdd33_ldo3_pd(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_vdd33_ldo3_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for PeriLdo {
    #[inline(always)]
    fn default() -> PeriLdo {
        PeriLdo(0)
    }
}
impl core::fmt::Debug for PeriLdo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PeriLdo")
            .field("en_ldo18", &self.en_ldo18())
            .field("ldo18_vref_sel", &self.ldo18_vref_sel())
            .field("ldo18_pd", &self.ldo18_pd())
            .field("en_vdd33_ldo2", &self.en_vdd33_ldo2())
            .field("vdd33_ldo2_set_vout", &self.vdd33_ldo2_set_vout())
            .field("vdd33_ldo2_pd", &self.vdd33_ldo2_pd())
            .field("en_vdd33_ldo3", &self.en_vdd33_ldo3())
            .field("vdd33_ldo3_set_vout", &self.vdd33_ldo3_set_vout())
            .field("vdd33_ldo3_pd", &self.vdd33_ldo3_pd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PeriLdo {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "PeriLdo {{ en_ldo18: {=bool:?}, ldo18_vref_sel: {=u8:?}, ldo18_pd: {=bool:?}, en_vdd33_ldo2: {=bool:?}, vdd33_ldo2_set_vout: {=u8:?}, vdd33_ldo2_pd: {=bool:?}, en_vdd33_ldo3: {=bool:?}, vdd33_ldo3_set_vout: {=u8:?}, vdd33_ldo3_pd: {=bool:?} }}" , self . en_ldo18 () , self . ldo18_vref_sel () , self . ldo18_pd () , self . en_vdd33_ldo2 () , self . vdd33_ldo2_set_vout () , self . vdd33_ldo2_pd () , self . en_vdd33_ldo3 () , self . vdd33_ldo3_set_vout () , self . vdd33_ldo3_pd ())
    }
}
#[doc = "PMU Reserved Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PmuRsvd(pub u32);
impl PmuRsvd {
    #[must_use]
    #[inline(always)]
    pub const fn reserve0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_reserve0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn reserve1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_reserve1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn reserve2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_reserve2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn reserve3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_reserve3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for PmuRsvd {
    #[inline(always)]
    fn default() -> PmuRsvd {
        PmuRsvd(0)
    }
}
impl core::fmt::Debug for PmuRsvd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PmuRsvd")
            .field("reserve0", &self.reserve0())
            .field("reserve1", &self.reserve1())
            .field("reserve2", &self.reserve2())
            .field("reserve3", &self.reserve3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PmuRsvd {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "PmuRsvd {{ reserve0: {=u8:?}, reserve1: {=u8:?}, reserve2: {=u8:?}, reserve3: {=u8:?} }}" , self . reserve0 () , self . reserve1 () , self . reserve2 () , self . reserve3 ())
    }
}
#[doc = "PMU Test Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PmuTr(pub u32);
impl PmuTr {
    #[doc = "test point select"]
    #[must_use]
    #[inline(always)]
    pub const fn pmu_dc_tr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "test point select"]
    #[inline(always)]
    pub const fn set_pmu_dc_tr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "macro select"]
    #[must_use]
    #[inline(always)]
    pub const fn pmu_dc_mr(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "macro select"]
    #[inline(always)]
    pub const fn set_pmu_dc_mr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
}
impl Default for PmuTr {
    #[inline(always)]
    fn default() -> PmuTr {
        PmuTr(0)
    }
}
impl core::fmt::Debug for PmuTr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PmuTr")
            .field("pmu_dc_tr", &self.pmu_dc_tr())
            .field("pmu_dc_mr", &self.pmu_dc_mr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PmuTr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PmuTr {{ pmu_dc_tr: {=u8:?}, pmu_dc_mr: {=u8:?} }}",
            self.pmu_dc_tr(),
            self.pmu_dc_mr()
        )
    }
}
#[doc = "PowerKey Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrkeyCnt(pub u32);
impl PwrkeyCnt {
    #[doc = "press high for RST_CNT*16 CLK_WDT cycles to reset the whole chip"]
    #[must_use]
    #[inline(always)]
    pub const fn rst_cnt(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0xffff;
        val as u16
    }
    #[doc = "press high for RST_CNT*16 CLK_WDT cycles to reset the whole chip"]
    #[inline(always)]
    pub const fn set_rst_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 4usize)) | (((val as u32) & 0xffff) << 4usize);
    }
}
impl Default for PwrkeyCnt {
    #[inline(always)]
    fn default() -> PwrkeyCnt {
        PwrkeyCnt(0)
    }
}
impl core::fmt::Debug for PwrkeyCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PwrkeyCnt")
            .field("rst_cnt", &self.rst_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PwrkeyCnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PwrkeyCnt {{ rst_cnt: {=u16:?} }}", self.rst_cnt())
    }
}
#[doc = "VRET Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VretCr(pub u32);
impl VretCr {
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn bm(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_bm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vbit(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_vbit(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn trim(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
    }
    #[doc = "VRET_LDO power up delay in number of CLK_LP cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn dly(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "VRET_LDO power up delay in number of CLK_LP cycles"]
    #[inline(always)]
    pub const fn set_dly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn rdy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for VretCr {
    #[inline(always)]
    fn default() -> VretCr {
        VretCr(0)
    }
}
impl core::fmt::Debug for VretCr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VretCr")
            .field("en", &self.en())
            .field("bm", &self.bm())
            .field("vbit", &self.vbit())
            .field("trim", &self.trim())
            .field("dly", &self.dly())
            .field("rdy", &self.rdy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VretCr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "VretCr {{ en: {=bool:?}, bm: {=bool:?}, vbit: {=u8:?}, trim: {=u8:?}, dly: {=u8:?}, rdy: {=bool:?} }}" , self . en () , self . bm () , self . vbit () , self . trim () , self . dly () , self . rdy ())
    }
}
#[doc = "VRTC Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VrtcCr(pub u32);
impl VrtcCr {
    #[must_use]
    #[inline(always)]
    pub const fn vrtc_vbit(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_vrtc_vbit(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn vrtc_trim(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_vrtc_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Brownout Reset Enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bor_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Brownout Reset Enable"]
    #[inline(always)]
    pub const fn set_bor_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn bor_vt_trim(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_bor_vt_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
    }
}
impl Default for VrtcCr {
    #[inline(always)]
    fn default() -> VrtcCr {
        VrtcCr(0)
    }
}
impl core::fmt::Debug for VrtcCr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VrtcCr")
            .field("vrtc_vbit", &self.vrtc_vbit())
            .field("vrtc_trim", &self.vrtc_trim())
            .field("bor_en", &self.bor_en())
            .field("bor_vt_trim", &self.bor_vt_trim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VrtcCr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "VrtcCr {{ vrtc_vbit: {=u8:?}, vrtc_trim: {=u8:?}, bor_en: {=bool:?}, bor_vt_trim: {=u8:?} }}" , self . vrtc_vbit () , self . vrtc_trim () , self . bor_en () , self . bor_vt_trim ())
    }
}
#[doc = "Wakeup Clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wcr(pub u32);
impl Wcr {
    #[doc = "Write 1 to clear WDT1 reboot flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wdt1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear WDT1 reboot flag"]
    #[inline(always)]
    pub const fn set_wdt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write 1 to clear WDT2 reboot flag"]
    #[must_use]
    #[inline(always)]
    pub const fn wdt2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear WDT2 reboot flag"]
    #[inline(always)]
    pub const fn set_wdt2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write 1 to clear PIN0 wakeup flag. Only valid if PIN wakeup is configured as edge trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn pin0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear PIN0 wakeup flag. Only valid if PIN wakeup is configured as edge trigger"]
    #[inline(always)]
    pub const fn set_pin0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write 1 to clear PIN1 wakeup flag."]
    #[must_use]
    #[inline(always)]
    pub const fn pin1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear PIN1 wakeup flag."]
    #[inline(always)]
    pub const fn set_pin1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Write 1 to clear PWRKEY reset flag"]
    #[must_use]
    #[inline(always)]
    pub const fn pwrkey(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear PWRKEY reset flag"]
    #[inline(always)]
    pub const fn set_pwrkey(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Write 1 to clear LOWBAT flag"]
    #[must_use]
    #[inline(always)]
    pub const fn lowbat(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear LOWBAT flag"]
    #[inline(always)]
    pub const fn set_lowbat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Write 1 to clear the AON wakeup IRQ status"]
    #[must_use]
    #[inline(always)]
    pub const fn aon(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear the AON wakeup IRQ status"]
    #[inline(always)]
    pub const fn set_aon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Wcr {
    #[inline(always)]
    fn default() -> Wcr {
        Wcr(0)
    }
}
impl core::fmt::Debug for Wcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wcr")
            .field("wdt1", &self.wdt1())
            .field("wdt2", &self.wdt2())
            .field("pin0", &self.pin0())
            .field("pin1", &self.pin1())
            .field("pwrkey", &self.pwrkey())
            .field("lowbat", &self.lowbat())
            .field("aon", &self.aon())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wcr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Wcr {{ wdt1: {=bool:?}, wdt2: {=bool:?}, pin0: {=bool:?}, pin1: {=bool:?}, pwrkey: {=bool:?}, lowbat: {=bool:?}, aon: {=bool:?} }}" , self . wdt1 () , self . wdt2 () , self . pin0 () , self . pin1 () , self . pwrkey () , self . lowbat () , self . aon ())
    }
}
#[doc = "Wakeup Enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wer(pub u32);
impl Wer {
    #[doc = "Set 1 to enable RTC as wakeup source"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable RTC as wakeup source"]
    #[inline(always)]
    pub const fn set_rtc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set 1 to enable WDT1 as reboot cause"]
    #[must_use]
    #[inline(always)]
    pub const fn wdt1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable WDT1 as reboot cause"]
    #[inline(always)]
    pub const fn set_wdt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set 1 to enable WDT2 as reboot cause"]
    #[must_use]
    #[inline(always)]
    pub const fn wdt2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable WDT2 as reboot cause"]
    #[inline(always)]
    pub const fn set_wdt2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set 1 to enable PIN0 as wakeup source"]
    #[must_use]
    #[inline(always)]
    pub const fn pin0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable PIN0 as wakeup source"]
    #[inline(always)]
    pub const fn set_pin0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set 1 to enable PIN1 as wakeup source"]
    #[must_use]
    #[inline(always)]
    pub const fn pin1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable PIN1 as wakeup source"]
    #[inline(always)]
    pub const fn set_pin1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If enabled, auto shut down upon battery low; and will power up if battery ready"]
    #[must_use]
    #[inline(always)]
    pub const fn lowbat(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If enabled, auto shut down upon battery low; and will power up if battery ready"]
    #[inline(always)]
    pub const fn set_lowbat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn chg(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_chg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Wer {
    #[inline(always)]
    fn default() -> Wer {
        Wer(0)
    }
}
impl core::fmt::Debug for Wer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wer")
            .field("rtc", &self.rtc())
            .field("wdt1", &self.wdt1())
            .field("wdt2", &self.wdt2())
            .field("pin0", &self.pin0())
            .field("pin1", &self.pin1())
            .field("lowbat", &self.lowbat())
            .field("chg", &self.chg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wer {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Wer {{ rtc: {=bool:?}, wdt1: {=bool:?}, wdt2: {=bool:?}, pin0: {=bool:?}, pin1: {=bool:?}, lowbat: {=bool:?}, chg: {=bool:?} }}" , self . rtc () , self . wdt1 () , self . wdt2 () , self . pin0 () , self . pin1 () , self . lowbat () , self . chg ())
    }
}
#[doc = "Wakeup Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WkupCnt(pub u32);
impl WkupCnt {
    #[must_use]
    #[inline(always)]
    pub const fn pin0_cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_pin0_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pin1_cnt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_pin1_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for WkupCnt {
    #[inline(always)]
    fn default() -> WkupCnt {
        WkupCnt(0)
    }
}
impl core::fmt::Debug for WkupCnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WkupCnt")
            .field("pin0_cnt", &self.pin0_cnt())
            .field("pin1_cnt", &self.pin1_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WkupCnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WkupCnt {{ pin0_cnt: {=u16:?}, pin1_cnt: {=u16:?} }}",
            self.pin0_cnt(),
            self.pin1_cnt()
        )
    }
}
#[doc = "Wakeup Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wsr(pub u32);
impl Wsr {
    #[doc = "Indicates the wakeup status from RTC. Note: the status is masked by WER"]
    #[must_use]
    #[inline(always)]
    pub const fn rtc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the wakeup status from RTC. Note: the status is masked by WER"]
    #[inline(always)]
    pub const fn set_rtc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates reboot by WDT1"]
    #[must_use]
    #[inline(always)]
    pub const fn wdt1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates reboot by WDT1"]
    #[inline(always)]
    pub const fn set_wdt1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates reboot by WDT2"]
    #[must_use]
    #[inline(always)]
    pub const fn wdt2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates reboot by WDT2"]
    #[inline(always)]
    pub const fn set_wdt2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pin0(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pin0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pin1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pin1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn iwdt(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_iwdt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pwrkey(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_pwrkey(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates auto reboot due to battery low"]
    #[must_use]
    #[inline(always)]
    pub const fn lowbat(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates auto reboot due to battery low"]
    #[inline(always)]
    pub const fn set_lowbat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn chg(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_chg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Wsr {
    #[inline(always)]
    fn default() -> Wsr {
        Wsr(0)
    }
}
impl core::fmt::Debug for Wsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wsr")
            .field("rtc", &self.rtc())
            .field("wdt1", &self.wdt1())
            .field("wdt2", &self.wdt2())
            .field("pin0", &self.pin0())
            .field("pin1", &self.pin1())
            .field("iwdt", &self.iwdt())
            .field("pwrkey", &self.pwrkey())
            .field("lowbat", &self.lowbat())
            .field("chg", &self.chg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wsr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Wsr {{ rtc: {=bool:?}, wdt1: {=bool:?}, wdt2: {=bool:?}, pin0: {=bool:?}, pin1: {=bool:?}, iwdt: {=bool:?}, pwrkey: {=bool:?}, lowbat: {=bool:?}, chg: {=bool:?} }}" , self . rtc () , self . wdt1 () , self . wdt2 () , self . pin0 () , self . pin1 () , self . iwdt () , self . pwrkey () , self . lowbat () , self . chg ())
    }
}
