#[doc = "cache counter register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cacnt(pub u32);
impl Cacnt {
    #[doc = "cmd-cmd interval counter in hclk cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn cache_ncc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "cmd-cmd interval counter in hclk cycles"]
    #[inline(always)]
    pub const fn set_cache_ncc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "data-cmd interval counter in hclk cycles"]
    #[must_use]
    #[inline(always)]
    pub const fn cache_ndc(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "data-cmd interval counter in hclk cycles"]
    #[inline(always)]
    pub const fn set_cache_ndc(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "timeout count register for ahb read"]
    #[must_use]
    #[inline(always)]
    pub const fn cache_tor(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "timeout count register for ahb read"]
    #[inline(always)]
    pub const fn set_cache_tor(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Cacnt {
    #[inline(always)]
    fn default() -> Cacnt {
        Cacnt(0)
    }
}
impl core::fmt::Debug for Cacnt {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cacnt")
            .field("cache_ncc", &self.cache_ncc())
            .field("cache_ndc", &self.cache_ndc())
            .field("cache_tor", &self.cache_tor())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cacnt {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Cacnt {{ cache_ncc: {=u8:?}, cache_ndc: {=u8:?}, cache_tor: {=u16:?} }}",
            self.cache_ncc(),
            self.cache_ndc(),
            self.cache_tor()
        )
    }
}
#[doc = "cache control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cacr(pub u32);
impl Cacr {
    #[doc = "Command index for cache read. CMD18 by default"]
    #[must_use]
    #[inline(always)]
    pub const fn read_index(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Command index for cache read. CMD18 by default"]
    #[inline(always)]
    pub const fn set_read_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Read command have a response"]
    #[must_use]
    #[inline(always)]
    pub const fn read_has_rsp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Read command have a response"]
    #[inline(always)]
    pub const fn set_read_has_rsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Read response is 136-bit, long response"]
    #[must_use]
    #[inline(always)]
    pub const fn read_long_rsp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Read response is 136-bit, long response"]
    #[inline(always)]
    pub const fn set_read_long_rsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Command index for stop. CMD12 by default"]
    #[must_use]
    #[inline(always)]
    pub const fn stop_index(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "Command index for stop. CMD12 by default"]
    #[inline(always)]
    pub const fn set_stop_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "Stop command have a response"]
    #[must_use]
    #[inline(always)]
    pub const fn stop_has_rsp(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Stop command have a response"]
    #[inline(always)]
    pub const fn set_stop_has_rsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Stop response is 136-bit, long response"]
    #[must_use]
    #[inline(always)]
    pub const fn stop_long_rsp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Stop response is 136-bit, long response"]
    #[inline(always)]
    pub const fn set_stop_long_rsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "cache depth is cache_block blocks"]
    #[must_use]
    #[inline(always)]
    pub const fn cache_block(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "cache depth is cache_block blocks"]
    #[inline(always)]
    pub const fn set_cache_block(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "cache prefetch depth is cache_pref_block blocks. Should be no less than cache_block"]
    #[must_use]
    #[inline(always)]
    pub const fn cache_pref_block(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "cache prefetch depth is cache_pref_block blocks. Should be no less than cache_block"]
    #[inline(always)]
    pub const fn set_cache_pref_block(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "1: generate ahb error response when error occur 0: no ahb error response generated. Could check cache_err interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn cache_hresp(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "1: generate ahb error response when error occur 0: no ahb error response generated. Could check cache_err interrupt"]
    #[inline(always)]
    pub const fn set_cache_hresp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "1: return ahb data without crc check 0: return ahb data after block crc pass"]
    #[must_use]
    #[inline(always)]
    pub const fn cache_nocrc(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "1: return ahb data without crc check 0: return ahb data after block crc pass"]
    #[inline(always)]
    pub const fn set_cache_nocrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "select card version 1: card size (=2GB, address of cmd18 is in byte 0: card size >2GB, address of cmd18 is in block"]
    #[must_use]
    #[inline(always)]
    pub const fn cache_sdsc(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "select card version 1: card size (=2GB, address of cmd18 is in byte 0: card size >2GB, address of cmd18 is in block"]
    #[inline(always)]
    pub const fn set_cache_sdsc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "force cache read done 1: start new fetch for miss access only after cache read done 0: start new fetch for miss access even when cache is still filling (read will be breaked by cmd12)"]
    #[must_use]
    #[inline(always)]
    pub const fn cache_force_read(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "force cache read done 1: start new fetch for miss access only after cache read done 0: start new fetch for miss access even when cache is still filling (read will be breaked by cmd12)"]
    #[inline(always)]
    pub const fn set_cache_force_read(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "enable ahb read timeout recover"]
    #[must_use]
    #[inline(always)]
    pub const fn cache_to_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "enable ahb read timeout recover"]
    #[inline(always)]
    pub const fn set_cache_to_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "enable cache 1: ahb read will return cached data 0: ahb read always return dummy data with no error response"]
    #[must_use]
    #[inline(always)]
    pub const fn cache_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "enable cache 1: ahb read will return cached data 0: ahb read always return dummy data with no error response"]
    #[inline(always)]
    pub const fn set_cache_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cacr {
    #[inline(always)]
    fn default() -> Cacr {
        Cacr(0)
    }
}
impl core::fmt::Debug for Cacr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cacr")
            .field("read_index", &self.read_index())
            .field("read_has_rsp", &self.read_has_rsp())
            .field("read_long_rsp", &self.read_long_rsp())
            .field("stop_index", &self.stop_index())
            .field("stop_has_rsp", &self.stop_has_rsp())
            .field("stop_long_rsp", &self.stop_long_rsp())
            .field("cache_block", &self.cache_block())
            .field("cache_pref_block", &self.cache_pref_block())
            .field("cache_hresp", &self.cache_hresp())
            .field("cache_nocrc", &self.cache_nocrc())
            .field("cache_sdsc", &self.cache_sdsc())
            .field("cache_force_read", &self.cache_force_read())
            .field("cache_to_en", &self.cache_to_en())
            .field("cache_en", &self.cache_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cacr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cacr {{ read_index: {=u8:?}, read_has_rsp: {=bool:?}, read_long_rsp: {=bool:?}, stop_index: {=u8:?}, stop_has_rsp: {=bool:?}, stop_long_rsp: {=bool:?}, cache_block: {=u8:?}, cache_pref_block: {=u8:?}, cache_hresp: {=bool:?}, cache_nocrc: {=bool:?}, cache_sdsc: {=bool:?}, cache_force_read: {=bool:?}, cache_to_en: {=bool:?}, cache_en: {=bool:?} }}" , self . read_index () , self . read_has_rsp () , self . read_long_rsp () , self . stop_index () , self . stop_has_rsp () , self . stop_long_rsp () , self . cache_block () , self . cache_pref_block () , self . cache_hresp () , self . cache_nocrc () , self . cache_sdsc () , self . cache_force_read () , self . cache_to_en () , self . cache_en ())
    }
}
#[doc = "cache offset register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Caoff(pub u32);
impl Caoff {
    #[doc = "offset to map ahb address to sd address for ahb access"]
    #[must_use]
    #[inline(always)]
    pub const fn cache_offset(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "offset to map ahb address to sd address for ahb access"]
    #[inline(always)]
    pub const fn set_cache_offset(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Caoff {
    #[inline(always)]
    fn default() -> Caoff {
        Caoff(0)
    }
}
impl core::fmt::Debug for Caoff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Caoff")
            .field("cache_offset", &self.cache_offset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Caoff {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Caoff {{ cache_offset: {=u32:?} }}", self.cache_offset())
    }
}
#[doc = "command argument register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Car(pub u32);
impl Car {
    #[doc = "Command argument"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_arg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Command argument"]
    #[inline(always)]
    pub const fn set_cmd_arg(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Car {
    #[inline(always)]
    fn default() -> Car {
        Car(0)
    }
}
impl core::fmt::Debug for Car {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Car")
            .field("cmd_arg", &self.cmd_arg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Car {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Car {{ cmd_arg: {=u32:?} }}", self.cmd_arg())
    }
}
#[doc = "cache status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Casr(pub u32);
impl Casr {
    #[doc = "Set 1 to request sd normal access. sd_req will be cleared automatically after sd_busy asserted"]
    #[must_use]
    #[inline(always)]
    pub const fn sd_req(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to request sd normal access. sd_req will be cleared automatically after sd_busy asserted"]
    #[inline(always)]
    pub const fn set_sd_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Read 1 indicates sd is ready for normal access. Ahb access will be hold during sd_busy asserted. After sd normal access done, write 1 to clear, and ahb access will continue"]
    #[must_use]
    #[inline(always)]
    pub const fn sd_busy(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Read 1 indicates sd is ready for normal access. Ahb access will be hold during sd_busy asserted. After sd normal access done, write 1 to clear, and ahb access will continue"]
    #[inline(always)]
    pub const fn set_sd_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Indicates cache is working"]
    #[must_use]
    #[inline(always)]
    pub const fn cache_busy(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates cache is working"]
    #[inline(always)]
    pub const fn set_cache_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set 1 to flush cache. Should set when cache not busy."]
    #[must_use]
    #[inline(always)]
    pub const fn cache_flush(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Set 1 to flush cache. Should set when cache not busy."]
    #[inline(always)]
    pub const fn set_cache_flush(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Casr {
    #[inline(always)]
    fn default() -> Casr {
        Casr(0)
    }
}
impl core::fmt::Debug for Casr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Casr")
            .field("sd_req", &self.sd_req())
            .field("sd_busy", &self.sd_busy())
            .field("cache_busy", &self.cache_busy())
            .field("cache_flush", &self.cache_flush())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Casr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Casr {{ sd_req: {=bool:?}, sd_busy: {=bool:?}, cache_busy: {=bool:?}, cache_flush: {=bool:?} }}" , self . sd_req () , self . sd_busy () , self . cache_busy () , self . cache_flush ())
    }
}
#[doc = "command control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccr(pub u32);
impl Ccr {
    #[doc = "Command start write 1 to start command TX, and when begin to TX command, the bit will return into 0."]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Command start write 1 to start command TX, and when begin to TX command, the bit will return into 0."]
    #[inline(always)]
    pub const fn set_cmd_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "TX command enable 1: enable TX command 0: disable TX command"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_tx_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "TX command enable 1: enable TX command 0: disable TX command"]
    #[inline(always)]
    pub const fn set_cmd_tx_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Command pending enable When prepare to send stop command, this bit should be set. Controller will calculate a proper time point to send out the command to guarantee all the data have been transferred. And this is mainly used in stream mode. Recommend using set_block_count (SD/MMC basis command) to control transferring data for block mode. If send stop command for canceling this transfer (such as CRC error in multi-block), no need to set the bit."]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_pend(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Command pending enable When prepare to send stop command, this bit should be set. Controller will calculate a proper time point to send out the command to guarantee all the data have been transferred. And this is mainly used in stream mode. Recommend using set_block_count (SD/MMC basis command) to control transferring data for block mode. If send stop command for canceling this transfer (such as CRC error in multi-block), no need to set the bit."]
    #[inline(always)]
    pub const fn set_cmd_pend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "1: Response expected after command 0: No response expected after command"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_has_rsp(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "1: Response expected after command 0: No response expected after command"]
    #[inline(always)]
    pub const fn set_cmd_has_rsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "1: Response will be 136-bit, long response 0: Response will be 48-bit, normal response"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_long_rsp(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "1: Response will be 136-bit, long response 0: Response will be 48-bit, normal response"]
    #[inline(always)]
    pub const fn set_cmd_long_rsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Command index"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_index(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x3f;
        val as u8
    }
    #[doc = "Command index"]
    #[inline(always)]
    pub const fn set_cmd_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
    }
}
impl Default for Ccr {
    #[inline(always)]
    fn default() -> Ccr {
        Ccr(0)
    }
}
impl core::fmt::Debug for Ccr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ccr")
            .field("cmd_start", &self.cmd_start())
            .field("cmd_tx_en", &self.cmd_tx_en())
            .field("cmd_pend", &self.cmd_pend())
            .field("cmd_has_rsp", &self.cmd_has_rsp())
            .field("cmd_long_rsp", &self.cmd_long_rsp())
            .field("cmd_index", &self.cmd_index())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ccr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ccr {{ cmd_start: {=bool:?}, cmd_tx_en: {=bool:?}, cmd_pend: {=bool:?}, cmd_has_rsp: {=bool:?}, cmd_long_rsp: {=bool:?}, cmd_index: {=u8:?} }}" , self . cmd_start () , self . cmd_tx_en () , self . cmd_pend () , self . cmd_has_rsp () , self . cmd_long_rsp () , self . cmd_index ())
    }
}
#[doc = "clock duty cycle register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdcr(pub u32);
impl Cdcr {
    #[doc = "1: the sd clock is 50% duty cycle 0: the high level of the sd clock is 1 hclk cycle"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_config(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1: the sd clock is 50% duty cycle 0: the high level of the sd clock is 1 hclk cycle"]
    #[inline(always)]
    pub const fn set_clk_config(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Cdcr {
    #[inline(always)]
    fn default() -> Cdcr {
        Cdcr(0)
    }
}
impl core::fmt::Debug for Cdcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cdcr")
            .field("clk_config", &self.clk_config())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cdcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Cdcr {{ clk_config: {=bool:?} }}", self.clk_config())
    }
}
#[doc = "card interface control and card detect register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cdr(pub u32);
impl Cdr {
    #[doc = "Use sd_data\\[3\\] to do card detect 0: use special pin to do card detect / write protect. (Currently not supported) 1: use sd_data\\[3\\] to do card detect (default)"]
    #[must_use]
    #[inline(always)]
    pub const fn sd_data3_cd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Use sd_data\\[3\\] to do card detect 0: use special pin to do card detect / write protect. (Currently not supported) 1: use sd_data\\[3\\] to do card detect (default)"]
    #[inline(always)]
    pub const fn set_sd_data3_cd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "select input sample timing (according to itiming config)"]
    #[must_use]
    #[inline(always)]
    pub const fn itiming_sel(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "select input sample timing (according to itiming config)"]
    #[inline(always)]
    pub const fn set_itiming_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "select output timing (according to otiming config)"]
    #[must_use]
    #[inline(always)]
    pub const fn otiming_sel(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "select output timing (according to otiming config)"]
    #[inline(always)]
    pub const fn set_otiming_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable card detect Only when the bit is valid, controller does card detect. If use sd_data\\[3\\] to do card detect, the bit should be cleared when transfer valid data."]
    #[must_use]
    #[inline(always)]
    pub const fn en_cd(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable card detect Only when the bit is valid, controller does card detect. If use sd_data\\[3\\] to do card detect, the bit should be cleared when transfer valid data."]
    #[inline(always)]
    pub const fn set_en_cd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Card detect high level valid 0: detect low level means card exist 1: detect high level means card exist (default)"]
    #[must_use]
    #[inline(always)]
    pub const fn cd_hvalid(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Card detect high level valid 0: detect low level means card exist 1: detect high level means card exist (default)"]
    #[inline(always)]
    pub const fn set_cd_hvalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Open Drain mode for cmd line (for eMMC identification mode) 0: cmd line is push-pull 1: cmd line is open-drain"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_od(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Open Drain mode for cmd line (for eMMC identification mode) 0: cmd line is push-pull 1: cmd line is open-drain"]
    #[inline(always)]
    pub const fn set_cmd_od(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "define input timing"]
    #[must_use]
    #[inline(always)]
    pub const fn itiming(&self) -> u16 {
        let val = (self.0 >> 6usize) & 0x1fff;
        val as u16
    }
    #[doc = "define input timing"]
    #[inline(always)]
    pub const fn set_itiming(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 6usize)) | (((val as u32) & 0x1fff) << 6usize);
    }
    #[doc = "define output timing"]
    #[must_use]
    #[inline(always)]
    pub const fn otiming(&self) -> u16 {
        let val = (self.0 >> 19usize) & 0x1fff;
        val as u16
    }
    #[doc = "define output timing"]
    #[inline(always)]
    pub const fn set_otiming(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 19usize)) | (((val as u32) & 0x1fff) << 19usize);
    }
}
impl Default for Cdr {
    #[inline(always)]
    fn default() -> Cdr {
        Cdr(0)
    }
}
impl core::fmt::Debug for Cdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cdr")
            .field("sd_data3_cd", &self.sd_data3_cd())
            .field("itiming_sel", &self.itiming_sel())
            .field("otiming_sel", &self.otiming_sel())
            .field("en_cd", &self.en_cd())
            .field("cd_hvalid", &self.cd_hvalid())
            .field("cmd_od", &self.cmd_od())
            .field("itiming", &self.itiming())
            .field("otiming", &self.otiming())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cdr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Cdr {{ sd_data3_cd: {=bool:?}, itiming_sel: {=bool:?}, otiming_sel: {=bool:?}, en_cd: {=bool:?}, cd_hvalid: {=bool:?}, cmd_od: {=bool:?}, itiming: {=u16:?}, otiming: {=u16:?} }}" , self . sd_data3_cd () , self . itiming_sel () , self . otiming_sel () , self . en_cd () , self . cd_hvalid () , self . cmd_od () , self . itiming () , self . otiming ())
    }
}
#[doc = "CE-ATA/SDIO mode register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ceata(pub u32);
impl Ceata {
    #[doc = "Select the card type, default is sd card 0: sd card mode 1: CE-ATA device mode"]
    #[must_use]
    #[inline(always)]
    pub const fn ata_mode(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Select the card type, default is sd card 0: sd card mode 1: CE-ATA device mode"]
    #[inline(always)]
    pub const fn set_ata_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Select the sdio card mode, default is sd card 0: sd card mode , no sdio card interrupt 1: sdio card mode , enable sdio card interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn enable_sdio_irq(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Select the sdio card mode, default is sd card 0: sd card mode , no sdio card interrupt 1: sdio card mode , enable sdio card interrupt"]
    #[inline(always)]
    pub const fn set_enable_sdio_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Select the sdio host 4 wires interrupt support 0: host not support 4 wires interrupt on single-block data transfers 1: host support 4 wires interrupt on single-block data transfers"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio_4wires_irq(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Select the sdio host 4 wires interrupt support 0: host not support 4 wires interrupt on single-block data transfers 1: host support 4 wires interrupt on single-block data transfers"]
    #[inline(always)]
    pub const fn set_sdio_4wires_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Select the sdio host 4 wires interrupt on multi-block support 0: host not support 4 wires interrupt on multi-block data transfers 1: host support 4 wires interrupt on multi-block data transfers"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio_4wires_multi_irq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Select the sdio host 4 wires interrupt on multi-block support 0: host not support 4 wires interrupt on multi-block data transfers 1: host support 4 wires interrupt on multi-block data transfers"]
    #[inline(always)]
    pub const fn set_sdio_4wires_multi_irq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Ceata {
    #[inline(always)]
    fn default() -> Ceata {
        Ceata(0)
    }
}
impl core::fmt::Debug for Ceata {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ceata")
            .field("ata_mode", &self.ata_mode())
            .field("enable_sdio_irq", &self.enable_sdio_irq())
            .field("sdio_4wires_irq", &self.sdio_4wires_irq())
            .field("sdio_4wires_multi_irq", &self.sdio_4wires_multi_irq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ceata {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ceata {{ ata_mode: {=bool:?}, enable_sdio_irq: {=bool:?}, sdio_4wires_irq: {=bool:?}, sdio_4wires_multi_irq: {=bool:?} }}" , self . ata_mode () , self . enable_sdio_irq () , self . sdio_4wires_irq () , self . sdio_4wires_multi_irq ())
    }
}
#[doc = "clock control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkcr(pub u32);
impl Clkcr {
    #[doc = "Disable SD card clock 1: stop SD card clock 0: SD card clock generated"]
    #[must_use]
    #[inline(always)]
    pub const fn stop_clk(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Disable SD card clock 1: stop SD card clock 0: SD card clock generated"]
    #[inline(always)]
    pub const fn set_stop_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Void FIFO error 0: close the function 1: open the function If open it, when FIFO will be overrun or underrun soon, the SD_CLK and the clock enable of this module will be closed, and wait to host to read or write FIFO. Note: this function needs to be supported by card."]
    #[must_use]
    #[inline(always)]
    pub const fn void_fifo_error(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Void FIFO error 0: close the function 1: open the function If open it, when FIFO will be overrun or underrun soon, the SD_CLK and the clock enable of this module will be closed, and wait to host to read or write FIFO. Note: this function needs to be supported by card."]
    #[inline(always)]
    pub const fn set_void_fifo_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "select clock delay for rx sample 0: no delay 1: delay level 1 (~1.5ns typical) 2: delay level 2 (~3ns typical) 3: delay level 3 (~5ns typical)"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_tune_sel(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "select clock delay for rx sample 0: no delay 1: delay level 1 (~1.5ns typical) 2: delay level 2 (~3ns typical) 3: delay level 3 (~5ns typical)"]
    #[inline(always)]
    pub const fn set_clk_tune_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Divide card clock counter. 0 is illegal. sd_clock = hclk/(div + 1) If hclk is 240M and div is 599, 400KHz SD clock will be generated."]
    #[must_use]
    #[inline(always)]
    pub const fn div(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x1fff;
        val as u16
    }
    #[doc = "Divide card clock counter. 0 is illegal. sd_clock = hclk/(div + 1) If hclk is 240M and div is 599, 400KHz SD clock will be generated."]
    #[inline(always)]
    pub const fn set_div(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 8usize)) | (((val as u32) & 0x1fff) << 8usize);
    }
}
impl Default for Clkcr {
    #[inline(always)]
    fn default() -> Clkcr {
        Clkcr(0)
    }
}
impl core::fmt::Debug for Clkcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Clkcr")
            .field("stop_clk", &self.stop_clk())
            .field("void_fifo_error", &self.void_fifo_error())
            .field("clk_tune_sel", &self.clk_tune_sel())
            .field("div", &self.div())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Clkcr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Clkcr {{ stop_clk: {=bool:?}, void_fifo_error: {=bool:?}, clk_tune_sel: {=u8:?}, div: {=u16:?} }}" , self . stop_clk () , self . void_fifo_error () , self . clk_tune_sel () , self . div ())
    }
}
#[doc = "card debug port1 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgr1(pub u32);
impl Dbgr1 {
    #[doc = "command state for debug only"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_st(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "command state for debug only"]
    #[inline(always)]
    pub const fn set_cmd_st(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "data state for debug only"]
    #[must_use]
    #[inline(always)]
    pub const fn data_st(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x7fff;
        val as u16
    }
    #[doc = "data state for debug only"]
    #[inline(always)]
    pub const fn set_data_st(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
    }
}
impl Default for Dbgr1 {
    #[inline(always)]
    fn default() -> Dbgr1 {
        Dbgr1(0)
    }
}
impl core::fmt::Debug for Dbgr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbgr1")
            .field("cmd_st", &self.cmd_st())
            .field("data_st", &self.data_st())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbgr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dbgr1 {{ cmd_st: {=u16:?}, data_st: {=u16:?} }}",
            self.cmd_st(),
            self.data_st()
        )
    }
}
#[doc = "card debug port2 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgr2(pub u32);
impl Dbgr2 {
    #[doc = "for debug only"]
    #[must_use]
    #[inline(always)]
    pub const fn host_word_counter(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn set_host_word_counter(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "for debug only"]
    #[must_use]
    #[inline(always)]
    pub const fn valid_data_cou(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn set_valid_data_cou(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[doc = "for debug only"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_sel(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "for debug only"]
    #[inline(always)]
    pub const fn set_dbg_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Dbgr2 {
    #[inline(always)]
    fn default() -> Dbgr2 {
        Dbgr2(0)
    }
}
impl core::fmt::Debug for Dbgr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbgr2")
            .field("host_word_counter", &self.host_word_counter())
            .field("valid_data_cou", &self.valid_data_cou())
            .field("dbg_sel", &self.dbg_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbgr2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dbgr2 {{ host_word_counter: {=u16:?}, valid_data_cou: {=u16:?}, dbg_sel: {=u8:?} }}",
            self.host_word_counter(),
            self.valid_data_cou(),
            self.dbg_sel()
        )
    }
}
#[doc = "data control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcr(pub u32);
impl Dcr {
    #[doc = "Start transfer data set 1 to let the controller begin to transfer data (in fact, go into wait write or wait read state). After begin to transfer, this bit will be back to 0."]
    #[must_use]
    #[inline(always)]
    pub const fn data_start(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Start transfer data set 1 to let the controller begin to transfer data (in fact, go into wait write or wait read state). After begin to transfer, this bit will be back to 0."]
    #[inline(always)]
    pub const fn set_data_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transfer data enable 0: disable transfer data. After disable data transfer, stop command should be sent to card 1: enable data transfer"]
    #[must_use]
    #[inline(always)]
    pub const fn tran_data_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer data enable 0: disable transfer data. After disable data transfer, stop command should be sent to card 1: enable data transfer"]
    #[inline(always)]
    pub const fn set_tran_data_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Write or read 0: write data into card 1: read data from card"]
    #[must_use]
    #[inline(always)]
    pub const fn r_wn(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Write or read 0: write data into card 1: read data from card"]
    #[inline(always)]
    pub const fn set_r_wn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Data transfer mode 0: block 1: stream"]
    #[must_use]
    #[inline(always)]
    pub const fn stream_mode(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Data transfer mode 0: block 1: stream"]
    #[inline(always)]
    pub const fn set_stream_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Wide data bus mode 00: 1 wire bus 01: 4 wires wide bus 1X: reserved"]
    #[must_use]
    #[inline(always)]
    pub const fn wire_mode(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x03;
        val as u8
    }
    #[doc = "Wide data bus mode 00: 1 wire bus 01: 4 wires wide bus 1X: reserved"]
    #[inline(always)]
    pub const fn set_wire_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
    }
    #[doc = "Data block size is block_size+1 (max 2048 bytes) 0: 1 byte 0x1ff: 512 bytes"]
    #[must_use]
    #[inline(always)]
    pub const fn block_size(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "Data block size is block_size+1 (max 2048 bytes) 0: 1 byte 0x1ff: 512 bytes"]
    #[inline(always)]
    pub const fn set_block_size(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for Dcr {
    #[inline(always)]
    fn default() -> Dcr {
        Dcr(0)
    }
}
impl core::fmt::Debug for Dcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dcr")
            .field("data_start", &self.data_start())
            .field("tran_data_en", &self.tran_data_en())
            .field("r_wn", &self.r_wn())
            .field("stream_mode", &self.stream_mode())
            .field("wire_mode", &self.wire_mode())
            .field("block_size", &self.block_size())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dcr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dcr {{ data_start: {=bool:?}, tran_data_en: {=bool:?}, r_wn: {=bool:?}, stream_mode: {=bool:?}, wire_mode: {=u8:?}, block_size: {=u16:?} }}" , self . data_start () , self . tran_data_en () , self . r_wn () , self . stream_mode () , self . wire_mode () , self . block_size ())
    }
}
#[doc = "data length register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dlr(pub u32);
impl Dlr {
    #[doc = "Data length value. The number of data bytes is data_len+1. The number of data bytes should be a multiple of data block size. 0 is 1 byte. 0x1ff is 512 bytes. Max is 63.5KB."]
    #[must_use]
    #[inline(always)]
    pub const fn data_len(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data length value. The number of data bytes is data_len+1. The number of data bytes should be a multiple of data block size. 0 is 1 byte. 0x1ff is 512 bytes. Max is 63.5KB."]
    #[inline(always)]
    pub const fn set_data_len(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "The number of blocks which have been transferred successfully 1 = 1 block transferred It is cleared when start transfer data bit is set."]
    #[must_use]
    #[inline(always)]
    pub const fn block_tran_num(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "The number of blocks which have been transferred successfully 1 = 1 block transferred It is cleared when start transfer data bit is set."]
    #[inline(always)]
    pub const fn set_block_tran_num(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Dlr {
    #[inline(always)]
    fn default() -> Dlr {
        Dlr(0)
    }
}
impl core::fmt::Debug for Dlr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dlr")
            .field("data_len", &self.data_len())
            .field("block_tran_num", &self.block_tran_num())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dlr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Dlr {{ data_len: {=u16:?}, block_tran_num: {=u16:?} }}",
            self.data_len(),
            self.block_tran_num()
        )
    }
}
#[doc = "data status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsr(pub u32);
impl Dsr {
    #[doc = "The status of each sd data pad status"]
    #[must_use]
    #[inline(always)]
    pub const fn sd_data_i_ll(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The status of each sd data pad status"]
    #[inline(always)]
    pub const fn set_sd_data_i_ll(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Dsr {
    #[inline(always)]
    fn default() -> Dsr {
        Dsr(0)
    }
}
impl core::fmt::Debug for Dsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dsr")
            .field("sd_data_i_ll", &self.sd_data_i_ll())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dsr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Dsr {{ sd_data_i_ll: {=u8:?} }}", self.sd_data_i_ll())
    }
}
#[doc = "FIFO entry"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifo(pub u32);
impl Fifo {
    #[doc = "Entry to access internal FIFO. Access should be word-aligned, ranging from 0x200 to 0x3fc. Inside the range, write to any address will push the data into the FIFO, and read any address will pop a word from the FIFO."]
    #[must_use]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Entry to access internal FIFO. Access should be word-aligned, ranging from 0x200 to 0x3fc. Inside the range, write to any address will push the data into the FIFO, and read any address will pop a word from the FIFO."]
    #[inline(always)]
    pub const fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Fifo {
    #[inline(always)]
    fn default() -> Fifo {
        Fifo(0)
    }
}
impl core::fmt::Debug for Fifo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifo").field("data", &self.data()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Fifo {{ data: {=u32:?} }}", self.data())
    }
}
#[doc = "command and data interrupt mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ier(pub u32);
impl Ier {
    #[doc = "Command done bit mask for interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_done_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Command done bit mask for interrupt"]
    #[inline(always)]
    pub const fn set_cmd_done_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Command CRC error bit mask for interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_rsp_crc_mask(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Command CRC error bit mask for interrupt"]
    #[inline(always)]
    pub const fn set_cmd_rsp_crc_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Command timeout bit mask for interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_timeout_mask(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Command timeout bit mask for interrupt"]
    #[inline(always)]
    pub const fn set_cmd_timeout_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Data transfer done bit mask for interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn data_done_mask(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Data transfer done bit mask for interrupt"]
    #[inline(always)]
    pub const fn set_data_done_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Data CRC error bit mask for interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn data_crc_mask(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Data CRC error bit mask for interrupt"]
    #[inline(always)]
    pub const fn set_data_crc_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Data timeout bit mask for interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn data_timeout_mask(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Data timeout bit mask for interrupt"]
    #[inline(always)]
    pub const fn set_data_timeout_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Wide bus start bits error bit mask for interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn startbit_error_mask(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Wide bus start bits error bit mask for interrupt"]
    #[inline(always)]
    pub const fn set_startbit_error_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "FIFO underrun bit mask for interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_underrun_mask(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO underrun bit mask for interrupt"]
    #[inline(always)]
    pub const fn set_fifo_underrun_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "FIFO overrun bit mask for interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_overrun_mask(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO overrun bit mask for interrupt"]
    #[inline(always)]
    pub const fn set_fifo_overrun_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Command sent mask for interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_sent_mask(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Command sent mask for interrupt"]
    #[inline(always)]
    pub const fn set_cmd_sent_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Detect card insert mask for interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn card_insert_mask(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Detect card insert mask for interrupt"]
    #[inline(always)]
    pub const fn set_card_insert_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Detect card remove mask for interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn card_remove_mask(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Detect card remove mask for interrupt"]
    #[inline(always)]
    pub const fn set_card_remove_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Detect SDIO interrupt(data\\[1\\]) mask for interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio_mask(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Detect SDIO interrupt(data\\[1\\]) mask for interrupt"]
    #[inline(always)]
    pub const fn set_sdio_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "cache error mask for interrupt"]
    #[must_use]
    #[inline(always)]
    pub const fn cache_err_mask(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "cache error mask for interrupt"]
    #[inline(always)]
    pub const fn set_cache_err_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Ier {
    #[inline(always)]
    fn default() -> Ier {
        Ier(0)
    }
}
impl core::fmt::Debug for Ier {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ier")
            .field("cmd_done_mask", &self.cmd_done_mask())
            .field("cmd_rsp_crc_mask", &self.cmd_rsp_crc_mask())
            .field("cmd_timeout_mask", &self.cmd_timeout_mask())
            .field("data_done_mask", &self.data_done_mask())
            .field("data_crc_mask", &self.data_crc_mask())
            .field("data_timeout_mask", &self.data_timeout_mask())
            .field("startbit_error_mask", &self.startbit_error_mask())
            .field("fifo_underrun_mask", &self.fifo_underrun_mask())
            .field("fifo_overrun_mask", &self.fifo_overrun_mask())
            .field("cmd_sent_mask", &self.cmd_sent_mask())
            .field("card_insert_mask", &self.card_insert_mask())
            .field("card_remove_mask", &self.card_remove_mask())
            .field("sdio_mask", &self.sdio_mask())
            .field("cache_err_mask", &self.cache_err_mask())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ier {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ier {{ cmd_done_mask: {=bool:?}, cmd_rsp_crc_mask: {=bool:?}, cmd_timeout_mask: {=bool:?}, data_done_mask: {=bool:?}, data_crc_mask: {=bool:?}, data_timeout_mask: {=bool:?}, startbit_error_mask: {=bool:?}, fifo_underrun_mask: {=bool:?}, fifo_overrun_mask: {=bool:?}, cmd_sent_mask: {=bool:?}, card_insert_mask: {=bool:?}, card_remove_mask: {=bool:?}, sdio_mask: {=bool:?}, cache_err_mask: {=bool:?} }}" , self . cmd_done_mask () , self . cmd_rsp_crc_mask () , self . cmd_timeout_mask () , self . data_done_mask () , self . data_crc_mask () , self . data_timeout_mask () , self . startbit_error_mask () , self . fifo_underrun_mask () , self . fifo_overrun_mask () , self . cmd_sent_mask () , self . card_insert_mask () , self . card_remove_mask () , self . sdio_mask () , self . cache_err_mask ())
    }
}
#[doc = "response command argument1 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rar1(pub u32);
impl Rar1 {
    #[doc = "Response command content If long response, it is rsp_arg\\[39:8\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn rsp_arg1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Response command content If long response, it is rsp_arg\\[39:8\\]"]
    #[inline(always)]
    pub const fn set_rsp_arg1(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rar1 {
    #[inline(always)]
    fn default() -> Rar1 {
        Rar1(0)
    }
}
impl core::fmt::Debug for Rar1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rar1")
            .field("rsp_arg1", &self.rsp_arg1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rar1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rar1 {{ rsp_arg1: {=u32:?} }}", self.rsp_arg1())
    }
}
#[doc = "response command argument2 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rar2(pub u32);
impl Rar2 {
    #[doc = "Long response, it is rsp_arg\\[71:40\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn rsp_arg2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Long response, it is rsp_arg\\[71:40\\]"]
    #[inline(always)]
    pub const fn set_rsp_arg2(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rar2 {
    #[inline(always)]
    fn default() -> Rar2 {
        Rar2(0)
    }
}
impl core::fmt::Debug for Rar2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rar2")
            .field("rsp_arg2", &self.rsp_arg2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rar2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rar2 {{ rsp_arg2: {=u32:?} }}", self.rsp_arg2())
    }
}
#[doc = "response command argument3 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rar3(pub u32);
impl Rar3 {
    #[doc = "Long response, it is rsp_arg\\[103:72\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn rsp_arg3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Long response, it is rsp_arg\\[103:72\\]"]
    #[inline(always)]
    pub const fn set_rsp_arg3(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Rar3 {
    #[inline(always)]
    fn default() -> Rar3 {
        Rar3(0)
    }
}
impl core::fmt::Debug for Rar3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rar3")
            .field("rsp_arg3", &self.rsp_arg3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rar3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rar3 {{ rsp_arg3: {=u32:?} }}", self.rsp_arg3())
    }
}
#[doc = "response command argument4 register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rar4(pub u32);
impl Rar4 {
    #[doc = "Long response, it is rsp_arg\\[127:104\\]"]
    #[must_use]
    #[inline(always)]
    pub const fn rsp_arg4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Long response, it is rsp_arg\\[127:104\\]"]
    #[inline(always)]
    pub const fn set_rsp_arg4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Rar4 {
    #[inline(always)]
    fn default() -> Rar4 {
        Rar4(0)
    }
}
impl core::fmt::Debug for Rar4 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rar4")
            .field("rsp_arg4", &self.rsp_arg4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rar4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rar4 {{ rsp_arg4: {=u32:?} }}", self.rsp_arg4())
    }
}
#[doc = "response command index register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rir(pub u32);
impl Rir {
    #[doc = "Response command index"]
    #[must_use]
    #[inline(always)]
    pub const fn rsp_index(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Response command index"]
    #[inline(always)]
    pub const fn set_rsp_index(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Rir {
    #[inline(always)]
    fn default() -> Rir {
        Rir(0)
    }
}
impl core::fmt::Debug for Rir {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rir")
            .field("rsp_index", &self.rsp_index())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rir {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Rir {{ rsp_index: {=u8:?} }}", self.rsp_index())
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
#[doc = "command and data status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Command busy 1: busy, and when busy, start TX command bit is no usage and should not modify the relative register 0: command idle"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Command busy 1: busy, and when busy, start TX command bit is no usage and should not modify the relative register 0: command idle"]
    #[inline(always)]
    pub const fn set_cmd_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Command done Read 1: transfer command done, and start a new transfer will take the bit into 0 Read 0: command transferring or idle Write 1: clear the bit Write 0: no any influence to the bit"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_done(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Command done Read 1: transfer command done, and start a new transfer will take the bit into 0 Read 0: command transferring or idle Write 1: clear the bit Write 0: no any influence to the bit"]
    #[inline(always)]
    pub const fn set_cmd_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Command response CRC error status Read 1: response CRC error Read 0: response CRC right Write 1: clear the bit Write 0: no any influence to the bit"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_rsp_crc(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Command response CRC error status Read 1: response CRC error Read 0: response CRC right Write 1: clear the bit Write 0: no any influence to the bit"]
    #[inline(always)]
    pub const fn set_cmd_rsp_crc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Command timeout (response timeout) Read 1: timeout Read 0: no timeout Write 1: clear the bit Write 0: no any influence to the bit"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_timeout(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Command timeout (response timeout) Read 1: timeout Read 0: no timeout Write 1: clear the bit Write 0: no any influence to the bit"]
    #[inline(always)]
    pub const fn set_cmd_timeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transfer Data busy 1: busy, and when busy, start transfer data bit is no usage and you should not modify the relative register. If want to do this, first disable transfer data enable bit, then the busy bit will be back to 0, and this transfer will also be cancelled. 0: data idle"]
    #[must_use]
    #[inline(always)]
    pub const fn data_busy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transfer Data busy 1: busy, and when busy, start transfer data bit is no usage and you should not modify the relative register. If want to do this, first disable transfer data enable bit, then the busy bit will be back to 0, and this transfer will also be cancelled. 0: data idle"]
    #[inline(always)]
    pub const fn set_data_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Data transfer done Read 1: transfer data done, and start a new transfer will take the bit into 0 Read 0: data transferring or idle Write 1: clear the bit Write 0: no any influence to the bit"]
    #[must_use]
    #[inline(always)]
    pub const fn data_done(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Data transfer done Read 1: transfer data done, and start a new transfer will take the bit into 0 Read 0: data transferring or idle Write 1: clear the bit Write 0: no any influence to the bit"]
    #[inline(always)]
    pub const fn set_data_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Data CRC error Read 1: data CRC error Read 0: data CRC right Write 1: clear the bit Write 0: no any influence to the bit"]
    #[must_use]
    #[inline(always)]
    pub const fn data_crc(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Data CRC error Read 1: data CRC error Read 0: data CRC right Write 1: clear the bit Write 0: no any influence to the bit"]
    #[inline(always)]
    pub const fn set_data_crc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Data timeout Read 1: timeout Read 0: no timeout Write 1: clear the bit Write 0: no any influence to the bit"]
    #[must_use]
    #[inline(always)]
    pub const fn data_timeout(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Data timeout Read 1: timeout Read 0: no timeout Write 1: clear the bit Write 0: no any influence to the bit"]
    #[inline(always)]
    pub const fn set_data_timeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Wide bus start bits error Didn't detect all start bits in data bus Read 1: start bits error Read 0: no start bits error Write 1: clear the bit Write 0: no any influence to the bit"]
    #[must_use]
    #[inline(always)]
    pub const fn startbit_error(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Wide bus start bits error Didn't detect all start bits in data bus Read 1: start bits error Read 0: no start bits error Write 1: clear the bit Write 0: no any influence to the bit"]
    #[inline(always)]
    pub const fn set_startbit_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "FIFO underrun Read 1: FIFO underrun error Read 0: no FIFO underrun error Write 1: clear the bit Write 0: no any influence to the bit"]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_underrun(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO underrun Read 1: FIFO underrun error Read 0: no FIFO underrun error Write 1: clear the bit Write 0: no any influence to the bit"]
    #[inline(always)]
    pub const fn set_fifo_underrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "FIFO overrun Read 1: FIFO overrun error Read 0: no FIFO overrun error Write 1: clear the bit Write 0: no any influence to the bit"]
    #[must_use]
    #[inline(always)]
    pub const fn fifo_overrun(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO overrun Read 1: FIFO overrun error Read 0: no FIFO overrun error Write 1: clear the bit Write 0: no any influence to the bit"]
    #[inline(always)]
    pub const fn set_fifo_overrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Command sent (perhaps no response back yet) Read 1: command sent. When command start bit is set, the bit will also be back to 0 Read 0: command transferring or others Write 1: clear the bit Write 0: no any influence to the bit"]
    #[must_use]
    #[inline(always)]
    pub const fn cmd_sent(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Command sent (perhaps no response back yet) Read 1: command sent. When command start bit is set, the bit will also be back to 0 Read 0: command transferring or others Write 1: clear the bit Write 0: no any influence to the bit"]
    #[inline(always)]
    pub const fn set_cmd_sent(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Detect card inserted Read 1: detect card inserted. When detect card removed bit is set, the bit will also be back to 0 Read 0: no meaning Write 1: clear the bit Write 0: no any influence to the bit"]
    #[must_use]
    #[inline(always)]
    pub const fn card_insert(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Detect card inserted Read 1: detect card inserted. When detect card removed bit is set, the bit will also be back to 0 Read 0: no meaning Write 1: clear the bit Write 0: no any influence to the bit"]
    #[inline(always)]
    pub const fn set_card_insert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Detect card removed Read 1: detect card removed. When detect card inserted bit is set, the bit will also be back to 0 Read 0: no meaning Write 1: clear the bit Write 0: no any influence to the bit"]
    #[must_use]
    #[inline(always)]
    pub const fn card_remove(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Detect card removed Read 1: detect card removed. When detect card inserted bit is set, the bit will also be back to 0 Read 0: no meaning Write 1: clear the bit Write 0: no any influence to the bit"]
    #[inline(always)]
    pub const fn set_card_remove(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Card exist status Read 1: card exist Read 0: no card exist This bit will be valid after enable detect card."]
    #[must_use]
    #[inline(always)]
    pub const fn card_exist(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Card exist status Read 1: card exist Read 0: no card exist This bit will be valid after enable detect card."]
    #[inline(always)]
    pub const fn set_card_exist(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Detect SDIO Card Interrupt Read 1: detect sdio card generating interrupt Read 0: no interrupt Write 1: clear the bit Write 0: no any influence to the bit"]
    #[must_use]
    #[inline(always)]
    pub const fn sdio(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Detect SDIO Card Interrupt Read 1: detect sdio card generating interrupt Read 0: no interrupt Write 1: clear the bit Write 0: no any influence to the bit"]
    #[inline(always)]
    pub const fn set_sdio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Detect cache error Read 1: cache error occur Read 0: no cache error Write 1: clear the bit Write 0: no any influence to the bit"]
    #[must_use]
    #[inline(always)]
    pub const fn cache_err(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Detect cache error Read 1: cache error occur Read 0: no cache error Write 1: clear the bit Write 0: no any influence to the bit"]
    #[inline(always)]
    pub const fn set_cache_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
impl core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr")
            .field("cmd_busy", &self.cmd_busy())
            .field("cmd_done", &self.cmd_done())
            .field("cmd_rsp_crc", &self.cmd_rsp_crc())
            .field("cmd_timeout", &self.cmd_timeout())
            .field("data_busy", &self.data_busy())
            .field("data_done", &self.data_done())
            .field("data_crc", &self.data_crc())
            .field("data_timeout", &self.data_timeout())
            .field("startbit_error", &self.startbit_error())
            .field("fifo_underrun", &self.fifo_underrun())
            .field("fifo_overrun", &self.fifo_overrun())
            .field("cmd_sent", &self.cmd_sent())
            .field("card_insert", &self.card_insert())
            .field("card_remove", &self.card_remove())
            .field("card_exist", &self.card_exist())
            .field("sdio", &self.sdio())
            .field("cache_err", &self.cache_err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Sr {{ cmd_busy: {=bool:?}, cmd_done: {=bool:?}, cmd_rsp_crc: {=bool:?}, cmd_timeout: {=bool:?}, data_busy: {=bool:?}, data_done: {=bool:?}, data_crc: {=bool:?}, data_timeout: {=bool:?}, startbit_error: {=bool:?}, fifo_underrun: {=bool:?}, fifo_overrun: {=bool:?}, cmd_sent: {=bool:?}, card_insert: {=bool:?}, card_remove: {=bool:?}, card_exist: {=bool:?}, sdio: {=bool:?}, cache_err: {=bool:?} }}" , self . cmd_busy () , self . cmd_done () , self . cmd_rsp_crc () , self . cmd_timeout () , self . data_busy () , self . data_done () , self . data_crc () , self . data_timeout () , self . startbit_error () , self . fifo_underrun () , self . fifo_overrun () , self . cmd_sent () , self . card_insert () , self . card_remove () , self . card_exist () , self . sdio () , self . cache_err ())
    }
}
#[doc = "timeout count register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tor(pub u32);
impl Tor {
    #[doc = "Used to determine how much time waiting response or data bus busy is timeout, and decreased under card clock. Set to 400000 for 1s timeout if interface clock is 400KHz."]
    #[must_use]
    #[inline(always)]
    pub const fn timeout_cnt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Used to determine how much time waiting response or data bus busy is timeout, and decreased under card clock. Set to 400000 for 1s timeout if interface clock is 400KHz."]
    #[inline(always)]
    pub const fn set_timeout_cnt(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tor {
    #[inline(always)]
    fn default() -> Tor {
        Tor(0)
    }
}
impl core::fmt::Debug for Tor {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tor")
            .field("timeout_cnt", &self.timeout_cnt())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tor {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Tor {{ timeout_cnt: {=u32:?} }}", self.timeout_cnt())
    }
}
