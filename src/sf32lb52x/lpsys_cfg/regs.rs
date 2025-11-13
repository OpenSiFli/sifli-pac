#[doc = "Debug Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgr(pub u32);
impl Dbgr {
    #[doc = "reserved for debug"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_l(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn set_sel_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "reserved for debug"]
    #[must_use]
    #[inline(always)]
    pub const fn sel_h(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn set_sel_h(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "reserved for debug"]
    #[must_use]
    #[inline(always)]
    pub const fn biten_l(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn set_biten_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "reserved for debug"]
    #[must_use]
    #[inline(always)]
    pub const fn biten_h(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn set_biten_h(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "reserved for debug"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_sel(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn set_clk_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
    #[doc = "reserved for debug"]
    #[must_use]
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn set_clk_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "set 1 to send NMI interrupt to HCPU"]
    #[must_use]
    #[inline(always)]
    pub const fn lp2hp_nmi(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "set 1 to send NMI interrupt to HCPU"]
    #[inline(always)]
    pub const fn set_lp2hp_nmi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "HP2LP NMI interrupt enable"]
    #[must_use]
    #[inline(always)]
    pub const fn hp2lp_nmie(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "HP2LP NMI interrupt enable"]
    #[inline(always)]
    pub const fn set_hp2lp_nmie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "HP2LP NMI interrupt flag"]
    #[must_use]
    #[inline(always)]
    pub const fn hp2lp_nmif(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "HP2LP NMI interrupt flag"]
    #[inline(always)]
    pub const fn set_hp2lp_nmif(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "reserved for debug"]
    #[must_use]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dbgr {
    #[inline(always)]
    fn default() -> Dbgr {
        Dbgr(0)
    }
}
impl core::fmt::Debug for Dbgr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbgr")
            .field("sel_l", &self.sel_l())
            .field("sel_h", &self.sel_h())
            .field("biten_l", &self.biten_l())
            .field("biten_h", &self.biten_h())
            .field("clk_sel", &self.clk_sel())
            .field("clk_en", &self.clk_en())
            .field("lp2hp_nmi", &self.lp2hp_nmi())
            .field("hp2lp_nmie", &self.hp2lp_nmie())
            .field("hp2lp_nmif", &self.hp2lp_nmif())
            .field("ready", &self.ready())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbgr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Dbgr {{ sel_l: {=u8:?}, sel_h: {=u8:?}, biten_l: {=u8:?}, biten_h: {=u8:?}, clk_sel: {=u8:?}, clk_en: {=bool:?}, lp2hp_nmi: {=bool:?}, hp2lp_nmie: {=bool:?}, hp2lp_nmif: {=bool:?}, ready: {=bool:?} }}" , self . sel_l () , self . sel_h () , self . biten_l () , self . biten_h () , self . clk_sel () , self . clk_en () , self . lp2hp_nmi () , self . hp2lp_nmie () , self . hp2lp_nmif () , self . ready ())
    }
}
#[doc = "Memory Debug Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mdbgr(pub u32);
impl Mdbgr {
    #[doc = "reserved for debug"]
    #[must_use]
    #[inline(always)]
    pub const fn ls_ram0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn set_ls_ram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "reserved for debug"]
    #[must_use]
    #[inline(always)]
    pub const fn ls_ram1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn set_ls_ram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "reserved for debug"]
    #[must_use]
    #[inline(always)]
    pub const fn ls_rom(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn set_ls_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "reserved for debug"]
    #[must_use]
    #[inline(always)]
    pub const fn pd_rom(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn set_pd_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Mdbgr {
    #[inline(always)]
    fn default() -> Mdbgr {
        Mdbgr(0)
    }
}
impl core::fmt::Debug for Mdbgr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mdbgr")
            .field("ls_ram0", &self.ls_ram0())
            .field("ls_ram1", &self.ls_ram1())
            .field("ls_rom", &self.ls_rom())
            .field("pd_rom", &self.pd_rom())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mdbgr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Mdbgr {{ ls_ram0: {=bool:?}, ls_ram1: {=bool:?}, ls_rom: {=bool:?}, pd_rom: {=bool:?} }}" , self . ls_ram0 () , self . ls_ram1 () , self . ls_rom () , self . pd_rom ())
    }
}
#[doc = "PTA Pin Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PtaPinr(pub u32);
impl PtaPinr {
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[must_use]
    #[inline(always)]
    pub const fn bt_active(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[inline(always)]
    pub const fn set_bt_active(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[must_use]
    #[inline(always)]
    pub const fn bt_collision(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[inline(always)]
    pub const fn set_bt_collision(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[must_use]
    #[inline(always)]
    pub const fn bt_priority(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[inline(always)]
    pub const fn set_bt_priority(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[must_use]
    #[inline(always)]
    pub const fn wlan_active(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[inline(always)]
    pub const fn set_wlan_active(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
}
impl Default for PtaPinr {
    #[inline(always)]
    fn default() -> PtaPinr {
        PtaPinr(0)
    }
}
impl core::fmt::Debug for PtaPinr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PtaPinr")
            .field("bt_active", &self.bt_active())
            .field("bt_collision", &self.bt_collision())
            .field("bt_priority", &self.bt_priority())
            .field("wlan_active", &self.wlan_active())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PtaPinr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "PtaPinr {{ bt_active: {=u8:?}, bt_collision: {=u8:?}, bt_priority: {=u8:?}, wlan_active: {=u8:?} }}" , self . bt_active () , self . bt_collision () , self . bt_priority () , self . wlan_active ())
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
#[doc = "Mirrored RTC Date Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcDr(pub u32);
impl RtcDr {
    #[doc = "Date units in BCD format"]
    #[must_use]
    #[inline(always)]
    pub const fn du(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Date units in BCD format"]
    #[inline(always)]
    pub const fn set_du(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Date tens in BCD format"]
    #[must_use]
    #[inline(always)]
    pub const fn dt(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Date tens in BCD format"]
    #[inline(always)]
    pub const fn set_dt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Month units in BCD format"]
    #[must_use]
    #[inline(always)]
    pub const fn mu(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Month units in BCD format"]
    #[inline(always)]
    pub const fn set_mu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Month tens in BCD format"]
    #[must_use]
    #[inline(always)]
    pub const fn mt(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Month tens in BCD format"]
    #[inline(always)]
    pub const fn set_mt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Week day units 000: forbidden 001: Monday ... 111: Sunday"]
    #[must_use]
    #[inline(always)]
    pub const fn wd(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Week day units 000: forbidden 001: Monday ... 111: Sunday"]
    #[inline(always)]
    pub const fn set_wd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Year units in BCD format"]
    #[must_use]
    #[inline(always)]
    pub const fn yu(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Year units in BCD format"]
    #[inline(always)]
    pub const fn set_yu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Year tens in BCD format"]
    #[must_use]
    #[inline(always)]
    pub const fn yt(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "Year tens in BCD format"]
    #[inline(always)]
    pub const fn set_yt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Century flag"]
    #[must_use]
    #[inline(always)]
    pub const fn cb(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Century flag"]
    #[inline(always)]
    pub const fn set_cb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "reserved for debug"]
    #[must_use]
    #[inline(always)]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RtcDr {
    #[inline(always)]
    fn default() -> RtcDr {
        RtcDr(0)
    }
}
impl core::fmt::Debug for RtcDr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RtcDr")
            .field("du", &self.du())
            .field("dt", &self.dt())
            .field("mu", &self.mu())
            .field("mt", &self.mt())
            .field("wd", &self.wd())
            .field("yu", &self.yu())
            .field("yt", &self.yt())
            .field("cb", &self.cb())
            .field("err", &self.err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RtcDr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "RtcDr {{ du: {=u8:?}, dt: {=u8:?}, mu: {=u8:?}, mt: {=bool:?}, wd: {=u8:?}, yu: {=u8:?}, yt: {=u8:?}, cb: {=bool:?}, err: {=bool:?} }}" , self . du () , self . dt () , self . mu () , self . mt () , self . wd () , self . yu () , self . yt () , self . cb () , self . err ())
    }
}
#[doc = "Mirrored RTC Time Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RtcTr(pub u32);
impl RtcTr {
    #[doc = "Sub-second counter"]
    #[must_use]
    #[inline(always)]
    pub const fn ss(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Sub-second counter"]
    #[inline(always)]
    pub const fn set_ss(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Second units in BCD format"]
    #[must_use]
    #[inline(always)]
    pub const fn su(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "Second units in BCD format"]
    #[inline(always)]
    pub const fn set_su(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Second tens in BCD format"]
    #[must_use]
    #[inline(always)]
    pub const fn st(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "Second tens in BCD format"]
    #[inline(always)]
    pub const fn set_st(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "Minute units in BCD format"]
    #[must_use]
    #[inline(always)]
    pub const fn mnu(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x0f;
        val as u8
    }
    #[doc = "Minute units in BCD format"]
    #[inline(always)]
    pub const fn set_mnu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 18usize)) | (((val as u32) & 0x0f) << 18usize);
    }
    #[doc = "Minute tens in BCD format"]
    #[must_use]
    #[inline(always)]
    pub const fn mnt(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x07;
        val as u8
    }
    #[doc = "Minute tens in BCD format"]
    #[inline(always)]
    pub const fn set_mnt(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 22usize)) | (((val as u32) & 0x07) << 22usize);
    }
    #[doc = "Hour units in BCD format"]
    #[must_use]
    #[inline(always)]
    pub const fn hu(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x0f;
        val as u8
    }
    #[doc = "Hour units in BCD format"]
    #[inline(always)]
    pub const fn set_hu(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 25usize)) | (((val as u32) & 0x0f) << 25usize);
    }
    #[doc = "Hour tens in BCD format"]
    #[must_use]
    #[inline(always)]
    pub const fn ht(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x03;
        val as u8
    }
    #[doc = "Hour tens in BCD format"]
    #[inline(always)]
    pub const fn set_ht(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
    }
    #[doc = "AM/PM notation 0: AM 1: PM"]
    #[must_use]
    #[inline(always)]
    pub const fn pm(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "AM/PM notation 0: AM 1: PM"]
    #[inline(always)]
    pub const fn set_pm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for RtcTr {
    #[inline(always)]
    fn default() -> RtcTr {
        RtcTr(0)
    }
}
impl core::fmt::Debug for RtcTr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RtcTr")
            .field("ss", &self.ss())
            .field("su", &self.su())
            .field("st", &self.st())
            .field("mnu", &self.mnu())
            .field("mnt", &self.mnt())
            .field("hu", &self.hu())
            .field("ht", &self.ht())
            .field("pm", &self.pm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RtcTr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "RtcTr {{ ss: {=u16:?}, su: {=u8:?}, st: {=u8:?}, mnu: {=u8:?}, mnt: {=u8:?}, hu: {=u8:?}, ht: {=u8:?}, pm: {=bool:?} }}" , self . ss () , self . su () , self . st () , self . mnu () , self . mnt () , self . hu () , self . ht () , self . pm ())
    }
}
#[doc = "System Configure Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscr(pub u32);
impl Syscr {
    #[doc = "If set to 1, WDT2 reset will reboot the whole chip"]
    #[must_use]
    #[inline(always)]
    pub const fn wdt2_reboot(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, WDT2 reset will reboot the whole chip"]
    #[inline(always)]
    pub const fn set_wdt2_reboot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "reserved for debug"]
    #[must_use]
    #[inline(always)]
    pub const fn dbg_swap(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x03;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn set_dbg_swap(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
    }
    #[doc = "select work mode 0: D 1: S"]
    #[must_use]
    #[inline(always)]
    pub const fn ldo_vsel(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "select work mode 0: D 1: S"]
    #[inline(always)]
    pub const fn set_ldo_vsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Syscr {
    #[inline(always)]
    fn default() -> Syscr {
        Syscr(0)
    }
}
impl core::fmt::Debug for Syscr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Syscr")
            .field("wdt2_reboot", &self.wdt2_reboot())
            .field("dbg_swap", &self.dbg_swap())
            .field("ldo_vsel", &self.ldo_vsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Syscr {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(
            f,
            "Syscr {{ wdt2_reboot: {=bool:?}, dbg_swap: {=u8:?}, ldo_vsel: {=bool:?} }}",
            self.wdt2_reboot(),
            self.dbg_swap(),
            self.ldo_vsel()
        )
    }
}
#[doc = "ULP Memory Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ulpmcr(pub u32);
impl Ulpmcr {
    #[doc = "reserved for debug"]
    #[must_use]
    #[inline(always)]
    pub const fn ram_rm(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn set_ram_rm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "reserved for debug"]
    #[must_use]
    #[inline(always)]
    pub const fn ram_rme(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn set_ram_rme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "reserved for debug"]
    #[must_use]
    #[inline(always)]
    pub const fn ram_ra(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn set_ram_ra(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "reserved for debug"]
    #[must_use]
    #[inline(always)]
    pub const fn ram_wa(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x07;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn set_ram_wa(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 7usize)) | (((val as u32) & 0x07) << 7usize);
    }
    #[doc = "reserved for debug"]
    #[must_use]
    #[inline(always)]
    pub const fn ram_wpulse(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x07;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn set_ram_wpulse(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 10usize)) | (((val as u32) & 0x07) << 10usize);
    }
    #[doc = "reserved for debug"]
    #[must_use]
    #[inline(always)]
    pub const fn ram_rdy(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn set_ram_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "reserved for debug"]
    #[must_use]
    #[inline(always)]
    pub const fn rom_rm(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn set_rom_rm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "reserved for debug"]
    #[must_use]
    #[inline(always)]
    pub const fn rom_rme(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn set_rom_rme(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "reserved for debug"]
    #[must_use]
    #[inline(always)]
    pub const fn rom_dis(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn set_rom_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "reserved for debug"]
    #[must_use]
    #[inline(always)]
    pub const fn force_on(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "reserved for debug"]
    #[inline(always)]
    pub const fn set_force_on(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ulpmcr {
    #[inline(always)]
    fn default() -> Ulpmcr {
        Ulpmcr(0)
    }
}
impl core::fmt::Debug for Ulpmcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ulpmcr")
            .field("ram_rm", &self.ram_rm())
            .field("ram_rme", &self.ram_rme())
            .field("ram_ra", &self.ram_ra())
            .field("ram_wa", &self.ram_wa())
            .field("ram_wpulse", &self.ram_wpulse())
            .field("ram_rdy", &self.ram_rdy())
            .field("rom_rm", &self.rom_rm())
            .field("rom_rme", &self.rom_rme())
            .field("rom_dis", &self.rom_dis())
            .field("force_on", &self.force_on())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ulpmcr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Ulpmcr {{ ram_rm: {=u8:?}, ram_rme: {=bool:?}, ram_ra: {=u8:?}, ram_wa: {=u8:?}, ram_wpulse: {=u8:?}, ram_rdy: {=bool:?}, rom_rm: {=u8:?}, rom_rme: {=bool:?}, rom_dis: {=bool:?}, force_on: {=bool:?} }}" , self . ram_rm () , self . ram_rme () , self . ram_ra () , self . ram_wa () , self . ram_wpulse () , self . ram_rdy () , self . rom_rm () , self . rom_rme () , self . rom_dis () , self . force_on ())
    }
}
#[doc = "USART4 Pin Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usart4Pinr(pub u32);
impl Usart4Pinr {
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[must_use]
    #[inline(always)]
    pub const fn txd_pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[inline(always)]
    pub const fn set_txd_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[must_use]
    #[inline(always)]
    pub const fn rxd_pin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[inline(always)]
    pub const fn set_rxd_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[must_use]
    #[inline(always)]
    pub const fn rts_pin(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[inline(always)]
    pub const fn set_rts_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[must_use]
    #[inline(always)]
    pub const fn cts_pin(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[inline(always)]
    pub const fn set_cts_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
}
impl Default for Usart4Pinr {
    #[inline(always)]
    fn default() -> Usart4Pinr {
        Usart4Pinr(0)
    }
}
impl core::fmt::Debug for Usart4Pinr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usart4Pinr")
            .field("txd_pin", &self.txd_pin())
            .field("rxd_pin", &self.rxd_pin())
            .field("rts_pin", &self.rts_pin())
            .field("cts_pin", &self.cts_pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usart4Pinr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usart4Pinr {{ txd_pin: {=u8:?}, rxd_pin: {=u8:?}, rts_pin: {=u8:?}, cts_pin: {=u8:?} }}" , self . txd_pin () , self . rxd_pin () , self . rts_pin () , self . cts_pin ())
    }
}
#[doc = "USART5 Pin Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usart5Pinr(pub u32);
impl Usart5Pinr {
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[must_use]
    #[inline(always)]
    pub const fn txd_pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[inline(always)]
    pub const fn set_txd_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[must_use]
    #[inline(always)]
    pub const fn rxd_pin(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[inline(always)]
    pub const fn set_rxd_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[must_use]
    #[inline(always)]
    pub const fn rts_pin(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[inline(always)]
    pub const fn set_rts_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[must_use]
    #[inline(always)]
    pub const fn cts_pin(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x07;
        val as u8
    }
    #[doc = "Connect function pin to selected IO(PB). 0 to 3 for PB00 to PB03. Other values for floating."]
    #[inline(always)]
    pub const fn set_cts_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
    }
}
impl Default for Usart5Pinr {
    #[inline(always)]
    fn default() -> Usart5Pinr {
        Usart5Pinr(0)
    }
}
impl core::fmt::Debug for Usart5Pinr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usart5Pinr")
            .field("txd_pin", &self.txd_pin())
            .field("rxd_pin", &self.rxd_pin())
            .field("rts_pin", &self.rts_pin())
            .field("cts_pin", &self.cts_pin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usart5Pinr {
    fn format(&self, f: defmt::Formatter) {
        defmt :: write ! (f , "Usart5Pinr {{ txd_pin: {=u8:?}, rxd_pin: {=u8:?}, rts_pin: {=u8:?}, cts_pin: {=u8:?} }}" , self . txd_pin () , self . rxd_pin () , self . rts_pin () , self . cts_pin ())
    }
}
