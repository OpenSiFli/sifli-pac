#[doc = "TSEN Analog Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TsenCtrlReg(pub u32);
impl TsenCtrlReg {
    #[doc = "power up tsen"]
    #[inline(always)]
    pub const fn anau_tsen_pu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "power up tsen"]
    #[inline(always)]
    pub fn set_anau_tsen_pu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "resetb for tsen"]
    #[inline(always)]
    pub const fn anau_tsen_rstb(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "resetb for tsen"]
    #[inline(always)]
    pub fn set_anau_tsen_rstb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "enable tsen run"]
    #[inline(always)]
    pub const fn anau_tsen_run(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "enable tsen run"]
    #[inline(always)]
    pub fn set_anau_tsen_run(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "bias current selection to tune vba"]
    #[inline(always)]
    pub const fn anau_tsen_ig_vbe(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "bias current selection to tune vba"]
    #[inline(always)]
    pub fn set_anau_tsen_ig_vbe(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "select internal clock frequency"]
    #[inline(always)]
    pub const fn anau_tsen_fck_sel(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "select internal clock frequency"]
    #[inline(always)]
    pub fn set_anau_tsen_fck_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "signature-mode enable"]
    #[inline(always)]
    pub const fn anau_tsen_sgn_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "signature-mode enable"]
    #[inline(always)]
    pub fn set_anau_tsen_sgn_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "serial-parallel output selection"]
    #[inline(always)]
    pub const fn anau_tsen_ser_par_sel(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "serial-parallel output selection"]
    #[inline(always)]
    pub fn set_anau_tsen_ser_par_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "tsen ready"]
    #[inline(always)]
    pub const fn anau_tsen_rdy(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "tsen ready"]
    #[inline(always)]
    pub fn set_anau_tsen_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Enable tsen digital module"]
    #[inline(always)]
    pub const fn anau_tsen_en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Enable tsen digital module"]
    #[inline(always)]
    pub fn set_anau_tsen_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "gen tsen clk by divide hclk by anau_tsen_clk_div"]
    #[inline(always)]
    pub const fn anau_tsen_clk_div(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x3f;
        val as u8
    }
    #[doc = "gen tsen clk by divide hclk by anau_tsen_clk_div"]
    #[inline(always)]
    pub fn set_anau_tsen_clk_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
    }
}
impl Default for TsenCtrlReg {
    #[inline(always)]
    fn default() -> TsenCtrlReg {
        TsenCtrlReg(0)
    }
}
impl core::fmt::Debug for TsenCtrlReg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TsenCtrlReg")
            .field("anau_tsen_pu", &self.anau_tsen_pu())
            .field("anau_tsen_rstb", &self.anau_tsen_rstb())
            .field("anau_tsen_run", &self.anau_tsen_run())
            .field("anau_tsen_ig_vbe", &self.anau_tsen_ig_vbe())
            .field("anau_tsen_fck_sel", &self.anau_tsen_fck_sel())
            .field("anau_tsen_sgn_en", &self.anau_tsen_sgn_en())
            .field("anau_tsen_ser_par_sel", &self.anau_tsen_ser_par_sel())
            .field("anau_tsen_rdy", &self.anau_tsen_rdy())
            .field("anau_tsen_en", &self.anau_tsen_en())
            .field("anau_tsen_clk_div", &self.anau_tsen_clk_div())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TsenCtrlReg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TsenCtrlReg {{ anau_tsen_pu: {=bool:?}, anau_tsen_rstb: {=bool:?}, anau_tsen_run: {=bool:?}, anau_tsen_ig_vbe: {=u8:?}, anau_tsen_fck_sel: {=u8:?}, anau_tsen_sgn_en: {=bool:?}, anau_tsen_ser_par_sel: {=bool:?}, anau_tsen_rdy: {=bool:?}, anau_tsen_en: {=bool:?}, anau_tsen_clk_div: {=u8:?} }}" , self . anau_tsen_pu () , self . anau_tsen_rstb () , self . anau_tsen_run () , self . anau_tsen_ig_vbe () , self . anau_tsen_fck_sel () , self . anau_tsen_sgn_en () , self . anau_tsen_ser_par_sel () , self . anau_tsen_rdy () , self . anau_tsen_en () , self . anau_tsen_clk_div ())
    }
}
#[doc = "Tsen IRQ Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TsenIrq(pub u32);
impl TsenIrq {
    #[inline(always)]
    pub const fn tsen_icr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_tsen_icr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn tsen_imr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_tsen_imr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn tsen_irsr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_tsen_irsr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn tsen_isr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_tsen_isr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for TsenIrq {
    #[inline(always)]
    fn default() -> TsenIrq {
        TsenIrq(0)
    }
}
impl core::fmt::Debug for TsenIrq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TsenIrq")
            .field("tsen_icr", &self.tsen_icr())
            .field("tsen_imr", &self.tsen_imr())
            .field("tsen_irsr", &self.tsen_irsr())
            .field("tsen_isr", &self.tsen_isr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TsenIrq {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TsenIrq {{ tsen_icr: {=bool:?}, tsen_imr: {=bool:?}, tsen_irsr: {=bool:?}, tsen_isr: {=bool:?} }}" , self . tsen_icr () , self . tsen_imr () , self . tsen_irsr () , self . tsen_isr ())
    }
}
#[doc = "Tsen Read Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TsenRdata(pub u32);
impl TsenRdata {
    #[inline(always)]
    pub const fn tsen_rdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub fn set_tsen_rdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for TsenRdata {
    #[inline(always)]
    fn default() -> TsenRdata {
        TsenRdata(0)
    }
}
impl core::fmt::Debug for TsenRdata {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TsenRdata")
            .field("tsen_rdata", &self.tsen_rdata())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TsenRdata {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TsenRdata {{ tsen_rdata: {=u16:?} }}", self.tsen_rdata())
    }
}
