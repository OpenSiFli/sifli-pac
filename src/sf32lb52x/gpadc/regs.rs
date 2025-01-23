#[doc = "ADC Analog Config Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcCfgReg1(pub u32);
impl AdcCfgReg1 {
    #[inline(always)]
    pub const fn anau_gpadc_cmref_fast_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_anau_gpadc_cmref_fast_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn anau_gpadc_p_int_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_anau_gpadc_p_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn anau_gpadc_cl_dly(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_anau_gpadc_cl_dly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[inline(always)]
    pub const fn anau_gpadc_en_v18(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_anau_gpadc_en_v18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Set GPADC in single-ended mode, signal range at P-input: 0 ~ VREF"]
    #[inline(always)]
    pub const fn anau_gpadc_se(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Set GPADC in single-ended mode, signal range at P-input: 0 ~ VREF"]
    #[inline(always)]
    pub fn set_anau_gpadc_se(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Short GPADC P and N input to CMREF, i.e., VREF/2"]
    #[inline(always)]
    pub const fn anau_gpadc_mute(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Short GPADC P and N input to CMREF, i.e., VREF/2"]
    #[inline(always)]
    pub fn set_anau_gpadc_mute(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Select N-side input channel for GPADC, 0 for channel 0, 7 for channel 7, effective when force on"]
    #[inline(always)]
    pub const fn anau_gpadc_sel_nch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "Select N-side input channel for GPADC, 0 for channel 0, 7 for channel 7, effective when force on"]
    #[inline(always)]
    pub fn set_anau_gpadc_sel_nch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "Select P-side input channel for GPADC, 0 for channel 0, 7 for channel 7, effective when force on"]
    #[inline(always)]
    pub const fn anau_gpadc_sel_pch(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Select P-side input channel for GPADC, 0 for channel 0, 7 for channel 7, effective when force on"]
    #[inline(always)]
    pub fn set_anau_gpadc_sel_pch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Set reference voltage for LDOREF, range = 0.35V(0) ~ 0.65V(15), step = 20mV"]
    #[inline(always)]
    pub const fn anau_gpadc_ldovref_sel(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x0f;
        val as u8
    }
    #[doc = "Set reference voltage for LDOREF, range = 0.35V(0) ~ 0.65V(15), step = 20mV"]
    #[inline(always)]
    pub fn set_anau_gpadc_ldovref_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 15usize)) | (((val as u32) & 0x0f) << 15usize);
    }
    #[doc = "Enable LDORF for ADC VREF"]
    #[inline(always)]
    pub const fn anau_gpadc_ldoref_en(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enable LDORF for ADC VREF"]
    #[inline(always)]
    pub fn set_anau_gpadc_ldoref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Set comparator input CM in sampling phase, 0.539V (0) / 0.578V (1) / 0.642V (2) / 0.784V (3)"]
    #[inline(always)]
    pub const fn anau_gpadc_vsp(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Set comparator input CM in sampling phase, 0.539V (0) / 0.578V (1) / 0.642V (2) / 0.784V (3)"]
    #[inline(always)]
    pub fn set_anau_gpadc_vsp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Tune ADC comparator CL= 3: 40f, range: 10fF (0) ~ 80fF (7) / 10fF step"]
    #[inline(always)]
    pub const fn anau_gpadc_cmpcl(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x07;
        val as u8
    }
    #[doc = "Tune ADC comparator CL= 3: 40f, range: 10fF (0) ~ 80fF (7) / 10fF step"]
    #[inline(always)]
    pub fn set_anau_gpadc_cmpcl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
    }
    #[doc = "Tune CDAC CM voltage 375mV range (increasing) / 25mV step, 8: for 0.5V Vcm,in"]
    #[inline(always)]
    pub const fn anau_gpadc_cmm(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x1f;
        val as u8
    }
    #[doc = "Tune CDAC CM voltage 375mV range (increasing) / 25mV step, 8: for 0.5V Vcm,in"]
    #[inline(always)]
    pub fn set_anau_gpadc_cmm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
    }
}
impl Default for AdcCfgReg1 {
    #[inline(always)]
    fn default() -> AdcCfgReg1 {
        AdcCfgReg1(0)
    }
}
impl core::fmt::Debug for AdcCfgReg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcCfgReg1")
            .field("anau_gpadc_cmref_fast_en", &self.anau_gpadc_cmref_fast_en())
            .field("anau_gpadc_p_int_en", &self.anau_gpadc_p_int_en())
            .field("anau_gpadc_cl_dly", &self.anau_gpadc_cl_dly())
            .field("anau_gpadc_en_v18", &self.anau_gpadc_en_v18())
            .field("anau_gpadc_se", &self.anau_gpadc_se())
            .field("anau_gpadc_mute", &self.anau_gpadc_mute())
            .field("anau_gpadc_sel_nch", &self.anau_gpadc_sel_nch())
            .field("anau_gpadc_sel_pch", &self.anau_gpadc_sel_pch())
            .field("anau_gpadc_ldovref_sel", &self.anau_gpadc_ldovref_sel())
            .field("anau_gpadc_ldoref_en", &self.anau_gpadc_ldoref_en())
            .field("anau_gpadc_vsp", &self.anau_gpadc_vsp())
            .field("anau_gpadc_cmpcl", &self.anau_gpadc_cmpcl())
            .field("anau_gpadc_cmm", &self.anau_gpadc_cmm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcCfgReg1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AdcCfgReg1 {
            anau_gpadc_cmref_fast_en: bool,
            anau_gpadc_p_int_en: bool,
            anau_gpadc_cl_dly: u8,
            anau_gpadc_en_v18: bool,
            anau_gpadc_se: bool,
            anau_gpadc_mute: bool,
            anau_gpadc_sel_nch: u8,
            anau_gpadc_sel_pch: u8,
            anau_gpadc_ldovref_sel: u8,
            anau_gpadc_ldoref_en: bool,
            anau_gpadc_vsp: u8,
            anau_gpadc_cmpcl: u8,
            anau_gpadc_cmm: u8,
        }
        let proxy = AdcCfgReg1 {
            anau_gpadc_cmref_fast_en: self.anau_gpadc_cmref_fast_en(),
            anau_gpadc_p_int_en: self.anau_gpadc_p_int_en(),
            anau_gpadc_cl_dly: self.anau_gpadc_cl_dly(),
            anau_gpadc_en_v18: self.anau_gpadc_en_v18(),
            anau_gpadc_se: self.anau_gpadc_se(),
            anau_gpadc_mute: self.anau_gpadc_mute(),
            anau_gpadc_sel_nch: self.anau_gpadc_sel_nch(),
            anau_gpadc_sel_pch: self.anau_gpadc_sel_pch(),
            anau_gpadc_ldovref_sel: self.anau_gpadc_ldovref_sel(),
            anau_gpadc_ldoref_en: self.anau_gpadc_ldoref_en(),
            anau_gpadc_vsp: self.anau_gpadc_vsp(),
            anau_gpadc_cmpcl: self.anau_gpadc_cmpcl(),
            anau_gpadc_cmm: self.anau_gpadc_cmm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ADC Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcCtrlReg(pub u32);
impl AdcCtrlReg {
    #[doc = "0: single conversion mode 1: continuous conversion mode"]
    #[inline(always)]
    pub const fn adc_op_mode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0: single conversion mode 1: continuous conversion mode"]
    #[inline(always)]
    pub fn set_adc_op_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write 1 to start GPADC,(don't need clear )"]
    #[inline(always)]
    pub const fn adc_start(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to start GPADC,(don't need clear )"]
    #[inline(always)]
    pub fn set_adc_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write 1 to stop GPADC in continuous mode(need write 0 to clear)"]
    #[inline(always)]
    pub const fn adc_stop(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to stop GPADC in continuous mode(need write 0 to clear)"]
    #[inline(always)]
    pub fn set_adc_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "GPADC will wait INIT_TIME ADCCLK cycles to start sample/conversion after being trigged"]
    #[inline(always)]
    pub const fn init_time(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "GPADC will wait INIT_TIME ADCCLK cycles to start sample/conversion after being trigged"]
    #[inline(always)]
    pub fn set_init_time(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[doc = "Enable DMA interface"]
    #[inline(always)]
    pub const fn dma_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA interface"]
    #[inline(always)]
    pub fn set_dma_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enable timer trigger function"]
    #[inline(always)]
    pub const fn timer_trig_en(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable timer trigger function"]
    #[inline(always)]
    pub fn set_timer_trig_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable input channel setting in ADC_CFG_REG1"]
    #[inline(always)]
    pub const fn chnl_sel_frc_en(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enable input channel setting in ADC_CFG_REG1"]
    #[inline(always)]
    pub fn set_chnl_sel_frc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable GPADC core"]
    #[inline(always)]
    pub const fn frc_en_adc(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable GPADC core"]
    #[inline(always)]
    pub fn set_frc_en_adc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Timer trigger source select"]
    #[inline(always)]
    pub const fn timer_trig_src_sel(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Timer trigger source select"]
    #[inline(always)]
    pub fn set_timer_trig_src_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "0: pulse no edge detect needed 1: level,need edge detect"]
    #[inline(always)]
    pub const fn timer_trig_typ(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "0: pulse no edge detect needed 1: level,need edge detect"]
    #[inline(always)]
    pub fn set_timer_trig_typ(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "0: combined data 1: raw data"]
    #[inline(always)]
    pub const fn dma_data_sel(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "0: combined data 1: raw data"]
    #[inline(always)]
    pub fn set_dma_data_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[inline(always)]
    pub const fn data_samp_dly(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_data_samp_dly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 17usize)) | (((val as u32) & 0x0f) << 17usize);
    }
}
impl Default for AdcCtrlReg {
    #[inline(always)]
    fn default() -> AdcCtrlReg {
        AdcCtrlReg(0)
    }
}
impl core::fmt::Debug for AdcCtrlReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcCtrlReg")
            .field("adc_op_mode", &self.adc_op_mode())
            .field("adc_start", &self.adc_start())
            .field("adc_stop", &self.adc_stop())
            .field("init_time", &self.init_time())
            .field("dma_en", &self.dma_en())
            .field("timer_trig_en", &self.timer_trig_en())
            .field("chnl_sel_frc_en", &self.chnl_sel_frc_en())
            .field("frc_en_adc", &self.frc_en_adc())
            .field("timer_trig_src_sel", &self.timer_trig_src_sel())
            .field("timer_trig_typ", &self.timer_trig_typ())
            .field("dma_data_sel", &self.dma_data_sel())
            .field("data_samp_dly", &self.data_samp_dly())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcCtrlReg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AdcCtrlReg {
            adc_op_mode: bool,
            adc_start: bool,
            adc_stop: bool,
            init_time: u8,
            dma_en: bool,
            timer_trig_en: bool,
            chnl_sel_frc_en: bool,
            frc_en_adc: bool,
            timer_trig_src_sel: u8,
            timer_trig_typ: bool,
            dma_data_sel: bool,
            data_samp_dly: u8,
        }
        let proxy = AdcCtrlReg {
            adc_op_mode: self.adc_op_mode(),
            adc_start: self.adc_start(),
            adc_stop: self.adc_stop(),
            init_time: self.init_time(),
            dma_en: self.dma_en(),
            timer_trig_en: self.timer_trig_en(),
            chnl_sel_frc_en: self.chnl_sel_frc_en(),
            frc_en_adc: self.frc_en_adc(),
            timer_trig_src_sel: self.timer_trig_src_sel(),
            timer_trig_typ: self.timer_trig_typ(),
            dma_data_sel: self.dma_data_sel(),
            data_samp_dly: self.data_samp_dly(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ADC Control Register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcCtrlReg2(pub u32);
impl AdcCtrlReg2 {
    #[inline(always)]
    pub const fn samp_width(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_samp_width(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[inline(always)]
    pub const fn conv_width(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_conv_width(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for AdcCtrlReg2 {
    #[inline(always)]
    fn default() -> AdcCtrlReg2 {
        AdcCtrlReg2(0)
    }
}
impl core::fmt::Debug for AdcCtrlReg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcCtrlReg2")
            .field("samp_width", &self.samp_width())
            .field("conv_width", &self.conv_width())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcCtrlReg2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AdcCtrlReg2 {
            samp_width: u32,
            conv_width: u8,
        }
        let proxy = AdcCtrlReg2 {
            samp_width: self.samp_width(),
            conv_width: self.conv_width(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ADC Read Data For DMA"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcDmaRdata(pub u32);
impl AdcDmaRdata {
    #[inline(always)]
    pub const fn dma_rdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[inline(always)]
    pub fn set_dma_rdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[inline(always)]
    pub const fn dma_rdata_raw(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x1fff;
        val as u16
    }
    #[inline(always)]
    pub fn set_dma_rdata_raw(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
    }
}
impl Default for AdcDmaRdata {
    #[inline(always)]
    fn default() -> AdcDmaRdata {
        AdcDmaRdata(0)
    }
}
impl core::fmt::Debug for AdcDmaRdata {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcDmaRdata")
            .field("dma_rdata", &self.dma_rdata())
            .field("dma_rdata_raw", &self.dma_rdata_raw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcDmaRdata {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AdcDmaRdata {
            dma_rdata: u16,
            dma_rdata_raw: u16,
        }
        let proxy = AdcDmaRdata {
            dma_rdata: self.dma_rdata(),
            dma_rdata_raw: self.dma_rdata_raw(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ADC Read Data0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcRdata0(pub u32);
impl AdcRdata0 {
    #[inline(always)]
    pub const fn slot0_rdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub fn set_slot0_rdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[inline(always)]
    pub const fn slot1_rdata(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub fn set_slot1_rdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for AdcRdata0 {
    #[inline(always)]
    fn default() -> AdcRdata0 {
        AdcRdata0(0)
    }
}
impl core::fmt::Debug for AdcRdata0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcRdata0")
            .field("slot0_rdata", &self.slot0_rdata())
            .field("slot1_rdata", &self.slot1_rdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcRdata0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AdcRdata0 {
            slot0_rdata: u16,
            slot1_rdata: u16,
        }
        let proxy = AdcRdata0 {
            slot0_rdata: self.slot0_rdata(),
            slot1_rdata: self.slot1_rdata(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ADC Read Data1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcRdata1(pub u32);
impl AdcRdata1 {
    #[inline(always)]
    pub const fn slot2_rdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub fn set_slot2_rdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[inline(always)]
    pub const fn slot3_rdata(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub fn set_slot3_rdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for AdcRdata1 {
    #[inline(always)]
    fn default() -> AdcRdata1 {
        AdcRdata1(0)
    }
}
impl core::fmt::Debug for AdcRdata1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcRdata1")
            .field("slot2_rdata", &self.slot2_rdata())
            .field("slot3_rdata", &self.slot3_rdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcRdata1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AdcRdata1 {
            slot2_rdata: u16,
            slot3_rdata: u16,
        }
        let proxy = AdcRdata1 {
            slot2_rdata: self.slot2_rdata(),
            slot3_rdata: self.slot3_rdata(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ADC Read Data2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcRdata2(pub u32);
impl AdcRdata2 {
    #[inline(always)]
    pub const fn slot4_rdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub fn set_slot4_rdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[inline(always)]
    pub const fn slot5_rdata(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub fn set_slot5_rdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for AdcRdata2 {
    #[inline(always)]
    fn default() -> AdcRdata2 {
        AdcRdata2(0)
    }
}
impl core::fmt::Debug for AdcRdata2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcRdata2")
            .field("slot4_rdata", &self.slot4_rdata())
            .field("slot5_rdata", &self.slot5_rdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcRdata2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AdcRdata2 {
            slot4_rdata: u16,
            slot5_rdata: u16,
        }
        let proxy = AdcRdata2 {
            slot4_rdata: self.slot4_rdata(),
            slot5_rdata: self.slot5_rdata(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ADC Read Data3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcRdata3(pub u32);
impl AdcRdata3 {
    #[inline(always)]
    pub const fn slot6_rdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub fn set_slot6_rdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[inline(always)]
    pub const fn slot7_rdata(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub fn set_slot7_rdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for AdcRdata3 {
    #[inline(always)]
    fn default() -> AdcRdata3 {
        AdcRdata3(0)
    }
}
impl core::fmt::Debug for AdcRdata3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcRdata3")
            .field("slot6_rdata", &self.slot6_rdata())
            .field("slot7_rdata", &self.slot7_rdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcRdata3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AdcRdata3 {
            slot6_rdata: u16,
            slot7_rdata: u16,
        }
        let proxy = AdcRdata3 {
            slot6_rdata: self.slot6_rdata(),
            slot7_rdata: self.slot7_rdata(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ADC Slot0 Config Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcSlot0Reg(pub u32);
impl AdcSlot0Reg {
    #[inline(always)]
    pub const fn slot_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_slot_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn pchnl_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_pchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[inline(always)]
    pub const fn nchnl_sel(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_nchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
}
impl Default for AdcSlot0Reg {
    #[inline(always)]
    fn default() -> AdcSlot0Reg {
        AdcSlot0Reg(0)
    }
}
impl core::fmt::Debug for AdcSlot0Reg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcSlot0Reg")
            .field("slot_en", &self.slot_en())
            .field("pchnl_sel", &self.pchnl_sel())
            .field("nchnl_sel", &self.nchnl_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcSlot0Reg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AdcSlot0Reg {
            slot_en: bool,
            pchnl_sel: u8,
            nchnl_sel: u8,
        }
        let proxy = AdcSlot0Reg {
            slot_en: self.slot_en(),
            pchnl_sel: self.pchnl_sel(),
            nchnl_sel: self.nchnl_sel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ADC Slot1 Config Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcSlot1Reg(pub u32);
impl AdcSlot1Reg {
    #[inline(always)]
    pub const fn slot_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_slot_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn pchnl_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_pchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[inline(always)]
    pub const fn nchnl_sel(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_nchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
}
impl Default for AdcSlot1Reg {
    #[inline(always)]
    fn default() -> AdcSlot1Reg {
        AdcSlot1Reg(0)
    }
}
impl core::fmt::Debug for AdcSlot1Reg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcSlot1Reg")
            .field("slot_en", &self.slot_en())
            .field("pchnl_sel", &self.pchnl_sel())
            .field("nchnl_sel", &self.nchnl_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcSlot1Reg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AdcSlot1Reg {
            slot_en: bool,
            pchnl_sel: u8,
            nchnl_sel: u8,
        }
        let proxy = AdcSlot1Reg {
            slot_en: self.slot_en(),
            pchnl_sel: self.pchnl_sel(),
            nchnl_sel: self.nchnl_sel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ADC Slot2 Config Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcSlot2Reg(pub u32);
impl AdcSlot2Reg {
    #[inline(always)]
    pub const fn slot_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_slot_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn pchnl_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_pchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[inline(always)]
    pub const fn nchnl_sel(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_nchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
}
impl Default for AdcSlot2Reg {
    #[inline(always)]
    fn default() -> AdcSlot2Reg {
        AdcSlot2Reg(0)
    }
}
impl core::fmt::Debug for AdcSlot2Reg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcSlot2Reg")
            .field("slot_en", &self.slot_en())
            .field("pchnl_sel", &self.pchnl_sel())
            .field("nchnl_sel", &self.nchnl_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcSlot2Reg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AdcSlot2Reg {
            slot_en: bool,
            pchnl_sel: u8,
            nchnl_sel: u8,
        }
        let proxy = AdcSlot2Reg {
            slot_en: self.slot_en(),
            pchnl_sel: self.pchnl_sel(),
            nchnl_sel: self.nchnl_sel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ADC Slot3 Config Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcSlot3Reg(pub u32);
impl AdcSlot3Reg {
    #[inline(always)]
    pub const fn slot_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_slot_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn pchnl_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_pchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[inline(always)]
    pub const fn nchnl_sel(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_nchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
}
impl Default for AdcSlot3Reg {
    #[inline(always)]
    fn default() -> AdcSlot3Reg {
        AdcSlot3Reg(0)
    }
}
impl core::fmt::Debug for AdcSlot3Reg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcSlot3Reg")
            .field("slot_en", &self.slot_en())
            .field("pchnl_sel", &self.pchnl_sel())
            .field("nchnl_sel", &self.nchnl_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcSlot3Reg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AdcSlot3Reg {
            slot_en: bool,
            pchnl_sel: u8,
            nchnl_sel: u8,
        }
        let proxy = AdcSlot3Reg {
            slot_en: self.slot_en(),
            pchnl_sel: self.pchnl_sel(),
            nchnl_sel: self.nchnl_sel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ADC Slot4 Config Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcSlot4Reg(pub u32);
impl AdcSlot4Reg {
    #[inline(always)]
    pub const fn slot_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_slot_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn pchnl_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_pchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[inline(always)]
    pub const fn nchnl_sel(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_nchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
}
impl Default for AdcSlot4Reg {
    #[inline(always)]
    fn default() -> AdcSlot4Reg {
        AdcSlot4Reg(0)
    }
}
impl core::fmt::Debug for AdcSlot4Reg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcSlot4Reg")
            .field("slot_en", &self.slot_en())
            .field("pchnl_sel", &self.pchnl_sel())
            .field("nchnl_sel", &self.nchnl_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcSlot4Reg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AdcSlot4Reg {
            slot_en: bool,
            pchnl_sel: u8,
            nchnl_sel: u8,
        }
        let proxy = AdcSlot4Reg {
            slot_en: self.slot_en(),
            pchnl_sel: self.pchnl_sel(),
            nchnl_sel: self.nchnl_sel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ADC Slot5 Config Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcSlot5Reg(pub u32);
impl AdcSlot5Reg {
    #[inline(always)]
    pub const fn slot_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_slot_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn pchnl_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_pchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[inline(always)]
    pub const fn nchnl_sel(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_nchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
}
impl Default for AdcSlot5Reg {
    #[inline(always)]
    fn default() -> AdcSlot5Reg {
        AdcSlot5Reg(0)
    }
}
impl core::fmt::Debug for AdcSlot5Reg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcSlot5Reg")
            .field("slot_en", &self.slot_en())
            .field("pchnl_sel", &self.pchnl_sel())
            .field("nchnl_sel", &self.nchnl_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcSlot5Reg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AdcSlot5Reg {
            slot_en: bool,
            pchnl_sel: u8,
            nchnl_sel: u8,
        }
        let proxy = AdcSlot5Reg {
            slot_en: self.slot_en(),
            pchnl_sel: self.pchnl_sel(),
            nchnl_sel: self.nchnl_sel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ADC Slot6 Config Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcSlot6Reg(pub u32);
impl AdcSlot6Reg {
    #[inline(always)]
    pub const fn slot_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_slot_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn pchnl_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_pchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[inline(always)]
    pub const fn nchnl_sel(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_nchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
}
impl Default for AdcSlot6Reg {
    #[inline(always)]
    fn default() -> AdcSlot6Reg {
        AdcSlot6Reg(0)
    }
}
impl core::fmt::Debug for AdcSlot6Reg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcSlot6Reg")
            .field("slot_en", &self.slot_en())
            .field("pchnl_sel", &self.pchnl_sel())
            .field("nchnl_sel", &self.nchnl_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcSlot6Reg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AdcSlot6Reg {
            slot_en: bool,
            pchnl_sel: u8,
            nchnl_sel: u8,
        }
        let proxy = AdcSlot6Reg {
            slot_en: self.slot_en(),
            pchnl_sel: self.pchnl_sel(),
            nchnl_sel: self.nchnl_sel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "ADC Slot7 Config Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcSlot7Reg(pub u32);
impl AdcSlot7Reg {
    #[inline(always)]
    pub const fn slot_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_slot_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn pchnl_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_pchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[inline(always)]
    pub const fn nchnl_sel(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_nchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
}
impl Default for AdcSlot7Reg {
    #[inline(always)]
    fn default() -> AdcSlot7Reg {
        AdcSlot7Reg(0)
    }
}
impl core::fmt::Debug for AdcSlot7Reg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AdcSlot7Reg")
            .field("slot_en", &self.slot_en())
            .field("pchnl_sel", &self.pchnl_sel())
            .field("nchnl_sel", &self.nchnl_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AdcSlot7Reg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AdcSlot7Reg {
            slot_en: bool,
            pchnl_sel: u8,
            nchnl_sel: u8,
        }
        let proxy = AdcSlot7Reg {
            slot_en: self.slot_en(),
            pchnl_sel: self.pchnl_sel(),
            nchnl_sel: self.nchnl_sel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "GPADC IRQ Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpadcIrq(pub u32);
impl GpadcIrq {
    #[inline(always)]
    pub const fn gpadc_icr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_gpadc_icr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn gpadc_imr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_gpadc_imr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn gpadc_irsr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_gpadc_irsr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn gpadc_isr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_gpadc_isr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for GpadcIrq {
    #[inline(always)]
    fn default() -> GpadcIrq {
        GpadcIrq(0)
    }
}
impl core::fmt::Debug for GpadcIrq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GpadcIrq")
            .field("gpadc_icr", &self.gpadc_icr())
            .field("gpadc_imr", &self.gpadc_imr())
            .field("gpadc_irsr", &self.gpadc_irsr())
            .field("gpadc_isr", &self.gpadc_isr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GpadcIrq {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct GpadcIrq {
            gpadc_icr: bool,
            gpadc_imr: bool,
            gpadc_irsr: bool,
            gpadc_isr: bool,
        }
        let proxy = GpadcIrq {
            gpadc_icr: self.gpadc_icr(),
            gpadc_imr: self.gpadc_imr(),
            gpadc_irsr: self.gpadc_irsr(),
            gpadc_isr: self.gpadc_isr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "GPADC Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpadcStatus(pub u32);
impl GpadcStatus {
    #[inline(always)]
    pub const fn adc_done(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_adc_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn slot_done(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_slot_done(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
    }
    #[inline(always)]
    pub const fn cur_slot(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_cur_slot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
}
impl Default for GpadcStatus {
    #[inline(always)]
    fn default() -> GpadcStatus {
        GpadcStatus(0)
    }
}
impl core::fmt::Debug for GpadcStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GpadcStatus")
            .field("adc_done", &self.adc_done())
            .field("slot_done", &self.slot_done())
            .field("cur_slot", &self.cur_slot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GpadcStatus {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct GpadcStatus {
            adc_done: bool,
            slot_done: u8,
            cur_slot: u8,
        }
        let proxy = GpadcStatus {
            adc_done: self.adc_done(),
            slot_done: self.slot_done(),
            cur_slot: self.cur_slot(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
