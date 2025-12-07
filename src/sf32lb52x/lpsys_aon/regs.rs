#[doc = "Active Mode Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Acr(pub u32);
impl Acr {
    #[doc = "Request hrc48 in active mode"]
    #[must_use]
    #[inline(always)]
    pub const fn hrc48_req(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Request hrc48 in active mode"]
    #[inline(always)]
    pub const fn set_hrc48_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Request hxt48 in active mode"]
    #[must_use]
    #[inline(always)]
    pub const fn hxt48_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Request hxt48 in active mode"]
    #[inline(always)]
    pub const fn set_hxt48_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Request power during Active mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pwr_req(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Request power during Active mode"]
    #[inline(always)]
    pub const fn set_pwr_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "for debug only"]
    #[must_use]
    #[inline(always)]
    pub const fn extpwr_req(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn set_extpwr_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Indicate hrc48 is ready"]
    #[must_use]
    #[inline(always)]
    pub const fn hrc48_rdy(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Indicate hrc48 is ready"]
    #[inline(always)]
    pub const fn set_hrc48_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Indicate hxt48 is ready"]
    #[must_use]
    #[inline(always)]
    pub const fn hxt48_rdy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Indicate hxt48 is ready"]
    #[inline(always)]
    pub const fn set_hxt48_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Acr {
    #[inline(always)]
    fn default() -> Acr {
        Acr(0)
    }
}
impl core::fmt::Debug for Acr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Acr")
            .field("hrc48_req", &self.hrc48_req())
            .field("hxt48_req", &self.hxt48_req())
            .field("pwr_req", &self.pwr_req())
            .field("extpwr_req", &self.extpwr_req())
            .field("hrc48_rdy", &self.hrc48_rdy())
            .field("hxt48_rdy", &self.hxt48_rdy())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Acr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Acr {{ hrc48_req: {=bool:?}, hxt48_req: {=bool:?}, pwr_req: {=bool:?}, extpwr_req: {=bool:?}, hrc48_rdy: {=bool:?}, hxt48_rdy: {=bool:?} }}" , self . hrc48_req () , self . hxt48_req () , self . pwr_req () , self . extpwr_req () , self . hrc48_rdy () , self . hxt48_rdy ())
    }
}
#[doc = "BT actual sleep time"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Actual(pub u32);
impl Actual {
    #[doc = "bt actual sleep time in cycles of clk_rtc. If not woken up by software or external interrupt, sleep_cnt counts up every clk_rtc cycle, until reaches sleep_target"]
    #[must_use]
    #[inline(always)]
    pub const fn sleep_cnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "bt actual sleep time in cycles of clk_rtc. If not woken up by software or external interrupt, sleep_cnt counts up every clk_rtc cycle, until reaches sleep_target"]
    #[inline(always)]
    pub const fn set_sleep_cnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
}
impl Default for Actual {
    #[inline(always)]
    fn default() -> Actual {
        Actual(0)
    }
}
impl core::fmt::Debug for Actual {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Actual")
            .field("sleep_cnt", &self.sleep_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Actual {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Actual {{ sleep_cnt: {=u32:?} }}", self.sleep_cnt())
    }
}
#[doc = "Analog Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Anacr(pub u32);
impl Anacr {
    #[doc = "Set 1 to force IO(PB) into retention mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pb_iso(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to force IO(PB) into retention mode"]
    #[inline(always)]
    pub const fn set_pb_iso(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Set 1 to force off all LPSYS related analog modules"]
    #[must_use]
    #[inline(always)]
    pub const fn vlp_iso(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to force off all LPSYS related analog modules"]
    #[inline(always)]
    pub const fn set_vlp_iso(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Anacr {
    #[inline(always)]
    fn default() -> Anacr {
        Anacr(0)
    }
}
impl core::fmt::Debug for Anacr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Anacr")
            .field("pb_iso", &self.pb_iso())
            .field("vlp_iso", &self.vlp_iso())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Anacr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Anacr {{ pb_iso: {=bool:?}, vlp_iso: {=bool:?} }}",
            self.pb_iso(),
            self.vlp_iso()
        )
    }
}
#[doc = "Control Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr1(pub u32);
impl Cr1 {
    #[doc = "mode for wakeup PIN0 (PA24) 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, 4/5/6/7: pos or neg edge"]
    #[must_use]
    #[inline(always)]
    pub const fn pin0_mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "mode for wakeup PIN0 (PA24) 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, 4/5/6/7: pos or neg edge"]
    #[inline(always)]
    pub const fn set_pin0_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "mode for wakeup PIN1 (PA25)"]
    #[must_use]
    #[inline(always)]
    pub const fn pin1_mode(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "mode for wakeup PIN1 (PA25)"]
    #[inline(always)]
    pub const fn set_pin1_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "mode for wakeup PIN2 (PA26)"]
    #[must_use]
    #[inline(always)]
    pub const fn pin2_mode(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "mode for wakeup PIN2 (PA26)"]
    #[inline(always)]
    pub const fn set_pin2_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "mode for wakeup PIN3 (PA27)"]
    #[must_use]
    #[inline(always)]
    pub const fn pin3_mode(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "mode for wakeup PIN3 (PA27)"]
    #[inline(always)]
    pub const fn set_pin3_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "for debug only"]
    #[must_use]
    #[inline(always)]
    pub const fn pinout_sel0(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x07;
        val as u8
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn set_pinout_sel0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
    }
    #[doc = "for debug only"]
    #[must_use]
    #[inline(always)]
    pub const fn pinout_sel1(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x07;
        val as u8
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn set_pinout_sel1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
    }
    #[doc = "Enable global timer"]
    #[must_use]
    #[inline(always)]
    pub const fn gtim_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable global timer"]
    #[inline(always)]
    pub const fn set_gtim_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cr1 {
    #[inline(always)]
    fn default() -> Cr1 {
        Cr1(0)
    }
}
impl core::fmt::Debug for Cr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr1")
            .field("pin0_mode", &self.pin0_mode())
            .field("pin1_mode", &self.pin1_mode())
            .field("pin2_mode", &self.pin2_mode())
            .field("pin3_mode", &self.pin3_mode())
            .field("pinout_sel0", &self.pinout_sel0())
            .field("pinout_sel1", &self.pinout_sel1())
            .field("gtim_en", &self.gtim_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cr1 {{ pin0_mode: {=u8:?}, pin1_mode: {=u8:?}, pin2_mode: {=u8:?}, pin3_mode: {=u8:?}, pinout_sel0: {=u8:?}, pinout_sel1: {=u8:?}, gtim_en: {=bool:?} }}" , self . pin0_mode () , self . pin1_mode () , self . pin2_mode () , self . pin3_mode () , self . pinout_sel0 () , self . pinout_sel1 () , self . gtim_en ())
    }
}
#[doc = "Control Register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr2(pub u32);
impl Cr2 {
    #[doc = "mode for wakeup PIN10 (PA34) 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, 4/5/6/7: pos or neg edge"]
    #[must_use]
    #[inline(always)]
    pub const fn pin10_mode(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "mode for wakeup PIN10 (PA34) 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, 4/5/6/7: pos or neg edge"]
    #[inline(always)]
    pub const fn set_pin10_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "mode for wakeup PIN11 (PA35)"]
    #[must_use]
    #[inline(always)]
    pub const fn pin11_mode(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "mode for wakeup PIN11 (PA35)"]
    #[inline(always)]
    pub const fn set_pin11_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "mode for wakeup PIN12 (PA36)"]
    #[must_use]
    #[inline(always)]
    pub const fn pin12_mode(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "mode for wakeup PIN12 (PA36)"]
    #[inline(always)]
    pub const fn set_pin12_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "mode for wakeup PIN13 (PA37)"]
    #[must_use]
    #[inline(always)]
    pub const fn pin13_mode(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "mode for wakeup PIN13 (PA37)"]
    #[inline(always)]
    pub const fn set_pin13_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "mode for wakeup PIN14 (PA38)"]
    #[must_use]
    #[inline(always)]
    pub const fn pin14_mode(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "mode for wakeup PIN14 (PA38)"]
    #[inline(always)]
    pub const fn set_pin14_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "mode for wakeup PIN15 (PA39)"]
    #[must_use]
    #[inline(always)]
    pub const fn pin15_mode(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "mode for wakeup PIN15 (PA39)"]
    #[inline(always)]
    pub const fn set_pin15_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
}
impl Default for Cr2 {
    #[inline(always)]
    fn default() -> Cr2 {
        Cr2(0)
    }
}
impl core::fmt::Debug for Cr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr2")
            .field("pin10_mode", &self.pin10_mode())
            .field("pin11_mode", &self.pin11_mode())
            .field("pin12_mode", &self.pin12_mode())
            .field("pin13_mode", &self.pin13_mode())
            .field("pin14_mode", &self.pin14_mode())
            .field("pin15_mode", &self.pin15_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cr2 {{ pin10_mode: {=u8:?}, pin11_mode: {=u8:?}, pin12_mode: {=u8:?}, pin13_mode: {=u8:?}, pin14_mode: {=u8:?}, pin15_mode: {=u8:?} }}" , self . pin10_mode () , self . pin11_mode () , self . pin12_mode () , self . pin13_mode () , self . pin14_mode () , self . pin15_mode ())
    }
}
#[doc = "Control Register 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cr3(pub u32);
impl Cr3 {
    #[doc = "mode for wakeup PIN16 (PA40) 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, 4/5/6/7: pos or neg edge"]
    #[must_use]
    #[inline(always)]
    pub const fn pin16_mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "mode for wakeup PIN16 (PA40) 0 - high level, 1 - low level, 2 - pos edge, 3 - neg edge, 4/5/6/7: pos or neg edge"]
    #[inline(always)]
    pub const fn set_pin16_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "mode for wakeup PIN17 (PA41)"]
    #[must_use]
    #[inline(always)]
    pub const fn pin17_mode(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "mode for wakeup PIN17 (PA41)"]
    #[inline(always)]
    pub const fn set_pin17_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "mode for wakeup PIN18 (PA42)"]
    #[must_use]
    #[inline(always)]
    pub const fn pin18_mode(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "mode for wakeup PIN18 (PA42)"]
    #[inline(always)]
    pub const fn set_pin18_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "mode for wakeup PIN19 (PA43)"]
    #[must_use]
    #[inline(always)]
    pub const fn pin19_mode(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "mode for wakeup PIN19 (PA43)"]
    #[inline(always)]
    pub const fn set_pin19_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "mode for wakeup PIN20 (PA44)"]
    #[must_use]
    #[inline(always)]
    pub const fn pin20_mode(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "mode for wakeup PIN20 (PA44)"]
    #[inline(always)]
    pub const fn set_pin20_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
}
impl Default for Cr3 {
    #[inline(always)]
    fn default() -> Cr3 {
        Cr3(0)
    }
}
impl core::fmt::Debug for Cr3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cr3")
            .field("pin16_mode", &self.pin16_mode())
            .field("pin17_mode", &self.pin17_mode())
            .field("pin18_mode", &self.pin18_mode())
            .field("pin19_mode", &self.pin19_mode())
            .field("pin20_mode", &self.pin20_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cr3 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cr3 {{ pin16_mode: {=u8:?}, pin17_mode: {=u8:?}, pin18_mode: {=u8:?}, pin19_mode: {=u8:?}, pin20_mode: {=u8:?} }}" , self . pin16_mode () , self . pin17_mode () , self . pin18_mode () , self . pin19_mode () , self . pin20_mode ())
    }
}
#[doc = "Deep Sleep Ctrl Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dscr(pub u32);
impl Dscr {
    #[doc = "Request hrc48 in Deep Sleep mode"]
    #[must_use]
    #[inline(always)]
    pub const fn hrc48_req(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Request hrc48 in Deep Sleep mode"]
    #[inline(always)]
    pub const fn set_hrc48_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Request hxt48 in Deep Sleep mode"]
    #[must_use]
    #[inline(always)]
    pub const fn hxt48_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Request hxt48 in Deep Sleep mode"]
    #[inline(always)]
    pub const fn set_hxt48_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Request power during Deep Sleep mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pwr_req(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Request power during Deep Sleep mode"]
    #[inline(always)]
    pub const fn set_pwr_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "for debug only"]
    #[must_use]
    #[inline(always)]
    pub const fn extpwr_req(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn set_extpwr_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Dscr {
    #[inline(always)]
    fn default() -> Dscr {
        Dscr(0)
    }
}
impl core::fmt::Debug for Dscr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dscr")
            .field("hrc48_req", &self.hrc48_req())
            .field("hxt48_req", &self.hxt48_req())
            .field("pwr_req", &self.pwr_req())
            .field("extpwr_req", &self.extpwr_req())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dscr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dscr {{ hrc48_req: {=bool:?}, hxt48_req: {=bool:?}, pwr_req: {=bool:?}, extpwr_req: {=bool:?} }}" , self . hrc48_req () , self . hxt48_req () , self . pwr_req () , self . extpwr_req ())
    }
}
#[doc = "Global Timer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtimr(pub u32);
impl Gtimr {
    #[doc = "Global timer value"]
    #[must_use]
    #[inline(always)]
    pub const fn cnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Global timer value"]
    #[inline(always)]
    pub const fn set_cnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gtimr {
    #[inline(always)]
    fn default() -> Gtimr {
        Gtimr(0)
    }
}
impl core::fmt::Debug for Gtimr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gtimr").field("cnt", &self.cnt()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gtimr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Gtimr {{ cnt: {=u32:?} }}", self.cnt())
    }
}
#[doc = "Inter System Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Issr(pub u32);
impl Issr {
    #[doc = "write 1 to request HPSYS to stay in active mode"]
    #[must_use]
    #[inline(always)]
    pub const fn lp2hp_req(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to request HPSYS to stay in active mode"]
    #[inline(always)]
    pub const fn set_lp2hp_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "indicate HPSYS request exists"]
    #[must_use]
    #[inline(always)]
    pub const fn hp2lp_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "indicate HPSYS request exists"]
    #[inline(always)]
    pub const fn set_hp2lp_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "write 1 to indicates LPSYS is active"]
    #[must_use]
    #[inline(always)]
    pub const fn lp_active(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to indicates LPSYS is active"]
    #[inline(always)]
    pub const fn set_lp_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "read 1 indicates HPSYS is active"]
    #[must_use]
    #[inline(always)]
    pub const fn hp_active(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "read 1 indicates HPSYS is active"]
    #[inline(always)]
    pub const fn set_hp_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Issr {
    #[inline(always)]
    fn default() -> Issr {
        Issr(0)
    }
}
impl core::fmt::Debug for Issr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Issr")
            .field("lp2hp_req", &self.lp2hp_req())
            .field("hp2lp_req", &self.hp2lp_req())
            .field("lp_active", &self.lp_active())
            .field("hp_active", &self.hp_active())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Issr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Issr {{ lp2hp_req: {=bool:?}, hp2lp_req: {=bool:?}, lp_active: {=bool:?}, hp_active: {=bool:?} }}" , self . lp2hp_req () , self . hp2lp_req () , self . lp_active () , self . hp_active ())
    }
}
#[doc = "Light Sleep Ctrl Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lscr(pub u32);
impl Lscr {
    #[doc = "Request hrc48 in Light Sleep mode"]
    #[must_use]
    #[inline(always)]
    pub const fn hrc48_req(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Request hrc48 in Light Sleep mode"]
    #[inline(always)]
    pub const fn set_hrc48_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Request hxt48 in Light Sleep mode"]
    #[must_use]
    #[inline(always)]
    pub const fn hxt48_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Request hxt48 in Light Sleep mode"]
    #[inline(always)]
    pub const fn set_hxt48_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Request power during Light Sleep mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pwr_req(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Request power during Light Sleep mode"]
    #[inline(always)]
    pub const fn set_pwr_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "for debug only"]
    #[must_use]
    #[inline(always)]
    pub const fn extpwr_req(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn set_extpwr_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Lscr {
    #[inline(always)]
    fn default() -> Lscr {
        Lscr(0)
    }
}
impl core::fmt::Debug for Lscr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lscr")
            .field("hrc48_req", &self.hrc48_req())
            .field("hxt48_req", &self.hxt48_req())
            .field("pwr_req", &self.pwr_req())
            .field("extpwr_req", &self.extpwr_req())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lscr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Lscr {{ hrc48_req: {=bool:?}, hxt48_req: {=bool:?}, pwr_req: {=bool:?}, extpwr_req: {=bool:?} }}" , self . hrc48_req () , self . hxt48_req () , self . pwr_req () , self . extpwr_req ())
    }
}
#[doc = "Pointer Counter Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcr(pub u32);
impl Pcr {
    #[doc = "LCPU PC pointer address"]
    #[must_use]
    #[inline(always)]
    pub const fn pc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "LCPU PC pointer address"]
    #[inline(always)]
    pub const fn set_pc(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pcr {
    #[inline(always)]
    fn default() -> Pcr {
        Pcr(0)
    }
}
impl core::fmt::Debug for Pcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pcr").field("pc", &self.pc()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Pcr {{ pc: {=u32:?} }}", self.pc())
    }
}
#[doc = "Power Mode Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmr(pub u32);
impl Pmr {
    #[doc = "Power Mode: 2'h0 - active/idle; 2'h1 - light sleep; 2'h2 - deep sleep; 2'h3 - standby"]
    #[must_use]
    #[inline(always)]
    pub const fn mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Power Mode: 2'h0 - active/idle; 2'h1 - light sleep; 2'h2 - deep sleep; 2'h3 - standby"]
    #[inline(always)]
    pub const fn set_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Stall CPU out of reset. Should be cleared before LCPU run"]
    #[must_use]
    #[inline(always)]
    pub const fn cpuwait(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Stall CPU out of reset. Should be cleared before LCPU run"]
    #[inline(always)]
    pub const fn set_cpuwait(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set 1 to force enter low power mode. Will be cleared automatically"]
    #[must_use]
    #[inline(always)]
    pub const fn force_sleep(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to force enter low power mode. Will be cleared automatically"]
    #[inline(always)]
    pub const fn set_force_sleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pmr {
    #[inline(always)]
    fn default() -> Pmr {
        Pmr(0)
    }
}
impl core::fmt::Debug for Pmr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pmr")
            .field("mode", &self.mode())
            .field("cpuwait", &self.cpuwait())
            .field("force_sleep", &self.force_sleep())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pmr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Pmr {{ mode: {=u8:?}, cpuwait: {=bool:?}, force_sleep: {=bool:?} }}",
            self.mode(),
            self.cpuwait(),
            self.force_sleep()
        )
    }
}
#[doc = "time before bt awake"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PreWkup(pub u32);
impl PreWkup {
    #[doc = "cycles of clk_rtc for hxt48 ready before bt awake."]
    #[must_use]
    #[inline(always)]
    pub const fn xtal_time(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "cycles of clk_rtc for hxt48 ready before bt awake."]
    #[inline(always)]
    pub const fn set_xtal_time(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "cycles of clk_rtc for LPSYS ready before bt awake."]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_time(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "cycles of clk_rtc for LPSYS ready before bt awake."]
    #[inline(always)]
    pub const fn set_wkup_time(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for PreWkup {
    #[inline(always)]
    fn default() -> PreWkup {
        PreWkup(0)
    }
}
impl core::fmt::Debug for PreWkup {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PreWkup")
            .field("xtal_time", &self.xtal_time())
            .field("wkup_time", &self.wkup_time())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PreWkup {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PreWkup {{ xtal_time: {=u16:?}, wkup_time: {=u16:?} }}",
            self.xtal_time(),
            self.wkup_time()
        )
    }
}
#[doc = "Reserved Register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reserve0(pub u32);
impl Reserve0 {
    #[doc = "for debug only"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Reserve0 {
    #[inline(always)]
    fn default() -> Reserve0 {
        Reserve0(0)
    }
}
impl core::fmt::Debug for Reserve0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reserve0")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reserve0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Reserve0 {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "Reserved Register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reserve1(pub u32);
impl Reserve1 {
    #[doc = "for debug only"]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Reserve1 {
    #[inline(always)]
    fn default() -> Reserve1 {
        Reserve1(0)
    }
}
impl core::fmt::Debug for Reserve1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reserve1")
            .field("data", &self.data())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reserve1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Reserve1 {{ data: {=u32:?} }}", self.data())
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
#[doc = "Standby Mode Ctrl Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbcr(pub u32);
impl Sbcr {
    #[doc = "Request hrc48 in Standby mode"]
    #[must_use]
    #[inline(always)]
    pub const fn hrc48_req(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Request hrc48 in Standby mode"]
    #[inline(always)]
    pub const fn set_hrc48_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Request hxt48 in Standby mode"]
    #[must_use]
    #[inline(always)]
    pub const fn hxt48_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Request hxt48 in Standby mode"]
    #[inline(always)]
    pub const fn set_hxt48_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Request power during Standby mode"]
    #[must_use]
    #[inline(always)]
    pub const fn pwr_req(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Request power during Standby mode"]
    #[inline(always)]
    pub const fn set_pwr_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "for debug only"]
    #[must_use]
    #[inline(always)]
    pub const fn extpwr_req(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn set_extpwr_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "for debug only"]
    #[must_use]
    #[inline(always)]
    pub const fn pd_ram0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn set_pd_ram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "for debug only"]
    #[must_use]
    #[inline(always)]
    pub const fn pd_ram1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn set_pd_ram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Sbcr {
    #[inline(always)]
    fn default() -> Sbcr {
        Sbcr(0)
    }
}
impl core::fmt::Debug for Sbcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sbcr")
            .field("hrc48_req", &self.hrc48_req())
            .field("hxt48_req", &self.hxt48_req())
            .field("pwr_req", &self.pwr_req())
            .field("extpwr_req", &self.extpwr_req())
            .field("pd_ram0", &self.pd_ram0())
            .field("pd_ram1", &self.pd_ram1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sbcr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Sbcr {{ hrc48_req: {=bool:?}, hxt48_req: {=bool:?}, pwr_req: {=bool:?}, extpwr_req: {=bool:?}, pd_ram0: {=bool:?}, pd_ram1: {=bool:?} }}" , self . hrc48_req () , self . hxt48_req () , self . pwr_req () , self . extpwr_req () , self . pd_ram0 () , self . pd_ram1 ())
    }
}
#[doc = "BT sleep configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SlpCfg(pub u32);
impl SlpCfg {
    #[doc = "for debug only"]
    #[must_use]
    #[inline(always)]
    pub const fn xtal_always_on(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn set_xtal_always_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "for debug only"]
    #[must_use]
    #[inline(always)]
    pub const fn xtal_force_off(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn set_xtal_force_off(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for SlpCfg {
    #[inline(always)]
    fn default() -> SlpCfg {
        SlpCfg(0)
    }
}
impl core::fmt::Debug for SlpCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SlpCfg")
            .field("xtal_always_on", &self.xtal_always_on())
            .field("xtal_force_off", &self.xtal_force_off())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SlpCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SlpCfg {{ xtal_always_on: {=bool:?}, xtal_force_off: {=bool:?} }}",
            self.xtal_always_on(),
            self.xtal_force_off()
        )
    }
}
#[doc = "BT sleep control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SlpCtrl(pub u32);
impl SlpCtrl {
    #[doc = "bt sleep request. Will be cleared automatically"]
    #[must_use]
    #[inline(always)]
    pub const fn sleep_req(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "bt sleep request. Will be cleared automatically"]
    #[inline(always)]
    pub const fn set_sleep_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "software request to wakeup bt. Will be cleared automatically"]
    #[must_use]
    #[inline(always)]
    pub const fn wkup_req(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "software request to wakeup bt. Will be cleared automatically"]
    #[inline(always)]
    pub const fn set_wkup_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "bt sleep status. 1 means bt is sleeping and sleep_cnt is counting up"]
    #[must_use]
    #[inline(always)]
    pub const fn sleep_status(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "bt sleep status. 1 means bt is sleeping and sleep_cnt is counting up"]
    #[inline(always)]
    pub const fn set_sleep_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "xtal request status. 1 means bt is requiring xtal."]
    #[must_use]
    #[inline(always)]
    pub const fn xtal_req(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "xtal request status. 1 means bt is requiring xtal."]
    #[inline(always)]
    pub const fn set_xtal_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "bt wakeup source. 1 means bt has not enter sleep or has enter wakeup procedure"]
    #[must_use]
    #[inline(always)]
    pub const fn bt_wkup(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "bt wakeup source. 1 means bt has not enter sleep or has enter wakeup procedure"]
    #[inline(always)]
    pub const fn set_bt_wkup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for SlpCtrl {
    #[inline(always)]
    fn default() -> SlpCtrl {
        SlpCtrl(0)
    }
}
impl core::fmt::Debug for SlpCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SlpCtrl")
            .field("sleep_req", &self.sleep_req())
            .field("wkup_req", &self.wkup_req())
            .field("sleep_status", &self.sleep_status())
            .field("xtal_req", &self.xtal_req())
            .field("bt_wkup", &self.bt_wkup())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SlpCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "SlpCtrl {{ sleep_req: {=bool:?}, wkup_req: {=bool:?}, sleep_status: {=bool:?}, xtal_req: {=bool:?}, bt_wkup: {=bool:?} }}" , self . sleep_req () , self . wkup_req () , self . sleep_status () , self . xtal_req () , self . bt_wkup ())
    }
}
#[doc = "Stack Pointer Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spr(pub u32);
impl Spr {
    #[doc = "LCPU stack pointer address"]
    #[must_use]
    #[inline(always)]
    pub const fn sp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "LCPU stack pointer address"]
    #[inline(always)]
    pub const fn set_sp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Spr {
    #[inline(always)]
    fn default() -> Spr {
        Spr(0)
    }
}
impl core::fmt::Debug for Spr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Spr").field("sp", &self.sp()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Spr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Spr {{ sp: {=u32:?} }}", self.sp())
    }
}
#[doc = "BT sleep time target"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Target(pub u32);
impl Target {
    #[doc = "bt sleep time target in cycles of clk_rtc"]
    #[must_use]
    #[inline(always)]
    pub const fn sleep_target(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "bt sleep time target in cycles of clk_rtc"]
    #[inline(always)]
    pub const fn set_sleep_target(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
}
impl Default for Target {
    #[inline(always)]
    fn default() -> Target {
        Target(0)
    }
}
impl core::fmt::Debug for Target {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Target")
            .field("sleep_target", &self.sleep_target())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Target {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Target {{ sleep_target: {=u32:?} }}",
            self.sleep_target()
        )
    }
}
#[doc = "Wakeup Clear register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wcr(pub u32);
impl Wcr {
    #[doc = "Write 1 to clear PA24 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn pin0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear PA24 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[inline(always)]
    pub const fn set_pin0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Write 1 to clear PA25 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn pin1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear PA25 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[inline(always)]
    pub const fn set_pin1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Write 1 to clear PA26 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn pin2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear PA26 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[inline(always)]
    pub const fn set_pin2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Write 1 to clear PA27 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn pin3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear PA27 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[inline(always)]
    pub const fn set_pin3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Write 1 to clear PA34 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn pin10(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear PA34 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[inline(always)]
    pub const fn set_pin10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Write 1 to clear PA35 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn pin11(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear PA35 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[inline(always)]
    pub const fn set_pin11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Write 1 to clear PA36 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn pin12(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear PA36 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[inline(always)]
    pub const fn set_pin12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Write 1 to clear PA37 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn pin13(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear PA37 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[inline(always)]
    pub const fn set_pin13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Write 1 to clear PA38 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn pin14(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear PA38 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[inline(always)]
    pub const fn set_pin14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Write 1 to clear PA39 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn pin15(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear PA39 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[inline(always)]
    pub const fn set_pin15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Write 1 to clear PA40 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn pin16(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear PA40 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[inline(always)]
    pub const fn set_pin16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Write 1 to clear PA41 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn pin17(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear PA41 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[inline(always)]
    pub const fn set_pin17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Write 1 to clear PA42 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn pin18(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear PA42 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[inline(always)]
    pub const fn set_pin18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Write 1 to clear PA43 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn pin19(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear PA43 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[inline(always)]
    pub const fn set_pin19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Write 1 to clear PA44 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[must_use]
    #[inline(always)]
    pub const fn pin20(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to clear PA44 wakeup source. Only valid if PIN wakeup is configured as edge trigger"]
    #[inline(always)]
    pub const fn set_pin20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
            .field("pin0", &self.pin0())
            .field("pin1", &self.pin1())
            .field("pin2", &self.pin2())
            .field("pin3", &self.pin3())
            .field("pin10", &self.pin10())
            .field("pin11", &self.pin11())
            .field("pin12", &self.pin12())
            .field("pin13", &self.pin13())
            .field("pin14", &self.pin14())
            .field("pin15", &self.pin15())
            .field("pin16", &self.pin16())
            .field("pin17", &self.pin17())
            .field("pin18", &self.pin18())
            .field("pin19", &self.pin19())
            .field("pin20", &self.pin20())
            .field("aon", &self.aon())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wcr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Wcr {{ pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin10: {=bool:?}, pin11: {=bool:?}, pin12: {=bool:?}, pin13: {=bool:?}, pin14: {=bool:?}, pin15: {=bool:?}, pin16: {=bool:?}, pin17: {=bool:?}, pin18: {=bool:?}, pin19: {=bool:?}, pin20: {=bool:?}, aon: {=bool:?} }}" , self . pin0 () , self . pin1 () , self . pin2 () , self . pin3 () , self . pin10 () , self . pin11 () , self . pin12 () , self . pin13 () , self . pin14 () , self . pin15 () , self . pin16 () , self . pin17 () , self . pin18 () , self . pin19 () , self . pin20 () , self . aon ())
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
    #[doc = "Set 1 to enable IO(PB) as wakeup source"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable IO(PB) as wakeup source"]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set 1 to enable LPTIM3 as wakeup source"]
    #[must_use]
    #[inline(always)]
    pub const fn lptim3(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable LPTIM3 as wakeup source"]
    #[inline(always)]
    pub const fn set_lptim3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set 1 to enable BT as wakeup source"]
    #[must_use]
    #[inline(always)]
    pub const fn bt(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable BT as wakeup source"]
    #[inline(always)]
    pub const fn set_bt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Set 1 to enable HPSYS request as wakeup source"]
    #[must_use]
    #[inline(always)]
    pub const fn hp2lp_req(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable HPSYS request as wakeup source"]
    #[inline(always)]
    pub const fn set_hp2lp_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Set 1 to enable MAILBOX1 as wakeup source"]
    #[must_use]
    #[inline(always)]
    pub const fn hp2lp_irq(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable MAILBOX1 as wakeup source"]
    #[inline(always)]
    pub const fn set_hp2lp_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Set 1 to enable PA24 as wakeup source"]
    #[must_use]
    #[inline(always)]
    pub const fn pin0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable PA24 as wakeup source"]
    #[inline(always)]
    pub const fn set_pin0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Set 1 to enable PA25 as wakeup source"]
    #[must_use]
    #[inline(always)]
    pub const fn pin1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable PA25 as wakeup source"]
    #[inline(always)]
    pub const fn set_pin1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Set 1 to enable PA26 as wakeup source"]
    #[must_use]
    #[inline(always)]
    pub const fn pin2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable PA26 as wakeup source"]
    #[inline(always)]
    pub const fn set_pin2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Set 1 to enable PA27 as wakeup source"]
    #[must_use]
    #[inline(always)]
    pub const fn pin3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable PA27 as wakeup source"]
    #[inline(always)]
    pub const fn set_pin3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Set 1 to enable PA34 as wakeup source"]
    #[must_use]
    #[inline(always)]
    pub const fn pin10(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable PA34 as wakeup source"]
    #[inline(always)]
    pub const fn set_pin10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Set 1 to enable PA35 as wakeup source"]
    #[must_use]
    #[inline(always)]
    pub const fn pin11(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable PA35 as wakeup source"]
    #[inline(always)]
    pub const fn set_pin11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Set 1 to enable PA36 as wakeup source"]
    #[must_use]
    #[inline(always)]
    pub const fn pin12(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable PA36 as wakeup source"]
    #[inline(always)]
    pub const fn set_pin12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Set 1 to enable PA37 as wakeup source"]
    #[must_use]
    #[inline(always)]
    pub const fn pin13(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable PA37 as wakeup source"]
    #[inline(always)]
    pub const fn set_pin13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Set 1 to enable PA38 as wakeup source"]
    #[must_use]
    #[inline(always)]
    pub const fn pin14(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable PA38 as wakeup source"]
    #[inline(always)]
    pub const fn set_pin14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Set 1 to enable PA39 as wakeup source"]
    #[must_use]
    #[inline(always)]
    pub const fn pin15(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable PA39 as wakeup source"]
    #[inline(always)]
    pub const fn set_pin15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Set 1 to enable PA40 as wakeup source"]
    #[must_use]
    #[inline(always)]
    pub const fn pin16(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable PA40 as wakeup source"]
    #[inline(always)]
    pub const fn set_pin16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Set 1 to enable PA41 as wakeup source"]
    #[must_use]
    #[inline(always)]
    pub const fn pin17(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable PA41 as wakeup source"]
    #[inline(always)]
    pub const fn set_pin17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Set 1 to enable PA42 as wakeup source"]
    #[must_use]
    #[inline(always)]
    pub const fn pin18(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable PA42 as wakeup source"]
    #[inline(always)]
    pub const fn set_pin18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Set 1 to enable PA43 as wakeup source"]
    #[must_use]
    #[inline(always)]
    pub const fn pin19(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable PA43 as wakeup source"]
    #[inline(always)]
    pub const fn set_pin19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Set 1 to enable PA44 as wakeup source"]
    #[must_use]
    #[inline(always)]
    pub const fn pin20(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to enable PA44 as wakeup source"]
    #[inline(always)]
    pub const fn set_pin20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
            .field("gpio2", &self.gpio2())
            .field("lptim3", &self.lptim3())
            .field("bt", &self.bt())
            .field("hp2lp_req", &self.hp2lp_req())
            .field("hp2lp_irq", &self.hp2lp_irq())
            .field("pin0", &self.pin0())
            .field("pin1", &self.pin1())
            .field("pin2", &self.pin2())
            .field("pin3", &self.pin3())
            .field("pin10", &self.pin10())
            .field("pin11", &self.pin11())
            .field("pin12", &self.pin12())
            .field("pin13", &self.pin13())
            .field("pin14", &self.pin14())
            .field("pin15", &self.pin15())
            .field("pin16", &self.pin16())
            .field("pin17", &self.pin17())
            .field("pin18", &self.pin18())
            .field("pin19", &self.pin19())
            .field("pin20", &self.pin20())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wer {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Wer {{ rtc: {=bool:?}, gpio2: {=bool:?}, lptim3: {=bool:?}, bt: {=bool:?}, hp2lp_req: {=bool:?}, hp2lp_irq: {=bool:?}, pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin10: {=bool:?}, pin11: {=bool:?}, pin12: {=bool:?}, pin13: {=bool:?}, pin14: {=bool:?}, pin15: {=bool:?}, pin16: {=bool:?}, pin17: {=bool:?}, pin18: {=bool:?}, pin19: {=bool:?}, pin20: {=bool:?} }}" , self . rtc () , self . gpio2 () , self . lptim3 () , self . bt () , self . hp2lp_req () , self . hp2lp_irq () , self . pin0 () , self . pin1 () , self . pin2 () , self . pin3 () , self . pin10 () , self . pin11 () , self . pin12 () , self . pin13 () , self . pin14 () , self . pin15 () , self . pin16 () , self . pin17 () , self . pin18 () , self . pin19 () , self . pin20 ())
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
    #[doc = "Indicates the wakeup status from IO(PB). Note: the status is masked by WER"]
    #[must_use]
    #[inline(always)]
    pub const fn gpio2(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the wakeup status from IO(PB). Note: the status is masked by WER"]
    #[inline(always)]
    pub const fn set_gpio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates the wakeup status from LPTIM3. Note: the status is masked by WER"]
    #[must_use]
    #[inline(always)]
    pub const fn lptim3(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the wakeup status from LPTIM3. Note: the status is masked by WER"]
    #[inline(always)]
    pub const fn set_lptim3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Indicates the wakeup status from BT. Note: the status is masked by WER"]
    #[must_use]
    #[inline(always)]
    pub const fn bt(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the wakeup status from BT. Note: the status is masked by WER"]
    #[inline(always)]
    pub const fn set_bt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Indicates the wakeup status from HPSYS request. Note: the status is masked by WER"]
    #[must_use]
    #[inline(always)]
    pub const fn hp2lp_req(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the wakeup status from HPSYS request. Note: the status is masked by WER"]
    #[inline(always)]
    pub const fn set_hp2lp_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Indicates the wakeup status from MAILBOX1. Note: the status is masked by WER"]
    #[must_use]
    #[inline(always)]
    pub const fn hp2lp_irq(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the wakeup status from MAILBOX1. Note: the status is masked by WER"]
    #[inline(always)]
    pub const fn set_hp2lp_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Indicates the wakeup status from PA24 request. Note: the status is masked by WER"]
    #[must_use]
    #[inline(always)]
    pub const fn pin0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the wakeup status from PA24 request. Note: the status is masked by WER"]
    #[inline(always)]
    pub const fn set_pin0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Indicates the wakeup status from PA25 request. Note: the status is masked by WER"]
    #[must_use]
    #[inline(always)]
    pub const fn pin1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the wakeup status from PA25 request. Note: the status is masked by WER"]
    #[inline(always)]
    pub const fn set_pin1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Indicates the wakeup status from PA26 request. Note: the status is masked by WER"]
    #[must_use]
    #[inline(always)]
    pub const fn pin2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the wakeup status from PA26 request. Note: the status is masked by WER"]
    #[inline(always)]
    pub const fn set_pin2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Indicates the wakeup status from PA27 request. Note: the status is masked by WER"]
    #[must_use]
    #[inline(always)]
    pub const fn pin3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the wakeup status from PA27 request. Note: the status is masked by WER"]
    #[inline(always)]
    pub const fn set_pin3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Indicates the wakeup status from PA34 request. Note: the status is masked by WER"]
    #[must_use]
    #[inline(always)]
    pub const fn pin10(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the wakeup status from PA34 request. Note: the status is masked by WER"]
    #[inline(always)]
    pub const fn set_pin10(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Indicates the wakeup status from PA35 request. Note: the status is masked by WER"]
    #[must_use]
    #[inline(always)]
    pub const fn pin11(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the wakeup status from PA35 request. Note: the status is masked by WER"]
    #[inline(always)]
    pub const fn set_pin11(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Indicates the wakeup status from PA36 request. Note: the status is masked by WER"]
    #[must_use]
    #[inline(always)]
    pub const fn pin12(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the wakeup status from PA36 request. Note: the status is masked by WER"]
    #[inline(always)]
    pub const fn set_pin12(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Indicates the wakeup status from PA37 request. Note: the status is masked by WER"]
    #[must_use]
    #[inline(always)]
    pub const fn pin13(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the wakeup status from PA37 request. Note: the status is masked by WER"]
    #[inline(always)]
    pub const fn set_pin13(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Indicates the wakeup status from PA38 request. Note: the status is masked by WER"]
    #[must_use]
    #[inline(always)]
    pub const fn pin14(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the wakeup status from PA38 request. Note: the status is masked by WER"]
    #[inline(always)]
    pub const fn set_pin14(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Indicates the wakeup status from PA39 request. Note: the status is masked by WER"]
    #[must_use]
    #[inline(always)]
    pub const fn pin15(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the wakeup status from PA39 request. Note: the status is masked by WER"]
    #[inline(always)]
    pub const fn set_pin15(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Indicates the wakeup status from PA40 request. Note: the status is masked by WER"]
    #[must_use]
    #[inline(always)]
    pub const fn pin16(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the wakeup status from PA40 request. Note: the status is masked by WER"]
    #[inline(always)]
    pub const fn set_pin16(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Indicates the wakeup status from PA41 request. Note: the status is masked by WER"]
    #[must_use]
    #[inline(always)]
    pub const fn pin17(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the wakeup status from PA41 request. Note: the status is masked by WER"]
    #[inline(always)]
    pub const fn set_pin17(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Indicates the wakeup status from PA42 request. Note: the status is masked by WER"]
    #[must_use]
    #[inline(always)]
    pub const fn pin18(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the wakeup status from PA42 request. Note: the status is masked by WER"]
    #[inline(always)]
    pub const fn set_pin18(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Indicates the wakeup status from PA43 request. Note: the status is masked by WER"]
    #[must_use]
    #[inline(always)]
    pub const fn pin19(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the wakeup status from PA43 request. Note: the status is masked by WER"]
    #[inline(always)]
    pub const fn set_pin19(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Indicates the wakeup status from PA44 request. Note: the status is masked by WER"]
    #[must_use]
    #[inline(always)]
    pub const fn pin20(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the wakeup status from PA44 request. Note: the status is masked by WER"]
    #[inline(always)]
    pub const fn set_pin20(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
            .field("gpio2", &self.gpio2())
            .field("lptim3", &self.lptim3())
            .field("bt", &self.bt())
            .field("hp2lp_req", &self.hp2lp_req())
            .field("hp2lp_irq", &self.hp2lp_irq())
            .field("pin0", &self.pin0())
            .field("pin1", &self.pin1())
            .field("pin2", &self.pin2())
            .field("pin3", &self.pin3())
            .field("pin10", &self.pin10())
            .field("pin11", &self.pin11())
            .field("pin12", &self.pin12())
            .field("pin13", &self.pin13())
            .field("pin14", &self.pin14())
            .field("pin15", &self.pin15())
            .field("pin16", &self.pin16())
            .field("pin17", &self.pin17())
            .field("pin18", &self.pin18())
            .field("pin19", &self.pin19())
            .field("pin20", &self.pin20())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wsr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Wsr {{ rtc: {=bool:?}, gpio2: {=bool:?}, lptim3: {=bool:?}, bt: {=bool:?}, hp2lp_req: {=bool:?}, hp2lp_irq: {=bool:?}, pin0: {=bool:?}, pin1: {=bool:?}, pin2: {=bool:?}, pin3: {=bool:?}, pin10: {=bool:?}, pin11: {=bool:?}, pin12: {=bool:?}, pin13: {=bool:?}, pin14: {=bool:?}, pin15: {=bool:?}, pin16: {=bool:?}, pin17: {=bool:?}, pin18: {=bool:?}, pin19: {=bool:?}, pin20: {=bool:?} }}" , self . rtc () , self . gpio2 () , self . lptim3 () , self . bt () , self . hp2lp_req () , self . hp2lp_irq () , self . pin0 () , self . pin1 () , self . pin2 () , self . pin3 () , self . pin10 () , self . pin11 () , self . pin12 () , self . pin13 () , self . pin14 () , self . pin15 () , self . pin16 () , self . pin17 () , self . pin18 () , self . pin19 () , self . pin20 ())
    }
}
