#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcPathCfg0(pub u32);
impl AdcPathCfg0 {
    #[doc = "adc left channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB"]
    #[inline(always)]
    pub const fn rough_vol_l(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "adc left channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB"]
    #[inline(always)]
    pub fn set_rough_vol_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "adc left channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute"]
    #[inline(always)]
    pub const fn fine_vol_l(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "adc left channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute"]
    #[inline(always)]
    pub fn set_fine_vol_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "adc right channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB"]
    #[inline(always)]
    pub const fn rough_vol_r(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "adc right channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB"]
    #[inline(always)]
    pub fn set_rough_vol_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "adc right channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute"]
    #[inline(always)]
    pub const fn fine_vol_r(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "adc right channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute"]
    #[inline(always)]
    pub fn set_fine_vol_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "adc path source select 1'h0: select audio codec 1'h1: select external interface"]
    #[inline(always)]
    pub const fn src_sel(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "adc path source select 1'h0: select audio codec 1'h1: select external interface"]
    #[inline(always)]
    pub fn set_src_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "swap adc path left and right channel data"]
    #[inline(always)]
    pub const fn data_swap(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "swap adc path left and right channel data"]
    #[inline(always)]
    pub fn set_data_swap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "rx to tx loopback enable"]
    #[inline(always)]
    pub const fn rx2tx_loopback(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "rx to tx loopback enable"]
    #[inline(always)]
    pub fn set_rx2tx_loopback(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for AdcPathCfg0 {
    #[inline(always)]
    fn default() -> AdcPathCfg0 {
        AdcPathCfg0(0)
    }
}
impl core::fmt::Debug for AdcPathCfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcPathCfg0")
            .field("rough_vol_l", &self.rough_vol_l())
            .field("fine_vol_l", &self.fine_vol_l())
            .field("rough_vol_r", &self.rough_vol_r())
            .field("fine_vol_r", &self.fine_vol_r())
            .field("src_sel", &self.src_sel())
            .field("data_swap", &self.data_swap())
            .field("rx2tx_loopback", &self.rx2tx_loopback())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcPathCfg0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AdcPathCfg0 {
            rough_vol_l: u8,
            fine_vol_l: u8,
            rough_vol_r: u8,
            fine_vol_r: u8,
            src_sel: bool,
            data_swap: bool,
            rx2tx_loopback: bool,
        }
        let proxy = AdcPathCfg0 {
            rough_vol_l: self.rough_vol_l(),
            fine_vol_l: self.fine_vol_l(),
            rough_vol_r: self.rough_vol_r(),
            fine_vol_r: self.fine_vol_r(),
            src_sel: self.src_sel(),
            data_swap: self.data_swap(),
            rx2tx_loopback: self.rx2tx_loopback(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "audprc enable"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "audprc enable"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "audprc software reset, high active"]
    #[inline(always)]
    pub const fn sreset(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "audprc software reset, high active"]
    #[inline(always)]
    pub fn set_sreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "dac path fifo flush, high active"]
    #[inline(always)]
    pub const fn dac_path_flush(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "dac path fifo flush, high active"]
    #[inline(always)]
    pub fn set_dac_path_flush(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "adc path fifo flush, high active"]
    #[inline(always)]
    pub const fn adc_path_flush(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "adc path fifo flush, high active"]
    #[inline(always)]
    pub fn set_adc_path_flush(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "dac path software reset, high active"]
    #[inline(always)]
    pub const fn dac_path_sreset(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "dac path software reset, high active"]
    #[inline(always)]
    pub fn set_dac_path_sreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "adc path software reset, high active"]
    #[inline(always)]
    pub const fn adc_path_sreset(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "adc path software reset, high active"]
    #[inline(always)]
    pub fn set_adc_path_sreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "dac path enable"]
    #[inline(always)]
    pub const fn dac_path_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "dac path enable"]
    #[inline(always)]
    pub fn set_dac_path_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "adc path enable"]
    #[inline(always)]
    pub const fn adc_path_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "adc path enable"]
    #[inline(always)]
    pub fn set_adc_path_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "auto clock gating enable, high active"]
    #[inline(always)]
    pub const fn auto_gate_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "auto clock gating enable, high active"]
    #[inline(always)]
    pub fn set_auto_gate_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "audio strobe clock select 0: use xtal clock to generate strobe 1: use pll clock to generate strobe"]
    #[inline(always)]
    pub const fn stb_clk_sel(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "audio strobe clock select 0: use xtal clock to generate strobe 1: use pll clock to generate strobe"]
    #[inline(always)]
    pub fn set_stb_clk_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "audprc clock divider, 0 and 1 means divide by 1"]
    #[inline(always)]
    pub const fn audclk_div(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "audprc clock divider, 0 and 1 means divide by 1"]
    #[inline(always)]
    pub fn set_audclk_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "audprc clock divider update, write 1 to update"]
    #[inline(always)]
    pub const fn audclk_div_update(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "audprc clock divider update, write 1 to update"]
    #[inline(always)]
    pub fn set_audclk_div_update(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
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
            .field("enable", &self.enable())
            .field("sreset", &self.sreset())
            .field("dac_path_flush", &self.dac_path_flush())
            .field("adc_path_flush", &self.adc_path_flush())
            .field("dac_path_sreset", &self.dac_path_sreset())
            .field("adc_path_sreset", &self.adc_path_sreset())
            .field("dac_path_en", &self.dac_path_en())
            .field("adc_path_en", &self.adc_path_en())
            .field("auto_gate_en", &self.auto_gate_en())
            .field("stb_clk_sel", &self.stb_clk_sel())
            .field("audclk_div", &self.audclk_div())
            .field("audclk_div_update", &self.audclk_div_update())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cfg {
            enable: bool,
            sreset: bool,
            dac_path_flush: bool,
            adc_path_flush: bool,
            dac_path_sreset: bool,
            adc_path_sreset: bool,
            dac_path_en: bool,
            adc_path_en: bool,
            auto_gate_en: bool,
            stb_clk_sel: bool,
            audclk_div: u8,
            audclk_div_update: bool,
        }
        let proxy = Cfg {
            enable: self.enable(),
            sreset: self.sreset(),
            dac_path_flush: self.dac_path_flush(),
            adc_path_flush: self.adc_path_flush(),
            dac_path_sreset: self.dac_path_sreset(),
            adc_path_sreset: self.adc_path_sreset(),
            dac_path_en: self.dac_path_en(),
            adc_path_en: self.adc_path_en(),
            auto_gate_en: self.auto_gate_en(),
            stb_clk_sel: self.stb_clk_sel(),
            audclk_div: self.audclk_div(),
            audclk_div_update: self.audclk_div_update(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg0(pub u32);
impl DacEqCfg0 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg0 {
    #[inline(always)]
    fn default() -> DacEqCfg0 {
        DacEqCfg0(0)
    }
}
impl core::fmt::Debug for DacEqCfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg0")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg0 {
            coef: u32,
        }
        let proxy = DacEqCfg0 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg1(pub u32);
impl DacEqCfg1 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg1 {
    #[inline(always)]
    fn default() -> DacEqCfg1 {
        DacEqCfg1(0)
    }
}
impl core::fmt::Debug for DacEqCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg1")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg1 {
            coef: u32,
        }
        let proxy = DacEqCfg1 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg10(pub u32);
impl DacEqCfg10 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg10 {
    #[inline(always)]
    fn default() -> DacEqCfg10 {
        DacEqCfg10(0)
    }
}
impl core::fmt::Debug for DacEqCfg10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg10")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg10 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg10 {
            coef: u32,
        }
        let proxy = DacEqCfg10 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg11(pub u32);
impl DacEqCfg11 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg11 {
    #[inline(always)]
    fn default() -> DacEqCfg11 {
        DacEqCfg11(0)
    }
}
impl core::fmt::Debug for DacEqCfg11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg11")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg11 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg11 {
            coef: u32,
        }
        let proxy = DacEqCfg11 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg12(pub u32);
impl DacEqCfg12 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg12 {
    #[inline(always)]
    fn default() -> DacEqCfg12 {
        DacEqCfg12(0)
    }
}
impl core::fmt::Debug for DacEqCfg12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg12")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg12 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg12 {
            coef: u32,
        }
        let proxy = DacEqCfg12 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg13(pub u32);
impl DacEqCfg13 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg13 {
    #[inline(always)]
    fn default() -> DacEqCfg13 {
        DacEqCfg13(0)
    }
}
impl core::fmt::Debug for DacEqCfg13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg13")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg13 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg13 {
            coef: u32,
        }
        let proxy = DacEqCfg13 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg14(pub u32);
impl DacEqCfg14 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg14 {
    #[inline(always)]
    fn default() -> DacEqCfg14 {
        DacEqCfg14(0)
    }
}
impl core::fmt::Debug for DacEqCfg14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg14")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg14 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg14 {
            coef: u32,
        }
        let proxy = DacEqCfg14 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg15(pub u32);
impl DacEqCfg15 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg15 {
    #[inline(always)]
    fn default() -> DacEqCfg15 {
        DacEqCfg15(0)
    }
}
impl core::fmt::Debug for DacEqCfg15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg15")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg15 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg15 {
            coef: u32,
        }
        let proxy = DacEqCfg15 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg16(pub u32);
impl DacEqCfg16 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg16 {
    #[inline(always)]
    fn default() -> DacEqCfg16 {
        DacEqCfg16(0)
    }
}
impl core::fmt::Debug for DacEqCfg16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg16")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg16 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg16 {
            coef: u32,
        }
        let proxy = DacEqCfg16 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg17(pub u32);
impl DacEqCfg17 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg17 {
    #[inline(always)]
    fn default() -> DacEqCfg17 {
        DacEqCfg17(0)
    }
}
impl core::fmt::Debug for DacEqCfg17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg17")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg17 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg17 {
            coef: u32,
        }
        let proxy = DacEqCfg17 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg18(pub u32);
impl DacEqCfg18 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg18 {
    #[inline(always)]
    fn default() -> DacEqCfg18 {
        DacEqCfg18(0)
    }
}
impl core::fmt::Debug for DacEqCfg18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg18")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg18 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg18 {
            coef: u32,
        }
        let proxy = DacEqCfg18 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg19(pub u32);
impl DacEqCfg19 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg19 {
    #[inline(always)]
    fn default() -> DacEqCfg19 {
        DacEqCfg19(0)
    }
}
impl core::fmt::Debug for DacEqCfg19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg19")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg19 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg19 {
            coef: u32,
        }
        let proxy = DacEqCfg19 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg2(pub u32);
impl DacEqCfg2 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg2 {
    #[inline(always)]
    fn default() -> DacEqCfg2 {
        DacEqCfg2(0)
    }
}
impl core::fmt::Debug for DacEqCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg2")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg2 {
            coef: u32,
        }
        let proxy = DacEqCfg2 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg20(pub u32);
impl DacEqCfg20 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg20 {
    #[inline(always)]
    fn default() -> DacEqCfg20 {
        DacEqCfg20(0)
    }
}
impl core::fmt::Debug for DacEqCfg20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg20")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg20 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg20 {
            coef: u32,
        }
        let proxy = DacEqCfg20 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg21(pub u32);
impl DacEqCfg21 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg21 {
    #[inline(always)]
    fn default() -> DacEqCfg21 {
        DacEqCfg21(0)
    }
}
impl core::fmt::Debug for DacEqCfg21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg21")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg21 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg21 {
            coef: u32,
        }
        let proxy = DacEqCfg21 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg22(pub u32);
impl DacEqCfg22 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg22 {
    #[inline(always)]
    fn default() -> DacEqCfg22 {
        DacEqCfg22(0)
    }
}
impl core::fmt::Debug for DacEqCfg22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg22")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg22 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg22 {
            coef: u32,
        }
        let proxy = DacEqCfg22 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg23(pub u32);
impl DacEqCfg23 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg23 {
    #[inline(always)]
    fn default() -> DacEqCfg23 {
        DacEqCfg23(0)
    }
}
impl core::fmt::Debug for DacEqCfg23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg23")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg23 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg23 {
            coef: u32,
        }
        let proxy = DacEqCfg23 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg24(pub u32);
impl DacEqCfg24 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg24 {
    #[inline(always)]
    fn default() -> DacEqCfg24 {
        DacEqCfg24(0)
    }
}
impl core::fmt::Debug for DacEqCfg24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg24")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg24 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg24 {
            coef: u32,
        }
        let proxy = DacEqCfg24 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg25(pub u32);
impl DacEqCfg25 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg25 {
    #[inline(always)]
    fn default() -> DacEqCfg25 {
        DacEqCfg25(0)
    }
}
impl core::fmt::Debug for DacEqCfg25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg25")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg25 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg25 {
            coef: u32,
        }
        let proxy = DacEqCfg25 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg26(pub u32);
impl DacEqCfg26 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg26 {
    #[inline(always)]
    fn default() -> DacEqCfg26 {
        DacEqCfg26(0)
    }
}
impl core::fmt::Debug for DacEqCfg26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg26")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg26 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg26 {
            coef: u32,
        }
        let proxy = DacEqCfg26 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg27(pub u32);
impl DacEqCfg27 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg27 {
    #[inline(always)]
    fn default() -> DacEqCfg27 {
        DacEqCfg27(0)
    }
}
impl core::fmt::Debug for DacEqCfg27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg27")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg27 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg27 {
            coef: u32,
        }
        let proxy = DacEqCfg27 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg28(pub u32);
impl DacEqCfg28 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg28 {
    #[inline(always)]
    fn default() -> DacEqCfg28 {
        DacEqCfg28(0)
    }
}
impl core::fmt::Debug for DacEqCfg28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg28")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg28 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg28 {
            coef: u32,
        }
        let proxy = DacEqCfg28 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg29(pub u32);
impl DacEqCfg29 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg29 {
    #[inline(always)]
    fn default() -> DacEqCfg29 {
        DacEqCfg29(0)
    }
}
impl core::fmt::Debug for DacEqCfg29 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg29")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg29 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg29 {
            coef: u32,
        }
        let proxy = DacEqCfg29 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg3(pub u32);
impl DacEqCfg3 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg3 {
    #[inline(always)]
    fn default() -> DacEqCfg3 {
        DacEqCfg3(0)
    }
}
impl core::fmt::Debug for DacEqCfg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg3")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg3 {
            coef: u32,
        }
        let proxy = DacEqCfg3 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg30(pub u32);
impl DacEqCfg30 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg30 {
    #[inline(always)]
    fn default() -> DacEqCfg30 {
        DacEqCfg30(0)
    }
}
impl core::fmt::Debug for DacEqCfg30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg30")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg30 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg30 {
            coef: u32,
        }
        let proxy = DacEqCfg30 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg31(pub u32);
impl DacEqCfg31 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg31 {
    #[inline(always)]
    fn default() -> DacEqCfg31 {
        DacEqCfg31(0)
    }
}
impl core::fmt::Debug for DacEqCfg31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg31")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg31 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg31 {
            coef: u32,
        }
        let proxy = DacEqCfg31 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg32(pub u32);
impl DacEqCfg32 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg32 {
    #[inline(always)]
    fn default() -> DacEqCfg32 {
        DacEqCfg32(0)
    }
}
impl core::fmt::Debug for DacEqCfg32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg32")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg32 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg32 {
            coef: u32,
        }
        let proxy = DacEqCfg32 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg33(pub u32);
impl DacEqCfg33 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg33 {
    #[inline(always)]
    fn default() -> DacEqCfg33 {
        DacEqCfg33(0)
    }
}
impl core::fmt::Debug for DacEqCfg33 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg33")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg33 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg33 {
            coef: u32,
        }
        let proxy = DacEqCfg33 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg34(pub u32);
impl DacEqCfg34 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg34 {
    #[inline(always)]
    fn default() -> DacEqCfg34 {
        DacEqCfg34(0)
    }
}
impl core::fmt::Debug for DacEqCfg34 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg34")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg34 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg34 {
            coef: u32,
        }
        let proxy = DacEqCfg34 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg35(pub u32);
impl DacEqCfg35 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg35 {
    #[inline(always)]
    fn default() -> DacEqCfg35 {
        DacEqCfg35(0)
    }
}
impl core::fmt::Debug for DacEqCfg35 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg35")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg35 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg35 {
            coef: u32,
        }
        let proxy = DacEqCfg35 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg36(pub u32);
impl DacEqCfg36 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg36 {
    #[inline(always)]
    fn default() -> DacEqCfg36 {
        DacEqCfg36(0)
    }
}
impl core::fmt::Debug for DacEqCfg36 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg36")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg36 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg36 {
            coef: u32,
        }
        let proxy = DacEqCfg36 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg37(pub u32);
impl DacEqCfg37 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg37 {
    #[inline(always)]
    fn default() -> DacEqCfg37 {
        DacEqCfg37(0)
    }
}
impl core::fmt::Debug for DacEqCfg37 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg37")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg37 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg37 {
            coef: u32,
        }
        let proxy = DacEqCfg37 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg38(pub u32);
impl DacEqCfg38 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg38 {
    #[inline(always)]
    fn default() -> DacEqCfg38 {
        DacEqCfg38(0)
    }
}
impl core::fmt::Debug for DacEqCfg38 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg38")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg38 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg38 {
            coef: u32,
        }
        let proxy = DacEqCfg38 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg39(pub u32);
impl DacEqCfg39 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg39 {
    #[inline(always)]
    fn default() -> DacEqCfg39 {
        DacEqCfg39(0)
    }
}
impl core::fmt::Debug for DacEqCfg39 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg39")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg39 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg39 {
            coef: u32,
        }
        let proxy = DacEqCfg39 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg4(pub u32);
impl DacEqCfg4 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg4 {
    #[inline(always)]
    fn default() -> DacEqCfg4 {
        DacEqCfg4(0)
    }
}
impl core::fmt::Debug for DacEqCfg4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg4")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg4 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg4 {
            coef: u32,
        }
        let proxy = DacEqCfg4 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg40(pub u32);
impl DacEqCfg40 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg40 {
    #[inline(always)]
    fn default() -> DacEqCfg40 {
        DacEqCfg40(0)
    }
}
impl core::fmt::Debug for DacEqCfg40 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg40")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg40 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg40 {
            coef: u32,
        }
        let proxy = DacEqCfg40 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg41(pub u32);
impl DacEqCfg41 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg41 {
    #[inline(always)]
    fn default() -> DacEqCfg41 {
        DacEqCfg41(0)
    }
}
impl core::fmt::Debug for DacEqCfg41 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg41")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg41 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg41 {
            coef: u32,
        }
        let proxy = DacEqCfg41 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg42(pub u32);
impl DacEqCfg42 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg42 {
    #[inline(always)]
    fn default() -> DacEqCfg42 {
        DacEqCfg42(0)
    }
}
impl core::fmt::Debug for DacEqCfg42 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg42")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg42 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg42 {
            coef: u32,
        }
        let proxy = DacEqCfg42 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg43(pub u32);
impl DacEqCfg43 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg43 {
    #[inline(always)]
    fn default() -> DacEqCfg43 {
        DacEqCfg43(0)
    }
}
impl core::fmt::Debug for DacEqCfg43 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg43")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg43 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg43 {
            coef: u32,
        }
        let proxy = DacEqCfg43 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg44(pub u32);
impl DacEqCfg44 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg44 {
    #[inline(always)]
    fn default() -> DacEqCfg44 {
        DacEqCfg44(0)
    }
}
impl core::fmt::Debug for DacEqCfg44 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg44")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg44 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg44 {
            coef: u32,
        }
        let proxy = DacEqCfg44 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg45(pub u32);
impl DacEqCfg45 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg45 {
    #[inline(always)]
    fn default() -> DacEqCfg45 {
        DacEqCfg45(0)
    }
}
impl core::fmt::Debug for DacEqCfg45 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg45")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg45 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg45 {
            coef: u32,
        }
        let proxy = DacEqCfg45 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg46(pub u32);
impl DacEqCfg46 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg46 {
    #[inline(always)]
    fn default() -> DacEqCfg46 {
        DacEqCfg46(0)
    }
}
impl core::fmt::Debug for DacEqCfg46 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg46")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg46 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg46 {
            coef: u32,
        }
        let proxy = DacEqCfg46 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg47(pub u32);
impl DacEqCfg47 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg47 {
    #[inline(always)]
    fn default() -> DacEqCfg47 {
        DacEqCfg47(0)
    }
}
impl core::fmt::Debug for DacEqCfg47 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg47")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg47 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg47 {
            coef: u32,
        }
        let proxy = DacEqCfg47 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg48(pub u32);
impl DacEqCfg48 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg48 {
    #[inline(always)]
    fn default() -> DacEqCfg48 {
        DacEqCfg48(0)
    }
}
impl core::fmt::Debug for DacEqCfg48 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg48")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg48 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg48 {
            coef: u32,
        }
        let proxy = DacEqCfg48 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg49(pub u32);
impl DacEqCfg49 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg49 {
    #[inline(always)]
    fn default() -> DacEqCfg49 {
        DacEqCfg49(0)
    }
}
impl core::fmt::Debug for DacEqCfg49 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg49")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg49 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg49 {
            coef: u32,
        }
        let proxy = DacEqCfg49 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg5(pub u32);
impl DacEqCfg5 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg5 {
    #[inline(always)]
    fn default() -> DacEqCfg5 {
        DacEqCfg5(0)
    }
}
impl core::fmt::Debug for DacEqCfg5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg5")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg5 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg5 {
            coef: u32,
        }
        let proxy = DacEqCfg5 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg6(pub u32);
impl DacEqCfg6 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg6 {
    #[inline(always)]
    fn default() -> DacEqCfg6 {
        DacEqCfg6(0)
    }
}
impl core::fmt::Debug for DacEqCfg6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg6")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg6 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg6 {
            coef: u32,
        }
        let proxy = DacEqCfg6 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg7(pub u32);
impl DacEqCfg7 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg7 {
    #[inline(always)]
    fn default() -> DacEqCfg7 {
        DacEqCfg7(0)
    }
}
impl core::fmt::Debug for DacEqCfg7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg7")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg7 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg7 {
            coef: u32,
        }
        let proxy = DacEqCfg7 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg8(pub u32);
impl DacEqCfg8 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg8 {
    #[inline(always)]
    fn default() -> DacEqCfg8 {
        DacEqCfg8(0)
    }
}
impl core::fmt::Debug for DacEqCfg8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg8")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg8 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg8 {
            coef: u32,
        }
        let proxy = DacEqCfg8 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacEqCfg9(pub u32);
impl DacEqCfg9 {
    #[inline(always)]
    pub const fn coef(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_coef(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DacEqCfg9 {
    #[inline(always)]
    fn default() -> DacEqCfg9 {
        DacEqCfg9(0)
    }
}
impl core::fmt::Debug for DacEqCfg9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacEqCfg9")
            .field("coef", &self.coef())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacEqCfg9 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacEqCfg9 {
            coef: u32,
        }
        let proxy = DacEqCfg9 { coef: self.coef() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacPathCfg0(pub u32);
impl DacPathCfg0 {
    #[doc = "dac mixer left channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB"]
    #[inline(always)]
    pub const fn rough_vol_l(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "dac mixer left channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB"]
    #[inline(always)]
    pub fn set_rough_vol_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "dac mixer left channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute"]
    #[inline(always)]
    pub const fn fine_vol_l(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "dac mixer left channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute"]
    #[inline(always)]
    pub fn set_fine_vol_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "dac mixer right channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB"]
    #[inline(always)]
    pub const fn rough_vol_r(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "dac mixer right channel rough volume control range from -36dB to 54dB step is 6dB 4'h0: -36dB 4'h1: -30dB ...... 4'h6: 0dB ...... 4'he: 48dB 4'hf: 54dB"]
    #[inline(always)]
    pub fn set_rough_vol_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "dac mixer right channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute"]
    #[inline(always)]
    pub const fn fine_vol_r(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "dac mixer right channel fine volume control range from 0dB to 6dB step is 0.5dB 4'h0: 0dB 4'h1: 0.5dB ...... 4'hb: 5.5dB 4'hc, 4'hd, 4'he, 4'hf: mute"]
    #[inline(always)]
    pub fn set_fine_vol_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "dac mixer left channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute"]
    #[inline(always)]
    pub const fn mixlsrc0(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "dac mixer left channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute"]
    #[inline(always)]
    pub fn set_mixlsrc0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "dac mixer left channel input source1 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute"]
    #[inline(always)]
    pub const fn mixlsrc1(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x07;
        val as u8
    }
    #[doc = "dac mixer left channel input source1 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute"]
    #[inline(always)]
    pub fn set_mixlsrc1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 19usize)) | (((val as u32) & 0x07) << 19usize);
    }
    #[doc = "dac mixer right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute"]
    #[inline(always)]
    pub const fn mixrsrc0(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x07;
        val as u8
    }
    #[doc = "dac mixer right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute"]
    #[inline(always)]
    pub fn set_mixrsrc0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
    }
    #[doc = "dac mixer right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute"]
    #[inline(always)]
    pub const fn mixrsrc1(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x07;
        val as u8
    }
    #[doc = "dac mixer right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:tx ch2 3'h3:tx ch3 3'h4:mute other: mute"]
    #[inline(always)]
    pub fn set_mixrsrc1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
    }
    #[doc = "dac path destination select 2'h0: select audio codec 2'h1: select external interface 2'h2: select apb interface 2'h3: reserved"]
    #[inline(always)]
    pub const fn dst_sel(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "dac path destination select 2'h0: select audio codec 2'h1: select external interface 2'h2: select apb interface 2'h3: reserved"]
    #[inline(always)]
    pub fn set_dst_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
}
impl Default for DacPathCfg0 {
    #[inline(always)]
    fn default() -> DacPathCfg0 {
        DacPathCfg0(0)
    }
}
impl core::fmt::Debug for DacPathCfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacPathCfg0")
            .field("rough_vol_l", &self.rough_vol_l())
            .field("fine_vol_l", &self.fine_vol_l())
            .field("rough_vol_r", &self.rough_vol_r())
            .field("fine_vol_r", &self.fine_vol_r())
            .field("mixlsrc0", &self.mixlsrc0())
            .field("mixlsrc1", &self.mixlsrc1())
            .field("mixrsrc0", &self.mixrsrc0())
            .field("mixrsrc1", &self.mixrsrc1())
            .field("dst_sel", &self.dst_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacPathCfg0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacPathCfg0 {
            rough_vol_l: u8,
            fine_vol_l: u8,
            rough_vol_r: u8,
            fine_vol_r: u8,
            mixlsrc0: u8,
            mixlsrc1: u8,
            mixrsrc0: u8,
            mixrsrc1: u8,
            dst_sel: u8,
        }
        let proxy = DacPathCfg0 {
            rough_vol_l: self.rough_vol_l(),
            fine_vol_l: self.fine_vol_l(),
            rough_vol_r: self.rough_vol_r(),
            fine_vol_r: self.fine_vol_r(),
            mixlsrc0: self.mixlsrc0(),
            mixlsrc1: self.mixlsrc1(),
            mixrsrc0: self.mixrsrc0(),
            mixrsrc1: self.mixrsrc1(),
            dst_sel: self.dst_sel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacPathCfg1(pub u32);
impl DacPathCfg1 {
    #[doc = "dac mux left channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute"]
    #[inline(always)]
    pub const fn muxlsrc0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "dac mux left channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute"]
    #[inline(always)]
    pub fn set_muxlsrc0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "dac mux left channel input source1 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute"]
    #[inline(always)]
    pub const fn muxlsrc1(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "dac mux left channel input source1 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute"]
    #[inline(always)]
    pub fn set_muxlsrc1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "dac mux right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute"]
    #[inline(always)]
    pub const fn muxrsrc0(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "dac mux right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute"]
    #[inline(always)]
    pub fn set_muxrsrc0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "dac mux right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute"]
    #[inline(always)]
    pub const fn muxrsrc1(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "dac mux right channel input source0 select 3'h0:tx ch0 3'h1:tx ch1 3'h2:rx ch0 3'h3:rx ch1 3'h4:mute other: mute"]
    #[inline(always)]
    pub fn set_muxrsrc1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "equalizer channel enable 2'b11: enable both channel 2'b10: enable right chanel only 2'b01: enable left channel only 2'b00: bypass equalizer"]
    #[inline(always)]
    pub const fn eq_ch_en(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "equalizer channel enable 2'b11: enable both channel 2'b10: enable right chanel only 2'b01: enable left channel only 2'b00: bypass equalizer"]
    #[inline(always)]
    pub fn set_eq_ch_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "set equalizer stage, max is 10."]
    #[inline(always)]
    pub const fn eq_stage(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x0f;
        val as u8
    }
    #[doc = "set equalizer stage, max is 10."]
    #[inline(always)]
    pub fn set_eq_stage(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 14usize)) | (((val as u32) & 0x0f) << 14usize);
    }
    #[doc = "equalizer clear done flag"]
    #[inline(always)]
    pub const fn eq_clr_done(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "equalizer clear done flag"]
    #[inline(always)]
    pub fn set_eq_clr_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "equalizer clear request"]
    #[inline(always)]
    pub const fn eq_clr(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "equalizer clear request"]
    #[inline(always)]
    pub fn set_eq_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "source rate converter channel enable"]
    #[inline(always)]
    pub const fn src_ch_en(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "source rate converter channel enable"]
    #[inline(always)]
    pub fn set_src_ch_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "1st stage hbf enable"]
    #[inline(always)]
    pub const fn src_hbf1_en(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "1st stage hbf enable"]
    #[inline(always)]
    pub fn set_src_hbf1_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "1st stage hbf mode: 0: upsampling 1: downsampling"]
    #[inline(always)]
    pub const fn src_hbf1_mode(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "1st stage hbf mode: 0: upsampling 1: downsampling"]
    #[inline(always)]
    pub fn set_src_hbf1_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "2nd stage hbf enable"]
    #[inline(always)]
    pub const fn src_hbf2_en(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "2nd stage hbf enable"]
    #[inline(always)]
    pub fn set_src_hbf2_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "2nd stage hbf mode: 0: upsampling 1: downsampling"]
    #[inline(always)]
    pub const fn src_hbf2_mode(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "2nd stage hbf mode: 0: upsampling 1: downsampling"]
    #[inline(always)]
    pub fn set_src_hbf2_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "3rd stage hbf enable"]
    #[inline(always)]
    pub const fn src_hbf3_en(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "3rd stage hbf enable"]
    #[inline(always)]
    pub fn set_src_hbf3_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "3rd stage hbf mode: 0: upsampling 1: downsampling"]
    #[inline(always)]
    pub const fn src_hbf3_mode(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "3rd stage hbf mode: 0: upsampling 1: downsampling"]
    #[inline(always)]
    pub fn set_src_hbf3_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "src channel internal data clear done"]
    #[inline(always)]
    pub const fn src_ch_clr_done(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[doc = "src channel internal data clear done"]
    #[inline(always)]
    pub fn set_src_ch_clr_done(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[doc = "clear src channal internal data"]
    #[inline(always)]
    pub const fn src_ch_clr(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "clear src channal internal data"]
    #[inline(always)]
    pub fn set_src_ch_clr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for DacPathCfg1 {
    #[inline(always)]
    fn default() -> DacPathCfg1 {
        DacPathCfg1(0)
    }
}
impl core::fmt::Debug for DacPathCfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacPathCfg1")
            .field("muxlsrc0", &self.muxlsrc0())
            .field("muxlsrc1", &self.muxlsrc1())
            .field("muxrsrc0", &self.muxrsrc0())
            .field("muxrsrc1", &self.muxrsrc1())
            .field("eq_ch_en", &self.eq_ch_en())
            .field("eq_stage", &self.eq_stage())
            .field("eq_clr_done", &self.eq_clr_done())
            .field("eq_clr", &self.eq_clr())
            .field("src_ch_en", &self.src_ch_en())
            .field("src_hbf1_en", &self.src_hbf1_en())
            .field("src_hbf1_mode", &self.src_hbf1_mode())
            .field("src_hbf2_en", &self.src_hbf2_en())
            .field("src_hbf2_mode", &self.src_hbf2_mode())
            .field("src_hbf3_en", &self.src_hbf3_en())
            .field("src_hbf3_mode", &self.src_hbf3_mode())
            .field("src_ch_clr_done", &self.src_ch_clr_done())
            .field("src_ch_clr", &self.src_ch_clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacPathCfg1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacPathCfg1 {
            muxlsrc0: u8,
            muxlsrc1: u8,
            muxrsrc0: u8,
            muxrsrc1: u8,
            eq_ch_en: u8,
            eq_stage: u8,
            eq_clr_done: bool,
            eq_clr: bool,
            src_ch_en: u8,
            src_hbf1_en: bool,
            src_hbf1_mode: bool,
            src_hbf2_en: bool,
            src_hbf2_mode: bool,
            src_hbf3_en: bool,
            src_hbf3_mode: bool,
            src_ch_clr_done: u8,
            src_ch_clr: u8,
        }
        let proxy = DacPathCfg1 {
            muxlsrc0: self.muxlsrc0(),
            muxlsrc1: self.muxlsrc1(),
            muxrsrc0: self.muxrsrc0(),
            muxrsrc1: self.muxrsrc1(),
            eq_ch_en: self.eq_ch_en(),
            eq_stage: self.eq_stage(),
            eq_clr_done: self.eq_clr_done(),
            eq_clr: self.eq_clr(),
            src_ch_en: self.src_ch_en(),
            src_hbf1_en: self.src_hbf1_en(),
            src_hbf1_mode: self.src_hbf1_mode(),
            src_hbf2_en: self.src_hbf2_en(),
            src_hbf2_mode: self.src_hbf2_mode(),
            src_hbf3_en: self.src_hbf3_en(),
            src_hbf3_mode: self.src_hbf3_mode(),
            src_ch_clr_done: self.src_ch_clr_done(),
            src_ch_clr: self.src_ch_clr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacPathCfg2(pub u32);
impl DacPathCfg2 {
    #[doc = "sinc filter ratio, s31.30 format. Range from 0~2"]
    #[inline(always)]
    pub const fn sinc_ratio(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "sinc filter ratio, s31.30 format. Range from 0~2"]
    #[inline(always)]
    pub fn set_sinc_ratio(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "sinc filter enable"]
    #[inline(always)]
    pub const fn src_sinc_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "sinc filter enable"]
    #[inline(always)]
    pub fn set_src_sinc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for DacPathCfg2 {
    #[inline(always)]
    fn default() -> DacPathCfg2 {
        DacPathCfg2(0)
    }
}
impl core::fmt::Debug for DacPathCfg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacPathCfg2")
            .field("sinc_ratio", &self.sinc_ratio())
            .field("src_sinc_en", &self.src_sinc_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacPathCfg2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacPathCfg2 {
            sinc_ratio: u32,
            src_sinc_en: bool,
        }
        let proxy = DacPathCfg2 {
            sinc_ratio: self.sinc_ratio(),
            src_sinc_en: self.src_sinc_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DacPathCfg3(pub u32);
impl DacPathCfg3 {
    #[doc = "dac mixer left channel volume ramp enable"]
    #[inline(always)]
    pub const fn ramp_en_l(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "dac mixer left channel volume ramp enable"]
    #[inline(always)]
    pub fn set_ramp_en_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "dac mixer left channel volume ramp mode: 1: slowly ramp to target volume. Step is 0.5db 0: directly ramp to target volume."]
    #[inline(always)]
    pub const fn ramp_mode_l(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "dac mixer left channel volume ramp mode: 1: slowly ramp to target volume. Step is 0.5db 0: directly ramp to target volume."]
    #[inline(always)]
    pub fn set_ramp_mode_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "dac mixer left channel volume adjustment during 0 volume cross enable"]
    #[inline(always)]
    pub const fn zero_adjust_en_l(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "dac mixer left channel volume adjustment during 0 volume cross enable"]
    #[inline(always)]
    pub fn set_zero_adjust_en_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "dac mixer left channel volume ramp interval."]
    #[inline(always)]
    pub const fn ramp_interval_l(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "dac mixer left channel volume ramp interval."]
    #[inline(always)]
    pub fn set_ramp_interval_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[doc = "dac mixer left channel ramp module status"]
    #[inline(always)]
    pub const fn ramp_stat_l(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x03;
        val as u8
    }
    #[doc = "dac mixer left channel ramp module status"]
    #[inline(always)]
    pub fn set_ramp_stat_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 7usize)) | (((val as u32) & 0x03) << 7usize);
    }
    #[doc = "dac mixer right channel volume ramp enable"]
    #[inline(always)]
    pub const fn ramp_en_r(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "dac mixer right channel volume ramp enable"]
    #[inline(always)]
    pub fn set_ramp_en_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "dac mixer right channel volume ramp mode: 1: slowly ramp to target volume. Step is 0.5db 0: directly ramp to target volume."]
    #[inline(always)]
    pub const fn ramp_mode_r(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "dac mixer right channel volume ramp mode: 1: slowly ramp to target volume. Step is 0.5db 0: directly ramp to target volume."]
    #[inline(always)]
    pub fn set_ramp_mode_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "dac mixer right channel volume adjustment during 0 volume cross enable"]
    #[inline(always)]
    pub const fn zero_adjust_en_r(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "dac mixer right channel volume adjustment during 0 volume cross enable"]
    #[inline(always)]
    pub fn set_zero_adjust_en_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "dac mixer right channel volume ramp interval."]
    #[inline(always)]
    pub const fn ramp_interval_r(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "dac mixer right channel volume ramp interval."]
    #[inline(always)]
    pub fn set_ramp_interval_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "dac mixer right channel ramp module status"]
    #[inline(always)]
    pub const fn ramp_stat_r(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "dac mixer right channel ramp module status"]
    #[inline(always)]
    pub fn set_ramp_stat_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for DacPathCfg3 {
    #[inline(always)]
    fn default() -> DacPathCfg3 {
        DacPathCfg3(0)
    }
}
impl core::fmt::Debug for DacPathCfg3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DacPathCfg3")
            .field("ramp_en_l", &self.ramp_en_l())
            .field("ramp_mode_l", &self.ramp_mode_l())
            .field("zero_adjust_en_l", &self.zero_adjust_en_l())
            .field("ramp_interval_l", &self.ramp_interval_l())
            .field("ramp_stat_l", &self.ramp_stat_l())
            .field("ramp_en_r", &self.ramp_en_r())
            .field("ramp_mode_r", &self.ramp_mode_r())
            .field("zero_adjust_en_r", &self.zero_adjust_en_r())
            .field("ramp_interval_r", &self.ramp_interval_r())
            .field("ramp_stat_r", &self.ramp_stat_r())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DacPathCfg3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DacPathCfg3 {
            ramp_en_l: bool,
            ramp_mode_l: bool,
            zero_adjust_en_l: bool,
            ramp_interval_l: u8,
            ramp_stat_l: u8,
            ramp_en_r: bool,
            ramp_mode_r: bool,
            zero_adjust_en_r: bool,
            ramp_interval_r: u8,
            ramp_stat_r: u8,
        }
        let proxy = DacPathCfg3 {
            ramp_en_l: self.ramp_en_l(),
            ramp_mode_l: self.ramp_mode_l(),
            zero_adjust_en_l: self.zero_adjust_en_l(),
            ramp_interval_l: self.ramp_interval_l(),
            ramp_stat_l: self.ramp_stat_l(),
            ramp_en_r: self.ramp_en_r(),
            ramp_mode_r: self.ramp_mode_r(),
            zero_adjust_en_r: self.zero_adjust_en_r(),
            ramp_interval_r: self.ramp_interval_r(),
            ramp_stat_r: self.ramp_stat_r(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id(pub u32);
impl Id {
    #[doc = "revision id"]
    #[inline(always)]
    pub const fn rev(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "revision id"]
    #[inline(always)]
    pub fn set_rev(&mut self, val: u32) {
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
        f.debug_struct("Id").field("rev", &self.rev()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Id {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Id {
            rev: u32,
        }
        let proxy = Id { rev: self.rev() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irq(pub u32);
impl Irq {
    #[doc = "tx channel 0 fifo overflow, write 1 to clear"]
    #[inline(always)]
    pub const fn tx0_fifo_of(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "tx channel 0 fifo overflow, write 1 to clear"]
    #[inline(always)]
    pub fn set_tx0_fifo_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "tx channel 1 fifo overflow, write 1 to clear"]
    #[inline(always)]
    pub const fn tx1_fifo_of(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "tx channel 1 fifo overflow, write 1 to clear"]
    #[inline(always)]
    pub fn set_tx1_fifo_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "tx channel 2 fifo overflow, write 1 to clear"]
    #[inline(always)]
    pub const fn tx2_fifo_of(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "tx channel 2 fifo overflow, write 1 to clear"]
    #[inline(always)]
    pub fn set_tx2_fifo_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "tx channel 3 fifo overflow, write 1 to clear"]
    #[inline(always)]
    pub const fn tx3_fifo_of(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "tx channel 3 fifo overflow, write 1 to clear"]
    #[inline(always)]
    pub fn set_tx3_fifo_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "rx channel 0 fifo underflow, write 1 to clear"]
    #[inline(always)]
    pub const fn rx0_fifo_uf(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "rx channel 0 fifo underflow, write 1 to clear"]
    #[inline(always)]
    pub fn set_rx0_fifo_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "rx channel 1 fifo underflow, write 1 to clear"]
    #[inline(always)]
    pub const fn rx1_fifo_uf(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "rx channel 1 fifo underflow, write 1 to clear"]
    #[inline(always)]
    pub fn set_rx1_fifo_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "tx output fifo underflow, write 1 to clear"]
    #[inline(always)]
    pub const fn tx_out_fifo_uf(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "tx output fifo underflow, write 1 to clear"]
    #[inline(always)]
    pub fn set_tx_out_fifo_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "rx input fifo overflow, write 1 to clear"]
    #[inline(always)]
    pub const fn rx_in_fifo_of(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "rx input fifo overflow, write 1 to clear"]
    #[inline(always)]
    pub fn set_rx_in_fifo_of(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "tx_out channel 0 fifo underflow, write 1 to clear"]
    #[inline(always)]
    pub const fn tx_out0_fifo_uf(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "tx_out channel 0 fifo underflow, write 1 to clear"]
    #[inline(always)]
    pub fn set_tx_out0_fifo_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "tx_out channel 1 fifo underflow, write 1 to clear"]
    #[inline(always)]
    pub const fn tx_out1_fifo_uf(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "tx_out channel 1 fifo underflow, write 1 to clear"]
    #[inline(always)]
    pub fn set_tx_out1_fifo_uf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "tx channel 0 fifo overflow mask, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn tx0_fifo_of_mask(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "tx channel 0 fifo overflow mask, 0: mask the interrupt"]
    #[inline(always)]
    pub fn set_tx0_fifo_of_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "tx channel 1 fifo overflow mask, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn tx1_fifo_of_mask(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "tx channel 1 fifo overflow mask, 0: mask the interrupt"]
    #[inline(always)]
    pub fn set_tx1_fifo_of_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "tx channel 2 fifo overflow mask, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn tx2_fifo_of_mask(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "tx channel 2 fifo overflow mask, 0: mask the interrupt"]
    #[inline(always)]
    pub fn set_tx2_fifo_of_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "tx channel 3 fifo overflow mask, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn tx3_fifo_of_mask(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "tx channel 3 fifo overflow mask, 0: mask the interrupt"]
    #[inline(always)]
    pub fn set_tx3_fifo_of_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "rx channel 0 fifo underflow mask, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn rx0_fifo_uf_mask(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "rx channel 0 fifo underflow mask, 0: mask the interrupt"]
    #[inline(always)]
    pub fn set_rx0_fifo_uf_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "rx channel 1 fifo underflow mask, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn rx1_fifo_uf_mask(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "rx channel 1 fifo underflow mask, 0: mask the interrupt"]
    #[inline(always)]
    pub fn set_rx1_fifo_uf_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "tx output fifo underflow mask, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn tx_out_fifo_uf_mask(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "tx output fifo underflow mask, 0: mask the interrupt"]
    #[inline(always)]
    pub fn set_tx_out_fifo_uf_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "rx input fifo overflow mask, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn rx_in_fifo_of_mask(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "rx input fifo overflow mask, 0: mask the interrupt"]
    #[inline(always)]
    pub fn set_rx_in_fifo_of_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "tx_out channel 0 fifo underflow mask, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn tx_out0_fifo_uf_mask(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "tx_out channel 0 fifo underflow mask, 0: mask the interrupt"]
    #[inline(always)]
    pub fn set_tx_out0_fifo_uf_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "tx_out channel 1 fifo underflow mask, 0: mask the interrupt"]
    #[inline(always)]
    pub const fn tx_out1_fifo_uf_mask(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "tx_out channel 1 fifo underflow mask, 0: mask the interrupt"]
    #[inline(always)]
    pub fn set_tx_out1_fifo_uf_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
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
            .field("tx0_fifo_of", &self.tx0_fifo_of())
            .field("tx1_fifo_of", &self.tx1_fifo_of())
            .field("tx2_fifo_of", &self.tx2_fifo_of())
            .field("tx3_fifo_of", &self.tx3_fifo_of())
            .field("rx0_fifo_uf", &self.rx0_fifo_uf())
            .field("rx1_fifo_uf", &self.rx1_fifo_uf())
            .field("tx_out_fifo_uf", &self.tx_out_fifo_uf())
            .field("rx_in_fifo_of", &self.rx_in_fifo_of())
            .field("tx_out0_fifo_uf", &self.tx_out0_fifo_uf())
            .field("tx_out1_fifo_uf", &self.tx_out1_fifo_uf())
            .field("tx0_fifo_of_mask", &self.tx0_fifo_of_mask())
            .field("tx1_fifo_of_mask", &self.tx1_fifo_of_mask())
            .field("tx2_fifo_of_mask", &self.tx2_fifo_of_mask())
            .field("tx3_fifo_of_mask", &self.tx3_fifo_of_mask())
            .field("rx0_fifo_uf_mask", &self.rx0_fifo_uf_mask())
            .field("rx1_fifo_uf_mask", &self.rx1_fifo_uf_mask())
            .field("tx_out_fifo_uf_mask", &self.tx_out_fifo_uf_mask())
            .field("rx_in_fifo_of_mask", &self.rx_in_fifo_of_mask())
            .field("tx_out0_fifo_uf_mask", &self.tx_out0_fifo_uf_mask())
            .field("tx_out1_fifo_uf_mask", &self.tx_out1_fifo_uf_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irq {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Irq {
            tx0_fifo_of: bool,
            tx1_fifo_of: bool,
            tx2_fifo_of: bool,
            tx3_fifo_of: bool,
            rx0_fifo_uf: bool,
            rx1_fifo_uf: bool,
            tx_out_fifo_uf: bool,
            rx_in_fifo_of: bool,
            tx_out0_fifo_uf: bool,
            tx_out1_fifo_uf: bool,
            tx0_fifo_of_mask: bool,
            tx1_fifo_of_mask: bool,
            tx2_fifo_of_mask: bool,
            tx3_fifo_of_mask: bool,
            rx0_fifo_uf_mask: bool,
            rx1_fifo_uf_mask: bool,
            tx_out_fifo_uf_mask: bool,
            rx_in_fifo_of_mask: bool,
            tx_out0_fifo_uf_mask: bool,
            tx_out1_fifo_uf_mask: bool,
        }
        let proxy = Irq {
            tx0_fifo_of: self.tx0_fifo_of(),
            tx1_fifo_of: self.tx1_fifo_of(),
            tx2_fifo_of: self.tx2_fifo_of(),
            tx3_fifo_of: self.tx3_fifo_of(),
            rx0_fifo_uf: self.rx0_fifo_uf(),
            rx1_fifo_uf: self.rx1_fifo_uf(),
            tx_out_fifo_uf: self.tx_out_fifo_uf(),
            rx_in_fifo_of: self.rx_in_fifo_of(),
            tx_out0_fifo_uf: self.tx_out0_fifo_uf(),
            tx_out1_fifo_uf: self.tx_out1_fifo_uf(),
            tx0_fifo_of_mask: self.tx0_fifo_of_mask(),
            tx1_fifo_of_mask: self.tx1_fifo_of_mask(),
            tx2_fifo_of_mask: self.tx2_fifo_of_mask(),
            tx3_fifo_of_mask: self.tx3_fifo_of_mask(),
            rx0_fifo_uf_mask: self.rx0_fifo_uf_mask(),
            rx1_fifo_uf_mask: self.rx1_fifo_uf_mask(),
            tx_out_fifo_uf_mask: self.tx_out_fifo_uf_mask(),
            rx_in_fifo_of_mask: self.rx_in_fifo_of_mask(),
            tx_out0_fifo_uf_mask: self.tx_out0_fifo_uf_mask(),
            tx_out1_fifo_uf_mask: self.tx_out1_fifo_uf_mask(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ReservedIn(pub u32);
impl ReservedIn {
    #[doc = "reserved control 0"]
    #[inline(always)]
    pub const fn ctrl_0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "reserved control 0"]
    #[inline(always)]
    pub fn set_ctrl_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "reserved control 1"]
    #[inline(always)]
    pub const fn ctrl_1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "reserved control 1"]
    #[inline(always)]
    pub fn set_ctrl_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "reserved control 2"]
    #[inline(always)]
    pub const fn ctrl_2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "reserved control 2"]
    #[inline(always)]
    pub fn set_ctrl_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for ReservedIn {
    #[inline(always)]
    fn default() -> ReservedIn {
        ReservedIn(0)
    }
}
impl core::fmt::Debug for ReservedIn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ReservedIn")
            .field("ctrl_0", &self.ctrl_0())
            .field("ctrl_1", &self.ctrl_1())
            .field("ctrl_2", &self.ctrl_2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ReservedIn {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ReservedIn {
            ctrl_0: u8,
            ctrl_1: u8,
            ctrl_2: u8,
        }
        let proxy = ReservedIn {
            ctrl_0: self.ctrl_0(),
            ctrl_1: self.ctrl_1(),
            ctrl_2: self.ctrl_2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ReservedOut(pub u32);
impl ReservedOut {
    #[doc = "reserved status"]
    #[inline(always)]
    pub const fn stat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "reserved status"]
    #[inline(always)]
    pub fn set_stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
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
            .field("stat", &self.stat())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ReservedOut {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ReservedOut {
            stat: u8,
        }
        let proxy = ReservedOut { stat: self.stat() };
        defmt::write!(f, "{}", proxy)
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
        #[derive(defmt :: Format)]
        struct Rsvd1 {}
        let proxy = Rsvd1 {};
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxCh0Cfg(pub u32);
impl RxCh0Cfg {
    #[doc = "rx channel 0 enable"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "rx channel 0 enable"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "rx format 0: 16-bit mode 1: 24-bit mode"]
    #[inline(always)]
    pub const fn format(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "rx format 0: 16-bit mode 1: 24-bit mode"]
    #[inline(always)]
    pub fn set_format(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "rx mode 1'h0: mono mode 1'h1: stereo mode This bit is only used for 16-bit mode, in 24-bit mode, channel can only be set in mono mode. In 16-bit stereo mode, rx channel 1 is not working, both left and right audio data comes from channel 0."]
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "rx mode 1'h0: mono mode 1'h1: stereo mode This bit is only used for 16-bit mode, in 24-bit mode, channel can only be set in mono mode. In 16-bit stereo mode, rx channel 1 is not working, both left and right audio data comes from channel 0."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "1: mask the dma request for rx ch0"]
    #[inline(always)]
    pub const fn dma_msk(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1: mask the dma request for rx ch0"]
    #[inline(always)]
    pub fn set_dma_msk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "rx fifo counter"]
    #[inline(always)]
    pub const fn fifo_cnt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "rx fifo counter"]
    #[inline(always)]
    pub fn set_fifo_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for RxCh0Cfg {
    #[inline(always)]
    fn default() -> RxCh0Cfg {
        RxCh0Cfg(0)
    }
}
impl core::fmt::Debug for RxCh0Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxCh0Cfg")
            .field("enable", &self.enable())
            .field("format", &self.format())
            .field("mode", &self.mode())
            .field("dma_msk", &self.dma_msk())
            .field("fifo_cnt", &self.fifo_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxCh0Cfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RxCh0Cfg {
            enable: bool,
            format: bool,
            mode: bool,
            dma_msk: bool,
            fifo_cnt: u8,
        }
        let proxy = RxCh0Cfg {
            enable: self.enable(),
            format: self.format(),
            mode: self.mode(),
            dma_msk: self.dma_msk(),
            fifo_cnt: self.fifo_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxCh0Entry(pub u32);
impl RxCh0Entry {
    #[doc = "rx channel 0 data entry"]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "rx channel 0 data entry"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RxCh0Entry {
    #[inline(always)]
    fn default() -> RxCh0Entry {
        RxCh0Entry(0)
    }
}
impl core::fmt::Debug for RxCh0Entry {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxCh0Entry")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxCh0Entry {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RxCh0Entry {
            data: u32,
        }
        let proxy = RxCh0Entry { data: self.data() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxCh1Cfg(pub u32);
impl RxCh1Cfg {
    #[doc = "rx channel 1 enable"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "rx channel 1 enable"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "rx format 0: 16-bit mode 1: 24-bit mode"]
    #[inline(always)]
    pub const fn format(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "rx format 0: 16-bit mode 1: 24-bit mode"]
    #[inline(always)]
    pub fn set_format(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "1: mask the dma request for rx ch1"]
    #[inline(always)]
    pub const fn dma_msk(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1: mask the dma request for rx ch1"]
    #[inline(always)]
    pub fn set_dma_msk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "rx fifo counter"]
    #[inline(always)]
    pub const fn fifo_cnt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "rx fifo counter"]
    #[inline(always)]
    pub fn set_fifo_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for RxCh1Cfg {
    #[inline(always)]
    fn default() -> RxCh1Cfg {
        RxCh1Cfg(0)
    }
}
impl core::fmt::Debug for RxCh1Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxCh1Cfg")
            .field("enable", &self.enable())
            .field("format", &self.format())
            .field("dma_msk", &self.dma_msk())
            .field("fifo_cnt", &self.fifo_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxCh1Cfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RxCh1Cfg {
            enable: bool,
            format: bool,
            dma_msk: bool,
            fifo_cnt: u8,
        }
        let proxy = RxCh1Cfg {
            enable: self.enable(),
            format: self.format(),
            dma_msk: self.dma_msk(),
            fifo_cnt: self.fifo_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxCh1Entry(pub u32);
impl RxCh1Entry {
    #[doc = "rx channel 1 data entry"]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "rx channel 1 data entry"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RxCh1Entry {
    #[inline(always)]
    fn default() -> RxCh1Entry {
        RxCh1Entry(0)
    }
}
impl core::fmt::Debug for RxCh1Entry {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxCh1Entry")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxCh1Entry {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RxCh1Entry {
            data: u32,
        }
        let proxy = RxCh1Entry { data: self.data() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stb(pub u32);
impl Stb {
    #[doc = "dac strobe divider"]
    #[inline(always)]
    pub const fn dac_div(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "dac strobe divider"]
    #[inline(always)]
    pub fn set_dac_div(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "adc strobe divider"]
    #[inline(always)]
    pub const fn adc_div(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "adc strobe divider"]
    #[inline(always)]
    pub fn set_adc_div(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Stb {
    #[inline(always)]
    fn default() -> Stb {
        Stb(0)
    }
}
impl core::fmt::Debug for Stb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stb")
            .field("dac_div", &self.dac_div())
            .field("adc_div", &self.adc_div())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stb {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Stb {
            dac_div: u16,
            adc_div: u16,
        }
        let proxy = Stb {
            dac_div: self.dac_div(),
            adc_div: self.adc_div(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxCh0Cfg(pub u32);
impl TxCh0Cfg {
    #[doc = "tx channel 0 enable"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "tx channel 0 enable"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "tx format 0: 16-bit mode 1: 24-bit mode"]
    #[inline(always)]
    pub const fn format(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "tx format 0: 16-bit mode 1: 24-bit mode"]
    #[inline(always)]
    pub fn set_format(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "tx mode 1'h0: mono mode 1'h1: stereo mode This bit is only used for 16-bit mode, in 24-bit mode, channel can only be set in mono mode. In 16-bit stereo mode, tx channel 1 is not working, both left and right audio data comes from channel 0."]
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "tx mode 1'h0: mono mode 1'h1: stereo mode This bit is only used for 16-bit mode, in 24-bit mode, channel can only be set in mono mode. In 16-bit stereo mode, tx channel 1 is not working, both left and right audio data comes from channel 0."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "1: mask the dma request for tx ch0"]
    #[inline(always)]
    pub const fn dma_msk(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1: mask the dma request for tx ch0"]
    #[inline(always)]
    pub fn set_dma_msk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "tx fifo counter"]
    #[inline(always)]
    pub const fn fifo_cnt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "tx fifo counter"]
    #[inline(always)]
    pub fn set_fifo_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for TxCh0Cfg {
    #[inline(always)]
    fn default() -> TxCh0Cfg {
        TxCh0Cfg(0)
    }
}
impl core::fmt::Debug for TxCh0Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxCh0Cfg")
            .field("enable", &self.enable())
            .field("format", &self.format())
            .field("mode", &self.mode())
            .field("dma_msk", &self.dma_msk())
            .field("fifo_cnt", &self.fifo_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxCh0Cfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TxCh0Cfg {
            enable: bool,
            format: bool,
            mode: bool,
            dma_msk: bool,
            fifo_cnt: u8,
        }
        let proxy = TxCh0Cfg {
            enable: self.enable(),
            format: self.format(),
            mode: self.mode(),
            dma_msk: self.dma_msk(),
            fifo_cnt: self.fifo_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxCh0Entry(pub u32);
impl TxCh0Entry {
    #[doc = "tx channel 0 data entry"]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "tx channel 0 data entry"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TxCh0Entry {
    #[inline(always)]
    fn default() -> TxCh0Entry {
        TxCh0Entry(0)
    }
}
impl core::fmt::Debug for TxCh0Entry {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxCh0Entry")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxCh0Entry {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TxCh0Entry {
            data: u32,
        }
        let proxy = TxCh0Entry { data: self.data() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxCh1Cfg(pub u32);
impl TxCh1Cfg {
    #[doc = "tx channel 0 enable"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "tx channel 0 enable"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "tx format 0: 16-bit mode 1: 24-bit mode"]
    #[inline(always)]
    pub const fn format(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "tx format 0: 16-bit mode 1: 24-bit mode"]
    #[inline(always)]
    pub fn set_format(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "1: mask the dma request for tx ch1"]
    #[inline(always)]
    pub const fn dma_msk(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1: mask the dma request for tx ch1"]
    #[inline(always)]
    pub fn set_dma_msk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "tx fifo counter"]
    #[inline(always)]
    pub const fn fifo_cnt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "tx fifo counter"]
    #[inline(always)]
    pub fn set_fifo_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for TxCh1Cfg {
    #[inline(always)]
    fn default() -> TxCh1Cfg {
        TxCh1Cfg(0)
    }
}
impl core::fmt::Debug for TxCh1Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxCh1Cfg")
            .field("enable", &self.enable())
            .field("format", &self.format())
            .field("dma_msk", &self.dma_msk())
            .field("fifo_cnt", &self.fifo_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxCh1Cfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TxCh1Cfg {
            enable: bool,
            format: bool,
            dma_msk: bool,
            fifo_cnt: u8,
        }
        let proxy = TxCh1Cfg {
            enable: self.enable(),
            format: self.format(),
            dma_msk: self.dma_msk(),
            fifo_cnt: self.fifo_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxCh1Entry(pub u32);
impl TxCh1Entry {
    #[doc = "tx channel 1 data entry"]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "tx channel 1 data entry"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TxCh1Entry {
    #[inline(always)]
    fn default() -> TxCh1Entry {
        TxCh1Entry(0)
    }
}
impl core::fmt::Debug for TxCh1Entry {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxCh1Entry")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxCh1Entry {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TxCh1Entry {
            data: u32,
        }
        let proxy = TxCh1Entry { data: self.data() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxCh2Cfg(pub u32);
impl TxCh2Cfg {
    #[doc = "tx channel 0 enable"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "tx channel 0 enable"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "tx format 0: 16-bit mode 1: 24-bit mode"]
    #[inline(always)]
    pub const fn format(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "tx format 0: 16-bit mode 1: 24-bit mode"]
    #[inline(always)]
    pub fn set_format(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "tx mode 1'h0: mono mode 1'h1: stereo mode This bit is only used for 16-bit mode, in 24-bit mode, channel can only be set in mono mode. In 16-bit stereo mode, tx channel 3 is not working, both left and right audio data comes from channel 2."]
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "tx mode 1'h0: mono mode 1'h1: stereo mode This bit is only used for 16-bit mode, in 24-bit mode, channel can only be set in mono mode. In 16-bit stereo mode, tx channel 3 is not working, both left and right audio data comes from channel 2."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "1: mask the dma request for tx ch2"]
    #[inline(always)]
    pub const fn dma_msk(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1: mask the dma request for tx ch2"]
    #[inline(always)]
    pub fn set_dma_msk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "tx fifo counter"]
    #[inline(always)]
    pub const fn fifo_cnt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "tx fifo counter"]
    #[inline(always)]
    pub fn set_fifo_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for TxCh2Cfg {
    #[inline(always)]
    fn default() -> TxCh2Cfg {
        TxCh2Cfg(0)
    }
}
impl core::fmt::Debug for TxCh2Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxCh2Cfg")
            .field("enable", &self.enable())
            .field("format", &self.format())
            .field("mode", &self.mode())
            .field("dma_msk", &self.dma_msk())
            .field("fifo_cnt", &self.fifo_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxCh2Cfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TxCh2Cfg {
            enable: bool,
            format: bool,
            mode: bool,
            dma_msk: bool,
            fifo_cnt: u8,
        }
        let proxy = TxCh2Cfg {
            enable: self.enable(),
            format: self.format(),
            mode: self.mode(),
            dma_msk: self.dma_msk(),
            fifo_cnt: self.fifo_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxCh2Entry(pub u32);
impl TxCh2Entry {
    #[doc = "tx channel 2 data entry"]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "tx channel 2 data entry"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TxCh2Entry {
    #[inline(always)]
    fn default() -> TxCh2Entry {
        TxCh2Entry(0)
    }
}
impl core::fmt::Debug for TxCh2Entry {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxCh2Entry")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxCh2Entry {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TxCh2Entry {
            data: u32,
        }
        let proxy = TxCh2Entry { data: self.data() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxCh3Cfg(pub u32);
impl TxCh3Cfg {
    #[doc = "tx channel 0 enable"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "tx channel 0 enable"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "tx format 0: 16-bit mode 1: 24-bit mode"]
    #[inline(always)]
    pub const fn format(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "tx format 0: 16-bit mode 1: 24-bit mode"]
    #[inline(always)]
    pub fn set_format(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "1: mask the dma request for tx ch3"]
    #[inline(always)]
    pub const fn dma_msk(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1: mask the dma request for tx ch3"]
    #[inline(always)]
    pub fn set_dma_msk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "tx fifo counter"]
    #[inline(always)]
    pub const fn fifo_cnt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "tx fifo counter"]
    #[inline(always)]
    pub fn set_fifo_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for TxCh3Cfg {
    #[inline(always)]
    fn default() -> TxCh3Cfg {
        TxCh3Cfg(0)
    }
}
impl core::fmt::Debug for TxCh3Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxCh3Cfg")
            .field("enable", &self.enable())
            .field("format", &self.format())
            .field("dma_msk", &self.dma_msk())
            .field("fifo_cnt", &self.fifo_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxCh3Cfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TxCh3Cfg {
            enable: bool,
            format: bool,
            dma_msk: bool,
            fifo_cnt: u8,
        }
        let proxy = TxCh3Cfg {
            enable: self.enable(),
            format: self.format(),
            dma_msk: self.dma_msk(),
            fifo_cnt: self.fifo_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxCh3Entry(pub u32);
impl TxCh3Entry {
    #[doc = "tx channel 3 data entry"]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "tx channel 3 data entry"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TxCh3Entry {
    #[inline(always)]
    fn default() -> TxCh3Entry {
        TxCh3Entry(0)
    }
}
impl core::fmt::Debug for TxCh3Entry {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxCh3Entry")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxCh3Entry {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TxCh3Entry {
            data: u32,
        }
        let proxy = TxCh3Entry { data: self.data() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxOutCh0Cfg(pub u32);
impl TxOutCh0Cfg {
    #[doc = "tx out channel 0 enable"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "tx out channel 0 enable"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "tx out format 0: 16-bit mode 1: 24-bit mode"]
    #[inline(always)]
    pub const fn format(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "tx out format 0: 16-bit mode 1: 24-bit mode"]
    #[inline(always)]
    pub fn set_format(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "tx out mode 1'h0: mono mode 1'h1: stereo mode This bit is only used for 16-bit mode, in 24-bit mode, channel can only be set in mono mode. In 16-bit stereo mode, rx channel 1 is not working, both left and right audio data comes from channel 0."]
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "tx out mode 1'h0: mono mode 1'h1: stereo mode This bit is only used for 16-bit mode, in 24-bit mode, channel can only be set in mono mode. In 16-bit stereo mode, rx channel 1 is not working, both left and right audio data comes from channel 0."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "1: mask the dma request for tx out ch0"]
    #[inline(always)]
    pub const fn dma_msk(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1: mask the dma request for tx out ch0"]
    #[inline(always)]
    pub fn set_dma_msk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "tx out fifo counter"]
    #[inline(always)]
    pub const fn fifo_cnt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "tx out fifo counter"]
    #[inline(always)]
    pub fn set_fifo_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for TxOutCh0Cfg {
    #[inline(always)]
    fn default() -> TxOutCh0Cfg {
        TxOutCh0Cfg(0)
    }
}
impl core::fmt::Debug for TxOutCh0Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxOutCh0Cfg")
            .field("enable", &self.enable())
            .field("format", &self.format())
            .field("mode", &self.mode())
            .field("dma_msk", &self.dma_msk())
            .field("fifo_cnt", &self.fifo_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxOutCh0Cfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TxOutCh0Cfg {
            enable: bool,
            format: bool,
            mode: bool,
            dma_msk: bool,
            fifo_cnt: u8,
        }
        let proxy = TxOutCh0Cfg {
            enable: self.enable(),
            format: self.format(),
            mode: self.mode(),
            dma_msk: self.dma_msk(),
            fifo_cnt: self.fifo_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxOutCh0Entry(pub u32);
impl TxOutCh0Entry {
    #[doc = "tx out channel 0 data entry"]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "tx out channel 0 data entry"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TxOutCh0Entry {
    #[inline(always)]
    fn default() -> TxOutCh0Entry {
        TxOutCh0Entry(0)
    }
}
impl core::fmt::Debug for TxOutCh0Entry {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxOutCh0Entry")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxOutCh0Entry {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TxOutCh0Entry {
            data: u32,
        }
        let proxy = TxOutCh0Entry { data: self.data() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxOutCh1Cfg(pub u32);
impl TxOutCh1Cfg {
    #[doc = "tx out channel 1 enable"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "tx out channel 1 enable"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "tx out format 0: 16-bit mode 1: 24-bit mode"]
    #[inline(always)]
    pub const fn format(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "tx out format 0: 16-bit mode 1: 24-bit mode"]
    #[inline(always)]
    pub fn set_format(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "1: mask the dma request for tx out ch1"]
    #[inline(always)]
    pub const fn dma_msk(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1: mask the dma request for tx out ch1"]
    #[inline(always)]
    pub fn set_dma_msk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "tx out fifo counter"]
    #[inline(always)]
    pub const fn fifo_cnt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "tx out fifo counter"]
    #[inline(always)]
    pub fn set_fifo_cnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for TxOutCh1Cfg {
    #[inline(always)]
    fn default() -> TxOutCh1Cfg {
        TxOutCh1Cfg(0)
    }
}
impl core::fmt::Debug for TxOutCh1Cfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxOutCh1Cfg")
            .field("enable", &self.enable())
            .field("format", &self.format())
            .field("dma_msk", &self.dma_msk())
            .field("fifo_cnt", &self.fifo_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxOutCh1Cfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TxOutCh1Cfg {
            enable: bool,
            format: bool,
            dma_msk: bool,
            fifo_cnt: u8,
        }
        let proxy = TxOutCh1Cfg {
            enable: self.enable(),
            format: self.format(),
            dma_msk: self.dma_msk(),
            fifo_cnt: self.fifo_cnt(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxOutCh1Entry(pub u32);
impl TxOutCh1Entry {
    #[doc = "tx out channel 1 data entry"]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "tx out channel 1 data entry"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TxOutCh1Entry {
    #[inline(always)]
    fn default() -> TxOutCh1Entry {
        TxOutCh1Entry(0)
    }
}
impl core::fmt::Debug for TxOutCh1Entry {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxOutCh1Entry")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxOutCh1Entry {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TxOutCh1Entry {
            data: u32,
        }
        let proxy = TxOutCh1Entry { data: self.data() };
        defmt::write!(f, "{}", proxy)
    }
}
