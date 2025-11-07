#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AudioI2sSlMerge(pub u32);
impl AudioI2sSlMerge {
    #[doc = "when work as an I2S slave, and external I2S master TX/RX share an only BCLK/LRCK, we need set this bit high. 0: I2S slave use separated timing control port. TX_BCLK_IN/TX_LRCK_IN and RX_BCLK/RX_LRCK_IN are separated. 1: I2S slave use the same BCLK/LRCK, the TX_BCLK_IN/TX_LRCK also is used for RX controller."]
    #[must_use]
    #[inline(always)]
    pub const fn slave_timing_merge(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "when work as an I2S slave, and external I2S master TX/RX share an only BCLK/LRCK, we need set this bit high. 0: I2S slave use separated timing control port. TX_BCLK_IN/TX_LRCK_IN and RX_BCLK/RX_LRCK_IN are separated. 1: I2S slave use the same BCLK/LRCK, the TX_BCLK_IN/TX_LRCK also is used for RX controller."]
    #[inline(always)]
    pub const fn set_slave_timing_merge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for AudioI2sSlMerge {
    #[inline(always)]
    fn default() -> AudioI2sSlMerge {
        AudioI2sSlMerge(0)
    }
}
impl core::fmt::Debug for AudioI2sSlMerge {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AudioI2sSlMerge")
            .field("slave_timing_merge", &self.slave_timing_merge())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AudioI2sSlMerge {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AudioI2sSlMerge {{ slave_timing_merge: {=bool:?} }}",
            self.slave_timing_merge()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AudioRxBclkDiv(pub u32);
impl AudioRxBclkDiv {
    #[doc = "RX serial bit clock duty cycle 5 for 48K FS 4 for 44.1K FS 5 for 32KFS 10 for 24K FS 8 for 22.05K FS 15 for 16K FS 20 for 12K FS 16 for 11.025K FS 30 for 8KFs"]
    #[must_use]
    #[inline(always)]
    pub const fn duty(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "RX serial bit clock duty cycle 5 for 48K FS 4 for 44.1K FS 5 for 32KFS 10 for 24K FS 8 for 22.05K FS 15 for 16K FS 20 for 12K FS 16 for 11.025K FS 30 for 8KFs"]
    #[inline(always)]
    pub const fn set_duty(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for AudioRxBclkDiv {
    #[inline(always)]
    fn default() -> AudioRxBclkDiv {
        AudioRxBclkDiv(0)
    }
}
impl core::fmt::Debug for AudioRxBclkDiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AudioRxBclkDiv")
            .field("duty", &self.duty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AudioRxBclkDiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AudioRxBclkDiv {{ duty: {=u16:?} }}", self.duty())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AudioRxFuncEn(pub u32);
impl AudioRxFuncEn {
    #[doc = "1: enable 0: disable"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1: enable 0: disable"]
    #[inline(always)]
    pub const fn set_rx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1: select external rx interface 0: select internal apb rx interface"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_intf_sel(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1: select external rx interface 0: select internal apb rx interface"]
    #[inline(always)]
    pub const fn set_rx_intf_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for AudioRxFuncEn {
    #[inline(always)]
    fn default() -> AudioRxFuncEn {
        AudioRxFuncEn(0)
    }
}
impl core::fmt::Debug for AudioRxFuncEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AudioRxFuncEn")
            .field("rx_en", &self.rx_en())
            .field("rx_intf_sel", &self.rx_intf_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AudioRxFuncEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AudioRxFuncEn {{ rx_en: {=bool:?}, rx_intf_sel: {=bool:?} }}",
            self.rx_en(),
            self.rx_intf_sel()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AudioRxLrckDiv(pub u32);
impl AudioRxLrckDiv {
    #[doc = "RX LRCK duty cycle low: 125 for 48K FS 136 for 44.1K FS 190 for 32K FS 250 for 24K FS 272 for 22.05K FS 375 for 16K FS 500 for 12K FS 544 for 11.025K FS 750 for 8K FS Note: 1)duty_cycle = 12M/FS"]
    #[must_use]
    #[inline(always)]
    pub const fn duty_low(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "RX LRCK duty cycle low: 125 for 48K FS 136 for 44.1K FS 190 for 32K FS 250 for 24K FS 272 for 22.05K FS 375 for 16K FS 500 for 12K FS 544 for 11.025K FS 750 for 8K FS Note: 1)duty_cycle = 12M/FS"]
    #[inline(always)]
    pub const fn set_duty_low(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "RX LRCK duty cycle high: 125 for 48K FS 136 for 44.1K FS 185 for 32K FS 250 for 24K FS 272 for 22.05K FS 375 for 16K FS 500 for 12K FS 544 for 11.025K FS 750 for 8K FS"]
    #[must_use]
    #[inline(always)]
    pub const fn duty_high(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "RX LRCK duty cycle high: 125 for 48K FS 136 for 44.1K FS 185 for 32K FS 250 for 24K FS 272 for 22.05K FS 375 for 16K FS 500 for 12K FS 544 for 11.025K FS 750 for 8K FS"]
    #[inline(always)]
    pub const fn set_duty_high(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for AudioRxLrckDiv {
    #[inline(always)]
    fn default() -> AudioRxLrckDiv {
        AudioRxLrckDiv(0)
    }
}
impl core::fmt::Debug for AudioRxLrckDiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AudioRxLrckDiv")
            .field("duty_low", &self.duty_low())
            .field("duty_high", &self.duty_high())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AudioRxLrckDiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AudioRxLrckDiv {{ duty_low: {=u16:?}, duty_high: {=u16:?} }}",
            self.duty_low(),
            self.duty_high()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AudioRxPause(pub u32);
impl AudioRxPause {
    #[doc = "RX pause control when rx_enable = 1. 1: pause 0: RX work"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_pause(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RX pause control when rx_enable = 1. 1: pause 0: RX work"]
    #[inline(always)]
    pub const fn set_rx_pause(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for AudioRxPause {
    #[inline(always)]
    fn default() -> AudioRxPause {
        AudioRxPause(0)
    }
}
impl core::fmt::Debug for AudioRxPause {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AudioRxPause")
            .field("rx_pause", &self.rx_pause())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AudioRxPause {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AudioRxPause {{ rx_pause: {=bool:?} }}", self.rx_pause())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AudioRxPcmDw(pub u32);
impl AudioRxPcmDw {
    #[doc = "For I2S and left justified mode, M can be 8,13,14,16 For right justified mode, M can be 8, 13, 14, 16, 18, 20, 22, 24"]
    #[must_use]
    #[inline(always)]
    pub const fn pcm_data_width(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "For I2S and left justified mode, M can be 8,13,14,16 For right justified mode, M can be 8, 13, 14, 16, 18, 20, 22, 24"]
    #[inline(always)]
    pub const fn set_pcm_data_width(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for AudioRxPcmDw {
    #[inline(always)]
    fn default() -> AudioRxPcmDw {
        AudioRxPcmDw(0)
    }
}
impl core::fmt::Debug for AudioRxPcmDw {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AudioRxPcmDw")
            .field("pcm_data_width", &self.pcm_data_width())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AudioRxPcmDw {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AudioRxPcmDw {{ pcm_data_width: {=u8:?} }}",
            self.pcm_data_width()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AudioRxSerialTiming(pub u32);
impl AudioRxSerialTiming {
    #[doc = "00: I2S 01: Left justified 10: right justified 11: reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn timing(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "00: I2S 01: Left justified 10: right justified 11: reserved"]
    #[inline(always)]
    pub const fn set_timing(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "audio code receiver mode select. 0: master mode, 1: slave mode"]
    #[must_use]
    #[inline(always)]
    pub const fn slave_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "audio code receiver mode select. 0: master mode, 1: slave mode"]
    #[inline(always)]
    pub const fn set_slave_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "RX LRCK polarity control. 0: disable RX_LRCK inventor 1: enable RX_LRCK inventor for standard I2S, set tx_lrck_pol to low for Left/Right Justified, set tx_lrck_pol to hgih"]
    #[must_use]
    #[inline(always)]
    pub const fn lrck_pol(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "RX LRCK polarity control. 0: disable RX_LRCK inventor 1: enable RX_LRCK inventor for standard I2S, set tx_lrck_pol to low for Left/Right Justified, set tx_lrck_pol to hgih"]
    #[inline(always)]
    pub const fn set_lrck_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for AudioRxSerialTiming {
    #[inline(always)]
    fn default() -> AudioRxSerialTiming {
        AudioRxSerialTiming(0)
    }
}
impl core::fmt::Debug for AudioRxSerialTiming {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AudioRxSerialTiming")
            .field("timing", &self.timing())
            .field("slave_en", &self.slave_en())
            .field("lrck_pol", &self.lrck_pol())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AudioRxSerialTiming {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AudioRxSerialTiming {{ timing: {=u8:?}, slave_en: {=bool:?}, lrck_pol: {=bool:?} }}",
            self.timing(),
            self.slave_en(),
            self.lrck_pol()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AudioSerialTiming(pub u32);
impl AudioSerialTiming {
    #[doc = "00: I2S mode 01: Left justified 10: right justified 11: reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn timing(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "00: I2S mode 01: Left justified 10: right justified 11: reserved"]
    #[inline(always)]
    pub const fn set_timing(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "audio code transmit mode select. 0: master mode, 1: slave mode"]
    #[must_use]
    #[inline(always)]
    pub const fn slave_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "audio code transmit mode select. 0: master mode, 1: slave mode"]
    #[inline(always)]
    pub const fn set_slave_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "TX LRCK polarity control. 0: disable TX_LRCK inventor 1: enable TX_LRCK inventor for standard I2S, set tx_lrck_pol to low for Left/Right Justified, set tx_lrck_pol to hgih"]
    #[must_use]
    #[inline(always)]
    pub const fn lrck_pol(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "TX LRCK polarity control. 0: disable TX_LRCK inventor 1: enable TX_LRCK inventor for standard I2S, set tx_lrck_pol to low for Left/Right Justified, set tx_lrck_pol to hgih"]
    #[inline(always)]
    pub const fn set_lrck_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for AudioSerialTiming {
    #[inline(always)]
    fn default() -> AudioSerialTiming {
        AudioSerialTiming(0)
    }
}
impl core::fmt::Debug for AudioSerialTiming {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AudioSerialTiming")
            .field("timing", &self.timing())
            .field("slave_en", &self.slave_en())
            .field("lrck_pol", &self.lrck_pol())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AudioSerialTiming {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AudioSerialTiming {{ timing: {=u8:?}, slave_en: {=bool:?}, lrck_pol: {=bool:?} }}",
            self.timing(),
            self.slave_en(),
            self.lrck_pol()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AudioTxBclkDiv(pub u32);
impl AudioTxBclkDiv {
    #[doc = "TX serial bit clock duty cycle 5 for 48K FS 4 for 44.1K FS 5 for 32KFS 10 for 24K FS 8 for 22.05K FS 15 for 16K FS 20 for 12K FS 16 for 11.025K FS 30 for 8KFs"]
    #[must_use]
    #[inline(always)]
    pub const fn duty(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "TX serial bit clock duty cycle 5 for 48K FS 4 for 44.1K FS 5 for 32KFS 10 for 24K FS 8 for 22.05K FS 15 for 16K FS 20 for 12K FS 16 for 11.025K FS 30 for 8KFs"]
    #[inline(always)]
    pub const fn set_duty(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for AudioTxBclkDiv {
    #[inline(always)]
    fn default() -> AudioTxBclkDiv {
        AudioTxBclkDiv(0)
    }
}
impl core::fmt::Debug for AudioTxBclkDiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AudioTxBclkDiv")
            .field("duty", &self.duty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AudioTxBclkDiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AudioTxBclkDiv {{ duty: {=u8:?} }}", self.duty())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AudioTxFormat(pub u32);
impl AudioTxFormat {
    #[doc = "I2S out pcm data width M >= 16, common value: 16, 18, 20, 22, 24"]
    #[must_use]
    #[inline(always)]
    pub const fn pcm_data_width(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "I2S out pcm data width M >= 16, common value: 16, 18, 20, 22, 24"]
    #[inline(always)]
    pub const fn set_pcm_data_width(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for AudioTxFormat {
    #[inline(always)]
    fn default() -> AudioTxFormat {
        AudioTxFormat(0)
    }
}
impl core::fmt::Debug for AudioTxFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AudioTxFormat")
            .field("pcm_data_width", &self.pcm_data_width())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AudioTxFormat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AudioTxFormat {{ pcm_data_width: {=u8:?} }}",
            self.pcm_data_width()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AudioTxFuncEn(pub u32);
impl AudioTxFuncEn {
    #[doc = "1: enable 0:disable"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1: enable 0:disable"]
    #[inline(always)]
    pub const fn set_tx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1: select external tx interface 0: select internal apb tx interface"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_intf_sel(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1: select external tx interface 0: select internal apb tx interface"]
    #[inline(always)]
    pub const fn set_tx_intf_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for AudioTxFuncEn {
    #[inline(always)]
    fn default() -> AudioTxFuncEn {
        AudioTxFuncEn(0)
    }
}
impl core::fmt::Debug for AudioTxFuncEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AudioTxFuncEn")
            .field("tx_en", &self.tx_en())
            .field("tx_intf_sel", &self.tx_intf_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AudioTxFuncEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AudioTxFuncEn {{ tx_en: {=bool:?}, tx_intf_sel: {=bool:?} }}",
            self.tx_en(),
            self.tx_intf_sel()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AudioTxLrckDiv(pub u32);
impl AudioTxLrckDiv {
    #[doc = "TX LRCK duty cycle low: 125 for 48K FS 136 for 44.1K FS 190 for 32K FS 250 for 24K FS 272 for 22.05K FS 375 for 16K FS 500 for 12K FS 544 for 11.025K FS 750 for 8K FS Note: 1)duty_cycle = 12M/FS"]
    #[must_use]
    #[inline(always)]
    pub const fn duty_low(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "TX LRCK duty cycle low: 125 for 48K FS 136 for 44.1K FS 190 for 32K FS 250 for 24K FS 272 for 22.05K FS 375 for 16K FS 500 for 12K FS 544 for 11.025K FS 750 for 8K FS Note: 1)duty_cycle = 12M/FS"]
    #[inline(always)]
    pub const fn set_duty_low(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "TX LRCK duty cycle high: 125 for 48K FS 136 for 44.1K FS 185 for 32K FS 250 for 24K FS 272 for 22.05K FS 375 for 16K FS 500 for 12K FS 544 for 11.025K FS 750 for 8K FS"]
    #[must_use]
    #[inline(always)]
    pub const fn duty_high(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "TX LRCK duty cycle high: 125 for 48K FS 136 for 44.1K FS 185 for 32K FS 250 for 24K FS 272 for 22.05K FS 375 for 16K FS 500 for 12K FS 544 for 11.025K FS 750 for 8K FS"]
    #[inline(always)]
    pub const fn set_duty_high(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for AudioTxLrckDiv {
    #[inline(always)]
    fn default() -> AudioTxLrckDiv {
        AudioTxLrckDiv(0)
    }
}
impl core::fmt::Debug for AudioTxLrckDiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AudioTxLrckDiv")
            .field("duty_low", &self.duty_low())
            .field("duty_high", &self.duty_high())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AudioTxLrckDiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "AudioTxLrckDiv {{ duty_low: {=u16:?}, duty_high: {=u16:?} }}",
            self.duty_low(),
            self.duty_high()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AudioTxPause(pub u32);
impl AudioTxPause {
    #[doc = "TX pause control when tx_enable = 1. 1: pause 0: TX work"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_pause(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TX pause control when tx_enable = 1. 1: pause 0: TX work"]
    #[inline(always)]
    pub const fn set_tx_pause(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for AudioTxPause {
    #[inline(always)]
    fn default() -> AudioTxPause {
        AudioTxPause(0)
    }
}
impl core::fmt::Debug for AudioTxPause {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AudioTxPause")
            .field("tx_pause", &self.tx_pause())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AudioTxPause {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AudioTxPause {{ tx_pause: {=bool:?} }}", self.tx_pause())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BbPcmFormat(pub u32);
impl BbPcmFormat {
    #[doc = "Baseband Master PCM data width (>=8) Common value: 8, 13,14, 16, 18, 20, 22, 24. for I2S/Left Justified/Right Kistified timing, bb_pcm_dw >=16 For PCM timing, only 8, 13, 14, 16 configure value is available."]
    #[must_use]
    #[inline(always)]
    pub const fn pcm_dw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Baseband Master PCM data width (>=8) Common value: 8, 13,14, 16, 18, 20, 22, 24. for I2S/Left Justified/Right Kistified timing, bb_pcm_dw >=16 For PCM timing, only 8, 13, 14, 16 configure value is available."]
    #[inline(always)]
    pub const fn set_pcm_dw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "00: I2S timing, 01: Left Justified 10: Right Justified, 11: PCM timing"]
    #[must_use]
    #[inline(always)]
    pub const fn pcm_tim_sel(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "00: I2S timing, 01: Left Justified 10: Right Justified, 11: PCM timing"]
    #[inline(always)]
    pub const fn set_pcm_tim_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "0: short sync, 1: long sync"]
    #[must_use]
    #[inline(always)]
    pub const fn pcm_sync_flag(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "0: short sync, 1: long sync"]
    #[inline(always)]
    pub const fn set_pcm_sync_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Serial PCM data bit sequence. 0: MSB first, 1: LSB first"]
    #[must_use]
    #[inline(always)]
    pub const fn pcm_lsb_flag(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Serial PCM data bit sequence. 0: MSB first, 1: LSB first"]
    #[inline(always)]
    pub const fn set_pcm_lsb_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "0: no bb_i2s_lrck input inventor 1: enable bb_i2s_lrck input inventor for standard I2S, set tx_lrck_pol to low for Left/Right Justified, set tx_lrck_pol to high"]
    #[must_use]
    #[inline(always)]
    pub const fn i2s_lrck_pol(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "0: no bb_i2s_lrck input inventor 1: enable bb_i2s_lrck input inventor for standard I2S, set tx_lrck_pol to low for Left/Right Justified, set tx_lrck_pol to high"]
    #[inline(always)]
    pub const fn set_i2s_lrck_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "input BB pcm clock polarity: 0: rising edge for data transmitting, falling edge for data receiving 1: rising edge for data receiving, falling edge for data transmitting"]
    #[must_use]
    #[inline(always)]
    pub const fn pcm_clk_pol(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "input BB pcm clock polarity: 0: rising edge for data transmitting, falling edge for data receiving 1: rising edge for data receiving, falling edge for data transmitting"]
    #[inline(always)]
    pub const fn set_pcm_clk_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for BbPcmFormat {
    #[inline(always)]
    fn default() -> BbPcmFormat {
        BbPcmFormat(0)
    }
}
impl core::fmt::Debug for BbPcmFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BbPcmFormat")
            .field("pcm_dw", &self.pcm_dw())
            .field("pcm_tim_sel", &self.pcm_tim_sel())
            .field("pcm_sync_flag", &self.pcm_sync_flag())
            .field("pcm_lsb_flag", &self.pcm_lsb_flag())
            .field("i2s_lrck_pol", &self.i2s_lrck_pol())
            .field("pcm_clk_pol", &self.pcm_clk_pol())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BbPcmFormat {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "BbPcmFormat {{ pcm_dw: {=u8:?}, pcm_tim_sel: {=u8:?}, pcm_sync_flag: {=bool:?}, pcm_lsb_flag: {=bool:?}, i2s_lrck_pol: {=bool:?}, pcm_clk_pol: {=bool:?} }}" , self . pcm_dw () , self . pcm_tim_sel () , self . pcm_sync_flag () , self . pcm_lsb_flag () , self . i2s_lrck_pol () , self . pcm_clk_pol ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BtPcmClkDuty(pub u32);
impl BtPcmClkDuty {
    #[doc = "BT_PCM_CLK duty cycle = (GCLK/(bt_pcm_sync*bt_pcm_dw))"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_duty(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "BT_PCM_CLK duty cycle = (GCLK/(bt_pcm_sync*bt_pcm_dw))"]
    #[inline(always)]
    pub const fn set_clk_duty(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
}
impl Default for BtPcmClkDuty {
    #[inline(always)]
    fn default() -> BtPcmClkDuty {
        BtPcmClkDuty(0)
    }
}
impl core::fmt::Debug for BtPcmClkDuty {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BtPcmClkDuty")
            .field("clk_duty", &self.clk_duty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BtPcmClkDuty {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BtPcmClkDuty {{ clk_duty: {=u16:?} }}", self.clk_duty())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BtPcmDw(pub u32);
impl BtPcmDw {
    #[doc = "BT PCM master data width (>= 8), common value: 8, 13,14, 16"]
    #[must_use]
    #[inline(always)]
    pub const fn dw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "BT PCM master data width (>= 8), common value: 8, 13,14, 16"]
    #[inline(always)]
    pub const fn set_dw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for BtPcmDw {
    #[inline(always)]
    fn default() -> BtPcmDw {
        BtPcmDw(0)
    }
}
impl core::fmt::Debug for BtPcmDw {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BtPcmDw").field("dw", &self.dw()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BtPcmDw {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BtPcmDw {{ dw: {=u8:?} }}", self.dw())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BtPcmSyncDuty(pub u32);
impl BtPcmSyncDuty {
    #[doc = "PCM_SYNC duty cycle (bt_pcm_sync frequency = bt_pclk_clk/bt_pcm_sync_duty)"]
    #[must_use]
    #[inline(always)]
    pub const fn sync_duty(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "PCM_SYNC duty cycle (bt_pcm_sync frequency = bt_pclk_clk/bt_pcm_sync_duty)"]
    #[inline(always)]
    pub const fn set_sync_duty(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for BtPcmSyncDuty {
    #[inline(always)]
    fn default() -> BtPcmSyncDuty {
        BtPcmSyncDuty(0)
    }
}
impl core::fmt::Debug for BtPcmSyncDuty {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BtPcmSyncDuty")
            .field("sync_duty", &self.sync_duty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BtPcmSyncDuty {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BtPcmSyncDuty {{ sync_duty: {=u8:?} }}",
            self.sync_duty()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BtPcmTiming(pub u32);
impl BtPcmTiming {
    #[doc = "Serial PCM data bit sequence. 0: MSB first, 1: LSB first"]
    #[must_use]
    #[inline(always)]
    pub const fn lsb_flag(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Serial PCM data bit sequence. 0: MSB first, 1: LSB first"]
    #[inline(always)]
    pub const fn set_lsb_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "0: short sync, 1: long sync"]
    #[must_use]
    #[inline(always)]
    pub const fn sync_flag(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "0: short sync, 1: long sync"]
    #[inline(always)]
    pub const fn set_sync_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "BT PCM master output pcm clock polarity: 0: rising edge for data transmitting, falling edge for data receiving 1: rising edge for data receiving, falling edge for data transmitting"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_pol(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "BT PCM master output pcm clock polarity: 0: rising edge for data transmitting, falling edge for data receiving 1: rising edge for data receiving, falling edge for data transmitting"]
    #[inline(always)]
    pub const fn set_clk_pol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for BtPcmTiming {
    #[inline(always)]
    fn default() -> BtPcmTiming {
        BtPcmTiming(0)
    }
}
impl core::fmt::Debug for BtPcmTiming {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BtPcmTiming")
            .field("lsb_flag", &self.lsb_flag())
            .field("sync_flag", &self.sync_flag())
            .field("clk_pol", &self.clk_pol())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BtPcmTiming {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BtPcmTiming {{ lsb_flag: {=bool:?}, sync_flag: {=bool:?}, clk_pol: {=bool:?} }}",
            self.lsb_flag(),
            self.sync_flag(),
            self.clk_pol()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BtPhoneCtrl(pub u32);
impl BtPhoneCtrl {
    #[doc = "BT phone enable 0: disable, 1: enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bt_ph_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "BT phone enable 0: disable, 1: enable"]
    #[inline(always)]
    pub const fn set_bt_ph_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "background mixer enable 0: disable, 1: enable"]
    #[must_use]
    #[inline(always)]
    pub const fn bt_back_mix_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "background mixer enable 0: disable, 1: enable"]
    #[inline(always)]
    pub const fn set_bt_back_mix_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "0: disable the smooth filter for background mixer 1: enable the smooth filer for background mixer"]
    #[must_use]
    #[inline(always)]
    pub const fn bt_mix_smooth_filter_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "0: disable the smooth filter for background mixer 1: enable the smooth filer for background mixer"]
    #[inline(always)]
    pub const fn set_bt_mix_smooth_filter_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "BT path select 0: digital path, 1: analog path"]
    #[must_use]
    #[inline(always)]
    pub const fn bt_path_sel(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "BT path select 0: digital path, 1: analog path"]
    #[inline(always)]
    pub const fn set_bt_path_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "bypass baseband PCM signals to BT VCI master: 0: no bypass, 1: bypass"]
    #[must_use]
    #[inline(always)]
    pub const fn bt_pcm_if_bps(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "bypass baseband PCM signals to BT VCI master: 0: no bypass, 1: bypass"]
    #[inline(always)]
    pub const fn set_bt_pcm_if_bps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "bypass baseband I2S interface to audio codec i2s interface 0: no bypass, 1: bypass"]
    #[must_use]
    #[inline(always)]
    pub const fn bb_i2s_bps_to_cdc(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "bypass baseband I2S interface to audio codec i2s interface 0: no bypass, 1: bypass"]
    #[inline(always)]
    pub const fn set_bb_i2s_bps_to_cdc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for BtPhoneCtrl {
    #[inline(always)]
    fn default() -> BtPhoneCtrl {
        BtPhoneCtrl(0)
    }
}
impl core::fmt::Debug for BtPhoneCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BtPhoneCtrl")
            .field("bt_ph_en", &self.bt_ph_en())
            .field("bt_back_mix_en", &self.bt_back_mix_en())
            .field("bt_mix_smooth_filter_en", &self.bt_mix_smooth_filter_en())
            .field("bt_path_sel", &self.bt_path_sel())
            .field("bt_pcm_if_bps", &self.bt_pcm_if_bps())
            .field("bb_i2s_bps_to_cdc", &self.bb_i2s_bps_to_cdc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BtPhoneCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "BtPhoneCtrl {{ bt_ph_en: {=bool:?}, bt_back_mix_en: {=bool:?}, bt_mix_smooth_filter_en: {=bool:?}, bt_path_sel: {=bool:?}, bt_pcm_if_bps: {=bool:?}, bb_i2s_bps_to_cdc: {=bool:?} }}" , self . bt_ph_en () , self . bt_back_mix_en () , self . bt_mix_smooth_filter_en () , self . bt_path_sel () , self . bt_pcm_if_bps () , self . bb_i2s_bps_to_cdc ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BtVolCtrl(pub u32);
impl BtVolCtrl {
    #[doc = "BT master volume"]
    #[must_use]
    #[inline(always)]
    pub const fn vol(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "BT master volume"]
    #[inline(always)]
    pub const fn set_vol(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "BT volume adjust enable"]
    #[must_use]
    #[inline(always)]
    pub const fn vol_adj_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "BT volume adjust enable"]
    #[inline(always)]
    pub const fn set_vol_adj_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for BtVolCtrl {
    #[inline(always)]
    fn default() -> BtVolCtrl {
        BtVolCtrl(0)
    }
}
impl core::fmt::Debug for BtVolCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BtVolCtrl")
            .field("vol", &self.vol())
            .field("vol_adj_en", &self.vol_adj_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BtVolCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "BtVolCtrl {{ vol: {=u8:?}, vol_adj_en: {=bool:?} }}",
            self.vol(),
            self.vol_adj_en()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugLoop(pub u32);
impl DebugLoop {
    #[doc = "TX-->RX Loop debug control: 0: disable 1: enable, internally connect TX SDTO to RX SDTI"]
    #[must_use]
    #[inline(always)]
    pub const fn da2ad_loop_back(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TX-->RX Loop debug control: 0: disable 1: enable, internally connect TX SDTO to RX SDTI"]
    #[inline(always)]
    pub const fn set_da2ad_loop_back(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX-->TX Loop debug control: 0: disable 1: enable, internally connect RX Resampled PCM to TX Resample PCM input"]
    #[must_use]
    #[inline(always)]
    pub const fn ad2da_loop_back(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX-->TX Loop debug control: 0: disable 1: enable, internally connect RX Resampled PCM to TX Resample PCM input"]
    #[inline(always)]
    pub const fn set_ad2da_loop_back(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "clock select 0: xtal clock 1: pll clock"]
    #[must_use]
    #[inline(always)]
    pub const fn sp_clk_sel(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "clock select 0: xtal clock 1: pll clock"]
    #[inline(always)]
    pub const fn set_sp_clk_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "update sp clock divider"]
    #[must_use]
    #[inline(always)]
    pub const fn sp_clk_div_update(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "update sp clock divider"]
    #[inline(always)]
    pub const fn set_sp_clk_div_update(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "sp clock divider value"]
    #[must_use]
    #[inline(always)]
    pub const fn sp_clk_div(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "sp clock divider value"]
    #[inline(always)]
    pub const fn set_sp_clk_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for DebugLoop {
    #[inline(always)]
    fn default() -> DebugLoop {
        DebugLoop(0)
    }
}
impl core::fmt::Debug for DebugLoop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugLoop")
            .field("da2ad_loop_back", &self.da2ad_loop_back())
            .field("ad2da_loop_back", &self.ad2da_loop_back())
            .field("sp_clk_sel", &self.sp_clk_sel())
            .field("sp_clk_div_update", &self.sp_clk_div_update())
            .field("sp_clk_div", &self.sp_clk_div())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugLoop {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "DebugLoop {{ da2ad_loop_back: {=bool:?}, ad2da_loop_back: {=bool:?}, sp_clk_sel: {=bool:?}, sp_clk_div_update: {=bool:?}, sp_clk_div: {=u8:?} }}" , self . da2ad_loop_back () , self . ad2da_loop_back () , self . sp_clk_sel () , self . sp_clk_div_update () , self . sp_clk_div ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DmaMask(pub u32);
impl DmaMask {
    #[doc = "RX DMA mask enable:1: mask0: do not mask"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_dma_mask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RX DMA mask enable:1: mask0: do not mask"]
    #[inline(always)]
    pub const fn set_rx_dma_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TX DMA mask enable:1: mask0: do not mask"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_dma_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TX DMA mask enable:1: mask0: do not mask"]
    #[inline(always)]
    pub const fn set_tx_dma_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for DmaMask {
    #[inline(always)]
    fn default() -> DmaMask {
        DmaMask(0)
    }
}
impl core::fmt::Debug for DmaMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DmaMask")
            .field("rx_dma_mask", &self.rx_dma_mask())
            .field("tx_dma_mask", &self.tx_dma_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DmaMask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "DmaMask {{ rx_dma_mask: {=bool:?}, tx_dma_mask: {=bool:?} }}",
            self.rx_dma_mask(),
            self.tx_dma_mask()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FifoStatus(pub u32);
impl FifoStatus {
    #[doc = "FIFO Status output: Bit \\[7:0\\] = {tx_full,tx_empty,tx_almost_full,tx_almost_empty,rx_full,rx_empty,rx_almost_full,rx_almost_empty}"]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_status_out(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "FIFO Status output: Bit \\[7:0\\] = {tx_full,tx_empty,tx_almost_full,tx_almost_empty,rx_full,rx_empty,rx_almost_full,rx_almost_empty}"]
    #[inline(always)]
    pub const fn set_fifo_status_out(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for FifoStatus {
    #[inline(always)]
    fn default() -> FifoStatus {
        FifoStatus(0)
    }
}
impl core::fmt::Debug for FifoStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FifoStatus")
            .field("fifo_status_out", &self.fifo_status_out())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FifoStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "FifoStatus {{ fifo_status_out: {=u8:?} }}",
            self.fifo_status_out()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntMask(pub u32);
impl IntMask {
    #[doc = "Interrupt mask for RX FIFO push overflow, high active"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_fifo_int_mask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt mask for RX FIFO push overflow, high active"]
    #[inline(always)]
    pub const fn set_rx_fifo_int_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt mask for TX FIFO pop underflow, high active"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_fifo_int_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt mask for TX FIFO pop underflow, high active"]
    #[inline(always)]
    pub const fn set_tx_fifo_int_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IntMask {
    #[inline(always)]
    fn default() -> IntMask {
        IntMask(0)
    }
}
impl core::fmt::Debug for IntMask {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntMask")
            .field("rx_fifo_int_mask", &self.rx_fifo_int_mask())
            .field("tx_fifo_int_mask", &self.tx_fifo_int_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntMask {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntMask {{ rx_fifo_int_mask: {=bool:?}, tx_fifo_int_mask: {=bool:?} }}",
            self.rx_fifo_int_mask(),
            self.tx_fifo_int_mask()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntStatus(pub u32);
impl IntStatus {
    #[doc = "RX FIFO push overflow"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_fifo_overflow(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "RX FIFO push overflow"]
    #[inline(always)]
    pub const fn set_rx_fifo_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TX FIFO pop underflow"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_fifo_underflow(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "TX FIFO pop underflow"]
    #[inline(always)]
    pub const fn set_tx_fifo_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for IntStatus {
    #[inline(always)]
    fn default() -> IntStatus {
        IntStatus(0)
    }
}
impl core::fmt::Debug for IntStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntStatus")
            .field("rx_fifo_overflow", &self.rx_fifo_overflow())
            .field("tx_fifo_underflow", &self.tx_fifo_underflow())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "IntStatus {{ rx_fifo_overflow: {=bool:?}, tx_fifo_underflow: {=bool:?} }}",
            self.rx_fifo_overflow(),
            self.tx_fifo_underflow()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RecordDataSel(pub u32);
impl RecordDataSel {
    #[doc = "0: I2S audio recording 1: BT recording"]
    #[must_use]
    #[inline(always)]
    pub const fn rs_data_sel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0: I2S audio recording 1: BT recording"]
    #[inline(always)]
    pub const fn set_rs_data_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for RecordDataSel {
    #[inline(always)]
    fn default() -> RecordDataSel {
        RecordDataSel(0)
    }
}
impl core::fmt::Debug for RecordDataSel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RecordDataSel")
            .field("rs_data_sel", &self.rs_data_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RecordDataSel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RecordDataSel {{ rs_data_sel: {=bool:?} }}",
            self.rs_data_sel()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RecordFormat(pub u32);
impl RecordFormat {
    #[doc = "0: 8bit 1: 16bit RX fifo data format: Mono 8 bit (unsigned): RX FIFO_DIN\\[31:0\\] = {L3,L2,L1,L0}, each four samples need one FIFO write operation Stereo 8 bit (unsigned): RX_FIFO_DIN\\[31:0\\] = {R1,L1,R0,L0}, each tow samples need one FIFO write operation Mono 16 bit (Signed 2's complement): RX_FIFO_DIN\\[31:0\\] = {L1,L0}, each two samples need one FIFO write operation Stereo 16 bit (Signed 2's complement): RX_FIFO_DIN\\[31:0\\] = {R0,L0}, each sample need one FIFO write operation"]
    #[must_use]
    #[inline(always)]
    pub const fn dw(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0: 8bit 1: 16bit RX fifo data format: Mono 8 bit (unsigned): RX FIFO_DIN\\[31:0\\] = {L3,L2,L1,L0}, each four samples need one FIFO write operation Stereo 8 bit (unsigned): RX_FIFO_DIN\\[31:0\\] = {R1,L1,R0,L0}, each tow samples need one FIFO write operation Mono 16 bit (Signed 2's complement): RX_FIFO_DIN\\[31:0\\] = {L1,L0}, each two samples need one FIFO write operation Stereo 16 bit (Signed 2's complement): RX_FIFO_DIN\\[31:0\\] = {R0,L0}, each sample need one FIFO write operation"]
    #[inline(always)]
    pub const fn set_dw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1: mono recording, 0: stereo recording"]
    #[must_use]
    #[inline(always)]
    pub const fn track(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1: mono recording, 0: stereo recording"]
    #[inline(always)]
    pub const fn set_track(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for RecordFormat {
    #[inline(always)]
    fn default() -> RecordFormat {
        RecordFormat(0)
    }
}
impl core::fmt::Debug for RecordFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RecordFormat")
            .field("dw", &self.dw())
            .field("track", &self.track())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RecordFormat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RecordFormat {{ dw: {=bool:?}, track: {=bool:?} }}",
            self.dw(),
            self.track()
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
pub struct Rsvd10(pub u32);
impl Rsvd10 {}
impl Default for Rsvd10 {
    #[inline(always)]
    fn default() -> Rsvd10 {
        Rsvd10(0)
    }
}
impl core::fmt::Debug for Rsvd10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd10").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd10 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd10 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd11(pub u32);
impl Rsvd11 {}
impl Default for Rsvd11 {
    #[inline(always)]
    fn default() -> Rsvd11 {
        Rsvd11(0)
    }
}
impl core::fmt::Debug for Rsvd11 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd11").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd11 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd11 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd12(pub u32);
impl Rsvd12 {}
impl Default for Rsvd12 {
    #[inline(always)]
    fn default() -> Rsvd12 {
        Rsvd12(0)
    }
}
impl core::fmt::Debug for Rsvd12 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd12").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd12 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd12 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd13(pub u32);
impl Rsvd13 {}
impl Default for Rsvd13 {
    #[inline(always)]
    fn default() -> Rsvd13 {
        Rsvd13(0)
    }
}
impl core::fmt::Debug for Rsvd13 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd13").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd13 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd13 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd14(pub u32);
impl Rsvd14 {}
impl Default for Rsvd14 {
    #[inline(always)]
    fn default() -> Rsvd14 {
        Rsvd14(0)
    }
}
impl core::fmt::Debug for Rsvd14 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd14").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd14 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd14 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd15(pub u32);
impl Rsvd15 {}
impl Default for Rsvd15 {
    #[inline(always)]
    fn default() -> Rsvd15 {
        Rsvd15(0)
    }
}
impl core::fmt::Debug for Rsvd15 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd15").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd15 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd15 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd16(pub u32);
impl Rsvd16 {}
impl Default for Rsvd16 {
    #[inline(always)]
    fn default() -> Rsvd16 {
        Rsvd16(0)
    }
}
impl core::fmt::Debug for Rsvd16 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd16").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd16 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd16 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd17(pub u32);
impl Rsvd17 {}
impl Default for Rsvd17 {
    #[inline(always)]
    fn default() -> Rsvd17 {
        Rsvd17(0)
    }
}
impl core::fmt::Debug for Rsvd17 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd17").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd17 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd17 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd18(pub u32);
impl Rsvd18 {}
impl Default for Rsvd18 {
    #[inline(always)]
    fn default() -> Rsvd18 {
        Rsvd18(0)
    }
}
impl core::fmt::Debug for Rsvd18 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd18").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd18 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd18 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd19(pub u32);
impl Rsvd19 {}
impl Default for Rsvd19 {
    #[inline(always)]
    fn default() -> Rsvd19 {
        Rsvd19(0)
    }
}
impl core::fmt::Debug for Rsvd19 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd19").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd19 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd19 {{ }}",)
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
pub struct Rsvd20(pub u32);
impl Rsvd20 {}
impl Default for Rsvd20 {
    #[inline(always)]
    fn default() -> Rsvd20 {
        Rsvd20(0)
    }
}
impl core::fmt::Debug for Rsvd20 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd20").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd20 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd20 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd21(pub u32);
impl Rsvd21 {}
impl Default for Rsvd21 {
    #[inline(always)]
    fn default() -> Rsvd21 {
        Rsvd21(0)
    }
}
impl core::fmt::Debug for Rsvd21 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd21").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd21 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd21 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd22(pub u32);
impl Rsvd22 {}
impl Default for Rsvd22 {
    #[inline(always)]
    fn default() -> Rsvd22 {
        Rsvd22(0)
    }
}
impl core::fmt::Debug for Rsvd22 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd22").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd22 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd22 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd23(pub u32);
impl Rsvd23 {}
impl Default for Rsvd23 {
    #[inline(always)]
    fn default() -> Rsvd23 {
        Rsvd23(0)
    }
}
impl core::fmt::Debug for Rsvd23 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd23").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd23 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd23 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd24(pub u32);
impl Rsvd24 {}
impl Default for Rsvd24 {
    #[inline(always)]
    fn default() -> Rsvd24 {
        Rsvd24(0)
    }
}
impl core::fmt::Debug for Rsvd24 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd24").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd24 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd24 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd25(pub u32);
impl Rsvd25 {}
impl Default for Rsvd25 {
    #[inline(always)]
    fn default() -> Rsvd25 {
        Rsvd25(0)
    }
}
impl core::fmt::Debug for Rsvd25 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd25").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd25 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd25 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd26(pub u32);
impl Rsvd26 {}
impl Default for Rsvd26 {
    #[inline(always)]
    fn default() -> Rsvd26 {
        Rsvd26(0)
    }
}
impl core::fmt::Debug for Rsvd26 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd26").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd26 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd26 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd27(pub u32);
impl Rsvd27 {}
impl Default for Rsvd27 {
    #[inline(always)]
    fn default() -> Rsvd27 {
        Rsvd27(0)
    }
}
impl core::fmt::Debug for Rsvd27 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd27").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd27 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd27 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd28(pub u32);
impl Rsvd28 {}
impl Default for Rsvd28 {
    #[inline(always)]
    fn default() -> Rsvd28 {
        Rsvd28(0)
    }
}
impl core::fmt::Debug for Rsvd28 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd28").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd28 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd28 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd29(pub u32);
impl Rsvd29 {}
impl Default for Rsvd29 {
    #[inline(always)]
    fn default() -> Rsvd29 {
        Rsvd29(0)
    }
}
impl core::fmt::Debug for Rsvd29 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd29").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd29 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd29 {{ }}",)
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
pub struct Rsvd30(pub u32);
impl Rsvd30 {}
impl Default for Rsvd30 {
    #[inline(always)]
    fn default() -> Rsvd30 {
        Rsvd30(0)
    }
}
impl core::fmt::Debug for Rsvd30 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd30").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd30 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd30 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd31(pub u32);
impl Rsvd31 {}
impl Default for Rsvd31 {
    #[inline(always)]
    fn default() -> Rsvd31 {
        Rsvd31(0)
    }
}
impl core::fmt::Debug for Rsvd31 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd31").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd31 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd31 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd32(pub u32);
impl Rsvd32 {}
impl Default for Rsvd32 {
    #[inline(always)]
    fn default() -> Rsvd32 {
        Rsvd32(0)
    }
}
impl core::fmt::Debug for Rsvd32 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd32").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd32 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd32 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd33(pub u32);
impl Rsvd33 {}
impl Default for Rsvd33 {
    #[inline(always)]
    fn default() -> Rsvd33 {
        Rsvd33(0)
    }
}
impl core::fmt::Debug for Rsvd33 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd33").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd33 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd33 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd34(pub u32);
impl Rsvd34 {}
impl Default for Rsvd34 {
    #[inline(always)]
    fn default() -> Rsvd34 {
        Rsvd34(0)
    }
}
impl core::fmt::Debug for Rsvd34 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd34").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd34 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd34 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd35(pub u32);
impl Rsvd35 {}
impl Default for Rsvd35 {
    #[inline(always)]
    fn default() -> Rsvd35 {
        Rsvd35(0)
    }
}
impl core::fmt::Debug for Rsvd35 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd35").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd35 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd35 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd36(pub u32);
impl Rsvd36 {}
impl Default for Rsvd36 {
    #[inline(always)]
    fn default() -> Rsvd36 {
        Rsvd36(0)
    }
}
impl core::fmt::Debug for Rsvd36 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd36").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd36 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd36 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd37(pub u32);
impl Rsvd37 {}
impl Default for Rsvd37 {
    #[inline(always)]
    fn default() -> Rsvd37 {
        Rsvd37(0)
    }
}
impl core::fmt::Debug for Rsvd37 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd37").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd37 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd37 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd38(pub u32);
impl Rsvd38 {}
impl Default for Rsvd38 {
    #[inline(always)]
    fn default() -> Rsvd38 {
        Rsvd38(0)
    }
}
impl core::fmt::Debug for Rsvd38 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd38").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd38 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd38 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd39(pub u32);
impl Rsvd39 {}
impl Default for Rsvd39 {
    #[inline(always)]
    fn default() -> Rsvd39 {
        Rsvd39(0)
    }
}
impl core::fmt::Debug for Rsvd39 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd39").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd39 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd39 {{ }}",)
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
pub struct Rsvd40(pub u32);
impl Rsvd40 {}
impl Default for Rsvd40 {
    #[inline(always)]
    fn default() -> Rsvd40 {
        Rsvd40(0)
    }
}
impl core::fmt::Debug for Rsvd40 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd40").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd40 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd40 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd41(pub u32);
impl Rsvd41 {}
impl Default for Rsvd41 {
    #[inline(always)]
    fn default() -> Rsvd41 {
        Rsvd41(0)
    }
}
impl core::fmt::Debug for Rsvd41 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd41").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd41 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd41 {{ }}",)
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
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd6(pub u32);
impl Rsvd6 {}
impl Default for Rsvd6 {
    #[inline(always)]
    fn default() -> Rsvd6 {
        Rsvd6(0)
    }
}
impl core::fmt::Debug for Rsvd6 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd6").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd6 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd6 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd7(pub u32);
impl Rsvd7 {}
impl Default for Rsvd7 {
    #[inline(always)]
    fn default() -> Rsvd7 {
        Rsvd7(0)
    }
}
impl core::fmt::Debug for Rsvd7 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd7").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd7 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd7 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd8(pub u32);
impl Rsvd8 {}
impl Default for Rsvd8 {
    #[inline(always)]
    fn default() -> Rsvd8 {
        Rsvd8(0)
    }
}
impl core::fmt::Debug for Rsvd8 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd8").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd8 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd8 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsvd9(pub u32);
impl Rsvd9 {}
impl Default for Rsvd9 {
    #[inline(always)]
    fn default() -> Rsvd9 {
        Rsvd9(0)
    }
}
impl core::fmt::Debug for Rsvd9 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rsvd9").finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rsvd9 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rsvd9 {{ }}",)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxChSel(pub u32);
impl RxChSel {
    #[doc = "RX re-sampling module setting: 00: RD right = RX right 01: RD right = RX left 10,11: RD right = (RX left + RX right)/2"]
    #[must_use]
    #[inline(always)]
    pub const fn right_channel_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "RX re-sampling module setting: 00: RD right = RX right 01: RD right = RX left 10,11: RD right = (RX left + RX right)/2"]
    #[inline(always)]
    pub const fn set_right_channel_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "RX re-sampling module setting: 00: RD left = RX left 01: RD left = RX right 10,11: RD left = (RX left + RX right)/2"]
    #[must_use]
    #[inline(always)]
    pub const fn left_channel_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "RX re-sampling module setting: 00: RD left = RX left 01: RD left = RX right 10,11: RD left = (RX left + RX right)/2"]
    #[inline(always)]
    pub const fn set_left_channel_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
}
impl Default for RxChSel {
    #[inline(always)]
    fn default() -> RxChSel {
        RxChSel(0)
    }
}
impl core::fmt::Debug for RxChSel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxChSel")
            .field("right_channel_sel", &self.right_channel_sel())
            .field("left_channel_sel", &self.left_channel_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxChSel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RxChSel {{ right_channel_sel: {=u8:?}, left_channel_sel: {=u8:?} }}",
            self.right_channel_sel(),
            self.left_channel_sel()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxDmaEntry(pub u32);
impl RxDmaEntry {
    #[doc = "RX DMA entry"]
    #[must_use]
    #[inline(always)]
    pub const fn rx_dma_entry(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "RX DMA entry"]
    #[inline(always)]
    pub const fn set_rx_dma_entry(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for RxDmaEntry {
    #[inline(always)]
    fn default() -> RxDmaEntry {
        RxDmaEntry(0)
    }
}
impl core::fmt::Debug for RxDmaEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxDmaEntry")
            .field("rx_dma_entry", &self.rx_dma_entry())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxDmaEntry {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RxDmaEntry {{ rx_dma_entry: {=u32:?} }}",
            self.rx_dma_entry()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxReSample(pub u32);
impl RxReSample {
    #[doc = "0: Disable RX re-sample smooth filter 1: Enable RX re-sample smooth filter"]
    #[must_use]
    #[inline(always)]
    pub const fn smooth_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0: Disable RX re-sample smooth filter 1: Enable RX re-sample smooth filter"]
    #[inline(always)]
    pub const fn set_smooth_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for RxReSample {
    #[inline(always)]
    fn default() -> RxReSample {
        RxReSample(0)
    }
}
impl core::fmt::Debug for RxReSample {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxReSample")
            .field("smooth_en", &self.smooth_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxReSample {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RxReSample {{ smooth_en: {=bool:?} }}", self.smooth_en())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxReSampleClkDiv(pub u32);
impl RxReSampleClkDiv {
    #[doc = "source PCM sample clock duty cycle: 250 for 48K FS 272 for 44.1K FS 375 for 32K FS 500 for 24K FS 544 for 22.05K FS 750 for 16K FS 1000 for 12K FS 1088 for 11.025K FS 1500 for 8K FS Note: 1)duty_cycle = 12M/FS"]
    #[must_use]
    #[inline(always)]
    pub const fn rs_duty(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "source PCM sample clock duty cycle: 250 for 48K FS 272 for 44.1K FS 375 for 32K FS 500 for 24K FS 544 for 22.05K FS 750 for 16K FS 1000 for 12K FS 1088 for 11.025K FS 1500 for 8K FS Note: 1)duty_cycle = 12M/FS"]
    #[inline(always)]
    pub const fn set_rs_duty(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for RxReSampleClkDiv {
    #[inline(always)]
    fn default() -> RxReSampleClkDiv {
        RxReSampleClkDiv(0)
    }
}
impl core::fmt::Debug for RxReSampleClkDiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxReSampleClkDiv")
            .field("rs_duty", &self.rs_duty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxReSampleClkDiv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "RxReSampleClkDiv {{ rs_duty: {=u16:?} }}",
            self.rs_duty()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxDmaEntry(pub u32);
impl TxDmaEntry {
    #[doc = "TX DMA entry"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_dma_entry(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "TX DMA entry"]
    #[inline(always)]
    pub const fn set_tx_dma_entry(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for TxDmaEntry {
    #[inline(always)]
    fn default() -> TxDmaEntry {
        TxDmaEntry(0)
    }
}
impl core::fmt::Debug for TxDmaEntry {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxDmaEntry")
            .field("tx_dma_entry", &self.tx_dma_entry())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxDmaEntry {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TxDmaEntry {{ tx_dma_entry: {=u32:?} }}",
            self.tx_dma_entry()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxEqualizerEn(pub u32);
impl TxEqualizerEn {
    #[doc = "0: Disable TX equalizer 1: Enable TX equalizer equalizer is not implemented"]
    #[must_use]
    #[inline(always)]
    pub const fn tx_equalizer_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0: Disable TX equalizer 1: Enable TX equalizer equalizer is not implemented"]
    #[inline(always)]
    pub const fn set_tx_equalizer_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for TxEqualizerEn {
    #[inline(always)]
    fn default() -> TxEqualizerEn {
        TxEqualizerEn(0)
    }
}
impl core::fmt::Debug for TxEqualizerEn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxEqualizerEn")
            .field("tx_equalizer_en", &self.tx_equalizer_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxEqualizerEn {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TxEqualizerEn {{ tx_equalizer_en: {=bool:?} }}",
            self.tx_equalizer_en()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxEqualizerGain1(pub u32);
impl TxEqualizerGain1 {
    #[must_use]
    #[inline(always)]
    pub const fn band1_gain(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_band1_gain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn band2_gain(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_band2_gain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn band3_gain(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_band3_gain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn band4_gain(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_band4_gain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn band5_gain(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_band5_gain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn band6_gain(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_band6_gain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
    }
}
impl Default for TxEqualizerGain1 {
    #[inline(always)]
    fn default() -> TxEqualizerGain1 {
        TxEqualizerGain1(0)
    }
}
impl core::fmt::Debug for TxEqualizerGain1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxEqualizerGain1")
            .field("band1_gain", &self.band1_gain())
            .field("band2_gain", &self.band2_gain())
            .field("band3_gain", &self.band3_gain())
            .field("band4_gain", &self.band4_gain())
            .field("band5_gain", &self.band5_gain())
            .field("band6_gain", &self.band6_gain())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxEqualizerGain1 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TxEqualizerGain1 {{ band1_gain: {=u8:?}, band2_gain: {=u8:?}, band3_gain: {=u8:?}, band4_gain: {=u8:?}, band5_gain: {=u8:?}, band6_gain: {=u8:?} }}" , self . band1_gain () , self . band2_gain () , self . band3_gain () , self . band4_gain () , self . band5_gain () , self . band6_gain ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxEqualizerGain2(pub u32);
impl TxEqualizerGain2 {
    #[must_use]
    #[inline(always)]
    pub const fn band7_gain(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_band7_gain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn band8_gain(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_band8_gain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn band9_gain(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_band9_gain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[must_use]
    #[inline(always)]
    pub const fn band10_gain(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub const fn set_band10_gain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
    }
}
impl Default for TxEqualizerGain2 {
    #[inline(always)]
    fn default() -> TxEqualizerGain2 {
        TxEqualizerGain2(0)
    }
}
impl core::fmt::Debug for TxEqualizerGain2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxEqualizerGain2")
            .field("band7_gain", &self.band7_gain())
            .field("band8_gain", &self.band8_gain())
            .field("band9_gain", &self.band9_gain())
            .field("band10_gain", &self.band10_gain())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxEqualizerGain2 {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "TxEqualizerGain2 {{ band7_gain: {=u8:?}, band8_gain: {=u8:?}, band9_gain: {=u8:?}, band10_gain: {=u8:?} }}" , self . band7_gain () , self . band8_gain () , self . band9_gain () , self . band10_gain ())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxLrBalCtrl(pub u32);
impl TxLrBalCtrl {
    #[doc = "Balance volume control: 0000: Reserved, 0001: -1.5dB, 0010: -3.0dB, 0011: -4.5dB, 0100: -6.0dB, 0101: -7.5dB, 0110: -9.0dB, 0111: -10.5dB, 1000: -12dB, 1001: -13.5dB, 1010: -15dB, 1011: -16.5dB, 1100: -18dB, 1101: -19.5dB, 1110: -21dB, 1111: mute Note: 1) bit\\[5:0\\] = 101111 for left mute 2) bit\\[5:0\\] = 011111 for right mute 3) bit\\[5:4\\] = 00 or 11, bit\\[3:0\\] is don't care 4) +1.5db = 20log(1+1/4-1/16+1/1024) 5) -1.5dB = 20log(1-1/8-1/32-1/512-1/2048)"]
    #[must_use]
    #[inline(always)]
    pub const fn bal_vol(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Balance volume control: 0000: Reserved, 0001: -1.5dB, 0010: -3.0dB, 0011: -4.5dB, 0100: -6.0dB, 0101: -7.5dB, 0110: -9.0dB, 0111: -10.5dB, 1000: -12dB, 1001: -13.5dB, 1010: -15dB, 1011: -16.5dB, 1100: -18dB, 1101: -19.5dB, 1110: -21dB, 1111: mute Note: 1) bit\\[5:0\\] = 101111 for left mute 2) bit\\[5:0\\] = 011111 for right mute 3) bit\\[5:4\\] = 00 or 11, bit\\[3:0\\] is don't care 4) +1.5db = 20log(1+1/4-1/16+1/1024) 5) -1.5dB = 20log(1-1/8-1/32-1/512-1/2048)"]
    #[inline(always)]
    pub const fn set_bal_vol(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "LR balance enable: 00: both left and right in full volume 10: left channel balance volume adjustment enable 01: right channel balance volume adjustment enable 11: reserved, still kepp left and right in full volume"]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "LR balance enable: 00: both left and right in full volume 10: left channel balance volume adjustment enable 01: right channel balance volume adjustment enable 11: reserved, still kepp left and right in full volume"]
    #[inline(always)]
    pub const fn set_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
}
impl Default for TxLrBalCtrl {
    #[inline(always)]
    fn default() -> TxLrBalCtrl {
        TxLrBalCtrl(0)
    }
}
impl core::fmt::Debug for TxLrBalCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxLrBalCtrl")
            .field("bal_vol", &self.bal_vol())
            .field("en", &self.en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxLrBalCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TxLrBalCtrl {{ bal_vol: {=u8:?}, en: {=u8:?} }}",
            self.bal_vol(),
            self.en()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxPcmChSel(pub u32);
impl TxPcmChSel {
    #[doc = "TX re-sampling module setting: 00: TX right = source right 01: TX right = source left 10,11: TX right = (source left + source right)/2"]
    #[must_use]
    #[inline(always)]
    pub const fn right_channel_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "TX re-sampling module setting: 00: TX right = source right 01: TX right = source left 10,11: TX right = (source left + source right)/2"]
    #[inline(always)]
    pub const fn set_right_channel_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "TX re-sampling module setting: 00: TX left = source left 01: TX left = source right 10,11: TX left = (source left + source right)/2"]
    #[must_use]
    #[inline(always)]
    pub const fn left_channel_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "TX re-sampling module setting: 00: TX left = source left 01: TX left = source right 10,11: TX left = (source left + source right)/2"]
    #[inline(always)]
    pub const fn set_left_channel_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
}
impl Default for TxPcmChSel {
    #[inline(always)]
    fn default() -> TxPcmChSel {
        TxPcmChSel(0)
    }
}
impl core::fmt::Debug for TxPcmChSel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxPcmChSel")
            .field("right_channel_sel", &self.right_channel_sel())
            .field("left_channel_sel", &self.left_channel_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxPcmChSel {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TxPcmChSel {{ right_channel_sel: {=u8:?}, left_channel_sel: {=u8:?} }}",
            self.right_channel_sel(),
            self.left_channel_sel()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxPcmFormat(pub u32);
impl TxPcmFormat {
    #[doc = "tx source pcm data width N(N>=8) common value is 8,13,14,16,18,20,22,24 This data width indicate the tx fifo output data width. When writing to tx fifo, please refer to following format: Mono 8 bit: fifo_data\\[31:0\\] = {L3,L2,L1,L0}, each word contains 4 samples, so four samples need read one word Stereo 8 bit: fifo_data\\[31:0\\] = { R1,L1,R0,L0 }, each word contains 2 samples, so two samples need read one word Mono 13/14/16 bit: fifo_data\\[31:0\\] = {L1,L0}, each word contains 2 samples, so two samples need read one word Stereo 13/14/16 bit: fifo_data\\[31:0\\] = {R0,L0}, each word contains 1 samples, so each sample need read one word Mono 18/20/22/24 bit: fifo_data\\[31:0\\] = L0, each word contains 1 samples, so each sample need read one word Stereo 18/20/22/24 bit: fifo_data\\[31:0\\]\\[0\\] = {L0}, fifo_data\\[31:0\\]\\[1\\]={R0}, each 2 words contain 1 samples, so each sample need read two word"]
    #[must_use]
    #[inline(always)]
    pub const fn dw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "tx source pcm data width N(N>=8) common value is 8,13,14,16,18,20,22,24 This data width indicate the tx fifo output data width. When writing to tx fifo, please refer to following format: Mono 8 bit: fifo_data\\[31:0\\] = {L3,L2,L1,L0}, each word contains 4 samples, so four samples need read one word Stereo 8 bit: fifo_data\\[31:0\\] = { R1,L1,R0,L0 }, each word contains 2 samples, so two samples need read one word Mono 13/14/16 bit: fifo_data\\[31:0\\] = {L1,L0}, each word contains 2 samples, so two samples need read one word Stereo 13/14/16 bit: fifo_data\\[31:0\\] = {R0,L0}, each word contains 1 samples, so each sample need read one word Mono 18/20/22/24 bit: fifo_data\\[31:0\\] = L0, each word contains 1 samples, so each sample need read one word Stereo 18/20/22/24 bit: fifo_data\\[31:0\\]\\[0\\] = {L0}, fifo_data\\[31:0\\]\\[1\\]={R0}, each 2 words contain 1 samples, so each sample need read two word"]
    #[inline(always)]
    pub const fn set_dw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "0: stereo 1: mono"]
    #[must_use]
    #[inline(always)]
    pub const fn track_flag(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "0: stereo 1: mono"]
    #[inline(always)]
    pub const fn set_track_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for TxPcmFormat {
    #[inline(always)]
    fn default() -> TxPcmFormat {
        TxPcmFormat(0)
    }
}
impl core::fmt::Debug for TxPcmFormat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxPcmFormat")
            .field("dw", &self.dw())
            .field("track_flag", &self.track_flag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxPcmFormat {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "TxPcmFormat {{ dw: {=u8:?}, track_flag: {=bool:?} }}",
            self.dw(),
            self.track_flag()
        )
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxPcmSampleClk(pub u32);
impl TxPcmSampleClk {
    #[doc = "source PCM sample clock duty cycle(with GCLK=12MHz): 250 for 48K FS 272 for 44.1K FS 375 for 32K FS 500 for 24K FS 544 for 22.05K FS 750 for 16K FS 1000 for 12K FS 1088 for 11.025K FS 1500 for 8K FS"]
    #[must_use]
    #[inline(always)]
    pub const fn fs_duty(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "source PCM sample clock duty cycle(with GCLK=12MHz): 250 for 48K FS 272 for 44.1K FS 375 for 32K FS 500 for 24K FS 544 for 22.05K FS 750 for 16K FS 1000 for 12K FS 1088 for 11.025K FS 1500 for 8K FS"]
    #[inline(always)]
    pub const fn set_fs_duty(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
}
impl Default for TxPcmSampleClk {
    #[inline(always)]
    fn default() -> TxPcmSampleClk {
        TxPcmSampleClk(0)
    }
}
impl core::fmt::Debug for TxPcmSampleClk {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxPcmSampleClk")
            .field("fs_duty", &self.fs_duty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxPcmSampleClk {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TxPcmSampleClk {{ fs_duty: {=u16:?} }}", self.fs_duty())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxRsSmooth(pub u32);
impl TxRsSmooth {
    #[doc = "0: Disable TX re-sample smooth filter 1: Enable TX re-sample smooth filter This function is not implemented."]
    #[must_use]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0: Disable TX re-sample smooth filter 1: Enable TX re-sample smooth filter This function is not implemented."]
    #[inline(always)]
    pub const fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for TxRsSmooth {
    #[inline(always)]
    fn default() -> TxRsSmooth {
        TxRsSmooth(0)
    }
}
impl core::fmt::Debug for TxRsSmooth {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxRsSmooth")
            .field("en", &self.en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxRsSmooth {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TxRsSmooth {{ en: {=bool:?} }}", self.en())
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxVolCtrl(pub u32);
impl TxVolCtrl {
    #[doc = "volume control: 0000: +6dB, 0001: +4.5dB, 0010: +3dB, 0011: +1.5dB, 0100: 0dB, 0101: -1.5dB, 0110: -3.0dB, 0111: -4.5dB, 1000: -6.0dB, 1001: -7.5dB, 1010: -9dB, 1011: -10.5dB, 1100: -12dB, 1101: -13.5dB, 1110: -15dB, 1111: mute Note: 1) +1.5db = 20log(1+1/4-1/16+1/1024) 2) -1.5dB = 20log(1-1/8-1/32-1/512-1/2048)"]
    #[must_use]
    #[inline(always)]
    pub const fn vol(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "volume control: 0000: +6dB, 0001: +4.5dB, 0010: +3dB, 0011: +1.5dB, 0100: 0dB, 0101: -1.5dB, 0110: -3.0dB, 0111: -4.5dB, 1000: -6.0dB, 1001: -7.5dB, 1010: -9dB, 1011: -10.5dB, 1100: -12dB, 1101: -13.5dB, 1110: -15dB, 1111: mute Note: 1) +1.5db = 20log(1+1/4-1/16+1/1024) 2) -1.5dB = 20log(1-1/8-1/32-1/512-1/2048)"]
    #[inline(always)]
    pub const fn set_vol(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for TxVolCtrl {
    #[inline(always)]
    fn default() -> TxVolCtrl {
        TxVolCtrl(0)
    }
}
impl core::fmt::Debug for TxVolCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxVolCtrl")
            .field("vol", &self.vol())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxVolCtrl {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TxVolCtrl {{ vol: {=u8:?} }}", self.vol())
    }
}
