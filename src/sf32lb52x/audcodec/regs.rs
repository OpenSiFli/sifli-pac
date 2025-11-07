#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc1Cfg1(pub u32);
impl Adc1Cfg1 {
    #[doc = "peripheral circuits biasmode"]
    #[must_use]
    #[inline(always)]
    pub const fn peri_bm(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "peripheral circuits biasmode"]
    #[inline(always)]
    pub const fn set_peri_bm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "inverse output clock"]
    #[must_use]
    #[inline(always)]
    pub const fn clkout_inv(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "inverse output clock"]
    #[inline(always)]
    pub const fn set_clkout_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VCM quick settling"]
    #[must_use]
    #[inline(always)]
    pub const fn vcmst(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VCM quick settling"]
    #[inline(always)]
    pub const fn set_vcmst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "chopping frequncy 0x0:÷8 0x1:÷16 0x2:÷32 0x3:÷64"]
    #[must_use]
    #[inline(always)]
    pub const fn fchop_sel(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "chopping frequncy 0x0:÷8 0x1:÷16 0x2:÷32 0x3:÷64"]
    #[inline(always)]
    pub const fn set_fchop_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "vref code from proper vcm in flash7 0x0:1.2V 0x1:1.4V 0x7:2.6V"]
    #[must_use]
    #[inline(always)]
    pub const fn vref_sel(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "vref code from proper vcm in flash7 0x0:1.2V 0x1:1.4V 0x7:2.6V"]
    #[inline(always)]
    pub const fn set_vref_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "bias mode of 2nd and 3rd opamp"]
    #[must_use]
    #[inline(always)]
    pub const fn bm_int2(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "bias mode of 2nd and 3rd opamp"]
    #[inline(always)]
    pub const fn set_bm_int2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "bias mode of first opamp"]
    #[must_use]
    #[inline(always)]
    pub const fn bm_int1(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "bias mode of first opamp"]
    #[inline(always)]
    pub const fn set_bm_int1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "start voltage 0x0:VCM+200mV 0x7:VCM+550mV"]
    #[must_use]
    #[inline(always)]
    pub const fn vst_sel(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "start voltage 0x0:VCM+200mV 0x7:VCM+550mV"]
    #[inline(always)]
    pub const fn set_vst_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "gaincode: 0x0:-6dB 0x1:0dB ... 0x4:18dB"]
    #[must_use]
    #[inline(always)]
    pub const fn gc(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "gaincode: 0x0:-6dB 0x1:0dB ... 0x4:18dB"]
    #[inline(always)]
    pub const fn set_gc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "enable negative DAC1"]
    #[must_use]
    #[inline(always)]
    pub const fn dacn_en(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "enable negative DAC1"]
    #[inline(always)]
    pub const fn set_dacn_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "enable differential input mode"]
    #[must_use]
    #[inline(always)]
    pub const fn diff_en(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "enable differential input mode"]
    #[inline(always)]
    pub const fn set_diff_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "sampling frequency: 0x0:9.6M 0x1:8.82M 0x2:4.8M 0x3:4.41M"]
    #[must_use]
    #[inline(always)]
    pub const fn fsp(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x03;
        val as u8
    }
    #[doc = "sampling frequency: 0x0:9.6M 0x1:8.82M 0x2:4.8M 0x3:4.41M"]
    #[inline(always)]
    pub const fn set_fsp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 23usize)) | (((val as u32) & 0x03) << 23usize);
    }
}
impl Default for Adc1Cfg1 {
    #[inline(always)]
    fn default() -> Adc1Cfg1 {
        Adc1Cfg1(0)
    }
}
impl core::fmt::Debug for Adc1Cfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adc1Cfg1")
            .field("peri_bm", &self.peri_bm())
            .field("clkout_inv", &self.clkout_inv())
            .field("vcmst", &self.vcmst())
            .field("fchop_sel", &self.fchop_sel())
            .field("vref_sel", &self.vref_sel())
            .field("bm_int2", &self.bm_int2())
            .field("bm_int1", &self.bm_int1())
            .field("vst_sel", &self.vst_sel())
            .field("gc", &self.gc())
            .field("dacn_en", &self.dacn_en())
            .field("diff_en", &self.diff_en())
            .field("fsp", &self.fsp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adc1Cfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Adc1Cfg1 {{ peri_bm: {=u8:?}, clkout_inv: {=bool:?}, vcmst: {=bool:?}, fchop_sel: {=u8:?}, vref_sel: {=u8:?}, bm_int2: {=u8:?}, bm_int1: {=u8:?}, vst_sel: {=u8:?}, gc: {=u8:?}, dacn_en: {=bool:?}, diff_en: {=bool:?}, fsp: {=u8:?} }}" , self . peri_bm () , self . clkout_inv () , self . vcmst () , self . fchop_sel () , self . vref_sel () , self . bm_int2 () , self . bm_int1 () , self . vst_sel () , self . gc () , self . dacn_en () , self . diff_en () , self . fsp ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc1Cfg2(pub u32);
impl Adc1Cfg2 {
    #[doc = "clear adc"]
    #[must_use]
    #[inline(always)]
    pub const fn clear(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "clear adc"]
    #[inline(always)]
    pub const fn set_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "chopping enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chop_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "chopping enable"]
    #[inline(always)]
    pub const fn set_chop_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "reset adc"]
    #[must_use]
    #[inline(always)]
    pub const fn rstb(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "reset adc"]
    #[inline(always)]
    pub const fn set_rstb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "enable adc"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "enable adc"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Adc1Cfg2 {
    #[inline(always)]
    fn default() -> Adc1Cfg2 {
        Adc1Cfg2(0)
    }
}
impl core::fmt::Debug for Adc1Cfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adc1Cfg2")
            .field("clear", &self.clear())
            .field("chop_en", &self.chop_en())
            .field("rstb", &self.rstb())
            .field("en", &self.en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adc1Cfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adc1Cfg2 {{ clear: {=bool:?}, chop_en: {=bool:?}, rstb: {=bool:?}, en: {=bool:?} }}",
            self.clear(),
            self.chop_en(),
            self.rstb(),
            self.en()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc2Cfg1(pub u32);
impl Adc2Cfg1 {
    #[doc = "peripheral circuits biasmode"]
    #[must_use]
    #[inline(always)]
    pub const fn peri_bm(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "peripheral circuits biasmode"]
    #[inline(always)]
    pub const fn set_peri_bm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "inverse output clock"]
    #[must_use]
    #[inline(always)]
    pub const fn clkout_inv(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "inverse output clock"]
    #[inline(always)]
    pub const fn set_clkout_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "VCM quick settling"]
    #[must_use]
    #[inline(always)]
    pub const fn vcmst(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "VCM quick settling"]
    #[inline(always)]
    pub const fn set_vcmst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "chopping frequncy 0x0:÷8 0x1:÷16 0x2:÷32 0x3:÷64"]
    #[must_use]
    #[inline(always)]
    pub const fn fchop_sel(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "chopping frequncy 0x0:÷8 0x1:÷16 0x2:÷32 0x3:÷64"]
    #[inline(always)]
    pub const fn set_fchop_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "vref code from proper vcm in flash7 0x0:1.2V 0x1:1.4V 0x7:2.6V"]
    #[must_use]
    #[inline(always)]
    pub const fn vref_sel(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "vref code from proper vcm in flash7 0x0:1.2V 0x1:1.4V 0x7:2.6V"]
    #[inline(always)]
    pub const fn set_vref_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "bias mode of 2nd and 3rd opamp"]
    #[must_use]
    #[inline(always)]
    pub const fn bm_int2(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "bias mode of 2nd and 3rd opamp"]
    #[inline(always)]
    pub const fn set_bm_int2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "bias mode of first opamp"]
    #[must_use]
    #[inline(always)]
    pub const fn bm_int1(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "bias mode of first opamp"]
    #[inline(always)]
    pub const fn set_bm_int1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "start voltage 0x0:VCM+200mV 0x7:VCM+550mV"]
    #[must_use]
    #[inline(always)]
    pub const fn vst_sel(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "start voltage 0x0:VCM+200mV 0x7:VCM+550mV"]
    #[inline(always)]
    pub const fn set_vst_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "gaincode: 0x0:-10dB 0xa:0dB 0x1e:20dB"]
    #[must_use]
    #[inline(always)]
    pub const fn gc(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x1f;
        val as u8
    }
    #[doc = "gaincode: 0x0:-10dB 0xa:0dB 0x1e:20dB"]
    #[inline(always)]
    pub const fn set_gc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
    }
    #[doc = "sampling frequency: 0x0:9.6M 0x1:8.82M 0x2:4.8M 0x3:4.41M"]
    #[must_use]
    #[inline(always)]
    pub const fn fsp(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x03;
        val as u8
    }
    #[doc = "sampling frequency: 0x0:9.6M 0x1:8.82M 0x2:4.8M 0x3:4.41M"]
    #[inline(always)]
    pub const fn set_fsp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 23usize)) | (((val as u32) & 0x03) << 23usize);
    }
}
impl Default for Adc2Cfg1 {
    #[inline(always)]
    fn default() -> Adc2Cfg1 {
        Adc2Cfg1(0)
    }
}
impl core::fmt::Debug for Adc2Cfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adc2Cfg1")
            .field("peri_bm", &self.peri_bm())
            .field("clkout_inv", &self.clkout_inv())
            .field("vcmst", &self.vcmst())
            .field("fchop_sel", &self.fchop_sel())
            .field("vref_sel", &self.vref_sel())
            .field("bm_int2", &self.bm_int2())
            .field("bm_int1", &self.bm_int1())
            .field("vst_sel", &self.vst_sel())
            .field("gc", &self.gc())
            .field("fsp", &self.fsp())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adc2Cfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Adc2Cfg1 {{ peri_bm: {=u8:?}, clkout_inv: {=bool:?}, vcmst: {=bool:?}, fchop_sel: {=u8:?}, vref_sel: {=u8:?}, bm_int2: {=u8:?}, bm_int1: {=u8:?}, vst_sel: {=u8:?}, gc: {=u8:?}, fsp: {=u8:?} }}" , self . peri_bm () , self . clkout_inv () , self . vcmst () , self . fchop_sel () , self . vref_sel () , self . bm_int2 () , self . bm_int1 () , self . vst_sel () , self . gc () , self . fsp ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc2Cfg2(pub u32);
impl Adc2Cfg2 {
    #[doc = "clear adc"]
    #[must_use]
    #[inline(always)]
    pub const fn clear(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "clear adc"]
    #[inline(always)]
    pub const fn set_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "chopping enable"]
    #[must_use]
    #[inline(always)]
    pub const fn chop_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "chopping enable"]
    #[inline(always)]
    pub const fn set_chop_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "reset adc"]
    #[must_use]
    #[inline(always)]
    pub const fn rstb(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "reset adc"]
    #[inline(always)]
    pub const fn set_rstb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "enable adc"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "enable adc"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Adc2Cfg2 {
    #[inline(always)]
    fn default() -> Adc2Cfg2 {
        Adc2Cfg2(0)
    }
}
impl core::fmt::Debug for Adc2Cfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Adc2Cfg2")
            .field("clear", &self.clear())
            .field("chop_en", &self.chop_en())
            .field("rstb", &self.rstb())
            .field("en", &self.en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Adc2Cfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Adc2Cfg2 {{ clear: {=bool:?}, chop_en: {=bool:?}, rstb: {=bool:?}, en: {=bool:?} }}",
            self.clear(),
            self.chop_en(),
            self.rstb(),
            self.en()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcAnaCfg(pub u32);
impl AdcAnaCfg {
    #[doc = "micbias chopping enable"]
    #[must_use]
    #[inline(always)]
    pub const fn micbias_chop_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "micbias chopping enable"]
    #[inline(always)]
    pub const fn set_micbias_chop_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "micbias enable"]
    #[must_use]
    #[inline(always)]
    pub const fn micbias_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "micbias enable"]
    #[inline(always)]
    pub const fn set_micbias_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "ADC cap code"]
    #[must_use]
    #[inline(always)]
    pub const fn capcode(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x1f;
        val as u8
    }
    #[doc = "ADC cap code"]
    #[inline(always)]
    pub const fn set_capcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u32) & 0x1f) << 2usize);
    }
}
impl Default for AdcAnaCfg {
    #[inline(always)]
    fn default() -> AdcAnaCfg {
        AdcAnaCfg(0)
    }
}
impl core::fmt::Debug for AdcAnaCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcAnaCfg")
            .field("micbias_chop_en", &self.micbias_chop_en())
            .field("micbias_en", &self.micbias_en())
            .field("capcode", &self.capcode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcAnaCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AdcAnaCfg {{ micbias_chop_en: {=bool:?}, micbias_en: {=bool:?}, capcode: {=u8:?} }}",
            self.micbias_chop_en(),
            self.micbias_en(),
            self.capcode()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcCfg(pub u32);
impl AdcCfg {
    #[doc = "ADC oversample rate 3'b000: 200 3'b001: 300 3'b010: 400 3'b011: 600 other: reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn osr_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "ADC oversample rate 3'b000: 200 3'b001: 300 3'b010: 400 3'b011: 600 other: reserved"]
    #[inline(always)]
    pub const fn set_osr_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "adc operation mode 2'h0: normal mode: send adc data out through rx interface 2'h1: apb mode: send adc data out through apb interface 2'h2: raw data apb mode: send adc raw data out through apb interface 2'h3: reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn op_mode(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "adc operation mode 2'h0: normal mode: send adc data out through rx interface 2'h1: apb mode: send adc data out through apb interface 2'h2: raw data apb mode: send adc raw data out through apb interface 2'h3: reserved"]
    #[inline(always)]
    pub const fn set_op_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
    #[doc = "adc path reset, set 1 to reset adc path"]
    #[must_use]
    #[inline(always)]
    pub const fn path_reset(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "adc path reset, set 1 to reset adc path"]
    #[inline(always)]
    pub const fn set_path_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "adc clock source select 1: pll 0: xtal"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_src_sel(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "adc clock source select 1: pll 0: xtal"]
    #[inline(always)]
    pub const fn set_clk_src_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "adc clock divider"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_div(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "adc clock divider"]
    #[inline(always)]
    pub const fn set_clk_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for AdcCfg {
    #[inline(always)]
    fn default() -> AdcCfg {
        AdcCfg(0)
    }
}
impl core::fmt::Debug for AdcCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcCfg")
            .field("osr_sel", &self.osr_sel())
            .field("op_mode", &self.op_mode())
            .field("path_reset", &self.path_reset())
            .field("clk_src_sel", &self.clk_src_sel())
            .field("clk_div", &self.clk_div())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "AdcCfg {{ osr_sel: {=u8:?}, op_mode: {=u8:?}, path_reset: {=bool:?}, clk_src_sel: {=bool:?}, clk_div: {=u8:?} }}" , self . osr_sel () , self . op_mode () , self . path_reset () , self . clk_src_sel () , self . clk_div ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcCh0Cfg(pub u32);
impl AdcCh0Cfg {
    #[doc = "adc channel enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "adc channel enable"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "high-pass filter bypass"]
    #[must_use]
    #[inline(always)]
    pub const fn hpf_bypass(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "high-pass filter bypass"]
    #[inline(always)]
    pub const fn set_hpf_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "high-pass filter coefficient"]
    #[must_use]
    #[inline(always)]
    pub const fn hpf_coef(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "high-pass filter coefficient"]
    #[inline(always)]
    pub const fn set_hpf_coef(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[doc = "adc strobe inverter"]
    #[must_use]
    #[inline(always)]
    pub const fn stb_inv(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "adc strobe inverter"]
    #[inline(always)]
    pub const fn set_stb_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "dma interface enable in apb mode and raw data apb mode 1: enable adc ch0 dma request interface 0: disable adc ch0 dma request interface"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "dma interface enable in apb mode and raw data apb mode 1: enable adc ch0 dma request interface 0: disable adc ch0 dma request interface"]
    #[inline(always)]
    pub const fn set_dma_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "adc rough volume control range from -60dB to 30dB step is 6dB 4'h0: -60dB 4'h1: -54dB ...... 4'ha: 0dB ...... 4'he: 24dB 4'hf: 30dB"]
    #[must_use]
    #[inline(always)]
    pub const fn rough_vol(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "adc rough volume control range from -60dB to 30dB step is 6dB 4'h0: -60dB 4'h1: -54dB ...... 4'ha: 0dB ...... 4'he: 24dB 4'hf: 30dB"]
    #[inline(always)]
    pub const fn set_rough_vol(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "adc fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute"]
    #[must_use]
    #[inline(always)]
    pub const fn fine_vol(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "adc fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute"]
    #[inline(always)]
    pub const fn set_fine_vol(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "adc data format 1: 16-bit 0: 24-bit this bit only affect the data format accessed by apb interface. For 24-bit, every 24-bit data occupied 32-bit word. Bit\\[31:24\\] are zeros. For 16-bit mode, every 32-bit word contains two 16-bit audio data{D1, D0}"]
    #[must_use]
    #[inline(always)]
    pub const fn data_format(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "adc data format 1: 16-bit 0: 24-bit this bit only affect the data format accessed by apb interface. For 24-bit, every 24-bit data occupied 32-bit word. Bit\\[31:24\\] are zeros. For 16-bit mode, every 32-bit word contains two 16-bit audio data{D1, D0}"]
    #[inline(always)]
    pub const fn set_data_format(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "adc saturation detect"]
    #[must_use]
    #[inline(always)]
    pub const fn sat_det_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "adc saturation detect"]
    #[inline(always)]
    pub const fn set_sat_det_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "adc saturation detect pattern length 2'b00: 16 2'b01: 24 2'b10: 32 2'b11: 48"]
    #[must_use]
    #[inline(always)]
    pub const fn sat_det_len(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "adc saturation detect pattern length 2'b00: 16 2'b01: 24 2'b10: 32 2'b11: 48"]
    #[inline(always)]
    pub const fn set_sat_det_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
}
impl Default for AdcCh0Cfg {
    #[inline(always)]
    fn default() -> AdcCh0Cfg {
        AdcCh0Cfg(0)
    }
}
impl core::fmt::Debug for AdcCh0Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcCh0Cfg")
            .field("enable", &self.enable())
            .field("hpf_bypass", &self.hpf_bypass())
            .field("hpf_coef", &self.hpf_coef())
            .field("stb_inv", &self.stb_inv())
            .field("dma_en", &self.dma_en())
            .field("rough_vol", &self.rough_vol())
            .field("fine_vol", &self.fine_vol())
            .field("data_format", &self.data_format())
            .field("sat_det_en", &self.sat_det_en())
            .field("sat_det_len", &self.sat_det_len())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcCh0Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "AdcCh0Cfg {{ enable: {=bool:?}, hpf_bypass: {=bool:?}, hpf_coef: {=u8:?}, stb_inv: {=bool:?}, dma_en: {=bool:?}, rough_vol: {=u8:?}, fine_vol: {=u8:?}, data_format: {=bool:?}, sat_det_en: {=bool:?}, sat_det_len: {=u8:?} }}" , self . enable () , self . hpf_bypass () , self . hpf_coef () , self . stb_inv () , self . dma_en () , self . rough_vol () , self . fine_vol () , self . data_format () , self . sat_det_en () , self . sat_det_len ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcCh0Entry(pub u32);
impl AdcCh0Entry {
    #[doc = "adc channel0 data output"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "adc channel0 data output"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AdcCh0Entry {
    #[inline(always)]
    fn default() -> AdcCh0Entry {
        AdcCh0Entry(0)
    }
}
impl core::fmt::Debug for AdcCh0Entry {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcCh0Entry")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcCh0Entry {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AdcCh0Entry {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcCh1Cfg(pub u32);
impl AdcCh1Cfg {
    #[doc = "adc channel enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "adc channel enable"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "high-pass filter bypass"]
    #[must_use]
    #[inline(always)]
    pub const fn hpf_bypass(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "high-pass filter bypass"]
    #[inline(always)]
    pub const fn set_hpf_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "high-pass filter coefficient"]
    #[must_use]
    #[inline(always)]
    pub const fn hpf_coef(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "high-pass filter coefficient"]
    #[inline(always)]
    pub const fn set_hpf_coef(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[doc = "adc strobe inverter"]
    #[must_use]
    #[inline(always)]
    pub const fn stb_inv(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "adc strobe inverter"]
    #[inline(always)]
    pub const fn set_stb_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "dma interface enable in apb mode and raw data apb mode 1: enable adc ch0 dma request interface 0: disable adc ch0 dma request interface"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "dma interface enable in apb mode and raw data apb mode 1: enable adc ch0 dma request interface 0: disable adc ch0 dma request interface"]
    #[inline(always)]
    pub const fn set_dma_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "adc rough volume control range from -60dB to 30dB step is 6dB 4'h0: -60dB 4'h1: -54dB ...... 4'ha: 0dB ...... 4'he: 24dB 4'hf: 30dB"]
    #[must_use]
    #[inline(always)]
    pub const fn rough_vol(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "adc rough volume control range from -60dB to 30dB step is 6dB 4'h0: -60dB 4'h1: -54dB ...... 4'ha: 0dB ...... 4'he: 24dB 4'hf: 30dB"]
    #[inline(always)]
    pub const fn set_rough_vol(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "adc fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute"]
    #[must_use]
    #[inline(always)]
    pub const fn fine_vol(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "adc fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute"]
    #[inline(always)]
    pub const fn set_fine_vol(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "adc data format 1: 16-bit 0: 24-bit this bit only affect the data format accessed by apb interface. For 24-bit, every 24-bit data occupied 32-bit word. Bit\\[31:24\\] are zeros. For 16-bit mode, every 32-bit word contains two 16-bit audio data{D1, D0}"]
    #[must_use]
    #[inline(always)]
    pub const fn data_format(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "adc data format 1: 16-bit 0: 24-bit this bit only affect the data format accessed by apb interface. For 24-bit, every 24-bit data occupied 32-bit word. Bit\\[31:24\\] are zeros. For 16-bit mode, every 32-bit word contains two 16-bit audio data{D1, D0}"]
    #[inline(always)]
    pub const fn set_data_format(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "adc saturation detect"]
    #[must_use]
    #[inline(always)]
    pub const fn sat_det_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "adc saturation detect"]
    #[inline(always)]
    pub const fn set_sat_det_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "adc saturation detect pattern length 2'b00: 16 2'b01: 24 2'b10: 32 2'b11: 48"]
    #[must_use]
    #[inline(always)]
    pub const fn sat_det_len(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "adc saturation detect pattern length 2'b00: 16 2'b01: 24 2'b10: 32 2'b11: 48"]
    #[inline(always)]
    pub const fn set_sat_det_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
}
impl Default for AdcCh1Cfg {
    #[inline(always)]
    fn default() -> AdcCh1Cfg {
        AdcCh1Cfg(0)
    }
}
impl core::fmt::Debug for AdcCh1Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcCh1Cfg")
            .field("enable", &self.enable())
            .field("hpf_bypass", &self.hpf_bypass())
            .field("hpf_coef", &self.hpf_coef())
            .field("stb_inv", &self.stb_inv())
            .field("dma_en", &self.dma_en())
            .field("rough_vol", &self.rough_vol())
            .field("fine_vol", &self.fine_vol())
            .field("data_format", &self.data_format())
            .field("sat_det_en", &self.sat_det_en())
            .field("sat_det_len", &self.sat_det_len())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcCh1Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "AdcCh1Cfg {{ enable: {=bool:?}, hpf_bypass: {=bool:?}, hpf_coef: {=u8:?}, stb_inv: {=bool:?}, dma_en: {=bool:?}, rough_vol: {=u8:?}, fine_vol: {=u8:?}, data_format: {=bool:?}, sat_det_en: {=bool:?}, sat_det_len: {=u8:?} }}" , self . enable () , self . hpf_bypass () , self . hpf_coef () , self . stb_inv () , self . dma_en () , self . rough_vol () , self . fine_vol () , self . data_format () , self . sat_det_en () , self . sat_det_len ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcCh1Entry(pub u32);
impl AdcCh1Entry {
    #[doc = "adc channel1 data output"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "adc channel1 data output"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for AdcCh1Entry {
    #[inline(always)]
    fn default() -> AdcCh1Entry {
        AdcCh1Entry(0)
    }
}
impl core::fmt::Debug for AdcCh1Entry {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcCh1Entry")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcCh1Entry {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AdcCh1Entry {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ApbStat(pub u32);
impl ApbStat {
    #[must_use]
    #[inline(always)]
    pub const fn dac_ch0_fifo_cnt(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_dac_ch0_fifo_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dac_ch1_fifo_cnt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_dac_ch1_fifo_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_ch0_fifo_cnt(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_adc_ch0_fifo_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn adc_ch1_fifo_cnt(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_adc_ch1_fifo_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
}
impl Default for ApbStat {
    #[inline(always)]
    fn default() -> ApbStat {
        ApbStat(0)
    }
}
impl core::fmt::Debug for ApbStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ApbStat")
            .field("dac_ch0_fifo_cnt", &self.dac_ch0_fifo_cnt())
            .field("dac_ch1_fifo_cnt", &self.dac_ch1_fifo_cnt())
            .field("adc_ch0_fifo_cnt", &self.adc_ch0_fifo_cnt())
            .field("adc_ch1_fifo_cnt", &self.adc_ch1_fifo_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ApbStat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "ApbStat {{ dac_ch0_fifo_cnt: {=u8:?}, dac_ch1_fifo_cnt: {=u8:?}, adc_ch0_fifo_cnt: {=u8:?}, adc_ch1_fifo_cnt: {=u8:?} }}" , self . dac_ch0_fifo_cnt () , self . dac_ch1_fifo_cnt () , self . adc_ch0_fifo_cnt () , self . adc_ch1_fifo_cnt ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgCfg0(pub u32);
impl BgCfg0 {
    #[doc = "enable bandgap"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "enable bandgap"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1: bandgap lp mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_mode(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1: bandgap lp mode"]
    #[inline(always)]
    pub const fn set_lp_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "set vref, 12: 2.2V"]
    #[must_use]
    #[inline(always)]
    pub const fn vref_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "set vref, 12: 2.2V"]
    #[inline(always)]
    pub const fn set_vref_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[doc = "enable bandgap chop"]
    #[must_use]
    #[inline(always)]
    pub const fn en_chop(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "enable bandgap chop"]
    #[inline(always)]
    pub const fn set_en_chop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "enable bandgap sample"]
    #[must_use]
    #[inline(always)]
    pub const fn en_smpl(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "enable bandgap sample"]
    #[inline(always)]
    pub const fn set_en_smpl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "enable bandgap rc filter"]
    #[must_use]
    #[inline(always)]
    pub const fn en_rcflt(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "enable bandgap rc filter"]
    #[inline(always)]
    pub const fn set_en_rcflt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "select mic vref"]
    #[must_use]
    #[inline(always)]
    pub const fn mic_vref_sel(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "select mic vref"]
    #[inline(always)]
    pub const fn set_mic_vref_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "enable bg opamp"]
    #[must_use]
    #[inline(always)]
    pub const fn en_amp(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "enable bg opamp"]
    #[inline(always)]
    pub const fn set_en_amp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "set vc"]
    #[must_use]
    #[inline(always)]
    pub const fn set_vc(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "set vc"]
    #[inline(always)]
    pub const fn set_set_vc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for BgCfg0 {
    #[inline(always)]
    fn default() -> BgCfg0 {
        BgCfg0(0)
    }
}
impl core::fmt::Debug for BgCfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BgCfg0")
            .field("en", &self.en())
            .field("lp_mode", &self.lp_mode())
            .field("vref_sel", &self.vref_sel())
            .field("en_chop", &self.en_chop())
            .field("en_smpl", &self.en_smpl())
            .field("en_rcflt", &self.en_rcflt())
            .field("mic_vref_sel", &self.mic_vref_sel())
            .field("en_amp", &self.en_amp())
            .field("set_vc", &self.set_vc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BgCfg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "BgCfg0 {{ en: {=bool:?}, lp_mode: {=bool:?}, vref_sel: {=u8:?}, en_chop: {=bool:?}, en_smpl: {=bool:?}, en_rcflt: {=bool:?}, mic_vref_sel: {=u8:?}, en_amp: {=bool:?}, set_vc: {=bool:?} }}" , self . en () , self . lp_mode () , self . vref_sel () , self . en_chop () , self . en_smpl () , self . en_rcflt () , self . mic_vref_sel () , self . en_amp () , self . set_vc ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgCfg1(pub u32);
impl BgCfg1 {
    #[doc = "bg sample clock high cycle width, based on 0: stop bg sample clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sampclk_hi(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "bg sample clock high cycle width, based on 0: stop bg sample clock"]
    #[inline(always)]
    pub const fn set_sampclk_hi(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BgCfg1 {
    #[inline(always)]
    fn default() -> BgCfg1 {
        BgCfg1(0)
    }
}
impl core::fmt::Debug for BgCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BgCfg1")
            .field("sampclk_hi", &self.sampclk_hi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BgCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BgCfg1 {{ sampclk_hi: {=u32:?} }}", self.sampclk_hi())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgCfg2(pub u32);
impl BgCfg2 {
    #[doc = "bg sample clock low cycle width. 0: stop bg sample clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sampclk_lo(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "bg sample clock low cycle width. 0: stop bg sample clock"]
    #[inline(always)]
    pub const fn set_sampclk_lo(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for BgCfg2 {
    #[inline(always)]
    fn default() -> BgCfg2 {
        BgCfg2(0)
    }
}
impl core::fmt::Debug for BgCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BgCfg2")
            .field("sampclk_lo", &self.sampclk_lo())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BgCfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BgCfg2 {{ sampclk_lo: {=u32:?} }}", self.sampclk_lo())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "adc codec enable"]
    #[must_use]
    #[inline(always)]
    pub const fn adc_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "adc codec enable"]
    #[inline(always)]
    pub const fn set_adc_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "dac codec enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dac_enable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "dac codec enable"]
    #[inline(always)]
    pub const fn set_dac_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "codec dac sine 1k mode"]
    #[must_use]
    #[inline(always)]
    pub const fn dac_1k_mode(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "codec dac sine 1k mode"]
    #[inline(always)]
    pub const fn set_dac_1k_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "codec adc enable delay count 0: no delay 1: 32 pclk 2: 64 pclk 3: 128 pclk"]
    #[must_use]
    #[inline(always)]
    pub const fn adc_en_dly_sel(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x03;
        val as u8
    }
    #[doc = "codec adc enable delay count 0: no delay 1: 32 pclk 2: 64 pclk 3: 128 pclk"]
    #[inline(always)]
    pub const fn set_adc_en_dly_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
    }
}
impl Default for Cfg {
    #[inline(always)]
    fn default() -> Cfg {
        Cfg(0)
    }
}
impl core::fmt::Debug for Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg")
            .field("adc_enable", &self.adc_enable())
            .field("dac_enable", &self.dac_enable())
            .field("dac_1k_mode", &self.dac_1k_mode())
            .field("adc_en_dly_sel", &self.adc_en_dly_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cfg {{ adc_enable: {=bool:?}, dac_enable: {=bool:?}, dac_1k_mode: {=bool:?}, adc_en_dly_sel: {=u8:?} }}" , self . adc_enable () , self . dac_enable () , self . dac_1k_mode () , self . adc_en_dly_sel ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CommonCfg(pub u32);
impl CommonCfg {
    #[doc = "DC test point select"]
    #[must_use]
    #[inline(always)]
    pub const fn dc_tr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "DC test point select"]
    #[inline(always)]
    pub const fn set_dc_tr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "DC test Block select"]
    #[must_use]
    #[inline(always)]
    pub const fn dc_br(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "DC test Block select"]
    #[inline(always)]
    pub const fn set_dc_br(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "DC test Macro select"]
    #[must_use]
    #[inline(always)]
    pub const fn dc_mr(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "DC test Macro select"]
    #[inline(always)]
    pub const fn set_dc_mr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
}
impl Default for CommonCfg {
    #[inline(always)]
    fn default() -> CommonCfg {
        CommonCfg(0)
    }
}
impl core::fmt::Debug for CommonCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CommonCfg")
            .field("dc_tr", &self.dc_tr())
            .field("dc_br", &self.dc_br())
            .field("dc_mr", &self.dc_mr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CommonCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CommonCfg {{ dc_tr: {=u8:?}, dc_br: {=u8:?}, dc_mr: {=u8:?} }}",
            self.dc_tr(),
            self.dc_br(),
            self.dc_mr()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac1Cfg(pub u32);
impl Dac1Cfg {
    #[doc = "enable os dac"]
    #[must_use]
    #[inline(always)]
    pub const fn en_os_dac(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "enable os dac"]
    #[inline(always)]
    pub const fn set_en_os_dac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "os dac"]
    #[must_use]
    #[inline(always)]
    pub const fn os_dac(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "os dac"]
    #[inline(always)]
    pub const fn set_os_dac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "dac gain"]
    #[must_use]
    #[inline(always)]
    pub const fn gain(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "dac gain"]
    #[inline(always)]
    pub const fn set_gain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "dac short switch"]
    #[must_use]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "dac short switch"]
    #[inline(always)]
    pub const fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "dac clk polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pol_clk(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "dac clk polarity"]
    #[inline(always)]
    pub const fn set_pol_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "0: 3.3V sup, 1: 1.8V supply"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_mode(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "0: 3.3V sup, 1: 1.8V supply"]
    #[inline(always)]
    pub const fn set_lp_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "select vcm"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_vcm(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "select vcm"]
    #[inline(always)]
    pub const fn set_sel_vcm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "bias mode"]
    #[must_use]
    #[inline(always)]
    pub const fn bm(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "bias mode"]
    #[inline(always)]
    pub const fn set_bm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "enable chop"]
    #[must_use]
    #[inline(always)]
    pub const fn en_chop(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "enable chop"]
    #[inline(always)]
    pub const fn set_en_chop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "enable amp"]
    #[must_use]
    #[inline(always)]
    pub const fn en_amp(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "enable amp"]
    #[inline(always)]
    pub const fn set_en_amp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "enable vcm"]
    #[must_use]
    #[inline(always)]
    pub const fn en_vcm(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "enable vcm"]
    #[inline(always)]
    pub const fn set_en_vcm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "enable dac"]
    #[must_use]
    #[inline(always)]
    pub const fn en_dac(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "enable dac"]
    #[inline(always)]
    pub const fn set_en_dac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "select Vstart"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_vstart(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "select Vstart"]
    #[inline(always)]
    pub const fn set_sel_vstart(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
}
impl Default for Dac1Cfg {
    #[inline(always)]
    fn default() -> Dac1Cfg {
        Dac1Cfg(0)
    }
}
impl core::fmt::Debug for Dac1Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dac1Cfg")
            .field("en_os_dac", &self.en_os_dac())
            .field("os_dac", &self.os_dac())
            .field("gain", &self.gain())
            .field("sr", &self.sr())
            .field("pol_clk", &self.pol_clk())
            .field("lp_mode", &self.lp_mode())
            .field("sel_vcm", &self.sel_vcm())
            .field("bm", &self.bm())
            .field("en_chop", &self.en_chop())
            .field("en_amp", &self.en_amp())
            .field("en_vcm", &self.en_vcm())
            .field("en_dac", &self.en_dac())
            .field("sel_vstart", &self.sel_vstart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dac1Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dac1Cfg {{ en_os_dac: {=bool:?}, os_dac: {=u8:?}, gain: {=u8:?}, sr: {=bool:?}, pol_clk: {=bool:?}, lp_mode: {=bool:?}, sel_vcm: {=u8:?}, bm: {=u8:?}, en_chop: {=bool:?}, en_amp: {=bool:?}, en_vcm: {=bool:?}, en_dac: {=bool:?}, sel_vstart: {=u8:?} }}" , self . en_os_dac () , self . os_dac () , self . gain () , self . sr () , self . pol_clk () , self . lp_mode () , self . sel_vcm () , self . bm () , self . en_chop () , self . en_amp () , self . en_vcm () , self . en_dac () , self . sel_vstart ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac2Cfg(pub u32);
impl Dac2Cfg {
    #[doc = "enable os dac"]
    #[must_use]
    #[inline(always)]
    pub const fn en_os_dac(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "enable os dac"]
    #[inline(always)]
    pub const fn set_en_os_dac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "os dac"]
    #[must_use]
    #[inline(always)]
    pub const fn os_dac(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "os dac"]
    #[inline(always)]
    pub const fn set_os_dac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "dac gain"]
    #[must_use]
    #[inline(always)]
    pub const fn gain(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "dac gain"]
    #[inline(always)]
    pub const fn set_gain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "dac short switch"]
    #[must_use]
    #[inline(always)]
    pub const fn sr(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "dac short switch"]
    #[inline(always)]
    pub const fn set_sr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "dac clk polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn pol_clk(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "dac clk polarity"]
    #[inline(always)]
    pub const fn set_pol_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "0: 3.3V sup, 1: 1.8V supply"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_mode(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "0: 3.3V sup, 1: 1.8V supply"]
    #[inline(always)]
    pub const fn set_lp_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "select vcm"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_vcm(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "select vcm"]
    #[inline(always)]
    pub const fn set_sel_vcm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "bias mode"]
    #[must_use]
    #[inline(always)]
    pub const fn bm(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[doc = "bias mode"]
    #[inline(always)]
    pub const fn set_bm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[doc = "enable chop"]
    #[must_use]
    #[inline(always)]
    pub const fn en_chop(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "enable chop"]
    #[inline(always)]
    pub const fn set_en_chop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "enable amp"]
    #[must_use]
    #[inline(always)]
    pub const fn en_amp(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "enable amp"]
    #[inline(always)]
    pub const fn set_en_amp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "enable vcm"]
    #[must_use]
    #[inline(always)]
    pub const fn en_vcm(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "enable vcm"]
    #[inline(always)]
    pub const fn set_en_vcm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "enable dac"]
    #[must_use]
    #[inline(always)]
    pub const fn en_dac(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "enable dac"]
    #[inline(always)]
    pub const fn set_en_dac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "select Vstart"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_vstart(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "select Vstart"]
    #[inline(always)]
    pub const fn set_sel_vstart(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
}
impl Default for Dac2Cfg {
    #[inline(always)]
    fn default() -> Dac2Cfg {
        Dac2Cfg(0)
    }
}
impl core::fmt::Debug for Dac2Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dac2Cfg")
            .field("en_os_dac", &self.en_os_dac())
            .field("os_dac", &self.os_dac())
            .field("gain", &self.gain())
            .field("sr", &self.sr())
            .field("pol_clk", &self.pol_clk())
            .field("lp_mode", &self.lp_mode())
            .field("sel_vcm", &self.sel_vcm())
            .field("bm", &self.bm())
            .field("en_chop", &self.en_chop())
            .field("en_amp", &self.en_amp())
            .field("en_vcm", &self.en_vcm())
            .field("en_dac", &self.en_dac())
            .field("sel_vstart", &self.sel_vstart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dac2Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dac2Cfg {{ en_os_dac: {=bool:?}, os_dac: {=u8:?}, gain: {=u8:?}, sr: {=bool:?}, pol_clk: {=bool:?}, lp_mode: {=bool:?}, sel_vcm: {=u8:?}, bm: {=u8:?}, en_chop: {=bool:?}, en_amp: {=bool:?}, en_vcm: {=bool:?}, en_dac: {=bool:?}, sel_vstart: {=u8:?} }}" , self . en_os_dac () , self . os_dac () , self . gain () , self . sr () , self . pol_clk () , self . lp_mode () , self . sel_vcm () , self . bm () , self . en_chop () , self . en_amp () , self . en_vcm () , self . en_dac () , self . sel_vstart ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacCfg(pub u32);
impl DacCfg {
    #[doc = "DAC oversample rate 4'b0000: 100 4'b0001: 150 4'b0010: 200 4'b0011: 300(sdm osr = 150) 4'b0100: 300(sdm osr = 300) 4'b0101: 400 4'b0110: 600 4'b0111: 800 4'b1000: 1200 4'b1001: 256 4'b1010: 512 4'b1011: 1024 other: reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn osr_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "DAC oversample rate 4'b0000: 100 4'b0001: 150 4'b0010: 200 4'b0011: 300(sdm osr = 150) 4'b0100: 300(sdm osr = 300) 4'b0101: 400 4'b0110: 600 4'b0111: 800 4'b1000: 1200 4'b1001: 256 4'b1010: 512 4'b1011: 1024 other: reserved"]
    #[inline(always)]
    pub const fn set_osr_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "dac operation mode 2'h0: normal mode: send dac data through tx interface 2'h1: apb mode: send dac data out through apb interface 2'h2, 2'h3: reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn op_mode(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "dac operation mode 2'h0: normal mode: send dac data through tx interface 2'h1: apb mode: send dac data out through apb interface 2'h2, 2'h3: reserved"]
    #[inline(always)]
    pub const fn set_op_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "dac path reset, set 1 to reset dac path"]
    #[must_use]
    #[inline(always)]
    pub const fn path_reset(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "dac path reset, set 1 to reset dac path"]
    #[inline(always)]
    pub const fn set_path_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "dac clock source select 1: pll 0: xtal"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_src_sel(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "dac clock source select 1: pll 0: xtal"]
    #[inline(always)]
    pub const fn set_clk_src_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "dac clock divider"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_div(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "dac clock divider"]
    #[inline(always)]
    pub const fn set_clk_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "set 1 to manually set hbf, interp3, sinc and sdm module"]
    #[must_use]
    #[inline(always)]
    pub const fn manual_osr_mode(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "set 1 to manually set hbf, interp3, sinc and sdm module"]
    #[inline(always)]
    pub const fn set_manual_osr_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn hbf1_bypass_m(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_hbf1_bypass_m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn hbf2_bypass_m(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_hbf2_bypass_m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn hbf3_bypass_m(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_hbf3_bypass_m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn hbf4_bypass_m(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_hbf4_bypass_m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn interp3_bypass_m(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_interp3_bypass_m(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "0:25 1:50 2:16 3:32 4:64"]
    #[must_use]
    #[inline(always)]
    pub const fn sinc_rate_sel_m(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x07;
        val as u8
    }
    #[doc = "0:25 1:50 2:16 3:32 4:64"]
    #[inline(always)]
    pub const fn set_sinc_rate_sel_m(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
    }
    #[doc = "0:100 1:150 2:300 3:256"]
    #[must_use]
    #[inline(always)]
    pub const fn sdm_osr_sel_m(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x03;
        val as u8
    }
    #[doc = "0:100 1:150 2:300 3:256"]
    #[inline(always)]
    pub const fn set_sdm_osr_sel_m(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
    }
}
impl Default for DacCfg {
    #[inline(always)]
    fn default() -> DacCfg {
        DacCfg(0)
    }
}
impl core::fmt::Debug for DacCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacCfg")
            .field("osr_sel", &self.osr_sel())
            .field("op_mode", &self.op_mode())
            .field("path_reset", &self.path_reset())
            .field("clk_src_sel", &self.clk_src_sel())
            .field("clk_div", &self.clk_div())
            .field("manual_osr_mode", &self.manual_osr_mode())
            .field("hbf1_bypass_m", &self.hbf1_bypass_m())
            .field("hbf2_bypass_m", &self.hbf2_bypass_m())
            .field("hbf3_bypass_m", &self.hbf3_bypass_m())
            .field("hbf4_bypass_m", &self.hbf4_bypass_m())
            .field("interp3_bypass_m", &self.interp3_bypass_m())
            .field("sinc_rate_sel_m", &self.sinc_rate_sel_m())
            .field("sdm_osr_sel_m", &self.sdm_osr_sel_m())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "DacCfg {{ osr_sel: {=u8:?}, op_mode: {=u8:?}, path_reset: {=bool:?}, clk_src_sel: {=bool:?}, clk_div: {=u8:?}, manual_osr_mode: {=bool:?}, hbf1_bypass_m: {=bool:?}, hbf2_bypass_m: {=bool:?}, hbf3_bypass_m: {=bool:?}, hbf4_bypass_m: {=bool:?}, interp3_bypass_m: {=bool:?}, sinc_rate_sel_m: {=u8:?}, sdm_osr_sel_m: {=u8:?} }}" , self . osr_sel () , self . op_mode () , self . path_reset () , self . clk_src_sel () , self . clk_div () , self . manual_osr_mode () , self . hbf1_bypass_m () , self . hbf2_bypass_m () , self . hbf3_bypass_m () , self . hbf4_bypass_m () , self . interp3_bypass_m () , self . sinc_rate_sel_m () , self . sdm_osr_sel_m ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacCh0Cfg(pub u32);
impl DacCh0Cfg {
    #[doc = "dac channel enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "dac channel enable"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "dac output mute, set 1 to mute the output"]
    #[must_use]
    #[inline(always)]
    pub const fn dout_mute(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "dac output mute, set 1 to mute the output"]
    #[inline(always)]
    pub const fn set_dout_mute(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "dem output mode 2'h0: no shift for dem output 2'h1: shift dem output incrementally 2'h2: shift dem output according to input 2'h3: reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn dem_mode(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "dem output mode 2'h0: no shift for dem output 2'h1: shift dem output incrementally 2'h2: shift dem output according to input 2'h3: reserved"]
    #[inline(always)]
    pub const fn set_dem_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "dac input stb fifo cnt"]
    #[must_use]
    #[inline(always)]
    pub const fn stb_fifo_cnt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "dac input stb fifo cnt"]
    #[inline(always)]
    pub const fn set_stb_fifo_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "dma interface enable in apb mode 1: enable dac ch0 dma request interface 0: disable dac ch0 dma request interface"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "dma interface enable in apb mode 1: enable dac ch0 dma request interface 0: disable dac ch0 dma request interface"]
    #[inline(always)]
    pub const fn set_dma_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "dac rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB"]
    #[must_use]
    #[inline(always)]
    pub const fn rough_vol(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "dac rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB"]
    #[inline(always)]
    pub const fn set_rough_vol(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "dac fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute"]
    #[must_use]
    #[inline(always)]
    pub const fn fine_vol(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "dac fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute"]
    #[inline(always)]
    pub const fn set_fine_vol(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "dac data format 1: 16-bit 0: 24-bit this bit only affect the data format accessed by apb interface. For 24-bit, every 24-bit data occupied 32-bit word. Bit\\[31:24\\] are zeros. For 16-bit mode, every 32-bit word contains two 16-bit audio data{D1, D0}"]
    #[must_use]
    #[inline(always)]
    pub const fn data_format(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "dac data format 1: 16-bit 0: 24-bit this bit only affect the data format accessed by apb interface. For 24-bit, every 24-bit data occupied 32-bit word. Bit\\[31:24\\] are zeros. For 16-bit mode, every 32-bit word contains two 16-bit audio data{D1, D0}"]
    #[inline(always)]
    pub const fn set_data_format(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "dac sinc filter gain"]
    #[must_use]
    #[inline(always)]
    pub const fn sinc_gain(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x01ff;
        val as u16
    }
    #[doc = "dac sinc filter gain"]
    #[inline(always)]
    pub const fn set_sinc_gain(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 17usize)) | (((val as u32) & 0x01ff) << 17usize);
    }
    #[doc = "sdm dither gain"]
    #[must_use]
    #[inline(always)]
    pub const fn dither_gain(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x07;
        val as u8
    }
    #[doc = "sdm dither gain"]
    #[inline(always)]
    pub const fn set_dither_gain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val as u32) & 0x07) << 26usize);
    }
    #[doc = "sdm dither enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dither_en(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "sdm dither enable"]
    #[inline(always)]
    pub const fn set_dither_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "analog dac clock polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_ana_pol(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "analog dac clock polarity"]
    #[inline(always)]
    pub const fn set_clk_ana_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for DacCh0Cfg {
    #[inline(always)]
    fn default() -> DacCh0Cfg {
        DacCh0Cfg(0)
    }
}
impl core::fmt::Debug for DacCh0Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacCh0Cfg")
            .field("enable", &self.enable())
            .field("dout_mute", &self.dout_mute())
            .field("dem_mode", &self.dem_mode())
            .field("stb_fifo_cnt", &self.stb_fifo_cnt())
            .field("dma_en", &self.dma_en())
            .field("rough_vol", &self.rough_vol())
            .field("fine_vol", &self.fine_vol())
            .field("data_format", &self.data_format())
            .field("sinc_gain", &self.sinc_gain())
            .field("dither_gain", &self.dither_gain())
            .field("dither_en", &self.dither_en())
            .field("clk_ana_pol", &self.clk_ana_pol())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacCh0Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "DacCh0Cfg {{ enable: {=bool:?}, dout_mute: {=bool:?}, dem_mode: {=u8:?}, stb_fifo_cnt: {=u8:?}, dma_en: {=bool:?}, rough_vol: {=u8:?}, fine_vol: {=u8:?}, data_format: {=bool:?}, sinc_gain: {=u16:?}, dither_gain: {=u8:?}, dither_en: {=bool:?}, clk_ana_pol: {=bool:?} }}" , self . enable () , self . dout_mute () , self . dem_mode () , self . stb_fifo_cnt () , self . dma_en () , self . rough_vol () , self . fine_vol () , self . data_format () , self . sinc_gain () , self . dither_gain () , self . dither_en () , self . clk_ana_pol ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacCh0CfgExt(pub u32);
impl DacCh0CfgExt {
    #[doc = "volume ramp enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ramp_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "volume ramp enable"]
    #[inline(always)]
    pub const fn set_ramp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "volume ramp mode: 1: slowly ramp to target volume. Step is 0.5db 0: directly ramp to target volume."]
    #[must_use]
    #[inline(always)]
    pub const fn ramp_mode(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "volume ramp mode: 1: slowly ramp to target volume. Step is 0.5db 0: directly ramp to target volume."]
    #[inline(always)]
    pub const fn set_ramp_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "enable volume adjustment during 0 volume cross."]
    #[must_use]
    #[inline(always)]
    pub const fn zero_adjust_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "enable volume adjustment during 0 volume cross."]
    #[inline(always)]
    pub const fn set_zero_adjust_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "volume ramp interval."]
    #[must_use]
    #[inline(always)]
    pub const fn ramp_interval(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "volume ramp interval."]
    #[inline(always)]
    pub const fn set_ramp_interval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[doc = "ramp module status"]
    #[must_use]
    #[inline(always)]
    pub const fn ramp_stat(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[doc = "ramp module status"]
    #[inline(always)]
    pub const fn set_ramp_stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
}
impl Default for DacCh0CfgExt {
    #[inline(always)]
    fn default() -> DacCh0CfgExt {
        DacCh0CfgExt(0)
    }
}
impl core::fmt::Debug for DacCh0CfgExt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacCh0CfgExt")
            .field("ramp_en", &self.ramp_en())
            .field("ramp_mode", &self.ramp_mode())
            .field("zero_adjust_en", &self.zero_adjust_en())
            .field("ramp_interval", &self.ramp_interval())
            .field("ramp_stat", &self.ramp_stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacCh0CfgExt {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "DacCh0CfgExt {{ ramp_en: {=bool:?}, ramp_mode: {=bool:?}, zero_adjust_en: {=bool:?}, ramp_interval: {=u8:?}, ramp_stat: {=u8:?} }}" , self . ramp_en () , self . ramp_mode () , self . zero_adjust_en () , self . ramp_interval () , self . ramp_stat ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacCh0Dc(pub u32);
impl DacCh0Dc {
    #[doc = "dac ch0 dc offset"]
    #[must_use]
    #[inline(always)]
    pub const fn offset(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "dac ch0 dc offset"]
    #[inline(always)]
    pub const fn set_offset(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacCh0Dc {
    #[inline(always)]
    fn default() -> DacCh0Dc {
        DacCh0Dc(0)
    }
}
impl core::fmt::Debug for DacCh0Dc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacCh0Dc")
            .field("offset", &self.offset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacCh0Dc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DacCh0Dc {{ offset: {=u32:?} }}", self.offset())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacCh0Debug(pub u32);
impl DacCh0Debug {
    #[doc = "debug dac output"]
    #[must_use]
    #[inline(always)]
    pub const fn data_out(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "debug dac output"]
    #[inline(always)]
    pub const fn set_data_out(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "debug bypass mode"]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "debug bypass mode"]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for DacCh0Debug {
    #[inline(always)]
    fn default() -> DacCh0Debug {
        DacCh0Debug(0)
    }
}
impl core::fmt::Debug for DacCh0Debug {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacCh0Debug")
            .field("data_out", &self.data_out())
            .field("bypass", &self.bypass())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacCh0Debug {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DacCh0Debug {{ data_out: {=u16:?}, bypass: {=bool:?} }}",
            self.data_out(),
            self.bypass()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacCh0Entry(pub u32);
impl DacCh0Entry {
    #[doc = "dac channel0 data input"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "dac channel0 data input"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DacCh0Entry {
    #[inline(always)]
    fn default() -> DacCh0Entry {
        DacCh0Entry(0)
    }
}
impl core::fmt::Debug for DacCh0Entry {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacCh0Entry")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacCh0Entry {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DacCh0Entry {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacCh1Cfg(pub u32);
impl DacCh1Cfg {
    #[doc = "dac channel enable"]
    #[must_use]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "dac channel enable"]
    #[inline(always)]
    pub const fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "dac output mute, set 1 to mute the output"]
    #[must_use]
    #[inline(always)]
    pub const fn dout_mute(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "dac output mute, set 1 to mute the output"]
    #[inline(always)]
    pub const fn set_dout_mute(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "dem output mode 2'h0: no shift for dem output 2'h1: shift dem output incrementally 2'h2: shift dem output according to input 2'h3: reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn dem_mode(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "dem output mode 2'h0: no shift for dem output 2'h1: shift dem output incrementally 2'h2: shift dem output according to input 2'h3: reserved"]
    #[inline(always)]
    pub const fn set_dem_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "dac input stb fifo cnt"]
    #[must_use]
    #[inline(always)]
    pub const fn stb_fifo_cnt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "dac input stb fifo cnt"]
    #[inline(always)]
    pub const fn set_stb_fifo_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "dma interface enable in apb mode 1: enable dac ch0 dma request interface 0: disable dac ch0 dma request interface"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "dma interface enable in apb mode 1: enable dac ch0 dma request interface 0: disable dac ch0 dma request interface"]
    #[inline(always)]
    pub const fn set_dma_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "dac rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB"]
    #[must_use]
    #[inline(always)]
    pub const fn rough_vol(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "dac rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB"]
    #[inline(always)]
    pub const fn set_rough_vol(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "dac fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute"]
    #[must_use]
    #[inline(always)]
    pub const fn fine_vol(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "dac fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute"]
    #[inline(always)]
    pub const fn set_fine_vol(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "dac data format 1: 16-bit 0: 24-bit this bit only affect the data format accessed by apb interface. For 24-bit, every 24-bit data occupied 32-bit word. Bit\\[31:24\\] are zeros. For 16-bit mode, every 32-bit word contains two 16-bit audio data{D1, D0}"]
    #[must_use]
    #[inline(always)]
    pub const fn data_format(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "dac data format 1: 16-bit 0: 24-bit this bit only affect the data format accessed by apb interface. For 24-bit, every 24-bit data occupied 32-bit word. Bit\\[31:24\\] are zeros. For 16-bit mode, every 32-bit word contains two 16-bit audio data{D1, D0}"]
    #[inline(always)]
    pub const fn set_data_format(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "dac sinc filter gain"]
    #[must_use]
    #[inline(always)]
    pub const fn sinc_gain(&self) -> u16 {
        let val = (self.0 >> 17usize) & 0x01ff;
        val as u16
    }
    #[doc = "dac sinc filter gain"]
    #[inline(always)]
    pub const fn set_sinc_gain(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 17usize)) | (((val as u32) & 0x01ff) << 17usize);
    }
    #[doc = "sdm dither gain"]
    #[must_use]
    #[inline(always)]
    pub const fn dither_gain(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x07;
        val as u8
    }
    #[doc = "sdm dither gain"]
    #[inline(always)]
    pub const fn set_dither_gain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val as u32) & 0x07) << 26usize);
    }
    #[doc = "sdm dither enable"]
    #[must_use]
    #[inline(always)]
    pub const fn dither_en(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "sdm dither enable"]
    #[inline(always)]
    pub const fn set_dither_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "analog dac clock polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_ana_pol(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "analog dac clock polarity"]
    #[inline(always)]
    pub const fn set_clk_ana_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for DacCh1Cfg {
    #[inline(always)]
    fn default() -> DacCh1Cfg {
        DacCh1Cfg(0)
    }
}
impl core::fmt::Debug for DacCh1Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacCh1Cfg")
            .field("enable", &self.enable())
            .field("dout_mute", &self.dout_mute())
            .field("dem_mode", &self.dem_mode())
            .field("stb_fifo_cnt", &self.stb_fifo_cnt())
            .field("dma_en", &self.dma_en())
            .field("rough_vol", &self.rough_vol())
            .field("fine_vol", &self.fine_vol())
            .field("data_format", &self.data_format())
            .field("sinc_gain", &self.sinc_gain())
            .field("dither_gain", &self.dither_gain())
            .field("dither_en", &self.dither_en())
            .field("clk_ana_pol", &self.clk_ana_pol())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacCh1Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "DacCh1Cfg {{ enable: {=bool:?}, dout_mute: {=bool:?}, dem_mode: {=u8:?}, stb_fifo_cnt: {=u8:?}, dma_en: {=bool:?}, rough_vol: {=u8:?}, fine_vol: {=u8:?}, data_format: {=bool:?}, sinc_gain: {=u16:?}, dither_gain: {=u8:?}, dither_en: {=bool:?}, clk_ana_pol: {=bool:?} }}" , self . enable () , self . dout_mute () , self . dem_mode () , self . stb_fifo_cnt () , self . dma_en () , self . rough_vol () , self . fine_vol () , self . data_format () , self . sinc_gain () , self . dither_gain () , self . dither_en () , self . clk_ana_pol ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacCh1CfgExt(pub u32);
impl DacCh1CfgExt {
    #[doc = "volume ramp enable"]
    #[must_use]
    #[inline(always)]
    pub const fn ramp_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "volume ramp enable"]
    #[inline(always)]
    pub const fn set_ramp_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "volume ramp mode: 1: slowly ramp to target volume. Step is 0.5db 0: directly ramp to target volume."]
    #[must_use]
    #[inline(always)]
    pub const fn ramp_mode(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "volume ramp mode: 1: slowly ramp to target volume. Step is 0.5db 0: directly ramp to target volume."]
    #[inline(always)]
    pub const fn set_ramp_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "enable volume adjustment during 0 volume cross."]
    #[must_use]
    #[inline(always)]
    pub const fn zero_adjust_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "enable volume adjustment during 0 volume cross."]
    #[inline(always)]
    pub const fn set_zero_adjust_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "volume ramp interval."]
    #[must_use]
    #[inline(always)]
    pub const fn ramp_interval(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "volume ramp interval."]
    #[inline(always)]
    pub const fn set_ramp_interval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[doc = "ramp module status"]
    #[must_use]
    #[inline(always)]
    pub const fn ramp_stat(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[doc = "ramp module status"]
    #[inline(always)]
    pub const fn set_ramp_stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
}
impl Default for DacCh1CfgExt {
    #[inline(always)]
    fn default() -> DacCh1CfgExt {
        DacCh1CfgExt(0)
    }
}
impl core::fmt::Debug for DacCh1CfgExt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacCh1CfgExt")
            .field("ramp_en", &self.ramp_en())
            .field("ramp_mode", &self.ramp_mode())
            .field("zero_adjust_en", &self.zero_adjust_en())
            .field("ramp_interval", &self.ramp_interval())
            .field("ramp_stat", &self.ramp_stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacCh1CfgExt {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "DacCh1CfgExt {{ ramp_en: {=bool:?}, ramp_mode: {=bool:?}, zero_adjust_en: {=bool:?}, ramp_interval: {=u8:?}, ramp_stat: {=u8:?} }}" , self . ramp_en () , self . ramp_mode () , self . zero_adjust_en () , self . ramp_interval () , self . ramp_stat ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacCh1Dc(pub u32);
impl DacCh1Dc {
    #[doc = "dac ch1 dc offset"]
    #[must_use]
    #[inline(always)]
    pub const fn offset(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "dac ch1 dc offset"]
    #[inline(always)]
    pub const fn set_offset(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacCh1Dc {
    #[inline(always)]
    fn default() -> DacCh1Dc {
        DacCh1Dc(0)
    }
}
impl core::fmt::Debug for DacCh1Dc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacCh1Dc")
            .field("offset", &self.offset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacCh1Dc {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DacCh1Dc {{ offset: {=u32:?} }}", self.offset())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacCh1Debug(pub u32);
impl DacCh1Debug {
    #[doc = "debug dac output"]
    #[must_use]
    #[inline(always)]
    pub const fn data_out(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "debug dac output"]
    #[inline(always)]
    pub const fn set_data_out(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "debug bypass mode"]
    #[must_use]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "debug bypass mode"]
    #[inline(always)]
    pub const fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for DacCh1Debug {
    #[inline(always)]
    fn default() -> DacCh1Debug {
        DacCh1Debug(0)
    }
}
impl core::fmt::Debug for DacCh1Debug {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacCh1Debug")
            .field("data_out", &self.data_out())
            .field("bypass", &self.bypass())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacCh1Debug {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DacCh1Debug {{ data_out: {=u16:?}, bypass: {=bool:?} }}",
            self.data_out(),
            self.bypass()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacCh1Entry(pub u32);
impl DacCh1Entry {
    #[doc = "dac channel0 data input"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "dac channel0 data input"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for DacCh1Entry {
    #[inline(always)]
    fn default() -> DacCh1Entry {
        DacCh1Entry(0)
    }
}
impl core::fmt::Debug for DacCh1Entry {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacCh1Entry")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacCh1Entry {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DacCh1Entry {{ data: {=u32:?} }}", self.data())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id(pub u32);
impl Id {
    #[doc = "function id"]
    #[must_use]
    #[inline(always)]
    pub const fn func(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "function id"]
    #[inline(always)]
    pub const fn set_func(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Id {
    #[inline(always)]
    fn default() -> Id {
        Id(0)
    }
}
impl core::fmt::Debug for Id {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Id").field("func", &self.func()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Id {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Id {{ func: {=u32:?} }}", self.func())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irq(pub u32);
impl Irq {
    #[doc = "dac ch0 apb fifo overflow interrupt status. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn dac_ch0_apb_of(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "dac ch0 apb fifo overflow interrupt status. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_dac_ch0_apb_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "dac ch0 output fifo underflow interrupt status. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn dac_ch0_out_uf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "dac ch0 output fifo underflow interrupt status. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_dac_ch0_out_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "dac ch0 input stb fifo overflow interrupt status. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn dac_ch0_stb_of(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "dac ch0 input stb fifo overflow interrupt status. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_dac_ch0_stb_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "dac ch1 apb fifo overflow interrupt status. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn dac_ch1_apb_of(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "dac ch1 apb fifo overflow interrupt status. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_dac_ch1_apb_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "dac ch1 output fifo underflow interrupt status. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn dac_ch1_out_uf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "dac ch1 output fifo underflow interrupt status. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_dac_ch1_out_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "dac ch1 input stb fifo overflow interrupt status. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn dac_ch1_stb_of(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "dac ch1 input stb fifo overflow interrupt status. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_dac_ch1_stb_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "adc ch0 apb fifo overflow interrupt status. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn adc_ch0_apb_of(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "adc ch0 apb fifo overflow interrupt status. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_adc_ch0_apb_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "adc ch0 apb fifo underflow interrupt status. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn adc_ch0_apb_uf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "adc ch0 apb fifo underflow interrupt status. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_adc_ch0_apb_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "adc ch0 saturation interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn adc_ch0_sat(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "adc ch0 saturation interrupt"]
    #[inline(always)]
    pub const fn set_adc_ch0_sat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "adc ch1 apb fifo overflow interrupt status. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn adc_ch1_apb_of(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "adc ch1 apb fifo overflow interrupt status. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_adc_ch1_apb_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "adc ch1 apb fifo underflow interrupt status. Write 1 to clear."]
    #[must_use]
    #[inline(always)]
    pub const fn adc_ch1_apb_uf(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "adc ch1 apb fifo underflow interrupt status. Write 1 to clear."]
    #[inline(always)]
    pub const fn set_adc_ch1_apb_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "adc ch1 saturation interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn adc_ch1_sat(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "adc ch1 saturation interrupt"]
    #[inline(always)]
    pub const fn set_adc_ch1_sat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for Irq {
    #[inline(always)]
    fn default() -> Irq {
        Irq(0)
    }
}
impl core::fmt::Debug for Irq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Irq")
            .field("dac_ch0_apb_of", &self.dac_ch0_apb_of())
            .field("dac_ch0_out_uf", &self.dac_ch0_out_uf())
            .field("dac_ch0_stb_of", &self.dac_ch0_stb_of())
            .field("dac_ch1_apb_of", &self.dac_ch1_apb_of())
            .field("dac_ch1_out_uf", &self.dac_ch1_out_uf())
            .field("dac_ch1_stb_of", &self.dac_ch1_stb_of())
            .field("adc_ch0_apb_of", &self.adc_ch0_apb_of())
            .field("adc_ch0_apb_uf", &self.adc_ch0_apb_uf())
            .field("adc_ch0_sat", &self.adc_ch0_sat())
            .field("adc_ch1_apb_of", &self.adc_ch1_apb_of())
            .field("adc_ch1_apb_uf", &self.adc_ch1_apb_uf())
            .field("adc_ch1_sat", &self.adc_ch1_sat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irq {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Irq {{ dac_ch0_apb_of: {=bool:?}, dac_ch0_out_uf: {=bool:?}, dac_ch0_stb_of: {=bool:?}, dac_ch1_apb_of: {=bool:?}, dac_ch1_out_uf: {=bool:?}, dac_ch1_stb_of: {=bool:?}, adc_ch0_apb_of: {=bool:?}, adc_ch0_apb_uf: {=bool:?}, adc_ch0_sat: {=bool:?}, adc_ch1_apb_of: {=bool:?}, adc_ch1_apb_uf: {=bool:?}, adc_ch1_sat: {=bool:?} }}" , self . dac_ch0_apb_of () , self . dac_ch0_out_uf () , self . dac_ch0_stb_of () , self . dac_ch1_apb_of () , self . dac_ch1_out_uf () , self . dac_ch1_stb_of () , self . adc_ch0_apb_of () , self . adc_ch0_apb_uf () , self . adc_ch0_sat () , self . adc_ch1_apb_of () , self . adc_ch1_apb_uf () , self . adc_ch1_sat ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrqMsk(pub u32);
impl IrqMsk {
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn dac_ch0_apb_of(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[inline(always)]
    pub const fn set_dac_ch0_apb_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn dac_ch0_out_uf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[inline(always)]
    pub const fn set_dac_ch0_out_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn dac_ch0_stb_of(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[inline(always)]
    pub const fn set_dac_ch0_stb_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn dac_ch1_apb_of(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[inline(always)]
    pub const fn set_dac_ch1_apb_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn dac_ch1_out_uf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[inline(always)]
    pub const fn set_dac_ch1_out_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn dac_ch1_stb_of(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[inline(always)]
    pub const fn set_dac_ch1_stb_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn adc_ch0_apb_of(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[inline(always)]
    pub const fn set_adc_ch0_apb_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn adc_ch0_apb_uf(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[inline(always)]
    pub const fn set_adc_ch0_apb_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn adc_ch0_sat(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[inline(always)]
    pub const fn set_adc_ch0_sat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn adc_ch1_apb_of(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[inline(always)]
    pub const fn set_adc_ch1_apb_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn adc_ch1_apb_uf(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[inline(always)]
    pub const fn set_adc_ch1_apb_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[must_use]
    #[inline(always)]
    pub const fn adc_ch1_sat(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "interrupt mask. 0: mask the interrupt."]
    #[inline(always)]
    pub const fn set_adc_ch1_sat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for IrqMsk {
    #[inline(always)]
    fn default() -> IrqMsk {
        IrqMsk(0)
    }
}
impl core::fmt::Debug for IrqMsk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IrqMsk")
            .field("dac_ch0_apb_of", &self.dac_ch0_apb_of())
            .field("dac_ch0_out_uf", &self.dac_ch0_out_uf())
            .field("dac_ch0_stb_of", &self.dac_ch0_stb_of())
            .field("dac_ch1_apb_of", &self.dac_ch1_apb_of())
            .field("dac_ch1_out_uf", &self.dac_ch1_out_uf())
            .field("dac_ch1_stb_of", &self.dac_ch1_stb_of())
            .field("adc_ch0_apb_of", &self.adc_ch0_apb_of())
            .field("adc_ch0_apb_uf", &self.adc_ch0_apb_uf())
            .field("adc_ch0_sat", &self.adc_ch0_sat())
            .field("adc_ch1_apb_of", &self.adc_ch1_apb_of())
            .field("adc_ch1_apb_uf", &self.adc_ch1_apb_uf())
            .field("adc_ch1_sat", &self.adc_ch1_sat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IrqMsk {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "IrqMsk {{ dac_ch0_apb_of: {=bool:?}, dac_ch0_out_uf: {=bool:?}, dac_ch0_stb_of: {=bool:?}, dac_ch1_apb_of: {=bool:?}, dac_ch1_out_uf: {=bool:?}, dac_ch1_stb_of: {=bool:?}, adc_ch0_apb_of: {=bool:?}, adc_ch0_apb_uf: {=bool:?}, adc_ch0_sat: {=bool:?}, adc_ch1_apb_of: {=bool:?}, adc_ch1_apb_uf: {=bool:?}, adc_ch1_sat: {=bool:?} }}" , self . dac_ch0_apb_of () , self . dac_ch0_out_uf () , self . dac_ch0_stb_of () , self . dac_ch1_apb_of () , self . dac_ch1_out_uf () , self . dac_ch1_stb_of () , self . adc_ch0_apb_of () , self . adc_ch0_apb_uf () , self . adc_ch0_sat () , self . adc_ch1_apb_of () , self . adc_ch1_apb_uf () , self . adc_ch1_sat ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllCalCfg(pub u32);
impl PllCalCfg {
    #[doc = "calibration enable"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "calibration enable"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "calibration done"]
    #[must_use]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "calibration done"]
    #[inline(always)]
    pub const fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "calibration length"]
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "calibration length"]
    #[inline(always)]
    pub const fn set_len(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PllCalCfg {
    #[inline(always)]
    fn default() -> PllCalCfg {
        PllCalCfg(0)
    }
}
impl core::fmt::Debug for PllCalCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllCalCfg")
            .field("en", &self.en())
            .field("done", &self.done())
            .field("len", &self.len())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllCalCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllCalCfg {{ en: {=bool:?}, done: {=bool:?}, len: {=u16:?} }}",
            self.en(),
            self.done(),
            self.len()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllCalResult(pub u32);
impl PllCalResult {
    #[doc = "xtal calibration counter result"]
    #[must_use]
    #[inline(always)]
    pub const fn xtal_cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "xtal calibration counter result"]
    #[inline(always)]
    pub const fn set_xtal_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "pll calibration counter result"]
    #[must_use]
    #[inline(always)]
    pub const fn pll_cnt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "pll calibration counter result"]
    #[inline(always)]
    pub const fn set_pll_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for PllCalResult {
    #[inline(always)]
    fn default() -> PllCalResult {
        PllCalResult(0)
    }
}
impl core::fmt::Debug for PllCalResult {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllCalResult")
            .field("xtal_cnt", &self.xtal_cnt())
            .field("pll_cnt", &self.pll_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllCalResult {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllCalResult {{ xtal_cnt: {=u16:?}, pll_cnt: {=u16:?} }}",
            self.xtal_cnt(),
            self.pll_cnt()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllCfg0(pub u32);
impl PllCfg0 {
    #[doc = "Icp os"]
    #[must_use]
    #[inline(always)]
    pub const fn icp_os_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Icp os"]
    #[inline(always)]
    pub const fn set_icp_os_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "1: pll open"]
    #[must_use]
    #[inline(always)]
    pub const fn open(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "1: pll open"]
    #[inline(always)]
    pub const fn set_open(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "select Icp, 1:1.25u"]
    #[must_use]
    #[inline(always)]
    pub const fn icp_sel(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x1f;
        val as u8
    }
    #[doc = "select Icp, 1:1.25u"]
    #[inline(always)]
    pub const fn set_icp_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
    }
    #[doc = "ldo vref, 7:1.1V"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_vref_ana(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "ldo vref, 7:1.1V"]
    #[inline(always)]
    pub const fn set_sel_vref_ana(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "enable ana block"]
    #[must_use]
    #[inline(always)]
    pub const fn en_ana(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "enable ana block"]
    #[inline(always)]
    pub const fn set_en_ana(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "1: lp mode"]
    #[must_use]
    #[inline(always)]
    pub const fn vco_lp_mode(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "1: lp mode"]
    #[inline(always)]
    pub const fn set_vco_lp_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "VCO Fcode"]
    #[must_use]
    #[inline(always)]
    pub const fn fc_vco(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x1f;
        val as u8
    }
    #[doc = "VCO Fcode"]
    #[inline(always)]
    pub const fn set_fc_vco(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 17usize)) | (((val as u32) & 0x1f) << 17usize);
    }
    #[doc = "vco bais filter"]
    #[must_use]
    #[inline(always)]
    pub const fn en_vco_flt(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "vco bais filter"]
    #[inline(always)]
    pub const fn set_en_vco_flt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "ldo vref, 7:1.1V"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_vref_vco(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x0f;
        val as u8
    }
    #[doc = "ldo vref, 7:1.1V"]
    #[inline(always)]
    pub const fn set_sel_vref_vco(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 23usize)) | (((val as u32) & 0x0f) << 23usize);
    }
    #[doc = "enable vco"]
    #[must_use]
    #[inline(always)]
    pub const fn en_vco(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "enable vco"]
    #[inline(always)]
    pub const fn set_en_vco(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "enable I array"]
    #[must_use]
    #[inline(always)]
    pub const fn en_iary(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "enable I array"]
    #[inline(always)]
    pub const fn set_en_iary(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "select ref clock, 2: 24MHz"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_ckref(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x03;
        val as u8
    }
    #[doc = "select ref clock, 2: 24MHz"]
    #[inline(always)]
    pub const fn set_sel_ckref(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
    }
}
impl Default for PllCfg0 {
    #[inline(always)]
    fn default() -> PllCfg0 {
        PllCfg0(0)
    }
}
impl core::fmt::Debug for PllCfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllCfg0")
            .field("icp_os_sel", &self.icp_os_sel())
            .field("open", &self.open())
            .field("icp_sel", &self.icp_sel())
            .field("sel_vref_ana", &self.sel_vref_ana())
            .field("en_ana", &self.en_ana())
            .field("vco_lp_mode", &self.vco_lp_mode())
            .field("fc_vco", &self.fc_vco())
            .field("en_vco_flt", &self.en_vco_flt())
            .field("sel_vref_vco", &self.sel_vref_vco())
            .field("en_vco", &self.en_vco())
            .field("en_iary", &self.en_iary())
            .field("sel_ckref", &self.sel_ckref())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllCfg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "PllCfg0 {{ icp_os_sel: {=u8:?}, open: {=bool:?}, icp_sel: {=u8:?}, sel_vref_ana: {=u8:?}, en_ana: {=bool:?}, vco_lp_mode: {=bool:?}, fc_vco: {=u8:?}, en_vco_flt: {=bool:?}, sel_vref_vco: {=u8:?}, en_vco: {=bool:?}, en_iary: {=bool:?}, sel_ckref: {=u8:?} }}" , self . icp_os_sel () , self . open () , self . icp_sel () , self . sel_vref_ana () , self . en_ana () , self . vco_lp_mode () , self . fc_vco () , self . en_vco_flt () , self . sel_vref_vco () , self . en_vco () , self . en_iary () , self . sel_ckref ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllCfg1(pub u32);
impl PllCfg1 {
    #[doc = "select R3"]
    #[must_use]
    #[inline(always)]
    pub const fn r3_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "select R3"]
    #[inline(always)]
    pub const fn set_r3_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "select Rz"]
    #[must_use]
    #[inline(always)]
    pub const fn rz_sel(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "select Rz"]
    #[inline(always)]
    pub const fn set_rz_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "select C2"]
    #[must_use]
    #[inline(always)]
    pub const fn c2_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "select C2"]
    #[inline(always)]
    pub const fn set_c2_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "select Cz"]
    #[must_use]
    #[inline(always)]
    pub const fn cz_sel(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[doc = "select Cz"]
    #[inline(always)]
    pub const fn set_cz_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
    #[doc = "reset CSD, high active"]
    #[must_use]
    #[inline(always)]
    pub const fn csd_rst(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "reset CSD, high active"]
    #[inline(always)]
    pub const fn set_csd_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "enable CSD"]
    #[must_use]
    #[inline(always)]
    pub const fn csd_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "enable CSD"]
    #[inline(always)]
    pub const fn set_csd_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for PllCfg1 {
    #[inline(always)]
    fn default() -> PllCfg1 {
        PllCfg1(0)
    }
}
impl core::fmt::Debug for PllCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllCfg1")
            .field("r3_sel", &self.r3_sel())
            .field("rz_sel", &self.rz_sel())
            .field("c2_sel", &self.c2_sel())
            .field("cz_sel", &self.cz_sel())
            .field("csd_rst", &self.csd_rst())
            .field("csd_en", &self.csd_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllCfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "PllCfg1 {{ r3_sel: {=u8:?}, rz_sel: {=u8:?}, c2_sel: {=u8:?}, cz_sel: {=u8:?}, csd_rst: {=bool:?}, csd_en: {=bool:?} }}" , self . r3_sel () , self . rz_sel () , self . c2_sel () , self . cz_sel () , self . csd_rst () , self . csd_en ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllCfg2(pub u32);
impl PllCfg2 {
    #[doc = "mmd stg"]
    #[must_use]
    #[inline(always)]
    pub const fn mmd_stg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "mmd stg"]
    #[inline(always)]
    pub const fn set_mmd_stg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "select dtest"]
    #[must_use]
    #[inline(always)]
    pub const fn tr_dtest(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "select dtest"]
    #[inline(always)]
    pub const fn set_tr_dtest(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[doc = "enable dtest"]
    #[must_use]
    #[inline(always)]
    pub const fn te_dtest(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "enable dtest"]
    #[inline(always)]
    pub const fn set_te_dtest(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "resetb sync"]
    #[must_use]
    #[inline(always)]
    pub const fn rstb_sync_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "resetb sync"]
    #[inline(always)]
    pub const fn set_rstb_sync_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "resetb"]
    #[must_use]
    #[inline(always)]
    pub const fn rstb(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "resetb"]
    #[inline(always)]
    pub const fn set_rstb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "ldo vref, 7:1.1V"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_vref_dig(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x0f;
        val as u8
    }
    #[doc = "ldo vref, 7:1.1V"]
    #[inline(always)]
    pub const fn set_sel_vref_dig(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
    }
    #[doc = "enable dig block"]
    #[must_use]
    #[inline(always)]
    pub const fn en_dig(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "enable dig block"]
    #[inline(always)]
    pub const fn set_en_dig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "enable vctrl buf"]
    #[must_use]
    #[inline(always)]
    pub const fn en_lf_tstbuf(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "enable vctrl buf"]
    #[inline(always)]
    pub const fn set_en_lf_tstbuf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "select vcin, 4: 550mV"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_lf_vcin(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "select vcin, 4: 550mV"]
    #[inline(always)]
    pub const fn set_sel_lf_vcin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "enable vcin for vco"]
    #[must_use]
    #[inline(always)]
    pub const fn en_lf_vcin(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "enable vcin for vco"]
    #[inline(always)]
    pub const fn set_en_lf_vcin(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for PllCfg2 {
    #[inline(always)]
    fn default() -> PllCfg2 {
        PllCfg2(0)
    }
}
impl core::fmt::Debug for PllCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllCfg2")
            .field("mmd_stg", &self.mmd_stg())
            .field("tr_dtest", &self.tr_dtest())
            .field("te_dtest", &self.te_dtest())
            .field("rstb_sync_en", &self.rstb_sync_en())
            .field("rstb", &self.rstb())
            .field("sel_vref_dig", &self.sel_vref_dig())
            .field("en_dig", &self.en_dig())
            .field("en_lf_tstbuf", &self.en_lf_tstbuf())
            .field("sel_lf_vcin", &self.sel_lf_vcin())
            .field("en_lf_vcin", &self.en_lf_vcin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllCfg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "PllCfg2 {{ mmd_stg: {=u8:?}, tr_dtest: {=u8:?}, te_dtest: {=bool:?}, rstb_sync_en: {=bool:?}, rstb: {=bool:?}, sel_vref_dig: {=u8:?}, en_dig: {=bool:?}, en_lf_tstbuf: {=bool:?}, sel_lf_vcin: {=u8:?}, en_lf_vcin: {=bool:?} }}" , self . mmd_stg () , self . tr_dtest () , self . te_dtest () , self . rstb_sync_en () , self . rstb () , self . sel_vref_dig () , self . en_dig () , self . en_lf_tstbuf () , self . sel_lf_vcin () , self . en_lf_vcin ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllCfg3(pub u32);
impl PllCfg3 {
    #[doc = "sdm input"]
    #[must_use]
    #[inline(always)]
    pub const fn sdin(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[doc = "sdm input"]
    #[inline(always)]
    pub const fn set_sdin(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
    #[doc = "FCW"]
    #[must_use]
    #[inline(always)]
    pub const fn fcw(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x1f;
        val as u8
    }
    #[doc = "FCW"]
    #[inline(always)]
    pub const fn set_fcw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
    }
    #[doc = "write 1 to update FCW and SDIN value"]
    #[must_use]
    #[inline(always)]
    pub const fn sdm_update(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to update FCW and SDIN value"]
    #[inline(always)]
    pub const fn set_sdm_update(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "1: bypass FCW and SDIN sdm control signal"]
    #[must_use]
    #[inline(always)]
    pub const fn sdmin_bypass(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "1: bypass FCW and SDIN sdm control signal"]
    #[inline(always)]
    pub const fn set_sdmin_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "sdm mode"]
    #[must_use]
    #[inline(always)]
    pub const fn sdm_mode(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "sdm mode"]
    #[inline(always)]
    pub const fn set_sdm_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "enable sdm dither"]
    #[must_use]
    #[inline(always)]
    pub const fn en_sdm_dither(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "enable sdm dither"]
    #[inline(always)]
    pub const fn set_en_sdm_dither(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "sdm dither"]
    #[must_use]
    #[inline(always)]
    pub const fn sdm_dither(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "sdm dither"]
    #[inline(always)]
    pub const fn set_sdm_dither(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "enable sdm"]
    #[must_use]
    #[inline(always)]
    pub const fn en_sdm(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "enable sdm"]
    #[inline(always)]
    pub const fn set_en_sdm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "sdm dig clk polarity"]
    #[must_use]
    #[inline(always)]
    pub const fn sdmclk_pol(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "sdm dig clk polarity"]
    #[inline(always)]
    pub const fn set_sdmclk_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for PllCfg3 {
    #[inline(always)]
    fn default() -> PllCfg3 {
        PllCfg3(0)
    }
}
impl core::fmt::Debug for PllCfg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllCfg3")
            .field("sdin", &self.sdin())
            .field("fcw", &self.fcw())
            .field("sdm_update", &self.sdm_update())
            .field("sdmin_bypass", &self.sdmin_bypass())
            .field("sdm_mode", &self.sdm_mode())
            .field("en_sdm_dither", &self.en_sdm_dither())
            .field("sdm_dither", &self.sdm_dither())
            .field("en_sdm", &self.en_sdm())
            .field("sdmclk_pol", &self.sdmclk_pol())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllCfg3 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "PllCfg3 {{ sdin: {=u32:?}, fcw: {=u8:?}, sdm_update: {=bool:?}, sdmin_bypass: {=bool:?}, sdm_mode: {=bool:?}, en_sdm_dither: {=bool:?}, sdm_dither: {=bool:?}, en_sdm: {=bool:?}, sdmclk_pol: {=bool:?} }}" , self . sdin () , self . fcw () , self . sdm_update () , self . sdmin_bypass () , self . sdm_mode () , self . en_sdm_dither () , self . sdm_dither () , self . en_sdm () , self . sdmclk_pol ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllCfg4(pub u32);
impl PllCfg4 {
    #[doc = "DIVB dac chop clk"]
    #[must_use]
    #[inline(always)]
    pub const fn divb_clk_chop_dac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "DIVB dac chop clk"]
    #[inline(always)]
    pub const fn set_divb_clk_chop_dac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "DIVA dac chop clk"]
    #[must_use]
    #[inline(always)]
    pub const fn diva_clk_chop_dac(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x07;
        val as u8
    }
    #[doc = "DIVA dac chop clk"]
    #[inline(always)]
    pub const fn set_diva_clk_chop_dac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
    }
    #[doc = "enable dac chop clk"]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_chop_dac(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "enable dac chop clk"]
    #[inline(always)]
    pub const fn set_en_clk_chop_dac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DIVA dac clk"]
    #[must_use]
    #[inline(always)]
    pub const fn diva_clk_dac(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x1f;
        val as u8
    }
    #[doc = "DIVA dac clk"]
    #[inline(always)]
    pub const fn set_diva_clk_dac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
    }
    #[doc = "enable dac clk"]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_dac(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "enable dac clk"]
    #[inline(always)]
    pub const fn set_en_clk_dac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "1: select 9.6MHz as DAC clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_clk_dac(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "1: select 9.6MHz as DAC clock"]
    #[inline(always)]
    pub const fn set_sel_clk_dac(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "0: xtal 1: pll"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_clk_dac_source(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x03;
        val as u8
    }
    #[doc = "0: xtal 1: pll"]
    #[inline(always)]
    pub const fn set_sel_clk_dac_source(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
    }
    #[doc = "select dig clk 0: pll 1: 24MHz from xtal"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_clk_dig(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "select dig clk 0: pll 1: 24MHz from xtal"]
    #[inline(always)]
    pub const fn set_sel_clk_dig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "strength"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_dig_str(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "strength"]
    #[inline(always)]
    pub const fn set_clk_dig_str(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "DIVA dig clk"]
    #[must_use]
    #[inline(always)]
    pub const fn diva_clk_dig(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x1f;
        val as u8
    }
    #[doc = "DIVA dig clk"]
    #[inline(always)]
    pub const fn set_diva_clk_dig(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
    }
    #[doc = "enable dig clk"]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_dig(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "enable dig clk"]
    #[inline(always)]
    pub const fn set_en_clk_dig(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for PllCfg4 {
    #[inline(always)]
    fn default() -> PllCfg4 {
        PllCfg4(0)
    }
}
impl core::fmt::Debug for PllCfg4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllCfg4")
            .field("divb_clk_chop_dac", &self.divb_clk_chop_dac())
            .field("diva_clk_chop_dac", &self.diva_clk_chop_dac())
            .field("en_clk_chop_dac", &self.en_clk_chop_dac())
            .field("diva_clk_dac", &self.diva_clk_dac())
            .field("en_clk_dac", &self.en_clk_dac())
            .field("sel_clk_dac", &self.sel_clk_dac())
            .field("sel_clk_dac_source", &self.sel_clk_dac_source())
            .field("sel_clk_dig", &self.sel_clk_dig())
            .field("clk_dig_str", &self.clk_dig_str())
            .field("diva_clk_dig", &self.diva_clk_dig())
            .field("en_clk_dig", &self.en_clk_dig())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllCfg4 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "PllCfg4 {{ divb_clk_chop_dac: {=u8:?}, diva_clk_chop_dac: {=u8:?}, en_clk_chop_dac: {=bool:?}, diva_clk_dac: {=u8:?}, en_clk_dac: {=bool:?}, sel_clk_dac: {=bool:?}, sel_clk_dac_source: {=u8:?}, sel_clk_dig: {=bool:?}, clk_dig_str: {=u8:?}, diva_clk_dig: {=u8:?}, en_clk_dig: {=bool:?} }}" , self . divb_clk_chop_dac () , self . diva_clk_chop_dac () , self . en_clk_chop_dac () , self . diva_clk_dac () , self . en_clk_dac () , self . sel_clk_dac () , self . sel_clk_dac_source () , self . sel_clk_dig () , self . clk_dig_str () , self . diva_clk_dig () , self . en_clk_dig ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllCfg5(pub u32);
impl PllCfg5 {
    #[doc = "DIVB bg chop clk"]
    #[must_use]
    #[inline(always)]
    pub const fn divb_clk_chop_bg(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "DIVB bg chop clk"]
    #[inline(always)]
    pub const fn set_divb_clk_chop_bg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "DIVA bg chop clk"]
    #[must_use]
    #[inline(always)]
    pub const fn diva_clk_chop_bg(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x1f;
        val as u8
    }
    #[doc = "DIVA bg chop clk"]
    #[inline(always)]
    pub const fn set_diva_clk_chop_bg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u32) & 0x1f) << 2usize);
    }
    #[doc = "enable bg chop clk"]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_chop_bg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "enable bg chop clk"]
    #[inline(always)]
    pub const fn set_en_clk_chop_bg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "DIVB ref chop clk"]
    #[must_use]
    #[inline(always)]
    pub const fn divb_clk_chop_refgen(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "DIVB ref chop clk"]
    #[inline(always)]
    pub const fn set_divb_clk_chop_refgen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "DIVA ref chop clk"]
    #[must_use]
    #[inline(always)]
    pub const fn diva_clk_chop_refgen(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "DIVA ref chop clk"]
    #[inline(always)]
    pub const fn set_diva_clk_chop_refgen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "enable ref chop clk"]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_chop_refgen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "enable ref chop clk"]
    #[inline(always)]
    pub const fn set_en_clk_chop_refgen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DIVB dac2 chop clk"]
    #[must_use]
    #[inline(always)]
    pub const fn divb_clk_chop_dac2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "DIVB dac2 chop clk"]
    #[inline(always)]
    pub const fn set_divb_clk_chop_dac2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "DIVA dac2 chop clk"]
    #[must_use]
    #[inline(always)]
    pub const fn diva_clk_chop_dac2(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "DIVA dac2 chop clk"]
    #[inline(always)]
    pub const fn set_diva_clk_chop_dac2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "enable dac2 chop clk"]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_chop_dac2(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "enable dac2 chop clk"]
    #[inline(always)]
    pub const fn set_en_clk_chop_dac2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DIVA dac2 clk"]
    #[must_use]
    #[inline(always)]
    pub const fn diva_clk_dac2(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x1f;
        val as u8
    }
    #[doc = "DIVA dac2 clk"]
    #[inline(always)]
    pub const fn set_diva_clk_dac2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 22usize)) | (((val as u32) & 0x1f) << 22usize);
    }
    #[doc = "enable dac2 clk"]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_dac2(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "enable dac2 clk"]
    #[inline(always)]
    pub const fn set_en_clk_dac2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "1: select 9.6MHz as DAC clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_clk_dac2(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "1: select 9.6MHz as DAC clock"]
    #[inline(always)]
    pub const fn set_sel_clk_dac2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for PllCfg5 {
    #[inline(always)]
    fn default() -> PllCfg5 {
        PllCfg5(0)
    }
}
impl core::fmt::Debug for PllCfg5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllCfg5")
            .field("divb_clk_chop_bg", &self.divb_clk_chop_bg())
            .field("diva_clk_chop_bg", &self.diva_clk_chop_bg())
            .field("en_clk_chop_bg", &self.en_clk_chop_bg())
            .field("divb_clk_chop_refgen", &self.divb_clk_chop_refgen())
            .field("diva_clk_chop_refgen", &self.diva_clk_chop_refgen())
            .field("en_clk_chop_refgen", &self.en_clk_chop_refgen())
            .field("divb_clk_chop_dac2", &self.divb_clk_chop_dac2())
            .field("diva_clk_chop_dac2", &self.diva_clk_chop_dac2())
            .field("en_clk_chop_dac2", &self.en_clk_chop_dac2())
            .field("diva_clk_dac2", &self.diva_clk_dac2())
            .field("en_clk_dac2", &self.en_clk_dac2())
            .field("sel_clk_dac2", &self.sel_clk_dac2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllCfg5 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "PllCfg5 {{ divb_clk_chop_bg: {=u8:?}, diva_clk_chop_bg: {=u8:?}, en_clk_chop_bg: {=bool:?}, divb_clk_chop_refgen: {=u8:?}, diva_clk_chop_refgen: {=u8:?}, en_clk_chop_refgen: {=bool:?}, divb_clk_chop_dac2: {=u8:?}, diva_clk_chop_dac2: {=u8:?}, en_clk_chop_dac2: {=bool:?}, diva_clk_dac2: {=u8:?}, en_clk_dac2: {=bool:?}, sel_clk_dac2: {=bool:?} }}" , self . divb_clk_chop_bg () , self . diva_clk_chop_bg () , self . en_clk_chop_bg () , self . divb_clk_chop_refgen () , self . diva_clk_chop_refgen () , self . en_clk_chop_refgen () , self . divb_clk_chop_dac2 () , self . diva_clk_chop_dac2 () , self . en_clk_chop_dac2 () , self . diva_clk_dac2 () , self . en_clk_dac2 () , self . sel_clk_dac2 ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllCfg6(pub u32);
impl PllCfg6 {
    #[doc = "select clk to test"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_tst_clk(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "select clk to test"]
    #[inline(always)]
    pub const fn set_sel_tst_clk(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "enable test clk"]
    #[must_use]
    #[inline(always)]
    pub const fn en_tst_clk(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "enable test clk"]
    #[inline(always)]
    pub const fn set_en_tst_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "enable RC CAL clk"]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_rccal(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "enable RC CAL clk"]
    #[inline(always)]
    pub const fn set_en_clk_rccal(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "select micbias chop clk"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_clk_chop_micbias(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "select micbias chop clk"]
    #[inline(always)]
    pub const fn set_sel_clk_chop_micbias(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "enable micbias chop clk"]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_chop_micbias(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "enable micbias chop clk"]
    #[inline(always)]
    pub const fn set_en_clk_chop_micbias(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "select adc2 clk"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_clk_adc2(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "select adc2 clk"]
    #[inline(always)]
    pub const fn set_sel_clk_adc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "DIVA adc2 clk"]
    #[must_use]
    #[inline(always)]
    pub const fn diva_clk_adc2(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "DIVA adc2 clk"]
    #[inline(always)]
    pub const fn set_diva_clk_adc2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
    }
    #[doc = "enable adc2 clk"]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_adc2(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "enable adc2 clk"]
    #[inline(always)]
    pub const fn set_en_clk_adc2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "select adc1 clk"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_clk_adc1(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "select adc1 clk"]
    #[inline(always)]
    pub const fn set_sel_clk_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "DIVA adc1 clk"]
    #[must_use]
    #[inline(always)]
    pub const fn diva_clk_adc1(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "DIVA adc1 clk"]
    #[inline(always)]
    pub const fn set_diva_clk_adc1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "enable adc1 clk"]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_adc1(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "enable adc1 clk"]
    #[inline(always)]
    pub const fn set_en_clk_adc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "select adc0 clk"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_clk_adc0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "select adc0 clk"]
    #[inline(always)]
    pub const fn set_sel_clk_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DIVA adc0 clk"]
    #[must_use]
    #[inline(always)]
    pub const fn diva_clk_adc0(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x07;
        val as u8
    }
    #[doc = "DIVA adc0 clk"]
    #[inline(always)]
    pub const fn set_diva_clk_adc0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
    }
    #[doc = "enable adc0 clk"]
    #[must_use]
    #[inline(always)]
    pub const fn en_clk_adc0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "enable adc0 clk"]
    #[inline(always)]
    pub const fn set_en_clk_adc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "0: xtal, 1: pll"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_clk_adc_source(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[doc = "0: xtal, 1: pll"]
    #[inline(always)]
    pub const fn set_sel_clk_adc_source(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
}
impl Default for PllCfg6 {
    #[inline(always)]
    fn default() -> PllCfg6 {
        PllCfg6(0)
    }
}
impl core::fmt::Debug for PllCfg6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllCfg6")
            .field("sel_tst_clk", &self.sel_tst_clk())
            .field("en_tst_clk", &self.en_tst_clk())
            .field("en_clk_rccal", &self.en_clk_rccal())
            .field("sel_clk_chop_micbias", &self.sel_clk_chop_micbias())
            .field("en_clk_chop_micbias", &self.en_clk_chop_micbias())
            .field("sel_clk_adc2", &self.sel_clk_adc2())
            .field("diva_clk_adc2", &self.diva_clk_adc2())
            .field("en_clk_adc2", &self.en_clk_adc2())
            .field("sel_clk_adc1", &self.sel_clk_adc1())
            .field("diva_clk_adc1", &self.diva_clk_adc1())
            .field("en_clk_adc1", &self.en_clk_adc1())
            .field("sel_clk_adc0", &self.sel_clk_adc0())
            .field("diva_clk_adc0", &self.diva_clk_adc0())
            .field("en_clk_adc0", &self.en_clk_adc0())
            .field("sel_clk_adc_source", &self.sel_clk_adc_source())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllCfg6 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "PllCfg6 {{ sel_tst_clk: {=u8:?}, en_tst_clk: {=bool:?}, en_clk_rccal: {=bool:?}, sel_clk_chop_micbias: {=u8:?}, en_clk_chop_micbias: {=bool:?}, sel_clk_adc2: {=bool:?}, diva_clk_adc2: {=u8:?}, en_clk_adc2: {=bool:?}, sel_clk_adc1: {=bool:?}, diva_clk_adc1: {=u8:?}, en_clk_adc1: {=bool:?}, sel_clk_adc0: {=bool:?}, diva_clk_adc0: {=u8:?}, en_clk_adc0: {=bool:?}, sel_clk_adc_source: {=u8:?} }}" , self . sel_tst_clk () , self . en_tst_clk () , self . en_clk_rccal () , self . sel_clk_chop_micbias () , self . en_clk_chop_micbias () , self . sel_clk_adc2 () , self . diva_clk_adc2 () , self . en_clk_adc2 () , self . sel_clk_adc1 () , self . diva_clk_adc1 () , self . en_clk_adc1 () , self . sel_clk_adc0 () , self . diva_clk_adc0 () , self . en_clk_adc0 () , self . sel_clk_adc_source ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllStat(pub u32);
impl PllStat {
    #[doc = "1:pll unlock"]
    #[must_use]
    #[inline(always)]
    pub const fn unlock(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1:pll unlock"]
    #[inline(always)]
    pub const fn set_unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "slip up"]
    #[must_use]
    #[inline(always)]
    pub const fn slipped_up(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "slip up"]
    #[inline(always)]
    pub const fn set_slipped_up(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "slip dn"]
    #[must_use]
    #[inline(always)]
    pub const fn slipped_dn(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "slip dn"]
    #[inline(always)]
    pub const fn set_slipped_dn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for PllStat {
    #[inline(always)]
    fn default() -> PllStat {
        PllStat(0)
    }
}
impl core::fmt::Debug for PllStat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PllStat")
            .field("unlock", &self.unlock())
            .field("slipped_up", &self.slipped_up())
            .field("slipped_dn", &self.slipped_dn())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PllStat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PllStat {{ unlock: {=bool:?}, slipped_up: {=bool:?}, slipped_dn: {=bool:?} }}",
            self.unlock(),
            self.slipped_up(),
            self.slipped_dn()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RefgenCfg(pub u32);
impl RefgenCfg {
    #[doc = "enable ref gen"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "enable ref gen"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "enable ref gen chop"]
    #[must_use]
    #[inline(always)]
    pub const fn en_chop(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "enable ref gen chop"]
    #[inline(always)]
    pub const fn set_en_chop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "bias mode"]
    #[must_use]
    #[inline(always)]
    pub const fn bm(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "bias mode"]
    #[inline(always)]
    pub const fn set_bm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "1: lpmode(adc), 0:dac"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_mode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "1: lpmode(adc), 0:dac"]
    #[inline(always)]
    pub const fn set_lp_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "low vol mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lv_mode(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "low vol mode"]
    #[inline(always)]
    pub const fn set_lv_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "sel Rz, 0: 1uF cap"]
    #[must_use]
    #[inline(always)]
    pub const fn rzsel(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "sel Rz, 0: 1uF cap"]
    #[inline(always)]
    pub const fn set_rzsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "discharge vref"]
    #[must_use]
    #[inline(always)]
    pub const fn dischg(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "discharge vref"]
    #[inline(always)]
    pub const fn set_dischg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for RefgenCfg {
    #[inline(always)]
    fn default() -> RefgenCfg {
        RefgenCfg(0)
    }
}
impl core::fmt::Debug for RefgenCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RefgenCfg")
            .field("en", &self.en())
            .field("en_chop", &self.en_chop())
            .field("bm", &self.bm())
            .field("lp_mode", &self.lp_mode())
            .field("lv_mode", &self.lv_mode())
            .field("rzsel", &self.rzsel())
            .field("dischg", &self.dischg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RefgenCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "RefgenCfg {{ en: {=bool:?}, en_chop: {=bool:?}, bm: {=u8:?}, lp_mode: {=bool:?}, lv_mode: {=bool:?}, rzsel: {=u8:?}, dischg: {=bool:?} }}" , self . en () , self . en_chop () , self . bm () , self . lp_mode () , self . lv_mode () , self . rzsel () , self . dischg ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ReservedIn0(pub u32);
impl ReservedIn0 {
    #[doc = "reserved control 0"]
    #[must_use]
    #[inline(always)]
    pub const fn ctrl0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "reserved control 0"]
    #[inline(always)]
    pub const fn set_ctrl0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "reserved control 1"]
    #[must_use]
    #[inline(always)]
    pub const fn ctrl1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "reserved control 1"]
    #[inline(always)]
    pub const fn set_ctrl1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "reserved control 2"]
    #[must_use]
    #[inline(always)]
    pub const fn ctrl2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "reserved control 2"]
    #[inline(always)]
    pub const fn set_ctrl2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "reserved control 3"]
    #[must_use]
    #[inline(always)]
    pub const fn ctrl3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "reserved control 3"]
    #[inline(always)]
    pub const fn set_ctrl3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for ReservedIn0 {
    #[inline(always)]
    fn default() -> ReservedIn0 {
        ReservedIn0(0)
    }
}
impl core::fmt::Debug for ReservedIn0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ReservedIn0")
            .field("ctrl0", &self.ctrl0())
            .field("ctrl1", &self.ctrl1())
            .field("ctrl2", &self.ctrl2())
            .field("ctrl3", &self.ctrl3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ReservedIn0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ReservedIn0 {{ ctrl0: {=u8:?}, ctrl1: {=u8:?}, ctrl2: {=u8:?}, ctrl3: {=u8:?} }}",
            self.ctrl0(),
            self.ctrl1(),
            self.ctrl2(),
            self.ctrl3()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ReservedIn1(pub u32);
impl ReservedIn1 {
    #[doc = "reserved control 4"]
    #[must_use]
    #[inline(always)]
    pub const fn ctrl4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "reserved control 4"]
    #[inline(always)]
    pub const fn set_ctrl4(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "reserved control 5"]
    #[must_use]
    #[inline(always)]
    pub const fn ctrl5(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "reserved control 5"]
    #[inline(always)]
    pub const fn set_ctrl5(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for ReservedIn1 {
    #[inline(always)]
    fn default() -> ReservedIn1 {
        ReservedIn1(0)
    }
}
impl core::fmt::Debug for ReservedIn1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ReservedIn1")
            .field("ctrl4", &self.ctrl4())
            .field("ctrl5", &self.ctrl5())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ReservedIn1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ReservedIn1 {{ ctrl4: {=u8:?}, ctrl5: {=u8:?} }}",
            self.ctrl4(),
            self.ctrl5()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ReservedOut(pub u32);
impl ReservedOut {
    #[doc = "reserved status0"]
    #[must_use]
    #[inline(always)]
    pub const fn stat0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "reserved status0"]
    #[inline(always)]
    pub const fn set_stat0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "reserved status1"]
    #[must_use]
    #[inline(always)]
    pub const fn stat1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "reserved status1"]
    #[inline(always)]
    pub const fn set_stat1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for ReservedOut {
    #[inline(always)]
    fn default() -> ReservedOut {
        ReservedOut(0)
    }
}
impl core::fmt::Debug for ReservedOut {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ReservedOut")
            .field("stat0", &self.stat0())
            .field("stat1", &self.stat1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ReservedOut {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "ReservedOut {{ stat0: {=u8:?}, stat1: {=u8:?} }}",
            self.stat0(),
            self.stat1()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd1(pub u32);
impl Rsvd1 {}
impl Default for Rsvd1 {
    #[inline(always)]
    fn default() -> Rsvd1 {
        Rsvd1(0)
    }
}
impl core::fmt::Debug for Rsvd1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd1").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd1 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd2(pub u32);
impl Rsvd2 {}
impl Default for Rsvd2 {
    #[inline(always)]
    fn default() -> Rsvd2 {
        Rsvd2(0)
    }
}
impl core::fmt::Debug for Rsvd2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd2").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd2 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd3(pub u32);
impl Rsvd3 {}
impl Default for Rsvd3 {
    #[inline(always)]
    fn default() -> Rsvd3 {
        Rsvd3(0)
    }
}
impl core::fmt::Debug for Rsvd3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd3").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd3 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd4(pub u32);
impl Rsvd4 {}
impl Default for Rsvd4 {
    #[inline(always)]
    fn default() -> Rsvd4 {
        Rsvd4(0)
    }
}
impl core::fmt::Debug for Rsvd4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd4").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd4 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd5(pub u32);
impl Rsvd5 {}
impl Default for Rsvd5 {
    #[inline(always)]
    fn default() -> Rsvd5 {
        Rsvd5(0)
    }
}
impl core::fmt::Debug for Rsvd5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd5").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd5 {{ }}",)
    }
}
