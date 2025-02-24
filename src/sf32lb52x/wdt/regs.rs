#[doc = "WatchDog Counter Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdtCcr(pub u32);
impl WdtCcr {
    #[doc = "SinglePulse /Write 8'h76 to restart, write8'h34 to stop, else do nothing"]
    #[inline(always)]
    pub const fn counter_control(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "SinglePulse /Write 8'h76 to restart, write8'h34 to stop, else do nothing"]
    #[inline(always)]
    pub fn set_counter_control(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for WdtCcr {
    #[inline(always)]
    fn default() -> WdtCcr {
        WdtCcr(0)
    }
}
impl core::fmt::Debug for WdtCcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WdtCcr")
            .field("counter_control", &self.counter_control())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WdtCcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WdtCcr {{ counter_control: {=u8:?} }}",
            self.counter_control()
        )
    }
}
#[doc = "WatchDog Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdtCr(pub u32);
impl WdtCr {
    #[doc = "reset pulse length in number of wdt clock cycles"]
    #[inline(always)]
    pub const fn reset_length(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "reset pulse length in number of wdt clock cycles"]
    #[inline(always)]
    pub fn set_reset_length(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "0:reset only, 1:interrupt and reset"]
    #[inline(always)]
    pub const fn response_mode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "0:reset only, 1:interrupt and reset"]
    #[inline(always)]
    pub fn set_response_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for WdtCr {
    #[inline(always)]
    fn default() -> WdtCr {
        WdtCr(0)
    }
}
impl core::fmt::Debug for WdtCr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WdtCr")
            .field("reset_length", &self.reset_length())
            .field("response_mode", &self.response_mode())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WdtCr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WdtCr {{ reset_length: {=u8:?}, response_mode: {=bool:?} }}",
            self.reset_length(),
            self.response_mode()
        )
    }
}
#[doc = "WatchDog Counter Value 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdtCvr0(pub u32);
impl WdtCvr0 {
    #[doc = "Count Value for 1st TimeOut"]
    #[inline(always)]
    pub const fn count_value_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Count Value for 1st TimeOut"]
    #[inline(always)]
    pub fn set_count_value_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for WdtCvr0 {
    #[inline(always)]
    fn default() -> WdtCvr0 {
        WdtCvr0(0)
    }
}
impl core::fmt::Debug for WdtCvr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WdtCvr0")
            .field("count_value_0", &self.count_value_0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WdtCvr0 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WdtCvr0 {{ count_value_0: {=u32:?} }}",
            self.count_value_0()
        )
    }
}
#[doc = "WatchDog Counter Value 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdtCvr1(pub u32);
impl WdtCvr1 {
    #[doc = "Count Value for 2nd TimeOut"]
    #[inline(always)]
    pub const fn count_value_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Count Value for 2nd TimeOut"]
    #[inline(always)]
    pub fn set_count_value_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for WdtCvr1 {
    #[inline(always)]
    fn default() -> WdtCvr1 {
        WdtCvr1(0)
    }
}
impl core::fmt::Debug for WdtCvr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WdtCvr1")
            .field("count_value_1", &self.count_value_1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WdtCvr1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WdtCvr1 {{ count_value_1: {=u32:?} }}",
            self.count_value_1()
        )
    }
}
#[doc = "WatchDog Flag Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdtFg(pub u32);
impl WdtFg {
    #[doc = "SinglePulse/A pulse to clear reset flag"]
    #[inline(always)]
    pub const fn rst_fg_clr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SinglePulse/A pulse to clear reset flag"]
    #[inline(always)]
    pub fn set_rst_fg_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1 indicates wdt has already reset system"]
    #[inline(always)]
    pub const fn rst_fg(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1 indicates wdt has already reset system"]
    #[inline(always)]
    pub fn set_rst_fg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SinglePulse/A pulse to clear sync flag"]
    #[inline(always)]
    pub const fn sync_fg_clr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SinglePulse/A pulse to clear sync flag"]
    #[inline(always)]
    pub fn set_sync_fg_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "1 indicates one transition from system clk to wdt clk has complicated"]
    #[inline(always)]
    pub const fn sync_fg(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1 indicates one transition from system clk to wdt clk has complicated"]
    #[inline(always)]
    pub fn set_sync_fg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for WdtFg {
    #[inline(always)]
    fn default() -> WdtFg {
        WdtFg(0)
    }
}
impl core::fmt::Debug for WdtFg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WdtFg")
            .field("rst_fg_clr", &self.rst_fg_clr())
            .field("rst_fg", &self.rst_fg())
            .field("sync_fg_clr", &self.sync_fg_clr())
            .field("sync_fg", &self.sync_fg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WdtFg {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "WdtFg {{ rst_fg_clr: {=bool:?}, rst_fg: {=bool:?}, sync_fg_clr: {=bool:?}, sync_fg: {=bool:?} }}" , self . rst_fg_clr () , self . rst_fg () , self . sync_fg_clr () , self . sync_fg ())
    }
}
#[doc = "WatchDog Interrupt Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdtIcr(pub u32);
impl WdtIcr {
    #[doc = "SinglePulse /A pulse to clear interrupt"]
    #[inline(always)]
    pub const fn int_clr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SinglePulse /A pulse to clear interrupt"]
    #[inline(always)]
    pub fn set_int_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for WdtIcr {
    #[inline(always)]
    fn default() -> WdtIcr {
        WdtIcr(0)
    }
}
impl core::fmt::Debug for WdtIcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WdtIcr")
            .field("int_clr", &self.int_clr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WdtIcr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "WdtIcr {{ int_clr: {=bool:?} }}", self.int_clr())
    }
}
#[doc = "WatchDog Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdtSr(pub u32);
impl WdtSr {
    #[doc = "Interrupt assert when 1"]
    #[inline(always)]
    pub const fn int_assert(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt assert when 1"]
    #[inline(always)]
    pub fn set_int_assert(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Watchdog runs when 1, else 0"]
    #[inline(always)]
    pub const fn wdt_active(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Watchdog runs when 1, else 0"]
    #[inline(always)]
    pub fn set_wdt_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for WdtSr {
    #[inline(always)]
    fn default() -> WdtSr {
        WdtSr(0)
    }
}
impl core::fmt::Debug for WdtSr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WdtSr")
            .field("int_assert", &self.int_assert())
            .field("wdt_active", &self.wdt_active())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WdtSr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WdtSr {{ int_assert: {=bool:?}, wdt_active: {=bool:?} }}",
            self.int_assert(),
            self.wdt_active()
        )
    }
}
#[doc = "WatchDog Write Protect Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WdtWp(pub u32);
impl WdtWp {
    #[doc = "write 0x58ab99fc generate write_protect, write 0x51ff8621 to release"]
    #[inline(always)]
    pub const fn wrpt(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "write 0x58ab99fc generate write_protect, write 0x51ff8621 to release"]
    #[inline(always)]
    pub fn set_wrpt(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
    }
    #[doc = "1 indicates write protect is active"]
    #[inline(always)]
    pub const fn wrpt_st(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "1 indicates write protect is active"]
    #[inline(always)]
    pub fn set_wrpt_st(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for WdtWp {
    #[inline(always)]
    fn default() -> WdtWp {
        WdtWp(0)
    }
}
impl core::fmt::Debug for WdtWp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WdtWp")
            .field("wrpt", &self.wrpt())
            .field("wrpt_st", &self.wrpt_st())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WdtWp {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "WdtWp {{ wrpt: {=u32:?}, wrpt_st: {=bool:?} }}",
            self.wrpt(),
            self.wrpt_st()
        )
    }
}
