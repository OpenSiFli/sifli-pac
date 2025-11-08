#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg0(pub u32);
impl Cfg0 {
    #[doc = "1:Enable pdm module; 0: Disable pdm module"]
    #[must_use]
    #[inline(always)]
    pub const fn pdmcoreen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1:Enable pdm module; 0: Disable pdm module"]
    #[inline(always)]
    pub const fn set_pdmcoreen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:Clk select dll 3.072MHz; 0: Clk selct xtal 9.6MHz"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:Clk select dll 3.072MHz; 0: Clk selct xtal 9.6MHz"]
    #[inline(always)]
    pub const fn set_clk_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Clock frequency division ratio of 3.072MHz or 9.6MHz according to register clk_sel"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_div(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "Clock frequency division ratio of 3.072MHz or 9.6MHz according to register clk_sel"]
    #[inline(always)]
    pub const fn set_clk_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[doc = "1: Enable left channel pdm data sampling; 0: Disable left channel pdm data sampling"]
    #[must_use]
    #[inline(always)]
    pub const fn left_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable left channel pdm data sampling; 0: Disable left channel pdm data sampling"]
    #[inline(always)]
    pub const fn set_left_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "1: Enable right channel pdm data sampling; 0: Disable right channel pdm data sampling"]
    #[must_use]
    #[inline(always)]
    pub const fn right_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable right channel pdm data sampling; 0: Disable right channel pdm data sampling"]
    #[inline(always)]
    pub const fn set_right_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "1:Enable double channels pdm data sampling; 0: Disable double channels pdm data sampling"]
    #[must_use]
    #[inline(always)]
    pub const fn stereo_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1:Enable double channels pdm data sampling; 0: Disable double channels pdm data sampling"]
    #[inline(always)]
    pub const fn set_stereo_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "1: Swap right channel and left channel pdm data; 0: Not swap right channel and left channel pdm data"]
    #[must_use]
    #[inline(always)]
    pub const fn swap_en(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "1: Swap right channel and left channel pdm data; 0: Not swap right channel and left channel pdm data"]
    #[inline(always)]
    pub const fn set_swap_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for Cfg0 {
    #[inline(always)]
    fn default() -> Cfg0 {
        Cfg0(0)
    }
}
impl core::fmt::Debug for Cfg0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg0")
            .field("pdmcoreen", &self.pdmcoreen())
            .field("clk_sel", &self.clk_sel())
            .field("clk_div", &self.clk_div())
            .field("left_en", &self.left_en())
            .field("right_en", &self.right_en())
            .field("stereo_en", &self.stereo_en())
            .field("swap_en", &self.swap_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg0 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cfg0 {{ pdmcoreen: {=bool:?}, clk_sel: {=bool:?}, clk_div: {=u8:?}, left_en: {=bool:?}, right_en: {=bool:?}, stereo_en: {=bool:?}, swap_en: {=bool:?} }}" , self . pdmcoreen () , self . clk_sel () , self . clk_div () , self . left_en () , self . right_en () , self . stereo_en () , self . swap_en ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg1(pub u32);
impl Cfg1 {
    #[doc = "The number of delay dff before the left data stream in processing"]
    #[must_use]
    #[inline(always)]
    pub const fn sample_dly_l(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "The number of delay dff before the left data stream in processing"]
    #[inline(always)]
    pub const fn set_sample_dly_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "The number of delay dff before the right data stream in processing"]
    #[must_use]
    #[inline(always)]
    pub const fn sample_dly_r(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "The number of delay dff before the right data stream in processing"]
    #[inline(always)]
    pub const fn set_sample_dly_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
}
impl Default for Cfg1 {
    #[inline(always)]
    fn default() -> Cfg1 {
        Cfg1(0)
    }
}
impl core::fmt::Debug for Cfg1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfg1")
            .field("sample_dly_l", &self.sample_dly_l())
            .field("sample_dly_r", &self.sample_dly_r())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfg1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cfg1 {{ sample_dly_l: {=u8:?}, sample_dly_r: {=u8:?} }}",
            self.sample_dly_l(),
            self.sample_dly_r()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FifoCfg(pub u32);
impl FifoCfg {
    #[doc = "1: combine left channel and right channel; 0: not combine left channel and right channel"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_con(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1: combine left channel and right channel; 0: not combine left channel and right channel"]
    #[inline(always)]
    pub const fn set_byte_con(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1: 16bits output ; 0: 24bits output ;2: 8bits output ; 3: 32bits output"]
    #[must_use]
    #[inline(always)]
    pub const fn byte_trunc(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "1: 16bits output ; 0: 24bits output ;2: 8bits output ; 3: 32bits output"]
    #[inline(always)]
    pub const fn set_byte_trunc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "the number of data left shift for higher data accuracy"]
    #[must_use]
    #[inline(always)]
    pub const fn pdm_shift(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "the number of data left shift for higher data accuracy"]
    #[inline(always)]
    pub const fn set_pdm_shift(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "1:disable right channel dma request; 0: enable right channel dma request"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_dma_msk_r(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "1:disable right channel dma request; 0: enable right channel dma request"]
    #[inline(always)]
    pub const fn set_rx_dma_msk_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "1:disable left channel dma request; 0: enable left channel dma request"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_dma_msk_l(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "1:disable left channel dma request; 0: enable left channel dma request"]
    #[inline(always)]
    pub const fn set_rx_dma_msk_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "1:exchange storage location of left and right channel; 0: don't exchange storage location of left and right channel"]
    #[must_use]
    #[inline(always)]
    pub const fn lr_chg(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1:exchange storage location of left and right channel; 0: don't exchange storage location of left and right channel"]
    #[inline(always)]
    pub const fn set_lr_chg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for FifoCfg {
    #[inline(always)]
    fn default() -> FifoCfg {
        FifoCfg(0)
    }
}
impl core::fmt::Debug for FifoCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FifoCfg")
            .field("byte_con", &self.byte_con())
            .field("byte_trunc", &self.byte_trunc())
            .field("pdm_shift", &self.pdm_shift())
            .field("rx_dma_msk_r", &self.rx_dma_msk_r())
            .field("rx_dma_msk_l", &self.rx_dma_msk_l())
            .field("lr_chg", &self.lr_chg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FifoCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "FifoCfg {{ byte_con: {=bool:?}, byte_trunc: {=u8:?}, pdm_shift: {=u8:?}, rx_dma_msk_r: {=bool:?}, rx_dma_msk_l: {=bool:?}, lr_chg: {=bool:?} }}" , self . byte_con () , self . byte_trunc () , self . pdm_shift () , self . rx_dma_msk_r () , self . rx_dma_msk_l () , self . lr_chg ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FifoSt(pub u32);
impl FifoSt {
    #[doc = "1 indicates right channel fifo is less than two datas left"]
    #[must_use]
    #[inline(always)]
    pub const fn almost_empty_r(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1 indicates right channel fifo is less than two datas left"]
    #[inline(always)]
    pub const fn set_almost_empty_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1 indicates right channel fifo is less than two full"]
    #[must_use]
    #[inline(always)]
    pub const fn almost_full_r(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1 indicates right channel fifo is less than two full"]
    #[inline(always)]
    pub const fn set_almost_full_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "1 indicates right channel fifo is empty"]
    #[must_use]
    #[inline(always)]
    pub const fn empty_r(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "1 indicates right channel fifo is empty"]
    #[inline(always)]
    pub const fn set_empty_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "1 indicates right channel fifo is full"]
    #[must_use]
    #[inline(always)]
    pub const fn full_r(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1 indicates right channel fifo is full"]
    #[inline(always)]
    pub const fn set_full_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "1 indicates left channel fifo is less than two datas left"]
    #[must_use]
    #[inline(always)]
    pub const fn almost_empty_l(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "1 indicates left channel fifo is less than two datas left"]
    #[inline(always)]
    pub const fn set_almost_empty_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "1 indicates left channel fifo is less than two full"]
    #[must_use]
    #[inline(always)]
    pub const fn almost_full_l(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "1 indicates left channel fifo is less than two full"]
    #[inline(always)]
    pub const fn set_almost_full_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "1 indicates left channel fifo is empty"]
    #[must_use]
    #[inline(always)]
    pub const fn empty_l(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "1 indicates left channel fifo is empty"]
    #[inline(always)]
    pub const fn set_empty_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "1 indicates left channel fifo is full"]
    #[must_use]
    #[inline(always)]
    pub const fn full_l(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "1 indicates left channel fifo is full"]
    #[inline(always)]
    pub const fn set_full_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for FifoSt {
    #[inline(always)]
    fn default() -> FifoSt {
        FifoSt(0)
    }
}
impl core::fmt::Debug for FifoSt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FifoSt")
            .field("almost_empty_r", &self.almost_empty_r())
            .field("almost_full_r", &self.almost_full_r())
            .field("empty_r", &self.empty_r())
            .field("full_r", &self.full_r())
            .field("almost_empty_l", &self.almost_empty_l())
            .field("almost_full_l", &self.almost_full_l())
            .field("empty_l", &self.empty_l())
            .field("full_l", &self.full_l())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FifoSt {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "FifoSt {{ almost_empty_r: {=bool:?}, almost_full_r: {=bool:?}, empty_r: {=bool:?}, full_r: {=bool:?}, almost_empty_l: {=bool:?}, almost_full_l: {=bool:?}, empty_l: {=bool:?}, full_l: {=bool:?} }}" , self . almost_empty_r () , self . almost_full_r () , self . empty_r () , self . full_r () , self . almost_empty_l () , self . almost_full_l () , self . empty_l () , self . full_l ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HpfCfg(pub u32);
impl HpfCfg {
    #[doc = "coefficient of high-pass filter"]
    #[must_use]
    #[inline(always)]
    pub const fn hpf_coeff(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "coefficient of high-pass filter"]
    #[inline(always)]
    pub const fn set_hpf_coeff(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "1:bypass-high pass filter ; 0: enable high-pass filter"]
    #[must_use]
    #[inline(always)]
    pub const fn hpf_bypass(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "1:bypass-high pass filter ; 0: enable high-pass filter"]
    #[inline(always)]
    pub const fn set_hpf_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "1:high-pass filter normal operation ; 0:reset high-pass filter"]
    #[must_use]
    #[inline(always)]
    pub const fn hpf_rst(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "1:high-pass filter normal operation ; 0:reset high-pass filter"]
    #[inline(always)]
    pub const fn set_hpf_rst(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for HpfCfg {
    #[inline(always)]
    fn default() -> HpfCfg {
        HpfCfg(0)
    }
}
impl core::fmt::Debug for HpfCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HpfCfg")
            .field("hpf_coeff", &self.hpf_coeff())
            .field("hpf_bypass", &self.hpf_bypass())
            .field("hpf_rst", &self.hpf_rst())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HpfCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "HpfCfg {{ hpf_coeff: {=u8:?}, hpf_bypass: {=bool:?}, hpf_rst: {=bool:?} }}",
            self.hpf_coeff(),
            self.hpf_bypass(),
            self.hpf_rst()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntClr(pub u32);
impl IntClr {
    #[doc = "clear right channel irq"]
    #[must_use]
    #[inline(always)]
    pub const fn int_clr_r(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "clear right channel irq"]
    #[inline(always)]
    pub const fn set_int_clr_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "clear left channel irq"]
    #[must_use]
    #[inline(always)]
    pub const fn int_clr_l(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "clear left channel irq"]
    #[inline(always)]
    pub const fn set_int_clr_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IntClr {
    #[inline(always)]
    fn default() -> IntClr {
        IntClr(0)
    }
}
impl core::fmt::Debug for IntClr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntClr")
            .field("int_clr_r", &self.int_clr_r())
            .field("int_clr_l", &self.int_clr_l())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntClr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntClr {{ int_clr_r: {=bool:?}, int_clr_l: {=bool:?} }}",
            self.int_clr_r(),
            self.int_clr_l()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntMsk(pub u32);
impl IntMsk {
    #[doc = "1:disable right channel irq to system; 0: enable right channel irq to system"]
    #[must_use]
    #[inline(always)]
    pub const fn int_mask_r(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1:disable right channel irq to system; 0: enable right channel irq to system"]
    #[inline(always)]
    pub const fn set_int_mask_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1:disable left channel irq to system; 0: enable left channel irq to system"]
    #[must_use]
    #[inline(always)]
    pub const fn int_mask_l(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1:disable left channel irq to system; 0: enable left channel irq to system"]
    #[inline(always)]
    pub const fn set_int_mask_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IntMsk {
    #[inline(always)]
    fn default() -> IntMsk {
        IntMsk(0)
    }
}
impl core::fmt::Debug for IntMsk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntMsk")
            .field("int_mask_r", &self.int_mask_r())
            .field("int_mask_l", &self.int_mask_l())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntMsk {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntMsk {{ int_mask_r: {=bool:?}, int_mask_l: {=bool:?} }}",
            self.int_mask_r(),
            self.int_mask_l()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntSt(pub u32);
impl IntSt {
    #[doc = "1 indicates right channel fifo has already overflowed and as irq at same time"]
    #[must_use]
    #[inline(always)]
    pub const fn overflow_r(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1 indicates right channel fifo has already overflowed and as irq at same time"]
    #[inline(always)]
    pub const fn set_overflow_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1 indicates left channel fifo has already overflowed and as irq at same time"]
    #[must_use]
    #[inline(always)]
    pub const fn overflow_l(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1 indicates left channel fifo has already overflowed and as irq at same time"]
    #[inline(always)]
    pub const fn set_overflow_l(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IntSt {
    #[inline(always)]
    fn default() -> IntSt {
        IntSt(0)
    }
}
impl core::fmt::Debug for IntSt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntSt")
            .field("overflow_r", &self.overflow_r())
            .field("overflow_l", &self.overflow_l())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntSt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntSt {{ overflow_r: {=bool:?}, overflow_l: {=bool:?} }}",
            self.overflow_r(),
            self.overflow_l()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LpfCfg6(pub u32);
impl LpfCfg6 {
    #[doc = "1:downsampling rate of low pass filter is two;0:No downsampling of low pass filter"]
    #[must_use]
    #[inline(always)]
    pub const fn lpf_ds(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "1:downsampling rate of low pass filter is two;0:No downsampling of low pass filter"]
    #[inline(always)]
    pub const fn set_lpf_ds(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "1:bypass low-pass filter ; 0: enable low-pass filter"]
    #[must_use]
    #[inline(always)]
    pub const fn lpf_bypass(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "1:bypass low-pass filter ; 0: enable low-pass filter"]
    #[inline(always)]
    pub const fn set_lpf_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for LpfCfg6 {
    #[inline(always)]
    fn default() -> LpfCfg6 {
        LpfCfg6(0)
    }
}
impl core::fmt::Debug for LpfCfg6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LpfCfg6")
            .field("lpf_ds", &self.lpf_ds())
            .field("lpf_bypass", &self.lpf_bypass())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LpfCfg6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "LpfCfg6 {{ lpf_ds: {=bool:?}, lpf_bypass: {=bool:?} }}",
            self.lpf_ds(),
            self.lpf_bypass()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PgaCfg(pub u32);
impl PgaCfg {
    #[doc = "left channel gain control , the range is -15dB~45dB. Resolution is 0.5dB/LSB"]
    #[must_use]
    #[inline(always)]
    pub const fn pga_gain_l(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "left channel gain control , the range is -15dB~45dB. Resolution is 0.5dB/LSB"]
    #[inline(always)]
    pub const fn set_pga_gain_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "right channel gain control , the range is -15dB~45dB. Resolution is 0.5dB/LSB"]
    #[must_use]
    #[inline(always)]
    pub const fn pga_gain_r(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x7f;
        val as u8
    }
    #[doc = "right channel gain control , the range is -15dB~45dB. Resolution is 0.5dB/LSB"]
    #[inline(always)]
    pub const fn set_pga_gain_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 7usize)) | (((val as u32) & 0x7f) << 7usize);
    }
}
impl Default for PgaCfg {
    #[inline(always)]
    fn default() -> PgaCfg {
        PgaCfg(0)
    }
}
impl core::fmt::Debug for PgaCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PgaCfg")
            .field("pga_gain_l", &self.pga_gain_l())
            .field("pga_gain_r", &self.pga_gain_r())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PgaCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "PgaCfg {{ pga_gain_l: {=u8:?}, pga_gain_r: {=u8:?} }}",
            self.pga_gain_l(),
            self.pga_gain_r()
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
pub struct SincCfg(pub u32);
impl SincCfg {
    #[doc = "dowmsampling rate of sinc filter"]
    #[must_use]
    #[inline(always)]
    pub const fn sinc_rate(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "dowmsampling rate of sinc filter"]
    #[inline(always)]
    pub const fn set_sinc_rate(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "1:select four differentiators in sinc filter; 0:select three differentiators in sinc filter"]
    #[must_use]
    #[inline(always)]
    pub const fn sinc_order_sel(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1:select four differentiators in sinc filter; 0:select three differentiators in sinc filter"]
    #[inline(always)]
    pub const fn set_sinc_order_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for SincCfg {
    #[inline(always)]
    fn default() -> SincCfg {
        SincCfg(0)
    }
}
impl core::fmt::Debug for SincCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SincCfg")
            .field("sinc_rate", &self.sinc_rate())
            .field("sinc_order_sel", &self.sinc_order_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SincCfg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "SincCfg {{ sinc_rate: {=u8:?}, sinc_order_sel: {=bool:?} }}",
            self.sinc_rate(),
            self.sinc_order_sel()
        )
    }
}
