#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CalCfg(pub u32);
impl CalCfg {
    #[doc = "osc force enable"]
    #[inline(always)]
    pub const fn osc_clk_force_on(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "osc force enable"]
    #[inline(always)]
    pub fn set_osc_clk_force_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "osc clock select"]
    #[inline(always)]
    pub const fn osc_clk_sel(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x07;
        val as u8
    }
    #[doc = "osc clock select"]
    #[inline(always)]
    pub fn set_osc_clk_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
    }
    #[doc = "calibration enable"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "calibration enable"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "calibration done"]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "calibration done"]
    #[inline(always)]
    pub fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "calibration length"]
    #[inline(always)]
    pub const fn length(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "calibration length"]
    #[inline(always)]
    pub fn set_length(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CalCfg {
    #[inline(always)]
    fn default() -> CalCfg {
        CalCfg(0)
    }
}
impl core::fmt::Debug for CalCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CalCfg")
            .field("osc_clk_force_on", &self.osc_clk_force_on())
            .field("osc_clk_sel", &self.osc_clk_sel())
            .field("enable", &self.enable())
            .field("done", &self.done())
            .field("length", &self.length())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CalCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "CalCfg {{ osc_clk_force_on: {=bool:?}, osc_clk_sel: {=u8:?}, enable: {=bool:?}, done: {=bool:?}, length: {=u16:?} }}" , self . osc_clk_force_on () , self . osc_clk_sel () , self . enable () , self . done () , self . length ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CalResult(pub u32);
impl CalResult {
    #[doc = "pclk calibration counter result"]
    #[inline(always)]
    pub const fn pclk_cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "pclk calibration counter result"]
    #[inline(always)]
    pub fn set_pclk_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "osc clock calibration counter result"]
    #[inline(always)]
    pub const fn osc_cnt(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "osc clock calibration counter result"]
    #[inline(always)]
    pub fn set_osc_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for CalResult {
    #[inline(always)]
    fn default() -> CalResult {
        CalResult(0)
    }
}
impl core::fmt::Debug for CalResult {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CalResult")
            .field("pclk_cnt", &self.pclk_cnt())
            .field("osc_cnt", &self.osc_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CalResult {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "CalResult {{ pclk_cnt: {=u16:?}, osc_cnt: {=u16:?} }}",
            self.pclk_cnt(),
            self.osc_cnt()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub const fn auto_clock_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "auto clock gating enable"]
    #[inline(always)]
    pub fn set_auto_clock_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "set 1 to use external seed to generate random number"]
    #[inline(always)]
    pub const fn use_ext_seed(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "set 1 to use external seed to generate random number"]
    #[inline(always)]
    pub fn set_use_ext_seed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "random seed internal VN corrector check threshold"]
    #[inline(always)]
    pub const fn reject_threshold(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "random seed internal VN corrector check threshold"]
    #[inline(always)]
    pub fn set_reject_threshold(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
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
            .field("auto_clock_enable", &self.auto_clock_enable())
            .field("use_ext_seed", &self.use_ext_seed())
            .field("reject_threshold", &self.reject_threshold())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cfg {{ auto_clock_enable: {=bool:?}, use_ext_seed: {=bool:?}, reject_threshold: {=u8:?} }}" , self . auto_clock_enable () , self . use_ext_seed () , self . reject_threshold ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "write 1 to trigger the random seed generation engine"]
    #[inline(always)]
    pub const fn gen_seed_start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to trigger the random seed generation engine"]
    #[inline(always)]
    pub fn set_gen_seed_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "write 1 to trigger the random number generation engine"]
    #[inline(always)]
    pub const fn gen_rand_num_start(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "write 1 to trigger the random number generation engine"]
    #[inline(always)]
    pub fn set_gen_rand_num_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set 1 to stop random seed generation. This will reset the random seed generation engine. After release the stop bit, user should write 1 to gen_seed_start to trigger the random seed engine."]
    #[inline(always)]
    pub const fn gen_seed_stop(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to stop random seed generation. This will reset the random seed generation engine. After release the stop bit, user should write 1 to gen_seed_start to trigger the random seed engine."]
    #[inline(always)]
    pub fn set_gen_seed_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set 1 to stop random number generation and update. This will reset the random number generation engine. After release the stop bit, user should write 1 to gen_rand_num_start to trigger the random number engine."]
    #[inline(always)]
    pub const fn gen_rand_num_stop(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to stop random number generation and update. This will reset the random number generation engine. After release the stop bit, user should write 1 to gen_rand_num_start to trigger the random number engine."]
    #[inline(always)]
    pub fn set_gen_rand_num_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set 1 to suspend random number generation and update. Set 0 to recover the process."]
    #[inline(always)]
    pub const fn gen_rand_num_suspend(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to suspend random number generation and update. Set 0 to recover the process."]
    #[inline(always)]
    pub fn set_gen_rand_num_suspend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("gen_seed_start", &self.gen_seed_start())
            .field("gen_rand_num_start", &self.gen_rand_num_start())
            .field("gen_seed_stop", &self.gen_seed_stop())
            .field("gen_rand_num_stop", &self.gen_rand_num_stop())
            .field("gen_rand_num_suspend", &self.gen_rand_num_suspend())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ctrl {{ gen_seed_start: {=bool:?}, gen_rand_num_start: {=bool:?}, gen_seed_stop: {=bool:?}, gen_rand_num_stop: {=bool:?}, gen_rand_num_suspend: {=bool:?} }}" , self . gen_seed_start () , self . gen_rand_num_start () , self . gen_seed_stop () , self . gen_rand_num_stop () , self . gen_rand_num_suspend ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irq(pub u32);
impl Irq {
    #[doc = "random seed generation done raw interrupt"]
    #[inline(always)]
    pub const fn seed_gen_done(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "random seed generation done raw interrupt"]
    #[inline(always)]
    pub fn set_seed_gen_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "random number available raw interrupt"]
    #[inline(always)]
    pub const fn rand_num_avail(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "random number available raw interrupt"]
    #[inline(always)]
    pub fn set_rand_num_avail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "prng lockup raw interrupt"]
    #[inline(always)]
    pub const fn prng_lockup(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "prng lockup raw interrupt"]
    #[inline(always)]
    pub fn set_prng_lockup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "random seed generation done interrupt mask"]
    #[inline(always)]
    pub const fn seed_gen_done_msk(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "random seed generation done interrupt mask"]
    #[inline(always)]
    pub fn set_seed_gen_done_msk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "random number available interrupt mask"]
    #[inline(always)]
    pub const fn rand_num_avail_msk(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "random number available interrupt mask"]
    #[inline(always)]
    pub fn set_rand_num_avail_msk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "prng lockup interrupt mask"]
    #[inline(always)]
    pub const fn prng_lockup_msk(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "prng lockup interrupt mask"]
    #[inline(always)]
    pub fn set_prng_lockup_msk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
            .field("seed_gen_done", &self.seed_gen_done())
            .field("rand_num_avail", &self.rand_num_avail())
            .field("prng_lockup", &self.prng_lockup())
            .field("seed_gen_done_msk", &self.seed_gen_done_msk())
            .field("rand_num_avail_msk", &self.rand_num_avail_msk())
            .field("prng_lockup_msk", &self.prng_lockup_msk())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irq {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Irq {{ seed_gen_done: {=bool:?}, rand_num_avail: {=bool:?}, prng_lockup: {=bool:?}, seed_gen_done_msk: {=bool:?}, rand_num_avail_msk: {=bool:?}, prng_lockup_msk: {=bool:?} }}" , self . seed_gen_done () , self . rand_num_avail () , self . prng_lockup () , self . seed_gen_done_msk () , self . rand_num_avail_msk () , self . prng_lockup_msk ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RandNum0(pub u32);
impl RandNum0 {
    #[doc = "random number value0"]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "random number value0"]
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RandNum0 {
    #[inline(always)]
    fn default() -> RandNum0 {
        RandNum0(0)
    }
}
impl core::fmt::Debug for RandNum0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RandNum0")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RandNum0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RandNum0 {{ val: {=u32:?} }}", self.val())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RandNum1(pub u32);
impl RandNum1 {
    #[doc = "random number value1"]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "random number value1"]
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RandNum1 {
    #[inline(always)]
    fn default() -> RandNum1 {
        RandNum1(0)
    }
}
impl core::fmt::Debug for RandNum1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RandNum1")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RandNum1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RandNum1 {{ val: {=u32:?} }}", self.val())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RandNum2(pub u32);
impl RandNum2 {
    #[doc = "random number value2"]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "random number value2"]
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RandNum2 {
    #[inline(always)]
    fn default() -> RandNum2 {
        RandNum2(0)
    }
}
impl core::fmt::Debug for RandNum2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RandNum2")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RandNum2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RandNum2 {{ val: {=u32:?} }}", self.val())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RandNum3(pub u32);
impl RandNum3 {
    #[doc = "random number value3"]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "random number value3"]
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RandNum3 {
    #[inline(always)]
    fn default() -> RandNum3 {
        RandNum3(0)
    }
}
impl core::fmt::Debug for RandNum3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RandNum3")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RandNum3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RandNum3 {{ val: {=u32:?} }}", self.val())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RandNum4(pub u32);
impl RandNum4 {
    #[doc = "random number value4"]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "random number value4"]
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RandNum4 {
    #[inline(always)]
    fn default() -> RandNum4 {
        RandNum4(0)
    }
}
impl core::fmt::Debug for RandNum4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RandNum4")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RandNum4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RandNum4 {{ val: {=u32:?} }}", self.val())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RandNum5(pub u32);
impl RandNum5 {
    #[doc = "random number value5"]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "random number value5"]
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RandNum5 {
    #[inline(always)]
    fn default() -> RandNum5 {
        RandNum5(0)
    }
}
impl core::fmt::Debug for RandNum5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RandNum5")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RandNum5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RandNum5 {{ val: {=u32:?} }}", self.val())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RandNum6(pub u32);
impl RandNum6 {
    #[doc = "random number value6"]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "random number value6"]
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RandNum6 {
    #[inline(always)]
    fn default() -> RandNum6 {
        RandNum6(0)
    }
}
impl core::fmt::Debug for RandNum6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RandNum6")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RandNum6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RandNum6 {{ val: {=u32:?} }}", self.val())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RandNum7(pub u32);
impl RandNum7 {
    #[doc = "random number value7"]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "random number value7"]
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RandNum7 {
    #[inline(always)]
    fn default() -> RandNum7 {
        RandNum7(0)
    }
}
impl core::fmt::Debug for RandNum7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RandNum7")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RandNum7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RandNum7 {{ val: {=u32:?} }}", self.val())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RandSeed0(pub u32);
impl RandSeed0 {
    #[doc = "random seed value0. If using external random seed, write value to this register will update the random seed in use."]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "random seed value0. If using external random seed, write value to this register will update the random seed in use."]
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RandSeed0 {
    #[inline(always)]
    fn default() -> RandSeed0 {
        RandSeed0(0)
    }
}
impl core::fmt::Debug for RandSeed0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RandSeed0")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RandSeed0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RandSeed0 {{ val: {=u32:?} }}", self.val())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RandSeed1(pub u32);
impl RandSeed1 {
    #[doc = "random seed value1. If using external random seed, write value to this register will update the random seed in use."]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "random seed value1. If using external random seed, write value to this register will update the random seed in use."]
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RandSeed1 {
    #[inline(always)]
    fn default() -> RandSeed1 {
        RandSeed1(0)
    }
}
impl core::fmt::Debug for RandSeed1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RandSeed1")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RandSeed1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RandSeed1 {{ val: {=u32:?} }}", self.val())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RandSeed2(pub u32);
impl RandSeed2 {
    #[doc = "random seed value2. If using external random seed, write value to this register will update the random seed in use."]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "random seed value2. If using external random seed, write value to this register will update the random seed in use."]
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RandSeed2 {
    #[inline(always)]
    fn default() -> RandSeed2 {
        RandSeed2(0)
    }
}
impl core::fmt::Debug for RandSeed2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RandSeed2")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RandSeed2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RandSeed2 {{ val: {=u32:?} }}", self.val())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RandSeed3(pub u32);
impl RandSeed3 {
    #[doc = "random seed value3. If using external random seed, write value to this register will update the random seed in use."]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "random seed value3. If using external random seed, write value to this register will update the random seed in use."]
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RandSeed3 {
    #[inline(always)]
    fn default() -> RandSeed3 {
        RandSeed3(0)
    }
}
impl core::fmt::Debug for RandSeed3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RandSeed3")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RandSeed3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RandSeed3 {{ val: {=u32:?} }}", self.val())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RandSeed4(pub u32);
impl RandSeed4 {
    #[doc = "random seed value4. If using external random seed, write value to this register will update the random seed in use."]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "random seed value4. If using external random seed, write value to this register will update the random seed in use."]
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RandSeed4 {
    #[inline(always)]
    fn default() -> RandSeed4 {
        RandSeed4(0)
    }
}
impl core::fmt::Debug for RandSeed4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RandSeed4")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RandSeed4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RandSeed4 {{ val: {=u32:?} }}", self.val())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RandSeed5(pub u32);
impl RandSeed5 {
    #[doc = "random seed value5. If using external random seed, write value to this register will update the random seed in use."]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "random seed value5. If using external random seed, write value to this register will update the random seed in use."]
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RandSeed5 {
    #[inline(always)]
    fn default() -> RandSeed5 {
        RandSeed5(0)
    }
}
impl core::fmt::Debug for RandSeed5 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RandSeed5")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RandSeed5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RandSeed5 {{ val: {=u32:?} }}", self.val())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RandSeed6(pub u32);
impl RandSeed6 {
    #[doc = "random seed value6. If using external random seed, write value to this register will update the random seed in use."]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "random seed value6. If using external random seed, write value to this register will update the random seed in use."]
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RandSeed6 {
    #[inline(always)]
    fn default() -> RandSeed6 {
        RandSeed6(0)
    }
}
impl core::fmt::Debug for RandSeed6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RandSeed6")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RandSeed6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RandSeed6 {{ val: {=u32:?} }}", self.val())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RandSeed7(pub u32);
impl RandSeed7 {
    #[doc = "random seed value7. If using external random seed, write value to this register will update the random seed in use."]
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "random seed value7. If using external random seed, write value to this register will update the random seed in use."]
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RandSeed7 {
    #[inline(always)]
    fn default() -> RandSeed7 {
        RandSeed7(0)
    }
}
impl core::fmt::Debug for RandSeed7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RandSeed7")
            .field("val", &self.val())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RandSeed7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RandSeed7 {{ val: {=u32:?} }}", self.val())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "random seed engine busy flag"]
    #[inline(always)]
    pub const fn seed_gen_busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "random seed engine busy flag"]
    #[inline(always)]
    pub fn set_seed_gen_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "random seed valid flag"]
    #[inline(always)]
    pub const fn seed_valid(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "random seed valid flag"]
    #[inline(always)]
    pub fn set_seed_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "random number engine busy flag"]
    #[inline(always)]
    pub const fn rand_num_gen_busy(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "random number engine busy flag"]
    #[inline(always)]
    pub fn set_rand_num_gen_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "random number valid flag"]
    #[inline(always)]
    pub const fn rand_num_valid(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "random number valid flag"]
    #[inline(always)]
    pub fn set_rand_num_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
impl core::fmt::Debug for Stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat")
            .field("seed_gen_busy", &self.seed_gen_busy())
            .field("seed_valid", &self.seed_valid())
            .field("rand_num_gen_busy", &self.rand_num_gen_busy())
            .field("rand_num_valid", &self.rand_num_valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Stat {{ seed_gen_busy: {=bool:?}, seed_valid: {=bool:?}, rand_num_gen_busy: {=bool:?}, rand_num_valid: {=bool:?} }}" , self . seed_gen_busy () , self . seed_valid () , self . rand_num_gen_busy () , self . rand_num_valid ())
    }
}
