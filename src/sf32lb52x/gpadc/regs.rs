#[doc = "ADC Analog Config Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CfgReg1(pub u32);
impl CfgReg1 {
    #[must_use]
    #[inline(always)]
    pub const fn anau_gpadc_cmref_fast_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_anau_gpadc_cmref_fast_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn anau_gpadc_p_int_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_anau_gpadc_p_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn anau_gpadc_cl_dly(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_anau_gpadc_cl_dly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn anau_gpadc_en_v18(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_anau_gpadc_en_v18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Set GPADC in single-ended mode, signal range at P-input: 0 ~ VREF"]
    #[must_use]
    #[inline(always)]
    pub const fn anau_gpadc_se(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Set GPADC in single-ended mode, signal range at P-input: 0 ~ VREF"]
    #[inline(always)]
    pub const fn set_anau_gpadc_se(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Short GPADC P and N input to CMREF, i.e., VREF/2"]
    #[must_use]
    #[inline(always)]
    pub const fn anau_gpadc_mute(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Short GPADC P and N input to CMREF, i.e., VREF/2"]
    #[inline(always)]
    pub const fn set_anau_gpadc_mute(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Select N-side input channel for GPADC, 0 for channel 0, 7 for channel 7, effective when force on"]
    #[must_use]
    #[inline(always)]
    pub const fn anau_gpadc_sel_nch(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "Select N-side input channel for GPADC, 0 for channel 0, 7 for channel 7, effective when force on"]
    #[inline(always)]
    pub const fn set_anau_gpadc_sel_nch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "Select P-side input channel for GPADC, 0 for channel 0, 7 for channel 7, effective when force on"]
    #[must_use]
    #[inline(always)]
    pub const fn anau_gpadc_sel_pch(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Select P-side input channel for GPADC, 0 for channel 0, 7 for channel 7, effective when force on"]
    #[inline(always)]
    pub const fn set_anau_gpadc_sel_pch(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Set reference voltage for LDOREF, range = 0.35V(0) ~ 0.65V(15), step = 20mV"]
    #[must_use]
    #[inline(always)]
    pub const fn anau_gpadc_ldovref_sel(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x0f;
        val as u8
    }
    #[doc = "Set reference voltage for LDOREF, range = 0.35V(0) ~ 0.65V(15), step = 20mV"]
    #[inline(always)]
    pub const fn set_anau_gpadc_ldovref_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 15usize)) | (((val as u32) & 0x0f) << 15usize);
    }
    #[doc = "Enable LDORF for ADC VREF"]
    #[must_use]
    #[inline(always)]
    pub const fn anau_gpadc_ldoref_en(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enable LDORF for ADC VREF"]
    #[inline(always)]
    pub const fn set_anau_gpadc_ldoref_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Set comparator input CM in sampling phase, 0.539V (0) / 0.578V (1) / 0.642V (2) / 0.784V (3)"]
    #[must_use]
    #[inline(always)]
    pub const fn anau_gpadc_vsp(&self) -> super::vals::Vsp {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Vsp::from_bits(val as u8)
    }
    #[doc = "Set comparator input CM in sampling phase, 0.539V (0) / 0.578V (1) / 0.642V (2) / 0.784V (3)"]
    #[inline(always)]
    pub const fn set_anau_gpadc_vsp(&mut self, val: super::vals::Vsp) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "Tune ADC comparator CL= 3: 40f, range: 10fF (0) ~ 80fF (7) / 10fF step"]
    #[must_use]
    #[inline(always)]
    pub const fn anau_gpadc_cmpcl(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x07;
        val as u8
    }
    #[doc = "Tune ADC comparator CL= 3: 40f, range: 10fF (0) ~ 80fF (7) / 10fF step"]
    #[inline(always)]
    pub const fn set_anau_gpadc_cmpcl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
    }
    #[doc = "Tune CDAC CM voltage 375mV range (increasing) / 25mV step, 8: for 0.5V Vcm,in"]
    #[must_use]
    #[inline(always)]
    pub const fn anau_gpadc_cmm(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x1f;
        val as u8
    }
    #[doc = "Tune CDAC CM voltage 375mV range (increasing) / 25mV step, 8: for 0.5V Vcm,in"]
    #[inline(always)]
    pub const fn set_anau_gpadc_cmm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
    }
}
impl Default for CfgReg1 {
    #[inline(always)]
    fn default() -> CfgReg1 {
        CfgReg1(0)
    }
}
impl core::fmt::Debug for CfgReg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CfgReg1")
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
impl defmt::Format for CfgReg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "CfgReg1 {{ anau_gpadc_cmref_fast_en: {=bool:?}, anau_gpadc_p_int_en: {=bool:?}, anau_gpadc_cl_dly: {=u8:?}, anau_gpadc_en_v18: {=bool:?}, anau_gpadc_se: {=bool:?}, anau_gpadc_mute: {=bool:?}, anau_gpadc_sel_nch: {=u8:?}, anau_gpadc_sel_pch: {=u8:?}, anau_gpadc_ldovref_sel: {=u8:?}, anau_gpadc_ldoref_en: {=bool:?}, anau_gpadc_vsp: {:?}, anau_gpadc_cmpcl: {=u8:?}, anau_gpadc_cmm: {=u8:?} }}" , self . anau_gpadc_cmref_fast_en () , self . anau_gpadc_p_int_en () , self . anau_gpadc_cl_dly () , self . anau_gpadc_en_v18 () , self . anau_gpadc_se () , self . anau_gpadc_mute () , self . anau_gpadc_sel_nch () , self . anau_gpadc_sel_pch () , self . anau_gpadc_ldovref_sel () , self . anau_gpadc_ldoref_en () , self . anau_gpadc_vsp () , self . anau_gpadc_cmpcl () , self . anau_gpadc_cmm ())
    }
}
#[doc = "ADC Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlReg(pub u32);
impl CtrlReg {
    #[doc = "0: single conversion mode 1: continuous conversion mode"]
    #[must_use]
    #[inline(always)]
    pub const fn adc_op_mode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0: single conversion mode 1: continuous conversion mode"]
    #[inline(always)]
    pub const fn set_adc_op_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write 1 to start GPADC,(don't need clear )"]
    #[must_use]
    #[inline(always)]
    pub const fn adc_start(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to start GPADC,(don't need clear )"]
    #[inline(always)]
    pub const fn set_adc_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write 1 to stop GPADC in continuous mode(need write 0 to clear)"]
    #[must_use]
    #[inline(always)]
    pub const fn adc_stop(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to stop GPADC in continuous mode(need write 0 to clear)"]
    #[inline(always)]
    pub const fn set_adc_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "GPADC will wait INIT_TIME ADCCLK cycles to start sample/conversion after being trigged"]
    #[must_use]
    #[inline(always)]
    pub const fn init_time(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x0f;
        val as u8
    }
    #[doc = "GPADC will wait INIT_TIME ADCCLK cycles to start sample/conversion after being trigged"]
    #[inline(always)]
    pub const fn set_init_time(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 3usize)) | (((val as u32) & 0x0f) << 3usize);
    }
    #[doc = "Enable DMA interface"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable DMA interface"]
    #[inline(always)]
    pub const fn set_dma_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Enable timer trigger function"]
    #[must_use]
    #[inline(always)]
    pub const fn timer_trig_en(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Enable timer trigger function"]
    #[inline(always)]
    pub const fn set_timer_trig_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Enable input channel setting in ADC_CFG_REG1"]
    #[must_use]
    #[inline(always)]
    pub const fn chnl_sel_frc_en(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Enable input channel setting in ADC_CFG_REG1"]
    #[inline(always)]
    pub const fn set_chnl_sel_frc_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable GPADC core"]
    #[must_use]
    #[inline(always)]
    pub const fn frc_en_adc(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable GPADC core"]
    #[inline(always)]
    pub const fn set_frc_en_adc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Timer trigger source select"]
    #[must_use]
    #[inline(always)]
    pub const fn timer_trig_src_sel(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Timer trigger source select"]
    #[inline(always)]
    pub const fn set_timer_trig_src_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "0: pulse no edge detect needed 1: level,need edge detect"]
    #[must_use]
    #[inline(always)]
    pub const fn timer_trig_typ(&self) -> super::vals::TimerTrigTyp {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::TimerTrigTyp::from_bits(val as u8)
    }
    #[doc = "0: pulse no edge detect needed 1: level,need edge detect"]
    #[inline(always)]
    pub const fn set_timer_trig_typ(&mut self, val: super::vals::TimerTrigTyp) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "0: combined data 1: raw data"]
    #[must_use]
    #[inline(always)]
    pub const fn dma_data_sel(&self) -> super::vals::DmaDataSel {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::DmaDataSel::from_bits(val as u8)
    }
    #[doc = "0: combined data 1: raw data"]
    #[inline(always)]
    pub const fn set_dma_data_sel(&mut self, val: super::vals::DmaDataSel) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn data_samp_dly(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_data_samp_dly(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 17usize)) | (((val as u32) & 0x0f) << 17usize);
    }
}
impl Default for CtrlReg {
    #[inline(always)]
    fn default() -> CtrlReg {
        CtrlReg(0)
    }
}
impl core::fmt::Debug for CtrlReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlReg")
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
impl defmt::Format for CtrlReg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "CtrlReg {{ adc_op_mode: {=bool:?}, adc_start: {=bool:?}, adc_stop: {=bool:?}, init_time: {=u8:?}, dma_en: {=bool:?}, timer_trig_en: {=bool:?}, chnl_sel_frc_en: {=bool:?}, frc_en_adc: {=bool:?}, timer_trig_src_sel: {=u8:?}, timer_trig_typ: {:?}, dma_data_sel: {:?}, data_samp_dly: {=u8:?} }}" , self . adc_op_mode () , self . adc_start () , self . adc_stop () , self . init_time () , self . dma_en () , self . timer_trig_en () , self . chnl_sel_frc_en () , self . frc_en_adc () , self . timer_trig_src_sel () , self . timer_trig_typ () , self . dma_data_sel () , self . data_samp_dly ())
    }
}
#[doc = "ADC Control Register2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlReg2(pub u32);
impl CtrlReg2 {
    #[must_use]
    #[inline(always)]
    pub const fn samp_width(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub const fn set_samp_width(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn conv_width(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_conv_width(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for CtrlReg2 {
    #[inline(always)]
    fn default() -> CtrlReg2 {
        CtrlReg2(0)
    }
}
impl core::fmt::Debug for CtrlReg2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlReg2")
            .field("samp_width", &self.samp_width())
            .field("conv_width", &self.conv_width())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlReg2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CtrlReg2 {{ samp_width: {=u32:?}, conv_width: {=u8:?} }}",
            self.samp_width(),
            self.conv_width()
        )
    }
}
#[doc = "ADC Read Data For DMA"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaRdata(pub u32);
impl DmaRdata {
    #[must_use]
    #[inline(always)]
    pub const fn dma_rdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_dma_rdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn dma_rdata_raw(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x1fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_dma_rdata_raw(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
    }
}
impl Default for DmaRdata {
    #[inline(always)]
    fn default() -> DmaRdata {
        DmaRdata(0)
    }
}
impl core::fmt::Debug for DmaRdata {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaRdata")
            .field("dma_rdata", &self.dma_rdata())
            .field("dma_rdata_raw", &self.dma_rdata_raw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaRdata {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaRdata {{ dma_rdata: {=u16:?}, dma_rdata_raw: {=u16:?} }}",
            self.dma_rdata(),
            self.dma_rdata_raw()
        )
    }
}
#[doc = "GPADC IRQ Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpadcIrq(pub u32);
impl GpadcIrq {
    #[must_use]
    #[inline(always)]
    pub const fn gpadc_icr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_gpadc_icr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn gpadc_imr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_gpadc_imr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn gpadc_irsr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_gpadc_irsr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn gpadc_isr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_gpadc_isr(&mut self, val: bool) {
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
        defmt :: write ! (f , "GpadcIrq {{ gpadc_icr: {=bool:?}, gpadc_imr: {=bool:?}, gpadc_irsr: {=bool:?}, gpadc_isr: {=bool:?} }}" , self . gpadc_icr () , self . gpadc_imr () , self . gpadc_irsr () , self . gpadc_isr ())
    }
}
#[doc = "GPADC Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpadcStatus(pub u32);
impl GpadcStatus {
    #[must_use]
    #[inline(always)]
    pub const fn adc_done(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_adc_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn slot_done(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub const fn set_slot_done(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 1usize)) | (((val as u32) & 0xff) << 1usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn cur_slot(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_cur_slot(&mut self, val: u8) {
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
        defmt::write!(
            f,
            "GpadcStatus {{ adc_done: {=bool:?}, slot_done: {=u8:?}, cur_slot: {=u8:?} }}",
            self.adc_done(),
            self.slot_done(),
            self.cur_slot()
        )
    }
}
#[doc = "ADC Read Data0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rdata(pub u32);
impl Rdata {
    #[must_use]
    #[inline(always)]
    pub const fn even_slot_rdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_even_slot_rdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn odd_slot_rdata(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub const fn set_odd_slot_rdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Rdata {
    #[inline(always)]
    fn default() -> Rdata {
        Rdata(0)
    }
}
impl core::fmt::Debug for Rdata {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rdata")
            .field("even_slot_rdata", &self.even_slot_rdata())
            .field("odd_slot_rdata", &self.odd_slot_rdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rdata {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Rdata {{ even_slot_rdata: {=u16:?}, odd_slot_rdata: {=u16:?} }}",
            self.even_slot_rdata(),
            self.odd_slot_rdata()
        )
    }
}
#[doc = "ADC Slot0 Config Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slot0Reg(pub u32);
impl Slot0Reg {
    #[must_use]
    #[inline(always)]
    pub const fn slot_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_slot_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pchnl_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_pchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn nchnl_sel(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_nchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
}
impl Default for Slot0Reg {
    #[inline(always)]
    fn default() -> Slot0Reg {
        Slot0Reg(0)
    }
}
impl core::fmt::Debug for Slot0Reg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Slot0Reg")
            .field("slot_en", &self.slot_en())
            .field("pchnl_sel", &self.pchnl_sel())
            .field("nchnl_sel", &self.nchnl_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Slot0Reg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Slot0Reg {{ slot_en: {=bool:?}, pchnl_sel: {=u8:?}, nchnl_sel: {=u8:?} }}",
            self.slot_en(),
            self.pchnl_sel(),
            self.nchnl_sel()
        )
    }
}
#[doc = "ADC Slot1 Config Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slot1Reg(pub u32);
impl Slot1Reg {
    #[must_use]
    #[inline(always)]
    pub const fn slot_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_slot_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pchnl_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_pchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn nchnl_sel(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_nchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
}
impl Default for Slot1Reg {
    #[inline(always)]
    fn default() -> Slot1Reg {
        Slot1Reg(0)
    }
}
impl core::fmt::Debug for Slot1Reg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Slot1Reg")
            .field("slot_en", &self.slot_en())
            .field("pchnl_sel", &self.pchnl_sel())
            .field("nchnl_sel", &self.nchnl_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Slot1Reg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Slot1Reg {{ slot_en: {=bool:?}, pchnl_sel: {=u8:?}, nchnl_sel: {=u8:?} }}",
            self.slot_en(),
            self.pchnl_sel(),
            self.nchnl_sel()
        )
    }
}
#[doc = "ADC Slot2 Config Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slot2Reg(pub u32);
impl Slot2Reg {
    #[must_use]
    #[inline(always)]
    pub const fn slot_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_slot_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pchnl_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_pchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn nchnl_sel(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_nchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
}
impl Default for Slot2Reg {
    #[inline(always)]
    fn default() -> Slot2Reg {
        Slot2Reg(0)
    }
}
impl core::fmt::Debug for Slot2Reg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Slot2Reg")
            .field("slot_en", &self.slot_en())
            .field("pchnl_sel", &self.pchnl_sel())
            .field("nchnl_sel", &self.nchnl_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Slot2Reg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Slot2Reg {{ slot_en: {=bool:?}, pchnl_sel: {=u8:?}, nchnl_sel: {=u8:?} }}",
            self.slot_en(),
            self.pchnl_sel(),
            self.nchnl_sel()
        )
    }
}
#[doc = "ADC Slot3 Config Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slot3Reg(pub u32);
impl Slot3Reg {
    #[must_use]
    #[inline(always)]
    pub const fn slot_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_slot_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pchnl_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_pchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn nchnl_sel(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_nchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
}
impl Default for Slot3Reg {
    #[inline(always)]
    fn default() -> Slot3Reg {
        Slot3Reg(0)
    }
}
impl core::fmt::Debug for Slot3Reg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Slot3Reg")
            .field("slot_en", &self.slot_en())
            .field("pchnl_sel", &self.pchnl_sel())
            .field("nchnl_sel", &self.nchnl_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Slot3Reg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Slot3Reg {{ slot_en: {=bool:?}, pchnl_sel: {=u8:?}, nchnl_sel: {=u8:?} }}",
            self.slot_en(),
            self.pchnl_sel(),
            self.nchnl_sel()
        )
    }
}
#[doc = "ADC Slot4 Config Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slot4Reg(pub u32);
impl Slot4Reg {
    #[must_use]
    #[inline(always)]
    pub const fn slot_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_slot_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pchnl_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_pchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn nchnl_sel(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_nchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
}
impl Default for Slot4Reg {
    #[inline(always)]
    fn default() -> Slot4Reg {
        Slot4Reg(0)
    }
}
impl core::fmt::Debug for Slot4Reg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Slot4Reg")
            .field("slot_en", &self.slot_en())
            .field("pchnl_sel", &self.pchnl_sel())
            .field("nchnl_sel", &self.nchnl_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Slot4Reg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Slot4Reg {{ slot_en: {=bool:?}, pchnl_sel: {=u8:?}, nchnl_sel: {=u8:?} }}",
            self.slot_en(),
            self.pchnl_sel(),
            self.nchnl_sel()
        )
    }
}
#[doc = "ADC Slot5 Config Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slot5Reg(pub u32);
impl Slot5Reg {
    #[must_use]
    #[inline(always)]
    pub const fn slot_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_slot_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pchnl_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_pchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn nchnl_sel(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_nchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
}
impl Default for Slot5Reg {
    #[inline(always)]
    fn default() -> Slot5Reg {
        Slot5Reg(0)
    }
}
impl core::fmt::Debug for Slot5Reg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Slot5Reg")
            .field("slot_en", &self.slot_en())
            .field("pchnl_sel", &self.pchnl_sel())
            .field("nchnl_sel", &self.nchnl_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Slot5Reg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Slot5Reg {{ slot_en: {=bool:?}, pchnl_sel: {=u8:?}, nchnl_sel: {=u8:?} }}",
            self.slot_en(),
            self.pchnl_sel(),
            self.nchnl_sel()
        )
    }
}
#[doc = "ADC Slot6 Config Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slot6Reg(pub u32);
impl Slot6Reg {
    #[must_use]
    #[inline(always)]
    pub const fn slot_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_slot_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pchnl_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_pchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn nchnl_sel(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_nchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
}
impl Default for Slot6Reg {
    #[inline(always)]
    fn default() -> Slot6Reg {
        Slot6Reg(0)
    }
}
impl core::fmt::Debug for Slot6Reg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Slot6Reg")
            .field("slot_en", &self.slot_en())
            .field("pchnl_sel", &self.pchnl_sel())
            .field("nchnl_sel", &self.nchnl_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Slot6Reg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Slot6Reg {{ slot_en: {=bool:?}, pchnl_sel: {=u8:?}, nchnl_sel: {=u8:?} }}",
            self.slot_en(),
            self.pchnl_sel(),
            self.nchnl_sel()
        )
    }
}
#[doc = "ADC Slot7 Config Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slot7Reg(pub u32);
impl Slot7Reg {
    #[must_use]
    #[inline(always)]
    pub const fn slot_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub const fn set_slot_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn pchnl_sel(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_pchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn nchnl_sel(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub const fn set_nchnl_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 11usize)) | (((val as u32) & 0x07) << 11usize);
    }
}
impl Default for Slot7Reg {
    #[inline(always)]
    fn default() -> Slot7Reg {
        Slot7Reg(0)
    }
}
impl core::fmt::Debug for Slot7Reg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Slot7Reg")
            .field("slot_en", &self.slot_en())
            .field("pchnl_sel", &self.pchnl_sel())
            .field("nchnl_sel", &self.nchnl_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Slot7Reg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Slot7Reg {{ slot_en: {=bool:?}, pchnl_sel: {=u8:?}, nchnl_sel: {=u8:?} }}",
            self.slot_en(),
            self.pchnl_sel(),
            self.nchnl_sel()
        )
    }
}
